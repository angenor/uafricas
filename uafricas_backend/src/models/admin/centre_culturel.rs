use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────
pub const ADMIN_CENTRE_LISTE_COLONNES: &str =
    "cc.id, cc.nom, cc.type_centre::text AS type_centre, cc.ville, cc.actif, cc.created_at,
     p.nom AS pays_nom,
     (SELECT COUNT(*) FROM culture.membre_centre mc WHERE mc.centre_culturel_id = cc.id) AS nombre_membres,
     (SELECT COUNT(*) FROM culture.programmation_centre pc WHERE pc.centre_culturel_id = cc.id) AS nombre_programmations";

pub const ADMIN_CENTRE_DETAIL_COLONNES: &str =
    "cc.id, cc.nom, cc.slug, cc.type_centre::text AS type_centre, cc.description, cc.image_couverture_url,
     cc.pays_id, cc.ville, cc.adresse,
     cc.longitude::float8 AS longitude, cc.latitude::float8 AS latitude, cc.actif,
     cc.cree_par, cc.created_at, cc.updated_at,
     p.nom AS pays_nom,
     u.nom AS cree_par_nom, u.prenom AS cree_par_prenom";

pub const CENTRE_TRI_COLONNES: &[&str] = &[
    "created_at", "nom", "ville",
];

// ── Responses DTO ─────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminCentreListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub type_centre: String,
    pub ville: Option<String>,
    pub pays_nom: Option<String>,
    pub actif: bool,
    pub nombre_membres: Option<i64>,
    pub nombre_programmations: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminCentreDetailRow {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub type_centre: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub actif: bool,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub cree_par_nom: String,
    pub cree_par_prenom: String,
}

#[derive(Debug, Serialize)]
pub struct AdminCentreDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub type_centre: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub actif: bool,
    pub cree_par: Uuid,
    pub cree_par_nom: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub membres: Vec<AdminMembreCentreResponse>,
}

impl AdminCentreDetailRow {
    pub fn to_response(&self, membres: Vec<AdminMembreCentreResponse>) -> AdminCentreDetailResponse {
        AdminCentreDetailResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
            type_centre: self.type_centre.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            pays_id: self.pays_id,
            ville: self.ville.clone(),
            adresse: self.adresse.clone(),
            longitude: self.longitude,
            latitude: self.latitude,
            actif: self.actif,
            cree_par: self.cree_par,
            cree_par_nom: format!("{} {}", self.cree_par_prenom, self.cree_par_nom),
            created_at: self.created_at,
            updated_at: self.updated_at,
            pays_nom: self.pays_nom.clone(),
            membres,
        }
    }
}

// ── Membre centre ─────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminMembreCentreResponse {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub utilisateur_nom: String,
    pub utilisateur_prenom: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

// ── Requests DTO ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct CreerCentreRequest {
    pub nom: String,
    pub type_centre: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierCentreRequest {
    pub nom: Option<String>,
    pub type_centre: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub actif: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AjouterMembreRequest {
    pub utilisateur_id: Uuid,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct ModifierMembreRequest {
    pub role: String,
}

// ── Query Params ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct AdminCentreQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub pays_id: Option<Uuid>,
}
