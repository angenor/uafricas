use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub const ADMIN_FAVORI_LISTE_COLONNES: &str =
    "af.utilisateur_id, af.annonce_id, af.created_at,
     u.nom AS utilisateur_nom, u.prenom AS utilisateur_prenom, u.email AS utilisateur_email,
     a.titre AS annonce_titre, a.etat::text AS annonce_etat";

pub const FAVORI_TRI_COLONNES: &[&str] = &["created_at"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminFavoriListeResponse {
    pub utilisateur_id: Uuid,
    pub annonce_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub utilisateur_nom: String,
    pub utilisateur_prenom: String,
    pub utilisateur_email: String,
    pub annonce_titre: String,
    pub annonce_etat: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminFavoriStatsRow {
    pub annonce_id: Uuid,
    pub annonce_titre: String,
    pub nombre_favoris: i64,
}

#[derive(Debug, Deserialize)]
pub struct AdminFavoriQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub annonce_id: Option<String>,
}
