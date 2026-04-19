//! Modèles Afripulse — personnalités, savoirs pratiques, recommandations, photos
//! visiteurs. Alignés sur `11c_country_profile_afripulse.sql` (§III SQL SoT).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sqlx(type_name = "country_profile.categorie_site_touristique", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CategorieSiteTouristique {
    Emblematique,
    Prive,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[sqlx(type_name = "country_profile.categorie_savoir", rename_all = "snake_case")]
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
#[sqlx(type_name = "country_profile.domaine_personnalite", rename_all = "snake_case")]
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
