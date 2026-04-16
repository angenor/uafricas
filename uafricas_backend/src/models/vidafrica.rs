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
    pub langues_disponibles: Vec<String>,
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
    pub created_at: DateTime<Utc>,
}

// ── Sous-titres publics ──────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct SousTitresResponse {
    pub langue: String,
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
