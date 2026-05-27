//! Liste publique des pays (Retrouve Amis).

use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::errors::ApiErreur;
use crate::models::retrouve_amis::*;
use crate::ApiResponse;



/// GET /api/retrouve-amis/pays
/// Liste des pays actifs (id, nom) sans authentification
pub async fn lister_pays(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let pays: Vec<PaysInfo> = sqlx::query_as(
        "SELECT id, nom FROM shared.pays WHERE actif = TRUE ORDER BY nom ASC",
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(pays),
        error: None,
    }))
}

