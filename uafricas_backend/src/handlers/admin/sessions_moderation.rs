//! Endpoints admin pour la fermeture/réactivation administrative d'une salle
//! Afrolang et l'historique de modération
//! (feature 001-ressources-fermeture-session, contracts/admin-moderation.md).

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::LivekitConfig;
use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::sessions_moderation::{
    EvenementModerationResponse, FermetureAdminRequest, ReactivationRequest,
    TypeEvenementModeration,
};
use crate::models::notification;
use crate::models::pagination::PaginatedResponse;
use crate::services::{audit, livekit_moderation};
use crate::verifier_permission;
use crate::ApiResponse;

const MOTIF_FERMETURE_MIN: usize = 10;
const MOTIF_FERMETURE_MAX: usize = 1000;
const COMMENTAIRE_REACTIVATION_MAX: usize = 1000;
const MOTIF_PUBLIC_PARTICIPANT: &str =
    "Cette session a été fermée par l'administration.";

// ──────────────────────────────────────────────────────────────────────────
// POST /api/admin/afrolang/sessions/{session_id}/fermer-admin
// ──────────────────────────────────────────────────────────────────────────

/// Ferme une session live pour cause d'abus et désactive la salle hôte.
pub async fn fermer_session_admin(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    livekit_config: web::Data<LivekitConfig>,
    path: web::Path<Uuid>,
    body: web::Json<FermetureAdminRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "moderer");

    let session_id = path.into_inner();
    let motif = body.motif.trim().to_string();
    if motif.len() < MOTIF_FERMETURE_MIN || motif.len() > MOTIF_FERMETURE_MAX {
        return Err(ApiErreur::Validation(format!(
            "Le motif doit contenir entre {} et {} caractères",
            MOTIF_FERMETURE_MIN, MOTIF_FERMETURE_MAX
        )));
    }

    // Récupérer la session + résoudre la salle publique cible.
    let session_info: Option<(Option<Uuid>, Option<Uuid>, String)> = sqlx::query_as(
        "SELECT salle_id, salle_privee_id, etat::TEXT
         FROM afrolang.session WHERE id = $1",
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (salle_id_directe, salle_privee_id, etat) = session_info
        .ok_or_else(|| ApiErreur::NonTrouve("Session introuvable".into()))?;

    let salle_id: Uuid = if let Some(s) = salle_id_directe {
        s
    } else if let Some(sp_id) = salle_privee_id {
        sqlx::query_scalar::<_, Uuid>(
            "SELECT salle_id FROM afrolang.salle_privee WHERE id = $1",
        )
        .bind(sp_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Salle privée parente introuvable".into()))?
    } else {
        return Err(ApiErreur::Validation(
            "Session sans rattachement à une salle".into(),
        ));
    };

    // Idempotence FR-021 : 409 si déjà désactivée.
    let salle_etat: Option<(Option<DateTime<Utc>>, Option<DateTime<Utc>>)> = sqlx::query_as(
        "SELECT desactivee_admin_at, deleted_at FROM afrolang.salle WHERE id = $1",
    )
    .bind(salle_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (desactivee_admin_at, deleted_at) =
        salle_etat.ok_or_else(|| ApiErreur::NonTrouve("Salle introuvable".into()))?;
    if deleted_at.is_some() {
        return Err(ApiErreur::NonTrouve("Salle introuvable".into()));
    }
    if desactivee_admin_at.is_some() {
        return Err(ApiErreur::Conflit("Salle déjà désactivée".into()));
    }

    // Transaction : session → terminée, salle → désactivée, évènement append-only.
    let mut tx = pool.begin().await?;

    if etat == "en_cours" || etat == "planifiee" {
        sqlx::query(
            "UPDATE afrolang.session
             SET etat = 'terminee', termine_at = NOW(), updated_at = NOW()
             WHERE id = $1",
        )
        .bind(session_id)
        .execute(&mut *tx)
        .await?;
    }

    sqlx::query(
        "UPDATE afrolang.salle
         SET desactivee_admin_at = NOW(), desactivee_par = $2,
             motif_desactivation = $3, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(salle_id)
    .bind(admin.id)
    .bind(&motif)
    .execute(&mut *tx)
    .await?;

    let evt_id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.evenement_moderation_salle
            (salle_id, session_concernee_id, type_action, admin_id, motif)
         VALUES ($1, $2, 'fermeture_admin', $3, $4)
         RETURNING id",
    )
    .bind(salle_id)
    .bind(session_id)
    .bind(admin.id)
    .bind(&motif)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    // Hors transaction : LiveKit + notifications.
    let room_name = format!("afrolang-{}", session_id);
    let _ = livekit_moderation::fermer_session_admin(
        livekit_config.get_ref(),
        &room_name,
        MOTIF_PUBLIC_PARTICIPANT,
    )
    .await;

    // Notifier les participants actifs (sans motif).
    let participants: Vec<Uuid> = sqlx::query_scalar(
        "SELECT utilisateur_id FROM afrolang.session_participant
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(session_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let lien_participant = format!("/afrolang/session/{}", session_id);
    for p_id in &participants {
        notification::creer_notification(
            pool.get_ref(),
            *p_id,
            notification::afrolang::SESSION_FERMEE_ADMIN,
            "La session a été fermée par l'administration.",
            Some(&lien_participant),
        )
        .await;
    }

    // Notifier les admins de salle (avec motif) ou le créateur de salle privée.
    let admins_salle: Vec<Uuid> = sqlx::query_scalar(
        "SELECT utilisateur_id FROM afrolang.salle_administrateur
         WHERE salle_id = $1 AND actif = TRUE",
    )
    .bind(salle_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let mut destinataires_admin: Vec<Uuid> = admins_salle.clone();
    if let Some(sp_id) = salle_privee_id {
        if let Ok(Some(createur)) = sqlx::query_scalar::<_, Uuid>(
            "SELECT cree_par FROM afrolang.salle_privee WHERE id = $1",
        )
        .bind(sp_id)
        .fetch_optional(pool.get_ref())
        .await
        {
            destinataires_admin.push(createur);
        }
    }
    destinataires_admin.sort();
    destinataires_admin.dedup();

    let lien_salle = format!("/afrolang/{}", salle_id);
    for adm_id in &destinataires_admin {
        notification::creer_notification(
            pool.get_ref(),
            *adm_id,
            notification::afrolang::SALLE_DESACTIVEE_ADMIN,
            &format!(
                "La salle a été désactivée par l'administration. Motif : {}",
                motif
            ),
            Some(&lien_salle),
        )
        .await;
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);

    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "afrolang",
        "salle",
        Some(salle_id),
        Some(serde_json::json!({ "desactivee_admin_at": null })),
        Some(serde_json::json!({
            "desactivee_admin_at": "NOW()",
            "motif_desactivation": motif,
            "session_concernee_id": session_id,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "afrolang",
        "evenement_moderation_salle",
        Some(evt_id),
        None,
        Some(serde_json::json!({
            "type_action": "fermeture_admin",
            "salle_id": salle_id,
            "session_concernee_id": session_id,
            "motif": motif,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "salle_id": salle_id,
            "session_id": session_id,
            "fermeture": {
                "admin_id": admin.id,
                "motif": motif,
                "created_at": Utc::now(),
            },
            "participants_notifies_count": participants.len(),
        })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────────────────
// POST /api/admin/afrolang/salles/{salle_id}/reactiver
// ──────────────────────────────────────────────────────────────────────────

/// Réactive une salle préalablement désactivée par l'administration.
pub async fn reactiver_salle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ReactivationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "moderer");

    let salle_id = path.into_inner();
    let commentaire = body
        .commentaire
        .as_ref()
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty());
    if let Some(ref c) = commentaire {
        if c.len() > COMMENTAIRE_REACTIVATION_MAX {
            return Err(ApiErreur::Validation(format!(
                "Le commentaire ne doit pas dépasser {} caractères",
                COMMENTAIRE_REACTIVATION_MAX
            )));
        }
    }

    let salle_etat: Option<(Option<DateTime<Utc>>, Option<DateTime<Utc>>)> = sqlx::query_as(
        "SELECT desactivee_admin_at, deleted_at FROM afrolang.salle WHERE id = $1",
    )
    .bind(salle_id)
    .fetch_optional(pool.get_ref())
    .await?;
    let (desactivee_admin_at, deleted_at) =
        salle_etat.ok_or_else(|| ApiErreur::NonTrouve("Salle introuvable".into()))?;
    if deleted_at.is_some() {
        return Err(ApiErreur::NonTrouve("Salle introuvable".into()));
    }
    if desactivee_admin_at.is_none() {
        return Err(ApiErreur::Conflit(
            "La salle n'est pas désactivée".into(),
        ));
    }

    let mut tx = pool.begin().await?;

    let reactivee_at: DateTime<Utc> = sqlx::query_scalar(
        "UPDATE afrolang.salle
         SET desactivee_admin_at = NULL, desactivee_par = NULL, motif_desactivation = NULL,
             reactivee_at = NOW(), reactivee_par = $2, commentaire_reactivation = $3,
             updated_at = NOW()
         WHERE id = $1
         RETURNING reactivee_at",
    )
    .bind(salle_id)
    .bind(admin.id)
    .bind(commentaire.as_deref())
    .fetch_one(&mut *tx)
    .await?;

    let evt_id: Uuid = sqlx::query_scalar(
        "INSERT INTO afrolang.evenement_moderation_salle
            (salle_id, session_concernee_id, type_action, admin_id, motif)
         VALUES ($1, NULL, 'reactivation_admin', $2, $3)
         RETURNING id",
    )
    .bind(salle_id)
    .bind(admin.id)
    .bind(commentaire.as_deref())
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    // Notifier admins de salle.
    let admins_salle: Vec<Uuid> = sqlx::query_scalar(
        "SELECT utilisateur_id FROM afrolang.salle_administrateur
         WHERE salle_id = $1 AND actif = TRUE",
    )
    .bind(salle_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    // Notifier aussi les créateurs des salles privées rattachées.
    let createurs_sp: Vec<Uuid> = sqlx::query_scalar(
        "SELECT DISTINCT cree_par FROM afrolang.salle_privee
         WHERE salle_id = $1 AND deleted_at IS NULL",
    )
    .bind(salle_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let mut destinataires: Vec<Uuid> = admins_salle;
    destinataires.extend(createurs_sp);
    destinataires.sort();
    destinataires.dedup();

    let lien = format!("/afrolang/{}", salle_id);
    for d in &destinataires {
        notification::creer_notification(
            pool.get_ref(),
            *d,
            notification::afrolang::SALLE_REACTIVEE_ADMIN,
            "La salle a été réactivée par l'administration.",
            Some(&lien),
        )
        .await;
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);

    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "UPDATE",
        "afrolang",
        "salle",
        Some(salle_id),
        Some(serde_json::json!({ "desactivee_admin_at": "<set>" })),
        Some(serde_json::json!({
            "desactivee_admin_at": null,
            "reactivee_at": reactivee_at,
            "commentaire_reactivation": commentaire,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "afrolang",
        "evenement_moderation_salle",
        Some(evt_id),
        None,
        Some(serde_json::json!({
            "type_action": "reactivation_admin",
            "salle_id": salle_id,
            "motif": commentaire,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "salle_id": salle_id,
            "reactivee_at": reactivee_at,
            "reactivee_par": admin.id,
            "commentaire": commentaire,
        })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────────────────
// GET /api/admin/afrolang/salles/{salle_id}/historique-moderation
// ──────────────────────────────────────────────────────────────────────────

#[derive(Debug, serde::Deserialize)]
pub struct HistoriquePagination {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct EvenementModerationRow {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub session_concernee_id: Option<Uuid>,
    #[sqlx(rename = "type_action")]
    pub type_action: TypeEvenementModeration,
    pub admin_id: Uuid,
    pub motif: Option<String>,
    pub created_at: DateTime<Utc>,
    pub admin_nom: Option<String>,
    pub admin_prenom: Option<String>,
}

/// Historique paginé des fermetures/réactivations d'une salle.
pub async fn historique_moderation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    query: web::Query<HistoriquePagination>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let salle_id = path.into_inner();
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * limit;

    let salle_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM afrolang.salle WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_existe {
        return Err(ApiErreur::NonTrouve("Salle introuvable".into()));
    }

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM afrolang.evenement_moderation_salle WHERE salle_id = $1",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;

    let rows = sqlx::query_as::<_, EvenementModerationRow>(
        "SELECT e.id, e.salle_id, e.session_concernee_id, e.type_action,
                e.admin_id, e.motif, e.created_at,
                u.nom AS admin_nom, u.prenom AS admin_prenom
         FROM afrolang.evenement_moderation_salle e
         LEFT JOIN iam.utilisateur u ON u.id = e.admin_id
         WHERE e.salle_id = $1
         ORDER BY e.created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(salle_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let items: Vec<EvenementModerationResponse> = rows
        .into_iter()
        .map(|r| EvenementModerationResponse {
            id: r.id,
            salle_id: r.salle_id,
            session_concernee_id: r.session_concernee_id,
            type_action: r.type_action,
            admin_id: r.admin_id,
            admin_nom: r.admin_nom,
            admin_prenom: r.admin_prenom,
            motif: r.motif,
            created_at: r.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, limit)),
        error: None,
    }))
}
