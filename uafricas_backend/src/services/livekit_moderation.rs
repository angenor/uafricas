//! Wrappers LiveKit pour la modération de session (feature 001-session-moderation).
//!
//! Isolent l'usage de `livekit_api::services::room::RoomClient` aux deux primitives
//! utilisées par la modération :
//! - [`update_participant_can_publish_data`] — bascule `ParticipantPermission.can_publish_data`
//!   via `UpdateParticipant` ; le SFU LiveKit refuse alors lui-même tout DataPacket
//!   émis par un participant non autorisé (FR-015).
//! - [`publier_evenement_moderation`] — diffuse un DataPacket JSON sur le canal
//!   `RELIABLE` à tous les participants de la room (FR-014/FR-023).
//!
//! Le `host` d'API LiveKit est dérivé de la `LivekitConfig.url` (qui peut être
//! `ws://`/`wss://` côté client) en la convertissant en `http://`/`https://`.

use livekit_api::services::room::{RoomClient, SendDataOptions, UpdateParticipantOptions};
use livekit_protocol as proto;

use crate::config::LivekitConfig;
use crate::errors::ApiErreur;

/// Convertit l'URL WebSocket LiveKit du client en URL HTTP pour l'API serveur.
fn host_api(cfg: &LivekitConfig) -> String {
    let url = cfg.url.trim();
    if let Some(rest) = url.strip_prefix("wss://") {
        format!("https://{}", rest)
    } else if let Some(rest) = url.strip_prefix("ws://") {
        format!("http://{}", rest)
    } else {
        url.to_string()
    }
}

fn room_client(cfg: &LivekitConfig) -> RoomClient {
    RoomClient::with_api_key(&host_api(cfg), &cfg.api_key, &cfg.api_secret)
}

/// Bascule la permission `can_publish_data` d'un participant donné dans une room
/// LiveKit. Conserve les autres permissions par défaut (`can_subscribe=true`,
/// `can_publish=true`) afin de ne pas couper l'audio/vidéo.
///
/// Retourne `Ok(())` même si l'opération échoue côté LiveKit — l'enforcement
/// reste appliqué via le statut Postgres au prochain `set_permissions` /
/// rejointe ; l'erreur est journalisée mais ne propage pas une 500 au client.
pub async fn update_participant_can_publish_data(
    cfg: &LivekitConfig,
    room_name: &str,
    identity: &str,
    autorise: bool,
) -> Result<(), ApiErreur> {
    let permission = proto::ParticipantPermission {
        can_subscribe: true,
        can_publish: true,
        can_publish_data: autorise,
        ..Default::default()
    };

    let options = UpdateParticipantOptions {
        permission: Some(permission),
        ..Default::default()
    };

    if let Err(err) = room_client(cfg)
        .update_participant(room_name, identity, options)
        .await
    {
        // Cas typique : participant pas encore connecté à la room.
        // On ne fait pas remonter 500 — le statut Postgres reste la source de
        // vérité et sera réappliqué à la jointure (cf. hook participant_joined).
        eprintln!(
            "[livekit_moderation] update_participant({}, {}, can_publish_data={}) a échoué : {}",
            room_name, identity, autorise, err
        );
    }
    Ok(())
}

/// Publie un DataPacket JSON sur le canal `RELIABLE` à tous les participants
/// de la room. Utilisé pour notifier en temps réel les mutations de modération
/// (permission_update, spotlight).
pub async fn publier_evenement_moderation(
    cfg: &LivekitConfig,
    room_name: &str,
    payload: &serde_json::Value,
) -> Result<(), ApiErreur> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Sérialisation DataPacket: {}", e)))?;

    let options = SendDataOptions {
        kind: proto::data_packet::Kind::Reliable,
        topic: Some("moderation".to_string()),
        ..Default::default()
    };

    if let Err(err) = room_client(cfg).send_data(room_name, bytes, options).await {
        eprintln!(
            "[livekit_moderation] send_data({}) a échoué : {}",
            room_name, err
        );
    }
    Ok(())
}
