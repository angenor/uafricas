// Modèles du domaine social : rendez-vous en visioconférence entre amis.
// Source de vérité : doc/bd/schemas/31_social_rendez_vous.sql
//
// Le backend orchestre uniquement la prise de rendez-vous (proposer → répondre /
// contre-proposer → accepter → annuler) ; la visioconférence est pair-à-pair
// (WebRTC/PeerJS). Les peer-id sont déterministes (research §7), aucune
// signalisation applicative n'est persistée.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::amitie::MembreLight;

/// Durées de créneau autorisées (FR-007/010).
pub const DUREES_AUTORISEES: [i16; 4] = [15, 30, 45, 60];

/// Marges de la fenêtre d'ouverture de la salle visio (research §8) :
/// `[date_heure − 5 min, date_heure + durée + 15 min]`.
const MARGE_AVANT_MIN: i64 = 5;
const MARGE_APRES_MIN: i64 = 15;

/// Colonnes brutes de `social.rendez_vous` (préfixe `rv.`). Conservé par
/// convention de domaine ; les lectures enrichies passent par `SELECT_AVEC_AUTRE`.
#[allow(dead_code)]
pub const COLONNES: &str = "rv.id, rv.initiateur_id, rv.destinataire_id, rv.sujet, rv.description, \
     rv.date_heure, rv.duree_minutes, rv.statut::text AS statut, rv.tour_id, \
     rv.created_at, rv.updated_at";

/// Fragment SELECT réutilisable : colonnes du rendez-vous + « autre » membre
/// (le participant ≠ `$1`, l'utilisateur courant). Les handlers append leur
/// clause `WHERE` et bindent `moi` en `$1`.
pub const SELECT_AVEC_AUTRE: &str = "SELECT \
     rv.id, rv.initiateur_id, rv.destinataire_id, rv.sujet, rv.description, \
     rv.date_heure, rv.duree_minutes, rv.statut::text AS statut, rv.tour_id, \
     rv.created_at, rv.updated_at, \
     u.id AS autre_id, u.nom AS autre_nom, u.prenom AS autre_prenom, u.slug AS autre_slug, \
     u.photo_url AS autre_photo_url, u.fonction AS autre_fonction, p.nom AS autre_pays \
     FROM social.rendez_vous rv \
     JOIN iam.utilisateur u \
        ON u.id = CASE WHEN rv.initiateur_id = $1 THEN rv.destinataire_id ELSE rv.initiateur_id END \
     LEFT JOIN shared.pays p ON p.id = u.pays_residence_id";

// ────────────────────────────────────────────────────────────────
// Lignes & DTO
// ────────────────────────────────────────────────────────────────

/// Ligne brute `social.rendez_vous` (sans jointure).
#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct RendezVous {
    pub id: Uuid,
    pub initiateur_id: Uuid,
    pub destinataire_id: Uuid,
    pub sujet: String,
    pub description: Option<String>,
    pub date_heure: DateTime<Utc>,
    pub duree_minutes: i16,
    pub statut: String,
    pub tour_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ligne jointe : rendez-vous + « autre » membre (MembreLight aliasé `autre_*`).
/// Construite par `SELECT_AVEC_AUTRE`.
#[derive(Debug, FromRow)]
pub struct RendezVousAvecAutreRow {
    pub id: Uuid,
    pub initiateur_id: Uuid,
    #[allow(dead_code)]
    pub destinataire_id: Uuid,
    pub sujet: String,
    pub description: Option<String>,
    pub date_heure: DateTime<Utc>,
    pub duree_minutes: i16,
    pub statut: String,
    pub tour_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub autre_id: Uuid,
    pub autre_nom: String,
    pub autre_prenom: String,
    pub autre_slug: Option<String>,
    pub autre_photo_url: Option<String>,
    pub autre_fonction: Option<String>,
    pub autre_pays: Option<String>,
}

impl RendezVousAvecAutreRow {
    /// Extrait le MembreLight de l'autre participant.
    pub fn autre(&self) -> MembreLight {
        MembreLight {
            id: self.autre_id,
            nom: self.autre_nom.clone(),
            prenom: self.autre_prenom.clone(),
            slug: self.autre_slug.clone(),
            photo_url: self.autre_photo_url.clone(),
            fonction: self.autre_fonction.clone(),
            pays: self.autre_pays.clone(),
        }
    }

    /// Construit le DTO de réponse, en calculant les champs dérivés pour `moi`
    /// à l'instant `maintenant`.
    pub fn to_response(&self, moi: Uuid, maintenant: DateTime<Utc>) -> RendezVousResponse {
        RendezVousResponse {
            id: self.id,
            sujet: self.sujet.clone(),
            description: self.description.clone(),
            date_heure: self.date_heure,
            duree_minutes: self.duree_minutes,
            statut: self.statut.clone(),
            tour_id: self.tour_id,
            mon_tour: self.tour_id == moi,
            suis_initiateur: self.initiateur_id == moi,
            etat_derive: etat_derive(&self.statut, self.date_heure, self.duree_minutes, maintenant),
            peut_rejoindre: peut_rejoindre(
                &self.statut,
                self.date_heure,
                self.duree_minutes,
                maintenant,
            ),
            autre: self.autre(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// DTO de réponse enrichi (contracts/rendez-vous.md §DTO).
#[derive(Debug, Serialize)]
pub struct RendezVousResponse {
    pub id: Uuid,
    pub sujet: String,
    pub description: Option<String>,
    pub date_heure: DateTime<Utc>,
    pub duree_minutes: i16,
    pub statut: String,
    pub tour_id: Uuid,
    pub mon_tour: bool,
    pub suis_initiateur: bool,
    /// « expire » | « termine » | null (dérivé, jamais persisté).
    pub etat_derive: Option<String>,
    pub peut_rejoindre: bool,
    pub autre: MembreLight,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Page de rendez-vous (FR-019/020).
#[derive(Debug, Serialize)]
pub struct ListeRendezVous {
    pub items: Vec<RendezVousResponse>,
    pub page: i64,
    pub taille: i64,
    pub total: i64,
}

/// Configuration P2P renvoyée par `/salle` (contracts §4).
#[derive(Debug, Serialize)]
pub struct SalleResponse {
    pub rendez_vous_id: Uuid,
    pub mon_peer_id: String,
    pub pair_peer_id: String,
    pub suis_appelant: bool,
    pub autre: MembreLight,
}

// ────────────────────────────────────────────────────────────────
// Corps de requête
// ────────────────────────────────────────────────────────────────

/// Corps de `POST /api/rendez-vous` (proposer, FR-006..FR-011).
#[derive(Debug, Deserialize)]
pub struct ProposerBody {
    pub destinataire_id: Uuid,
    pub sujet: String,
    pub description: Option<String>,
    pub date_heure: DateTime<Utc>,
    pub duree_minutes: i16,
}

/// Corps de `POST /{id}/contre-proposer` (FR-015..FR-017).
#[derive(Debug, Deserialize)]
pub struct ContreProposerBody {
    pub date_heure: DateTime<Utc>,
    pub duree_minutes: i16,
}

// ────────────────────────────────────────────────────────────────
// Calculs dérivés (lecture seule — pas de transition d'état)
// ────────────────────────────────────────────────────────────────

/// Fin prévue du créneau.
pub fn fin_creneau(date_heure: DateTime<Utc>, duree_minutes: i16) -> DateTime<Utc> {
    date_heure + Duration::minutes(duree_minutes as i64)
}

/// État dérivé d'un rendez-vous (research §2) :
/// - « expire » : `propose` dont la date est passée ;
/// - « termine » : `accepte` dont la fenêtre est écoulée ;
/// - `None` sinon.
pub fn etat_derive(
    statut: &str,
    date_heure: DateTime<Utc>,
    duree_minutes: i16,
    maintenant: DateTime<Utc>,
) -> Option<String> {
    match statut {
        "propose" if date_heure < maintenant => Some("expire".to_string()),
        "accepte" if fin_creneau(date_heure, duree_minutes) < maintenant => {
            Some("termine".to_string())
        }
        _ => None,
    }
}

/// La salle visio est ouvrable (FR-024, research §8) : `accepte` ET
/// `NOW() ∈ [date_heure − 5 min, date_heure + durée + 15 min]`.
pub fn peut_rejoindre(
    statut: &str,
    date_heure: DateTime<Utc>,
    duree_minutes: i16,
    maintenant: DateTime<Utc>,
) -> bool {
    if statut != "accepte" {
        return false;
    }
    let debut = date_heure - Duration::minutes(MARGE_AVANT_MIN);
    let fin = fin_creneau(date_heure, duree_minutes) + Duration::minutes(MARGE_APRES_MIN);
    maintenant >= debut && maintenant <= fin
}

/// Peer-id déterministe pour la signalisation P2P (research §7) :
/// `uafr-{hex(sha256(rendez_vous_id ‖ participant_id))}` tronqué à 32 caractères hex.
/// L'UUID du rendez-vous (renvoyé uniquement par l'API authentifiée) sert de secret.
pub fn peer_id(rendez_vous_id: Uuid, participant_id: Uuid) -> String {
    let mut hasher = Sha256::new();
    hasher.update(rendez_vous_id.as_bytes());
    hasher.update(participant_id.as_bytes());
    let digest = hasher.finalize();
    let hex: String = digest.iter().take(16).map(|o| format!("{o:02x}")).collect();
    format!("uafr-{hex}")
}

// ────────────────────────────────────────────────────────────────
// Évènements SSE (forme générique, sans contenu sensible — research §6)
// ────────────────────────────────────────────────────────────────

fn evt(type_: &str, rendez_vous_id: Uuid) -> serde_json::Value {
    serde_json::json!({ "type": type_, "rendez_vous_id": rendez_vous_id })
}

pub fn evt_rdv_propose(rendez_vous_id: Uuid) -> serde_json::Value {
    evt("rdv_propose", rendez_vous_id)
}

pub fn evt_rdv_accepte(rendez_vous_id: Uuid) -> serde_json::Value {
    evt("rdv_accepte", rendez_vous_id)
}

pub fn evt_rdv_refuse(rendez_vous_id: Uuid) -> serde_json::Value {
    evt("rdv_refuse", rendez_vous_id)
}

pub fn evt_rdv_contre_propose(rendez_vous_id: Uuid) -> serde_json::Value {
    evt("rdv_contre_propose", rendez_vous_id)
}

pub fn evt_rdv_annule(rendez_vous_id: Uuid) -> serde_json::Value {
    evt("rdv_annule", rendez_vous_id)
}
