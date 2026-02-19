use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::services::audit;
use crate::models::admin::media::{
    AdminMediaDetailRow, AdminMediaDetailResponse, AdminMediaListeResponse, AdminMediaQueryParams,
    ADMIN_MEDIA_DETAIL_COLONNES, ADMIN_MEDIA_LISTE_COLONNES, MEDIA_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/medias
pub async fn lister_medias(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminMediaQueryParams>,
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
            conditions.push(format!("LOWER(m.nom_original) LIKE ${}", bind_index));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref type_mime) = params.type_mime {
        let t = type_mime.trim();
        if !t.is_empty() {
            conditions.push(format!("m.type_mime LIKE ${}", bind_index));
            bind_values.push(format!("{}%", t));
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(MEDIA_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!("SELECT COUNT(*) FROM shared.media m WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM shared.media m WHERE {} ORDER BY m.{} {} LIMIT {} OFFSET {}",
        ADMIN_MEDIA_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminMediaListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/medias/:id
pub async fn obtenir_media(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM shared.media m WHERE m.id = $1",
        ADMIN_MEDIA_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminMediaDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Media non trouve".into()))?;

    let uploaded_by_nom: Option<String> = if let Some(uid) = row.uploaded_by {
        sqlx::query_scalar(
            "SELECT CONCAT(prenom, ' ', nom) FROM iam.utilisateur WHERE id = $1"
        )
        .bind(uid)
        .fetch_optional(pool.get_ref())
        .await?
    } else {
        None
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminMediaDetailResponse {
            id: row.id,
            nom_original: row.nom_original,
            chemin_stockage: row.chemin_stockage,
            url_publique: row.url_publique,
            type_mime: row.type_mime,
            taille_octets: row.taille_octets,
            largeur: row.largeur,
            hauteur: row.hauteur,
            duree_secondes: row.duree_secondes,
            uploaded_by: row.uploaded_by,
            uploaded_by_nom,
            created_at: row.created_at,
        }),
        error: None,
    }))
}

/// DELETE /api/admin/medias/:id
pub async fn supprimer_media(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "supprimer");
    let id = path.into_inner();

    let chemin: Option<String> = sqlx::query_scalar(
        "SELECT chemin_stockage FROM shared.media WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?;

    if chemin.is_none() {
        return Err(ApiErreur::NonTrouve("Media non trouve".into()));
    }

    sqlx::query("DELETE FROM shared.media WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    if let Some(path_str) = chemin {
        let _ = std::fs::remove_file(&path_str);
    }

    log::info!("Admin {} a supprime le media {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "shared",
        "media",
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
