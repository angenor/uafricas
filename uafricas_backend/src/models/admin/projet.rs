use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ── Colonnes SQL ────────────────────────────────────────────

pub const ADMIN_PROJET_LISTE_COLONNES: &str =
    "pj.id, pj.titre, pj.slug,
     pj.etat::text AS etat,
     pj.nom_organisation, pj.cout_total::float8 AS cout_total,
     pj.devise, pj.duree_mois,
     pj.cree_par, pj.created_at,
     pa.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom";

pub const ADMIN_PROJET_DETAIL_COLONNES: &str =
    "pj.id, pj.titre, pj.slug,
     pj.nom_organisation, pj.description_organisation, pj.site_web,
     pj.pays_id, pj.ville, pj.contact_email, pj.contact_telephone,
     pj.cout_total::float8 AS cout_total, pj.devise, pj.duree_mois,
     pj.date_commencement_souhaitee,
     pj.description, pj.objectifs, pj.resultats_attendus,
     pj.activites_programmees, pj.echeanciers,
     pj.contribution_autonomisation, pj.difficultes_risques,
     pj.etat::text AS etat,
     pj.cree_par, pj.traite_par, pj.created_at, pj.updated_at,
     pa.nom AS pays_nom,
     u.nom AS auteur_nom, u.prenom AS auteur_prenom, u.email AS auteur_email,
     t.nom AS traite_par_nom, t.prenom AS traite_par_prenom";

pub const PROJET_TRI_COLONNES: &[&str] = &["created_at", "titre", "cout_total", "duree_mois", "etat"];

pub const PROJET_JOINS: &str =
    "LEFT JOIN shared.pays pa ON pa.id = pj.pays_id
     JOIN iam.utilisateur u ON u.id = pj.cree_par
     LEFT JOIN iam.utilisateur t ON t.id = pj.traite_par";

// ── Structs de listing ──────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminProjetListeResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub etat: String,
    pub nom_organisation: Option<String>,
    pub cout_total: Option<f64>,
    pub devise: Option<String>,
    pub duree_mois: Option<i32>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
}

// ── Structs de detail ───────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminProjetDetailResponse {
    pub id: Uuid,
    pub titre: String,
    pub slug: Option<String>,
    pub nom_organisation: Option<String>,
    pub description_organisation: Option<String>,
    pub site_web: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub cout_total: Option<f64>,
    pub devise: Option<String>,
    pub duree_mois: Option<i32>,
    pub date_commencement_souhaitee: Option<NaiveDate>,
    pub description: String,
    pub objectifs: String,
    pub resultats_attendus: Option<String>,
    pub activites_programmees: Option<String>,
    pub echeanciers: Option<String>,
    pub contribution_autonomisation: Option<String>,
    pub difficultes_risques: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub traite_par: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub auteur_nom: String,
    pub auteur_prenom: String,
    pub auteur_email: String,
    pub traite_par_nom: Option<String>,
    pub traite_par_prenom: Option<String>,
}

// ── Sous-entites ────────────────────────────────────────────

#[derive(Debug, Serialize, FromRow)]
pub struct AdminProjetDocument {
    pub id: Uuid,
    pub nom: String,
    pub url: String,
    pub type_mime: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── Requetes (DTOs) ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerProjetRequest {
    pub titre: String,
    pub description: String,
    pub objectifs: String,
    pub nom_organisation: Option<String>,
    pub description_organisation: Option<String>,
    pub site_web: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub cout_total: Option<f64>,
    pub devise: Option<String>,
    pub duree_mois: Option<i32>,
    pub date_commencement_souhaitee: Option<String>,
    pub resultats_attendus: Option<String>,
    pub activites_programmees: Option<String>,
    pub echeanciers: Option<String>,
    pub contribution_autonomisation: Option<String>,
    pub difficultes_risques: Option<String>,
    pub etat: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierProjetRequest {
    pub titre: Option<String>,
    pub description: Option<String>,
    pub objectifs: Option<String>,
    pub nom_organisation: Option<String>,
    pub description_organisation: Option<String>,
    pub site_web: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub contact_email: Option<String>,
    pub contact_telephone: Option<String>,
    pub cout_total: Option<f64>,
    pub devise: Option<String>,
    pub duree_mois: Option<i32>,
    pub date_commencement_souhaitee: Option<String>,
    pub resultats_attendus: Option<String>,
    pub activites_programmees: Option<String>,
    pub echeanciers: Option<String>,
    pub contribution_autonomisation: Option<String>,
    pub difficultes_risques: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangerEtatProjetRequest {
    pub etat: String,
}

#[derive(Debug, Deserialize)]
pub struct AjouterDocumentProjetRequest {
    pub nom: String,
    pub url: String,
    pub type_mime: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminProjetQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub etat: Option<String>,
    pub pays_id: Option<String>,
    pub organisation: Option<String>,
}
