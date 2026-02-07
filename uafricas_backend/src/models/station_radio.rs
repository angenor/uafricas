use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────────────────

pub const STATION_RADIO_COLONNES: &str =
    "sr.id, sr.nom, sr.slug, sr.description, sr.stream_url, sr.image_couverture_url,
     sr.genre, sr.genres_liste, sr.pays_id, sr.ville,
     sr.type_station::text AS type_station, sr.etat,
     sr.cree_par, sr.created_at, sr.updated_at";

// ── Structs DB ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StationRadioRow {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: String,
    pub image_couverture_url: Option<String>,
    pub genre: Option<String>,
    pub genres_liste: Vec<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub type_station: String,
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
pub struct StationRadioResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: String,
    pub image_couverture_url: Option<String>,
    pub genre: Option<String>,
    pub genres_liste: Vec<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub type_station: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct StationRadioListeResponse {
    pub stations: Vec<StationRadioResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Query Params ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct StationRadioQueryParams {
    pub recherche: Option<String>,
    pub type_station: Option<String>,
    pub pays: Option<String>,
    pub genre: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ── Formulaire de création ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerStationRadioForm {
    pub nom: String,
    pub description: Option<String>,
    pub stream_url: String,
    pub genre: Option<String>,
    pub genres_liste: Option<Vec<String>>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub type_station: Option<String>,
}

// ── Mapping ───────────────────────────────────────────────────────────

pub fn mapper_type_station_frontend(db_val: &str) -> String {
    match db_val {
        "nationale" => "Nationales".to_string(),
        "locale" => "Local".to_string(),
        "internationale" => "International".to_string(),
        autre => autre.to_string(),
    }
}

pub fn mapper_type_station_db(frontend_val: &str) -> String {
    match frontend_val {
        "Nationales" | "nationales" | "nationale" => "nationale".to_string(),
        "Local" | "local" | "locale" => "locale".to_string(),
        "International" | "international" | "internationale" => "internationale".to_string(),
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

impl StationRadioRow {
    pub fn to_response(&self) -> StationRadioResponse {
        StationRadioResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            stream_url: self.stream_url.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            genre: self.genre.clone(),
            genres_liste: self.genres_liste.clone(),
            pays: self.pays_nom.clone(),
            ville: self.ville.clone(),
            type_station: mapper_type_station_frontend(&self.type_station),
            created_at: self.created_at,
        }
    }
}
