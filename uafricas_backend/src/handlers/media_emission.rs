//! Programmes conteneurs — gestion par les co-détenteurs et lecture publique
//! (feature 009-medias-programmes-episodes, US1).
//!
//! Routes membres — la garde est `garde_detenteur`, **jamais**
//! `AdminUtilisateur` : ce sont des routes de gestion de son propre support.
//!
//!   POST   /api/medias/{type_support}/{support_id}/emissions
//!   GET    /api/medias/{type_support}/{support_id}/emissions   (vue détenteur)
//!   PUT    /api/medias/emissions/{id}
//!   DELETE /api/medias/emissions/{id}
//!
//! Ce module porte aussi les **agrégations de lecture publique** réutilisées par
//! `television.rs` et `stations_radio.rs` : elles chargent les programmes et
//! leurs aperçus d'épisodes pour une page entière en **deux requêtes**, sans
//! requête N+1 (SC-009).

use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_detention::{exiger_utilisateur_id, garde_detenteur};
use crate::handlers::media_social;
use crate::models::media_detention::{table_contenu_pour_support, table_pour_support, valider_type_support};
use crate::models::media_emission::{
    colonne_support, colonnes_emission, type_media_emission, valider_cadence, EmissionRequest,
    EmissionResponse, EmissionRow,
};
use crate::models::media_episode::{
    colonnes_episode, table_episode, type_media_episode, EpisodeResponse, EpisodeRow,
    ORDRE_EPISODES,
};
use crate::models::television::generer_slug;
use crate::services::audit;
use crate::ApiResponse;

/// Aperçu d'épisodes servi par les sections. Au-delà, la page du programme
/// prend le relais — c'est ce qui tient la promesse de navigabilité à
/// 500 épisodes.
pub const MAX_EPISODES_APERCU: i64 = 12;

// ═══════════════════════════════════════════════════════════════════════════
// Agrégations de lecture — sans requête N+1
// ═══════════════════════════════════════════════════════════════════════════

/// Programmes publiés de plusieurs supports, en **une seule requête**.
///
/// La fenêtre `ROW_NUMBER() OVER (PARTITION BY support)` borne le nombre de
/// programmes par support sans boucler côté Rust, et `COUNT(*) OVER (PARTITION
/// BY support)` rapporte le total dans la même passe — un second aller-retour
/// de comptage serait précisément le N+1 que SC-010 proscrit.
///
/// Un programme sans épisode publié est **écarté** (FR-011) : la jointure
/// latérale de comptage le filtre par `nombre_episodes > 0`.
pub async fn emissions_publiees_par_supports(
    pool: &PgPool,
    type_support: &str,
    support_ids: &[Uuid],
    max_par_support: i64,
) -> Result<HashMap<Uuid, (Vec<EmissionResponse>, i64)>, ApiErreur> {
    let mut resultat: HashMap<Uuid, (Vec<EmissionResponse>, i64)> = HashMap::new();
    if support_ids.is_empty() {
        return Ok(resultat);
    }

    let colonnes = colonnes_emission(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;
    let table_emission = table_contenu_pour_support(type_support).expect("type de support validé");
    let table_support = table_pour_support(type_support).expect("type de support validé");
    let table_ep = table_episode(type_support).expect("type de support validé");
    let colonne = colonne_support(type_support).expect("type de support validé");

    let rows = sqlx::query_as::<_, EmissionRow>(&format!(
        "SELECT * FROM (
            SELECT {colonnes},
                   s.nom AS support_nom, s.slug AS support_slug,
                   cat.nom AS theme_phare_nom,
                   agg.nombre_episodes, agg.dernier_episode_at,
                   NULL::bigint AS episodes_en_attente, NULL::bigint AS episodes_rejetes,
                   COUNT(*)     OVER (PARTITION BY e.{colonne}) AS total_emissions,
                   ROW_NUMBER() OVER (PARTITION BY e.{colonne}
                                      ORDER BY agg.dernier_episode_at DESC NULLS LAST,
                                               e.created_at DESC, e.id) AS rang
              FROM {table_emission} e
              JOIN {table_support} s ON s.id = e.{colonne}
              LEFT JOIN shared.categorie cat ON cat.id = e.theme_phare_id
              JOIN LATERAL (
                  SELECT COUNT(*) AS nombre_episodes, MAX(ep.valide_at) AS dernier_episode_at
                    FROM {table_ep} ep
                   WHERE ep.emission_id = e.id
                     AND ep.etat = 'publie' AND ep.deleted_at IS NULL
              ) agg ON agg.nombre_episodes > 0
             WHERE e.{colonne} = ANY($1)
               AND e.etat = 'publie' AND e.deleted_at IS NULL
         ) fenetre
         WHERE rang <= $2"
    ))
    .bind(support_ids)
    .bind(max_par_support)
    .fetch_all(pool)
    .await?;

    for row in &rows {
        let entree = resultat
            .entry(row.support_id)
            .or_insert_with(|| (Vec::new(), row.total_emissions.unwrap_or(0)));
        entree.0.push(row.to_response(type_support));
    }

    Ok(resultat)
}

/// Aperçu d'épisodes de plusieurs programmes, en **une seule requête**.
///
/// Les plus récents d'abord — un aperçu annonce l'actualité de la série, la
/// page du programme servant l'ordre de diffusion.
pub async fn episodes_apercu_par_emissions(
    pool: &PgPool,
    type_support: &str,
    emission_ids: &[Uuid],
    max_par_emission: i64,
) -> Result<HashMap<Uuid, Vec<EpisodeResponse>>, ApiErreur> {
    let mut resultat: HashMap<Uuid, Vec<EpisodeResponse>> = HashMap::new();
    if emission_ids.is_empty() {
        return Ok(resultat);
    }

    let colonnes = colonnes_episode(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;
    let table_ep = table_episode(type_support).expect("type de support validé");

    let rows = sqlx::query_as::<_, EpisodeRow>(&format!(
        "SELECT * FROM (
            SELECT {colonnes},
                   ROW_NUMBER() OVER (PARTITION BY ep.emission_id
                                      ORDER BY ep.created_at DESC, ep.id DESC) AS rang
              FROM {table_ep} ep
             WHERE ep.emission_id = ANY($1)
               AND ep.etat = 'publie' AND ep.deleted_at IS NULL
         ) fenetre
         WHERE rang <= $2"
    ))
    .bind(emission_ids)
    .bind(max_par_emission)
    .fetch_all(pool)
    .await?;

    for row in &rows {
        resultat
            .entry(row.emission_id)
            .or_default()
            .push(row.to_response(type_support));
    }

    Ok(resultat)
}

/// Greffe les aperçus d'épisodes sur une liste de programmes, puis les
/// compteurs d'interaction des deux niveaux — **jamais agrégés** (FR-048).
pub async fn greffer_apercus_et_compteurs(
    pool: &PgPool,
    type_support: &str,
    emissions: &mut [&mut EmissionResponse],
    moi: Option<Uuid>,
) -> Result<(), ApiErreur> {
    let ids: Vec<Uuid> = emissions.iter().map(|e| e.id).collect();
    let apercus = episodes_apercu_par_emissions(pool, type_support, &ids, MAX_EPISODES_APERCU).await?;

    let mut ids_episodes: Vec<Uuid> = Vec::new();
    for emission in emissions.iter_mut() {
        if let Some(liste) = apercus.get(&emission.id) {
            ids_episodes.extend(liste.iter().map(|e| e.id));
            emission.episodes_apercu = liste
                .iter()
                .map(|e| EpisodeResponse {
                    interactions: None,
                    ..cloner_episode(e)
                })
                .collect();
        }
    }

    let type_emission = type_media_emission(type_support).expect("type de support validé");
    let type_episode = type_media_episode(type_support).expect("type de support validé");

    let compteurs_emissions = media_social::compteurs_pour(pool, type_emission, &ids, moi).await?;
    let compteurs_episodes =
        media_social::compteurs_pour(pool, type_episode, &ids_episodes, moi).await?;

    for emission in emissions.iter_mut() {
        emission.interactions = compteurs_emissions.get(&emission.id).cloned();
        for episode in emission.episodes_apercu.iter_mut() {
            episode.interactions = compteurs_episodes.get(&episode.id).cloned();
        }
    }

    Ok(())
}

/// `EpisodeResponse` ne dérive pas `Clone` — il porte des `Option` imbriquées
/// que rien d'autre ne recopie. Ce constructeur explicite évite de l'imposer à
/// tout le module pour un seul usage.
fn cloner_episode(e: &EpisodeResponse) -> EpisodeResponse {
    EpisodeResponse {
        id: e.id,
        emission_id: e.emission_id,
        titre: e.titre.clone(),
        slug: e.slug.clone(),
        description: e.description.clone(),
        image_couverture_url: e.image_couverture_url.clone(),
        media_url: e.media_url.clone(),
        video_url: e.video_url.clone(),
        audio_url: e.audio_url.clone(),
        source_media: e.source_media.clone(),
        numero_episode: e.numero_episode,
        ordre: e.ordre,
        duree_minutes: e.duree_minutes,
        a_la_une: e.a_la_une,
        a_la_une_globale: e.a_la_une_globale,
        etat: e.etat.clone(),
        motif_rejet: e.motif_rejet.clone(),
        valide_at: e.valide_at,
        type_support: e.type_support.clone(),
        emission: e.emission.clone(),
        emission_cadence: e.emission_cadence.clone(),
        support: e.support.clone(),
        nombre_signalements: e.nombre_signalements,
        cree_par: e.cree_par,
        auteur_nom_complet: e.auteur_nom_complet.clone(),
        created_at: e.created_at,
        updated_at: e.updated_at,
        interactions: e.interactions.clone(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Lecture publique d'un programme
// ═══════════════════════════════════════════════════════════════════════════

/// Détail d'un programme publié, par son slug.
///
/// `404` si le programme n'est pas publié **ou n'a aucun épisode publié** : un
/// programme vide n'existe pas pour le public (FR-011).
pub async fn obtenir_emission_par_slug(
    pool: &PgPool,
    type_support: &str,
    slug: &str,
    moi: Option<Uuid>,
) -> Result<EmissionResponse, ApiErreur> {
    let colonnes = colonnes_emission(type_support).expect("type de support validé");
    let table_emission = table_contenu_pour_support(type_support).expect("type de support validé");
    let table_support = table_pour_support(type_support).expect("type de support validé");
    let table_ep = table_episode(type_support).expect("type de support validé");
    let colonne = colonne_support(type_support).expect("type de support validé");

    let row = sqlx::query_as::<_, EmissionRow>(&format!(
        "SELECT {colonnes},
                s.nom AS support_nom, s.slug AS support_slug,
                cat.nom AS theme_phare_nom,
                (SELECT COUNT(*) FROM {table_ep} ep
                  WHERE ep.emission_id = e.id AND ep.etat = 'publie' AND ep.deleted_at IS NULL)
                    AS nombre_episodes,
                (SELECT MAX(ep.valide_at) FROM {table_ep} ep
                  WHERE ep.emission_id = e.id AND ep.etat = 'publie' AND ep.deleted_at IS NULL)
                    AS dernier_episode_at
           FROM {table_emission} e
           JOIN {table_support} s ON s.id = e.{colonne}
           LEFT JOIN shared.categorie cat ON cat.id = e.theme_phare_id
          WHERE e.slug = $1 AND e.etat = 'publie' AND e.deleted_at IS NULL"
    ))
    .bind(slug)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Programme non trouvé".into()))?;

    if row.nombre_episodes.unwrap_or(0) == 0 {
        return Err(ApiErreur::NonTrouve("Programme non trouvé".into()));
    }

    let mut reponse = row.to_response(type_support);
    let type_emission = type_media_emission(type_support).expect("type de support validé");
    let compteurs = media_social::compteurs_pour(pool, type_emission, &[reponse.id], moi).await?;
    // Compteurs de L'ÉMISSION SEULE — jamais la somme de ceux de ses épisodes
    // (FR-048). Les épisodes portent les leurs, servis par leur propre page.
    reponse.interactions = compteurs.get(&reponse.id).cloned();

    Ok(reponse)
}

/// Épisodes publiés d'un programme, paginés — c'est ce qui tient la promesse de
/// navigabilité à 500 épisodes (SC-009).
pub async fn lister_episodes_emission(
    pool: &PgPool,
    type_support: &str,
    emission_id: Uuid,
    page: i64,
    taille: i64,
    moi: Option<Uuid>,
) -> Result<serde_json::Value, ApiErreur> {
    let colonnes = colonnes_episode(type_support).expect("type de support validé");
    let table_ep = table_episode(type_support).expect("type de support validé");
    let table_emission = table_contenu_pour_support(type_support).expect("type de support validé");
    let offset = (page - 1).max(0) * taille;

    // Suspendre un programme retire ses épisodes de l'espace public **sans les
    // supprimer** (FR-011) : le filtre porte donc aussi sur le parent, sans quoi
    // cette route servirait le catalogue d'un programme retiré de l'antenne.
    let publiee: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table_emission}
            WHERE id = $1 AND etat = 'publie' AND deleted_at IS NULL)"
    ))
    .bind(emission_id)
    .fetch_one(pool)
    .await?;
    if !publiee {
        return Err(ApiErreur::NonTrouve("Programme non trouvé".into()));
    }

    let total: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM {table_ep}
          WHERE emission_id = $1 AND etat = 'publie' AND deleted_at IS NULL"
    ))
    .bind(emission_id)
    .fetch_one(pool)
    .await?;

    let rows = sqlx::query_as::<_, EpisodeRow>(&format!(
        "SELECT {colonnes}
           FROM {table_ep} ep
          WHERE ep.emission_id = $1 AND ep.etat = 'publie' AND ep.deleted_at IS NULL
          ORDER BY {ORDRE_EPISODES}
          LIMIT $2 OFFSET $3"
    ))
    .bind(emission_id)
    .bind(taille)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let mut episodes: Vec<EpisodeResponse> =
        rows.iter().map(|r| r.to_response(type_support)).collect();

    let ids: Vec<Uuid> = episodes.iter().map(|e| e.id).collect();
    let type_episode = type_media_episode(type_support).expect("type de support validé");
    let compteurs = media_social::compteurs_pour(pool, type_episode, &ids, moi).await?;
    for episode in episodes.iter_mut() {
        episode.interactions = compteurs.get(&episode.id).cloned();
    }

    Ok(serde_json::json!({
        "episodes": episodes,
        "pagination": {
            "page": page,
            "taille": taille,
            "total": total,
            "total_pages": (total as f64 / taille as f64).ceil() as i64,
        }
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// POST /api/medias/{type_support}/{support_id}/emissions
// ═══════════════════════════════════════════════════════════════════════════

/// Créer un programme, **sans exiger le moindre fichier** (FR-003) : c'est tout
/// le sens du conteneur — on déclare une série avant d'avoir tourné le premier
/// épisode.
///
/// Le programme naît `brouillon` ; il passe `publie` dès qu'un de ses épisodes
/// l'est (voir `admin::media_moderation_episode::valider_episode`).
pub async fn creer_emission(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<EmissionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    body.valider()?;
    let table_emission = table_contenu_pour_support(&type_support).expect("type de support validé");
    let colonne = colonne_support(&type_support).expect("type de support validé");

    let slug = slug_unique(pool.get_ref(), table_emission, &generer_slug(&body.titre)).await?;
    let cadence = body.cadence_ou_defaut();
    valider_cadence(cadence)?;

    let categorie_radio = if type_support == "station_radio" {
        body.categorie_radio.as_deref()
    }
    else {
        None
    };

    let emission_id: Uuid = if type_support == "station_radio" {
        sqlx::query_scalar(&format!(
            "INSERT INTO {table_emission}
                (station_id, titre, slug, description, image_couverture_url,
                 info_animateur, info_producteur, langue, categorie_radio,
                 theme_phare_id, theme_phare_autre, cadence, etat, cree_par)
             VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6, $7, COALESCE($8, 'Français'),
                     $9::media_content.categorie_radio, $10, $11, $12, 'brouillon', $13)
             RETURNING id"
        ))
        .bind(support_id)
        .bind(body.titre.trim())
        .bind(&slug)
        .bind(body.description.as_deref().map(str::trim))
        .bind(body.image_couverture_url.as_deref().map(str::trim))
        .bind(body.info_animateur.as_deref().map(str::trim))
        .bind(body.info_producteur.as_deref().map(str::trim))
        .bind(body.langue.as_deref().map(str::trim))
        .bind(categorie_radio)
        .bind(body.theme_phare_id)
        .bind(body.theme_phare_autre.as_deref().map(str::trim))
        .bind(cadence)
        .bind(moi)
        .fetch_one(pool.get_ref())
        .await?
    }
    else {
        sqlx::query_scalar(&format!(
            "INSERT INTO {table_emission}
                ({colonne}, titre, slug, description, image_couverture_url,
                 info_animateur, info_producteur, langue,
                 theme_phare_id, theme_phare_autre, cadence, etat, cree_par)
             VALUES ($1, $2, $3, COALESCE($4, ''), $5, $6, $7, COALESCE($8, 'Français'),
                     $9, $10, $11, 'brouillon', $12)
             RETURNING id"
        ))
        .bind(support_id)
        .bind(body.titre.trim())
        .bind(&slug)
        .bind(body.description.as_deref().map(str::trim))
        .bind(body.image_couverture_url.as_deref().map(str::trim))
        .bind(body.info_animateur.as_deref().map(str::trim))
        .bind(body.info_producteur.as_deref().map(str::trim))
        .bind(body.langue.as_deref().map(str::trim))
        .bind(body.theme_phare_id)
        .bind(body.theme_phare_autre.as_deref().map(str::trim))
        .bind(cadence)
        .bind(moi)
        .fetch_one(pool.get_ref())
        .await?
    };

    journaliser(
        &req,
        pool.get_ref(),
        moi,
        "CREATE",
        &type_support,
        emission_id,
        None,
        Some(serde_json::json!({
            "titre": body.titre.trim(),
            "cadence": cadence,
            "support_id": support_id,
        })),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": emission_id, "slug": slug })),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/{type_support}/{support_id}/emissions — vue détenteur
// ═══════════════════════════════════════════════════════════════════════════

/// **Toutes** les émissions du support, y compris `brouillon` et sans épisode,
/// avec le décompte par état — c'est le tableau de bord de FR-042.
pub async fn lister_emissions_detenteur(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    let emissions = charger_emissions_du_support(pool.get_ref(), &type_support, support_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "emissions": emissions })),
        error: None,
    }))
}

/// Vue de gestion : décomptes par état inclus, aucun filtre sur `etat`.
/// Partagée avec le back-office (`admin::radio_tele`).
pub async fn charger_emissions_du_support(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
) -> Result<Vec<EmissionResponse>, ApiErreur> {
    let colonnes = colonnes_emission(type_support)
        .ok_or_else(|| ApiErreur::Validation("Type de support inconnu".into()))?;
    let table_emission = table_contenu_pour_support(type_support).expect("type de support validé");
    let table_support = table_pour_support(type_support).expect("type de support validé");
    let table_ep = table_episode(type_support).expect("type de support validé");
    let colonne = colonne_support(type_support).expect("type de support validé");

    let rows = sqlx::query_as::<_, EmissionRow>(&format!(
        "SELECT {colonnes},
                s.nom AS support_nom, s.slug AS support_slug,
                cat.nom AS theme_phare_nom,
                (SELECT COUNT(*) FROM {table_ep} ep
                  WHERE ep.emission_id = e.id AND ep.etat = 'publie' AND ep.deleted_at IS NULL)
                    AS nombre_episodes,
                (SELECT MAX(ep.valide_at) FROM {table_ep} ep
                  WHERE ep.emission_id = e.id AND ep.etat = 'publie' AND ep.deleted_at IS NULL)
                    AS dernier_episode_at,
                (SELECT COUNT(*) FROM {table_ep} ep
                  WHERE ep.emission_id = e.id AND ep.etat = 'en_attente' AND ep.deleted_at IS NULL)
                    AS episodes_en_attente,
                (SELECT COUNT(*) FROM {table_ep} ep
                  WHERE ep.emission_id = e.id AND ep.etat = 'rejete' AND ep.deleted_at IS NULL)
                    AS episodes_rejetes
           FROM {table_emission} e
           JOIN {table_support} s ON s.id = e.{colonne}
           LEFT JOIN shared.categorie cat ON cat.id = e.theme_phare_id
          WHERE e.{colonne} = $1 AND e.deleted_at IS NULL
          ORDER BY e.titre ASC, e.id ASC"
    ))
    .bind(support_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.iter().map(|r| r.to_response(type_support)).collect())
}

// ═══════════════════════════════════════════════════════════════════════════
// PUT / DELETE /api/medias/emissions/{id}
// ═══════════════════════════════════════════════════════════════════════════

pub async fn modifier_emission(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<EmissionRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let emission_id = chemin.into_inner();
    let (type_support, support_id, ancien) = contexte_emission(pool.get_ref(), emission_id).await?;
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    body.valider()?;
    let cadence = body.cadence_ou_defaut();
    valider_cadence(cadence)?;
    let table_emission = table_contenu_pour_support(&type_support).expect("type de support validé");

    sqlx::query(&format!(
        "UPDATE {table_emission}
            SET titre = $2, description = COALESCE($3, description),
                image_couverture_url = $4, info_animateur = $5, info_producteur = $6,
                langue = COALESCE($7, langue), theme_phare_id = $8,
                theme_phare_autre = $9, cadence = $10, updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(emission_id)
    .bind(body.titre.trim())
    .bind(body.description.as_deref().map(str::trim))
    .bind(body.image_couverture_url.as_deref().map(str::trim))
    .bind(body.info_animateur.as_deref().map(str::trim))
    .bind(body.info_producteur.as_deref().map(str::trim))
    .bind(body.langue.as_deref().map(str::trim))
    .bind(body.theme_phare_id)
    .bind(body.theme_phare_autre.as_deref().map(str::trim))
    .bind(cadence)
    .execute(pool.get_ref())
    .await?;

    journaliser(
        &req,
        pool.get_ref(),
        moi,
        "UPDATE",
        &type_support,
        emission_id,
        Some(ancien),
        Some(serde_json::json!({ "titre": body.titre.trim(), "cadence": cadence })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": emission_id })),
        error: None,
    }))
}

/// Suppression douce d'un programme.
///
/// **Refus `409`** tant qu'il compte des épisodes publiés (FR-010), avec leur
/// décompte dans le message. La clé étrangère `ON DELETE RESTRICT` fait le même
/// refus en dernier recours — celui-ci sert à le dire en français.
pub async fn supprimer_emission(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let emission_id = chemin.into_inner();
    let (type_support, support_id, ancien) = contexte_emission(pool.get_ref(), emission_id).await?;
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "co_detenteur").await?;

    refuser_si_episodes_publies(pool.get_ref(), &type_support, emission_id).await?;

    let table_emission = table_contenu_pour_support(&type_support).expect("type de support validé");
    sqlx::query(&format!(
        "UPDATE {table_emission} SET etat = 'supprime', deleted_at = NOW(), updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL"
    ))
    .bind(emission_id)
    .execute(pool.get_ref())
    .await?;

    journaliser(
        &req,
        pool.get_ref(),
        moi,
        "DELETE",
        &type_support,
        emission_id,
        Some(ancien),
        Some(serde_json::json!({ "etat": "supprime" })),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse::<()> {
        success: true,
        data: None,
        error: None,
    }))
}

/// FR-010 — une émission ne disparaît pas sous ses épisodes publiés.
pub async fn refuser_si_episodes_publies(
    pool: &PgPool,
    type_support: &str,
    emission_id: Uuid,
) -> Result<(), ApiErreur> {
    let table_ep = table_episode(type_support).expect("type de support validé");
    let publies: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM {table_ep}
          WHERE emission_id = $1 AND etat = 'publie' AND deleted_at IS NULL"
    ))
    .bind(emission_id)
    .fetch_one(pool)
    .await?;

    if publies > 0 {
        return Err(ApiErreur::Conflit(format!(
            "Ce programme compte {} épisode(s) publié(s) : retirez-les d'abord, ou dépubliez le programme.",
            publies
        )));
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// Utilitaires partagés
// ═══════════════════════════════════════════════════════════════════════════

/// Support porteur d'une émission, plus un instantané de son état — la garde ne
/// peut s'exercer qu'après avoir su de quel support elle relève.
pub async fn contexte_emission(
    pool: &PgPool,
    emission_id: Uuid,
) -> Result<(String, Uuid, serde_json::Value), ApiErreur> {
    for (type_support, table, colonne) in [
        ("chaine_tv", "media_content.emission_tele", "chaine_id"),
        ("station_radio", "media_content.emission_radio", "station_id"),
    ] {
        let ligne: Option<(Uuid, String, String, String)> = sqlx::query_as(&format!(
            "SELECT {colonne}, titre, cadence, etat FROM {table}
              WHERE id = $1 AND deleted_at IS NULL"
        ))
        .bind(emission_id)
        .fetch_optional(pool)
        .await?;

        if let Some((support_id, titre, cadence, etat)) = ligne {
            let instantane = serde_json::json!({
                "type_support": type_support,
                "support_id": support_id,
                "titre": titre,
                "cadence": cadence,
                "etat": etat,
            });
            return Ok((type_support.to_string(), support_id, instantane));
        }
    }

    Err(ApiErreur::NonTrouve("Programme introuvable".into()))
}

/// Slug unique : le suffixe numérique est ajouté tant que la contrainte
/// `UNIQUE` refuserait la ligne. Deux programmes homonymes sur deux chaînes
/// différentes sont légitimes — le slug, lui, est global.
pub async fn slug_unique(pool: &PgPool, table: &str, base: &str) -> Result<String, ApiErreur> {
    let base = if base.trim().is_empty() { "programme" } else { base };
    let mut candidat = base.to_string();
    let mut suffixe = 2;

    loop {
        let pris: bool = sqlx::query_scalar(&format!(
            "SELECT EXISTS(SELECT 1 FROM {table} WHERE slug = $1)"
        ))
        .bind(&candidat)
        .fetch_one(pool)
        .await?;

        if !pris {
            return Ok(candidat);
        }
        candidat = format!("{}-{}", base, suffixe);
        suffixe += 1;
        if suffixe > 500 {
            return Ok(format!("{}-{}", base, Uuid::new_v4()));
        }
    }
}

/// Toute mutation d'émission est journalisée (FR-045, principe VII).
pub async fn journaliser(
    req: &HttpRequest,
    pool: &PgPool,
    moi: Uuid,
    action: &str,
    type_support: &str,
    emission_id: Uuid,
    avant: Option<serde_json::Value>,
    apres: Option<serde_json::Value>,
) {
    let table = if type_support == "station_radio" {
        "emission_radio"
    }
    else {
        "emission_tele"
    };
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(moi),
        action,
        "media_content",
        table,
        Some(emission_id),
        avant,
        apres,
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}

/// Valide un type de support venu de l'URL, avec le message du projet.
pub fn exiger_type_support(type_support: &str) -> Result<(), ApiErreur> {
    valider_type_support(type_support)
}
