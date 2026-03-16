// ════════════════════════════════════════════════════════════════════════════
// Handlers — Notifications et doublons
// ════════════════════════════════════════════════════════════════════════════

use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::models::notification::*;
use crate::services::audit;
use crate::ApiResponse;

fn extraire_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let header = req.headers().get("Authorization").and_then(|h| h.to_str().ok())
        .ok_or_else(|| ApiErreur::NonAutorise("Token manquant".into()))?;
    let token = crate::jwt::extraire_token_du_header(header)?;
    let jwt_config = req.app_data::<web::Data<crate::config::JwtConfig>>()
        .ok_or_else(|| ApiErreur::BaseDeDonnees("Config JWT manquante".into()))?;
    let claims = crate::jwt::valider_token(token, &jwt_config.secret)?;
    claims.sub.parse::<Uuid>().map_err(|_| ApiErreur::NonAutorise("ID invalide".into()))
}

// GET /api/notifications/compteur
pub async fn compteur_notifications(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;
    let (nb,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM arbre_genealogique.notifications WHERE destinataire_id = $1 AND lu = FALSE")
        .bind(uid).fetch_one(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"non_lues": nb})), error: None }))
}

// GET /api/notifications
pub async fn lister_notifications(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;
    let notifs = sqlx::query_as::<_, NotificationRow>(
        "SELECT id, destinataire_id, type AS type_, message, lien_action, lu, created_at
         FROM arbre_genealogique.notifications WHERE destinataire_id = $1 ORDER BY created_at DESC LIMIT 50",
    ).bind(uid).fetch_all(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(notifs), error: None }))
}

// POST /api/notifications/{id}/lire
pub async fn marquer_lue(req: HttpRequest, pool: web::Data<PgPool>, chemin: web::Path<Uuid>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;
    let id = chemin.into_inner();
    sqlx::query("UPDATE arbre_genealogique.notifications SET lu = TRUE WHERE id = $1 AND destinataire_id = $2")
        .bind(id).bind(uid).execute(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"message": "Notification marquée comme lue"})), error: None }))
}

// POST /api/notifications/tout-lire
pub async fn tout_marquer_lu(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;
    let result = sqlx::query("UPDATE arbre_genealogique.notifications SET lu = TRUE WHERE destinataire_id = $1 AND lu = FALSE")
        .bind(uid).execute(pool.get_ref()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"nb_marquees": result.rows_affected()})), error: None }))
}

// GET /api/arbre/doublons
pub async fn detecter_doublons(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;

    let arbre_id: Option<Uuid> = sqlx::query_scalar("SELECT id FROM arbre_genealogique.arbres WHERE utilisateur_id = $1 AND deleted_at IS NULL")
        .bind(uid).fetch_optional(pool.get_ref()).await?;
    let Some(arbre_id) = arbre_id else {
        return Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"doublons": [], "total": 0})), error: None }));
    };

    // Comparer les personnes du même arbre entre elles via pg_trgm
    let doublons: Vec<(Uuid, String, Option<String>, Option<i16>, Option<String>, Uuid, String, Option<String>, Option<i16>, Option<String>, f64)> = sqlx::query_as(
        "SELECT p1.id, p1.nom, p1.prenoms, p1.naissance_annee, p1.naissance_lieu,
                p2.id, p2.nom, p2.prenoms, p2.naissance_annee, p2.naissance_lieu,
                (0.50 * similarity(p1.nom_normalise, p2.nom_normalise)
                 + 0.30 * COALESCE(similarity(p1.prenoms_normalise, p2.prenoms_normalise), 0.0)
                 + 0.20 * CASE
                    WHEN p1.naissance_annee IS NULL OR p2.naissance_annee IS NULL THEN 0.4
                    ELSE EXP(-POWER(p1.naissance_annee - p2.naissance_annee, 2) / 50.0)
                  END
                ) AS score
         FROM arbre_genealogique.personnes p1
         JOIN arbre_genealogique.rattachements r1 ON r1.personne_id = p1.id AND r1.arbre_id = $1 AND r1.deleted_at IS NULL
         JOIN arbre_genealogique.personnes p2 ON p2.id > p1.id
         JOIN arbre_genealogique.rattachements r2 ON r2.personne_id = p2.id AND r2.arbre_id = $1 AND r2.deleted_at IS NULL
         WHERE p1.deleted_at IS NULL AND p2.deleted_at IS NULL
           AND p1.nom_normalise IS NOT NULL AND p2.nom_normalise IS NOT NULL
           AND p1.nom_normalise % p2.nom_normalise
           AND NOT EXISTS (
               SELECT 1 FROM arbre_genealogique.doublons_ignores di
               WHERE di.arbre_id = $1
                 AND LEAST(di.personne_a_id, di.personne_b_id) = LEAST(p1.id, p2.id)
                 AND GREATEST(di.personne_a_id, di.personne_b_id) = GREATEST(p1.id, p2.id)
           )
         AND (0.50 * similarity(p1.nom_normalise, p2.nom_normalise)
                 + 0.30 * COALESCE(similarity(p1.prenoms_normalise, p2.prenoms_normalise), 0.0)
                 + 0.20 * CASE
                    WHEN p1.naissance_annee IS NULL OR p2.naissance_annee IS NULL THEN 0.4
                    ELSE EXP(-POWER(p1.naissance_annee - p2.naissance_annee, 2) / 50.0)
                  END) >= 0.70
         ORDER BY score DESC
         LIMIT 20",
    ).bind(arbre_id).fetch_all(pool.get_ref()).await?;

    let resultats: Vec<serde_json::Value> = doublons.into_iter().map(|(a_id, a_nom, a_pre, a_an, a_lieu, b_id, b_nom, b_pre, b_an, b_lieu, score)| {
        json!({
            "personne_a": {"id": a_id, "nom": a_nom, "prenoms": a_pre, "naissance_annee": a_an, "naissance_lieu": a_lieu},
            "personne_b": {"id": b_id, "nom": b_nom, "prenoms": b_pre, "naissance_annee": b_an, "naissance_lieu": b_lieu},
            "score": score
        })
    }).collect();

    let total = resultats.len();
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"doublons": resultats, "total": total})), error: None }))
}

// POST /api/arbre/doublons/ignorer
pub async fn ignorer_doublon(req: HttpRequest, pool: web::Data<PgPool>, body: web::Json<IgnorerDoublonDto>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;
    let arbre_id: Uuid = sqlx::query_scalar("SELECT id FROM arbre_genealogique.arbres WHERE utilisateur_id = $1 AND deleted_at IS NULL")
        .bind(uid).fetch_one(pool.get_ref()).await?;

    sqlx::query("INSERT INTO arbre_genealogique.doublons_ignores (arbre_id, personne_a_id, personne_b_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
        .bind(arbre_id).bind(body.personne_a_id).bind(body.personne_b_id).execute(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"message": "Doublon ignoré"})), error: None }))
}

// POST /api/arbre/doublons/fusionner
pub async fn fusionner_doublons(req: HttpRequest, pool: web::Data<PgPool>, body: web::Json<FusionDoublonDto>) -> Result<HttpResponse, ApiErreur> {
    let uid = extraire_utilisateur_id(&req)?;

    let mut tx = pool.begin().await?;

    // 1. Mettre à jour la personne gardée avec les champs choisis
    sqlx::query(
        "UPDATE arbre_genealogique.personnes SET
            nom = $1, prenoms = $2, genre = $3,
            naissance_annee = $4, naissance_mois = $5, naissance_jour = $6, naissance_lieu = $7,
            deces_annee = $8, deces_mois = $9, deces_jour = $10, deces_lieu = $11,
            updated_at = NOW()
         WHERE id = $1",
    )
    .bind(&body.nom).bind(&body.prenoms).bind(&body.genre)
    .bind(body.naissance_annee).bind(body.naissance_mois).bind(body.naissance_jour).bind(&body.naissance_lieu)
    .bind(body.deces_annee).bind(body.deces_mois).bind(body.deces_jour).bind(&body.deces_lieu)
    .bind(body.personne_a_garder_id)
    .execute(&mut *tx).await?;

    // 2. Obtenir le rattachement de la personne gardée
    let ratt_garder: Uuid = sqlx::query_scalar(
        "SELECT r.id FROM arbre_genealogique.rattachements r
         JOIN arbre_genealogique.arbres a ON a.id = r.arbre_id
         WHERE r.personne_id = $1 AND a.utilisateur_id = $2 AND r.deleted_at IS NULL",
    ).bind(body.personne_a_garder_id).bind(uid).fetch_one(&mut *tx).await?;

    let ratt_supprimer: Uuid = sqlx::query_scalar(
        "SELECT r.id FROM arbre_genealogique.rattachements r
         JOIN arbre_genealogique.arbres a ON a.id = r.arbre_id
         WHERE r.personne_id = $1 AND a.utilisateur_id = $2 AND r.deleted_at IS NULL",
    ).bind(body.personne_a_supprimer_id).bind(uid).fetch_one(&mut *tx).await?;

    // 3. Transférer les liens de la personne supprimée vers la personne gardée
    sqlx::query("UPDATE arbre_genealogique.liens_familiaux SET rattachement_source_id = $1 WHERE rattachement_source_id = $2 AND deleted_at IS NULL")
        .bind(ratt_garder).bind(ratt_supprimer).execute(&mut *tx).await?;
    sqlx::query("UPDATE arbre_genealogique.liens_familiaux SET rattachement_cible_id = $1 WHERE rattachement_cible_id = $2 AND deleted_at IS NULL")
        .bind(ratt_garder).bind(ratt_supprimer).execute(&mut *tx).await?;

    // 4. Soft-delete le rattachement et la personne supprimée
    sqlx::query("UPDATE arbre_genealogique.rattachements SET deleted_at = NOW() WHERE id = $1")
        .bind(ratt_supprimer).execute(&mut *tx).await?;
    sqlx::query("UPDATE arbre_genealogique.personnes SET deleted_at = NOW() WHERE id = $1")
        .bind(body.personne_a_supprimer_id).execute(&mut *tx).await?;

    tx.commit().await?;

    audit::log_action(pool.get_ref(), Some(uid), "fusionner_doublons", "arbre_genealogique", "personnes",
        Some(body.personne_a_garder_id), Some(json!({"supprime": body.personne_a_supprimer_id})),
        Some(json!({"garde": body.personne_a_garder_id})),
        audit::extraire_ip(&req).as_deref(), audit::extraire_user_agent(&req).as_deref()).await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(json!({"personne_id": body.personne_a_garder_id})), error: None }))
}
