use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::partenariat::*;
use crate::models::pagination::PaginationParams;
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ────────────────────────────────────────────────────────────────
// GET /api/admin/partenariats : Liste paginee
// ────────────────────────────────────────────────────────────────

pub async fn lister_partenariats(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminPartenariatQueryParams>,
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

    if let Some(ref type_p) = params.type_partenariat {
        let trimmed = type_p.trim();
        if !trimmed.is_empty() {
            conditions.push(format!("p.type_partenariat = ${}", bind_index));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref org_id) = params.organisation_id {
        conditions.push(format!("p.organisation_id = ${}::uuid", bind_index));
        bind_values.push(org_id.to_string());
        bind_index += 1;
    }

    if let Some(actif) = params.actif {
        conditions.push(format!("p.actif = ${}", bind_index));
        bind_values.push(actif.to_string());
        bind_index += 1;
    }

    let where_clause = if conditions.is_empty() {
        "TRUE".to_string()
    } else {
        conditions.join(" AND ")
    };

    let tri_colonne = pagination.colonne_tri(PARTENARIAT_TRI_COLONNES, "created_at");
    let tri_direction = pagination.direction_tri();
    let order_clause = format!("ORDER BY p.{} {}", tri_colonne, tri_direction);

    let count_query = format!(
        "SELECT COUNT(*) FROM iam.partenariat p WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_query = format!(
        "SELECT p.id, p.organisation_id, o.denomination AS organisation_denomination,
                p.type_partenariat, p.date_debut, p.date_fin, p.actif, p.created_at
         FROM iam.partenariat p
         JOIN iam.organisation o ON o.id = p.organisation_id
         WHERE {} {} LIMIT ${} OFFSET ${}",
        where_clause, order_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, AdminPartenariatListeResponse>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let partenariats = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(crate::models::pagination::PaginatedResponse::new(
            partenariats, total, page, par_page,
        )),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// GET /api/admin/partenariats/{id} : Detail
// ────────────────────────────────────────────────────────────────

pub async fn obtenir_partenariat(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "voir");

    let id = chemin.into_inner();

    let row = sqlx::query_as::<_, AdminPartenariatRow>(
        &format!(
            "SELECT {} FROM iam.partenariat p WHERE p.id = $1",
            ADMIN_PARTENARIAT_COLONNES
        )
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Partenariat non trouve".into()))?;

    let organisation_denomination: String = sqlx::query_scalar(
        "SELECT denomination FROM iam.organisation WHERE id = $1"
    )
    .bind(row.organisation_id)
    .fetch_one(pool.get_ref())
    .await?;

    let approuve_par_nom = if let Some(uid) = row.approuve_par {
        sqlx::query_scalar::<_, String>(
            "SELECT CONCAT(prenom, ' ', nom) FROM iam.utilisateur WHERE id = $1"
        )
        .bind(uid)
        .fetch_optional(pool.get_ref())
        .await?
    } else {
        None
    };

    let response = AdminPartenariatDetailResponse {
        id: row.id,
        organisation_id: row.organisation_id,
        organisation_denomination,
        type_partenariat: row.type_partenariat,
        description: row.description,
        date_debut: row.date_debut,
        date_fin: row.date_fin,
        actif: row.actif,
        approuve_par: row.approuve_par,
        approuve_par_nom,
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
// POST /api/admin/partenariats : Creation
// ────────────────────────────────────────────────────────────────

pub async fn creer_partenariat(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerPartenariatRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    // Verifier que l'organisation existe
    let org_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.organisation WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(body.organisation_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !org_existe {
        return Err(ApiErreur::NonTrouve("Organisation non trouvee".into()));
    }

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.partenariat (organisation_id, type_partenariat, description, date_debut, date_fin, approuve_par)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id"
    )
    .bind(body.organisation_id)
    .bind(body.type_partenariat.as_deref())
    .bind(body.description.as_deref())
    .bind(body.date_debut)
    .bind(body.date_fin)
    .bind(admin.id)
    .fetch_one(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le partenariat {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "iam",
        "partenariat",
        Some(id),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// PUT /api/admin/partenariats/{id} : Modification
// ────────────────────────────────────────────────────────────────

pub async fn modifier_partenariat(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierPartenariatRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let id = chemin.into_inner();

    let mut sets: Vec<String> = Vec::new();
    let mut bind_index = 1u32;
    let mut bind_strings: Vec<String> = Vec::new();

    if let Some(ref type_p) = body.type_partenariat {
        sets.push(format!("type_partenariat = ${}", bind_index));
        bind_strings.push(type_p.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref desc) = body.description {
        sets.push(format!("description = ${}", bind_index));
        bind_strings.push(desc.trim().to_string());
        bind_index += 1;
    }
    if let Some(ref date_d) = body.date_debut {
        sets.push(format!("date_debut = ${}", bind_index));
        bind_strings.push(date_d.to_string());
        bind_index += 1;
    }
    if let Some(ref date_f) = body.date_fin {
        sets.push(format!("date_fin = ${}", bind_index));
        bind_strings.push(date_f.to_string());
        bind_index += 1;
    }
    if let Some(actif) = body.actif {
        sets.push(format!("actif = ${}", bind_index));
        bind_strings.push(actif.to_string());
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());

    let query = format!(
        "UPDATE iam.partenariat SET {} WHERE id = ${}",
        sets.join(", "),
        bind_index
    );

    let mut q = sqlx::query(&query);
    for s in &bind_strings {
        q = q.bind(s.as_str());
    }
    q = q.bind(id);

    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Partenariat non trouve".into()));
    }

    log::info!("Admin {} a modifie le partenariat {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "iam",
        "partenariat",
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
// DELETE /api/admin/partenariats/{id}, Suppression reelle
// ────────────────────────────────────────────────────────────────

pub async fn supprimer_partenariat(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "supprimer");

    let id = chemin.into_inner();

    let result = sqlx::query("DELETE FROM iam.partenariat WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Partenariat non trouve".into()));
    }

    log::info!("Admin {} a supprime le partenariat {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "iam",
        "partenariat",
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
