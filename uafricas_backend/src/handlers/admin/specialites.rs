use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::specialite::{
    AdminSpecialiteDetailResponse, AdminSpecialiteListeResponse, AdminSpecialiteQueryParams,
    CreerSpecialiteRequest, ModifierSpecialiteRequest, ADMIN_SPECIALITE_LISTE_COLONNES,
    SPECIALITE_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::verifier_permission;
use crate::ApiResponse;

/// GET /api/admin/specialites
pub async fn lister_specialites(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminSpecialiteQueryParams>,
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
            conditions.push(format!("(LOWER(s.nom) LIKE ${bi} OR LOWER(s.slug) LIKE ${bi})", bi = bind_index));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(SPECIALITE_TRI_COLONNES, "nom");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    let count_sql = format!("SELECT COUNT(*) FROM iam.specialite_bibliotheque s WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values { count_q = count_q.bind(v); }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    let select_sql = format!(
        "SELECT {} FROM iam.specialite_bibliotheque s WHERE {} ORDER BY s.{} {} LIMIT {} OFFSET {}",
        ADMIN_SPECIALITE_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminSpecialiteListeResponse>(&select_sql);
    for v in &bind_values { select_q = select_q.bind(v); }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/specialites/:id
pub async fn obtenir_specialite(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "voir");
    let id = path.into_inner();

    let row = sqlx::query_as::<_, AdminSpecialiteListeResponse>(
        "SELECT s.id, s.nom, s.slug FROM iam.specialite_bibliotheque s WHERE s.id = $1"
    )
    .bind(id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Specialite non trouvee".into()))?;

    let nombre_utilisateurs: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM iam.utilisateur_specialite WHERE specialite_id = $1"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AdminSpecialiteDetailResponse {
            id: row.id,
            nom: row.nom,
            slug: row.slug,
            nombre_utilisateurs,
        }),
        error: None,
    }))
}

/// POST /api/admin/specialites
pub async fn creer_specialite(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    body: web::Json<CreerSpecialiteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");

    let nom = body.nom.trim();
    if nom.is_empty() {
        return Err(ApiErreur::Validation("Le nom de la specialite est requis".into()));
    }

    let slug = nom.to_lowercase().replace(' ', "-").replace(['\'', '"', '.', ','], "");
    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO iam.specialite_bibliotheque (id, nom, slug) VALUES ($1, $2, $3)")
        .bind(id)
        .bind(nom)
        .bind(&slug)
        .execute(pool.get_ref())
        .await?;

    log::info!("Admin {} a cree la specialite {} ({})", admin.id, nom, id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/specialites/:id
pub async fn modifier_specialite(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierSpecialiteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "modifier");
    let id = path.into_inner();

    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM iam.specialite_bibliotheque WHERE id = $1)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Specialite non trouvee".into()));
    }

    if let Some(ref nom) = body.nom {
        let n = nom.trim();
        if n.is_empty() {
            return Err(ApiErreur::Validation("Le nom ne peut pas etre vide".into()));
        }
        let slug = n.to_lowercase().replace(' ', "-").replace(['\'', '"', '.', ','], "");
        sqlx::query("UPDATE iam.specialite_bibliotheque SET nom = $1, slug = $2 WHERE id = $3")
            .bind(n)
            .bind(&slug)
            .bind(id)
            .execute(pool.get_ref())
            .await?;
    }

    log::info!("Admin {} a modifie la specialite {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/specialites/:id
pub async fn supprimer_specialite(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "referentiel", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query("DELETE FROM iam.specialite_bibliotheque WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Specialite non trouvee".into()));
    }

    log::info!("Admin {} a supprime la specialite {}", admin.id, id);

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}
