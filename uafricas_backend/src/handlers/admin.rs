use actix_web::HttpResponse;
use serde::Serialize;

use crate::middleware::admin::AdminUtilisateur;
use crate::ApiResponse;

/// Reponse pour GET /api/admin/me
#[derive(Debug, Serialize)]
pub struct AdminMeResponse {
    pub id: String,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<AdminPermissionResponse>,
}

#[derive(Debug, Serialize)]
pub struct AdminPermissionResponse {
    pub slug: String,
    pub type_ressource: String,
    pub action: String,
}

/// GET /api/admin/me
/// Retourne l'utilisateur admin courant avec ses roles et permissions
pub async fn admin_me(
    admin: AdminUtilisateur,
) -> Result<HttpResponse, crate::errors::ApiErreur> {
    let permissions = admin
        .permissions
        .iter()
        .map(|p| AdminPermissionResponse {
            slug: p.slug.clone(),
            type_ressource: p.type_ressource.clone(),
            action: p.action.clone(),
        })
        .collect();

    let reponse = AdminMeResponse {
        id: admin.id.to_string(),
        nom: admin.nom,
        prenom: admin.prenom,
        email: admin.email,
        roles: admin.roles,
        permissions,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}
