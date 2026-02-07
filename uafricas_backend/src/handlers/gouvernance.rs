use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::errors::ApiErreur;
use crate::models::gouvernance::{
    ContributionListeResponse, ContributionQueryParams, ContributionRow, GouvernanceStats,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// GET /api/gouvernance/stats — Statistiques de gouvernance
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_stats(pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let (factcheck,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM governance.factcheck
         WHERE deleted_at IS NULL AND etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let (badhabits,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM governance.bad_habit
         WHERE deleted_at IS NULL AND etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let (ideaforces,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM governance.idea_force
         WHERE deleted_at IS NULL AND etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let (total_likes,): (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(nombre_likes), 0)::BIGINT
         FROM governance.factcheck
         WHERE deleted_at IS NULL AND etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let stats = GouvernanceStats {
        total: factcheck + badhabits + ideaforces,
        factcheck,
        badhabits,
        ideaforces,
        total_likes,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(stats),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// GET /api/gouvernance/contributions — Liste paginee des contributions
// ──────────────────────────────────────────────────────────────
pub async fn lister_contributions(
    pool: web::Data<PgPool>,
    params: web::Query<ContributionQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Filtre optionnel par type
    let filtre_type = params.type_contribution.as_deref();

    let query = build_contributions_query(filtre_type);

    let rows = sqlx::query_as::<_, ContributionRow>(&query)
        .bind(par_page)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let total = rows.first().map(|r| r.total_count).unwrap_or(0);
    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    let contributions = rows.iter().map(|r| r.to_response()).collect();

    let reponse = ContributionListeResponse {
        contributions,
        total,
        page,
        par_page,
        total_pages,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

/// Construit la requete UNION ALL pour les 3 types de contributions
fn build_contributions_query(filtre_type: Option<&str>) -> String {
    let mut sous_requetes = Vec::new();

    let inclure_factcheck = filtre_type.is_none() || filtre_type == Some("factcheck");
    let inclure_badhabits = filtre_type.is_none() || filtre_type == Some("badhabits");
    let inclure_ideaforces = filtre_type.is_none() || filtre_type == Some("ideaforces");

    if inclure_factcheck {
        sous_requetes.push(
            "SELECT f.id, 'factcheck'::TEXT AS type_contribution,
                    LEFT(f.contenu, 200) AS titre,
                    f.contenu AS description,
                    f.nombre_likes, 0 AS nombre_soutiens,
                    f.etat AS statut,
                    f.cree_par, f.pays_id, f.created_at,
                    NULL::VARCHAR AS region, NULL::VARCHAR AS ville
             FROM governance.factcheck f
             WHERE f.deleted_at IS NULL AND f.etat = 'publie'"
                .to_string(),
        );
    }

    if inclure_badhabits {
        sous_requetes.push(
            "SELECT b.id, 'badhabits'::TEXT AS type_contribution,
                    b.titre,
                    b.description_generale AS description,
                    0 AS nombre_likes, b.nombre_soutiens,
                    b.etat AS statut,
                    b.cree_par, b.pays_id, b.created_at,
                    b.region, b.ville_quartier_zone AS ville
             FROM governance.bad_habit b
             WHERE b.deleted_at IS NULL AND b.etat = 'publie'"
                .to_string(),
        );
    }

    if inclure_ideaforces {
        sous_requetes.push(
            "SELECT i.id, 'ideaforces'::TEXT AS type_contribution,
                    i.titre,
                    i.description_generale AS description,
                    0 AS nombre_likes, i.nombre_soutiens,
                    i.etat AS statut,
                    i.cree_par, i.pays_id, i.created_at,
                    i.region, i.ville_quartier_zone AS ville
             FROM governance.idea_force i
             WHERE i.deleted_at IS NULL AND i.etat = 'publie'"
                .to_string(),
        );
    }

    // Si aucun type valide, retourner une requete vide
    if sous_requetes.is_empty() {
        return "SELECT NULL::UUID AS id, ''::TEXT AS type_contribution,
                       ''::TEXT AS titre, ''::TEXT AS description,
                       0 AS nombre_likes, 0 AS nombre_soutiens,
                       ''::TEXT AS statut, NOW() AS created_at,
                       NULL::UUID AS auteur_id,
                       NULL::VARCHAR AS auteur_prenom,
                       NULL::VARCHAR AS auteur_nom,
                       NULL::VARCHAR AS auteur_photo_url,
                       NULL::VARCHAR AS pays_nom,
                       NULL::VARCHAR AS region,
                       NULL::VARCHAR AS ville,
                       0::BIGINT AS total_count
                WHERE FALSE"
            .to_string();
    }

    let union = sous_requetes.join(" UNION ALL ");

    format!(
        "SELECT c.id, c.type_contribution, c.titre, c.description,
                c.nombre_likes, c.nombre_soutiens, c.statut, c.created_at,
                u.id AS auteur_id,
                u.prenom AS auteur_prenom,
                u.nom AS auteur_nom,
                u.photo_url AS auteur_photo_url,
                p.nom AS pays_nom,
                c.region, c.ville,
                COUNT(*) OVER() AS total_count
         FROM ({}) c
         LEFT JOIN iam.utilisateur u ON c.cree_par = u.id
         LEFT JOIN shared.pays p ON c.pays_id = p.id
         ORDER BY c.created_at DESC
         LIMIT $1 OFFSET $2",
        union
    )
}
