//! Interactions communautaires sur les médias radio et télé : réactions,
//! commentaires, partages et signalements (feature 001-refonte-tele-radio,
//! US3 et US7 — migration 09k).
//!
//! Les quatre tables sont génériques, discriminées par `(type_media, media_id)`
//! sur les quatre supports et contenus. Calqué sur `element_social`, qui rend
//! le même service aux sous-objets afripulse.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Les quatre cibles possibles d'une interaction.
///
/// **Whitelist de littéraux** : ces valeurs sont interpolées dans le SQL des
/// requêtes UNION du mur communautaire. Elles ne proviennent JAMAIS directement
/// de l'entrée client — celle-ci est d'abord confrontée à cette liste, et
/// rejetée si elle n'y figure pas.
pub const TYPES_MEDIA_AUTORISES: [&str; 4] = [
    "chaine_tv",
    "station_radio",
    "programme_tele",
    "programme_radio",
];

/// Au-delà de ce nombre de signalements DISTINCTS, le contenu bascule
/// automatiquement en `etat = 'suspendu'` (FR-050).
///
/// Comparateur `>` : la suspension survient au 11ᵉ signalement, à l'identique
/// des deux mécanismes les plus récents du projet
/// (`contribution_signalement.rs`, `session_signalement.rs`).
pub const SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA: i64 = 10;

/// Description SQL d'un type de média : table qualifiée, colonne portant le
/// titre affichable, et préfixe d'URL de sa page de détail publique.
///
/// Les quatre tables partagent la même forme (`id`, `slug`, `etat`,
/// `deleted_at`, `image_couverture_url`, `nombre_signalements`), seul le nom du
/// titre varie. Les valeurs renvoyées sont des littéraux fixes → interpolation
/// SQL sûre.
pub struct DescripteurMedia {
    pub table: &'static str,
    pub colonne_titre: &'static str,
    /// Segment d'URL de la page de détail, à suffixer du slug.
    pub base_url: &'static str,
}

pub fn descripteur_pour_type(type_media: &str) -> Option<DescripteurMedia> {
    match type_media {
        "chaine_tv" => Some(DescripteurMedia {
            table: "media_content.chaine_tv",
            colonne_titre: "nom",
            base_url: "/medias/chaines",
        }),
        "station_radio" => Some(DescripteurMedia {
            table: "media_content.station_radio",
            colonne_titre: "nom",
            base_url: "/medias/stations",
        }),
        "programme_tele" => Some(DescripteurMedia {
            table: "media_content.programme_tele",
            colonne_titre: "nom_emission",
            base_url: "/medias/programmes-tele",
        }),
        "programme_radio" => Some(DescripteurMedia {
            table: "media_content.programme_radio",
            colonne_titre: "nom_emission",
            base_url: "/medias/programmes-radio",
        }),
        _ => None,
    }
}

/// Table qualifiée d'un type de média, ou None si le type n'est pas supporté.
pub fn table_pour_type(type_media: &str) -> Option<&'static str> {
    descripteur_pour_type(type_media).map(|d| d.table)
}

// ────────────────────────────────────────────────────────────────
// Réactions like / dislike (FR-023)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ReactionMediaRequest {
    /// "like" ou "dislike". `null` retire la réaction existante.
    pub type_reaction: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReactionMediaEtat {
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    /// Réaction du membre courant : "like" | "dislike" | null
    pub ma_reaction: Option<String>,
}

/// Compteurs d'interaction agrégés, greffés sur les DTO de contenu pour éviter
/// un aller-retour par carte (FR-027).
#[derive(Debug, Serialize, Default, Clone)]
pub struct CompteursInteraction {
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub nombre_commentaires: i32,
    pub nombre_partages: i32,
    pub ma_reaction: Option<String>,
}

/// Ligne brute d'agrégation des compteurs d'un contenu.
#[derive(Debug, FromRow)]
pub struct CompteursRow {
    pub media_id: Uuid,
    pub nombre_likes: i64,
    pub nombre_dislikes: i64,
    pub nombre_commentaires: i64,
    pub nombre_partages: i64,
    pub ma_reaction: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Commentaires (FR-024)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CommentaireMediaRequest {
    pub contenu: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentaireQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, FromRow)]
pub struct CommentaireMediaRow {
    pub id: Uuid,
    pub contenu: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CommentaireMediaResponse {
    pub id: Uuid,
    pub contenu: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub auteur: AuteurApercu,
    /// Vrai si le membre courant peut supprimer ce commentaire — la suppression
    /// est réservée à son auteur.
    pub est_mien: bool,
}

#[derive(Debug, Serialize)]
pub struct CommentaireListeResponse {
    pub commentaires: Vec<CommentaireMediaResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ────────────────────────────────────────────────────────────────
// Partages vers le mur communautaire (FR-025)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PartageMediaRequest {
    pub legende: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PartageMediaQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Ligne brute enrichie : UNION des quatre types → contenu + auteur du partage.
#[derive(Debug, FromRow)]
pub struct PartageMediaRow {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    pub type_media: String,
    pub media_id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub image_url: Option<String>,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuteurApercu {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MediaApercu {
    /// 'chaine_tv' | 'station_radio' | 'programme_tele' | 'programme_radio'
    pub type_media: String,
    pub media_id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub image_url: Option<String>,
    /// URL de la page de détail, prête à l'emploi côté carte du mur.
    pub url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartageMediaResponse {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    pub media: MediaApercu,
    pub auteur: AuteurApercu,
}

#[derive(Debug, Serialize)]
pub struct PartageMediaListeResponse {
    pub partages: Vec<PartageMediaResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ────────────────────────────────────────────────────────────────
// Signalements (FR-049, FR-050)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SignalerMediaRequest {
    pub motif: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignalementMediaEtat {
    pub nombre_signalements: i32,
    pub suspendu: bool,
    pub deja_signale: bool,
}

// ── File de modération administrative (FR-051) ───────────────────

/// Filtres de la file des contenus signalés.
#[derive(Debug, Deserialize)]
pub struct SignalementsAdminFiltres {
    pub type_media: Option<String>,
    /// `true` : seuls les contenus déjà retirés de l'antenne.
    pub suspendu: Option<bool>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Une ligne de la file : le contenu visé, son état et ses signalements.
///
/// Les quatre tables n'ayant pas la même colonne de titre, la requête l'aliase
/// en `titre` — d'où une struct unique quel que soit `type_media`.
#[derive(Debug, FromRow)]
pub struct ContenuSignaleRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub etat: String,
    pub nombre_signalements: i32,
    pub image_couverture_url: Option<String>,
    pub dernier_signalement: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ContenuSignaleResponse {
    pub id: Uuid,
    pub type_media: String,
    pub titre: String,
    pub slug: Option<String>,
    pub etat: String,
    pub nombre_signalements: i32,
    pub image_couverture_url: Option<String>,
    pub url_detail: Option<String>,
    pub dernier_signalement: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ContenusSignalesListeResponse {
    pub contenus: Vec<ContenuSignaleResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Détail d'un signalement individuel, pour l'écran d'arbitrage.
#[derive(Debug, FromRow)]
pub struct SignalementDetailRow {
    pub id: Uuid,
    pub motif: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub signale_par: Uuid,
    pub auteur_nom: Option<String>,
    pub auteur_prenom: Option<String>,
    pub auteur_photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignalementDetailResponse {
    pub id: Uuid,
    pub motif: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub auteur: AuteurApercu,
}

/// Décision administrative sur un contenu signalé (FR-051).
#[derive(Debug, Deserialize)]
pub struct ChangerEtatMediaRequest {
    /// `publie` (rétablissement, remet le compteur à zéro), `suspendu` ou
    /// `supprime` (suppression définitive, soft delete).
    pub etat: String,
}

/// Les trois seules destinations admissibles d'une décision de modération.
pub const ETATS_MODERATION_MEDIA: [&str; 3] = ["publie", "suspendu", "supprime"];
