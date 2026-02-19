use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub const ADMIN_DOMAINE_LISTE_COLONNES: &str =
    "d.id, d.nom, d.slug, d.icone, d.actif, d.created_at";

pub const ADMIN_DOMAINE_DETAIL_COLONNES: &str =
    "d.id, d.nom, d.slug, d.description, d.icone, d.actif, d.created_at, d.updated_at";

pub const DOMAINE_TRI_COLONNES: &[&str] = &["created_at", "nom", "slug"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminDomaineListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub icone: Option<String>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminDomaineDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreerDomaineRequest {
    pub nom: String,
    pub description: Option<String>,
    pub icone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierDomaineRequest {
    pub nom: Option<String>,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub actif: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AdminDomaineQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
}
