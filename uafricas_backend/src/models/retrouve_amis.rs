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
    pub correspondance_id: Uuid,
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
#[derive(Debug, Serialize, Clone)]
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
    pub created_at: DateTime<Utc>,
    pub expire_at: Option<DateTime<Utc>>,
}

/// DTO pour une notification dans la liste
#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_notif: String,
    pub correspondance_id: Uuid,
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
