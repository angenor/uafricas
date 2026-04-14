// ════════════════════════════════════════════════════════════════════════════
// Modèles — Notifications et doublons
// ════════════════════════════════════════════════════════════════════════════

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// ── Types de notifications Afrolang (feature 005, research Décision 10) ──
// Utilisés comme clés de `type` dans la table notifications (quelle qu'elle soit)
// — centralisées ici pour éviter les typos et garantir la cohérence entre
// handlers producteurs et UI consommatrice.

pub mod afrolang {
    pub const PROPOSITION_VALIDEE: &str = "afrolang.proposition_validee";
    pub const PROPOSITION_REFUSEE: &str = "afrolang.proposition_refusee";
    pub const MODERATION_REPRISE: &str = "afrolang.moderation_reprise";
    pub const ADHESION_DEMANDEE: &str = "afrolang.adhesion_demandee";
    pub const ADHESION_ACCEPTEE: &str = "afrolang.adhesion_acceptee";
    pub const ADHESION_REFUSEE: &str = "afrolang.adhesion_refusee";
    pub const ADHESION_GROUPE_COMPLET: &str = "afrolang.adhesion_groupe_complet";
    pub const INVITATION_RECUE: &str = "afrolang.invitation_recue";
    pub const INVITATION_REFUSEE: &str = "afrolang.invitation_refusee";
    pub const SALLE_PRIVEE_ARCHIVEE: &str = "afrolang.salle_privee_archivee";
    pub const LIEN_EXTERNE_PUBLIE: &str = "afrolang.lien_externe_publie";
    pub const LIEN_EXTERNE_REFUSE: &str = "afrolang.lien_externe_refuse";
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct NotificationRow {
    pub id: Uuid,
    pub destinataire_id: Uuid,
    #[serde(rename = "type")]
    pub type_: String,
    pub message: String,
    pub lien_action: Option<String>,
    pub lu: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct FusionDoublonDto {
    pub personne_a_garder_id: Uuid,
    pub personne_a_supprimer_id: Uuid,
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance_annee: Option<i16>,
    pub naissance_mois: Option<i16>,
    pub naissance_jour: Option<i16>,
    pub naissance_lieu: Option<String>,
    pub deces_annee: Option<i16>,
    pub deces_mois: Option<i16>,
    pub deces_jour: Option<i16>,
    pub deces_lieu: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IgnorerDoublonDto {
    pub personne_a_id: Uuid,
    pub personne_b_id: Uuid,
}

/// Crée une notification de façon non-bloquante (fire-and-forget)
pub async fn creer_notification(
    pool: &PgPool,
    destinataire_id: Uuid,
    type_notif: &str,
    message: &str,
    lien_action: Option<&str>,
) {
    let _ = sqlx::query(
        "INSERT INTO arbre_genealogique.notifications (destinataire_id, type, message, lien_action)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(destinataire_id)
    .bind(type_notif)
    .bind(message)
    .bind(lien_action)
    .execute(pool)
    .await;
}
