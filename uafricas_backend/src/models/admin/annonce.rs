use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ────────────────────────────────────────────

pub const ADMIN_ANNONCE_LISTE_COLONNES: &str =
    "a.id, a.titre, a.slug,
     a.type_operation::text AS type_operation,
     a.etat::text AS etat,
     a.condition_article::text AS condition_article,
     a.prix::float8 AS prix, a.devise,
     a.ville, a.nombre_vues, a.quantite,
     a.cree_par, a.created_at,
     c.nom AS categorie_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom,
     (SELECT p.nom FROM marketplace.annonce_pays ap
      JOIN shared.pays p ON p.id = ap.pays_id
      WHERE ap.annonce_id = a.id LIMIT 1) AS pays_nom,
     (SELECT am.media_url FROM marketplace.annonce_media am
      WHERE am.annonce_id = a.id AND am.est_principale = TRUE
      LIMIT 1) AS photo_url";

pub const ADMIN_ANNONCE_DETAIL_COLONNES: &str =
    "a.id, a.titre, a.slug, a.description,
     a.type_operation::text AS type_operation,
     a.etat::text AS etat,
     a.condition_article::text AS condition_article,
     a.prix::float8 AS prix, a.devise,
     a.prix_negociable, a.ville, a.adresse,
     a.longitude::float8 AS longitude, a.latitude::float8 AS latitude,
     a.type_contact::text AS type_contact, a.contact_info,
     a.quantite, a.nombre_vues,
     a.cree_par, a.expire_at, a.created_at, a.updated_at,
     a.categorie_id,
     c.nom AS categorie_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email";

pub const ANNONCE_TRI_COLONNES: &[&str] = &["created_at", "titre", "prix", "nombre_vues", "etat"];

// ── Structs de listing ──────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminAnnonceListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub type_operation: String,
    pub etat: String,
    pub condition_article: Option<String>,
    pub prix: Option<f64>,
    pub devise: Option<String>,
    pub ville: Option<String>,
    pub nombre_vues: i32,
    pub quantite: Option<i32>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub categorie_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub pays_nom: Option<String>,
    pub photo_url: Option<String>,
}

// ── Structs de detail ───────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminAnnonceDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_operation: String,
    pub etat: String,
    pub condition_article: Option<String>,
    pub prix: Option<f64>,
    pub devise: Option<String>,
    pub prix_negociable: Option<bool>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub type_contact: Option<String>,
    pub contact_info: Option<String>,
    pub quantite: Option<i32>,
    pub nombre_vues: i32,
    pub cree_par: Uuid,
    pub expire_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub categorie_id: Option<Uuid>,
    pub categorie_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
}

// ── Sous-entites ────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminAnnoncePays {
    pub pays_id: Uuid,
    pub pays_nom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminAnnonceMedia {
    pub id: Uuid,
    pub media_url: String,
    pub type_mime: Option<String>,
    pub est_principale: Option<bool>,
    pub ordre: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// ── Requetes (DTOs) ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerAnnonceRequest {
    pub titre: String,
    pub description: String,
    pub type_operation: String,
    pub categorie_id: Option<Uuid>,
    pub condition_article: Option<String>,
    pub prix: Option<f64>,
    pub devise: Option<String>,
    pub prix_negociable: Option<bool>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub type_contact: Option<String>,
    pub contact_info: Option<String>,
    pub quantite: Option<i32>,
    pub etat: Option<String>,
    pub expire_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierAnnonceRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub type_operation: Option<String>,
    pub categorie_id: Option<Uuid>,
    pub condition_article: Option<String>,
    pub prix: Option<f64>,
    pub devise: Option<String>,
    pub prix_negociable: Option<bool>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub type_contact: Option<String>,
    pub contact_info: Option<String>,
    pub quantite: Option<i32>,
    pub expire_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatAnnonceRequest {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct AjouterPaysAnnonceRequest {
    pub pays_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct AjouterMediaAnnonceRequest {
    pub media_url: String,
    pub type_mime: Option<String>,
    pub est_principale: Option<bool>,
    pub ordre: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ReordonnerMediasRequest {
    pub ordres: Vec<MediaOrdre>,
}

#[derive(Debug, Deserialize)]
pub struct MediaOrdre {
    pub media_id: Uuid,
    pub ordre: i32,
}

#[derive(Debug, Deserialize)]
pub struct AdminAnnonceQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub type_operation: Option<String>,
    pub categorie_id: Option<String>,
    pub pays_id: Option<String>,
    pub cree_par: Option<String>,
}
