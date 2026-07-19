//! Programmes radio — table `media_content.programme_radio` (cf. migration 09g).
//!
//! Ces contenus n'avaient jusqu'ici **aucune exposition publique**, alors que
//! leur équivalent télévision en comptait trois. Ce module comble ce manque
//! (FR-020) et sert les sections de la page Radio.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::media_social::CompteursInteraction;

use crate::models::station_radio::StationRadioResponse;

// ── Colonnes SQL ──────────────────────────────────────────────────────

pub const PROGRAMME_RADIO_COLONNES: &str =
    "pr.id, pr.nom_emission, pr.slug, pr.description, pr.image_couverture_url,
     pr.audio_url, pr.info_animateur, pr.info_producteur, pr.pays_id,
     pr.est_international, pr.langue, pr.categorie_radio::text AS categorie_radio,
     pr.etat, pr.cree_par, pr.station_id, pr.a_la_une,
     pr.theme_phare_id, pr.theme_phare_autre, pr.nombre_signalements,
     pr.created_at, pr.updated_at";

// ── Struct DB ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProgrammeRadioRow {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub audio_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub est_international: bool,
    pub langue: String,
    pub categorie_radio: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub station_id: Option<Uuid>,
    pub a_la_une: bool,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub nombre_signalements: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs jointures optionnels
    #[sqlx(default)]
    pub pays_nom: Option<String>,
    #[sqlx(default)]
    pub station_nom: Option<String>,
    #[sqlx(default)]
    pub station_slug: Option<String>,
    #[sqlx(default)]
    pub theme_phare_nom: Option<String>,
}

// ── Response DTOs ─────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ProgrammeRadioResponse {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub audio_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays: Option<String>,
    pub est_international: bool,
    pub langue: String,
    pub categorie_radio: Option<String>,
    pub station_id: Option<Uuid>,
    pub station_nom: Option<String>,
    pub station_slug: Option<String>,
    pub a_la_une: bool,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub theme_phare_nom: Option<String>,
    /// "hebergee" | "externe" | "aucune" — pilote le choix du lecteur (FR-056).
    pub source_media: String,
    pub created_at: DateTime<Utc>,
    /// Réactions, commentaires et partages agrégés (FR-027). `None` tant que
    /// l'appelant ne les a pas greffés.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<CompteursInteraction>,
}

#[derive(Debug, Serialize)]
pub struct ProgrammeRadioListeResponse {
    pub programmes: Vec<ProgrammeRadioResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Sections de la page Radio (US2) ───────────────────────────────────
// Une section = une station, son émission mise en évidence et une rangée de
// ses autres contenus. Le direct est proposé au même titre que les émissions
// enregistrées quand la station porte un `stream_url` (FR-016).

#[derive(Debug, Serialize)]
pub struct StationSectionResponse {
    pub station: StationRadioResponse,
    /// `stream_url` renseigné — le direct est alors offert comme un contenu.
    pub direct_disponible: bool,
    pub mis_en_evidence: Option<ProgrammeRadioResponse>,
    pub contenus: Vec<ProgrammeRadioResponse>,
    pub total_contenus: i64,
    /// Ce que la grille programme à l'instant de la requête, et ce qui suit
    /// (US5, FR-039). `None` si la station n'a aucune grille active — la
    /// section retombe alors sur son contenu mis en évidence (FR-041).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffusion_en_cours: Option<crate::models::media_programmation::CreneauResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creneau_suivant: Option<crate::models::media_programmation::CreneauResponse>,
}

#[derive(Debug, Serialize)]
pub struct StationSectionsListeResponse {
    pub sections: Vec<StationSectionResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Query Params ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ProgrammeRadioQueryParams {
    pub recherche: Option<String>,
    pub pays: Option<String>,
    pub station: Option<Uuid>,
    pub categorie_radio: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ── Mapping ───────────────────────────────────────────────────────────

/// Même règle que pour la télévision : un fichier servi par la plateforme est
/// lisible par une balise native, un lien tiers exige son propre lecteur.
pub fn source_media(url: Option<&str>) -> String {
    match url {
        Some(u) if u.starts_with("/uploads/") => "hebergee".to_string(),
        Some(_) => "externe".to_string(),
        None => "aucune".to_string(),
    }
}

// ── Conversions ───────────────────────────────────────────────────────

impl ProgrammeRadioRow {
    pub fn to_response(&self) -> ProgrammeRadioResponse {
        ProgrammeRadioResponse {
            id: self.id,
            nom_emission: self.nom_emission.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            audio_url: self.audio_url.clone(),
            info_animateur: self.info_animateur.clone(),
            info_producteur: self.info_producteur.clone(),
            pays: self.pays_nom.clone(),
            est_international: self.est_international,
            langue: self.langue.clone(),
            categorie_radio: self.categorie_radio.clone(),
            station_id: self.station_id,
            station_nom: self.station_nom.clone(),
            station_slug: self.station_slug.clone(),
            a_la_une: self.a_la_une,
            theme_phare_id: self.theme_phare_id,
            theme_phare_autre: self.theme_phare_autre.clone(),
            theme_phare_nom: self.theme_phare_nom.clone(),
            source_media: source_media(self.audio_url.as_deref()),
            created_at: self.created_at,
            interactions: None,
        }
    }
}
