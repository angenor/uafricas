use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ──────────────────────────────────────────────────────────────
// Réponses (contenu complet, réservé au back-office)
// ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminLeconResponse {
    pub id: Uuid,
    pub chapitre_id: Uuid,
    pub titre: String,
    pub contenu: Option<String>,
    pub video_url: Option<String>,
    pub document_url: Option<String>,
    pub duree_minutes: Option<i32>,
    pub ordre: i32,
}

#[derive(Debug, FromRow)]
pub struct AdminChapitreRow {
    pub id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub ordre: i32,
}

#[derive(Debug, Serialize)]
pub struct AdminChapitreResponse {
    pub id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub ordre: i32,
    pub lecons: Vec<AdminLeconResponse>,
}

// ──────────────────────────────────────────────────────────────
// Requêtes
// ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerChapitreRequest {
    pub titre: String,
    pub description: Option<String>,
}

/// PUT = remplacement complet du chapitre.
#[derive(Debug, Deserialize)]
pub struct ModifierChapitreRequest {
    pub titre: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreerLeconRequest {
    pub titre: String,
    pub contenu: Option<String>,
    pub video_url: Option<String>,
    pub document_url: Option<String>,
    pub duree_minutes: Option<i32>,
}

/// PUT = remplacement complet de la leçon (champs vides -> NULL).
#[derive(Debug, Deserialize)]
pub struct ModifierLeconRequest {
    pub titre: String,
    pub contenu: Option<String>,
    pub video_url: Option<String>,
    pub document_url: Option<String>,
    pub duree_minutes: Option<i32>,
}

/// Réordonnancement : la nouvelle position de chaque élément = son index dans `ids`.
#[derive(Debug, Deserialize)]
pub struct ReordonnerRequest {
    pub ids: Vec<Uuid>,
}
