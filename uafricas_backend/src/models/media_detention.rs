//! Co-détention des supports médias — chaînes et stations
//! (feature 001-refonte-tele-radio, US5 — migration 09m).
//!
//! Plusieurs membres détiennent un même support, à des rôles distincts. Ces
//! modèles servent des routes **membre** : la garde n'est pas
//! `AdminUtilisateur` mais `garde_detenteur`, qui lit `support_detenteur`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::ApiErreur;

/// Les deux seuls types de support détenables, miroir de l'ENUM
/// `media_content.type_support_media`. Un contenu n'a pas de détenteur propre :
/// il appartient à son support.
pub const TYPES_SUPPORT_AUTORISES: [&str; 2] = ["chaine_tv", "station_radio"];

/// Rôles, du plus au moins étendu — l'ordre est signifiant, `role_au_moins`
/// s'en sert pour comparer.
pub const ROLES_DETENTEUR: [&str; 3] = ["proprietaire", "co_detenteur", "programmateur"];

/// Table du support, whitelistée : jamais d'interpolation de l'entrée brute
/// dans une requête (Principe IV).
pub fn table_pour_support(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some("media_content.chaine_tv"),
        "station_radio" => Some("media_content.station_radio"),
        _ => None,
    }
}

/// Table des contenus programmables sur ce type de support.
pub fn table_contenu_pour_support(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some("media_content.programme_tele"),
        "station_radio" => Some("media_content.programme_radio"),
        _ => None,
    }
}

pub fn valider_type_support(type_support: &str) -> Result<(), ApiErreur> {
    if TYPES_SUPPORT_AUTORISES.contains(&type_support) {
        Ok(())
    } else {
        Err(ApiErreur::Validation(format!(
            "Type de support « {} » non supporté",
            type_support
        )))
    }
}

/// Rang d'un rôle : 0 pour le plus étendu. Sert à écrire « au moins
/// co-détenteur » sans énumérer les rôles à chaque appel.
pub fn rang_role(role: &str) -> Option<usize> {
    ROLES_DETENTEUR.iter().position(|r| *r == role)
}

// ────────────────────────────────────────────────────────────────
// Détenteurs
// ────────────────────────────────────────────────────────────────

pub const SUPPORT_DETENTEUR_COLONNES: &str = "sd.id, sd.type_support::text AS type_support,
     sd.support_id, sd.utilisateur_id, sd.role::text AS role, sd.designe_par,
     sd.designe_at, sd.actif, sd.retire_at, sd.created_at, sd.updated_at";

#[derive(Debug, FromRow)]
pub struct SupportDetenteurRow {
    pub id: Uuid,
    pub type_support: String,
    pub support_id: Uuid,
    pub utilisateur_id: Uuid,
    pub role: String,
    pub designe_par: Option<Uuid>,
    pub designe_at: DateTime<Utc>,
    pub actif: bool,
    pub retire_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(default)]
    pub utilisateur_nom: Option<String>,
    #[sqlx(default)]
    pub utilisateur_prenom: Option<String>,
    #[sqlx(default)]
    pub utilisateur_email: Option<String>,
    #[sqlx(default)]
    pub utilisateur_photo: Option<String>,
    /// Renseigné par la liste « mes supports » uniquement.
    #[sqlx(default)]
    pub support_nom: Option<String>,
    #[sqlx(default)]
    pub support_slug: Option<String>,
    #[sqlx(default)]
    pub support_image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SupportDetenteurResponse {
    pub id: Uuid,
    pub type_support: String,
    pub support_id: Uuid,
    pub utilisateur_id: Uuid,
    pub role: String,
    pub designe_par: Option<Uuid>,
    pub designe_at: DateTime<Utc>,
    pub actif: bool,
    pub retire_at: Option<DateTime<Utc>>,
    pub utilisateur_nom: Option<String>,
    pub utilisateur_prenom: Option<String>,
    pub utilisateur_email: Option<String>,
    pub utilisateur_photo: Option<String>,
    pub support_nom: Option<String>,
    pub support_slug: Option<String>,
    pub support_image: Option<String>,
}

impl SupportDetenteurRow {
    pub fn to_response(self) -> SupportDetenteurResponse {
        SupportDetenteurResponse {
            id: self.id,
            type_support: self.type_support,
            support_id: self.support_id,
            utilisateur_id: self.utilisateur_id,
            role: self.role,
            designe_par: self.designe_par,
            designe_at: self.designe_at,
            actif: self.actif,
            retire_at: self.retire_at,
            utilisateur_nom: self.utilisateur_nom,
            utilisateur_prenom: self.utilisateur_prenom,
            utilisateur_email: self.utilisateur_email,
            utilisateur_photo: self.utilisateur_photo,
            support_nom: self.support_nom,
            support_slug: self.support_slug,
            support_image: self.support_image,
        }
    }
}

// ────────────────────────────────────────────────────────────────
// Invitations
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct InviterDetenteurRequest {
    pub email: String,
    /// Défaut `co_detenteur` — un propriétaire ne s'invite pas, il naît de la
    /// validation de la proposition (US4).
    pub role: Option<String>,
}

impl InviterDetenteurRequest {
    /// Le rôle invité ne peut pas être `proprietaire` : l'index unique
    /// `uq_support_un_proprietaire` n'en admet qu'un, et il est déjà pris par
    /// l'auteur du support. Refuser ici donne un message plutôt qu'une
    /// violation de contrainte.
    pub fn role_valide(&self) -> Result<&'static str, ApiErreur> {
        match self.role.as_deref().map(str::trim).unwrap_or("co_detenteur") {
            "co_detenteur" => Ok("co_detenteur"),
            "programmateur" => Ok("programmateur"),
            "proprietaire" => Err(ApiErreur::Validation(
                "Un support n'a qu'un seul propriétaire : invitez au rôle de co-détenteur ou de programmateur".into(),
            )),
            autre => Err(ApiErreur::Validation(format!(
                "Rôle de détenteur « {} » inconnu",
                autre
            ))),
        }
    }
}

/// Désignation directe par un administrateur — recours quand plus personne ne
/// peut inviter (support sans détenteur actif, propriétaire injoignable).
#[derive(Debug, Deserialize)]
pub struct AjouterDetenteurAdminRequest {
    pub utilisateur_id: Uuid,
    pub role: Option<String>,
}

pub const INVITATION_DETENTEUR_COLONNES: &str = "i.id, i.type_support::text AS type_support,
     i.support_id, i.email_invite, i.utilisateur_invite_id, i.role::text AS role,
     i.statut, i.invite_par, i.created_at, i.expire_at, i.traitee_le";

#[derive(Debug, FromRow)]
pub struct InvitationDetenteurRow {
    pub id: Uuid,
    pub type_support: String,
    pub support_id: Uuid,
    pub email_invite: String,
    pub utilisateur_invite_id: Option<Uuid>,
    pub role: String,
    pub statut: String,
    pub invite_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
    pub traitee_le: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub support_nom: Option<String>,
    #[sqlx(default)]
    pub support_slug: Option<String>,
    #[sqlx(default)]
    pub invite_par_nom: Option<String>,
    #[sqlx(default)]
    pub invite_par_prenom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct InvitationDetenteurResponse {
    pub id: Uuid,
    pub type_support: String,
    pub support_id: Uuid,
    pub email_invite: String,
    pub utilisateur_invite_id: Option<Uuid>,
    pub role: String,
    pub statut: String,
    pub invite_par: Uuid,
    pub invite_par_nom: Option<String>,
    pub invite_par_prenom: Option<String>,
    pub support_nom: Option<String>,
    pub support_slug: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
    pub traitee_le: Option<DateTime<Utc>>,
    /// Calculé à la lecture — une invitation périmée reste `en_attente` en base
    /// (aucune tâche de fond ne la bascule), c'est la lecture qui tranche.
    pub expiree: bool,
}

impl InvitationDetenteurRow {
    pub fn to_response(self) -> InvitationDetenteurResponse {
        let expiree = self.statut == "en_attente" && self.expire_at < Utc::now();
        InvitationDetenteurResponse {
            id: self.id,
            type_support: self.type_support,
            support_id: self.support_id,
            email_invite: self.email_invite,
            utilisateur_invite_id: self.utilisateur_invite_id,
            role: self.role,
            statut: self.statut,
            invite_par: self.invite_par,
            invite_par_nom: self.invite_par_nom,
            invite_par_prenom: self.invite_par_prenom,
            support_nom: self.support_nom,
            support_slug: self.support_slug,
            created_at: self.created_at,
            expire_at: self.expire_at,
            traitee_le: self.traitee_le,
            expiree,
        }
    }
}

// ────────────────────────────────────────────────────────────────
// Mise en relation (FR-046)
// ────────────────────────────────────────────────────────────────

pub const CONTENU_MESSAGE_MAX: usize = 2000;

#[derive(Debug, Deserialize)]
pub struct ContacterSupportRequest {
    pub message: String,
}
