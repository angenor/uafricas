use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL : Stations Radio ────────────────────────────

pub const ADMIN_STATION_RADIO_LISTE_COLONNES: &str =
    "s.id, s.nom, s.type_station::TEXT as type_station, s.genre, s.etat,
     pays.nom AS pays_nom, s.ville, s.origine_publication, s.created_at";

pub const ADMIN_STATION_RADIO_DETAIL_COLONNES: &str =
    "s.id, s.nom, s.slug, s.description, s.stream_url, s.audio_url, s.image_couverture_url,
     s.genre, s.genres_liste, s.pays_id, pays.nom AS pays_nom, s.ville,
     s.type_station::TEXT as type_station, s.a_la_une,
     s.origine_publication, s.role_partie_prenante, s.role_partie_prenante_autre,
     s.contact_email, s.contact_telephone, s.contact_whatsapp,
     s.contact_site_web, s.contact_adresse,
     s.nombre_signalements,
     s.etat, s.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     s.created_at, s.updated_at";

pub const STATION_RADIO_TRI_COLONNES: &[&str] = &[
    "created_at", "nom", "type_station", "etat",
];

// ── Colonnes SQL : Chaînes TV ────────────────────────────────

pub const ADMIN_CHAINE_TV_LISTE_COLONNES: &str =
    "c.id, c.nom, c.categorie::TEXT as categorie, c.etat, c.est_en_direct,
     pays.nom AS pays_nom, c.langue, c.origine_publication, c.created_at";

pub const ADMIN_CHAINE_TV_DETAIL_COLONNES: &str =
    "c.id, c.nom, c.slug, c.description, c.stream_url, c.image_couverture_url,
     c.categorie::TEXT as categorie, c.pays_id, pays.nom AS pays_nom,
     c.langue, c.est_en_direct, c.origine_publication,
     c.role_partie_prenante, c.role_partie_prenante_autre,
     c.contact_email, c.contact_telephone, c.contact_whatsapp,
     c.contact_site_web, c.contact_adresse,
     c.nombre_signalements,
     c.etat, c.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     c.created_at, c.updated_at";

pub const CHAINE_TV_TRI_COLONNES: &[&str] = &[
    "created_at", "nom", "categorie", "etat",
];

// ── Row & Response : Station Radio ───────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminStationRadioListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub type_station: String,
    pub genre: Option<String>,
    pub etat: String,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    /// « africans » ou « territoire » : départage /medias/radio/africans de /medias/radio/nationales.
    pub origine_publication: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminStationRadioDetailRow {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub audio_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub genre: Option<String>,
    pub genres_liste: Vec<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub type_station: String,
    pub a_la_une: bool,
    /// « africans » ou « territoire » : départage /medias/radio/africans de /medias/radio/nationales.
    pub origine_publication: String,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p).
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
    pub nombre_signalements: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminStationRadioDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub audio_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub genre: Option<String>,
    pub genres_liste: Vec<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub ville: Option<String>,
    pub type_station: String,
    pub a_la_une: bool,
    /// « africans » ou « territoire » : départage /medias/radio/africans de /medias/radio/nationales.
    pub origine_publication: String,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p).
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
    pub nombre_signalements: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminStationRadioDetailRow {
    pub fn to_response(&self) -> AdminStationRadioDetailResponse {
        AdminStationRadioDetailResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            stream_url: self.stream_url.clone(),
            audio_url: self.audio_url.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            genre: self.genre.clone(),
            genres_liste: self.genres_liste.clone(),
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            ville: self.ville.clone(),
            type_station: self.type_station.clone(),
            a_la_une: self.a_la_une,
            origine_publication: self.origine_publication.clone(),
            role_partie_prenante: self.role_partie_prenante.clone(),
            role_partie_prenante_autre: self.role_partie_prenante_autre.clone(),
            contact_email: self.contact_email.clone(),
            contact_telephone: self.contact_telephone.clone(),
            contact_whatsapp: self.contact_whatsapp.clone(),
            contact_site_web: self.contact_site_web.clone(),
            contact_adresse: self.contact_adresse.clone(),
            nombre_signalements: self.nombre_signalements,
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Row & Response : Chaîne TV ───────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminChaineTvListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub categorie: String,
    pub etat: String,
    pub est_en_direct: bool,
    pub pays_nom: Option<String>,
    pub langue: String,
    /// « africans » (Africans Télé International) ou « territoire », cf. 09o.
    pub origine_publication: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminChaineTvDetailRow {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: String,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub langue: String,
    pub est_en_direct: bool,
    pub origine_publication: String,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p).
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
    pub nombre_signalements: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminChaineTvDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: String,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub langue: String,
    pub est_en_direct: bool,
    pub origine_publication: String,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p).
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
    pub nombre_signalements: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminChaineTvDetailRow {
    pub fn to_response(&self) -> AdminChaineTvDetailResponse {
        AdminChaineTvDetailResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            stream_url: self.stream_url.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            categorie: self.categorie.clone(),
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            langue: self.langue.clone(),
            est_en_direct: self.est_en_direct,
            origine_publication: self.origine_publication.clone(),
            role_partie_prenante: self.role_partie_prenante.clone(),
            role_partie_prenante_autre: self.role_partie_prenante_autre.clone(),
            contact_email: self.contact_email.clone(),
            contact_telephone: self.contact_telephone.clone(),
            contact_whatsapp: self.contact_whatsapp.clone(),
            contact_site_web: self.contact_site_web.clone(),
            contact_adresse: self.contact_adresse.clone(),
            nombre_signalements: self.nombre_signalements,
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Requests DTO ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerStationRadioRequest {
    pub nom: String,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub audio_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub genre: Option<String>,
    pub genres_liste: Option<Vec<String>>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub type_station: Option<String>,
    pub a_la_une: Option<bool>,
    /// « africans » ou « territoire » : départage /medias/radio/africans de /medias/radio/nationales.
    pub origine_publication: Option<String>,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p), affichées sur la page du
    /// support une fois celui-ci publié.
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierStationRadioRequest {
    pub etat: Option<String>,
    pub nom: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub audio_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub genre: Option<String>,
    pub genres_liste: Option<Vec<String>>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub type_station: Option<String>,
    pub a_la_une: Option<bool>,
    /// « africans » ou « territoire » : départage /medias/radio/africans de /medias/radio/nationales.
    pub origine_publication: Option<String>,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p), affichées sur la page du
    /// support une fois celui-ci publié.
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreerChaineTvRequest {
    pub nom: String,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: Option<String>,
    pub pays_id: Option<Uuid>,
    pub langue: Option<String>,
    pub est_en_direct: Option<bool>,
    /// « africans » ou « territoire » : alimente le filtre « Africans Télé
    /// International » de /medias/tele. Les deux familles cohabitent sur la
    /// même page, contrairement à la radio (cf. 09o).
    pub origine_publication: Option<String>,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p), affichées sur la page du
    /// support une fois celui-ci publié.
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierChaineTvRequest {
    pub etat: Option<String>,
    pub nom: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: Option<String>,
    pub pays_id: Option<Uuid>,
    pub langue: Option<String>,
    pub est_en_direct: Option<bool>,
    /// « africans » ou « territoire » : cf. `CreerChaineTvRequest`.
    pub origine_publication: Option<String>,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p), affichées sur la page du
    /// support une fois celui-ci publié.
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
}

// ── Programmes conteneurs et épisodes, back-office (009) ────
// Les DTO de lecture sont ceux du domaine (`models::media_emission`,
// `models::media_episode`) : le back-office affiche les mêmes objets que le
// public, avec des filtres différents. Seules les requêtes ÉCRITES diffèrent, 
// l'administration choisit le support et l'état, un co-détenteur non.

#[derive(Debug, Deserialize)]
pub struct AdminEmissionRequest {
    /// « chaine_tv » ou « station_radio » : requis à la création.
    pub type_support: Option<String>,
    pub support_id: Option<Uuid>,
    pub titre: String,
    pub description: Option<String>,
    pub cadence: Option<String>,
    pub image_couverture_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub langue: Option<String>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub categorie_radio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatEmissionRequest {
    pub etat: String,
}

// ── Query Params ─────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AdminStationRadioQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub type_station: Option<String>,
    pub pays_id: Option<Uuid>,
    pub etat: Option<String>,
    /// Filtre la liste admin sur `origine_publication` (« africans » ou « territoire »).
    pub origine: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminChaineTvQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub categorie: Option<String>,
    pub pays_id: Option<Uuid>,
    pub etat: Option<String>,
    /// Filtre la liste admin sur `origine_publication` (« africans » ou « territoire »).
    pub origine: Option<String>,
}

// ── Utilitaires ──────────────────────────────────────────────

pub fn generer_slug(nom: &str) -> String {
    nom.to_lowercase()
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
