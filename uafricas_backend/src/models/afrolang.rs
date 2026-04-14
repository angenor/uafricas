use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ══════════════════════════════════════════════════════════════════════════
// Enums (feature 005)
// ══════════════════════════════════════════════════════════════════════════
//
// Les noms de types SQL sont qualifiés `afrolang.xxx`. sqlx gère le mapping
// avec #[sqlx(type_name = "afrolang.xxx", rename_all = "snake_case")].

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.etat_proposition", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EtatProposition {
    EnAttente,
    Approuvee,
    Refusee,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.motif_salle_privee", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MotifSallePrivee {
    ApprentissageEnfants,
    ReseautageAdulte,
    EchangesGroupe,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.visibilite_salle_privee", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VisibiliteSallePrivee {
    Fermee,
    Visible,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.type_adhesion", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TypeAdhesion {
    Demande,
    Invitation,
    Abonne,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.etat_adhesion", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EtatAdhesion {
    EnAttente,
    Acceptee,
    Refusee,
    GroupeComplet,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.type_ressource", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TypeRessource {
    Fichier,
    LienExterne,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.etat_ressource", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EtatRessource {
    Publiee,
    EnAttenteValidation,
    Refusee,
}

// ══════════════════════════════════════════════════════════════════════════
// Constantes SQL
// ══════════════════════════════════════════════════════════════════════════

/// Colonnes de base pour afrolang.salle (feature 005 : plus de moderateur_id
/// direct ; remplacé par la table salle_moderateur).
pub const SALLE_COLONNES: &str =
    "s.id, s.titre, s.slug, s.description, s.image_couverture_url,
     s.langue_cible, s.langue_code, s.alphabet, s.dictionnaire_url,
     s.groupe_ethnique_id, s.actif, s.cree_par,
     s.created_at, s.updated_at, s.deleted_at";

/// Colonnes de base pour afrolang.salle_privee
pub const SALLE_PRIVEE_COLONNES: &str =
    "sp.id, sp.salle_id, sp.titre, sp.description, sp.code_acces,
     sp.image_couverture_url, sp.max_participants,
     sp.motif::TEXT AS motif, sp.declaration_adulte_at,
     sp.visibilite::TEXT AS visibilite, sp.archivee_at,
     sp.actif, sp.cree_par,
     sp.created_at, sp.updated_at, sp.deleted_at";

/// Colonnes de base pour afrolang.session
/// `salle_privee_id` et `salle_id` sont exclusifs (XOR via CHECK ck_session_contexte)
pub const SESSION_COLONNES: &str =
    "ses.id, ses.salle_privee_id, ses.salle_id, ses.titre, ses.etat::TEXT,
     ses.moderateur_id, ses.date_debut_prevue, ses.demarre_at,
     ses.termine_at, ses.duree_secondes, ses.max_participants,
     ses.nombre_participants_pic, ses.tableau_blanc_actif,
     ses.noeud_id, ses.cree_par, ses.created_at, ses.updated_at";

/// Colonnes de base pour afrolang.proposition_salle
pub const PROPOSITION_SALLE_COLONNES: &str =
    "ps.id, ps.nom_groupe_ethnique, ps.pays_id, ps.groupe_ethnique_id,
     ps.langue_cible, ps.description, ps.etat::TEXT AS etat,
     ps.motif_refus, ps.salle_id_creee, ps.propose_par,
     ps.decide_par, ps.decide_at,
     ps.created_at, ps.updated_at, ps.deleted_at";

/// Colonnes de base pour afrolang.salle_moderateur
pub const SALLE_MODERATEUR_COLONNES: &str =
    "sm.id, sm.salle_id, sm.utilisateur_id, sm.designe_par,
     sm.designe_at, sm.disponibilite, sm.actif, sm.retire_at,
     sm.created_at, sm.updated_at";

/// Colonnes de base pour afrolang.salle_privee_adhesion
pub const SALLE_PRIVEE_ADHESION_COLONNES: &str =
    "spa.id, spa.salle_privee_id, spa.utilisateur_id,
     spa.type::TEXT AS type_adhesion, spa.etat::TEXT AS etat_adhesion,
     spa.initiateur_id, spa.decideur_id, spa.decided_at,
     spa.created_at, spa.updated_at, spa.deleted_at";

/// Colonnes de base pour afrolang.ressource_salle
pub const RESSOURCE_SALLE_COLONNES: &str =
    "rs.id, rs.salle_id, rs.titre, rs.description,
     rs.type::TEXT AS type_ressource, rs.fichier_url, rs.lien_url,
     rs.etat::TEXT AS etat_ressource, rs.motif_refus,
     rs.ajoute_par, rs.valide_par, rs.valide_at,
     rs.created_at, rs.updated_at, rs.deleted_at";

/// Colonnes de base pour afrolang.message_session
pub const MESSAGE_SESSION_COLONNES: &str =
    "ms.id, ms.session_id, ms.auteur_id, ms.contenu,
     ms.created_at, ms.deleted_at";

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
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Uuid,
    pub actif: bool,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    // Champs calcules (sous-requetes)
    #[sqlx(default)]
    pub nombre_salles_privees: Option<i64>,
    #[sqlx(default)]
    pub sessions_en_cours: Option<i64>,
    #[sqlx(default)]
    pub nombre_moderateurs_attitres: Option<i64>,
    #[sqlx(default)]
    pub ressources_count: Option<i64>,
    // JOIN groupe_ethnique + pays
    #[sqlx(default)]
    pub groupe_ethnique_nom: Option<String>,
    #[sqlx(default)]
    pub fiche_pays_id: Option<Uuid>,
    #[sqlx(default)]
    pub pays_nom: Option<String>,
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
    pub motif: String,
    pub declaration_adulte_at: DateTime<Utc>,
    pub visibilite: String,
    pub archivee_at: Option<DateTime<Utc>>,
    pub actif: bool,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
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
    pub salle_privee_id: Option<Uuid>,
    pub salle_id: Option<Uuid>,
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

#[derive(Debug, FromRow)]
pub struct PropositionSalleRow {
    pub id: Uuid,
    pub nom_groupe_ethnique: String,
    pub pays_id: Option<Uuid>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub langue_cible: Option<String>,
    pub description: Option<String>,
    pub etat: String,
    pub motif_refus: Option<String>,
    pub salle_id_creee: Option<Uuid>,
    pub propose_par: Uuid,
    pub decide_par: Option<Uuid>,
    pub decide_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    // JOINs auteur
    #[sqlx(default)]
    pub proposant_nom: Option<String>,
    #[sqlx(default)]
    pub proposant_prenom: Option<String>,
    #[sqlx(default)]
    pub proposant_email: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct SalleModerateurRow {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub utilisateur_id: Uuid,
    pub designe_par: Uuid,
    pub designe_at: DateTime<Utc>,
    pub disponibilite: Option<String>,
    pub actif: bool,
    pub retire_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // JOINs utilisateur
    #[sqlx(default)]
    pub utilisateur_nom: Option<String>,
    #[sqlx(default)]
    pub utilisateur_prenom: Option<String>,
    #[sqlx(default)]
    pub utilisateur_photo: Option<String>,
    #[sqlx(default)]
    pub utilisateur_email: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct SallePriveeAdhesionRow {
    pub id: Uuid,
    pub salle_privee_id: Uuid,
    pub utilisateur_id: Uuid,
    pub type_adhesion: String,
    pub etat_adhesion: String,
    pub initiateur_id: Uuid,
    pub decideur_id: Option<Uuid>,
    pub decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    // JOINs utilisateur concerné
    #[sqlx(default)]
    pub utilisateur_nom: Option<String>,
    #[sqlx(default)]
    pub utilisateur_prenom: Option<String>,
    #[sqlx(default)]
    pub utilisateur_photo: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct RessourceSalleRow {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub type_ressource: String,
    pub fichier_url: Option<String>,
    pub lien_url: Option<String>,
    pub etat_ressource: String,
    pub motif_refus: Option<String>,
    pub ajoute_par: Uuid,
    pub valide_par: Option<Uuid>,
    pub valide_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    // JOINs auteur
    #[sqlx(default)]
    pub auteur_nom: Option<String>,
    #[sqlx(default)]
    pub auteur_prenom: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct MessageSessionRow {
    pub id: Uuid,
    pub session_id: Uuid,
    pub auteur_id: Uuid,
    pub contenu: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    // JOINs auteur
    #[sqlx(default)]
    pub auteur_nom: Option<String>,
    #[sqlx(default)]
    pub auteur_prenom: Option<String>,
    #[sqlx(default)]
    pub auteur_photo: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct GroupeEthniqueResume {
    pub id: Uuid,
    pub nom: String,
    pub fiche_pays_id: Uuid,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub salle_id: Option<Uuid>,
    pub salle_slug: Option<String>,
    pub salle_active: Option<bool>,
    pub proposition_en_attente: Option<bool>,
}

/// Colonnes pour le résumé « groupe ethnique + salle » (T023)
pub const GROUPE_ETHNIQUE_RESUME_COLONNES: &str =
    "ge.id, ge.nom, ge.fiche_pays_id,
     fp.pays_id AS pays_id, p.nom AS pays_nom,
     s.id AS salle_id, s.slug AS salle_slug,
     (s.id IS NOT NULL AND s.actif = TRUE) AS salle_active,
     EXISTS(
         SELECT 1 FROM afrolang.proposition_salle ps
         WHERE ps.groupe_ethnique_id = ge.id
           AND ps.etat = 'en_attente'
           AND ps.deleted_at IS NULL
     ) AS proposition_en_attente";

// ══════════════════════════════════════════════════════════════════════════
// DTOs Response (Serialize)
// ══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct GroupeEthniqueResumeResponse {
    pub id: Uuid,
    pub nom: String,
    pub fiche_pays_id: Uuid,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub salle_id: Option<Uuid>,
    pub salle_slug: Option<String>,
    pub salle_active: bool,
    pub proposition_en_attente: bool,
}

#[derive(Debug, Serialize)]
pub struct GroupeEthniqueLightResponse {
    pub id: Uuid,
    pub nom: String,
    pub fiche_pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SalleResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Uuid,
    pub groupe_ethnique: Option<GroupeEthniqueLightResponse>,
    pub actif: bool,
    pub nombre_salles_privees: i64,
    pub sessions_en_cours: i64,
    pub nombre_moderateurs_attitres: i64,
    pub ressources_count: i64,
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
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Uuid,
    pub groupe_ethnique: Option<GroupeEthniqueLightResponse>,
    pub actif: bool,
    pub moderateurs_attitres: Vec<ModerateurAttitreResponse>,
    pub nombre_salles_privees: i64,
    pub sessions_en_cours: i64,
    pub ressources_count: i64,
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
pub struct ModerateurAttitreResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub photo_url: Option<String>,
    pub email: Option<String>,
    pub disponibilite: Option<String>,
    pub designe_at: DateTime<Utc>,
    pub actif: bool,
}

#[derive(Debug, Serialize)]
pub struct SallePriveeResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
    pub motif: String,
    pub declaration_adulte_at: DateTime<Utc>,
    pub visibilite: String,
    pub archivee_at: Option<DateTime<Utc>>,
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
    pub motif: String,
    pub declaration_adulte_at: DateTime<Utc>,
    pub visibilite: String,
    pub archivee_at: Option<DateTime<Utc>>,
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
    pub salle_privee_id: Option<Uuid>,
    pub salle_id: Option<Uuid>,
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
    pub salle_privee_id: Option<Uuid>,
    pub salle_id: Option<Uuid>,
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

#[derive(Debug, Serialize)]
pub struct GroupeEthniqueListeResponse {
    pub groupes: Vec<GroupeEthniqueResumeResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Propositions de salles ──────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PropositionSalleResponse {
    pub id: Uuid,
    pub nom_groupe_ethnique: String,
    pub pays_id: Option<Uuid>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub langue_cible: Option<String>,
    pub description: Option<String>,
    pub etat: String,
    pub motif_refus: Option<String>,
    pub salle_id_creee: Option<Uuid>,
    pub propose_par: Uuid,
    pub decide_par: Option<Uuid>,
    pub decide_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Adhésions ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdhesionResponse {
    pub id: Uuid,
    pub salle_privee_id: Uuid,
    pub utilisateur_id: Uuid,
    pub utilisateur_nom: Option<String>,
    pub utilisateur_prenom: Option<String>,
    pub utilisateur_photo: Option<String>,
    pub type_adhesion: String,
    pub etat: String,
    pub initiateur_id: Uuid,
    pub decideur_id: Option<Uuid>,
    pub decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Ressources ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct RessourceSalleResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub type_ressource: String,
    pub fichier_url: Option<String>,
    pub lien_url: Option<String>,
    pub etat: String,
    pub motif_refus: Option<String>,
    pub ajoute_par: Uuid,
    pub auteur_nom: Option<String>,
    pub auteur_prenom: Option<String>,
    pub valide_par: Option<Uuid>,
    pub valide_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Messages de session ─────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct MessageSessionResponse {
    pub id: Uuid,
    pub session_id: Uuid,
    pub auteur_id: Uuid,
    pub auteur_nom: Option<String>,
    pub auteur_prenom: Option<String>,
    pub auteur_photo: Option<String>,
    pub contenu: String,
    pub created_at: DateTime<Utc>,
}

// ══════════════════════════════════════════════════════════════════════════
// Structs de requete (Deserialize)
// ══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct SalleFiltres {
    pub recherche: Option<String>,
    pub langue: Option<String>,
    pub langue_code: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
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
pub struct GroupeEthniqueFiltres {
    pub q: Option<String>,
    pub pays_id: Option<Uuid>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSallePriveeRequest {
    pub titre: String,
    pub description: Option<String>,
    pub code_acces: Option<String>,
    pub max_participants: Option<i32>,
    pub motif: Option<String>,
    #[serde(default)]
    pub declaration_adulte: bool,
    pub visibilite: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSalleRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
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

#[derive(Debug, Deserialize)]
pub struct CreerPropositionRequest {
    pub nom_groupe_ethnique: String,
    pub pays_id: Option<Uuid>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub langue_cible: Option<String>,
    pub description: Option<String>,
}

/// Requête de transfert du rôle de modérateur de session (US3)
#[derive(Debug, Deserialize)]
pub struct TransfererModerationRequest {
    pub destinataire_id: Uuid,
}

// ── US5 : Visibilité, adhésions, invitations ─────────────────────────────

/// Changement de visibilité d'une salle privée (créateur uniquement)
#[derive(Debug, Deserialize)]
pub struct ChangerVisibiliteRequest {
    pub visibilite: String,
}

/// Demande d'adhésion à une salle privée visible (aucun payload)
#[derive(Debug, Deserialize, Default)]
pub struct DemanderAdhesionRequest {}

/// Invitation d'un membre par le créateur (toutes salles privées)
#[derive(Debug, Deserialize)]
pub struct InviterMembreRequest {
    pub utilisateur_id: Uuid,
}

/// Décision sur une demande ou invitation (acceptée / refusée)
#[derive(Debug, Deserialize)]
pub struct DecisionAdhesionRequest {
    pub decision: String,
}

/// Modification de la limite de participants (FR-036)
#[derive(Debug, Deserialize)]
pub struct ModifierMaxParticipantsRequest {
    pub max_participants: i32,
}

// ── US6 : Messagerie et ressources ───────────────────────────────────────

/// Création d'un message de session
#[derive(Debug, Deserialize)]
pub struct CreerMessageRequest {
    pub contenu: String,
}

/// Filtres d'historique de messages
#[derive(Debug, Deserialize)]
pub struct MessagesFiltres {
    pub since: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
}

/// Soumission d'un lien externe par un membre (ressource modérée)
#[derive(Debug, Deserialize)]
pub struct CreerRessourceLienRequest {
    pub titre: String,
    pub description: Option<String>,
    pub lien_url: String,
}

/// Motif de refus d'un lien externe (admin/modérateur attitré)
#[derive(Debug, Deserialize)]
pub struct RefuserLienRequest {
    pub motif_refus: String,
}

/// DTO détail d'une proposition côté admin (avec proposant et conflits)
#[derive(Debug, Serialize)]
pub struct PropositionSalleAdminResponse {
    pub id: Uuid,
    pub nom_groupe_ethnique: String,
    pub pays_id: Option<Uuid>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub langue_cible: Option<String>,
    pub description: Option<String>,
    pub etat: String,
    pub motif_refus: Option<String>,
    pub salle_id_creee: Option<Uuid>,
    pub propose_par: Uuid,
    pub proposant_nom_complet: Option<String>,
    pub proposant_email: Option<String>,
    pub decide_par: Option<Uuid>,
    pub decide_at: Option<DateTime<Utc>>,
    pub salle_existante_id: Option<Uuid>,
    pub proposition_doublon_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ══════════════════════════════════════════════════════════════════════════
// Conversions Row → Response
// ══════════════════════════════════════════════════════════════════════════

impl SalleRow {
    pub fn to_groupe_ethnique_light(&self) -> Option<GroupeEthniqueLightResponse> {
        self.groupe_ethnique_nom
            .as_ref()
            .map(|nom| GroupeEthniqueLightResponse {
                id: self.groupe_ethnique_id,
                nom: nom.clone(),
                fiche_pays_id: self.fiche_pays_id,
                pays_nom: self.pays_nom.clone(),
            })
    }

    pub fn to_response(&self) -> SalleResponse {
        SalleResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            langue_cible: self.langue_cible.clone(),
            langue_code: self.langue_code.clone(),
            alphabet: self.alphabet.clone(),
            dictionnaire_url: self.dictionnaire_url.clone(),
            groupe_ethnique_id: self.groupe_ethnique_id,
            groupe_ethnique: self.to_groupe_ethnique_light(),
            actif: self.actif,
            nombre_salles_privees: self.nombre_salles_privees.unwrap_or(0),
            sessions_en_cours: self.sessions_en_cours.unwrap_or(0),
            nombre_moderateurs_attitres: self.nombre_moderateurs_attitres.unwrap_or(0),
            ressources_count: self.ressources_count.unwrap_or(0),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
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
            motif: self.motif.clone(),
            declaration_adulte_at: self.declaration_adulte_at,
            visibilite: self.visibilite.clone(),
            archivee_at: self.archivee_at,
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
            salle_id: self.salle_id,
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

impl GroupeEthniqueResume {
    pub fn to_response(&self) -> GroupeEthniqueResumeResponse {
        GroupeEthniqueResumeResponse {
            id: self.id,
            nom: self.nom.clone(),
            fiche_pays_id: self.fiche_pays_id,
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            salle_id: self.salle_id,
            salle_slug: self.salle_slug.clone(),
            salle_active: self.salle_active.unwrap_or(false),
            proposition_en_attente: self.proposition_en_attente.unwrap_or(false),
        }
    }
}

impl SalleModerateurRow {
    pub fn to_response(&self) -> ModerateurAttitreResponse {
        ModerateurAttitreResponse {
            id: self.id,
            salle_id: self.salle_id,
            utilisateur_id: self.utilisateur_id,
            nom: self.utilisateur_nom.clone(),
            prenom: self.utilisateur_prenom.clone(),
            photo_url: self.utilisateur_photo.clone(),
            email: self.utilisateur_email.clone(),
            disponibilite: self.disponibilite.clone(),
            designe_at: self.designe_at,
            actif: self.actif,
        }
    }
}

impl PropositionSalleRow {
    pub fn to_response(&self) -> PropositionSalleResponse {
        PropositionSalleResponse {
            id: self.id,
            nom_groupe_ethnique: self.nom_groupe_ethnique.clone(),
            pays_id: self.pays_id,
            groupe_ethnique_id: self.groupe_ethnique_id,
            langue_cible: self.langue_cible.clone(),
            description: self.description.clone(),
            etat: self.etat.clone(),
            motif_refus: self.motif_refus.clone(),
            salle_id_creee: self.salle_id_creee,
            propose_par: self.propose_par,
            decide_par: self.decide_par,
            decide_at: self.decide_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl SallePriveeAdhesionRow {
    pub fn to_response(&self) -> AdhesionResponse {
        AdhesionResponse {
            id: self.id,
            salle_privee_id: self.salle_privee_id,
            utilisateur_id: self.utilisateur_id,
            utilisateur_nom: self.utilisateur_nom.clone(),
            utilisateur_prenom: self.utilisateur_prenom.clone(),
            utilisateur_photo: self.utilisateur_photo.clone(),
            type_adhesion: self.type_adhesion.clone(),
            etat: self.etat_adhesion.clone(),
            initiateur_id: self.initiateur_id,
            decideur_id: self.decideur_id,
            decided_at: self.decided_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl RessourceSalleRow {
    pub fn to_response(&self) -> RessourceSalleResponse {
        RessourceSalleResponse {
            id: self.id,
            salle_id: self.salle_id,
            titre: self.titre.clone(),
            description: self.description.clone(),
            type_ressource: self.type_ressource.clone(),
            fichier_url: self.fichier_url.clone(),
            lien_url: self.lien_url.clone(),
            etat: self.etat_ressource.clone(),
            motif_refus: self.motif_refus.clone(),
            ajoute_par: self.ajoute_par,
            auteur_nom: self.auteur_nom.clone(),
            auteur_prenom: self.auteur_prenom.clone(),
            valide_par: self.valide_par,
            valide_at: self.valide_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl MessageSessionRow {
    pub fn to_response(&self) -> MessageSessionResponse {
        MessageSessionResponse {
            id: self.id,
            session_id: self.session_id,
            auteur_id: self.auteur_id,
            auteur_nom: self.auteur_nom.clone(),
            auteur_prenom: self.auteur_prenom.clone(),
            auteur_photo: self.auteur_photo.clone(),
            contenu: self.contenu.clone(),
            created_at: self.created_at,
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
