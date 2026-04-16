use actix_web::{web, HttpResponse};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::admin::vidafrica::label_langue;
use crate::models::vidafrica::{
    LangueDisponibleResponse, MotTimingResponse, SegmentPubliqueResponse, SousTitresResponse,
    VideoPubliqueDetailRow, VideoPubliqueDetailResponse, VideoPubliqueListeRow,
    VideoPubliqueListeResponse, VideoPubliqueQueryParams,
};
use crate::ApiResponse;

// ══════════════════════════════════════════════════════════════
// VIDÉOS PUBLIQUES
// ══════════════════════════════════════════════════════════════

/// GET /api/vidafrica/videos
pub async fn lister_videos_publiques(
    pool: web::Data<PgPool>,
    params: web::Query<VideoPubliqueQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let mut conditions = vec![
        "v.etat = 'publie'".to_string(),
        "v.deleted_at IS NULL".to_string(),
    ];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!("v.search_vector @@ plainto_tsquery('french', ${})", bind_index));
            bind_values.push(r.to_string());
            bind_index += 1;
        }
    }

    if let Some(ref langue) = params.langue {
        let l = langue.trim();
        if !l.is_empty() {
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM media_content.piste_sous_titre p
                 WHERE p.video_id = v.id AND p.langue::TEXT = ${} AND p.deleted_at IS NULL)",
                bind_index
            ));
            bind_values.push(l.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");

    // COUNT
    let count_sql = format!("SELECT COUNT(*) FROM media_content.video v WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT
    let select_sql = format!(
        "SELECT v.id, v.titre, v.slug, v.description, v.vignette_url, v.duree_secondes, v.created_at
         FROM media_content.video v
         WHERE {} ORDER BY v.created_at DESC LIMIT {} OFFSET {}",
        where_clause, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, VideoPubliqueListeRow>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let rows = select_q.fetch_all(pool.get_ref()).await?;

    // Charger les langues disponibles par vidéo
    let mut videos: Vec<VideoPubliqueListeResponse> = Vec::new();
    for row in &rows {
        let langues = sqlx::query_scalar::<_, String>(
            "SELECT langue::TEXT FROM media_content.piste_sous_titre
             WHERE video_id = $1 AND deleted_at IS NULL ORDER BY langue"
        )
        .bind(row.id)
        .fetch_all(pool.get_ref())
        .await?;

        videos.push(VideoPubliqueListeResponse {
            id: row.id,
            titre: row.titre.clone(),
            slug: row.slug.clone(),
            description: row.description.clone(),
            vignette_url: row.vignette_url.clone(),
            duree_secondes: row.duree_secondes,
            langues_disponibles: langues,
            created_at: row.created_at,
        });
    }

    let total_pages = ((total as f64) / (par_page as f64)).ceil() as i64;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "donnees": videos,
            "pagination": {
                "page": page,
                "par_page": par_page,
                "total": total,
                "total_pages": total_pages
            }
        })),
        error: None,
    }))
}

/// GET /api/vidafrica/videos/{slug}
pub async fn obtenir_video_publique(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = path.into_inner();

    let row = sqlx::query_as::<_, VideoPubliqueDetailRow>(
        "SELECT id, titre, slug, description, fichier_video_url, vignette_url, duree_secondes, created_at
         FROM media_content.video
         WHERE slug = $1 AND etat = 'publie' AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Vidéo non trouvée".into()))?;

    let langues = sqlx::query_scalar::<_, String>(
        "SELECT langue::TEXT FROM media_content.piste_sous_titre
         WHERE video_id = $1 AND deleted_at IS NULL ORDER BY langue"
    )
    .bind(row.id)
    .fetch_all(pool.get_ref())
    .await?;

    let reponse = VideoPubliqueDetailResponse {
        id: row.id,
        titre: row.titre,
        slug: row.slug,
        description: row.description,
        fichier_video_url: row.fichier_video_url,
        vignette_url: row.vignette_url,
        duree_secondes: row.duree_secondes,
        langues_disponibles: langues,
        created_at: row.created_at,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

/// GET /api/vidafrica/videos/{video_id}/sous-titres/{langue}
pub async fn obtenir_sous_titres(
    pool: web::Data<PgPool>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiErreur> {
    let (video_id_str, langue) = path.into_inner();
    let video_id = Uuid::parse_str(&video_id_str)
        .map_err(|_| ApiErreur::Validation("ID vidéo invalide".into()))?;

    // Vérifier vidéo publiée
    let existe = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM media_content.video WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL)"
    )
    .bind(video_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Vidéo non trouvée".into()));
    }

    // Trouver la piste
    let piste_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM media_content.piste_sous_titre
         WHERE video_id = $1 AND langue::TEXT = $2 AND deleted_at IS NULL"
    )
    .bind(video_id)
    .bind(&langue)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve(format!("Pas de sous-titres en {}", langue)))?;

    // Charger les segments
    let segment_rows = sqlx::query(
        "SELECT id, position, texte, debut_ms, fin_ms
         FROM media_content.segment_sous_titre
         WHERE piste_id = $1 ORDER BY position"
    )
    .bind(piste_id)
    .fetch_all(pool.get_ref())
    .await?;

    let mut segments: Vec<SegmentPubliqueResponse> = Vec::new();
    for row in &segment_rows {
        let seg_id: Uuid = row.get("id");
        let timings = sqlx::query_as::<_, MotTimingResponse>(
            "SELECT position, mot, debut_ms, fin_ms
             FROM media_content.timing_mot
             WHERE segment_id = $1 ORDER BY position"
        )
        .bind(seg_id)
        .fetch_all(pool.get_ref())
        .await?;

        segments.push(SegmentPubliqueResponse {
            position: row.get("position"),
            texte: row.get("texte"),
            debut_ms: row.get("debut_ms"),
            fin_ms: row.get("fin_ms"),
            mots: timings,
        });
    }

    let reponse = SousTitresResponse {
        langue: langue.clone(),
        segments,
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

/// GET /api/vidafrica/langues-sous-titres
pub async fn lister_langues_disponibles(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let rows = sqlx::query(
        "SELECT p.langue::TEXT AS code, COUNT(DISTINCT p.video_id) AS nombre_videos
         FROM media_content.piste_sous_titre p
         JOIN media_content.video v ON v.id = p.video_id
         WHERE v.etat = 'publie' AND v.deleted_at IS NULL AND p.deleted_at IS NULL
         GROUP BY p.langue
         ORDER BY nombre_videos DESC"
    )
    .fetch_all(pool.get_ref())
    .await?;

    let langues: Vec<LangueDisponibleResponse> = rows
        .iter()
        .map(|r| {
            let code: String = r.get("code");
            let label = label_langue(&code).to_string();
            LangueDisponibleResponse {
                code,
                label,
                nombre_videos: r.get("nombre_videos"),
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(langues),
        error: None,
    }))
}
