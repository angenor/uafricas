//! Partage et signalement de profils membres.
//! - Partage : mur communautaire `/publications` (calqué sur fiche_pays_social).
//! - Signalement : lutte contre faux profils / arnaques, suspension auto au seuil
//!   (calqué sur governance.factcheck_signalement).
//! Membres authentifiés uniquement pour les mutations.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::profil_social::{
    PartageProfilListeResponse, PartageProfilRequest, PartageProfilResponse, PartageProfilRow,
    PartageQueryParams, SignalementProfilEtat, SignalementProfilRequest,
    SEUIL_SIGNALEMENTS_PROFIL_SUSPENSION,
};
use crate::services::audit;

#[derive(serde::Serialize)]
struct ApiResponse<T: serde::Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// ────────────────────────────────────────────────────────────────
// Auth
// ────────────────────────────────────────────────────────────────

fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

/// Vérifie qu'un profil existe et est public (actif, non supprimé).
async fn profil_actif_existe(pool: &PgPool, profil_id: Uuid) -> Result<bool, ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur
            WHERE id = $1 AND deleted_at IS NULL AND etat = 'actif'
        )",
    )
    .bind(profil_id)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

// ────────────────────────────────────────────────────────────────
// Partage de profil
// ────────────────────────────────────────────────────────────────

/// Colonnes du mur des profils partagés (profil actif + auteur).
const PARTAGE_PROFIL_SELECT: &str = "SELECT
        pp.id, pp.legende, pp.created_at,
        up.id AS profil_id, up.nom AS profil_nom, up.prenom AS profil_prenom,
        up.photo_url AS profil_photo_url, up.fonction AS profil_fonction,
        up.ville AS profil_ville, pays.nom AS profil_pays,
        EXISTS (
            SELECT 1 FROM iam.expertise e
            WHERE e.utilisateur_id = up.id AND e.statut = 'valide' AND e.deleted_at IS NULL
        ) AS profil_est_expert,
        EXISTS (
            SELECT 1 FROM iam.demande_biblio_humaine d
            WHERE d.utilisateur_id = up.id AND d.statut = 'valide' AND d.deleted_at IS NULL
        ) AS profil_est_biblio,
        ua.id AS auteur_id,
        CASE WHEN ua.deleted_at IS NULL THEN ua.nom ELSE 'Membre' END AS auteur_nom,
        CASE WHEN ua.deleted_at IS NULL THEN ua.prenom ELSE 'retiré' END AS auteur_prenom,
        CASE WHEN ua.deleted_at IS NULL THEN ua.photo_url ELSE NULL END AS auteur_photo_url
     FROM social.partage_profil pp
     JOIN iam.utilisateur up ON up.id = pp.profil_id
     LEFT JOIN shared.pays pays ON pays.id = up.pays_residence_id
     JOIN iam.utilisateur ua ON ua.id = pp.utilisateur_id
     WHERE pp.deleted_at IS NULL AND up.deleted_at IS NULL AND up.etat = 'actif'";

/// POST /api/utilisateurs/{id}/partages, partager un profil sur le mur.
pub async fn partager_profil(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<PartageProfilRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let profil_id = chemin.into_inner();

    if !profil_actif_existe(pool.get_ref(), profil_id).await? {
        return Err(ApiErreur::NonTrouve("Profil non trouvé".to_string()));
    }

    let legende = body
        .legende
        .as_deref()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string());

    if let Some(ref l) = legende {
        if l.chars().count() > 500 {
            return Err(ApiErreur::Validation(
                "La légende ne doit pas dépasser 500 caractères".to_string(),
            ));
        }
    }

    let partage_id: Uuid = sqlx::query_scalar(
        "INSERT INTO social.partage_profil (profil_id, utilisateur_id, legende)
         VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(profil_id)
    .bind(utilisateur_id)
    .bind(&legende)
    .fetch_one(pool.get_ref())
    .await?;

    // Engagement : 1 point au membre dont le profil est partagé (non-bloquant).
    // Le profil se désigne lui-même : `type_objet = 'profil'`, `objet_id` =
    // identifiant du membre , exactement comme la table de partage ci-dessus.
    crate::services::engagement::crediter_partage(
        pool.get_ref(),
        "profil",
        profil_id,
        profil_id,
        utilisateur_id,
    )
    .await;

    let row = sqlx::query_as::<_, PartageProfilRow>(&format!(
        "{} AND pp.id = $1",
        PARTAGE_PROFIL_SELECT
    ))
    .bind(partage_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Partage non trouvé".to_string()))?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PartageProfilResponse::from(row)),
        error: None,
    }))
}

/// GET /api/utilisateurs/partages : mur communautaire des profils (public, paginé).
pub async fn lister_partages_profils(
    pool: web::Data<PgPool>,
    params: web::Query<PartageQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM social.partage_profil pp
         JOIN iam.utilisateur up ON up.id = pp.profil_id
         WHERE pp.deleted_at IS NULL AND up.deleted_at IS NULL AND up.etat = 'actif'",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let rows = sqlx::query_as::<_, PartageProfilRow>(&format!(
        "{} ORDER BY pp.created_at DESC LIMIT $1 OFFSET $2",
        PARTAGE_PROFIL_SELECT
    ))
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let partages: Vec<PartageProfilResponse> =
        rows.into_iter().map(PartageProfilResponse::from).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PartageProfilListeResponse {
            partages,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// Signalement de profil
// ────────────────────────────────────────────────────────────────

/// POST /api/utilisateurs/{id}/signalement, signaler un profil (faux/arnaque).
/// Suspension automatique du compte au-delà du seuil.
pub async fn signaler_profil(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<SignalementProfilRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let signale_par = exiger_utilisateur_id(&req)?;
    let profil_id = chemin.into_inner();

    if profil_id == signale_par {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas signaler votre propre profil".to_string(),
        ));
    }

    if !profil_actif_existe(pool.get_ref(), profil_id).await? {
        return Err(ApiErreur::NonTrouve("Profil non trouvé".to_string()));
    }

    let motif = body
        .motif
        .as_deref()
        .map(|m| m.trim())
        .filter(|m| !m.is_empty())
        .map(|m| m.chars().take(50).collect::<String>());
    let description = body
        .description
        .as_deref()
        .map(|d| d.trim())
        .filter(|d| !d.is_empty())
        .map(|d| d.chars().take(1000).collect::<String>());

    // Insert idempotent : un signalement par membre
    let resultat = sqlx::query(
        "INSERT INTO iam.signalement_profil (utilisateur_id, signale_par, motif, description)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (utilisateur_id, signale_par) DO NOTHING",
    )
    .bind(profil_id)
    .bind(signale_par)
    .bind(&motif)
    .bind(&description)
    .execute(pool.get_ref())
    .await?;
    let nouveau_signalement = resultat.rows_affected() > 0;

    // Recompte les signalements distincts
    let nombre: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM iam.signalement_profil WHERE utilisateur_id = $1",
    )
    .bind(profil_id)
    .fetch_one(pool.get_ref())
    .await?;

    // Suspension auto si seuil dépassé (uniquement depuis l'état actif)
    let doit_suspendre = nombre >= SEUIL_SIGNALEMENTS_PROFIL_SUSPENSION;
    let etat: String = sqlx::query_scalar(
        "UPDATE iam.utilisateur
         SET nombre_signalements = $2,
             etat = CASE WHEN $3 AND etat = 'actif' THEN 'suspendu'::iam.etat_utilisateur ELSE etat END,
             updated_at = NOW()
         WHERE id = $1
         RETURNING etat::text",
    )
    .bind(profil_id)
    .bind(nombre as i32)
    .bind(doit_suspendre)
    .fetch_one(pool.get_ref())
    .await?;

    if nouveau_signalement {
        audit::log_action(
            pool.get_ref(),
            Some(signale_par),
            if etat == "suspendu" { "SIGNALEMENT_SUSPENSION" } else { "SIGNALEMENT" },
            "iam",
            "utilisateur",
            Some(profil_id),
            None,
            Some(serde_json::json!({
                "nombre_signalements": nombre,
                "motif": motif,
                "suspendu": etat == "suspendu",
            })),
            audit::extraire_ip(&req).as_deref(),
            audit::extraire_user_agent(&req).as_deref(),
        )
        .await;
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SignalementProfilEtat {
            nombre_signalements: nombre as i32,
            suspendu: etat == "suspendu",
            etat,
            deja_signale: !nouveau_signalement,
        }),
        error: None,
    }))
}
