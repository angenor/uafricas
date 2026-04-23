use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ────────────────────────────────────────────────────────────────
// Row brute depuis la BDD (liste admin)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminDemandeBiblioRow {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub fonction: String,
    pub biographie: String,
    pub pays_nom: Option<String>,
    pub statut: String,
    pub specialites_noms: String,
    pub commentaire_admin: Option<String>,
    pub traite_par_nom: Option<String>,
    pub traite_le: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ────────────────────────────────────────────────────────────────
// DTOs de réponse
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminDemandeBiblioResponse {
    pub id: Uuid,
    #[serde(rename = "utilisateurId")]
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub fonction: String,
    pub statut: String,
    pub specialites: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminDemandeBiblioDetailResponse {
    pub id: Uuid,
    #[serde(rename = "utilisateurId")]
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub fonction: String,
    pub biographie: String,
    pub pays: Option<String>,
    pub statut: String,
    pub specialites: Vec<String>,
    #[serde(rename = "commentaireAdmin")]
    pub commentaire_admin: Option<String>,
    #[serde(rename = "traiteParNom")]
    pub traite_par_nom: Option<String>,
    #[serde(rename = "traiteLe")]
    pub traite_le: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminDemandeBiblioListeResponse {
    pub demandes: Vec<AdminDemandeBiblioResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ────────────────────────────────────────────────────────────────
// Paramètres de requête et body
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminDemandeBiblioQueryParams {
    pub statut: Option<String>,
    pub recherche: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TraiterDemandeBody {
    pub commentaire: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Conversions Row → Response
// ────────────────────────────────────────────────────────────────

impl AdminDemandeBiblioRow {
    fn specialites_vec(&self) -> Vec<String> {
        if self.specialites_noms.is_empty() {
            vec![]
        } else {
            self.specialites_noms.split(", ").map(|s| s.to_string()).collect()
        }
    }

    pub fn to_response(&self) -> AdminDemandeBiblioResponse {
        AdminDemandeBiblioResponse {
            id: self.id,
            utilisateur_id: self.utilisateur_id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            email: self.email.clone(),
            photo_url: self.photo_url.clone(),
            fonction: self.fonction.clone(),
            statut: self.statut.clone(),
            specialites: self.specialites_vec(),
            created_at: self.created_at,
        }
    }

    pub fn to_detail_response(&self) -> AdminDemandeBiblioDetailResponse {
        AdminDemandeBiblioDetailResponse {
            id: self.id,
            utilisateur_id: self.utilisateur_id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            email: self.email.clone(),
            photo_url: self.photo_url.clone(),
            fonction: self.fonction.clone(),
            biographie: self.biographie.clone(),
            pays: self.pays_nom.clone(),
            statut: self.statut.clone(),
            specialites: self.specialites_vec(),
            commentaire_admin: self.commentaire_admin.clone(),
            traite_par_nom: self.traite_par_nom.clone(),
            traite_le: self.traite_le,
            created_at: self.created_at,
        }
    }
}
