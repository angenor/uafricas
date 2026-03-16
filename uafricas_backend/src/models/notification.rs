// ════════════════════════════════════════════════════════════════════════════
// Modèles — Notifications et doublons
// ════════════════════════════════════════════════════════════════════════════

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

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
