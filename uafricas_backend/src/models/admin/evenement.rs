use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ─────────────────────────────────────────────

pub const ADMIN_EVENEMENT_LISTE_COLONNES: &str =
    "e.id, e.titre, e.format::TEXT as format, e.etat,
     e.date_heure_debut, e.date_heure_fin, e.nombre_places,
     pays.nom AS pays_nom, e.ville,
     u.nom || ' ' || u.prenom AS cree_par_nom,
     e.created_at";

pub const ADMIN_EVENEMENT_DETAIL_COLONNES: &str =
    "e.id, e.titre, e.slug, e.description, e.type as type_evenement,
     e.pays_id, pays.nom AS pays_nom, e.ville, e.adresse,
     e.date_heure_debut, e.date_heure_fin,
     e.image_couverture_url, e.format::TEXT as format,
     e.lien_en_ligne, e.langue, e.nombre_places,
     e.type_organisateur::TEXT AS type_organisateur,
     e.contact_nom, e.contact_email, e.contact_telephone, e.contact_site_web,
     e.etat, e.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     e.created_at, e.updated_at";

pub const EVENEMENT_TRI_COLONNES: &[&str] = &[
    "created_at", "titre", "etat", "date_heure_debut", "format", "nombre_places",
];

// ── Row (lecture BDD) ────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminEvenementListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub format: String,
    pub etat: String,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub nombre_places: Option<i32>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminEvenementDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_evenement: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub image_couverture_url: Option<String>,
    pub format: String,
    pub lien_en_ligne: Option<String>,
    pub langue: String,
    pub nombre_places: Option<i32>,
    pub type_organisateur: String,
    pub contact_nom: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_site_web: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminEvenementDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub type_evenement: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub image_couverture_url: Option<String>,
    pub format: String,
    pub lien_en_ligne: Option<String>,
    pub langue: String,
    pub nombre_places: Option<i32>,
    pub type_organisateur: String,
    pub contact_nom: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_site_web: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub nombre_inscriptions: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminEvenementDetailRow {
    pub fn to_response(&self, nombre_inscriptions: i64) -> AdminEvenementDetailResponse {
        AdminEvenementDetailResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            type_evenement: self.type_evenement.clone(),
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            ville: self.ville.clone(),
            adresse: self.adresse.clone(),
            date_heure_debut: self.date_heure_debut,
            date_heure_fin: self.date_heure_fin,
            image_couverture_url: self.image_couverture_url.clone(),
            format: self.format.clone(),
            lien_en_ligne: self.lien_en_ligne.clone(),
            langue: self.langue.clone(),
            nombre_places: self.nombre_places,
            type_organisateur: self.type_organisateur.clone(),
            contact_nom: self.contact_nom.clone(),
            contact_email: self.contact_email.clone(),
            contact_telephone: self.contact_telephone.clone(),
            contact_site_web: self.contact_site_web.clone(),
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            nombre_inscriptions,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Inscriptions ─────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminEvenementInscriptionResponse {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub statut: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminEvenementInscriptionStats {
    pub total: i64,
    pub inscrits: i64,
    pub confirmes: i64,
    pub annules: i64,
    pub presents: i64,
    pub absents: i64,
}

// ── Requests DTO ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerEvenementRequest {
    pub titre: String,
    pub description: String,
    pub type_evenement: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub date_heure_debut: String,
    pub date_heure_fin: Option<String>,
    pub image_couverture_url: Option<String>,
    pub format: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub langue: Option<String>,
    pub nombre_places: Option<i32>,
    pub type_organisateur: Option<String>,
    pub contact_nom: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_site_web: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierEvenementRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub type_evenement: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub date_heure_debut: Option<String>,
    pub date_heure_fin: Option<String>,
    pub image_couverture_url: Option<String>,
    pub format: Option<String>,
    pub lien_en_ligne: Option<String>,
    pub langue: Option<String>,
    pub nombre_places: Option<i32>,
    pub type_organisateur: Option<String>,
    pub contact_nom: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_site_web: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatEvenementRequest {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangerStatutInscriptionRequest {
    pub statut: String,
}

// ── Query Params ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminEvenementQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub format: Option<String>,
    pub etat: Option<String>,
    pub pays_id: Option<Uuid>,
    pub date_debut: Option<String>,
    pub date_fin: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminInscriptionQueryParams {
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
