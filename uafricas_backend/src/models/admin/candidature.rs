use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────
pub const ADMIN_CANDIDATURE_LISTE_COLONNES: &str =
    "c.id, c.statut::TEXT as statut,
     p.titre AS programme_titre,
     u.nom || ' ' || u.prenom AS candidat_nom, u.email AS candidat_email,
     c.created_at";

pub const ADMIN_CANDIDATURE_DETAIL_COLONNES: &str =
    "c.id, c.programme_id, p.titre AS programme_titre,
     c.candidat_id, u.nom AS candidat_nom, u.prenom AS candidat_prenom,
     u.email AS candidat_email, u.photo_url AS candidat_photo_url,
     c.lettre_motivation, c.cv_url,
     c.statut::TEXT as statut, c.notes_internes,
     c.traite_par, u_traite.nom || ' ' || u_traite.prenom AS traite_par_nom,
     c.created_at, c.updated_at";

pub const CANDIDATURE_TRI_COLONNES: &[&str] = &[
    "created_at", "statut",
];

// ── Row (lecture BDD) ─────────────────────────────────────
#[derive(Debug, FromRow)]
pub struct AdminCandidatureDetailRow {
    pub id: Uuid,
    pub programme_id: Uuid,
    pub programme_titre: String,
    pub candidat_id: Uuid,
    pub candidat_nom: String,
    pub candidat_prenom: String,
    pub candidat_email: String,
    pub candidat_photo_url: Option<String>,
    pub lettre_motivation: Option<String>,
    pub cv_url: Option<String>,
    pub statut: String,
    pub notes_internes: Option<String>,
    pub traite_par: Option<Uuid>,
    pub traite_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ── Responses DTO ─────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminCandidatureListeResponse {
    pub id: Uuid,
    pub statut: String,
    pub programme_titre: String,
    pub candidat_nom: String,
    pub candidat_email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminCandidatureDetailResponse {
    pub id: Uuid,
    pub programme_id: Uuid,
    pub programme_titre: String,
    pub candidat_id: Uuid,
    pub candidat_nom: String,
    pub candidat_prenom: String,
    pub candidat_email: String,
    pub candidat_photo_url: Option<String>,
    pub lettre_motivation: Option<String>,
    pub cv_url: Option<String>,
    pub statut: String,
    pub notes_internes: Option<String>,
    pub traite_par: Option<Uuid>,
    pub traite_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl AdminCandidatureDetailRow {
    pub fn to_response(&self) -> AdminCandidatureDetailResponse {
        AdminCandidatureDetailResponse {
            id: self.id,
            programme_id: self.programme_id,
            programme_titre: self.programme_titre.clone(),
            candidat_id: self.candidat_id,
            candidat_nom: self.candidat_nom.clone(),
            candidat_prenom: self.candidat_prenom.clone(),
            candidat_email: self.candidat_email.clone(),
            candidat_photo_url: self.candidat_photo_url.clone(),
            lettre_motivation: self.lettre_motivation.clone(),
            cv_url: self.cv_url.clone(),
            statut: self.statut.clone(),
            notes_internes: self.notes_internes.clone(),
            traite_par: self.traite_par,
            traite_par_nom: self.traite_par_nom.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Requests DTO ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct ChangerStatutCandidatureRequest {
    pub statut: String,
    pub notes_internes: Option<String>,
}

// ── Query Params ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct AdminCandidatureQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub statut: Option<String>,
    pub programme_id: Option<Uuid>,
}
