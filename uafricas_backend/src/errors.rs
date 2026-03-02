use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

/// Reponse d'erreur API standardisee
#[derive(Debug, Serialize)]
pub struct ApiErreurResponse {
    pub success: bool,
    pub data: Option<()>,
    pub error: Option<String>,
}

/// Erreurs applicatives de l'API
#[derive(Debug)]
pub enum ApiErreur {
    /// Ressource non trouvee (404)
    NonTrouve(String),
    /// Erreur de base de donnees (500)
    BaseDeDonnees(String),
    /// Erreur de validation des donnees (400)
    Validation(String),
    /// Erreur d'upload de fichier (400)
    Upload(String),
    /// Non autorise - identifiants invalides ou token invalide (401)
    NonAutorise(String),
    /// Acces interdit - authentifie mais pas les permissions requises (403)
    AccesInterdit(String),
    /// Conflit - par exemple email deja utilise (409)
    Conflit(String),
    /// Limite atteinte - trop de requetes (429)
    LimiteAtteinte(String),
}

impl fmt::Display for ApiErreur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiErreur::NonTrouve(msg) => write!(f, "Non trouve: {}", msg),
            ApiErreur::BaseDeDonnees(msg) => write!(f, "Erreur base de donnees: {}", msg),
            ApiErreur::Validation(msg) => write!(f, "Validation: {}", msg),
            ApiErreur::Upload(msg) => write!(f, "Upload: {}", msg),
            ApiErreur::NonAutorise(msg) => write!(f, "Non autorise: {}", msg),
            ApiErreur::AccesInterdit(msg) => write!(f, "Acces interdit: {}", msg),
            ApiErreur::Conflit(msg) => write!(f, "Conflit: {}", msg),
            ApiErreur::LimiteAtteinte(msg) => write!(f, "Limite atteinte: {}", msg),
        }
    }
}

impl ResponseError for ApiErreur {
    fn error_response(&self) -> HttpResponse {
        let reponse = ApiErreurResponse {
            success: false,
            data: None,
            error: Some(self.to_string()),
        };

        match self {
            ApiErreur::NonTrouve(_) => HttpResponse::NotFound().json(reponse),
            ApiErreur::Validation(_) | ApiErreur::Upload(_) => {
                HttpResponse::BadRequest().json(reponse)
            }
            ApiErreur::BaseDeDonnees(_) => {
                HttpResponse::InternalServerError().json(reponse)
            }
            ApiErreur::NonAutorise(_) => HttpResponse::Unauthorized().json(reponse),
            ApiErreur::AccesInterdit(_) => HttpResponse::Forbidden().json(reponse),
            ApiErreur::Conflit(_) => HttpResponse::Conflict().json(reponse),
            ApiErreur::LimiteAtteinte(_) => HttpResponse::TooManyRequests().json(reponse),
        }
    }
}

impl From<sqlx::Error> for ApiErreur {
    fn from(err: sqlx::Error) -> Self {
        log::error!("Erreur SQLx: {:?}", err);
        ApiErreur::BaseDeDonnees(err.to_string())
    }
}
