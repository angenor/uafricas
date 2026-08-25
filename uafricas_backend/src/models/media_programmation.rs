//! Grille de programmation récurrente d'un support média
//! (feature 001-refonte-tele-radio, US5, migration 09n ; recadrée par 09q).
//!
//! Un créneau n'est pas un instant mais une règle : « tous les jours à 20h30 »
//! ou « chaque mercredi à 18h ». Depuis 09q il désigne une **émission** et non
//! plus un fichier : l'épisode diffusé se déduit par **rotation**, calculée à la
//! lecture depuis `date_effet`. Aucune tâche de fond, aucune occurrence
//! matérialisée : le déterminisme exigé par FR-017 découle du fait que **rien
//! n'est stocké** : deux lectures d'une même occurrence recalculent le même
//! rang.

use chrono::{DateTime, NaiveDate, NaiveTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::ApiErreur;

pub const RECURRENCES: [&str; 2] = ["quotidien", "hebdomadaire"];

/// Fuseau par défaut, aligné sur celui de la migration.
pub const FUSEAU_DEFAUT: &str = "Africa/Abidjan";

/// Libellés des jours, index 0 = dimanche (convention `EXTRACT(DOW)`).
pub const JOURS_SEMAINE: [&str; 7] = [
    "dimanche", "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi",
];

// ────────────────────────────────────────────────────────────────
// Écriture
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreneauRequest {
    /// Remplace `contenu_id` : la grille porte sur des programmes (FR-014).
    pub emission_id: Uuid,
    pub recurrence: String,
    pub jour_semaine: Option<i16>,
    /// Format « HH:MM » ou « HH:MM:SS », heure locale du `fuseau`.
    pub heure_debut: String,
    pub duree_minutes: i32,
    pub fuseau: Option<String>,
    /// Origine du comptage des occurrences, au format « AAAA-MM-JJ ». Défaut :
    /// aujourd'hui. La déplacer **redéfinit la rotation** : c'est le seul levier
    /// dont dispose un détenteur pour choisir quel épisode passe quand.
    pub date_effet: Option<String>,
}

/// Requête validée : les CHECK SQL disent la même chose, c'est ici que les
/// messages sont en français.
pub struct CreneauValide {
    pub emission_id: Uuid,
    pub recurrence: String,
    pub jour_semaine: Option<i16>,
    pub heure_debut: NaiveTime,
    pub duree_minutes: i32,
    pub fuseau: String,
    pub date_effet: Option<NaiveDate>,
}

impl CreneauRequest {
    pub fn valider(&self) -> Result<CreneauValide, ApiErreur> {
        let recurrence = self.recurrence.trim().to_string();
        if !RECURRENCES.contains(&recurrence.as_str()) {
            return Err(ApiErreur::Validation(
                "La récurrence doit être « quotidien » ou « hebdomadaire »".into(),
            ));
        }

        // Un hebdomadaire sans jour ne serait jamais diffusé ; un quotidien
        // porteur d'un jour induirait son auteur en erreur.
        let jour_semaine = match recurrence.as_str() {
            "quotidien" => None,
            _ => match self.jour_semaine {
                Some(j) if (0..=6).contains(&j) => Some(j),
                _ => {
                    return Err(ApiErreur::Validation(
                        "Un créneau hebdomadaire doit désigner un jour de la semaine (0 = dimanche … 6 = samedi)".into(),
                    ));
                }
            },
        };

        let brut = self.heure_debut.trim();
        let heure_debut = NaiveTime::parse_from_str(brut, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(brut, "%H:%M"))
            .map_err(|_| {
                ApiErreur::Validation("L'heure de début doit être au format « HH:MM »".into())
            })?;

        if !(1..=1440).contains(&self.duree_minutes) {
            return Err(ApiErreur::Validation(
                "La durée doit être comprise entre 1 et 1440 minutes".into(),
            ));
        }

        // Un créneau ne franchit pas minuit : le scinder en deux. Vérifié ici
        // pour le message, et par ck_creneau_pas_minuit pour la garantie.
        let fin_secondes =
            heure_debut.num_seconds_from_midnight() as i64 + self.duree_minutes as i64 * 60;
        if fin_secondes > 24 * 3600 {
            return Err(ApiErreur::Validation(
                "Un créneau ne peut pas franchir minuit : scindez-le en deux créneaux".into(),
            ));
        }

        let fuseau = self
            .fuseau
            .as_deref()
            .map(str::trim)
            .filter(|f| !f.is_empty())
            .unwrap_or(FUSEAU_DEFAUT)
            .to_string();

        let date_effet = match self.date_effet.as_deref().map(str::trim).filter(|d| !d.is_empty()) {
            Some(brut) => Some(NaiveDate::parse_from_str(brut, "%Y-%m-%d").map_err(|_| {
                ApiErreur::Validation(
                    "La date d'effet doit être au format « AAAA-MM-JJ »".into(),
                )
            })?),
            None => None,
        };

        Ok(CreneauValide {
            emission_id: self.emission_id,
            recurrence,
            jour_semaine,
            heure_debut,
            duree_minutes: self.duree_minutes,
            fuseau,
            date_effet,
        })
    }
}

// ────────────────────────────────────────────────────────────────
// Lecture
// ────────────────────────────────────────────────────────────────

pub const CRENEAU_COLONNES: &str = "c.id, c.type_support::text AS type_support, c.support_id,
     c.emission_id, c.recurrence, c.jour_semaine, c.heure_debut, c.duree_minutes,
     c.fuseau, c.date_effet, c.cree_par, c.actif, c.created_at, c.updated_at";

#[derive(Debug, FromRow)]
pub struct CreneauRow {
    pub id: Uuid,
    pub type_support: String,
    pub support_id: Uuid,
    pub emission_id: Uuid,
    pub recurrence: String,
    pub jour_semaine: Option<i16>,
    pub heure_debut: NaiveTime,
    pub duree_minutes: i32,
    pub fuseau: String,
    pub date_effet: NaiveDate,
    pub cree_par: Uuid,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // ── Jointure sur l'émission programmée ──────────────────────
    #[sqlx(default)]
    pub emission_titre: Option<String>,
    #[sqlx(default)]
    pub emission_slug: Option<String>,
    #[sqlx(default)]
    pub emission_image: Option<String>,
    #[sqlx(default)]
    pub emission_etat: Option<String>,
    #[sqlx(default)]
    pub emission_cadence: Option<String>,
    #[sqlx(default)]
    pub nombre_episodes: Option<i64>,

    // ── Épisode résolu par rotation (SQL_DIFFUSION_* seulement) ─
    #[sqlx(default)]
    pub episode_id: Option<Uuid>,
    #[sqlx(default)]
    pub episode_titre: Option<String>,
    #[sqlx(default)]
    pub episode_slug: Option<String>,
    #[sqlx(default)]
    pub episode_media_url: Option<String>,
    #[sqlx(default)]
    pub episode_image: Option<String>,
    #[sqlx(default)]
    pub episode_numero: Option<i32>,
    #[sqlx(default)]
    pub rang_occurrence: Option<i64>,
    #[sqlx(default)]
    pub total_episodes: Option<i64>,
}

/// Référence légère à l'émission ou à l'épisode d'un créneau.
#[derive(Debug, Clone, Serialize)]
pub struct RefContenu {
    pub id: Uuid,
    pub titre: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_couverture_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numero_episode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cadence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre_episodes: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CreneauResponse {
    pub id: Uuid,
    pub type_support: String,
    pub support_id: Uuid,
    pub emission_id: Uuid,
    pub recurrence: String,
    pub jour_semaine: Option<i16>,
    pub jour_libelle: Option<String>,
    /// Sérialisée « HH:MM » : le front n'a que faire des secondes.
    pub heure_debut: String,
    pub duree_minutes: i32,
    pub fuseau: String,
    /// Sérialisée « AAAA-MM-JJ ».
    pub date_effet: String,
    pub cree_par: Uuid,
    pub actif: bool,
    /// Le programme désigné par le créneau.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission: Option<RefContenu>,
    /// L'épisode retenu par la rotation pour l'occurrence courante. `None` sur
    /// les lectures de grille, qui ne résolvent pas la rotation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode: Option<RefContenu>,
    /// Nombre d'occurrences écoulées depuis `date_effet` (FR-016).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rang_occurrence: Option<i64>,
    /// `rang_occurrence >= nombre_episodes_publies`, la rotation a bouclé et
    /// rejoue la série depuis le début (FR-020).
    pub est_rediffusion: bool,
    /// Le programme n'est plus diffusable (retiré, suspendu), ou n'a aucun
    /// épisode publié. Le créneau reste dans la grille, signalé à ses
    /// co-détenteurs (FR-021, FR-024).
    pub emission_indisponible: bool,
    /// Motif de l'indisponibilité, servi à la seule vue détenteur.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerte: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CreneauRow {
    /// Variante servie aux visiteurs qui ne détiennent pas le support.
    ///
    /// La grille reste publique : c'est une grille de programmes, mais les
    /// champs décrivant un programme RETIRÉ de l'antenne sont tus : les laisser
    /// exposerait le titre et la vignette de ce qui vient d'être suspendu pour
    /// violence ou racisme, faisant du signalement massif un révélateur.
    pub fn to_response_publique(self) -> CreneauResponse {
        let indisponible = matches!(self.emission_etat.as_deref(), Some(etat) if etat != "publie");
        let mut reponse = self.to_response();
        if indisponible {
            reponse.emission = None;
            reponse.episode = None;
        }
        // L'alerte est un outil de gestion : elle n'a rien à faire en public.
        reponse.alerte = None;
        reponse
    }

    pub fn to_response(self) -> CreneauResponse {
        let jour_libelle = self
            .jour_semaine
            .and_then(|j| JOURS_SEMAINE.get(j as usize))
            .map(|s| s.to_string());

        // `emission_etat` n'est renseigné que par les lectures qui joignent
        // l'émission ; sans jointure on ne présume pas d'une indisponibilité.
        let etat_ko = matches!(self.emission_etat.as_deref(), Some(etat) if etat != "publie");
        let sans_episode = self.nombre_episodes.map(|n| n == 0).unwrap_or(false);

        let alerte = if etat_ko {
            Some("emission_indisponible".to_string())
        }
        else if sans_episode {
            Some("aucun_episode_publie".to_string())
        }
        else {
            None
        };

        let total = self.total_episodes.unwrap_or(0);
        let est_rediffusion = match (self.rang_occurrence, total) {
            (Some(rang), t) if t > 0 => rang >= t,
            _ => false,
        };

        let emission = self.emission_titre.as_ref().map(|titre| RefContenu {
            id: self.emission_id,
            titre: titre.clone(),
            slug: self.emission_slug.clone(),
            image_couverture_url: self.emission_image.clone(),
            media_url: None,
            numero_episode: None,
            cadence: self.emission_cadence.clone(),
            nombre_episodes: self.nombre_episodes,
        });

        let episode = match (self.episode_id, self.episode_titre.as_ref()) {
            (Some(id), Some(titre)) => Some(RefContenu {
                id,
                titre: titre.clone(),
                slug: self.episode_slug.clone(),
                image_couverture_url: self.episode_image.clone(),
                media_url: self.episode_media_url.clone(),
                numero_episode: self.episode_numero,
                cadence: None,
                nombre_episodes: None,
            }),
            _ => None,
        };

        CreneauResponse {
            id: self.id,
            type_support: self.type_support,
            support_id: self.support_id,
            emission_id: self.emission_id,
            recurrence: self.recurrence,
            jour_semaine: self.jour_semaine,
            jour_libelle,
            heure_debut: self.heure_debut.format("%H:%M").to_string(),
            duree_minutes: self.duree_minutes,
            fuseau: self.fuseau,
            date_effet: self.date_effet.format("%Y-%m-%d").to_string(),
            cree_par: self.cree_par,
            actif: self.actif,
            emission,
            episode,
            rang_occurrence: self.rang_occurrence,
            est_rediffusion,
            emission_indisponible: etat_ko || sans_episode,
            alerte,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Ce qui passe maintenant, et ce qui suit, greffé sur les sections des pages
/// Télé et Radio.
#[derive(Debug, Serialize)]
pub struct DiffusionResponse {
    pub diffusion_en_cours: Option<CreneauResponse>,
    pub creneau_suivant: Option<CreneauResponse>,
}

// ────────────────────────────────────────────────────────────────
// Alertes de cadence (FR-024)
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AlerteCadenceRow {
    pub emission_id: Uuid,
    pub emission_titre: String,
    pub emission_slug: Option<String>,
    pub cadence: String,
    pub type_support: String,
    pub support_id: Uuid,
    pub support_nom: String,
    pub dernier_episode_at: Option<DateTime<Utc>>,
    pub episodes_en_attente: i64,
    pub nombre_episodes: i64,
}

#[derive(Debug, Serialize)]
pub struct AlerteCadenceResponse {
    pub emission: RefContenu,
    pub support: SupportRef,
    pub cadence: String,
    pub dernier_episode_at: Option<DateTime<Utc>>,
    pub prochaine_echeance: Option<DateTime<Utc>>,
    /// `approche` | `depassee` | `aucun_episode`
    pub niveau: String,
    /// Évite l'alerte accusatrice : le détenteur a fait sa part, c'est la file
    /// de modération qui n'a pas suivi.
    pub episodes_en_attente: i64,
}

#[derive(Debug, Serialize)]
pub struct SupportRef {
    pub r#type: String,
    pub id: Uuid,
    pub nom: String,
}
