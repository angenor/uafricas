use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Vidéo publique — Liste ───────────────────────────────────

#[derive(Debug, Serialize)]
pub struct VideoPubliqueListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub description: Option<String>,
    pub vignette_url: Option<String>,
    pub duree_secondes: Option<i32>,
    pub langues_disponibles: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct VideoPubliqueListeRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub description: Option<String>,
    pub vignette_url: Option<String>,
    pub duree_secondes: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// ── Vidéo publique — Détail ──────────────────────────────────

#[derive(Debug, Serialize)]
pub struct VideoPubliqueDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub description: Option<String>,
    pub fichier_video_url: String,
    pub vignette_url: Option<String>,
    pub duree_secondes: Option<i32>,
    pub territoires: Vec<String>,
    pub auteur_reel: Option<String>,
    pub langues_disponibles: Vec<String>,
    pub nombre_likes: i64,
    pub nombre_dislikes: i64,
    pub nombre_partages: i64,
    /// Réaction du membre connecté : "like" | "dislike" | null
    pub ma_reaction: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct VideoPubliqueDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub description: Option<String>,
    pub fichier_video_url: String,
    pub vignette_url: Option<String>,
    pub duree_secondes: Option<i32>,
    pub territoires: Vec<String>,
    pub auteur_reel: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── Sous-titres publics ──────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct SousTitresResponse {
    pub langue: String,
    /// Auteur de la piste publiée (déresponsabilisation : affiché sous la vidéo).
    pub auteur: Option<String>,
    pub segments: Vec<SegmentPubliqueResponse>,
}

#[derive(Debug, Serialize)]
pub struct SegmentPubliqueResponse {
    pub position: i32,
    pub texte: String,
    pub debut_ms: i32,
    pub fin_ms: i32,
    pub mots: Vec<MotTimingResponse>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MotTimingResponse {
    pub position: i32,
    pub mot: String,
    pub debut_ms: i32,
    pub fin_ms: i32,
}

// ── Langues disponibles ──────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct LangueDisponibleResponse {
    pub code: String,
    pub label: String,
    pub nombre_videos: i64,
}

// ── Query Params ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct VideoPubliqueQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub recherche: Option<String>,
    pub langue: Option<String>,
}

// ── Réactions (like / dislike) ───────────────────────────────

#[derive(Debug, Deserialize)]
pub struct VideoReactionRequest {
    pub type_reaction: String, // "like" | "dislike"
}

#[derive(Debug, Serialize)]
pub struct VideoReactionResponse {
    pub nombre_likes: i64,
    pub nombre_dislikes: i64,
    /// Réaction du membre connecté après bascule : "like" | "dislike" | null
    pub ma_reaction: Option<String>,
}

// ── Partage de vidéo (mur /publications) ─────────────────────

#[derive(Debug, Deserialize)]
pub struct PartageVideoRequest {
    pub legende: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PartageVideoQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Ligne brute du JOIN partage + vidéo partagée + auteur.
#[derive(Debug, FromRow)]
pub struct PartageVideoRow {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    // Vidéo partagée
    pub video_id: Uuid,
    pub video_titre: String,
    pub video_slug: String,
    pub video_vignette_url: Option<String>,
    pub video_duree_secondes: Option<i32>,
    // Auteur du partage
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartageVideoApercu {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub vignette_url: Option<String>,
    pub duree_secondes: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PartageVideoAuteur {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartageVideoResponse {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    pub video: PartageVideoApercu,
    pub auteur: PartageVideoAuteur,
}

#[derive(Debug, Serialize)]
pub struct PartageVideoListeResponse {
    pub partages: Vec<PartageVideoResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

impl From<PartageVideoRow> for PartageVideoResponse {
    fn from(row: PartageVideoRow) -> Self {
        PartageVideoResponse {
            id: row.id,
            legende: row.legende,
            created_at: row.created_at,
            video: PartageVideoApercu {
                id: row.video_id,
                titre: row.video_titre,
                slug: row.video_slug,
                vignette_url: row.video_vignette_url,
                duree_secondes: row.video_duree_secondes,
            },
            auteur: PartageVideoAuteur {
                id: row.auteur_id,
                nom: row.auteur_nom,
                prenom: row.auteur_prenom,
                photo_url: row.auteur_photo_url,
            },
        }
    }
}
