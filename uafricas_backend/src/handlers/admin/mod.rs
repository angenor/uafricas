pub mod utilisateurs;
pub mod organisations;
pub mod partenariats;
pub mod roles;
pub mod pays;
pub mod domaines;
pub mod categories;
pub mod tags;
pub mod medias;
pub mod specialites;
pub mod programmes;
pub mod candidatures;
pub mod annonces;
pub mod annonces_favoris;
pub mod innovations;
pub mod projets_admin;
pub mod africantives_admin;
pub mod centres_culturels;
pub mod programmations;
pub mod codimoi_admin;
pub mod salles;
pub mod salles_privees;
pub mod sessions_afrolang;
pub mod moderateurs_afrolang;
pub mod gouvernance;
pub mod radio_tele;
pub mod retrouve_amis;
pub mod evenements;
pub mod mooc;
pub mod livres;
pub mod audit;
pub mod profils_pays;
pub mod dashboard;
pub mod vidafrica;

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
