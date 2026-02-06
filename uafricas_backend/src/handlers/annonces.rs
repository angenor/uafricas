use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::annonce::{
    AnnonceDetailRow, AnnonceListeResponse, AnnonceListeRow, AnnonceMediaResponse,
    AnnonceMediaRow, AnnoncePaysRow, AnnonceQueryParams, ANNONCE_DETAIL_COLONNES,
    ANNONCE_LISTE_COLONNES,
};

/// Reponse API generique
#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ──────────────────────────────────────────────────────────────
// GET /api/annonces — Lister les annonces avec filtres et pagination
// ──────────────────────────────────────────────────────────────
pub async fn lister_annonces(
    pool: web::Data<PgPool>,
    params: web::Query<AnnonceQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Preparer le terme de recherche
    let terme_recherche: Option<String> = params
        .recherche
        .as_ref()
        .filter(|r| !r.trim().is_empty())
        .map(|r| format!("%{}%", r.to_lowercase()));

    // Preparer le filtre type_operation (supporte valeurs multiples separees par virgules)
    let type_operation_filtre: Option<String> = params
        .type_operation
        .as_ref()
        .filter(|t| !t.trim().is_empty())
        .cloned();

    // Preparer le filtre categorie
    let categorie_filtre: Option<String> = params
        .categorie
        .as_ref()
        .filter(|c| !c.trim().is_empty() && c.as_str() != "Tout")
        .cloned();

    // Clause ORDER BY dynamique
    let order_clause = match params.tri.as_deref() {
        Some("price-asc") => "ORDER BY COALESCE(a.prix, 0) ASC, a.created_at DESC",
        Some("price-desc") => "ORDER BY COALESCE(a.prix, 0) DESC, a.created_at DESC",
        _ => "ORDER BY a.created_at DESC",
    };

    // ── Requete COUNT ────────────────────────────────────────
    let count_query = format!(
        "SELECT COUNT(*) FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         WHERE a.deleted_at IS NULL
           AND a.etat = 'publiee'
           AND ($1::VARCHAR IS NULL OR a.type_operation::text = ANY(string_to_array($1, ',')))
           AND ($2::VARCHAR IS NULL OR c.nom = $2 OR c.slug = $2)
           AND ($3::FLOAT8 IS NULL OR a.prix >= $3)
           AND ($4::FLOAT8 IS NULL OR a.prix <= $4)
           AND ($5::VARCHAR IS NULL OR (
               LOWER(a.titre) LIKE $5
               OR LOWER(a.description) LIKE $5
               OR LOWER(COALESCE(a.ville, '')) LIKE $5
           ))"
    );

    let total: (i64,) = sqlx::query_as(&count_query)
        .bind(&type_operation_filtre)
        .bind(&categorie_filtre)
        .bind(params.prix_min)
        .bind(params.prix_max)
        .bind(&terme_recherche)
        .fetch_one(pool.get_ref())
        .await?;

    // ── Requete principale ───────────────────────────────────
    let query = format!(
        "SELECT {colonnes}
         FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.deleted_at IS NULL
           AND a.etat = 'publiee'
           AND ($1::VARCHAR IS NULL OR a.type_operation::text = ANY(string_to_array($1, ',')))
           AND ($2::VARCHAR IS NULL OR c.nom = $2 OR c.slug = $2)
           AND ($3::FLOAT8 IS NULL OR a.prix >= $3)
           AND ($4::FLOAT8 IS NULL OR a.prix <= $4)
           AND ($5::VARCHAR IS NULL OR (
               LOWER(a.titre) LIKE $5
               OR LOWER(a.description) LIKE $5
               OR LOWER(COALESCE(a.ville, '')) LIKE $5
           ))
         {order}
         LIMIT $6 OFFSET $7",
        colonnes = ANNONCE_LISTE_COLONNES,
        order = order_clause
    );

    let rows = sqlx::query_as::<_, AnnonceListeRow>(&query)
        .bind(&type_operation_filtre)
        .bind(&categorie_filtre)
        .bind(params.prix_min)
        .bind(params.prix_max)
        .bind(&terme_recherche)
        .bind(par_page)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let total_count = total.0;
    let total_pages = if total_count == 0 {
        1
    } else {
        (total_count as f64 / par_page as f64).ceil() as i64
    };

    let reponse = AnnonceListeResponse {
        annonces: rows.iter().map(|r| r.to_response()).collect(),
        total: total_count,
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

// ──────────────────────────────────────────────────────────────
// GET /api/annonces/{id} — Obtenir le detail d'une annonce
// ──────────────────────────────────────────────────────────────
pub async fn obtenir_annonce(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();

    // 1. Requete principale avec categorie + auteur
    let query = format!(
        "SELECT {}
         FROM marketplace.annonce a
         LEFT JOIN shared.categorie c ON c.id = a.categorie_id
         JOIN iam.utilisateur u ON u.id = a.cree_par
         WHERE a.id = $1 AND a.deleted_at IS NULL",
        ANNONCE_DETAIL_COLONNES
    );

    let annonce = sqlx::query_as::<_, AnnonceDetailRow>(&query)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve(format!("Annonce avec id {} non trouvee", id)))?;

    // 2. Incrementer le nombre de vues
    let _ = sqlx::query(
        "UPDATE marketplace.annonce SET nombre_vues = nombre_vues + 1 WHERE id = $1",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await;

    // 3. Charger tous les medias
    let media_rows = sqlx::query_as::<_, AnnonceMediaRow>(
        "SELECT id, media_url, type_mime, est_principale, ordre
         FROM marketplace.annonce_media
         WHERE annonce_id = $1
         ORDER BY est_principale DESC NULLS LAST, ordre ASC",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let medias: Vec<AnnonceMediaResponse> = media_rows
        .into_iter()
        .map(|m| AnnonceMediaResponse {
            id: m.id,
            media_url: m.media_url,
            type_mime: m.type_mime,
            est_principale: m.est_principale.unwrap_or(false),
            ordre: m.ordre.unwrap_or(0),
        })
        .collect();

    // 4. Charger tous les pays
    let pays_rows = sqlx::query_as::<_, AnnoncePaysRow>(
        "SELECT p.nom AS pays_nom
         FROM marketplace.annonce_pays ap
         JOIN shared.pays p ON p.id = ap.pays_id
         WHERE ap.annonce_id = $1",
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    let pays: Vec<String> = pays_rows.into_iter().map(|p| p.pays_nom).collect();

    // 5. Construire la reponse
    let reponse = annonce.to_detail_response(pays, medias);

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}
