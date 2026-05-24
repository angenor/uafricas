use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ══════════════════════════════════════════════════════════════════════════
// Enums (feature 005 — conservés après refonte 001-afrolang-salles-refonte)
// ══════════════════════════════════════════════════════════════════════════
//
// Les enums `EtatProposition`, `MotifSallePrivee`, `VisibiliteSallePrivee`,
// `TypeAdhesion`, `EtatAdhesion` ont été supprimés (tables/types SQL retirés).

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

/// Statut d'une proposition communautaire de salle publique
/// (feature 001-admin-salles-publiques).
#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.statut_proposition_salle", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PropositionStatut {
    EnAttente,
    Validee,
    Rejetee,
    Retiree,
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
     s.created_at, s.updated_at, s.deleted_at,
     s.desactivee_admin_at, s.desactivee_par, s.motif_desactivation,
     s.reactivee_at, s.reactivee_par, s.commentaire_reactivation";

/// Colonnes de base pour afrolang.salle_privee (refonte 2026-04)
///
/// Attention : `code_acces_hash` est inclus pour les chargements serveur
/// (vérification du code) mais ne DOIT JAMAIS être exposé dans un DTO public.
pub const SALLE_PRIVEE_COLONNES: &str =
    "sp.id, sp.salle_id, sp.titre, sp.description, sp.code_acces_hash,
     sp.image_couverture_url, sp.max_participants,
     sp.archivee_at, sp.actif, sp.cree_par,
     sp.created_at, sp.updated_at, sp.deleted_at";

/// Colonnes de base pour afrolang.session
/// `salle_privee_id` et `salle_id` sont exclusifs (XOR via CHECK ck_session_contexte)
pub const SESSION_COLONNES: &str =
    "ses.id, ses.salle_privee_id, ses.salle_id, ses.titre, ses.etat::TEXT,
     ses.moderateur_id, ses.date_debut_prevue, ses.demarre_at,
     ses.termine_at, ses.duree_secondes, ses.max_participants,
     ses.nombre_participants_pic, ses.tableau_blanc_actif,
     ses.noeud_id, ses.cree_par, ses.created_at, ses.updated_at";

/// Colonnes de base pour afrolang.salle_moderateur
pub const SALLE_MODERATEUR_COLONNES: &str =
    "sm.id, sm.salle_id, sm.utilisateur_id, sm.designe_par,
     sm.designe_at, sm.disponibilite, sm.actif, sm.retire_at,
     sm.created_at, sm.updated_at";

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

/// Colonnes de base pour afrolang.proposition_salle
/// (feature 001-admin-salles-publiques)
pub const COLONNES_PROPOSITION: &str =
    "ps.id, ps.auteur_id, ps.titre, ps.description, ps.justification,
     ps.langue_cible, ps.langue_code, ps.groupe_ethnique_id, ps.pays_origine_ids,
     ps.statut, ps.decideur, ps.decide_at, ps.commentaire_decision,
     ps.salle_id_creee, ps.created_at, ps.updated_at";

/// Colonnes de base pour afrolang.salle_administrateur
/// (feature 001-admin-salles-publiques)
pub const COLONNES_SALLE_ADMIN: &str =
    "sa.id, sa.salle_id, sa.utilisateur_id, sa.nomme_par, sa.nomme_at,
     sa.actif, sa.revoque_at, sa.revoque_par, sa.motif_revocation,
     sa.suspendu_at, sa.motif_suspension, sa.created_at, sa.updated_at";

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
    // Désactivation administrative (feature 001-ressources-fermeture-session)
    #[sqlx(default)]
    pub desactivee_admin_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub desactivee_par: Option<Uuid>,
    #[sqlx(default)]
    pub motif_desactivation: Option<String>,
    #[sqlx(default)]
    pub reactivee_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub reactivee_par: Option<Uuid>,
    #[sqlx(default)]
    pub commentaire_reactivation: Option<String>,
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
    // Pays d'origine agrégés via json_agg (feature 001-afrolang-pays-origine)
    #[sqlx(default)]
    pub pays_origine_json: Option<serde_json::Value>,
    // Administrateurs de la salle agrégés via json_agg (feature 001-admin-salles-publiques)
    #[sqlx(default)]
    pub administrateurs_json: Option<serde_json::Value>,
}

/// Ligne `afrolang.salle_privee` après refonte 2026-04.
///
/// `code_acces_hash` est chargé uniquement pour la vérification serveur
/// (endpoints `verifier-code` et modération du code). Ne jamais l'injecter
/// dans une réponse publique.
#[derive(Debug, FromRow)]
pub struct SallePriveeRow {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub code_acces_hash: String,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
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
///
/// Le concept de proposition publique a été supprimé (refonte 2026-04) :
/// la création d'une salle publique est désormais réservée aux admins.
/// La colonne `proposition_en_attente` reste dans le DTO pour compatibilité
/// frontend mais est toujours `FALSE`.
pub const GROUPE_ETHNIQUE_RESUME_COLONNES: &str =
    "ge.id, ge.nom, ge.fiche_pays_id,
     fp.pays_id AS pays_id, p.nom AS pays_nom,
     s.id AS salle_id, s.slug AS salle_slug,
     (s.id IS NOT NULL AND s.actif = TRUE) AS salle_active,
     FALSE AS proposition_en_attente";

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

/// Pays d'origine d'une salle Afrolang (feature 001-afrolang-pays-origine).
///
/// Vue allégée de `shared.pays` exposée par l'API publique et admin.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaysOrigineLight {
    pub id: Uuid,
    pub nom: String,
    pub code_iso2: Option<String>,
}

/// État de désactivation administrative d'une salle (feature 001-ressources-fermeture-session).
///
/// `motif` n'est rempli que pour les appelants admin ; il reste `None` dans les
/// réponses publiques (voir handler de lecture).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesactivationAdminInfo {
    pub desactivee_at: DateTime<Utc>,
    pub motif: Option<String>,
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
    pub pays_origine: Vec<PaysOrigineLight>,
    pub administrateurs: Vec<AdministrateurLight>,
    pub desactivee_admin: Option<DesactivationAdminInfo>,
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
    pub pays_origine: Vec<PaysOrigineLight>,
    pub administrateurs: Vec<AdministrateurLight>,
    pub desactivee_admin: Option<DesactivationAdminInfo>,
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

/// Réponse détaillée d'une salle privée (refonte 2026-04).
///
/// Ne contient jamais `code_acces_hash`. Toute salle privée est protégée par
/// un code secret — le flag `est_protegee` est donc toujours vrai et a été
/// supprimé du DTO.
#[derive(Debug, Serialize)]
pub struct SallePriveeResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub max_participants: Option<i32>,
    pub archivee_at: Option<DateTime<Utc>>,
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
    pub archivee_at: Option<DateTime<Utc>>,
    pub actif: bool,
    pub createur: CreateurResponse,
    pub salle_titre: Option<String>,
    pub salle_langue: Option<String>,
    pub session_en_cours: bool,
    pub sessions: Vec<SessionResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO public strictement aligné sur le contrat API (endpoint 1 & 2).
///
/// Ce DTO n'expose que les champs nécessaires à l'affichage côté front et
/// permet de signaler si l'utilisateur courant est l'auteur (pour le
/// court-circuit du code secret — FR-014).
#[derive(Debug, Serialize)]
pub struct SallePriveeAPI {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub auteur_id: Uuid,
    pub auteur_nom: Option<String>,
    pub session_en_cours: bool,
    pub est_auteur: bool,
    pub created_at: DateTime<Utc>,
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
    pub spotlight: Option<SpotlightInfo>,
    pub permissions_tableau_blanc_count: i64,
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
    pub par_page: i64,
    pub page: i64,
    pub total_pages: i64,
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
    pub pays_id: Option<Uuid>,
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
pub struct ModifierSalleRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub langue_cible: Option<String>,
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
}

/// Payload de modification d'une salle privée (auteur uniquement).
///
/// La modification du code d'accès passe par un endpoint dédié
/// (`PATCH /code-acces`), jamais via ce payload.
#[derive(Debug, Deserialize)]
pub struct ModifierSallePriveeRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub max_participants: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSessionRequest {
    pub titre: Option<String>,
    pub date_debut_prevue: Option<String>,
    pub max_participants: Option<i32>,
    pub tableau_blanc_actif: Option<bool>,
}

/// Requête de jointure d'une session publique (plus de code_acces depuis la
/// refonte : les codes d'accès ne concernent que les salles privées et sont
/// vérifiés via l'endpoint dédié `verifier-code`).
#[derive(Debug, Deserialize, Default)]
pub struct RejoindreRequest {}

/// Requête de transfert du rôle de modérateur de session (US3)
#[derive(Debug, Deserialize)]
pub struct TransfererModerationRequest {
    pub destinataire_id: Uuid,
}

/// Modification de la limite de participants (FR-036)
#[derive(Debug, Deserialize)]
pub struct ModifierMaxParticipantsRequest {
    pub max_participants: i32,
}

// ── Payloads refonte salles privées (001-afrolang-salles-refonte) ────────

/// Payload de création d'une salle privée depuis l'API publique.
///
/// Tous les champs sont obligatoires sauf `description`. Le `code_acces` est
/// transmis en clair (HTTPS) et immédiatement hashé côté serveur.
#[derive(Debug, Deserialize)]
pub struct CreerSallePriveePubliquePayload {
    pub salle_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub code_acces: String,
}

/// Payload de vérification d'un code d'accès (endpoint 3).
#[derive(Debug, Deserialize)]
pub struct VerifierCodeAccesRequest {
    pub code_acces: String,
}

/// Payload de modification du code d'accès par l'auteur (endpoint 5).
#[derive(Debug, Deserialize)]
pub struct ModifierCodeAccesRequest {
    pub nouveau_code_acces: String,
}

/// Réponse de l'endpoint `verifier-code` : remet un jeton d'accès éphémère
/// à présenter à l'endpoint `sessions/demarrer-ou-rejoindre`.
#[derive(Debug, Serialize)]
pub struct VerifierCodeAccesResponse {
    pub salle_privee_id: Uuid,
    pub acces_jeton: String,
    pub expires_at: DateTime<Utc>,
}

/// Réponse de l'endpoint `demarrer-ou-rejoindre` : informations LiveKit.
#[derive(Debug, Serialize)]
pub struct DemarrerRejoindreResponse {
    pub session_id: Uuid,
    pub livekit_url: String,
    pub livekit_token: String,
    pub moderateur_id: Option<Uuid>,
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

    pub fn to_pays_origine(&self) -> Vec<PaysOrigineLight> {
        self.pays_origine_json
            .as_ref()
            .and_then(|v| serde_json::from_value::<Vec<PaysOrigineLight>>(v.clone()).ok())
            .unwrap_or_default()
    }

    pub fn to_administrateurs(&self) -> Vec<AdministrateurLight> {
        self.administrateurs_json
            .as_ref()
            .and_then(|v| serde_json::from_value::<Vec<AdministrateurLight>>(v.clone()).ok())
            .unwrap_or_default()
    }

    /// Construit l'info de désactivation administrative pour un appelant non admin
    /// (le motif reste `None` côté public — voir [`Self::to_desactivation_admin`]).
    pub fn to_desactivation_public(&self) -> Option<DesactivationAdminInfo> {
        self.desactivee_admin_at
            .map(|desactivee_at| DesactivationAdminInfo {
                desactivee_at,
                motif: None,
            })
    }

    /// Variante admin : expose le motif détaillé.
    pub fn to_desactivation_admin(&self) -> Option<DesactivationAdminInfo> {
        self.desactivee_admin_at
            .map(|desactivee_at| DesactivationAdminInfo {
                desactivee_at,
                motif: self.motif_desactivation.clone(),
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
            pays_origine: self.to_pays_origine(),
            administrateurs: self.to_administrateurs(),
            // Lecture publique : motif systématiquement masqué (FR-020).
            // Le handler admin doit utiliser `to_desactivation_admin` pour
            // restituer le motif détaillé.
            desactivee_admin: self.to_desactivation_public(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl SallePriveeRow {
    /// Conversion en DTO legacy (utilisé par les endpoints admin / détails).
    /// N'expose jamais `code_acces_hash`.
    pub fn to_response(&self) -> SallePriveeResponse {
        SallePriveeResponse {
            id: self.id,
            salle_id: self.salle_id,
            titre: self.titre.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            max_participants: self.max_participants,
            archivee_at: self.archivee_at,
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

    /// Conversion en DTO public aligné sur le contrat (refonte 2026-04).
    /// Le champ `est_auteur` est calculé par rapport à l'utilisateur courant
    /// pour permettre au frontend de court-circuiter la saisie du code
    /// (FR-014). N'expose jamais `code_acces_hash`.
    pub fn to_api(&self, utilisateur_courant: Uuid) -> SallePriveeAPI {
        let auteur_nom = match (&self.createur_prenom, &self.createur_nom) {
            (Some(p), Some(n)) => Some(format!("{} {}", p, n)),
            (Some(p), None) => Some(p.clone()),
            (None, Some(n)) => Some(n.clone()),
            (None, None) => None,
        };
        SallePriveeAPI {
            id: self.id,
            salle_id: self.salle_id,
            titre: self.titre.clone(),
            description: self.description.clone(),
            auteur_id: self.cree_par,
            auteur_nom,
            session_en_cours: self.session_en_cours.unwrap_or(false),
            est_auteur: self.cree_par == utilisateur_courant,
            created_at: self.created_at,
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

// ══════════════════════════════════════════════════════════════════════════
// Feature 001-admin-salles-publiques — Propositions & administrateurs salle
// ══════════════════════════════════════════════════════════════════════════

/// Ligne brute lue de `afrolang.proposition_salle` (avec jointures auteur,
/// décideur, groupe ethnique, et json_agg des pays d'origine).
#[derive(Debug, FromRow)]
pub struct PropositionSalleRow {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub titre: String,
    pub description: String,
    pub justification: String,
    pub langue_cible: String,
    pub langue_code: Option<String>,
    pub groupe_ethnique_id: Uuid,
    pub pays_origine_ids: Vec<Uuid>,
    pub statut: PropositionStatut,
    pub decideur: Option<Uuid>,
    pub decide_at: Option<DateTime<Utc>>,
    pub commentaire_decision: Option<String>,
    pub salle_id_creee: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Jointures auteur
    #[sqlx(default)]
    pub auteur_nom: Option<String>,
    #[sqlx(default)]
    pub auteur_prenom: Option<String>,
    // Jointures décideur
    #[sqlx(default)]
    pub decideur_nom: Option<String>,
    #[sqlx(default)]
    pub decideur_prenom: Option<String>,
    // Jointure groupe ethnique
    #[sqlx(default)]
    pub groupe_ethnique_nom: Option<String>,
    // json_agg des pays d'origine
    #[sqlx(default)]
    pub pays_origine_json: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Clone)]
pub struct UtilisateurLight {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct GroupeEthniqueLight {
    pub id: Uuid,
    pub nom: String,
}

/// DTO public d'une proposition de salle (contracts/public.md & admin.md)
#[derive(Debug, Serialize)]
pub struct PropositionResponse {
    pub id: Uuid,
    pub auteur: UtilisateurLight,
    pub titre: String,
    pub description: String,
    pub justification: String,
    pub langue_cible: String,
    pub langue_code: Option<String>,
    pub groupe_ethnique: GroupeEthniqueLight,
    pub pays_origine: Vec<PaysOrigineLight>,
    pub statut: PropositionStatut,
    pub decideur: Option<UtilisateurLight>,
    pub decide_at: Option<DateTime<Utc>>,
    pub commentaire_decision: Option<String>,
    pub salle_id_creee: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PropositionSalleRow {
    pub fn to_response(&self) -> PropositionResponse {
        let pays_origine: Vec<PaysOrigineLight> = self
            .pays_origine_json
            .as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        let decideur = match (self.decideur, &self.decideur_nom, &self.decideur_prenom) {
            (Some(id), Some(nom), Some(prenom)) => Some(UtilisateurLight {
                id,
                nom: nom.clone(),
                prenom: prenom.clone(),
            }),
            _ => None,
        };

        PropositionResponse {
            id: self.id,
            auteur: UtilisateurLight {
                id: self.auteur_id,
                nom: self.auteur_nom.clone().unwrap_or_default(),
                prenom: self.auteur_prenom.clone().unwrap_or_default(),
            },
            titre: self.titre.clone(),
            description: self.description.clone(),
            justification: self.justification.clone(),
            langue_cible: self.langue_cible.clone(),
            langue_code: self.langue_code.clone(),
            groupe_ethnique: GroupeEthniqueLight {
                id: self.groupe_ethnique_id,
                nom: self.groupe_ethnique_nom.clone().unwrap_or_default(),
            },
            pays_origine,
            statut: self.statut.clone(),
            decideur,
            decide_at: self.decide_at,
            commentaire_decision: self.commentaire_decision.clone(),
            salle_id_creee: self.salle_id_creee,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PropositionListeResponse {
    pub items: Vec<PropositionResponse>,
    pub total: i64,
    pub page: i64,
    pub taille: i64,
}

/// Vue allégée d'un administrateur de salle pour l'affichage public sur la
/// fiche de salle. Filtré sur `actif=TRUE`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdministrateurLight {
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
    pub nomme_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct SalleAdministrateurRow {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub utilisateur_id: Uuid,
    pub nomme_par: Uuid,
    pub nomme_at: DateTime<Utc>,
    pub actif: bool,
    pub revoque_at: Option<DateTime<Utc>>,
    pub revoque_par: Option<Uuid>,
    pub motif_revocation: Option<String>,
    pub suspendu_at: Option<DateTime<Utc>>,
    pub motif_suspension: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Jointures utilisateur
    #[sqlx(default)]
    pub utilisateur_nom: Option<String>,
    #[sqlx(default)]
    pub utilisateur_prenom: Option<String>,
    #[sqlx(default)]
    pub utilisateur_email: Option<String>,
    #[sqlx(default)]
    pub utilisateur_photo: Option<String>,
    // Jointures nomme_par
    #[sqlx(default)]
    pub nomme_par_nom: Option<String>,
    #[sqlx(default)]
    pub nomme_par_prenom: Option<String>,
    // Jointures revoque_par
    #[sqlx(default)]
    pub revoque_par_nom: Option<String>,
    #[sqlx(default)]
    pub revoque_par_prenom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UtilisateurAdminResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SalleAdministrateurResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub utilisateur: UtilisateurAdminResponse,
    pub nomme_par: UtilisateurLight,
    pub nomme_at: DateTime<Utc>,
    pub actif: bool,
    pub revoque_at: Option<DateTime<Utc>>,
    pub revoque_par: Option<UtilisateurLight>,
    pub motif_revocation: Option<String>,
    pub suspendu_at: Option<DateTime<Utc>>,
    pub motif_suspension: Option<String>,
}

impl SalleAdministrateurRow {
    pub fn to_response(&self) -> SalleAdministrateurResponse {
        SalleAdministrateurResponse {
            id: self.id,
            salle_id: self.salle_id,
            utilisateur: UtilisateurAdminResponse {
                id: self.utilisateur_id,
                nom: self.utilisateur_nom.clone().unwrap_or_default(),
                prenom: self.utilisateur_prenom.clone().unwrap_or_default(),
                email: self.utilisateur_email.clone().unwrap_or_default(),
                photo_url: self.utilisateur_photo.clone(),
            },
            nomme_par: UtilisateurLight {
                id: self.nomme_par,
                nom: self.nomme_par_nom.clone().unwrap_or_default(),
                prenom: self.nomme_par_prenom.clone().unwrap_or_default(),
            },
            nomme_at: self.nomme_at,
            actif: self.actif,
            revoque_at: self.revoque_at,
            revoque_par: match (self.revoque_par, &self.revoque_par_nom, &self.revoque_par_prenom) {
                (Some(id), Some(nom), Some(prenom)) => Some(UtilisateurLight {
                    id,
                    nom: nom.clone(),
                    prenom: prenom.clone(),
                }),
                _ => None,
            },
            motif_revocation: self.motif_revocation.clone(),
            suspendu_at: self.suspendu_at,
            motif_suspension: self.motif_suspension.clone(),
        }
    }
}

// ── Payloads de requête ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SoumettrePropositionRequest {
    pub titre: String,
    pub description: String,
    pub justification: String,
    pub langue_cible: String,
    pub langue_code: Option<String>,
    pub groupe_ethnique_id: Uuid,
    pub pays_origine_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct DecisionRequest {
    pub commentaire: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NommerAdministrateurRequest {
    pub utilisateur_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct RevoquerAdministrateurRequest {
    pub motif: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PropositionMesFiltres {
    pub statut: Option<PropositionStatut>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PropositionAdminFiltres {
    pub statut: Option<PropositionStatut>,
    pub langue_code: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub auteur_id: Option<Uuid>,
    pub date_debut: Option<DateTime<Utc>>,
    pub date_fin: Option<DateTime<Utc>>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
    pub tri: Option<String>,
}

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

// ══════════════════════════════════════════════════════════════════════════
// Feature 001-session-moderation
// ══════════════════════════════════════════════════════════════════════════

/// Colonnes lues sur `afrolang.session_permission_tableau_blanc`.
pub const COLONNES_PERMISSION_TB: &str =
    "sptb.session_id, sptb.utilisateur_id, sptb.accorde_par, sptb.accorde_at";

#[derive(Debug, FromRow)]
pub struct PermissionTableauBlancRow {
    pub session_id: Uuid,
    pub utilisateur_id: Uuid,
    pub accorde_par: Uuid,
    pub accorde_at: DateTime<Utc>,
    #[sqlx(default)]
    pub nom: Option<String>,
    #[sqlx(default)]
    pub prenom: Option<String>,
    #[sqlx(default)]
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PermissionTableauBlancResponse {
    pub utilisateur_id: Uuid,
    pub nom_complet: String,
    pub avatar_url: Option<String>,
    pub accorde_par: Uuid,
    pub accorde_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ModerateurOfficeResponse {
    pub utilisateur_id: Uuid,
    pub nom_complet: String,
    pub avatar_url: Option<String>,
    pub niveau: NiveauModerateur,
}

#[derive(Debug, Serialize)]
pub struct PermissionsTableauBlancListeResponse {
    pub session_id: Uuid,
    pub moderateurs_office: Vec<ModerateurOfficeResponse>,
    pub permissions_individuelles: Vec<PermissionTableauBlancResponse>,
    pub mon_niveau_moderateur: Option<NiveauModerateur>,
}

#[derive(Debug, Deserialize)]
pub struct AccorderPermissionPayload {
    pub utilisateur_id: Uuid,
}

#[derive(Debug, Serialize, Clone)]
pub struct SpotlightInfo {
    pub utilisateur_id: Uuid,
    pub nom_complet: String,
    pub avatar_url: Option<String>,
    pub mis_en_evidence_par: Uuid,
    pub mis_en_evidence_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct MettreEnEvidencePayload {
    pub utilisateur_id: Uuid,
}

/// Niveau de modérateur calculé applicatif (FR-001/FR-001b).
/// Sérialisé en `snake_case` pour le frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NiveauModerateur {
    AdminPlateforme,
    AdminSalle,
    ModerateurAttitre,
    CreateurSallePrivee,
}

impl NiveauModerateur {
    /// Capacité spotlight : tout sauf modérateur attitré (FR-001b).
    pub fn peut_spotlight(&self) -> bool {
        matches!(
            self,
            Self::AdminPlateforme | Self::AdminSalle | Self::CreateurSallePrivee
        )
    }
}
