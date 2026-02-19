use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub const ADMIN_SPECIALITE_LISTE_COLONNES: &str = "s.id, s.nom, s.slug";

pub const SPECIALITE_TRI_COLONNES: &[&str] = &["nom", "slug"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSpecialiteListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct AdminSpecialiteDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub nombre_utilisateurs: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreerSpecialiteRequest {
    pub nom: String,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSpecialiteRequest {
    pub nom: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminSpecialiteQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
}
