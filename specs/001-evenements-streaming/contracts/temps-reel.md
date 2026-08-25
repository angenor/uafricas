# Contrat temps réel : DataPackets LiveKit & SSE

Deux canaux temps réel coexistent :
1. **DataPackets LiveKit** (in-room, client↔client via SFU) : chat, réactions, lever-la-main, modération. Éphémères, non stockés.
2. **SSE messagerie** (serveur→client, hors room) : notification de démarrage du direct, pour rafraîchir la page événement et la cloche.

---

## 1. DataPackets LiveKit (canal `Reliable`)

Émis/reçus via `room.localParticipant.publishData(...)` (client) ou `livekit_moderation::publier_evenement_moderation` (backend). Décodés via `TextDecoder` → `JSON.parse`. Un client **ignore ses propres paquets** (compare `participant.identity`). Parité avec les payloads afrolang existants.

### Chat (client → room, tous, `can_publish_data:true`)
```json
{ "type": "chat", "message": "Bonjour à tous", "identite": "<uuid>", "nom": "Awa Diop", "ts": 1717245000000 }
```
Affiché par `EvenementDirectChat.vue`. Le message est échappé par Vue (anti-XSS, Principe IV). Aucun stockage (FR-007, éphémère).

### Réaction (client → room, tous)
```json
{ "type": "reaction", "emoji": "👏", "identite": "<uuid>", "ts": 1717245000000 }
```
Joué en overlay éphémère par `EvenementDirectReactions.vue` (réutilise le pattern `AfrolangReactionsOverlay`).

### Lever la main (DataPacket informatif + endpoint backend)
Le spectateur appelle `POST …/direct/lever-main` ; le backend diffuse :
```json
{ "type": "moderation", "subtype": "main_levee", "payload": { "utilisateur_id": "<uuid>", "nom": "Awa Diop", "levee": true } }
```
L'organisateur met à jour sa liste de demandes en direct (et la retrouve via `GET …/direct` → `demandes_parole`).

### Modération (backend → room)
```json
{ "type": "moderation", "subtype": "role_update", "payload": { "utilisateur_id": "<uuid>", "role": "intervenant" } }
{ "type": "moderation", "subtype": "retire",      "payload": { "utilisateur_id": "<uuid>" } }
{ "type": "admin",      "subtype": "session_fermee", "motif_public": "Le direct est terminé." }
```
- `role_update` : le client **destinataire** (compare `utilisateur_id` à son identité) active/désactive ses contrôles caméra/micro en conséquence.
- `retire` : le client destinataire se déconnecte et revient sur la page événement.
- `session_fermee` : émis par `fermer_session_admin` (réutilisé) ; tous les clients quittent.

---

## 2. SSE messagerie (serveur → client)

Émis via `RegistreSse::publier(inscrit_id, &payload)` à l'ouverture du direct, à chaque inscrit. Format générique (parité `rdv_*`), sans contenu sensible :
```json
{ "type": "event_stream_demarre", "evenement_id": "<uuid>" }
```

**Dispatch frontend** : branche à ajouter dans `app/plugins/messagerie.client.ts` :
```ts
} else if (typeof evt?.type === 'string' && evt.type.startsWith('event_stream_')) {
  gererEvenementStream(evt)   // useEvenements : rafraîchit l'état du direct si la page concernée est ouverte
  compteurNonLues()           // rafraîchit le badge cloche
}
```

**Cloche persistante** : en parallèle du SSE, `creer_notification(pool, inscrit_id, "evenement_direct_demarre", "Le direct de « <titre> » a commencé", Some("/evenements/<id>"))`. Type affiché avec icône/couleur par défaut (extension optionnelle de `app/mocks/notifications.ts` pour une icône `video` dédiée).

---

## Grants LiveKit par rôle (rappel, cf. data-model & D2)

| Rôle | room_join | can_publish | can_subscribe | can_publish_data |
|------|-----------|-------------|---------------|------------------|
| organisateur | ✅ | ✅ | ✅ | ✅ |
| intervenant | ✅ | ✅ | ✅ | ✅ |
| spectateur | ✅ | ❌ | ✅ | ✅ |

`can_publish_data: true` pour tous → chat / réactions / lever-la-main fonctionnent quel que soit le rôle, sans diffusion média non sollicitée.
