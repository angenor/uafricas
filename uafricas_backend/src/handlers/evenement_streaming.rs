// Handlers : Événements en streaming direct (feature 001-evenements-streaming).
//
// Modèle webinaire : l'organisateur (= evenement.cree_par) ouvre une session de
// direct, diffuse caméra/micro/écran ; les inscrits regardent (token scopé
// `can_publish:false`) et interagissent en DataPackets LiveKit (chat / réactions /
// lever-la-main, éphémères). Réutilisation maximale (Principe V) : infra LiveKit +
// `livekit_moderation`, auth JWT in-handler, `est_inscrit`, notifications cloche +
// SSE messagerie, audit. Aucun média n'est stocké (flux via SFU LiveKit).
//
// États session persistés : `en_cours`, `terminee`. « en attente » et `statut_direct`
// sont dérivés à la lecture. Pas de cron : l'arrêt de sécurité (D6) et la cascade
// d'annulation (FR-016) sont appliqués paresseusement à chaque lecture/jointure.

use actix_web::{web, HttpRequest, HttpResponse};
use livekit_api::access_token::{AccessToken, VideoGrants};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::LivekitConfig;
use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::evenement_streaming::{
    calc_arret_securite_at, can_publish_pour_role, est_diffusable, fenetre_ouverture_at,
    statut_direct, DemandeParole, EtatDirectResponse, SessionRow, TokenDirectResponse,
    EVENEMENT_SESSION_COLONNES, MAX_PARTICIPANTS_DEFAUT, ROLE_INTERVENANT, ROLE_ORGANISATEUR,
    ROLE_SPECTATEUR,
};
use crate::services::audit;
use crate::services::livekit_moderation;
use crate::services::messagerie_sse::RegistreSse;
use crate::ApiResponse;

// ════════════════════════════════════════════════════════════════
// Authentification & helpers partagés (T008)
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

/// `room_name` LiveKit déterministe à partir de l'id de session.
fn room_name(session_id: Uuid) -> String {
    format!("evenement-{}", session_id)
}

/// Informations de l'événement nécessaires au direct (FromRow ad hoc).
#[derive(Debug, sqlx::FromRow)]
struct EvenementDirectInfo {
    id: Uuid,
    format: String,
    etat: String,
    cree_par: Uuid,
    date_heure_debut: chrono::DateTime<chrono::Utc>,
    date_heure_fin: Option<chrono::DateTime<chrono::Utc>>,
    titre: String,
}

impl EvenementDirectInfo {
    fn est_annule(&self) -> bool {
        self.etat == "annule"
    }
}

/// Charge l'événement (404 si introuvable / supprimé). Ne valide PAS le format :
/// le `GET …/direct` doit pouvoir renvoyer `statut_direct = "indisponible"` pour un
/// présentiel ; la validation diffusable (422) se fait à l'ouverture.
async fn charger_evenement(pool: &PgPool, id: Uuid) -> Result<EvenementDirectInfo, ApiErreur> {
    sqlx::query_as::<_, EvenementDirectInfo>(
        "SELECT id, format::text AS format, etat, cree_par, date_heure_debut, date_heure_fin, titre
         FROM media_content.evenement
         WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Événement introuvable".to_string()))
}

/// Session de direct active (`en_cours`) d'un événement (unique via index partiel).
async fn charger_session_active(
    pool: &PgPool,
    evenement_id: Uuid,
) -> Result<Option<SessionRow>, ApiErreur> {
    let query = format!(
        "SELECT {EVENEMENT_SESSION_COLONNES} FROM media_content.evenement_session
         WHERE evenement_id = $1 AND etat = 'en_cours' LIMIT 1"
    );
    Ok(sqlx::query_as::<_, SessionRow>(&query)
        .bind(evenement_id)
        .fetch_optional(pool)
        .await?)
}

/// Session la plus récente (active ou terminée), pour dériver `statut_direct`.
async fn charger_derniere_session(
    pool: &PgPool,
    evenement_id: Uuid,
) -> Result<Option<SessionRow>, ApiErreur> {
    let query = format!(
        "SELECT {EVENEMENT_SESSION_COLONNES} FROM media_content.evenement_session
         WHERE evenement_id = $1 ORDER BY demarre_at DESC LIMIT 1"
    );
    Ok(sqlx::query_as::<_, SessionRow>(&query)
        .bind(evenement_id)
        .fetch_optional(pool)
        .await?)
}

async fn compter_participants_actifs(pool: &PgPool, session_id: Uuid) -> Result<i64, ApiErreur> {
    Ok(sqlx::query_scalar(
        "SELECT COUNT(*) FROM media_content.evenement_session_participant
         WHERE session_id = $1 AND quitte_at IS NULL",
    )
    .bind(session_id)
    .fetch_one(pool)
    .await?)
}

async fn est_inscrit(pool: &PgPool, evenement_id: Uuid, utilisateur_id: Uuid) -> Result<bool, ApiErreur> {
    Ok(sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM media_content.evenement_inscription
            WHERE evenement_id = $1 AND utilisateur_id = $2 AND statut != 'annule'
        )",
    )
    .bind(evenement_id)
    .bind(utilisateur_id)
    .fetch_one(pool)
    .await?)
}

/// « Prénom Nom » pour le nom affiché LiveKit / les libellés (parité afrolang).
async fn nom_utilisateur(pool: &PgPool, utilisateur_id: Uuid) -> String {
    let res: Option<(String, Option<String>)> =
        sqlx::query_as("SELECT nom, prenom FROM iam.utilisateur WHERE id = $1")
            .bind(utilisateur_id)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten();
    match res {
        Some((nom, prenom)) => format!("{} {}", prenom.as_deref().unwrap_or(""), nom)
            .trim()
            .to_string(),
        None => "Un membre".to_string(),
    }
}

/// Clôture paresseuse (D6 / FR-016) : si la session est `en_cours` et que l'arrêt de
/// sécurité est dépassé OU l'événement annulé, passe `terminee` + best-effort
/// `fermer_session_admin`. Retourne `true` si une clôture a eu lieu.
async fn cloturer_si_necessaire(
    pool: &PgPool,
    cfg: &LivekitConfig,
    session: &SessionRow,
    evenement_annule: bool,
) -> Result<bool, ApiErreur> {
    if session.etat != "en_cours" {
        return Ok(false);
    }
    let maintenant = chrono::Utc::now();
    if maintenant <= session.arret_securite_at && !evenement_annule {
        return Ok(false);
    }
    sqlx::query(
        "UPDATE media_content.evenement_session
         SET etat = 'terminee', termine_at = NOW(),
             duree_secondes = EXTRACT(EPOCH FROM (NOW() - demarre_at))::int, updated_at = NOW()
         WHERE id = $1 AND etat = 'en_cours'",
    )
    .bind(session.id)
    .execute(pool)
    .await?;
    let motif = if evenement_annule {
        "L'événement a été annulé."
    } else {
        "Le direct est terminé."
    };
    let _ = livekit_moderation::fermer_session_admin(cfg, &room_name(session.id), motif).await;
    Ok(true)
}

/// Force la clôture de la session active d'un événement (cascade d'annulation, FR-016).
/// Appelée par l'admin quand un événement passe à `annule` : déconnecte immédiatement
/// les participants (best-effort LiveKit) au lieu d'attendre la clôture paresseuse à la
/// prochaine lecture. No-op s'il n'y a aucune session active.
pub async fn forcer_cloture_session(
    pool: &PgPool,
    cfg: &LivekitConfig,
    evenement_id: Uuid,
) -> Result<(), ApiErreur> {
    if let Some(session) = charger_session_active(pool, evenement_id).await? {
        cloturer_si_necessaire(pool, cfg, &session, true).await?;
    }
    Ok(())
}

/// Génère le token LiveKit scopé selon le rôle (D2). Spectateur : `can_publish:false`.
fn generer_token(
    cfg: &LivekitConfig,
    salle: &str,
    utilisateur_id: Uuid,
    nom: &str,
    role: &str,
) -> Result<String, ApiErreur> {
    AccessToken::with_api_key(&cfg.api_key, &cfg.api_secret)
        .with_identity(&utilisateur_id.to_string())
        .with_name(nom)
        .with_grants(VideoGrants {
            room_join: true,
            room: salle.to_string(),
            can_publish: can_publish_pour_role(role),
            can_subscribe: true,
            can_publish_data: true,
            ..Default::default()
        })
        .to_jwt()
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Génération token LiveKit : {}", e)))
}

/// Journalise une mutation de session sans contenu de chat ni média (Principe VII).
async fn auditer(
    pool: &PgPool,
    req: &HttpRequest,
    moi: Uuid,
    action: &str,
    session_id: Uuid,
    nouvel_etat: serde_json::Value,
) {
    let ip = audit::extraire_ip(req);
    let ua = audit::extraire_user_agent(req);
    audit::log_action(
        pool,
        Some(moi),
        action,
        "media_content",
        "evenement_session",
        Some(session_id),
        None,
        Some(nouvel_etat),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;
}

/// Notifie chaque inscrit de l'ouverture du direct : cloche persistante + SSE (D9).
async fn notifier_ouverture(pool: &PgPool, sse: &RegistreSse, evenement_id: Uuid, titre: &str) {
    let inscrits: Vec<Uuid> = sqlx::query_scalar(
        "SELECT utilisateur_id FROM media_content.evenement_inscription
         WHERE evenement_id = $1 AND statut != 'annule'",
    )
    .bind(evenement_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let lien = format!("/evenements/{}", evenement_id);
    let message = format!("Le direct de « {} » a commencé", titre);
    let evt = json!({ "type": "event_stream_demarre", "evenement_id": evenement_id });
    for inscrit_id in inscrits {
        crate::models::notification::creer_notification(
            pool,
            inscrit_id,
            "evenement_direct_demarre",
            &message,
            Some(&lien),
        )
        .await;
        sse.publier(inscrit_id, &evt);
    }
}

// ════════════════════════════════════════════════════════════════
// US1 (foundational) : État du direct (T009)
// ════════════════════════════════════════════════════════════════

/// GET /api/evenements/{id}/direct : état dérivé pour l'appelant (JWT optionnel).
pub async fn etat_direct(
    pool: web::Data<PgPool>,
    cfg: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();
    let moi = extraire_utilisateur_id(&req);
    let evt = charger_evenement(pool.get_ref(), id).await?;

    // Clôture paresseuse de l'arrêt de sécurité / cascade annulation (FR-016, D6).
    let mut session = charger_derniere_session(pool.get_ref(), id).await?;
    if let Some(s) = &session {
        if cloturer_si_necessaire(pool.get_ref(), cfg.get_ref(), s, evt.est_annule()).await? {
            session = charger_derniere_session(pool.get_ref(), id).await?;
        }
    }

    let maintenant = chrono::Utc::now();
    let diffusable = est_diffusable(&evt.format);
    let statut = statut_direct(
        diffusable,
        evt.est_annule(),
        session.as_ref(),
        evt.date_heure_debut,
        maintenant,
    );

    let est_organisateur = moi == Some(evt.cree_par);
    let est_inscrit_flag = match moi {
        Some(uid) => est_inscrit(pool.get_ref(), id, uid).await?,
        None => false,
    };

    // Session active (en_direct) → compteur + capacité + demandes de parole.
    let session_active = session.as_ref().filter(|s| s.etat == "en_cours" && statut == "en_direct");
    let session_id = session_active.map(|s| s.id);
    let max_participants = session_active
        .map(|s| s.max_participants)
        .unwrap_or(MAX_PARTICIPANTS_DEFAUT);
    let nombre_participants = match session_id {
        Some(sid) => compter_participants_actifs(pool.get_ref(), sid).await?,
        None => 0,
    };

    let peut_ouvrir = est_organisateur
        && diffusable
        && evt.etat == "publie"
        && session.is_none()
        && maintenant >= fenetre_ouverture_at(evt.date_heure_debut);

    let peut_rejoindre = statut == "en_direct"
        && (est_organisateur || est_inscrit_flag)
        && (est_organisateur || nombre_participants < max_participants as i64);

    // Demandes de parole : vue organisateur uniquement (FR-022).
    let demandes_parole = if est_organisateur {
        if let Some(sid) = session_id {
            charger_demandes_parole(pool.get_ref(), sid).await?
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(EtatDirectResponse {
            statut_direct: statut.to_string(),
            peut_ouvrir,
            peut_rejoindre,
            est_organisateur,
            est_inscrit: est_inscrit_flag,
            session_id,
            nombre_participants,
            max_participants,
            fenetre_ouverture_at: fenetre_ouverture_at(evt.date_heure_debut),
            demandes_parole,
        }),
        error: None,
    }))
}

/// Liste des spectateurs ayant la main levée (ordre chronologique).
async fn charger_demandes_parole(
    pool: &PgPool,
    session_id: Uuid,
) -> Result<Vec<DemandeParole>, ApiErreur> {
    let rows: Vec<(Uuid, String, Option<String>, Option<chrono::DateTime<chrono::Utc>>)> =
        sqlx::query_as(
            "SELECT p.utilisateur_id, u.nom, u.prenom, p.main_levee_at
             FROM media_content.evenement_session_participant p
             JOIN iam.utilisateur u ON u.id = p.utilisateur_id
             WHERE p.session_id = $1 AND p.main_levee = TRUE AND p.quitte_at IS NULL
             ORDER BY p.main_levee_at ASC",
        )
        .bind(session_id)
        .fetch_all(pool)
        .await?;

    Ok(rows
        .into_iter()
        .map(|(utilisateur_id, nom, prenom, main_levee_at)| DemandeParole {
            utilisateur_id,
            nom: format!("{} {}", prenom.as_deref().unwrap_or(""), nom)
                .trim()
                .to_string(),
            main_levee_at,
        })
        .collect())
}

// ════════════════════════════════════════════════════════════════
// US1/US2 (foundational) : Rejoindre (open-or-join) (T010)
// ════════════════════════════════════════════════════════════════

/// POST /api/evenements/{id}/direct/rejoindre, ouvre la session (organisateur) ou
/// rejoint l'active. Renvoie le token LiveKit scopé par rôle.
pub async fn rejoindre(
    pool: web::Data<PgPool>,
    cfg: web::Data<LivekitConfig>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();
    let moi = utilisateur_courant(&req)?;
    let evt = charger_evenement(pool.get_ref(), id).await?;
    let est_organisateur = evt.cree_par == moi;

    // Éligibilité (D7) : organisateur OU inscrit.
    if !est_organisateur && !est_inscrit(pool.get_ref(), id, moi).await? {
        return Err(ApiErreur::AccesInterdit(
            "Inscrivez-vous d'abord pour rejoindre ce direct".to_string(),
        ));
    }

    // Clôture paresseuse de l'éventuelle session expirée avant toute décision.
    let mut session = charger_session_active(pool.get_ref(), id).await?;
    if let Some(s) = &session {
        if cloturer_si_necessaire(pool.get_ref(), cfg.get_ref(), s, evt.est_annule()).await? {
            session = None;
        }
    }

    if evt.est_annule() {
        return Err(ApiErreur::Conflit(
            "Cet événement a été annulé".to_string(),
        ));
    }

    // open-or-join.
    let (session, created) = match session {
        Some(s) => (s, false),
        None => {
            // Ouverture : organisateur uniquement, format diffusable, publié, dans la fenêtre.
            if !est_organisateur {
                return Err(ApiErreur::Conflit(
                    "Le direct n'a pas encore été ouvert par l'organisateur".to_string(),
                ));
            }
            if !est_diffusable(&evt.format) {
                return Err(ApiErreur::Validation(
                    "Cet événement n'est pas diffusable en direct (format non en ligne)".to_string(),
                ));
            }
            if evt.etat != "publie" {
                return Err(ApiErreur::Validation(
                    "L'événement doit être publié pour ouvrir le direct".to_string(),
                ));
            }
            if chrono::Utc::now() < fenetre_ouverture_at(evt.date_heure_debut) {
                return Err(ApiErreur::Validation(
                    "Le direct ne peut être ouvert que 15 minutes avant le début".to_string(),
                ));
            }
            let arret = calc_arret_securite_at(evt.date_heure_debut, evt.date_heure_fin);
            let query = format!(
                "INSERT INTO media_content.evenement_session
                    (evenement_id, etat, organisateur_id, max_participants, arret_securite_at)
                 VALUES ($1, 'en_cours', $2, $3, $4)
                 RETURNING {EVENEMENT_SESSION_COLONNES}"
            );
            let s = sqlx::query_as::<_, SessionRow>(&query)
                .bind(id)
                .bind(moi)
                .bind(MAX_PARTICIPANTS_DEFAUT)
                .bind(arret)
                .fetch_one(pool.get_ref())
                .await?;
            (s, true)
        }
    };

    // Rôle : organisateur fixe ; sinon préserve un rôle intervenant déjà accordé.
    let role: String = if est_organisateur {
        ROLE_ORGANISATEUR.to_string()
    } else {
        let existant: Option<String> = sqlx::query_scalar(
            "SELECT role FROM media_content.evenement_session_participant
             WHERE session_id = $1 AND utilisateur_id = $2",
        )
        .bind(session.id)
        .bind(moi)
        .fetch_optional(pool.get_ref())
        .await?;
        match existant.as_deref() {
            Some(ROLE_INTERVENANT) => ROLE_INTERVENANT.to_string(),
            _ => ROLE_SPECTATEUR.to_string(),
        }
    };

    // Capacité (D8) : refus 409 à un nouvel arrivant non-organisateur si pleine.
    if !created && !est_organisateur {
        let actifs = compter_participants_actifs(pool.get_ref(), session.id).await?;
        let deja_present: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM media_content.evenement_session_participant
             WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL)",
        )
        .bind(session.id)
        .bind(moi)
        .fetch_one(pool.get_ref())
        .await?;
        if !deja_present && actifs >= session.max_participants as i64 {
            return Err(ApiErreur::Conflit(
                "Capacité atteinte, réessayez plus tard".to_string(),
            ));
        }
    }

    // Upsert participant (reconnexion = quitte_at NULL, FR-014) sans dégrader le rôle.
    sqlx::query(
        "INSERT INTO media_content.evenement_session_participant
            (session_id, utilisateur_id, role)
         VALUES ($1, $2, $3)
         ON CONFLICT (session_id, utilisateur_id)
         DO UPDATE SET quitte_at = NULL, rejoint_at = NOW(), role = $3",
    )
    .bind(session.id)
    .bind(moi)
    .bind(&role)
    .execute(pool.get_ref())
    .await?;

    // Pic de présence.
    sqlx::query(
        "UPDATE media_content.evenement_session
         SET nombre_participants_pic = GREATEST(nombre_participants_pic,
             (SELECT COUNT(*) FROM media_content.evenement_session_participant
              WHERE session_id = $1 AND quitte_at IS NULL)),
             updated_at = NOW()
         WHERE id = $1",
    )
    .bind(session.id)
    .execute(pool.get_ref())
    .await?;

    let salle = room_name(session.id);
    let nom = nom_utilisateur(pool.get_ref(), moi).await;
    let token = generer_token(cfg.get_ref(), &salle, moi, &nom, &role)?;

    // À l'ouverture : audit + notifications (cloche + SSE) aux inscrits.
    if created {
        auditer(
            pool.get_ref(),
            &req,
            moi,
            "OUVRIR",
            session.id,
            json!({ "etat": "en_cours", "evenement_id": id }),
        )
        .await;
        notifier_ouverture(pool.get_ref(), sse.get_ref(), id, &evt.titre).await;
    }

    let enveloppe = ApiResponse {
        success: true,
        data: Some(TokenDirectResponse {
            session_id: session.id,
            room_name: salle,
            livekit_url: cfg.url.clone(),
            token,
            role,
        }),
        error: None,
    };
    if created {
        Ok(HttpResponse::Created().json(enveloppe))
    } else {
        Ok(HttpResponse::Ok().json(enveloppe))
    }
}

// ════════════════════════════════════════════════════════════════
// US1 (foundational) : Quitter (T011)
// ════════════════════════════════════════════════════════════════

/// POST /api/evenements/{id}/direct/quitter, marque l'appelant sorti (idempotent).
pub async fn quitter(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();
    let moi = utilisateur_courant(&req)?;

    if let Some(session) = charger_session_active(pool.get_ref(), id).await? {
        sqlx::query(
            "UPDATE media_content.evenement_session_participant
             SET quitte_at = NOW(),
                 duree_secondes = COALESCE(duree_secondes, 0)
                     + EXTRACT(EPOCH FROM (NOW() - rejoint_at))::int
             WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL",
        )
        .bind(session.id)
        .bind(moi)
        .execute(pool.get_ref())
        .await?;
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({ "quitte": true })),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US2 : Clôturer (T018)
// ════════════════════════════════════════════════════════════════

/// POST /api/evenements/{id}/direct/cloturer, organisateur uniquement (403 sinon).
pub async fn cloturer(
    pool: web::Data<PgPool>,
    cfg: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();
    let moi = utilisateur_courant(&req)?;
    let evt = charger_evenement(pool.get_ref(), id).await?;
    if evt.cree_par != moi {
        return Err(ApiErreur::AccesInterdit(
            "Seul l'organisateur peut clôturer le direct".to_string(),
        ));
    }

    if let Some(session) = charger_session_active(pool.get_ref(), id).await? {
        sqlx::query(
            "UPDATE media_content.evenement_session
             SET etat = 'terminee', termine_at = NOW(),
                 duree_secondes = EXTRACT(EPOCH FROM (NOW() - demarre_at))::int, updated_at = NOW()
             WHERE id = $1 AND etat = 'en_cours'",
        )
        .bind(session.id)
        .execute(pool.get_ref())
        .await?;

        let _ = livekit_moderation::fermer_session_admin(
            cfg.get_ref(),
            &room_name(session.id),
            "Le direct est terminé.",
        )
        .await;

        auditer(
            pool.get_ref(),
            &req,
            moi,
            "CLOTURER",
            session.id,
            json!({ "etat": "terminee" }),
        )
        .await;
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({ "cloture": true })),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// US4 : Lever la main & modération (T026)
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct LeverMainBody {
    pub levee: Option<bool>,
}

/// POST /api/evenements/{id}/direct/lever-main, toggle `main_levee` (spectateur).
pub async fn lever_main(
    pool: web::Data<PgPool>,
    cfg: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: Option<web::Json<LeverMainBody>>,
) -> Result<HttpResponse, ApiErreur> {
    let id = chemin.into_inner();
    let moi = utilisateur_courant(&req)?;

    let session = charger_session_active(pool.get_ref(), id)
        .await?
        .ok_or_else(|| ApiErreur::Conflit("Aucun direct en cours".to_string()))?;

    // Rôle courant : seul un spectateur peut lever la main (FR-022).
    let role: Option<(String, bool)> = sqlx::query_as(
        "SELECT role, main_levee FROM media_content.evenement_session_participant
         WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL",
    )
    .bind(session.id)
    .bind(moi)
    .fetch_optional(pool.get_ref())
    .await?;

    let (role_courant, main_actuelle) =
        role.ok_or_else(|| ApiErreur::Conflit("Vous n'êtes pas dans ce direct".to_string()))?;
    if role_courant != ROLE_SPECTATEUR {
        return Err(ApiErreur::Validation(
            "Seuls les spectateurs peuvent lever la main".to_string(),
        ));
    }

    let nouvelle = body.and_then(|b| b.levee).unwrap_or(!main_actuelle);
    sqlx::query(
        "UPDATE media_content.evenement_session_participant
         SET main_levee = $3,
             main_levee_at = CASE WHEN $3 THEN NOW() ELSE NULL END
         WHERE session_id = $1 AND utilisateur_id = $2",
    )
    .bind(session.id)
    .bind(moi)
    .bind(nouvelle)
    .execute(pool.get_ref())
    .await?;

    let nom = nom_utilisateur(pool.get_ref(), moi).await;
    let _ = livekit_moderation::publier_evenement_moderation(
        cfg.get_ref(),
        &room_name(session.id),
        &json!({
            "type": "moderation",
            "subtype": "main_levee",
            "payload": { "utilisateur_id": moi, "nom": nom, "levee": nouvelle }
        }),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(json!({ "main_levee": nouvelle })),
        error: None,
    }))
}

enum ActionModeration {
    Promouvoir,
    Retrograder,
    Retirer,
}

/// POST …/participants/{utilisateur_id}/promouvoir, organisateur uniquement.
pub async fn promouvoir(
    pool: web::Data<PgPool>,
    cfg: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    moderer(pool, cfg, req, chemin, ActionModeration::Promouvoir).await
}

/// POST …/participants/{utilisateur_id}/retrograder, organisateur uniquement.
pub async fn retrograder(
    pool: web::Data<PgPool>,
    cfg: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    moderer(pool, cfg, req, chemin, ActionModeration::Retrograder).await
}

/// POST …/participants/{utilisateur_id}/retirer, organisateur uniquement.
pub async fn retirer(
    pool: web::Data<PgPool>,
    cfg: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, ApiErreur> {
    moderer(pool, cfg, req, chemin, ActionModeration::Retirer).await
}

/// Logique commune de modération : organisateur de l'événement uniquement (403),
/// mutation `role` / `quitte_at` en base + application LiveKit (D3) + DataPacket.
async fn moderer(
    pool: web::Data<PgPool>,
    cfg: web::Data<LivekitConfig>,
    req: HttpRequest,
    chemin: web::Path<(Uuid, Uuid)>,
    action: ActionModeration,
) -> Result<HttpResponse, ApiErreur> {
    let (id, cible) = chemin.into_inner();
    let moi = utilisateur_courant(&req)?;
    let evt = charger_evenement(pool.get_ref(), id).await?;
    if evt.cree_par != moi {
        return Err(ApiErreur::AccesInterdit(
            "Seul l'organisateur peut modérer le direct".to_string(),
        ));
    }
    let session = charger_session_active(pool.get_ref(), id)
        .await?
        .ok_or_else(|| ApiErreur::Conflit("Aucun direct en cours".to_string()))?;
    let salle = room_name(session.id);

    let (donnees, action_audit) = match action {
        ActionModeration::Promouvoir => {
            sqlx::query(
                "UPDATE media_content.evenement_session_participant
                 SET role = 'intervenant', main_levee = FALSE, main_levee_at = NULL
                 WHERE session_id = $1 AND utilisateur_id = $2",
            )
            .bind(session.id)
            .bind(cible)
            .execute(pool.get_ref())
            .await?;
            let _ = livekit_moderation::update_participant_can_publish(
                cfg.get_ref(),
                &salle,
                &cible.to_string(),
                true,
            )
            .await;
            let _ = livekit_moderation::publier_evenement_moderation(
                cfg.get_ref(),
                &salle,
                &json!({ "type": "moderation", "subtype": "role_update",
                         "payload": { "utilisateur_id": cible, "role": ROLE_INTERVENANT } }),
            )
            .await;
            (json!({ "role": ROLE_INTERVENANT }), "PROMOUVOIR")
        }
        ActionModeration::Retrograder => {
            sqlx::query(
                "UPDATE media_content.evenement_session_participant
                 SET role = 'spectateur'
                 WHERE session_id = $1 AND utilisateur_id = $2",
            )
            .bind(session.id)
            .bind(cible)
            .execute(pool.get_ref())
            .await?;
            let _ = livekit_moderation::update_participant_can_publish(
                cfg.get_ref(),
                &salle,
                &cible.to_string(),
                false,
            )
            .await;
            let _ = livekit_moderation::publier_evenement_moderation(
                cfg.get_ref(),
                &salle,
                &json!({ "type": "moderation", "subtype": "role_update",
                         "payload": { "utilisateur_id": cible, "role": ROLE_SPECTATEUR } }),
            )
            .await;
            (json!({ "role": ROLE_SPECTATEUR }), "RETROGRADER")
        }
        ActionModeration::Retirer => {
            if cible == evt.cree_par {
                return Err(ApiErreur::Validation(
                    "L'organisateur ne peut pas être retiré".to_string(),
                ));
            }
            sqlx::query(
                "UPDATE media_content.evenement_session_participant
                 SET quitte_at = NOW(),
                     duree_secondes = COALESCE(duree_secondes, 0)
                         + EXTRACT(EPOCH FROM (NOW() - rejoint_at))::int
                 WHERE session_id = $1 AND utilisateur_id = $2 AND quitte_at IS NULL",
            )
            .bind(session.id)
            .bind(cible)
            .execute(pool.get_ref())
            .await?;
            let _ = livekit_moderation::retirer_participant(
                cfg.get_ref(),
                &salle,
                &cible.to_string(),
            )
            .await;
            let _ = livekit_moderation::publier_evenement_moderation(
                cfg.get_ref(),
                &salle,
                &json!({ "type": "moderation", "subtype": "retire",
                         "payload": { "utilisateur_id": cible } }),
            )
            .await;
            (json!({ "retire": true }), "RETIRER")
        }
    };

    auditer(
        pool.get_ref(),
        &req,
        moi,
        action_audit,
        session.id,
        json!({ "cible": cible, "resultat": donnees }),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(donnees),
        error: None,
    }))
}
