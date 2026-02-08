use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ────────────────────────────────────────────────────────────────
// Colonnes SQL
// ────────────────────────────────────────────────────────────────

/// Colonnes pour SELECT sur iam.utilisateur u + shared.pays p
/// Les specialites sont recuperees dans une sous-requete separee
pub const BIBLIO_HUMAINE_COLONNES: &str =
    "u.id,
     u.nom,
     u.prenom,
     u.photo_url,
     u.fonction,
     u.biographie,
     u.ville,
     u.created_at,
     p.nom AS pays_nom,
     COALESCE(
         (SELECT string_agg(sb.nom, ', ' ORDER BY sb.nom)
          FROM iam.utilisateur_specialite us2
          JOIN iam.specialite_bibliotheque sb ON sb.id = us2.specialite_id
          WHERE us2.utilisateur_id = u.id),
         ''
     ) AS specialites_noms,
     COALESCE(
         (SELECT sb2.nom
          FROM iam.utilisateur_specialite us3
          JOIN iam.specialite_bibliotheque sb2 ON sb2.id = us3.specialite_id
          WHERE us3.utilisateur_id = u.id
          LIMIT 1),
         ''
     ) AS specialite_principale";

// ────────────────────────────────────────────────────────────────
// Row brute depuis la BDD
// ────────────────────────────────────────────────────────────────

#[derive(Debug, FromRow)]
pub struct BiblioHumaineRow {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
    pub fonction: Option<String>,
    pub biographie: Option<String>,
    pub ville: Option<String>,
    pub created_at: DateTime<Utc>,
    pub pays_nom: Option<String>,
    pub specialites_noms: String,
    pub specialite_principale: String,
}

// ────────────────────────────────────────────────────────────────
// Response DTOs
// ────────────────────────────────────────────────────────────────

/// DTO pour une bibliotheque humaine (liste et detail)
#[derive(Debug, Serialize)]
pub struct BiblioHumaineResponse {
    pub id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub nom: String,
    pub prenom: String,
    #[serde(rename = "photoUrl")]
    pub photo_url: Option<String>,
    pub fonction: String,
    pub pays: String,
    pub specialite: String,
    pub specialites: Vec<String>,
    pub biographie: String,
    pub ville: Option<String>,
    #[serde(rename = "dateInscription")]
    pub date_inscription: DateTime<Utc>,
}

/// Reponse paginee pour la liste
#[derive(Debug, Serialize)]
pub struct BiblioHumaineListeResponse {
    pub bibliotheques: Vec<BiblioHumaineResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

/// DTO pour une specialite
#[derive(Debug, Serialize)]
pub struct SpecialiteResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
}

#[derive(Debug, FromRow)]
pub struct SpecialiteRow {
    pub id: Uuid,
    pub nom: String,
    pub slug: String,
}

/// Parametres de requete pour le listing
#[derive(Debug, Deserialize)]
pub struct BiblioHumaineQueryParams {
    pub recherche: Option<String>,
    pub specialite: Option<String>,
    pub pays: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// Body pour l'inscription en tant que bibliotheque humaine
#[derive(Debug, Deserialize)]
pub struct InscriptionBiblioBody {
    pub specialites: Vec<String>,
}

// ────────────────────────────────────────────────────────────────
// Conversions Row → Response
// ────────────────────────────────────────────────────────────────

impl BiblioHumaineRow {
    pub fn to_response(&self) -> BiblioHumaineResponse {
        let specialites: Vec<String> = if self.specialites_noms.is_empty() {
            vec![]
        } else {
            self.specialites_noms
                .split(", ")
                .map(|s| s.to_string())
                .collect()
        };

        BiblioHumaineResponse {
            id: self.id,
            user_id: self.id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            photo_url: self.photo_url.clone(),
            fonction: self.fonction.clone().unwrap_or_else(|| "Expert".to_string()),
            pays: self.pays_nom.clone().unwrap_or_else(|| "International".to_string()),
            specialite: if self.specialite_principale.is_empty() {
                "Bibliothèque Humaine".to_string()
            } else {
                self.specialite_principale.clone()
            },
            specialites,
            biographie: self.biographie.clone().unwrap_or_default(),
            ville: self.ville.clone(),
            date_inscription: self.created_at,
        }
    }
}

impl SpecialiteRow {
    pub fn to_response(&self) -> SpecialiteResponse {
        SpecialiteResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
        }
    }
}
