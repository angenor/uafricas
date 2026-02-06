use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour le mapping centre_culturel
/// Les casts ::float8 convertissent les DECIMAL PostgreSQL en f64 pour sqlx
pub const CENTRE_CULTUREL_COLONNES: &str =
    "id, nom, slug, description, image_couverture_url, pays_id, ville, adresse,
     longitude::float8 AS longitude, latitude::float8 AS latitude,
     actif, cree_par, created_at, updated_at";

/// Colonnes a selectionner pour le mapping programmation_centre
/// Le cast mode::text convertit l'enum PostgreSQL en texte pour sqlx
pub const PROGRAMMATION_COLONNES: &str =
    "id, centre_culturel_id, titre, description, lieu, mode::text AS mode,
     lien_en_ligne, date_heure_debut, date_heure_fin, nombre_places,
     cree_par, created_at, updated_at";

/// Representation complete d'un centre culturel en base de donnees
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CentreCulturel {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub pays_id: Option<Uuid>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub actif: bool,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Representation complete d'une programmation en base de donnees
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProgrammationCentre {
    pub id: Uuid,
    pub centre_culturel_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub lieu: Option<String>,
    pub mode: String,
    pub lien_en_ligne: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub nombre_places: Option<i32>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ──────────────────────────────────────────────────────────────
// DTOs de reponse
// ──────────────────────────────────────────────────────────────

/// DTO pour un centre dans la liste (sans programmations)
#[derive(Debug, Serialize)]
pub struct CentreCulturelResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub nombre_programmations: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour le detail d'un centre avec ses programmations et membres
#[derive(Debug, Serialize)]
pub struct CentreCulturelDetailResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub image_couverture_url: Option<String>,
    pub ville: Option<String>,
    pub adresse: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub membres: Vec<MembreCentreResponse>,
    pub programmations: Vec<ProgrammationResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO pour une programmation
#[derive(Debug, Serialize)]
pub struct ProgrammationResponse {
    pub id: Uuid,
    pub centre_culturel_id: Uuid,
    pub titre: String,
    pub description: Option<String>,
    pub lieu: Option<String>,
    pub mode: String,
    pub lien_en_ligne: Option<String>,
    pub date_heure_debut: DateTime<Utc>,
    pub date_heure_fin: Option<DateTime<Utc>>,
    pub nombre_places: Option<i32>,
    pub statut: String,
    pub created_at: DateTime<Utc>,
}

/// DTO pour le detail d'une programmation avec info du centre parent
#[derive(Debug, Serialize)]
pub struct ProgrammationDetailResponse {
    pub programmation: ProgrammationResponse,
    pub centre: CentreCulturelInfoResponse,
}

/// Info resumee du centre (pour inclusion dans detail programmation)
#[derive(Debug, Serialize)]
pub struct CentreCulturelInfoResponse {
    pub id: Uuid,
    pub nom: String,
    pub slug: Option<String>,
}

/// Representation d'un membre de centre (JOIN avec iam.utilisateur)
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MembreCentre {
    pub id: Uuid,
    pub role: String,
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
    pub telephone: Option<String>,
}

/// DTO pour un membre de centre dans la reponse API
#[derive(Debug, Serialize)]
pub struct MembreCentreResponse {
    pub nom: String,
    pub prenom: Option<String>,
    pub email: String,
    pub telephone: Option<String>,
    pub role: String,
    pub role_label: String,
}

/// Parametres de requete pour le listing des centres
#[derive(Debug, Deserialize)]
pub struct CentreCulturelQueryParams {
    pub recherche: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// Conversions
// ──────────────────────────────────────────────────────────────

impl ProgrammationCentre {
    /// Calculer le statut a partir des dates
    pub fn calculer_statut(&self) -> String {
        let maintenant = Utc::now();
        if self.date_heure_debut > maintenant {
            "a_venir".to_string()
        } else if let Some(fin) = self.date_heure_fin {
            if fin < maintenant {
                "termine".to_string()
            } else {
                "en_cours".to_string()
            }
        } else {
            "en_cours".to_string()
        }
    }

    /// Mapper le mode DB vers le format frontend
    fn mapper_mode(&self) -> String {
        match self.mode.as_str() {
            "en_ligne" => "en-ligne".to_string(),
            autre => autre.to_string(),
        }
    }

    pub fn to_response(&self) -> ProgrammationResponse {
        ProgrammationResponse {
            id: self.id,
            centre_culturel_id: self.centre_culturel_id,
            titre: self.titre.clone(),
            description: self.description.clone(),
            lieu: self.lieu.clone(),
            mode: self.mapper_mode(),
            lien_en_ligne: self.lien_en_ligne.clone(),
            date_heure_debut: self.date_heure_debut,
            date_heure_fin: self.date_heure_fin,
            nombre_places: self.nombre_places,
            statut: self.calculer_statut(),
            created_at: self.created_at,
        }
    }
}

impl MembreCentre {
    /// Mapper le role DB vers un label lisible en francais
    fn role_label(role: &str) -> String {
        match role {
            "president" => "Président".to_string(),
            "vice_president" => "Vice-président".to_string(),
            "resp_communication" => "Responsable Communication".to_string(),
            "membre" => "Membre".to_string(),
            autre => autre.to_string(),
        }
    }

    pub fn to_response(&self) -> MembreCentreResponse {
        MembreCentreResponse {
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            email: self.email.clone(),
            telephone: self.telephone.clone(),
            role: self.role.clone(),
            role_label: Self::role_label(&self.role),
        }
    }
}

impl CentreCulturel {
    pub fn to_detail_response(
        &self,
        membres: Vec<MembreCentreResponse>,
        programmations: Vec<ProgrammationResponse>,
    ) -> CentreCulturelDetailResponse {
        CentreCulturelDetailResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            image_couverture_url: self.image_couverture_url.clone(),
            ville: self.ville.clone(),
            adresse: self.adresse.clone(),
            latitude: self.latitude,
            longitude: self.longitude,
            membres,
            programmations,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    pub fn to_info_response(&self) -> CentreCulturelInfoResponse {
        CentreCulturelInfoResponse {
            id: self.id,
            nom: self.nom.clone(),
            slug: self.slug.clone(),
        }
    }
}
