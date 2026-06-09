// Registre en mémoire des appels directs en visioconférence (sans rendez-vous).
// Un appel est éphémère : aucune persistance en base. L'UUID de l'appel sert de
// secret partagé pour dériver les peer-id (comme l'UUID de rendez-vous), et la
// visioconférence reste pair-à-pair (WebRTC/PeerJS). Mono-instance, cohérent
// avec `RegistreSse` (Décision 2).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

/// Durée de vie d'un appel dans le registre (sonnerie + établissement P2P).
/// Au-delà, l'entrée est purgée paresseusement : l'appel est considéré manqué.
const TTL_MINUTES: i64 = 2;

/// Un appel direct en cours de sonnerie / d'établissement.
#[derive(Clone, Copy)]
pub struct AppelEnCours {
    pub appelant_id: Uuid,
    pub destinataire_id: Uuid,
    pub cree_a: DateTime<Utc>,
}

impl AppelEnCours {
    /// Autre participant vu de `moi`, ou `None` si `moi` n'est pas concerné.
    pub fn autre_que(&self, moi: Uuid) -> Option<Uuid> {
        if moi == self.appelant_id {
            Some(self.destinataire_id)
        } else if moi == self.destinataire_id {
            Some(self.appelant_id)
        } else {
            None
        }
    }
}

/// Registre partagé des appels directs en cours (clé = UUID d'appel).
#[derive(Clone, Default)]
pub struct RegistreAppels {
    inner: Arc<Mutex<HashMap<Uuid, AppelEnCours>>>,
}

impl RegistreAppels {
    pub fn new() -> Self {
        Self::default()
    }

    /// Purge paresseuse des entrées expirées (TTL dépassé).
    fn purger(map: &mut HashMap<Uuid, AppelEnCours>, maintenant: DateTime<Utc>) {
        let limite = maintenant - Duration::minutes(TTL_MINUTES);
        map.retain(|_, a| a.cree_a > limite);
    }

    /// Enregistre un nouvel appel.
    pub fn ouvrir(&self, appel_id: Uuid, appelant_id: Uuid, destinataire_id: Uuid) {
        let maintenant = Utc::now();
        let mut map = self.inner.lock().unwrap();
        Self::purger(&mut map, maintenant);
        map.insert(
            appel_id,
            AppelEnCours {
                appelant_id,
                destinataire_id,
                cree_a: maintenant,
            },
        );
    }

    /// Récupère un appel non expiré.
    pub fn obtenir(&self, appel_id: Uuid) -> Option<AppelEnCours> {
        let maintenant = Utc::now();
        let mut map = self.inner.lock().unwrap();
        Self::purger(&mut map, maintenant);
        map.get(&appel_id).copied()
    }

    /// Retire un appel (refus, annulation ou fin).
    pub fn retirer(&self, appel_id: Uuid) -> Option<AppelEnCours> {
        let mut map = self.inner.lock().unwrap();
        map.remove(&appel_id)
    }
}
