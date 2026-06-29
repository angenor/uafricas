use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::formation_contenu::{
    AdminChapitreResponse, AdminChapitreRow, AdminLeconResponse,
    CreerChapitreRequest, CreerLeconRequest, ModifierChapitreRequest, ModifierLeconRequest,
    ReordonnerRequest,
};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// Formats acceptés pour l'upload de média de leçon
const EXTENSIONS_VIDEO: &[&str] = &["mp4", "webm", "mov", "m4v", "ogv"];
const EXTENSIONS_DOC: &[&str] = &["pdf"];
const TAILLE_MAX_VIDEO: usize = 300 * 1024 * 1024; // 300 Mo
const TAILLE_MAX_DOC: usize = 30 * 1024 * 1024; //   30 Mo

// ──────────────────────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────────────────────

/// Vérifier qu'une formation existe (non supprimée).
async fn verifier_mooc(pool: &PgPool, mooc_id: Uuid) -> Result<(), ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM media_content.mooc WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(mooc_id)
    .fetch_one(pool)
    .await?;
    if existe {
        Ok(())
    } else {
        Err(ApiErreur::NonTrouve("Formation non trouvée".into()))
    }
}

/// Récupérer le mooc_id parent d'un chapitre (et vérifier l'existence).
async fn mooc_du_chapitre(pool: &PgPool, chapitre_id: Uuid) -> Result<Uuid, ApiErreur> {
    sqlx::query_scalar(
        "SELECT mooc_id FROM media_content.formation_chapitre
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(chapitre_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Chapitre non trouvé".into()))
}

/// Récupérer le chapitre_id parent d'une leçon (et vérifier l'existence).
async fn chapitre_de_la_lecon(pool: &PgPool, lecon_id: Uuid) -> Result<Uuid, ApiErreur> {
    sqlx::query_scalar(
        "SELECT chapitre_id FROM media_content.formation_lecon
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(lecon_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Leçon non trouvée".into()))
}

fn texte_optionnel(valeur: &Option<String>) -> Option<String> {
    valeur
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

async fn journaliser(
    pool: &PgPool,
    req: &HttpRequest,
    admin_id: Uuid,
    action: &str,
    table: &str,
    cible: Uuid,
) {
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(admin_id),
        action,
        "media_content",
        table,
        Some(cible),
        None,
        None,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}

/// Recalculer en masse la progression (et le statut) de tous les inscrits d'une
/// formation. À appeler après tout changement du NOMBRE de leçons (création /
/// suppression de leçon ou de chapitre) : la colonne `mooc_inscription.progression`
/// est dénormalisée et ne se recalcule sinon que lorsqu'un inscrit (dé)marque une
/// leçon. Idempotent.
async fn recalculer_progression_mooc(pool: &PgPool, mooc_id: Uuid) -> Result<(), ApiErreur> {
    sqlx::query(
        "UPDATE media_content.mooc_inscription mi
         SET progression = sub.prog,
             statut = CASE WHEN sub.prog >= 100 THEN 'complete'
                           WHEN sub.done > 0 THEN 'en_cours'
                           ELSE 'inscrit' END,
             updated_at = NOW()
         FROM (
             SELECT i.utilisateur_id,
                    COALESCE(d.done, 0) AS done,
                    CASE WHEN t.total > 0
                         THEN ROUND(COALESCE(d.done, 0)::numeric * 100 / t.total, 2)
                         ELSE 0 END AS prog
             FROM media_content.mooc_inscription i
             CROSS JOIN (
                 SELECT COUNT(*) AS total FROM media_content.formation_lecon l
                 JOIN media_content.formation_chapitre c ON l.chapitre_id = c.id
                 WHERE c.mooc_id = $1 AND c.deleted_at IS NULL AND l.deleted_at IS NULL
             ) t
             LEFT JOIN (
                 SELECT lc.utilisateur_id, COUNT(*) AS done
                 FROM media_content.formation_lecon_completion lc
                 JOIN media_content.formation_lecon l ON lc.lecon_id = l.id
                 JOIN media_content.formation_chapitre c ON l.chapitre_id = c.id
                 WHERE c.mooc_id = $1 AND c.deleted_at IS NULL AND l.deleted_at IS NULL
                 GROUP BY lc.utilisateur_id
             ) d ON d.utilisateur_id = i.utilisateur_id
             WHERE i.mooc_id = $1
         ) sub
         WHERE mi.mooc_id = $1 AND mi.utilisateur_id = sub.utilisateur_id
           AND mi.statut <> 'abandonne'",
    )
    .bind(mooc_id)
    .execute(pool)
    .await?;
    Ok(())
}

// ──────────────────────────────────────────────────────────────
// GET /api/admin/mooc/{id}/chapitres — Programme complet
// ──────────────────────────────────────────────────────────────
pub async fn lister_contenu(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "voir");
    let mooc_id = path.into_inner();
    verifier_mooc(pool.get_ref(), mooc_id).await?;

    let chapitres = sqlx::query_as::<_, AdminChapitreRow>(
        "SELECT id, titre, description, ordre FROM media_content.formation_chapitre
         WHERE mooc_id = $1 AND deleted_at IS NULL
         ORDER BY ordre ASC, created_at ASC",
    )
    .bind(mooc_id)
    .fetch_all(pool.get_ref())
    .await?;

    let lecons = sqlx::query_as::<_, AdminLeconResponse>(
        "SELECT l.id, l.chapitre_id, l.titre, l.contenu, l.video_url,
                l.document_url, l.duree_minutes, l.ordre
         FROM media_content.formation_lecon l
         JOIN media_content.formation_chapitre c ON l.chapitre_id = c.id
         WHERE c.mooc_id = $1 AND c.deleted_at IS NULL AND l.deleted_at IS NULL
         ORDER BY l.ordre ASC, l.created_at ASC",
    )
    .bind(mooc_id)
    .fetch_all(pool.get_ref())
    .await?;

    let reponse: Vec<AdminChapitreResponse> = chapitres
        .into_iter()
        .map(|ch| {
            let lecons_ch: Vec<AdminLeconResponse> = lecons
                .iter()
                .filter(|l| l.chapitre_id == ch.id)
                .map(|l| AdminLeconResponse {
                    id: l.id,
                    chapitre_id: l.chapitre_id,
                    titre: l.titre.clone(),
                    contenu: l.contenu.clone(),
                    video_url: l.video_url.clone(),
                    document_url: l.document_url.clone(),
                    duree_minutes: l.duree_minutes,
                    ordre: l.ordre,
                })
                .collect();
            AdminChapitreResponse {
                id: ch.id,
                titre: ch.titre,
                description: ch.description,
                ordre: ch.ordre,
                lecons: lecons_ch,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(reponse),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/admin/mooc/{id}/chapitres — Créer un chapitre
// ──────────────────────────────────────────────────────────────
pub async fn creer_chapitre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerChapitreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");
    let mooc_id = path.into_inner();
    verifier_mooc(pool.get_ref(), mooc_id).await?;

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre du chapitre est requis".into()));
    }

    let ordre: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(ordre), -1) + 1 FROM media_content.formation_chapitre
         WHERE mooc_id = $1 AND deleted_at IS NULL",
    )
    .bind(mooc_id)
    .fetch_one(pool.get_ref())
    .await?;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO media_content.formation_chapitre (id, mooc_id, titre, description, ordre)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(id)
    .bind(mooc_id)
    .bind(titre)
    .bind(texte_optionnel(&body.description))
    .bind(ordre)
    .execute(pool.get_ref())
    .await?;

    journaliser(pool.get_ref(), &req, admin.id, "CREATE", "formation_chapitre", id).await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "ordre": ordre })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// PUT /api/admin/chapitres/{chapitre_id} — Modifier un chapitre
// ──────────────────────────────────────────────────────────────
pub async fn modifier_chapitre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierChapitreRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");
    let chapitre_id = path.into_inner();
    mooc_du_chapitre(pool.get_ref(), chapitre_id).await?;

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre du chapitre est requis".into()));
    }

    sqlx::query(
        "UPDATE media_content.formation_chapitre
         SET titre = $1, description = $2, updated_at = NOW()
         WHERE id = $3 AND deleted_at IS NULL",
    )
    .bind(titre)
    .bind(texte_optionnel(&body.description))
    .bind(chapitre_id)
    .execute(pool.get_ref())
    .await?;

    journaliser(pool.get_ref(), &req, admin.id, "UPDATE", "formation_chapitre", chapitre_id).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": chapitre_id })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// DELETE /api/admin/chapitres/{chapitre_id} — Supprimer (soft) un chapitre
// + ses leçons
// ──────────────────────────────────────────────────────────────
pub async fn supprimer_chapitre(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "supprimer");
    let chapitre_id = path.into_inner();
    let mooc_id = mooc_du_chapitre(pool.get_ref(), chapitre_id).await?;

    sqlx::query(
        "UPDATE media_content.formation_lecon SET deleted_at = NOW(), updated_at = NOW()
         WHERE chapitre_id = $1 AND deleted_at IS NULL",
    )
    .bind(chapitre_id)
    .execute(pool.get_ref())
    .await?;

    sqlx::query(
        "UPDATE media_content.formation_chapitre SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(chapitre_id)
    .execute(pool.get_ref())
    .await?;

    journaliser(pool.get_ref(), &req, admin.id, "DELETE", "formation_chapitre", chapitre_id).await;
    recalculer_progression_mooc(pool.get_ref(), mooc_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// PUT /api/admin/mooc/{id}/chapitres/reordonner — Réordonner les chapitres
// ──────────────────────────────────────────────────────────────
pub async fn reordonner_chapitres(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ReordonnerRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");
    let mooc_id = path.into_inner();
    verifier_mooc(pool.get_ref(), mooc_id).await?;

    // Atomique : tout réordonner ou rien (évite un ordre partiellement réécrit).
    let mut tx = pool.get_ref().begin().await?;
    for (index, id) in body.ids.iter().enumerate() {
        sqlx::query(
            "UPDATE media_content.formation_chapitre SET ordre = $1, updated_at = NOW()
             WHERE id = $2 AND mooc_id = $3 AND deleted_at IS NULL",
        )
        .bind(index as i32)
        .bind(id)
        .bind(mooc_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/admin/chapitres/{chapitre_id}/lecons — Créer une leçon
// ──────────────────────────────────────────────────────────────
pub async fn creer_lecon(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<CreerLeconRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");
    let chapitre_id = path.into_inner();
    let mooc_id = mooc_du_chapitre(pool.get_ref(), chapitre_id).await?;

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre de la leçon est requis".into()));
    }

    let ordre: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(ordre), -1) + 1 FROM media_content.formation_lecon
         WHERE chapitre_id = $1 AND deleted_at IS NULL",
    )
    .bind(chapitre_id)
    .fetch_one(pool.get_ref())
    .await?;

    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO media_content.formation_lecon
         (id, chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(id)
    .bind(chapitre_id)
    .bind(titre)
    .bind(texte_optionnel(&body.contenu))
    .bind(texte_optionnel(&body.video_url))
    .bind(texte_optionnel(&body.document_url))
    .bind(body.duree_minutes)
    .bind(ordre)
    .execute(pool.get_ref())
    .await?;

    journaliser(pool.get_ref(), &req, admin.id, "CREATE", "formation_lecon", id).await;
    recalculer_progression_mooc(pool.get_ref(), mooc_id).await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": id, "ordre": ordre })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// PUT /api/admin/lecons/{lecon_id} — Modifier (remplacement complet)
// ──────────────────────────────────────────────────────────────
pub async fn modifier_lecon(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierLeconRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");
    let lecon_id = path.into_inner();
    chapitre_de_la_lecon(pool.get_ref(), lecon_id).await?;

    let titre = body.titre.trim();
    if titre.is_empty() {
        return Err(ApiErreur::Validation("Le titre de la leçon est requis".into()));
    }

    sqlx::query(
        "UPDATE media_content.formation_lecon
         SET titre = $1, contenu = $2, video_url = $3, document_url = $4,
             duree_minutes = $5, updated_at = NOW()
         WHERE id = $6 AND deleted_at IS NULL",
    )
    .bind(titre)
    .bind(texte_optionnel(&body.contenu))
    .bind(texte_optionnel(&body.video_url))
    .bind(texte_optionnel(&body.document_url))
    .bind(body.duree_minutes)
    .bind(lecon_id)
    .execute(pool.get_ref())
    .await?;

    journaliser(pool.get_ref(), &req, admin.id, "UPDATE", "formation_lecon", lecon_id).await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": lecon_id })),
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// DELETE /api/admin/lecons/{lecon_id} — Supprimer (soft)
// ──────────────────────────────────────────────────────────────
pub async fn supprimer_lecon(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "supprimer");
    let lecon_id = path.into_inner();
    let chapitre_id = chapitre_de_la_lecon(pool.get_ref(), lecon_id).await?;
    let mooc_id = mooc_du_chapitre(pool.get_ref(), chapitre_id).await?;

    sqlx::query(
        "UPDATE media_content.formation_lecon SET deleted_at = NOW(), updated_at = NOW()
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(lecon_id)
    .execute(pool.get_ref())
    .await?;

    journaliser(pool.get_ref(), &req, admin.id, "DELETE", "formation_lecon", lecon_id).await;
    recalculer_progression_mooc(pool.get_ref(), mooc_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// PUT /api/admin/chapitres/{chapitre_id}/lecons/reordonner
// ──────────────────────────────────────────────────────────────
pub async fn reordonner_lecons(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ReordonnerRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "mooc", "modifier");
    let chapitre_id = path.into_inner();
    mooc_du_chapitre(pool.get_ref(), chapitre_id).await?;

    // Atomique : tout réordonner ou rien.
    let mut tx = pool.get_ref().begin().await?;
    for (index, id) in body.ids.iter().enumerate() {
        sqlx::query(
            "UPDATE media_content.formation_lecon SET ordre = $1, updated_at = NOW()
             WHERE id = $2 AND chapitre_id = $3 AND deleted_at IS NULL",
        )
        .bind(index as i32)
        .bind(id)
        .bind(chapitre_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ──────────────────────────────────────────────────────────────
// POST /api/admin/mooc/upload — Téléverser une vidéo ou un PDF de leçon
// Renvoie { url } (chemin relatif /uploads/formations/...)
// ──────────────────────────────────────────────────────────────
pub async fn uploader_fichier_formation(
    admin: AdminUtilisateur,
    upload_dir: web::Data<String>,
    mut payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ApiErreur> {
    use futures_util::StreamExt;
    use std::io::Write;

    verifier_permission!(admin, "mooc", "modifier");

    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| ApiErreur::Upload(format!("Champ multipart invalide: {}", e)))?;

        let content_disposition = field.content_disposition().cloned();
        let nom_champ = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_name())
            .unwrap_or("")
            .to_string();
        if nom_champ != "fichier" {
            while let Some(chunk) = field.next().await {
                let _ = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture: {}", e)))?;
            }
            continue;
        }

        let nom_fichier = content_disposition
            .as_ref()
            .and_then(|cd| cd.get_filename())
            .map(|s| s.to_string())
            .unwrap_or_default();
        let ext = std::path::Path::new(&nom_fichier)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        let (sous_dossier, taille_max) = if EXTENSIONS_VIDEO.contains(&ext.as_str()) {
            ("videos", TAILLE_MAX_VIDEO)
        } else if EXTENSIONS_DOC.contains(&ext.as_str()) {
            ("documents", TAILLE_MAX_DOC)
        } else {
            return Err(ApiErreur::Validation(format!(
                "Format de fichier non supporté: « {} ». Vidéos: {} ; Documents: {}.",
                if ext.is_empty() { "inconnu" } else { &ext },
                EXTENSIONS_VIDEO.join(", "),
                EXTENSIONS_DOC.join(", "),
            )));
        };

        let nom_stocke = format!("{}.{}", Uuid::new_v4(), ext);
        let chemin_relatif = format!("/uploads/formations/{}/{}", sous_dossier, nom_stocke);
        let chemin_complet = format!("{}/formations/{}/{}", upload_dir.get_ref(), sous_dossier, nom_stocke);

        if let Some(parent) = std::path::Path::new(&chemin_complet).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le répertoire: {}", e)))?;
        }
        let mut fichier = std::fs::File::create(&chemin_complet)
            .map_err(|e| ApiErreur::Upload(format!("Impossible de créer le fichier: {}", e)))?;

        let mut taille: usize = 0;
        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|e| ApiErreur::Upload(format!("Erreur lecture fichier: {}", e)))?;
            taille += data.len();
            if taille > taille_max {
                drop(fichier);
                let _ = std::fs::remove_file(&chemin_complet);
                return Err(ApiErreur::Validation(format!(
                    "Fichier trop volumineux (max {} Mo)",
                    taille_max / (1024 * 1024)
                )));
            }
            fichier
                .write_all(&data)
                .map_err(|e| ApiErreur::Upload(format!("Erreur écriture fichier: {}", e)))?;
        }

        if taille == 0 {
            let _ = std::fs::remove_file(&chemin_complet);
            return Err(ApiErreur::Validation("Fichier vide".into()));
        }

        log::info!("Admin {} a uploadé un fichier de formation {} ({} octets)", admin.id, chemin_relatif, taille);

        return Ok(HttpResponse::Created().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "url": chemin_relatif })),
            error: None,
        }));
    }

    Err(ApiErreur::Validation("Aucun fichier reçu (champ `fichier`).".into()))
}
