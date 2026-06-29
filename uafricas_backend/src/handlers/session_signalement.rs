//! Signalement communautaire d'une salle afrolang depuis une session live.
//!
//! Tout membre connecté présent dans une session peut signaler la salle hôte
//! (abus, contenu inapproprié, propos haineux…). Les signalements s'accumulent
//! sur la salle PERSISTANTE (pas la session éphémère), à travers les sessions
//! successives. Au-delà de 10 signalements distincts, la salle est
//! automatiquement SUSPENDUE (retirée du listing public + fermée aux nouvelles
//! jointures) jusqu'à réactivation par un admin. Jamais de désuspension auto.
//!
//! Pattern calqué sur `contribution_signalement` (insert idempotent ON CONFLICT
//! + recompte + bascule de suspension + audit).

use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::services::audit;

/// Au-delà de ce nombre de signalements distincts, la salle est suspendue.
pub const SEUIL_SIGNALEMENTS_SUSPENSION_SALLE: i64 = 10;

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SignalerSessionRequest {
    pub motif: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignalementSessionEtat {
    pub nombre_signalements: i32,
    pub suspendu: bool,
    pub deja_signale: bool,
}

fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Résout la salle publique hôte d'une session (directe ou via la salle privée).
async fn resoudre_salle_id(pool: &PgPool, session_id: Uuid) -> Result<Uuid, ApiErreur> {
    let info: Option<(Option<Uuid>, Option<Uuid>)> = sqlx::query_as(
        "SELECT salle_id, salle_privee_id FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;

    let (salle_id_directe, salle_privee_id) =
        info.ok_or_else(|| ApiErreur::NonTrouve("Session introuvable".to_string()))?;

    if let Some(s) = salle_id_directe {
        return Ok(s);
    }
    if let Some(sp_id) = salle_privee_id {
        return sqlx::query_scalar::<_, Uuid>(
            "SELECT salle_id FROM afrolang.salle_privee WHERE id = $1",
        )
        .bind(sp_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Salle parente introuvable".to_string()));
    }
    Err(ApiErreur::Validation(
        "Session sans rattachement à une salle".to_string(),
    ))
}

/// POST /api/afrolang/sessions/{id}/signalement
pub async fn signaler_session(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<SignalerSessionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = extraire_utilisateur_id(&req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))?;
    let session_id = chemin.into_inner();

    let salle_id = resoudre_salle_id(pool.get_ref(), session_id).await?;

    // La salle doit exister et ne pas être supprimée.
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Salle introuvable".to_string()));
    }

    // Insertion idempotente : un seul signalement par membre et par salle.
    let resultat = sqlx::query(
        "INSERT INTO afrolang.signalement_salle
            (salle_id, session_id, signale_par, motif, description)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (salle_id, signale_par) DO NOTHING",
    )
    .bind(salle_id)
    .bind(session_id)
    .bind(utilisateur_id)
    .bind(body.motif.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(
        body.description
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty()),
    )
    .execute(pool.get_ref())
    .await?;
    let nouveau_signalement = resultat.rows_affected() > 0;

    // Recompter les signalements distincts sur la salle.
    let nombre: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.signalement_salle WHERE salle_id = $1",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;

    let doit_suspendre = nombre > SEUIL_SIGNALEMENTS_SUSPENSION_SALLE;

    // Met à jour le compteur ; suspend uniquement (jamais de réactivation auto).
    let suspendu: bool = sqlx::query_scalar(
        "UPDATE afrolang.salle
         SET nombre_signalements = $2,
             suspendu = (suspendu OR $3),
             updated_at = NOW()
         WHERE id = $1
         RETURNING suspendu",
    )
    .bind(salle_id)
    .bind(nombre as i32)
    .bind(doit_suspendre)
    .fetch_one(pool.get_ref())
    .await?;

    if nouveau_signalement {
        let ip = audit::extraire_ip(&req);
        let ua = audit::extraire_user_agent(&req);
        let action = if doit_suspendre && suspendu {
            "SIGNALEMENT_SUSPENSION"
        } else {
            "SIGNALEMENT"
        };
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            action,
            "afrolang",
            "salle",
            Some(salle_id),
            None,
            Some(serde_json::json!({ "session_id": session_id })),
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
    }

    if doit_suspendre && suspendu {
        log::warn!(
            "Salle afrolang {salle_id} suspendue automatiquement ({nombre} signalements)"
        );
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SignalementSessionEtat {
            nombre_signalements: nombre as i32,
            suspendu,
            deja_signale: !nouveau_signalement,
        }),
        error: None,
    }))
}
