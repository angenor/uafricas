//! Émissions — les **programmes conteneurs** des chaînes et des stations
//! (feature 009-medias-programmes-episodes, migration 09q).
//!
//! Vocabulaire : `emission_*` en base et dans le code = « **Programme** » à
//! l'écran ; `episode_*` = « **Épisode** ». Le décalage est délibéré — le mot
//! « programme » désignait déjà l'unité diffusable avant 09q, et réutiliser
//! l'identifiant aurait fait lire silencieusement une table de sens opposé
//! (research.md R1).
//!
//! **Un seul jeu de structs sert la télé et la radio.** Les deux tables ne
//! diffèrent que par le nom de leur clé de rattachement (`chaine_id` /
//! `station_id`) et par `categorie_radio` : les constantes de colonnes les
//! aplanissent par alias (`AS support_id`, `NULL::text AS categorie_radio`).
//! Dupliquer les structs n'aurait apporté aucune garantie supplémentaire, le
//! projet interrogeant PostgreSQL par requêtes runtime.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::media_episode::EpisodeResponse;
use crate::models::media_social::CompteursInteraction;

// ────────────────────────────────────────────────────────────────
// Cadence — FR-013
// ────────────────────────────────────────────────────────────────

/// Périodicité déclarée d'un programme. Alimente les alertes de FR-024 ; la
/// **rotation**, elle, se déduit de la récurrence du créneau et non de cette
/// valeur — un programme hebdomadaire peut être diffusé tous les jours.
pub const CADENCES_AUTORISEES: [&str; 3] = ["quotidienne", "hebdomadaire", "ponctuelle"];

pub fn valider_cadence(valeur: &str) -> Result<(), ApiErreur> {
    if CADENCES_AUTORISEES.contains(&valeur) {
        Ok(())
    } else {
        Err(ApiErreur::Validation(format!(
            "Cadence « {} » inconnue : attendu quotidienne, hebdomadaire ou ponctuelle",
            valeur
        )))
    }
}

/// Marge d'anticipation de l'alerte de cadence (FR-024, contrat membre §5).
/// Elle doit absorber le délai de validation administrative : prévenir le jour
/// même laisserait le détenteur sans recours.
pub fn heures_anticipation_alerte(cadence: &str) -> Option<i64> {
    match cadence {
        "hebdomadaire" => Some(48),
        "quotidienne" => Some(6),
        _ => None, // « ponctuelle » : aucune échéance, donc aucune alerte
    }
}

// ────────────────────────────────────────────────────────────────
// Aiguillage télé / radio
// ────────────────────────────────────────────────────────────────

/// Table d'émissions d'un type de support. Littéraux fixes → interpolation SQL
/// sûre (Principe IV : jamais l'entrée brute dans une requête).
pub fn table_emission(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some("media_content.emission_tele"),
        "station_radio" => Some("media_content.emission_radio"),
        _ => None,
    }
}

/// Colonne de rattachement au support, propre à chaque table.
pub fn colonne_support(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some("chaine_id"),
        "station_radio" => Some("station_id"),
        _ => None,
    }
}

/// Type de média (discriminant des interactions 09k) porté par l'émission.
pub fn type_media_emission(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some("emission_tele"),
        "station_radio" => Some("emission_radio"),
        _ => None,
    }
}

/// Type de support déduit du type de média d'une émission — opération inverse.
pub fn support_pour_type_media_emission(type_media: &str) -> Option<&'static str> {
    match type_media {
        "emission_tele" => Some("chaine_tv"),
        "emission_radio" => Some("station_radio"),
        _ => None,
    }
}

// ────────────────────────────────────────────────────────────────
// Colonnes SQL
// ────────────────────────────────────────────────────────────────

/// Alias `e`. `chaine_id` est exposé sous `support_id` pour que la même struct
/// serve les deux familles ; `categorie_radio` est neutralisé côté télé.
pub const EMISSION_TELE_COLONNES: &str =
    "e.id, e.chaine_id AS support_id, e.titre, e.slug, e.description,
     e.image_couverture_url, e.info_animateur, e.info_producteur, e.langue,
     NULL::text AS categorie_radio,
     e.theme_phare_id, e.theme_phare_autre, e.cadence, e.etat,
     e.nombre_signalements, e.cree_par, e.created_at, e.updated_at";

pub const EMISSION_RADIO_COLONNES: &str =
    "e.id, e.station_id AS support_id, e.titre, e.slug, e.description,
     e.image_couverture_url, e.info_animateur, e.info_producteur, e.langue,
     e.categorie_radio::text AS categorie_radio,
     e.theme_phare_id, e.theme_phare_autre, e.cadence, e.etat,
     e.nombre_signalements, e.cree_par, e.created_at, e.updated_at";

pub fn colonnes_emission(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some(EMISSION_TELE_COLONNES),
        "station_radio" => Some(EMISSION_RADIO_COLONNES),
        _ => None,
    }
}

// ────────────────────────────────────────────────────────────────
// Struct DB
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct EmissionRow {
    pub id: Uuid,
    pub support_id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub langue: String,
    /// Renseigné côté radio uniquement.
    pub categorie_radio: Option<String>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub cadence: String,
    pub etat: String,
    pub nombre_signalements: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // ── Jointures et agrégats optionnels ────────────────────────
    #[sqlx(default)]
    pub support_nom: Option<String>,
    #[sqlx(default)]
    pub support_slug: Option<String>,
    #[sqlx(default)]
    pub theme_phare_nom: Option<String>,
    /// Épisodes **publiés** uniquement (FR-012).
    #[sqlx(default)]
    pub nombre_episodes: Option<i64>,
    #[sqlx(default)]
    pub dernier_episode_at: Option<DateTime<Utc>>,
    /// Décomptes de la vue détenteur (FR-042) — jamais servis au public.
    #[sqlx(default)]
    pub episodes_en_attente: Option<i64>,
    #[sqlx(default)]
    pub episodes_rejetes: Option<i64>,
    /// Nombre total d'émissions du support, rapporté par la fenêtre de la
    /// requête de sections : il évite un second aller-retour de comptage.
    #[sqlx(default)]
    pub total_emissions: Option<i64>,
}

// ────────────────────────────────────────────────────────────────
// DTO
// ────────────────────────────────────────────────────────────────

/// Référence nommée légère — thème phare, chaîne, station.
#[derive(Debug, Clone, Serialize)]
pub struct RefNommee {
    pub id: Uuid,
    pub nom: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmissionResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub langue: String,
    pub cadence: String,
    pub etat: String,
    /// `chaine_tv` ou `station_radio` — l'appelant sait déjà lequel, mais les
    /// composants partagés du frontend, non.
    pub type_support: String,
    pub support_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<RefNommee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorie_radio: Option<String>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_phare: Option<RefNommee>,
    /// Épisodes **publiés** (FR-012). 0 quand l'agrégat n'a pas été demandé.
    pub nombre_episodes: i64,
    pub dernier_episode_at: Option<DateTime<Utc>>,
    /// Borné à 12 par les endpoints de sections — au-delà, la page du programme.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub episodes_apercu: Vec<EpisodeResponse>,
    /// Vue détenteur et back-office seulement (FR-042).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes_en_attente: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episodes_rejetes: Option<i64>,
    pub nombre_signalements: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Compteurs de **l'émission seule** — jamais la somme de ceux de ses
    /// épisodes (FR-048).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<CompteursInteraction>,
}

impl EmissionRow {
    pub fn to_response(&self, type_support: &str) -> EmissionResponse {
        EmissionResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            info_animateur: self.info_animateur.clone(),
            info_producteur: self.info_producteur.clone(),
            langue: self.langue.clone(),
            cadence: self.cadence.clone(),
            etat: self.etat.clone(),
            type_support: type_support.to_string(),
            support_id: self.support_id,
            support: self.support_nom.as_ref().map(|nom| RefNommee {
                id: self.support_id,
                nom: nom.clone(),
                slug: self.support_slug.clone(),
            }),
            categorie_radio: self.categorie_radio.clone(),
            theme_phare_id: self.theme_phare_id,
            theme_phare_autre: self.theme_phare_autre.clone(),
            theme_phare: match (self.theme_phare_id, self.theme_phare_nom.as_ref()) {
                (Some(id), Some(nom)) => Some(RefNommee {
                    id,
                    nom: nom.clone(),
                    slug: None,
                }),
                _ => None,
            },
            nombre_episodes: self.nombre_episodes.unwrap_or(0),
            dernier_episode_at: self.dernier_episode_at,
            episodes_apercu: Vec::new(),
            episodes_en_attente: self.episodes_en_attente,
            episodes_rejetes: self.episodes_rejetes,
            nombre_signalements: self.nombre_signalements,
            cree_par: self.cree_par,
            created_at: self.created_at,
            updated_at: self.updated_at,
            interactions: None,
        }
    }
}

// ────────────────────────────────────────────────────────────────
// Requêtes entrantes
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EmissionRequest {
    pub titre: String,
    pub description: Option<String>,
    /// Défaut `ponctuelle` — une émission naît sans engagement de périodicité.
    pub cadence: Option<String>,
    pub image_couverture_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub langue: Option<String>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    /// Radio uniquement, ignoré côté télé.
    pub categorie_radio: Option<String>,
    /// Réservé à l'administration : un co-détenteur ne choisit pas l'état.
    pub etat: Option<String>,
}

impl EmissionRequest {
    pub fn valider(&self) -> Result<(), ApiErreur> {
        if self.titre.trim().is_empty() {
            return Err(ApiErreur::Validation(
                "Le titre du programme est obligatoire".into(),
            ));
        }
        if let Some(cadence) = self.cadence.as_deref() {
            valider_cadence(cadence)?;
        }
        Ok(())
    }

    pub fn cadence_ou_defaut(&self) -> &str {
        self.cadence.as_deref().unwrap_or("ponctuelle")
    }
}

#[derive(Debug, Deserialize)]
pub struct EmissionQueryParams {
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub cadence: Option<String>,
    pub support_id: Option<Uuid>,
    /// `tele` | `radio` — filtre de la liste back-office (FR-046).
    pub r#type: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}
