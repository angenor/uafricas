use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::JwtConfig;
use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::utilisateur::*;

/// Reponse API generique
#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Recuperer les roles d'un utilisateur depuis la BDD
async fn recuperer_roles(pool: &PgPool, utilisateur_id: Uuid) -> Result<Vec<String>, ApiErreur> {
    let roles = sqlx::query_scalar::<_, String>(
        "SELECT r.slug FROM iam.role r
         JOIN iam.utilisateur_role ur ON ur.role_id = r.id
         WHERE ur.utilisateur_id = $1",
    )
    .bind(utilisateur_id)
    .fetch_all(pool)
    .await?;

    Ok(roles)
}

/// Generer le slug utilisateur : prenom-nom-{8 chars uuid}
fn generer_slug(prenom: &str, nom: &str) -> String {
    let base = format!("{}-{}", prenom, nom)
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'à' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            ' ' | '\t' => '-',
            c if c.is_alphanumeric() || c == '-' => c,
            _ => '-',
        })
        .collect::<String>();

    let court_uuid = &Uuid::new_v4().to_string()[..8];
    format!("{}-{}", base, court_uuid)
}

/// Creer un refresh token en BDD et retourner le token brut
async fn creer_refresh_token(
    pool: &PgPool,
    utilisateur_id: Uuid,
    config: &JwtConfig,
) -> Result<String, ApiErreur> {
    let token_brut = jwt::generer_refresh_token();
    let token_hash = jwt::hasher_refresh_token(&token_brut);
    let expire_at = Utc::now() + Duration::days(config.refresh_expiration_days);

    sqlx::query(
        "INSERT INTO iam.refresh_token (utilisateur_id, token_hash, expire_at)
         VALUES ($1, $2, $3)",
    )
    .bind(utilisateur_id)
    .bind(&token_hash)
    .bind(expire_at)
    .execute(pool)
    .await?;

    Ok(token_brut)
}

/// POST /api/auth/inscription
/// Creer un nouveau compte utilisateur
pub async fn inscription(
    pool: web::Data<PgPool>,
    jwt_config: web::Data<JwtConfig>,
    body: web::Json<InscriptionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let req = body.into_inner();

    // Validation des champs
    if req.nom.trim().is_empty() || req.prenom.trim().is_empty() || req.email.trim().is_empty() {
        return Err(ApiErreur::Validation(
            "Nom, prenom et email sont obligatoires".into(),
        ));
    }

    if req.mot_de_passe.len() < 6 {
        return Err(ApiErreur::Validation(
            "Le mot de passe doit contenir au moins 6 caracteres".into(),
        ));
    }

    if req.mot_de_passe != req.confirmation_mot_de_passe {
        return Err(ApiErreur::Validation(
            "Les mots de passe ne correspondent pas".into(),
        ));
    }

    // Verifier unicite email
    let existe = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur WHERE email = $1 AND deleted_at IS NULL)",
    )
    .bind(&req.email)
    .fetch_one(pool.get_ref())
    .await?;

    if existe {
        return Err(ApiErreur::Conflit(
            "Un compte avec cet email existe deja".into(),
        ));
    }

    // Hasher le mot de passe
    let hash = bcrypt::hash(&req.mot_de_passe, 12)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur hashage mot de passe: {}", e)))?;

    let slug = generer_slug(&req.prenom, &req.nom);

    // Inserer l'utilisateur (etat = 'actif' car pas de verification email)
    let utilisateur = sqlx::query_as::<_, Utilisateur>(&format!(
        "INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, slug, etat)
         VALUES ($1, $2, $3, $4, $5, 'actif'::iam.etat_utilisateur)
         RETURNING {}",
        UTILISATEUR_COLONNES
    ))
    .bind(req.nom.trim())
    .bind(req.prenom.trim())
    .bind(req.email.trim())
    .bind(&hash)
    .bind(&slug)
    .fetch_one(pool.get_ref())
    .await?;

    // Assigner le role 'utilisateur' par defaut
    sqlx::query(
        "INSERT INTO iam.utilisateur_role (utilisateur_id, role_id)
         SELECT $1, id FROM iam.role WHERE slug = 'utilisateur'",
    )
    .bind(utilisateur.id)
    .execute(pool.get_ref())
    .await?;

    // Recuperer les roles
    let roles = recuperer_roles(pool.get_ref(), utilisateur.id).await?;

    // Generer les tokens
    let access_token = jwt::generer_access_token(utilisateur.id, jwt_config.get_ref())?;
    let refresh_token = creer_refresh_token(pool.get_ref(), utilisateur.id, jwt_config.get_ref()).await?;

    log::info!("Nouvel utilisateur inscrit: {} ({})", utilisateur.email, utilisateur.id);

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(AuthResponse {
            utilisateur: utilisateur.to_response(roles),
            access_token,
            refresh_token,
        }),
        error: None,
    }))
}

/// POST /api/auth/connexion
/// Connecter un utilisateur existant
pub async fn connexion(
    pool: web::Data<PgPool>,
    jwt_config: web::Data<JwtConfig>,
    body: web::Json<ConnexionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let req = body.into_inner();

    // Chercher l'utilisateur par email
    let utilisateur = sqlx::query_as::<_, Utilisateur>(&format!(
        "SELECT {} FROM iam.utilisateur WHERE email = $1 AND deleted_at IS NULL",
        UTILISATEUR_COLONNES
    ))
    .bind(&req.email)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonAutorise("Email ou mot de passe incorrect".into()))?;

    // Verifier le mot de passe
    let valide = bcrypt::verify(&req.mot_de_passe, &utilisateur.mot_de_passe_hash)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Erreur verification mot de passe: {}", e)))?;

    if !valide {
        return Err(ApiErreur::NonAutorise(
            "Email ou mot de passe incorrect".into(),
        ));
    }

    // Verifier l'etat du compte
    match utilisateur.etat.as_str() {
        "actif" | "en_attente" => {}
        "suspendu" => {
            return Err(ApiErreur::NonAutorise(
                "Votre compte est suspendu. Contactez le support.".into(),
            ));
        }
        "bloque" => {
            return Err(ApiErreur::NonAutorise(
                "Votre compte est bloque. Contactez le support.".into(),
            ));
        }
        _ => {
            return Err(ApiErreur::NonAutorise(
                "Compte non disponible".into(),
            ));
        }
    }

    // Mettre a jour la derniere connexion
    sqlx::query("UPDATE iam.utilisateur SET derniere_connexion = NOW() WHERE id = $1")
        .bind(utilisateur.id)
        .execute(pool.get_ref())
        .await?;

    // Recuperer les roles
    let roles = recuperer_roles(pool.get_ref(), utilisateur.id).await?;

    // Generer les tokens
    let access_token = jwt::generer_access_token(utilisateur.id, jwt_config.get_ref())?;
    let refresh_token = creer_refresh_token(pool.get_ref(), utilisateur.id, jwt_config.get_ref()).await?;

    log::info!("Utilisateur connecte: {} ({})", utilisateur.email, utilisateur.id);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AuthResponse {
            utilisateur: utilisateur.to_response(roles),
            access_token,
            refresh_token,
        }),
        error: None,
    }))
}

/// POST /api/auth/deconnexion
/// Revoquer le refresh token
pub async fn deconnexion(
    pool: web::Data<PgPool>,
    body: web::Json<RafraichirTokenRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let token_hash = jwt::hasher_refresh_token(&body.refresh_token);

    // Revoquer le token (silencieux si non trouve)
    sqlx::query("UPDATE iam.refresh_token SET revoque = TRUE WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(pool.get_ref())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// GET /api/auth/moi
/// Recuperer le profil de l'utilisateur connecte
pub async fn moi(
    pool: web::Data<PgPool>,
    jwt_config: web::Data<JwtConfig>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    // Extraire le token du header Authorization
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ApiErreur::NonAutorise("Header Authorization manquant".into()))?;

    let token = jwt::extraire_token_du_header(header)?;
    let claims = jwt::valider_token(token, &jwt_config.secret)?;

    let utilisateur_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiErreur::NonAutorise("Token invalide: sub n'est pas un UUID".into()))?;

    // Recuperer l'utilisateur
    let utilisateur = sqlx::query_as::<_, Utilisateur>(&format!(
        "SELECT {} FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL",
        UTILISATEUR_COLONNES
    ))
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonAutorise("Utilisateur non trouve".into()))?;

    let roles = recuperer_roles(pool.get_ref(), utilisateur.id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(utilisateur.to_response(roles)),
        error: None,
    }))
}

/// POST /api/auth/rafraichir
/// Rafraichir les tokens (rotation du refresh token)
pub async fn rafraichir_token(
    pool: web::Data<PgPool>,
    jwt_config: web::Data<JwtConfig>,
    body: web::Json<RafraichirTokenRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let token_hash = jwt::hasher_refresh_token(&body.refresh_token);

    // Chercher le refresh token en BDD
    let row = sqlx::query_as::<_, (Uuid, Uuid, chrono::DateTime<Utc>, bool)>(
        "SELECT id, utilisateur_id, expire_at, revoque
         FROM iam.refresh_token
         WHERE token_hash = $1",
    )
    .bind(&token_hash)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonAutorise("Refresh token invalide".into()))?;

    let (token_id, utilisateur_id, expire_at, revoque) = row;

    if revoque {
        return Err(ApiErreur::NonAutorise("Refresh token revoque".into()));
    }

    if expire_at < Utc::now() {
        return Err(ApiErreur::NonAutorise("Refresh token expire".into()));
    }

    // Revoquer l'ancien refresh token
    sqlx::query("UPDATE iam.refresh_token SET revoque = TRUE WHERE id = $1")
        .bind(token_id)
        .execute(pool.get_ref())
        .await?;

    // Recuperer l'utilisateur
    let utilisateur = sqlx::query_as::<_, Utilisateur>(&format!(
        "SELECT {} FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL",
        UTILISATEUR_COLONNES
    ))
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonAutorise("Utilisateur non trouve".into()))?;

    let roles = recuperer_roles(pool.get_ref(), utilisateur.id).await?;

    // Generer de nouveaux tokens
    let access_token = jwt::generer_access_token(utilisateur.id, jwt_config.get_ref())?;
    let refresh_token = creer_refresh_token(pool.get_ref(), utilisateur.id, jwt_config.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(AuthResponse {
            utilisateur: utilisateur.to_response(roles),
            access_token,
            refresh_token,
        }),
        error: None,
    }))
}
