use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour le mapping (exclut search_vector TSVECTOR)
/// Le cast acces::text convertit l'enum PostgreSQL en texte pour sqlx
pub const LIVRE_COLONNES: &str =
    "id, titre, slug, description, image_couverture_url, document_pdf_url,
     type_document, categorie_id, acces::text AS acces, info_auteur, date_publication,
     rapport_auteur, condition_diffusion, acceptation_diffusion, langue,
     nombre_pages, isbn, nombre_telechargements, nombre_vues, etat,
     cree_par, created_at, updated_at, deleted_at";

/// Representation complete d'un livre en base de donnees
/// Correspond a la table media_content.livre
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Livre {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub document_pdf_url: String,
    pub type_document: String,
    pub categorie_id: Option<Uuid>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// DTO pour la reponse API
#[derive(Debug, Serialize)]
pub struct LivreResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub document_pdf_url: String,
    pub type_document: String,
    pub acces: String,
    pub info_auteur: String,
    pub date_publication: Option<NaiveDate>,
    pub rapport_auteur: Option<String>,
    pub acceptation_diffusion: bool,
    pub nombre_telechargements: i32,
    pub nombre_vues: i32,
    pub etat: String,
    pub created_at: DateTime<Utc>,
}

impl From<Livre> for LivreResponse {
    fn from(l: Livre) -> Self {
        Self {
            id: l.id,
            titre: l.titre,
            slug: l.slug,
            description: l.description,
            image_couverture_url: l.image_couverture_url,
            document_pdf_url: l.document_pdf_url,
            type_document: l.type_document,
            acces: l.acces,
            info_auteur: l.info_auteur,
            date_publication: l.date_publication,
            rapport_auteur: l.rapport_auteur,
            acceptation_diffusion: l.acceptation_diffusion,
            nombre_telechargements: l.nombre_telechargements,
            nombre_vues: l.nombre_vues,
            etat: l.etat,
            created_at: l.created_at,
        }
    }
}

/// Parametres de requete pour le listing des livres
#[derive(Debug, Deserialize)]
pub struct LivreQueryParams {
    pub recherche: Option<String>,
    pub type_document: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Reponse paginee pour le listing des livres
#[derive(Debug, Serialize)]
pub struct LivreListeResponse {
    pub livres: Vec<LivreResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}
