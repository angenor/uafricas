//! Prestataire de paiement : **unique point de bascule vers CinetPay**.
//!
//! Ce module est délibérément le fichier le plus court du domaine « cadeaux » :
//! c'est son intérêt. Le jour où l'encaissement réel arrive, seuls les corps de
//! `initier` et `confirmer` changent (plus l'ajout d'un handler de webhook
//! signé) : le catalogue, le journal comptable, la répartition 90/10 et
//! l'attribution des points sont déjà définitifs et n'ont pas à bouger.
//!
//! **Pas de trait `PrestatairePaiement`.** Une abstraction à implémentation
//! unique ne ferait gagner aucune ligne le jour du remplacement, et le Principe V
//! (YAGNI) l'interdit. Deux fonctions concrètes suffisent à isoler l'appel.
//!
//! Le drapeau `simule` est écrit **sur la transaction**, jamais déduit de la
//! configuration au moment de la lecture : c'est lui qui rend la purge de fin de
//! phase de test exacte, même le jour où des transactions réelles et simulées
//! cohabitent (research R7).

use chrono::Utc;
use uuid::Uuid;

/// Durée de validité d'une intention de paiement. Au-delà, la confirmation est
/// refusée et la transaction bascule en `expire`, par résolution **paresseuse**
/// à la lecture, jamais par une tâche de fond.
pub const EXPIRATION_MINUTES: i64 = 30;

/// Intention de paiement retournée par le prestataire.
pub struct IntentionPaiement {
    /// Référence opposable, unique. Sert de clé d'accès à la confirmation.
    pub reference: String,
    /// Le paiement est-il simulé ? Recopié tel quel sur la transaction.
    pub simule: bool,
}

/// Issue d'un paiement, telle que rapportée par le prestataire.
pub enum EtatPaiement {
    Abouti,
    Echoue,
}

/// Ouvre une intention de paiement et renvoie sa référence.
///
/// En simulation, aucun appel réseau : la référence est fabriquée localement
/// sous la forme `SIM-{date}-{suffixe}`. Le préfixe `SIM-` est **lisible à
/// l'œil nu** dans le journal d'administration, ce qui vaut mieux qu'un drapeau
/// booléen isolé quand on relit une ligne de comptabilité.
///
/// `montant` et `reference_metier` ne servent pas encore : ils sont ce que
/// CinetPay exigera, et les recevoir dès maintenant évite d'avoir à remonter la
/// signature dans le handler le jour du basculement.
pub fn initier(montant: i32, reference_metier: Uuid) -> IntentionPaiement {
    let _ = montant;
    let suffixe = reference_metier.simple().to_string();
    IntentionPaiement {
        reference: format!(
            "SIM-{}-{}",
            Utc::now().format("%Y%m%d"),
            suffixe[..8].to_uppercase()
        ),
        simule: true,
    }
}

/// Recueille l'issue du paiement.
///
/// En simulation, l'issue est **dictée par l'appelant** (`aboutir`) : le membre
/// choisit explicitement de faire aboutir ou échouer son paiement, ce qu'exige
/// le scénario de recette « échec = 0 point, 0 répartition » (SC-005). À
/// l'arrivée de CinetPay, `aboutir` disparaît de la signature et l'issue est
/// lue dans la charge utile signée du webhook.
pub fn confirmer(reference: &str, aboutir: bool) -> EtatPaiement {
    log::info!(
        "Paiement simulé {reference} : {}",
        if aboutir { "abouti" } else { "échoué" }
    );
    if aboutir { EtatPaiement::Abouti } else { EtatPaiement::Echoue }
}
