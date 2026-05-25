use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ────────────────────────────────────────────────────────────────
// Enums Afripulse (alignés sur country_profile.type_objet_contribution
// et country_profile.section_afripulse — cf. 11c_country_profile_afripulse.sql)
// ────────────────────────────────────────────────────────────────

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sqlx(type_name = "country_profile.type_objet_contribution", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TypeObjetContribution {
    FichePays,
    SiteTouristique,
    SecteurDeveloppement,
    PersonnaliteConnue,
    SavoirPratique,
    RecommandationVisiteur,
    PhotoVisiteur,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sqlx(type_name = "country_profile.section_afripulse", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SectionAfripulse {
    SitesEmblematiques,
    SitesPrives,
    SecteursOpportunites,
    Personnalites,
    SavoirAvantVoyager,
    Recommandations,
    GaleriePhotos,
}

// ────────────────────────────────────────────────────────────────
// Constantes SQL
// ────────────────────────────────────────────────────────────────

/// Colonnes pour SELECT sur contribution_fiche + JOIN iam.utilisateur (auteur).
/// Inclut les colonnes Afripulse (type_objet_contribution, section_afripulse,
/// target_id, nouvelle_valeur_jsonb, ancienne_valeur_jsonb, pieces_jointes).
pub const CONTRIBUTION_COLONNES: &str =
    "cf.id, cf.fiche_pays_id, cf.cree_par, cf.section,
     cf.type_contribution::text AS type_contribution,
     cf.ancienne_valeur, cf.nouvelle_valeur, cf.justification,
     cf.etat::text AS etat, cf.traite_par, cf.note_moderation, cf.traite_at,
     cf.created_at, cf.updated_at,
     cf.type_objet_contribution::text AS type_objet_contribution,
     cf.section_afripulse::text AS section_afripulse,
     cf.target_id,
     cf.nouvelle_valeur_jsonb, cf.ancienne_valeur_jsonb, cf.pieces_jointes,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.photo_url AS auteur_photo_url";

/// Sections modifiables d'une fiche pays
pub const SECTIONS_VALIDES: &[&str] = &[
    "population",
    "superficie_km2",
    "biographie",
    "contexte",
    "contexte_historique",
    "slogan",
    "hymne_national",
    "langue_officielle",
    "langues_populaires",
    "monnaie",
    "fuseau_horaire",
    // Bloc « À savoir avant de voyager »
    "voyage_langue_internationale",
    "voyage_langue_locale",
    "voyage_infos_visa",
    "voyage_infos_sanitaires",
    "voyage_meteo",
    "voyage_prises_electriques",
    "voyage_contacts_tourisme",
    "voyage_recommandations_securite",
    "groupe_ethnique",
    "site_touristique",
    "secteur_developpement",
];

pub fn section_est_valide(section: &str) -> bool {
    SECTIONS_VALIDES.contains(&section)
}

// ────────────────────────────────────────────────────────────────
// Row structs (from DB)
// ────────────────────────────────────────────────────────────────

/// Ligne brute issue de la BDD (JOIN contribution_fiche + iam.utilisateur).
/// Les champs Afripulse (type_objet_contribution, section_afripulse, target_id,
/// *_jsonb, pieces_jointes) sont renseignés pour les nouvelles contributions ;
/// les contributions legacy conservent leurs colonnes TEXT (ancienne/nouvelle_valeur).
#[derive(Debug, FromRow)]
pub struct ContributionFicheRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub cree_par: Uuid,
    pub section: String,
    pub type_contribution: String,
    pub ancienne_valeur: Option<String>,
    pub nouvelle_valeur: String,
    pub justification: Option<String>,
    pub etat: String,
    pub traite_par: Option<Uuid>,
    pub note_moderation: Option<String>,
    pub traite_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Afripulse (11c_country_profile_afripulse.sql)
    pub type_objet_contribution: String,
    pub section_afripulse: Option<String>,
    pub target_id: Option<Uuid>,
    pub nouvelle_valeur_jsonb: Option<serde_json::Value>,
    pub ancienne_valeur_jsonb: Option<serde_json::Value>,
    pub pieces_jointes: serde_json::Value,
    // JOIN iam.utilisateur
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_photo_url: Option<String>,
}

/// Ligne pour les contributeurs (agrege)
#[derive(Debug, FromRow)]
pub struct ContributeurRow {
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
    pub nombre_contributions: i64,
}

// ────────────────────────────────────────────────────────────────
// Response DTOs
// ────────────────────────────────────────────────────────────────

/// Auteur d'une contribution
#[derive(Debug, Serialize)]
pub struct ContributionAuteurResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
}

/// DTO pour une contribution
#[derive(Debug, Serialize)]
pub struct ContributionFicheResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub section: String,
    pub type_contribution: String,
    pub ancienne_valeur: Option<String>,
    pub nouvelle_valeur: String,
    pub justification: Option<String>,
    pub etat: String,
    pub auteur: ContributionAuteurResponse,
    pub traite_par: Option<Uuid>,
    pub note_moderation: Option<String>,
    pub traite_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Reponse paginee pour la liste des contributions
#[derive(Debug, Serialize)]
pub struct ContributionListeResponse {
    pub contributions: Vec<ContributionFicheResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// DTO pour un contributeur
#[derive(Debug, Serialize)]
pub struct ContributeurResponse {
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
    pub nombre_contributions: i64,
}

// ────────────────────────────────────────────────────────────────
// Request DTOs
// ────────────────────────────────────────────────────────────────

/// Body pour soumettre une contribution.
///
/// Deux modes coexistent :
///   • **Legacy** (fiche pays scalaire) : `section` + `nouvelle_valeur` TEXT uniquement.
///   • **Afripulse** : `type_objet_contribution` ≠ "fiche_pays" ; `nouvelle_valeur_jsonb`
///     (payload structuré) ; `section_afripulse` ; `target_id` pour edition/suppression.
#[derive(Debug, Deserialize)]
pub struct CreerContributionBody {
    pub section: String,
    pub type_contribution: Option<String>,
    #[serde(default)]
    pub nouvelle_valeur: Option<String>,
    pub justification: Option<String>,
    // Afripulse
    pub type_objet_contribution: Option<String>,
    pub section_afripulse: Option<String>,
    pub target_id: Option<Uuid>,
    pub nouvelle_valeur_jsonb: Option<serde_json::Value>,
}

/// Parametres de requete pour lister les contributions
#[derive(Debug, Deserialize)]
pub struct ContributionQueryParams {
    pub etat: Option<String>,
    pub section: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Body pour la moderation (validation/rejet)
#[derive(Debug, Deserialize)]
pub struct ModerationBody {
    pub note_moderation: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Fonctions de construction
// ────────────────────────────────────────────────────────────────

/// Construit un ContributionFicheResponse depuis une row
pub fn construire_contribution_response(row: &ContributionFicheRow) -> ContributionFicheResponse {
    ContributionFicheResponse {
        id: row.id,
        fiche_pays_id: row.fiche_pays_id,
        section: row.section.clone(),
        type_contribution: row.type_contribution.clone(),
        ancienne_valeur: row.ancienne_valeur.clone(),
        nouvelle_valeur: row.nouvelle_valeur.clone(),
        justification: row.justification.clone(),
        etat: row.etat.clone(),
        auteur: ContributionAuteurResponse {
            id: row.cree_par,
            nom: row.auteur_nom.clone(),
            prenom: row.auteur_prenom.clone(),
            photo_url: row.auteur_photo_url.clone(),
        },
        traite_par: row.traite_par,
        note_moderation: row.note_moderation.clone(),
        traite_at: row.traite_at,
        created_at: row.created_at,
    }
}

/// Construit un ContributeurResponse depuis une row
pub fn construire_contributeur_response(row: &ContributeurRow) -> ContributeurResponse {
    ContributeurResponse {
        utilisateur_id: row.utilisateur_id,
        nom: row.nom.clone(),
        prenom: row.prenom.clone(),
        photo_url: row.photo_url.clone(),
        nombre_contributions: row.nombre_contributions,
    }
}
