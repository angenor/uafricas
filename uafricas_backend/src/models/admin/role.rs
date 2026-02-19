use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Response DTOs ───────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminRoleListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub description: Option<String>,
    pub est_systeme: bool,
    pub nombre_utilisateurs: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminRoleDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub description: Option<String>,
    pub est_systeme: bool,
    pub permissions: Vec<PermissionInfo>,
    pub nombre_utilisateurs: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PermissionInfo {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub description: Option<String>,
    pub type_ressource: String,
    pub action: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PermissionListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub description: Option<String>,
    pub type_ressource: String,
    pub action: String,
    pub created_at: DateTime<Utc>,
}

// ── Row struct pour le detail ───────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminRoleRow {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
    pub description: Option<String>,
    pub est_systeme: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Request DTOs ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerRoleRequest {
    pub nom: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierRoleRequest {
    pub nom: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssignerPermissionsRequest {
    pub permission_ids: Vec<Uuid>,
}

// ── Query Params ────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminRoleQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
}

pub const ROLE_TRI_COLONNES: &[&str] = &[
    "created_at", "updated_at", "nom", "slug",
];
