// ════════════════════════════════════════════════════════════════════════════
// Modèles — Matching inter-arbres et découvertes
// ════════════════════════════════════════════════════════════════════════════

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

// ─── Structs BDD (FromRow) ──────────────────────────────────────────────

#[derive(Debug, sqlx::FromRow)]
pub struct SuggestionCorrespondance {
    pub id: Uuid,
    pub rattachement_a_id: Uuid,
    pub rattachement_b_id: Uuid,
    pub score: f32,
    pub score_nom: Option<f32>,
    pub score_prenoms: Option<f32>,
    pub score_date: Option<f32>,
    pub score_lieu: Option<f32>,
    pub score_genre: Option<f32>,
    pub statut: String,
    pub confirmee_par_a: bool,
    pub confirmee_par_b: bool,
    pub detectee_le: DateTime<Utc>,
    pub confirmee_le: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct DemandeContact {
    pub id: Uuid,
    pub suggestion_id: Uuid,
    pub demandeur_id: Uuid,
    pub destinataire_id: Uuid,
    pub statut: String,
    pub created_at: DateTime<Utc>,
    pub traitee_le: Option<DateTime<Utc>>,
}

// ─── Struct pour jointure suggestion + personnes ────────────────────────

#[derive(Debug, sqlx::FromRow)]
pub struct SuggestionAvecPersonnesRow {
    // Suggestion
    pub suggestion_id: Uuid,
    pub score: f32,
    pub score_nom: Option<f32>,
    pub score_prenoms: Option<f32>,
    pub score_date: Option<f32>,
    pub score_lieu: Option<f32>,
    pub score_genre: Option<f32>,
    pub statut: String,
    pub confirmee_par_a: bool,
    pub confirmee_par_b: bool,
    pub detectee_le: DateTime<Utc>,
    pub confirmee_le: Option<DateTime<Utc>>,
    // Personne A
    pub a_personne_id: Uuid,
    pub a_rattachement_id: Uuid,
    pub a_nom: String,
    pub a_prenoms: Option<String>,
    pub a_naissance_annee: Option<i16>,
    pub a_naissance_lieu: Option<String>,
    pub a_genre: Option<String>,
    // Personne B
    pub b_personne_id: Uuid,
    pub b_rattachement_id: Uuid,
    pub b_nom: String,
    pub b_prenoms: Option<String>,
    pub b_naissance_annee: Option<i16>,
    pub b_naissance_lieu: Option<String>,
    pub b_genre: Option<String>,
    // Arbres
    pub a_arbre_id: Uuid,
    pub b_arbre_id: Uuid,
    // Utilisateur de l'autre arbre (pour anonymisation)
    pub b_utilisateur_id: Uuid,
}

// ─── DTOs de réponse (Serialize) ────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PersonneResumeResponse {
    pub id: Uuid,
    pub rattachement_id: Uuid,
    pub nom: String,
    pub prenoms: Option<String>,
    pub naissance_annee: Option<i16>,
    pub naissance_lieu: Option<String>,
    pub genre: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DetailsScoreResponse {
    pub nom: Option<f32>,
    pub prenoms: Option<f32>,
    pub date: Option<f32>,
    pub lieu: Option<f32>,
    pub genre: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct SuggestionResponse {
    pub id: Uuid,
    pub ma_personne: PersonneResumeResponse,
    pub personne_matchee: PersonneResumeResponse,
    pub score: f32,
    pub details_score: DetailsScoreResponse,
    pub statut: String,
    pub membre_id_anonymise: String,
    pub detectee_le: DateTime<Utc>,
    pub confirmee_le: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct DecouvertesResponse {
    pub en_attente: SectionDecouvertes,
    pub en_cours: SectionDecouvertes,
    pub confirmees: SectionDecouvertes,
}

#[derive(Debug, Serialize)]
pub struct SectionDecouvertes {
    pub suggestions: Vec<SuggestionResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct DemandeContactResponse {
    pub id: Uuid,
    pub suggestion_id: Uuid,
    pub statut: String,
    pub profil_membre: Option<ProfilPublicResponse>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProfilPublicResponse {
    pub nom: String,
    pub prenom: String,
    pub email: String,
}

// ─── Helper : anonymiser un UUID utilisateur ────────────────────────────

pub fn anonymiser_membre(utilisateur_id: Uuid) -> String {
    let hex = utilisateur_id.to_string();
    let court = &hex[hex.len().saturating_sub(4)..];
    format!("Membre #{}", court)
}
