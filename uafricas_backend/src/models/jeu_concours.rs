//! Modèles des concours (feature 014, famille B).
//!
//! Un concours ne lit JAMAIS la création qu'on y dépose : le vote, le
//! dépouillement et les récompenses ne connaissent qu'une participation. Un
//! futur défi vidéo ou de traduction ajoute un format, pas un cycle (FR-025).
//!
//! Règle de sérialisation qui porte l'anonymat du vote : pendant l'appel et le
//! vote, une participation est servie par [`ParticipationAnonyme`], un type qui
//! N'A PAS de champ auteur (FR-039), sur le modèle `EpreuveServie` / `Correction`.

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

// ─── Concours ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ConcoursRow {
    pub id: Uuid,
    pub format: String,
    pub titre: String,
    pub theme: String,
    pub reglement: String,
    pub rattachement: Option<String>,
    pub image_url: Option<String>,
    pub appel_debut: DateTime<Utc>,
    pub vote_debut: DateTime<Utc>,
    pub vote_fin: DateTime<Utc>,
    pub participations_max: i16,
    pub minimum_participations: i16,
    pub votes_max: Option<i32>,
    pub presentations_min: i16,
    pub jury: bool,
    pub jury_finalistes: i16,
    pub jury_delai_jours: i16,
    pub prime_participation: Option<i16>,
    pub prime_podium: Option<Vec<i16>>,
    pub etat: String,
    pub motif_annulation: Option<String>,
    pub podium_jury: Option<Vec<Uuid>>,
    pub delibere_at: Option<DateTime<Utc>>,
    pub resultats_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub const CONCOURS_COLONNES: &str = "c.id, c.format, c.titre, c.theme, c.reglement, c.rattachement, \
     c.image_url, c.appel_debut, c.vote_debut, c.vote_fin, c.participations_max, \
     c.minimum_participations, c.votes_max, c.presentations_min, c.jury, c.jury_finalistes, \
     c.jury_delai_jours, c.prime_participation, c.prime_podium, c.etat, c.motif_annulation, \
     c.podium_jury, c.delibere_at, c.resultats_at, c.created_at";

/// La phase d'un concours, CALCULÉE à la lecture d'après ses dates et son état :
/// elle n'est jamais stockée (research D6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    AVenir,
    Appel,
    Vote,
    Deliberation,
    Resultats,
    Annule,
}

impl Phase {
    pub fn code(self) -> &'static str {
        match self {
            Phase::AVenir => "a_venir",
            Phase::Appel => "appel",
            Phase::Vote => "vote",
            Phase::Deliberation => "deliberation",
            Phase::Resultats => "resultats",
            Phase::Annule => "annule",
        }
    }
}

impl ConcoursRow {
    /// La phase à l'instant donné. `Resultats` peut être renvoyé AVANT que les
    /// résultats soient écrits : c'est le signal, pour `resoudre_concours`,
    /// qu'il faut les établir.
    pub fn phase(&self, maintenant: DateTime<Utc>) -> Phase {
        if self.etat == "annule" {
            return Phase::Annule;
        }
        if self.etat == "resultats" {
            return Phase::Resultats;
        }
        if maintenant < self.appel_debut {
            Phase::AVenir
        } else if maintenant < self.vote_debut {
            Phase::Appel
        } else if maintenant < self.vote_fin {
            Phase::Vote
        } else if self.jury
            && self.delibere_at.is_none()
            && maintenant < self.vote_fin + Duration::days(i64::from(self.jury_delai_jours))
        {
            Phase::Deliberation
        } else {
            Phase::Resultats
        }
    }
}

// ─── Participations ──────────────────────────────────────────────────────────

/// Une participation telle que tout le monde la voit pendant l'appel et le
/// vote : AUCUN champ ne dit qui l'a déposée (FR-039).
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ParticipationAnonyme {
    pub id: Uuid,
    pub media_type: String,
    pub media_url: String,
    pub legende: Option<String>,
}

/// L'auteur d'une participation, révélé avec les résultats.
#[derive(Debug, Clone, Serialize)]
pub struct AuteurParticipation {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
}

// ─── DTO publics ─────────────────────────────────────────────────────────────

/// Un concours vu de l'espace Activités.
#[derive(Debug, Serialize)]
pub struct ConcoursPublic {
    pub id: Uuid,
    pub format: String,
    pub titre: String,
    pub theme: String,
    pub reglement: String,
    pub rattachement: Option<String>,
    pub image_url: Option<String>,
    pub phase: Phase,
    pub appel_debut: DateTime<Utc>,
    pub vote_debut: DateTime<Utc>,
    pub vote_fin: DateTime<Utc>,
    pub participations_max: i16,
    pub jury: bool,
    pub motif_annulation: Option<String>,
    pub participations_publiees: i64,
    /// Présent seulement pour un membre connecté.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moi: Option<MoiConcours>,
}

/// Ce qu'un membre connecté voit de sa propre situation dans un concours.
#[derive(Debug, Serialize)]
pub struct MoiConcours {
    pub participations: Vec<MaParticipation>,
    pub peut_participer: bool,
    pub peut_voter: bool,
    pub votes_exprimes: i64,
    pub votes_max: Option<i32>,
}

/// Une participation vue par son AUTEUR : son état, et le motif d'un rejet.
#[derive(Debug, Serialize, FromRow)]
pub struct MaParticipation {
    pub id: Uuid,
    pub etat: String,
    pub media_type: String,
    pub media_url: String,
    pub legende: Option<String>,
    pub motif_rejet: Option<String>,
    pub created_at: DateTime<Utc>,
}
