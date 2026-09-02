// Handlers du domaine social : rendez-vous en visioconférence entre amis.
// Le backend orchestre la prise de rendez-vous (proposer → répondre /
// contre-proposer → accepter → annuler) et l'accès à la salle visio P2P ;
// aucun média ne transite par le serveur (WebRTC pair-à-pair).
//
// Réutilisation maximale du domaine social (Principe V) : MembreLight,
// amitié/blocage, RegistreSse, notification cloche unifiée, audit.

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::amitie::{obtenir_membre_light, paire_canonique};
use crate::models::notification::creer_notification;
use crate::models::rendez_vous::{
    evt_rdv_accepte, evt_rdv_annule, evt_rdv_contre_propose, evt_rdv_propose, evt_rdv_refuse,
    peer_id, peut_rejoindre, ContreProposerBody, ListeRendezVous, ProposerBody,
    RendezVousAvecAutreRow, SalleResponse, DUREES_AUTORISEES, SELECT_AVEC_AUTRE,
};
use crate::services::audit;
use crate::services::messagerie_sse::RegistreSse;
use crate::ApiResponse;

/// Lien d'action de la cloche : ouvre le panneau messagerie sur l'onglet RDV.
const LIEN_RDV: &str = "/?rdv=1";

// ════════════════════════════════════════════════════════════════
// Authentification & helpers de relation
// ════════════════════════════════════════════════════════════════

fn extraire_utilisateur_id(req: &HttpRequest) -> Option<Uuid> {
    let header = req.headers().get("Authorization")?.to_str().ok()?;
    let token = header.strip_prefix("Bearer ")?;
    let secret = std::env::var("JWT_SECRET").ok()?;
    let claims = jwt::valider_token(token, &secret).ok()?;
    Uuid::parse_str(&claims.sub).ok()
}

fn utilisateur_courant(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    extraire_utilisateur_id(req)
        .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

/// Blocage dans un sens ou l'autre (réutilise le schéma social).
async fn blocage_existe(pool: &PgPool, a: Uuid, b: Uuid) -> Result<bool, ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.blocage
            WHERE (bloqueur_id = $1 AND bloque_id = $2)
               OR (bloqueur_id = $2 AND bloque_id = $1)
        )",
    )
    .bind(a)
    .bind(b)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

/// Amitié active entre a et b (ordre canonique).
async fn amitie_existe(pool: &PgPool, a: Uuid, b: Uuid) -> Result<bool, ApiErreur> {
    let (min, max) = paire_canonique(a, b);
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.amitie
            WHERE utilisateur_a_id = $1 AND utilisateur_b_id = $2
        )",
    )
    .bind(min)
    .bind(max)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

/// Revérifie amitié active + absence de blocage à chaque action (FR-034).
async fn verifier_relation(pool: &PgPool, moi: Uuid, autre: Uuid) -> Result<(), ApiErreur> {
    if blocage_existe(pool, moi, autre).await? || !amitie_existe(pool, moi, autre).await? {
        return Err(ApiErreur::AccesInterdit(
            "Action impossible avec ce membre".to_string(),
        ));
    }
    Ok(())
}

/// « Prénom Nom » de l'acteur pour le libellé de notification (jamais le sujet).
async fn nom_acteur(pool: &PgPool, acteur_id: Uuid) -> String {
    match obtenir_membre_light(pool, acteur_id).await {
        Ok(Some(m)) => format!("{} {}", m.prenom, m.nom),
        _ => "Un membre".to_string(),
    }
}

/// Charge un rendez-vous (avec l'« autre » membre) pour un participant.
/// 404 si introuvable, supprimé, ou non-participant.
async fn charger_avec_autre(
    pool: &PgPool,
    moi: Uuid,
    id: Uuid,
) -> Result<RendezVousAvecAutreRow, ApiErreur> {
    let query = format!(
        "{SELECT_AVEC_AUTRE} WHERE rv.id = $2 AND rv.deleted_at IS NULL \
         AND (rv.initiateur_id = $1 OR rv.destinataire_id = $1)"
    );
    sqlx::query_as::<_, RendezVousAvecAutreRow>(&query)
        .bind(moi)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Rendez-vous introuvable".to_string()))
}

/// Validation commune sujet/durée/date pour proposer & contre-proposer.
fn valider_duree(duree_minutes: i16) -> Result<(), ApiErreur> {
    if !DUREES_AUTORISEES.contains(&duree_minutes) {
        return Err(ApiErreur::Validation(
            "Durée invalide (15, 30, 45 ou 60 minutes)".to_string(),
        ));
    }
    Ok(())
}

fn valider_date_future(date_heure: chrono::DateTime<Utc>) -> Result<(), ApiErreur> {
    if date_heure <= Utc::now() {
        return Err(ApiErreur::Validation(
            "La date du rendez-vous doit être dans le futur".to_string(),
        ));
    }
    Ok(())
}

/// Journalise une mutation sans contenu sensible (FR-033).
async fn auditer(
    pool: &PgPool,
    req: &HttpRequest,
    moi: Uuid,
    action: &str,
    id: Uuid,
    nouvel_etat: serde_json::Value,
) {
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(moi),
        action,
        "social",
        "rendez_vous",
        Some(id),
        None,
        Some(nouvel_etat),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}

// ════════════════════════════════════════════════════════════════
// Lecture (US foundational)
// ════════════════════════════════════════════════════════════════

#[derive(Debug, serde::Deserialize)]
pub struct ListerParams {
    pub filtre: Option<String>,
    pub page: Option<i64>,
    pub taille: Option<i64>,
}

/// Clause SQL + sens de tri pour chaque filtre (data-model.md).
fn clause_filtre(filtre: Option<&str>) -> (&'static str, &'static str) {
    match filtre {
        Some("attente_moi") => (
            "rv.statut = 'propose' AND rv.tour_id = $1 AND rv.date_heure >= NOW()",
            "ASC",
        ),
        Some("attente_autre") => (
            "rv.statut = 'propose' AND rv.tour_id <> $1 AND rv.date_heure >= NOW()",
            "ASC",
        ),
        Some("a_venir") => (
            "rv.statut = 'accepte' AND (rv.date_heure + make_interval(mins => rv.duree_minutes::int)) >= NOW()",
            "ASC",
        ),
        Some("passes") => (
            "(rv.statut IN ('refuse','annule') \
              OR (rv.statut = 'propose' AND rv.date_heure < NOW()) \
              OR (rv.statut = 'accepte' AND (rv.date_heure + make_interval(mins => rv.duree_minutes::int)) < NOW()))",
            "DESC",
        ),
        _ => ("TRUE", "DESC"),
    }
}

/// GET /api/rendez-vous?filtre=&page=&taille=, Liste paginée (FR-019/020).
pub async fn lister(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<ListerParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let page = params.page.unwrap_or(1).max(1);
    let taille = params.taille.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * taille;
    let (clause, tri) = clause_filtre(params.filtre.as_deref());

    let base = format!(
        "(rv.initiateur_id = $1 OR rv.destinataire_id = $1) AND rv.deleted_at IS NULL AND {clause}"
    );

    let total: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM social.rendez_vous rv WHERE {base}"
    ))
    .bind(moi)
    .fetch_one(pool.get_ref())
    .await?;

    let query = format!(
        "{SELECT_AVEC_AUTRE} WHERE {base} ORDER BY rv.date_heure {tri} LIMIT $2 OFFSET $3"
    );
    let rows = sqlx::query_as::<_, RendezVousAvecAutreRow>(&query)
        .bind(moi)
        .bind(taille)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await?;

    let maintenant = Utc::now();
    let items = rows.iter().map(|r| r.to_response(moi, maintenant)).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(ListeRendezVous {
            items,
            page,
            taille,
            total,
        }),
        error: None,
    }))
}

/// GET /api/rendez-vous/{id} : Détail pour un participant.
pub async fn detail(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();
    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(moi, Utc::now())),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US1 : Proposer
// ════════════════════════════════════════════════════════════════

/// POST /api/rendez-vous : Proposer un rendez-vous (FR-006..FR-011).
pub async fn proposer(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    body: web::Json<ProposerBody>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let cible = body.destinataire_id;

    // FR-009 : pas soi-même.
    if cible == moi {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas vous proposer un rendez-vous à vous-même".to_string(),
        ));
    }
    // FR-006/010 : sujet non vide ≤ 150.
    let sujet = body.sujet.trim();
    if sujet.is_empty() || sujet.chars().count() > 150 {
        return Err(ApiErreur::Validation(
            "Le sujet est obligatoire (150 caractères maximum)".to_string(),
        ));
    }
    valider_duree(body.duree_minutes)?;
    valider_date_future(body.date_heure)?;
    // FR-001/003 : amis + non bloqués.
    verifier_relation(pool.get_ref(), moi, cible).await?;

    let description = body
        .description
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO social.rendez_vous
            (initiateur_id, destinataire_id, sujet, description, date_heure, duree_minutes, statut, tour_id)
         VALUES ($1, $2, $3, $4, $5, $6, 'propose', $2)
         RETURNING id",
    )
    .bind(moi)
    .bind(cible)
    .bind(sujet)
    .bind(description)
    .bind(body.date_heure)
    .bind(body.duree_minutes)
    .fetch_one(pool.get_ref())
    .await?;

    // Notification cloche + temps réel au destinataire (sans contenu sensible).
    let acteur = nom_acteur(pool.get_ref(), moi).await;
    creer_notification(
        pool.get_ref(),
        cible,
        "rdv_propose",
        &format!("{acteur} vous a proposé un rendez-vous."),
        Some(LIEN_RDV),
    )
    .await;
    sse.publier(cible, &evt_rdv_propose(id));

    auditer(
        pool.get_ref(),
        &req,
        moi,
        "CREATE",
        id,
        serde_json::json!({ "destinataire_id": cible, "statut": "propose", "date_heure": body.date_heure }),
    )
    .await;

    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(row.to_response(moi, Utc::now())),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US2 : Répondre (accepter / refuser / contre-proposer)
// ════════════════════════════════════════════════════════════════

/// POST /api/rendez-vous/{id}/accepter (FR-013, FR-035).
pub async fn accepter(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    repondre(pool, sse, req, chemin, Reponse::Accepter).await
}

/// POST /api/rendez-vous/{id}/refuser (FR-014).
pub async fn refuser(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    repondre(pool, sse, req, chemin, Reponse::Refuser).await
}

enum Reponse {
    Accepter,
    Refuser,
}

/// Logique commune accepter/refuser : verrouillage optimiste `propose` + `tour_id=moi`.
async fn repondre(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    reponse: Reponse,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;
    let autre = row.autre_id;
    verifier_relation(pool.get_ref(), moi, autre).await?;

    let (statut, type_notif, message, action) = match reponse {
        Reponse::Accepter => (
            "accepte",
            "rdv_accepte",
            "a accepté votre rendez-vous.",
            "ACCEPTER",
        ),
        Reponse::Refuser => (
            "refuse",
            "rdv_refuse",
            "a refusé votre rendez-vous.",
            "REFUSER",
        ),
    };

    let maj = sqlx::query(
        "UPDATE social.rendez_vous
         SET statut = $3::social.statut_rendez_vous, updated_at = NOW()
         WHERE id = $1 AND tour_id = $2 AND statut = 'propose' AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(moi)
    .bind(statut)
    .execute(pool.get_ref())
    .await?;

    if maj.rows_affected() == 0 {
        return Err(ApiErreur::Conflit(
            "Ce rendez-vous a déjà été modifié ou ce n'est pas à vous de répondre".to_string(),
        ));
    }

    let acteur = nom_acteur(pool.get_ref(), moi).await;
    creer_notification(
        pool.get_ref(),
        autre,
        type_notif,
        &format!("{acteur} {message}"),
        Some(LIEN_RDV),
    )
    .await;
    sse.publier(
        autre,
        &match reponse {
            Reponse::Accepter => evt_rdv_accepte(id),
            Reponse::Refuser => evt_rdv_refuse(id),
        },
    );

    auditer(
        pool.get_ref(),
        &req,
        moi,
        action,
        id,
        serde_json::json!({ "statut": statut }),
    )
    .await;

    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(moi, Utc::now())),
        error: None,
    }))
}

/// POST /api/rendez-vous/{id}/contre-proposer (FR-015..FR-018).
pub async fn contre_proposer(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<ContreProposerBody>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    valider_duree(body.duree_minutes)?;
    valider_date_future(body.date_heure)?;

    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;
    let autre = row.autre_id;
    verifier_relation(pool.get_ref(), moi, autre).await?;

    // Reste 'propose', met à jour le créneau, bascule le tour vers l'autre.
    // L'exigence `statut='propose'` interdit la contre-proposition d'un 'accepte' (FR-018 → 409).
    let maj = sqlx::query(
        "UPDATE social.rendez_vous
         SET date_heure = $3, duree_minutes = $4, tour_id = $5, updated_at = NOW()
         WHERE id = $1 AND tour_id = $2 AND statut = 'propose' AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(moi)
    .bind(body.date_heure)
    .bind(body.duree_minutes)
    .bind(autre)
    .execute(pool.get_ref())
    .await?;

    if maj.rows_affected() == 0 {
        return Err(ApiErreur::Conflit(
            "Ce rendez-vous ne peut plus être contre-proposé".to_string(),
        ));
    }

    let acteur = nom_acteur(pool.get_ref(), moi).await;
    creer_notification(
        pool.get_ref(),
        autre,
        "rdv_contre_propose",
        &format!("{acteur} a proposé un nouveau créneau."),
        Some(LIEN_RDV),
    )
    .await;
    sse.publier(autre, &evt_rdv_contre_propose(id));

    auditer(
        pool.get_ref(),
        &req,
        moi,
        "CONTRE_PROPOSER",
        id,
        serde_json::json!({ "statut": "propose", "tour_id": autre, "date_heure": body.date_heure }),
    )
    .await;

    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(moi, Utc::now())),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US3 : Annuler
// ════════════════════════════════════════════════════════════════

/// POST /api/rendez-vous/{id}/annuler (FR-022).
pub async fn annuler(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;
    let autre = row.autre_id;
    verifier_relation(pool.get_ref(), moi, autre).await?;

    // Autorisé à l'un OU l'autre participant tant que propose/accepte.
    let maj = sqlx::query(
        "UPDATE social.rendez_vous
         SET statut = 'annule', updated_at = NOW()
         WHERE id = $1 AND (initiateur_id = $2 OR destinataire_id = $2)
           AND statut IN ('propose', 'accepte') AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(moi)
    .execute(pool.get_ref())
    .await?;

    if maj.rows_affected() == 0 {
        return Err(ApiErreur::Conflit(
            "Ce rendez-vous ne peut plus être annulé".to_string(),
        ));
    }

    let acteur = nom_acteur(pool.get_ref(), moi).await;
    creer_notification(
        pool.get_ref(),
        autre,
        "rdv_annule",
        &format!("{acteur} a annulé un rendez-vous."),
        Some(LIEN_RDV),
    )
    .await;
    sse.publier(autre, &evt_rdv_annule(id));

    auditer(
        pool.get_ref(),
        &req,
        moi,
        "ANNULER",
        id,
        serde_json::json!({ "statut": "annule" }),
    )
    .await;

    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(row.to_response(moi, Utc::now())),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US4 : Salle visioconférence P2P
// ════════════════════════════════════════════════════════════════

/// GET /api/rendez-vous/{id}/salle : Configuration P2P (FR-024).
/// Seulement si `accepte`, participant, amis/non bloqués, et NOW() dans la fenêtre.
pub async fn salle(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    let row = charger_avec_autre(pool.get_ref(), moi, id).await?;
    let autre = row.autre_id;
    verifier_relation(pool.get_ref(), moi, autre).await?;

    if row.statut != "accepte" {
        return Err(ApiErreur::Conflit(
            "La salle n'est disponible que pour un rendez-vous accepté".to_string(),
        ));
    }
    if !peut_rejoindre(&row.statut, row.date_heure, row.duree_minutes, Utc::now()) {
        return Err(ApiErreur::Conflit(
            "La salle n'est pas encore ouverte ou le rendez-vous est terminé".to_string(),
        ));
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SalleResponse {
            rendez_vous_id: id,
            mon_peer_id: peer_id(id, moi),
            pair_peer_id: peer_id(id, autre),
            // Anti-glare : le plus petit UUID initie l'appel (research §7).
            suis_appelant: moi < autre,
            autre: row.autre(),
        }),
        error: None,
    }))
}
