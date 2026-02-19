use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub const ADMIN_MEDIA_LISTE_COLONNES: &str =
    "m.id, m.nom_original, m.url_publique, m.type_mime, m.taille_octets, m.created_at";

pub const ADMIN_MEDIA_DETAIL_COLONNES: &str =
    "m.id, m.nom_original, m.chemin_stockage, m.url_publique, m.type_mime,
     m.taille_octets, m.largeur, m.hauteur, m.duree_secondes, m.uploaded_by, m.created_at";

pub const MEDIA_TRI_COLONNES: &[&str] = &["created_at", "nom_original", "type_mime", "taille_octets"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminMediaListeResponse {
    pub id: Uuid,
    pub nom_original: String,
    pub url_publique: Option<String>,
    pub type_mime: Option<String>,
    pub taille_octets: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminMediaDetailResponse {
    pub id: Uuid,
    pub nom_original: String,
    pub chemin_stockage: String,
    pub url_publique: Option<String>,
    pub type_mime: Option<String>,
    pub taille_octets: Option<i64>,
    pub largeur: Option<i32>,
    pub hauteur: Option<i32>,
    pub duree_secondes: Option<i32>,
    pub uploaded_by: Option<Uuid>,
    pub uploaded_by_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminMediaDetailRow {
    pub id: Uuid,
    pub nom_original: String,
    pub chemin_stockage: String,
    pub url_publique: Option<String>,
    pub type_mime: Option<String>,
    pub taille_octets: Option<i64>,
    pub largeur: Option<i32>,
    pub hauteur: Option<i32>,
    pub duree_secondes: Option<i32>,
    pub uploaded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AdminMediaQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub type_mime: Option<String>,
}
