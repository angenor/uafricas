use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL pour le listing (avec jointures aplaties) ──────────

/// Colonnes SELECT pour le listing des projets
/// Alias `p.` pour la table projet
/// Sous-requete pour l'image de couverture depuis projet_document
pub const PROJET_LISTE_COLONNES: &str =
    "p.id, p.titre, p.slug, p.description,
     p.nom_organisation, p.cout_total::FLOAT8 AS cout_total, p.devise, p.duree_mois,
     p.date_commencement_souhaitee, p.ville,
     p.etat::text AS etat, p.cree_par,
     p.created_at, p.updated_at,
     pays.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom,
     u.email AS auteur_email, u.photo_url AS auteur_photo_url,
     (SELECT pd.url FROM innovation.projet_document pd
      WHERE pd.projet_id = p.id ORDER BY pd.created_at ASC LIMIT 1) AS image_couverture_url";

// ── Row aplati pour le listing ──────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct ProjetListeRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub nom_organisation: Option<String>,
    pub cout_total: Option<f64>,
    pub devise: Option<String>,
    pub duree_mois: Option<i32>,
    pub date_commencement_souhaitee: Option<NaiveDate>,
    pub ville: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs joints
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub auteur_photo_url: Option<String>,
    // Sous-requete
    pub image_couverture_url: Option<String>,
}

// ── Row pour le detail ──────────────────────────────────────────────

pub const PROJET_DETAIL_COLONNES: &str =
    "p.id, p.titre, p.slug, p.description,
     p.nom_organisation, p.description_organisation, p.site_web,
     p.cout_total::FLOAT8 AS cout_total, p.devise, p.duree_mois,
     p.date_commencement_souhaitee, p.ville,
     p.contact_email::text AS contact_email, p.contact_telephone,
     p.objectifs, p.resultats_attendus, p.activites_programmees,
     p.echeanciers, p.contribution_autonomisation, p.difficultes_risques,
     p.etat::text AS etat, p.cree_par,
     p.created_at, p.updated_at,
     pays.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom,
     u.email AS auteur_email, u.photo_url AS auteur_photo_url";

#[derive(Debug, FromRow)]
pub struct ProjetDetailRow {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub nom_organisation: Option<String>,
    pub description_organisation: Option<String>,
    pub site_web: Option<String>,
    pub cout_total: Option<f64>,
    pub devise: Option<String>,
    pub duree_mois: Option<i32>,
    pub date_commencement_souhaitee: Option<NaiveDate>,
    pub ville: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub objectifs: String,
    pub resultats_attendus: Option<String>,
    pub activites_programmees: Option<String>,
    pub echeanciers: Option<String>,
    pub contribution_autonomisation: Option<String>,
    pub difficultes_risques: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Champs joints
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub auteur_photo_url: Option<String>,
}

// ── DTOs de reponse ─────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ProjetAuteurResponse {
    pub uid: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
}

/// DTO pour un document attache a un projet
#[derive(Debug, Serialize, FromRow)]
pub struct ProjetDocumentResponse {
    pub id: Uuid,
    pub nom: String,
    pub url: String,
    pub type_mime: Option<String>,
}

/// DTO pour un projet dans la liste
#[derive(Debug, Serialize)]
pub struct ProjetResponse {
    pub id: Uuid,
    pub titre: String,
    pub description: String,
    pub organisation: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub cout_total: Option<f64>,
    pub devise: String,
    pub duree: String,
    pub date_debut_souhaitee: Option<String>,
    pub statut: String,
    pub image_couverture: Option<String>,
    pub user: ProjetAuteurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le detail d'un projet
#[derive(Debug, Serialize)]
pub struct ProjetDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub description: String,
    pub organisation: Option<String>,
    pub description_organisation: Option<String>,
    pub site_web: Option<String>,
    pub pays: Option<String>,
    pub ville: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub cout_total: Option<f64>,
    pub devise: String,
    pub duree: String,
    pub date_debut_souhaitee: Option<String>,
    pub objectifs: Vec<String>,
    pub resultats_attendus: Option<String>,
    pub activites_programmees: Option<String>,
    pub echeanciers: Option<String>,
    pub contribution_autonomisation: Option<String>,
    pub difficultes_risques: Option<String>,
    pub statut: String,
    pub documents: Vec<ProjetDocumentResponse>,
    pub user: ProjetAuteurResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Reponse paginee pour le listing
#[derive(Debug, Serialize)]
pub struct ProjetListeResponse {
    pub projets: Vec<ProjetResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// Statistiques des projets
#[derive(Debug, Serialize)]
pub struct ProjetStatistiquesResponse {
    pub total: i64,
    pub valides: i64,
    pub en_cours: i64,
    pub termines: i64,
}

// ── Parametres de requete ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ProjetQueryParams {
    pub recherche: Option<String>,
    pub pays: Option<String>,
    pub budget_max: Option<f64>,
    pub duree: Option<String>,
    pub tri: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

// ── Fonctions utilitaires ───────────────────────────────────────────

/// Mapper l'etat DB vers le statut frontend
pub fn mapper_etat_frontend(etat: &str) -> String {
    match etat {
        "soumis" => "soumis".to_string(),
        "en_revue" => "soumis".to_string(),
        "approuve" => "valide".to_string(),
        "en_cours" => "en_cours".to_string(),
        "termine" => "termine".to_string(),
        "suspendu" => "rejete".to_string(),
        "rejete" => "rejete".to_string(),
        autre => autre.to_string(),
    }
}

/// Formater la duree en mois en chaine lisible
pub fn formater_duree(mois: Option<i32>) -> String {
    match mois {
        Some(1) => "1 mois".to_string(),
        Some(m) => format!("{} mois", m),
        None => "Non spécifié".to_string(),
    }
}

/// Parser le champ objectifs TEXT en Vec<String>
pub fn parser_objectifs(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.trim().trim_start_matches('-').trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Joindre les objectifs Vec<String> en TEXT pour stockage
pub fn joindre_objectifs(objectifs: &[String]) -> String {
    objectifs
        .iter()
        .map(|o| format!("- {}", o))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generer un slug a partir du titre
pub fn generer_slug(titre: &str) -> String {
    titre
        .to_lowercase()
        .replace('é', "e")
        .replace('è', "e")
        .replace('ê', "e")
        .replace('ë', "e")
        .replace('à', "a")
        .replace('â', "a")
        .replace('ä', "a")
        .replace('ù', "u")
        .replace('û', "u")
        .replace('ü', "u")
        .replace('ô', "o")
        .replace('ö', "o")
        .replace('î', "i")
        .replace('ï', "i")
        .replace('ç', "c")
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

// ── Conversions Row -> Response ─────────────────────────────────────

impl ProjetListeRow {
    pub fn to_response(&self) -> ProjetResponse {
        ProjetResponse {
            id: self.id,
            titre: self.titre.clone(),
            description: self.description.clone(),
            organisation: self.nom_organisation.clone(),
            pays: self.pays_nom.clone(),
            ville: self.ville.clone(),
            cout_total: self.cout_total,
            devise: self.devise.clone().unwrap_or_else(|| "XOF".to_string()),
            duree: formater_duree(self.duree_mois),
            date_debut_souhaitee: self
                .date_commencement_souhaitee
                .map(|d| d.format("%Y-%m-%d").to_string()),
            statut: mapper_etat_frontend(&self.etat),
            image_couverture: self.image_couverture_url.clone(),
            user: ProjetAuteurResponse {
                uid: self.cree_par,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                photo_url: self.auteur_photo_url.clone(),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl ProjetDetailRow {
    pub fn to_detail_response(&self, documents: Vec<ProjetDocumentResponse>) -> ProjetDetailResponse {
        ProjetDetailResponse {
            id: self.id,
            titre: self.titre.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            organisation: self.nom_organisation.clone(),
            description_organisation: self.description_organisation.clone(),
            site_web: self.site_web.clone(),
            pays: self.pays_nom.clone(),
            ville: self.ville.clone(),
            contact_email: self.contact_email.clone(),
            contact_telephone: self.contact_telephone.clone(),
            cout_total: self.cout_total,
            devise: self.devise.clone().unwrap_or_else(|| "XOF".to_string()),
            duree: formater_duree(self.duree_mois),
            date_debut_souhaitee: self
                .date_commencement_souhaitee
                .map(|d| d.format("%Y-%m-%d").to_string()),
            objectifs: parser_objectifs(&self.objectifs),
            resultats_attendus: self.resultats_attendus.clone(),
            activites_programmees: self.activites_programmees.clone(),
            echeanciers: self.echeanciers.clone(),
            contribution_autonomisation: self.contribution_autonomisation.clone(),
            difficultes_risques: self.difficultes_risques.clone(),
            statut: mapper_etat_frontend(&self.etat),
            documents,
            user: ProjetAuteurResponse {
                uid: self.cree_par,
                nom: self.auteur_nom.clone(),
                prenom: self.auteur_prenom.clone(),
                photo_url: self.auteur_photo_url.clone(),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
