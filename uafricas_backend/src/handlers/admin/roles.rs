use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::role::*;
use crate::models::pagination::PaginationParams;
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ────────────────────────────────────────────────────────────────
// GET /api/admin/roles — Liste paginee
// ────────────────────────────────────────────────────────────────

pub async fn lister_roles(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminRoleQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let mut conditions: Vec<String> = Vec::new();
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref recherche) = params.recherche {
        let trimmed = recherche.trim();
        if !trimmed.is_empty() {
            let terme = format!("%{}%", trimmed.to_lowercase());
            conditions.push(format!(
                "(LOWER(r.nom) LIKE ${idx} OR LOWER(r.slug) LIKE ${idx} OR LOWER(r.description) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = if conditions.is_empty() {
        "TRUE".to_string()
    } else {
        conditions.join(" AND ")
    };

    let tri_colonne = pagination.colonne_tri(ROLE_TRI_COLONNES, "created_at");
    let tri_direction = pagination.direction_tri();
    let order_clause = format!("ORDER BY r.{} {}", tri_colonne, tri_direction);

    let count_query = format!(
        "SELECT COUNT(*) FROM iam.role r WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_query = format!(
        "SELECT r.id, r.nom, r.slug, r.description, r.est_systeme,
                (SELECT COUNT(*) FROM iam.utilisateur_role ur WHERE ur.role_id = r.id) AS nombre_utilisateurs,
                r.created_at
         FROM iam.role r
         WHERE {} {} LIMIT ${} OFFSET ${}",
        where_clause, order_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, AdminRoleListeResponse>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let roles = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(crate::models::pagination::PaginatedResponse::new(
            roles, total, page, par_page,
        )),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// GET /api/admin/roles/{id} — Detail avec permissions
// ────────────────────────────────────────────────────────────────

pub async fn obtenir_role(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "voir");

    let id = chemin.into_inner();

    let row = sqlx::query_as::<_, AdminRoleRow>(
        "SELECT id, nom, slug, description, est_systeme, created_at, updated_at
         FROM iam.role WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Role non trouve".into()))?;

    let permissions = sqlx::query_as::<_, PermissionInfo>(
        "SELECT p.id, p.nom, p.slug, p.description, p.type_ressource, p.action
         FROM iam.permission p
         JOIN iam.role_permission rp ON p.id = rp.permission_id
         WHERE rp.role_id = $1
         ORDER BY p.type_ressource, p.action"
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let nombre_utilisateurs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM iam.utilisateur_role WHERE role_id = $1"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    let response = AdminRoleDetailResponse {
        id: row.id,
        nom: row.nom,
        slug: row.slug,
        description: row.description,
        est_systeme: row.est_systeme,
        permissions,
        nombre_utilisateurs,
        created_at: row.created_at,
        updated_at: row.updated_at,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(response),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// POST /api/admin/roles — Creation
// ────────────────────────────────────────────────────────────────

pub async fn creer_role(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerRoleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    if body.nom.trim().is_empty() {
        return Err(ApiErreur::Validation("Nom du role requis".into()));
    }

    let slug = body.nom.trim().to_lowercase()
        .replace(' ', "_")
        .replace(['\'', '"', '.', ','], "");

    // Verifier unicite
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.role WHERE slug = $1)"
    )
    .bind(&slug)
    .fetch_one(pool.get_ref())
    .await?;

    if existe {
        return Err(ApiErreur::Conflit("Un role avec ce slug existe deja".into()));
    }

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.role (nom, slug, description, est_systeme)
         VALUES ($1, $2, $3, false)
         RETURNING id"
    )
    .bind(body.nom.trim())
    .bind(&slug)
    .bind(body.description.as_deref())
    .fetch_one(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le role {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "iam",
        "role",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "slug": slug })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// PUT /api/admin/roles/{id} — Modification
// ────────────────────────────────────────────────────────────────

pub async fn modifier_role(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierRoleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let id = chemin.into_inner();

    // Verifier que le role n'est pas systeme
    let est_systeme: bool = sqlx::query_scalar(
        "SELECT est_systeme FROM iam.role WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Role non trouve".into()))?;

    if est_systeme {
        return Err(ApiErreur::AccesInterdit("Les roles systeme ne peuvent pas etre modifies".into()));
    }

    let mut sets: Vec<String> = Vec::new();
    let mut bind_index = 1u32;
    let mut bind_strings: Vec<String> = Vec::new();

    if let Some(ref nom) = body.nom {
        sets.push(format!("nom = ${}", bind_index));
        bind_strings.push(nom.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_strings.push(desc.trim().to_string());
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());

    let query = format!(
        "UPDATE iam.role SET {} WHERE id = ${}",
        sets.join(", "),
        bind_index
    );

    let mut q = sqlx::query(&query);
    for s in &bind_strings {
        q = q.bind(s.as_str());
    }
    q = q.bind(id);

    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie le role {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "iam",
        "role",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// DELETE /api/admin/roles/{id} — Suppression
// ────────────────────────────────────────────────────────────────

pub async fn supprimer_role(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "supprimer");

    let id = chemin.into_inner();

    // Verifier que le role n'est pas systeme
    let est_systeme: bool = sqlx::query_scalar(
        "SELECT est_systeme FROM iam.role WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Role non trouve".into()))?;

    if est_systeme {
        return Err(ApiErreur::AccesInterdit("Les roles systeme ne peuvent pas etre supprimes".into()));
    }

    // Verifier qu'aucun utilisateur n'est assigne
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM iam.utilisateur_role WHERE role_id = $1"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    if count > 0 {
        return Err(ApiErreur::Conflit(
            format!("{} utilisateur(s) sont encore assignes a ce role", count)
        ));
    }

    sqlx::query("DELETE FROM iam.role WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    log::info!("Admin {} a supprime le role {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "iam",
        "role",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// POST /api/admin/roles/{id}/permissions — Assigner permissions
// ────────────────────────────────────────────────────────────────

pub async fn assigner_permissions(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<AssignerPermissionsRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let role_id = chemin.into_inner();

    // Verifier que le role existe
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.role WHERE id = $1)"
    )
    .bind(role_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Role non trouve".into()));
    }

    // Supprimer les permissions existantes et reassigner
    sqlx::query("DELETE FROM iam.role_permission WHERE role_id = $1")
        .bind(role_id)
        .execute(pool.get_ref())
        .await?;

    for perm_id in &body.permission_ids {
        sqlx::query(
            "INSERT INTO iam.role_permission (role_id, permission_id)
             VALUES ($1, $2)
             ON CONFLICT DO NOTHING"
        )
        .bind(role_id)
        .bind(perm_id)
        .execute(pool.get_ref())
        .await?;
    }

    log::info!(
        "Admin {} a assigne {} permissions au role {}",
        admin.id,
        body.permission_ids.len(),
        role_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "iam",
        "role_permission",
        Some(role_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "role_id": role_id,
            "permissions_count": body.permission_ids.len()
        })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// DELETE /api/admin/roles/{id}/permissions/{perm_id}
// ────────────────────────────────────────────────────────────────

pub async fn retirer_permission(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let (role_id, permission_id) = chemin.into_inner();

    let result = sqlx::query(
        "DELETE FROM iam.role_permission WHERE role_id = $1 AND permission_id = $2"
    )
    .bind(role_id)
    .bind(permission_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Assignation non trouvee".into()));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "iam",
        "role_permission",
        Some(role_id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// GET /api/admin/permissions — Liste de toutes les permissions
// ────────────────────────────────────────────────────────────────

pub async fn lister_permissions(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "voir");

    let permissions = sqlx::query_as::<_, PermissionListeResponse>(
        "SELECT id, nom, slug, description, type_ressource, action, created_at
         FROM iam.permission
         ORDER BY type_ressource, action"
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(permissions),
        error: None,
    }))
}
