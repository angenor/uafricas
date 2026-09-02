use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ══════════════════════════════════════════════════════════════
// ── Fiche Pays (CRUD principal) ──────────────────────────────
// ══════════════════════════════════════════════════════════════

pub const ADMIN_FICHE_PAYS_LISTE_COLONNES: &str =
    "fp.id, fp.pays_id, p.nom AS pays_nom, p.code_iso2 AS pays_code,
     fp.image_drapeau_url, fp.slogan, fp.population,
     fp.superficie_km2::float8 AS superficie_km2,
     u.nom || ' ' || u.prenom AS cree_par_nom,
     fp.created_at";

pub const ADMIN_FICHE_PAYS_DETAIL_COLONNES: &str =
    "fp.id, fp.pays_id, p.nom AS pays_nom, p.code_iso2 AS pays_code,
     fp.image_couverture_url, fp.slogan,
     fp.superficie_km2::float8 AS superficie_km2, fp.population,
     fp.biographie, fp.contexte, fp.contexte_historique,
     fp.image_drapeau_url, fp.image_embleme_url, fp.image_devise_url,
     fp.hymne_national, fp.langue_officielle, fp.langues_populaires,
     fp.monnaie, fp.fuseau_horaire,
     fp.drapeau_description, fp.embleme_description, fp.hymne_description,
     fp.fleur_nationale, fp.fleur_description,
     fp.animal_national, fp.animal_description,
     fp.oiseau_national, fp.oiseau_description,
     fp.bloquee, fp.nombre_signalements,
     fp.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
     fp.created_at, fp.updated_at";

pub const FICHE_PAYS_TRI_COLONNES: &[&str] = &["created_at", "population"];

#[derive(Debug, Serialize, FromRow)]
pub struct AdminFichePayListeResponse {
    pub id: Uuid,
    pub pays_id: Uuid,
    pub pays_nom: String,
    pub pays_code: Option<String>,
    pub image_drapeau_url: Option<String>,
    pub slogan: Option<String>,
    pub population: Option<i64>,
    pub superficie_km2: Option<f64>,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct AdminFichePayDetailRow {
    pub id: Uuid,
    pub pays_id: Uuid,
    pub pays_nom: String,
    pub pays_code: Option<String>,
    pub image_couverture_url: Option<String>,
    pub slogan: Option<String>,
    pub superficie_km2: Option<f64>,
    pub population: Option<i64>,
    pub biographie: Option<String>,
    pub contexte: Option<String>,
    pub contexte_historique: Option<String>,
    pub image_drapeau_url: Option<String>,
    pub image_embleme_url: Option<String>,
    pub image_devise_url: Option<String>,
    pub hymne_national: Option<String>,
    pub langue_officielle: Option<String>,
    pub langues_populaires: Option<String>,
    pub monnaie: Option<String>,
    pub fuseau_horaire: Option<String>,
    // Symboles nationaux (schéma 11l)
    pub drapeau_description: Option<String>,
    pub embleme_description: Option<String>,
    pub hymne_description: Option<String>,
    pub fleur_nationale: Option<String>,
    pub fleur_description: Option<String>,
    pub animal_national: Option<String>,
    pub animal_description: Option<String>,
    pub oiseau_national: Option<String>,
    pub oiseau_description: Option<String>,
    pub bloquee: bool,
    pub nombre_signalements: i32,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminFichePayDetailResponse {
    pub id: Uuid,
    pub pays_id: Uuid,
    pub pays_nom: String,
    pub pays_code: Option<String>,
    pub image_couverture_url: Option<String>,
    pub slogan: Option<String>,
    pub superficie_km2: Option<f64>,
    pub population: Option<i64>,
    pub biographie: Option<String>,
    pub contexte: Option<String>,
    pub contexte_historique: Option<String>,
    pub image_drapeau_url: Option<String>,
    pub image_embleme_url: Option<String>,
    pub image_devise_url: Option<String>,
    pub hymne_national: Option<String>,
    pub langue_officielle: Option<String>,
    pub langues_populaires: Option<String>,
    pub monnaie: Option<String>,
    pub fuseau_horaire: Option<String>,
    // Symboles nationaux (schéma 11l)
    pub drapeau_description: Option<String>,
    pub embleme_description: Option<String>,
    pub hymne_description: Option<String>,
    pub fleur_nationale: Option<String>,
    pub fleur_description: Option<String>,
    pub animal_national: Option<String>,
    pub animal_description: Option<String>,
    pub oiseau_national: Option<String>,
    pub oiseau_description: Option<String>,
    pub bloquee: bool,
    pub nombre_signalements: i32,
    pub cree_par: Uuid,
    pub cree_par_nom: Option<String>,
    pub nb_regions: i64,
    pub nb_groupes_ethniques: i64,
    pub nb_alliances: i64,
    pub nb_contes: i64,
    pub nb_sites_touristiques: i64,
    pub nb_secteurs: i64,
    pub nb_saisons: i64,
    pub nb_liens_interethniques: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Compteurs de sous-entites
pub struct SousEntiteCounts {
    pub nb_regions: i64,
    pub nb_groupes_ethniques: i64,
    pub nb_alliances: i64,
    pub nb_contes: i64,
    pub nb_sites_touristiques: i64,
    pub nb_secteurs: i64,
    pub nb_saisons: i64,
    pub nb_liens_interethniques: i64,
}

impl AdminFichePayDetailRow {
    pub fn to_response(&self, counts: SousEntiteCounts) -> AdminFichePayDetailResponse {
        AdminFichePayDetailResponse {
            id: self.id,
            pays_id: self.pays_id,
            pays_nom: self.pays_nom.clone(),
            pays_code: self.pays_code.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            slogan: self.slogan.clone(),
            superficie_km2: self.superficie_km2,
            population: self.population,
            biographie: self.biographie.clone(),
            contexte: self.contexte.clone(),
            contexte_historique: self.contexte_historique.clone(),
            image_drapeau_url: self.image_drapeau_url.clone(),
            image_embleme_url: self.image_embleme_url.clone(),
            image_devise_url: self.image_devise_url.clone(),
            hymne_national: self.hymne_national.clone(),
            langue_officielle: self.langue_officielle.clone(),
            langues_populaires: self.langues_populaires.clone(),
            monnaie: self.monnaie.clone(),
            fuseau_horaire: self.fuseau_horaire.clone(),
            drapeau_description: self.drapeau_description.clone(),
            embleme_description: self.embleme_description.clone(),
            hymne_description: self.hymne_description.clone(),
            fleur_nationale: self.fleur_nationale.clone(),
            fleur_description: self.fleur_description.clone(),
            animal_national: self.animal_national.clone(),
            animal_description: self.animal_description.clone(),
            oiseau_national: self.oiseau_national.clone(),
            oiseau_description: self.oiseau_description.clone(),
            bloquee: self.bloquee,
            nombre_signalements: self.nombre_signalements,
            cree_par: self.cree_par,
            cree_par_nom: self.cree_par_nom.clone(),
            nb_regions: counts.nb_regions,
            nb_groupes_ethniques: counts.nb_groupes_ethniques,
            nb_alliances: counts.nb_alliances,
            nb_contes: counts.nb_contes,
            nb_sites_touristiques: counts.nb_sites_touristiques,
            nb_secteurs: counts.nb_secteurs,
            nb_saisons: counts.nb_saisons,
            nb_liens_interethniques: counts.nb_liens_interethniques,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreerFichePayRequest {
    pub pays_id: Uuid,
    pub slogan: Option<String>,
    pub superficie_km2: Option<f64>,
    pub population: Option<i64>,
    pub biographie: Option<String>,
    pub contexte: Option<String>,
    pub contexte_historique: Option<String>,
    pub image_couverture_url: Option<String>,
    pub image_drapeau_url: Option<String>,
    pub image_embleme_url: Option<String>,
    pub image_devise_url: Option<String>,
    pub hymne_national: Option<String>,
    pub langue_officielle: Option<String>,
    pub langues_populaires: Option<String>,
    pub monnaie: Option<String>,
    pub fuseau_horaire: Option<String>,
    // Symboles nationaux (schéma 11l)
    pub drapeau_description: Option<String>,
    pub embleme_description: Option<String>,
    pub hymne_description: Option<String>,
    pub fleur_nationale: Option<String>,
    pub fleur_description: Option<String>,
    pub animal_national: Option<String>,
    pub animal_description: Option<String>,
    pub oiseau_national: Option<String>,
    pub oiseau_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierFichePayRequest {
    pub slogan: Option<String>,
    pub superficie_km2: Option<f64>,
    pub population: Option<i64>,
    pub biographie: Option<String>,
    pub contexte: Option<String>,
    pub contexte_historique: Option<String>,
    pub image_couverture_url: Option<String>,
    pub image_drapeau_url: Option<String>,
    pub image_embleme_url: Option<String>,
    pub image_devise_url: Option<String>,
    pub hymne_national: Option<String>,
    pub langue_officielle: Option<String>,
    pub langues_populaires: Option<String>,
    pub monnaie: Option<String>,
    pub fuseau_horaire: Option<String>,
    // Symboles nationaux (schéma 11l)
    pub drapeau_description: Option<String>,
    pub embleme_description: Option<String>,
    pub hymne_description: Option<String>,
    pub fleur_nationale: Option<String>,
    pub fleur_description: Option<String>,
    pub animal_national: Option<String>,
    pub animal_description: Option<String>,
    pub oiseau_national: Option<String>,
    pub oiseau_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminFichePayQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub tri_par: Option<String>,
    pub tri_dir: Option<String>,
    pub recherche: Option<String>,
    pub continent: Option<String>,
}

// ══════════════════════════════════════════════════════════════
// ── Regions ──────────────────────────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminRegionResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom: String,
    pub chef_lieu: Option<String>,
    pub description: Option<String>,
    pub population: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerRegionRequest {
    pub nom: String,
    pub chef_lieu: Option<String>,
    pub description: Option<String>,
    pub population: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierRegionRequest {
    pub nom: Option<String>,
    pub chef_lieu: Option<String>,
    pub description: Option<String>,
    pub population: Option<i64>,
}

// ══════════════════════════════════════════════════════════════
// ── Groupes ethniques ────────────────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminGroupeEthniqueResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom: String,
    pub description: Option<String>,
    pub objets_culturels_distinctifs: Option<String>,
    pub population_estimee: Option<String>,
    pub langues: Option<String>,
    pub region_id: Option<Uuid>,
    pub region_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerGroupeEthniqueRequest {
    pub nom: String,
    pub description: Option<String>,
    pub objets_culturels_distinctifs: Option<String>,
    pub population_estimee: Option<String>,
    pub langues: Option<String>,
    pub region_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierGroupeEthniqueRequest {
    pub nom: Option<String>,
    pub description: Option<String>,
    pub objets_culturels_distinctifs: Option<String>,
    pub population_estimee: Option<String>,
    pub langues: Option<String>,
    pub region_id: Option<Uuid>,
}

// ══════════════════════════════════════════════════════════════
// ── Alliances interethniques ─────────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminAllianceResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom: String,
    pub description: Option<String>,
    pub groupes_impliques: Option<String>,
    pub signification: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerAllianceRequest {
    pub nom: String,
    pub description: Option<String>,
    pub groupes_impliques: Option<String>,
    pub signification: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierAllianceRequest {
    pub nom: Option<String>,
    pub description: Option<String>,
    pub groupes_impliques: Option<String>,
    pub signification: Option<String>,
}

// ══════════════════════════════════════════════════════════════
// ── Contes & Histoires ───────────────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminConteResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub titre: String,
    pub contenu: Option<String>,
    pub type_conte: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub groupe_ethnique_nom: Option<String>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerConteRequest {
    pub titre: String,
    pub contenu: Option<String>,
    #[serde(rename = "type")]
    pub type_conte: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierConteRequest {
    pub titre: Option<String>,
    pub contenu: Option<String>,
    #[serde(rename = "type")]
    pub type_conte: Option<String>,
    pub groupe_ethnique_id: Option<Uuid>,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminConteQueryParams {
    pub type_conte: Option<String>,
}

// ══════════════════════════════════════════════════════════════
// ── Sites touristiques ───────────────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSiteTouristiqueResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom: String,
    pub categorie: Option<String>,
    pub sous_type: Option<String>,
    pub description: Option<String>,
    pub info_pertinente: Option<String>,
    pub image_url: Option<String>,
    pub images: Vec<String>,
    pub gestionnaire: Option<String>,
    pub ville: Option<String>,
    pub village: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub contact_telephone: Option<String>,
    pub contact_courriel: Option<String>,
    pub contact_adresse: Option<String>,
    pub constitution_statut_juridique: Option<String>,
    pub constitution_numero: Option<String>,
    pub constitution_document_url: Option<String>,
    pub site_web_url: Option<String>,
    pub verifie: bool,
    pub region_id: Option<Uuid>,
    pub region_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSiteTouristiqueRequest {
    pub nom: String,
    pub categorie: Option<String>,
    pub sous_type: Option<String>,
    pub description: Option<String>,
    pub info_pertinente: Option<String>,
    pub image_url: Option<String>,
    pub images: Option<Vec<String>>,
    pub gestionnaire: Option<String>,
    pub ville: Option<String>,
    pub village: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub contact_telephone: Option<String>,
    pub contact_courriel: Option<String>,
    pub contact_adresse: Option<String>,
    pub constitution_statut_juridique: Option<String>,
    pub constitution_numero: Option<String>,
    pub constitution_document_url: Option<String>,
    pub site_web_url: Option<String>,
    pub region_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSiteTouristiqueRequest {
    pub nom: Option<String>,
    pub categorie: Option<String>,
    pub sous_type: Option<String>,
    pub description: Option<String>,
    pub info_pertinente: Option<String>,
    pub image_url: Option<String>,
    pub images: Option<Vec<String>>,
    pub gestionnaire: Option<String>,
    pub ville: Option<String>,
    pub village: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub contact_telephone: Option<String>,
    pub contact_courriel: Option<String>,
    pub contact_adresse: Option<String>,
    pub constitution_statut_juridique: Option<String>,
    pub constitution_numero: Option<String>,
    pub constitution_document_url: Option<String>,
    pub site_web_url: Option<String>,
    pub region_id: Option<Uuid>,
}

/// Body du toggle admin du badge « Vérifié » (US3).
#[derive(Debug, Deserialize)]
pub struct VerificationSiteBody {
    pub verifie: bool,
}

/// Body de modération d'un avis de site (US5, masquage admin).
#[derive(Debug, Deserialize)]
pub struct MasquerAvisBody {
    pub masque: bool,
}

// ══════════════════════════════════════════════════════════════
// ── Secteurs de developpement ────────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSecteurResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSecteurRequest {
    pub nom: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSecteurRequest {
    pub nom: Option<String>,
    pub description: Option<String>,
}

// ══════════════════════════════════════════════════════════════
// ── Saisons ──────────────────────────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminSaisonResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom: String,
    pub description: Option<String>,
    pub mois_debut: Option<i32>,
    pub mois_fin: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerSaisonRequest {
    pub nom: String,
    pub description: Option<String>,
    pub mois_debut: Option<i32>,
    pub mois_fin: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierSaisonRequest {
    pub nom: Option<String>,
    pub description: Option<String>,
    pub mois_debut: Option<i32>,
    pub mois_fin: Option<i32>,
}

// ══════════════════════════════════════════════════════════════
// ── Liens interethniques ─────────────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminLienInterethniqueResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub pays_lie_id: Option<Uuid>,
    pub pays_lie_nom: Option<String>,
    pub description: Option<String>,
    pub type_lien: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreerLienInterethniqueRequest {
    pub pays_lie_id: Option<Uuid>,
    pub description: Option<String>,
    pub type_lien: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierLienInterethniqueRequest {
    pub pays_lie_id: Option<Uuid>,
    pub description: Option<String>,
    pub type_lien: Option<String>,
}

// ══════════════════════════════════════════════════════════════
// ── Contributions collaboratives ─────────────────────────────
// ══════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct AdminContributionListeResponse {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub pays_nom: String,
    pub section: String,
    /// Type d'objet visé (site, recette, personnalité…). Sans lui, la liste
    /// n'annonce que la section et le modérateur ne sait pas ce qu'il valide.
    pub type_objet_contribution: Option<String>,
    pub type_contribution: String,
    pub etat: String,
    pub contributeur_nom: Option<String>,
    pub traite_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
    pub traite_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AdminContributionDetailRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub pays_nom: String,
    pub section: String,
    pub type_contribution: String,
    pub ancienne_valeur: Option<String>,
    pub nouvelle_valeur: Option<String>,
    pub justification: Option<String>,
    pub etat: String,
    pub cree_par: Uuid,
    pub contributeur_nom: Option<String>,
    pub traite_par: Option<Uuid>,
    pub traite_par_nom: Option<String>,
    pub note_moderation: Option<String>,
    pub traite_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AdminContributionQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub etat: Option<String>,
    pub fiche_pays_id: Option<Uuid>,
    pub cree_par: Option<Uuid>,
    /// Filtre Afripulse : site_touristique, secteur_developpement, personnalite_connue,
    /// savoir_pratique, recommandation_visiteur, photo_visiteur, fiche_pays.
    pub type_objet: Option<String>,
    /// Filtre Afripulse : sites_emblematiques, sites_prives, secteurs_opportunites,
    /// personnalites, savoir_avant_voyager, recommandations, galerie_photos.
    pub section: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModererContributionRequest {
    pub etat: String,
    pub note_moderation: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RetirerContributionRequest {
    /// Motif obligatoire (10..1000 car.) expliquant le retrait.
    pub motif: String,
}

/// Contribution concurrente (meme fiche_pays, meme type_objet, meme target_id,
/// etat=en_attente) : avertit l'admin qu'elles seront marquees 'obsolete' a
/// l'approbation.
#[derive(Debug, Serialize, FromRow)]
pub struct AdminContributionConcurrente {
    pub id: Uuid,
    pub cree_par_nom: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Piece jointe d'une contribution photo_visiteur en attente (URL servie par
/// actix-files `/uploads/opportunite-afrique/photos/...`).
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct AdminContributionPieceJointe {
    pub chemin_fichier: String,
    pub legende: String,
    pub format: String,
    pub taille_octets: i64,
    pub largeur: i64,
    pub hauteur: i64,
    pub url_signee: String,
}

/// Reponse enrichie GET /api/admin/profils-pays/contributions/{id}, inclut
/// diff structure JSONB + pieces jointes + contributions concurrentes (T040).
#[derive(Debug, Serialize)]
pub struct AdminContributionDetailResponse {
    #[serde(flatten)]
    pub base: AdminContributionDetailRow,
    pub type_objet_contribution: String,
    pub section_afripulse: Option<String>,
    pub target_id: Option<Uuid>,
    pub nouvelle_valeur_jsonb: Option<serde_json::Value>,
    pub ancienne_valeur_jsonb: Option<serde_json::Value>,
    pub pieces_jointes: Vec<AdminContributionPieceJointe>,
    pub contributions_concurrentes: Vec<AdminContributionConcurrente>,
}
