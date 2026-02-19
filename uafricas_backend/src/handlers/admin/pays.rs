use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::pays::{
    AdminPaysDetailRow, AdminPaysListeResponse, AdminPaysQueryParams,
    CreerPaysRequest, ModifierPaysRequest, ADMIN_PAYS_DETAIL_COLONNES,
    ADMIN_PAYS_LISTE_COLONNES, PAYS_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/pays
pub async fn lister_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminPaysQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["p.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(p.nom) LIKE ${bi} OR LOWER(p.code_iso2) LIKE ${bi} OR LOWER(p.capitale) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref continent) = params.continent {
        let c = continent.trim();
        if !c.is_empty() {
            conditions.push(format!("LOWER(p.continent) = ${}", bind_index));
            bind_values.push(c.to_lowercase());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(PAYS_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!("SELECT COUNT(*) FROM shared.pays p WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM shared.pays p WHERE {} ORDER BY p.{} {} LIMIT {} OFFSET {}",
        ADMIN_PAYS_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminPaysListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/pays/:id
pub async fn obtenir_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM shared.pays p WHERE p.id = $1 AND p.deleted_at IS NULL",
        ADMIN_PAYS_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminPaysDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Pays non trouve".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}

/// POST /api/admin/pays
pub async fn creer_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CreerPaysRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom du pays est requis".into()));
    }

    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO shared.pays (id, nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, longitude, latitude)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(id)
    .bind(nom)
    .bind(body.code_iso2.as_deref().map(|s| s.trim()))
    .bind(body.code_iso3.as_deref().map(|s| s.trim()))
    .bind(body.indicatif_tel.as_deref().map(|s| s.trim()))
    .bind(body.capitale.as_deref().map(|s| s.trim()))
    .bind(body.continent.as_deref().map(|s| s.trim()))
    .bind(body.longitude)
    .bind(body.latitude)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le pays {} ({})", admin.id, nom, id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/pays/:id
pub async fn modifier_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierPaysRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.pays WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Pays non trouve".into()));
    }

    let mut sets = Vec::new();
    let mut bind_strings: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    macro_rules! ajouter_champ_str {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ${}", $col, bind_index));
                bind_strings.push(val.trim().to_string());
                bind_index += 1;
            }
        };
    }

    ajouter_champ_str!(body.nom, "nom");
    ajouter_champ_str!(body.code_iso2, "code_iso2");
    ajouter_champ_str!(body.code_iso3, "code_iso3");
    ajouter_champ_str!(body.indicatif_tel, "indicatif_tel");
    ajouter_champ_str!(body.capitale, "capitale");
    ajouter_champ_str!(body.continent, "continent");

    if let Some(lon) = body.longitude {
        sets.push(format!("longitude = {}", lon));
    }
    if let Some(lat) = body.latitude {
        sets.push(format!("latitude = {}", lat));
    }
    if let Some(actif) = body.actif {
        sets.push(format!("actif = {}", actif));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE shared.pays SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie le pays {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/pays/:id
pub async fn supprimer_pays(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE shared.pays SET deleted_at = NOW(), actif = false, updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Pays non trouve".into()));
    }

    log::info!("Admin {} a supprime le pays {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}
