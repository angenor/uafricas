//! Modèles pour les ressources contribuées par la communauté au niveau salle
//! (feature 001-ressources-fermeture-session, data-model.md §6).
//!
//! Une ressource contribuée est rattachée à une `afrolang.salle` (corpus cumulatif
//! partagé par toutes les sessions de la salle) et peut être de 4 types : document
//! téléversé, vidéo YouTube, lien web, ou recommandation d'accompagnateur (workflow
//! de consentement explicite a posteriori).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ══════════════════════════════════════════════════════════════════════════
// Enums
// ══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.type_ressource_contribuee", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TypeRessourceContribuee {
    Document,
    VideoYoutube,
    Accompagnateur,
    LienWeb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "afrolang.statut_accompagnateur", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum StatutAccompagnateur {
    EnAttente,
    Acceptee,
    Refusee,
    Retiree,
}

// ══════════════════════════════════════════════════════════════════════════
// Entité brute (FromRow)
// ══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct RessourceContribuee {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub session_origine_id: Option<Uuid>,
    pub auteur_id: Uuid,
    pub r#type: TypeRessourceContribuee,
    pub titre: String,
    pub description: Option<String>,
    pub fichier_url: Option<String>,
    pub fichier_taille_octets: Option<i64>,
    pub fichier_mime: Option<String>,
    pub video_url: Option<String>,
    pub video_id_youtube: Option<String>,
    pub lien_url: Option<String>,
    pub membre_recommande_id: Option<Uuid>,
    pub motif_recommandation: Option<String>,
    pub statut_accompagnateur: Option<StatutAccompagnateur>,
    pub motif_refus: Option<String>,
    pub reponse_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub supprime_par: Option<Uuid>,
}

// ══════════════════════════════════════════════════════════════════════════
// DTOs auxiliaires
// ══════════════════════════════════════════════════════════════════════════

/// Vue allégée d'un utilisateur (auteur d'une contribution ou membre recommandé).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuteurLight {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub avatar_url: Option<String>,
}

/// Info publique d'une recommandation d'accompagnateur.
///
/// Le champ `statut` reste exposé publiquement uniquement pour les recommandations
/// `acceptee` ; le filtrage de visibilité est appliqué côté handler de lecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccompagnateurPublicInfo {
    pub membre: AuteurLight,
    pub motif: String,
    pub statut: StatutAccompagnateur,
}

// ══════════════════════════════════════════════════════════════════════════
// DTOs de réponse
// ══════════════════════════════════════════════════════════════════════════

/// DTO public d'une ressource contribuée (champs internes admin masqués).
#[derive(Debug, Clone, Serialize)]
pub struct RessourceContribueeResponse {
    pub id: Uuid,
    pub r#type: TypeRessourceContribuee,
    pub titre: String,
    pub description: Option<String>,
    pub auteur: AuteurLight,
    pub session_origine_id: Option<Uuid>,
    pub fichier_url: Option<String>,
    pub fichier_taille_octets: Option<i64>,
    pub video_id_youtube: Option<String>,
    pub video_url: Option<String>,
    pub lien_url: Option<String>,
    pub accompagnateur: Option<AccompagnateurPublicInfo>,
    pub created_at: DateTime<Utc>,
}

/// DTO admin : ajoute les champs internes (auteur_id brut, supprime_par, deleted_at).
#[derive(Debug, Clone, Serialize)]
pub struct RessourceContribueeAdminResponse {
    pub id: Uuid,
    pub salle_id: Uuid,
    pub r#type: TypeRessourceContribuee,
    pub titre: String,
    pub description: Option<String>,
    pub auteur: AuteurLight,
    pub session_origine_id: Option<Uuid>,
    pub fichier_url: Option<String>,
    pub fichier_taille_octets: Option<i64>,
    pub fichier_mime: Option<String>,
    pub video_id_youtube: Option<String>,
    pub video_url: Option<String>,
    pub lien_url: Option<String>,
    pub accompagnateur: Option<AccompagnateurPublicInfo>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub supprime_par: Option<Uuid>,
}
