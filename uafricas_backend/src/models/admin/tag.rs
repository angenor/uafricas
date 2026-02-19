use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub const ADMIN_TAG_LISTE_COLONNES: &str = "t.id, t.nom, t.slug, t.created_at";

pub const TAG_TRI_COLONNES: &[&str] = &["created_at", "nom", "slug"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminTagListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminTagDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub nombre_utilisations: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreerTagRequest {
    pub nom: String,
}

#[derive(Debug, Deserialize)]
pub struct ModifierTagRequest {
    pub nom: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminTagQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
}
