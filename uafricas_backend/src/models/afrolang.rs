use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ══════════════════════════════════════════════════════════════════════════
// Constantes SQL
// ══════════════════════════════════════════════════════════════════════════

/// Colonnes de base pour afrolang.salle
pub const SALLE_COLONNES: &str =
    "s.id, s.titre, s.slug, s.description, s.image_couverture_url,
     s.langue_cible, s.moderateur_id, s.actif, s.cree_par,
     s.created_at, s.updated_at";

/// Colonnes de base pour afrolang.salle_privee (inclut code_acces pour usage interne)
pub const SALLE_PRIVEE_COLONNES: &str =
    "sp.id, sp.salle_id, sp.titre, sp.description, sp.code_acces,
     sp.image_couverture_url, sp.max_participants, sp.actif, sp.cree_par,
     sp.created_at, sp.updated_at";

/// Colonnes de base pour afrolang.session (cast enum → text)
pub const SESSION_COLONNES: &str =
    "ses.id, ses.salle_privee_id, ses.titre, ses.etat::TEXT,
     ses.moderateur_id, ses.date_debut_prevue, ses.demarre_at,
     ses.termine_at, ses.duree_secondes, ses.max_participants,
     ses.nombre_participants_pic, ses.tableau_blanc_actif,
     ses.noeud_id, ses.cree_par, ses.created_at, ses.updated_at";

// ══════════════════════════════════════════════════════════════════════════
// Structs FromRow
// ══════════════════════════════════════════════════════════════════════════

#[derive(Debug, FromRow)]
pub struct SalleRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub langue_cible: Option<String>,
    pub moderateur_id: Option<Uuid>,
    pub actif: bool,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs calcules (sous-requetes)
    #[sqlx(default)]
    pub nombre_salles_privees: Option<i64>,
    #[sqlx(default)]
    pub sessions_en_cours: Option<i64>,
    // JOINs moderateur
    #[sqlx(default)]
    pub moderateur_nom: Option<String>,
    #[sqlx(default)]
    pub moderateur_prenom: Option<String>,
    #[sqlx(default)]
    pub moderateur_photo: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct SallePriveeRow {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub code_acces: Option<String>,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
    pub actif: bool,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // JOINs createur
    #[sqlx(default)]
    pub createur_nom: Option<String>,
    #[sqlx(default)]
    pub createur_prenom: Option<String>,
    #[sqlx(default)]
    pub createur_photo: Option<String>,
    // JOINs salle parente
    #[sqlx(default)]
    pub salle_titre: Option<String>,
    #[sqlx(default)]
    pub salle_langue: Option<String>,
    // Sous-requete session en cours
    #[sqlx(default)]
    pub session_en_cours: Option<bool>,
}

#[derive(Debug, FromRow)]
pub struct SessionRow {
    pub id: Uuid,
    pub salle_privee_id: Uuid,
    pub titre: Option<String>,
    pub etat: String,
    pub moderateur_id: Option<Uuid>,
    pub date_debut_prevue: Option<DateTime<Utc>>,
    pub demarre_at: Option<DateTime<Utc>>,
    pub termine_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
    pub max_participants: Option<i32>,
    pub nombre_participants_pic: Option<i32>,
    pub tableau_blanc_actif: bool,
    pub noeud_id: Option<String>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct SessionParticipantRow {
    pub id: Uuid,
    pub session_id: Uuid,
    pub utilisateur_id: Uuid,
    pub role_session: String,
    pub rejoint_at: DateTime<Utc>,
    pub quitte_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
    // JOINs utilisateur
    #[sqlx(default)]
    pub nom: Option<String>,
    #[sqlx(default)]
    pub prenom: Option<String>,
    #[sqlx(default)]
    pub photo_url: Option<String>,
}

// ══════════════════════════════════════════════════════════════════════════
// DTOs Response (Serialize)
// ══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct SalleResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub langue_cible: Option<String>,
    pub actif: bool,
    pub nombre_salles_privees: i64,
    pub sessions_en_cours: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SalleDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub langue_cible: Option<String>,
    pub actif: bool,
    pub moderateur: Option<ModerateurResponse>,
    pub nombre_salles_privees: i64,
    pub sessions_en_cours: i64,
    pub salles_privees: Vec<SallePriveeResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ModerateurResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SallePriveeResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
    pub est_protegee: bool,
    pub actif: bool,
    pub createur: CreateurResponse,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub session_en_cours: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateurResponse {
    pub id: Uuid,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SallePriveeDetailResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
    pub est_protegee: bool,
    pub actif: bool,
    pub createur: CreateurResponse,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub session_en_cours: bool,
    pub sessions: Vec<SessionResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub salle_privee_id: Uuid,
    pub titre: Option<String>,
    pub etat: String,
    pub date_debut_prevue: Option<DateTime<Utc>>,
    pub demarre_at: Option<DateTime<Utc>>,
    pub termine_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
    pub max_participants: Option<i32>,
    pub nombre_participants_pic: Option<i32>,
    pub tableau_blanc_actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SessionDetailResponse {
    pub id: Uuid,
    pub salle_privee_id: Uuid,
    pub titre: Option<String>,
    pub etat: String,
    pub moderateur: Option<ModerateurResponse>,
    pub date_debut_prevue: Option<DateTime<Utc>>,
    pub demarre_at: Option<DateTime<Utc>>,
    pub termine_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
    pub max_participants: Option<i32>,
    pub nombre_participants_pic: Option<i32>,
    pub tableau_blanc_actif: bool,
    pub participants: Vec<ParticipantResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ParticipantResponse {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub photo_url: Option<String>,
    pub role_session: String,
    pub rejoint_at: DateTime<Utc>,
    pub quitte_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct AfrolangStatsResponse {
    pub total_salles: i64,
    pub total_salles_privees: i64,
    pub sessions_en_cours: i64,
    pub sessions_terminees: i64,
    pub total_participants_uniques: i64,
}

#[derive(Debug, Serialize)]
pub struct SalleListeResponse {
    pub salles: Vec<SalleResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct SallePriveeListeResponse {
    pub salles_privees: Vec<SallePriveeResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct SessionListeResponse {
    pub sessions: Vec<SessionResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ══════════════════════════════════════════════════════════════════════════
// Structs de requete (Deserialize)
// ══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct SalleFiltres {
    pub recherche: Option<String>,
    pub langue: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SallePriveeFiltres {
    pub recherche: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SessionFiltres {
    pub etat: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSallePriveeRequest {
    pub titre: String,
    pub description: Option<String>,
    pub code_acces: Option<String>,
    pub max_participants: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSalleRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub langue_cible: Option<String>,
    pub moderateur_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSallePriveeRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub code_acces: Option<String>,
    pub max_participants: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSessionRequest {
    pub titre: Option<String>,
    pub date_debut_prevue: Option<String>,
    pub max_participants: Option<i32>,
    pub tableau_blanc_actif: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RejoindreRequest {
    pub code_acces: Option<String>,
}

// ══════════════════════════════════════════════════════════════════════════
// Conversions Row → Response
// ══════════════════════════════════════════════════════════════════════════

impl SalleRow {
    pub fn to_response(&self) -> SalleResponse {
        SalleResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            langue_cible: self.langue_cible.clone(),
            actif: self.actif,
            nombre_salles_privees: self.nombre_salles_privees.unwrap_or(0),
            sessions_en_cours: self.sessions_en_cours.unwrap_or(0),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn to_moderateur(&self) -> Option<ModerateurResponse> {
        self.moderateur_id.map(|id| ModerateurResponse {
            id,
            nom: self.moderateur_nom.clone().unwrap_or_default(),
            prenom: self.moderateur_prenom.clone(),
            photo_url: self.moderateur_photo.clone(),
        })
    }
}

impl SallePriveeRow {
    pub fn to_response(&self) -> SallePriveeResponse {
        SallePriveeResponse {
            id: self.id,
            salle_id: self.salle_id,
            titre: self.titre.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            max_participants: self.max_participants,
            est_protegee: self.code_acces.is_some(),
            actif: self.actif,
            createur: CreateurResponse {
                id: self.cree_par,
                nom: self.createur_nom.clone(),
                prenom: self.createur_prenom.clone(),
                photo_url: self.createur_photo.clone(),
            },
            salle_titre: self.salle_titre.clone(),
            salle_langue: self.salle_langue.clone(),
            session_en_cours: self.session_en_cours.unwrap_or(false),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl SessionRow {
    pub fn to_response(&self) -> SessionResponse {
        SessionResponse {
            id: self.id,
            salle_privee_id: self.salle_privee_id,
            titre: self.titre.clone(),
            etat: self.etat.clone(),
            date_debut_prevue: self.date_debut_prevue,
            demarre_at: self.demarre_at,
            termine_at: self.termine_at,
            duree_secondes: self.duree_secondes,
            max_participants: self.max_participants,
            nombre_participants_pic: self.nombre_participants_pic,
            tableau_blanc_actif: self.tableau_blanc_actif,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl SessionParticipantRow {
    pub fn to_response(&self) -> ParticipantResponse {
        ParticipantResponse {
            id: self.id,
            utilisateur_id: self.utilisateur_id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            photo_url: self.photo_url.clone(),
            role_session: self.role_session.clone(),
            rejoint_at: self.rejoint_at,
            quitte_at: self.quitte_at,
            duree_secondes: self.duree_secondes,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Utilitaires
// ══════════════════════════════════════════════════════════════════════════

pub fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â', 'ä'], "a")
        .replace(['ù', 'û', 'ü'], "u")
        .replace(['î', 'ï'], "i")
        .replace(['ô', 'ö'], "o")
        .replace('ç', "c")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}
