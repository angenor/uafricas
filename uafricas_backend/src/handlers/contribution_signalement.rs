//! Signalement des CONTRIBUTIONS individuelles d'une fiche pays (afripulse).
//!
//! Remplace l'ancien signalement « fiche entière » : chaque objet contribué
//! (recette, site touristique, personnalité, secteur, savoir pratique) est
//! signalable séparément. Au-delà de 10 signalements distincts, l'objet est
//! automatiquement **suspendu** (reste visible avec un bandeau, mais figé)
//! jusqu'à réactivation par un admin.
//!
//! Table générique `country_profile.signalement_contribution` clé
//! (type_objet, objet_id, signale_par). Pattern calqué sur
//! `governance.factcheck_signalement` (insert idempotent + recompte + bascule).

use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::services::audit;

/// Au-delà de ce nombre de signalements distincts, la contribution est suspendue.
pub const SEUIL_SIGNALEMENTS_SUSPENSION_CONTRIBUTION: i64 = 10;

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

/// Mappe un `type_objet` (valeur de l'enum afripulse) vers
/// `(table country_profile, possède une colonne deleted_at)`.
///
/// Seuls les 5 objets contribués sont signalables ; `fiche_pays`,
/// `photo_visiteur` et `recommandation_visiteur` sont volontairement exclus.
/// Les noms de table sont des littéraux fixes → interpolation SQL sûre.
pub fn table_et_softdelete(type_objet: &str) -> Option<(&'static str, bool)> {
    match type_objet {
        "site_touristique" => Some(("country_profile.site_touristique", true)),
        "secteur_developpement" => Some(("country_profile.secteur_developpement", false)),
        "recette_culinaire" => Some(("country_profile.recette_culinaire", true)),
        "personnalite_connue" => Some(("country_profile.personnalite_connue", true)),
        "savoir_pratique" => Some(("country_profile.savoir_pratique", true)),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
pub struct SignalerContributionRequest {
    pub motif: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SignalementContributionEtat {
    pub nombre_signalements: i32,
    pub suspendu: bool,
    pub deja_signale: bool,
}

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

/// POST /api/fiches-pays/contributions/{type_objet}/{objet_id}/signalement
pub async fn signaler_contribution(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<SignalerContributionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let utilisateur_id = exiger_utilisateur_id(&req)?;
    let (type_objet, objet_id) = chemin.into_inner();

    let (table, a_soft_delete) = table_et_softdelete(&type_objet)
        .ok_or_else(|| ApiErreur::Validation("Type de contribution non signalable".to_string()))?;

    // L'objet doit exister (et ne pas être supprimé pour les tables soft-delete).
    let clause_delete = if a_soft_delete { " AND deleted_at IS NULL" } else { "" };
    let existe: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table} WHERE id = $1{clause_delete})"
    ))
    .bind(objet_id)
    .fetch_one(pool.get_ref())
    .await?;
    if !existe {
        return Err(ApiErreur::NonTrouve("Contribution introuvable".to_string()));
    }

    // Insertion idempotente : un seul signalement par membre et par objet.
    let resultat = sqlx::query(
        "INSERT INTO country_profile.signalement_contribution
            (type_objet, objet_id, signale_par, motif, description)
         VALUES ($1::country_profile.type_objet_contribution, $2, $3, $4, $5)
         ON CONFLICT (type_objet, objet_id, signale_par) DO NOTHING",
    )
    .bind(&type_objet)
    .bind(objet_id)
    .bind(utilisateur_id)
    .bind(body.motif.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(
        body.description
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty()),
    )
    .execute(pool.get_ref())
    .await?;
    let nouveau_signalement = resultat.rows_affected() > 0;

    // Recompter les signalements distincts.
    let nombre: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM country_profile.signalement_contribution
         WHERE type_objet = $1::country_profile.type_objet_contribution AND objet_id = $2",
    )
    .bind(&type_objet)
    .bind(objet_id)
    .fetch_one(pool.get_ref())
    .await?;

    let doit_suspendre = nombre > SEUIL_SIGNALEMENTS_SUSPENSION_CONTRIBUTION;

    // Met à jour le compteur ; suspend uniquement (jamais de réactivation auto).
    let suspendu: bool = sqlx::query_scalar(&format!(
        "UPDATE {table}
         SET nombre_signalements = $2,
             suspendu = (suspendu OR $3)
         WHERE id = $1
         RETURNING suspendu"
    ))
    .bind(objet_id)
    .bind(nombre as i32)
    .bind(doit_suspendre)
    .fetch_one(pool.get_ref())
    .await?;

    if nouveau_signalement {
        let ip = audit::extraire_ip(&req);
        let ua = audit::extraire_user_agent(&req);
        let action = if doit_suspendre && suspendu {
            "SIGNALEMENT_SUSPENSION"
        } else {
            "SIGNALEMENT"
        };
        audit::log_action(
            pool.get_ref(),
            Some(utilisateur_id),
            action,
            "country_profile",
            &type_objet,
            Some(objet_id),
            None,
            None,
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;
    }

    if doit_suspendre && suspendu {
        log::warn!(
            "Contribution {type_objet}/{objet_id} suspendue automatiquement ({nombre} signalements)"
        );
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SignalementContributionEtat {
            nombre_signalements: nombre as i32,
            suspendu,
            deja_signale: !nouveau_signalement,
        }),
        error: None,
    }))
}
