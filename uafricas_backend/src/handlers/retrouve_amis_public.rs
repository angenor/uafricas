use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::errors::ApiErreur;
use crate::models::retrouve_amis::*;
use crate::ApiResponse;

// ══════════════════════════════════════════════════════════════
// Handlers publics — Avis de recherche (sans authentification)
// ══════════════════════════════════════════════════════════════
//
// Ces handlers sont enregistres HORS du scope JWT dans routes.rs.
// Ils ne necessitent pas de token d'authentification.
//
// Endpoints :
// - GET  /api/retrouve-amis/public/{slug}          → detail_avis_public
// - GET  /api/retrouve-amis/public/rechercher       → rechercher_avis_publics
// - POST /api/retrouve-amis/public/{slug}/partage   → incrementer_partage

/// GET /api/retrouve-amis/public/{slug}
/// Recuperer les details d'un avis public par son slug
/// Retourne le detail complet si actif, un message d'etat si cloture/suspendu, 404 si depublie
pub async fn detail_avis_public(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = path.into_inner();

    // Chercher l'avis par slug (inclut tous les etats pour affichage conditionnel)
    let row: Option<AvisPublicDetailRow> = sqlx::query_as(&format!(
        "SELECT {}
         FROM retrouve_amis.avis_recherche a
         JOIN iam.utilisateur u ON u.id = a.auteur_id
         LEFT JOIN shared.pays p ON p.id = a.pays_id
         WHERE a.slug = $1 AND a.deleted_at IS NULL",
        AVIS_PUBLIC_DETAIL_COLONNES
    ))
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?;

    let row = match row {
        Some(r) => r,
        None => {
            return Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
                success: false,
                data: None,
                error: Some("Avis non disponible.".to_string()),
            }));
        }
    };

    // Si l'avis est depublie (est_public = FALSE), retourner 404
    let est_public: (bool,) = sqlx::query_as(
        "SELECT est_public FROM retrouve_amis.avis_recherche WHERE slug = $1"
    )
    .bind(&slug)
    .fetch_one(pool.get_ref())
    .await?;

    if !est_public.0 {
        return Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Avis non disponible.".to_string()),
        }));
    }

    // Si l'avis n'est pas actif (cloture ou suspendu), retourner un message d'etat
    if row.etat != "actif" {
        let message = match row.etat.as_str() {
            "cloture" => "Cette personne a été retrouvée !",
            "suspendu" => "Cet avis a été temporairement retiré.",
            _ => "Cet avis n'est plus disponible.",
        };

        return Ok(HttpResponse::Ok()
            .insert_header(("X-Robots-Tag", "noindex, nofollow"))
            .json(ApiResponse {
                success: true,
                data: Some(AvisPublicEtatResponse {
                    slug: slug.clone(),
                    etat: row.etat.clone(),
                    message: message.to_string(),
                }),
                error: None,
            }));
    }

    // Avis actif : retourner le detail complet
    let detail = row.to_detail_response();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(detail),
        error: None,
    }))
}
