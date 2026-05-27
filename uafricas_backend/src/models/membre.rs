use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ────────────────────────────────────────────────────────────────
// Colonnes SQL
// ────────────────────────────────────────────────────────────────

/// Colonnes pour SELECT sur iam.utilisateur u + shared.pays p
/// Expose uniquement des champs publics (pas d'email, telephone, etc.)
pub const MEMBRE_COLONNES: &str = "u.id, u.nom, u.prenom, u.slug, u.photo_url,
     u.fonction, u.ville, u.biographie,
     u.created_at AS date_inscription,
     p.nom AS pays_nom,
     ARRAY(
         SELECT r.slug FROM iam.role r
         JOIN iam.utilisateur_role ur ON ur.role_id = r.id
         WHERE ur.utilisateur_id = u.id
     )::text[] AS roles,
     EXISTS (
         SELECT 1 FROM iam.expertise e
         WHERE e.utilisateur_id = u.id AND e.statut = 'valide' AND e.deleted_at IS NULL
     ) AS est_expert,
     EXISTS (
         SELECT 1 FROM iam.demande_biblio_humaine d
         WHERE d.utilisateur_id = u.id AND d.statut = 'valide' AND d.deleted_at IS NULL
     ) AS est_biblio";

// ────────────────────────────────────────────────────────────────
// Row brute depuis la BDD
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct MembreRow {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub fonction: Option<String>,
    pub ville: Option<String>,
    pub biographie: Option<String>,
    pub date_inscription: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub roles: Vec<String>,
    pub est_expert: bool,
    pub est_biblio: bool,
}

// ────────────────────────────────────────────────────────────────
// Response DTOs
// ────────────────────────────────────────────────────────────────

/// DTO pour un membre (liste et detail)
#[derive(Debug, Serialize)]
pub struct MembreResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub slug: Option<String>,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub fonction: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub biographie: Option<String>,
    pub roles: Vec<String>,
    #[serde(rename = "dateInscription")]
    pub date_inscription: DateTime<Utc>,
    #[serde(rename = "estExpert")]
    pub est_expert: bool,
    #[serde(rename = "estBiblio")]
    pub est_biblio: bool,
}

/// Reponse paginee pour la liste des membres
#[derive(Debug, Serialize)]
pub struct MembreListeResponse {
    pub membres: Vec<MembreResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Parametres de requete pour le listing
#[derive(Debug, Deserialize)]
pub struct MembreQueryParams {
    pub recherche: Option<String>,
    /// Filtre par type : "expert" ou "biblio"
    pub r#type: Option<String>,
    pub pays: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ────────────────────────────────────────────────────────────────
// Conversions Row → Response
// ────────────────────────────────────────────────────────────────

impl MembreRow {
    /// Convertir une row en DTO de reponse
    pub fn to_response(&self) -> MembreResponse {
        MembreResponse {
            id: self.id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            slug: self.slug.clone(),
            photo_url: self.photo_url.clone(),
            fonction: self.fonction.clone(),
            pays: self.pays_nom.clone(),
            ville: self.ville.clone(),
            biographie: self.biographie.clone(),
            roles: self.roles.clone(),
            date_inscription: self.date_inscription,
            est_expert: self.est_expert,
            est_biblio: self.est_biblio,
        }
    }
}
