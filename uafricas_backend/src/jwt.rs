use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::config::JwtConfig;
use crate::errors::ApiErreur;

/// Claims contenus dans le JWT access token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Identifiant utilisateur (UUID sous forme de String)
    pub sub: String,
    /// Timestamp d'expiration
    pub exp: usize,
    /// Timestamp de creation
    pub iat: usize,
}

/// Generer un access token JWT
pub fn generer_access_token(
    utilisateur_id: Uuid,
    config: &JwtConfig,
) -> Result<String, ApiErreur> {
    let maintenant = Utc::now();
    let expiration = maintenant + Duration::minutes(config.expiration_minutes);

    let claims = Claims {
        sub: utilisateur_id.to_string(),
        exp: expiration.timestamp() as usize,
        iat: maintenant.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur generation JWT: {}", e)))
}

/// Valider un access token JWT et retourner les claims
pub fn valider_token(token: &str, secret: &str) -> Result<Claims, ApiErreur> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| ApiErreur::NonAutorise(format!("Token invalide: {}", e)))
}

/// Generer un refresh token (chaine aleatoire hex de 64 caracteres)
pub fn generer_refresh_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.r#gen();
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}

/// Hasher un refresh token avec SHA-256 avant stockage en BDD
pub fn hasher_refresh_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Extraire le Bearer token du header Authorization
pub fn extraire_token_du_header(header_value: &str) -> Result<&str, ApiErreur> {
    header_value
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            ApiErreur::NonAutorise(
                "Format Authorization invalide, attendu: Bearer <token>".into(),
            )
        })
}
