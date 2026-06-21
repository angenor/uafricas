use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL pour la requete de listing (avec jointures aplaties) ────

/// Colonnes SELECT pour le listing des annonces
/// Utilise l'alias `a.` pour la table annonce
/// Les enums PostgreSQL sont castes en ::text pour compatibilite sqlx
/// Les DECIMAL sont castes en ::float8 pour mapping vers f64
pub const ANNONCE_LISTE_COLONNES: &str =
    "a.id, a.titre, a.slug, a.description,
     a.type_operation::text AS type_operation,
     a.condition_article::text AS condition_article,
     a.prix::float8 AS prix, a.devise,
     a.prix_negociable, a.ville, a.adresse,
     a.type_contact::text AS type_contact, a.contact_info,
     a.quantite, a.nombre_vues, a.cree_par,
     a.created_at, a.updated_at,
     c.nom AS categorie_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email,
     u.telephone_verifie AS auteur_telephone_verifie,
     u.documents_verifie AS auteur_documents_verifie,
     u.compte_verifie_admin AS auteur_compte_verifie_admin,
     (SELECT am.media_url FROM marketplace.annonce_media am
      WHERE am.annonce_id = a.id AND am.est_principale = TRUE
      LIMIT 1) AS photo_url,
     (SELECT p.nom FROM marketplace.annonce_pays ap
      JOIN shared.pays p ON p.id = ap.pays_id
      WHERE ap.annonce_id = a.id LIMIT 1) AS pays_nom";

// ── Row aplati pour le listing ───────────────────────────────

/// Row unifie pour la requete de listing (annonce + jointures aplaties)
#[derive(Debug, FromRow)]
pub struct AnnonceListeRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_operation: String,
    pub condition_article: Option<String>,
    pub prix: Option<f64>,
    pub devise: Option<String>,
    pub prix_negociable: Option<bool>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub type_contact: Option<String>,
    pub contact_info: Option<String>,
    pub quantite: Option<i32>,
    pub nombre_vues: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs joints
    pub categorie_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub auteur_telephone_verifie: Option<bool>,
    pub auteur_documents_verifie: Option<bool>,
    pub auteur_compte_verifie_admin: Option<bool>,
    pub photo_url: Option<String>,
    pub pays_nom: Option<String>,
}

// ── Row pour le detail (memes champs de base) ────────────────

/// Row pour la requete de detail (sans sous-requetes, les relations sont chargees a part)
#[derive(Debug, FromRow)]
pub struct AnnonceDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_operation: String,
    pub condition_article: Option<String>,
    pub prix: Option<f64>,
    pub devise: Option<String>,
    pub prix_negociable: Option<bool>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub type_contact: Option<String>,
    pub contact_info: Option<String>,
    pub type_annonceur: Option<String>,
    pub nom_entreprise: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_email: Option<String>,
    pub contact_adresse: Option<String>,
    pub site_web_url: Option<String>,
    pub quantite: Option<i32>,
    pub nombre_vues: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Jointures
    pub categorie_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub auteur_telephone_verifie: Option<bool>,
    pub auteur_documents_verifie: Option<bool>,
    pub auteur_compte_verifie_admin: Option<bool>,
}

/// Colonnes SELECT pour le detail d'une annonce
pub const ANNONCE_DETAIL_COLONNES: &str =
    "a.id, a.titre, a.slug, a.description,
     a.type_operation::text AS type_operation,
     a.condition_article::text AS condition_article,
     a.prix::float8 AS prix, a.devise,
     a.prix_negociable, a.ville, a.adresse,
     a.longitude::float8 AS longitude, a.latitude::float8 AS latitude,
     a.type_contact::text AS type_contact, a.contact_info,
     a.type_annonceur::text AS type_annonceur, a.nom_entreprise,
     a.contact_telephone, a.contact_email, a.contact_adresse, a.site_web_url,
     a.quantite, a.nombre_vues, a.cree_par,
     a.created_at, a.updated_at,
     c.nom AS categorie_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email,
     u.telephone_verifie AS auteur_telephone_verifie,
     u.documents_verifie AS auteur_documents_verifie,
     u.compte_verifie_admin AS auteur_compte_verifie_admin";

// ── Rows pour les relations ──────────────────────────────────

/// Row pour les medias d'une annonce
#[derive(Debug, FromRow)]
pub struct AnnonceMediaRow {
    pub id: Uuid,
    pub media_url: String,
    pub type_mime: Option<String>,
    pub est_principale: Option<bool>,
    pub ordre: Option<i32>,
}

/// Row pour les pays d'une annonce
#[derive(Debug, FromRow)]
pub struct AnnoncePaysRow {
    pub pays_nom: String,
}

// ── DTOs de reponse ──────────────────────────────────────────

/// DTO auteur dans la reponse API
#[derive(Debug, Serialize)]
pub struct AnnonceAuteurResponse {
    pub uid: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    /// Numero de telephone verifie par OTP
    pub telephone_verifie: bool,
    /// Piece d'identite verifiee
    pub documents_verifie: bool,
    /// Compte valide/actif par l'administration
    pub compte_valide: bool,
}

/// DTO media dans la reponse detail
#[derive(Debug, Serialize)]
pub struct AnnonceMediaResponse {
    pub id: Uuid,
    pub media_url: String,
    pub type_mime: Option<String>,
    pub est_principale: bool,
    pub ordre: i32,
}

/// DTO pour une annonce dans la liste
#[derive(Debug, Serialize)]
pub struct AnnonceResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_echange: String,
    pub categorie: String,
    pub condition_article: String,
    pub prix: f64,
    pub devise: String,
    pub prix_negociable: bool,
    pub pays: String,
    pub ville: Option<String>,
    pub tel: Option<String>,
    pub photo_url: Option<String>,
    pub quantite: Option<i32>,
    pub nombre_vues: i32,
    pub user: AnnonceAuteurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le detail d'une annonce
#[derive(Debug, Serialize)]
pub struct AnnonceDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_echange: String,
    pub categorie: String,
    pub condition_article: String,
    pub prix: f64,
    pub devise: String,
    pub prix_negociable: bool,
    pub pays: Vec<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub type_contact: String,
    pub contact_info: Option<String>,
    /// 'particulier' ou 'entreprise'
    pub type_annonceur: String,
    /// Nom de l'entreprise (si type_annonceur = entreprise)
    pub nom_entreprise: Option<String>,
    /// Coordonnées publiques — renseignées uniquement pour une entreprise
    pub contact_telephone: Option<String>,
    pub contact_email: Option<String>,
    pub contact_adresse: Option<String>,
    /// Site web ou page réseau social (facultatif)
    pub site_web_url: Option<String>,
    pub quantite: Option<i32>,
    pub nombre_vues: i32,
    pub medias: Vec<AnnonceMediaResponse>,
    pub photo_url: Option<String>,
    pub user: AnnonceAuteurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Reponse paginee pour le listing des annonces
#[derive(Debug, Serialize)]
pub struct AnnonceListeResponse {
    pub annonces: Vec<AnnonceResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── DTOs membre (feature 001-marche-achat-vente-troc-don) ────

/// Corps JSON pour contacter l'auteur d'une annonce via la messagerie.
#[derive(Debug, Deserialize)]
pub struct ContacterAuteurRequest {
    pub message: String,
}

/// Colonnes SELECT pour « Mes annonces » (inclut `etat`, vue propriétaire).
pub const MES_ANNONCES_COLONNES: &str =
    "a.id, a.titre, a.slug, a.description,
     a.type_operation::text AS type_operation,
     a.condition_article::text AS condition_article,
     a.prix::float8 AS prix, a.devise,
     a.prix_negociable, a.ville, a.adresse,
     a.type_contact::text AS type_contact, a.contact_info,
     a.quantite, a.nombre_vues, a.cree_par,
     a.etat::text AS etat,
     a.created_at, a.updated_at,
     c.nom AS categorie_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email,
     u.telephone_verifie AS auteur_telephone_verifie,
     u.documents_verifie AS auteur_documents_verifie,
     u.compte_verifie_admin AS auteur_compte_verifie_admin,
     (SELECT am.media_url FROM marketplace.annonce_media am
      WHERE am.annonce_id = a.id AND am.est_principale = TRUE
      LIMIT 1) AS photo_url,
     (SELECT p.nom FROM marketplace.annonce_pays ap
      JOIN shared.pays p ON p.id = ap.pays_id
      WHERE ap.annonce_id = a.id LIMIT 1) AS pays_nom,
     (SELECT COUNT(*) FROM marketplace.annonce_media am WHERE am.annonce_id = a.id) AS nombre_medias";

/// Row pour « Mes annonces » (listing propriétaire avec état).
#[derive(Debug, FromRow)]
pub struct MesAnnonceRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_operation: String,
    pub condition_article: Option<String>,
    pub prix: Option<f64>,
    pub devise: Option<String>,
    pub prix_negociable: Option<bool>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub type_contact: Option<String>,
    pub contact_info: Option<String>,
    pub quantite: Option<i32>,
    pub nombre_vues: i32,
    pub cree_par: Uuid,
    pub etat: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub categorie_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub auteur_telephone_verifie: Option<bool>,
    pub auteur_documents_verifie: Option<bool>,
    pub auteur_compte_verifie_admin: Option<bool>,
    pub photo_url: Option<String>,
    pub pays_nom: Option<String>,
    pub nombre_medias: i64,
}

/// DTO d'un item « Mes annonces » (étend la réponse de liste avec `etat`).
#[derive(Debug, Serialize)]
pub struct MesAnnoncesItemResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_echange: String,
    pub categorie: String,
    pub condition_article: String,
    pub prix: f64,
    pub devise: String,
    pub prix_negociable: bool,
    pub pays: String,
    pub ville: Option<String>,
    pub photo_url: Option<String>,
    pub quantite: Option<i32>,
    pub nombre_vues: i32,
    pub nombre_medias: i64,
    pub etat: String,
    pub user: AnnonceAuteurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MesAnnonceRow {
    pub fn to_response(&self) -> MesAnnoncesItemResponse {
        MesAnnoncesItemResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            type_echange: mapper_type_operation(&self.type_operation),
            categorie: self.categorie_nom.clone().unwrap_or_else(|| "Autre".to_string()),
            condition_article: mapper_condition(
                self.condition_article.as_deref().unwrap_or("non_applicable"),
            ),
            prix: self.prix.unwrap_or(0.0),
            devise: self.devise.clone().unwrap_or_else(|| "XOF".to_string()),
            prix_negociable: self.prix_negociable.unwrap_or(false),
            pays: self.pays_nom.clone().unwrap_or_else(|| "Non spécifié".to_string()),
            ville: self.ville.clone(),
            photo_url: self.photo_url.clone(),
            quantite: self.quantite,
            nombre_vues: self.nombre_vues,
            nombre_medias: self.nombre_medias,
            etat: self.etat.clone(),
            user: AnnonceAuteurResponse {
                uid: self.cree_par,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                email: self.auteur_email.clone(),
                telephone_verifie: self.auteur_telephone_verifie.unwrap_or(false),
                documents_verifie: self.auteur_documents_verifie.unwrap_or(false),
                compte_valide: self.auteur_compte_verifie_admin.unwrap_or(false),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Réponse paginée pour « Mes annonces ».
#[derive(Debug, Serialize)]
pub struct MesAnnoncesResponse {
    pub annonces: Vec<MesAnnoncesItemResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Parametres de requete ────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AnnonceQueryParams {
    pub recherche: Option<String>,
    pub type_operation: Option<String>,
    pub categorie: Option<String>,
    pub prix_min: Option<f64>,
    pub prix_max: Option<f64>,
    pub tri: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Pagination des listings membre (« Mes annonces », favoris).
#[derive(Debug, Deserialize)]
pub struct MembreListeParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub etat: Option<String>,
}

// ── Conversions ──────────────────────────────────────────────

/// Mapper type_operation DB (snake_case) vers label frontend (PascalCase)
pub fn mapper_type_operation(db_val: &str) -> String {
    match db_val {
        "vente" => "Vente".to_string(),
        "troc" => "Troc".to_string(),
        "don" => "Don".to_string(),
        "association" => "Association".to_string(),
        "opportunite" => "Opportunité".to_string(),
        autre => autre.to_string(),
    }
}

/// Mapper condition_article DB vers label frontend
pub fn mapper_condition(db_val: &str) -> String {
    match db_val {
        "neuf" => "Neuf".to_string(),
        "occasion" => "Occasion".to_string(),
        "reconditionne" => "Reconditionné".to_string(),
        "non_applicable" => "Non applicable".to_string(),
        autre => autre.to_string(),
    }
}

impl AnnonceListeRow {
    /// Convertir une row de listing en DTO de reponse
    pub fn to_response(&self) -> AnnonceResponse {
        AnnonceResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            type_echange: mapper_type_operation(&self.type_operation),
            categorie: self.categorie_nom.clone().unwrap_or_else(|| "Autre".to_string()),
            condition_article: mapper_condition(
                self.condition_article.as_deref().unwrap_or("non_applicable"),
            ),
            prix: self.prix.unwrap_or(0.0),
            devise: self.devise.clone().unwrap_or_else(|| "XOF".to_string()),
            prix_negociable: self.prix_negociable.unwrap_or(false),
            pays: self.pays_nom.clone().unwrap_or_else(|| "Non spécifié".to_string()),
            ville: self.ville.clone(),
            tel: self.contact_info.clone(),
            photo_url: self.photo_url.clone(),
            quantite: self.quantite,
            nombre_vues: self.nombre_vues,
            user: AnnonceAuteurResponse {
                uid: self.cree_par,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                email: self.auteur_email.clone(),
                telephone_verifie: self.auteur_telephone_verifie.unwrap_or(false),
                documents_verifie: self.auteur_documents_verifie.unwrap_or(false),
                compte_valide: self.auteur_compte_verifie_admin.unwrap_or(false),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl AnnonceDetailRow {
    /// Convertir une row de detail en DTO de reponse avec les relations
    pub fn to_detail_response(
        &self,
        pays: Vec<String>,
        medias: Vec<AnnonceMediaResponse>,
    ) -> AnnonceDetailResponse {
        let photo_url = medias
            .iter()
            .find(|m| m.est_principale)
            .map(|m| m.media_url.clone())
            .or_else(|| medias.first().map(|m| m.media_url.clone()));

        let est_entreprise = self.type_annonceur.as_deref() == Some("entreprise");

        AnnonceDetailResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            type_echange: mapper_type_operation(&self.type_operation),
            categorie: self.categorie_nom.clone().unwrap_or_else(|| "Autre".to_string()),
            condition_article: mapper_condition(
                self.condition_article.as_deref().unwrap_or("non_applicable"),
            ),
            prix: self.prix.unwrap_or(0.0),
            devise: self.devise.clone().unwrap_or_else(|| "XOF".to_string()),
            prix_negociable: self.prix_negociable.unwrap_or(false),
            pays,
            ville: self.ville.clone(),
            adresse: self.adresse.clone(),
            longitude: self.longitude,
            latitude: self.latitude,
            type_contact: self.type_contact.clone().unwrap_or_else(|| "email".to_string()),
            contact_info: self.contact_info.clone(),
            type_annonceur: self
                .type_annonceur
                .clone()
                .unwrap_or_else(|| "particulier".to_string()),
            nom_entreprise: self.nom_entreprise.clone(),
            // Confidentialité : coordonnées publiques exposées uniquement pour une entreprise.
            contact_telephone: if est_entreprise { self.contact_telephone.clone() } else { None },
            contact_email: if est_entreprise { self.contact_email.clone() } else { None },
            contact_adresse: if est_entreprise { self.contact_adresse.clone() } else { None },
            site_web_url: self.site_web_url.clone(),
            quantite: self.quantite,
            nombre_vues: self.nombre_vues,
            medias,
            photo_url,
            user: AnnonceAuteurResponse {
                uid: self.cree_par,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                email: self.auteur_email.clone(),
                telephone_verifie: self.auteur_telephone_verifie.unwrap_or(false),
                documents_verifie: self.auteur_documents_verifie.unwrap_or(false),
                compte_valide: self.auteur_compte_verifie_admin.unwrap_or(false),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
