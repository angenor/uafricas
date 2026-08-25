// Handlers du domaine social : demandes d'amitié, amitiés, notifications.
// User Stories 1 (envoyer) et 2 (répondre). Les blocages et la gestion fine
// (annuler, retirer, bloquer) relèvent de la US4 (Phase 6).

use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::amitie::{
    creer_amitie, creer_notification, evt_demande_acceptee, evt_demande_recue, obtenir_membre_light,
    paire_canonique, AmiResponse, BlocageResponse, BloquerBody, CreerDemandeBody,
    DemandeAvecMembreRow, DemandeEnvoyeeResponse, DemandeRecueResponse, DemandeResponse,
    EtatRelationResponse, EtatsLotBody, MembreAvecDateRow, NotificationResponse, NotificationRow,
};
use crate::services::audit;
use crate::services::messagerie_sse::RegistreSse;
use crate::ApiResponse;

// FR-014 : nombre maximum de demandes envoyées par 24 h glissantes (Décision 6)
const MAX_DEMANDES_24H: i64 = 30;
// Garde-fou sur la taille du lot d'états relationnels (FR-016)
const MAX_IDS_LOT: usize = 50;

/// Extraire l'utilisateur connecté depuis le header Authorization.
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

// ════════════════════════════════════════════════════════════════
// Helpers de relation
// ════════════════════════════════════════════════════════════════

/// Vérifie l'existence d'un blocage dans un sens ou l'autre.
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

/// Vérifie si une amitié existe entre a et b (ordre canonique).
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

/// Calcule l'état relationnel de `moi` vis-à-vis de chaque id (set-based, anti N+1).
/// Renvoie une map id -> (etat, demande_id éventuelle).
async fn calculer_etats(
    pool: &PgPool,
    moi: Uuid,
    ids: &[Uuid],
) -> Result<HashMap<Uuid, (String, Option<Uuid>)>, ApiErreur> {
    let mut resultat: HashMap<Uuid, (String, Option<Uuid>)> = HashMap::new();
    if ids.is_empty() {
        return Ok(resultat);
    }

    // 1. Membres actifs parmi la liste
    let actifs: Vec<Uuid> = sqlx::query_scalar(
        "SELECT id FROM iam.utilisateur
         WHERE id = ANY($1) AND etat = 'actif' AND deleted_at IS NULL",
    )
    .bind(ids)
    .fetch_all(pool)
    .await?;
    let actifs: std::collections::HashSet<Uuid> = actifs.into_iter().collect();

    // 2. Amitiés entre moi et la liste
    let amities: Vec<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT utilisateur_a_id, utilisateur_b_id FROM social.amitie
         WHERE (utilisateur_a_id = $1 AND utilisateur_b_id = ANY($2))
            OR (utilisateur_b_id = $1 AND utilisateur_a_id = ANY($2))",
    )
    .bind(moi)
    .bind(ids)
    .fetch_all(pool)
    .await?;
    let mut amis: std::collections::HashSet<Uuid> = std::collections::HashSet::new();
    for (a, b) in amities {
        amis.insert(if a == moi { b } else { a });
    }

    // 3. Blocages entre moi et la liste (avec orientation)
    let blocages: Vec<(Uuid, Uuid)> = sqlx::query_as(
        "SELECT bloqueur_id, bloque_id FROM social.blocage
         WHERE (bloqueur_id = $1 AND bloque_id = ANY($2))
            OR (bloque_id = $1 AND bloqueur_id = ANY($2))",
    )
    .bind(moi)
    .bind(ids)
    .fetch_all(pool)
    .await?;
    let mut bloque_par_moi: std::collections::HashSet<Uuid> = std::collections::HashSet::new();
    let mut me_bloque: std::collections::HashSet<Uuid> = std::collections::HashSet::new();
    for (bloqueur, bloque) in blocages {
        if bloqueur == moi {
            bloque_par_moi.insert(bloque);
        } else {
            me_bloque.insert(bloqueur);
        }
    }

    // 4. Demandes en attente entre moi et la liste (avec sens + id)
    let demandes: Vec<(Uuid, Uuid, Uuid)> = sqlx::query_as(
        "SELECT id, demandeur_id, destinataire_id FROM social.demande_amitie
         WHERE statut = 'en_attente' AND deleted_at IS NULL
           AND ((demandeur_id = $1 AND destinataire_id = ANY($2))
             OR (destinataire_id = $1 AND demandeur_id = ANY($2)))",
    )
    .bind(moi)
    .bind(ids)
    .fetch_all(pool)
    .await?;
    let mut envoyee: HashMap<Uuid, Uuid> = HashMap::new(); // autre -> demande_id
    let mut recue: HashMap<Uuid, Uuid> = HashMap::new();
    for (demande_id, demandeur, destinataire) in demandes {
        if demandeur == moi {
            envoyee.insert(destinataire, demande_id);
        } else {
            recue.insert(demandeur, demande_id);
        }
    }

    // 5. Composition par priorité
    for &id in ids {
        if id == moi {
            resultat.insert(id, ("aucune".to_string(), None));
            continue;
        }
        if !actifs.contains(&id) {
            resultat.insert(id, ("indisponible".to_string(), None));
            continue;
        }
        if bloque_par_moi.contains(&id) {
            resultat.insert(id, ("bloque_par_moi".to_string(), None));
            continue;
        }
        if me_bloque.contains(&id) {
            // Du point de vue de moi, le membre est inatteignable.
            resultat.insert(id, ("indisponible".to_string(), None));
            continue;
        }
        if amis.contains(&id) {
            resultat.insert(id, ("amis".to_string(), None));
            continue;
        }
        if let Some(&did) = envoyee.get(&id) {
            resultat.insert(id, ("demande_envoyee".to_string(), Some(did)));
            continue;
        }
        if let Some(&did) = recue.get(&id) {
            resultat.insert(id, ("demande_recue".to_string(), Some(did)));
            continue;
        }
        resultat.insert(id, ("aucune".to_string(), None));
    }

    Ok(resultat)
}

// Push SSE : nouvelle demande reçue (enrichie du demandeur).
async fn pousser_demande_recue(sse: &RegistreSse, pool: &PgPool, dest: Uuid, demande_id: Uuid, demandeur_id: Uuid) {
    if let Ok(Some(m)) = obtenir_membre_light(pool, demandeur_id).await {
        sse.publier(dest, &evt_demande_recue(demande_id, &m));
    }
}

// Push SSE : demande acceptée (enrichie de celui qui a accepté).
async fn pousser_demande_acceptee(sse: &RegistreSse, pool: &PgPool, dest: Uuid, acteur_id: Uuid) {
    if let Ok(Some(m)) = obtenir_membre_light(pool, acteur_id).await {
        sse.publier(dest, &evt_demande_acceptee(&m));
    }
}

// ════════════════════════════════════════════════════════════════
// US1 : Envoyer une demande / consulter l'état
// ════════════════════════════════════════════════════════════════

/// POST /api/amities/demandes : Envoyer une demande d'amitié (FR-001).
pub async fn creer_demande(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    body: web::Json<CreerDemandeBody>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let cible = body.destinataire_id;

    // FR-002 : pas soi-même
    if cible == moi {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas vous envoyer une demande à vous-même".to_string(),
        ));
    }

    // FR-015 : destinataire actif et non supprimé
    let cible_active: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM iam.utilisateur
            WHERE id = $1 AND etat = 'actif' AND deleted_at IS NULL
        )",
    )
    .bind(cible)
    .fetch_one(pool.get_ref())
    .await?;
    if !cible_active {
        return Err(ApiErreur::Validation(
            "Ce membre n'est pas disponible".to_string(),
        ));
    }

    // FR-013 : blocage dans un sens ou l'autre
    if blocage_existe(pool.get_ref(), moi, cible).await? {
        return Err(ApiErreur::AccesInterdit(
            "Une demande n'est pas possible avec ce membre".to_string(),
        ));
    }

    // FR-003 : amitié déjà existante
    if amitie_existe(pool.get_ref(), moi, cible).await? {
        return Err(ApiErreur::Conflit(
            "Vous êtes déjà amis avec ce membre".to_string(),
        ));
    }

    // FR-003 : demande déjà en attente dans mon sens
    let deja_envoyee: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.demande_amitie
            WHERE demandeur_id = $1 AND destinataire_id = $2
              AND statut = 'en_attente' AND deleted_at IS NULL
        )",
    )
    .bind(moi)
    .bind(cible)
    .fetch_one(pool.get_ref())
    .await?;
    if deja_envoyee {
        return Err(ApiErreur::Conflit(
            "Une demande est déjà en attente pour ce membre".to_string(),
        ));
    }

    // FR-009 / R3 : demande inverse en attente → auto-acceptation croisée
    let demande_inverse: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM social.demande_amitie
         WHERE demandeur_id = $1 AND destinataire_id = $2
           AND statut = 'en_attente' AND deleted_at IS NULL",
    )
    .bind(cible)
    .bind(moi)
    .fetch_optional(pool.get_ref())
    .await?;

    if let Some(demande_id) = demande_inverse {
        let mut tx = pool.begin().await?;
        sqlx::query(
            "UPDATE social.demande_amitie
             SET statut = 'acceptee', traite_at = NOW()
             WHERE id = $1",
        )
        .bind(demande_id)
        .execute(&mut *tx)
        .await?;
        creer_amitie(&mut *tx, moi, cible).await?;
        // L'émetteur de la demande inverse (la cible) est notifié de l'acceptation.
        creer_notification(&mut *tx, cible, "demande_acceptee", Some(demande_id), Some(moi))
            .await?;
        tx.commit().await?;

        // Temps réel : la cible (émettrice de la demande inverse) voit l'acceptation.
        pousser_demande_acceptee(&sse, pool.get_ref(), cible, moi).await;

        let ip = audit::extraire_ip(&req);
        let ua = audit::extraire_user_agent(&req);
        audit::log_action(
            pool.get_ref(),
            Some(moi),
            "UPDATE",
            "social",
            "demande_amitie",
            Some(demande_id),
            None,
            Some(serde_json::json!({ "statut": "acceptee", "auto_acceptation": true })),
            ip.as_deref(),
            ua.as_deref(),
        )
        .await;

        return Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(DemandeResponse {
                demande_id,
                statut: "amis".to_string(),
            }),
            error: None,
        }));
    }

    // FR-014 : rate-limit 30 / 24 h glissantes
    let nb_recentes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM social.demande_amitie
         WHERE demandeur_id = $1 AND created_at > NOW() - INTERVAL '24 hours'",
    )
    .bind(moi)
    .fetch_one(pool.get_ref())
    .await?;
    if nb_recentes >= MAX_DEMANDES_24H {
        return Err(ApiErreur::LimiteAtteinte(
            "Vous avez atteint la limite de demandes d'amitié sur 24 h".to_string(),
        ));
    }

    // Insertion de la demande + notification, en transaction
    let mut tx = pool.begin().await?;
    let demande_id: Uuid = sqlx::query_scalar(
        "INSERT INTO social.demande_amitie (demandeur_id, destinataire_id)
         VALUES ($1, $2) RETURNING id",
    )
    .bind(moi)
    .bind(cible)
    .fetch_one(&mut *tx)
    .await?;
    creer_notification(&mut *tx, cible, "demande_recue", Some(demande_id), Some(moi)).await?;
    tx.commit().await?;

    // Temps réel : le destinataire reçoit la demande (FR-005).
    pousser_demande_recue(&sse, pool.get_ref(), cible, demande_id, moi).await;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "social",
        "demande_amitie",
        Some(demande_id),
        None,
        Some(serde_json::json!({ "destinataire_id": cible, "statut": "en_attente" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(DemandeResponse {
            demande_id,
            statut: "en_attente".to_string(),
        }),
        error: None,
    }))
}

/// GET /api/amities/etat/{utilisateur_id}, État de la relation (FR-016).
pub async fn etat_relation(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let cible = chemin.into_inner();

    let etats = calculer_etats(pool.get_ref(), moi, &[cible]).await?;
    let (etat, demande_id) = etats
        .get(&cible)
        .cloned()
        .unwrap_or(("aucune".to_string(), None));

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(EtatRelationResponse { etat, demande_id }),
        error: None,
    }))
}

/// POST /api/amities/etats : États relationnels en lot pour l'annuaire (FR-016).
pub async fn etats_relation_lot(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<EtatsLotBody>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;

    if body.utilisateur_ids.len() > MAX_IDS_LOT {
        return Err(ApiErreur::Validation(format!(
            "Trop d'identifiants (max {})",
            MAX_IDS_LOT
        )));
    }

    let etats = calculer_etats(pool.get_ref(), moi, &body.utilisateur_ids).await?;
    let map: HashMap<String, String> = etats
        .into_iter()
        .map(|(id, (etat, _))| (id.to_string(), etat))
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(map),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US2 : Répondre à une demande / lister / notifications
// ════════════════════════════════════════════════════════════════

#[derive(sqlx::FromRow)]
struct DemandePourDecision {
    demandeur_id: Uuid,
    destinataire_id: Uuid,
    statut: String,
}

/// Charge une demande et garde 404 si introuvable.
async fn charger_demande(pool: &PgPool, demande_id: Uuid) -> Result<DemandePourDecision, ApiErreur> {
    sqlx::query_as::<_, DemandePourDecision>(
        "SELECT demandeur_id, destinataire_id, statut::text AS statut
         FROM social.demande_amitie
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(demande_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Demande introuvable".to_string()))
}

/// POST /api/amities/demandes/{id}/accepter, Accepter (FR-007).
pub async fn accepter_demande(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let demande_id = chemin.into_inner();

    let demande = charger_demande(pool.get_ref(), demande_id).await?;
    if demande.destinataire_id != moi {
        return Err(ApiErreur::AccesInterdit(
            "Vous n'êtes pas le destinataire de cette demande".to_string(),
        ));
    }
    if demande.statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette demande a déjà été traitée".to_string(),
        ));
    }

    let mut tx = pool.begin().await?;
    sqlx::query(
        "UPDATE social.demande_amitie SET statut = 'acceptee', traite_at = NOW() WHERE id = $1",
    )
    .bind(demande_id)
    .execute(&mut *tx)
    .await?;
    creer_amitie(&mut *tx, demande.demandeur_id, demande.destinataire_id).await?;
    // Notification à l'émetteur de la demande.
    creer_notification(
        &mut *tx,
        demande.demandeur_id,
        "demande_acceptee",
        Some(demande_id),
        Some(moi),
    )
    .await?;
    tx.commit().await?;

    // Temps réel : l'émetteur voit que sa demande est acceptée (FR-007).
    pousser_demande_acceptee(&sse, pool.get_ref(), demande.demandeur_id, moi).await;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "social",
        "demande_amitie",
        Some(demande_id),
        None,
        Some(serde_json::json!({ "statut": "acceptee" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "statut": "amis" })),
        error: None,
    }))
}

/// POST /api/amities/demandes/{id}/refuser, Refuser (FR-008, pas de notification).
pub async fn refuser_demande(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let demande_id = chemin.into_inner();

    let demande = charger_demande(pool.get_ref(), demande_id).await?;
    if demande.destinataire_id != moi {
        return Err(ApiErreur::AccesInterdit(
            "Vous n'êtes pas le destinataire de cette demande".to_string(),
        ));
    }
    if demande.statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette demande a déjà été traitée".to_string(),
        ));
    }

    sqlx::query(
        "UPDATE social.demande_amitie SET statut = 'refusee', traite_at = NOW() WHERE id = $1",
    )
    .bind(demande_id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "social",
        "demande_amitie",
        Some(demande_id),
        None,
        Some(serde_json::json!({ "statut": "refusee" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "statut": "refusee" })),
        error: None,
    }))
}

#[derive(Debug, serde::Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}

/// GET /api/amities/demandes/recues : Demandes en attente reçues (US2).
pub async fn lister_demandes_recues(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let rows = sqlx::query_as::<_, DemandeAvecMembreRow>(
        "SELECT d.id AS demande_id, d.created_at,
                u.id AS membre_id, u.nom, u.prenom, u.slug, u.photo_url, u.fonction, p.nom AS pays
         FROM social.demande_amitie d
         JOIN iam.utilisateur u ON u.id = d.demandeur_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE d.destinataire_id = $1 AND d.statut = 'en_attente' AND d.deleted_at IS NULL
         ORDER BY d.created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(moi)
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let demandes: Vec<DemandeRecueResponse> = rows
        .iter()
        .map(|r| DemandeRecueResponse {
            demande_id: r.demande_id,
            demandeur: r.membre(),
            created_at: r.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(demandes),
        error: None,
    }))
}

/// GET /api/amities/demandes/envoyees : Demandes en attente envoyées (US4 anticipé).
pub async fn lister_demandes_envoyees(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let rows = sqlx::query_as::<_, DemandeAvecMembreRow>(
        "SELECT d.id AS demande_id, d.created_at,
                u.id AS membre_id, u.nom, u.prenom, u.slug, u.photo_url, u.fonction, p.nom AS pays
         FROM social.demande_amitie d
         JOIN iam.utilisateur u ON u.id = d.destinataire_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE d.demandeur_id = $1 AND d.statut = 'en_attente' AND d.deleted_at IS NULL
         ORDER BY d.created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(moi)
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let demandes: Vec<DemandeEnvoyeeResponse> = rows
        .iter()
        .map(|r| DemandeEnvoyeeResponse {
            demande_id: r.demande_id,
            destinataire: r.membre(),
            created_at: r.created_at,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(demandes),
        error: None,
    }))
}

// ── Notifications relationnelles (FR-017) ────────────────────────

/// GET /api/amities/notifications : Notifications sociales paginées.
pub async fn lister_notifications(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;

    let rows = sqlx::query_as::<_, NotificationRow>(
        "SELECT n.id, n.type::text AS type_notif, n.demande_id, n.lu, n.created_at,
                a.id AS acteur_id, a.nom AS acteur_nom, a.prenom AS acteur_prenom,
                a.slug AS acteur_slug, a.photo_url AS acteur_photo_url,
                a.fonction AS acteur_fonction, p.nom AS acteur_pays
         FROM social.notification n
         LEFT JOIN iam.utilisateur a ON a.id = n.acteur_id
         LEFT JOIN shared.pays p ON p.id = a.pays_residence_id
         WHERE n.destinataire_id = $1
         ORDER BY n.created_at DESC
         LIMIT $2 OFFSET $3",
    )
    .bind(moi)
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let notifications: Vec<NotificationResponse> = rows.iter().map(|r| r.to_response()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(notifications),
        error: None,
    }))
}

/// PATCH /api/amities/notifications/{id}/lu, Marquer une notification lue.
pub async fn marquer_notification_lue(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let notif_id = chemin.into_inner();

    sqlx::query(
        "UPDATE social.notification SET lu = TRUE WHERE id = $1 AND destinataire_id = $2",
    )
    .bind(notif_id)
    .bind(moi)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "lu": true })),
        error: None,
    }))
}

/// PATCH /api/amities/notifications/tout-lu, Tout marquer lu.
pub async fn marquer_tout_lu(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;

    sqlx::query(
        "UPDATE social.notification SET lu = TRUE WHERE destinataire_id = $1 AND lu = FALSE",
    )
    .bind(moi)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "lu": true })),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US4 : Gérer ses relations & blocage
// ════════════════════════════════════════════════════════════════

/// DELETE /api/amities/demandes/{id} : Annuler une demande émise en attente (FR-010).
pub async fn annuler_demande(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let demande_id = chemin.into_inner();

    let demande = charger_demande(pool.get_ref(), demande_id).await?;
    // FR-010 : seul l'émetteur peut annuler sa propre demande.
    if demande.demandeur_id != moi {
        return Err(ApiErreur::AccesInterdit(
            "Vous n'êtes pas l'émetteur de cette demande".to_string(),
        ));
    }
    if demande.statut != "en_attente" {
        return Err(ApiErreur::Conflit(
            "Cette demande a déjà été traitée".to_string(),
        ));
    }

    sqlx::query(
        "UPDATE social.demande_amitie SET statut = 'annulee', traite_at = NOW() WHERE id = $1",
    )
    .bind(demande_id)
    .execute(pool.get_ref())
    .await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "UPDATE",
        "social",
        "demande_amitie",
        Some(demande_id),
        None,
        Some(serde_json::json!({ "statut": "annulee" })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "statut": "annulee" })),
        error: None,
    }))
}

#[derive(Debug, serde::Deserialize)]
pub struct ListeAmisParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub recherche: Option<String>,
}

/// GET /api/amities : Liste des amis de l'utilisateur courant (FR-011, FR-026, privée).
pub async fn lister_amis(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    params: web::Query<ListeAmisParams>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let page = params.page.unwrap_or(1).max(1);
    let par_page = params.par_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * par_page;
    // Motif ILIKE optionnel (recherche par nom/prénom).
    let motif = params
        .recherche
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| format!("%{s}%"));

    let rows = sqlx::query_as::<_, MembreAvecDateRow>(
        "SELECT u.id AS membre_id, u.nom, u.prenom, u.slug, u.photo_url, u.fonction,
                p.nom AS pays, a.created_at AS date_relation
         FROM social.amitie a
         JOIN iam.utilisateur u
            ON u.id = CASE WHEN a.utilisateur_a_id = $1 THEN a.utilisateur_b_id ELSE a.utilisateur_a_id END
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE (a.utilisateur_a_id = $1 OR a.utilisateur_b_id = $1)
           AND u.deleted_at IS NULL
           AND ($2::text IS NULL OR u.nom ILIKE $2 OR u.prenom ILIKE $2)
         ORDER BY a.created_at DESC
         LIMIT $3 OFFSET $4",
    )
    .bind(moi)
    .bind(motif.as_deref())
    .bind(par_page)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await?;

    let amis: Vec<AmiResponse> = rows
        .iter()
        .map(|r| AmiResponse {
            utilisateur: r.membre(),
            ami_depuis: r.date_relation,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(amis),
        error: None,
    }))
}

/// DELETE /api/amities/{utilisateur_id}, Retirer un ami (FR-012).
/// La conversation est conservée mais devient verrouillée (FR-025, verrouillage
/// implicite : `lister_conversations` la marque `verrouillee` dès que l'amitié n'existe plus).
pub async fn retirer_ami(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let cible = chemin.into_inner();
    let (min, max) = paire_canonique(moi, cible);

    let resultat = sqlx::query(
        "DELETE FROM social.amitie WHERE utilisateur_a_id = $1 AND utilisateur_b_id = $2",
    )
    .bind(min)
    .bind(max)
    .execute(pool.get_ref())
    .await?;

    if resultat.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Vous n'êtes pas amis avec ce membre".to_string(),
        ));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "DELETE",
        "social",
        "amitie",
        None,
        None,
        Some(serde_json::json!({ "utilisateur_id": cible })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "retire": true })),
        error: None,
    }))
}

// ── Blocages (FR-013) ─────────────────────────────────────────────

/// POST /api/blocages : Bloquer un membre (FR-013, R4).
/// Transaction : crée le blocage, supprime l'amitié, annule les demandes actives
/// entre les deux membres ; la conversation est de fait verrouillée.
pub async fn bloquer(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<BloquerBody>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let cible = body.utilisateur_id;

    if cible == moi {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas vous bloquer vous-même".to_string(),
        ));
    }

    // Idempotence métier : refus si déjà bloqué.
    let deja_bloque: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.blocage WHERE bloqueur_id = $1 AND bloque_id = $2
        )",
    )
    .bind(moi)
    .bind(cible)
    .fetch_one(pool.get_ref())
    .await?;
    if deja_bloque {
        return Err(ApiErreur::Conflit(
            "Ce membre est déjà bloqué".to_string(),
        ));
    }

    let (min, max) = paire_canonique(moi, cible);
    let mut tx = pool.begin().await?;
    sqlx::query("INSERT INTO social.blocage (bloqueur_id, bloque_id) VALUES ($1, $2)")
        .bind(moi)
        .bind(cible)
        .execute(&mut *tx)
        .await?;
    // Rupture de l'amitié (R4).
    sqlx::query("DELETE FROM social.amitie WHERE utilisateur_a_id = $1 AND utilisateur_b_id = $2")
        .bind(min)
        .bind(max)
        .execute(&mut *tx)
        .await?;
    // Annulation des demandes actives dans les deux sens (R4).
    sqlx::query(
        "UPDATE social.demande_amitie SET statut = 'annulee', traite_at = NOW()
         WHERE statut = 'en_attente' AND deleted_at IS NULL
           AND ((demandeur_id = $1 AND destinataire_id = $2)
             OR (demandeur_id = $2 AND destinataire_id = $1))",
    )
    .bind(moi)
    .bind(cible)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "social",
        "blocage",
        None,
        None,
        Some(serde_json::json!({ "bloque_id": cible })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "bloque": true })),
        error: None,
    }))
}

/// DELETE /api/blocages/{utilisateur_id}, Débloquer un membre (FR-013).
pub async fn debloquer(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let cible = chemin.into_inner();

    let resultat =
        sqlx::query("DELETE FROM social.blocage WHERE bloqueur_id = $1 AND bloque_id = $2")
            .bind(moi)
            .bind(cible)
            .execute(pool.get_ref())
            .await?;

    if resultat.rows_affected() == 0 {
        return Err(ApiErreur::NonTrouve(
            "Ce membre n'est pas bloqué".to_string(),
        ));
    }

    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "DELETE",
        "social",
        "blocage",
        None,
        None,
        Some(serde_json::json!({ "bloque_id": cible })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "debloque": true })),
        error: None,
    }))
}

/// GET /api/blocages : Liste des membres bloqués par l'utilisateur courant (FR-013).
pub async fn lister_blocages(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;

    let rows = sqlx::query_as::<_, MembreAvecDateRow>(
        "SELECT u.id AS membre_id, u.nom, u.prenom, u.slug, u.photo_url, u.fonction,
                p.nom AS pays, b.created_at AS date_relation
         FROM social.blocage b
         JOIN iam.utilisateur u ON u.id = b.bloque_id
         LEFT JOIN shared.pays p ON p.id = u.pays_residence_id
         WHERE b.bloqueur_id = $1
         ORDER BY b.created_at DESC",
    )
    .bind(moi)
    .fetch_all(pool.get_ref())
    .await?;

    let blocages: Vec<BlocageResponse> = rows
        .iter()
        .map(|r| BlocageResponse {
            utilisateur: r.membre(),
            depuis: r.date_relation,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(blocages),
        error: None,
    }))
}
