use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// PARTIE 1 : Chaînes TV (table media_content.chaine_tv)
// ═══════════════════════════════════════════════════════════════════════════

// ── Colonnes SQL ──────────────────────────────────────────────────────

pub const CHAINE_TV_COLONNES: &str =
    "ct.id, ct.nom, ct.slug, ct.description, ct.stream_url, ct.image_couverture_url,
     ct.categorie::text AS categorie, ct.pays_id, ct.langue, ct.est_en_direct,
     ct.etat, ct.cree_par, ct.created_at, ct.updated_at";

// ── Structs DB ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChaineTvRow {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: String,
    pub pays_id: Option<Uuid>,
    pub langue: String,
    pub est_en_direct: bool,
    pub etat: String,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs jointures optionnels
    #[sqlx(default)]
    pub pays_nom: Option<String>,
}

// ── Response DTOs ─────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ChaineTvResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: String,
    pub pays: Option<String>,
    pub langue: String,
    pub est_en_direct: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ChaineTvListeResponse {
    pub chaines: Vec<ChaineTvResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Query Params ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ChaineTvQueryParams {
    pub recherche: Option<String>,
    pub categorie: Option<String>,
    pub pays: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ── Formulaire de création ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerChaineTvForm {
    pub nom: String,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub categorie: Option<String>,
    pub pays: Option<String>,
    pub langue: Option<String>,
}

// ── Mapping ───────────────────────────────────────────────────────────

pub fn mapper_categorie_chaine_frontend(db_val: &str) -> String {
    match db_val {
        "generaliste" => "Généraliste".to_string(),
        "info" => "Info".to_string(),
        "sport" => "Sport".to_string(),
        "culture" => "Culture".to_string(),
        "divertissement" => "Divertissement".to_string(),
        "religieux" => "Religieux".to_string(),
        "education" => "Éducation".to_string(),
        "musique" => "Musique".to_string(),
        autre => autre.to_string(),
    }
}

pub fn mapper_categorie_chaine_db(frontend_val: &str) -> String {
    match frontend_val {
        "Généraliste" | "Generaliste" | "generaliste" => "generaliste".to_string(),
        "Info" | "info" | "Informations" => "info".to_string(),
        "Sport" | "sport" => "sport".to_string(),
        "Culture" | "culture" => "culture".to_string(),
        "Divertissement" | "divertissement" => "divertissement".to_string(),
        "Religieux" | "religieux" => "religieux".to_string(),
        "Éducation" | "Education" | "education" => "education".to_string(),
        "Musique" | "musique" => "musique".to_string(),
        autre => autre.to_lowercase(),
    }
}

pub fn generer_slug(nom: &str) -> String {
    nom.to_lowercase()
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

// ── Conversions ───────────────────────────────────────────────────────

impl ChaineTvRow {
    pub fn to_response(&self) -> ChaineTvResponse {
        ChaineTvResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            stream_url: self.stream_url.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            categorie: mapper_categorie_chaine_frontend(&self.categorie),
            pays: self.pays_nom.clone(),
            langue: self.langue.clone(),
            est_en_direct: self.est_en_direct,
            created_at: self.created_at,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PARTIE 2 : Programmes Vedettes (table programme_radio_tele, type='tele')
// ═══════════════════════════════════════════════════════════════════════════

pub const PROGRAMME_TELE_COLONNES: &str =
    "prt.id, prt.nom_emission, prt.slug, prt.description, prt.image_couverture_url,
     prt.video_url, prt.info_animateur, prt.info_producteur, prt.pays_id,
     prt.est_international, prt.langue, prt.etat, prt.cree_par,
     prt.chaine_id, prt.a_la_une, prt.created_at, prt.updated_at";

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProgrammeTeleRow {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub video_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub est_international: bool,
    pub langue: String,
    pub etat: String,
    pub cree_par: Uuid,
    pub chaine_id: Option<Uuid>,
    pub a_la_une: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs jointures optionnels
    #[sqlx(default)]
    pub pays_nom: Option<String>,
    #[sqlx(default)]
    pub chaine_nom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProgrammeTeleResponse {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub video_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays: Option<String>,
    pub est_international: bool,
    pub langue: String,
    pub chaine_id: Option<Uuid>,
    pub chaine_nom: Option<String>,
    pub a_la_une: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProgrammeTeleListeResponse {
    pub programmes: Vec<ProgrammeTeleResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize)]
pub struct ProgrammeTeleQueryParams {
    pub recherche: Option<String>,
    pub pays: Option<String>,
    pub chaine: Option<Uuid>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreerProgrammeTeleForm {
    pub nom_emission: String,
    pub description: String,
    pub video_url: String,
    pub image_couverture_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays: Option<String>,
    pub est_international: Option<bool>,
    pub langue: Option<String>,
    pub chaine_id: Option<Uuid>,
}

// ── Stats Télévision ──────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct TelevisionStats {
    pub nombre_chaines: i64,
    pub nombre_pays: i64,
    pub nombre_programmes: i64,
    pub nombre_chaines_en_direct: i64,
}

// ── Conversions ───────────────────────────────────────────────────────

impl ProgrammeTeleRow {
    pub fn to_response(&self) -> ProgrammeTeleResponse {
        ProgrammeTeleResponse {
            id: self.id,
            nom_emission: self.nom_emission.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            video_url: self.video_url.clone(),
            info_animateur: self.info_animateur.clone(),
            info_producteur: self.info_producteur.clone(),
            pays: self.pays_nom.clone(),
            est_international: self.est_international,
            langue: self.langue.clone(),
            chaine_id: self.chaine_id,
            chaine_nom: self.chaine_nom.clone(),
            a_la_une: self.a_la_une,
            created_at: self.created_at,
        }
    }
}
