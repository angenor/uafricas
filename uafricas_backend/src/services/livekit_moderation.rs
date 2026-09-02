//! Wrappers LiveKit pour la modération de session (feature 001-session-moderation).
//!
//! Isolent l'usage de `livekit_api::services::room::RoomClient` aux deux primitives
//! utilisées par la modération :
//! - [`update_participant_can_publish_data`], bascule `ParticipantPermission.can_publish_data`
//!   via `UpdateParticipant` ; le SFU LiveKit refuse alors lui-même tout DataPacket
//!   émis par un participant non autorisé (FR-015).
//! - [`publier_evenement_moderation`] : diffuse un DataPacket JSON sur le canal
//!   `RELIABLE` à tous les participants de la room (FR-014/FR-023).
//!
//! Le `host` d'API LiveKit est dérivé de la `LivekitConfig.api_url` (URL de
//! l'API serveur, distincte de l'URL WebSocket cliente) en la convertissant en
//! `http://`/`https://`.

use livekit_api::services::room::{RoomClient, SendDataOptions, UpdateParticipantOptions};
use livekit_protocol as proto;

use crate::config::LivekitConfig;
use crate::errors::ApiErreur;

/// Convertit l'URL LiveKit de l'API serveur (`api_url`) en URL HTTP.
///
/// On utilise `api_url` (et non `url`, l'URL WebSocket cliente) : derriere un
/// reverse-proxy, le client Twirp de `livekit_api` ecrase le chemin de l'URL, si
/// bien qu'un prefixe de proxy (ex. `/livekit-ws`) est perdu et la requete part
/// vers la mauvaise route. `api_url` doit donc pointer directement sur LiveKit.
fn host_api(cfg: &LivekitConfig) -> String {
    let url = cfg.api_url.trim();
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
/// Retourne `Ok(())` même si l'opération échoue côté LiveKit, l'enforcement
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
        // On ne fait pas remonter 500 : le statut Postgres reste la source de
        // vérité et sera réappliqué à la jointure (cf. hook participant_joined).
        eprintln!(
            "[livekit_moderation] update_participant({}, {}, can_publish_data={}) a échoué : {}",
            room_name, identity, autorise, err
        );
    }
    Ok(())
}

/// Bascule la permission `can_publish` d'un participant (modèle webinaire, feature
/// 001-evenements-streaming, D3). Sert à promouvoir un spectateur en intervenant
/// (`autorise = true`) ou à le rétrograder (`autorise = false`). `can_subscribe` et
/// `can_publish_data` restent `true` (le rétrogradé continue de regarder et d'envoyer
/// chat/réactions). Changer une permission LiveKit exige la clé API serveur → ce
/// toggle passe obligatoirement par le backend (le client ne peut pas s'auto-promouvoir).
///
/// Erreur LiveKit journalisée mais non bloquante (parité avec les autres wrappers) :
/// Postgres reste la source de vérité (`role`) et la permission est réappliquée à la
/// reconnexion via le token scopé.
pub async fn update_participant_can_publish(
    cfg: &LivekitConfig,
    room_name: &str,
    identity: &str,
    autorise: bool,
) -> Result<(), ApiErreur> {
    let permission = proto::ParticipantPermission {
        can_subscribe: true,
        can_publish: autorise,
        can_publish_data: true,
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
        eprintln!(
            "[livekit_moderation] update_participant({}, {}, can_publish={}) a échoué : {}",
            room_name, identity, autorise, err
        );
    }
    Ok(())
}

/// Retire (kick) un participant d'une room LiveKit via `RoomClient.remove_participant`
/// (feature 001-evenements-streaming, D3). Déconnecte effectivement le participant du
/// SFU. Erreur journalisée mais non bloquante, le `quitte_at` Postgres reste la source
/// de vérité ; un participant déjà absent provoque une erreur ignorée.
pub async fn retirer_participant(
    cfg: &LivekitConfig,
    room_name: &str,
    identity: &str,
) -> Result<(), ApiErreur> {
    if let Err(err) = room_client(cfg)
        .remove_participant(room_name, identity)
        .await
    {
        eprintln!(
            "[livekit_moderation] remove_participant({}, {}) a échoué : {}",
            room_name, identity, err
        );
    }
    Ok(())
}

/// Ferme administrativement une session : diffuse un DataPacket `RELIABLE`
/// `{type:'admin', subtype:'session_fermee', motif_public}` à tous les
/// participants, puis détruit la room LiveKit (kick effectif < 5 s, SC-005).
///
/// `motif_public` est volontairement court et générique, le motif détaillé
/// saisi par l'admin n'est PAS diffusé aux participants (FR : seuls les
/// admins de salle / créateur reçoivent le motif détaillé via notification).
///
/// Les erreurs LiveKit sont journalisées mais ne propagent pas une 500, la
/// mutation Postgres (etat='terminee' + salle.desactivee_admin) reste la source
/// de vérité et déclenche l'éjection effective au prochain heartbeat client.
pub async fn fermer_session_admin(
    cfg: &LivekitConfig,
    room_name: &str,
    motif_public: &str,
) -> Result<(), ApiErreur> {
    // 1. Diffuser l'évènement aux participants encore connectés
    let payload = serde_json::json!({
        "type": "admin",
        "subtype": "session_fermee",
        "motif_public": motif_public,
    });
    let _ = publier_evenement_moderation(cfg, room_name, &payload).await;

    // 2. Détruire la room LiveKit (déconnexion forcée de tous les pairs)
    if let Err(err) = room_client(cfg).delete_room(room_name).await {
        eprintln!(
            "[livekit_moderation] delete_room({}) a échoué : {}",
            room_name, err
        );
    }
    Ok(())
}

/// Coupe le micro d'un participant depuis le serveur (`MutePublishedTrack`).
///
/// Le mute est AUTORITAIRE : il s'applique au SFU, pas au bon vouloir du client
/// visé. Comme dans Meet, le participant reste libre de se réactiver ensuite, 
/// couper définitivement la parole relèverait de `can_publish`, pas d'un mute.
///
/// Retourne `true` si une piste micro active a bien été coupée. `false` couvre
/// les cas normaux (participant déconnecté, micro déjà coupé, aucune piste
/// publiée) : ce sont des non-évènements, pas des erreurs.
pub async fn couper_micro_participant(
    cfg: &LivekitConfig,
    room_name: &str,
    identity: &str,
) -> Result<bool, ApiErreur> {
    let client = room_client(cfg);

    let infos = match client.get_participant(room_name, identity).await {
        Ok(infos) => infos,
        Err(err) => {
            // Cas courant : le participant a quitté la room entre-temps.
            eprintln!(
                "[livekit_moderation] get_participant({}, {}) a échoué : {}",
                room_name, identity, err
            );
            return Ok(false);
        }
    };

    let Some(sid) = sid_micro_actif(&infos) else {
        return Ok(false);
    };

    if let Err(err) = client
        .mute_published_track(room_name, identity, &sid, true)
        .await
    {
        eprintln!(
            "[livekit_moderation] mute_published_track({}, {}, {}) a échoué : {}",
            room_name, identity, sid, err
        );
        return Ok(false);
    }
    Ok(true)
}

/// Coupe le micro de TOUS les participants de la room, sauf les identités
/// exclues (auteur de l'action et modérateurs protégés).
///
/// La liste des participants vient de LiveKit et non de Postgres : seuls comptent
/// ceux réellement connectés au SFU, et `session_participant` peut porter des
/// lignes obsolètes (déconnexion brutale sans `quitte_at`).
///
/// Retourne les identités effectivement coupées.
pub async fn couper_micros_room(
    cfg: &LivekitConfig,
    room_name: &str,
    exclusions: &[String],
) -> Result<Vec<String>, ApiErreur> {
    let client = room_client(cfg);

    let participants = match client.list_participants(room_name).await {
        Ok(p) => p,
        Err(err) => {
            eprintln!(
                "[livekit_moderation] list_participants({}) a échoué : {}",
                room_name, err
            );
            return Ok(Vec::new());
        }
    };

    let mut coupes = Vec::new();
    for infos in participants {
        let identity = infos.identity.clone();
        if exclusions.iter().any(|e| e == &identity) {
            continue;
        }
        let Some(sid) = sid_micro_actif(&infos) else {
            continue;
        };
        match client
            .mute_published_track(room_name, &identity, &sid, true)
            .await
        {
            Ok(_) => coupes.push(identity),
            Err(err) => eprintln!(
                "[livekit_moderation] mute_published_track({}, {}) a échoué : {}",
                room_name, identity, err
            ),
        }
    }
    Ok(coupes)
}

/// SID de la piste micro publiée et NON déjà coupée d'un participant.
/// `None` si le participant ne publie pas de micro ou l'a déjà coupé, inutile
/// alors d'appeler LiveKit.
fn sid_micro_actif(infos: &proto::ParticipantInfo) -> Option<String> {
    infos
        .tracks
        .iter()
        .find(|t| t.source == proto::TrackSource::Microphone as i32 && !t.muted)
        .map(|t| t.sid.clone())
}

/// Publie un DataPacket JSON sur le canal `RELIABLE` à tous les participants
/// de la room. Utilisé pour notifier en temps réel les mutations de modération
/// (permission_update, spotlight).
pub async fn publier_evenement_moderation(
    cfg: &LivekitConfig,
    room_name: &str,
    payload: &serde_json::Value,
) -> Result<(), ApiErreur> {
    publier_evenement(cfg, room_name, payload, "moderation").await
}

/// Publie un DataPacket JSON `RELIABLE` sur un `topic` donné.
///
/// Passer par le serveur plutôt que par le client émetteur est nécessaire dès
/// que l'émetteur n'a pas `can_publish_data` : c'est le cas de tout participant
/// ordinaire, à qui le token de session refuse le canal data (réservé au
/// tableau blanc). La clé API serveur, elle, n'est pas soumise à ce grant.
pub async fn publier_evenement(
    cfg: &LivekitConfig,
    room_name: &str,
    payload: &serde_json::Value,
    topic: &str,
) -> Result<(), ApiErreur> {
    let bytes = serde_json::to_vec(payload)
        .map_err(|e| ApiErreur::BaseDeDonnees(format!("Sérialisation DataPacket: {}", e)))?;

    let options = SendDataOptions {
        kind: proto::data_packet::Kind::Reliable,
        topic: Some(topic.to_string()),
        ..Default::default()
    };

    if let Err(err) = room_client(cfg).send_data(room_name, bytes, options).await {
        eprintln!(
            "[livekit_moderation] send_data({}, topic={}) a échoué : {}",
            room_name, topic, err
        );
    }
    Ok(())
}
