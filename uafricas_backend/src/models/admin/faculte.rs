use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ════════════════════════════════════════════════════════════════════
// ÉCOLES PARTENAIRES
// ════════════════════════════════════════════════════════════════════

pub const ADMIN_ECOLE_LISTE_COLONNES: &str =
    "e.id, e.nom, e.ville, e.pays_id, e.type::text AS type_ecole, e.actif, e.created_at,
     p.nom AS pays_nom,
     (SELECT COUNT(*) FROM exchange.faculte f WHERE f.ecole_partenaire_id = e.id AND f.deleted_at IS NULL) AS nombre_facultes";

pub const ADMIN_ECOLE_DETAIL_COLONNES: &str =
    "e.id, e.nom, e.ville, e.pays_id, e.type::text AS type_ecole, e.site_web,
     e.email_contact, e.telephone_contact, e.whatsapp_contact, e.actif,
     e.created_at, e.updated_at, p.nom AS pays_nom";

pub const ECOLE_TRI_COLONNES: &[&str] = &["created_at", "nom", "ville"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminEcoleListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub ville: String,
    pub pays_id: Uuid,
    pub type_ecole: String,
    pub pays_nom: Option<String>,
    pub actif: bool,
    pub nombre_facultes: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminEcoleDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub ville: String,
    pub pays_id: Uuid,
    pub type_ecole: String,
    pub site_web: Option<String>,
    pub email_contact: String,
    pub telephone_contact: Option<String>,
    pub whatsapp_contact: Option<String>,
    pub actif: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreerEcoleRequest {
    pub nom: String,
    pub ville: String,
    pub pays_id: Uuid,
    #[serde(rename = "type")]
    pub type_ecole: String,
    pub site_web: Option<String>,
    pub email_contact: String,
    pub telephone_contact: Option<String>,
    pub whatsapp_contact: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierEcoleRequest {
    pub nom: String,
    pub ville: String,
    pub pays_id: Uuid,
    #[serde(rename = "type")]
    pub type_ecole: String,
    pub site_web: Option<String>,
    pub email_contact: String,
    pub telephone_contact: Option<String>,
    pub whatsapp_contact: Option<String>,
    pub actif: Option<bool>,
}

// ════════════════════════════════════════════════════════════════════
// FACULTÉS
// ════════════════════════════════════════════════════════════════════

pub const ADMIN_FACULTE_LISTE_COLONNES: &str =
    "f.id, f.titre, f.acronyme, f.statut, f.accepte_nouveaux_inscrits,
     f.nombre_inscrits_total, f.created_at,
     e.nom AS ecole_nom, p.nom AS pays_nom";

pub const ADMIN_FACULTE_DETAIL_COLONNES: &str =
    "f.id, f.titre, f.acronyme, f.slug, f.description, f.image_couverture_url, f.logo_url,
     f.ecole_partenaire_id, f.domaines_etudes, f.programmes_licence, f.programmes_master,
     f.programmes_doctorat, f.programmes_certificats, f.diplome_minimum, f.langues_enseignement,
     f.frais_scolarite_min, f.frais_scolarite_max, f.bourses_possibles, f.periodes_inscription,
     f.points_forts, f.accepte_nouveaux_inscrits, f.statut, f.referent_id,
     f.nombre_inscrits_total, f.nombre_inscrits_annee, f.created_at, f.updated_at,
     e.nom AS ecole_nom";

pub const FACULTE_TRI_COLONNES: &[&str] = &["created_at", "titre", "acronyme"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminFaculteListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub acronyme: String,
    pub statut: String,
    pub accepte_nouveaux_inscrits: bool,
    pub nombre_inscrits_total: i32,
    pub ecole_nom: Option<String>,
    pub pays_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminFaculteDetailResponse {
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
    pub ecole_nom: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreerFaculteRequest {
    pub titre: String,
    pub acronyme: String,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub logo_url: Option<String>,
    pub ecole_partenaire_id: Uuid,
    #[serde(default)]
    pub domaines_etudes: Vec<String>,
    #[serde(default)]
    pub programmes_licence: Vec<String>,
    #[serde(default)]
    pub programmes_master: Vec<String>,
    #[serde(default)]
    pub programmes_doctorat: Vec<String>,
    #[serde(default)]
    pub programmes_certificats: Vec<String>,
    pub diplome_minimum: Option<String>,
    #[serde(default)]
    pub langues_enseignement: Vec<String>,
    pub frais_scolarite_min: Option<i32>,
    pub frais_scolarite_max: Option<i32>,
    #[serde(default)]
    pub bourses_possibles: bool,
    pub periodes_inscription: Option<String>,
    #[serde(default)]
    pub points_forts: Vec<String>,
    #[serde(default = "vrai")]
    pub accepte_nouveaux_inscrits: bool,
    pub statut: Option<String>,
    pub referent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierFaculteRequest {
    pub titre: String,
    pub acronyme: String,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub logo_url: Option<String>,
    pub ecole_partenaire_id: Uuid,
    #[serde(default)]
    pub domaines_etudes: Vec<String>,
    #[serde(default)]
    pub programmes_licence: Vec<String>,
    #[serde(default)]
    pub programmes_master: Vec<String>,
    #[serde(default)]
    pub programmes_doctorat: Vec<String>,
    #[serde(default)]
    pub programmes_certificats: Vec<String>,
    pub diplome_minimum: Option<String>,
    #[serde(default)]
    pub langues_enseignement: Vec<String>,
    pub frais_scolarite_min: Option<i32>,
    pub frais_scolarite_max: Option<i32>,
    #[serde(default)]
    pub bourses_possibles: bool,
    pub periodes_inscription: Option<String>,
    #[serde(default)]
    pub points_forts: Vec<String>,
    #[serde(default = "vrai")]
    pub accepte_nouveaux_inscrits: bool,
    pub statut: Option<String>,
    pub referent_id: Option<Uuid>,
    pub nombre_inscrits_total: Option<i32>,
    pub nombre_inscrits_annee: Option<i32>,
}

fn vrai() -> bool {
    true
}

// ── Query Params communs ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct AdminFaculteQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub ecole_partenaire_id: Option<Uuid>,
    pub statut: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminEcoleQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub pays_id: Option<Uuid>,
}
