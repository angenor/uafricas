use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ────────────────────────────────────────────

pub const ADMIN_INNOVATION_LISTE_COLONNES: &str =
    "i.id, i.titre, i.slug,
     i.etat::text AS etat,
     i.image_couverture_url, i.ville, i.nombre_vues,
     i.cree_par, i.created_at,
     d.nom AS domaine_nom,
     o.denomination AS organisation_nom,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const ADMIN_INNOVATION_DETAIL_COLONNES: &str =
    "i.id, i.titre, i.slug, i.description,
     i.etat::text AS etat,
     i.image_couverture_url, i.ville, i.nombre_vues,
     i.domaine_id, i.organisation_id, i.pays_id,
     i.cree_par, i.created_at, i.updated_at,
     d.nom AS domaine_nom,
     o.denomination AS organisation_nom,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email";

pub const INNOVATION_TRI_COLONNES: &[&str] = &["created_at", "titre", "nombre_vues", "etat"];

pub const INNOVATION_JOINS: &str =
    "LEFT JOIN shared.domaine_secteur d ON d.id = i.domaine_id
     LEFT JOIN iam.organisation o ON o.id = i.organisation_id
     LEFT JOIN shared.pays p ON p.id = i.pays_id
     JOIN iam.utilisateur u ON u.id = i.cree_par";

// ── Structs de listing ──────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminInnovationListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub etat: String,
    pub image_couverture_url: Option<String>,
    pub ville: Option<String>,
    pub nombre_vues: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub domaine_nom: Option<String>,
    pub organisation_nom: Option<String>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

// ── Structs de detail ───────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminInnovationDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub etat: String,
    pub image_couverture_url: Option<String>,
    pub ville: Option<String>,
    pub nombre_vues: i32,
    pub domaine_id: Option<Uuid>,
    pub organisation_id: Option<Uuid>,
    pub pays_id: Option<Uuid>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub domaine_nom: Option<String>,
    pub organisation_nom: Option<String>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
}

// ── Sous-entites ────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminInnovationMedia {
    pub id: Uuid,
    pub media_url: String,
    pub type_mime: Option<String>,
    pub ordre: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// ── Requetes (DTOs) ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerInnovationRequest {
    pub titre: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub organisation_id: Option<Uuid>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierInnovationRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub organisation_id: Option<Uuid>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatInnovationRequest {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct AjouterMediaInnovationRequest {
    pub media_url: String,
    pub type_mime: Option<String>,
    pub ordre: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct AdminInnovationQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub domaine_id: Option<String>,
    pub pays_id: Option<String>,
    pub organisation_id: Option<String>,
}
