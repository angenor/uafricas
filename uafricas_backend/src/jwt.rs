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

// ── Jetons d'accès salle privée Afrolang (refonte 2026-04) ───────────────
//
// Après vérification du code secret, l'endpoint `verifier-code` émet un
// jeton JWT court qui évite à l'utilisateur de re-saisir le code pendant
// la session (A2 spec). Le jeton est validé lors du démarrage/jointure de
// session via le header `X-Afrolang-Acces-Jeton`.

/// Claims du jeton d'accès salle privée
#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimsAccesSallePrivee {
    /// Identifiant de la salle privée (UUID en String)
    pub salle_privee_id: String,
    /// Identifiant de l'utilisateur ayant saisi le bon code
    pub utilisateur_id: String,
    /// Timestamp d'expiration (Unix epoch)
    pub exp: usize,
    /// Timestamp de création (Unix epoch)
    pub iat: usize,
}

/// Génère un jeton d'accès éphémère pour une salle privée.
///
/// `ttl_secondes` est la durée de validité en secondes (typiquement 4 h).
/// Utilise `JWT_SECRET` (variable d'environnement) et l'algorithme HS256.
pub fn creer_acces_jeton_salle_privee(
    salle_privee_id: Uuid,
    utilisateur_id: Uuid,
    ttl_secondes: i64,
) -> Result<(String, chrono::DateTime<Utc>), ApiErreur> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        ApiErreur::BaseDeDonnees("JWT_SECRET non configuré".into())
    })?;
    let maintenant = Utc::now();
    let expiration = maintenant + Duration::seconds(ttl_secondes);

    let claims = ClaimsAccesSallePrivee {
        salle_privee_id: salle_privee_id.to_string(),
        utilisateur_id: utilisateur_id.to_string(),
        exp: expiration.timestamp() as usize,
        iat: maintenant.timestamp() as usize,
    };

    let jeton = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| {
        ApiErreur::BaseDeDonnees(format!("Erreur génération jeton accès salle privée : {}", e))
    })?;

    Ok((jeton, expiration))
}

/// Valide un jeton d'accès salle privée.
///
/// Vérifie la signature, l'expiration et la cohérence avec la salle privée
/// et l'utilisateur cibles. Retourne une erreur `NonAutorise` si le jeton
/// est invalide, expiré ou rattaché à une autre salle/utilisateur.
pub fn valider_acces_jeton_salle_privee(
    jeton: &str,
    salle_privee_id: Uuid,
    utilisateur_id: Uuid,
) -> Result<(), ApiErreur> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| {
        ApiErreur::BaseDeDonnees("JWT_SECRET non configuré".into())
    })?;

    let claims = decode::<ClaimsAccesSallePrivee>(
        jeton,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| ApiErreur::NonAutorise(format!("Jeton d'accès invalide : {}", e)))?;

    let attendu_salle = salle_privee_id.to_string();
    let attendu_user = utilisateur_id.to_string();
    if claims.salle_privee_id != attendu_salle {
        return Err(ApiErreur::NonAutorise(
            "Jeton d'accès rattaché à une autre salle privée".into(),
        ));
    }
    if claims.utilisateur_id != attendu_user {
        return Err(ApiErreur::NonAutorise(
            "Jeton d'accès rattaché à un autre utilisateur".into(),
        ));
    }
    Ok(())
}
