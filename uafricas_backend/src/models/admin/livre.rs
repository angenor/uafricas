use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ─────────────────────────────────────────────

pub const ADMIN_LIVRE_LISTE_COLONNES: &str =
    "l.id, l.titre, l.type_document, l.acces::TEXT as acces, l.etat,
     l.info_auteur, l.nombre_vues, l.nombre_telechargements,
     cat.nom AS categorie_nom,
     u.nom || ' ' || u.prenom AS cree_par_nom,
     l.created_at";

pub const ADMIN_LIVRE_DETAIL_COLONNES: &str =
    "l.id, l.titre, l.slug, l.description, l.image_couverture_url, l.document_pdf_url,
     l.type_document, l.categorie_id, cat.nom AS categorie_nom,
     l.acces::TEXT as acces, l.info_auteur, l.date_publication,
     l.rapport_auteur, l.condition_diffusion, l.acceptation_diffusion,
     l.langue, l.nombre_pages, l.isbn,
     l.nombre_telechargements, l.nombre_vues,
     l.etat, l.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     l.created_at, l.updated_at";

pub const LIVRE_TRI_COLONNES: &[&str] = &[
    "created_at", "titre", "etat", "type_document", "nombre_vues", "nombre_telechargements",
];

// ── Row & Response ───────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminLivreListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub type_document: String,
    pub acces: String,
    pub etat: String,
    pub info_auteur: String,
    pub nombre_vues: i32,
    pub nombre_telechargements: i32,
    pub categorie_nom: Option<String>,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminLivreDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub document_pdf_url: String,
    pub type_document: String,
    pub categorie_id: Option<Uuid>,
    pub categorie_nom: Option<String>,
    pub acces: String,
    pub info_auteur: String,
    pub date_publication: Option<NaiveDate>,
    pub rapport_auteur: Option<String>,
    pub condition_diffusion: Option<String>,
    pub acceptation_diffusion: bool,
    pub langue: Option<String>,
    pub nombre_pages: Option<i32>,
    pub isbn: Option<String>,
    pub nombre_telechargements: i32,
    pub nombre_vues: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminLivreDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub document_pdf_url: String,
    pub type_document: String,
    pub categorie_id: Option<Uuid>,
    pub categorie_nom: Option<String>,
    pub acces: String,
    pub info_auteur: String,
    pub date_publication: Option<NaiveDate>,
    pub rapport_auteur: Option<String>,
    pub condition_diffusion: Option<String>,
    pub acceptation_diffusion: bool,
    pub langue: Option<String>,
    pub nombre_pages: Option<i32>,
    pub isbn: Option<String>,
    pub nombre_telechargements: i32,
    pub nombre_vues: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub tags: Vec<AdminLivreTagResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminLivreTagResponse {
    pub tag_id: Uuid,
    pub tag_nom: String,
}

impl AdminLivreDetailRow {
    pub fn to_response(&self, tags: Vec<AdminLivreTagResponse>) -> AdminLivreDetailResponse {
        AdminLivreDetailResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            document_pdf_url: self.document_pdf_url.clone(),
            type_document: self.type_document.clone(),
            categorie_id: self.categorie_id,
            categorie_nom: self.categorie_nom.clone(),
            acces: self.acces.clone(),
            info_auteur: self.info_auteur.clone(),
            date_publication: self.date_publication,
            rapport_auteur: self.rapport_auteur.clone(),
            condition_diffusion: self.condition_diffusion.clone(),
            acceptation_diffusion: self.acceptation_diffusion,
            langue: self.langue.clone(),
            nombre_pages: self.nombre_pages,
            isbn: self.isbn.clone(),
            nombre_telechargements: self.nombre_telechargements,
            nombre_vues: self.nombre_vues,
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            tags,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Requests DTO ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerLivreRequest {
    pub titre: String,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub document_pdf_url: String,
    pub type_document: String,
    pub categorie_id: Option<Uuid>,
    pub acces: Option<String>,
    pub info_auteur: String,
    pub date_publication: Option<String>,
    pub rapport_auteur: Option<String>,
    pub condition_diffusion: Option<String>,
    pub acceptation_diffusion: Option<bool>,
    pub langue: Option<String>,
    pub nombre_pages: Option<i32>,
    pub isbn: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierLivreRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub document_pdf_url: Option<String>,
    pub type_document: Option<String>,
    pub categorie_id: Option<Uuid>,
    pub acces: Option<String>,
    pub info_auteur: Option<String>,
    pub date_publication: Option<String>,
    pub rapport_auteur: Option<String>,
    pub condition_diffusion: Option<String>,
    pub acceptation_diffusion: Option<bool>,
    pub langue: Option<String>,
    pub nombre_pages: Option<i32>,
    pub isbn: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatLivreRequest {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct AjouterTagRequest {
    pub tag_id: Uuid,
}

// ── Query Params ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminLivreQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub type_document: Option<String>,
    pub categorie_id: Option<Uuid>,
    pub acces: Option<String>,
    pub etat: Option<String>,
}

// ── Utilitaires ──────────────────────────────────────────────

pub fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace(['é', 'è', 'ê', 'ë'], "e")
        .replace(['à', 'â', 'ä'], "a")
        .replace(['ù', 'û', 'ü'], "u")
        .replace(['î', 'ï'], "i")
        .replace(['ô', 'ö'], "o")
        .replace('ç', "c")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}
