use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ─────────────────────────────────────────────

pub const AUDIT_LISTE_COLONNES: &str =
    "a.id, a.action, a.schema_name, a.table_name, a.record_id,
     a.ip_address::TEXT AS ip_address,
     a.created_at,
     u.nom || ' ' || u.prenom AS utilisateur_nom,
     a.utilisateur_id";

pub const AUDIT_DETAIL_COLONNES: &str =
    "a.id, a.action, a.schema_name, a.table_name, a.record_id,
     a.ancien_etat, a.nouvel_etat,
     a.ip_address::TEXT AS ip_address, a.user_agent,
     a.created_at,
     a.utilisateur_id,
     u.nom || ' ' || u.prenom AS utilisateur_nom,
     u.email AS utilisateur_email";

pub const AUDIT_TRI_COLONNES: &[&str] = &[
    "created_at", "action", "schema_name", "table_name",
];

// ── Row & Response (liste) ──────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AuditListeResponse {
    pub id: Uuid,
    pub action: String,
    pub schema_name: String,
    pub table_name: String,
    pub record_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub utilisateur_nom: Option<String>,
    pub utilisateur_id: Option<Uuid>,
}

// ── Row & Response (detail) ─────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AuditDetailRow {
    pub id: Uuid,
    pub action: String,
    pub schema_name: String,
    pub table_name: String,
    pub record_id: Option<Uuid>,
    pub ancien_etat: Option<serde_json::Value>,
    pub nouvel_etat: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub utilisateur_id: Option<Uuid>,
    pub utilisateur_nom: Option<String>,
    pub utilisateur_email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuditDetailResponse {
    pub id: Uuid,
    pub action: String,
    pub schema_name: String,
    pub table_name: String,
    pub record_id: Option<Uuid>,
    pub ancien_etat: Option<serde_json::Value>,
    pub nouvel_etat: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub utilisateur_id: Option<Uuid>,
    pub utilisateur_nom: Option<String>,
    pub utilisateur_email: Option<String>,
}

impl AuditDetailRow {
    pub fn to_response(self) -> AuditDetailResponse {
        AuditDetailResponse {
            id: self.id,
            action: self.action,
            schema_name: self.schema_name,
            table_name: self.table_name,
            record_id: self.record_id,
            ancien_etat: self.ancien_etat,
            nouvel_etat: self.nouvel_etat,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            created_at: self.created_at,
            utilisateur_id: self.utilisateur_id,
            utilisateur_nom: self.utilisateur_nom,
            utilisateur_email: self.utilisateur_email,
        }
    }
}

// ── Query Params ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AuditQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub action: Option<String>,
    pub utilisateur_id: Option<Uuid>,
    pub schema_name: Option<String>,
    pub table_name: Option<String>,
    pub ip_address: Option<String>,
    pub date_debut: Option<String>,
    pub date_fin: Option<String>,
    pub recherche: Option<String>,
}
