use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::centre_culturel::{
    CentreCulturel, CentreCulturelQueryParams, CentreCulturelResponse,
    ProgrammationCentre, ProgrammationDetailResponse,
    CENTRE_CULTUREL_COLONNES, PROGRAMMATION_COLONNES,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// GET /api/centres-culturels — Lister les centres culturels actifs
// ──────────────────────────────────────────────────────────────
pub async fn lister_centres(
    pool: web::Data<PgPool>,
    params: web::Query<CentreCulturelQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let centres: Vec<CentreCulturel> = if let Some(ref recherche) = params.recherche {
        let terme = format!("%{}%", recherche.to_lowercase());

        let query = format!(
            "SELECT {} FROM culture.centre_culturel
             WHERE actif = TRUE
               AND (LOWER(nom) LIKE $1 OR LOWER(ville) LIKE $1 OR LOWER(adresse) LIKE $1)
             ORDER BY nom ASC",
            CENTRE_CULTUREL_COLONNES
        );

        sqlx::query_as::<_, CentreCulturel>(&query)
            .bind(&terme)
            .fetch_all(pool.get_ref())
            .await?
    } else {
        let query = format!(
            "SELECT {} FROM culture.centre_culturel
             WHERE actif = TRUE
             ORDER BY nom ASC",
            CENTRE_CULTUREL_COLONNES
        );

        sqlx::query_as::<_, CentreCulturel>(&query)
            .fetch_all(pool.get_ref())
            .await?
    };

    // Recuperer le nombre de programmations pour chaque centre
    let mut reponse_centres: Vec<CentreCulturelResponse> = Vec::with_capacity(centres.len());

    for centre in &centres {
        let (count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM culture.programmation_centre WHERE centre_culturel_id = $1",
        )
        .bind(centre.id)
        .fetch_one(pool.get_ref())
        .await?;

        reponse_centres.push(CentreCulturelResponse {
            id: centre.id,
            nom: centre.nom.clone(),
            slug: centre.slug.clone(),
            description: centre.description.clone(),
            image_couverture_url: centre.image_couverture_url.clone(),
            ville: centre.ville.clone(),
            adresse: centre.adresse.clone(),
            latitude: centre.latitude,
            longitude: centre.longitude,
            nombre_programmations: count,
            created_at: centre.created_at,
            updated_at: centre.updated_at,
        });
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse_centres),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/centres-culturels/{id} — Obtenir un centre avec ses programmations
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_centre(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM culture.centre_culturel WHERE id = $1 AND actif = TRUE",
        CENTRE_CULTUREL_COLONNES
    );

    let centre = sqlx::query_as::<_, CentreCulturel>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Centre culturel avec id {} non trouve", id)))?;

    // Recuperer les programmations du centre
    let query_prog = format!(
        "SELECT {} FROM culture.programmation_centre
         WHERE centre_culturel_id = $1
         ORDER BY date_heure_debut ASC",
        PROGRAMMATION_COLONNES
    );

    let programmations = sqlx::query_as::<_, ProgrammationCentre>(&query_prog)
        .bind(id)
        .fetch_all(pool.get_ref())
        .await?;

    let prog_responses: Vec<_> = programmations.iter().map(|p| p.to_response()).collect();
    let reponse = centre.to_detail_response(prog_responses);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/centres-culturels/{centre_id}/programmations/{id}
// Obtenir le detail d'une programmation
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_programmation(
    pool: web::Data<PgPool>,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (centre_id, programmation_id) = chemin.into_inner();

    // Recuperer le centre
    let query_centre = format!(
        "SELECT {} FROM culture.centre_culturel WHERE id = $1 AND actif = TRUE",
        CENTRE_CULTUREL_COLONNES
    );

    let centre = sqlx::query_as::<_, CentreCulturel>(&query_centre)
        .bind(centre_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Centre culturel avec id {} non trouve", centre_id)))?;

    // Recuperer la programmation
    let query_prog = format!(
        "SELECT {} FROM culture.programmation_centre
         WHERE id = $1 AND centre_culturel_id = $2",
        PROGRAMMATION_COLONNES
    );

    let programmation = sqlx::query_as::<_, ProgrammationCentre>(&query_prog)
        .bind(programmation_id)
        .bind(centre_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| {
            ApiErreur::NonTrouve(format!(
                "Programmation avec id {} non trouvee dans le centre {}",
                programmation_id, centre_id
            ))
        })?;

    let reponse = ProgrammationDetailResponse {
        programmation: programmation.to_response(),
        centre: centre.to_info_response(),
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}
