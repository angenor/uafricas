use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub const ADMIN_CATEGORIE_LISTE_COLONNES: &str =
    "c.id, c.nom, c.slug, c.contexte, c.parent_id, c.icone, c.ordre, c.actif, c.created_at";

pub const ADMIN_CATEGORIE_DETAIL_COLONNES: &str =
    "c.id, c.nom, c.slug, c.contexte, c.parent_id, c.description, c.icone, c.ordre, c.actif, c.created_at, c.updated_at";

pub const CATEGORIE_TRI_COLONNES: &[&str] = &["created_at", "nom", "contexte", "ordre"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminCategorieListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub contexte: Option<String>,
    pub parent_id: Option<Uuid>,
    pub icone: Option<String>,
    pub ordre: Option<i32>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminCategorieDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub contexte: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub ordre: Option<i32>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminCategorieEnfant {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub icone: Option<String>,
    pub ordre: Option<i32>,
    pub actif: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreerCategorieRequest {
    pub nom: String,
    pub contexte: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub ordre: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierCategorieRequest {
    pub nom: Option<String>,
    pub contexte: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub icone: Option<String>,
    pub ordre: Option<i32>,
    pub actif: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AdminCategorieQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub contexte: Option<String>,
    pub parent_id: Option<String>,
}
