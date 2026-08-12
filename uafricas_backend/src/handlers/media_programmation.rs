//! Grille de programmation d'un support média
//! (feature 001-refonte-tele-radio US5 — migration 09n ; recadrée par 09q, US2).
//!
//! Endpoints :
//!   GET    /api/medias/{type_support}/{support_id}/grille      (public + `?vue=detenteur`)
//!   GET    /api/medias/{type_support}/{support_id}/diffusion   (public)
//!   POST   /api/medias/{type_support}/{support_id}/creneaux    (programmateur)
//!   PUT    /api/medias/creneaux/{id}                           (programmateur)
//!   DELETE /api/medias/creneaux/{id}                           (programmateur)
//!   GET    /api/medias/mes-alertes-cadence                     (membre)
//!
//! **Aucune tâche de fond.** Le créneau courant est résolu à la lecture par
//! `(NOW() AT TIME ZONE fuseau)`, et l'épisode diffusé par **rotation** depuis
//! `date_effet` — patron maison de la résolution paresseuse
//! (`rendez_vous.rs:184,190`).

use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_detention::{exiger_utilisateur_id, garde_detenteur};
use crate::models::media_detention::{table_contenu_pour_support, valider_type_support};
use crate::models::media_emission::{heures_anticipation_alerte, periode_heures_cadence};
use crate::models::media_episode::{colonne_media, table_episode};
use crate::models::media_programmation::{
    AlerteCadenceResponse, AlerteCadenceRow, CreneauRequest, CreneauRow, DiffusionResponse,
    RefContenu, SupportRef, CRENEAU_COLONNES,
};
use crate::services::audit;
use crate::ApiResponse;

// ═══════════════════════════════════════════════════════════════════════════
// Résolution paresseuse et rotation — le cœur d'US2
// ═══════════════════════════════════════════════════════════════════════════

/// Rang de l'occurrence courante, dans le **référentiel horaire du créneau**.
///
/// - Quotidien : nombre de jours écoulés depuis `date_effet`.
/// - Hebdomadaire : nombre de semaines écoulées depuis la PREMIÈRE occurrence,
///   c'est-à-dire le premier `jour_semaine` à partir de `date_effet`.
///
/// `floor()` et non la division entière : `/` tronque vers zéro en PostgreSQL,
/// ce qui décalerait le rang d'un cran pour une `date_effet` future.
const SQL_RANG_OCCURRENCE: &str = "
    CASE WHEN c.recurrence = 'quotidien'
         THEN ((NOW() AT TIME ZONE c.fuseau)::date - c.date_effet)::bigint
         ELSE floor((
                  ((NOW() AT TIME ZONE c.fuseau)::date - c.date_effet)
                  - ((7 + c.jour_semaine - EXTRACT(DOW FROM c.date_effet)::int) % 7)
              )::numeric / 7)::bigint
    END";

/// Épisode retenu pour l'occurrence courante.
///
/// La jointure latérale est **intérieure** : une émission sans épisode publié
/// (`total = 0`) ne produit aucune ligne, ce qui réalise FR-021 sans branche
/// supplémentaire — rien n'est annoncé plutôt qu'un créneau vide.
///
/// Le double modulo `((r % t) + t) % t` couvre une `date_effet` postérieure à
/// aujourd'hui : l'opérateur `%` de PostgreSQL conserve le signe du dividende,
/// un rang négatif produirait sinon un index hors borne.
const SQL_ROTATION: &str = "
    CROSS JOIN LATERAL (SELECT {RANG} AS rang) occ
    JOIN LATERAL (
        SELECT e.id, e.titre, e.slug, e.{COLONNE_MEDIA} AS media_url,
               e.image_couverture_url, e.numero_episode,
               ROW_NUMBER() OVER (ORDER BY e.ordre, e.created_at, e.id) - 1 AS idx,
               COUNT(*)     OVER ()                                        AS total
          FROM {TABLE_EPISODE} e
         WHERE e.emission_id = c.emission_id
           AND e.etat = 'publie' AND e.deleted_at IS NULL
    ) ep ON ep.idx = (((occ.rang % ep.total) + ep.total) % ep.total)";

const CHAMPS_ROTATION: &str = "
    m.titre AS emission_titre, m.slug AS emission_slug,
    m.image_couverture_url AS emission_image, m.etat AS emission_etat,
    m.cadence AS emission_cadence,
    ep.total AS nombre_episodes, ep.total AS total_episodes,
    ep.id AS episode_id, ep.titre AS episode_titre, ep.slug AS episode_slug,
    ep.media_url AS episode_media_url, ep.image_couverture_url AS episode_image,
    ep.numero_episode AS episode_numero,
    occ.rang AS rang_occurrence";

/// Sélection du créneau en cours sur un support, à l'instant de la lecture.
///
/// `(NOW() AT TIME ZONE c.fuseau)` ramène l'instant courant dans le référentiel
/// horaire de chaque créneau, ce qui autorise une grille panafricaine dont les
/// lignes n'ont pas toutes le même fuseau.
const SQL_DIFFUSION_EN_COURS: &str = "
    SELECT {COLONNES}, {CHAMPS}
      FROM media_content.creneau_programmation c
      JOIN {TABLE_EMISSION} m ON m.id = c.emission_id AND m.deleted_at IS NULL
      {ROTATION}
     WHERE c.type_support = $1::media_content.type_support_media
       AND c.support_id = $2
       AND c.actif = TRUE AND c.deleted_at IS NULL
       AND m.etat = 'publie'
       AND (c.recurrence = 'quotidien'
            OR c.jour_semaine = EXTRACT(DOW FROM (NOW() AT TIME ZONE c.fuseau))::smallint)
       AND (NOW() AT TIME ZONE c.fuseau)::time >= c.heure_debut
       AND (NOW() AT TIME ZONE c.fuseau)::time
             < c.heure_debut + make_interval(mins => c.duree_minutes)
     ORDER BY c.heure_debut DESC
     LIMIT 1";

/// Le prochain créneau du jour ; à défaut, le premier de la grille — qui
/// reviendra demain.
const SQL_CRENEAU_SUIVANT: &str = "
    SELECT {COLONNES}, {CHAMPS}
      FROM media_content.creneau_programmation c
      JOIN {TABLE_EMISSION} m ON m.id = c.emission_id AND m.deleted_at IS NULL
      {ROTATION}
     WHERE c.type_support = $1::media_content.type_support_media
       AND c.support_id = $2
       AND c.actif = TRUE AND c.deleted_at IS NULL
       AND m.etat = 'publie'
       AND (c.recurrence = 'quotidien'
            OR c.jour_semaine = EXTRACT(DOW FROM (NOW() AT TIME ZONE c.fuseau))::smallint)
       AND c.heure_debut > (NOW() AT TIME ZONE c.fuseau)::time
     ORDER BY c.heure_debut ASC
     LIMIT 1";

fn requete(modele: &str, type_support: &str) -> String {
    let table_emission = table_contenu_pour_support(type_support).unwrap_or("media_content.emission_tele");
    let table_ep = table_episode(type_support).unwrap_or("media_content.episode_tele");
    let colonne = colonne_media(type_support).unwrap_or("video_url");

    let rotation = SQL_ROTATION
        .replace("{RANG}", SQL_RANG_OCCURRENCE)
        .replace("{COLONNE_MEDIA}", colonne)
        .replace("{TABLE_EPISODE}", table_ep);

    modele
        .replace("{COLONNES}", CRENEAU_COLONNES)
        .replace("{CHAMPS}", CHAMPS_ROTATION)
        .replace("{TABLE_EMISSION}", table_emission)
        .replace("{ROTATION}", &rotation)
}

/// « Quel programme passe en ce moment, quel épisode, et qu'est-ce qui suit ? »
///
/// Réutilisée par les endpoints `sections` de `television.rs` et
/// `stations_radio.rs` : ces pages affichent « En ce moment » et « À suivre »
/// sans requête supplémentaire côté client. **Deux requêtes**, la rotation étant
/// une jointure latérale et non un aller-retour de plus (SC-010).
pub async fn diffusion_pour_support(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
) -> Result<DiffusionResponse, ApiErreur> {
    if table_contenu_pour_support(type_support).is_none() {
        return Ok(DiffusionResponse {
            diffusion_en_cours: None,
            creneau_suivant: None,
        });
    }

    let en_cours = sqlx::query_as::<_, CreneauRow>(&requete(SQL_DIFFUSION_EN_COURS, type_support))
        .bind(type_support)
        .bind(support_id)
        .fetch_optional(pool)
        .await?;

    let suivant = sqlx::query_as::<_, CreneauRow>(&requete(SQL_CRENEAU_SUIVANT, type_support))
        .bind(type_support)
        .bind(support_id)
        .fetch_optional(pool)
        .await?;

    Ok(DiffusionResponse {
        diffusion_en_cours: en_cours.map(|r| r.to_response()),
        creneau_suivant: suivant.map(|r| r.to_response()),
    })
}

/// GET /api/medias/{type_support}/{support_id}/diffusion
pub async fn obtenir_diffusion(
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;
    let diffusion = diffusion_pour_support(pool.get_ref(), &type_support, support_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(diffusion),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/{type_support}/{support_id}/grille
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct GrilleQuery {
    /// `detenteur` remonte aussi les créneaux **en défaut** — ceux dont
    /// l'émission n'a aucun épisode publié (FR-021, FR-024).
    pub vue: Option<String>,
}

/// La grille complète, lisible sans compte : c'est un programme de diffusion.
///
/// Un créneau dont l'émission n'a aucun épisode publié n'est **pas renvoyé au
/// public** (FR-021) ; il reste visible du détenteur, assorti de son alerte.
pub async fn lister_grille(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    params: web::Query<GrilleQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;
    let table_emission = table_contenu_pour_support(&type_support).expect("type de support validé");
    let table_ep = table_episode(&type_support).expect("type de support validé");

    // Le décompte d'épisodes publiés se lit ici, sans rotation : la grille dit
    // ce qui est programmé, pas ce qui passe à l'instant.
    let rows = sqlx::query_as::<_, CreneauRow>(&format!(
        "SELECT {CRENEAU_COLONNES},
                m.titre AS emission_titre, m.slug AS emission_slug,
                m.image_couverture_url AS emission_image, m.etat AS emission_etat,
                m.cadence AS emission_cadence,
                (SELECT COUNT(*) FROM {table_ep} e
                  WHERE e.emission_id = c.emission_id
                    AND e.etat = 'publie' AND e.deleted_at IS NULL) AS nombre_episodes
           FROM media_content.creneau_programmation c
           LEFT JOIN {table_emission} m ON m.id = c.emission_id AND m.deleted_at IS NULL
          WHERE c.type_support = $1::media_content.type_support_media
            AND c.support_id = $2
            AND c.actif = TRUE AND c.deleted_at IS NULL
          ORDER BY c.jour_semaine ASC NULLS FIRST, c.heure_debut ASC"
    ))
    .bind(&type_support)
    .bind(support_id)
    .fetch_all(pool.get_ref())
    .await?;

    // Un détenteur voit la grille entière, y compris ce qui a été retiré de
    // l'antenne — c'est à lui que l'alerte doit signaler un créneau en défaut.
    let est_detenteur = match crate::handlers::media_social::extraire_utilisateur_id(&req) {
        Some(moi) => garde_detenteur(
            pool.get_ref(),
            &type_support,
            support_id,
            moi,
            "programmateur",
        )
        .await
        .is_ok(),
        None => false,
    };
    let vue_detenteur = est_detenteur && params.vue.as_deref() == Some("detenteur");

    let creneaux: Vec<_> = rows
        .into_iter()
        .map(|r| {
            if est_detenteur {
                r.to_response()
            }
            else {
                r.to_response_publique()
            }
        })
        // Le public ne voit pas un créneau qui n'annonce rien (FR-021).
        .filter(|c| vue_detenteur || est_detenteur || !c.emission_indisponible)
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "creneaux": creneaux })),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// POST /api/medias/{type_support}/{support_id}/creneaux
// ═══════════════════════════════════════════════════════════════════════════

/// Séquence obligatoire (FR-022, edge case « co-détenteurs en concurrence ») :
///
/// 1. `BEGIN`
/// 2. `SELECT id FROM <support> WHERE id = $1 FOR UPDATE` — verrou sur le
///    **support parent**, qui sérialise toutes les modifications de sa grille,
///    y compris les insertions concurrentes qu'un `FOR UPDATE` sur les créneaux
///    existants ne verrouillerait pas
/// 3. recherche de chevauchement
/// 4. conflit → `409` détaillé, **sans** écrire
/// 5. `INSERT`, puis audit
/// 6. `COMMIT`
pub async fn creer_creneau(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
    body: web::Json<CreneauRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let (type_support, support_id) = chemin.into_inner();
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "programmateur").await?;

    let creneau = body.valider()?;
    let table_support = crate::models::media_detention::table_pour_support(&type_support)
        .expect("type de support validé");
    let table_emission = table_contenu_pour_support(&type_support).expect("type de support validé");

    let mut tx = pool.begin().await?;

    // (2) Verrou sérialisant la grille de ce support.
    sqlx::query(&format!("SELECT id FROM {table_support} WHERE id = $1 FOR UPDATE"))
        .bind(support_id)
        .execute(&mut *tx)
        .await?;

    // Un créneau ne programme que du contenu de son propre support : un
    // programme d'une autre chaîne n'a rien à faire dans cette grille.
    let emission_ok: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table_emission}
            WHERE id = $1 AND {colonne_support} = $2 AND deleted_at IS NULL)",
        colonne_support = colonne_support(&type_support)
    ))
    .bind(creneau.emission_id)
    .bind(support_id)
    .fetch_one(&mut *tx)
    .await?;
    if !emission_ok {
        return Err(ApiErreur::Validation(
            "Ce programme n'appartient pas à ce support".into(),
        ));
    }

    // (3) et (4)
    if let Some(conflit) = chevauchement(&mut tx, &type_support, support_id, &creneau, None).await? {
        return Err(ApiErreur::Conflit(conflit));
    }

    let creneau_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.creneau_programmation
            (type_support, support_id, emission_id, recurrence, jour_semaine,
             heure_debut, duree_minutes, fuseau, date_effet, cree_par)
         VALUES ($1::media_content.type_support_media, $2, $3, $4, $5, $6, $7, $8,
                 COALESCE($9::date, CURRENT_DATE), $10)
         RETURNING id",
    )
    .bind(&type_support)
    .bind(support_id)
    .bind(creneau.emission_id)
    .bind(&creneau.recurrence)
    .bind(creneau.jour_semaine)
    .bind(creneau.heure_debut)
    .bind(creneau.duree_minutes)
    .bind(&creneau.fuseau)
    .bind(creneau.date_effet)
    .bind(moi)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "media_content",
        "creneau_programmation",
        Some(creneau_id),
        None,
        Some(instantane(&creneau, &type_support, support_id)),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    // L'épisode courant est renvoyé pour que le détenteur voie IMMÉDIATEMENT
    // l'effet de sa date d'effet : sans lui, la rotation reste une abstraction.
    let episode_actuel = episode_actuel_du_creneau(pool.get_ref(), &type_support, creneau_id).await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": creneau_id,
            "episode_actuel": episode_actuel,
        })),
        error: None,
    }))
}

fn colonne_support(type_support: &str) -> &'static str {
    match type_support {
        "chaine_tv" => "chaine_id",
        _ => "station_id",
    }
}

/// Épisode que la rotation désignera à la prochaine occurrence de ce créneau.
///
/// Requête **hors plage horaire** : contrairement à `diffusion_pour_support`,
/// elle ne demande pas que l'heure courante tombe dans le créneau — c'est un
/// aperçu, pas une diffusion.
async fn episode_actuel_du_creneau(
    pool: &PgPool,
    type_support: &str,
    creneau_id: Uuid,
) -> Result<Option<RefContenu>, ApiErreur> {
    let table_emission = match table_contenu_pour_support(type_support) {
        Some(t) => t,
        None => return Ok(None),
    };
    let table_ep = table_episode(type_support).expect("type de support validé");
    let colonne = colonne_media(type_support).expect("type de support validé");

    let rotation = SQL_ROTATION
        .replace("{RANG}", SQL_RANG_OCCURRENCE)
        .replace("{COLONNE_MEDIA}", colonne)
        .replace("{TABLE_EPISODE}", table_ep);

    let ligne: Option<(Uuid, String, Option<String>, Option<String>, Option<i32>, i64)> =
        sqlx::query_as(&format!(
            "SELECT ep.id, ep.titre, ep.slug, ep.media_url, ep.numero_episode, occ.rang
               FROM media_content.creneau_programmation c
               JOIN {table_emission} m ON m.id = c.emission_id AND m.deleted_at IS NULL
               {rotation}
              WHERE c.id = $1 AND c.deleted_at IS NULL
              LIMIT 1"
        ))
        .bind(creneau_id)
        .fetch_optional(pool)
        .await?;

    Ok(ligne.map(|(id, titre, slug, media_url, numero, _rang)| RefContenu {
        id,
        titre,
        slug,
        image_couverture_url: None,
        media_url,
        numero_episode: numero,
        cadence: None,
        nombre_episodes: None,
    }))
}

fn instantane(
    creneau: &crate::models::media_programmation::CreneauValide,
    type_support: &str,
    support_id: Uuid,
) -> serde_json::Value {
    serde_json::json!({
        "type_support": type_support,
        "support_id": support_id,
        "emission_id": creneau.emission_id,
        "recurrence": creneau.recurrence,
        "jour_semaine": creneau.jour_semaine,
        "heure_debut": creneau.heure_debut.format("%H:%M").to_string(),
        "duree_minutes": creneau.duree_minutes,
        "fuseau": creneau.fuseau,
        "date_effet": creneau.date_effet.map(|d| d.format("%Y-%m-%d").to_string()),
    })
}

/// Deux arcs d'un cadran circulaire de `modulo` minutes se recouvrent-ils ?
///
/// Ramener chaque créneau à un instant UTC le fait potentiellement enjamber
/// minuit, alors même qu'il ne l'enjambe pas dans son fuseau local : la
/// comparaison linéaire `debut_a < fin_b && debut_b < fin_a` devient fausse.
/// Sur un cadran, deux arcs se recouvrent si l'un commence avant que l'autre
/// ne s'achève, dans un sens ou dans l'autre.
fn arcs_se_recouvrent(debut_a: i32, duree_a: i32, debut_b: i32, duree_b: i32, modulo: i32) -> bool {
    (debut_b - debut_a).rem_euclid(modulo) < duree_a
        || (debut_a - debut_b).rem_euclid(modulo) < duree_b
}

/// Position d'un créneau sur le cadran UTC, en minutes.
fn position_utc(
    heure: chrono::NaiveTime,
    jour_semaine: Option<i16>,
    decalage_min: i32,
    hebdomadaire: bool,
) -> i32 {
    use chrono::Timelike;
    let minutes_locales = (heure.num_seconds_from_midnight() / 60) as i32;
    if hebdomadaire {
        let jour = jour_semaine.unwrap_or(0) as i32;
        (jour * 1440 + minutes_locales - decalage_min).rem_euclid(10_080)
    }
    else {
        (minutes_locales - decalage_min).rem_euclid(1440)
    }
}

/// Cherche un créneau actif du support qui se chevaucherait avec celui proposé.
///
/// **Les fuseaux sont pris en compte.** Une comparaison d'heures locales naïves
/// diverge de `SQL_DIFFUSION_EN_COURS`, qui résout lui le créneau courant par
/// `NOW() AT TIME ZONE fuseau` — et elle se trompe dans les deux sens.
async fn chevauchement(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    type_support: &str,
    support_id: Uuid,
    creneau: &crate::models::media_programmation::CreneauValide,
    exclure_id: Option<Uuid>,
) -> Result<Option<String>, ApiErreur> {
    let candidats: Vec<(String, Option<i16>, chrono::NaiveTime, i32, i32, i32, String)> = sqlx::query_as(
        "SELECT c.recurrence, c.jour_semaine, c.heure_debut, c.duree_minutes,
                (EXTRACT(EPOCH FROM ((NOW() AT TIME ZONE c.fuseau)
                                   - (NOW() AT TIME ZONE 'UTC'))) / 60)::int AS decalage_min,
                (EXTRACT(EPOCH FROM ((NOW() AT TIME ZONE $3)
                                   - (NOW() AT TIME ZONE 'UTC'))) / 60)::int AS decalage_candidat,
                c.fuseau
           FROM media_content.creneau_programmation c
          WHERE c.type_support = $1::media_content.type_support_media
            AND c.support_id = $2
            AND c.actif = TRUE AND c.deleted_at IS NULL
            AND ($4::uuid IS NULL OR c.id <> $4)
          ORDER BY c.jour_semaine ASC NULLS FIRST, c.heure_debut ASC",
    )
    .bind(type_support)
    .bind(support_id)
    .bind(&creneau.fuseau)
    .bind(exclure_id)
    .fetch_all(&mut **tx)
    .await?;

    let propose_hebdo = creneau.recurrence == "hebdomadaire";

    let conflit = candidats.into_iter().find(
        |(recurrence, jour, heure, duree, decalage, decalage_candidat, _fuseau)| {
            let existant_hebdo = recurrence == "hebdomadaire";
            let sur_la_semaine = propose_hebdo && existant_hebdo;
            let modulo = if sur_la_semaine { 10_080 } else { 1440 };

            let position_proposee = position_utc(
                creneau.heure_debut,
                creneau.jour_semaine,
                *decalage_candidat,
                sur_la_semaine,
            );
            let position_existante = position_utc(*heure, *jour, *decalage, sur_la_semaine);

            arcs_se_recouvrent(
                position_proposee,
                creneau.duree_minutes,
                position_existante,
                *duree,
                modulo,
            )
        },
    );

    Ok(conflit.map(|(recurrence, jour, heure, duree, _, _, fuseau)| {
        let quand = match jour
            .and_then(|j| crate::models::media_programmation::JOURS_SEMAINE.get(j as usize))
        {
            Some(libelle) => format!("le {}", libelle),
            None => "chaque jour".to_string(),
        };
        format!(
            "Ce créneau en chevauche un autre ({} {}, à {} pendant {} minutes, fuseau {}). \
             Ajustez l'horaire ou la durée.",
            recurrence,
            quand,
            heure.format("%H:%M"),
            duree,
            fuseau
        )
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// PUT /api/medias/creneaux/{id}
// ═══════════════════════════════════════════════════════════════════════════

pub async fn modifier_creneau(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
    body: web::Json<CreneauRequest>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let creneau_id = chemin.into_inner();

    let (type_support, support_id, ancien) = charger_contexte(pool.get_ref(), creneau_id).await?;
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "programmateur").await?;

    let creneau = body.valider()?;
    let table_support = crate::models::media_detention::table_pour_support(&type_support)
        .expect("type de support validé");
    let table_emission = table_contenu_pour_support(&type_support).expect("type de support validé");

    let mut tx = pool.begin().await?;

    sqlx::query(&format!("SELECT id FROM {table_support} WHERE id = $1 FOR UPDATE"))
        .bind(support_id)
        .execute(&mut *tx)
        .await?;

    let emission_ok: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table_emission}
            WHERE id = $1 AND {colonne_support} = $2 AND deleted_at IS NULL)",
        colonne_support = colonne_support(&type_support)
    ))
    .bind(creneau.emission_id)
    .bind(support_id)
    .fetch_one(&mut *tx)
    .await?;
    if !emission_ok {
        return Err(ApiErreur::Validation(
            "Ce programme n'appartient pas à ce support".into(),
        ));
    }

    // Le créneau modifié s'exclut lui-même de la recherche de conflit, sans
    // quoi il se chevaucherait toujours avec sa propre version d'origine.
    if let Some(conflit) =
        chevauchement(&mut tx, &type_support, support_id, &creneau, Some(creneau_id)).await?
    {
        return Err(ApiErreur::Conflit(conflit));
    }

    sqlx::query(
        "UPDATE media_content.creneau_programmation
            SET emission_id = $2, recurrence = $3, jour_semaine = $4,
                heure_debut = $5, duree_minutes = $6, fuseau = $7,
                date_effet = COALESCE($8::date, date_effet), updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(creneau_id)
    .bind(creneau.emission_id)
    .bind(&creneau.recurrence)
    .bind(creneau.jour_semaine)
    .bind(creneau.heure_debut)
    .bind(creneau.duree_minutes)
    .bind(&creneau.fuseau)
    .bind(creneau.date_effet)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "media_content",
        "creneau_programmation",
        Some(creneau_id),
        Some(ancien),
        Some(instantane(&creneau, &type_support, support_id)),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    let episode_actuel = episode_actuel_du_creneau(pool.get_ref(), &type_support, creneau_id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "id": creneau_id,
            "episode_actuel": episode_actuel,
        })),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// DELETE /api/medias/creneaux/{id}
// ═══════════════════════════════════════════════════════════════════════════

pub async fn supprimer_creneau(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let creneau_id = chemin.into_inner();

    let (type_support, support_id, ancien) = charger_contexte(pool.get_ref(), creneau_id).await?;
    garde_detenteur(pool.get_ref(), &type_support, support_id, moi, "programmateur").await?;

    sqlx::query(
        "UPDATE media_content.creneau_programmation
            SET actif = FALSE, deleted_at = NOW(), updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(creneau_id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "DELETE",
        "media_content",
        "creneau_programmation",
        Some(creneau_id),
        Some(ancien),
        Some(serde_json::json!({ "actif": false, "deleted_at": "NOW()" })),
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

/// Support porteur du créneau et instantané de son état — la garde ne peut
/// s'exercer qu'après avoir su de quel support il relève.
async fn charger_contexte(
    pool: &PgPool,
    creneau_id: Uuid,
) -> Result<(String, Uuid, serde_json::Value), ApiErreur> {
    let ligne: Option<(
        String,
        Uuid,
        Uuid,
        String,
        Option<i16>,
        chrono::NaiveTime,
        i32,
        String,
        chrono::NaiveDate,
    )> = sqlx::query_as(
        "SELECT type_support::text, support_id, emission_id, recurrence, jour_semaine,
                heure_debut, duree_minutes, fuseau, date_effet
           FROM media_content.creneau_programmation
          WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(creneau_id)
    .fetch_optional(pool)
    .await?;

    let (type_support, support_id, emission_id, recurrence, jour, heure, duree, fuseau, date_effet) =
        ligne.ok_or_else(|| ApiErreur::NonTrouve("Créneau introuvable".into()))?;

    let instantane = serde_json::json!({
        "type_support": type_support,
        "support_id": support_id,
        "emission_id": emission_id,
        "recurrence": recurrence,
        "jour_semaine": jour,
        "heure_debut": heure.format("%H:%M").to_string(),
        "duree_minutes": duree,
        "fuseau": fuseau,
        "date_effet": date_effet.format("%Y-%m-%d").to_string(),
    });

    Ok((type_support, support_id, instantane))
}

// ═══════════════════════════════════════════════════════════════════════════
// GET /api/medias/mes-alertes-cadence
// ═══════════════════════════════════════════════════════════════════════════

/// Alertes de cadence des programmes détenus (FR-024).
///
/// Calcul **à la lecture**, aucune tâche de fond : pour chaque émission
/// périodique d'un support détenu, on compare la date du dernier épisode publié
/// à la cadence déclarée. `episodes_en_attente` évite l'alerte trompeuse — le
/// détenteur a fait sa part, la file de modération n'a pas suivi.
pub async fn mes_alertes_cadence(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = exiger_utilisateur_id(&req)?;
    let mut alertes: Vec<AlerteCadenceResponse> = Vec::new();

    for (type_support, table_support, table_emission, table_ep, colonne) in [
        (
            "chaine_tv",
            "media_content.chaine_tv",
            "media_content.emission_tele",
            "media_content.episode_tele",
            "chaine_id",
        ),
        (
            "station_radio",
            "media_content.station_radio",
            "media_content.emission_radio",
            "media_content.episode_radio",
            "station_id",
        ),
    ] {
        let lignes = sqlx::query_as::<_, AlerteCadenceRow>(&format!(
            "SELECT m.id AS emission_id, m.titre AS emission_titre, m.slug AS emission_slug,
                    m.cadence,
                    '{type_support}'::text AS type_support,
                    s.id AS support_id, s.nom AS support_nom,
                    (SELECT MAX(e.valide_at) FROM {table_ep} e
                      WHERE e.emission_id = m.id AND e.etat = 'publie' AND e.deleted_at IS NULL)
                        AS dernier_episode_at,
                    (SELECT COUNT(*) FROM {table_ep} e
                      WHERE e.emission_id = m.id AND e.etat = 'en_attente' AND e.deleted_at IS NULL)
                        AS episodes_en_attente,
                    (SELECT COUNT(*) FROM {table_ep} e
                      WHERE e.emission_id = m.id AND e.etat = 'publie' AND e.deleted_at IS NULL)
                        AS nombre_episodes
               FROM {table_emission} m
               JOIN {table_support} s ON s.id = m.{colonne}
              WHERE m.deleted_at IS NULL
                AND m.cadence <> 'ponctuelle'
                AND EXISTS (SELECT 1 FROM media_content.support_detenteur sd
                             WHERE sd.type_support = $1::media_content.type_support_media
                               AND sd.support_id = s.id
                               AND sd.utilisateur_id = $2
                               AND sd.actif = TRUE)"
        ))
        .bind(type_support)
        .bind(moi)
        .fetch_all(pool.get_ref())
        .await?;

        for ligne in lignes {
            let anticipation = match heures_anticipation_alerte(&ligne.cadence) {
                Some(h) => h,
                None => continue,
            };
            // 010 — la période vient de la cadence et n'est plus déduite d'un
            // « sinon » à deux branches. Le calcul précédent traitait TOUT ce
            // qui n'était pas quotidien comme hebdomadaire : un programme
            // mensuel aurait été signalé en retard dès le 8ᵉ jour.
            let periode_heures = match periode_heures_cadence(&ligne.cadence) {
                Some(h) => h,
                None => continue,
            };

            let (prochaine_echeance, niveau) = match ligne.dernier_episode_at {
                None => (None, "aucun_episode".to_string()),
                Some(dernier) => {
                    let echeance = dernier + chrono::Duration::hours(periode_heures);
                    let maintenant = chrono::Utc::now();
                    let niveau = if maintenant >= echeance {
                        "depassee"
                    }
                    else if maintenant >= echeance - chrono::Duration::hours(anticipation) {
                        "approche"
                    }
                    else {
                        continue; // rien à signaler, l'échéance est encore loin
                    };
                    (Some(echeance), niveau.to_string())
                }
            };

            // Une émission sans aucun épisode publié est en défaut quelle que
            // soit son échéance (FR-021).
            let niveau = if ligne.nombre_episodes == 0 {
                "aucun_episode".to_string()
            }
            else {
                niveau
            };

            alertes.push(AlerteCadenceResponse {
                emission: RefContenu {
                    id: ligne.emission_id,
                    titre: ligne.emission_titre,
                    slug: ligne.emission_slug,
                    image_couverture_url: None,
                    media_url: None,
                    numero_episode: None,
                    cadence: Some(ligne.cadence.clone()),
                    nombre_episodes: Some(ligne.nombre_episodes),
                },
                support: SupportRef {
                    r#type: ligne.type_support,
                    id: ligne.support_id,
                    nom: ligne.support_nom,
                },
                cadence: ligne.cadence,
                dernier_episode_at: ligne.dernier_episode_at,
                prochaine_echeance,
                niveau,
                episodes_en_attente: ligne.episodes_en_attente,
            });
        }
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "alertes": alertes })),
        error: None,
    }))
}
