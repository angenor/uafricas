use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ────────────────────────────────────────────────────────────────
// Colonnes SQL
// ────────────────────────────────────────────────────────────────

/// Colonnes pour SELECT sur iam.expertise e + iam.utilisateur u + shared.pays p
pub const EXPERT_COLONNES: &str =
    "e.id AS expertise_id, e.utilisateur_id,
     e.domaine::text AS domaine,
     e.biographie, e.nb_annees_experience,
     e.rating::float8 AS rating,
     e.portfolio,
     e.situations_professionnelles::text[] AS situations_professionnelles,
     e.statut::text AS statut,
     e.created_at AS expertise_created_at,
     e.updated_at AS expertise_updated_at,
     u.nom, u.prenom, u.email, u.photo_url, u.ville,
     u.created_at AS date_inscription,
     p.nom AS pays_nom";

// ────────────────────────────────────────────────────────────────
// Row brute depuis la BDD
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct ExpertRow {
    pub expertise_id: Uuid,
    pub utilisateur_id: Uuid,
    pub domaine: String,
    pub biographie: String,
    pub nb_annees_experience: i32,
    pub rating: f64,
    pub portfolio: Option<String>,
    pub situations_professionnelles: Vec<String>,
    pub statut: String,
    pub expertise_created_at: DateTime<Utc>,
    pub expertise_updated_at: DateTime<Utc>,
    // Champs du JOIN iam.utilisateur
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub ville: Option<String>,
    pub date_inscription: DateTime<Utc>,
    // Champs du JOIN shared.pays
    pub pays_nom: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Response DTOs
// ────────────────────────────────────────────────────────────────

/// Info d'expertise imbriquee dans la reponse
#[derive(Debug, Serialize)]
pub struct ExpertiseInfoResponse {
    pub domaine: String,
    pub biographie: String,
    #[serde(rename = "nbAnneesExperience")]
    pub nb_annees_experience: i32,
    pub rating: f64,
    pub portfolio: Option<String>,
    pub statut: String,
}

/// DTO pour un expert (liste et detail)
#[derive(Debug, Serialize)]
pub struct ExpertResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    #[serde(rename = "photoURL")]
    pub photo_url: Option<String>,
    pub pays: String,
    pub ville: Option<String>,
    pub email: String,
    #[serde(rename = "expertiseInfo")]
    pub expertise_info: ExpertiseInfoResponse,
    #[serde(rename = "situationProfessionnelle")]
    pub situation_professionnelle: Vec<String>,
    #[serde(rename = "dateInscription")]
    pub date_inscription: DateTime<Utc>,
    #[serde(rename = "dateDerniereMiseAJour")]
    pub date_derniere_mise_a_jour: DateTime<Utc>,
}

/// Reponse paginee pour la liste des experts
#[derive(Debug, Serialize)]
pub struct ExpertListeResponse {
    pub experts: Vec<ExpertResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Parametres de requete pour le listing
#[derive(Debug, Deserialize)]
pub struct ExpertQueryParams {
    pub recherche: Option<String>,
    pub domaine: Option<String>,
    pub pays: Option<String>,
    pub situation: Option<String>,
    pub tri: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Body pour creer une candidature expert
#[derive(Debug, Deserialize)]
pub struct CandidatureExpertBody {
    pub domaine: String,
    pub biographie: String,
    pub nb_annees_experience: i32,
    pub portfolio: Option<String>,
    pub situations_professionnelles: Vec<String>,
}

// ────────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ────────────────────────────────────────────────────────────────

/// Mapper domaine DB (snake_case) vers label frontend
pub fn mapper_domaine_frontend(db_val: &str) -> String {
    match db_val {
        "agriculture" => "Agriculture".to_string(),
        "informatique" => "Informatique".to_string(),
        "electronique" => "Électronique".to_string(),
        "immobilier" => "Immobilier".to_string(),
        "mecanique" => "Mécanique".to_string(),
        "sante" => "Santé".to_string(),
        "education" => "Éducation".to_string(),
        "finance" => "Finance".to_string(),
        autre => autre.to_string(),
    }
}

/// Mapper domaine frontend vers valeur DB (snake_case)
pub fn mapper_domaine_db(frontend_val: &str) -> String {
    match frontend_val.to_lowercase().as_str() {
        "agriculture" => "agriculture".to_string(),
        "informatique" => "informatique".to_string(),
        "electronique" | "électronique" => "electronique".to_string(),
        "immobilier" => "immobilier".to_string(),
        "mecanique" | "mécanique" => "mecanique".to_string(),
        "sante" | "santé" => "sante".to_string(),
        "education" | "éducation" => "education".to_string(),
        "finance" => "finance".to_string(),
        autre => autre.to_string(),
    }
}

// ────────────────────────────────────────────────────────────────
// Conversions Row → Response
// ────────────────────────────────────────────────────────────────

impl ExpertRow {
    /// Convertir une row en DTO de reponse
    pub fn to_response(&self) -> ExpertResponse {
        ExpertResponse {
            id: self.utilisateur_id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            photo_url: self.photo_url.clone(),
            pays: self.pays_nom.clone().unwrap_or_else(|| "Non spécifié".to_string()),
            ville: self.ville.clone(),
            email: self.email.clone(),
            expertise_info: ExpertiseInfoResponse {
                domaine: mapper_domaine_frontend(&self.domaine),
                biographie: self.biographie.clone(),
                nb_annees_experience: self.nb_annees_experience,
                rating: self.rating,
                portfolio: self.portfolio.clone(),
                statut: self.statut.clone(),
            },
            situation_professionnelle: self.situations_professionnelles.clone(),
            date_inscription: self.date_inscription,
            date_derniere_mise_a_jour: self.expertise_updated_at,
        }
    }
}
