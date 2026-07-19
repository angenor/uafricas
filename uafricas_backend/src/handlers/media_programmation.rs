//! Grille de programmation d'un support média
//! (feature 001-refonte-tele-radio, US5 — migration 09n).
//!
//! Endpoints :
//!   GET    /api/medias/{type_support}/{support_id}/grille      (public)
//!   GET    /api/medias/{type_support}/{support_id}/diffusion   (public)
//!   POST   /api/medias/{type_support}/{support_id}/creneaux    (programmateur)
//!   PUT    /api/medias/creneaux/{id}                           (programmateur)
//!   DELETE /api/medias/creneaux/{id}                           (programmateur)
//!
//! **Aucune tâche de fond.** Le créneau courant est résolu à la lecture, par
//! calcul SQL sur `(NOW() AT TIME ZONE fuseau)` — patron maison de la
//! résolution paresseuse (`rendez_vous.rs:184,190`, R7).

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::handlers::media_detention::{exiger_utilisateur_id, garde_detenteur};
use crate::models::media_detention::{table_contenu_pour_support, valider_type_support};
use crate::models::media_programmation::{
    CreneauRequest, CreneauRow, DiffusionResponse, CRENEAU_COLONNES,
};
use crate::services::audit;
use crate::ApiResponse;

// ═══════════════════════════════════════════════════════════════════════════
// Résolution paresseuse — le cœur d'US5
// ═══════════════════════════════════════════════════════════════════════════

/// Sélection du créneau en cours sur un support, à l'instant de la lecture.
///
/// `(NOW() AT TIME ZONE c.fuseau)` ramène l'instant courant dans le référentiel
/// horaire de chaque créneau, ce qui autorise une grille panafricaine dont les
/// lignes n'ont pas toutes le même fuseau.
///
/// Le contenu est joint et filtré sur `etat = 'publie'` : un contenu suspendu
/// ou retiré cesse d'être diffusé à la requête suivante, sans intervention
/// (FR-041, FR-043, edge case « contenu signalé pendant sa diffusion »).
const SQL_DIFFUSION_EN_COURS: &str = "
    SELECT {COLONNES},
           p.nom_emission AS contenu_nom, p.slug AS contenu_slug,
           p.image_couverture_url AS contenu_image, p.etat AS contenu_etat
      FROM media_content.creneau_programmation c
      JOIN {TABLE_CONTENU} p ON p.id = c.contenu_id AND p.deleted_at IS NULL
     WHERE c.type_support = $1::media_content.type_support_media
       AND c.support_id = $2
       AND c.actif = TRUE AND c.deleted_at IS NULL
       AND p.etat = 'publie'
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
    SELECT {COLONNES},
           p.nom_emission AS contenu_nom, p.slug AS contenu_slug,
           p.image_couverture_url AS contenu_image, p.etat AS contenu_etat
      FROM media_content.creneau_programmation c
      JOIN {TABLE_CONTENU} p ON p.id = c.contenu_id AND p.deleted_at IS NULL
     WHERE c.type_support = $1::media_content.type_support_media
       AND c.support_id = $2
       AND c.actif = TRUE AND c.deleted_at IS NULL
       AND p.etat = 'publie'
       AND (c.recurrence = 'quotidien'
            OR c.jour_semaine = EXTRACT(DOW FROM (NOW() AT TIME ZONE c.fuseau))::smallint)
       AND c.heure_debut > (NOW() AT TIME ZONE c.fuseau)::time
     ORDER BY c.heure_debut ASC
     LIMIT 1";

fn requete(modele: &str, table_contenu: &str) -> String {
    modele
        .replace("{COLONNES}", CRENEAU_COLONNES)
        .replace("{TABLE_CONTENU}", table_contenu)
}

/// « Qu'est-ce qui passe en ce moment, et qu'est-ce qui suit ? »
///
/// Réutilisée par les endpoints `sections` de `television.rs` et
/// `stations_radio.rs` : ces pages affichent « En ce moment » et « À suivre »
/// sans requête supplémentaire côté client (FR-039).
pub async fn diffusion_pour_support(
    pool: &PgPool,
    type_support: &str,
    support_id: Uuid,
) -> Result<DiffusionResponse, ApiErreur> {
    let table_contenu = match table_contenu_pour_support(type_support) {
        Some(t) => t,
        None => {
            return Ok(DiffusionResponse {
                diffusion_en_cours: None,
                creneau_suivant: None,
            })
        }
    };

    let en_cours = sqlx::query_as::<_, CreneauRow>(&requete(SQL_DIFFUSION_EN_COURS, table_contenu))
        .bind(type_support)
        .bind(support_id)
        .fetch_optional(pool)
        .await?;

    let suivant = sqlx::query_as::<_, CreneauRow>(&requete(SQL_CRENEAU_SUIVANT, table_contenu))
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

/// La grille complète, lisible sans compte : c'est un programme de diffusion.
/// Les créneaux dont le contenu n'est plus publié sont conservés et marqués
/// `contenu_indisponible`, pour que leurs co-détenteurs les corrigent (FR-043).
pub async fn lister_grille(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    chemin: web::Path<(String, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    let (type_support, support_id) = chemin.into_inner();
    valider_type_support(&type_support)?;
    let table_contenu = table_contenu_pour_support(&type_support).expect("type de support validé");

    let rows = sqlx::query_as::<_, CreneauRow>(&format!(
        "SELECT {CRENEAU_COLONNES},
                p.nom_emission AS contenu_nom, p.slug AS contenu_slug,
                p.image_couverture_url AS contenu_image, p.etat AS contenu_etat
           FROM media_content.creneau_programmation c
           LEFT JOIN {table_contenu} p ON p.id = c.contenu_id AND p.deleted_at IS NULL
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
    // l'antenne — c'est à lui que FR-041 doit signaler un créneau invalide.
    // Pour tout autre visiteur, les champs décrivant un contenu retiré sont tus.
    let est_detenteur = match crate::handlers::media_social::extraire_utilisateur_id(&req) {
        Some(moi) => crate::handlers::media_detention::garde_detenteur(
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

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(
            rows.into_iter()
                .map(|r| {
                    if est_detenteur {
                        r.to_response()
                    }
                    else {
                        r.to_response_publique()
                    }
                })
                .collect::<Vec<_>>(),
        ),
        error: None,
    }))
}

// ═══════════════════════════════════════════════════════════════════════════
// POST /api/medias/{type_support}/{support_id}/creneaux
// ═══════════════════════════════════════════════════════════════════════════

/// Séquence obligatoire (FR-040, edge case « co-détenteurs en concurrence ») :
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
    let table_contenu = table_contenu_pour_support(&type_support).expect("type de support validé");

    let mut tx = pool.begin().await?;

    // (2) Verrou sérialisant la grille de ce support.
    sqlx::query(&format!("SELECT id FROM {table_support} WHERE id = $1 FOR UPDATE"))
        .bind(support_id)
        .execute(&mut *tx)
        .await?;

    // Un créneau ne programme que du contenu de son propre support : une
    // émission d'une autre chaîne n'a rien à faire dans cette grille.
    let contenu_ok: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table_contenu}
            WHERE id = $1 AND {colonne_support} = $2 AND deleted_at IS NULL)",
        colonne_support = colonne_support(&type_support)
    ))
    .bind(creneau.contenu_id)
    .bind(support_id)
    .fetch_one(&mut *tx)
    .await?;
    if !contenu_ok {
        return Err(ApiErreur::Validation(
            "Ce contenu n'appartient pas à ce support".into(),
        ));
    }

    // (3) et (4)
    if let Some(conflit) = chevauchement(
        &mut tx,
        &type_support,
        support_id,
        &creneau,
        None,
    )
    .await?
    {
        return Err(ApiErreur::Conflit(conflit));
    }

    let creneau_id: Uuid = sqlx::query_scalar(
        "INSERT INTO media_content.creneau_programmation
            (type_support, support_id, contenu_id, recurrence, jour_semaine,
             heure_debut, duree_minutes, fuseau, cree_par)
         VALUES ($1::media_content.type_support_media, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id",
    )
    .bind(&type_support)
    .bind(support_id)
    .bind(creneau.contenu_id)
    .bind(&creneau.recurrence)
    .bind(creneau.jour_semaine)
    .bind(creneau.heure_debut)
    .bind(creneau.duree_minutes)
    .bind(&creneau.fuseau)
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

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": creneau_id })),
        error: None,
    }))
}

fn colonne_support(type_support: &str) -> &'static str {
    match type_support {
        "chaine_tv" => "chaine_id",
        _ => "station_id",
    }
}

fn instantane(
    creneau: &crate::models::media_programmation::CreneauValide,
    type_support: &str,
    support_id: Uuid,
) -> serde_json::Value {
    serde_json::json!({
        "type_support": type_support,
        "support_id": support_id,
        "contenu_id": creneau.contenu_id,
        "recurrence": creneau.recurrence,
        "jour_semaine": creneau.jour_semaine,
        "heure_debut": creneau.heure_debut.format("%H:%M").to_string(),
        "duree_minutes": creneau.duree_minutes,
        "fuseau": creneau.fuseau,
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
///
/// `decalage_min` est l'écart du fuseau à UTC au jour courant : soustrait de
/// l'heure locale, il ramène tous les créneaux d'un même support sur un
/// référentiel unique. Pour un créneau hebdomadaire, le jour est intégré au
/// calcul — un décalage de fuseau peut faire changer de jour.
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
/// `NOW() AT TIME ZONE fuseau` — et elle se trompe dans les deux sens : elle
/// accepte deux créneaux réellement simultanés (dont l'un évincerait
/// silencieusement l'autre au `LIMIT 1`) et refuse en 409 des créneaux sans
/// conflit réel. La garantie de non-chevauchement, cœur d'US5, ne tient que si
/// les deux logiques partagent le même référentiel.
///
/// Les candidats sont ramenés en mémoire — un support en compte quelques
/// dizaines au plus, et le verrou `FOR UPDATE` sur le support parent est déjà
/// posé — puis comparés sur un cadran circulaire : quotidien contre quotidien
/// sur 1 440 minutes, hebdomadaire contre hebdomadaire sur 10 080. Un créneau
/// quotidien revenant chaque jour, sa confrontation à un hebdomadaire se joue
/// sur le cadran journalier.
async fn chevauchement(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    type_support: &str,
    support_id: Uuid,
    creneau: &crate::models::media_programmation::CreneauValide,
    exclure_id: Option<Uuid>,
) -> Result<Option<String>, ApiErreur> {
    // `decalage_min` : écart du fuseau à UTC, calculé au jour courant par
    // PostgreSQL, qui porte la base de fuseaux. `decalage_candidat` est celui
    // du créneau proposé, obtenu dans la même requête pour rester cohérent.
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
            // Dès qu'un des deux revient chaque jour, la confrontation se joue
            // sur le cadran journalier : le jour de l'autre y coïncide toujours.
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
        let quand = match jour.and_then(|j| {
            crate::models::media_programmation::JOURS_SEMAINE.get(j as usize)
        }) {
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
    let table_contenu = table_contenu_pour_support(&type_support).expect("type de support validé");

    let mut tx = pool.begin().await?;

    sqlx::query(&format!("SELECT id FROM {table_support} WHERE id = $1 FOR UPDATE"))
        .bind(support_id)
        .execute(&mut *tx)
        .await?;

    let contenu_ok: bool = sqlx::query_scalar(&format!(
        "SELECT EXISTS(SELECT 1 FROM {table_contenu}
            WHERE id = $1 AND {colonne_support} = $2 AND deleted_at IS NULL)",
        colonne_support = colonne_support(&type_support)
    ))
    .bind(creneau.contenu_id)
    .bind(support_id)
    .fetch_one(&mut *tx)
    .await?;
    if !contenu_ok {
        return Err(ApiErreur::Validation(
            "Ce contenu n'appartient pas à ce support".into(),
        ));
    }

    // Le créneau modifié s'exclut lui-même de la recherche de conflit, sans
    // quoi il se chevaucherait toujours avec sa propre version d'origine.
    if let Some(conflit) = chevauchement(
        &mut tx,
        &type_support,
        support_id,
        &creneau,
        Some(creneau_id),
    )
    .await?
    {
        return Err(ApiErreur::Conflit(conflit));
    }

    sqlx::query(
        "UPDATE media_content.creneau_programmation
            SET contenu_id = $2, recurrence = $3, jour_semaine = $4,
                heure_debut = $5, duree_minutes = $6, fuseau = $7, updated_at = NOW()
          WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(creneau_id)
    .bind(creneau.contenu_id)
    .bind(&creneau.recurrence)
    .bind(creneau.jour_semaine)
    .bind(creneau.heure_debut)
    .bind(creneau.duree_minutes)
    .bind(&creneau.fuseau)
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

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "id": creneau_id })),
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
    let ligne: Option<(String, Uuid, Uuid, String, Option<i16>, chrono::NaiveTime, i32, String)> =
        sqlx::query_as(
            "SELECT type_support::text, support_id, contenu_id, recurrence, jour_semaine,
                    heure_debut, duree_minutes, fuseau
               FROM media_content.creneau_programmation
              WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(creneau_id)
        .fetch_optional(pool)
        .await?;

    let (type_support, support_id, contenu_id, recurrence, jour, heure, duree, fuseau) =
        ligne.ok_or_else(|| ApiErreur::NonTrouve("Créneau introuvable".into()))?;

    let instantane = serde_json::json!({
        "type_support": type_support,
        "support_id": support_id,
        "contenu_id": contenu_id,
        "recurrence": recurrence,
        "jour_semaine": jour,
        "heure_debut": heure.format("%H:%M").to_string(),
        "duree_minutes": duree,
        "fuseau": fuseau,
    });

    Ok((type_support, support_id, instantane))
}
