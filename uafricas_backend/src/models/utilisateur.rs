use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Colonnes a selectionner pour le mapping utilisateur
/// Le cast etat::text convertit l'enum PostgreSQL en texte pour sqlx
pub const UTILISATEUR_COLONNES: &str =
    "id, nom, prenom, email, mot_de_passe_hash, slug, photo_url,
     etat::text AS etat, email_verifie, derniere_connexion,
     created_at, updated_at, deleted_at";

/// Colonnes etendues pour le profil (inclut champs editables)
pub const PROFIL_COLONNES: &str =
    "id, nom, prenom, email, mot_de_passe_hash, slug, photo_url,
     telephone, genre::text AS genre, date_naissance, fonction,
     localite, ville, biographie, langue_preferee,
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

/// Representation etendue d'un utilisateur pour le profil
#[derive(Debug, FromRow)]
pub struct ProfilUtilisateur {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub mot_de_passe_hash: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub telephone: Option<String>,
    pub genre: String,
    pub date_naissance: Option<NaiveDate>,
    pub fonction: Option<String>,
    pub localite: Option<String>,
    pub ville: Option<String>,
    pub biographie: Option<String>,
    pub langue_preferee: String,
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

/// Corps de la requete PUT /api/auth/profil
#[derive(Debug, Deserialize)]
pub struct ModifierProfilRequest {
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub telephone: Option<String>,
    pub genre: Option<String>,
    pub date_naissance: Option<NaiveDate>,
    pub fonction: Option<String>,
    pub localite: Option<String>,
    pub ville: Option<String>,
    pub biographie: Option<String>,
    pub langue_preferee: Option<String>,
}

/// Corps de la requete POST /api/auth/changer-mot-de-passe
#[derive(Debug, Deserialize)]
pub struct ChangerMotDePasseRequest {
    pub ancien_mot_de_passe: String,
    pub nouveau_mot_de_passe: String,
    pub confirmation_mot_de_passe: String,
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

/// Reponse etendue du profil utilisateur
#[derive(Debug, Serialize, Clone)]
pub struct ProfilResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub telephone: Option<String>,
    pub genre: String,
    pub date_naissance: Option<NaiveDate>,
    pub fonction: Option<String>,
    pub localite: Option<String>,
    pub ville: Option<String>,
    pub biographie: Option<String>,
    pub langue_preferee: String,
    pub etat: String,
    pub email_verifie: bool,
    pub roles: Vec<String>,
    pub derniere_connexion: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
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

impl ProfilUtilisateur {
    /// Convertir en DTO profil etendu (sans hash ni champs sensibles)
    pub fn to_profil_response(&self, roles: Vec<String>) -> ProfilResponse {
        ProfilResponse {
            id: self.id,
            nom: self.nom.clone(),
            prenom: self.prenom.clone(),
            email: self.email.clone(),
            slug: self.slug.clone(),
            photo_url: self.photo_url.clone(),
            telephone: self.telephone.clone(),
            genre: self.genre.clone(),
            date_naissance: self.date_naissance,
            fonction: self.fonction.clone(),
            localite: self.localite.clone(),
            ville: self.ville.clone(),
            biographie: self.biographie.clone(),
            langue_preferee: self.langue_preferee.clone(),
            etat: self.etat.clone(),
            email_verifie: self.email_verifie,
            roles,
            derniere_connexion: self.derniere_connexion,
            created_at: self.created_at,
        }
    }
}
