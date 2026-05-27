//! Modèles admin pour la fermeture/réactivation administrative d'une salle Afrolang
//! et l'historique de modération (feature 001-ressources-fermeture-session, data-model.md §5).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.type_evenement_moderation", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TypeEvenementModeration {
    FermetureAdmin,
    ReactivationAdmin,
}

/// Évènement de modération append-only sur une salle.
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct EvenementModerationSalle {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub session_concernee_id: Option<Uuid>,
    pub type_action: TypeEvenementModeration,
    pub admin_id: Uuid,
    pub motif: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// DTO de réponse enrichi du nom de l'admin auteur de l'action.
#[derive(Debug, Clone, Serialize)]
pub struct EvenementModerationResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub session_concernee_id: Option<Uuid>,
    pub type_action: TypeEvenementModeration,
    pub admin_id: Uuid,
    pub admin_nom: Option<String>,
    pub admin_prenom: Option<String>,
    pub motif: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Corps de requête : fermeture administrative d'une session pour abus.
/// `motif` : 10..=1000 caractères, obligatoire.
#[derive(Debug, Clone, Deserialize)]
pub struct FermetureAdminRequest {
    pub motif: String,
}

/// Corps de requête : réactivation administrative d'une salle.
/// `commentaire` facultatif, ≤ 1000 caractères.
#[derive(Debug, Clone, Deserialize)]
pub struct ReactivationRequest {
    pub commentaire: Option<String>,
}
