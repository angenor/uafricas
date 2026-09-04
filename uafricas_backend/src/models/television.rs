use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::media_emission::EmissionResponse;
use crate::models::media_episode::EpisodeResponse;
use crate::models::media_social::CompteursInteraction;
use crate::models::media_support::{CouverturePublique, ThematiquePublique};
use crate::services::contacts_media::ContactsSupport;

// ═══════════════════════════════════════════════════════════════════════════
// PARTIE 1 : Chaînes TV (table media_content.chaine_tv)
// ═══════════════════════════════════════════════════════════════════════════

// ── Colonnes SQL ──────────────────────────────────────────────────────

pub const CHAINE_TV_COLONNES: &str =
    "ct.id, ct.nom, ct.slug, ct.description, ct.stream_url, ct.image_couverture_url,
     ct.categorie::text AS categorie, ct.langue, ct.est_en_direct,
     ct.etat, ct.origine_publication, ct.est_thematique,
     ct.role_partie_prenante, ct.role_partie_prenante_autre,
     ct.contact_email, ct.contact_telephone, ct.contact_whatsapp,
     ct.contact_site_web, ct.contact_adresse,
     ct.nombre_signalements, ct.cree_par, ct.created_at, ct.updated_at";

// ── Structs DB ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ChaineTvRow {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: String,
    pub langue: String,
    pub est_en_direct: bool,
    pub etat: String,
    /// « africans » (Africans Télé International) ou « territoire », cf. 09o.
    pub origine_publication: String,
    /// Chaîne thématique (09v) : une seule thématique de grille, et tous les
    /// territoires d'office. Exclut toute couverture territoriale.
    pub est_thematique: bool,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p), toutes facultatives.
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub contact_whatsapp: Option<String>,
    pub contact_site_web: Option<String>,
    pub contact_adresse: Option<String>,
    pub nombre_signalements: i32,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Response DTOs ─────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ChaineTvResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: String,
    pub langue: String,
    pub est_en_direct: bool,
    /// Distingue les chaînes de la plateforme (« africans ») de celles d'un
    /// territoire : c'est le filtre « Africans Télé International ».
    pub origine_publication: String,
    /// Chaîne thématique (09v). Le client s'en sert pour n'afficher NI le champ
    /// « territoires » ni une seconde thématique : les deux sont sans objet.
    pub est_thematique: bool,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
    /// Coordonnées publiques de l'équipe (09p). Absent du JSON quand la chaîne
    /// n'en publie aucune : le bloc « Contacts » disparaît alors de sa page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsSupport>,
    /// Thématiques déclarées (US3, table `support_thematique`). Vide tant que
    /// l'appelant ne les a pas greffées : servir une carte n'oblige pas à les
    /// charger.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thematiques: Vec<ThematiquePublique>,
    /// Couverture territoriale déclarée (US4). `None` tant qu'elle n'est pas
    /// greffée.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub couverture: Option<CouverturePublique>,
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
pub struct ChaineTvListeResponse {
    pub chaines: Vec<ChaineTvResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

// ── Query Params ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ChaineTvQueryParams {
    pub recherche: Option<String>,
    pub categorie: Option<String>,
    /// Territoire couvert (`support_territoire`), et non plus le pays unique de
    /// la chaîne : celui-ci a disparu avec 09v, une chaîne pouvant en couvrir
    /// plusieurs. Les chaînes continentales remontent sur chaque territoire.
    pub territoire: Option<Uuid>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ── Formulaire de création ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerChaineTvForm {
    pub nom: String,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub categorie: Option<String>,
    pub langue: Option<String>,
}

// ── Mapping ───────────────────────────────────────────────────────────

pub fn mapper_categorie_chaine_frontend(db_val: &str) -> String {
    match db_val {
        "generaliste" => "Généraliste".to_string(),
        "info" => "Info".to_string(),
        "sport" => "Sport".to_string(),
        "culture" => "Culture".to_string(),
        "divertissement" => "Divertissement".to_string(),
        "religieux" => "Religieux".to_string(),
        "education" => "Éducation".to_string(),
        "musique" => "Musique".to_string(),
        autre => autre.to_string(),
    }
}

pub fn mapper_categorie_chaine_db(frontend_val: &str) -> String {
    match frontend_val {
        "Généraliste" | "Generaliste" | "generaliste" => "generaliste".to_string(),
        "Info" | "info" | "Informations" => "info".to_string(),
        "Sport" | "sport" => "sport".to_string(),
        "Culture" | "culture" => "culture".to_string(),
        "Divertissement" | "divertissement" => "divertissement".to_string(),
        "Religieux" | "religieux" => "religieux".to_string(),
        "Éducation" | "Education" | "education" => "education".to_string(),
        "Musique" | "musique" => "musique".to_string(),
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

impl ChaineTvRow {
    pub fn to_response(&self) -> ChaineTvResponse {
        ChaineTvResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            stream_url: self.stream_url.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            categorie: mapper_categorie_chaine_frontend(&self.categorie),
            langue: self.langue.clone(),
            est_en_direct: self.est_en_direct,
            origine_publication: self.origine_publication.clone(),
            est_thematique: self.est_thematique,
            role_partie_prenante: self.role_partie_prenante.clone(),
            role_partie_prenante_autre: self.role_partie_prenante_autre.clone(),
            contacts: ContactsSupport::depuis(
                self.contact_email.as_deref(),
                self.contact_telephone.as_deref(),
                self.contact_whatsapp.as_deref(),
                self.contact_site_web.as_deref(),
                self.contact_adresse.as_deref(),
            ),
            thematiques: Vec::new(),
            couverture: None,
            // Greffée après coup par les appelants, comme les thématiques : servir
            // une carte n'oblige pas à charger l'équipe.
            equipe: Vec::new(),
            created_at: self.created_at,
            interactions: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PARTIE 2 : Programmes et épisodes télé (tables emission_tele / episode_tele)
// ═══════════════════════════════════════════════════════════════════════════
// `programme_tele` a disparu avec 09q : le conteneur est l'émission
// (`models::media_emission`), l'unité diffusable l'épisode
// (`models::media_episode`). Ce module ne garde que ce qui relève de la PAGE
// Télé : sections, vedette, référentiels de filtre.


// ── Sections de la page Télé (US1) ────────────────────────────────────
// Une section = une chaîne et ses PROGRAMMES publiés, et non plus une vignette
// par vidéo. Chaque programme annonce son nombre d'épisodes et un aperçu borné ;
// au-delà, la page du programme prend le relais (SC-009).

#[derive(Debug, Serialize)]
pub struct TeleSectionResponse {
    pub chaine: ChaineTvResponse,
    /// Programmes publiés de la chaîne comptant au moins un épisode publié
    /// (FR-011). Un programme vide reste visible du détenteur et de
    /// l'administration, jamais du public.
    pub emissions: Vec<EmissionResponse>,
    pub total_emissions: i64,
    /// Ce que la grille programme à l'instant de la requête, et ce qui suit
    /// (US2). `None` si la chaîne n'a aucune grille active, ou si l'émission
    /// programmée n'a aucun épisode publié (FR-021).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diffusion_en_cours: Option<crate::models::media_programmation::CreneauResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creneau_suivant: Option<crate::models::media_programmation::CreneauResponse>,
}

#[derive(Debug, Serialize)]
pub struct TeleSectionsListeResponse {
    pub sections: Vec<TeleSectionResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize)]
pub struct TeleSectionsQueryParams {
    pub recherche: Option<String>,
    pub categorie: Option<String>,
    /// « africans » ou « territoire » (09o), filtre « Africans Télé International ».
    pub origine: Option<String>,
    /// Identifiant d'un thème phare (`shared.categorie`, contexte « media ») :
    /// remonte les chaînes qui diffusent au moins un programme sur ce thème.
    pub theme: Option<Uuid>,
    /// Thématiques DÉCLARÉES par le support (US3), entendues comme un OU :
    /// `?thematique=<uuid>,<uuid>`.
    ///
    /// **Liste séparée par des virgules, et non clé répétée** : `web::Query`
    /// s'appuie sur `serde_urlencoded`, qui ne sait pas agréger plusieurs
    /// occurrences d'une même clé dans un `Vec` : il échoue en 400 dès la
    /// PREMIÈRE valeur (« invalid type: string, expected a sequence »), même
    /// seule. Un `Vec<Uuid>` ici rendrait donc le filtre inutilisable ; le
    /// parser en une passe est ce qui évite d'ajouter `serde_qs` pour un champ.
    ///
    /// À ne pas confondre avec `theme`, qui porte sur le thème phare des
    /// programmes diffusés.
    pub thematique: Option<String>,
    /// Territoire couvert (US4) : remonte aussi les supports continentaux
    /// (FR-036).
    pub territoire: Option<Uuid>,
    /// `true` restreint aux chaînes actuellement en direct.
    pub en_direct: Option<bool>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    /// Programmes servis par section. Borné côté handler.
    pub contenus_par_section: Option<i64>,
    /// Épisodes d'aperçu par programme. Borné à 12 côté handler.
    pub episodes_par_emission: Option<i64>,
}

// ── Vedette de la page Télé ───────────────────────────────────────────
// La vedette désigne une UNITÉ DIFFUSABLE (FR-052) : c'est l'épisode qui porte
// `a_la_une_globale`, accompagné de son programme et de sa chaîne.

#[derive(Debug, Serialize)]
pub struct VedetteTeleResponse {
    pub episode: EpisodeResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission: Option<EmissionResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chaine: Option<ChaineTvResponse>,
    /// `true` quand aucune vedette n'est désignée et que l'épisode servi est le
    /// plus récent publié : l'interface le signale plutôt que de faire croire à
    /// un choix éditorial.
    pub est_repli: bool,
}

// ── Stats Télévision ──────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct TelevisionStats {
    pub nombre_chaines: i64,
    /// Territoires distincts couverts par au moins une chaîne publiée.
    pub nombre_pays: i64,
    pub nombre_programmes: i64,
    pub nombre_chaines_en_direct: i64,
}
