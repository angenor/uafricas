//! DTOs pour les interactions sociales sur les profils membres :
//! partage sur le mur communautaire (/publications) et signalement.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Seuil de signalements distincts au-delà duquel un profil est suspendu auto.
pub const SEUIL_SIGNALEMENTS_PROFIL_SUSPENSION: i64 = 20;

// ────────────────────────────────────────────────────────────────
// Partage de profil
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PartageProfilRequest {
    pub legende: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PartageQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Ligne brute du JOIN partage + profil partagé + auteur
#[derive(Debug, FromRow)]
pub struct PartageProfilRow {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    // Profil partagé
    pub profil_id: Uuid,
    pub profil_nom: String,
    pub profil_prenom: String,
    pub profil_photo_url: Option<String>,
    pub profil_fonction: Option<String>,
    pub profil_pays: Option<String>,
    pub profil_ville: Option<String>,
    pub profil_est_expert: bool,
    pub profil_est_biblio: bool,
    // Auteur du partage
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartageProfilAuteur {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartageProfilApercu {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
    pub fonction: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub est_expert: bool,
    pub est_biblio: bool,
}

#[derive(Debug, Serialize)]
pub struct PartageProfilResponse {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    pub profil: PartageProfilApercu,
    pub auteur: PartageProfilAuteur,
}

#[derive(Debug, Serialize)]
pub struct PartageProfilListeResponse {
    pub partages: Vec<PartageProfilResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

impl From<PartageProfilRow> for PartageProfilResponse {
    fn from(row: PartageProfilRow) -> Self {
        PartageProfilResponse {
            id: row.id,
            legende: row.legende,
            created_at: row.created_at,
            profil: PartageProfilApercu {
                id: row.profil_id,
                nom: row.profil_nom,
                prenom: row.profil_prenom,
                photo_url: row.profil_photo_url,
                fonction: row.profil_fonction,
                pays: row.profil_pays,
                ville: row.profil_ville,
                est_expert: row.profil_est_expert,
                est_biblio: row.profil_est_biblio,
            },
            auteur: PartageProfilAuteur {
                id: row.auteur_id,
                nom: row.auteur_nom,
                prenom: row.auteur_prenom,
                photo_url: row.auteur_photo_url,
            },
        }
    }
}

// ────────────────────────────────────────────────────────────────
// Signalement de profil
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SignalementProfilRequest {
    pub motif: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignalementProfilEtat {
    pub nombre_signalements: i32,
    pub etat: String,
    pub deja_signale: bool,
    pub suspendu: bool,
}
