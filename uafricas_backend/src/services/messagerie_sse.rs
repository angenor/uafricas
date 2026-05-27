// Service de diffusion temps réel (SSE) pour la messagerie privée.
//
// Registre de connexions en mémoire (mono-instance, Décision 2 de research.md) :
// HashMap<utilisateur_id, Vec<Sender>>. Chaque connexion SSE ajoute un sender ;
// `publier` diffuse à toutes les connexions d'un utilisateur et retire au passage
// les connexions fermées (nettoyage paresseux). Aucune nouvelle dépendance :
// tokio (mpsc, time) et futures-util sont déjà présents.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::web::Bytes;
use futures_util::Stream;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

/// Intervalle de keep-alive (commentaire SSE) pour maintenir la connexion ouverte
/// à travers les proxies. Inférieur aux timeouts usuels (60 s).
const KEEP_ALIVE: Duration = Duration::from_secs(20);

/// Registre partagé des connexions SSE. Cloner partage le même état (Arc interne).
#[derive(Clone, Default)]
pub struct RegistreSse {
    inner: Arc<Mutex<HashMap<Uuid, Vec<UnboundedSender<String>>>>>,
}

impl RegistreSse {
    pub fn new() -> Self {
        Self::default()
    }

    /// Abonne une nouvelle connexion pour `utilisateur_id`. Renvoie le récepteur
    /// dont le flux sera streamé au client.
    pub fn abonner(&self, utilisateur_id: Uuid) -> UnboundedReceiver<String> {
        let (tx, rx) = unbounded_channel::<String>();
        let mut map = self.inner.lock().unwrap();
        map.entry(utilisateur_id).or_default().push(tx);
        rx
    }

    /// Publie un évènement (sérialisé en JSON) à toutes les connexions d'un
    /// utilisateur. Les connexions fermées sont retirées au passage.
    pub fn publier<T: serde::Serialize>(&self, utilisateur_id: Uuid, evenement: &T) {
        let json = match serde_json::to_string(evenement) {
            Ok(j) => j,
            Err(e) => {
                log::error!("Sérialisation évènement SSE échouée : {}", e);
                return;
            }
        };
        let mut map = self.inner.lock().unwrap();
        if let Some(senders) = map.get_mut(&utilisateur_id) {
            senders.retain(|tx| tx.send(json.clone()).is_ok());
            if senders.is_empty() {
                map.remove(&utilisateur_id);
            }
        }
    }
}

/// Construit le flux SSE à partir d'un récepteur : chaque message JSON devient
/// une trame `data: <json>\n\n` ; en l'absence de message pendant `KEEP_ALIVE`,
/// une trame commentaire `: keep-alive` est émise pour garder la connexion vivante.
pub fn flux_sse(
    mut rx: UnboundedReceiver<String>,
) -> impl Stream<Item = Result<Bytes, actix_web::Error>> {
    // Trame initiale pour ouvrir le flux immédiatement côté client.
    let initial = futures_util::stream::once(async {
        Ok::<_, actix_web::Error>(Bytes::from_static(b": connected\n\n"))
    });

    let corps = futures_util::stream::poll_fn(move |cx| {
        use std::task::Poll;
        // Sonde non bloquante du canal, sinon arme le timer de keep-alive.
        match rx.poll_recv(cx) {
            Poll::Ready(Some(msg)) => {
                Poll::Ready(Some(Ok(Bytes::from(format!("data: {msg}\n\n")))))
            }
            Poll::Ready(None) => Poll::Ready(None), // canal fermé → fin du flux
            Poll::Pending => Poll::Pending,
        }
    });

    // Keep-alive périodique fusionné au flux des messages.
    let pings = futures_util::stream::unfold((), |()| async {
        tokio::time::sleep(KEEP_ALIVE).await;
        Some((
            Ok::<_, actix_web::Error>(Bytes::from_static(b": keep-alive\n\n")),
            (),
        ))
    });

    use futures_util::StreamExt;
    initial.chain(futures_util::stream::select(corps, pings))
}
