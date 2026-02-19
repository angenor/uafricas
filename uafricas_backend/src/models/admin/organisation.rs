use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ────────────────────────────────────────────────

pub const ADMIN_ORGANISATION_COLONNES: &str =
    "o.id, o.denomination, o.slug, o.type_organisation, o.pays_id,
     o.email, o.telephone, o.adresse, o.ville, o.site_web,
     o.logo_url, o.description, o.document_legal_url, o.numero_registre,
     o.etat, o.cree_par, o.created_at, o.updated_at";

pub const ORGANISATION_TRI_COLONNES: &[&str] = &[
    "created_at", "updated_at", "denomination", "etat", "ville",
];

// ── Row structs ─────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminOrganisationRow {
    pub id: Uuid,
    pub denomination: String,
    pub slug: Option<String>,
    pub type_organisation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub adresse: Option<String>,
    pub ville: Option<String>,
    pub site_web: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
    pub document_legal_url: Option<String>,
    pub numero_registre: Option<String>,
    pub etat: String,
    pub cree_par: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Response DTOs ───────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminOrganisationListeResponse {
    pub id: Uuid,
    pub denomination: String,
    pub type_organisation: Option<String>,
    pub pays_nom: Option<String>,
    pub etat: String,
    pub ville: Option<String>,
    pub nombre_membres: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminOrganisationDetailResponse {
    pub id: Uuid,
    pub denomination: String,
    pub slug: Option<String>,
    pub type_organisation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub adresse: Option<String>,
    pub ville: Option<String>,
    pub site_web: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
    pub document_legal_url: Option<String>,
    pub numero_registre: Option<String>,
    pub etat: String,
    pub cree_par: Option<Uuid>,
    pub cree_par_nom: Option<String>,
    pub nombre_membres: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Request DTOs ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerOrganisationRequest {
    pub denomination: String,
    pub type_organisation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub adresse: Option<String>,
    pub ville: Option<String>,
    pub site_web: Option<String>,
    pub description: Option<String>,
    pub numero_registre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierOrganisationRequest {
    pub denomination: Option<String>,
    pub type_organisation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub adresse: Option<String>,
    pub ville: Option<String>,
    pub site_web: Option<String>,
    pub description: Option<String>,
    pub numero_registre: Option<String>,
    pub etat: Option<String>,
}

// ── Query Params ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminOrganisationQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub type_organisation: Option<String>,
    pub pays: Option<String>,
    pub etat: Option<String>,
}
