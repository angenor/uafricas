use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour le mapping exchange.programme
/// Les casts ::text convertissent les enums PostgreSQL en texte pour sqlx
pub const PROGRAMME_COLONNES: &str =
    "p.id, p.titre, p.slug, p.description,
     p.image_couverture_url, p.document_legal_url,
     p.pays_id, p.ville, p.adresse,
     p.prise_en_charge_billet, p.prise_en_charge_hebergement,
     p.prise_en_charge_subsistance, p.prise_en_charge_details,
     p.duree::text AS duree, p.domaine_id,
     p.date_debut, p.date_fin,
     p.nombre_places, p.prerequis, p.langues_requises,
     p.etat::text AS etat, p.interafricain,
     p.type_organisation::text AS type_organisation,
     p.candidat_retenu_id, p.candidat_retenu_at,
     p.cree_par, p.created_at, p.updated_at";

/// Representation d'un programme en base de donnees
#[derive(Debug, FromRow)]
pub struct SabbatiqueRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub image_couverture_url: Option<String>,
    pub document_legal_url: Option<String>,
    pub pays_id: Uuid,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub prise_en_charge_billet: bool,
    pub prise_en_charge_hebergement: bool,
    pub prise_en_charge_subsistance: bool,
    pub prise_en_charge_details: Option<String>,
    pub duree: String,
    pub domaine_id: Option<Uuid>,
    pub date_debut: NaiveDate,
    pub date_fin: Option<NaiveDate>,
    pub nombre_places: Option<i32>,
    pub prerequis: Option<String>,
    pub langues_requises: Option<String>,
    pub etat: String,
    pub interafricain: bool,
    pub type_organisation: Option<String>,
    pub candidat_retenu_id: Option<Uuid>,
    pub candidat_retenu_at: Option<DateTime<Utc>>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Info organisateur jointe depuis iam.utilisateur
#[derive(Debug, Serialize, FromRow)]
pub struct OrganisateurInfo {
    pub id: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
    pub photo_url: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// DTOs de reponse
// ──────────────────────────────────────────────────────────────

/// DTO pour un programme dans la liste
#[derive(Debug, Serialize)]
pub struct SabbatiqueResponse {
    pub id: Uuid,
    pub titre: String,
    pub description: String,
    pub couverture_url: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub domaine: Option<String>,
    pub duree: String,
    pub duree_label: String,
    pub date_debut: NaiveDate,
    pub date_fin: Option<NaiveDate>,
    pub interafricain: bool,
    pub statut: String,
    pub prise_en_charge: Vec<String>,
    pub nombre_places: Option<i32>,
    pub nombre_candidatures: i64,
    pub type_organisation: Option<String>,
    pub type_organisation_label: Option<String>,
    pub candidat_retenu: Option<CandidatRetenuResponse>,
    pub user: OrganisateurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le detail d'un programme
#[derive(Debug, Serialize)]
pub struct SabbatiqueDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub couverture_url: Option<String>,
    pub document_url: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub domaine: Option<String>,
    pub duree: String,
    pub duree_label: String,
    pub date_debut: NaiveDate,
    pub date_fin: Option<NaiveDate>,
    pub interafricain: bool,
    pub statut: String,
    pub prise_en_charge: Vec<String>,
    pub prise_en_charge_details: Option<String>,
    pub nombre_places: Option<i32>,
    pub nombre_candidatures: i64,
    pub prerequis: Option<String>,
    pub langues_requises: Option<String>,
    pub type_organisation: Option<String>,
    pub type_organisation_label: Option<String>,
    pub candidat_retenu: Option<CandidatRetenuResponse>,
    /// Vrai si l'utilisateur connecte est l'organisateur du programme
    pub est_organisateur: bool,
    /// Vrai si l'utilisateur connecte a deja candidate
    pub a_deja_candidate: bool,
    pub user: OrganisateurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour l'organisateur
#[derive(Debug, Serialize)]
pub struct OrganisateurResponse {
    pub uid: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
    pub photo_url: Option<String>,
}

/// DTO public du candidat retenu (selection finale, affichee a tous)
#[derive(Debug, Serialize)]
pub struct CandidatRetenuResponse {
    pub uid: Uuid,
    pub nom: String,
    pub prenom: Option<String>,
    pub retenu_at: Option<DateTime<Utc>>,
}

/// Ligne de candidature en base
#[derive(Debug, FromRow)]
pub struct CandidatureRow {
    pub id: Uuid,
    pub candidat_id: Uuid,
    pub lettre_motivation: Option<String>,
    pub cv_url: Option<String>,
    pub lien_expertise: Option<String>,
    pub nom_etat_civil: Option<String>,
    pub fonction_actuelle: Option<String>,
    pub lieu_residence: Option<String>,
    pub statut_emploi: Option<String>,
    pub repond_profil: bool,
    pub statut: String,
    pub created_at: DateTime<Utc>,
}

/// DTO de candidature (vue organisateur)
#[derive(Debug, Serialize)]
pub struct CandidatureResponse {
    pub id: Uuid,
    pub candidat: OrganisateurResponse,
    pub nom_etat_civil: Option<String>,
    pub fonction_actuelle: Option<String>,
    pub lieu_residence: Option<String>,
    pub statut_emploi: Option<String>,
    pub statut_emploi_label: Option<String>,
    pub repond_profil: bool,
    pub lettre_motivation: Option<String>,
    pub cv_url: Option<String>,
    pub lien_expertise: Option<String>,
    pub statut: String,
    pub est_retenu: bool,
    pub created_at: DateTime<Utc>,
}

/// Reponse paginee pour le listing
#[derive(Debug, Serialize)]
pub struct SabbatiqueListeResponse {
    pub programmes: Vec<SabbatiqueResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Parametres de requete pour le listing
#[derive(Debug, Deserialize)]
pub struct SabbatiqueQueryParams {
    pub recherche: Option<String>,
    #[serde(rename = "type")]
    pub type_programme: Option<String>,
    pub pays: Option<String>,
    pub domaine: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ──────────────────────────────────────────────────────────────
// Fonctions de mapping
// ──────────────────────────────────────────────────────────────

/// Calculer le statut affiche a partir de etat + dates
pub fn calculer_statut(etat: &str, debut: &NaiveDate, fin: Option<&NaiveDate>) -> String {
    match etat {
        "annule" => "annule".to_string(),
        "termine" => "termine".to_string(),
        "suspendu" => "suspendu".to_string(),
        "brouillon" | "en_attente_validation" => "en_attente".to_string(),
        _ => {
            let aujourdhui = Utc::now().date_naive();
            if aujourdhui < *debut {
                "ouvert".to_string()
            } else if let Some(fin) = fin {
                if aujourdhui > *fin {
                    "termine".to_string()
                } else {
                    "en_cours".to_string()
                }
            } else {
                "en_cours".to_string()
            }
        }
    }
}

/// Label lisible pour la duree
pub fn duree_label(duree: &str) -> String {
    match duree {
        "1_semaine" => "1 semaine".to_string(),
        "2_semaines" => "2 semaines".to_string(),
        "3_semaines" => "3 semaines".to_string(),
        "6_semaines" => "6 semaines".to_string(),
        "1_mois" => "1 mois".to_string(),
        "2_mois" => "2 mois".to_string(),
        "3_mois" => "3 mois".to_string(),
        "6_mois" => "6 mois".to_string(),
        "1_an" => "1 an".to_string(),
        autre => autre.to_string(),
    }
}

/// Construire la liste des prises en charge
pub fn prises_en_charge(billet: bool, hebergement: bool, subsistance: bool) -> Vec<String> {
    let mut liste = Vec::new();
    if billet {
        liste.push("Billet d'avion".to_string());
    }
    if hebergement {
        liste.push("Hébergement".to_string());
    }
    if subsistance {
        liste.push("Frais de subsistance".to_string());
    }
    liste
}

/// Label lisible pour le type d'organisation soumettante
pub fn type_organisation_label(valeur: &str) -> String {
    match valeur {
        "association" => "Association".to_string(),
        "entreprise" => "Entreprise".to_string(),
        "service_public" => "Service public".to_string(),
        autre => autre.to_string(),
    }
}

/// Label lisible pour le statut d'emploi du candidat
pub fn statut_emploi_label(valeur: &str) -> String {
    match valeur {
        "en_emploi" => "En emploi".to_string(),
        "retraite" => "Retraité".to_string(),
        autre => autre.to_string(),
    }
}

/// Generer un slug URL-safe a partir du titre
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
