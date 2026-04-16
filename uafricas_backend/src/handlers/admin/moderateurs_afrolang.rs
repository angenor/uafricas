use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::moderateurs_afrolang::DesignerModerateurRequest;
use crate::models::afrolang::{
    ModerateurAttitreResponse, SalleModerateurRow, SALLE_MODERATEUR_COLONNES,
};
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

// ══════════════════════════════════════════════════════════════════
// GET /api/admin/afrolang/salles/{salle_id}/moderateurs
// ══════════════════════════════════════════════════════════════════

pub async fn lister_moderateurs_attitres(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "voir");

    let salle_id = path.into_inner();

    let sql = format!(
        "SELECT {},
            u.nom AS utilisateur_nom,
            u.prenom AS utilisateur_prenom,
            u.photo_url AS utilisateur_photo,
            u.email AS utilisateur_email
         FROM afrolang.salle_moderateur sm
         LEFT JOIN iam.utilisateur u ON u.id = sm.utilisateur_id
         WHERE sm.salle_id = $1
         ORDER BY sm.actif DESC, sm.designe_at ASC",
        SALLE_MODERATEUR_COLONNES
    );

    let rows = sqlx::query_as::<_, SalleModerateurRow>(&sql)
        .bind(salle_id)
        .fetch_all(pool.get_ref())
        .await?;

    let items: Vec<ModerateurAttitreResponse> = rows.iter().map(|r| r.to_response()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(items),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════
// POST /api/admin/afrolang/salles/{salle_id}/moderateurs
// ══════════════════════════════════════════════════════════════════

pub async fn designer_moderateur(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<DesignerModerateurRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let salle_id = path.into_inner();

    // Vérifier l'existence de la salle
    let salle_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM afrolang.salle
            WHERE id = $1 AND deleted_at IS NULL
        )",
    )
    .bind(salle_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !salle_existe {
        return Err(ApiErreur::NonTrouve("Salle non trouvée".into()));
    }

    // Vérifier l'utilisateur
    let user_existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur
            WHERE id = $1 AND etat::text = 'actif' AND deleted_at IS NULL
        )",
    )
    .bind(body.utilisateur_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !user_existe {
        return Err(ApiErreur::Validation(
            "Utilisateur introuvable ou inactif".into(),
        ));
    }

    let disponibilite = body.disponibilite.as_deref().map(str::trim);

    // Upsert : réactivation si ligne existe déjà avec actif=FALSE
    let existant: Option<(Uuid, bool)> = sqlx::query_as(
        "SELECT id, actif FROM afrolang.salle_moderateur
         WHERE salle_id = $1 AND utilisateur_id = $2",
    )
    .bind(salle_id)
    .bind(body.utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let (id_moderateur, ancien_etat) = match existant {
        Some((id, actif)) if actif => {
            return Err(ApiErreur::Validation(
                "Ce modérateur est déjà actif sur cette salle".into(),
            ));
        }
        Some((id, _)) => {
            sqlx::query(
                "UPDATE afrolang.salle_moderateur
                 SET actif = TRUE,
                     retire_at = NULL,
                     disponibilite = $3,
                     designe_par = $4,
                     designe_at = NOW(),
                     updated_at = NOW()
                 WHERE id = $1 AND salle_id = $2",
            )
            .bind(id)
            .bind(salle_id)
            .bind(disponibilite)
            .bind(admin.id)
            .execute(pool.get_ref())
            .await?;
            (id, serde_json::json!({ "actif": false }))
        }
        None => {
            let id: Uuid = sqlx::query_scalar(
                "INSERT INTO afrolang.salle_moderateur
                    (salle_id, utilisateur_id, designe_par, disponibilite, actif)
                 VALUES ($1, $2, $3, $4, TRUE)
                 RETURNING id",
            )
            .bind(salle_id)
            .bind(body.utilisateur_id)
            .bind(admin.id)
            .bind(disponibilite)
            .fetch_one(pool.get_ref())
            .await?;
            (id, serde_json::Value::Null)
        }
    };

    // Audit
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    let nouveau = serde_json::json!({
        "actif": true,
        "utilisateur_id": body.utilisateur_id,
        "salle_id": salle_id,
    });
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "CREATE",
        "afrolang",
        "salle_moderateur",
        Some(id_moderateur),
        if ancien_etat.is_null() { None } else { Some(ancien_etat) },
        Some(nouveau),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": id_moderateur,
            "salle_id": salle_id,
            "utilisateur_id": body.utilisateur_id,
            "actif": true,
        })),
        error: None,
    }))
}

// ══════════════════════════════════════════════════════════════════
// DELETE /api/admin/afrolang/salles/{salle_id}/moderateurs/{utilisateur_id}
// ══════════════════════════════════════════════════════════════════

pub async fn retirer_moderateur(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "afrolang", "modifier");

    let (salle_id, utilisateur_id) = path.into_inner();

    let result = sqlx::query(
        "UPDATE afrolang.salle_moderateur
         SET actif = FALSE,
             retire_at = NOW(),
             updated_at = NOW()
         WHERE salle_id = $1 AND utilisateur_id = $2 AND actif = TRUE",
    )
    .bind(salle_id)
    .bind(utilisateur_id)
    .execute(pool.get_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Modérateur attitré actif non trouvé pour cette salle".into(),
        ));
    }

    // Audit
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(admin.id),
        "DELETE",
        "afrolang",
        "salle_moderateur",
        None,
        Some(serde_json::json!({ "actif": true })),
        Some(serde_json::json!({ "actif": false, "retire_at": "NOW()" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}
