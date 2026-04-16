use serde::Deserialize;
use uuid::Uuid;

// ── Requêtes admin pour les modérateurs Afrolang attitrés (feature 005, US3) ──

/// Requête de désignation d'un modérateur attitré sur une salle publique.
#[derive(Debug, Deserialize)]
pub struct DesignerModerateurRequest {
    pub utilisateur_id: Uuid,
    pub disponibilite: Option<String>,
}
