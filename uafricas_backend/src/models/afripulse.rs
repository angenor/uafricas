//! Modèles Afripulse : personnalités, savoirs pratiques, recommandations, photos
//! visiteurs. Alignés sur `11c_country_profile_afripulse.sql` (§III SQL SoT).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sqlx(
    type_name = "country_profile.categorie_site_touristique",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum CategorieSiteTouristique {
    Emblematique,
    Prive,
}

/// Sous-type d'un site touristique (précise la famille `categorie`).
/// 20 valeurs : 12 emblématiques + 8 privées. Alignée sur l'enum SQL
/// `country_profile.sous_type_site` (11d_…). La cohérence famille↔sous-type est
/// validée en code via [`sous_type_appartient_a`] (Principe V, un seul enum).
#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sqlx(
    type_name = "country_profile.sous_type_site",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum SousTypeSite {
    // Emblématiques
    Plage,
    Monument,
    ReliefNaturel,
    ParcNaturel,
    Mosquee,
    Eglise,
    Pont,
    Route,
    ServicePublic,
    ImmeubleEdifice,
    MerRiviere,
    SiteNaturel,
    // Privés
    Hotel,
    PlagePrivee,
    EspaceJeux,
    AgricultureTouristique,
    ResidenceTouristique,
    Restaurant,
    Discotheque,
    BarMaquis,
}

/// Sous-types autorisés pour la famille « emblematique » (FR-003).
const SOUS_TYPES_EMBLEMATIQUES: &[&str] = &[
    "plage",
    "monument",
    "relief_naturel",
    "parc_naturel",
    "mosquee",
    "eglise",
    "pont",
    "route",
    "service_public",
    "immeuble_edifice",
    "mer_riviere",
    "site_naturel",
];

/// Sous-types autorisés pour la famille « prive » (FR-003).
const SOUS_TYPES_PRIVES: &[&str] = &[
    "hotel",
    "plage_privee",
    "espace_jeux",
    "agriculture_touristique",
    "residence_touristique",
    "restaurant",
    "discotheque",
    "bar_maquis",
];

/// Vérifie que `sous_type` (valeur SQL snake_case) appartient bien à la famille
/// `categorie` (FR-003). Source unique du mapping famille↔sous-type.
pub fn sous_type_appartient_a(categorie: &CategorieSiteTouristique, sous_type: &str) -> bool {
    match categorie {
        CategorieSiteTouristique::Emblematique => SOUS_TYPES_EMBLEMATIQUES.contains(&sous_type),
        CategorieSiteTouristique::Prive => SOUS_TYPES_PRIVES.contains(&sous_type),
    }
}

/// Vérifie qu'une valeur de sous-type est l'une des 20 valeurs connues
/// (toutes familles confondues) : utilisé pour ignorer un filtre invalide.
pub fn sous_type_site_valide(sous_type: &str) -> bool {
    SOUS_TYPES_EMBLEMATIQUES.contains(&sous_type) || SOUS_TYPES_PRIVES.contains(&sous_type)
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sqlx(
    type_name = "country_profile.categorie_savoir",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum CategorieSavoir {
    LangueArgot,
    Coutumes,
    Etiquette,
    Securite,
    Sante,
    Transports,
    Autre,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sqlx(
    type_name = "country_profile.domaine_personnalite",
    rename_all = "snake_case"
)]
#[serde(rename_all = "snake_case")]
pub enum DomainePersonnalite {
    Politique,
    ArtisteMusicien,
    ArtisteAutre,
    Sportif,
    Entrepreneur,
    Scientifique,
    MilitaireHistorique,
    Autre,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct PersonnaliteConnueRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom_complet: String,
    pub domaine: String,
    pub biographie_courte: String,
    pub annee_naissance: Option<i16>,
    pub annee_deces: Option<i16>,
    pub portrait_url: Option<String>,
    pub lien_reference: Option<String>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct SavoirPratiqueRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub titre: String,
    pub categorie: String,
    pub explication: String,
    pub exemple: Option<String>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct RecommandationVisiteurRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub utilisateur_id: Uuid,
    pub note: i16,
    pub commentaire: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct PhotoVisiteurRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub utilisateur_id: Uuid,
    pub chemin_fichier: String,
    pub legende: String,
    pub format: String,
    pub taille_octets: i32,
    pub largeur_px: i16,
    pub hauteur_px: i16,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Avis noté 1–5 d'un visiteur sur un site touristique. Aligné sur
/// `country_profile.avis_site` (11d_…). Écriture directe (D2) : un avis actif
/// au plus par (utilisateur, site).
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct AvisSiteRow {
    pub id: Uuid,
    pub site_id: Uuid,
    pub utilisateur_id: Uuid,
    pub note: i16,
    pub commentaire: String,
    pub masque_par_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
