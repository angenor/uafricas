// Handlers du domaine messagerie : flux SSE temps réel, conversations, messages.
// User Story 3 : chat texte 1-1 entre amis.

use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiErreur;
use crate::jwt;
use crate::models::amitie::paire_canonique;
use crate::models::messagerie::{
    evt_message, evt_message_supprime, evt_non_lus, ConversationRow, MessageResponse, MessageRow,
};
use crate::services::audit;
use crate::services::messagerie_sse::{flux_sse, RegistreSse};
use crate::ApiResponse;

const CONTENU_MAX: usize = 2000;
const LIMITE_DEFAUT: i64 = 30;
const LIMITE_MAX: i64 = 50;

// ════════════════════════════════════════════════════════════════
// Helpers
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

async fn blocage_existe(pool: &PgPool, a: Uuid, b: Uuid) -> Result<bool, ApiErreur> {
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.blocage
            WHERE (bloqueur_id = $1 AND bloque_id = $2) OR (bloqueur_id = $2 AND bloque_id = $1)
        )",
    )
    .bind(a)
    .bind(b)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

async fn amitie_active(pool: &PgPool, a: Uuid, b: Uuid) -> Result<bool, ApiErreur> {
    let (min, max) = paire_canonique(a, b);
    let existe: bool = sqlx::query_scalar(
        "SELECT EXISTS(
            SELECT 1 FROM social.amitie WHERE utilisateur_a_id = $1 AND utilisateur_b_id = $2
        )",
    )
    .bind(min)
    .bind(max)
    .fetch_one(pool)
    .await?;
    Ok(existe)
}

async fn trouver_conversation(pool: &PgPool, a: Uuid, b: Uuid) -> Result<Option<Uuid>, ApiErreur> {
    let (min, max) = paire_canonique(a, b);
    let id: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM social.conversation
         WHERE utilisateur_a_id = $1 AND utilisateur_b_id = $2",
    )
    .bind(min)
    .bind(max)
    .fetch_optional(pool)
    .await?;
    Ok(id)
}

async fn obtenir_ou_creer_conversation(pool: &PgPool, a: Uuid, b: Uuid) -> Result<Uuid, ApiErreur> {
    let (min, max) = paire_canonique(a, b);
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO social.conversation (utilisateur_a_id, utilisateur_b_id)
         VALUES ($1, $2)
         ON CONFLICT (utilisateur_a_id, utilisateur_b_id)
            DO UPDATE SET utilisateur_a_id = EXCLUDED.utilisateur_a_id
         RETURNING id",
    )
    .bind(min)
    .bind(max)
    .fetch_one(pool)
    .await?;
    Ok(id)
}

/// Compte les messages non lus reçus par `moi` (tous fils confondus).
async fn compter_non_lus_total(pool: &PgPool, moi: Uuid) -> Result<i64, ApiErreur> {
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM social.message m
         JOIN social.conversation c ON c.id = m.conversation_id
         WHERE (c.utilisateur_a_id = $1 OR c.utilisateur_b_id = $1)
           AND m.expediteur_id <> $1 AND m.lu_at IS NULL AND m.deleted_at IS NULL",
    )
    .bind(moi)
    .fetch_one(pool)
    .await?;
    Ok(total)
}

// ════════════════════════════════════════════════════════════════
// Flux SSE (T028)
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct FluxQuery {
    pub token: Option<String>,
}

/// GET /api/messagerie/flux?token=<jwt>, Flux temps réel serveur→client.
/// Auth par token en query (EventSource ne supporte pas les en-têtes, Décision 3).
pub async fn flux(
    sse: web::Data<RegistreSse>,
    query: web::Query<FluxQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let token = query
        .token
        .as_deref()
        .ok_or_else(|| ApiErreur::NonAutorise("Token requis".to_string()))?;
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| ApiErreur::BaseDeDonnees("JWT_SECRET non configuré".to_string()))?;
    let claims = jwt::valider_token(token, &secret)?;
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiErreur::NonAutorise("Token invalide".to_string()))?;

    let rx = sse.abonner(user_id);

    Ok(HttpResponse::Ok()
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("X-Accel-Buffering", "no"))
        .content_type("text/event-stream")
        .streaming(flux_sse(rx)))
}

// ════════════════════════════════════════════════════════════════
// Conversations & messages
// ════════════════════════════════════════════════════════════════

/// GET /api/messagerie/conversations : Liste des conversations (FR-020).
pub async fn lister_conversations(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;

    let rows = sqlx::query_as::<_, ConversationRow>(
        "SELECT
            c.id AS conversation_id,
            ami.id AS ami_id, ami.nom, ami.prenom, ami.slug, ami.photo_url, ami.fonction,
            p.nom AS pays,
            (SELECT m.contenu FROM social.message m
                WHERE m.conversation_id = c.id AND m.deleted_at IS NULL
                ORDER BY m.created_at DESC LIMIT 1) AS dernier_contenu,
            (SELECT m.created_at FROM social.message m
                WHERE m.conversation_id = c.id AND m.deleted_at IS NULL
                ORDER BY m.created_at DESC LIMIT 1) AS dernier_at,
            (SELECT COUNT(*) FROM social.message m
                WHERE m.conversation_id = c.id AND m.expediteur_id <> $1
                  AND m.lu_at IS NULL AND m.deleted_at IS NULL) AS non_lus,
            EXISTS(SELECT 1 FROM social.amitie a
                WHERE a.utilisateur_a_id = LEAST($1, ami.id)
                  AND a.utilisateur_b_id = GREATEST($1, ami.id)) AS amis,
            EXISTS(SELECT 1 FROM social.blocage b
                WHERE (b.bloqueur_id = $1 AND b.bloque_id = ami.id)
                   OR (b.bloqueur_id = ami.id AND b.bloque_id = $1)) AS bloque,
            c.annonce_id,
            an.titre AS annonce_titre,
            an.slug AS annonce_slug
         FROM social.conversation c
         JOIN iam.utilisateur ami
            ON ami.id = CASE WHEN c.utilisateur_a_id = $1 THEN c.utilisateur_b_id ELSE c.utilisateur_a_id END
         LEFT JOIN shared.pays p ON p.id = ami.pays_residence_id
         LEFT JOIN marketplace.annonce an ON an.id = c.annonce_id
         WHERE (c.utilisateur_a_id = $1 OR c.utilisateur_b_id = $1)
           AND ami.deleted_at IS NULL
         ORDER BY c.dernier_message_at DESC NULLS LAST, c.created_at DESC",
    )
    .bind(moi)
    .fetch_all(pool.get_ref())
    .await?;

    let conversations: Vec<_> = rows.iter().map(|r| r.to_response()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(conversations),
        error: None,
    }))
}

#[derive(Debug, Deserialize)]
pub struct MessagesQuery {
    pub avant: Option<DateTime<Utc>>,
    pub limite: Option<i64>,
}

/// GET /api/messagerie/conversations/{ami_id}/messages, Historique paginé (FR-023).
pub async fn lister_messages(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    query: web::Query<MessagesQuery>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let ami_id = chemin.into_inner();

    // FR-022 : blocage interdit l'accès
    if blocage_existe(pool.get_ref(), moi, ami_id).await? {
        return Err(ApiErreur::AccesInterdit(
            "Conversation indisponible".to_string(),
        ));
    }

    // Conversation existante (historique lisible même si l'amitié a cessé) ;
    // sinon, on la crée à la volée uniquement si les membres sont amis.
    let conv_id = match trouver_conversation(pool.get_ref(), moi, ami_id).await? {
        Some(id) => id,
        None => {
            if amitie_active(pool.get_ref(), moi, ami_id).await? {
                obtenir_ou_creer_conversation(pool.get_ref(), moi, ami_id).await?
            } else {
                return Err(ApiErreur::AccesInterdit(
                    "Vous n'êtes pas amis avec ce membre".to_string(),
                ));
            }
        }
    };

    let limite = query.limite.unwrap_or(LIMITE_DEFAUT).clamp(1, LIMITE_MAX);

    // Page la plus récente (ou plus ancienne que `avant`), triée du plus récent au plus ancien.
    let rows = sqlx::query_as::<_, MessageRow>(
        "SELECT id, expediteur_id, contenu, created_at, lu_at, deleted_at
         FROM social.message
         WHERE conversation_id = $1 AND ($2::timestamptz IS NULL OR created_at < $2)
         ORDER BY created_at DESC
         LIMIT $3",
    )
    .bind(conv_id)
    .bind(query.avant)
    .bind(limite)
    .fetch_all(pool.get_ref())
    .await?;

    // Réordonner du plus ancien au plus récent dans la page renvoyée.
    let messages: Vec<MessageResponse> = rows.iter().rev().map(|r| r.to_response()).collect();

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(messages),
        error: None,
    }))
}

#[derive(Debug, Deserialize)]
pub struct EnvoyerMessageBody {
    pub contenu: String,
}

/// POST /api/messagerie/conversations/{ami_id}/messages, Envoyer un message (FR-021).
pub async fn envoyer_message(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
    body: web::Json<EnvoyerMessageBody>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let ami_id = chemin.into_inner();

    // FR-027 : contenu non vide et ≤ 2000 caractères
    let contenu = body.contenu.clone();
    let longueur = contenu.chars().count();
    if contenu.trim().is_empty() || longueur > CONTENU_MAX {
        return Err(ApiErreur::Validation(
            "Le message doit contenir entre 1 et 2000 caractères".to_string(),
        ));
    }

    // FR-022 / R5 : amitié active requise, aucun blocage
    if blocage_existe(pool.get_ref(), moi, ami_id).await? {
        return Err(ApiErreur::AccesInterdit(
            "Messagerie indisponible avec ce membre".to_string(),
        ));
    }
    // R5 / D2 : envoi autorisé si amitié active OU si une conversation existe
    // déjà (ex. née d'un contact d'annonce du Marché Africain, sans amitié).
    let conversation_existante = trouver_conversation(pool.get_ref(), moi, ami_id).await?;
    if conversation_existante.is_none() && !amitie_active(pool.get_ref(), moi, ami_id).await? {
        return Err(ApiErreur::AccesInterdit(
            "Vous devez être amis pour démarrer cette conversation".to_string(),
        ));
    }

    let conv_id = match conversation_existante {
        Some(id) => id,
        None => obtenir_ou_creer_conversation(pool.get_ref(), moi, ami_id).await?,
    };

    let (message_id, created_at): (Uuid, DateTime<Utc>) = sqlx::query_as(
        "INSERT INTO social.message (conversation_id, expediteur_id, contenu)
         VALUES ($1, $2, $3) RETURNING id, created_at",
    )
    .bind(conv_id)
    .bind(moi)
    .bind(&contenu)
    .fetch_one(pool.get_ref())
    .await?;

    sqlx::query("UPDATE social.conversation SET dernier_message_at = NOW() WHERE id = $1")
        .bind(conv_id)
        .execute(pool.get_ref())
        .await?;

    let message = MessageResponse {
        id: message_id,
        expediteur_id: moi,
        contenu: Some(contenu),
        supprime: false,
        created_at,
        lu_at: None,
    };

    // Push temps réel au destinataire et aux autres connexions de l'expéditeur.
    let evt = evt_message(conv_id, &message);
    sse.publier(ami_id, &evt);
    sse.publier(moi, &evt);

    // Audit : métadonnées uniquement, jamais le contenu (Décision 9).
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "CREATE",
        "social",
        "message",
        Some(message_id),
        None,
        Some(serde_json::json!({
            "conversation_id": conv_id,
            "expediteur_id": moi,
            "longueur": longueur,
        })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "message": message })),
        error: None,
    }))
}

/// POST /api/messagerie/conversations/{ami_id}/lu, Marquer les messages reçus lus (FR-024).
pub async fn marquer_conversation_lue(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let ami_id = chemin.into_inner();

    if let Some(conv_id) = trouver_conversation(pool.get_ref(), moi, ami_id).await? {
        sqlx::query(
            "UPDATE social.message SET lu_at = NOW()
             WHERE conversation_id = $1 AND expediteur_id = $2
               AND lu_at IS NULL AND deleted_at IS NULL",
        )
        .bind(conv_id)
        .bind(ami_id)
        .execute(pool.get_ref())
        .await?;

        // Synchronise les autres connexions de l'utilisateur (badge non-lus).
        sse.publier(moi, &evt_non_lus(conv_id, 0));
    }

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "non_lus": 0 })),
        error: None,
    }))
}

#[derive(sqlx::FromRow)]
struct MessageProprietaireRow {
    expediteur_id: Uuid,
    conversation_id: Uuid,
    utilisateur_a_id: Uuid,
    utilisateur_b_id: Uuid,
}

/// DELETE /api/messagerie/messages/{id}, Supprimer un de ses messages (FR-028).
pub async fn supprimer_message(
    pool: web::Data<PgPool>,
    sse: web::Data<RegistreSse>,
    req: HttpRequest,
    chemin: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let message_id = chemin.into_inner();

    let row = sqlx::query_as::<_, MessageProprietaireRow>(
        "SELECT m.expediteur_id, m.conversation_id, c.utilisateur_a_id, c.utilisateur_b_id
         FROM social.message m
         JOIN social.conversation c ON c.id = m.conversation_id
         WHERE m.id = $1",
    )
    .bind(message_id)
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiErreur::NonTrouve("Message introuvable".to_string()))?;

    if row.expediteur_id != moi {
        return Err(ApiErreur::AccesInterdit(
            "Vous ne pouvez supprimer que vos propres messages".to_string(),
        ));
    }

    sqlx::query("UPDATE social.message SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL")
        .bind(message_id)
        .execute(pool.get_ref())
        .await?;

    // Push aux deux participants.
    let autre = if row.utilisateur_a_id == moi {
        row.utilisateur_b_id
    } else {
        row.utilisateur_a_id
    };
    let evt = evt_message_supprime(row.conversation_id, message_id);
    sse.publier(moi, &evt);
    sse.publier(autre, &evt);

    // Audit : métadonnées uniquement (Décision 9).
    let ip = audit::extraire_ip(&req);
    let ua = audit::extraire_user_agent(&req);
    audit::log_action(
        pool.get_ref(),
        Some(moi),
        "DELETE",
        "social",
        "message",
        Some(message_id),
        None,
        Some(serde_json::json!({ "conversation_id": row.conversation_id })),
        ip.as_deref(),
        ua.as_deref(),
    )
    .await;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "supprime": true })),
        error: None,
    }))
}

/// GET /api/messagerie/non-lus : Compteur global de messages non lus (FR-024).
pub async fn compteur_non_lus(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiErreur> {
    let moi = utilisateur_courant(&req)?;
    let total = compter_non_lus_total(pool.get_ref(), moi).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({ "total": total })),
        error: None,
    }))
}
