//! Épisodes : versement, ordre, déplacement et mise en avant
//! (feature 009-medias-programmes-episodes, US1).
//!
//! Routes membres, gardées par `garde_detenteur`, **jamais**
//! `AdminUtilisateur` :
//!
//!   POST   /api/medias/emissions/{id}/episodes
//!   PUT    /api/medias/episodes/{id}
//!   DELETE /api/medias/episodes/{id}
//!   PUT    /api/medias/emissions/{id}/episodes/reordonner
//!   PATCH  /api/medias/episodes/{id}/emission
//!   PATCH  /api/medias/episodes/{id}/a-la-une
//!   GET    /api/medias/emissions/{id}/episodes            (vue détenteur)
//!
//! **Invariant central (FR-040)** : un épisode versé par un co-détenteur naît
//! `en_attente`. Le client ne décide pas de l'état : toute valeur transmise est
//! ignorée. C'est ce qui ferme le trou de l'ancien modèle, où un co-détenteur
//! publiait sans revue.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_detention::{exiger_utilisateur_id, garde_detenteur};
use crate::handlers::media_emission::{contexte_emission, journaliser, slug_unique};
use crate::handlers::media_social;
use crate::models::media_episode::{
    colonne_media, colonnes_episode, table_episode, type_media_episode, DeplacerEpisodeRequest,
    EpisodeRequest, EpisodeResponse, EpisodeRow, ReordonnancementRequest, ORDRE_EPISODES,
};
use crate::models::television::generer_slug;
use crate::services::audit;
use crate::ApiResponse;

// ═══════════════════════════════════════════════════════════════════════════
// POST /api/medias/emissions/{id}/episodes
// ═══════════════════════════════════════════════════════════════════════════

/// Verser un épisode dans un programme.
///
/// Deux invariants, tenus par le SERVEUR et non par le client (FR-007, FR-040) :
///
/// - `etat = 'en_attente'` **toujours** ;
/// - `ordre = COALESCE(MAX(ordre), -1) + 1` sur l'émission, l'épisode prend
///   rang **à la fin**, sans déplacer les existants ni altérer l'occurrence en
///   cours de la rotation (FR-019). Patron déjà en production dans
///   `admin/formation_contenu.rs`.
pub async fn creer_episode(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<EpisodeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let emission_id = chemin.into_inner();
    let (type_support, support_id, _) = contexte_emission(pool.get_ref(), emission_id).await?;
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    body.valider()?;
    let table_ep = table_episode(&type_support).expect("type de support validé");
    let colonne = colonne_media(&type_support).expect("type de support validé");

    let slug = slug_unique(pool.get_ref(), table_ep, &generer_slug(&body.titre)).await?;
    let media = body.media();

    let (episode_id, ordre): (Uuid, i32) = sqlx::query_as(&format!(
        "INSERT INTO {table_ep}
            (emission_id, titre, slug, description, image_couverture_url, {colonne},
             numero_episode, duree_minutes, ordre, etat, cree_par)
         VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6, $7, $8,
                 (SELECT COALESCE(MAX(ordre), -1) + 1 FROM {table_ep}
                   WHERE emission_id = $1 AND deleted_at IS NULL),
                 'en_attente', $9)
         RETURNING id, ordre"
    ))
    .bind(emission_id)
    .bind(body.titre.trim())
    .bind(&slug)
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.image_couverture_url.as_deref().map(str::trim))
    .bind(media.as_deref())
    .bind(body.numero_episode)
    .bind(body.duree_minutes)
    .bind(moi)
    .fetch_one(pool.get_ref())
    .await?;

    journaliser_episode(
        &req,
        pool.get_ref(),
        moi,
        "CREATE",
        &type_support,
        episode_id,
        None,
        Some(serde_json::json!({
            "emission_id": emission_id,
            "titre": body.titre.trim(),
            "etat": "en_attente",
            "ordre": ordre,
        })),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": episode_id,
            "slug": slug,
            "etat": "en_attente",
            "ordre": ordre,
        })),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PUT /api/medias/episodes/{id}
// ═══════════════════════════════════════════════════════════════════════════

/// Modifier un épisode.
///
/// Deux bascules d'état, et elles seules :
///
/// - un épisode **publié dont le média change** repasse `en_attente` : c'est le
///   même principe que `PUT …/media` sur les propositions : ce qui a été validé,
///   c'est le fichier, pas seulement le titre. Une modification purement
///   éditoriale (titre, description, image) reste publiée ;
/// - un épisode **rejeté** que l'on modifie repasse `en_attente` et son
///   `motif_rejet` est effacé : c'est le parcours de correction-resoumission de
///   FR-041.
pub async fn modifier_episode(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<EpisodeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;
    garde_detenteur(pool.get_ref(), &ctx.type_support, ctx.support_id, moi, "co_detenteur").await?;

    body.valider()?;
    let table_ep = table_episode(&ctx.type_support).expect("type de support validé");
    let colonne = colonne_media(&ctx.type_support).expect("type de support validé");

    let media = body.media();
    let media_change = match (media.as_deref(), ctx.media_url.as_deref()) {
        (Some(nouveau), Some(ancien)) => nouveau != ancien,
        (Some(_), None) => true,
        // Un média absent de la requête n'efface pas celui en place.
        (None, _) => false,
    };

    // `rejete` → `en_attente` avec effacement du motif ; `publie` + média changé
    // → `en_attente`. Dans tous les autres cas l'état est laissé tel quel.
    let nouvel_etat = match ctx.etat.as_str() {
        "rejete" => Some("en_attente"),
        "publie" if media_change => Some("en_attente"),
        _ => None,
    };

    sqlx::query(&format!(
        "UPDATE {table_ep}
            SET titre = $2,
                description = COALESCE($3, description),
                image_couverture_url = $4,
                {colonne} = COALESCE($5, {colonne}),
                numero_episode = $6,
                duree_minutes = $7,
                etat = COALESCE($8, etat),
                motif_rejet = CASE WHEN $8 IS NOT NULL THEN NULL ELSE motif_rejet END,
                valide_par = CASE WHEN $8 IS NOT NULL THEN NULL ELSE valide_par END,
                valide_at  = CASE WHEN $8 IS NOT NULL THEN NULL ELSE valide_at  END,
                updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(episode_id)
    .bind(body.titre.trim())
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.image_couverture_url.as_deref().map(str::trim))
    .bind(media.as_deref())
    .bind(body.numero_episode)
    .bind(body.duree_minutes)
    .bind(nouvel_etat)
    .execute(pool.get_ref())
    .await?;

    journaliser_episode(
        &req,
        pool.get_ref(),
        moi,
        "UPDATE",
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        Some(serde_json::json!({
            "titre": body.titre.trim(),
            "etat": nouvel_etat.unwrap_or(&ctx.etat),
        })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": episode_id,
            "etat": nouvel_etat.unwrap_or(&ctx.etat),
        })),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// DELETE /api/medias/episodes/{id}
// ═══════════════════════════════════════════════════════════════════════════

/// Suppression douce. Le cycle de rotation se recalcule à la lecture suivante
/// (FR-019) : aucune action supplémentaire n'est requise, et c'est bien tout
/// l'intérêt d'une rotation calculée plutôt que matérialisée.
pub async fn supprimer_episode(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;
    garde_detenteur(pool.get_ref(), &ctx.type_support, ctx.support_id, moi, "co_detenteur").await?;

    let table_ep = table_episode(&ctx.type_support).expect("type de support validé");
    sqlx::query(&format!(
        "UPDATE {table_ep} SET etat = 'supprime', a_la_une = FALSE,
                deleted_at = NOW(), updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(episode_id)
    .execute(pool.get_ref())
    .await?;

    journaliser_episode(
        &req,
        pool.get_ref(),
        moi,
        "DELETE",
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        Some(serde_json::json!({ "etat": "supprime" })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PUT /api/medias/emissions/{id}/episodes/reordonner
// ═══════════════════════════════════════════════════════════════════════════

/// Réécriture **atomique** de l'ordre : tout réordonner ou rien (patron de
/// `admin/formation_contenu.rs`).
///
/// `400` si la liste ne couvre pas **exactement** les épisodes de l'émission :
/// une liste partielle laisserait des rangs incohérents, et le nouvel ordre
/// serait faux dès la prochaine occurrence.
pub async fn reordonner_episodes(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<ReordonnancementRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let emission_id = chemin.into_inner();
    let (type_support, support_id, _) = contexte_emission(pool.get_ref(), emission_id).await?;
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    let table_ep = table_episode(&type_support).expect("type de support validé");

    let mut tx = pool.begin().await?;

    let existants: Vec<Uuid> = sqlx::query_scalar(&format!(
        "SELECT id FROM {table_ep}
          WHERE emission_id = $1 AND deleted_at IS NULL FOR UPDATE"
    ))
    .bind(emission_id)
    .fetch_all(&mut *tx)
    .await?;

    let soumis: std::collections::HashSet<Uuid> =
        body.ordres.iter().map(|o| o.episode_id).collect();
    let attendus: std::collections::HashSet<Uuid> = existants.iter().copied().collect();

    if soumis != attendus {
        return Err(ApiErreur::Validation(format!(
            "La liste doit couvrir exactement les {} épisode(s) du programme : {} reçu(s).",
            attendus.len(),
            soumis.len()
        )));
    }

    for ordre in &body.ordres {
        sqlx::query(&format!(
            "UPDATE {table_ep} SET ordre = $2, updated_at = NOW()
              WHERE id = $1 AND emission_id = $3 AND deleted_at IS NULL"
        ))
        .bind(ordre.episode_id)
        .bind(ordre.ordre)
        .bind(emission_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    journaliser(
        &req,
        pool.get_ref(),
        moi,
        "UPDATE",
        &type_support,
        emission_id,
        None,
        Some(serde_json::json!({ "reordonnancement": body.ordres.len() })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PATCH /api/medias/episodes/{id}/emission
// ═══════════════════════════════════════════════════════════════════════════

/// Déplacer un épisode vers un autre programme **du même support** (`400`
/// sinon).
///
/// L'épisode conserve intégralement ses interactions, rien à faire : elles sont
/// indexées par `(type_media, media_id)` et ni l'un ni l'autre ne change
/// (FR-009). Il prend rang **en fin** du nouveau programme ; les deux cycles de
/// rotation se recalculent d'eux-mêmes à la lecture suivante.
pub async fn deplacer_episode(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<DeplacerEpisodeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;
    garde_detenteur(pool.get_ref(), &ctx.type_support, ctx.support_id, moi, "co_detenteur").await?;

    let (type_cible, support_cible, _) = contexte_emission(pool.get_ref(), body.emission_id).await?;
    if type_cible != ctx.type_support || support_cible != ctx.support_id {
        return Err(ApiErreur::Validation(
            "Un épisode ne se déplace qu'entre programmes d'un même support".into(),
        ));
    }
    if body.emission_id == ctx.emission_id {
        return Err(ApiErreur::Validation(
            "Cet épisode appartient déjà à ce programme".into(),
        ));
    }

    let table_ep = table_episode(&ctx.type_support).expect("type de support validé");

    // La mise en avant est propre à un programme : la conserver ferait deux
    // épisodes « à la une » dans le programme d'accueil, ce que l'index unique
    // partiel refuserait.
    sqlx::query(&format!(
        "UPDATE {table_ep}
            SET emission_id = $2,
                a_la_une = FALSE,
                ordre = (SELECT COALESCE(MAX(ordre), -1) + 1 FROM {table_ep}
                          WHERE emission_id = $2 AND deleted_at IS NULL),
                updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(episode_id)
    .bind(body.emission_id)
    .execute(pool.get_ref())
    .await?;

    journaliser_episode(
        &req,
        pool.get_ref(),
        moi,
        "UPDATE",
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        Some(serde_json::json!({ "emission_id": body.emission_id })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PATCH /api/medias/episodes/{id}/a-la-une
// ═══════════════════════════════════════════════════════════════════════════

/// Désigne l'épisode mis en avant **pour son support**.
///
/// La bascule de l'ancien à `FALSE` et la désignation du nouveau tiennent dans
/// **une même transaction** : sans cela, l'index unique partiel est violé dès
/// que deux désignations se croisent (règle héritée de 09j §3).
pub async fn mettre_a_la_une(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let episode_id = chemin.into_inner();
    let ctx = contexte_episode(pool.get_ref(), episode_id).await?;
    garde_detenteur(pool.get_ref(), &ctx.type_support, ctx.support_id, moi, "co_detenteur").await?;

    if ctx.etat != "publie" {
        return Err(ApiErreur::Conflit(
            "Seul un épisode publié peut être mis en avant".into(),
        ));
    }

    basculer_a_la_une(pool.get_ref(), &ctx.type_support, ctx.support_id, episode_id).await?;

    journaliser_episode(
        &req,
        pool.get_ref(),
        moi,
        "UPDATE",
        &ctx.type_support,
        episode_id,
        Some(ctx.instantane()),
        Some(serde_json::json!({ "a_la_une": true })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// Bascule atomique de la mise en avant, à l'échelle d'un SUPPORT.
///
/// Partagée avec le back-office (`admin::radio_tele`) : deux implémentations
/// divergentes de la même transaction seraient le meilleur moyen d'en avoir une
/// fausse.
pub async fn basculer_a_la_une(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
    episode_id: Uuid,
) -> Result<(), ApiErreur> {
    let table_ep = table_episode(type_support).expect("type de support validé");
    let table_emission =
        crate::models::media_detention::table_contenu_pour_support(type_support)
            .expect("type de support validé");
    let colonne = crate::models::media_emission::colonne_support(type_support)
        .expect("type de support validé");

    let mut tx = pool.begin().await?;

    sqlx::query(&format!(
        "UPDATE {table_ep} SET a_la_une = FALSE, updated_at = NOW()
          WHERE a_la_une = TRUE AND deleted_at IS NULL
            AND emission_id IN (SELECT id FROM {table_emission}
                                 WHERE {colonne} = $1 AND deleted_at IS NULL)"
    ))
    .bind(support_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(&format!(
        "UPDATE {table_ep} SET a_la_une = TRUE, updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(episode_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/emissions/{id}/episodes, vue détenteur
// ═══════════════════════════════════════════════════════════════════════════

/// **Tous** les épisodes du programme, quel que soit leur état, avec le motif
/// des rejets : c'est l'écran de suivi de FR-042.
pub async fn lister_episodes_detenteur(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let emission_id = chemin.into_inner();
    let (type_support, support_id, _) = contexte_emission(pool.get_ref(), emission_id).await?;
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    let episodes = charger_episodes(pool.get_ref(), &type_support, emission_id, None).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "episodes": episodes })),
        error: None,
    }))
}

/// Épisodes d'un programme, filtrables par état. Partagée avec le back-office.
pub async fn charger_episodes(
    pool: &PgPool,
    type_support: &str,
    emission_id: Uuid,
    etat: Option<&str>,
) -> Result<Vec<EpisodeResponse>, ApiErreur> {
    let colonnes = colonnes_episode(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;
    let table_ep = table_episode(type_support).expect("type de support validé");

    let rows = sqlx::query_as::<_, EpisodeRow>(&format!(
        "SELECT {colonnes},
                u.nom AS auteur_nom, u.prenom AS auteur_prenom
           FROM {table_ep} ep
           LEFT JOIN iam.utilisateur u ON u.id = ep.cree_par
          WHERE ep.emission_id = $1 AND ep.deleted_at IS NULL
            AND ($2::text IS NULL OR ep.etat = $2)
          ORDER BY {ORDRE_EPISODES}"
    ))
    .bind(emission_id)
    .bind(etat)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(|r| r.to_response(type_support)).collect())
}

// ═══════════════════════════════════════════════════════════════════════════
// Lecture publique d'un épisode
// ═══════════════════════════════════════════════════════════════════════════

/// Détail d'un épisode publié, par son slug.
///
/// **Remplace `GET /api/television/programmes/slug/{slug}`.** Les slugs ayant
/// été conservés par 09q, les adresses publiques existantes continuent de
/// résoudre (FR-056) et pointent désormais sur la page d'épisode.
pub async fn obtenir_episode_par_slug(
    pool: &PgPool,
    type_support: &str,
    slug: &str,
    moi: Option<Uuid>,
) -> Result<serde_json::Value, ApiErreur> {
    let colonnes = colonnes_episode(type_support).expect("type de support validé");
    let table_ep = table_episode(type_support).expect("type de support validé");
    let table_emission =
        crate::models::media_detention::table_contenu_pour_support(type_support)
            .expect("type de support validé");
    let table_support =
        crate::models::media_detention::table_pour_support(type_support).expect("type validé");
    let colonne = crate::models::media_emission::colonne_support(type_support)
        .expect("type de support validé");

    // Le programme ET l'épisode doivent être publiés : suspendre un programme
    // retire ses épisodes de l'espace public sans les supprimer (FR-011).
    let row = sqlx::query_as::<_, EpisodeRow>(&format!(
        "SELECT {colonnes},
                m.titre AS emission_titre, m.slug AS emission_slug,
                m.cadence AS emission_cadence,
                s.id AS support_id, s.nom AS support_nom, s.slug AS support_slug
           FROM {table_ep} ep
           JOIN {table_emission} m ON m.id = ep.emission_id AND m.deleted_at IS NULL
           JOIN {table_support} s ON s.id = m.{colonne}
          WHERE ep.slug = $1 AND ep.etat = 'publie' AND ep.deleted_at IS NULL
            AND m.etat = 'publie' AND s.etat = 'publie'"
    ))
    .bind(slug)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Épisode non trouvé".into()))?;

    let emission_id = row.emission_id;
    let mut reponse = row.to_response(type_support);

    let type_episode = type_media_episode(type_support).expect("type de support validé");
    let compteurs = media_social::compteurs_pour(pool, type_episode, &[reponse.id], moi).await?;
    reponse.interactions = compteurs.get(&reponse.id).cloned();

    // « Propose les autres épisodes de la même émission » (US1 §4).
    let voisins = sqlx::query_as::<_, EpisodeRow>(&format!(
        "SELECT {colonnes}
           FROM {table_ep} ep
          WHERE ep.emission_id = $1 AND ep.id <> $2
            AND ep.etat = 'publie' AND ep.deleted_at IS NULL
          ORDER BY {ORDRE_EPISODES}
          LIMIT 12"
    ))
    .bind(emission_id)
    .bind(reponse.id)
    .fetch_all(pool)
    .await?;

    Ok(serde_json::json!({
        "episode": reponse,
        "episodes_voisins": voisins
            .iter()
            .map(|r| r.to_response(type_support))
            .collect::<Vec<_>>(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// Contexte d'un épisode
// ═══════════════════════════════════════════════════════════════════════════

pub struct ContexteEpisode {
    pub type_support: String,
    pub support_id: Uuid,
    pub emission_id: Uuid,
    pub etat: String,
    pub titre: String,
    pub media_url: Option<String>,
    pub cree_par: Uuid,
}

impl ContexteEpisode {
    pub fn instantane(&self) -> serde_json::Value {
        serde_json::json!({
            "type_support": self.type_support,
            "support_id": self.support_id,
            "emission_id": self.emission_id,
            "titre": self.titre,
            "etat": self.etat,
        })
    }
}

/// Support et programme porteurs d'un épisode. La garde ne peut s'exercer
/// qu'après avoir su de quel support il relève.
pub async fn contexte_episode(
    pool: &PgPool,
    episode_id: Uuid,
) -> Result<ContexteEpisode, ApiErreur> {
    for (type_support, table_ep, table_emission, colonne, colonne_media) in [
        (
            "chaine_tv",
            "media_content.episode_tele",
            "media_content.emission_tele",
            "chaine_id",
            "video_url",
        ),
        (
            "station_radio",
            "media_content.episode_radio",
            "media_content.emission_radio",
            "station_id",
            "audio_url",
        ),
    ] {
        let ligne: Option<(Uuid, Uuid, String, String, Option<String>, Uuid)> =
            sqlx::query_as(&format!(
                "SELECT m.{colonne}, ep.emission_id, ep.etat, ep.titre, ep.{colonne_media}, ep.cree_par
                   FROM {table_ep} ep
                   JOIN {table_emission} m ON m.id = ep.emission_id
                  WHERE ep.id = $1 AND ep.deleted_at IS NULL"
            ))
            .bind(episode_id)
            .fetch_optional(pool)
            .await?;

        if let Some((support_id, emission_id, etat, titre, media_url, cree_par)) = ligne {
            return Ok(ContexteEpisode {
                type_support: type_support.to_string(),
                support_id,
                emission_id,
                etat,
                titre,
                media_url,
                cree_par,
            });
        }
    }

    Err(ApiErreur::NonTrouve("Épisode introuvable".into()))
}

/// Toute mutation d'épisode est journalisée (FR-045, principe VII).
pub async fn journaliser_episode(
    req: &HttpRequest,
    pool: &PgPool,
    moi: Uuid,
    action: &str,
    type_support: &str,
    episode_id: Uuid,
    avant: Option<serde_json::Value>,
    apres: Option<serde_json::Value>,
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
        Some(moi),
        action,
        "media_content",
        table,
        Some(episode_id),
        avant,
        apres,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}
