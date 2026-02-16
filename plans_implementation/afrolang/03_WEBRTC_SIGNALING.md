# Phase 3 — WebRTC Signaling & Visioconference

> **Statut** : `BLOQUE par Phase 2`
> **Progression** : 0/11 taches
> **Bloque par** : [Phase 2 — Frontend UI](./02_FRONTEND_UI.md) (toutes les pages et composants doivent etre crees)
> **Debloque** : [Phase 4 — Tableau blanc](./04_TABLEAU_BLANC.md)

---

## Contexte inter-phases

```
✅ = termine    🔄 = en cours    ⬜ = a faire    🔒 = bloque

[⬜] Phase 1 — Backend REST         (doit etre terminee)
[⬜] Phase 2 — Frontend UI          (doit etre terminee)
[🔒] Phase 3 — WebRTC Signaling     ◄── VOUS ETES ICI
[🔒] Phase 4 — Tableau blanc        (attend Phase 3 complete)
```

**Ce que les Phases precedentes fournissent :**

| Depuis | Element | Utilise ici pour |
|--------|---------|------------------|
| Phase 1, tache 1.7 | `POST /sessions/{id}/rejoindre` | Verifier acces + enregistrer participant avant de generer le token LiveKit |
| Phase 1, tache 1.7 | `PUT /sessions/{id}/demarrer` | Demarrer la session avant de creer la room LiveKit |
| Phase 1, tache 1.7 | `PUT /sessions/{id}/terminer` | Fermer la room LiveKit + sauvegarder les donnees |
| Phase 1, tache 1.7 | `POST /sessions/{id}/quitter` | Cleanup participant quand il quitte la room |
| Phase 2, tache 2.2 | `useAfrolang.ts` composable | Sera enrichi avec `genererTokenSession()` |
| Phase 2, tache 2.14 | `session/[id].vue` placeholder | Sera transformee en page visioconference complete |
| Phase 2, tache 2.9 | `SallePriveeJoinModal.vue` | Sera modifie pour appeler `genererTokenSession()` au lieu de `rejoindreSession()` |

**Ce que cette phase produit pour la Phase 4 :**
- `AfrolangRoom.vue` : conteneur de la visioconference, Phase 4 y ajoutera le panneau whiteboard
- `AfrolangSidebar.vue` : panneau lateral, Phase 4 ajoutera l'onglet "Tableau blanc"
- `AfrolangControls.vue` : barre de controles, Phase 4 activera le bouton "Tableau blanc"
- DataChannels LiveKit : Phase 4 les utilisera pour synchroniser les operations tldraw

---

## Progression

- [ ] **3.1** Deployer LiveKit en dev (Docker Compose + config)
- [ ] **3.2** Ajouter `livekit-api` a `Cargo.toml`
- [ ] **3.3** Ajouter variables env LiveKit a `config.rs`
- [ ] **3.4** Creer handler `generer_token_session` dans `handlers/afrolang.rs` (enrichit Phase 1)
- [ ] **3.5** Ajouter route `/sessions/{id}/token` dans `routes.rs` (enrichit Phase 1)
- [ ] **3.6** Installer `livekit-client` + `@livekit/components-vue` (pnpm)
- [ ] **3.7** Creer `app/components/afrolang/AfrolangRoom.vue`
- [ ] **3.8** Creer `app/components/afrolang/AfrolangVideoGrid.vue` + `AfrolangParticipantTile.vue`
- [ ] **3.9** Creer `app/components/afrolang/AfrolangControls.vue`
- [ ] **3.10** Creer `app/components/afrolang/AfrolangSidebar.vue`
- [ ] **3.11** Transformer `app/pages/afrolang/session/[id].vue` (remplacer placeholder Phase 2 par AfrolangRoom)

---

## 3.1 — Deployer LiveKit en dev

### Docker Compose

Ajouter au `docker-compose.yml` existant :

```yaml
  livekit:
    image: livekit/livekit-server:latest
    ports:
      - "7880:7880"   # WebSocket signaling
      - "7881:7881"   # HTTP API
      - "7882:7882"   # WebRTC TCP
      - "50000-50100:50000-50100/udp"  # WebRTC UDP
    environment:
      - LIVEKIT_KEYS=devkey: secret
    volumes:
      - ./livekit.yaml:/livekit.yaml
    command: --config /livekit.yaml --dev
```

### `livekit.yaml` (config minimale dev)

```yaml
port: 7880
rtc:
  tcp_port: 7882
  port_range_start: 50000
  port_range_end: 50100
  use_external_ip: false
keys:
  devkey: secret
```

### Production (plus tard)

En production, LiveKit devrait etre deploye sur un VPS separe avec :
- **coturn** pour TURN/STUN (traversee NAT)
- **Redis** pour le scaling multi-noeud
- Ports UDP ouverts (50000-60000)
- TLS (via reverse proxy nginx/caddy)

---

## 3.2–3.3 — Backend : dependance + config

### 3.2 — `Cargo.toml`

```toml
[dependencies]
livekit-api = "0.4"   # SDK Rust officiel LiveKit (generation tokens)
```

### 3.3 — `config.rs` — Variables d'environnement additionnelles

```rust
// Ajouter a la struct Config
pub livekit_url: String,        // LIVEKIT_URL (defaut: ws://localhost:7880)
pub livekit_api_key: String,    // LIVEKIT_API_KEY (defaut: devkey)
pub livekit_api_secret: String, // LIVEKIT_API_SECRET (defaut: secret)
```

```env
# .env — Ajouter
LIVEKIT_URL=ws://localhost:7880
LIVEKIT_API_KEY=devkey
LIVEKIT_API_SECRET=secret
```

---

## 3.4–3.5 — Backend : endpoint token

### 3.4 — Handler `generer_token_session`

> **Enrichit** `src/handlers/afrolang.rs` cree en Phase 1 (tache 1.5–1.8)

```rust
pub async fn generer_token_session(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    path: web::Path<Uuid>,          // session_id
    body: web::Json<RejoindreRequest>, // ← meme struct que Phase 1
) -> Result<HttpResponse, ApiErreur> {
    let session_id = path.into_inner();
    let user_id = extraire_utilisateur_id(&req)?;  // ← fonction existante Phase 1

    // 1. Charger la session (← reutilise fonction Phase 1)
    let session = charger_session(&pool, session_id).await?;

    // 2. Verifier que la session est en_cours
    if session.etat != "en_cours" {
        return Err(ApiErreur::BadRequest("Session non active".into()));
    }

    // 3. Verifier le code_acces (← meme logique que Phase 1, tache 1.7)
    let salle_privee = charger_salle_privee(&pool, session.salle_privee_id).await?;
    if let Some(code) = &salle_privee.code_acces {
        let code_fourni = body.code_acces.as_deref().unwrap_or("");
        if code_fourni != code {
            return Err(ApiErreur::NonAutorise("Code d'acces incorrect".into()));
        }
    }

    // 4. Verifier max_participants
    let nb_participants = compter_participants_actifs(&pool, session_id).await?;
    if nb_participants >= session.max_participants.unwrap_or(50) as i64 {
        return Err(ApiErreur::BadRequest("Session complete".into()));
    }

    // 5. Generer le token LiveKit (← NOUVEAU Phase 3)
    let user = charger_utilisateur(&pool, user_id).await?;
    let room_name = format!("afrolang-{}", session_id);
    let is_moderator = session.moderateur_id == Some(user_id);

    let token = livekit_api::access_token::AccessToken::with_api_key(
        &config.livekit_api_key,
        &config.livekit_api_secret,
    )
    .with_identity(&user_id.to_string())
    .with_name(&format!("{} {}", user.prenom.unwrap_or_default(), user.nom))
    .with_grants(livekit_api::access_token::VideoGrants {
        room_join: true,
        room: room_name.clone(),
        can_publish: true,
        can_subscribe: true,
        can_publish_data: true,    // pour le tableau blanc (Phase 4)
        ..Default::default()
    })
    .to_jwt()?;

    // 6. Enregistrer le participant (← reutilise fonction Phase 1)
    enregistrer_participant(&pool, session_id, user_id, is_moderator).await?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "token": token,
            "room_name": room_name,
            "livekit_url": config.livekit_url,
            "is_moderator": is_moderator,
        })),
        error: None,
    }))
}
```

### 3.5 — Route

Ajouter dans le scope `/afrolang` de `routes.rs` (sous les routes sessions Phase 1) :

```rust
// ── Phase 3 : Token LiveKit ──
.route("/sessions/{id}/token", web::post().to(afrolang::generer_token_session))
```

---

## 3.6 — Frontend : dependances

```bash
cd uafricas_frontend && pnpm add livekit-client @livekit/components-vue
```

---

## 3.7–3.10 — Frontend : composants WebRTC

### 3.7 — `AfrolangRoom.vue`

> **Ce composant sera enrichi en Phase 4** pour integrer `AfrolangWhiteboard.vue`

```vue
<template>
  <div class="flex h-screen bg-base-300">
    <!-- Zone video principale -->
    <div class="flex-1 flex flex-col">
      <!-- Header -->
      <div class="navbar bg-base-200">
        <span class="text-lg font-bold">{{ session.titre }}</span>
        <span class="badge badge-error animate-pulse ml-2">En direct</span>
        <span class="ml-auto font-mono">{{ dureeFormatee }}</span>
      </div>

      <!-- Grille video (3.8) -->
      <AfrolangVideoGrid :participants="participants" />

      <!-- Controles (3.9) -->
      <AfrolangControls
        :micro-actif="microActif"
        :camera-active="cameraActive"
        :ecran-partage="ecranPartage"
        :est-moderateur="estModerateur"
        @toggle-micro="toggleMicro"
        @toggle-camera="toggleCamera"
        @toggle-ecran="toggleEcranPartage"
        @toggle-tableau-blanc="toggleTableauBlanc"
        @quitter="quitterSession"
        @terminer="terminerSession"
      />
      <!-- NOTE: @toggle-tableau-blanc sera fonctionnel en Phase 4 -->
    </div>

    <!-- Sidebar (3.10) — Phase 4 ajoutera l'onglet whiteboard ici -->
    <AfrolangSidebar
      v-if="sidebarOuverte"
      :participants="participants"
      :session-id="sessionId"
    />
  </div>
</template>
```

### 3.8 — `AfrolangVideoGrid.vue` + `AfrolangParticipantTile.vue`

Layout adaptatif :
```
1 participant:    2 participants:    3-4 participants:    5+ : grid auto-fit
┌──────────┐     ┌─────┬─────┐     ┌─────┬─────┐       ┌────┬────┬────┐
│           │     │     │     │     │     │     │       │    │    │    │
│   100%    │     │ 50% │ 50% │     │     │     │       ├────┼────┼────┤
│           │     │     │     │     ├─────┼─────┤       │    │    │    │
└──────────┘     └─────┴─────┘     │     │     │       └────┴────┴────┘
                                    └─────┴─────┘
```

### 3.9 — `AfrolangControls.vue`

```
┌──────────────────────────────────────────────────────┐
│  Micro  Camera  Ecran  [WB]  Participants  │ Terminer │ Quitter │
└──────────────────────────────────────────────────────┘
```

- **[WB]** : bouton "Tableau blanc" — present mais **desactive** en Phase 3 (sera active en Phase 4)
- **Terminer** : visible seulement pour le moderateur

### 3.10 — `AfrolangSidebar.vue`

Panneau lateral avec liste des participants.

> **Phase 4 ajoutera** un onglet "Tableau blanc" avec `AfrolangWhiteboard.vue`

---

## 3.11 — Transformer la page session

> **Modifie** `app/pages/afrolang/session/[id].vue` cree en Phase 2 (tache 2.14)

La page passe de "placeholder metadonnees" a "salle de visioconference" :

```vue
<script setup>
// ── Reutilise le composable Phase 2 ──
import { obtenirSession } from '~/composables/useAfrolang'

const route = useRoute()
const { data: session } = await useFetch(`/api/afrolang/sessions/${route.params.id}`)

// ── NOUVEAU Phase 3 : gestion token LiveKit ──
const token = ref<string | null>(null)
const roomName = ref<string>('')
const livekitUrl = ref<string>('')

async function rejoindre(codeAcces?: string) {
  // Appelle le NOUVEL endpoint Phase 3
  const response = await $fetch(`/api/afrolang/sessions/${route.params.id}/token`, {
    method: 'POST',
    body: { code_acces: codeAcces },
    headers: { Authorization: `Bearer ${accessToken}` },
  })
  token.value = response.data.token
  roomName.value = response.data.room_name
  livekitUrl.value = response.data.livekit_url
}
</script>

<template>
  <!-- Mode conference (token obtenu) — NOUVEAU Phase 3 -->
  <AfrolangRoom
    v-if="token"
    :token="token"
    :room-name="roomName"
    :livekit-url="livekitUrl"
    :session="session"
    @quitter="onQuitter"
  />

  <!-- Mode preview (placeholder Phase 2, conserve) -->
  <div v-else>
    <!-- Metadonnees session + bouton Rejoindre → SallePriveeJoinModal -->
  </div>
</template>
```

### Enrichir le composable `useAfrolang.ts`

Ajouter la fonction token (Phase 2, tache 2.2 mentionnait deja cet emplacement) :

```typescript
// ── Phase 3 : generation token LiveKit ──
export interface TokenResponse {
  token: string
  room_name: string
  livekit_url: string
  is_moderator: boolean
}

export async function genererTokenSession(sessionId: string, codeAcces?: string): Promise<TokenResponse> {
  const userStore = useUserStore()
  const response = await $fetch<{ data: TokenResponse }>(`/api/afrolang/sessions/${sessionId}/token`, {
    method: 'POST',
    body: { code_acces: codeAcces },
    headers: { Authorization: `Bearer ${userStore.accessToken}` },
  })
  return response.data
}
```

---

## Evenements WebSocket geres par le frontend

| Evenement | Action |
|-----------|--------|
| `ParticipantConnected` | Ajouter a la grille video |
| `ParticipantDisconnected` | Retirer de la grille, appeler `quitterSession` API (Phase 1) |
| `TrackSubscribed` | Attacher le flux video/audio |
| `TrackUnsubscribed` | Detacher le flux |
| `DataReceived` | **Phase 4** : operations tableau blanc |
| `RoomDisconnected` | Rediriger vers la page salle privee (Phase 2) |

## Gestion de la perte de connexion

```
Deconnexion detectee
  → Afficher "Reconnexion en cours..."
  → Tenter reconnexion automatique (3 tentatives, backoff exponentiel)
  → Si echec → Afficher "Connexion perdue" + bouton "Revenir a la salle"
```

---

## Pourquoi LiveKit ?

| Critere | LiveKit | Janus | mediasoup |
|---------|---------|-------|-----------|
| Auto-hebergeable | Oui (Go binary) | Oui | Oui (Node.js) |
| SDK Client | JS, React, Vue, Swift, Kotlin | JS seulement | JS + wrapping |
| SDK Serveur | Rust, Go, Python, Node.js | REST API | Node.js |
| Scaling | Natif multi-noeud (Redis) | Manuel | Manuel |
| DataChannels | Oui (pour Phase 4 whiteboard) | N/A | N/A |
| Complexite | Faible | Elevee | Moyenne |

### Alternative rapide : Jitsi Meet (iframe)

Si LiveKit est trop lourd initialement :

```vue
<iframe
  :src="`https://meet.jit.si/${roomName}?jwt=${token}`"
  allow="camera; microphone; display-capture"
  style="width: 100%; height: 100vh; border: none;"
/>
```

---

## Recapitulatif fichiers

### Fichiers a creer (6)
| Fichier | Tache |
|---------|-------|
| `livekit.yaml` | 3.1 |
| `app/components/afrolang/AfrolangRoom.vue` | 3.7 |
| `app/components/afrolang/AfrolangVideoGrid.vue` | 3.8 |
| `app/components/afrolang/AfrolangParticipantTile.vue` | 3.8 |
| `app/components/afrolang/AfrolangControls.vue` | 3.9 |
| `app/components/afrolang/AfrolangSidebar.vue` | 3.10 |

### Fichiers a modifier (7)
| Fichier | Modification | Phase d'origine |
|---------|-------------|-----------------|
| `docker-compose.yml` | Ajouter service LiveKit | — |
| `uafricas_backend/Cargo.toml` | Ajouter `livekit-api` | — |
| `uafricas_backend/src/config.rs` | Variables LiveKit | — |
| `uafricas_backend/src/handlers/afrolang.rs` | Ajouter `generer_token_session` | Phase 1 |
| `uafricas_backend/src/routes.rs` | Ajouter route `/sessions/{id}/token` | Phase 1 |
| `app/composables/useAfrolang.ts` | Ajouter `genererTokenSession()` | Phase 2 |
| `app/pages/afrolang/session/[id].vue` | Integrer `AfrolangRoom` | Phase 2 |

---

## Critere de completion Phase 3

> **La Phase 4 peut commencer UNIQUEMENT quand :**
> - [ ] Tous les 11 points de la progression sont coches
> - [ ] LiveKit tourne en local (`docker compose up` sans erreur)
> - [ ] Le token est genere correctement (`POST /sessions/{id}/token`)
> - [ ] La visioconference fonctionne (2 navigateurs connectes se voient/entendent)
> - [ ] Les controles marchent (micro, camera, partage d'ecran)
> - [ ] Rejoindre/quitter met a jour les participants dans la base
> - [ ] Le bouton "Tableau blanc" est present mais desactive (pret pour Phase 4)
>
> Quand c'est fait → mettre le statut a `TERMINE` dans [00_OVERVIEW.md](./00_OVERVIEW.md) et debloquer Phase 4.
