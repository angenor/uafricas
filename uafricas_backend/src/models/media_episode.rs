//! Épisodes — les **unités diffusables** d'un programme
//! (feature 009-medias-programmes-episodes, migration 09q).
//!
//! Un épisode porte le média (`video_url` en télé, `audio_url` en radio), son
//! rang dans le programme, et son état de modération. Ses `id` et `slug` sont
//! **repris à l'identique** des anciennes lignes `programme_*` : les
//! interactions (09k) et les adresses publiques suivent sans réécriture.
//!
//! Comme pour les émissions, un seul jeu de structs sert les deux familles ; les
//! constantes de colonnes aplanissent `video_url` / `audio_url` sous
//! `media_url`, et neutralisent `a_la_une_globale` côté radio (la vedette plein
//! écran n'existe que sur l'espace Télé).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::media_emission::RefNommee;
use crate::models::media_social::CompteursInteraction;

// ────────────────────────────────────────────────────────────────
// États — FR-040, FR-041
// ────────────────────────────────────────────────────────────────

/// Cycle de vie d'un épisode. `rejete` est la valeur ajoutée par 09q : elle rend
/// le refus **motivé et corrigeable**, là où l'ancien modèle ne connaissait que
/// la suppression.
pub const ETATS_EPISODE: [&str; 6] = [
    "brouillon",
    "en_attente",
    "publie",
    "rejete",
    "suspendu",
    "supprime",
];

/// Longueur minimale d'un motif de rejet — même garde applicative que le rejet
/// d'une proposition média (09l). Un « non » sans explication laisse l'auteur
/// sans rien à corriger.
pub const MOTIF_REJET_MIN: usize = 10;

pub fn valider_etat_episode(valeur: &str) -> Result<(), ApiErreur> {
    if ETATS_EPISODE.contains(&valeur) {
        Ok(())
    } else {
        Err(ApiErreur::Validation(format!(
            "État d'épisode « {} » inconnu",
            valeur
        )))
    }
}

// ────────────────────────────────────────────────────────────────
// Aiguillage télé / radio
// ────────────────────────────────────────────────────────────────

pub fn table_episode(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some("media_content.episode_tele"),
        "station_radio" => Some("media_content.episode_radio"),
        _ => None,
    }
}

/// Colonne portant le média, propre à chaque table.
pub fn colonne_media(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some("video_url"),
        "station_radio" => Some("audio_url"),
        _ => None,
    }
}

/// Type de média (discriminant des interactions 09k) porté par l'épisode.
pub fn type_media_episode(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some("episode_tele"),
        "station_radio" => Some("episode_radio"),
        _ => None,
    }
}

pub fn support_pour_type_media_episode(type_media: &str) -> Option<&'static str> {
    match type_media {
        "episode_tele" => Some("chaine_tv"),
        "episode_radio" => Some("station_radio"),
        _ => None,
    }
}

/// `tele` | `radio` → type de support. Sert les filtres du back-office.
pub fn support_pour_famille(famille: &str) -> Option<&'static str> {
    match famille {
        "tele" => Some("chaine_tv"),
        "radio" => Some("station_radio"),
        _ => None,
    }
}

pub fn famille_pour_support(type_support: &str) -> &'static str {
    match type_support {
        "station_radio" => "radio",
        _ => "tele",
    }
}

// ────────────────────────────────────────────────────────────────
// Colonnes SQL
// ────────────────────────────────────────────────────────────────

/// Alias `ep`. `a_la_une_globale` est neutralisé côté radio — la colonne
/// n'existe pas, l'espace Radio n'ayant pas de vedette plein écran.
pub const EPISODE_TELE_COLONNES: &str =
    "ep.id, ep.emission_id, ep.titre, ep.slug, ep.description,
     ep.image_couverture_url, ep.video_url AS media_url, ep.numero_episode,
     ep.ordre, ep.duree_minutes, ep.a_la_une, ep.a_la_une_globale, ep.etat,
     ep.motif_rejet, ep.valide_par, ep.valide_at, ep.nombre_signalements,
     ep.cree_par, ep.created_at, ep.updated_at";

pub const EPISODE_RADIO_COLONNES: &str =
    "ep.id, ep.emission_id, ep.titre, ep.slug, ep.description,
     ep.image_couverture_url, ep.audio_url AS media_url, ep.numero_episode,
     ep.ordre, ep.duree_minutes, ep.a_la_une, FALSE AS a_la_une_globale, ep.etat,
     ep.motif_rejet, ep.valide_par, ep.valide_at, ep.nombre_signalements,
     ep.cree_par, ep.created_at, ep.updated_at";

pub fn colonnes_episode(type_support: &str) -> Option<&'static str> {
    match type_support {
        "chaine_tv" => Some(EPISODE_TELE_COLONNES),
        "station_radio" => Some(EPISODE_RADIO_COLONNES),
        _ => None,
    }
}

/// Ordre de rotation, unique référence de tri des épisodes (research.md R10).
/// Le tri secondaire par `created_at, id` rend l'ordre **total et stable** même
/// en cas d'ex æquo sur `ordre` — c'est ce qui autorise à ne poser aucune
/// contrainte d'unicité sur `(emission_id, ordre)`.
pub const ORDRE_EPISODES: &str = "ep.ordre, ep.created_at, ep.id";

/// Source du média, déduite de l'URL — elle décide du lecteur employé côté
/// frontend. Une URL YouTube injectée dans un `<video>` ne joue pas.
pub fn source_media(url: Option<&str>) -> String {
    match url {
        Some(u) if u.starts_with("/uploads/") => "hebergee".to_string(),
        Some(_) => "externe".to_string(),
        None => "aucune".to_string(),
    }
}

// ────────────────────────────────────────────────────────────────
// Struct DB
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct EpisodeRow {
    pub id: Uuid,
    pub emission_id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    /// `video_url` en télé, `audio_url` en radio.
    pub media_url: Option<String>,
    pub numero_episode: Option<i32>,
    pub ordre: i32,
    pub duree_minutes: Option<i32>,
    pub a_la_une: bool,
    pub a_la_une_globale: bool,
    pub etat: String,
    pub motif_rejet: Option<String>,
    pub valide_par: Option<Uuid>,
    pub valide_at: Option<DateTime<Utc>>,
    pub nombre_signalements: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // ── Jointures optionnelles ──────────────────────────────────
    #[sqlx(default)]
    pub emission_titre: Option<String>,
    #[sqlx(default)]
    pub emission_slug: Option<String>,
    #[sqlx(default)]
    pub emission_cadence: Option<String>,
    #[sqlx(default)]
    pub support_id: Option<Uuid>,
    #[sqlx(default)]
    pub support_nom: Option<String>,
    #[sqlx(default)]
    pub support_slug: Option<String>,
    #[sqlx(default)]
    pub auteur_nom: Option<String>,
    #[sqlx(default)]
    pub auteur_prenom: Option<String>,
    /// File de modération : prochaine occurrence à l'antenne du programme,
    /// calculée **à la lecture** depuis ses créneaux (FR-043).
    #[sqlx(default)]
    pub prochaine_echeance: Option<DateTime<Utc>>,
}

// ────────────────────────────────────────────────────────────────
// DTO
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct EpisodeResponse {
    pub id: Uuid,
    pub emission_id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    /// Champ **uniforme** des deux familles — c'est lui que consomment les
    /// composants partagés du frontend.
    pub media_url: Option<String>,
    /// Doublons typés, servis pour la seule famille concernée : ils rendent la
    /// réponse lisible sans connaître le type de support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_url: Option<String>,
    /// "hebergee" | "externe" | "aucune" — pilote le choix du lecteur.
    pub source_media: String,
    pub numero_episode: Option<i32>,
    pub ordre: i32,
    pub duree_minutes: Option<i32>,
    pub a_la_une: bool,
    pub a_la_une_globale: bool,
    pub etat: String,
    /// Servi à l'auteur et à l'administration ; jamais nécessaire au public,
    /// qui ne voit pas les épisodes rejetés.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motif_rejet: Option<String>,
    pub valide_at: Option<DateTime<Utc>>,
    pub type_support: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission: Option<RefNommee>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_cadence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<RefNommee>,
    pub nombre_signalements: i32,
    pub cree_par: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auteur_nom_complet: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Compteurs de **l'épisode seul** (FR-048).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<CompteursInteraction>,
}

impl EpisodeRow {
    pub fn to_response(&self, type_support: &str) -> EpisodeResponse {
        let est_tele = type_support == "chaine_tv";
        EpisodeResponse {
            id: self.id,
            emission_id: self.emission_id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            media_url: self.media_url.clone(),
            video_url: if est_tele {
                self.media_url.clone()
            } else {
                None
            },
            audio_url: if est_tele {
                None
            } else {
                self.media_url.clone()
            },
            source_media: source_media(self.media_url.as_deref()),
            numero_episode: self.numero_episode,
            ordre: self.ordre,
            duree_minutes: self.duree_minutes,
            a_la_une: self.a_la_une,
            a_la_une_globale: self.a_la_une_globale,
            etat: self.etat.clone(),
            motif_rejet: self.motif_rejet.clone(),
            valide_at: self.valide_at,
            type_support: type_support.to_string(),
            emission: self.emission_titre.as_ref().map(|titre| RefNommee {
                id: self.emission_id,
                nom: titre.clone(),
                slug: self.emission_slug.clone(),
            }),
            emission_cadence: self.emission_cadence.clone(),
            support: match (self.support_id, self.support_nom.as_ref()) {
                (Some(id), Some(nom)) => Some(RefNommee {
                    id,
                    nom: nom.clone(),
                    slug: self.support_slug.clone(),
                }),
                _ => None,
            },
            nombre_signalements: self.nombre_signalements,
            cree_par: self.cree_par,
            auteur_nom_complet: match (self.auteur_prenom.as_ref(), self.auteur_nom.as_ref()) {
                (Some(p), Some(n)) => Some(format!("{} {}", p, n)),
                (None, Some(n)) => Some(n.clone()),
                (Some(p), None) => Some(p.clone()),
                _ => None,
            },
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
pub struct EpisodeRequest {
    pub titre: String,
    pub description: Option<String>,
    /// Champ uniforme accepté ; `video_url` et `audio_url` sont tolérés en
    /// alias pour ne pas casser les clients existants.
    pub media_url: Option<String>,
    pub video_url: Option<String>,
    pub audio_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub numero_episode: Option<i32>,
    pub duree_minutes: Option<i32>,
    /// Réservé à l'administration. Ignoré sur les routes membres : un épisode
    /// versé par un co-détenteur naît **toujours** `en_attente` (FR-040).
    pub etat: Option<String>,
}

impl EpisodeRequest {
    /// URL du média, quel que soit le nom sous lequel le client l'a envoyée.
    pub fn media(&self) -> Option<String> {
        self.media_url
            .as_ref()
            .or(self.video_url.as_ref())
            .or(self.audio_url.as_ref())
            .map(|u| u.trim().to_string())
            .filter(|u| !u.is_empty())
    }

    pub fn valider(&self) -> Result<(), ApiErreur> {
        if self.titre.trim().is_empty() {
            return Err(ApiErreur::Validation(
                "Le titre de l'épisode est obligatoire".into(),
            ));
        }
        if let Some(duree) = self.duree_minutes
            && duree <= 0
        {
            return Err(ApiErreur::Validation(
                "La durée d'un épisode doit être strictement positive".into(),
            ));
        }
        Ok(())
    }
}

/// Réordonnancement **atomique** — tout réordonner ou rien (patron de
/// `admin/formation_contenu.rs`).
#[derive(Debug, Deserialize)]
pub struct ReordonnancementRequest {
    pub ordres: Vec<OrdreEpisode>,
}

#[derive(Debug, Deserialize)]
pub struct OrdreEpisode {
    pub episode_id: Uuid,
    pub ordre: i32,
}

#[derive(Debug, Deserialize)]
pub struct DeplacerEpisodeRequest {
    pub emission_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct RejeterEpisodeRequest {
    pub motif: String,
}

impl RejeterEpisodeRequest {
    pub fn motif_valide(&self) -> Result<String, ApiErreur> {
        let motif = self.motif.trim();
        if motif.chars().count() < MOTIF_REJET_MIN {
            return Err(ApiErreur::Validation(format!(
                "Le motif de rejet doit compter au moins {} caractères : l'auteur doit savoir quoi corriger",
                MOTIF_REJET_MIN
            )));
        }
        Ok(motif.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct EpisodeQueryParams {
    pub etat: Option<String>,
    /// `tele` | `radio`
    pub r#type: Option<String>,
    pub support_id: Option<Uuid>,
    pub emission_id: Option<Uuid>,
    /// `echeance` (défaut) | `anciennete` — file de modération, FR-043.
    pub tri: Option<String>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
    pub par_page: Option<i64>,
}

impl EpisodeQueryParams {
    pub fn page_effective(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn taille_effective(&self, defaut: i64) -> i64 {
        self.taille
            .or(self.par_page)
            .unwrap_or(defaut)
            .clamp(1, 100)
    }
}

/// Ligne de la file de modération (contrat admin §1). L'échéance et l'ancienneté
/// sont calculées **à la lecture** — aucune tâche de fond.
#[derive(Debug, Serialize)]
pub struct EpisodeModerationResponse {
    #[serde(flatten)]
    pub episode: EpisodeResponse,
    pub soumis_at: DateTime<Utc>,
    pub anciennete_heures: i64,
    /// `None` si l'émission n'est programmée nulle part.
    pub prochaine_echeance: Option<DateTime<Utc>>,
    pub heures_avant_echeance: Option<i64>,
}
