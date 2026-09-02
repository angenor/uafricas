// Modèle : Événements en streaming direct (feature 001-evenements-streaming).
//
// Session de diffusion temps réel rattachée à un événement « en ligne » / « hybride »
// (modèle webinaire). Calque allégé d'afrolang.session. États persistés : `en_cours`,
// `terminee`. L'état « en attente de l'organisateur » et le `statut_direct` exposé au
// frontend sont DÉRIVÉS à la lecture (jamais stockés, Principe V, parité événements/rdv).
//
// Aucun média n'est persisté (flux via SFU LiveKit). Chat / réactions / lever-la-main
// circulent en DataPackets LiveKit éphémères ; seul `main_levee` est reflété en base
// pour fournir à l'organisateur une liste de demandes fiable (FR-022).

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

// ── Constantes métier (research.md D6/D8) ───────────────────────────────────

/// Le direct peut être ouvert à partir de 15 min avant le début (D6).
pub const FENETRE_OUVERTURE_MINUTES: i64 = 15;
/// Marge d'arrêt de sécurité absolu au-delà de la fin (ou de la durée présumée) (D6).
pub const ARRET_SECURITE_HEURES: i64 = 2;
/// Durée présumée d'un événement sans `date_heure_fin` renseignée, avant la marge (D6).
pub const DUREE_PRESUMEE_HEURES: i64 = 2;
/// Capacité par défaut d'une session (D8 ; cohérent SC-004 ≥ 100 spectateurs).
pub const MAX_PARTICIPANTS_DEFAUT: i32 = 100;

/// Rôles d'un participant dans une session (détermine `can_publish`).
pub const ROLE_ORGANISATEUR: &str = "organisateur";
pub const ROLE_INTERVENANT: &str = "intervenant";
pub const ROLE_SPECTATEUR: &str = "spectateur";

/// Formats d'événement diffusables en direct (FR-001/019).
pub const FORMATS_DIFFUSABLES: [&str; 2] = ["en_ligne", "hybride"];

/// Colonnes de `media_content.evenement_session` pour le mapping sqlx.
pub const EVENEMENT_SESSION_COLONNES: &str = "id, evenement_id, etat, organisateur_id, \
    demarre_at, termine_at, duree_secondes, max_participants, nombre_participants_pic, \
    arret_securite_at, noeud_id, created_at, updated_at";

// ── Structs FromRow ─────────────────────────────────────────────────────────

/// Ligne `media_content.evenement_session`.
#[derive(Debug, FromRow)]
pub struct SessionRow {
    pub id: Uuid,
    pub evenement_id: Uuid,
    pub etat: String,
    pub organisateur_id: Uuid,
    pub demarre_at: DateTime<Utc>,
    pub termine_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
    pub max_participants: i32,
    pub nombre_participants_pic: i32,
    pub arret_securite_at: DateTime<Utc>,
    pub noeud_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ligne `media_content.evenement_session_participant`.
#[derive(Debug, FromRow)]
pub struct ParticipantRow {
    pub id: Uuid,
    pub session_id: Uuid,
    pub utilisateur_id: Uuid,
    pub role: String,
    pub main_levee: bool,
    pub main_levee_at: Option<DateTime<Utc>>,
    pub rejoint_at: DateTime<Utc>,
    pub quitte_at: Option<DateTime<Utc>>,
    pub duree_secondes: Option<i32>,
}

// ── DTO Response (Serialize) ─────────────────────────────────────────────────

/// Une demande de parole en attente (vue organisateur uniquement).
#[derive(Debug, Serialize)]
pub struct DemandeParole {
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub main_levee_at: Option<DateTime<Utc>>,
}

/// État du direct pour l'appelant (`GET /api/evenements/{id}/direct`).
#[derive(Debug, Serialize)]
pub struct EtatDirectResponse {
    pub statut_direct: String,
    pub peut_ouvrir: bool,
    pub peut_rejoindre: bool,
    pub est_organisateur: bool,
    pub est_inscrit: bool,
    pub session_id: Option<Uuid>,
    pub nombre_participants: i64,
    pub max_participants: i32,
    pub fenetre_ouverture_at: DateTime<Utc>,
    /// Renseigné uniquement si `est_organisateur` (sinon vide).
    pub demandes_parole: Vec<DemandeParole>,
}

/// Réponse de jointure : configuration LiveKit scopée par rôle.
#[derive(Debug, Serialize)]
pub struct TokenDirectResponse {
    pub session_id: Uuid,
    pub room_name: String,
    pub livekit_url: String,
    pub token: String,
    pub role: String,
}

// ── Calculs purs (dérivation à la lecture, testables, sans I/O) ─────────────

/// Le format de l'événement autorise-t-il une diffusion en direct ? (FR-001/019)
pub fn est_diffusable(format: &str) -> bool {
    FORMATS_DIFFUSABLES.contains(&format)
}

/// Instant à partir duquel l'organisateur peut ouvrir le direct (D6).
pub fn fenetre_ouverture_at(debut: DateTime<Utc>) -> DateTime<Utc> {
    debut - Duration::minutes(FENETRE_OUVERTURE_MINUTES)
}

/// Échéance d'arrêt de sécurité absolu, figée à l'ouverture (D6) :
/// `fin + 2h`, ou `début + durée_présumée + 2h` à défaut de fin.
pub fn calc_arret_securite_at(
    debut: DateTime<Utc>,
    fin: Option<DateTime<Utc>>,
) -> DateTime<Utc> {
    let base = fin.unwrap_or_else(|| debut + Duration::hours(DUREE_PRESUMEE_HEURES));
    base + Duration::hours(ARRET_SECURITE_HEURES)
}

/// Le rôle confère-t-il le droit de diffuser (caméra/micro/écran) ? (D2)
pub fn can_publish_pour_role(role: &str) -> bool {
    role == ROLE_ORGANISATEUR || role == ROLE_INTERVENANT
}

/// `statut_direct` dérivé exposé au frontend (jamais stocké, data-model.md).
///
/// `derniere_session` = la session la plus récente de l'événement (active ou
/// terminée), `None` si aucune n'a jamais existé. La clôture paresseuse de
/// l'arrêt de sécurité est traitée ici : une session `en_cours` dont
/// `arret_securite_at` est dépassé est considérée comme `termine`.
pub fn statut_direct(
    diffusable: bool,
    evenement_annule: bool,
    derniere_session: Option<&SessionRow>,
    debut: DateTime<Utc>,
    maintenant: DateTime<Utc>,
) -> &'static str {
    if !diffusable || evenement_annule {
        return "indisponible";
    }
    match derniere_session {
        Some(s) if s.etat == "en_cours" && maintenant <= s.arret_securite_at => "en_direct",
        Some(_) => "termine",
        None => {
            if maintenant >= fenetre_ouverture_at(debut) {
                "en_attente"
            } else {
                "indisponible"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(h: i64) -> DateTime<Utc> {
        Utc::now() + Duration::hours(h)
    }

    #[test]
    fn diffusable_uniquement_en_ligne_et_hybride() {
        assert!(est_diffusable("en_ligne"));
        assert!(est_diffusable("hybride"));
        assert!(!est_diffusable("presentiel"));
    }

    #[test]
    fn can_publish_organisateur_et_intervenant() {
        assert!(can_publish_pour_role(ROLE_ORGANISATEUR));
        assert!(can_publish_pour_role(ROLE_INTERVENANT));
        assert!(!can_publish_pour_role(ROLE_SPECTATEUR));
    }

    #[test]
    fn fenetre_ouvre_15min_avant() {
        let debut = t(1);
        assert_eq!(
            fenetre_ouverture_at(debut),
            debut - Duration::minutes(15)
        );
    }

    #[test]
    fn arret_securite_depuis_fin_ou_duree_presumee() {
        let debut = t(0);
        let fin = debut + Duration::hours(3);
        assert_eq!(calc_arret_securite_at(debut, Some(fin)), fin + Duration::hours(2));
        // Sans fin : début + 2h présumées + 2h marge.
        assert_eq!(
            calc_arret_securite_at(debut, None),
            debut + Duration::hours(4)
        );
    }

    #[test]
    fn statut_indisponible_si_non_diffusable_ou_annule() {
        let now = Utc::now();
        assert_eq!(statut_direct(false, false, None, now, now), "indisponible");
        assert_eq!(statut_direct(true, true, None, now, now), "indisponible");
    }

    #[test]
    fn statut_en_attente_dans_la_fenetre_sans_session() {
        let debut = t(0); // début maintenant → dans la fenêtre
        assert_eq!(statut_direct(true, false, None, debut, Utc::now()), "en_attente");
    }

    #[test]
    fn statut_indisponible_avant_la_fenetre() {
        let debut = t(2); // dans 2h → hors fenêtre (>15min avant)
        assert_eq!(statut_direct(true, false, None, debut, Utc::now()), "indisponible");
    }
}
