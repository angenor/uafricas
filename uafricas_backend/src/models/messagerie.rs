// Modèles du domaine messagerie : conversations, messages, évènements SSE.
// Source de vérité : doc/bd/schemas/29_social.sql

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::amitie::MembreLight;

// ────────────────────────────────────────────────────────────────
// Lignes brutes
// ────────────────────────────────────────────────────────────────

/// Ligne brute social.conversation
#[allow(dead_code)]
#[derive(Debug, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub utilisateur_a_id: Uuid,
    pub utilisateur_b_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub dernier_message_at: Option<DateTime<Utc>>,
}

/// Ligne brute social.message
#[derive(Debug, FromRow)]
pub struct MessageRow {
    pub id: Uuid,
    pub expediteur_id: Uuid,
    pub contenu: String,
    pub created_at: DateTime<Utc>,
    pub lu_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl MessageRow {
    pub fn to_response(&self) -> MessageResponse {
        let supprime = self.deleted_at.is_some();
        MessageResponse {
            id: self.id,
            expediteur_id: self.expediteur_id,
            // FR-028 : un message supprimé n'expose plus son contenu
            contenu: if supprime { None } else { Some(self.contenu.clone()) },
            supprime,
            created_at: self.created_at,
            lu_at: self.lu_at,
        }
    }
}

/// Ligne jointe pour la liste des conversations.
#[derive(Debug, FromRow)]
pub struct ConversationRow {
    pub conversation_id: Uuid,
    pub ami_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub slug: Option<String>,
    pub photo_url: Option<String>,
    pub fonction: Option<String>,
    pub pays: Option<String>,
    pub dernier_contenu: Option<String>,
    pub dernier_at: Option<DateTime<Utc>>,
    pub non_lus: i64,
    pub amis: bool,
    pub bloque: bool,
}

const EXTRAIT_MAX: usize = 120;

impl ConversationRow {
    pub fn to_response(&self) -> ConversationResponse {
        let dernier_message = match (&self.dernier_contenu, self.dernier_at) {
            (Some(contenu), Some(at)) => Some(ExtraitMessage {
                extrait: tronquer(contenu, EXTRAIT_MAX),
                created_at: at,
            }),
            _ => None,
        };
        ConversationResponse {
            conversation_id: self.conversation_id,
            ami: MembreLight {
                id: self.ami_id,
                nom: self.nom.clone(),
                prenom: self.prenom.clone(),
                slug: self.slug.clone(),
                photo_url: self.photo_url.clone(),
                fonction: self.fonction.clone(),
                pays: self.pays.clone(),
            },
            dernier_message,
            non_lus: self.non_lus,
            // FR-025 : verrouillée si l'amitié n'existe plus ou si blocage
            verrouillee: !self.amis || self.bloque,
        }
    }
}

fn tronquer(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let tronque: String = s.chars().take(max).collect();
        format!("{tronque}…")
    }
}

// ────────────────────────────────────────────────────────────────
// DTO Response
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ExtraitMessage {
    pub extrait: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ConversationResponse {
    pub conversation_id: Uuid,
    pub ami: MembreLight,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dernier_message: Option<ExtraitMessage>,
    pub non_lus: i64,
    pub verrouillee: bool,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub id: Uuid,
    pub expediteur_id: Uuid,
    pub contenu: Option<String>,
    pub supprime: bool,
    pub created_at: DateTime<Utc>,
    pub lu_at: Option<DateTime<Utc>>,
}

// ────────────────────────────────────────────────────────────────
// Constructeurs d'évènements SSE (format du contrat api.md)
// ────────────────────────────────────────────────────────────────

/// `{ type: "message", conversation_id, message: {...} }`
pub fn evt_message(conversation_id: Uuid, message: &MessageResponse) -> serde_json::Value {
    serde_json::json!({
        "type": "message",
        "conversation_id": conversation_id,
        "message": message,
    })
}

/// `{ type: "message_supprime", conversation_id, message_id }`
pub fn evt_message_supprime(conversation_id: Uuid, message_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "type": "message_supprime",
        "conversation_id": conversation_id,
        "message_id": message_id,
    })
}

/// `{ type: "non_lus", conversation_id, non_lus }`
pub fn evt_non_lus(conversation_id: Uuid, non_lus: i64) -> serde_json::Value {
    serde_json::json!({
        "type": "non_lus",
        "conversation_id": conversation_id,
        "non_lus": non_lus,
    })
}
