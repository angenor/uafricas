use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ─────────────────────────────────────────────

pub const ADMIN_VIDEO_LISTE_COLONNES: &str =
    "v.id, v.titre, v.slug, v.vignette_url, v.duree_secondes, v.etat,
     (SELECT COUNT(*) FROM media_content.piste_sous_titre p
      WHERE p.video_id = v.id AND p.deleted_at IS NULL)::INTEGER AS nombre_pistes,
     v.created_at";

pub const ADMIN_VIDEO_DETAIL_COLONNES: &str =
    "v.id, v.titre, v.slug, v.description, v.fichier_video_url,
     v.vignette_url, v.duree_secondes, v.taille_octets, v.format_video,
     v.etat, v.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     v.created_at, v.updated_at";

pub const VIDEO_TRI_COLONNES: &[&str] = &[
    "created_at", "titre", "etat", "duree_secondes",
];

// ── Langues valides ──────────────────────────────────────────

pub const LANGUES_VALIDES: &[&str] = &[
    "francais", "anglais", "arabe", "portugais", "swahili",
    "wolof", "haoussa", "amharique", "zoulou", "lingala",
    "bambara", "yoruba", "peul", "espagnol", "mandarin",
];

// ── Labels langues ───────────────────────────────────────────

pub fn label_langue(code: &str) -> &str {
    match code {
        "francais" => "Français",
        "anglais" => "Anglais",
        "arabe" => "Arabe",
        "portugais" => "Portugais",
        "swahili" => "Swahili",
        "wolof" => "Wolof",
        "haoussa" => "Haoussa",
        "amharique" => "Amharique",
        "zoulou" => "Zoulou",
        "lingala" => "Lingala",
        "bambara" => "Bambara",
        "yoruba" => "Yoruba",
        "peul" => "Peul",
        "espagnol" => "Espagnol",
        "mandarin" => "Mandarin",
        _ => code,
    }
}

// ── Vidéos — Liste ───────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminVideoListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub vignette_url: Option<String>,
    pub duree_secondes: Option<i32>,
    pub etat: String,
    pub nombre_pistes: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// ── Vidéos — Détail ──────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AdminVideoDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub description: Option<String>,
    pub fichier_video_url: String,
    pub vignette_url: Option<String>,
    pub duree_secondes: Option<i32>,
    pub taille_octets: Option<i64>,
    pub format_video: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminVideoDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: String,
    pub description: Option<String>,
    pub fichier_video_url: String,
    pub vignette_url: Option<String>,
    pub duree_secondes: Option<i32>,
    pub taille_octets: Option<i64>,
    pub format_video: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub pistes: Vec<AdminPisteSousTitreResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminVideoDetailRow {
    pub fn to_response(&self, pistes: Vec<AdminPisteSousTitreResponse>) -> AdminVideoDetailResponse {
        AdminVideoDetailResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            fichier_video_url: self.fichier_video_url.clone(),
            vignette_url: self.vignette_url.clone(),
            duree_secondes: self.duree_secondes,
            taille_octets: self.taille_octets,
            format_video: self.format_video.clone(),
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            pistes,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Query Params ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminVideoQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
}

// ── Requêtes de mutation ─────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ChangerEtatVideoRequest {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct CreerPisteRequest {
    pub langue: String,
}

#[derive(Debug, Deserialize)]
pub struct CreerSegmentRequest {
    pub texte: String,
    pub debut_ms: i32,
    pub fin_ms: i32,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSegmentRequest {
    pub texte: Option<String>,
    pub debut_ms: Option<i32>,
    pub fin_ms: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ReordonnerSegmentsRequest {
    pub ordre: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct TimingMotItem {
    pub position: i32,
    pub mot: String,
    pub debut_ms: i32,
    pub fin_ms: i32,
}

#[derive(Debug, Deserialize)]
pub struct EnregistrerTimingsMotRequest {
    pub timings: Vec<TimingMotItem>,
}

// ── Pistes — Response ────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminPisteSousTitreResponse {
    pub id: Uuid,
    pub langue: String,
    pub est_complete: bool,
    pub nombre_segments: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// ── Segments — Response ──────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AdminSegmentSousTitreResponse {
    pub id: Uuid,
    pub position: i32,
    pub texte: String,
    pub debut_ms: i32,
    pub fin_ms: i32,
    pub timings_mot: Vec<TimingMotResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSegmentRow {
    pub id: Uuid,
    pub position: i32,
    pub texte: String,
    pub debut_ms: i32,
    pub fin_ms: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TimingMotResponse {
    pub position: i32,
    pub mot: String,
    pub debut_ms: i32,
    pub fin_ms: i32,
}

// ── Slug ─────────────────────────────────────────────────────

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
