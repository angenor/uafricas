//! Grille de programmation récurrente d'un support média
//! (feature 001-refonte-tele-radio, US5 — migration 09n).
//!
//! Un créneau n'est pas un instant mais une règle : « tous les jours à 20h30 »
//! ou « chaque mercredi à 18h ». Le contenu diffusé se déduit à la lecture, par
//! calcul SQL sur `(NOW() AT TIME ZONE fuseau)` — aucune tâche de fond (R7).

use chrono::{DateTime, NaiveTime, Timelike, Utc};
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
    pub contenu_id: Uuid,
    pub recurrence: String,
    pub jour_semaine: Option<i16>,
    /// Format « HH:MM » ou « HH:MM:SS », heure locale du `fuseau`.
    pub heure_debut: String,
    pub duree_minutes: i32,
    pub fuseau: Option<String>,
}

/// Requête validée — les CHECK SQL disent la même chose, c'est ici que les
/// messages sont en français.
pub struct CreneauValide {
    pub contenu_id: Uuid,
    pub recurrence: String,
    pub jour_semaine: Option<i16>,
    pub heure_debut: NaiveTime,
    pub duree_minutes: i32,
    pub fuseau: String,
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

        Ok(CreneauValide {
            contenu_id: self.contenu_id,
            recurrence,
            jour_semaine,
            heure_debut,
            duree_minutes: self.duree_minutes,
            fuseau,
        })
    }
}

// ────────────────────────────────────────────────────────────────
// Lecture
// ────────────────────────────────────────────────────────────────

pub const CRENEAU_COLONNES: &str = "c.id, c.type_support::text AS type_support, c.support_id,
     c.contenu_id, c.recurrence, c.jour_semaine, c.heure_debut, c.duree_minutes,
     c.fuseau, c.cree_par, c.actif, c.created_at, c.updated_at";

#[derive(Debug, FromRow)]
pub struct CreneauRow {
    pub id: Uuid,
    pub type_support: String,
    pub support_id: Uuid,
    pub contenu_id: Uuid,
    pub recurrence: String,
    pub jour_semaine: Option<i16>,
    pub heure_debut: NaiveTime,
    pub duree_minutes: i32,
    pub fuseau: String,
    pub cree_par: Uuid,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Jointure sur le contenu programmé — absente des lectures brutes.
    #[sqlx(default)]
    pub contenu_nom: Option<String>,
    #[sqlx(default)]
    pub contenu_slug: Option<String>,
    #[sqlx(default)]
    pub contenu_image: Option<String>,
    #[sqlx(default)]
    pub contenu_etat: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreneauResponse {
    pub id: Uuid,
    pub type_support: String,
    pub support_id: Uuid,
    pub contenu_id: Uuid,
    pub recurrence: String,
    pub jour_semaine: Option<i16>,
    pub jour_libelle: Option<String>,
    /// Sérialisée « HH:MM » : le front n'a que faire des secondes.
    pub heure_debut: String,
    pub duree_minutes: i32,
    pub fuseau: String,
    pub cree_par: Uuid,
    pub actif: bool,
    pub contenu_nom: Option<String>,
    pub contenu_slug: Option<String>,
    pub contenu_image: Option<String>,
    /// Le contenu programmé n'est plus diffusable (retiré, suspendu). Le
    /// créneau reste dans la grille, signalé à ses co-détenteurs (FR-043).
    pub contenu_indisponible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CreneauRow {
    /// Variante servie aux visiteurs qui ne détiennent pas le support.
    ///
    /// La grille reste publique — c'est une grille de programmes — mais les
    /// champs décrivant un contenu RETIRÉ de l'antenne sont tus : les laisser
    /// exposerait le titre et la vignette de ce qui vient d'être suspendu pour
    /// violence ou racisme, faisant du signalement massif un révélateur.
    /// `contenu_indisponible` reste renseigné, ce qui suffit à afficher un
    /// créneau invalide sans le décrire (FR-041).
    pub fn to_response_publique(self) -> CreneauResponse {
        let indisponible =
            matches!(self.contenu_etat.as_deref(), Some(etat) if etat != "publie");
        let mut reponse = self.to_response();
        if indisponible {
            reponse.contenu_nom = None;
            reponse.contenu_slug = None;
            reponse.contenu_image = None;
        }
        reponse
    }

    pub fn to_response(self) -> CreneauResponse {
        let jour_libelle = self
            .jour_semaine
            .and_then(|j| JOURS_SEMAINE.get(j as usize))
            .map(|s| s.to_string());
        // `contenu_etat` n'est renseigné que par les lectures qui joignent le
        // contenu ; sans jointure on ne présume pas d'une indisponibilité.
        let contenu_indisponible = matches!(self.contenu_etat.as_deref(), Some(etat) if etat != "publie");

        CreneauResponse {
            id: self.id,
            type_support: self.type_support,
            support_id: self.support_id,
            contenu_id: self.contenu_id,
            recurrence: self.recurrence,
            jour_semaine: self.jour_semaine,
            jour_libelle,
            heure_debut: self.heure_debut.format("%H:%M").to_string(),
            duree_minutes: self.duree_minutes,
            fuseau: self.fuseau,
            cree_par: self.cree_par,
            actif: self.actif,
            contenu_nom: self.contenu_nom,
            contenu_slug: self.contenu_slug,
            contenu_image: self.contenu_image,
            contenu_indisponible,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Ce qui passe maintenant, et ce qui suit — greffé sur les sections des pages
/// Télé et Radio (FR-039).
#[derive(Debug, Serialize)]
pub struct DiffusionResponse {
    pub diffusion_en_cours: Option<CreneauResponse>,
    pub creneau_suivant: Option<CreneauResponse>,
}
