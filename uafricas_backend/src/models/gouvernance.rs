use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Stats de gouvernance ──────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct GouvernanceStats {
    pub total: i64,
    pub factcheck: i64,
    pub badhabits: i64,
    pub ideaforces: i64,
    pub total_likes: i64,
}

// ── Row unifie pour la requete UNION ALL ──────────────────────

#[derive(Debug, FromRow)]
pub struct ContributionRow {
    pub id: Uuid,
    pub type_contribution: String,
    pub titre: String,
    pub description: String,
    pub nombre_likes: i32,
    pub nombre_soutiens: i32,
    pub statut: String,
    pub created_at: DateTime<Utc>,
    pub auteur_id: Option<Uuid>,
    pub auteur_prenom: Option<String>,
    pub auteur_nom: Option<String>,
    pub auteur_photo_url: Option<String>,
    pub pays_nom: Option<String>,
    pub region: Option<String>,
    pub ville: Option<String>,
    pub categorie: Option<String>,
    pub gravite: Option<String>,
    pub total_count: i64,
}

// ── DTOs de reponse API ───────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ContributionAuteurResponse {
    pub id: Uuid,
    pub prenom: String,
    pub nom: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContributionLocalisationResponse {
    pub pays: String,
    pub region: Option<String>,
    pub ville: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContributionStatsResponse {
    pub likes: i32,
    pub soutiens: i32,
}

#[derive(Debug, Serialize)]
pub struct ContributionResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_contribution: String,
    pub statut: String,
    pub titre: String,
    pub description: String,
    pub auteur: ContributionAuteurResponse,
    pub localisation: ContributionLocalisationResponse,
    pub date_creation: DateTime<Utc>,
    pub stats: ContributionStatsResponse,
    pub categorie: Option<String>,
    pub gravite: Option<String>,
    pub type_pratique: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContributionListeResponse {
    pub contributions: Vec<ContributionResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

impl ContributionRow {
    pub fn to_response(&self) -> ContributionResponse {
        // Pour l'instant, le backend ne stocke que des mauvaises pratiques
        // dans governance.bad_habit. Les bonnes pratiques n'ont pas encore
        // de table dediee : on expose donc "mauvaise" pour badhabits.
        let type_pratique = match self.type_contribution.as_str() {
            "badhabits" => Some("mauvaise".to_string()),
            _ => None,
        };
        ContributionResponse {
            id: self.id,
            type_contribution: self.type_contribution.clone(),
            statut: self.statut.clone(),
            titre: self.titre.clone(),
            description: self.description.clone(),
            auteur: ContributionAuteurResponse {
                id: self.auteur_id.unwrap_or_default(),
                prenom: self.auteur_prenom.clone().unwrap_or_default(),
                nom: self.auteur_nom.clone().unwrap_or_default(),
                photo_url: self.auteur_photo_url.clone(),
            },
            localisation: ContributionLocalisationResponse {
                pays: self.pays_nom.clone().unwrap_or_else(|| "Inconnu".to_string()),
                region: self.region.clone(),
                ville: self.ville.clone(),
            },
            date_creation: self.created_at,
            stats: ContributionStatsResponse {
                likes: self.nombre_likes,
                soutiens: self.nombre_soutiens,
            },
            categorie: self.categorie.clone(),
            gravite: self.gravite.clone(),
            type_pratique,
        }
    }
}

// ── Parametres de requete ─────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ContributionQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    #[serde(rename = "type")]
    pub type_contribution: Option<String>,
}
