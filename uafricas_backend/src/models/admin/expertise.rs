use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::expert::label_domaine;

// ────────────────────────────────────────────────────────────────
// Row brute depuis la BDD (liste + detail admin)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminDemandeExpertiseRow {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub domaine: String,
    pub domaine_autre: Option<String>,
    pub biographie: String,
    pub nb_annees_experience: i32,
    pub portfolio: Option<String>,
    pub situations_professionnelles: Vec<String>,
    pub pays_nom: Option<String>,
    pub statut: String,
    pub commentaire_admin: Option<String>,
    pub valide_par_nom: Option<String>,
    pub date_validation: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ────────────────────────────────────────────────────────────────
// DTOs de reponse
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminDemandeExpertiseResponse {
    pub id: Uuid,
    #[serde(rename = "utilisateurId")]
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub domaine: String,
    #[serde(rename = "nbAnneesExperience")]
    pub nb_annees_experience: i32,
    pub pays: Option<String>,
    pub statut: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminDemandeExpertiseDetailResponse {
    pub id: Uuid,
    #[serde(rename = "utilisateurId")]
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub domaine: String,
    pub biographie: String,
    #[serde(rename = "nbAnneesExperience")]
    pub nb_annees_experience: i32,
    pub portfolio: Option<String>,
    #[serde(rename = "situationsProfessionnelles")]
    pub situations_professionnelles: Vec<String>,
    pub pays: Option<String>,
    pub statut: String,
    #[serde(rename = "commentaireAdmin")]
    pub commentaire_admin: Option<String>,
    #[serde(rename = "valideParNom")]
    pub valide_par_nom: Option<String>,
    #[serde(rename = "dateValidation")]
    pub date_validation: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminDemandeExpertiseListeResponse {
    pub demandes: Vec<AdminDemandeExpertiseResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ────────────────────────────────────────────────────────────────
// Parametres de requete et body
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminDemandeExpertiseQueryParams {
    pub statut: Option<String>,
    pub recherche: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RejeterExpertiseBody {
    pub commentaire_admin: String,
}

// ────────────────────────────────────────────────────────────────
// Conversions Row → Response
// ────────────────────────────────────────────────────────────────

impl AdminDemandeExpertiseRow {
    pub fn to_response(&self) -> AdminDemandeExpertiseResponse {
        AdminDemandeExpertiseResponse {
            id: self.id,
            utilisateur_id: self.utilisateur_id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            email: self.email.clone(),
            photo_url: self.photo_url.clone(),
            domaine: label_domaine(&self.domaine, self.domaine_autre.as_deref()),
            nb_annees_experience: self.nb_annees_experience,
            pays: self.pays_nom.clone(),
            statut: self.statut.clone(),
            created_at: self.created_at,
        }
    }

    pub fn to_detail_response(&self) -> AdminDemandeExpertiseDetailResponse {
        AdminDemandeExpertiseDetailResponse {
            id: self.id,
            utilisateur_id: self.utilisateur_id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            email: self.email.clone(),
            photo_url: self.photo_url.clone(),
            domaine: label_domaine(&self.domaine, self.domaine_autre.as_deref()),
            biographie: self.biographie.clone(),
            nb_annees_experience: self.nb_annees_experience,
            portfolio: self.portfolio.clone(),
            situations_professionnelles: self.situations_professionnelles.clone(),
            pays: self.pays_nom.clone(),
            statut: self.statut.clone(),
            commentaire_admin: self.commentaire_admin.clone(),
            valide_par_nom: self.valide_par_nom.clone(),
            date_validation: self.date_validation,
            created_at: self.created_at,
        }
    }
}
