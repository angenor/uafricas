use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::services::audit;
use crate::models::admin::tag::{
    AdminTagDetailResponse, AdminTagListeResponse, AdminTagQueryParams,
    CreerTagRequest, ModifierTagRequest, ADMIN_TAG_LISTE_COLONNES, TAG_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/tags
pub async fn lister_tags(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminTagQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["1=1".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!("(LOWER(t.nom) LIKE ${bi} OR LOWER(t.slug) LIKE ${bi})", bi = bind_index));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(TAG_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!("SELECT COUNT(*) FROM shared.tag t WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM shared.tag t WHERE {} ORDER BY t.{} {} LIMIT {} OFFSET {}",
        ADMIN_TAG_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminTagListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/tags/:id
pub async fn obtenir_tag(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");
    let id = path.into_inner();

    let row = sqlx::query_as::<_, AdminTagListeResponse>(
        "SELECT t.id, t.nom, t.slug, t.created_at FROM shared.tag t WHERE t.id = $1"
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Tag non trouve".into()))?;

    let nombre_utilisations: i64 = 0;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminTagDetailResponse {
            id: row.id,
            nom: row.nom,
            slug: row.slug,
            created_at: row.created_at,
            nombre_utilisations,
        }),
        error: None,
    }))
}

/// POST /api/admin/tags
pub async fn creer_tag(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerTagRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom du tag est requis".into()));
    }

    let slug = nom.to_lowercase().replace(' ', "-").replace(['\'', '"', '.', ','], "");
    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO shared.tag (id, nom, slug) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(nom)
        .bind(&slug)
        .execute(pool.get_ref())
        .await?;

    log::info!("Admin {} a cree le tag {} ({})", admin.id, nom, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "shared",
        "tag",
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

/// PUT /api/admin/tags/:id
pub async fn modifier_tag(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierTagRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM shared.tag WHERE id = $1)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Tag non trouve".into()));
    }

    if let Some(ref nom) = body.nom {
        let n = nom.trim();
        if n.is_empty() {
            return Err(ApiErreur::Validation("Le nom du tag ne peut pas etre vide".into()));
        }
        let slug = n.to_lowercase().replace(' ', "-").replace(['\'', '"', '.', ','], "");
        sqlx::query("UPDATE shared.tag SET nom = $1, slug = $2 WHERE id = $3")
            .bind(n)
            .bind(&slug)
            .bind(id)
            .execute(pool.get_ref())
            .await?;
    }

    log::info!("Admin {} a modifie le tag {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "shared",
        "tag",
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

/// DELETE /api/admin/tags/:id
pub async fn supprimer_tag(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query("DELETE FROM shared.tag WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Tag non trouve".into()));
    }

    log::info!("Admin {} a supprime le tag {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "shared",
        "tag",
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
