// Handlers du domaine social : appel direct en visioconférence entre amis.
// Permet d'appeler un ami immédiatement, sans prise de rendez-vous. L'appel est
// éphémère (registre en mémoire `RegistreAppels`) ; aucun média ni signalisation
// ne transite par le serveur (WebRTC pair-à-pair). Réutilise MembreLight,
// amitié/blocage, RegistreSse et les peer-id déterministes du module rendez_vous.

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::amitie::{obtenir_membre_light, paire_canonique, MembreLight};
use crate::models::appel::{
    evt_appel_accepte, evt_appel_annule, evt_appel_entrant, evt_appel_refuse, AppelerBody,
    SalleAppelResponse,
};
use crate::models::rendez_vous::peer_id;
use crate::services::appels::RegistreAppels;
use crate::services::messagerie_sse::RegistreSse;
use crate::ApiResponse;

// ════════════════════════════════════════════════════════════════
// Authentification & helpers de relation (mêmes garanties que rendez_vous)
// ════════════════════════════════════════════════════════════════

fn utilisateur_courant(req: &HttpRequest) -> Result<Uuid, ApiErreur> {
    let header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());
    let token = header.and_then(|h| h.strip_prefix("Bearer "));
    let secret = std::env::var("JWT_SECRET").ok();
    match (token, secret) {
        (Some(t), Some(s)) => jwt::valider_token(t, &s)
            .ok()
            .and_then(|c| Uuid::parse_str(&c.sub).ok()),
        _ => None,
    }
    .ok_or_else(|| ApiErreur::NonAutorise("Authentification requise".to_string()))
}

/// Blocage dans un sens ou l'autre.
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

/// Revérifie amitié active + absence de blocage (un appel n'est possible
/// qu'entre amis non bloqués).
async fn verifier_relation(pool: &PgPool, moi: Uuid, autre: Uuid) -> Result<(), ApiErreur> {
    if blocage_existe(pool, moi, autre).await? || !amitie_existe(pool, moi, autre).await? {
        return Err(ApiErreur::AccesInterdit(
            "Action impossible avec ce membre".to_string(),
        ));
    }
    Ok(())
}

/// Charge le MembreLight d'un participant (404 si introuvable).
async fn membre_light(pool: &PgPool, id: Uuid) -> Result<MembreLight, ApiErreur> {
    obtenir_membre_light(pool, id)
        .await?
        .ok_or_else(|| ApiErreur::NonTrouve("Membre introuvable".to_string()))
}

/// Réponse de succès sans données (refus / annulation).
fn ok_vide() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({})),
        error: None,
    })
}

// ════════════════════════════════════════════════════════════════
// Démarrer un appel
// ════════════════════════════════════════════════════════════════

/// POST /api/appels : Démarrer un appel direct vers un ami.
/// Crée un appel éphémère, fait sonner le destinataire (SSE) et renvoie la
/// config P2P à l'appelant pour ouvrir la salle immédiatement.
pub async fn appeler(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    registre: web::Data<RegistreAppels>,
    req: HttpRequest,
    body: web::Json<AppelerBody>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let cible = body.destinataire_id;

    if cible == moi {
        return Err(ApiErreur::Validation(
            "Vous ne pouvez pas vous appeler vous-même".to_string(),
        ));
    }
    verifier_relation(pool.get_ref(), moi, cible).await?;

    let video = body.video;
    let appel_id = Uuid::new_v4();
    registre.ouvrir(appel_id, moi, cible, video);

    // Sonnerie temps réel au destinataire (l'appelant accompagne l'évènement).
    let appelant = membre_light(pool.get_ref(), moi).await?;
    sse.publier(cible, &evt_appel_entrant(appel_id, &appelant, video));

    // Config P2P renvoyée à l'appelant.
    let autre = membre_light(pool.get_ref(), cible).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(SalleAppelResponse {
            appel_id,
            mon_peer_id: peer_id(appel_id, moi),
            pair_peer_id: peer_id(appel_id, cible),
            suis_appelant: moi < cible,
            video,
            autre,
        }),
        error: None,
    }))
}

// ════════════════════════════════════════════════════════════════
// Rejoindre (accepter) / refuser / annuler
// ════════════════════════════════════════════════════════════════

/// GET /api/appels/{id}/salle : Config P2P pour rejoindre l'appel (= accepter).
/// Le destinataire qui rejoint déclenche `appel_accepte` vers l'appelant.
pub async fn salle(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    registre: web::Data<RegistreAppels>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    let appel = registre
        .obtenir(id)
        .ok_or_else(|| ApiErreur::NonTrouve("Appel introuvable ou expiré".to_string()))?;
    let autre_id = appel
        .autre_que(moi)
        .ok_or_else(|| ApiErreur::AccesInterdit("Vous ne participez pas à cet appel".to_string()))?;
    verifier_relation(pool.get_ref(), moi, autre_id).await?;

    // Le destinataire qui rejoint prévient l'appelant (mise à jour du statut).
    if moi == appel.destinataire_id {
        sse.publier(appel.appelant_id, &evt_appel_accepte(id));
    }

    let autre = membre_light(pool.get_ref(), autre_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(SalleAppelResponse {
            appel_id: id,
            mon_peer_id: peer_id(id, moi),
            pair_peer_id: peer_id(id, autre_id),
            suis_appelant: moi < autre_id,
            video: appel.video,
            autre,
        }),
        error: None,
    }))
}

/// POST /api/appels/{id}/refuser : Le destinataire décline l'appel.
pub async fn refuser(
    sse: web::Data<RegistreSse>,
    registre: web::Data<RegistreAppels>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    if let Some(appel) = registre.obtenir(id) {
        if let Some(autre_id) = appel.autre_que(moi) {
            registre.retirer(id);
            sse.publier(autre_id, &evt_appel_refuse(id));
        }
    }
    Ok(ok_vide())
}

/// POST /api/appels/{id}/annuler : Un participant abandonne avant connexion
/// (l'appelant raccroche pendant la sonnerie). L'autre est prévenu.
pub async fn annuler(
    sse: web::Data<RegistreSse>,
    registre: web::Data<RegistreAppels>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let id = chemin.into_inner();

    if let Some(appel) = registre.obtenir(id) {
        if let Some(autre_id) = appel.autre_que(moi) {
            registre.retirer(id);
            sse.publier(autre_id, &evt_appel_annule(id));
        }
    }
    Ok(ok_vide())
}
