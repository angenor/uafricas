use actix_web::{web, HttpRequest, FromRequest};
use sqlx::PgPool;
use std::future::Future;
use std::pin::Pin;
use uuid::Uuid;

use crate::config::JwtConfig;
use crate::errors::ApiErreur;
use crate::jwt;

/// Informations de l'utilisateur admin authentifie
/// Utilise comme extracteur Actix dans les handlers admin
#[derive(Debug, Clone)]
pub struct AdminUtilisateur {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<PermissionInfo>,
}

/// Information sur une permission
#[derive(Debug, Clone)]
pub struct PermissionInfo {
    pub slug: String,
    pub type_ressource: String,
    pub action: String,
}

impl AdminUtilisateur {
    /// Verifie si l'admin a une permission specifique
    pub fn a_permission(&self, type_ressource: &str, action: &str) -> bool {
        self.permissions.iter().any(|p| {
            (p.type_ressource == "*" || p.type_ressource == type_ressource)
                && (p.action == "*" || p.action == action)
        })
    }
}

/// Extracteur Actix : valide le JWT, verifie le role admin, charge les permissions
impl FromRequest for AdminUtilisateur {
    type Error = ApiErreur;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            // 1. Extraire le token Bearer
            let header = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .ok_or_else(|| {
                    ApiErreur::NonAutorise("Token d'authentification requis".into())
                })?;

            let token = jwt::extraire_token_du_header(header)?;

            // 2. Valider le JWT
            let jwt_config = req
                .app_data::<web::Data<JwtConfig>>()
                .ok_or_else(|| {
                    ApiErreur::BaseDeDonnees("Configuration JWT non disponible".into())
                })?;

            let claims = jwt::valider_token(token, &jwt_config.secret)?;
            let utilisateur_id = Uuid::parse_str(&claims.sub)
                .map_err(|_| ApiErreur::NonAutorise("ID utilisateur invalide dans le token".into()))?;

            // 3. Recuperer l'utilisateur et ses roles depuis la BDD
            let pool = req
                .app_data::<web::Data<PgPool>>()
                .ok_or_else(|| {
                    ApiErreur::BaseDeDonnees("Pool de connexion non disponible".into())
                })?;

            let utilisateur = sqlx::query_as::<_, (String, String, String)>(
                "SELECT nom, prenom, email FROM iam.utilisateur
                 WHERE id = $1 AND etat::text = 'actif' AND deleted_at IS NULL"
            )
            .bind(utilisateur_id)
            .fetch_optional(pool.get_ref())
            .await
            .map_err(|e| ApiErreur::BaseDeDonnees(e.to_string()))?
            .ok_or_else(|| ApiErreur::NonAutorise("Utilisateur introuvable ou inactif".into()))?;

            // 4. Recuperer les roles
            let roles: Vec<String> = sqlx::query_scalar(
                "SELECT r.slug FROM iam.role r
                 JOIN iam.utilisateur_role ur ON r.id = ur.role_id
                 WHERE ur.utilisateur_id = $1"
            )
            .bind(utilisateur_id)
            .fetch_all(pool.get_ref())
            .await
            .map_err(|e| ApiErreur::BaseDeDonnees(e.to_string()))?;

            // 5. Verifier le role admin
            let est_admin = roles.iter().any(|r| r == "admin" || r == "super_admin");
            if !est_admin {
                return Err(ApiErreur::AccesInterdit(
                    "Acces reserve aux administrateurs".into(),
                ));
            }

            // 6. Charger les permissions
            let permissions = sqlx::query_as::<_, (String, String, String)>(
                "SELECT p.slug, p.type_ressource, p.action
                 FROM iam.permission p
                 JOIN iam.role_permission rp ON p.id = rp.permission_id
                 JOIN iam.utilisateur_role ur ON rp.role_id = ur.role_id
                 WHERE ur.utilisateur_id = $1"
            )
            .bind(utilisateur_id)
            .fetch_all(pool.get_ref())
            .await
            .map_err(|e| ApiErreur::BaseDeDonnees(e.to_string()))?
            .into_iter()
            .map(|(slug, type_ressource, action)| PermissionInfo {
                slug,
                type_ressource,
                action,
            })
            .collect();

            Ok(AdminUtilisateur {
                id: utilisateur_id,
                nom: utilisateur.0,
                prenom: utilisateur.1,
                email: utilisateur.2,
                roles,
                permissions,
            })
        })
    }
}

/// Macro-helper pour verifier une permission dans un handler admin
/// Usage: verifier_permission!(admin, "utilisateur", "modifier")?;
#[macro_export]
macro_rules! verifier_permission {
    ($admin:expr, $ressource:expr, $action:expr) => {
        if !$admin.a_permission($ressource, $action) {
            return Err($crate::errors::ApiErreur::AccesInterdit(
                format!("Permission requise: {} {}", $action, $ressource),
            ));
        }
    };
}
