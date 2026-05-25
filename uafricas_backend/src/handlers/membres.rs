use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::membre::{
    MembreListeResponse, MembreQueryParams, MembreRow, MEMBRE_COLONNES,
};

#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Endpoints
// ────────────────────────────────────────────────────────────────

/// GET /api/utilisateurs — Annuaire public paginee de tous les membres actifs
pub async fn lister_membres(
    pool: web::Data<PgPool>,
    params: web::Query<MembreQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).clamp(1, 50);
    let offset = (page - 1) * par_page;

    // Conditions de base : comptes actifs et non supprimes uniquement
    let mut conditions: Vec<String> = vec![
        "u.deleted_at IS NULL".to_string(),
        "u.etat = 'actif'".to_string(),
    ];
    let mut bind_index = 1u32;
    let mut bind_values: Vec<String> = Vec::new();

    // Filtre par type de membre (expert ou bibliotheque humaine)
    match params.r#type.as_deref().map(str::trim) {
        Some("expert") => conditions.push(
            "EXISTS (SELECT 1 FROM iam.expertise e
                WHERE e.utilisateur_id = u.id AND e.statut = 'valide' AND e.deleted_at IS NULL)"
                .to_string(),
        ),
        Some("biblio") => conditions.push(
            "EXISTS (SELECT 1 FROM iam.demande_biblio_humaine d
                WHERE d.utilisateur_id = u.id AND d.statut = 'valide' AND d.deleted_at IS NULL)"
                .to_string(),
        ),
        _ => {}
    }

    // Filtre par pays de residence
    if let Some(ref pays) = params.pays {
        let trimmed = pays.trim();
        if !trimmed.is_empty() {
            conditions.push(format!("LOWER(p.nom) = LOWER(${})", bind_index));
            bind_values.push(trimmed.to_string());
            bind_index += 1;
        }
    }

    // Recherche textuelle (nom, prenom, fonction, ville)
    if let Some(ref recherche) = params.recherche {
        let trimmed = recherche.trim();
        if !trimmed.is_empty() {
            let terme = format!("%{}%", trimmed.to_lowercase());
            conditions.push(format!(
                "(LOWER(u.nom) LIKE ${idx} OR LOWER(u.prenom) LIKE ${idx} \
                 OR LOWER(COALESCE(u.fonction, '')) LIKE ${idx} \
                 OR LOWER(COALESCE(u.ville, '')) LIKE ${idx})",
                idx = bind_index
            ));
            bind_values.push(terme);
            bind_index += 1;
        }
    }

    let where_clause = conditions.join(" AND ");

    // Compter le total
    let count_query = format!(
        "SELECT COUNT(*) FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_q = count_q.bind(val);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // Recuperer les membres
    let select_query = format!(
        "SELECT {} FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE {} ORDER BY u.created_at DESC LIMIT ${} OFFSET ${}",
        MEMBRE_COLONNES,
        where_clause,
        bind_index,
        bind_index + 1
    );

    let mut select_q = sqlx::query_as::<_, MembreRow>(&select_query);
    for val in &bind_values {
        select_q = select_q.bind(val);
    }
    select_q = select_q.bind(par_page).bind(offset);

    let rows = select_q.fetch_all(pool.get_ref()).await?;
    let membres: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(MembreListeResponse {
            membres,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

/// GET /api/utilisateurs/{id} — Detail public d'un membre par son id
pub async fn obtenir_membre(
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = chemin.into_inner();

    let query = format!(
        "SELECT {} FROM iam.utilisateur u
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE u.id = $1 AND u.deleted_at IS NULL AND u.etat = 'actif'",
        MEMBRE_COLONNES
    );

    let row = sqlx::query_as::<_, MembreRow>(&query)
        .bind(utilisateur_id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| {
            ApiErreur::NonTrouve(format!("Membre avec id {} non trouve", utilisateur_id))
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response()),
        error: None,
    }))
}
