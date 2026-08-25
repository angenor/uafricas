//! Page Radio : sections d'une station et référentiels de filtre.
//!
//! La table `media_content.programme_radio` a disparu avec la migration 09q :
//! le conteneur est désormais l'**émission** (`models::media_emission`,
//! `media_content.emission_radio`), l'unité diffusable l'**épisode**
//! (`models::media_episode`, `media_content.episode_radio`). Ce module ne garde
//! que ce qui relève de la page : la structure des sections, strictement
//! symétrique de celle de la télé.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::media_emission::EmissionResponse;
use crate::models::station_radio::StationRadioResponse;

// ── Sections de la page Radio ─────────────────────────────────────────
// Une section = une station et ses PROGRAMMES publiés. Le direct est proposé au
// même titre que les émissions enregistrées quand la station porte un
// `stream_url`.

#[derive(Debug, Serialize)]
pub struct StationSectionResponse {
    pub station: StationRadioResponse,
    /// `stream_url` renseigné : le direct est alors offert comme un contenu.
    pub direct_disponible: bool,
    /// Programmes publiés comptant au moins un épisode publié (FR-011).
    pub emissions: Vec<EmissionResponse>,
    pub total_emissions: i64,
    /// Ce que la grille programme à l'instant de la requête, et ce qui suit
    /// (US2). `None` si la station n'a aucune grille active, ou si l'émission
    /// programmée n'a aucun épisode publié (FR-021).
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
