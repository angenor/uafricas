# Phase 4 — Tableau blanc interactif

> **Statut** : `TERMINE`
> **Progression** : 9/9 taches
> **Bloque par** : ~~Phase 3~~ (terminee)
> **Debloque** : Rien (derniere phase)

---

## Contexte inter-phases

```
✅ = termine    🔄 = en cours    ⬜ = a faire    🔒 = bloque

[✅] Phase 1 — Backend REST         (terminee)
[✅] Phase 2 — Frontend UI          (terminee)
[✅] Phase 3 — WebRTC Signaling     (terminee)
[✅] Phase 4 — Tableau blanc        ◄── TERMINE
```

**Ce que les Phases precedentes fournissent :**

| Depuis | Element | Utilise ici pour |
|--------|---------|------------------|
| Phase 1, tache 1.2 | Table `afrolang.tableau_blanc` (schema SQL) | Stocker les snapshots JSONB |
| Phase 1, tache 1.7 | Handler `obtenir_session` | Verifier `tableau_blanc_actif` |
| Phase 2, tache 2.2 | `useAfrolang.ts` composable | Sera enrichi avec `obtenirTableauBlanc()`, `sauvegarderTableauBlanc()`, `effacerTableauBlanc()` |
| Phase 3, tache 3.7 | `AfrolangRoom.vue` | Sera modifie pour integrer `AfrolangWhiteboard.vue` en split-screen |
| Phase 3, tache 3.9 | `AfrolangControls.vue` | Le bouton "Tableau blanc" (desactive en Phase 3) sera active |
| Phase 3, tache 3.10 | `AfrolangSidebar.vue` | Recevra l'onglet "Tableau blanc" |
| Phase 3 | DataChannels LiveKit (`can_publish_data: true` dans le token) | Synchronisation temps reel des operations tldraw entre participants |

**Ce que cette phase produit :**
- Tableau blanc collaboratif fonctionnel pendant les sessions de visioconference
- Persistance des dessins (snapshot JSONB toutes les 30s + a la fermeture)
- Restauration a la reconnexion

---

## Progression

- [x] **4.1** Creer la mini-app tldraw (package `whiteboard/`)
- [x] **4.2** Builder et servir la mini-app en static
- [x] **4.3** Creer handler `obtenir_tableau_blanc` dans `handlers/afrolang.rs` (enrichit Phase 1)
- [x] **4.4** Creer handler `sauvegarder_tableau_blanc` dans `handlers/afrolang.rs`
- [x] **4.5** Creer handler `effacer_tableau_blanc` dans `handlers/afrolang.rs`
- [x] **4.6** Ajouter routes tableau blanc dans `routes.rs` (enrichit Phase 1)
- [x] **4.7** Creer `app/components/afrolang/AfrolangWhiteboard.vue`
- [x] **4.8** Modifier `AfrolangRoom.vue` + `AfrolangControls.vue` + `AfrolangSidebar.vue` (enrichit Phase 3)
- [x] **4.9** Enrichir `useAfrolang.ts` avec fonctions tableau blanc (enrichit Phase 2)

---

## 4.1 — Choix technologique : tldraw

| Critere | tldraw | Excalidraw | Fabric.js (custom) |
|---------|--------|------------|-------------------|
| Open source | Oui (Apache 2.0) | Oui (MIT) | Oui (MIT) |
| Framework | React (wrapper Vue possible) | React | Vanilla JS |
| Collaboration temps reel | tldraw sync (Yjs) | Yjs plugin | A implementer |
| Export JSONB | Oui (snapshot natif) | Oui (JSON) | Oui |
| Outils dessin | Tres riche | Riche | De base |
| Complexite integration | Moyenne | Moyenne | Elevee |

**Recommandation** : **tldraw** via iframe, avec synchronisation via les **DataChannels LiveKit** (configures en Phase 3 avec `can_publish_data: true`).

### Architecture de synchronisation

```
Participant A                    LiveKit SFU                   Participant B
┌──────────┐                   ┌──────────┐                   ┌──────────┐
│ tldraw   │── DataChannel ──►│ Forward  │── DataChannel ──►│ tldraw   │
│ (dessine)│   (Phase 3)      │ (relay)  │   (Phase 3)      │ (affiche)│
└──────────┘                   └──────────┘                   └──────────┘
                                    │
                                    │ Snapshot periodique (toutes les 30s)
                                    ▼
                              ┌──────────┐
                              │PostgreSQL │
                              │ afrolang. │
                              │ tableau_  │
                              │ blanc     │
                              │ (Phase 1) │
                              └──────────┘
```

---

## 4.1–4.2 — Mini-app tldraw (package separe)

### 4.1 — Structure du package

> tldraw est React. Pour l'integrer dans Vue 3/Nuxt, on cree une mini-app React servie en iframe.

```
whiteboard/
├── package.json     # React + tldraw
├── src/
│   └── App.tsx      # Composant tldraw avec bridge postMessage
├── index.html       # Entry point Vite
├── vite.config.ts   # Config build
└── dist/            # Build static → copie dans uafricas_frontend/public/whiteboard/
```

### `whiteboard/src/App.tsx`

```tsx
import { Tldraw, createTLStore } from 'tldraw'
import 'tldraw/tldraw.css'

export default function WhiteboardApp() {
  const store = createTLStore()

  // Recevoir les operations du parent Vue (relayees depuis DataChannel LiveKit Phase 3)
  useEffect(() => {
    window.addEventListener('message', (event) => {
      if (event.data?.type === 'apply-operation') {
        store.mergeRemoteChanges(() => {
          // Appliquer les changements distants
        })
      }
      if (event.data?.type === 'load-snapshot') {
        // Charger un snapshot existant (restauration depuis Phase 1 BDD)
        store.loadSnapshot(event.data.snapshot)
      }
      if (event.data?.type === 'clear') {
        store.clear()
      }
    })
  }, [])

  // Envoyer les operations locales au parent Vue
  store.listen((entry) => {
    window.parent.postMessage({
      type: 'tldraw-operation',
      payload: entry.changes,
    }, '*')
  })

  // Repondre aux demandes de snapshot (pour sauvegarde periodique)
  window.addEventListener('message', (event) => {
    if (event.data?.type === 'get-snapshot') {
      window.parent.postMessage({
        type: 'tldraw-snapshot',
        payload: store.getSnapshot(),
      }, '*')
    }
  })

  return (
    <div style={{ width: '100%', height: '100vh' }}>
      <Tldraw store={store} />
    </div>
  )
}
```

### 4.2 — Build et servir

```bash
cd whiteboard && pnpm build
# Copier dist/ → uafricas_frontend/public/whiteboard/
cp -r dist/* ../uafricas_frontend/public/whiteboard/
```

L'iframe pointera vers `/whiteboard/index.html` (servi en static par Nuxt).

---

## 4.3–4.6 — Backend : endpoints tableau blanc

> **Enrichissent** `src/handlers/afrolang.rs` cree en Phase 1

### 4.3 — `obtenir_tableau_blanc`

```rust
// GET /api/afrolang/sessions/{id}/tableau-blanc
pub async fn obtenir_tableau_blanc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = path.into_inner();
    // Verifier que la session existe (reutilise charger_session de Phase 1)
    let _session = charger_session(&pool, session_id).await?;

    let row = sqlx::query_as::<_, (serde_json::Value, i32)>(
        "SELECT donnees, version FROM afrolang.tableau_blanc WHERE session_id = $1"
    )
    .bind(session_id)
    .fetch_optional(pool.get_ref())
    .await?;

    match row {
        Some((donnees, version)) => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "donnees": donnees, "version": version })),
            error: None,
        })),
        None => Ok(HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(serde_json::json!({ "donnees": {}, "version": 0 })),
            error: None,
        })),
    }
}
```

### 4.4 — `sauvegarder_tableau_blanc`

```rust
// PUT /api/afrolang/sessions/{id}/tableau-blanc
pub async fn sauvegarder_tableau_blanc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = path.into_inner();
    let user_id = extraire_utilisateur_id(&req)?;  // ← Phase 1

    // Verifier moderateur (reutilise charger_session de Phase 1)
    let session = charger_session(&pool, session_id).await?;
    if session.moderateur_id != Some(user_id) {
        return Err(ApiErreur::NonAutorise("Seul le moderateur peut sauvegarder".into()));
    }

    // UPSERT (table Phase 1 : afrolang.tableau_blanc)
    sqlx::query(
        "INSERT INTO afrolang.tableau_blanc (session_id, donnees, version)
         VALUES ($1, $2, 1)
         ON CONFLICT (session_id)
         DO UPDATE SET donnees = $2, version = afrolang.tableau_blanc.version + 1, updated_at = NOW()"
    )
    .bind(session_id)
    .bind(&body.0)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some("ok"), error: None }))
}
```

### 4.5 — `effacer_tableau_blanc`

```rust
// DELETE /api/afrolang/sessions/{id}/tableau-blanc
pub async fn effacer_tableau_blanc(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiErreur> {
    let session_id = path.into_inner();
    let user_id = extraire_utilisateur_id(&req)?;  // ← Phase 1

    let session = charger_session(&pool, session_id).await?;
    if session.moderateur_id != Some(user_id) {
        return Err(ApiErreur::NonAutorise("Seul le moderateur peut effacer".into()));
    }

    sqlx::query(
        "UPDATE afrolang.tableau_blanc SET donnees = '{}', version = version + 1, updated_at = NOW() WHERE session_id = $1"
    )
    .bind(session_id)
    .execute(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(ApiResponse { success: true, data: Some("ok"), error: None }))
}
```

### 4.6 — Routes

Ajouter dans le scope `/afrolang` de `routes.rs` (sous la route token Phase 3) :

```rust
// ── Phase 4 : Tableau blanc ──
.route("/sessions/{id}/tableau-blanc", web::get().to(afrolang::obtenir_tableau_blanc))
.route("/sessions/{id}/tableau-blanc", web::put().to(afrolang::sauvegarder_tableau_blanc))
.route("/sessions/{id}/tableau-blanc", web::delete().to(afrolang::effacer_tableau_blanc))
```

---

## 4.7 — `AfrolangWhiteboard.vue`

```vue
<template>
  <div class="flex flex-col h-full bg-white rounded-lg overflow-hidden">
    <!-- Barre d'outils -->
    <div class="flex items-center gap-2 p-2 bg-base-200 border-b">
      <span class="font-semibold text-sm">Tableau blanc</span>
      <div class="flex-1" />
      <button
        v-if="estModerateur"
        class="btn btn-xs btn-ghost"
        @click="effacerTout"
      >
        Effacer tout
      </button>
      <button class="btn btn-xs btn-ghost" @click="$emit('fermer')">
        Fermer
      </button>
    </div>

    <!-- Zone de dessin tldraw (iframe → mini-app Phase 4.1-4.2) -->
    <iframe
      ref="whiteboardFrame"
      src="/whiteboard/index.html"
      class="flex-1 w-full border-0"
      allow="clipboard-read; clipboard-write"
    />
  </div>
</template>

<script setup lang="ts">
/**
 * Bridge de communication :
 * 1. iframe tldraw ←→ Ce composant Vue (postMessage)
 * 2. Ce composant Vue ←→ Autres participants (DataChannel LiveKit, Phase 3)
 * 3. Ce composant Vue ←→ Backend (API REST, Phase 1)
 */

const props = defineProps<{
  sessionId: string
  estModerateur: boolean
  room: any  // Room LiveKit (Phase 3)
}>()

const whiteboardFrame = ref<HTMLIFrameElement | null>(null)

// ── Recevoir operations des AUTRES participants via DataChannel (Phase 3) ──
props.room.on('dataReceived', (data: Uint8Array, participant: any) => {
  const message = JSON.parse(new TextDecoder().decode(data))
  if (message.type === 'whiteboard') {
    // Relayer a l'iframe tldraw
    whiteboardFrame.value?.contentWindow?.postMessage({
      type: 'apply-operation',
      payload: message.payload,
    }, '*')
  }
})

// ── Recevoir operations LOCALES depuis l'iframe tldraw ──
onMounted(() => {
  window.addEventListener('message', handleTldrawMessage)
  // Charger le snapshot existant depuis le backend (Phase 1)
  chargerSnapshot()
})

onUnmounted(() => {
  window.removeEventListener('message', handleTldrawMessage)
  if (snapshotInterval) clearInterval(snapshotInterval)
})

function handleTldrawMessage(event: MessageEvent) {
  if (event.data?.type === 'tldraw-operation') {
    // Diffuser aux autres via DataChannel LiveKit (Phase 3)
    const encoder = new TextEncoder()
    props.room.localParticipant.publishData(
      encoder.encode(JSON.stringify({
        type: 'whiteboard',
        payload: event.data.payload,
      }))
    )
  }
  if (event.data?.type === 'tldraw-snapshot') {
    // Sauvegarder le snapshot recu (reponse a 'get-snapshot')
    sauvegarderTableauBlanc(props.sessionId, event.data.payload)
  }
}

// ── Charger snapshot existant ──
async function chargerSnapshot() {
  const { donnees, version } = await obtenirTableauBlanc(props.sessionId) // ← Phase 2 enrichi (tache 4.9)
  if (version > 0) {
    whiteboardFrame.value?.contentWindow?.postMessage({
      type: 'load-snapshot',
      snapshot: donnees,
    }, '*')
  }
}

// ── Snapshot periodique (moderateur seulement, toutes les 30s) ──
let snapshotInterval: ReturnType<typeof setInterval> | null = null
if (props.estModerateur) {
  snapshotInterval = setInterval(() => {
    whiteboardFrame.value?.contentWindow?.postMessage({ type: 'get-snapshot' }, '*')
  }, 30_000)
}

// ── Effacer tout (moderateur) ──
async function effacerTout() {
  whiteboardFrame.value?.contentWindow?.postMessage({ type: 'clear' }, '*')
  await effacerTableauBlanc(props.sessionId) // ← Phase 2 enrichi (tache 4.9)
}
</script>
```

---

## 4.8 — Modifications composants Phase 3

### `AfrolangRoom.vue` — Ajouter mode split-screen

```
Mode normal (Phase 3):              Mode tableau blanc (Phase 4):
┌──────────────┐                    ┌────────┬────────┐
│              │                    │        │Tableau │
│   Video      │                    │ Video  │ blanc  │
│   Grid       │                    │ Grid   │(tldraw)│
│              │                    │        │        │
├──────────────┤                    ├────────┴────────┤
│  Controles   │                    │    Controles    │
└──────────────┘                    └─────────────────┘
```

Ajout : `v-if="tableauBlancOuvert"` pour afficher `AfrolangWhiteboard` en cote-a-cote.

### `AfrolangControls.vue` — Activer le bouton WB

Le bouton "Tableau blanc" etait present mais desactive en Phase 3 → l'activer maintenant.

### `AfrolangSidebar.vue` — Ajouter onglet optionnel

Ajouter un onglet "Tableau blanc" dans le panneau lateral (alternative au split-screen).

---

## 4.9 — Enrichir `useAfrolang.ts`

> **Modifie** le composable cree en Phase 2 (tache 2.1–2.2), deja enrichi en Phase 3 (tache 3.11)

```typescript
// ── Phase 4 : Fonctions tableau blanc ──

export interface TableauBlancData {
  donnees: Record<string, any>
  version: number
}

export async function obtenirTableauBlanc(sessionId: string): Promise<TableauBlancData> {
  const userStore = useUserStore()
  const response = await $fetch<{ data: TableauBlancData }>(
    `/api/afrolang/sessions/${sessionId}/tableau-blanc`,
    { headers: { Authorization: `Bearer ${userStore.accessToken}` } }
  )
  return response.data
}

export async function sauvegarderTableauBlanc(sessionId: string, donnees: any): Promise<void> {
  const userStore = useUserStore()
  await $fetch(`/api/afrolang/sessions/${sessionId}/tableau-blanc`, {
    method: 'PUT',
    body: donnees,
    headers: { Authorization: `Bearer ${userStore.accessToken}` },
  })
}

export async function effacerTableauBlanc(sessionId: string): Promise<void> {
  const userStore = useUserStore()
  await $fetch(`/api/afrolang/sessions/${sessionId}/tableau-blanc`, {
    method: 'DELETE',
    headers: { Authorization: `Bearer ${userStore.accessToken}` },
  })
}
```

---

## Persistance et restauration

### Sauvegarde automatique
1. **Toutes les 30 secondes** : le moderateur envoie le snapshot via `PUT /tableau-blanc` (tache 4.4)
2. **A la fermeture de session** : snapshot final sauvegarde (hook `beforeUnmount` + `terminerSession`)
3. **Versioning** : `version` est incremente a chaque sauvegarde (table Phase 1)

### Restauration
- Quand un participant rejoint une session en cours → `GET /tableau-blanc` (tache 4.3) pour charger le dernier snapshot
- Quand le moderateur ouvre le tableau blanc → charge le snapshot existant et reprend

---

## Considerations de performance

1. **Taille JSONB** : Limiter a 5MB par snapshot. Les snapshots tldraw sont generalement < 1MB.
2. **Frequence de sauvegarde** : 30 secondes — bon compromis securite/charge DB.
3. **DataChannel vs WebSocket** : On reutilise les DataChannels LiveKit (Phase 3) → pas de serveur WebSocket supplementaire.
4. **Compression** : Compresser les operations via DataChannel si necessaire (pako/gzip).

---

## Recapitulatif fichiers

### Fichiers a creer (5)
| Fichier | Tache |
|---------|-------|
| `whiteboard/package.json` | 4.1 |
| `whiteboard/src/App.tsx` | 4.1 |
| `whiteboard/index.html` | 4.1 |
| `whiteboard/vite.config.ts` | 4.1 |
| `app/components/afrolang/AfrolangWhiteboard.vue` | 4.7 |

### Fichiers a modifier (5)
| Fichier | Modification | Phase d'origine |
|---------|-------------|-----------------|
| `uafricas_backend/src/handlers/afrolang.rs` | Ajouter 3 handlers tableau blanc | Phase 1 |
| `uafricas_backend/src/routes.rs` | Ajouter 3 routes tableau blanc | Phase 1 |
| `app/components/afrolang/AfrolangRoom.vue` | Integrer split-screen whiteboard | Phase 3 |
| `app/components/afrolang/AfrolangControls.vue` | Activer bouton tableau blanc | Phase 3 |
| `app/composables/useAfrolang.ts` | Ajouter 3 fonctions API whiteboard | Phase 2 |

---

## Critere de completion Phase 4 (FINALE)

> **Le feature Afrolang est COMPLET quand :**
> - [ ] Tous les 9 points de la progression sont coches
> - [ ] La mini-app tldraw build sans erreur et est servie en static
> - [ ] Le tableau blanc s'affiche dans la room (split-screen ou sidebar)
> - [ ] Les dessins se synchronisent entre 2+ participants en temps reel (DataChannel)
> - [ ] Le snapshot se sauvegarde automatiquement toutes les 30s (verifier en BDD)
> - [ ] Un participant qui rejoint en cours de session voit le dessin existant (restauration)
> - [ ] Le moderateur peut effacer le tableau blanc
> - [ ] Le bouton "Tableau blanc" dans les controles fonctionne (toggle)
>
> Quand c'est fait → mettre le statut a `TERMINE` dans [00_OVERVIEW.md](./00_OVERVIEW.md).
> **Toutes les 4 phases sont terminees. Le feature Afrolang est complet.**
