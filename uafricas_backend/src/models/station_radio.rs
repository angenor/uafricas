use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::media_social::CompteursInteraction;
use crate::services::contacts_media::ContactsSupport;

// ── Colonnes SQL ──────────────────────────────────────────────────────

pub const STATION_RADIO_COLONNES: &str =
    "sr.id, sr.nom, sr.slug, sr.description, sr.stream_url, sr.audio_url, sr.image_couverture_url,
     sr.genre, sr.genres_liste, sr.pays_id, sr.ville,
     sr.type_station::text AS type_station, sr.a_la_une, sr.etat,
     sr.origine_publication, sr.role_partie_prenante, sr.role_partie_prenante_autre,
     sr.contact_email, sr.contact_telephone, sr.contact_whatsapp,
     sr.contact_site_web, sr.contact_adresse,
     sr.nombre_signalements,
     sr.cree_par, sr.created_at, sr.updated_at";

// ── Structs DB ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct StationRadioRow {
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
    pub ville: Option<String>,
    pub type_station: String,
    pub a_la_une: bool,
    pub etat: String,
    /// 'africans' → /medias/radio/africans · 'territoire' → /medias/radio/nationales.
    /// Porté par la page, jamais par un filtre utilisateur (FR-014).
    pub origine_publication: String,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p) — toutes facultatives.
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
    pub nombre_signalements: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs jointures optionnels
    #[sqlx(default)]
    pub pays_nom: Option<String>,
}

// ── Response DTOs ─────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct StationRadioResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub audio_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub genre: Option<String>,
    pub genres_liste: Vec<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub type_station: String,
    pub a_la_une: bool,
    pub origine_publication: String,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p). Absent du JSON quand la station
    /// n'en publie aucune — le bloc « Contacts » disparaît alors de sa page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsSupport>,
    /// Thématiques déclarées (US3, table `support_thematique`). Vide tant que
    /// l'appelant ne les a pas greffées.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thematiques: Vec<crate::models::media_support::ThematiquePublique>,
    /// Couverture territoriale déclarée (US4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub couverture: Option<crate::models::media_support::CouverturePublique>,
    /// Équipe éditoriale déclarée (010, table `membre_equipe`). Omise du JSON
    /// quand elle est vide : le bloc « équipe » disparaît alors des pages, sans
    /// cadre ni libellé creux (FR-007).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub equipe: Vec<crate::models::media_equipe::MembreEquipeResponse>,
    pub created_at: DateTime<Utc>,
    /// Réactions, commentaires et partages agrégés (FR-027). `None` tant que
    /// l'appelant ne les a pas greffés.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<CompteursInteraction>,
}

#[derive(Debug, Serialize)]
pub struct StationRadioListeResponse {
    pub stations: Vec<StationRadioResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Query Params ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct StationRadioQueryParams {
    pub recherche: Option<String>,
    pub type_station: Option<String>,
    pub pays: Option<String>,
    pub genre: Option<String>,
    /// 'africans' | 'territoire' — porté par la page appelante (FR-014).
    /// Une valeur hors whitelist est rejetée en 400 par le handler.
    pub origine: Option<String>,
    /// Thématiques DÉCLARÉES par le support (US3), entendues comme un OU :
    /// `?thematique=<uuid>,<uuid>`.
    ///
    /// **Liste séparée par des virgules, et non clé répétée** : `web::Query`
    /// s'appuie sur `serde_urlencoded`, qui ne sait pas agréger plusieurs
    /// occurrences d'une même clé dans un `Vec` — il échoue en 400 dès la
    /// PREMIÈRE valeur (« invalid type: string, expected a sequence »), même
    /// seule. Un `Vec<Uuid>` ici rendrait donc le filtre inutilisable ; le
    /// parser en une passe est ce qui évite d'ajouter `serde_qs` pour un champ.
    pub thematique: Option<String>,
    /// Territoire couvert (US4) — remonte aussi les stations continentales
    /// (FR-036).
    pub territoire: Option<Uuid>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    /// Pagination des sections (`/sections`) : nombre de programmes par rangée.
    pub contenus_par_section: Option<i64>,
}

/// Whitelist des origines de publication. Jamais d'interpolation de l'entrée
/// brute dans le SQL : la valeur est validée puis liée en paramètre.
pub const ORIGINES_PUBLICATION: [&str; 2] = ["africans", "territoire"];

pub fn origine_valide(valeur: &str) -> bool {
    ORIGINES_PUBLICATION.contains(&valeur)
}

// ── Formulaire de création ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerStationRadioForm {
    pub nom: String,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub audio_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub genre: Option<String>,
    pub genres_liste: Option<Vec<String>>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub type_station: Option<String>,
    pub a_la_une: Option<bool>,
    pub origine_publication: Option<String>,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
}

// ── Mapping ───────────────────────────────────────────────────────────

pub fn mapper_type_station_frontend(db_val: &str) -> String {
    match db_val {
        "nationale" => "Nationales".to_string(),
        "locale" => "Local".to_string(),
        "internationale" => "International".to_string(),
        autre => autre.to_string(),
    }
}

pub fn mapper_type_station_db(frontend_val: &str) -> String {
    match frontend_val {
        "Nationales" | "nationales" | "nationale" => "nationale".to_string(),
        "Local" | "local" | "locale" => "locale".to_string(),
        "International" | "international" | "internationale" => "internationale".to_string(),
        autre => autre.to_lowercase(),
    }
}

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

// ── Conversions ───────────────────────────────────────────────────────

impl StationRadioRow {
    pub fn to_response(&self) -> StationRadioResponse {
        StationRadioResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            stream_url: self.stream_url.clone(),
            audio_url: self.audio_url.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            genre: self.genre.clone(),
            genres_liste: self.genres_liste.clone(),
            pays: self.pays_nom.clone(),
            ville: self.ville.clone(),
            type_station: mapper_type_station_frontend(&self.type_station),
            a_la_une: self.a_la_une,
            origine_publication: self.origine_publication.clone(),
            role_partie_prenante: self.role_partie_prenante.clone(),
            role_partie_prenante_autre: self.role_partie_prenante_autre.clone(),
            contacts: ContactsSupport::depuis(
                self.contact_email.as_deref(),
                self.contact_telephone.as_deref(),
                self.contact_whatsapp.as_deref(),
                self.contact_site_web.as_deref(),
                self.contact_adresse.as_deref(),
            ),
            created_at: self.created_at,
            thematiques: Vec::new(),
            couverture: None,
            // Greffée après coup par les appelants, comme les thématiques : servir
            // une carte n'oblige pas à charger l'équipe.
            equipe: Vec::new(),
            interactions: None,
        }
    }
}
