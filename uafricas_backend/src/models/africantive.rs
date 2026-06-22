use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL pour le listing (avec jointures aplaties) ──────────

/// Colonnes SELECT pour le listing des africantives
/// Alias `a.` pour la table africantive
/// Les enums PostgreSQL sont castes en ::text pour compatibilite sqlx
pub const AFRICANTIVE_LISTE_COLONNES: &str =
    "a.id, a.titre, a.slug, a.description,
     a.image_couverture_url, a.ville,
     a.etat::text AS etat, a.cree_par,
     a.created_at, a.updated_at,
     d.nom AS domaine_nom, d.slug AS domaine_slug,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email";

// ── Row aplati pour le listing ──────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct AfricantiveListeRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub ville: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs joints
    pub domaine_nom: Option<String>,
    pub domaine_slug: Option<String>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
}

// ── Row pour le detail ──────────────────────────────────────────────

pub const AFRICANTIVE_DETAIL_COLONNES: &str =
    "a.id, a.titre, a.slug, a.description,
     a.image_couverture_url, a.ville,
     a.site_web_url, a.lien_reseau_social, a.domaine_autre,
     a.contact1_courriel, a.contact1_telephone, a.contact1_adresse,
     a.contact2_courriel, a.contact2_telephone, a.contact2_adresse,
     a.etat::text AS etat, a.cree_par,
     a.created_at, a.updated_at,
     d.nom AS domaine_nom, d.slug AS domaine_slug,
     p.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email,
     u.photo_url AS auteur_photo_url";

#[derive(Debug, FromRow)]
pub struct AfricantiveDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub ville: Option<String>,
    pub site_web_url: Option<String>,
    pub lien_reseau_social: Option<String>,
    pub domaine_autre: Option<String>,
    pub contact1_courriel: Option<String>,
    pub contact1_telephone: Option<String>,
    pub contact1_adresse: Option<String>,
    pub contact2_courriel: Option<String>,
    pub contact2_telephone: Option<String>,
    pub contact2_adresse: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs joints
    pub domaine_nom: Option<String>,
    pub domaine_slug: Option<String>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub auteur_photo_url: Option<String>,
}

// ── DTOs de reponse ─────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AfricantiveAuteurResponse {
    pub uid: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct AfricantiveAuteurDetailResponse {
    pub uid: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub photo_url: Option<String>,
}

/// DTO pour une africantive dans la liste
#[derive(Debug, Serialize)]
pub struct AfricantiveResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub domaine: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub user: AfricantiveAuteurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le detail d'une africantive
#[derive(Debug, Serialize)]
pub struct AfricantiveDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub domaine: Option<String>,
    pub domaine_slug: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub site_web_url: Option<String>,
    pub lien_reseau_social: Option<String>,
    pub contact1_courriel: Option<String>,
    pub contact1_telephone: Option<String>,
    pub contact1_adresse: Option<String>,
    pub contact2_courriel: Option<String>,
    pub contact2_telephone: Option<String>,
    pub contact2_adresse: Option<String>,
    pub user: AfricantiveAuteurDetailResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Reponse paginee pour le listing
#[derive(Debug, Serialize)]
pub struct AfricantiveListeResponse {
    pub africantives: Vec<AfricantiveResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Parametres de requete ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AfricantiveQueryParams {
    pub recherche: Option<String>,
    pub domaine: Option<String>,
    pub pays: Option<String>,
    pub tri: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ── Conversions ─────────────────────────────────────────────────────

pub fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace('é', "e")
        .replace('è', "e")
        .replace('ê', "e")
        .replace('ë', "e")
        .replace('à', "a")
        .replace('â', "a")
        .replace('ä', "a")
        .replace('ù', "u")
        .replace('û', "u")
        .replace('ü', "u")
        .replace('ô', "o")
        .replace('ö', "o")
        .replace('î', "i")
        .replace('ï', "i")
        .replace('ç', "c")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

impl AfricantiveListeRow {
    pub fn to_response(&self) -> AfricantiveResponse {
        AfricantiveResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            domaine: self.domaine_nom.clone(),
            pays: self.pays_nom.clone(),
            ville: self.ville.clone(),
            user: AfricantiveAuteurResponse {
                uid: self.cree_par,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                email: self.auteur_email.clone(),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl AfricantiveDetailRow {
    pub fn to_detail_response(&self) -> AfricantiveDetailResponse {
        AfricantiveDetailResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            // Domaine résolu, sinon précision « Autre » saisie librement
            domaine: self.domaine_nom.clone().or_else(|| self.domaine_autre.clone()),
            domaine_slug: self.domaine_slug.clone(),
            pays: self.pays_nom.clone(),
            ville: self.ville.clone(),
            site_web_url: self.site_web_url.clone(),
            lien_reseau_social: self.lien_reseau_social.clone(),
            contact1_courriel: self.contact1_courriel.clone(),
            contact1_telephone: self.contact1_telephone.clone(),
            contact1_adresse: self.contact1_adresse.clone(),
            contact2_courriel: self.contact2_courriel.clone(),
            contact2_telephone: self.contact2_telephone.clone(),
            contact2_adresse: self.contact2_adresse.clone(),
            user: AfricantiveAuteurDetailResponse {
                uid: self.cree_par,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                email: self.auteur_email.clone(),
                photo_url: self.auteur_photo_url.clone(),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
