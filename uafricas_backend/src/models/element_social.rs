//! Réactions (like/dislike) et partages communautaires des SOUS-OBJETS afripulse
//! (secteur, recette, site touristique, personnalité). Design générique par
//! (type_objet, objet_id) — cf. migration 11k. Calqué sur `fiche_pays_social`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Types de sous-objets partageables / réactionnables (sous-ensemble de
/// l'enum country_profile.type_objet_contribution).
pub const TYPES_ELEMENT_AUTORISES: [&str; 4] = [
    "secteur_developpement",
    "recette_culinaire",
    "site_touristique",
    "personnalite_connue",
];

/// Table + présence d'une colonne `deleted_at` pour un type d'objet donné.
/// Renvoie None si le type n'est pas supporté.
pub fn table_pour_type(type_objet: &str) -> Option<(&'static str, bool)> {
    match type_objet {
        "secteur_developpement" => Some(("country_profile.secteur_developpement", false)),
        "recette_culinaire" => Some(("country_profile.recette_culinaire", true)),
        "site_touristique" => Some(("country_profile.site_touristique", false)),
        "personnalite_connue" => Some(("country_profile.personnalite_connue", true)),
        _ => None,
    }
}

// ────────────────────────────────────────────────────────────────
// Réactions like / dislike
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ReactionElementRequest {
    /// "like" ou "dislike"
    pub type_reaction: String,
}

#[derive(Debug, Serialize)]
pub struct ReactionElementEtat {
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    /// Réaction du membre courant : "like" | "dislike" | null
    pub ma_reaction: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Partages vers le mur communautaire
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PartageElementRequest {
    pub legende: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PartageQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Ligne brute enrichie (UNION des 4 types → sous-objet + fiche + pays + auteur)
#[derive(Debug, FromRow)]
pub struct PartageElementRow {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    pub type_objet: String,
    pub objet_id: Uuid,
    pub fiche_pays_id: Uuid,
    pub territoire_nom: String,
    pub titre: String,
    pub image_url: Option<String>,
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
pub struct PartageElementApercu {
    /// 'secteur_developpement' | 'recette_culinaire' | 'site_touristique' | 'personnalite_connue'
    pub type_objet: String,
    pub objet_id: Uuid,
    pub fiche_pays_id: Uuid,
    pub territoire_nom: String,
    pub titre: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PartageElementResponse {
    pub id: Uuid,
    pub legende: Option<String>,
    pub created_at: DateTime<Utc>,
    pub element: PartageElementApercu,
    pub auteur: PartageAuteur,
}

#[derive(Debug, Serialize)]
pub struct PartageElementListeResponse {
    pub partages: Vec<PartageElementResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}
