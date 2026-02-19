use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────────

pub const ADMIN_SESSION_LISTE_COLONNES: &str =
    "se.id, se.titre, se.etat::text AS etat, se.date_debut_prevue,
     se.demarre_at, se.termine_at, se.duree_secondes,
     se.nombre_participants_pic, se.created_at,
     sp.titre AS salle_privee_titre,
     s.titre AS salle_titre, s.langue_cible AS salle_langue,
     u.nom AS moderateur_nom, u.prenom AS moderateur_prenom";

pub const ADMIN_SESSION_DETAIL_COLONNES: &str =
    "se.id, se.titre, se.etat::text AS etat, se.salle_privee_id,
     se.moderateur_id, se.date_debut_prevue,
     se.demarre_at, se.termine_at, se.duree_secondes,
     se.max_participants, se.nombre_participants_pic,
     se.tableau_blanc_actif, se.noeud_id,
     se.cree_par, se.created_at, se.updated_at,
     sp.titre AS salle_privee_titre,
     s.id AS salle_id, s.titre AS salle_titre, s.langue_cible AS salle_langue,
     u.nom AS moderateur_nom, u.prenom AS moderateur_prenom,
     cr.nom AS createur_nom, cr.prenom AS createur_prenom";

pub const SESSION_TRI_COLONNES: &[&str] = &[
    "created_at", "date_debut_prevue", "demarre_at", "duree_secondes", "nombre_participants_pic",
];

// ── Structs liste ─────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSessionListeResponse {
    pub id: Uuid,
    pub titre: Option<String>,
    pub etat: Option<String>,
    pub date_debut_prevue: Option<DateTime<Utc>>,
    pub demarre_at: Option<DateTime<Utc>>,
    pub termine_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
    pub nombre_participants_pic: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub salle_privee_titre: Option<String>,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub moderateur_nom: Option<String>,
    pub moderateur_prenom: Option<String>,
}

// ── Structs détail ────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminSessionDetailRow {
    pub id: Uuid,
    pub titre: Option<String>,
    pub etat: Option<String>,
    pub salle_privee_id: Uuid,
    pub moderateur_id: Option<Uuid>,
    pub date_debut_prevue: Option<DateTime<Utc>>,
    pub demarre_at: Option<DateTime<Utc>>,
    pub termine_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
    pub max_participants: Option<i32>,
    pub nombre_participants_pic: Option<i32>,
    pub tableau_blanc_actif: Option<bool>,
    pub noeud_id: Option<String>,
    pub cree_par: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub salle_privee_titre: Option<String>,
    pub salle_id: Option<Uuid>,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub moderateur_nom: Option<String>,
    pub moderateur_prenom: Option<String>,
    pub createur_nom: Option<String>,
    pub createur_prenom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminSessionDetailResponse {
    pub id: Uuid,
    pub titre: Option<String>,
    pub etat: Option<String>,
    pub salle_privee_id: Uuid,
    pub salle_privee_titre: Option<String>,
    pub salle_id: Option<Uuid>,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub moderateur_id: Option<Uuid>,
    pub moderateur_nom: Option<String>,
    pub date_debut_prevue: Option<DateTime<Utc>>,
    pub demarre_at: Option<DateTime<Utc>>,
    pub termine_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
    pub max_participants: Option<i32>,
    pub nombre_participants_pic: Option<i32>,
    pub tableau_blanc_actif: Option<bool>,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub participants: Vec<AdminSessionParticipantResponse>,
}

impl AdminSessionDetailRow {
    pub fn to_response(self, participants: Vec<AdminSessionParticipantResponse>) -> AdminSessionDetailResponse {
        let cree_par_nom = match (&self.createur_prenom, &self.createur_nom) {
            (Some(p), Some(n)) => Some(format!("{} {}", p, n)),
            _ => None,
        };
        let moderateur_nom = match (&self.moderateur_prenom, &self.moderateur_nom) {
            (Some(p), Some(n)) => Some(format!("{} {}", p, n)),
            _ => None,
        };
        AdminSessionDetailResponse {
            id: self.id,
            titre: self.titre,
            etat: self.etat,
            salle_privee_id: self.salle_privee_id,
            salle_privee_titre: self.salle_privee_titre,
            salle_id: self.salle_id,
            salle_titre: self.salle_titre,
            salle_langue: self.salle_langue,
            moderateur_id: self.moderateur_id,
            moderateur_nom,
            date_debut_prevue: self.date_debut_prevue,
            demarre_at: self.demarre_at,
            termine_at: self.termine_at,
            duree_secondes: self.duree_secondes,
            max_participants: self.max_participants,
            nombre_participants_pic: self.nombre_participants_pic,
            tableau_blanc_actif: self.tableau_blanc_actif,
            cree_par_nom,
            created_at: self.created_at,
            updated_at: self.updated_at,
            participants,
        }
    }
}

// ── Participants ──────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSessionParticipantResponse {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub utilisateur_nom: Option<String>,
    pub utilisateur_prenom: Option<String>,
    pub role_session: Option<String>,
    pub rejoint_at: Option<DateTime<Utc>>,
    pub quitte_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
}

// ── Tableau blanc ─────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminTableauBlancResponse {
    pub id: Uuid,
    pub session_id: Uuid,
    pub donnees: Option<serde_json::Value>,
    pub version: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Query params ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminSessionQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub salle_id: Option<Uuid>,
    pub salle_privee_id: Option<Uuid>,
    pub moderateur_id: Option<Uuid>,
    pub date_debut: Option<String>,
    pub date_fin: Option<String>,
}
