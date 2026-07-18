use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour le mapping media_content.mooc
/// Le cast format::text convertit l'enum PostgreSQL en texte pour sqlx
pub const MOOC_COLONNES: &str =
    "m.id, m.titre, m.slug, m.description, m.type AS type_formation,
     m.pays_id, m.ville,
     m.date_heure_debut, m.date_heure_fin,
     m.image_couverture_url, m.format::text AS format,
     m.lien_en_ligne, m.langue, m.nombre_places,
     m.prerequis, m.objectif, m.presentation, m.a_evaluation, m.est_certifiante,
     m.etat, m.cree_par, m.created_at, m.updated_at";

/// Representation d'un MOOC en base de donnees
#[derive(Debug, FromRow)]
pub struct MoocRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_formation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub image_couverture_url: Option<String>,
    pub format: String,
    pub lien_en_ligne: Option<String>,
    pub langue: String,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub objectif: Option<String>,
    pub presentation: Option<String>,
    pub a_evaluation: bool,
    pub est_certifiante: bool,
    pub etat: String,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Info formateur jointe depuis iam.utilisateur
#[derive(Debug, Serialize, FromRow)]
pub struct FormateurInfo {
    pub id: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
    pub photo_url: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// DTOs de reponse
// ──────────────────────────────────────────────────────────────

/// DTO pour un MOOC dans la liste
#[derive(Debug, Serialize)]
pub struct MoocResponse {
    pub id: Uuid,
    pub titre: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_formation: Option<String>,
    pub langue: String,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub couverture_url: Option<String>,
    pub statut: String,
    pub nombre_places: Option<i32>,
    pub nombre_inscrits: i64,
    pub formateur: FormateurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le detail d'un MOOC
#[derive(Debug, Serialize)]
pub struct MoocDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    #[serde(rename = "type")]
    pub type_formation: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub langue: String,
    pub format: String,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub couverture_url: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub statut: String,
    pub nombre_places: Option<i32>,
    pub nombre_inscrits: i64,
    pub prerequis: Option<String>,
    pub objectif: Option<String>,
    pub presentation: Option<String>,
    pub a_evaluation: bool,
    pub est_certifiante: bool,
    pub est_inscrit: bool,
    pub formateur: FormateurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le formateur
#[derive(Debug, Serialize)]
pub struct FormateurResponse {
    pub uid: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
    pub photo_url: Option<String>,
}

/// Reponse paginee pour le listing
#[derive(Debug, Serialize)]
pub struct MoocListeResponse {
    pub formations: Vec<MoocResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Parametres de requete pour le listing
#[derive(Debug, Deserialize)]
pub struct MoocQueryParams {
    pub recherche: Option<String>,
    #[serde(rename = "type")]
    pub type_formation: Option<String>,
    pub statut: Option<String>,
    pub gratuit: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ──────────────────────────────────────────────────────────────
// Fonctions de mapping
// ──────────────────────────────────────────────────────────────

/// Calculer le statut visible a partir de etat + dates + capacite
pub fn calculer_statut_mooc(
    etat: &str,
    debut: &DateTime<Utc>,
    fin: Option<&DateTime<Utc>>,
    nombre_inscrits: i64,
    nombre_places: Option<i32>,
) -> String {
    match etat {
        "annule" => "annule".to_string(),
        "termine" => "termine".to_string(),
        "en_cours" => "en_cours".to_string(),
        "suspendu" => "suspendu".to_string(),
        "publie" => {
            let maintenant = Utc::now();
            if let Some(fin) = fin {
                if maintenant > *fin {
                    return "termine".to_string();
                }
            }
            if maintenant > *debut {
                return "en_cours".to_string();
            }
            if let Some(max) = nombre_places {
                if nombre_inscrits >= max as i64 {
                    return "complet".to_string();
                }
            }
            "inscriptions_ouvertes".to_string()
        }
        _ => "programme".to_string(),
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
