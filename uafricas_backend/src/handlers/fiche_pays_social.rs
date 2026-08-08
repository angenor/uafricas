//! Réactions (like/dislike), signalements et partages communautaires des fiches
//! pays « Opportunités en Afrique ». Membres authentifiés uniquement pour les
//! mutations. Patterns calqués sur culture.codimoi_reaction (toggle binaire) et
//! governance.factcheck_signalement (blocage au seuil).

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::fiche_pays::region_depuis_code;
use crate::models::fiche_pays_social::{
    PartageAuteur, PartageFicheApercu, PartageFicheListeResponse, PartageFicheRequest,
    PartageFicheResponse, PartageFicheRow, PartageQueryParams, ReactionFicheEtat,
    ReactionFicheRequest, SignalementFicheEtat, SignalementFicheRequest,
    SEUIL_SIGNALEMENTS_BLOCAGE,
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

/// Extrait l'utilisateur connecté (optionnel) depuis le header Authorization.
pub fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

/// Exige un utilisateur connecté, sinon NonAutorise.
fn exiger_utilisateur_id(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

// ────────────────────────────────────────────────────────────────
// Helpers
// ────────────────────────────────────────────────────────────────

/// Résout un identifiant (UUID, code ISO2 ou nom) vers (fiche_id, bloquee).
async fn resoudre_fiche(pool: &PgPool, ident: &str) -> Result<(Uuid, bool), ApiErreur> {
    let row: Option<(Uuid, bool)> = sqlx::query_as(
        "SELECT fp.id, fp.bloquee
         FROM country_profile.fiche_pays fp
         JOIN shared.pays p ON p.id = fp.pays_id
         WHERE fp.id::text = $1 OR LOWER(p.code_iso2) = LOWER($1) OR LOWER(p.nom) = LOWER($1)",
    )
    .bind(ident)
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| ApiErreur::NonTrouve(format!("Fiche pays '{}' non trouvée", ident)))
}

/// État personnel (réaction + a_signalé) du membre courant sur une fiche.
/// Utilisé par `fiches_pays::obtenir_fiche` pour enrichir le détail.
pub async fn charger_etat_perso(
    pool: &PgPool,
    fiche_id: Uuid,
    utilisateur_id: Option<Uuid>,
) -> Result<(Option<String>, bool), ApiErreur> {
    let Some(uid) = utilisateur_id else {
        return Ok((None, false));
    };

    let ma_reaction: Option<String> = sqlx::query_scalar(
        "SELECT type_reaction FROM country_profile.reaction_fiche
         WHERE fiche_pays_id = $1 AND utilisateur_id = $2",
    )
    .bind(fiche_id)
    .bind(uid)
    .fetch_optional(pool)
    .await?;

    let a_signale: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM country_profile.signalement_fiche
            WHERE fiche_pays_id = $1 AND utilisateur_id = $2
         )",
    )
    .bind(fiche_id)
    .bind(uid)
    .fetch_one(pool)
    .await?;

    Ok((ma_reaction, a_signale))
}

// ────────────────────────────────────────────────────────────────
// POST /api/fiches-pays/{id}/reaction — like / dislike (toggle)
// ────────────────────────────────────────────────────────────────

pub async fn reagir_fiche(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<ReactionFicheRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let type_reaction = body.type_reaction.trim();
    if type_reaction != "like" && type_reaction != "dislike" {
        return Err(ApiErreur::Validation(
            "type_reaction doit être 'like' ou 'dislike'".to_string(),
        ));
    }

    let (fiche_id, bloquee) = resoudre_fiche(pool.get_ref(), &chemin.into_inner()).await?;
    if bloquee {
        return Err(ApiErreur::NonTrouve("Fiche pays non disponible".to_string()));
    }

    // Réaction existante de l'utilisateur ?
    let existante: Option<String> = sqlx::query_scalar(
        "SELECT type_reaction FROM country_profile.reaction_fiche
         WHERE fiche_pays_id = $1 AND utilisateur_id = $2",
    )
    .bind(fiche_id)
    .bind(utilisateur_id)
    .fetch_optional(pool.get_ref())
    .await?;

    let ma_reaction: Option<String> = match existante.as_deref() {
        // Même réaction → on l'enlève (toggle off)
        Some(actuelle) if actuelle == type_reaction => {
            sqlx::query(
                "DELETE FROM country_profile.reaction_fiche
                 WHERE fiche_pays_id = $1 AND utilisateur_id = $2",
            )
            .bind(fiche_id)
            .bind(utilisateur_id)
            .execute(pool.get_ref())
            .await?;
            None
        }
        // Réaction différente → bascule like↔dislike
        Some(_) => {
            sqlx::query(
                "UPDATE country_profile.reaction_fiche SET type_reaction = $3
                 WHERE fiche_pays_id = $1 AND utilisateur_id = $2",
            )
            .bind(fiche_id)
            .bind(utilisateur_id)
            .bind(type_reaction)
            .execute(pool.get_ref())
            .await?;
            Some(type_reaction.to_string())
        }
        // Aucune réaction → insertion
        None => {
            sqlx::query(
                "INSERT INTO country_profile.reaction_fiche (fiche_pays_id, utilisateur_id, type_reaction)
                 VALUES ($1, $2, $3)",
            )
            .bind(fiche_id)
            .bind(utilisateur_id)
            .bind(type_reaction)
            .execute(pool.get_ref())
            .await?;
            Some(type_reaction.to_string())
        }
    };

    // Recalcul atomique des compteurs depuis la table de réactions
    let (nombre_likes, nombre_dislikes): (i64, i64) = sqlx::query_as(
        "SELECT
            COUNT(*) FILTER (WHERE type_reaction = 'like'),
            COUNT(*) FILTER (WHERE type_reaction = 'dislike')
         FROM country_profile.reaction_fiche WHERE fiche_pays_id = $1",
    )
    .bind(fiche_id)
    .fetch_one(pool.get_ref())
    .await?;

    sqlx::query(
        "UPDATE country_profile.fiche_pays
         SET nombre_likes = $2, nombre_dislikes = $3 WHERE id = $1",
    )
    .bind(fiche_id)
    .bind(nombre_likes as i32)
    .bind(nombre_dislikes as i32)
    .execute(pool.get_ref())
    .await?;

    // Engagement : 1 point au créateur de la fiche par « j'aime » reçu.
    if ma_reaction.as_deref() == Some("like") {
        if let Ok(Some(cree_par)) = sqlx::query_scalar::<_, Uuid>(
            "SELECT cree_par FROM country_profile.fiche_pays WHERE id = $1",
        )
        .bind(fiche_id)
        .fetch_optional(pool.get_ref())
        .await
        {
            crate::services::engagement::crediter_jaime(
                pool.get_ref(),
                "fiche_pays",
                fiche_id,
                cree_par,
                utilisateur_id,
            )
            .await;
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ReactionFicheEtat {
            nombre_likes: nombre_likes as i32,
            nombre_dislikes: nombre_dislikes as i32,
            ma_reaction,
        }),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// POST /api/fiches-pays/{id}/signalement — bloque au seuil
// ────────────────────────────────────────────────────────────────

pub async fn signaler_fiche(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<SignalementFicheRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (fiche_id, _) = resoudre_fiche(pool.get_ref(), &chemin.into_inner()).await?;

    let motif = body
        .motif
        .as_deref()
        .map(|m| m.trim())
        .filter(|m| !m.is_empty())
        .map(|m| m.chars().take(1000).collect::<String>());

    // Insert idempotent (un signalement par utilisateur)
    sqlx::query(
        "INSERT INTO country_profile.signalement_fiche (fiche_pays_id, utilisateur_id, motif)
         VALUES ($1, $2, $3)
         ON CONFLICT (fiche_pays_id, utilisateur_id) DO NOTHING",
    )
    .bind(fiche_id)
    .bind(utilisateur_id)
    .bind(&motif)
    .execute(pool.get_ref())
    .await?;

    // Recompte les signalements distincts + applique le blocage au seuil
    let nombre_signalements: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM country_profile.signalement_fiche WHERE fiche_pays_id = $1",
    )
    .bind(fiche_id)
    .fetch_one(pool.get_ref())
    .await?;
    let nombre_signalements = nombre_signalements as i32;

    let bloquee = nombre_signalements >= SEUIL_SIGNALEMENTS_BLOCAGE;

    sqlx::query(
        "UPDATE country_profile.fiche_pays
         SET nombre_signalements = $2,
             bloquee = (bloquee OR $3)
         WHERE id = $1",
    )
    .bind(fiche_id)
    .bind(nombre_signalements)
    .bind(bloquee)
    .execute(pool.get_ref())
    .await?;

    audit::log_action(
        pool.get_ref(),
        Some(utilisateur_id),
        if bloquee { "SIGNALEMENT_BLOCAGE" } else { "SIGNALEMENT" },
        "country_profile",
        "fiche_pays",
        Some(fiche_id),
        None,
        Some(serde_json::json!({ "nombre_signalements": nombre_signalements, "bloquee": bloquee })),
        audit::extraire_ip(&req).as_deref(),
        audit::extraire_user_agent(&req).as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SignalementFicheEtat {
            nombre_signalements,
            bloquee,
            a_signale: true,
        }),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// POST /api/fiches-pays/{id}/partages — partage vers le mur
// ────────────────────────────────────────────────────────────────

pub async fn partager_fiche(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<String>,
    body: web::Json<PartageFicheRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (fiche_id, bloquee) = resoudre_fiche(pool.get_ref(), &chemin.into_inner()).await?;
    if bloquee {
        return Err(ApiErreur::NonTrouve("Fiche pays non disponible".to_string()));
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
        "INSERT INTO country_profile.partage_fiche (fiche_pays_id, utilisateur_id, legende)
         VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(fiche_id)
    .bind(utilisateur_id)
    .bind(&legende)
    .fetch_one(pool.get_ref())
    .await?;

    // Engagement : 1 point au créateur de la fiche par partage reçu (non-bloquant).
    if let Some(auteur_id) =
        crate::services::engagement::resoudre_beneficiaire(pool.get_ref(), "fiche_pays", fiche_id)
            .await
    {
        crate::services::engagement::crediter_partage(
            pool.get_ref(),
            "fiche_pays",
            fiche_id,
            auteur_id,
            utilisateur_id,
        )
        .await;
    }

    // Recharge le partage enrichi pour le renvoyer (carte du mur)
    let row = charger_partage(pool.get_ref(), partage_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(partage_depuis_row(row)),
        error: None,
    }))
}

// ────────────────────────────────────────────────────────────────
// GET /api/fiches-pays/partages — mur communautaire (public, paginé)
// ────────────────────────────────────────────────────────────────

pub async fn lister_partages(
    pool: web::Data<PgPool>,
    params: web::Query<PartageQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 50);
    let offset = (page - 1) * par_page;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM country_profile.partage_fiche pf
         JOIN country_profile.fiche_pays fp ON fp.id = pf.fiche_pays_id
         WHERE pf.deleted_at IS NULL AND fp.bloquee = FALSE",
    )
    .fetch_one(pool.get_ref())
    .await?;

    let rows = sqlx::query_as::<_, PartageFicheRow>(&format!(
        "{} ORDER BY pf.created_at DESC LIMIT $1 OFFSET $2",
        PARTAGE_SELECT
    ))
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let partages: Vec<PartageFicheResponse> = rows.into_iter().map(partage_depuis_row).collect();

    let total_pages = if total == 0 {
        1
    } else {
        (total as f64 / par_page as f64).ceil() as i64
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PartageFicheListeResponse {
            partages,
            total,
            page,
            par_page,
            total_pages,
        }),
        error: None,
    }))
}

const PARTAGE_SELECT: &str = "SELECT
        pf.id, pf.legende, pf.created_at,
        fp.id AS fiche_id, p.nom AS pays_nom, p.code_iso2 AS pays_code,
        p.capitale AS pays_capitale, fp.slogan,
        fp.image_couverture_url, fp.image_drapeau_url,
        u.id AS auteur_id,
        CASE WHEN u.deleted_at IS NULL THEN u.nom ELSE 'Membre' END AS auteur_nom,
        CASE WHEN u.deleted_at IS NULL THEN u.prenom ELSE 'retiré' END AS auteur_prenom,
        CASE WHEN u.deleted_at IS NULL THEN u.photo_url ELSE NULL END AS auteur_photo_url
     FROM country_profile.partage_fiche pf
     JOIN country_profile.fiche_pays fp ON fp.id = pf.fiche_pays_id
     JOIN shared.pays p ON p.id = fp.pays_id
     JOIN iam.utilisateur u ON u.id = pf.utilisateur_id
     WHERE pf.deleted_at IS NULL AND fp.bloquee = FALSE";

async fn charger_partage(pool: &PgPool, partage_id: Uuid) -> Result<PartageFicheRow, ApiErreur> {
    sqlx::query_as::<_, PartageFicheRow>(&format!("{} AND pf.id = $1", PARTAGE_SELECT))
        .bind(partage_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Partage non trouvé".to_string()))
}

fn partage_depuis_row(row: PartageFicheRow) -> PartageFicheResponse {
    PartageFicheResponse {
        id: row.id,
        legende: row.legende,
        created_at: row.created_at,
        fiche: PartageFicheApercu {
            id: row.fiche_id,
            nom: row.pays_nom,
            capitale: row.pays_capitale,
            slogan: row.slogan,
            image_couverture: row.image_couverture_url,
            drapeau_url: row.image_drapeau_url,
            region: region_depuis_code(row.pays_code.as_deref()),
        },
        auteur: PartageAuteur {
            id: row.auteur_id,
            nom: row.auteur_nom,
            prenom: row.auteur_prenom,
            photo_url: row.auteur_photo_url,
        },
    }
}
