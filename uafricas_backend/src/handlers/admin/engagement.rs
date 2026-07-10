//! Endpoints d'administration du système d'engagement (permission `engagement`).
//! Barème paramétrable (règles, paliers, niveaux) + journal global + ajustement manuel.
//! Toute mutation est auditée (`log_action`, schema `engagement`).

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::engagement::{
    AjustementRequest, CreerPalierRequest, JournalAdminPage, JournalAdminParams, JournalAdminRow,
    ModifierNiveauRequest, ModifierPalierRequest, ModifierRegleRequest, NiveauAdmin, PalierAdmin,
    RegleAdmin,
};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ── Règles ──────────────────────────────────────────────────────────────────

/// GET /api/admin/engagement/regles
pub async fn lister_regles(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let regles = sqlx::query_as::<_, RegleAdmin>(
        "SELECT id, type_action, libelle, points, reputation_delta,
                plafond_journalier, plafond_mensuel, actif
         FROM engagement.regle_points ORDER BY type_action",
    )
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(regles), error: None }))
}

/// PUT /api/admin/engagement/regles/{id}
pub async fn modifier_regle(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierRegleRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    sqlx::query(
        "UPDATE engagement.regle_points SET
            libelle            = COALESCE($2, libelle),
            points             = COALESCE($3, points),
            reputation_delta   = COALESCE($4, reputation_delta),
            plafond_journalier = CASE WHEN $5 THEN $6 ELSE plafond_journalier END,
            plafond_mensuel    = CASE WHEN $7 THEN $8 ELSE plafond_mensuel END,
            actif              = COALESCE($9, actif),
            updated_at         = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(body.libelle.as_deref())
    .bind(body.points)
    .bind(body.reputation_delta)
    .bind(body.plafond_journalier.is_some())
    .bind(body.plafond_journalier.flatten())
    .bind(body.plafond_mensuel.is_some())
    .bind(body.plafond_mensuel.flatten())
    .bind(body.actif)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "engagement", "regle_points",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Paliers de popularité ───────────────────────────────────────────────────

/// GET /api/admin/engagement/paliers
pub async fn lister_paliers(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let paliers = sqlx::query_as::<_, PalierAdmin>(
        "SELECT id, seuil_likes, points, actif FROM engagement.palier_popularite ORDER BY seuil_likes",
    )
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(paliers), error: None }))
}

/// POST /api/admin/engagement/paliers
pub async fn creer_palier(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreerPalierRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    if body.seuil_likes <= 0 {
        return Err(ApiErreur::Validation("seuil_likes doit être positif".into()));
    }

    let palier = sqlx::query_as::<_, PalierAdmin>(
        "INSERT INTO engagement.palier_popularite (seuil_likes, points)
         VALUES ($1, $2)
         ON CONFLICT (seuil_likes) DO UPDATE SET points = EXCLUDED.points, actif = TRUE
         RETURNING id, seuil_likes, points, actif",
    )
    .bind(body.seuil_likes)
    .bind(body.points)
    .fetch_one(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "CREATE", "engagement", "palier_popularite",
        Some(palier.id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(palier), error: None }))
}

/// PUT /api/admin/engagement/paliers/{id}
pub async fn modifier_palier(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierPalierRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    sqlx::query(
        "UPDATE engagement.palier_popularite
         SET points = COALESCE($2, points), actif = COALESCE($3, actif)
         WHERE id = $1",
    )
    .bind(id)
    .bind(body.points)
    .bind(body.actif)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "engagement", "palier_popularite",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

/// DELETE /api/admin/engagement/paliers/{id} — désactivation (référencé par le journal).
pub async fn desactiver_palier(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    sqlx::query("UPDATE engagement.palier_popularite SET actif = FALSE WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "DELETE", "engagement", "palier_popularite",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Niveaux ─────────────────────────────────────────────────────────────────

/// GET /api/admin/engagement/niveaux
pub async fn lister_niveaux(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let niveaux = sqlx::query_as::<_, NiveauAdmin>(
        "SELECT id, code, libelle, seuil_min, ordre, badge_couleur, badge_icone
         FROM engagement.niveau ORDER BY ordre",
    )
    .fetch_all(pool.get_ref())
    .await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(niveaux), error: None }))
}

/// PUT /api/admin/engagement/niveaux/{id}
pub async fn modifier_niveau(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ModifierNiveauRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");
    let id = path.into_inner();

    sqlx::query(
        "UPDATE engagement.niveau SET
            libelle       = COALESCE($2, libelle),
            seuil_min     = COALESCE($3, seuil_min),
            badge_couleur = COALESCE($4, badge_couleur),
            badge_icone   = COALESCE($5, badge_icone)
         WHERE id = $1",
    )
    .bind(id)
    .bind(body.libelle.as_deref())
    .bind(body.seuil_min)
    .bind(body.badge_couleur.as_deref())
    .bind(body.badge_icone.as_deref())
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "UPDATE", "engagement", "niveau",
        Some(id), None, None, ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> { success: true, data: None, error: None }))
}

// ── Journal global ──────────────────────────────────────────────────────────

/// GET /api/admin/engagement/journal
pub async fn lister_journal(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<JournalAdminParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(30).clamp(1, 100);
    let offset = (page - 1) * taille;

    // Filtres optionnels via des paramètres nullables castés (protège contre l'injection).
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM engagement.mouvement_points m
         WHERE ($1::uuid IS NULL OR m.utilisateur_id = $1)
           AND ($2::text IS NULL OR m.type_action = $2)
           AND ($3::timestamptz IS NULL OR m.created_at >= $3::timestamptz)
           AND ($4::timestamptz IS NULL OR m.created_at <= $4::timestamptz)",
    )
    .bind(params.utilisateur_id)
    .bind(params.type_action.as_deref())
    .bind(params.depuis.as_deref())
    .bind(params.jusqu_a.as_deref())
    .fetch_one(pool.get_ref())
    .await?;

    let elements = sqlx::query_as::<_, JournalAdminRow>(
        "SELECT m.id, m.utilisateur_id,
                (u.prenom || ' ' || u.nom) AS utilisateur_nom,
                m.type_action, m.type_objet, m.objet_id, m.points, m.reputation_delta,
                m.solde_apres, m.plafond_atteint, m.created_at
         FROM engagement.mouvement_points m
         LEFT JOIN iam.utilisateur u ON u.id = m.utilisateur_id
         WHERE ($1::uuid IS NULL OR m.utilisateur_id = $1)
           AND ($2::text IS NULL OR m.type_action = $2)
           AND ($3::timestamptz IS NULL OR m.created_at >= $3::timestamptz)
           AND ($4::timestamptz IS NULL OR m.created_at <= $4::timestamptz)
         ORDER BY m.created_at DESC
         LIMIT $5 OFFSET $6",
    )
    .bind(params.utilisateur_id)
    .bind(params.type_action.as_deref())
    .bind(params.depuis.as_deref())
    .bind(params.jusqu_a.as_deref())
    .bind(taille)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(JournalAdminPage { elements, total, page, taille }),
        error: None,
    }))
}

/// POST /api/admin/engagement/ajustement — crédit/débit manuel motivé.
pub async fn ajuster_points(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<AjustementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "engagement", "gerer");

    crate::services::engagement::ajuster(
        pool.get_ref(),
        body.utilisateur_id,
        body.points,
        body.reputation_delta,
    )
    .await;

    let nouveau_solde: Option<i32> =
        sqlx::query_scalar("SELECT solde_points FROM engagement.compte WHERE utilisateur_id = $1")
            .bind(body.utilisateur_id)
            .fetch_optional(pool.get_ref())
            .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(), Some(admin.id), "AJUSTEMENT", "engagement", "compte",
        Some(body.utilisateur_id), None,
        Some(serde_json::json!({ "points": body.points, "reputation_delta": body.reputation_delta, "motif": body.motif })),
        ip.as_deref(), ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "solde_points": nouveau_solde.unwrap_or(0) })),
        error: None,
    }))
}
