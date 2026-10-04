//! Back-office des concours (feature 014, famille B) : programmation et
//! modération des participations.
//!
//! Toute lecture d'un concours passe par `resoudre_concours` (PC1) ; toute
//! mutation écrit une ligne d'audit (principe VII).

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::middleware::admin::AdminUtilisateur;
use crate::models::admin::jeu_concours::{
    BilanModeration, ConcoursAdmin, ConcoursQueryParams, ConcoursRequest, DecomptesConcours,
    ModerationGroupeeRequest, MotifRequest, ParticipationModeration, ParticipationsQueryParams,
};
use crate::models::jeu_concours::{ConcoursRow, Phase};
use crate::models::notification;
use crate::models::pagination::PaginatedResponse;
use crate::services::audit;
use crate::services::jeu_concours::{self as cycle, lien_concours, notif};
use crate::{verifier_permission, ApiResponse};

async fn auditer(
    pool: &PgPool,
    req: &HttpRequest,
    admin_id: Uuid,
    action: &str,
    table: &str,
    record_id: Option<Uuid>,
    avant: Option<serde_json::Value>,
    apres: Option<serde_json::Value>,
) {
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool, Some(admin_id), action, "jeu", table, record_id, avant, apres,
        ip.as_deref(), ua.as_deref(),
    )
    .await;
}

async fn decomptes(pool: &PgPool, id: Uuid) -> Result<DecomptesConcours, sqlx::Error> {
    sqlx::query_as::<_, DecomptesConcours>(
        "SELECT
            (SELECT COUNT(*) FROM jeu.participation WHERE concours_id = $1 AND etat = 'en_attente') AS en_attente,
            (SELECT COUNT(*) FROM jeu.participation WHERE concours_id = $1 AND etat = 'publiee')    AS publiees,
            (SELECT COUNT(*) FROM jeu.participation WHERE concours_id = $1 AND etat = 'rejetee')    AS rejetees,
            (SELECT COUNT(*) FROM jeu.confrontation WHERE concours_id = $1 AND choix_id IS NOT NULL) AS votes,
            (SELECT COUNT(*) FROM jeu.confrontation WHERE concours_id = $1 AND comptee)             AS votes_comptes",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

async fn vue_admin(pool: &PgPool, concours: ConcoursRow) -> Result<ConcoursAdmin, ApiErreur> {
    let d = decomptes(pool, concours.id).await?;
    Ok(ConcoursAdmin {
        phase: concours.phase(Utc::now()),
        concours,
        en_attente: d.en_attente,
        publiees: d.publiees,
        rejetees: d.rejetees,
        votes: d.votes,
        votes_comptes: d.votes_comptes,
    })
}

// ─── Concours ────────────────────────────────────────────────────────────────

/// GET /api/admin/jeu/concours : tous les concours, résolus, filtrables par phase.
pub async fn lister_concours(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<ConcoursQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");

    let ids: Vec<Uuid> = sqlx::query_scalar("SELECT id FROM jeu.concours ORDER BY appel_debut DESC")
        .fetch_all(pool.get_ref())
        .await?;
    let mut tous = Vec::with_capacity(ids.len());
    for id in ids {
        // PC1 : la liste fait avancer le cycle de chaque concours.
        let concours = cycle::resoudre_concours(pool.get_ref(), id).await?;
        let vue = vue_admin(pool.get_ref(), concours).await?;
        if params.phase.as_deref().is_none_or(|p| p.is_empty() || p == vue.phase.code()) {
            tous.push(vue);
        }
    }

    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let page = params.page.unwrap_or(1).max(1);
    let total = tous.len() as i64;
    let elements: Vec<ConcoursAdmin> =
        tous.into_iter().skip(((page - 1) * par_page) as usize).take(par_page as usize).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(elements, total, page, par_page)),
        error: None,
    }))
}

/// GET /api/admin/jeu/concours/{id}
pub async fn obtenir_concours(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let concours = cycle::resoudre_concours(pool.get_ref(), path.into_inner()).await?;
    let vue = vue_admin(pool.get_ref(), concours).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(vue), error: None }))
}

/// POST /api/admin/jeu/concours
pub async fn creer_concours(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<ConcoursRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let c = body.into_inner().nettoyer().map_err(ApiErreur::Validation)?;
    if c.appel_debut < Utc::now() - Duration::hours(1) {
        return Err(ApiErreur::Validation(
            "L'appel à participation ne peut pas s'ouvrir dans le passé".into(),
        ));
    }

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO jeu.concours
            (format, titre, theme, reglement, rattachement, image_url, appel_debut, vote_debut, vote_fin,
             participations_max, minimum_participations, votes_max, presentations_min, jury,
             jury_finalistes, jury_delai_jours, prime_participation, prime_podium, cree_par)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9,
                 COALESCE($10, 1), COALESCE($11, 4), $12, COALESCE($13, 10), COALESCE($14, FALSE),
                 COALESCE($15, 10), COALESCE($16, 7), $17, $18, $19)
         RETURNING id",
    )
    .bind(&c.format)
    .bind(&c.titre)
    .bind(&c.theme)
    .bind(&c.reglement)
    .bind(&c.rattachement)
    .bind(&c.image_url)
    .bind(c.appel_debut)
    .bind(c.vote_debut)
    .bind(c.vote_fin)
    .bind(c.participations_max)
    .bind(c.minimum_participations)
    .bind(c.votes_max)
    .bind(c.presentations_min)
    .bind(c.jury)
    .bind(c.jury_finalistes)
    .bind(c.jury_delai_jours)
    .bind(c.prime_participation)
    .bind(&c.prime_podium)
    .bind(admin.id)
    .fetch_one(pool.get_ref())
    .await?;

    let cree = cycle::charger_concours(pool.get_ref(), id).await?;
    auditer(pool.get_ref(), &req, admin.id, "CONCOURS_CREE", "concours", Some(id), None,
        serde_json::to_value(&cree).ok()).await;
    let vue = vue_admin(pool.get_ref(), cree).await?;
    Ok(HttpResponse::Created().json(ApiResponse { success: true, data: Some(vue), error: None }))
}

/// PUT /api/admin/jeu/concours/{id}
///
/// Avant l'ouverture de l'appel, tout se modifie. Après, seuls le thème, le
/// règlement, le visuel et les dates ENCORE À VENIR (FR-024) : changer les
/// règles d'un concours ouvert serait changer les règles en cours de partie.
pub async fn modifier_concours(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<ConcoursRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();
    let avant = cycle::resoudre_concours(pool.get_ref(), id).await?;
    let maintenant = Utc::now();
    let phase = avant.phase(maintenant);
    if matches!(phase, Phase::Resultats | Phase::Annule) {
        return Err(ApiErreur::Conflit("Un concours terminé ou annulé ne se modifie plus".into()));
    }
    let c = body.into_inner().nettoyer().map_err(ApiErreur::Validation)?;

    if phase != Phase::AVenir {
        let fige = |nom: &str, change: bool| {
            if change {
                Err(ApiErreur::Conflit(format!(
                    "« {nom} » ne se modifie plus : l'appel à participation est ouvert"
                )))
            } else {
                Ok(())
            }
        };
        fige("Format", c.format != avant.format)?;
        fige("Titre", c.titre != avant.titre)?;
        fige("Rattachement", c.rattachement != avant.rattachement)?;
        fige("Participations par membre", c.participations_max.unwrap_or(avant.participations_max) != avant.participations_max)?;
        fige("Minimum de participations", c.minimum_participations.unwrap_or(avant.minimum_participations) != avant.minimum_participations)?;
        fige("Plafond de votes", c.votes_max != avant.votes_max)?;
        fige("Présentations minimales", c.presentations_min.unwrap_or(avant.presentations_min) != avant.presentations_min)?;
        fige("Jury", c.jury.unwrap_or(avant.jury) != avant.jury)?;
        fige("Finalistes", c.jury_finalistes.unwrap_or(avant.jury_finalistes) != avant.jury_finalistes)?;
        fige("Délai du jury", c.jury_delai_jours.unwrap_or(avant.jury_delai_jours) != avant.jury_delai_jours)?;
        fige("Prime de participation", c.prime_participation != avant.prime_participation)?;
        fige("Primes du podium", c.prime_podium != avant.prime_podium)?;
        // Une date passée est acquise ; une date à venir peut bouger, mais pas
        // vers le passé.
        for (nom, ancienne, nouvelle) in [
            ("Ouverture de l'appel", avant.appel_debut, c.appel_debut),
            ("Ouverture du vote", avant.vote_debut, c.vote_debut),
            ("Clôture du vote", avant.vote_fin, c.vote_fin),
        ] {
            if nouvelle != ancienne && (ancienne <= maintenant || nouvelle <= maintenant) {
                return Err(ApiErreur::Conflit(format!(
                    "« {nom} » : seule une date encore à venir peut être déplacée, et pas dans le passé"
                )));
            }
        }
    }

    sqlx::query(
        "UPDATE jeu.concours SET
            format = $2, titre = $3, theme = $4, reglement = $5, rattachement = $6, image_url = $7,
            appel_debut = $8, vote_debut = $9, vote_fin = $10,
            participations_max = COALESCE($11, participations_max),
            minimum_participations = COALESCE($12, minimum_participations),
            votes_max = $13, presentations_min = COALESCE($14, presentations_min),
            jury = COALESCE($15, jury), jury_finalistes = COALESCE($16, jury_finalistes),
            jury_delai_jours = COALESCE($17, jury_delai_jours),
            prime_participation = $18, prime_podium = $19, updated_at = NOW()
          WHERE id = $1",
    )
    .bind(id)
    .bind(&c.format)
    .bind(&c.titre)
    .bind(&c.theme)
    .bind(&c.reglement)
    .bind(&c.rattachement)
    .bind(&c.image_url)
    .bind(c.appel_debut)
    .bind(c.vote_debut)
    .bind(c.vote_fin)
    .bind(c.participations_max)
    .bind(c.minimum_participations)
    .bind(c.votes_max)
    .bind(c.presentations_min)
    .bind(c.jury)
    .bind(c.jury_finalistes)
    .bind(c.jury_delai_jours)
    .bind(c.prime_participation)
    .bind(&c.prime_podium)
    .execute(pool.get_ref())
    .await?;

    let apres = cycle::charger_concours(pool.get_ref(), id).await?;
    auditer(pool.get_ref(), &req, admin.id, "CONCOURS_MODIFIE", "concours", Some(id),
        serde_json::to_value(&avant).ok(), serde_json::to_value(&apres).ok()).await;
    let vue = vue_admin(pool.get_ref(), apres).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(vue), error: None }))
}

/// DELETE /api/admin/jeu/concours/{id} : seulement avant l'ouverture de l'appel.
pub async fn supprimer_concours(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();
    let avant = cycle::resoudre_concours(pool.get_ref(), id).await?;
    if avant.phase(Utc::now()) != Phase::AVenir {
        return Err(ApiErreur::Conflit(
            "Un concours ouvert ne se supprime plus : annulez-le".into(),
        ));
    }
    sqlx::query("DELETE FROM jeu.concours WHERE id = $1").bind(id).execute(pool.get_ref()).await?;
    auditer(pool.get_ref(), &req, admin.id, "CONCOURS_SUPPRIME", "concours", Some(id),
        serde_json::to_value(&avant).ok(), None).await;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(serde_json::json!({ "supprime": true })), error: None }))
}

/// POST /api/admin/jeu/concours/{id}/annuler
pub async fn annuler_concours(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<MotifRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();
    let motif = body.motif.trim();
    if motif.is_empty() {
        return Err(ApiErreur::Validation("Le motif de l'annulation est obligatoire".into()));
    }
    let avant = cycle::resoudre_concours(pool.get_ref(), id).await?;
    if avant.etat != "actif" {
        return Err(ApiErreur::Conflit("Ce concours est déjà terminé ou annulé".into()));
    }
    cycle::annuler(pool.get_ref(), id, motif).await?;
    let apres = cycle::charger_concours(pool.get_ref(), id).await?;
    auditer(pool.get_ref(), &req, admin.id, "CONCOURS_ANNULE", "concours", Some(id),
        serde_json::to_value(&avant).ok(), serde_json::to_value(&apres).ok()).await;
    let vue = vue_admin(pool.get_ref(), apres).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(vue), error: None }))
}

// ─── Modération des participations ───────────────────────────────────────────

const MODERATION_SELECT: &str = "
    SELECT p.id, p.concours_id, c.titre AS concours_titre, p.auteur_id,
           u.nom AS auteur_nom, u.prenom AS auteur_prenom, p.media_type, p.media_url, p.legende,
           p.etat, p.motif_rejet, p.nombre_signalements, p.created_at
      FROM jeu.participation p
      JOIN jeu.concours c ON c.id = p.concours_id
      JOIN iam.utilisateur u ON u.id = p.auteur_id";

/// GET /api/admin/jeu/concours/participations : la file de modération,
/// tous concours confondus.
pub async fn lister_participations(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    params: web::Query<ParticipationsQueryParams>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let etat = params.etat.as_deref().filter(|e| !e.is_empty()).unwrap_or("en_attente");
    let par_page = params.par_page.unwrap_or(24).clamp(1, 100);
    let page = params.page.unwrap_or(1).max(1);

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM jeu.participation p
          WHERE ($1 = 'toutes' OR p.etat = $1) AND ($2::uuid IS NULL OR p.concours_id = $2)",
    )
    .bind(etat)
    .bind(params.concours)
    .fetch_one(pool.get_ref())
    .await?;

    let elements = sqlx::query_as::<_, ParticipationModeration>(&format!(
        "{MODERATION_SELECT}
          WHERE ($1 = 'toutes' OR p.etat = $1) AND ($2::uuid IS NULL OR p.concours_id = $2)
          ORDER BY p.created_at
          LIMIT $3 OFFSET $4"
    ))
    .bind(etat)
    .bind(params.concours)
    .bind(par_page)
    .bind((page - 1) * par_page)
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(PaginatedResponse::new(elements, total, page, par_page)),
        error: None,
    }))
}

async fn charger_moderation(pool: &PgPool, id: Uuid) -> Result<ParticipationModeration, ApiErreur> {
    sqlx::query_as::<_, ParticipationModeration>(&format!("{MODERATION_SELECT} WHERE p.id = $1"))
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Participation introuvable".into()))
}

/// Une décision de modération, commune à la décision unitaire et groupée.
/// `Ok(false)` : la participation n'était pas dans un état qui s'y prête.
async fn decider(
    pool: &PgPool,
    req: &HttpRequest,
    admin_id: Uuid,
    id: Uuid,
    accepter: bool,
    motif: Option<&str>,
) -> Result<bool, ApiErreur> {
    let avant = charger_moderation(pool, id).await?;
    let concours = cycle::resoudre_concours(pool, avant.concours_id).await?;
    if matches!(concours.phase(Utc::now()), Phase::Resultats | Phase::Annule) {
        return Err(ApiErreur::Conflit("Ce concours est terminé : ses participations sont figées".into()));
    }

    let touchees = if accepter {
        sqlx::query(
            "UPDATE jeu.participation
                SET etat = 'publiee', motif_rejet = NULL, modere_par = $2, modere_at = NOW(), updated_at = NOW()
              WHERE id = $1 AND etat = 'en_attente'",
        )
        .bind(id)
        .bind(admin_id)
        .execute(pool)
        .await?
        .rows_affected()
    } else {
        sqlx::query(
            "UPDATE jeu.participation
                SET etat = 'rejetee', motif_rejet = $3, modere_par = $2, modere_at = NOW(), updated_at = NOW()
              WHERE id = $1 AND etat IN ('en_attente', 'publiee')",
        )
        .bind(id)
        .bind(admin_id)
        .bind(motif)
        .execute(pool)
        .await?
        .rows_affected()
    };
    if touchees == 0 {
        return Ok(false);
    }

    let (type_notif, message, action) = if accepter {
        (
            notif::PARTICIPATION_ACCEPTEE,
            format!("Votre photo pour le concours « {} » est publiée.", concours.titre),
            "PARTICIPATION_ACCEPTEE",
        )
    } else {
        (
            notif::PARTICIPATION_REJETEE,
            format!(
                "Votre photo pour le concours « {} » n'a pas été retenue. Motif : {}",
                concours.titre,
                motif.unwrap_or_default()
            ),
            "PARTICIPATION_REJETEE",
        )
    };
    notification::creer_notification(pool, avant.auteur_id, type_notif, &message, Some(&lien_concours(concours.id))).await;

    let apres = charger_moderation(pool, id).await?;
    auditer(pool, req, admin_id, action, "participation", Some(id),
        serde_json::to_value(&avant).ok(), serde_json::to_value(&apres).ok()).await;
    Ok(true)
}

/// POST /api/admin/jeu/concours/participations/{id}/accepter
pub async fn accepter_participation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();
    if !decider(pool.get_ref(), &req, admin.id, id, true, None).await? {
        return Err(ApiErreur::Conflit("Seule une participation en attente peut être acceptée".into()));
    }
    let apres = charger_moderation(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

/// POST /api/admin/jeu/concours/participations/{id}/rejeter
pub async fn rejeter_participation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    body: web::Json<MotifRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let motif = body.motif.trim();
    if motif.is_empty() {
        return Err(ApiErreur::Validation("Le motif du rejet est obligatoire".into()));
    }
    let id = path.into_inner();
    if !decider(pool.get_ref(), &req, admin.id, id, false, Some(motif)).await? {
        return Err(ApiErreur::Conflit("Cette participation ne peut plus être rejetée".into()));
    }
    let apres = charger_moderation(pool.get_ref(), id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

/// POST /api/admin/jeu/concours/participations/moderation-groupee
pub async fn moderation_groupee(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<ModerationGroupeeRequest>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let accepter = match body.decision.as_str() {
        "accepter" => true,
        "rejeter" => false,
        _ => return Err(ApiErreur::Validation("Décision attendue : accepter ou rejeter".into())),
    };
    let motif = body.motif.as_deref().map(str::trim).filter(|m| !m.is_empty());
    if !accepter && motif.is_none() {
        return Err(ApiErreur::Validation("Le motif du rejet est obligatoire".into()));
    }

    let mut bilan = BilanModeration::default();
    for &id in &body.ids {
        match decider(pool.get_ref(), &req, admin.id, id, accepter, motif).await {
            Ok(true) if accepter => bilan.acceptees += 1,
            Ok(true) => bilan.rejetees += 1,
            _ => bilan.ignorees += 1,
        }
    }
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(bilan), error: None }))
}

/// POST /api/admin/jeu/concours/participations/{id}/retablir : `suspendue` →
/// `publiee`. Le compteur repart de zéro (sinon le seuil resterait franchi) ;
/// les signalements sont gardés pour l'historique.
pub async fn retablir_participation(
    admin: AdminUtilisateur,
    req: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let id = path.into_inner();
    let avant = charger_moderation(pool.get_ref(), id).await?;
    let touchees = sqlx::query(
        "UPDATE jeu.participation SET etat = 'publiee', nombre_signalements = 0, updated_at = NOW()
          WHERE id = $1 AND etat = 'suspendue'",
    )
    .bind(id)
    .execute(pool.get_ref())
    .await?
    .rows_affected();
    if touchees == 0 {
        return Err(ApiErreur::Conflit("Seule une participation suspendue se rétablit".into()));
    }
    let apres = charger_moderation(pool.get_ref(), id).await?;
    auditer(pool.get_ref(), &req, admin.id, "PARTICIPATION_RETABLIE", "participation", Some(id),
        serde_json::to_value(&avant).ok(), serde_json::to_value(&apres).ok()).await;
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(apres), error: None }))
}

/// GET /api/admin/jeu/concours/participations/{id}/signalements
pub async fn signalements_participation(
    admin: AdminUtilisateur,
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    verifier_permission!(admin, "jeu", "gerer");
    let lignes: Vec<(Uuid, String, String, String, chrono::DateTime<Utc>)> = sqlx::query_as(
        "SELECT s.id, u.prenom, u.nom, s.motif, s.created_at
           FROM jeu.signalement_participation s
           JOIN iam.utilisateur u ON u.id = s.utilisateur_id
          WHERE s.participation_id = $1
          ORDER BY s.created_at DESC",
    )
    .bind(path.into_inner())
    .fetch_all(pool.get_ref())
    .await?;
    let data: Vec<serde_json::Value> = lignes
        .into_iter()
        .map(|(id, prenom, nom, motif, date)| {
            serde_json::json!({ "id": id, "membre": format!("{prenom} {nom}"), "motif": motif, "created_at": date })
        })
        .collect();
    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some(data), error: None }))
}

