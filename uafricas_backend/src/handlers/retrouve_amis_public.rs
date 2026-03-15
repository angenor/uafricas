use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::errors::ApiErreur;
use crate::models::retrouve_amis::*;
use crate::services::audit;
use crate::ApiResponse;

// ══════════════════════════════════════════════════════════════
// Handlers publics — Avis de recherche (sans authentification)
// ══════════════════════════════════════════════════════════════
//
// Ces handlers sont enregistres HORS du scope JWT dans routes.rs.
// Ils ne necessitent pas de token d'authentification.
//
// Endpoints :
// - GET  /api/retrouve-amis/public/{slug}           → detail_avis_public
// - GET  /api/retrouve-amis/public/rechercher        → rechercher_avis_publics
// - POST /api/retrouve-amis/public/{slug}/partage    → incrementer_partage

/// POST /api/retrouve-amis/public/{slug}/partage
/// Incrementer le compteur de partages d'un avis public (sans auth)
pub async fn incrementer_partage(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = path.into_inner();

    // UPDATE atomique avec retour du nouveau compteur + id pour l'audit
    let result: Option<(i32, uuid::Uuid)> = sqlx::query_as(
        "UPDATE retrouve_amis.avis_recherche
         SET compteur_partages = compteur_partages + 1
         WHERE slug = $1 AND est_public = TRUE AND etat = 'actif' AND deleted_at IS NULL
         RETURNING compteur_partages, id",
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?;

    match result {
        Some((compteur, avis_id)) => {
            let ip = audit::extraire_ip(&req);
            let ua = audit::extraire_user_agent(&req);
            audit::log_action(
                pool.get_ref(),
                None,
                "UPDATE",
                "retrouve_amis",
                "avis_recherche",
                Some(avis_id),
                None,
                Some(serde_json::json!({ "compteur_partages": compteur })),
                ip.as_deref(),
                ua.as_deref(),
            )
            .await;

            Ok(HttpResponse::Ok().json(ApiResponse {
                success: true,
                data: Some(PartageResponse {
                    compteur_partages: compteur,
                }),
                error: None,
            }))
        }
        None => Ok(HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            error: Some("Avis non disponible.".to_string()),
        })),
    }
}

/// GET /api/retrouve-amis/public/rechercher
/// Lister et rechercher parmi les avis publics actifs
pub async fn rechercher_avis_publics(
    pool: web::Data<PgPool>,
    params: web::Query<RecherchePubliqueParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(12).min(50).max(1);
    let offset = (page - 1) * par_page;

    let tri = match params.tri.as_deref() {
        Some(t) if AVIS_PUBLIC_TRI_COLONNES.contains(&t) => format!("a.{}", t),
        _ => "a.created_at".to_string(),
    };
    let ordre = match params.ordre.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Construire les conditions WHERE dynamiquement
    let mut conditions = vec![
        "a.est_public = TRUE".to_string(),
        "a.etat = 'actif'".to_string(),
        "a.deleted_at IS NULL".to_string(),
    ];
    let mut bind_idx: u32 = 1;

    if params.pays_id.is_some() {
        conditions.push(format!("a.pays_id = ${}", bind_idx));
        bind_idx += 1;
    }

    if params.ville.as_ref().map_or(false, |v| !v.trim().is_empty()) {
        conditions.push(format!("a.ville ILIKE ${}", bind_idx));
        bind_idx += 1;
    }

    if params.ecole.as_ref().map_or(false, |v| !v.trim().is_empty()) {
        conditions.push(format!("a.ecole ILIKE ${}", bind_idx));
        bind_idx += 1;
    }

    if params.recherche.as_ref().map_or(false, |v| !v.trim().is_empty()) {
        conditions.push(format!(
            "a.search_vector @@ plainto_tsquery('french', ${})",
            bind_idx
        ));
        bind_idx += 1;
    }

    if params.type_relation.as_ref().map_or(false, |v| !v.trim().is_empty()) {
        conditions.push(format!(
            "a.type_relation = ${}::retrouve_amis.type_relation_recherche",
            bind_idx
        ));
    }

    let where_clause = conditions.join(" AND ");

    let count_sql = format!(
        "SELECT COUNT(*) FROM retrouve_amis.avis_recherche a WHERE {}",
        where_clause
    );
    let list_sql = format!(
        "SELECT {} FROM retrouve_amis.avis_recherche a
         JOIN iam.utilisateur u ON u.id = a.auteur_id
         LEFT JOIN shared.pays p ON p.id = a.pays_id
         WHERE {}
         ORDER BY {} {}
         LIMIT {} OFFSET {}",
        AVIS_PUBLIC_LISTE_COLONNES, where_clause, tri, ordre, par_page, offset
    );

    // Binder les parametres dynamiquement dans le bon ordre
    // Utiliser une approche sans macro pour la compatibilite Rust Edition 2024
    let pays_id_val = params.pays_id;
    let ville_val = params.ville.as_deref().filter(|v| !v.trim().is_empty()).map(|v| format!("%{}%", v.trim()));
    let ecole_val = params.ecole.as_deref().filter(|v| !v.trim().is_empty()).map(|v| format!("%{}%", v.trim()));
    let recherche_val = params.recherche.as_deref().filter(|v| !v.trim().is_empty()).map(|v| v.trim().to_string());
    let type_relation_val = params.type_relation.as_deref().filter(|v| !v.trim().is_empty()).map(|v| v.trim().to_string());

    macro_rules! bind_params {
        ($query:expr) => {{
            let mut q = $query;
            if let Some(pid) = pays_id_val {
                q = q.bind(pid);
            }
            if let Some(v) = &ville_val {
                q = q.bind(v.clone());
            }
            if let Some(e) = &ecole_val {
                q = q.bind(e.clone());
            }
            if let Some(r) = &recherche_val {
                q = q.bind(r.clone());
            }
            if let Some(tr) = &type_relation_val {
                q = q.bind(tr.clone());
            }
            q
        }};
    }

    let total: (i64,) = bind_params!(sqlx::query_as(&count_sql))
        .fetch_one(pool.get_ref())
        .await?;

    let rows: Vec<AvisPublicListeRow> = bind_params!(sqlx::query_as(&list_sql))
        .fetch_all(pool.get_ref())
        .await?;

    let avis: Vec<AvisPublicResumeResponse> =
        rows.iter().map(|r| r.to_resume_response()).collect();

    let pages = if total.0 > 0 {
        (total.0 + par_page - 1) / par_page
    } else {
        0
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ListeAvisPublicsResponse {
            avis,
            pagination: PaginationInfo {
                page,
                par_page,
                total: total.0,
                pages,
            },
        }),
        error: None,
    }))
}

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
