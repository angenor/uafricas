//! Contributions membres Vidafrica.
//!
//! Permet à tout utilisateur connecté (JWT) de :
//!   - proposer une nouvelle vidéo (créée en `brouillon`, validée ensuite par un admin) ;
//!   - contribuer une piste de sous-titres sur une vidéo (piste créée en `brouillon`,
//!     invisible au public jusqu'à publication par un admin) — segments + timings karaoké.
//!
//! L'authentification suit le même schéma que le marché membre (`handlers/annonces.rs`) :
//! le JWT n'est émis qu'aux comptes `actif`, l'`id` est extrait du header `Authorization`.
//! Les mutations de sous-titres exigent que la piste appartienne à l'utilisateur courant
//! ET soit encore en `brouillon` (une piste publiée n'est plus éditable par le membre).

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use sqlx::PgPool;
use std::io::Write;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::admin::vidafrica::{
    generer_slug, AdminPisteSousTitreResponse, AdminSegmentRow, AdminSegmentSousTitreResponse,
    CreerPisteRequest, CreerSegmentRequest, EnregistrerTimingsMotRequest, ModifierSegmentRequest,
    TimingMotResponse, LANGUES_VALIDES,
};
use crate::models::vidafrica::{
    PartageVideoRequest, PartageVideoResponse, PartageVideoRow, VideoReactionRequest,
    VideoReactionResponse,
};
use crate::services::audit;
use crate::ApiResponse;

// ══════════════════════════════════════════════════════════════
// AUTHENTIFICATION & CONTRÔLE DE PROPRIÉTÉ
// ══════════════════════════════════════════════════════════════

/// Extraire l'utilisateur connecté depuis le header Authorization (JWT Bearer).
fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

fn utilisateur_courant(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

/// Vérifie qu'une piste appartient à l'utilisateur courant et est éditable.
/// L'auteur peut éditer sa piste en `brouillon` comme en `publie` (modifications en
/// direct, sans nouvelle validation) ; seules les pistes `masque` (masquées par un
/// administrateur) sont gelées. Retourne le `video_id` de la piste si OK.
async fn verifier_piste_modifiable(
    pool: &PgPool,
    piste_id: Uuid,
    user_id: Uuid,
) -> Result<Uuid, ApiErreur> {
    let row = sqlx::query_as::<_, (Uuid, Uuid, String)>(
        "SELECT video_id, cree_par, etat FROM media_content.piste_sous_titre
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(piste_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Piste non trouvée".into()))?;

    let (video_id, cree_par, etat) = row;
    if cree_par != user_id {
        return Err(ApiErreur::AccesInterdit(
            "Vous n'êtes pas l'auteur de cette piste".into(),
        ));
    }
    if etat == "masque" {
        return Err(ApiErreur::AccesInterdit(
            "Cette piste a été masquée par un administrateur et n'est plus modifiable.".into(),
        ));
    }
    Ok(video_id)
}

/// Résout l'id de piste d'un segment puis vérifie qu'elle est modifiable par l'utilisateur.
async fn verifier_segment_modifiable(
    pool: &PgPool,
    segment_id: Uuid,
    user_id: Uuid,
) -> Result<Uuid, ApiErreur> {
    let piste_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT piste_id FROM media_content.segment_sous_titre WHERE id = $1",
    )
    .bind(segment_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Segment non trouvé".into()))?;

    verifier_piste_modifiable(pool, piste_id, user_id).await?;
    Ok(piste_id)
}

// ══════════════════════════════════════════════════════════════
// PROPOSITION DE VIDÉO (multipart)
// ══════════════════════════════════════════════════════════════

/// POST /api/vidafrica/videos — un membre propose une vidéo (état initial `brouillon`).
pub async fn proposer_video(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    upload_dir: web::Data<String>,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;

    let mut titre: Option<String> = None;
    let mut description: Option<String> = None;
    let mut fichier_video_url: Option<String> = None;
    let mut vignette_url: Option<String> = None;
    let mut taille_octets: Option<i64> = None;
    let mut format_video: Option<String> = None;
    let mut territoires: Vec<String> = Vec::new();
    let mut auteur_reel: Option<String> = None;
    let mut langue_originale: Option<String> = None;
    let mut decharge_droits = false;

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|e| ApiErreur::Upload(format!("Erreur lecture multipart: {}", e)))?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();

        match nom_champ.as_str() {
            "titre" => titre = Some(lire_champ_texte(&mut field).await?),
            "description" => description = Some(lire_champ_texte(&mut field).await?),
            "auteur_reel" => auteur_reel = Some(lire_champ_texte(&mut field).await?),
            // Langue parlée/chantée dans la vidéo (texte libre : langues Afrolang + « Autre »).
            "langue_originale" => langue_originale = Some(lire_champ_texte(&mut field).await?),
            // Décharge : « Je ne suis pas l'auteur de cette chanson… » — accepté si "true".
            "decharge_droits" => {
                let v = lire_champ_texte(&mut field).await?;
                decharge_droits = matches!(v.trim(), "true" | "1" | "on");
            }
            // Territoires : champ multipart répété (une entrée par territoire).
            "territoires" => {
                let t = lire_champ_texte(&mut field).await?.trim().to_string();
                if !t.is_empty() {
                    territoires.push(t);
                }
            }
            "fichier_video" => {
                let filename = content_disposition
                    .as_ref()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("video.mp4")
                    .to_string();

                let ext = filename.rsplit('.').next().unwrap_or("mp4").to_lowercase();
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

                let ext = filename.rsplit('.').next().unwrap_or("jpg").to_lowercase();
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

    let titre = titre
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .ok_or_else(|| ApiErreur::Validation("Le titre est requis".into()))?;

    let fichier_video_url = fichier_video_url
        .ok_or_else(|| ApiErreur::Validation("Le fichier vidéo est requis".into()))?;

    // La décharge de droits doit être acceptée pour proposer une vidéo.
    if !decharge_droits {
        return Err(ApiErreur::Validation(
            "Vous devez accepter la mention « Je ne suis pas l'auteur de cette chanson et ne \
             revendique aucun droit à ce sujet »"
                .into(),
        ));
    }

    let id = Uuid::new_v4();
    let slug = generer_slug(&titre);
    let description = description
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty());
    let auteur_reel = auteur_reel
        .map(|a| a.trim().to_string())
        .filter(|a| !a.is_empty());
    let langue_originale = langue_originale
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty());

    // État explicite 'brouillon' → file de modération admin.
    sqlx::query(
        "INSERT INTO media_content.video
         (id, titre, slug, description, fichier_video_url, vignette_url,
          taille_octets, format_video, territoires, decharge_droits, auteur_reel,
          langue_originale, etat, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 'brouillon', $13)",
    )
    .bind(id)
    .bind(&titre)
    .bind(&slug)
    .bind(&description)
    .bind(&fichier_video_url)
    .bind(&vignette_url)
    .bind(taille_octets)
    .bind(&format_video)
    .bind(&territoires)
    .bind(decharge_droits)
    .bind(&auteur_reel)
    .bind(&langue_originale)
    .bind(user_id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Membre {} a proposé la vidéo {} ({})", user_id, titre, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(user_id), "CREATE", "media_content", "video",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id,
            "slug": slug,
            "titre": titre,
            "etat": "brouillon",
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// RÉACTIONS (like / dislike) & PARTAGE
// ══════════════════════════════════════════════════════════════

/// Vérifie qu'une vidéo existe et est publiée. Retourne une erreur sinon.
async fn verifier_video_publiee(pool: &PgPool, video_id: Uuid) -> Result<(), ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.video
            WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL
         )",
    )
    .bind(video_id)
    .fetch_one(pool)
    .await?;

    if !existe {
        return Err(ApiErreur::NonTrouve("Vidéo non trouvée".into()));
    }
    Ok(())
}

/// Compte les likes/dislikes d'une vidéo (calculés à la lecture).
async fn compter_reactions_video(pool: &PgPool, video_id: Uuid) -> Result<(i64, i64), ApiErreur> {
    let counts: (i64, i64) = sqlx::query_as(
        "SELECT
            COUNT(*) FILTER (WHERE type_reaction = 'like'),
            COUNT(*) FILTER (WHERE type_reaction = 'dislike')
         FROM media_content.video_reaction WHERE video_id = $1",
    )
    .bind(video_id)
    .fetch_one(pool)
    .await?;
    Ok(counts)
}

/// POST /api/vidafrica/videos/{id}/reaction — aimer / ne pas aimer (toggle).
pub async fn reagir_video(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<VideoReactionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let video_id = path.into_inner();

    let type_reaction = body.type_reaction.trim().to_lowercase();
    if type_reaction != "like" && type_reaction != "dislike" {
        return Err(ApiErreur::Validation(
            "Type de réaction invalide (like ou dislike)".into(),
        ));
    }

    verifier_video_publiee(pool.get_ref(), video_id).await?;

    let reaction_existante: Option<String> = sqlx::query_scalar(
        "SELECT type_reaction FROM media_content.video_reaction
         WHERE video_id = $1 AND utilisateur_id = $2",
    )
    .bind(video_id)
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let ma_reaction: Option<String> = match reaction_existante {
        Some(ancien) if ancien == type_reaction => {
            // Bascule : retirer la réaction (même type).
            sqlx::query(
                "DELETE FROM media_content.video_reaction
                 WHERE video_id = $1 AND utilisateur_id = $2",
            )
            .bind(video_id)
            .bind(user_id)
            .execute(pool.get_ref())
            .await?;
            None
        }
        Some(_) => {
            // Changer le type (like ↔ dislike).
            sqlx::query(
                "UPDATE media_content.video_reaction
                 SET type_reaction = $1, updated_at = NOW()
                 WHERE video_id = $2 AND utilisateur_id = $3",
            )
            .bind(&type_reaction)
            .bind(video_id)
            .bind(user_id)
            .execute(pool.get_ref())
            .await?;
            Some(type_reaction.clone())
        }
        None => {
            // Nouvelle réaction.
            sqlx::query(
                "INSERT INTO media_content.video_reaction (video_id, utilisateur_id, type_reaction)
                 VALUES ($1, $2, $3)",
            )
            .bind(video_id)
            .bind(user_id)
            .bind(&type_reaction)
            .execute(pool.get_ref())
            .await?;
            Some(type_reaction.clone())
        }
    };

    let (nombre_likes, nombre_dislikes) = compter_reactions_video(pool.get_ref(), video_id).await?;

    // Engagement : 1 point à l'auteur de la vidéo par « j'aime » reçu (non-bloquant).
    if type_reaction == "like" {
        if let Ok(Some(cree_par)) = sqlx::query_scalar::<_, Uuid>(
            "SELECT cree_par FROM media_content.video WHERE id = $1",
        )
        .bind(video_id)
        .fetch_optional(pool.get_ref())
        .await
        {
            crate::services::engagement::crediter_jaime(
                pool.get_ref(),
                "video",
                video_id,
                cree_par,
                user_id,
            )
            .await;
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(VideoReactionResponse {
            nombre_likes,
            nombre_dislikes,
            ma_reaction,
        }),
        error: None,
    }))
}

/// POST /api/vidafrica/videos/{id}/partage — partager une vidéo sur le mur.
pub async fn partager_video(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<PartageVideoRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let video_id = path.into_inner();

    verifier_video_publiee(pool.get_ref(), video_id).await?;

    let legende = body
        .legende
        .as_deref()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string());

    if let Some(ref l) = legende {
        if l.chars().count() > 500 {
            return Err(ApiErreur::Validation(
                "La légende ne doit pas dépasser 500 caractères".into(),
            ));
        }
    }

    let partage_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.partage_video (video_id, utilisateur_id, legende)
         VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(video_id)
    .bind(user_id)
    .bind(&legende)
    .fetch_one(pool.get_ref())
    .await?;

    // Engagement : 1 point à l'auteur de la vidéo par partage reçu (non-bloquant).
    if let Some(auteur_id) =
        crate::services::engagement::resoudre_beneficiaire(pool.get_ref(), "video", video_id).await
    {
        crate::services::engagement::crediter_partage(
            pool.get_ref(),
            "video",
            video_id,
            auteur_id,
            user_id,
        )
        .await;
    }

    let row = sqlx::query_as::<_, PartageVideoRow>(&format!(
        "{} AND pv.id = $1",
        crate::handlers::vidafrica::PARTAGE_VIDEO_SELECT
    ))
    .bind(partage_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Partage non trouvé".into()))?;

    log::info!(
        "Membre {} a partagé la vidéo {} (partage {})",
        user_id, video_id, partage_id
    );

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PartageVideoResponse::from(row)),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// PISTES DE SOUS-TITRES (membre)
// ══════════════════════════════════════════════════════════════

/// GET /api/vidafrica/videos/{video_id}/mes-pistes
/// Retourne les langues déjà prises (toutes pistes non supprimées) et les pistes
/// dont l'utilisateur courant est l'auteur (pour reprendre une contribution).
pub async fn mes_pistes(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let video_id = path.into_inner();

    // Langues déjà prises par CE membre (un membre = une piste par langue ; d'autres
    // membres peuvent proposer la même langue de leur côté).
    let langues_prises = sqlx::query_scalar::<_, String>(
        "SELECT langue::TEXT FROM media_content.piste_sous_titre
         WHERE video_id = $1 AND cree_par = $2 AND deleted_at IS NULL",
    )
    .bind(video_id)
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await?;

    let mes_pistes = sqlx::query_as::<_, AdminPisteSousTitreResponse>(
        "SELECT p.id, p.langue::TEXT AS langue, p.est_complete,
                p.etat, p.cree_par, u.nom || ' ' || u.prenom AS cree_par_nom,
                (SELECT COUNT(*) FROM media_content.segment_sous_titre s
                 WHERE s.piste_id = p.id)::INTEGER AS nombre_segments,
                p.created_at
         FROM media_content.piste_sous_titre p
         JOIN iam.utilisateur u ON u.id = p.cree_par
         WHERE p.video_id = $1 AND p.cree_par = $2 AND p.deleted_at IS NULL
         ORDER BY p.created_at",
    )
    .bind(video_id)
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "langues_prises": langues_prises,
            "pistes": mes_pistes,
        })),
        error: None,
    }))
}

/// POST /api/vidafrica/videos/{video_id}/pistes — créer une piste membre (`brouillon`).
pub async fn creer_piste_membre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerPisteRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let video_id = path.into_inner();

    let langue = body.langue.trim();
    if !LANGUES_VALIDES.contains(&langue) {
        return Err(ApiErreur::Validation(format!(
            "Langue invalide. Valeurs acceptées: {}",
            LANGUES_VALIDES.join(", ")
        )));
    }

    // La vidéo doit exister (publiée ou en cours de proposition par le membre).
    let existe = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM media_content.video WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(video_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Vidéo non trouvée".into()));
    }

    // Un même membre ne peut avoir qu'une seule piste par langue ; d'autres membres
    // peuvent proposer leur propre version dans cette langue (l'admin arbitre laquelle
    // est publiée — une seule piste publiée par langue).
    let doublon = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM media_content.piste_sous_titre
         WHERE video_id = $1 AND langue::TEXT = $2 AND cree_par = $3 AND deleted_at IS NULL)",
    )
    .bind(video_id)
    .bind(langue)
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await?;
    if doublon {
        return Err(ApiErreur::Conflit(format!(
            "Vous avez déjà une piste en {} pour cette vidéo",
            langue
        )));
    }

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO media_content.piste_sous_titre (id, video_id, langue, etat, cree_par)
         VALUES ($1, $2, $3::media_content.langue_sous_titre, 'brouillon', $4)",
    )
    .bind(id)
    .bind(video_id)
    .bind(langue)
    .bind(user_id)
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Membre {} a créé la piste {} ({}) pour la vidéo {}",
        user_id, id, langue, video_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(user_id), "CREATE", "media_content", "piste_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "langue": langue, "etat": "brouillon" })),
        error: None,
    }))
}

/// DELETE /api/vidafrica/pistes/{id} — supprimer sa propre piste (encore `brouillon`).
pub async fn supprimer_piste_membre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let id = path.into_inner();

    verifier_piste_modifiable(pool.get_ref(), id, user_id).await?;

    sqlx::query(
        "UPDATE media_content.piste_sous_titre SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?;

    log::info!("Membre {} a supprimé sa piste {}", user_id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(user_id), "DELETE", "media_content", "piste_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// SEGMENTS (membre)
// ══════════════════════════════════════════════════════════════

/// GET /api/vidafrica/pistes/{piste_id}/segments — segments de sa propre piste.
pub async fn lister_segments_membre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let piste_id = path.into_inner();

    verifier_piste_modifiable(pool.get_ref(), piste_id, user_id).await?;

    let segments = sqlx::query_as::<_, AdminSegmentRow>(
        "SELECT id, position, texte, debut_ms, fin_ms, created_at, updated_at
         FROM media_content.segment_sous_titre
         WHERE piste_id = $1 ORDER BY position",
    )
    .bind(piste_id)
    .fetch_all(pool.get_ref())
    .await?;

    let mut resultat: Vec<AdminSegmentSousTitreResponse> = Vec::new();
    for seg in &segments {
        let timings = sqlx::query_as::<_, TimingMotResponse>(
            "SELECT position, mot, debut_ms, fin_ms
             FROM media_content.timing_mot
             WHERE segment_id = $1 ORDER BY position",
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

/// POST /api/vidafrica/pistes/{piste_id}/segments — ajouter un segment à sa piste.
pub async fn creer_segment_membre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerSegmentRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let piste_id = path.into_inner();

    let texte = body.texte.trim();
    if texte.is_empty() {
        return Err(ApiErreur::Validation("Le texte est requis".into()));
    }
    if body.debut_ms < 0 {
        return Err(ApiErreur::Validation(
            "Le timestamp de début doit être positif".into(),
        ));
    }
    if body.debut_ms >= body.fin_ms {
        return Err(ApiErreur::Validation(
            "Le timestamp de début doit être inférieur à la fin".into(),
        ));
    }

    verifier_piste_modifiable(pool.get_ref(), piste_id, user_id).await?;

    // Interdire un chevauchement avec un segment existant. Les bornes qui se
    // touchent (fin de l'un = début de l'autre) sont autorisées : c'est le cas
    // des segments contigus du mode « au fil de la lecture ».
    let chevauche = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.segment_sous_titre
            WHERE piste_id = $1 AND debut_ms < $3 AND fin_ms > $2
         )",
    )
    .bind(piste_id)
    .bind(body.debut_ms)
    .bind(body.fin_ms)
    .fetch_one(pool.get_ref())
    .await?;
    if chevauche {
        return Err(ApiErreur::Validation(
            "Ce segment chevauche un segment existant de la piste".into(),
        ));
    }

    let next_pos = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT MAX(position) FROM media_content.segment_sous_titre WHERE piste_id = $1",
    )
    .bind(piste_id)
    .fetch_one(pool.get_ref())
    .await?
    .unwrap_or(0)
        + 1;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO media_content.segment_sous_titre (id, piste_id, position, texte, debut_ms, fin_ms)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(id)
    .bind(piste_id)
    .bind(next_pos)
    .bind(texte)
    .bind(body.debut_ms)
    .bind(body.fin_ms)
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Membre {} a créé le segment {} (position {}) pour la piste {}",
        user_id, id, next_pos, piste_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(user_id), "CREATE", "media_content", "segment_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "position": next_pos })),
        error: None,
    }))
}

/// PUT /api/vidafrica/segments/{id} — modifier un segment de sa piste.
pub async fn modifier_segment_membre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierSegmentRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let id = path.into_inner();

    verifier_segment_modifiable(pool.get_ref(), id, user_id).await?;

    // Construction dynamique du SET (mêmes règles que l'admin).
    let texte = body.texte.as_ref().map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
    if texte.is_none() && body.debut_ms.is_none() && body.fin_ms.is_none() {
        return Err(ApiErreur::Validation("Aucun champ à modifier".into()));
    }

    // Si les bornes changent : valider début < fin et l'absence de chevauchement
    // avec les AUTRES segments de la piste (bornes qui se touchent autorisées).
    if body.debut_ms.is_some() || body.fin_ms.is_some() {
        let (piste_id, cur_debut, cur_fin) = sqlx::query_as::<_, (Uuid, i32, i32)>(
            "SELECT piste_id, debut_ms, fin_ms FROM media_content.segment_sous_titre WHERE id = $1",
        )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await?;
        let new_debut = body.debut_ms.unwrap_or(cur_debut);
        let new_fin = body.fin_ms.unwrap_or(cur_fin);
        if new_debut < 0 || new_debut >= new_fin {
            return Err(ApiErreur::Validation(
                "Le timestamp de début doit être positif et inférieur à la fin".into(),
            ));
        }
        let chevauche = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(
                SELECT 1 FROM media_content.segment_sous_titre
                WHERE piste_id = $1 AND id <> $2 AND debut_ms < $4 AND fin_ms > $3
             )",
        )
        .bind(piste_id)
        .bind(id)
        .bind(new_debut)
        .bind(new_fin)
        .fetch_one(pool.get_ref())
        .await?;
        if chevauche {
            return Err(ApiErreur::Validation(
                "Ce segment chevauche un autre segment de la piste".into(),
            ));
        }
    }

    let mut tx = pool.begin().await?;
    if let Some(ref t) = texte {
        sqlx::query("UPDATE media_content.segment_sous_titre SET texte = $1, updated_at = NOW() WHERE id = $2")
            .bind(t)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }
    if let Some(debut) = body.debut_ms {
        sqlx::query("UPDATE media_content.segment_sous_titre SET debut_ms = $1, updated_at = NOW() WHERE id = $2")
            .bind(debut)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }
    if let Some(fin) = body.fin_ms {
        sqlx::query("UPDATE media_content.segment_sous_titre SET fin_ms = $1, updated_at = NOW() WHERE id = $2")
            .bind(fin)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;

    log::info!("Membre {} a modifié le segment {}", user_id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(user_id), "UPDATE", "media_content", "segment_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

/// DELETE /api/vidafrica/segments/{id} — supprimer un segment de sa piste.
pub async fn supprimer_segment_membre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let id = path.into_inner();

    verifier_segment_modifiable(pool.get_ref(), id, user_id).await?;

    // Suppression physique (CASCADE sur timing_mot).
    sqlx::query("DELETE FROM media_content.segment_sous_titre WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    log::info!("Membre {} a supprimé le segment {}", user_id, id);

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(user_id), "DELETE", "media_content", "segment_sous_titre",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// TIMINGS MOT (membre — tap-to-mark)
// ══════════════════════════════════════════════════════════════

/// POST /api/vidafrica/segments/{segment_id}/timings-mot — enregistrer les timings mot.
pub async fn enregistrer_timings_mot_membre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<EnregistrerTimingsMotRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let segment_id = path.into_inner();

    if body.timings.is_empty() {
        return Err(ApiErreur::Validation(
            "La liste de timings ne peut pas être vide".into(),
        ));
    }
    for t in &body.timings {
        if t.debut_ms < 0 {
            return Err(ApiErreur::Validation(
                "Les timestamps doivent être positifs".into(),
            ));
        }
        if t.debut_ms >= t.fin_ms {
            return Err(ApiErreur::Validation(format!(
                "Mot '{}': début doit être < fin",
                t.mot
            )));
        }
    }

    let piste_id = verifier_segment_modifiable(pool.get_ref(), segment_id, user_id).await?;

    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM media_content.timing_mot WHERE segment_id = $1")
        .bind(segment_id)
        .execute(&mut *tx)
        .await?;

    for t in &body.timings {
        sqlx::query(
            "INSERT INTO media_content.timing_mot (id, segment_id, position, mot, debut_ms, fin_ms)
             VALUES ($1, $2, $3, $4, $5, $6)",
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

    // Recalcul de est_complete sur la piste.
    let tous_complets = sqlx::query_scalar::<_, bool>(
        "SELECT NOT EXISTS(
            SELECT 1 FROM media_content.segment_sous_titre s
            WHERE s.piste_id = $1
            AND NOT EXISTS(
                SELECT 1 FROM media_content.timing_mot t WHERE t.segment_id = s.id
            )
        )",
    )
    .bind(piste_id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "UPDATE media_content.piste_sous_titre SET est_complete = $1, updated_at = NOW() WHERE id = $2",
    )
    .bind(tous_complets)
    .bind(piste_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    log::info!(
        "Membre {} a enregistré {} timings mot pour le segment {}",
        user_id, body.timings.len(), segment_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(user_id), "CREATE", "media_content", "timing_mot",
        Some(segment_id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "segment_id": segment_id,
            "nombre_timings": body.timings.len()
        })),
        error: None,
    }))
}

/// DELETE /api/vidafrica/segments/{segment_id}/timings-mot — supprimer les timings mot.
pub async fn supprimer_timings_mot_membre(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let user_id = utilisateur_courant(&req)?;
    let segment_id = path.into_inner();

    let piste_id = verifier_segment_modifiable(pool.get_ref(), segment_id, user_id).await?;

    sqlx::query("DELETE FROM media_content.timing_mot WHERE segment_id = $1")
        .bind(segment_id)
        .execute(pool.get_ref())
        .await?;

    sqlx::query(
        "UPDATE media_content.piste_sous_titre SET est_complete = false, updated_at = NOW() WHERE id = $1",
    )
    .bind(piste_id)
    .execute(pool.get_ref())
    .await?;

    log::info!(
        "Membre {} a supprimé les timings mot du segment {}",
        user_id, segment_id
    );

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(user_id), "DELETE", "media_content", "timing_mot",
        Some(segment_id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "segment_id": segment_id })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════
// FONCTIONS UTILITAIRES (multipart)
// ══════════════════════════════════════════════════════════════

async fn lire_champ_texte(field: &mut actix_multipart::Field) -> Result<String, ApiErreur> {
    let mut contenu = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture champ: {}", e)))?;
        contenu.extend_from_slice(&data);
    }
    String::from_utf8(contenu)
        .map_err(|e| ApiErreur::Upload(format!("Encodage UTF-8 invalide: {}", e)))
}

async fn sauvegarder_fichier(
    field: &mut actix_multipart::Field,
    chemin: &str,
) -> Result<usize, ApiErreur> {
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
