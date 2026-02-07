use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour le mapping culture.codimoi
/// Le cast type::text convertit l'enum PostgreSQL en texte pour sqlx
pub const CODIMOI_COLONNES: &str =
    "id, type::text AS type, contenu, explication, nom_auteur_originel,
     pays_id, groupe_ethnique, couleur_fond, image_couverture_url,
     image_arriere_plan_url, etat, nombre_likes, nombre_dislikes, nombre_vues,
     cree_par, created_at, updated_at";

/// Representation d'un post codimoi en base de donnees
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CodiMoi {
    pub id: Uuid,
    #[sqlx(rename = "type")]
    pub type_post: String,
    pub contenu: String,
    pub explication: Option<String>,
    pub nom_auteur_originel: Option<String>,
    pub pays_id: Option<Uuid>,
    pub groupe_ethnique: Option<String>,
    pub couleur_fond: Option<String>,
    pub image_couverture_url: Option<String>,
    pub image_arriere_plan_url: Option<String>,
    pub etat: String,
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub nombre_vues: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Info utilisateur jointe depuis iam.utilisateur
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuteurInfo {
    pub id: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
}

/// Tag/hashtag joint depuis shared.tag
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TagInfo {
    pub nom: String,
}

/// Commentaire en base de donnees
#[derive(Debug, FromRow)]
pub struct CommentaireRow {
    pub id: Uuid,
    pub codimoi_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub contenu: String,
    pub cree_par: Uuid,
    pub nombre_likes: i32,
    pub created_at: DateTime<Utc>,
}

// ──────────────────────────────────────────────────────────────
// DTOs de reponse
// ──────────────────────────────────────────────────────────────

/// DTO pour un post dans la liste
#[derive(Debug, Serialize)]
pub struct CodiMoiResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub type_post: String,
    pub contenu: String,
    pub explication: Option<String>,
    pub nom_auteur_originel: Option<String>,
    pub pays: Option<String>,
    pub groupe_ethnique: Option<String>,
    pub couleur_fond: Option<String>,
    pub image_couverture_url: Option<String>,
    pub image_arriere_plan_url: Option<String>,
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub nombre_vues: i32,
    pub nombre_commentaires: i64,
    pub hashtags: Vec<String>,
    pub auteur: CodiMoiAuteurResponse,
    pub user_reaction: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// DTO pour l'auteur d'un post
#[derive(Debug, Serialize)]
pub struct CodiMoiAuteurResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
}

/// DTO pour la liste paginee
#[derive(Debug, Serialize)]
pub struct CodiMoiListeResponse {
    pub posts: Vec<CodiMoiResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
}

/// DTO pour un commentaire
#[derive(Debug, Serialize)]
pub struct CommentaireResponse {
    pub id: Uuid,
    pub contenu: String,
    pub parent_id: Option<Uuid>,
    pub nombre_likes: i32,
    pub auteur: CodiMoiAuteurResponse,
    pub created_at: DateTime<Utc>,
}

/// DTO pour la liste de commentaires
#[derive(Debug, Serialize)]
pub struct CommentaireListeResponse {
    pub commentaires: Vec<CommentaireResponse>,
    pub total: i64,
}

/// Parametres de requete pour le listing
#[derive(Debug, Deserialize)]
pub struct CodiMoiQueryParams {
    pub recherche: Option<String>,
    #[serde(rename = "type")]
    pub type_post: Option<String>,
    pub pays: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Corps de la requete de creation
#[derive(Debug, Deserialize)]
pub struct CreerCodiMoiRequest {
    #[serde(rename = "type")]
    pub type_post: String,
    pub contenu: String,
    pub explication: Option<String>,
    pub nom_auteur_originel: Option<String>,
    pub pays: Option<String>,
    pub groupe_ethnique: Option<String>,
    pub couleur_fond: Option<String>,
    pub hashtags: Option<Vec<String>>,
}

/// Corps de la requete de reaction
#[derive(Debug, Deserialize)]
pub struct ReactionRequest {
    pub type_reaction: String,
}

/// Corps de la requete de commentaire
#[derive(Debug, Deserialize)]
pub struct CreerCommentaireRequest {
    pub contenu: String,
    pub parent_id: Option<Uuid>,
}
