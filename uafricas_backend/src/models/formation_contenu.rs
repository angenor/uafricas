use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

// ──────────────────────────────────────────────────────────────
// Lignes BD (partagées public + admin)
// ──────────────────────────────────────────────────────────────

/// Une ligne de media_content.formation_chapitre
#[derive(Debug, FromRow)]
pub struct ChapitreRow {
    pub id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub ordre: i32,
}

/// Une ligne de media_content.formation_lecon
#[derive(Debug, FromRow)]
pub struct LeconRow {
    pub id: Uuid,
    pub chapitre_id: Uuid,
    pub titre: String,
    pub contenu: Option<String>,
    pub video_url: Option<String>,
    pub document_url: Option<String>,
    pub duree_minutes: Option<i32>,
    pub ordre: i32,
}

// ──────────────────────────────────────────────────────────────
// DTOs de réponse publics
// ──────────────────────────────────────────────────────────────

/// Une leçon telle qu'exposée publiquement.
/// Les champs de contenu (`contenu`, `video_url`, `document_url`) ne sont
/// renseignés que lorsque `accessible == true` (inscrit ou créateur).
#[derive(Debug, Serialize)]
pub struct LeconPublicResponse {
    pub id: Uuid,
    pub titre: String,
    pub duree_minutes: Option<i32>,
    pub ordre: i32,
    /// Le contenu est-il servi à l'appelant ?
    pub accessible: bool,
    /// Présence d'une vidéo (vrai même si non accessible, pour l'icône « lecture »).
    pub a_video: bool,
    /// Présence d'un document (vrai même si non accessible).
    pub a_document: bool,
    pub contenu: Option<String>,
    pub video_url: Option<String>,
    pub document_url: Option<String>,
    /// La leçon est-elle marquée terminée par l'utilisateur courant ?
    pub terminee: bool,
}

/// Un chapitre avec ses leçons.
#[derive(Debug, Serialize)]
pub struct ChapitrePublicResponse {
    pub id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub ordre: i32,
    pub lecons: Vec<LeconPublicResponse>,
}

/// Le programme complet d'une formation.
#[derive(Debug, Serialize)]
pub struct FormationContenuResponse {
    pub est_inscrit: bool,
    /// est_inscrit OU créateur de la formation : le contenu lui est servi.
    pub accessible: bool,
    pub nombre_lecons: i64,
    pub nombre_lecons_terminees: i64,
    pub progression: f64,
    pub chapitres: Vec<ChapitrePublicResponse>,
}

/// Réponse après (dé)marquage d'une leçon.
#[derive(Debug, Serialize)]
pub struct ProgressionResponse {
    pub progression: f64,
    pub nombre_lecons: i64,
    pub nombre_lecons_terminees: i64,
    /// État de la leçon ciblée après l'action.
    pub terminee: bool,
}
