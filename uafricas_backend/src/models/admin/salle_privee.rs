use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────────

pub const ADMIN_SALLE_PRIVEE_LISTE_COLONNES: &str =
    "sp.id, sp.titre, sp.salle_id, sp.max_participants, sp.actif, sp.created_at,
     s.titre AS salle_titre, s.langue_cible AS salle_langue,
     u.nom AS createur_nom, u.prenom AS createur_prenom,
     (SELECT COUNT(*) FROM afrolang.session se WHERE se.salle_privee_id = sp.id AND 1=1) AS nombre_sessions";

pub const ADMIN_SALLE_PRIVEE_DETAIL_COLONNES: &str =
    "sp.id, sp.titre, sp.description, sp.salle_id, sp.code_acces,
     sp.image_couverture_url, sp.max_participants, sp.actif,
     sp.cree_par, sp.created_at, sp.updated_at,
     s.titre AS salle_titre, s.langue_cible AS salle_langue,
     u.nom AS createur_nom, u.prenom AS createur_prenom,
     (SELECT COUNT(*) FROM afrolang.session se WHERE se.salle_privee_id = sp.id AND 1=1) AS nombre_sessions";

pub const SALLE_PRIVEE_TRI_COLONNES: &[&str] = &[
    "created_at", "titre", "max_participants",
];

// ── Structs liste ─────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSallePriveeListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub salle_id: Uuid,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub max_participants: Option<i32>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub createur_nom: Option<String>,
    pub createur_prenom: Option<String>,
    pub nombre_sessions: Option<i64>,
}

// ── Structs détail ────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminSallePriveeDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub salle_id: Uuid,
    pub code_acces: Option<String>,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
    pub actif: bool,
    pub cree_par: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub createur_nom: Option<String>,
    pub createur_prenom: Option<String>,
    pub nombre_sessions: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct AdminSallePriveeDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub salle_id: Uuid,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub code_acces: Option<String>,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
    pub actif: bool,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nombre_sessions: i64,
}

impl AdminSallePriveeDetailRow {
    pub fn to_response(self) -> AdminSallePriveeDetailResponse {
        let cree_par_nom = match (&self.createur_prenom, &self.createur_nom) {
            (Some(p), Some(n)) => Some(format!("{} {}", p, n)),
            _ => None,
        };
        AdminSallePriveeDetailResponse {
            id: self.id,
            titre: self.titre,
            description: self.description,
            salle_id: self.salle_id,
            salle_titre: self.salle_titre,
            salle_langue: self.salle_langue,
            code_acces: self.code_acces,
            image_couverture_url: self.image_couverture_url,
            max_participants: self.max_participants,
            actif: self.actif,
            cree_par_nom,
            created_at: self.created_at,
            updated_at: self.updated_at,
            nombre_sessions: self.nombre_sessions.unwrap_or(0),
        }
    }
}

// ── Query params ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminSallePriveeQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub salle_id: Option<Uuid>,
}
