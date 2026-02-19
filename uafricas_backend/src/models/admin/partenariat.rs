use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ────────────────────────────────────────────────

pub const ADMIN_PARTENARIAT_COLONNES: &str =
    "p.id, p.organisation_id, p.type_partenariat, p.description,
     p.date_debut, p.date_fin, p.actif, p.approuve_par,
     p.created_at, p.updated_at";

pub const PARTENARIAT_TRI_COLONNES: &[&str] = &[
    "created_at", "updated_at", "type_partenariat", "date_debut", "actif",
];

// ── Row struct ──────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminPartenariatRow {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub type_partenariat: Option<String>,
    pub description: Option<String>,
    pub date_debut: Option<NaiveDate>,
    pub date_fin: Option<NaiveDate>,
    pub actif: bool,
    pub approuve_par: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Response DTOs ───────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminPartenariatListeResponse {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub organisation_denomination: String,
    pub type_partenariat: Option<String>,
    pub date_debut: Option<NaiveDate>,
    pub date_fin: Option<NaiveDate>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminPartenariatDetailResponse {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub organisation_denomination: String,
    pub type_partenariat: Option<String>,
    pub description: Option<String>,
    pub date_debut: Option<NaiveDate>,
    pub date_fin: Option<NaiveDate>,
    pub actif: bool,
    pub approuve_par: Option<Uuid>,
    pub approuve_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Request DTOs ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerPartenariatRequest {
    pub organisation_id: Uuid,
    pub type_partenariat: Option<String>,
    pub description: Option<String>,
    pub date_debut: Option<NaiveDate>,
    pub date_fin: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierPartenariatRequest {
    pub type_partenariat: Option<String>,
    pub description: Option<String>,
    pub date_debut: Option<NaiveDate>,
    pub date_fin: Option<NaiveDate>,
    pub actif: Option<bool>,
}

// ── Query Params ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminPartenariatQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub type_partenariat: Option<String>,
    pub organisation_id: Option<Uuid>,
    pub actif: Option<bool>,
}
