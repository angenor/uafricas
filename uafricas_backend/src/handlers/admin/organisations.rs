use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::organisation::*;
use crate::models::pagination::PaginationParams;
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ────────────────────────────────────────────────────────────────
// GET /api/admin/organisations : Liste paginee
// ────────────────────────────────────────────────────────────────

pub async fn lister_organisations(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminOrganisationQueryParams>,
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

    let mut conditions: Vec<String> = vec!["o.deleted_at IS NULL".to_string()];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref etat) = params.etat {
        let trimmed = etat.trim();
        if !trimmed.is_empty() {
            conditions.push(format!("o.etat = ${}", bind_index));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref type_org) = params.type_organisation {
        let trimmed = type_org.trim();
        if !trimmed.is_empty() {
            conditions.push(format!("o.type_organisation = ${}", bind_index));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref pays) = params.pays {
        let trimmed = pays.trim();
        if !trimmed.is_empty() {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM shared.pays pp WHERE pp.id = o.pays_id AND LOWER(pp.nom) = LOWER(${}))",
                bind_index
            ));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref recherche) = params.recherche {
        let trimmed = recherche.trim();
        if !trimmed.is_empty() {
            let terme = format!("%{}%", trimmed.to_lowercase());
            conditions.push(format!(
                "(LOWER(o.denomination) LIKE ${idx} OR LOWER(o.email) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    let tri_colonne = pagination.colonne_tri(ORGANISATION_TRI_COLONNES, "created_at");
    let tri_direction = pagination.direction_tri();
    let order_clause = format!("ORDER BY o.{} {}", tri_colonne, tri_direction);

    // Count
    let count_query = format!(
        "SELECT COUNT(*) FROM iam.organisation o WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Select
    let select_query = format!(
        "SELECT o.id, o.denomination, o.type_organisation,
                p.nom AS pays_nom, o.etat, o.ville,
                (SELECT COUNT(*) FROM iam.utilisateur u WHERE u.organisation_id = o.id AND u.deleted_at IS NULL) AS nombre_membres,
                o.created_at
         FROM iam.organisation o
         LEFT JOIN shared.pays p ON p.id = o.pays_id
         WHERE {} {} LIMIT ${} OFFSET ${}",
        where_clause, order_clause, bind_index, bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, AdminOrganisationListeResponse>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let organisations = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(crate::models::pagination::PaginatedResponse::new(
            organisations, total, page, par_page,
        )),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// GET /api/admin/organisations/{id} : Detail
// ────────────────────────────────────────────────────────────────

pub async fn obtenir_organisation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "voir");

    let id = chemin.into_inner();

    let row = sqlx::query_as::<_, AdminOrganisationRow>(
        &format!(
            "SELECT {} FROM iam.organisation o WHERE o.id = $1 AND o.deleted_at IS NULL",
            ADMIN_ORGANISATION_COLONNES
        )
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Organisation non trouvee".into()))?;

    // Pays nom
    let pays_nom = if let Some(pid) = row.pays_id {
        sqlx::query_scalar::<_, String>("SELECT nom FROM shared.pays WHERE id = $1")
            .bind(pid)
            .fetch_optional(pool.get_ref())
            .await?
    } else {
        None
    };

    // Createur nom
    let cree_par_nom = if let Some(uid) = row.cree_par {
        sqlx::query_scalar::<_, String>(
            "SELECT CONCAT(prenom, ' ', nom) FROM iam.utilisateur WHERE id = $1"
        )
        .bind(uid)
        .fetch_optional(pool.get_ref())
        .await?
    } else {
        None
    };

    // Nombre membres
    let nombre_membres: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM iam.utilisateur WHERE organisation_id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    let response = AdminOrganisationDetailResponse {
        id: row.id,
        denomination: row.denomination,
        slug: row.slug,
        type_organisation: row.type_organisation,
        pays_id: row.pays_id,
        pays_nom,
        email: row.email,
        telephone: row.telephone,
        adresse: row.adresse,
        ville: row.ville,
        site_web: row.site_web,
        logo_url: row.logo_url,
        description: row.description,
        document_legal_url: row.document_legal_url,
        numero_registre: row.numero_registre,
        etat: row.etat,
        cree_par: row.cree_par,
        cree_par_nom,
        nombre_membres,
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
// POST /api/admin/organisations : Creation
// ────────────────────────────────────────────────────────────────

pub async fn creer_organisation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerOrganisationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    if body.denomination.trim().is_empty() {
        return Err(ApiErreur::Validation("Denomination requise".into()));
    }

    let slug = body.denomination.trim().to_lowercase()
        .replace(' ', "-")
        .replace(['\'', '"', '.', ','], "");

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO iam.organisation (denomination, slug, type_organisation, pays_id, email, telephone, adresse, ville, site_web, description, numero_registre, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'en_attente', $12)
         RETURNING id"
    )
    .bind(body.denomination.trim())
    .bind(&slug)
    .bind(body.type_organisation.as_deref())
    .bind(body.pays_id)
    .bind(body.email.as_deref())
    .bind(body.telephone.as_deref())
    .bind(body.adresse.as_deref())
    .bind(body.ville.as_deref())
    .bind(body.site_web.as_deref())
    .bind(body.description.as_deref())
    .bind(body.numero_registre.as_deref())
    .bind(admin.id)
    .fetch_one(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree l'organisation {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "iam",
        "organisation",
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
// PUT /api/admin/organisations/{id} : Modification
// ────────────────────────────────────────────────────────────────

pub async fn modifier_organisation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ModifierOrganisationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "modifier");

    let id = chemin.into_inner();

    let mut sets: Vec<String> = Vec::new();
    let mut bind_index = 1u32;
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_uuids: Vec<Option<Uuid>> = Vec::new();

    macro_rules! ajouter_champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    ajouter_champ_str!(body.denomination, "denomination");
    ajouter_champ_str!(body.type_organisation, "type_organisation");
    ajouter_champ_str!(body.email, "email");
    ajouter_champ_str!(body.telephone, "telephone");
    ajouter_champ_str!(body.adresse, "adresse");
    ajouter_champ_str!(body.ville, "ville");
    ajouter_champ_str!(body.site_web, "site_web");
    ajouter_champ_str!(body.description, "description");
    ajouter_champ_str!(body.numero_registre, "numero_registre");
    ajouter_champ_str!(body.etat, "etat");

    // UUID param
    if body.pays_id.is_some() {
        sets.push(format!("pays_id = ${}", bind_index));
        bind_uuids.push(body.pays_id);
        bind_index += 1;
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());

    let query = format!(
        "UPDATE iam.organisation SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "),
        bind_index
    );

    let mut q = sqlx::query(&query);
    // Bind strings first, then UUIDs, matching the bind_index order
    for s in &bind_strings {
        q = q.bind(s.as_str());
    }
    for u in &bind_uuids {
        q = q.bind(*u);
    }
    q = q.bind(id);

    let result = q.execute(pool.get_ref()).await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Organisation non trouvee".into()));
    }

    log::info!("Admin {} a modifie l'organisation {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "iam",
        "organisation",
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
// DELETE /api/admin/organisations/{id}, Soft delete
// ────────────────────────────────────────────────────────────────

pub async fn supprimer_organisation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "utilisateur", "supprimer");

    let id = chemin.into_inner();

    let result = sqlx::query(
        "UPDATE iam.organisation SET deleted_at = NOW(), etat = 'supprime', updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Organisation non trouvee".into()));
    }

    log::info!("Admin {} a supprime l'organisation {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "iam",
        "organisation",
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
