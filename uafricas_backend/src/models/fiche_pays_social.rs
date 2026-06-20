use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Seuil de signalements distincts au-delà duquel une fiche est bloquée
/// (retirée du public). Aligné sur le pattern de modération communautaire
/// des factchecks (cf. governance.factcheck_signalement, seuil 20).
pub const SEUIL_SIGNALEMENTS_BLOCAGE: i32 = 10;

// ────────────────────────────────────────────────────────────────
// Réactions like / dislike
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ReactionFicheRequest {
    /// "like" ou "dislike"
    pub type_reaction: String,
}

#[derive(Debug, Serialize)]
pub struct ReactionFicheEtat {
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    /// Réaction du membre courant : "like" | "dislike" | null
    pub ma_reaction: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Signalements
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SignalementFicheRequest {
    pub motif: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignalementFicheEtat {
    pub nombre_signalements: i32,
    pub bloquee: bool,
    pub a_signale: bool,
}

// ────────────────────────────────────────────────────────────────
// Partages vers le mur communautaire
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PartageFicheRequest {
    pub legende: Option<String>,
}

/// Ligne brute du partage enrichie (JOIN fiche_pays + shared.pays + iam.utilisateur)
#[derive(Debug, FromRow)]
pub struct PartageFicheRow {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    pub fiche_id: Uuid,
    pub pays_nom: String,
    pub pays_code: Option<String>,
    pub pays_capitale: Option<String>,
    pub slogan: Option<String>,
    pub image_couverture_url: Option<String>,
    pub image_drapeau_url: Option<String>,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartageAuteur {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartageFicheApercu {
    pub id: Uuid,
    pub nom: String,
    pub capitale: Option<String>,
    pub slogan: Option<String>,
    pub image_couverture: Option<String>,
    pub drapeau_url: Option<String>,
    pub region: String,
}

#[derive(Debug, Serialize)]
pub struct PartageFicheResponse {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    pub fiche: PartageFicheApercu,
    pub auteur: PartageAuteur,
}

#[derive(Debug, Serialize)]
pub struct PartageFicheListeResponse {
    pub partages: Vec<PartageFicheResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize)]
pub struct PartageQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}
