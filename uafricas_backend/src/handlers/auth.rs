use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::{JwtConfig, SmtpConfig};
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

/// Reponse apres inscription (sans tokens car email non verifie)
#[derive(Serialize)]
struct InscriptionResponse {
    message: String,
    email: String,
}

/// Requete pour verifier l'email
#[derive(Debug, Deserialize)]
pub struct VerifierEmailRequest {
    pub token: String,
}

/// Requete pour renvoyer l'email de verification
#[derive(Debug, Deserialize)]
pub struct RenvoyerVerificationRequest {
    pub email: String,
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

/// Creer un token de verification email en BDD et retourner le token brut
async fn creer_token_verification(
    pool: &PgPool,
    utilisateur_id: Uuid,
    expiration_hours: i64,
) -> Result<String, ApiErreur> {
    // Invalider les anciens tokens non utilises
    sqlx::query(
        "UPDATE iam.token_verification_email SET utilise = TRUE
         WHERE utilisateur_id = $1 AND utilise = FALSE",
    )
    .bind(utilisateur_id)
    .execute(pool)
    .await?;

    // Generer un nouveau token
    let token_brut = jwt::generer_refresh_token();
    let token_hash = jwt::hasher_refresh_token(&token_brut);
    let expire_at = Utc::now() + Duration::hours(expiration_hours);

    sqlx::query(
        "INSERT INTO iam.token_verification_email (utilisateur_id, token_hash, expire_at)
         VALUES ($1, $2, $3)",
    )
    .bind(utilisateur_id)
    .bind(&token_hash)
    .bind(expire_at)
    .execute(pool)
    .await?;

    Ok(token_brut)
}

/// Envoyer l'email de verification en arriere-plan
fn envoyer_verification_async(
    smtp_config: SmtpConfig,
    email: String,
    prenom: String,
    token: String,
) {
    let lien = format!(
        "{}/verification-email?token={}",
        smtp_config.frontend_url, token
    );

    tokio::spawn(async move {
        if let Err(e) =
            crate::email::envoyer_email_verification(&smtp_config, &email, &prenom, &lien).await
        {
            log::error!("Echec envoi email de verification a {}: {}", email, e);
        }
    });
}

/// POST /api/auth/inscription
/// Creer un nouveau compte utilisateur (etat = en_attente, email non verifie)
pub async fn inscription(
    pool: web::Data<PgPool>,
    smtp_config: web::Data<SmtpConfig>,
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
        "SELECT EXISTS(SELECT 1 FROM iam.utilisateur WHERE LOWER(email) = LOWER($1) AND deleted_at IS NULL)",
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

    // Inserer l'utilisateur (etat = 'en_attente', email_verifie = false)
    let utilisateur = sqlx::query_as::<_, Utilisateur>(&format!(
        "INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, slug, etat)
         VALUES ($1, $2, $3, $4, $5, 'en_attente'::iam.etat_utilisateur)
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

    // Generer le token de verification et envoyer l'email
    let token_brut = creer_token_verification(
        pool.get_ref(),
        utilisateur.id,
        smtp_config.verification_expiration_hours,
    )
    .await?;

    envoyer_verification_async(
        smtp_config.get_ref().clone(),
        utilisateur.email.clone(),
        utilisateur.prenom.clone(),
        token_brut,
    );

    log::info!(
        "Nouvel utilisateur inscrit (en attente de verification): {} ({})",
        utilisateur.email,
        utilisateur.id
    );

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(InscriptionResponse {
            message: "Compte cree avec succes. Verifiez votre email pour activer votre compte."
                .into(),
            email: utilisateur.email,
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
        "SELECT {} FROM iam.utilisateur WHERE LOWER(email) = LOWER($1) AND deleted_at IS NULL",
        UTILISATEUR_COLONNES
    ))
    .bind(&req.email)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonAutorise("Email ou mot de passe incorrect".into()))?;

    // Verifier le mot de passe
    let valide = bcrypt::verify(&req.mot_de_passe, &utilisateur.mot_de_passe_hash)
        .map_err(|e| {
            ApiErreur::BaseDeDonnees(format!("Erreur verification mot de passe: {}", e))
        })?;

    if !valide {
        return Err(ApiErreur::NonAutorise(
            "Email ou mot de passe incorrect".into(),
        ));
    }

    // Verifier l'etat du compte
    match utilisateur.etat.as_str() {
        "actif" => {}
        "en_attente" => {
            return Err(ApiErreur::NonAutorise(
                "Veuillez verifier votre adresse email avant de vous connecter. Consultez votre boite de reception.".into(),
            ));
        }
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
            return Err(ApiErreur::NonAutorise("Compte non disponible".into()));
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
    let refresh_token =
        creer_refresh_token(pool.get_ref(), utilisateur.id, jwt_config.get_ref()).await?;

    log::info!(
        "Utilisateur connecte: {} ({})",
        utilisateur.email,
        utilisateur.id
    );

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
    let refresh_token =
        creer_refresh_token(pool.get_ref(), utilisateur.id, jwt_config.get_ref()).await?;

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

/// POST /api/auth/verifier-email
/// Verifier l'adresse email avec le token recu par mail
pub async fn verifier_email(
    pool: web::Data<PgPool>,
    jwt_config: web::Data<JwtConfig>,
    body: web::Json<VerifierEmailRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let token_hash = jwt::hasher_refresh_token(&body.token);

    // Chercher le token de verification en BDD
    let row = sqlx::query_as::<_, (Uuid, Uuid, chrono::DateTime<Utc>, bool)>(
        "SELECT id, utilisateur_id, expire_at, utilise
         FROM iam.token_verification_email
         WHERE token_hash = $1",
    )
    .bind(&token_hash)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| {
        ApiErreur::Validation("Token de verification invalide ou deja utilise".into())
    })?;

    let (token_id, utilisateur_id, expire_at, utilise) = row;

    if utilise {
        return Err(ApiErreur::Validation(
            "Ce lien de verification a deja ete utilise".into(),
        ));
    }

    if expire_at < Utc::now() {
        return Err(ApiErreur::Validation(
            "Ce lien de verification a expire. Demandez un nouveau lien.".into(),
        ));
    }

    // Marquer le token comme utilise
    sqlx::query("UPDATE iam.token_verification_email SET utilise = TRUE WHERE id = $1")
        .bind(token_id)
        .execute(pool.get_ref())
        .await?;

    // Activer le compte utilisateur
    sqlx::query(
        "UPDATE iam.utilisateur
         SET email_verifie = TRUE, etat = 'actif'::iam.etat_utilisateur, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    // Recuperer l'utilisateur mis a jour
    let utilisateur = sqlx::query_as::<_, Utilisateur>(&format!(
        "SELECT {} FROM iam.utilisateur WHERE id = $1 AND deleted_at IS NULL",
        UTILISATEUR_COLONNES
    ))
    .bind(utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;

    let roles = recuperer_roles(pool.get_ref(), utilisateur.id).await?;

    // Generer les tokens de session (l'utilisateur est maintenant connecte)
    let access_token = jwt::generer_access_token(utilisateur.id, jwt_config.get_ref())?;
    let refresh_token =
        creer_refresh_token(pool.get_ref(), utilisateur.id, jwt_config.get_ref()).await?;

    log::info!(
        "Email verifie pour: {} ({})",
        utilisateur.email,
        utilisateur.id
    );

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

/// POST /api/auth/renvoyer-verification
/// Renvoyer l'email de verification
pub async fn renvoyer_verification(
    pool: web::Data<PgPool>,
    smtp_config: web::Data<SmtpConfig>,
    body: web::Json<RenvoyerVerificationRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let message_generique = "Si un compte avec cet email existe et n'est pas encore verifie, un nouveau lien vous sera envoye.";

    // Chercher l'utilisateur
    let utilisateur = sqlx::query_as::<_, Utilisateur>(&format!(
        "SELECT {} FROM iam.utilisateur WHERE LOWER(email) = LOWER($1) AND deleted_at IS NULL",
        UTILISATEUR_COLONNES
    ))
    .bind(&body.email)
    .fetch_optional(pool.get_ref())
    .await?;

    // Reponse generique pour ne pas reveler si l'email existe
    let utilisateur = match utilisateur {
        Some(u) => u,
        None => {
            return Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(serde_json::json!({ "message": message_generique })),
                error: None,
            }));
        }
    };

    // Verifier que le compte est bien en attente de verification
    if utilisateur.email_verifie || utilisateur.etat != "en_attente" {
        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "message": message_generique })),
            error: None,
        }));
    }

    // Generer un nouveau token et envoyer l'email
    let token_brut = creer_token_verification(
        pool.get_ref(),
        utilisateur.id,
        smtp_config.verification_expiration_hours,
    )
    .await?;

    envoyer_verification_async(
        smtp_config.get_ref().clone(),
        utilisateur.email.clone(),
        utilisateur.prenom.clone(),
        token_brut,
    );

    log::info!("Email de verification renvoye a: {}", utilisateur.email);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "message": message_generique })),
        error: None,
    }))
}
