use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::admin::vidafrica::label_langue;
use crate::models::vidafrica::{
    LangueDisponibleResponse, MotTimingResponse, PartageVideoListeResponse, PartageVideoQueryParams,
    PartageVideoResponse, PartageVideoRow, SegmentPubliqueResponse, SousTitresResponse,
    VideoPubliqueDetailRow, VideoPubliqueDetailResponse, VideoPubliqueListeRow,
    VideoPubliqueListeResponse, VideoPubliqueQueryParams,
};
use crate::ApiResponse;

/// SELECT du mur des partages de vidéos (JOIN vidéo publiée + auteur).
pub const PARTAGE_VIDEO_SELECT: &str = "SELECT
        pv.id, pv.legende, pv.created_at,
        v.id AS video_id, v.titre AS video_titre, v.slug AS video_slug,
        v.vignette_url AS video_vignette_url, v.duree_secondes AS video_duree_secondes,
        ua.id AS auteur_id,
        CASE WHEN ua.deleted_at IS NULL THEN ua.nom ELSE 'Membre' END AS auteur_nom,
        CASE WHEN ua.deleted_at IS NULL THEN ua.prenom ELSE 'retiré' END AS auteur_prenom,
        CASE WHEN ua.deleted_at IS NULL THEN ua.photo_url ELSE NULL END AS auteur_photo_url
     FROM media_content.partage_video pv
     JOIN media_content.video v ON v.id = pv.video_id
     JOIN iam.utilisateur ua ON ua.id = pv.utilisateur_id
     WHERE pv.deleted_at IS NULL AND v.deleted_at IS NULL AND v.etat = 'publie'";

/// Extraire l'utilisateur connecté depuis le header Authorization (OPTIONNEL).
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

// ══════════════════════════════════════════════════════════════
// VIDÉOS PUBLIQUES
// ══════════════════════════════════════════════════════════════

/// Colonnes du fil public de vidéos.
///
/// La liste porte les MÊMES champs d'interaction que le détail
/// (`obtenir_video_publique`) : compteurs, auteur déclaré et `ma_reaction`.
///
/// ⚠️ Ne pas les retirer « parce que c'est juste une liste ». La vignette du
/// fil (`vidafrica/CarteVideoFil.vue`) est interactive : elle dessine le pouce
/// allumé d'après `ma_reaction` et poste sur `/videos/{id}/reaction`, qui est
/// une BASCULE (même type ⇒ suppression, cf. `vidafrica_contribution::reagir_video`).
/// Si la liste omet `ma_reaction`, le front rabat sur `null`, le membre qui a
/// déjà aimé voit un pouce éteint, et son clic SUPPRIME son like au lieu d'en
/// poser un. Les compteurs afficheraient de leur côté 0 en permanence.
///
/// `cible` est la page déjà découpée (LIMIT/OFFSET appliqués dans la
/// sous-requête) : les latérales ne s'exécutent donc que sur les lignes
/// réellement servies — un seul aller-retour, aucun N+1.
const COLONNES_VIDEO_LISTE: &str = "cible.id, cible.titre, cible.slug, cible.description,
            cible.vignette_url, cible.duree_secondes, cible.auteur_reel, cible.created_at,
            COALESCE(l.langues, ARRAY[]::TEXT[]) AS langues_disponibles,
            COALESCE(r.likes, 0)                 AS nombre_likes,
            COALESCE(r.dislikes, 0)              AS nombre_dislikes,
            COALESCE(pa.total, 0)                AS nombre_partages,
            mienne.type_reaction                 AS ma_reaction";

/// Latérales d'agrégation du fil : langues publiées, réactions, partages, puis
/// la réaction du membre connecté (jointure simple : au plus une ligne).
const JOINTURES_VIDEO_LISTE: &str = "LEFT JOIN LATERAL (
             SELECT ARRAY_AGG(ps.langue::TEXT ORDER BY ps.langue) AS langues
               FROM media_content.piste_sous_titre ps
              WHERE ps.video_id = cible.id AND ps.etat = 'publie' AND ps.deleted_at IS NULL
         ) l ON TRUE
         LEFT JOIN LATERAL (
             SELECT COUNT(*) FILTER (WHERE type_reaction = 'like')    AS likes,
                    COUNT(*) FILTER (WHERE type_reaction = 'dislike') AS dislikes
               FROM media_content.video_reaction
              WHERE video_id = cible.id
         ) r ON TRUE
         LEFT JOIN LATERAL (
             SELECT COUNT(*) AS total
               FROM media_content.partage_video
              WHERE video_id = cible.id AND deleted_at IS NULL
         ) pa ON TRUE
         LEFT JOIN media_content.video_reaction mienne
                ON mienne.video_id = cible.id AND mienne.utilisateur_id = ";

/// GET /api/vidafrica/videos
///
/// Route PUBLIQUE : le JWT est facultatif. Anonyme ⇒ le paramètre utilisateur
/// est lié à NULL, `mienne.utilisateur_id = NULL` ne correspond à rien et
/// `ma_reaction` vaut `null` sans erreur — exactement la sémantique du détail.
pub async fn lister_videos_publiques(
    req: HttpRequest,
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
                 WHERE p.video_id = v.id AND p.langue::TEXT = ${}
                 AND p.etat = 'publie' AND p.deleted_at IS NULL)",
                bind_index
            ));
            bind_values.push(l.to_string());
            bind_index += 1;
        }
    }
    // Dernier paramètre : le membre connecté (NULL si anonyme).
    let placeholder_utilisateur = format!("${}", bind_index);
    let utilisateur_id = extraire_utilisateur_id(&req);

    let where_clause = conditions.join(" AND ");

    // COUNT
    let count_sql = format!("SELECT COUNT(*) FROM media_content.video v WHERE {}", where_clause);
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT : la page est découpée AVANT les latérales (sous-requête
    // ordonnée + LIMIT/OFFSET) pour n'agréger que les lignes servies.
    let select_sql = format!(
        "SELECT {}
         FROM (
             SELECT v.id, v.titre, v.slug, v.description, v.vignette_url,
                    v.duree_secondes, v.auteur_reel, v.created_at
               FROM media_content.video v
              WHERE {}
              ORDER BY v.created_at DESC
              LIMIT {} OFFSET {}
         ) cible
         {}{}
         ORDER BY cible.created_at DESC",
        COLONNES_VIDEO_LISTE,
        where_clause,
        par_page,
        offset,
        JOINTURES_VIDEO_LISTE,
        placeholder_utilisateur
    );
    let mut select_q = sqlx::query_as::<_, VideoPubliqueListeRow>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let rows = select_q.bind(utilisateur_id).fetch_all(pool.get_ref()).await?;

    let videos: Vec<VideoPubliqueListeResponse> = rows
        .into_iter()
        .map(|row| VideoPubliqueListeResponse {
            id: row.id,
            titre: row.titre,
            slug: row.slug,
            description: row.description,
            vignette_url: row.vignette_url,
            duree_secondes: row.duree_secondes,
            auteur_reel: row.auteur_reel,
            langues_disponibles: row.langues_disponibles,
            nombre_likes: row.nombre_likes,
            nombre_dislikes: row.nombre_dislikes,
            nombre_partages: row.nombre_partages,
            ma_reaction: row.ma_reaction,
            created_at: row.created_at,
        })
        .collect();

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
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> Result<HttpResponse, ApiErreur> {
    let slug = path.into_inner();

    let row = sqlx::query_as::<_, VideoPubliqueDetailRow>(
        "SELECT id, titre, slug, description, fichier_video_url, vignette_url, duree_secondes,
                territoires, auteur_reel, langue_originale, created_at
         FROM media_content.video
         WHERE slug = $1 AND etat = 'publie' AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Vidéo non trouvée".into()))?;

    let langues = sqlx::query_scalar::<_, String>(
        "SELECT langue::TEXT FROM media_content.piste_sous_titre
         WHERE video_id = $1 AND etat = 'publie' AND deleted_at IS NULL ORDER BY langue"
    )
    .bind(row.id)
    .fetch_all(pool.get_ref())
    .await?;

    // ── Compteurs d'interactions (calculés à la lecture) ──
    let (nombre_likes, nombre_dislikes): (i64, i64) = sqlx::query_as(
        "SELECT
            COUNT(*) FILTER (WHERE type_reaction = 'like'),
            COUNT(*) FILTER (WHERE type_reaction = 'dislike')
         FROM media_content.video_reaction WHERE video_id = $1",
    )
    .bind(row.id)
    .fetch_one(pool.get_ref())
    .await?;

    let nombre_partages: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.partage_video
         WHERE video_id = $1 AND deleted_at IS NULL",
    )
    .bind(row.id)
    .fetch_one(pool.get_ref())
    .await?;

    // ── Réaction du membre connecté (JWT optionnel) ──
    let ma_reaction: Option<String> = match extraire_utilisateur_id(&req) {
        Some(uid) => sqlx::query_scalar(
            "SELECT type_reaction FROM media_content.video_reaction
             WHERE video_id = $1 AND utilisateur_id = $2",
        )
        .bind(row.id)
        .bind(uid)
        .fetch_optional(pool.get_ref())
        .await?,
        None => None,
    };

    let reponse = VideoPubliqueDetailResponse {
        id: row.id,
        titre: row.titre,
        slug: row.slug,
        description: row.description,
        fichier_video_url: row.fichier_video_url,
        vignette_url: row.vignette_url,
        duree_secondes: row.duree_secondes,
        territoires: row.territoires,
        auteur_reel: row.auteur_reel,
        langue_originale: row.langue_originale,
        langues_disponibles: langues,
        nombre_likes,
        nombre_dislikes,
        nombre_partages,
        ma_reaction,
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

    // Trouver la piste publiée (une seule par langue) + son auteur.
    let (piste_id, auteur) = sqlx::query_as::<_, (Uuid, Option<String>)>(
        "SELECT p.id, NULLIF(TRIM(COALESCE(u.prenom, '') || ' ' || COALESCE(u.nom, '')), '')
         FROM media_content.piste_sous_titre p
         LEFT JOIN iam.utilisateur u ON u.id = p.cree_par
         WHERE p.video_id = $1 AND p.langue::TEXT = $2 AND p.etat = 'publie' AND p.deleted_at IS NULL"
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
        auteur,
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
         WHERE v.etat = 'publie' AND v.deleted_at IS NULL
           AND p.etat = 'publie' AND p.deleted_at IS NULL
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

/// GET /api/vidafrica/videos/partages : mur communautaire des vidéos (public, paginé).
pub async fn lister_partages_videos(
    pool: web::Data<PgPool>,
    params: web::Query<PartageVideoQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM media_content.partage_video pv
         JOIN media_content.video v ON v.id = pv.video_id
         WHERE pv.deleted_at IS NULL AND v.deleted_at IS NULL AND v.etat = 'publie'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let rows = sqlx::query_as::<_, PartageVideoRow>(&format!(
        "{} ORDER BY pv.created_at DESC LIMIT $1 OFFSET $2",
        PARTAGE_VIDEO_SELECT
    ))
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let partages: Vec<PartageVideoResponse> =
        rows.into_iter().map(PartageVideoResponse::from).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PartageVideoListeResponse {
            partages,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}
