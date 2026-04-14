use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::vidafrica::{
    AdminPisteSousTitreResponse, AdminSegmentRow, AdminSegmentSousTitreResponse,
    AdminVideoDetailRow, AdminVideoListeResponse, AdminVideoQueryParams,
    ChangerEtatVideoRequest, CreerPisteRequest, CreerSegmentRequest,
    EnregistrerTimingsMotRequest, ModifierSegmentRequest, ReordonnerSegmentsRequest,
    TimingMotResponse, ADMIN_VIDEO_DETAIL_COLONNES, ADMIN_VIDEO_LISTE_COLONNES,
    LANGUES_VALIDES, VIDEO_TRI_COLONNES,
};
use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ── Etats valides ────────────────────────────────────────────
const ETATS_VALIDES: &[&str] = &["brouillon", "publie", "suspendu", "supprime"];

// ══════════════════════════════════════════════════════════════
// VIDÉOS — CRUD
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/vidafrica/videos
pub async fn lister_videos(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<AdminVideoQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "voir");

    let pagination = PaginationParams {
        page: params.page,
        par_page: params.par_page,
        tri_par: params.tri_par.clone(),
        tri_dir: params.tri_dir.clone(),
    };

    let mut conditions = vec!["v.deleted_at IS NULL".to_string()];
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref recherche) = params.recherche {
        let r = recherche.trim();
        if !r.is_empty() {
            conditions.push(format!(
                "(LOWER(v.titre) LIKE ${bi} OR LOWER(COALESCE(v.description, '')) LIKE ${bi})",
                bi = bind_index
            ));
            bind_values.push(format!("%{}%", r.to_lowercase()));
            bind_index += 1;
        }
    }

    if let Some(ref etat) = params.etat {
        let e = etat.trim();
        if !e.is_empty() {
            conditions.push(format!("v.etat = ${}", bind_index));
            bind_values.push(e.to_string());
            bind_index += 1;
        }
    }
    let _ = bind_index;

    let where_clause = conditions.join(" AND ");
    let colonne = pagination.colonne_tri(VIDEO_TRI_COLONNES, "created_at");
    let direction = pagination.direction_tri();
    let page = pagination.page();
    let par_page = pagination.par_page();
    let offset = pagination.offset();

    // COUNT
    let count_sql = format!(
        "SELECT COUNT(*) FROM media_content.video v WHERE {}",
        where_clause
    );
    let mut count_q = sqlx::query_scalar::<_, i64>(&count_sql);
    for v in &bind_values {
        count_q = count_q.bind(v);
    }
    let total: i64 = count_q.fetch_one(pool.get_ref()).await?;

    // SELECT
    let select_sql = format!(
        "SELECT {} FROM media_content.video v WHERE {} ORDER BY v.{} {} LIMIT {} OFFSET {}",
        ADMIN_VIDEO_LISTE_COLONNES, where_clause, colonne, direction, par_page, offset
    );
    let mut select_q = sqlx::query_as::<_, AdminVideoListeResponse>(&select_sql);
    for v in &bind_values {
        select_q = select_q.bind(v);
    }
    let items = select_q.fetch_all(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(items, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/vidafrica/videos/{id}
pub async fn obtenir_video(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "voir");
    let id = path.into_inner();

    let sql = format!(
        "SELECT {} FROM media_content.video v
         JOIN iam.utilisateur u ON u.id = v.cree_par
         WHERE v.id = $1 AND v.deleted_at IS NULL",
        ADMIN_VIDEO_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminVideoDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Vidéo non trouvée".into()))?;

    // Charger les pistes
    let pistes = sqlx::query_as::<_, AdminPisteSousTitreResponse>(
        "SELECT p.id, p.langue::TEXT AS langue, p.est_complete,
                (SELECT COUNT(*) FROM media_content.segment_sous_titre s
                 WHERE s.piste_id = p.id)::INTEGER AS nombre_segments,
                p.created_at
         FROM media_content.piste_sous_titre p
         WHERE p.video_id = $1 AND p.deleted_at IS NULL
         ORDER BY p.created_at"
    )
    .bind(id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(pistes)),
        error: None,
    }))
}

/// POST /api/admin/vidafrica/videos (multipart/form-data)
pub async fn creer_video(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "modifier");

    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut fichier_video_url: Option<String> = None;
    let mut vignette_url: Option<String> = None;
    let mut taille_octets: Option<i64> = None;
    let mut format_video: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            ApiErreur::Upload(format!("Erreur lecture multipart: {}", e))
        })?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();

        match nom_champ.as_str() {
            "titre" => {
                titre = Some(lire_champ_texte(&mut field).await?);
            }
            "description" => {
                description = Some(lire_champ_texte(&mut field).await?);
            }
            "fichier_video" => {
                let filename = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("video.mp4")
                    .to_string();

                let ext = filename
                    .rsplit('.')
                    .next()
                    .unwrap_or("mp4")
                    .to_lowercase();
                if !["mp4", "webm"].contains(&ext.as_str()) {
                    return Err(ApiErreur::Validation(
                        "Format vidéo invalide. Formats acceptés: MP4, WebM".into(),
                    ));
                }

                let nom_fichier = format!("{}.{}", Uuid::new_v4(), ext);
                let chemin_relatif = format!("/uploads/videos/{}", nom_fichier);
                let chemin_complet = format!("{}/videos/{}", upload_dir.get_ref(), nom_fichier);

                let taille = sauvegarder_fichier(&mut field, &chemin_complet).await?;
                if taille > 500 * 1024 * 1024 {
                    // Supprimer le fichier trop gros
                    let _ = std::fs::remove_file(&chemin_complet);
                    return Err(ApiErreur::Validation(
                        "Le fichier vidéo dépasse la limite de 500 Mo".into(),
                    ));
                }
                taille_octets = Some(taille as i64);
                format_video = Some(ext);
                fichier_video_url = Some(chemin_relatif);
            }
            "vignette" => {
                let filename = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("vignette.jpg")
                    .to_string();

                let ext = filename
                    .rsplit('.')
                    .next()
                    .unwrap_or("jpg")
                    .to_lowercase();
                if !["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()) {
                    return Err(ApiErreur::Validation(
                        "Format vignette invalide. Formats acceptés: JPG, PNG, WebP".into(),
                    ));
                }

                let nom_fichier = format!("{}.{}", Uuid::new_v4(), ext);
                let chemin_relatif = format!("/uploads/vignettes/{}", nom_fichier);
                let chemin_complet = format!("{}/vignettes/{}", upload_dir.get_ref(), nom_fichier);

                let taille = sauvegarder_fichier(&mut field, &chemin_complet).await?;
                if taille > 5 * 1024 * 1024 {
                    let _ = std::fs::remove_file(&chemin_complet);
                    return Err(ApiErreur::Validation(
                        "La vignette dépasse la limite de 5 Mo".into(),
                    ));
                }
                vignette_url = Some(chemin_relatif);
            }
            _ => {
                log::warn!("Champ multipart vidafrica inconnu ignoré: {}", nom_champ);
            }
        }
    }

    // Validation
    let titre = titre
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est requis".into()))?;

    let fichier_video_url = fichier_video_url
        .ok_or_else(|| ApiErreur::Validation("Le fichier vidéo est requis".into()))?;

    let id = Uuid::new_v4();
    let slug = crate::models::admin::vidafrica::generer_slug(&titre);
    let description = description.map(|d| d.trim().to_string()).filter(|d| !d.is_empty());

    sqlx::query(
        "INSERT INTO media_content.video
         (id, titre, slug, description, fichier_video_url, vignette_url,
          taille_octets, format_video, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(id)
    .bind(&titre)
    .bind(&slug)
    .bind(&description)
    .bind(&fichier_video_url)
    .bind(&vignette_url)
    .bind(taille_octets)
    .bind(&format_video)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a créé la vidéo {} ({})", admin.id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "media_content", "video",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    // Retourner le détail
    let sql = format!(
        "SELECT {} FROM media_content.video v
         JOIN iam.utilisateur u ON u.id = v.cree_par
         WHERE v.id = $1",
        ADMIN_VIDEO_DETAIL_COLONNES
    );
    let row = sqlx::query_as::<_, AdminVideoDetailRow>(&sql)
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response(vec![])),
        error: None,
    }))
}

/// PUT /api/admin/vidafrica/videos/{id} (multipart/form-data)
pub async fn modifier_video(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    path: web::Path<Uuid>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "modifier");
    let id = path.into_inner();

    // Vérifier existence
    let existe = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM media_content.video WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Vidéo non trouvée".into()));
    }

    let mut set_clauses: Vec<String> = Vec::new();
    let mut bind_values: Vec<String> = Vec::new();
    let mut bind_index: u32 = 1;
    let mut nouvelle_vignette_url: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| {
            ApiErreur::Upload(format!("Erreur lecture multipart: {}", e))
        })?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();

        match nom_champ.as_str() {
            "titre" => {
                let val = lire_champ_texte(&mut field).await?;
                let val = val.trim().to_string();
                if !val.is_empty() {
                    set_clauses.push(format!("titre = ${}", bind_index));
                    bind_values.push(val.clone());
                    bind_index += 1;
                    let slug = crate::models::admin::vidafrica::generer_slug(&val);
                    set_clauses.push(format!("slug = ${}", bind_index));
                    bind_values.push(slug);
                    bind_index += 1;
                }
            }
            "description" => {
                let val = lire_champ_texte(&mut field).await?;
                set_clauses.push(format!("description = ${}", bind_index));
                bind_values.push(val.trim().to_string());
                bind_index += 1;
            }
            "vignette" => {
                let filename = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("vignette.jpg")
                    .to_string();

                let ext = filename.rsplit('.').next().unwrap_or("jpg").to_lowercase();
                if !["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()) {
                    return Err(ApiErreur::Validation(
                        "Format vignette invalide".into(),
                    ));
                }

                let nom_fichier = format!("{}.{}", Uuid::new_v4(), ext);
                let chemin_relatif = format!("/uploads/vignettes/{}", nom_fichier);
                let chemin_complet = format!("{}/vignettes/{}", upload_dir.get_ref(), nom_fichier);

                let taille = sauvegarder_fichier(&mut field, &chemin_complet).await?;
                if taille > 5 * 1024 * 1024 {
                    let _ = std::fs::remove_file(&chemin_complet);
                    return Err(ApiErreur::Validation("La vignette dépasse 5 Mo".into()));
                }
                nouvelle_vignette_url = Some(chemin_relatif);
            }
            _ => {}
        }
    }

    if let Some(ref url) = nouvelle_vignette_url {
        set_clauses.push(format!("vignette_url = ${}", bind_index));
        bind_values.push(url.clone());
        bind_index += 1;
    }

    if set_clauses.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ à modifier".into()));
    }

    set_clauses.push("updated_at = NOW()".to_string());
    let _ = bind_index;

    let sql = format!(
        "UPDATE media_content.video SET {} WHERE id = ${}",
        set_clauses.join(", "),
        bind_values.len() + 1
    );
    let mut query = sqlx::query(&sql);
    for v in &bind_values {
        query = query.bind(v);
    }
    query = query.bind(id);
    query.execute(pool.get_ref()).await?;

    log::info!("Admin {} a modifié la vidéo {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "media_content", "video",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PATCH /api/admin/vidafrica/videos/{id}/etat
pub async fn changer_etat_video(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ChangerEtatVideoRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "modifier");
    let id = path.into_inner();

    let etat = body.etat.trim();
    if !ETATS_VALIDES.contains(&etat) {
        return Err(ApiErreur::Validation(format!(
            "État invalide. Valeurs acceptées: {}",
            ETATS_VALIDES.join(", ")
        )));
    }

    let result = sqlx::query(
        "UPDATE media_content.video SET etat = $1, updated_at = NOW() WHERE id = $2 AND deleted_at IS NULL"
    )
    .bind(etat)
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Vidéo non trouvée".into()));
    }

    log::info!("Admin {} a changé l'état de la vidéo {} → {}", admin.id, id, etat);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "media_content", "video",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "etat": etat })),
        error: None,
    }))
}

/// DELETE /api/admin/vidafrica/videos/{id}
pub async fn supprimer_video(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.video SET deleted_at = NOW(), etat = 'supprime', updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Vidéo non trouvée".into()));
    }

    log::info!("Admin {} a supprimé la vidéo {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "media_content", "video",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// PISTES DE SOUS-TITRES
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/vidafrica/videos/{video_id}/pistes
pub async fn lister_pistes(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "voir");
    let video_id = path.into_inner();

    let pistes = sqlx::query_as::<_, AdminPisteSousTitreResponse>(
        "SELECT p.id, p.langue::TEXT AS langue, p.est_complete,
                (SELECT COUNT(*) FROM media_content.segment_sous_titre s
                 WHERE s.piste_id = p.id)::INTEGER AS nombre_segments,
                p.created_at
         FROM media_content.piste_sous_titre p
         WHERE p.video_id = $1 AND p.deleted_at IS NULL
         ORDER BY p.created_at"
    )
    .bind(video_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(pistes),
        error: None,
    }))
}

/// POST /api/admin/vidafrica/videos/{video_id}/pistes
pub async fn creer_piste(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerPisteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "modifier");
    let video_id = path.into_inner();

    let langue = body.langue.trim();
    if !LANGUES_VALIDES.contains(&langue) {
        return Err(ApiErreur::Validation(format!(
            "Langue invalide. Valeurs acceptées: {}",
            LANGUES_VALIDES.join(", ")
        )));
    }

    // Vérifier que la vidéo existe
    let existe = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM media_content.video WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(video_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Vidéo non trouvée".into()));
    }

    // Vérifier unicité langue
    let doublon = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM media_content.piste_sous_titre
         WHERE video_id = $1 AND langue::TEXT = $2 AND deleted_at IS NULL)"
    )
    .bind(video_id)
    .bind(langue)
    .fetch_one(pool.get_ref())
    .await?;

    if doublon {
        return Err(ApiErreur::Conflit(format!(
            "Une piste en {} existe déjà pour cette vidéo", langue
        )));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO media_content.piste_sous_titre (id, video_id, langue, cree_par)
         VALUES ($1, $2, $3::media_content.langue_sous_titre, $4)"
    )
    .bind(id)
    .bind(video_id)
    .bind(langue)
    .bind(admin.id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a créé la piste {} ({}) pour la vidéo {}", admin.id, id, langue, video_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "media_content", "piste_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "langue": langue })),
        error: None,
    }))
}

/// DELETE /api/admin/vidafrica/pistes/{id}
pub async fn supprimer_piste(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "supprimer");
    let id = path.into_inner();

    let result = sqlx::query(
        "UPDATE media_content.piste_sous_titre SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Piste non trouvée".into()));
    }

    log::info!("Admin {} a supprimé la piste {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "media_content", "piste_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// SEGMENTS DE SOUS-TITRES
// ══════════════════════════════════════════════════════════════

/// GET /api/admin/vidafrica/pistes/{piste_id}/segments
pub async fn lister_segments(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "voir");
    let piste_id = path.into_inner();

    let segments = sqlx::query_as::<_, AdminSegmentRow>(
        "SELECT id, position, texte, debut_ms, fin_ms, created_at, updated_at
         FROM media_content.segment_sous_titre
         WHERE piste_id = $1
         ORDER BY position"
    )
    .bind(piste_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Charger les timings mot pour chaque segment
    let mut resultat: Vec<AdminSegmentSousTitreResponse> = Vec::new();
    for seg in &segments {
        let timings = sqlx::query_as::<_, TimingMotResponse>(
            "SELECT position, mot, debut_ms, fin_ms
             FROM media_content.timing_mot
             WHERE segment_id = $1
             ORDER BY position"
        )
        .bind(seg.id)
        .fetch_all(pool.get_ref())
        .await?;

        resultat.push(AdminSegmentSousTitreResponse {
            id: seg.id,
            position: seg.position,
            texte: seg.texte.clone(),
            debut_ms: seg.debut_ms,
            fin_ms: seg.fin_ms,
            timings_mot: timings,
            created_at: seg.created_at,
            updated_at: seg.updated_at,
        });
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(resultat),
        error: None,
    }))
}

/// POST /api/admin/vidafrica/pistes/{piste_id}/segments
pub async fn creer_segment(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerSegmentRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "modifier");
    let piste_id = path.into_inner();

    // Validation
    let texte = body.texte.trim();
    if texte.is_empty() {
        return Err(ApiErreur::Validation("Le texte est requis".into()));
    }
    if body.debut_ms < 0 {
        return Err(ApiErreur::Validation("Le timestamp de début doit être positif".into()));
    }
    if body.debut_ms >= body.fin_ms {
        return Err(ApiErreur::Validation("Le timestamp de début doit être inférieur à la fin".into()));
    }

    // Vérifier que la piste existe
    let existe = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM media_content.piste_sous_titre WHERE id = $1 AND deleted_at IS NULL)"
    )
    .bind(piste_id)
    .fetch_one(pool.get_ref())
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Piste non trouvée".into()));
    }

    // Auto-incrémenter la position
    let next_pos = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT MAX(position) FROM media_content.segment_sous_titre WHERE piste_id = $1"
    )
    .bind(piste_id)
    .fetch_one(pool.get_ref())
    .await?
    .unwrap_or(0) + 1;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO media_content.segment_sous_titre (id, piste_id, position, texte, debut_ms, fin_ms)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(id)
    .bind(piste_id)
    .bind(next_pos)
    .bind(texte)
    .bind(body.debut_ms)
    .bind(body.fin_ms)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a créé le segment {} (position {}) pour la piste {}", admin.id, id, next_pos, piste_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "media_content", "segment_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "position": next_pos })),
        error: None,
    }))
}

/// PUT /api/admin/vidafrica/segments/{id}
pub async fn modifier_segment(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierSegmentRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "modifier");
    let id = path.into_inner();

    let mut set_clauses: Vec<String> = Vec::new();
    let mut bind_values_str: Vec<String> = Vec::new();
    let mut bind_values_i32: Vec<i32> = Vec::new();
    let mut bind_types: Vec<&str> = Vec::new();
    let mut bind_index: u32 = 1;

    if let Some(ref texte) = body.texte {
        let t = texte.trim();
        if !t.is_empty() {
            set_clauses.push(format!("texte = ${}", bind_index));
            bind_values_str.push(t.to_string());
            bind_types.push("str");
            bind_index += 1;
        }
    }
    if let Some(debut) = body.debut_ms {
        set_clauses.push(format!("debut_ms = ${}", bind_index));
        bind_values_i32.push(debut);
        bind_types.push("i32");
        bind_index += 1;
    }
    if let Some(fin) = body.fin_ms {
        set_clauses.push(format!("fin_ms = ${}", bind_index));
        bind_values_i32.push(fin);
        bind_types.push("i32");
        bind_index += 1;
    }

    if set_clauses.is_empty() {
        return Err(ApiErreur::Validation("Aucun champ à modifier".into()));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE media_content.segment_sous_titre SET {} WHERE id = ${}",
        set_clauses.join(", "),
        bind_index
    );

    let mut query = sqlx::query(&sql);
    let mut str_idx = 0;
    let mut i32_idx = 0;
    for t in &bind_types {
        match *t {
            "str" => {
                query = query.bind(&bind_values_str[str_idx]);
                str_idx += 1;
            }
            "i32" => {
                query = query.bind(bind_values_i32[i32_idx]);
                i32_idx += 1;
            }
            _ => {}
        }
    }
    query = query.bind(id);

    let result = query.execute(pool.get_ref()).await?;
    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Segment non trouvé".into()));
    }

    log::info!("Admin {} a modifié le segment {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "media_content", "segment_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/admin/vidafrica/segments/{id}
pub async fn supprimer_segment(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "supprimer");
    let id = path.into_inner();

    // Suppression physique (CASCADE sur timing_mot)
    let result = sqlx::query(
        "DELETE FROM media_content.segment_sous_titre WHERE id = $1"
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve("Segment non trouvé".into()));
    }

    log::info!("Admin {} a supprimé le segment {}", admin.id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "media_content", "segment_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// PUT /api/admin/vidafrica/pistes/{piste_id}/segments/reordonner
pub async fn reordonner_segments(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ReordonnerSegmentsRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "modifier");
    let piste_id = path.into_inner();

    if body.ordre.is_empty() {
        return Err(ApiErreur::Validation("La liste d'ordre ne peut pas être vide".into()));
    }

    // Mettre à jour les positions une par une dans une transaction
    let mut tx = pool.begin().await?;

    // Désactiver temporairement la contrainte unique en mettant des positions négatives
    for (i, segment_id) in body.ordre.iter().enumerate() {
        sqlx::query(
            "UPDATE media_content.segment_sous_titre SET position = $1, updated_at = NOW()
             WHERE id = $2 AND piste_id = $3"
        )
        .bind(-(i as i32 + 1))
        .bind(segment_id)
        .bind(piste_id)
        .execute(&mut *tx)
        .await?;
    }

    // Remettre les positions positives
    for (i, segment_id) in body.ordre.iter().enumerate() {
        sqlx::query(
            "UPDATE media_content.segment_sous_titre SET position = $1
             WHERE id = $2 AND piste_id = $3"
        )
        .bind(i as i32 + 1)
        .bind(segment_id)
        .bind(piste_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    log::info!("Admin {} a réordonné les segments de la piste {}", admin.id, piste_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "media_content", "segment_sous_titre",
        None, None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "piste_id": piste_id })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// TIMINGS MOT (TAP-TO-MARK)
// ══════════════════════════════════════════════════════════════

/// POST /api/admin/vidafrica/segments/{segment_id}/timings-mot
pub async fn enregistrer_timings_mot(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<EnregistrerTimingsMotRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "modifier");
    let segment_id = path.into_inner();

    if body.timings.is_empty() {
        return Err(ApiErreur::Validation("La liste de timings ne peut pas être vide".into()));
    }

    // Valider chaque timing
    for t in &body.timings {
        if t.debut_ms < 0 {
            return Err(ApiErreur::Validation("Les timestamps doivent être positifs".into()));
        }
        if t.debut_ms >= t.fin_ms {
            return Err(ApiErreur::Validation(
                format!("Mot '{}': début doit être < fin", t.mot)
            ));
        }
    }

    // Vérifier que le segment existe et récupérer son piste_id
    let piste_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT piste_id FROM media_content.segment_sous_titre WHERE id = $1"
    )
    .bind(segment_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Segment non trouvé".into()))?;

    let mut tx = pool.begin().await?;

    // Supprimer les timings existants
    sqlx::query("DELETE FROM media_content.timing_mot WHERE segment_id = $1")
        .bind(segment_id)
        .execute(&mut *tx)
        .await?;

    // Insérer les nouveaux timings
    for t in &body.timings {
        sqlx::query(
            "INSERT INTO media_content.timing_mot (id, segment_id, position, mot, debut_ms, fin_ms)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(Uuid::new_v4())
        .bind(segment_id)
        .bind(t.position)
        .bind(&t.mot)
        .bind(t.debut_ms)
        .bind(t.fin_ms)
        .execute(&mut *tx)
        .await?;
    }

    // Mettre à jour est_complete sur la piste si tous les segments ont des timings
    let tous_complets = sqlx::query_scalar::<_, bool>(
        "SELECT NOT EXISTS(
            SELECT 1 FROM media_content.segment_sous_titre s
            WHERE s.piste_id = $1
            AND NOT EXISTS(
                SELECT 1 FROM media_content.timing_mot t WHERE t.segment_id = s.id
            )
        )"
    )
    .bind(piste_id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE media_content.piste_sous_titre SET est_complete = $1, updated_at = NOW() WHERE id = $2"
    )
    .bind(tous_complets)
    .bind(piste_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    log::info!("Admin {} a enregistré {} timings mot pour le segment {}", admin.id, body.timings.len(), segment_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "media_content", "timing_mot",
        Some(segment_id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "segment_id": segment_id,
            "nombre_timings": body.timings.len()
        })),
        error: None,
    }))
}

/// DELETE /api/admin/vidafrica/segments/{segment_id}/timings-mot
pub async fn supprimer_timings_mot(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media_content", "supprimer");
    let segment_id = path.into_inner();

    // Récupérer le piste_id avant suppression
    let piste_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT piste_id FROM media_content.segment_sous_titre WHERE id = $1"
    )
    .bind(segment_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Segment non trouvé".into()))?;

    sqlx::query("DELETE FROM media_content.timing_mot WHERE segment_id = $1")
        .bind(segment_id)
        .execute(pool.get_ref())
        .await?;

    // Mettre à jour est_complete
    sqlx::query(
        "UPDATE media_content.piste_sous_titre SET est_complete = false, updated_at = NOW() WHERE id = $1"
    )
    .bind(piste_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Admin {} a supprimé les timings mot du segment {}", admin.id, segment_id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "media_content", "timing_mot",
        Some(segment_id), None, None, ip.as_deref(), ua.as_deref(),
    ).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "segment_id": segment_id })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// FONCTIONS UTILITAIRES
// ══════════════════════════════════════════════════════════════

/// Lire le contenu texte d'un champ multipart
async fn lire_champ_texte(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        contenu.extend_from_slice(&data);
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

/// Sauvegarder un fichier uploadé sur le disque local, retourne la taille en octets
async fn sauvegarder_fichier(field: &mut actix_multipart::Field, chemin: &str) -> Result<usize, ApiErreur> {
    if let Some(parent) = std::path::Path::new(chemin).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le répertoire: {}", e)))?;
    }

    let mut fichier = std::fs::File::create(chemin)
        .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le fichier: {}", e)))?;

    let mut taille: usize = 0;
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
        taille += data.len();
        fichier
            .write_all(&data)
            .map_err(|e| ApiErreur::Upload(format!("Erreur écriture fichier: {}", e)))?;
    }

    Ok(taille)
}
