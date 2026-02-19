use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────
pub const ADMIN_CODIMOI_LISTE_COLONNES: &str =
    "c.id, c.type::text AS type_codimoi, c.contenu, c.etat,
     c.nombre_likes, c.nombre_dislikes, c.created_at,
     p.nom AS pays_nom, c.groupe_ethnique,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const ADMIN_CODIMOI_DETAIL_COLONNES: &str =
    "c.id, c.type::text AS type_codimoi, c.contenu, c.explication,
     c.nom_auteur_originel, c.pays_id, c.groupe_ethnique,
     c.couleur_fond, c.image_couverture_url, c.image_arriere_plan_url,
     c.etat, c.nombre_likes, c.nombre_dislikes,
     c.cree_par, c.created_at, c.updated_at,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const CODIMOI_TRI_COLONNES: &[&str] = &[
    "created_at", "nombre_likes", "nombre_dislikes",
];

// ── Responses DTO ─────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminCodimoiListeResponse {
    pub id: Uuid,
    pub type_codimoi: Option<String>,
    pub contenu: String,
    pub etat: String,
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub pays_nom: Option<String>,
    pub groupe_ethnique: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminCodimoiDetailRow {
    pub id: Uuid,
    pub type_codimoi: Option<String>,
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
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

#[derive(Debug, Serialize)]
pub struct AdminCodimoiDetailResponse {
    pub id: Uuid,
    pub type_codimoi: Option<String>,
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
    pub cree_par: Uuid,
    pub cree_par_nom: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub tags: Vec<AdminCodimoiTagResponse>,
    pub nombre_commentaires: i64,
}

impl AdminCodimoiDetailRow {
    pub fn to_response(
        &self,
        tags: Vec<AdminCodimoiTagResponse>,
        nombre_commentaires: i64,
    ) -> AdminCodimoiDetailResponse {
        AdminCodimoiDetailResponse {
            id: self.id,
            type_codimoi: self.type_codimoi.clone(),
            contenu: self.contenu.clone(),
            explication: self.explication.clone(),
            nom_auteur_originel: self.nom_auteur_originel.clone(),
            pays_id: self.pays_id,
            groupe_ethnique: self.groupe_ethnique.clone(),
            couleur_fond: self.couleur_fond.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            image_arriere_plan_url: self.image_arriere_plan_url.clone(),
            etat: self.etat.clone(),
            nombre_likes: self.nombre_likes,
            nombre_dislikes: self.nombre_dislikes,
            cree_par: self.cree_par,
            cree_par_nom: format!("{} {}", self.auteur_prenom, self.auteur_nom),
            created_at: self.created_at,
            updated_at: self.updated_at,
            pays_nom: self.pays_nom.clone(),
            tags,
            nombre_commentaires,
        }
    }
}

// ── Tag ───────────────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminCodimoiTagResponse {
    pub tag_id: Uuid,
    pub tag_nom: String,
}

// ── Commentaire ───────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminCodimoiCommentaireRow {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub contenu: String,
    pub cree_par: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub nombre_likes: i32,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AdminCodimoiCommentaireResponse {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub contenu: String,
    pub cree_par: Uuid,
    pub auteur_nom: String,
    pub nombre_likes: i32,
    pub created_at: DateTime<Utc>,
    pub supprime: bool,
    pub enfants: Vec<AdminCodimoiCommentaireResponse>,
}

// ── Reaction stats ────────────────────────────────────────
#[derive(Debug, Serialize)]
pub struct AdminCodimoiReactionStats {
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
}

// ── Requests DTO ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct CreerCodimoiRequest {
    pub type_codimoi: String,
    pub contenu: String,
    pub explication: Option<String>,
    pub nom_auteur_originel: Option<String>,
    pub pays_id: Option<Uuid>,
    pub groupe_ethnique: Option<String>,
    pub couleur_fond: Option<String>,
    pub image_couverture_url: Option<String>,
    pub image_arriere_plan_url: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierCodimoiRequest {
    pub type_codimoi: Option<String>,
    pub contenu: Option<String>,
    pub explication: Option<String>,
    pub nom_auteur_originel: Option<String>,
    pub pays_id: Option<Uuid>,
    pub groupe_ethnique: Option<String>,
    pub couleur_fond: Option<String>,
    pub image_couverture_url: Option<String>,
    pub image_arriere_plan_url: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AjouterTagRequest {
    pub tag_id: Uuid,
}

// ── Query Params ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct AdminCodimoiQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub type_codimoi: Option<String>,
    pub pays_id: Option<Uuid>,
    pub groupe_ethnique: Option<String>,
    pub etat: Option<String>,
}
