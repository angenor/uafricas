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
     e.domaine_autre,
     e.biographie, e.nb_annees_experience,
     e.rating::float8 AS rating,
     e.portfolio,
     e.linkedin_url,
     e.cv_url,
     e.specialites,
     e.objectifs::text[] AS objectifs,
     e.realisations,
     e.situations_professionnelles::text[] AS situations_professionnelles,
     e.statut::text AS statut,
     (SELECT COUNT(*) FROM iam.note_expertise ne WHERE ne.expertise_id = e.id) AS nombre_notes,
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
    pub domaine_autre: Option<String>,
    pub biographie: String,
    pub nb_annees_experience: i32,
    pub rating: f64,
    pub portfolio: Option<String>,
    pub linkedin_url: Option<String>,
    pub cv_url: Option<String>,
    pub specialites: Vec<String>,
    pub objectifs: Vec<String>,
    pub realisations: Vec<String>,
    pub situations_professionnelles: Vec<String>,
    pub statut: String,
    pub nombre_notes: i64,
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
    /// Nombre de notes reçues (sert à afficher « (n avis) »)
    #[serde(rename = "nombreNotes")]
    pub nombre_notes: i64,
    /// Note attribuée par le membre connecté (1–5), ou None s'il n'a pas noté / non connecté
    #[serde(rename = "maNote")]
    pub ma_note: Option<i16>,
    pub portfolio: Option<String>,
    #[serde(rename = "linkedinUrl")]
    pub linkedin_url: Option<String>,
    #[serde(rename = "cvUrl")]
    pub cv_url: Option<String>,
    pub specialites: Vec<String>,
    /// Objectifs avec libellés lisibles (front)
    pub objectifs: Vec<String>,
    pub realisations: Vec<String>,
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
    /// Metier declare, compare sans tenir compte de la casse (FR-046).
    pub specialite: Option<String>,
    pub tri: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Body pour creer une candidature expert
#[derive(Debug, Deserialize)]
pub struct CandidatureExpertBody {
    pub domaine: String,
    /// Précision libre lorsque `domaine` vaut "autre"
    pub domaine_autre: Option<String>,
    pub biographie: String,
    pub nb_annees_experience: i32,
    pub portfolio: Option<String>,
    pub linkedin_url: Option<String>,
    /// URL du CV déjà uploadé (via POST /api/experts/cv)
    pub cv_url: Option<String>,
    #[serde(default)]
    pub specialites: Vec<String>,
    /// Objectifs actuels (valeurs enum DB)
    #[serde(default)]
    pub objectifs: Vec<String>,
    #[serde(default)]
    pub realisations: Vec<String>,
    pub situations_professionnelles: Vec<String>,
}

/// Body pour noter un expert (1–5)
#[derive(Debug, Deserialize)]
pub struct NoterExpertBody {
    pub note: i16,
}

/// Réponse après notation : nouvelle moyenne + nombre de notes + note de l'auteur
#[derive(Debug, Serialize)]
pub struct NoteExpertResponse {
    pub rating: f64,
    #[serde(rename = "nombreNotes")]
    pub nombre_notes: i64,
    #[serde(rename = "maNote")]
    pub ma_note: i16,
}

// ────────────────────────────────────────────────────────────────
// Suivi de la candidature du membre connecte (US3)
// ────────────────────────────────────────────────────────────────

/// Row brute de la candidature active du membre (lecture directe iam.expertise)
#[derive(Debug, FromRow)]
pub struct MaCandidatureRow {
    pub id: Uuid,
    pub domaine: String,
    pub domaine_autre: Option<String>,
    pub biographie: String,
    pub nb_annees_experience: i32,
    pub portfolio: Option<String>,
    pub linkedin_url: Option<String>,
    pub cv_url: Option<String>,
    pub specialites: Vec<String>,
    pub objectifs: Vec<String>,
    pub realisations: Vec<String>,
    pub situations_professionnelles: Vec<String>,
    pub statut: String,
    pub commentaire_admin: Option<String>,
    pub date_validation: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// DTO de suivi expose au membre (statut + commentaire de refus eventuel)
#[derive(Debug, Serialize)]
pub struct MaCandidatureResponse {
    pub id: Uuid,
    pub domaine: String,
    pub biographie: String,
    #[serde(rename = "nbAnneesExperience")]
    pub nb_annees_experience: i32,
    pub portfolio: Option<String>,
    #[serde(rename = "linkedinUrl")]
    pub linkedin_url: Option<String>,
    #[serde(rename = "cvUrl")]
    pub cv_url: Option<String>,
    pub specialites: Vec<String>,
    pub objectifs: Vec<String>,
    pub realisations: Vec<String>,
    #[serde(rename = "situationsProfessionnelles")]
    pub situations_professionnelles: Vec<String>,
    pub statut: String,
    #[serde(rename = "commentaireAdmin")]
    pub commentaire_admin: Option<String>,
    #[serde(rename = "dateValidation")]
    pub date_validation: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

impl MaCandidatureRow {
    pub fn to_response(&self) -> MaCandidatureResponse {
        MaCandidatureResponse {
            id: self.id,
            domaine: label_domaine(&self.domaine, self.domaine_autre.as_deref()),
            biographie: self.biographie.clone(),
            nb_annees_experience: self.nb_annees_experience,
            portfolio: self.portfolio.clone(),
            linkedin_url: self.linkedin_url.clone(),
            cv_url: self.cv_url.clone(),
            specialites: self.specialites.clone(),
            objectifs: self.objectifs.iter().map(|o| label_objectif(o)).collect(),
            realisations: self.realisations.clone(),
            situations_professionnelles: self.situations_professionnelles.clone(),
            statut: self.statut.clone(),
            commentaire_admin: self.commentaire_admin.clone(),
            date_validation: self.date_validation,
            created_at: self.created_at,
        }
    }
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
        "autre" => "Autre".to_string(),
        autre => autre.to_string(),
    }
}

/// Mapper un objectif DB (snake_case) vers un libellé lisible (front)
pub fn label_objectif(db_val: &str) -> String {
    match db_val {
        "reseautage" => "Réseautage".to_string(),
        "consultance" => "Consultance".to_string(),
        "recherche_emploi" => "Recherche d'emploi".to_string(),
        "offre_services_court_terme" => "Offre de services court terme".to_string(),
        "travail_vacances" => "Travail de vacances (Sabbafrica)".to_string(),
        "volontariat" => "Volontariat".to_string(),
        "benevolat" => "Bénévolat".to_string(),
        autre => autre.to_string(),
    }
}

/// Valeurs DB valides pour l'enum objectif_expertise
pub const OBJECTIFS_VALIDES: &[&str] = &[
    "reseautage", "consultance", "recherche_emploi",
    "offre_services_court_terme", "travail_vacances", "volontariat", "benevolat",
];

/// Label de domaine affiché : la précision libre si `domaine = 'autre'`, sinon le label standard.
pub fn label_domaine(db_val: &str, domaine_autre: Option<&str>) -> String {
    if db_val == "autre" {
        if let Some(precision) = domaine_autre.map(str::trim).filter(|p| !p.is_empty()) {
            return precision.to_string();
        }
    }
    mapper_domaine_frontend(db_val)
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
        "autre" => "autre".to_string(),
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
                domaine: label_domaine(&self.domaine, self.domaine_autre.as_deref()),
                biographie: self.biographie.clone(),
                nb_annees_experience: self.nb_annees_experience,
                rating: self.rating,
                nombre_notes: self.nombre_notes,
                ma_note: None,
                portfolio: self.portfolio.clone(),
                linkedin_url: self.linkedin_url.clone(),
                cv_url: self.cv_url.clone(),
                specialites: self.specialites.clone(),
                objectifs: self.objectifs.iter().map(|o| label_objectif(o)).collect(),
                realisations: self.realisations.clone(),
                statut: self.statut.clone(),
            },
            situation_professionnelle: self.situations_professionnelles.clone(),
            date_inscription: self.date_inscription,
            date_derniere_mise_a_jour: self.expertise_updated_at,
        }
    }
}
