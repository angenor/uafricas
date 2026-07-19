use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::media_social::CompteursInteraction;

// ═══════════════════════════════════════════════════════════════════════════
// PARTIE 1 : Chaînes TV (table media_content.chaine_tv)
// ═══════════════════════════════════════════════════════════════════════════

// ── Colonnes SQL ──────────────────────────────────────────────────────

pub const CHAINE_TV_COLONNES: &str =
    "ct.id, ct.nom, ct.slug, ct.description, ct.stream_url, ct.image_couverture_url,
     ct.categorie::text AS categorie, ct.pays_id, ct.langue, ct.est_en_direct,
     ct.etat, ct.role_partie_prenante, ct.role_partie_prenante_autre,
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
    pub pays_id: Option<Uuid>,
    pub langue: String,
    pub est_en_direct: bool,
    pub etat: String,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
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
pub struct ChaineTvResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub stream_url: Option<String>,
    pub image_couverture_url: Option<String>,
    pub categorie: String,
    pub pays: Option<String>,
    pub langue: String,
    pub est_en_direct: bool,
    pub role_partie_prenante: Option<String>,
    pub role_partie_prenante_autre: Option<String>,
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
    pub pays: Option<String>,
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
    pub pays: Option<String>,
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
            pays: self.pays_nom.clone(),
            langue: self.langue.clone(),
            est_en_direct: self.est_en_direct,
            role_partie_prenante: self.role_partie_prenante.clone(),
            role_partie_prenante_autre: self.role_partie_prenante_autre.clone(),
            created_at: self.created_at,
            interactions: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PARTIE 2 : Programmes Télé (table media_content.programme_tele, cf. 09g)
// ═══════════════════════════════════════════════════════════════════════════

pub const PROGRAMME_TELE_COLONNES: &str =
    "prt.id, prt.nom_emission, prt.slug, prt.description, prt.image_couverture_url,
     prt.video_url, prt.info_animateur, prt.info_producteur, prt.pays_id,
     prt.est_international, prt.langue, prt.etat, prt.cree_par,
     prt.chaine_id, prt.a_la_une, prt.a_la_une_globale,
     prt.theme_phare_id, prt.theme_phare_autre, prt.nombre_signalements,
     prt.created_at, prt.updated_at";

/// Source du média, déduite de l'URL — elle décide du lecteur employé côté
/// frontend (iframe tiers vs balise `<video>` native, FR-056). Une URL YouTube
/// injectée dans un `<video>` ne joue pas : c'est le bug latent que le retrait
/// du contenu provisoire codé en dur (FR-010) mettait à nu.
pub fn source_media(url: Option<&str>) -> String {
    match url {
        Some(u) if u.starts_with("/uploads/") => "hebergee".to_string(),
        Some(_) => "externe".to_string(),
        None => "aucune".to_string(),
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProgrammeTeleRow {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub video_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays_id: Option<Uuid>,
    pub est_international: bool,
    pub langue: String,
    pub etat: String,
    pub cree_par: Uuid,
    pub chaine_id: Option<Uuid>,
    pub a_la_une: bool,
    /// Vedette unique de toute la page Télé (FR-001) — à ne pas confondre avec
    /// `a_la_une`, qui met en avant un programme au sein de sa seule chaîne.
    pub a_la_une_globale: bool,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub nombre_signalements: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs jointures optionnels
    #[sqlx(default)]
    pub pays_nom: Option<String>,
    #[sqlx(default)]
    pub chaine_nom: Option<String>,
    #[sqlx(default)]
    pub chaine_slug: Option<String>,
    #[sqlx(default)]
    pub theme_phare_nom: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProgrammeTeleResponse {
    pub id: Uuid,
    pub nom_emission: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub video_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays: Option<String>,
    pub est_international: bool,
    pub langue: String,
    pub chaine_id: Option<Uuid>,
    pub chaine_nom: Option<String>,
    pub chaine_slug: Option<String>,
    pub a_la_une: bool,
    pub a_la_une_globale: bool,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
    pub theme_phare_nom: Option<String>,
    /// "hebergee" | "externe" | "aucune" — pilote le choix du lecteur (FR-056).
    pub source_media: String,
    pub created_at: DateTime<Utc>,
    /// Réactions, commentaires et partages agrégés (FR-027). `None` tant que
    /// l'appelant ne les a pas greffés — servir une carte n'oblige pas à les
    /// compter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<CompteursInteraction>,
}

/// Vedette de la page Télé : le programme mis en avant, plus l'indication qu'il
/// s'agit d'un repli faute de vedette désignée (FR-007).
#[derive(Debug, Serialize)]
pub struct ProgrammeVedetteResponse {
    #[serde(flatten)]
    pub programme: ProgrammeTeleResponse,
    pub est_repli: bool,
}

#[derive(Debug, Serialize)]
pub struct ProgrammeTeleListeResponse {
    pub programmes: Vec<ProgrammeTeleResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize)]
pub struct ProgrammeTeleQueryParams {
    pub recherche: Option<String>,
    pub pays: Option<String>,
    pub chaine: Option<Uuid>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreerProgrammeTeleForm {
    pub nom_emission: String,
    pub description: String,
    pub video_url: String,
    pub image_couverture_url: Option<String>,
    pub info_animateur: Option<String>,
    pub info_producteur: Option<String>,
    pub pays: Option<String>,
    pub est_international: Option<bool>,
    pub langue: Option<String>,
    pub chaine_id: Option<Uuid>,
    pub theme_phare_id: Option<Uuid>,
    pub theme_phare_autre: Option<String>,
}

// ── Sections de la page Télé (US1) ────────────────────────────────────
// Une section = une chaîne, son contenu mis en évidence et une rangée de ses
// autres contenus. Servir la page section par section évite de charger d'un
// bloc l'intégralité des programmes, et sert le chargement différé (FR-054).

#[derive(Debug, Serialize)]
pub struct TeleSectionResponse {
    pub chaine: ChaineTvResponse,
    /// `a_la_une` de la chaîne ; à défaut, son programme publié le plus récent.
    pub mis_en_evidence: Option<ProgrammeTeleResponse>,
    /// Les autres contenus publiés de la chaîne, hors `mis_en_evidence`.
    pub contenus: Vec<ProgrammeTeleResponse>,
    pub total_contenus: i64,
    /// Ce que la grille programme à l'instant de la requête, et ce qui suit
    /// (US5, FR-039). `None` si la chaîne n'a aucune grille active — la section
    /// retombe alors sur son contenu mis en évidence (FR-041).
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
    pub pays: Option<String>,
    pub categorie: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub contenus_par_section: Option<i64>,
}

// ── Stats Télévision ──────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct TelevisionStats {
    pub nombre_chaines: i64,
    pub nombre_pays: i64,
    pub nombre_programmes: i64,
    pub nombre_chaines_en_direct: i64,
}

// ── Conversions ───────────────────────────────────────────────────────

impl ProgrammeTeleRow {
    pub fn to_response(&self) -> ProgrammeTeleResponse {
        ProgrammeTeleResponse {
            id: self.id,
            nom_emission: self.nom_emission.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            video_url: self.video_url.clone(),
            info_animateur: self.info_animateur.clone(),
            info_producteur: self.info_producteur.clone(),
            pays: self.pays_nom.clone(),
            est_international: self.est_international,
            langue: self.langue.clone(),
            chaine_id: self.chaine_id,
            chaine_nom: self.chaine_nom.clone(),
            chaine_slug: self.chaine_slug.clone(),
            a_la_une: self.a_la_une,
            a_la_une_globale: self.a_la_une_globale,
            theme_phare_id: self.theme_phare_id,
            theme_phare_autre: self.theme_phare_autre.clone(),
            theme_phare_nom: self.theme_phare_nom.clone(),
            source_media: source_media(self.video_url.as_deref()),
            created_at: self.created_at,
            interactions: None,
        }
    }
}
