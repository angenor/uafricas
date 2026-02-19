use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ────────────────────────────────────────────

pub const ADMIN_AFRICANTIVE_LISTE_COLONNES: &str =
    "af.id, af.titre, af.slug,
     af.etat::text AS etat,
     af.image_couverture_url, af.ville,
     af.cree_par, af.created_at,
     d.nom AS domaine_nom,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const ADMIN_AFRICANTIVE_DETAIL_COLONNES: &str =
    "af.id, af.titre, af.slug, af.description,
     af.etat::text AS etat,
     af.image_couverture_url, af.ville,
     af.domaine_id, af.pays_id,
     af.cree_par, af.created_at, af.updated_at,
     d.nom AS domaine_nom,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email";

pub const AFRICANTIVE_TRI_COLONNES: &[&str] = &["created_at", "titre", "etat"];

pub const AFRICANTIVE_JOINS: &str =
    "LEFT JOIN shared.domaine_secteur d ON d.id = af.domaine_id
     LEFT JOIN shared.pays p ON p.id = af.pays_id
     JOIN iam.utilisateur u ON u.id = af.cree_par";

// ── Structs de listing ──────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminAfricantiveListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub etat: String,
    pub image_couverture_url: Option<String>,
    pub ville: Option<String>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub domaine_nom: Option<String>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

// ── Structs de detail ───────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminAfricantiveDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub etat: String,
    pub image_couverture_url: Option<String>,
    pub ville: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub pays_id: Option<Uuid>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub domaine_nom: Option<String>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
}

// ── Requetes (DTOs) ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerAfricantiveRequest {
    pub titre: String,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierAfricantiveRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatAfricantiveRequest {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminAfricantiveQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub domaine_id: Option<String>,
    pub pays_id: Option<String>,
}
