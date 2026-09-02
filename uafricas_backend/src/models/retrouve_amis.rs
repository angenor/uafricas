use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ══════════════════════════════════════════════════════════════
// Colonnes SQL
// ══════════════════════════════════════════════════════════════

/// Colonnes SELECT pour retrouve_amis.avis_recherche
/// Les enums PostgreSQL sont castes en ::text pour compatibilite sqlx
pub const AVIS_RECHERCHE_COLONNES: &str =
    "id, auteur_id, nom_recherche, prenom_recherche, surnom,
     ecole, ville, pays_id, periode_debut, periode_fin,
     description, etat::text AS etat,
     est_anonyme, genre_recherche::text AS genre_recherche,
     type_relation::text AS type_relation, type_relation_autre, comment_connu,
     localite_rencontre, ecole_rencontre, ville_rencontre,
     jamais_rencontre, rencontre_reseaux_sociaux, reseaux_sociaux,
     photo_url, description_physique,
     partage_coordonnees, coordonnees_email, coordonnees_telephone, coordonnees_whatsapp,
     created_at, updated_at, deleted_at";

/// Colonnes SELECT pour retrouve_amis.correspondance
/// Les enums et NUMERIC sont castes pour compatibilite sqlx
pub const CORRESPONDANCE_COLONNES: &str =
    "id, avis_id, type_cible::text AS type_cible,
     cible_avis_id, cible_utilisateur_id,
     score::float8 AS score, details_score,
     etat::text AS etat,
     accepte_par_a_at, accepte_par_b_at,
     coordonnees_a, coordonnees_b,
     expire_at, created_at, updated_at";

/// Colonnes SELECT pour retrouve_amis.notification_retrouve
/// Le mot-cle "type" est renomme en "type_notif" pour eviter le conflit Rust
pub const NOTIFICATION_COLONNES: &str =
    "id, utilisateur_id, correspondance_id,
     type::text AS type_notif, lu, created_at";

/// Colonnes SELECT pour retrouve_amis.parcours_trouvable
/// Le cast type_entree::text convertit l'enum PostgreSQL en texte
pub const PARCOURS_COLONNES: &str =
    "id, utilisateur_id, type_entree::text AS type_entree,
     nom, ville, pays_id, periode_debut, periode_fin,
     created_at, updated_at";

// ══════════════════════════════════════════════════════════════
// Constantes de tri
// ══════════════════════════════════════════════════════════════

/// Colonnes autorisees pour le tri des avis de recherche
pub const AVIS_TRI_COLONNES: &[&str] = &["created_at", "updated_at", "nom_recherche", "etat"];

// ══════════════════════════════════════════════════════════════
// Structs FromRow (mapping base de donnees)
// ══════════════════════════════════════════════════════════════

/// Representation d'un avis de recherche en base de donnees
/// Le champ search_vector (TSVECTOR) est exclu car non necessaire cote Rust
#[derive(Debug, FromRow)]
pub struct AvisRecherche {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays_id: Option<Uuid>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub description: Option<String>,
    pub etat: String,
    // Colonnes 003-retrouve-amis-public
    pub est_anonyme: bool,
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub type_relation_autre: Option<String>,
    pub comment_connu: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    pub jamais_rencontre: bool,
    pub rencontre_reseaux_sociaux: bool,
    pub reseaux_sociaux: Option<String>,
    pub photo_url: Option<String>,
    pub description_physique: Option<String>,
    pub partage_coordonnees: bool,
    pub coordonnees_email: Option<String>,
    pub coordonnees_telephone: Option<String>,
    pub coordonnees_whatsapp: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Representation d'une correspondance en base de donnees
#[derive(Debug, FromRow)]
pub struct Correspondance {
    pub id: Uuid,
    pub avis_id: Uuid,
    pub type_cible: String,
    pub cible_avis_id: Option<Uuid>,
    pub cible_utilisateur_id: Option<Uuid>,
    pub score: f64,
    pub details_score: Option<serde_json::Value>,
    pub etat: String,
    pub accepte_par_a_at: Option<DateTime<Utc>>,
    pub accepte_par_b_at: Option<DateTime<Utc>>,
    pub coordonnees_a: Option<serde_json::Value>,
    pub coordonnees_b: Option<serde_json::Value>,
    pub expire_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Representation d'une notification retrouve en base de donnees
/// Le champ "type" PostgreSQL est renomme "type_notif" via sqlx(rename)
#[derive(Debug, FromRow)]
pub struct NotificationRetrouve {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    /// Nullable : `avis_suspendu` et `demande_retrait` ne se rattachent a
    /// aucune correspondance. Un `Uuid` non optionnel faisait echouer sqlx a
    /// la deserialisation, et c'est TOUTE la liste qui tombait des la premiere
    /// notification de ce genre.
    pub correspondance_id: Option<Uuid>,
    #[sqlx(rename = "type_notif")]
    pub type_notif: String,
    pub lu: bool,
    pub created_at: DateTime<Utc>,
}

/// Representation d'un parcours trouvable en base de donnees
#[derive(Debug, FromRow)]
pub struct ParcoursTrouvable {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub type_entree: String,
    pub nom: String,
    pub ville: Option<String>,
    pub pays_id: Option<Uuid>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Resultat de la fonction de matching/scoring
/// Retourne par la requete SQL de calcul des correspondances
#[derive(Debug, FromRow)]
pub struct CorrespondanceResultat {
    pub type_cible: String,
    pub cible_id: Uuid,
    pub score_total: f64,
    pub details: serde_json::Value,
}

// ══════════════════════════════════════════════════════════════
// DTOs de reponse (Serialize)
// ══════════════════════════════════════════════════════════════

/// Info pays jointe depuis shared.pays
#[derive(Debug, Serialize, Clone, sqlx::FromRow)]
pub struct PaysInfo {
    pub id: Uuid,
    pub nom: String,
}

/// Resume anonymise d'une correspondance (avant acceptation mutuelle)
#[derive(Debug, Serialize, Clone)]
pub struct ResumeAnonyme {
    pub initiales: String,
    pub ville: Option<String>,
    pub periode: Option<String>,
    pub criteres_communs: Vec<String>,
}

/// DTO pour un avis de recherche dans la liste
#[derive(Debug, Serialize)]
pub struct AvisRechercheResponse {
    pub id: Uuid,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays: Option<PaysInfo>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub etat: String,
    pub nb_correspondances: i64,
    pub est_public: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub compteur_partages: i32,
    // Champs 003
    pub est_anonyme: bool,
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub type_relation_autre: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    pub jamais_rencontre: bool,
    pub rencontre_reseaux_sociaux: bool,
    pub reseaux_sociaux: Option<String>,
    pub photo_url: Option<String>,
    pub description_physique: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Reponse paginee pour le listing des avis de recherche
#[derive(Debug, Serialize)]
pub struct AvisRechercheListeResponse {
    pub avis: Vec<AvisRechercheResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
}

/// DTO pour le detail d'un avis de recherche (avec correspondances)
#[derive(Debug, Serialize)]
pub struct AvisRechercheDetailResponse {
    pub id: Uuid,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays: Option<PaysInfo>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub description: Option<String>,
    pub etat: String,
    // Champs 003
    pub est_anonyme: bool,
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub type_relation_autre: Option<String>,
    pub comment_connu: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    pub jamais_rencontre: bool,
    pub rencontre_reseaux_sociaux: bool,
    pub reseaux_sociaux: Option<String>,
    pub photo_url: Option<String>,
    pub description_physique: Option<String>,
    pub partage_coordonnees: bool,
    pub coordonnees_email: Option<String>,
    pub coordonnees_telephone: Option<String>,
    pub coordonnees_whatsapp: Option<String>,
    pub correspondances: Vec<CorrespondanceResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour une correspondance dans la liste
#[derive(Debug, Serialize)]
pub struct CorrespondanceResponse {
    pub id: Uuid,
    pub avis_id: Uuid,
    pub score: f64,
    pub etat: String,
    pub type_cible: String,
    pub resume_anonymise: ResumeAnonyme,
    pub mon_role: String,
    pub created_at: DateTime<Utc>,
    pub expire_at: Option<DateTime<Utc>>,
}

/// Reponse paginee pour le listing des correspondances
#[derive(Debug, Serialize)]
pub struct CorrespondanceListeResponse {
    pub correspondances: Vec<CorrespondanceResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
}

/// DTO pour le detail d'une correspondance (avec score detaille)
#[derive(Debug, Serialize)]
pub struct CorrespondanceDetailResponse {
    pub id: Uuid,
    pub avis_id: Uuid,
    pub score: f64,
    pub details_score: serde_json::Value,
    pub etat: String,
    pub type_cible: String,
    pub mon_role: String,
    pub resume_anonymise: ResumeAnonyme,
    pub coordonnees_partagees: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reponse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_reponse_publique: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expire_at: Option<DateTime<Utc>>,
}

/// DTO pour une notification dans la liste
#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_notif: String,
    /// Nullable comme en base : `avis_suspendu` et `demande_retrait` ne se
    /// rattachent a aucune correspondance.
    pub correspondance_id: Option<Uuid>,
    pub lu: bool,
    pub created_at: DateTime<Utc>,
}

/// Reponse paginee pour le listing des notifications
#[derive(Debug, Serialize)]
pub struct NotificationListeResponse {
    pub notifications: Vec<NotificationResponse>,
    pub total: i64,
    pub non_lues: i64,
    pub page: i64,
    pub par_page: i64,
}

/// DTO pour un parcours trouvable
#[derive(Debug, Serialize)]
pub struct ParcoursTrouvableResponse {
    pub id: Uuid,
    pub type_entree: String,
    pub nom: String,
    pub ville: Option<String>,
    pub pays: Option<PaysInfo>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
}

/// DTO pour le tableau de bord de l'utilisateur
#[derive(Debug, Serialize)]
pub struct TableauDeBord {
    pub avis_actifs: i64,
    pub avis_clotures: i64,
    pub correspondances_en_attente: i64,
    pub correspondances_mutuelles: i64,
    pub notifications_non_lues: i64,
    pub est_trouvable: bool,
    pub nb_parcours: i64,
}

/// DTO reponse apres creation d'un avis de recherche
#[derive(Debug, Serialize)]
pub struct CreerAvisResponse {
    pub id: Uuid,
    pub etat: String,
    pub slug: String,
    pub correspondances_trouvees: i64,
}

/// DTO reponse apres modification d'un avis de recherche
#[derive(Debug, Serialize)]
pub struct ModifierAvisResponse {
    pub id: Uuid,
    pub correspondances_trouvees: i64,
}

/// DTO reponse apres bascule du statut trouvable
#[derive(Debug, Serialize)]
pub struct BasculerTrouvableResponse {
    pub est_trouvable: bool,
    pub correspondances_trouvees: i64,
}

// ══════════════════════════════════════════════════════════════
// DTOs de requete (Deserialize)
// ══════════════════════════════════════════════════════════════

/// Corps de la requete de creation d'un avis de recherche
/// Note: sera remplace par multipart dans le handler (003), mais garde pour retrocompat
#[derive(Debug, Deserialize)]
pub struct CreerAvisRecherche {
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays_id: Option<Uuid>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub description: Option<String>,
    // Champs 003
    #[serde(default)]
    pub est_anonyme: bool,
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub comment_connu: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    #[serde(default)]
    pub jamais_rencontre: bool,
    pub description_physique: Option<String>,
    #[serde(default)]
    pub partage_coordonnees: bool,
    pub coordonnees_email: Option<String>,
    pub coordonnees_telephone: Option<String>,
    pub coordonnees_whatsapp: Option<String>,
}

/// Corps de la requete de modification d'un avis de recherche
#[derive(Debug, Deserialize)]
pub struct ModifierAvisRecherche {
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays_id: Option<Uuid>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub description: Option<String>,
    // Champs 003
    #[serde(default)]
    pub est_anonyme: bool,
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub comment_connu: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    #[serde(default)]
    pub jamais_rencontre: bool,
    pub description_physique: Option<String>,
    #[serde(default)]
    pub partage_coordonnees: bool,
    pub coordonnees_email: Option<String>,
    pub coordonnees_telephone: Option<String>,
    pub coordonnees_whatsapp: Option<String>,
}

/// Choix des coordonnees a partager lors de l'acceptation
#[derive(Debug, Deserialize, Serialize)]
pub struct CoordonneesChoix {
    pub email: bool,
    pub telephone: bool,
    pub messagerie: bool,
}

/// Corps de la requete d'acceptation d'une correspondance
#[derive(Debug, Deserialize)]
pub struct AccepterCorrespondance {
    pub coordonnees: CoordonneesChoix,
}

/// Corps de la requete de signalement d'un avis
#[derive(Debug, Deserialize)]
pub struct SignalerAvis {
    pub motif: String,
    pub description: Option<String>,
}

/// Corps de la requete de creation d'un parcours trouvable
#[derive(Debug, Deserialize)]
pub struct CreerParcours {
    pub type_entree: String,
    pub nom: String,
    pub ville: Option<String>,
    pub pays_id: Option<Uuid>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
}

/// Corps de la requete de modification d'un parcours trouvable
#[derive(Debug, Deserialize)]
pub struct ModifierParcours {
    pub type_entree: String,
    pub nom: String,
    pub ville: Option<String>,
    pub pays_id: Option<Uuid>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
}

/// Corps de la requete de bascule du statut trouvable
#[derive(Debug, Deserialize)]
pub struct BasculerTrouvable {
    pub est_trouvable: bool,
}

// ══════════════════════════════════════════════════════════════
// Partage public des avis de recherche (002-partage-avis-recherche)
// ══════════════════════════════════════════════════════════════

// ── Colonnes SQL ─────────────────────────────────────────────

/// Colonnes SELECT pour un avis public actif (detail complet)
pub const AVIS_PUBLIC_DETAIL_COLONNES: &str =
    "a.id, a.auteur_id, a.slug, a.nom_recherche, a.prenom_recherche,
     a.surnom, a.ecole, a.ville, a.periode_debut, a.periode_fin,
     a.description, a.etat::text AS etat,
     a.est_anonyme, a.genre_recherche::text AS genre_recherche,
     a.type_relation::text AS type_relation, a.comment_connu,
     a.localite_rencontre, a.ecole_rencontre, a.ville_rencontre,
     a.jamais_rencontre, a.photo_url, a.description_physique,
     a.compteur_partages, a.date_publication_publique, a.created_at,
     u.prenom AS auteur_prenom, u.nom AS auteur_nom,
     p.id AS pays_id, p.nom AS pays_nom";

/// Colonnes SELECT pour le listing des avis publics (resume)
pub const AVIS_PUBLIC_LISTE_COLONNES: &str =
    "a.id, a.slug, a.nom_recherche, a.prenom_recherche,
     a.etat::text AS etat,
     a.est_anonyme, a.genre_recherche::text AS genre_recherche,
     a.type_relation::text AS type_relation,
     a.localite_rencontre, a.ecole_rencontre, a.ville_rencontre,
     a.photo_url, a.description_physique,
     a.ville, a.periode_debut, a.periode_fin,
     a.compteur_partages, a.created_at,
     u.prenom AS auteur_prenom, u.nom AS auteur_nom,
     p.id AS pays_id, p.nom AS pays_nom";

/// Colonnes autorisees pour le tri des avis publics
pub const AVIS_PUBLIC_TRI_COLONNES: &[&str] = &["created_at", "compteur_partages"];

// ── Structs FromRow (mapping base de donnees) ────────────────

/// Ligne SQL pour le detail d'un avis public actif
#[derive(Debug, FromRow)]
pub struct AvisPublicDetailRow {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub slug: Option<String>,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub description: Option<String>,
    pub etat: String,
    // Champs 003
    pub est_anonyme: bool,
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub comment_connu: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    pub jamais_rencontre: bool,
    pub photo_url: Option<String>,
    pub description_physique: Option<String>,
    pub compteur_partages: i32,
    pub date_publication_publique: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub auteur_prenom: String,
    pub auteur_nom: String,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
}

/// Ligne SQL pour le listing des avis publics
#[derive(Debug, FromRow)]
pub struct AvisPublicListeRow {
    pub id: Uuid,
    pub slug: Option<String>,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub etat: String,
    // Champs 003
    pub est_anonyme: bool,
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    pub photo_url: Option<String>,
    pub description_physique: Option<String>,
    pub ville: Option<String>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub compteur_partages: i32,
    pub created_at: DateTime<Utc>,
    pub auteur_prenom: String,
    pub auteur_nom: String,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
}

// ── DTOs de reponse : Publics ────────────────────────────────

/// Detail complet d'un avis public actif
#[derive(Debug, Serialize)]
pub struct AvisPublicDetailResponse {
    pub id: Uuid,
    pub auteur_id: Uuid,
    pub slug: String,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub surnom: Option<String>,
    pub ecole: Option<String>,
    pub ville: Option<String>,
    pub pays: Option<PaysInfo>,
    pub periode_debut: Option<i32>,
    pub periode_fin: Option<i32>,
    pub description: Option<String>,
    // Champs 003
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub comment_connu: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    pub jamais_rencontre: bool,
    pub photo_url: Option<String>,
    pub description_physique: Option<String>,
    pub auteur_anonyme: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auteur_pseudonyme: Option<String>,
    pub etat: String,
    pub compteur_partages: i32,
    pub created_at: DateTime<Utc>,
}

/// Reponse pour un avis non-actif (cloture, suspendu)
#[derive(Debug, Serialize)]
pub struct AvisPublicEtatResponse {
    pub slug: String,
    pub etat: String,
    pub message: String,
}

/// Resume d'un avis public dans le listing
#[derive(Debug, Serialize)]
pub struct AvisPublicResumeResponse {
    pub id: Uuid,
    pub slug: String,
    pub nom_recherche: String,
    pub prenom_recherche: Option<String>,
    pub etat: String,
    // Champs 003
    pub genre_recherche: Option<String>,
    pub type_relation: Option<String>,
    pub localite_rencontre: Option<String>,
    pub ecole_rencontre: Option<String>,
    pub ville_rencontre: Option<String>,
    pub photo_url: Option<String>,
    pub description_physique: Option<String>,
    pub auteur_anonyme: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auteur_pseudonyme: Option<String>,
    pub ville: Option<String>,
    pub pays: Option<PaysInfo>,
    pub compteur_partages: i32,
    pub created_at: DateTime<Utc>,
}

/// Reponse paginee pour le listing des avis publics
#[derive(Debug, Serialize)]
pub struct ListeAvisPublicsResponse {
    pub avis: Vec<AvisPublicResumeResponse>,
    pub pagination: PaginationInfo,
}

/// Info de pagination standard
#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub par_page: i64,
    pub total: i64,
    pub pages: i64,
}

/// Reponse apres publication/depublication d'un avis
#[derive(Debug, Serialize)]
pub struct PublierAvisResponse {
    pub id: Uuid,
    pub est_public: bool,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_publication_publique: Option<DateTime<Utc>>,
}

/// Reponse apres reponse publique
#[derive(Debug, Serialize)]
pub struct ReponsePubliqueResponse {
    pub id: Uuid,
    pub correspondance_id: Uuid,
    pub message: String,
}

/// Reponse apres demande de retrait
#[derive(Debug, Serialize)]
pub struct DemandeRetraitResponse {
    pub id: Uuid,
    pub message: String,
}

/// Reponse apres increment du compteur de partages
#[derive(Debug, Serialize)]
pub struct PartageResponse {
    pub compteur_partages: i32,
}

// ── DTOs de requete : Publics ────────────────────────────────

/// Corps de la requete pour publier/depublier un avis
#[derive(Debug, Deserialize)]
pub struct PublierAvisRequest {
    pub est_public: bool,
}

/// Corps de la requete de reponse publique
#[derive(Debug, Deserialize)]
pub struct ReponsePubliqueRequest {
    pub type_reponse: String,
    pub message: String,
}

/// Corps de la requete de signalement depuis la page publique
#[derive(Debug, Deserialize)]
pub struct SignalerPublicRequest {
    pub motif: String,
    pub description: Option<String>,
}

/// Corps de la requete de demande de retrait
#[derive(Debug, Deserialize)]
pub struct DemandeRetraitRequest {
    pub motif: String,
}

/// Parametres de recherche des avis publics
#[derive(Debug, Deserialize)]
pub struct RecherchePubliqueParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub recherche: Option<String>,
    pub type_relation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub ecole: Option<String>,
    pub tri: Option<String>,
    pub ordre: Option<String>,
}

// ── Conversions FromRow → Response DTO ───────────────────────

impl AvisPublicDetailRow {
    /// Convertit la ligne SQL en DTO de detail public avec auteur anonymise
    pub fn to_detail_response(&self) -> AvisPublicDetailResponse {
        let auteur_pseudonyme = if self.est_anonyme {
            None
        } else {
            Some(format!(
                "{} {}.",
                self.auteur_prenom,
                self.auteur_nom.chars().next().unwrap_or(' ')
            ))
        };
        AvisPublicDetailResponse {
            id: self.id,
            auteur_id: self.auteur_id,
            slug: self.slug.clone().unwrap_or_default(),
            nom_recherche: self.nom_recherche.clone(),
            prenom_recherche: self.prenom_recherche.clone(),
            surnom: self.surnom.clone(),
            ecole: self.ecole.clone(),
            ville: self.ville.clone(),
            pays: self.pays_id.and_then(|id| {
                self.pays_nom.as_ref().map(|nom| PaysInfo {
                    id,
                    nom: nom.clone(),
                })
            }),
            periode_debut: self.periode_debut,
            periode_fin: self.periode_fin,
            description: self.description.clone(),
            genre_recherche: self.genre_recherche.clone(),
            type_relation: self.type_relation.clone(),
            comment_connu: self.comment_connu.clone(),
            localite_rencontre: self.localite_rencontre.clone(),
            ecole_rencontre: self.ecole_rencontre.clone(),
            ville_rencontre: self.ville_rencontre.clone(),
            jamais_rencontre: self.jamais_rencontre,
            photo_url: self.photo_url.clone(),
            description_physique: self.description_physique.clone(),
            auteur_anonyme: self.est_anonyme,
            auteur_pseudonyme,
            etat: self.etat.clone(),
            compteur_partages: self.compteur_partages,
            created_at: self.created_at,
        }
    }
}

impl AvisPublicListeRow {
    /// Convertit la ligne SQL en DTO de resume public
    pub fn to_resume_response(&self) -> AvisPublicResumeResponse {
        let auteur_pseudonyme = if self.est_anonyme {
            None
        } else {
            Some(format!(
                "{} {}.",
                self.auteur_prenom,
                self.auteur_nom.chars().next().unwrap_or(' ')
            ))
        };
        AvisPublicResumeResponse {
            id: self.id,
            slug: self.slug.clone().unwrap_or_default(),
            nom_recherche: self.nom_recherche.clone(),
            prenom_recherche: self.prenom_recherche.clone(),
            etat: self.etat.clone(),
            genre_recherche: self.genre_recherche.clone(),
            type_relation: self.type_relation.clone(),
            localite_rencontre: self.localite_rencontre.clone(),
            ecole_rencontre: self.ecole_rencontre.clone(),
            ville_rencontre: self.ville_rencontre.clone(),
            photo_url: self.photo_url.clone(),
            description_physique: self.description_physique.clone(),
            auteur_anonyme: self.est_anonyme,
            auteur_pseudonyme,
            ville: self.ville.clone(),
            pays: self.pays_id.and_then(|id| {
                self.pays_nom.as_ref().map(|nom| PaysInfo {
                    id,
                    nom: nom.clone(),
                })
            }),
            compteur_partages: self.compteur_partages,
            created_at: self.created_at,
        }
    }
}
