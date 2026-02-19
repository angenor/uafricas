use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::domaine::{
    AdminDomaineDetailResponse, AdminDomaineListeResponse, AdminDomaineQueryParams,
    CreerDomaineRequest, ModifierDomaineRequest, ADMIN_DOMAINE_DETAIL_COLONNES,
    ADMIN_DOMAINE_LISTE_COLONNES, DOMAINE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/domaines
pub async fn lister_domaines(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminDomaineQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["d.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!("(LOWER(d.nom) LIKE ${bi} OR LOWER(d.slug) LIKE ${bi})", bi = bind_index));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(DOMAINE_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!("SELECT COUNT(*) FROM shared.domaine_secteur d WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM shared.domaine_secteur d WHERE {} ORDER BY d.{} {} LIMIT {} OFFSET {}",
        ADMIN_DOMAINE_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminDomaineListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/domaines/:id
pub async fn obtenir_domaine(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM shared.domaine_secteur d WHERE d.id = $1 AND d.deleted_at IS NULL",
        ADMIN_DOMAINE_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminDomaineDetailResponse>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Domaine non trouve".into()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row),
        error: None,
    }))
}

/// POST /api/admin/domaines
pub async fn creer_domaine(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CreerDomaineRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom du domaine est requis".into()));
    }

    let slug = nom.to_lowercase().replace(' ', "-").replace(['\'', '"', '.', ','], "");
    let id = Uuid::new_v4();

    sqlx::query(
        "INSERT INTO shared.domaine_secteur (id, nom, slug, description, icone) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id)
    .bind(nom)
    .bind(&slug)
    .bind(body.description.as_deref().map(|s| s.trim()))
    .bind(body.icone.as_deref().map(|s| s.trim()))
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a cree le domaine {} ({})", admin.id, nom, id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/domaines/:id
pub async fn modifier_domaine(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierDomaineRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.domaine_secteur WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Domaine non trouve".into()));
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
    ajouter_champ_str!(body.description, "description");
    ajouter_champ_str!(body.icone, "icone");

    if let Some(ref nom) = body.nom {
        let slug = nom.trim().to_lowercase().replace(' ', "-").replace(['\'', '"', '.', ','], "");
        sets.push(format!("slug = ${}", bind_index));
        bind_strings.push(slug);
        bind_index += 1;
    }

    if let Some(actif) = body.actif {
        sets.push(format!("actif = {}", actif));
    }

    if sets.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ a modifier".into()));
    }

    sets.push("updated_at = NOW()".to_string());
    let sql = format!(
        "UPDATE shared.domaine_secteur SET {} WHERE id = ${} AND deleted_at IS NULL",
        sets.join(", "), bind_index
    );

    let mut q = sqlx::query(&sql);
    for v in &bind_strings { q = q.bind(v); }
    q = q.bind(id);
    q.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifie le domaine {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/domaines/:id
pub async fn supprimer_domaine(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE shared.domaine_secteur SET deleted_at = NOW(), actif = false, updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Domaine non trouve".into()));
    }

    log::info!("Admin {} a supprime le domaine {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}
