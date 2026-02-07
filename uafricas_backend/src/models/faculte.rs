use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour exchange.faculte
pub const FACULTE_COLONNES: &str =
    "f.id, f.titre, f.acronyme, f.slug, f.description,
     f.image_couverture_url, f.logo_url, f.ecole_partenaire_id,
     f.domaines_etudes, f.programmes_licence, f.programmes_master,
     f.programmes_doctorat, f.programmes_certificats,
     f.diplome_minimum, f.langues_enseignement,
     f.frais_scolarite_min, f.frais_scolarite_max,
     f.bourses_possibles, f.periodes_inscription,
     f.points_forts, f.accepte_nouveaux_inscrits,
     f.statut, f.referent_id,
     f.nombre_inscrits_total, f.nombre_inscrits_annee,
     f.created_at, f.updated_at";

/// Colonnes a selectionner pour exchange.ecole_partenaire
pub const ECOLE_COLONNES: &str =
    "e.id, e.nom, e.ville, e.pays_id, e.type, e.site_web,
     e.email_contact, e.telephone_contact, e.whatsapp_contact";

/// Representation d'une faculte en base de donnees
#[derive(Debug, FromRow)]
pub struct FaculteRow {
    pub id: Uuid,
    pub titre: String,
    pub acronyme: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub logo_url: Option<String>,
    pub ecole_partenaire_id: Uuid,
    pub domaines_etudes: Vec<String>,
    pub programmes_licence: Vec<String>,
    pub programmes_master: Vec<String>,
    pub programmes_doctorat: Vec<String>,
    pub programmes_certificats: Vec<String>,
    pub diplome_minimum: Option<String>,
    pub langues_enseignement: Vec<String>,
    pub frais_scolarite_min: Option<i32>,
    pub frais_scolarite_max: Option<i32>,
    pub bourses_possibles: bool,
    pub periodes_inscription: Option<String>,
    pub points_forts: Vec<String>,
    pub accepte_nouveaux_inscrits: bool,
    pub statut: String,
    pub referent_id: Option<Uuid>,
    pub nombre_inscrits_total: i32,
    pub nombre_inscrits_annee: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Representation d'une ecole partenaire en base de donnees
#[derive(Debug, FromRow)]
pub struct EcolePartenaireRow {
    pub id: Uuid,
    pub nom: String,
    pub ville: String,
    pub pays_id: Uuid,
    #[sqlx(rename = "type")]
    pub type_ecole: String,
    pub site_web: Option<String>,
    pub email_contact: String,
    pub telephone_contact: Option<String>,
    pub whatsapp_contact: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// DTOs de reponse
// ──────────────────────────────────────────────────────────────

/// DTO ecole partenaire
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EcolePartenaireResponse {
    pub nom: String,
    pub ville: String,
    pub pays: String,
    #[serde(rename = "type")]
    pub type_ecole: String,
    pub site_web: Option<String>,
    pub email_contact: String,
    pub telephone_contact: Option<String>,
    pub whatsapp_contact: Option<String>,
}

/// DTO programmes resume
#[derive(Debug, Serialize)]
pub struct ProgrammesResumeResponse {
    pub licence: Vec<String>,
    pub master: Vec<String>,
    pub doctorat: Vec<String>,
    pub certificats: Vec<String>,
}

/// DTO conditions admission
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionsAdmissionResponse {
    pub diplome_minimum: Option<String>,
    pub langues_enseignement: Vec<String>,
    pub frais_scolarite_annuels: FraisScolariteResponse,
    pub periodes_inscription: Option<String>,
}

/// DTO frais scolarite
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FraisScolariteResponse {
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub bourses_possibles: bool,
}

/// DTO stats
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse {
    pub nombre_inscrits_total: i32,
    pub nombre_inscrits_annee_en_cours: i32,
}

/// DTO referent plateforme
#[derive(Debug, Serialize)]
pub struct ReferentResponse {
    pub prenom: Option<String>,
    pub nom: String,
    pub role: String,
    pub email: String,
}

/// DTO pour une faculte dans la liste
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FaculteResponse {
    pub id: Uuid,
    pub titre: String,
    pub acronyme: String,
    pub description: String,
    pub image_couverture: Option<String>,
    pub ecole_partenaire: EcolePartenaireResponse,
    pub domaines_etudes: Vec<String>,
    pub accepte_nouveaux_inscrits: bool,
    pub stats: StatsResponse,
}

/// DTO pour le detail d'une faculte
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FaculteDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub acronyme: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture: Option<String>,
    pub logo: Option<String>,
    pub ecole_partenaire: EcolePartenaireResponse,
    pub domaines_etudes: Vec<String>,
    pub programmes_resume: ProgrammesResumeResponse,
    pub conditions_admission: ConditionsAdmissionResponse,
    pub points_forts: Vec<String>,
    pub accepte_nouveaux_inscrits: bool,
    pub statut: String,
    pub stats: StatsResponse,
    pub referent_plateforme: Option<ReferentResponse>,
}

/// Reponse paginee pour le listing
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FaculteListeResponse {
    pub facultes: Vec<FaculteResponse>,
    pub domaines: Vec<String>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Parametres de requete pour le listing
#[derive(Debug, Deserialize)]
pub struct FaculteQueryParams {
    pub recherche: Option<String>,
    pub domaine: Option<String>,
    pub type_ecole: Option<String>,
    pub ouvertes: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Info referent jointe depuis iam.utilisateur
#[derive(Debug, FromRow)]
pub struct ReferentInfo {
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
}
