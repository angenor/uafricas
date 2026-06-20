use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes pour SELECT sur country_profile.fiche_pays + shared.pays
pub const FICHE_PAYS_COLONNES: &str =
    "fp.id, fp.pays_id, fp.image_couverture_url, fp.slogan,
     fp.superficie_km2::text AS superficie_km2, fp.population, fp.biographie, fp.contexte,
     fp.image_drapeau_url, fp.image_embleme_url, fp.image_devise_url,
     fp.hymne_national, fp.langue_officielle, fp.langues_populaires,
     fp.monnaie, fp.fuseau_horaire,
     fp.voyage_langue_internationale, fp.voyage_langue_locale,
     fp.voyage_infos_visa, fp.voyage_infos_sanitaires, fp.voyage_meteo,
     fp.voyage_prises_electriques, fp.voyage_contacts_tourisme,
     fp.voyage_recommandations_securite,
     fp.nombre_likes, fp.nombre_dislikes, fp.nombre_signalements, fp.bloquee,
     fp.cree_par, fp.created_at, fp.updated_at,
     p.nom AS pays_nom, p.code_iso2 AS pays_code, p.capitale AS pays_capitale";

/// Ligne brute issue de la BDD (JOIN fiche_pays + shared.pays)
#[derive(Debug, FromRow)]
pub struct FichePaysRow {
    pub id: Uuid,
    pub pays_id: Uuid,
    pub image_couverture_url: Option<String>,
    pub slogan: Option<String>,
    pub superficie_km2: Option<String>,
    pub population: Option<i64>,
    pub biographie: Option<String>,
    pub contexte: Option<String>,
    pub image_drapeau_url: Option<String>,
    pub image_embleme_url: Option<String>,
    pub image_devise_url: Option<String>,
    pub hymne_national: Option<String>,
    pub langue_officielle: Option<String>,
    pub langues_populaires: Option<String>,
    pub monnaie: Option<String>,
    pub fuseau_horaire: Option<String>,
    // Bloc « À savoir avant de voyager »
    pub voyage_langue_internationale: Option<String>,
    pub voyage_langue_locale: Option<String>,
    pub voyage_infos_visa: Option<String>,
    pub voyage_infos_sanitaires: Option<String>,
    pub voyage_meteo: Option<String>,
    pub voyage_prises_electriques: Option<String>,
    pub voyage_contacts_tourisme: Option<String>,
    pub voyage_recommandations_securite: Option<String>,
    // Réactions / modération communautaire (schéma 11h)
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub nombre_signalements: i32,
    pub bloquee: bool,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs du JOIN shared.pays
    pub pays_nom: String,
    pub pays_code: Option<String>,
    pub pays_capitale: Option<String>,
}

/// Groupe ethnique associe a une fiche
#[derive(Debug, Serialize, FromRow)]
pub struct GroupeEthniqueRow {
    pub id: Uuid,
    pub nom: String,
    pub description: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Response DTOs
// ────────────────────────────────────────────────────────────────

/// DTO pour la liste des fiches pays
#[derive(Debug, Serialize)]
pub struct FichePaysResponse {
    pub id: Uuid,
    pub pays_id: Uuid,
    pub code: Option<String>,
    pub nom: String,
    pub capitale: Option<String>,
    pub image_couverture: Option<String>,
    pub slogan: Option<String>,
    pub superficie: Option<String>,
    pub population: Option<String>,
    pub monnaie: Option<String>,
    pub drapeau_url: Option<String>,
    pub region: String,
    pub nombre_contributions: i64,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le detail d'une fiche pays
#[derive(Debug, Serialize)]
pub struct FichePaysDetailResponse {
    pub id: Uuid,
    pub pays_id: Uuid,
    pub code: Option<String>,
    pub nom: String,
    pub capitale: Option<String>,
    pub image_couverture: Option<String>,
    pub slogan: Option<String>,
    pub superficie: Option<String>,
    pub population: Option<String>,
    pub monnaie: Option<String>,
    pub drapeau_url: Option<String>,
    pub embleme_url: Option<String>,
    pub devise: Option<String>,
    pub hymne_national: Option<String>,
    pub langue_officielle: Option<String>,
    pub langues: Vec<String>,
    pub ethnies: Vec<String>,
    pub region: String,
    pub biographie: Option<String>,
    pub contexte: Option<String>,
    pub fuseau_horaire: Option<String>,
    // Bloc « À savoir avant de voyager »
    pub voyage_langue_internationale: Option<String>,
    pub voyage_langue_locale: Option<String>,
    pub voyage_infos_visa: Option<String>,
    pub voyage_infos_sanitaires: Option<String>,
    pub voyage_meteo: Option<String>,
    pub voyage_prises_electriques: Option<String>,
    pub voyage_contacts_tourisme: Option<String>,
    pub voyage_recommandations_securite: Option<String>,
    pub nombre_contributions: i64,
    // Réactions / modération communautaire (schéma 11h)
    pub nombre_likes: i32,
    pub nombre_dislikes: i32,
    pub nombre_signalements: i32,
    pub bloquee: bool,
    /// Réaction de l'utilisateur courant : "like" | "dislike" | null
    pub ma_reaction: Option<String>,
    /// L'utilisateur courant a-t-il déjà signalé cette fiche ?
    pub a_signale: bool,
    pub updated_at: DateTime<Utc>,
}

/// Reponse paginee pour la liste
#[derive(Debug, Serialize)]
pub struct FichePaysListeResponse {
    pub fiches: Vec<FichePaysResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Parametres de requete pour la liste
#[derive(Debug, Deserialize)]
pub struct FichePaysQueryParams {
    pub recherche: Option<String>,
    pub region: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ────────────────────────────────────────────────────────────────
// Fonctions utilitaires
// ────────────────────────────────────────────────────────────────

/// Determine la region africaine a partir du code ISO 3166-1 alpha-2 (majuscules).
/// Classification couvrant les 55 pays africains, alignee sur les 5 regions de
/// la legende (Nord, Ouest, Centrale, Est, Australe).
pub fn region_depuis_code(code: Option<&str>) -> String {
    let r = match code.map(|c| c.to_uppercase()).as_deref() {
        // Afrique du Nord
        Some("DZ") | Some("EG") | Some("LY") | Some("MA") | Some("TN") | Some("SD")
        | Some("EH") => "Afrique du Nord",
        // Afrique de l'Ouest
        Some("BJ") | Some("BF") | Some("CV") | Some("CI") | Some("GM") | Some("GH")
        | Some("GN") | Some("GW") | Some("LR") | Some("ML") | Some("MR") | Some("NE")
        | Some("NG") | Some("SN") | Some("SL") | Some("TG") => "Afrique de l'Ouest",
        // Afrique Centrale
        Some("AO") | Some("CM") | Some("CF") | Some("TD") | Some("CG") | Some("CD")
        | Some("GQ") | Some("GA") | Some("ST") => "Afrique Centrale",
        // Afrique de l'Est
        Some("BI") | Some("KM") | Some("DJ") | Some("ER") | Some("ET") | Some("KE")
        | Some("MG") | Some("MU") | Some("RW") | Some("SC") | Some("SO") | Some("SS")
        | Some("TZ") | Some("UG") => "Afrique de l'Est",
        // Afrique Australe
        Some("BW") | Some("SZ") | Some("LS") | Some("MW") | Some("MZ") | Some("NA")
        | Some("ZA") | Some("ZM") | Some("ZW") => "Afrique Australe",
        _ => "Autre",
    };
    r.to_string()
}

/// Formate la population en texte lisible
pub fn formater_population(pop: Option<i64>) -> Option<String> {
    pop.map(|p| {
        if p >= 1_000_000 {
            let millions = p as f64 / 1_000_000.0;
            format!("{:.1} millions", millions)
        } else if p >= 1_000 {
            format!("{} mille", p / 1_000)
        } else {
            format!("{}", p)
        }
    })
}

/// Formate la superficie (String depuis DECIMAL cast) en texte lisible
pub fn formater_superficie(sup: Option<&str>) -> Option<String> {
    use std::fmt::Write;
    sup.map(|s| {
        let val: i64 = s.split('.').next()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0);
        // Formater avec separateur de milliers
        let s_str = val.to_string();
        let mut result = String::new();
        for (i, c) in s_str.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                let _ = write!(result, " ");
            }
            result.push(c);
        }
        let formatted: String = result.chars().rev().collect();
        format!("{} km²", formatted)
    })
}

/// Parse les langues populaires (separees par virgules) en Vec
pub fn parser_langues(langue_officielle: Option<&str>, langues_populaires: Option<&str>) -> Vec<String> {
    let mut langues = Vec::new();

    if let Some(off) = langue_officielle {
        for l in off.split(',') {
            let trimmed = l.trim().to_string();
            if !trimmed.is_empty() && !langues.contains(&trimmed) {
                langues.push(trimmed);
            }
        }
    }

    if let Some(pop) = langues_populaires {
        for l in pop.split(',') {
            let trimmed = l.trim().to_string();
            if !trimmed.is_empty() && !langues.contains(&trimmed) {
                langues.push(trimmed);
            }
        }
    }

    langues
}
