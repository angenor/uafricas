// Modèles du domaine social : appel direct en visioconférence entre amis.
// Aucune persistance — un appel est éphémère (registre en mémoire
// `services::appels::RegistreAppels`). La visioconférence est pair-à-pair
// (WebRTC/PeerJS) ; le backend ne fait que la mise en relation initiale et
// réutilise les peer-id déterministes du module `rendez_vous`.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::amitie::MembreLight;

/// Corps de `POST /api/appels` — démarrer un appel direct.
#[derive(Debug, Deserialize)]
pub struct AppelerBody {
    pub destinataire_id: Uuid,
    /// Vidéo activée d'emblée (`true`) ou appel « normal » caméra coupée par
    /// défaut (`false`). Absent = appel normal (rétrocompatibilité).
    #[serde(default)]
    pub video: bool,
}

/// Configuration P2P renvoyée à l'ouverture / l'acceptation d'un appel.
/// Structure alignée sur `SalleResponse` (rendez-vous) afin de réutiliser la
/// salle visio côté frontend.
#[derive(Debug, Serialize)]
pub struct SalleAppelResponse {
    pub appel_id: Uuid,
    pub mon_peer_id: String,
    pub pair_peer_id: String,
    /// Anti-glare : le plus petit UUID initie l'appel WebRTC (research §7).
    pub suis_appelant: bool,
    /// Vidéo activée d'emblée (`true`) ou appel « normal » caméra coupée par défaut.
    pub video: bool,
    pub autre: MembreLight,
}

// ────────────────────────────────────────────────────────────────
// Évènements SSE (forme générique, sans contenu sensible)
// ────────────────────────────────────────────────────────────────

/// `appel_entrant` : poussé au destinataire ; porte l'appelant (MembreLight)
/// pour afficher la sonnerie sans lecture supplémentaire.
pub fn evt_appel_entrant(appel_id: Uuid, appelant: &MembreLight, video: bool) -> serde_json::Value {
    serde_json::json!({ "type": "appel_entrant", "appel_id": appel_id, "appelant": appelant, "video": video })
}

/// `appel_accepte` : poussé à l'appelant quand le destinataire rejoint.
pub fn evt_appel_accepte(appel_id: Uuid) -> serde_json::Value {
    serde_json::json!({ "type": "appel_accepte", "appel_id": appel_id })
}

/// `appel_refuse` : poussé à l'appelant quand le destinataire décline.
pub fn evt_appel_refuse(appel_id: Uuid) -> serde_json::Value {
    serde_json::json!({ "type": "appel_refuse", "appel_id": appel_id })
}

/// `appel_annule` : poussé à l'autre participant quand l'appel est abandonné
/// avant connexion (l'appelant raccroche pendant la sonnerie).
pub fn evt_appel_annule(appel_id: Uuid) -> serde_json::Value {
    serde_json::json!({ "type": "appel_annule", "appel_id": appel_id })
}
