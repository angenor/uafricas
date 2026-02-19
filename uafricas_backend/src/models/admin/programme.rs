use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ──────────────────────────────────────────
pub const ADMIN_PROGRAMME_LISTE_COLONNES: &str =
    "p.id, p.titre, p.etat::TEXT as etat, p.duree::TEXT as duree,
     p.date_debut, p.date_fin, p.nombre_places,
     pays.nom AS pays_nom, ds.nom AS domaine_nom,
     u.nom || ' ' || u.prenom AS cree_par_nom,
     p.created_at";

pub const ADMIN_PROGRAMME_DETAIL_COLONNES: &str =
    "p.id, p.titre, p.slug, p.description, p.image_couverture_url, p.document_legal_url,
     p.pays_id, pays.nom AS pays_nom, p.ville, p.adresse,
     p.prise_en_charge_billet, p.prise_en_charge_hebergement,
     p.prise_en_charge_subsistance, p.prise_en_charge_details,
     p.duree::TEXT as duree, p.domaine_id, ds.nom AS domaine_nom,
     p.date_debut, p.date_fin, p.nombre_places,
     p.prerequis, p.langues_requises,
     p.etat::TEXT as etat,
     p.cree_par, u_cree.nom || ' ' || u_cree.prenom AS cree_par_nom,
     p.valide_par, u_val.nom || ' ' || u_val.prenom AS valide_par_nom,
     p.valide_at,
     p.created_at, p.updated_at";

pub const PROGRAMME_TRI_COLONNES: &[&str] = &[
    "created_at", "titre", "etat", "date_debut", "date_fin", "nombre_places",
];

// ── Row (lecture BDD) ─────────────────────────────────────
#[derive(Debug, FromRow)]
pub struct AdminProgrammeDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub document_legal_url: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub prise_en_charge_billet: bool,
    pub prise_en_charge_hebergement: bool,
    pub prise_en_charge_subsistance: bool,
    pub prise_en_charge_details: Option<String>,
    pub duree: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub domaine_nom: Option<String>,
    pub date_debut: Option<chrono::NaiveDate>,
    pub date_fin: Option<chrono::NaiveDate>,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub langues_requises: Option<Vec<String>>,
    pub etat: String,
    pub cree_par: Option<Uuid>,
    pub cree_par_nom: Option<String>,
    pub valide_par: Option<Uuid>,
    pub valide_par_nom: Option<String>,
    pub valide_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ── Responses DTO ─────────────────────────────────────────
#[derive(Debug, Serialize, FromRow)]
pub struct AdminProgrammeListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub etat: String,
    pub duree: Option<String>,
    pub date_debut: Option<chrono::NaiveDate>,
    pub date_fin: Option<chrono::NaiveDate>,
    pub nombre_places: Option<i32>,
    pub pays_nom: Option<String>,
    pub domaine_nom: Option<String>,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminProgrammeDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub document_legal_url: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub prise_en_charge_billet: bool,
    pub prise_en_charge_hebergement: bool,
    pub prise_en_charge_subsistance: bool,
    pub prise_en_charge_details: Option<String>,
    pub duree: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub domaine_nom: Option<String>,
    pub date_debut: Option<chrono::NaiveDate>,
    pub date_fin: Option<chrono::NaiveDate>,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub langues_requises: Option<Vec<String>>,
    pub etat: String,
    pub cree_par: Option<Uuid>,
    pub cree_par_nom: Option<String>,
    pub valide_par: Option<Uuid>,
    pub valide_par_nom: Option<String>,
    pub valide_at: Option<DateTime<Utc>>,
    pub nombre_candidatures: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl AdminProgrammeDetailRow {
    pub fn to_response(&self, nombre_candidatures: i64) -> AdminProgrammeDetailResponse {
        AdminProgrammeDetailResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            document_legal_url: self.document_legal_url.clone(),
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            ville: self.ville.clone(),
            adresse: self.adresse.clone(),
            prise_en_charge_billet: self.prise_en_charge_billet,
            prise_en_charge_hebergement: self.prise_en_charge_hebergement,
            prise_en_charge_subsistance: self.prise_en_charge_subsistance,
            prise_en_charge_details: self.prise_en_charge_details.clone(),
            duree: self.duree.clone(),
            domaine_id: self.domaine_id,
            domaine_nom: self.domaine_nom.clone(),
            date_debut: self.date_debut,
            date_fin: self.date_fin,
            nombre_places: self.nombre_places,
            prerequis: self.prerequis.clone(),
            langues_requises: self.langues_requises.clone(),
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            valide_par: self.valide_par,
            valide_par_nom: self.valide_par_nom.clone(),
            valide_at: self.valide_at,
            nombre_candidatures,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Requests DTO ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct CreerProgrammeRequest {
    pub titre: String,
    pub description: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub prise_en_charge_billet: Option<bool>,
    pub prise_en_charge_hebergement: Option<bool>,
    pub prise_en_charge_subsistance: Option<bool>,
    pub prise_en_charge_details: Option<String>,
    pub duree: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub date_debut: Option<String>,
    pub date_fin: Option<String>,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub langues_requises: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierProgrammeRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub prise_en_charge_billet: Option<bool>,
    pub prise_en_charge_hebergement: Option<bool>,
    pub prise_en_charge_subsistance: Option<bool>,
    pub prise_en_charge_details: Option<String>,
    pub duree: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub date_debut: Option<String>,
    pub date_fin: Option<String>,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub langues_requises: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatProgrammeRequest {
    pub etat: String,
}

// ── Utilitaires ───────────────────────────────────────────
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

// ── Query Params ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct AdminProgrammeQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub domaine_id: Option<Uuid>,
    pub duree: Option<String>,
    pub pays_id: Option<Uuid>,
}
