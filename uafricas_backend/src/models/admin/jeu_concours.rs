//! Modèles d'administration des concours (feature 014, famille B).

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::jeu_concours::{ConcoursRow, Phase};

/// Création ou modification d'un concours.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConcoursRequest {
    #[serde(default = "format_par_defaut")]
    pub format: String,
    pub titre: String,
    pub theme: String,
    pub reglement: String,
    #[serde(default)]
    pub rattachement: Option<String>,
    #[serde(default)]
    pub image_url: Option<String>,
    pub appel_debut: DateTime<Utc>,
    pub vote_debut: DateTime<Utc>,
    pub vote_fin: DateTime<Utc>,
    #[serde(default)]
    pub participations_max: Option<i16>,
    #[serde(default)]
    pub minimum_participations: Option<i16>,
    #[serde(default)]
    pub votes_max: Option<i32>,
    #[serde(default)]
    pub presentations_min: Option<i16>,
    #[serde(default)]
    pub jury: Option<bool>,
    #[serde(default)]
    pub jury_finalistes: Option<i16>,
    #[serde(default)]
    pub jury_delai_jours: Option<i16>,
    #[serde(default)]
    pub prime_participation: Option<i16>,
    #[serde(default)]
    pub prime_podium: Option<Vec<i16>>,
}

fn format_par_defaut() -> String {
    "photo".into()
}

fn texte(valeur: &Option<String>) -> Option<String> {
    valeur.as_deref().map(str::trim).filter(|v| !v.is_empty()).map(str::to_string)
}

impl ConcoursRequest {
    /// Nettoie et contrôle la demande ; le message nomme le champ fautif. Les
    /// mêmes règles sont en CHECK dans la table : elles ne sont ici que pour
    /// dire POURQUOI (FR-022).
    pub fn nettoyer(mut self) -> Result<Self, String> {
        self.format = self.format.trim().to_string();
        if self.format != "photo" {
            return Err("Format : seule la bataille de photos est disponible pour l'instant".into());
        }
        self.titre = self.titre.trim().to_string();
        self.theme = self.theme.trim().to_string();
        self.reglement = self.reglement.trim().to_string();
        if self.titre.is_empty() {
            return Err("Le titre est obligatoire".into());
        }
        if self.titre.chars().count() > 150 {
            return Err("Le titre : 150 signes au plus".into());
        }
        if self.theme.is_empty() {
            return Err("Le thème est obligatoire".into());
        }
        if self.reglement.is_empty() {
            return Err("Le règlement est obligatoire".into());
        }
        self.rattachement = texte(&self.rattachement).map(|r| r.to_lowercase());
        if let Some(r) = &self.rattachement {
            if !(3..=30).contains(&r.len()) || !r.chars().all(|c| c.is_ascii_lowercase() || c == '_') {
                return Err("Rattachement : un code de module (lettres minuscules et « _ »)".into());
            }
        }
        self.image_url = texte(&self.image_url);

        if self.appel_debut >= self.vote_debut {
            return Err("L'appel à participation doit s'ouvrir avant le vote".into());
        }
        if self.vote_fin < self.vote_debut + Duration::hours(24) {
            return Err("Le vote dure au moins 24 heures".into());
        }

        let entre = |nom: &str, v: Option<i16>, min: i16, max: i16| match v {
            Some(x) if !(min..=max).contains(&x) => Err(format!("{nom} : entre {min} et {max}")),
            _ => Ok(()),
        };
        entre("Participations par membre", self.participations_max, 1, 10)?;
        entre("Minimum de participations pour ouvrir le vote", self.minimum_participations, 2, i16::MAX)?;
        entre("Présentations minimales pour le podium", self.presentations_min, 1, i16::MAX)?;
        entre("Finalistes soumis au jury", self.jury_finalistes, 3, 30)?;
        entre("Délai du jury (jours)", self.jury_delai_jours, 1, 30)?;
        entre("Prime de participation", self.prime_participation, 0, i16::MAX)?;
        if let Some(v) = self.votes_max {
            if v < 10 {
                return Err("Plafond de votes : au moins 10".into());
            }
        }
        if let Some(p) = &self.prime_podium {
            if p.len() != 3 || p[2] < 0 || p[0] < p[1] || p[1] < p[2] {
                return Err("Primes du podium : trois montants positifs, du premier au troisième, sans augmenter".into());
            }
        }
        Ok(self)
    }
}

/// Un concours vu du back-office, avec ses décomptes.
#[derive(Debug, Serialize)]
pub struct ConcoursAdmin {
    #[serde(flatten)]
    pub concours: ConcoursRow,
    pub phase: Phase,
    pub en_attente: i64,
    pub publiees: i64,
    pub rejetees: i64,
    pub votes: i64,
    pub votes_comptes: i64,
}

/// Décomptes d'un concours, lus en une requête.
#[derive(Debug, FromRow)]
pub struct DecomptesConcours {
    pub en_attente: i64,
    pub publiees: i64,
    pub rejetees: i64,
    pub votes: i64,
    pub votes_comptes: i64,
}

/// Une participation dans la file de modération.
#[derive(Debug, Serialize, FromRow)]
pub struct ParticipationModeration {
    pub id: Uuid,
    pub concours_id: Uuid,
    pub concours_titre: String,
    pub auteur_id: Uuid,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub media_type: String,
    pub media_url: String,
    pub legende: Option<String>,
    pub etat: String,
    pub motif_rejet: Option<String>,
    pub nombre_signalements: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ConcoursQueryParams {
    pub phase: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ParticipationsQueryParams {
    pub etat: Option<String>,
    pub concours: Option<Uuid>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct MotifRequest {
    pub motif: String,
}

#[derive(Debug, Deserialize)]
pub struct ModerationGroupeeRequest {
    pub ids: Vec<Uuid>,
    /// `accepter` ou `rejeter`.
    pub decision: String,
    pub motif: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct BilanModeration {
    pub acceptees: i64,
    pub rejetees: i64,
    pub ignorees: i64,
}
