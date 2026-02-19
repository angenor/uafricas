use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────
pub const ADMIN_PAYS_LISTE_COLONNES: &str =
    "p.id, p.nom, p.code_iso2, p.code_iso3, p.capitale, p.continent, p.actif, p.created_at";

pub const ADMIN_PAYS_DETAIL_COLONNES: &str =
    "p.id, p.nom, p.code_iso2, p.code_iso3, p.indicatif_tel, p.capitale, p.continent,
     p.longitude, p.latitude, p.actif, p.created_at, p.updated_at";

pub const PAYS_TRI_COLONNES: &[&str] = &[
    "created_at", "nom", "code_iso2", "continent", "capitale",
];

// ── Row (lecture BDD) ─────────────────────────────────────
#[derive(Debug, FromRow)]
pub struct AdminPaysDetailRow {
    pub id: Uuid,
    pub nom: String,
    pub code_iso2: Option<String>,
    pub code_iso3: Option<String>,
    pub indicatif_tel: Option<String>,
    pub capitale: Option<String>,
    pub continent: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ── Responses DTO ─────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminPaysListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub code_iso2: Option<String>,
    pub code_iso3: Option<String>,
    pub capitale: Option<String>,
    pub continent: Option<String>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminPaysDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub code_iso2: Option<String>,
    pub code_iso3: Option<String>,
    pub indicatif_tel: Option<String>,
    pub capitale: Option<String>,
    pub continent: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl AdminPaysDetailRow {
    pub fn to_response(&self) -> AdminPaysDetailResponse {
        AdminPaysDetailResponse {
            id: self.id,
            nom: self.nom.clone(),
            code_iso2: self.code_iso2.clone(),
            code_iso3: self.code_iso3.clone(),
            indicatif_tel: self.indicatif_tel.clone(),
            capitale: self.capitale.clone(),
            continent: self.continent.clone(),
            longitude: self.longitude,
            latitude: self.latitude,
            actif: self.actif,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Requests DTO ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct CreerPaysRequest {
    pub nom: String,
    pub code_iso2: Option<String>,
    pub code_iso3: Option<String>,
    pub indicatif_tel: Option<String>,
    pub capitale: Option<String>,
    pub continent: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierPaysRequest {
    pub nom: Option<String>,
    pub code_iso2: Option<String>,
    pub code_iso3: Option<String>,
    pub indicatif_tel: Option<String>,
    pub capitale: Option<String>,
    pub continent: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub actif: Option<bool>,
}

// ── Query Params ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct AdminPaysQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub continent: Option<String>,
}
