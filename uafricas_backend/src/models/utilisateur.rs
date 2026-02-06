use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour le mapping utilisateur
/// Le cast etat::text convertit l'enum PostgreSQL en texte pour sqlx
pub const UTILISATEUR_COLONNES: &str =
    "id, nom, prenom, email, mot_de_passe_hash, slug, photo_url,
     etat::text AS etat, email_verifie, derniere_connexion,
     created_at, updated_at, deleted_at";

/// Representation d'un utilisateur en base de donnees
/// Correspond a la table iam.utilisateur
#[derive(Debug, FromRow)]
pub struct Utilisateur {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub mot_de_passe_hash: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub etat: String,
    pub email_verifie: bool,
    pub derniere_connexion: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// ── Request DTOs ────────────────────────────────────────────────

/// Corps de la requete POST /api/auth/inscription
#[derive(Debug, Deserialize)]
pub struct InscriptionRequest {
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub mot_de_passe: String,
    pub confirmation_mot_de_passe: String,
}

/// Corps de la requete POST /api/auth/connexion
#[derive(Debug, Deserialize)]
pub struct ConnexionRequest {
    pub email: String,
    pub mot_de_passe: String,
}

/// Corps de la requete POST /api/auth/rafraichir et /api/auth/deconnexion
#[derive(Debug, Deserialize)]
pub struct RafraichirTokenRequest {
    pub refresh_token: String,
}

// ── Response DTOs ───────────────────────────────────────────────

/// Informations utilisateur renvoyees dans les reponses API
#[derive(Debug, Serialize, Clone)]
pub struct UtilisateurResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub etat: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Reponse d'authentification (inscription + connexion + rafraichissement)
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub utilisateur: UtilisateurResponse,
    pub access_token: String,
    pub refresh_token: String,
}

impl Utilisateur {
    /// Convertir en DTO de reponse (sans hash ni champs sensibles)
    pub fn to_response(&self, roles: Vec<String>) -> UtilisateurResponse {
        UtilisateurResponse {
            id: self.id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            email: self.email.clone(),
            slug: self.slug.clone(),
            photo_url: self.photo_url.clone(),
            etat: self.etat.clone(),
            roles,
            created_at: self.created_at,
        }
    }
}
