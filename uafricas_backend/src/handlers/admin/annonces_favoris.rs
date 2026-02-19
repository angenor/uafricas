use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::annonce_favori::{
    AdminFavoriListeResponse, AdminFavoriQueryParams, AdminFavoriStatsRow,
    ADMIN_FAVORI_LISTE_COLONNES, FAVORI_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/annonces-favoris
pub async fn lister_favoris(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminFavoriQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["a.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(a.titre) LIKE ${bi} OR LOWER(u.nom) LIKE ${bi} OR LOWER(u.prenom) LIKE ${bi} OR LOWER(u.email) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref annonce_id) = params.annonce_id {
        let aid = annonce_id.trim();
        if !aid.is_empty() {
            if let Ok(_) = uuid::Uuid::parse_str(aid) {
                conditions.push(format!("af.annonce_id::text = ${}", bind_index));
                bind_values.push(aid.to_string());
                bind_index += 1;
            }
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(FAVORI_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!(
        "SELECT COUNT(*) FROM marketplace.annonce_favori af
         JOIN iam.utilisateur u ON u.id = af.utilisateur_id
         JOIN marketplace.annonce a ON a.id = af.annonce_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM marketplace.annonce_favori af
         JOIN iam.utilisateur u ON u.id = af.utilisateur_id
         JOIN marketplace.annonce a ON a.id = af.annonce_id
         WHERE {} ORDER BY af.{} {} LIMIT {} OFFSET {}",
        ADMIN_FAVORI_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminFavoriListeResponse>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/annonces-favoris/stats
pub async fn stats_favoris(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "marketplace", "voir");

    let top_annonces = sqlx::query_as::<_, AdminFavoriStatsRow>(
        "SELECT af.annonce_id, a.titre AS annonce_titre, COUNT(*) AS nombre_favoris
         FROM marketplace.annonce_favori af
         JOIN marketplace.annonce a ON a.id = af.annonce_id
         WHERE a.deleted_at IS NULL
         GROUP BY af.annonce_id, a.titre
         ORDER BY nombre_favoris DESC
         LIMIT 20",
    )
    .fetch_all(pool.get_ref())
    .await?;

    let total_favoris: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM marketplace.annonce_favori af
         JOIN marketplace.annonce a ON a.id = af.annonce_id
         WHERE a.deleted_at IS NULL",
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "total_favoris": total_favoris,
            "top_annonces": top_annonces,
        })),
        error: None,
    }))
}
