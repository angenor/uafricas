use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour le mapping media_content.evenement
/// Le cast format::text convertit l'enum PostgreSQL en texte pour sqlx
pub const EVENEMENT_COLONNES: &str =
    "e.id, e.titre, e.slug, e.description, e.type AS type_categorie,
     e.pays_id, e.ville, e.adresse,
     e.date_heure_debut, e.date_heure_fin,
     e.image_couverture_url, e.format::text AS format,
     e.lien_en_ligne, e.langue, e.nombre_places,
     e.etat, e.cree_par, e.created_at, e.updated_at";

/// Representation d'un evenement en base de donnees
#[derive(Debug, FromRow)]
pub struct EvenementRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_categorie: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub image_couverture_url: Option<String>,
    pub format: String,
    pub lien_en_ligne: Option<String>,
    pub langue: String,
    pub nombre_places: Option<i32>,
    pub etat: String,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Info organisateur jointe depuis iam.utilisateur
#[derive(Debug, Serialize, FromRow)]
pub struct OrganisateurInfo {
    pub id: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
    pub photo_url: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// DTOs de reponse
// ──────────────────────────────────────────────────────────────

/// DTO pour un evenement dans la liste
#[derive(Debug, Serialize)]
pub struct EvenementResponse {
    pub id: Uuid,
    pub titre: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_format: String,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub couverture_url: Option<String>,
    pub statut: String,
    pub nombre_places: Option<i32>,
    pub nombre_inscrits: i64,
    pub user: OrganisateurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le detail d'un evenement
#[derive(Debug, Serialize)]
pub struct EvenementDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    #[serde(rename = "type")]
    pub type_format: String,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub couverture_url: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub statut: String,
    pub nombre_places: Option<i32>,
    pub nombre_inscrits: i64,
    pub est_inscrit: bool,
    pub user: OrganisateurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour l'organisateur
#[derive(Debug, Serialize)]
pub struct OrganisateurResponse {
    pub uid: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
    pub photo_url: Option<String>,
}

/// Reponse paginee pour le listing
#[derive(Debug, Serialize)]
pub struct EvenementListeResponse {
    pub evenements: Vec<EvenementResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Parametres de requete pour le listing
#[derive(Debug, Deserialize)]
pub struct EvenementQueryParams {
    pub recherche: Option<String>,
    pub format: Option<String>,
    pub pays: Option<String>,
    pub annee: Option<i32>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ──────────────────────────────────────────────────────────────
// Fonctions de mapping
// ──────────────────────────────────────────────────────────────

/// Mapper format DB (snake_case) vers label frontend
pub fn mapper_format_frontend(db_val: &str) -> String {
    match db_val {
        "en_ligne" => "En ligne".to_string(),
        "presentiel" => "En présentiel".to_string(),
        "hybride" => "Hybride".to_string(),
        autre => autre.to_string(),
    }
}

/// Mapper format frontend vers valeur DB
pub fn mapper_format_db(frontend_val: &str) -> String {
    match frontend_val {
        "En ligne" | "en_ligne" => "en_ligne".to_string(),
        "En présentiel" | "En presentiel" | "presentiel" => "presentiel".to_string(),
        "Hybride" | "hybride" => "hybride".to_string(),
        autre => autre.to_lowercase().replace(' ', "_"),
    }
}

/// Calculer le statut temporel a partir de etat + dates
pub fn calculer_statut(etat: &str, debut: &DateTime<Utc>, fin: Option<&DateTime<Utc>>) -> String {
    match etat {
        "annule" => "annule".to_string(),
        "termine" => "termine".to_string(),
        _ => {
            let maintenant = Utc::now();
            if maintenant < *debut {
                "a_venir".to_string()
            } else if let Some(fin) = fin {
                if maintenant > *fin {
                    "termine".to_string()
                } else {
                    "en_cours".to_string()
                }
            } else {
                "en_cours".to_string()
            }
        }
    }
}

/// Generer un slug URL-safe a partir du titre
pub fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â', 'ä'], "a")
        .replace(['ù', 'û', 'ü'], "u")
        .replace(['î', 'ï'], "i")
        .replace(['ô', 'ö'], "o")
        .replace('ç', "c")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}
