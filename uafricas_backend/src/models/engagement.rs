//! DTO publics du système d'engagement (lecture). Reflètent le schéma
//! `engagement` (Principe III).

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Niveau/statut d'un membre (issu de `engagement.niveau`).
#[derive(Serialize, FromRow)]
pub struct NiveauInfo {
    pub code: String,
    pub libelle: String,
    pub seuil_min: i32,
    pub badge_couleur: Option<String>,
    pub badge_icone: Option<String>,
}

/// Prochain palier de niveau à atteindre (null si niveau maximal).
#[derive(Serialize)]
pub struct ProchainNiveau {
    pub code: String,
    pub libelle: String,
    pub seuil_min: i32,
    pub points_restants: i32,
}

/// Ligne brute du compte d'engagement.
#[derive(FromRow)]
pub struct CompteRow {
    pub solde_points: i32,
    pub solde_points_mensuel: i32,
    pub reputation: i32,
    pub niveau_code: String,
    pub dernier_mouvement_at: Option<DateTime<Utc>>,
}

/// Réponse « Mon compte d'engagement ».
#[derive(Serialize)]
pub struct CompteResponse {
    pub solde_points: i32,
    pub solde_points_mensuel: i32,
    pub reputation: i32,
    pub niveau: NiveauInfo,
    pub prochain_niveau: Option<ProchainNiveau>,
    pub dernier_mouvement_at: Option<DateTime<Utc>>,
}

/// Un mouvement de points (entrée de journal).
#[derive(Serialize, FromRow)]
pub struct MouvementResponse {
    pub id: Uuid,
    pub type_action: String,
    pub libelle: Option<String>,
    pub type_objet: Option<String>,
    pub objet_id: Option<Uuid>,
    pub points: i32,
    pub reputation_delta: i32,
    pub solde_apres: i32,
    pub plafond_atteint: bool,
    pub created_at: DateTime<Utc>,
}

/// Page du journal des points.
#[derive(Serialize)]
pub struct JournalPage {
    pub elements: Vec<MouvementResponse>,
    pub total: i64,
    pub page: i64,
    pub taille: i64,
}
