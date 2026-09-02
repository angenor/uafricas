//! Équipes éditoriales des supports et des programmes médias
//! (feature 010-medias-equipes-vitrine, migration 09t).
//!
//! **Une seule table pour quatre porteurs.** `type_porteur` désigne deux
//! supports (`chaine_tv`, `station_radio`) et deux programmes (`emission_tele`,
//! `emission_radio`) ; les colonnes et toutes les règles sont identiques aux
//! deux niveaux. C'est le patron de `support_thematique` (09r), lui-même
//! décalque des quatre tables d'interactions (09k).
//!
//! Ne pas confondre avec `support_detenteur` (09m), qui porte des **droits** :
//! un membre d'équipe est une fiche descriptive et ne confère aucun accès.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::ApiErreur;

// ────────────────────────────────────────────────────────────────
// Discriminants
// ────────────────────────────────────────────────────────────────

pub const TYPES_PORTEUR: [&str; 4] = [
    "chaine_tv",
    "station_radio",
    "emission_tele",
    "emission_radio",
];

/// Au-delà, la saisie relève de l'import de fichier, pas d'un formulaire.
pub const MEMBRES_EQUIPE_MAX: usize = 60;

pub fn valider_type_porteur(type_porteur: &str) -> Result<(), ApiErreur> {
    if TYPES_PORTEUR.contains(&type_porteur) {
        Ok(())
    } else {
        Err(ApiErreur::Validation(format!(
            "Porteur « {} » inconnu : attendu chaine_tv, station_radio, emission_tele ou emission_radio",
            type_porteur
        )))
    }
}

/// Table du porteur. Littéraux fixes → interpolation SQL sûre (Principe IV).
pub fn table_porteur(type_porteur: &str) -> Result<&'static str, ApiErreur> {
    match type_porteur {
        "chaine_tv" => Ok("media_content.chaine_tv"),
        "station_radio" => Ok("media_content.station_radio"),
        "emission_tele" => Ok("media_content.emission_tele"),
        "emission_radio" => Ok("media_content.emission_radio"),
        _ => Err(ApiErreur::Validation(format!(
            "Porteur « {} » inconnu",
            type_porteur
        ))),
    }
}

/// Support auquel se rattache le porteur : c'est lui qui porte la détention, et
/// donc la garde d'écriture. Un programme n'a pas de détenteur propre.
pub fn type_support_du_porteur(type_porteur: &str) -> Result<&'static str, ApiErreur> {
    match type_porteur {
        "chaine_tv" | "emission_tele" => Ok("chaine_tv"),
        "station_radio" | "emission_radio" => Ok("station_radio"),
        _ => Err(ApiErreur::Validation(format!(
            "Porteur « {} » inconnu",
            type_porteur
        ))),
    }
}

/// Discriminant d'équipe d'un **programme**, déduit du type de son support.
/// Chemin inverse de `type_support_du_porteur`, employé aux points de
/// suppression, qui connaissent le support et non le porteur.
pub fn type_porteur_emission(type_support: &str) -> Result<&'static str, ApiErreur> {
    match type_support {
        "chaine_tv" => Ok("emission_tele"),
        "station_radio" => Ok("emission_radio"),
        _ => Err(ApiErreur::Validation(format!(
            "Type de support « {} » inconnu",
            type_support
        ))),
    }
}

/// `true` quand le porteur est un programme : le support doit alors être résolu
/// par `contexte_emission` avant la garde de détention.
pub fn est_programme(type_porteur: &str) -> bool {
    matches!(type_porteur, "emission_tele" | "emission_radio")
}

// ────────────────────────────────────────────────────────────────
// Lecture
// ────────────────────────────────────────────────────────────────

pub const MEMBRE_EQUIPE_COLONNES: &str = "m.id, m.type_porteur, m.porteur_id, m.nom, m.prenom, \
     m.fonction, m.territoire, m.contact, m.ordre, m.created_at, m.updated_at";

#[derive(Debug, FromRow)]
pub struct MembreEquipeRow {
    pub id: Uuid,
    pub type_porteur: String,
    pub porteur_id: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub fonction: String,
    pub territoire: Option<String>,
    pub contact: Option<String>,
    pub ordre: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Issu d'un `LEFT JOIN iam.utilisateur u ON u.id = m.utilisateur_id AND
    /// u.deleted_at IS NULL` : et **non** de la colonne `m.utilisateur_id`.
    /// `None` quand le compte n'existe plus : le nom s'affiche alors en texte
    /// simple, sans lien mort (FR-014).
    pub compte_id: Option<Uuid>,
}

impl MembreEquipeRow {
    pub fn to_response(self) -> MembreEquipeResponse {
        MembreEquipeResponse {
            id: self.id,
            nom: self.nom,
            prenom: self.prenom,
            fonction: self.fonction,
            territoire: self.territoire,
            contact: self.contact,
            utilisateur_id: self.compte_id,
            ordre: self.ordre,
        }
    }
}

/// Tout champ facultatif est **omis du JSON** quand il est vide : c'est FR-007
/// (« aucun libellé vide ») réalisé côté contrat autant que côté rendu.
#[derive(Debug, Clone, Serialize)]
pub struct MembreEquipeResponse {
    pub id: Uuid,
    pub nom: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prenom: Option<String>,
    pub fonction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub territoire: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    /// Renseigné SEULEMENT si le compte existe et n'est pas supprimé (FR-014).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilisateur_id: Option<Uuid>,
    pub ordre: i32,
}

// ────────────────────────────────────────────────────────────────
// Écriture : remplacement intégral (D6)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct MembreEquipeRequest {
    pub nom: String,
    #[serde(default)]
    pub prenom: Option<String>,
    pub fonction: String,
    #[serde(default)]
    pub territoire: Option<String>,
    #[serde(default)]
    pub contact: Option<String>,
    /// Facultatif (FR-013) : une équipe entièrement composée de non-inscrits est
    /// acceptée sans réserve.
    #[serde(default)]
    pub utilisateur_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct EquipeRequest {
    #[serde(default)]
    pub membres: Vec<MembreEquipeRequest>,
}

/// Vide une chaîne optionnelle réduite à des espaces : sans cela, un champ
/// laissé blanc par le formulaire produirait un libellé vide à l'écran, que
/// `skip_serializing_if` ne filtrerait pas, `Some("")` n'est pas `None`.
pub fn normaliser_optionnel(valeur: &Option<String>) -> Option<String> {
    valeur
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_string)
}

impl EquipeRequest {
    /// Une liste **vide est valide** : c'est ainsi qu'on supprime toute
    /// l'équipe, et c'est le seul moyen de vider un bloc.
    pub fn valider(&self) -> Result<(), ApiErreur> {
        if self.membres.len() > MEMBRES_EQUIPE_MAX {
            return Err(ApiErreur::Validation(format!(
                "Une équipe ne peut compter plus de {} personnes",
                MEMBRES_EQUIPE_MAX
            )));
        }
        for membre in &self.membres {
            if membre.nom.trim().is_empty() {
                return Err(ApiErreur::Validation(
                    "Le nom d'un membre de l'équipe est obligatoire".into(),
                ));
            }
            if membre.fonction.trim().is_empty() {
                return Err(ApiErreur::Validation(
                    "La fonction d'un membre de l'équipe est obligatoire".into(),
                ));
            }
        }
        Ok(())
    }
}
