use serde::Deserialize;
use uuid::Uuid;

// ── Requêtes admin pour les propositions de salles publiques (feature 005, US2) ──

/// Requête d'approbation d'une proposition par un admin.
/// Crée la salle associée au groupe ethnique renseigné.
#[derive(Debug, Deserialize)]
pub struct ApprouverPropositionRequest {
    pub groupe_ethnique_id: Uuid,
    pub titre: Option<String>,
    pub image_couverture_url: Option<String>,
    pub langue_code: Option<String>,
    pub alphabet: Option<String>,
    pub dictionnaire_url: Option<String>,
}

/// Requête de refus d'une proposition par un admin.
/// `motif_refus` obligatoire, minimum 5 caractères (vérification applicative).
#[derive(Debug, Deserialize)]
pub struct RefuserPropositionRequest {
    pub motif_refus: String,
}

/// Query params pour la file d'attente des propositions côté admin.
#[derive(Debug, Deserialize)]
pub struct AdminPropositionQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub etat: Option<String>,
    pub q: Option<String>,
    pub pays_id: Option<Uuid>,
}
