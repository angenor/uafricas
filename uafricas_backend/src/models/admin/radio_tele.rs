use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL — Stations Radio ────────────────────────────

pub const ADMIN_STATION_RADIO_LISTE_COLONNES: &str =
    "s.id, s.nom, s.type_station::TEXT as type_station, s.genre, s.etat,
     pays.nom AS pays_nom, s.ville, s.origine_publication, s.created_at";

pub const ADMIN_STATION_RADIO_DETAIL_COLONNES: &str =
    "s.id, s.nom, s.slug, s.description, s.stream_url, s.audio_url, s.image_couverture_url,
     s.genre, s.genres_liste, s.pays_id, pays.nom AS pays_nom, s.ville,
     s.type_station::TEXT as type_station, s.a_la_une,
     s.origine_publication, s.role_partie_prenante, s.role_partie_prenante_autre,
     s.nombre_signalements,
     s.etat, s.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     s.created_at, s.updated_at";

pub const STATION_RADIO_TRI_COLONNES: &[&str] = &[
    "created_at", "nom", "type_station", "etat",
];

// ── Colonnes SQL — Chaînes TV ────────────────────────────────

pub const ADMIN_CHAINE_TV_LISTE_COLONNES: &str =
    "c.id, c.nom, c.categorie::TEXT as categorie, c.etat, c.est_en_direct,
     pays.nom AS pays_nom, c.langue, c.created_at";

pub const ADMIN_CHAINE_TV_DETAIL_COLONNES: &str =
    "c.id, c.nom, c.slug, c.description, c.stream_url, c.image_couverture_url,
     c.categorie::TEXT as categorie, c.pays_id, pays.nom AS pays_nom,
     c.langue, c.est_en_direct,
     c.role_partie_prenante, c.role_partie_prenante_autre, c.nombre_signalements,
     c.etat, c.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     c.created_at, c.updated_at";

pub const CHAINE_TV_TRI_COLONNES: &[&str] = &[
    "created_at", "nom", "categorie", "etat",
];

// ── Colonnes SQL — Programmes RADIO (émissions) ──────────────

pub const ADMIN_PROGRAMME_RADIO_LISTE_COLONNES: &str =
    "p.id, p.nom_emission, p.etat,
     p.categorie_radio::TEXT as categorie_radio, p.langue,
     pays.nom AS pays_nom, st.nom AS station_nom, p.a_la_une, p.created_at";

pub const ADMIN_PROGRAMME_RADIO_DETAIL_COLONNES: &str =
    "p.id, p.nom_emission, p.slug,
     p.description, p.image_couverture_url, p.audio_url,
     p.info_animateur, p.info_producteur,
     p.pays_id, pays.nom AS pays_nom, p.est_international, p.langue,
     p.categorie_radio::TEXT as categorie_radio,
     p.station_id, st.nom AS station_nom, p.a_la_une,
     p.theme_phare_id, p.theme_phare_autre, p.nombre_signalements,
     p.etat, p.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     p.created_at, p.updated_at";

pub const PROGRAMME_RADIO_TRI_COLONNES: &[&str] = &[
    "created_at", "nom_emission", "etat",
];

// ── Colonnes SQL — Programmes TÉLÉ ───────────────────────────

pub const ADMIN_PROGRAMME_TELE_LISTE_COLONNES: &str =
    "p.id, p.nom_emission, p.etat, p.langue,
     pays.nom AS pays_nom, ch.nom AS chaine_nom, p.a_la_une, p.a_la_une_globale,
     p.created_at";

pub const ADMIN_PROGRAMME_TELE_DETAIL_COLONNES: &str =
    "p.id, p.nom_emission, p.slug,
     p.description, p.image_couverture_url, p.video_url,
     p.info_animateur, p.info_producteur,
     p.pays_id, pays.nom AS pays_nom, p.est_international, p.langue,
     p.chaine_id, ch.nom AS chaine_nom, p.a_la_une, p.a_la_une_globale,
     p.theme_phare_id, p.theme_phare_autre, p.nombre_signalements,
     p.etat, p.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     p.created_at, p.updated_at";

pub const PROGRAMME_TELE_TRI_COLONNES: &[&str] = &[
    "created_at", "nom_emission", "etat",
];

// ── Row & Response — Station Radio ───────────────────────────

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
            nombre_signalements: self.nombre_signalements,
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Row & Response — Chaîne TV ───────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminChaineTvListeResponse {
    pub id: Uuid,
    pub nom: String,
    pub categorie: String,
    pub etat: String,
    pub est_en_direct: bool,
    pub pays_nom: Option<String>,
    pub langue: String,
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
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
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
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
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
            role_partie_prenante: self.role_partie_prenante.clone(),
            role_partie_prenante_autre: self.role_partie_prenante_autre.clone(),
            nombre_signalements: self.nombre_signalements,
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Row & Response — Programme RADIO ─────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminProgrammeRadioListeResponse {
    pub id: Uuid,
    pub nom_emission: String,
    pub etat: String,
    pub categorie_radio: Option<String>,
    pub langue: String,
    pub pays_nom: Option<String>,
    pub station_nom: Option<String>,
    pub a_la_une: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminProgrammeRadioDetailRow {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub audio_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub est_international: bool,
    pub langue: String,
    pub categorie_radio: Option<String>,
    pub station_id: Option<Uuid>,
    pub station_nom: Option<String>,
    pub a_la_une: bool,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub nombre_signalements: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminProgrammeRadioDetailResponse {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub audio_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub est_international: bool,
    pub langue: String,
    pub categorie_radio: Option<String>,
    pub station_id: Option<Uuid>,
    pub station_nom: Option<String>,
    pub a_la_une: bool,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub nombre_signalements: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminProgrammeRadioDetailRow {
    pub fn to_response(&self) -> AdminProgrammeRadioDetailResponse {
        AdminProgrammeRadioDetailResponse {
            id: self.id,
            nom_emission: self.nom_emission.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            audio_url: self.audio_url.clone(),
            info_animateur: self.info_animateur.clone(),
            info_producteur: self.info_producteur.clone(),
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            est_international: self.est_international,
            langue: self.langue.clone(),
            categorie_radio: self.categorie_radio.clone(),
            station_id: self.station_id,
            station_nom: self.station_nom.clone(),
            a_la_une: self.a_la_une,
            theme_phare_id: self.theme_phare_id,
            theme_phare_autre: self.theme_phare_autre.clone(),
            nombre_signalements: self.nombre_signalements,
            etat: self.etat.clone(),
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

// ── Row & Response — Programme TÉLÉ ──────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminProgrammeTeleListeResponse {
    pub id: Uuid,
    pub nom_emission: String,
    pub etat: String,
    pub langue: String,
    pub pays_nom: Option<String>,
    pub chaine_nom: Option<String>,
    pub a_la_une: bool,
    /// Vedette unique de TOUTE la page Télé, à distinguer de `a_la_une` qui vaut par chaîne.
    pub a_la_une_globale: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminProgrammeTeleDetailRow {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub video_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub est_international: bool,
    pub langue: String,
    pub chaine_id: Option<Uuid>,
    pub chaine_nom: Option<String>,
    pub a_la_une: bool,
    /// Vedette unique de TOUTE la page Télé, à distinguer de `a_la_une` qui vaut par chaîne.
    pub a_la_une_globale: bool,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub nombre_signalements: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminProgrammeTeleDetailResponse {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub video_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub pays_nom: Option<String>,
    pub est_international: bool,
    pub langue: String,
    pub chaine_id: Option<Uuid>,
    pub chaine_nom: Option<String>,
    pub a_la_une: bool,
    /// Vedette unique de TOUTE la page Télé, à distinguer de `a_la_une` qui vaut par chaîne.
    pub a_la_une_globale: bool,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub nombre_signalements: i32,
    pub etat: String,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminProgrammeTeleDetailRow {
    pub fn to_response(&self) -> AdminProgrammeTeleDetailResponse {
        AdminProgrammeTeleDetailResponse {
            id: self.id,
            nom_emission: self.nom_emission.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            video_url: self.video_url.clone(),
            info_animateur: self.info_animateur.clone(),
            info_producteur: self.info_producteur.clone(),
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            est_international: self.est_international,
            langue: self.langue.clone(),
            chaine_id: self.chaine_id,
            chaine_nom: self.chaine_nom.clone(),
            a_la_une: self.a_la_une,
            a_la_une_globale: self.a_la_une_globale,
            theme_phare_id: self.theme_phare_id,
            theme_phare_autre: self.theme_phare_autre.clone(),
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
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
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
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
}

// Programme RADIO
#[derive(Debug, Deserialize)]
pub struct CreerProgrammeRadioRequest {
    pub nom_emission: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub audio_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub est_international: Option<bool>,
    pub langue: Option<String>,
    pub categorie_radio: Option<String>,
    pub station_id: Option<Uuid>,
    pub a_la_une: Option<bool>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierProgrammeRadioRequest {
    pub etat: Option<String>,
    pub nom_emission: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub audio_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub est_international: Option<bool>,
    pub langue: Option<String>,
    pub categorie_radio: Option<String>,
    pub station_id: Option<Uuid>,
    pub a_la_une: Option<bool>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
}

// Programme TÉLÉ
#[derive(Debug, Deserialize)]
pub struct CreerProgrammeTeleRequest {
    pub nom_emission: String,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub video_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub est_international: Option<bool>,
    pub langue: Option<String>,
    pub chaine_id: Option<Uuid>,
    pub a_la_une: Option<bool>,
    /// Vedette unique de TOUTE la page Télé, à distinguer de `a_la_une` qui vaut par chaîne.
    pub a_la_une_globale: Option<bool>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierProgrammeTeleRequest {
    pub etat: Option<String>,
    pub nom_emission: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub video_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub est_international: Option<bool>,
    pub langue: Option<String>,
    pub chaine_id: Option<Uuid>,
    pub a_la_une: Option<bool>,
    /// Vedette unique de TOUTE la page Télé, à distinguer de `a_la_une` qui vaut par chaîne.
    pub a_la_une_globale: Option<bool>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
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
}

#[derive(Debug, Deserialize)]
pub struct AdminProgrammeRadioQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub categorie_radio: Option<String>,
    pub station_id: Option<Uuid>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminProgrammeTeleQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub chaine_id: Option<Uuid>,
    pub etat: Option<String>,
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
