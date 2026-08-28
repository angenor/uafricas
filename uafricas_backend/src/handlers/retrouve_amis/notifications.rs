//! Notifications (Retrouve Amis).

use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::retrouve_amis::*;
use crate::ApiResponse;

use super::commun::*;


/// GET /api/retrouve-amis/notifications
pub async fn lister_notifications(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    let lu = query.get("lu").and_then(|v| v.parse::<bool>().ok());
    let page: i64 = query.get("page").and_then(|v| v.parse().ok()).unwrap_or(1).max(1);
    let par_page: i64 = query.get("par_page").and_then(|v| v.parse().ok()).unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * par_page;

    let mut conditions = vec!["utilisateur_id = $1".to_string()];
    if let Some(l) = lu {
        conditions.push(format!("lu = {}", l));
    }
    let where_clause = conditions.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*) FROM retrouve_amis.notification_retrouve WHERE {}",
        where_clause
    );
    let non_lues_sql = "SELECT COUNT(*) FROM retrouve_amis.notification_retrouve WHERE utilisateur_id = $1 AND lu = FALSE";

    let list_sql = format!(
        "SELECT {colonnes}
         FROM retrouve_amis.notification_retrouve
         WHERE {ou}
         ORDER BY created_at DESC
         LIMIT {limite} OFFSET {decalage}",
        ou = where_clause,
        colonnes = NOTIFICATION_COLONNES,
        limite = par_page,
        decalage = offset,
    );

    let total: (i64,) = sqlx::query_as(&count_sql)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?;

    let non_lues: (i64,) = sqlx::query_as(non_lues_sql)
        .bind(utilisateur_id)
        .fetch_one(pool.get_ref())
        .await?;

    let rows: Vec<NotificationRetrouve> = sqlx::query_as(&list_sql)
        .bind(utilisateur_id)
        .fetch_all(pool.get_ref())
        .await?;

    let notifications: Vec<NotificationResponse> = rows
        .into_iter()
        .map(|n| NotificationResponse {
            id: n.id,
            type_notif: n.type_notif,
            correspondance_id: n.correspondance_id,
            lu: n.lu,
            created_at: n.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(NotificationListeResponse {
            notifications,
            total: total.0,
            non_lues: non_lues.0,
            page,
            par_page,
        }),
        error: None,
    }))
}

/// PATCH /api/retrouve-amis/notifications/{id}/lire
pub async fn marquer_lu(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;
    let notif_id = path.into_inner();

    let result = sqlx::query(
        "UPDATE retrouve_amis.notification_retrouve SET lu = TRUE
         WHERE id = $1 AND utilisateur_id = $2"
    )
    .bind(notif_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Notification introuvable".into()));
    }

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// PATCH /api/retrouve-amis/notifications/tout-lire
pub async fn tout_marquer_lu(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)?;

    let result = sqlx::query(
        "UPDATE retrouve_amis.notification_retrouve SET lu = TRUE
         WHERE utilisateur_id = $1 AND lu = FALSE"
    )
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({"mises_a_jour": result.rows_affected()})),
        error: None,
    }))
}


