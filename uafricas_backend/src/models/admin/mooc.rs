use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ─────────────────────────────────────────────

pub const ADMIN_MOOC_LISTE_COLONNES: &str =
    "m.id, m.titre, m.format::TEXT as format, m.etat,
     m.date_heure_debut, m.date_heure_fin, m.nombre_places,
     pays.nom AS pays_nom,
     u.nom || ' ' || u.prenom AS cree_par_nom,
     m.created_at";

pub const ADMIN_MOOC_DETAIL_COLONNES: &str =
    "m.id, m.titre, m.slug, m.description, m.type as type_formation,
     m.pays_id, pays.nom AS pays_nom, m.ville,
     m.date_heure_debut, m.date_heure_fin,
     m.image_couverture_url, m.format::TEXT as format,
     m.lien_en_ligne, m.langue, m.nombre_places, m.prerequis,
     m.objectif, m.presentation, m.a_evaluation, m.est_certifiante,
     m.etat, m.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     m.created_at, m.updated_at";

pub const MOOC_TRI_COLONNES: &[&str] = &[
    "created_at", "titre", "etat", "date_heure_debut", "format", "nombre_places",
];

// ── Row & Response ───────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminMoocListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub format: String,
    pub etat: String,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub nombre_places: Option<i32>,
    pub pays_nom: Option<String>,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminMoocDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_formation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub image_couverture_url: Option<String>,
    pub format: String,
    pub lien_en_ligne: Option<String>,
    pub langue: String,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub objectif: Option<String>,
    pub presentation: Option<String>,
    pub a_evaluation: bool,
    pub est_certifiante: bool,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminMoocDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_formation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub image_couverture_url: Option<String>,
    pub format: String,
    pub lien_en_ligne: Option<String>,
    pub langue: String,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub objectif: Option<String>,
    pub presentation: Option<String>,
    pub a_evaluation: bool,
    pub est_certifiante: bool,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub nombre_inscriptions: i64,
    pub progression_moyenne: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminMoocDetailRow {
    pub fn to_response(&self, nombre_inscriptions: i64, progression_moyenne: f64) -> AdminMoocDetailResponse {
        AdminMoocDetailResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            type_formation: self.type_formation.clone(),
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            ville: self.ville.clone(),
            date_heure_debut: self.date_heure_debut,
            date_heure_fin: self.date_heure_fin,
            image_couverture_url: self.image_couverture_url.clone(),
            format: self.format.clone(),
            lien_en_ligne: self.lien_en_ligne.clone(),
            langue: self.langue.clone(),
            nombre_places: self.nombre_places,
            prerequis: self.prerequis.clone(),
            objectif: self.objectif.clone(),
            presentation: self.presentation.clone(),
            a_evaluation: self.a_evaluation,
            est_certifiante: self.est_certifiante,
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            nombre_inscriptions,
            progression_moyenne,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Inscriptions ─────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminMoocInscriptionResponse {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub statut: String,
    pub progression: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminMoocInscriptionStats {
    pub total: i64,
    pub inscrits: i64,
    pub en_cours: i64,
    pub completes: i64,
    pub abandonnes: i64,
    pub progression_moyenne: f64,
}

// ── Requests DTO ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerMoocRequest {
    pub titre: String,
    pub description: String,
    pub type_formation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub date_heure_debut: String,
    pub date_heure_fin: Option<String>,
    pub image_couverture_url: Option<String>,
    pub format: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub langue: Option<String>,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub objectif: Option<String>,
    pub presentation: Option<String>,
    pub a_evaluation: Option<bool>,
    pub est_certifiante: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierMoocRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub type_formation: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub date_heure_debut: Option<String>,
    pub date_heure_fin: Option<String>,
    pub image_couverture_url: Option<String>,
    pub format: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub langue: Option<String>,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub objectif: Option<String>,
    pub presentation: Option<String>,
    pub a_evaluation: Option<bool>,
    pub est_certifiante: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatMoocRequest {
    pub etat: String,
}

// ── Query Params ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminMoocQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub format: Option<String>,
    pub type_formation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminMoocInscriptionQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub statut: Option<String>,
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
