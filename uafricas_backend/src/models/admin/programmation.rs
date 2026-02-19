use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────
pub const ADMIN_PROG_LISTE_COLONNES: &str =
    "pc.id, pc.titre, pc.lieu, pc.mode::text, pc.date_heure_debut, pc.date_heure_fin,
     pc.nombre_places, pc.created_at,
     cc.nom AS centre_nom";

pub const ADMIN_PROG_DETAIL_COLONNES: &str =
    "pc.id, pc.centre_culturel_id, pc.titre, pc.description, pc.lieu,
     pc.mode::text, pc.lien_en_ligne, pc.date_heure_debut, pc.date_heure_fin,
     pc.nombre_places, pc.cree_par, pc.created_at, pc.updated_at,
     cc.nom AS centre_nom,
     u.nom AS cree_par_nom, u.prenom AS cree_par_prenom";

pub const PROG_TRI_COLONNES: &[&str] = &[
    "created_at", "titre", "date_heure_debut",
];

// ── Responses DTO ─────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminProgListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub lieu: Option<String>,
    pub mode: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub nombre_places: Option<i32>,
    pub centre_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminProgDetailRow {
    pub id: Uuid,
    pub centre_culturel_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub lieu: Option<String>,
    pub mode: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub nombre_places: Option<i32>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub centre_nom: Option<String>,
    pub cree_par_nom: String,
    pub cree_par_prenom: String,
}

#[derive(Debug, Serialize)]
pub struct AdminProgDetailResponse {
    pub id: Uuid,
    pub centre_culturel_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub lieu: Option<String>,
    pub mode: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub nombre_places: Option<i32>,
    pub cree_par: Uuid,
    pub cree_par_nom: String,
    pub centre_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminProgDetailRow {
    pub fn to_response(&self) -> AdminProgDetailResponse {
        AdminProgDetailResponse {
            id: self.id,
            centre_culturel_id: self.centre_culturel_id,
            titre: self.titre.clone(),
            description: self.description.clone(),
            lieu: self.lieu.clone(),
            mode: self.mode.clone(),
            lien_en_ligne: self.lien_en_ligne.clone(),
            date_heure_debut: self.date_heure_debut,
            date_heure_fin: self.date_heure_fin,
            nombre_places: self.nombre_places,
            cree_par: self.cree_par,
            cree_par_nom: format!("{} {}", self.cree_par_prenom, self.cree_par_nom),
            centre_nom: self.centre_nom.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Requests DTO ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct CreerProgRequest {
    pub centre_culturel_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub lieu: Option<String>,
    pub mode: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub date_heure_debut: String,
    pub date_heure_fin: Option<String>,
    pub nombre_places: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierProgRequest {
    pub centre_culturel_id: Option<Uuid>,
    pub titre: Option<String>,
    pub description: Option<String>,
    pub lieu: Option<String>,
    pub mode: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub date_heure_debut: Option<String>,
    pub date_heure_fin: Option<String>,
    pub nombre_places: Option<i32>,
}

// ── Query Params ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct AdminProgQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub centre_culturel_id: Option<Uuid>,
    pub mode: Option<String>,
}
