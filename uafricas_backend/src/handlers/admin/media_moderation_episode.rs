//! File de modération des épisodes
//! (feature 009-medias-programmes-episodes, US1, FR-040 à FR-043).
//!
//! Endpoints :
//!   GET   /api/admin/medias/episodes
//!   PATCH /api/admin/medias/episodes/{id}/valider
//!   PATCH /api/admin/medias/episodes/{id}/rejeter
//!
//! Garde : `verifier_permission!(admin, "media", …)`. Attention au piège de
//! nommage : `"media"` couvre radio et télé, `"media_content"` couvre vidafrica
//! et `"programme"` désigne les programmes d'échange.
//!
//! **Pourquoi l'objet et non une proposition JSONB** : `proposition_media` (09l)
//! stocke le contenu proposé jusqu'à validation, précisément pour que « rien de
//! non validé n'existe dans les tables publiques ». Cette garantie vise les
//! contributeurs anonymes ; elle est ici sans objet, le co-détenteur étant déjà
//! autorisé sur le support et son fichier déjà téléversé. Créer l'épisode
//! directement donne gratuitement le suivi par son auteur, son exclusion de la
//! rotation (`etat = 'publie'` filtré partout) et sa resoumission après
//! correction.

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_episode::contexte_episode;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::media_episode::{
    colonnes_episode, support_pour_famille, table_episode, EpisodeModerationResponse,
    EpisodeQueryParams, EpisodeRow, RejeterEpisodeRequest,
};
use crate::models::notification;
use crate::services::audit;
use crate::verifier_permission;
use crate::ApiResponse;

/// Familles interrogées par la file : les deux, sauf filtre explicite.
const FAMILLES: [&str; 2] = ["tele", "radio"];

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/admin/medias/episodes
// ═══════════════════════════════════════════════════════════════════════════

/// File de modération, priorisée par échéance (FR-043).
///
/// Le tri `echeance` remonte d'abord les épisodes attendus à l'antenne, puis
/// ceux sans échéance par ancienneté : c'est ce qui empêche qu'un épisode dû
/// samedi soit traité au même rang qu'un contenu sans date.
///
/// `prochaine_echeance` se calcule **à la lecture** depuis les créneaux du
/// programme : aucune tâche de fond, aucune colonne à maintenir.
pub async fn lister_episodes(
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    params: web::Query<EpisodeQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "voir");

    let etat = params.etat.as_deref().unwrap_or("en_attente");
    let page = params.page_effective();
    let taille = params.taille_effective(25);

    // Les deux tables sont interrogées séparément puis fusionnées : leur colonne
    // de média diffère (`video_url` / `audio_url`), et un UNION imposerait de
    // dupliquer la liste de colonnes plutôt que de réutiliser les constantes.
    let familles: Vec<&str> = match params.r#type.as_deref() {
        Some(f) if FAMILLES.contains(&f) => vec![f],
        _ => FAMILLES.to_vec(),
    };

    let mut lignes: Vec<EpisodeModerationResponse> = Vec::new();

    for famille in familles {
        let type_support = support_pour_famille(famille).expect("famille validée");
        let colonnes = colonnes_episode(type_support).expect("type de support validé");
        let table_ep = table_episode(type_support).expect("type de support validé");
        let table_emission = crate::models::media_detention::table_contenu_pour_support(type_support)
            .expect("type de support validé");
        let table_support =
            crate::models::media_detention::table_pour_support(type_support).expect("type validé");
        let colonne = crate::models::media_emission::colonne_support(type_support)
            .expect("type de support validé");

        // `prochaine_echeance` : la plus proche occurrence à venir parmi les
        // créneaux actifs du programme, calculée dans le fuseau de chaque
        // créneau. Un créneau quotidien est dû aujourd'hui ou demain ; un
        // hebdomadaire, au prochain `jour_semaine`.
        let rows = sqlx::query_as::<_, EpisodeRow>(&format!(
            "SELECT {colonnes},
                    m.titre AS emission_titre, m.slug AS emission_slug,
                    m.cadence AS emission_cadence,
                    s.id AS support_id, s.nom AS support_nom, s.slug AS support_slug,
                    u.nom AS auteur_nom, u.prenom AS auteur_prenom,
                    (SELECT MIN(
                         ((NOW() AT TIME ZONE c.fuseau)::date
                          + CASE WHEN c.recurrence = 'quotidien'
                                 THEN (CASE WHEN (NOW() AT TIME ZONE c.fuseau)::time < c.heure_debut
                                            THEN 0 ELSE 1 END)
                                 ELSE ((7 + c.jour_semaine
                                        - EXTRACT(DOW FROM (NOW() AT TIME ZONE c.fuseau))::int) % 7)
                            END
                         + c.heure_debut) AT TIME ZONE c.fuseau)
                       FROM media_content.creneau_programmation c
                      WHERE c.emission_id = m.id
                        AND c.actif = TRUE AND c.deleted_at IS NULL) AS prochaine_echeance
               FROM {table_ep} ep
               JOIN {table_emission} m ON m.id = ep.emission_id AND m.deleted_at IS NULL
               JOIN {table_support} s ON s.id = m.{colonne}
               LEFT JOIN iam.utilisateur u ON u.id = ep.cree_par
              WHERE ep.deleted_at IS NULL
                AND ep.etat = $1
                AND ($2::uuid IS NULL OR s.id = $2)"
        ))
        .bind(etat)
        .bind(params.support_id)
        .fetch_all(pool.get_ref())
        .await?;

        let maintenant = Utc::now();
        for row in rows {
            let soumis_at = row.created_at;
            let echeance = row.prochaine_echeance;
            let episode = row.to_response(type_support);
            lignes.push(EpisodeModerationResponse {
                anciennete_heures: (maintenant - soumis_at).num_hours(),
                heures_avant_echeance: echeance.map(|e| (e - maintenant).num_hours()),
                prochaine_echeance: echeance,
                soumis_at,
                episode,
            });
        }
    }

    // Tri appliqué après fusion : les deux familles partagent la même file.
    match params.tri.as_deref().unwrap_or("echeance") {
        "anciennete" => lignes.sort_by_key(|l| l.soumis_at),
        // `echeance` : les épisodes attendus à l'antenne d'abord (la plus proche
        // en tête), les autres ensuite, par ancienneté.
        _ => lignes.sort_by(|a, b| match (a.prochaine_echeance, b.prochaine_echeance) {
            (Some(x), Some(y)) => x.cmp(&y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.soumis_at.cmp(&b.soumis_at),
        }),
    }

    let total = lignes.len() as i64;
    let debut = ((page - 1) * taille).min(total) as usize;
    let fin = (debut as i64 + taille).min(total) as usize;
    let page_courante: Vec<_> = lignes.drain(debut..fin).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "episodes": page_courante,
            "pagination": { "page": page, "taille": taille, "total": total },
        })),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PATCH /api/admin/medias/episodes/{id}/valider
// ═══════════════════════════════════════════════════════════════════════════

/// L'épisode passe `publie` et entre dans la rotation à l'occurrence suivante.
///
/// `409` s'il n'est pas `en_attente` : revalider un épisode déjà publié n'a pas
/// de sens, et le rendrait indistinguable d'une resoumission.
///
/// Le **programme** est publié dans la même transaction s'il ne l'était pas : un
/// programme dont un épisode est validé n'a plus de raison de rester brouillon,
/// et sans cela l'épisode resterait invisible du public (FR-011).
pub async fn valider_episode(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;

    if ctx.etat != "en_attente" {
        return Err(ApiErreur::Conflit(format!(
            "Cet épisode est « {} » : seul un épisode en attente peut être validé.",
            ctx.etat
        )));
    }
    if ctx.media_url.is_none() {
        return Err(ApiErreur::Conflit(
            "Cet épisode est dépourvu de média : il ne peut pas être publié.".into(),
        ));
    }

    let table_ep = table_episode(&ctx.type_support).expect("type de support validé");
    let table_emission =
        crate::models::media_detention::table_contenu_pour_support(&ctx.type_support)
            .expect("type de support validé");

    let mut tx = pool.begin().await?;

    sqlx::query(&format!(
        "UPDATE {table_ep}
            SET etat = 'publie', motif_rejet = NULL,
                valide_par = $2, valide_at = NOW(), updated_at = NOW()
          WHERE id = $1 AND etat = 'en_attente' AND deleted_at IS NULL"
    ))
    .bind(episode_id)
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(&format!(
        "UPDATE {table_emission} SET etat = 'publie', updated_at = NOW()
          WHERE id = $1 AND etat IN ('brouillon', 'en_attente') AND deleted_at IS NULL"
    ))
    .bind(ctx.emission_id)
    .execute(&mut *tx)
    .await?;

    notifier(
        &mut tx,
        ctx.cree_par,
        notification::media::EPISODE_VALIDE,
        &format!(
            "Votre épisode « {} » a été validé : il est désormais diffusé.",
            ctx.titre
        ),
    )
    .await?;

    tx.commit().await?;

    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        serde_json::json!({ "etat": "publie" }),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PATCH /api/admin/medias/episodes/{id}/rejeter
// ═══════════════════════════════════════════════════════════════════════════

/// Rejet **motivé** (FR-041, SC-008).
///
/// Le motif fait au moins 10 caractères, même garde applicative que le rejet
/// d'une proposition (09l) : et le CHECK `ck_episode_*_rejet_motive` interdit de
/// toute façon un rejet vide en base. L'auteur est notifié **avec le motif** :
/// sans lui, il n'a rien à corriger.
pub async fn rejeter_episode(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    admin: AdminUtilisateur,
    chemin: web::Path<Uuid>,
    body: web::Json<RejeterEpisodeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "media", "modifier");
    let episode_id = chemin.into_inner();
    let motif = body.motif_valide()?;
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;

    if ctx.etat != "en_attente" {
        return Err(ApiErreur::Conflit(format!(
            "Cet épisode est « {} » : seul un épisode en attente peut être rejeté.",
            ctx.etat
        )));
    }

    let table_ep = table_episode(&ctx.type_support).expect("type de support validé");
    let mut tx = pool.begin().await?;

    sqlx::query(&format!(
        "UPDATE {table_ep}
            SET etat = 'rejete', motif_rejet = $2,
                valide_par = $3, valide_at = NOW(), updated_at = NOW()
          WHERE id = $1 AND etat = 'en_attente' AND deleted_at IS NULL"
    ))
    .bind(episode_id)
    .bind(&motif)
    .bind(admin.id)
    .execute(&mut *tx)
    .await?;

    notifier(
        &mut tx,
        ctx.cree_par,
        notification::media::EPISODE_REJETE,
        &format!(
            "Votre épisode « {} » a été refusé. Motif : {}",
            ctx.titre, motif
        ),
    )
    .await?;

    tx.commit().await?;

    journaliser(
        &req,
        pool.get_ref(),
        admin.id,
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        serde_json::json!({ "etat": "rejete", "motif_rejet": motif }),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Utilitaires
// ═══════════════════════════════════════════════════════════════════════════

/// Notification émise **dans** la transaction de décision : elle fait partie de
/// la décision, pas d'un traitement ultérieur qui pourrait échouer seul.
async fn notifier(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    destinataire: Uuid,
    type_notif: &str,
    message: &str,
) -> Result<(), ApiErreur> {
    sqlx::query(
        "INSERT INTO arbre_genealogique.notifications (destinataire_id, type, message, lien_action)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(destinataire)
    .bind(type_notif)
    .bind(message)
    .bind(notification::media::LIEN_MES_SUPPORTS)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn journaliser(
    req: &HttpRequest,
    pool: &PgPool,
    admin_id: Uuid,
    type_support: &str,
    episode_id: Uuid,
    avant: Option<serde_json::Value>,
    apres: serde_json::Value,
) {
    let table = if type_support == "station_radio" {
        "episode_radio"
    }
    else {
        "episode_tele"
    };
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(admin_id),
        "UPDATE",
        "media_content",
        table,
        Some(episode_id),
        avant,
        Some(apres),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}
