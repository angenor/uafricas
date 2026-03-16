// ════════════════════════════════════════════════════════════════════════════
// Modèles — Collaboration et partage d'arbres
// ════════════════════════════════════════════════════════════════════════════

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Structs BDD ────────────────────────────────────────────────────────

#[derive(Debug, sqlx::FromRow)]
pub struct Invitation {
    pub id: Uuid,
    pub arbre_id: Uuid,
    pub email_invite: String,
    pub utilisateur_invite_id: Option<Uuid>,
    pub permission: String,
    pub statut: String,
    pub invite_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub expire_at: Option<DateTime<Utc>>,
    pub traitee_le: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Collaborateur {
    pub id: Uuid,
    pub arbre_id: Uuid,
    pub utilisateur_id: Uuid,
    pub permission: String,
    pub invitation_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// ─── Row types pour jointures ───────────────────────────────────────────

#[derive(Debug, sqlx::FromRow)]
pub struct InvitationAvecProprietaireRow {
    pub id: Uuid,
    pub arbre_id: Uuid,
    pub permission: String,
    pub statut: String,
    pub created_at: DateTime<Utc>,
    pub proprietaire_nom: String,
    pub proprietaire_prenom: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CollaborateurAvecProfilRow {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub permission: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ArbrePartageRow {
    pub arbre_id: Uuid,
    pub proprietaire_nom: String,
    pub proprietaire_prenom: String,
    pub permission: String,
    pub nb_personnes: i64,
    pub partage_depuis: DateTime<Utc>,
}

// ─── DTOs de réponse ────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct InvitationResponse {
    pub id: Uuid,
    pub arbre_id: Uuid,
    pub permission: String,
    pub statut: String,
    pub proprietaire_nom: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CollaborateurResponse {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub permission: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ArbreResumeResponse {
    pub id: Uuid,
    pub nb_personnes: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ArbrePartageResponse {
    pub arbre_id: Uuid,
    pub proprietaire_nom: String,
    pub permission: String,
    pub nb_personnes: i64,
    pub partage_depuis: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MesArbresResponse {
    pub mon_arbre: Option<ArbreResumeResponse>,
    pub arbres_partages: Vec<ArbrePartageResponse>,
}

// ─── DTOs de requête ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerInvitationDto {
    pub email: String,
    pub permission: String,
}

#[derive(Debug, Deserialize)]
pub struct ModifierPermissionDto {
    pub permission: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfidentialiteArbreDto {
    pub arbre_prive: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfidentialitePersonneDto {
    pub visible_matching: bool,
}

// ─── Helper : vérifier accès à un arbre ─────────────────────────────────

/// Retourne la permission de l'utilisateur sur l'arbre, ou None s'il n'a pas accès.
/// "proprietaire" est retourné si l'utilisateur est le propriétaire.
pub async fn verifier_acces_arbre(
    pool: &sqlx::PgPool,
    utilisateur_id: Uuid,
    arbre_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    // Vérifier si propriétaire
    let est_proprietaire: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM arbre_genealogique.arbres
         WHERE id = $1 AND utilisateur_id = $2 AND deleted_at IS NULL",
    )
    .bind(arbre_id)
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?;

    if est_proprietaire.is_some() {
        return Ok(Some("proprietaire".to_string()));
    }

    // Vérifier si collaborateur
    let permission: Option<String> = sqlx::query_scalar(
        "SELECT permission FROM arbre_genealogique.collaborateurs
         WHERE arbre_id = $1 AND utilisateur_id = $2",
    )
    .bind(arbre_id)
    .bind(utilisateur_id)
    .fetch_optional(pool)
    .await?;

    Ok(permission)
}
