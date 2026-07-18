//! DTO admin du système d'engagement (barème + journal).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Règle de barème (édition admin).
#[derive(Serialize, FromRow)]
pub struct RegleAdmin {
    pub id: Uuid,
    pub type_action: String,
    pub libelle: String,
    pub points: i32,
    pub reputation_delta: i32,
    pub plafond_journalier: Option<i32>,
    pub plafond_mensuel: Option<i32>,
    pub actif: bool,
}

#[derive(Deserialize)]
pub struct ModifierRegleRequest {
    pub libelle: Option<String>,
    pub points: Option<i32>,
    pub reputation_delta: Option<i32>,
    pub plafond_journalier: Option<Option<i32>>,
    pub plafond_mensuel: Option<Option<i32>>,
    pub actif: Option<bool>,
}

/// Palier de popularité.
#[derive(Serialize, FromRow)]
pub struct PalierAdmin {
    pub id: Uuid,
    pub seuil_likes: i32,
    pub points: i32,
    pub actif: bool,
}

#[derive(Deserialize)]
pub struct CreerPalierRequest {
    pub seuil_likes: i32,
    pub points: i32,
}

#[derive(Deserialize)]
pub struct ModifierPalierRequest {
    pub points: Option<i32>,
    pub actif: Option<bool>,
}

/// Seuil de niveau.
#[derive(Serialize, FromRow)]
pub struct NiveauAdmin {
    pub id: Uuid,
    pub code: String,
    pub libelle: String,
    pub seuil_min: i32,
    pub ordre: i16,
    pub badge_couleur: Option<String>,
    pub badge_icone: Option<String>,
}

#[derive(Deserialize)]
pub struct ModifierNiveauRequest {
    pub libelle: Option<String>,
    pub seuil_min: Option<i32>,
    pub badge_couleur: Option<String>,
    pub badge_icone: Option<String>,
}

/// Entrée du journal global (enrichie du nom du membre).
#[derive(Serialize, FromRow)]
pub struct JournalAdminRow {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub utilisateur_nom: Option<String>,
    pub type_action: String,
    pub type_objet: Option<String>,
    pub objet_id: Option<Uuid>,
    pub points: i32,
    pub reputation_delta: i32,
    pub solde_apres: i32,
    pub plafond_atteint: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct JournalAdminPage {
    pub elements: Vec<JournalAdminRow>,
    pub total: i64,
    pub page: i64,
    pub taille: i64,
}

#[derive(Deserialize)]
pub struct JournalAdminParams {
    pub utilisateur_id: Option<Uuid>,
    pub type_action: Option<String>,
    pub depuis: Option<String>,
    pub jusqu_a: Option<String>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

#[derive(Deserialize)]
pub struct AjustementRequest {
    pub utilisateur_id: Uuid,
    pub points: i32,
    #[serde(default)]
    pub reputation_delta: i32,
    pub motif: Option<String>,
}

/// Marque une contribution comme « mise en avant » (déclenche le +5 à l'auteur).
#[derive(Deserialize)]
pub struct MiseEnAvantRequest {
    pub type_objet: String,
    pub objet_id: Uuid,
}

/// État de mise en avant d'une contribution (pour le bouton admin).
#[derive(Serialize)]
pub struct MiseEnAvantEtat {
    pub mis_en_avant: bool,
}
