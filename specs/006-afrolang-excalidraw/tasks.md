---

description: "Task list — Migration du tableau blanc Afrolang vers Excalidraw"
---

# Tasks: Migration du tableau blanc Afrolang vers Excalidraw

**Input** : Design documents dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/specs/006-afrolang-excalidraw/`
**Prerequisites** : `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/postmessage.md`, `quickstart.md`
**Tests** : aucune tâche de test automatisé générée — la constitution (principe V) et la spec ne requièrent pas de framework de tests ; la validation se fait manuellement via `quickstart.md` étapes 5-6.
**Organisation** : tâches regroupées par user story pour permettre une livraison MVP incrémentale.

## Format : `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers distincts, aucune dépendance incomplète)
- **[Story]** : user story concernée (US1, US2, US3, US4)
- Chaque tâche cite un chemin de fichier absolu explicite

## Path Conventions

Monorepo UAfricas — deux emplacements modifiés :

- `whiteboard/src/App.tsx` (iframe React — refonte totale)
- `uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue` (composant Vue pont)
- `uafricas_frontend/public/whiteboard/` (actif statique régénéré par build)

**Backend, schéma SQL, pages Nuxt, `AfrolangRoom.vue`, `useAfrolang.ts` intouchés** (FR-012, FR-013).

---

## Phase 1 : Setup (Shared Infrastructure)

**Purpose** : retirer tldraw, ajouter Excalidraw, préparer le terrain.

- [X] T001 Retirer la dépendance `tldraw` dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/package.json` via `cd whiteboard && pnpm remove tldraw`
- [X] T002 Ajouter la dépendance `@excalidraw/excalidraw` (MIT, dernière stable ≥ 0.18) dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/package.json` via `cd whiteboard && pnpm add @excalidraw/excalidraw`
- [X] T003 Vérifier que `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/pnpm-lock.yaml` ne contient plus aucune occurrence `tldraw` (`grep -c tldraw pnpm-lock.yaml` doit renvoyer 0)

---

## Phase 2 : Foundational (Blocking Prerequisites)

**Purpose** : scaffolder la structure minimale de l'iframe Excalidraw qui servira de base à toutes les user stories. Aucune user story ne peut avancer tant que l'iframe ne monte pas le composant et ne pose pas le bridge `postMessage`.

**CRITICAL** : aucune US ne peut démarrer avant la fin de cette phase.

- [X] T004 Réécrire intégralement `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx` : remplacer le composant tldraw par `<Excalidraw langCode="fr-FR" />` monté en plein écran, avec capture de l'instance via `excalidrawAPI` dans une `useRef`. Conserver `main.tsx` et `index.html` tels quels.
- [X] T005 Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, émettre un message `{ type: 'excalidraw-ready' }` vers `window.parent` dès que l'`excalidrawAPI` est disponible (cf. `contracts/postmessage.md`).
- [X] T006 Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, ajouter un listener `window.addEventListener('message', handler)` qui dispatch sur les types `apply-operation`, `load-snapshot`, `get-snapshot`, `clear` (handlers vides à ce stade, à compléter par US).
- [X] T007 Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, introduire le flag `remote: boolean` (ref) initialisé à `false`, basculé à `true` avant toute application via `updateScene`, remis à `false` au prochain `onChange`.
- [X] T008 Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, implémenter la fonction utilitaire `estSnapshotExcalidrawValide(donnees: unknown): boolean` selon la règle de détection de `data-model.md` (rejet si clés `document`/`store`/`records` présentes ou si `elements` n'est pas un tableau).
- [X] T009 Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, implémenter `filterAppState(appState)` qui supprime les champs volatils listés dans `data-model.md` (`collaborators`, `selectedElementIds`, `editingElement`, etc.).
- [X] T010 [P] Builder l'iframe une première fois via `cd /Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard && pnpm build` pour détecter les erreurs TS/Vite et valider la configuration avant d'injecter la logique métier.
- [X] T011 Remplacer atomiquement l'actif statique : `rm -rf /Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard && cp -r /Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/dist /Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard`.

**Checkpoint** : iframe Excalidraw monte en dev (Nuxt + rafraîchissement d'une page Afrolang) avec scène vide et barre d'outils en français. Les user stories peuvent démarrer.

---

## Phase 3 : User Story 1 — Barre d'outils persistante en prod (Priority : P1) — MVP

**Goal** : déployer une iframe Excalidraw qui reste fonctionnelle indéfiniment en production, sans watermark ni désactivation automatique. Résout la contrainte bloquante déclenchante de la migration.

**Independent Test** : builder l'iframe, la copier dans `public/whiteboard/`, ouvrir une session Afrolang et laisser la fenêtre ouverte 15 minutes ; la barre d'outils reste visible. Grep du bundle ne contient plus de marqueurs tldraw.

- [X] T012 [US1] Vérifier que `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx` n'importe plus aucun symbole depuis `tldraw` ni `tldraw/*`. Rechercher toute occurrence résiduelle dans le dossier `whiteboard/src/` et la supprimer.
- [X] T013 [P] [US1] Supprimer tout CSS tldraw résiduel éventuellement importé dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/` (recherche `@import`, `tldraw.css`, `tldraw/editor.css`).
- [X] T014 [US1] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, importer le CSS officiel d'Excalidraw (`@excalidraw/excalidraw/index.css`) et confirmer que la barre d'outils s'affiche correctement au mount.
- [X] T015 [US1] Rebuilder (`cd /Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard && pnpm build`) puis recopier `dist/` dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard/` (répéter le pattern T011).
- [X] T016 [US1] Vérifier l'absence de marqueurs tldraw dans le bundle final : `grep -l "tl-watermark\|No tldraw license key provided" /Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard/assets/*.js` doit ne rien retourner (AC-6).
- [ ] T017 [US1] Tester manuellement avec deux comptes (`quickstart.md` Étape 5 points 1-4) : la barre d'outils Excalidraw s'affiche dans les salles publique et privée, ne disparaît pas, est en français (AC-1, FR-010, FR-011).

**Checkpoint MVP** : AC-1, AC-5, AC-6, AC-7 tenus. Le déploiement production est possible à ce stade en mode « dessin local uniquement ».

---

## Phase 4 : User Story 2 — Collaboration temps réel via LiveKit (Priority : P2)

**Goal** : tout participant voit en quasi-temps réel les tracés des autres, sans boucle d'écho.

**Independent Test** : deux profils navigateur connectés à la même session ; dessiner côté A, vérifier l'apparition côté B en < 500 ms ; déplacer un élément, le supprimer, ajouter du texte — chaque opération se répercute bidirectionnellement sans duplication.

- [X] T018 [US2] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, implémenter le callback `onChange(elements, appState, files)` avec débouncing 80 ms ; n'émettre `{ type: 'excalidraw-operation', payload: { elements, appState: filterAppState(appState) } }` à `window.parent` que si `remote === false`.
- [X] T019 [US2] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, implémenter le handler `apply-operation` : `remote.current = true; excalidrawAPI.updateScene({ elements: payload.elements, appState: payload.appState })` ; `remote` repasse à `false` au prochain `onChange`.
- [X] T020 [US2] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, remplacer les anciens handlers de messages tldraw par les nouveaux types `excalidraw-operation`, `excalidraw-ready`, `excalidraw-snapshot`, `excalidraw-image-rejected`. Conserver strictement l'interface de props (`sessionId`, `estModerateur`, `room`).
- [X] T021 [US2] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, implémenter le broadcast LiveKit : à la réception d'un `excalidraw-operation` de l'iframe, si `props.room?.state === 'connected'`, publier via `props.room.localParticipant.publishData(...)` ; sinon ignorer (mode dégradé FR-008).
- [X] T022 [US2] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, implémenter la réception LiveKit : souscrire à `RoomEvent.DataReceived` sur `props.room`, désérialiser le payload, et envoyer à l'iframe un `{ type: 'apply-operation', payload }` via `iframeRef.value.contentWindow.postMessage(..., targetOrigin)`.
- [X] T023 [US2] Rebuilder l'iframe (`pnpm build` + recopie dans `uafricas_frontend/public/whiteboard/`) après modifications React.
- [ ] T024 [US2] Valider manuellement avec deux profils (`quickstart.md` Étape 5 points 5-6) : dessin bidirectionnel < 500 ms, aucun écho sur l'auteur (AC-2, FR-003, FR-014).

**Checkpoint** : AC-2 tenu ; FR-002, FR-003, FR-014, FR-015 couverts.

---

## Phase 5 : User Story 3 — Persistance snapshot + Effacer tout (Priority : P2)

**Goal** : un modérateur retrouve son tableau à la réouverture ; il peut vider le tableau pour tous en un clic.

**Independent Test** : en tant que modérateur, dessiner, attendre 30 s, fermer l'onglet, rouvrir depuis la même URL → contenu restauré. Cliquer « Effacer tout » → le tableau se vide chez tous les participants connectés et à la prochaine ouverture aucun contenu ancien ne reste.

- [X] T025 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, implémenter le handler `get-snapshot` : construire `{ elements: excalidrawAPI.getSceneElements(), appState: filterAppState(excalidrawAPI.getAppState()), files: excalidrawAPI.getFiles() ?? {} }` et poster `{ type: 'excalidraw-snapshot', payload }` à `window.parent`.
- [X] T026 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, implémenter le handler `load-snapshot` : appliquer `estSnapshotExcalidrawValide(snapshot)` ; si invalide, substituer par `{ elements: [], appState: {}, files: {} }` ; sinon `remote.current = true; excalidrawAPI.updateScene({ elements, appState })` puis `if (snapshot.files) excalidrawAPI.addFiles(Object.values(snapshot.files))`.
- [X] T027 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, implémenter le handler `clear` : `remote.current = true; excalidrawAPI.resetScene()`.
- [X] T028 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, à réception du message `excalidraw-ready` de l'iframe, appeler `obtenirTableauBlanc(props.sessionId)` puis envoyer à l'iframe `{ type: 'load-snapshot', snapshot: donnees }` (sans modifier `useAfrolang.ts` — FR-013).
- [X] T029 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, si `props.estModerateur === true`, démarrer un `setInterval` de 30 000 ms qui envoie `{ type: 'get-snapshot' }` à l'iframe ; nettoyage dans `onBeforeUnmount`.
- [X] T030 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, à la réception d'un `excalidraw-snapshot` de l'iframe, appeler `sauvegarderTableauBlanc(props.sessionId, { type: 'excalidraw', version: 1, elements: payload.elements, appState: payload.appState, files: payload.files })`.
- [X] T031 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, ajouter dans `onBeforeUnmount` et sur `window.addEventListener('beforeunload', ...)` un snapshot final via `get-snapshot` synchrone si modérateur.
- [X] T032 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, ajouter un bouton « Effacer tout » conditionné par `v-if="estModerateur"` (Tailwind CSS v4 pur, sans daisyUI — principe VI). Au clic : (1) envoyer `{ type: 'clear' }` à l'iframe locale, (2) broadcast LiveKit `{ __clear: true }`, (3) appeler `effacerTableauBlanc(props.sessionId)`.
- [X] T033 [US3] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, dans le handler `DataReceived`, détecter `payload.__clear === true` et envoyer `{ type: 'clear' }` à l'iframe locale (pas d'appel `effacerTableauBlanc` côté récepteur — seul l'émetteur persiste).
- [X] T034 [US3] Rebuilder l'iframe et recopier dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard/`.
- [ ] T035 [US3] Valider manuellement (`quickstart.md` Étape 5 points 7-8) : restauration après fermeture/réouverture (AC-3), effacement global synchrone des participants (AC-4), absence du bouton pour les non-modérateurs (FR-007), résilience aux snapshots legacy tldraw (FR-009).

**Checkpoint** : AC-3, AC-4 tenus ; FR-004, FR-005, FR-006, FR-007, FR-009 couverts.

---

## Phase 6 : User Story 4 — Mode dégradé & resync reconnexion (Priority : P3)

**Goal** : dessiner localement reste possible hors connexion temps réel ; à la reconnexion, le tableau se resynchronise automatiquement sur le dernier snapshot serveur.

**Independent Test** : ouvrir le tableau hors salle active (pas de connexion LiveKit) → aucun crash, aucune erreur console. Puis dans une session, couper le wifi 10 s, le rétablir → le tableau retrouve le dernier snapshot persisté sans intervention manuelle.

- [X] T036 [US4] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, garde défensive : ne jamais appeler `props.room.localParticipant.publishData` si `props.room` est `null` ou si `props.room.state !== 'connected'` (couvrir FR-008).
- [X] T037 [US4] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, désactiver l'intervalle de snapshot 30 s tant que `props.room?.state !== 'connected'` (pas de persistance en mode dégradé).
- [X] T038 [US4] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, ajouter un `watch(() => props.room?.state, (nouveau, ancien) => { ... })` qui, lors d'une transition `Disconnected` → `Connected` (ou équivalent LiveKit), appelle `obtenirTableauBlanc(props.sessionId)` puis envoie `{ type: 'load-snapshot', snapshot: donnees }` à l'iframe (FR-016).
- [X] T039 [US4] Rebuilder l'iframe et recopier dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard/` (rebuild pour cohérence, même si aucun changement iframe strictement nécessaire ici).
- [ ] T040 [US4] Valider manuellement (`quickstart.md` Étape 5 point 11) : ouverture hors connexion, coupure/rétablissement réseau en session — pas d'erreur console, resync automatique observée.

**Checkpoint** : SC-004 tenu en mode dégradé et nominal ; FR-008, FR-016 couverts.

---

## Phase 7 : Cross-Cutting — Validation images (FR-001a)

**Goal** : bloquer côté client l'insertion d'images non conformes (taille > 2 Mo ou format non JPEG/PNG), avec feedback utilisateur.

**Independent Test** : coller une image JPEG valide < 2 Mo → ajoutée et diffusée. Coller un PDF ou un JPEG > 2 Mo → refus local, toast utilisateur, aucune diffusion ni persistance.

- [X] T041 [US2] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, intercepter l'événement `onPaste` du composant Excalidraw (ou du `window`) et pour chaque `File`/`Blob` de type image, valider : `file.type ∈ ['image/jpeg','image/png']` ET `file.size <= 2 * 1024 * 1024`. En cas d'échec, annuler l'insertion et émettre `{ type: 'excalidraw-image-rejected', payload: { raison: 'taille' | 'format', nomFichier } }` vers `window.parent`.
- [X] T042 [US2] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src/App.tsx`, répéter la validation pour l'import d'image via le bouton de la toolbar Excalidraw (accroche via les événements exposés par `onChange` + comparaison de `files` entrants).
- [X] T043 [US2] Dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`, à réception d'un message `excalidraw-image-rejected`, afficher un toast transitoire (Tailwind v4 pur, texte français) : « Image refusée : formats acceptés JPEG/PNG, taille max 2 Mo ».
- [X] T044 [US2] Rebuilder l'iframe et recopier dans `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard/`.
- [ ] T045 [US2] Valider manuellement (`quickstart.md` Étape 5 points 9-10) : image valide → OK ; image trop lourde ou PDF → refus + toast.

---

## Phase 8 : Polish & Cross-Cutting Concerns

**Purpose** : nettoyage final, validation AC-5/AC-6, déploiement, documentation.

- [X] T046 Exécuter `grep -rn "tldraw" /Users/mac/Documents/projets/uafricas_projets/uafricas/whiteboard/src /Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app /Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard 2>/dev/null | grep -v ".lock\|node_modules"`. Aucune sortie attendue (AC-5).
- [X] T047 [P] Exécuter `grep -l "tl-watermark\|No tldraw license key provided" /Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/public/whiteboard/assets/*.js`. Aucune sortie attendue (AC-6).
- [X] T048 [P] Vérifier que `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/components/afrolang/AfrolangRoom.vue`, `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/pages/afrolang/session/[id].vue` et `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/pages/afrolang/session/privee/[id].vue` n'ont subi aucune modification (`git status` + `git diff` ciblés) — AC-7, FR-012.
- [X] T049 [P] Vérifier que `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_frontend/app/composables/useAfrolang.ts` et l'ensemble de `/Users/mac/Documents/projets/uafricas_projets/uafricas/uafricas_backend/` n'ont subi aucune modification (FR-013).
- [ ] T050 Commit conventionnel unique : `git add whiteboard/package.json whiteboard/pnpm-lock.yaml whiteboard/src/App.tsx uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue uafricas_frontend/public/whiteboard && git commit -m "feat(afrolang-whiteboard): migration tldraw → Excalidraw pour résoudre la désactivation UI en prod"`.
- [X] T051 Mettre à jour `/Users/mac/Documents/projets/uafricas_projets/uafricas/CLAUDE.md` § Recent Changes avec une ligne résumant la migration et la nouvelle dépendance `@excalidraw/excalidraw` dans `whiteboard/` (Décision 8 de `research.md`).
- [ ] T052 Déployer en production via `./deploy.sh update` depuis la racine du monorepo.
- [ ] T053 Valider en production (`quickstart.md` Étape 6) : session Afrolang ouverte ≥ 15 minutes sur `https://www.africans-world.org`, barre d'outils persistante (AC-1, SC-001), aucune erreur console (SC-004), `grep -c "tl-watermark\|No tldraw license" /opt/uafricas/frontend_static/whiteboard/assets/*.js` côté VPS renvoie 0.

---

## Dependencies

### Dépendances entre phases

```
Phase 1 (Setup)
   ↓
Phase 2 (Foundational) — T004..T011  [BLOQUANT]
   ↓
Phase 3 (US1 — MVP) ────────────────┐
Phase 4 (US2 collab) ──────┐        │
Phase 5 (US3 persistance) ──┤       │   parallélisables entre elles
Phase 6 (US4 mode dégradé) ──┤      │   après P2 terminée
Phase 7 (Validation images) ──┘     │
   ↓
Phase 8 (Polish & Deploy)
```

### Dépendances fines

- **T001 → T002** : retirer tldraw avant d'ajouter Excalidraw (évite conflits React 19 hoisting).
- **T004 → T005..T009** : le scaffolding Excalidraw doit exister avant d'ajouter les handlers.
- **T010 → T011** : build avant copie statique.
- **US1 (Phase 3)** est le seul prérequis strict pour valider le MVP ; les US2/US3/US4 peuvent suivre dans n'importe quel ordre.
- **US3 T032 → T033** : le bouton « Effacer tout » doit exister avant de tester la réception distante.
- **Phase 7 (images)** est étiquetée `[US2]` car la diffusion d'image repose sur le pipeline LiveKit mis en place par US2 ; peut techniquement démarrer dès que US2 est livrée.
- **Phase 8** nécessite toutes les phases précédentes terminées.

## Parallel Opportunities

Exemples de tâches exécutables en parallèle (fichiers ou vérifications indépendantes) :

- **Pendant P2** : T010 (premier build) peut se préparer pendant que T004..T009 sont en review.
- **Pendant P3 (US1)** : T013 (nettoyage CSS) parallèle à T012 (nettoyage imports).
- **Pendant P8** : T047 / T048 / T049 (3 vérifications grep/git indépendantes) en parallèle.
- Les phases 4, 5 et 6 elles-mêmes peuvent être traitées en parallèle par trois développeurs si besoin, chacun sur son périmètre fonctionnel (US2 = collab, US3 = persistance, US4 = resync), car elles touchent des sections disjointes du même fichier `AfrolangWhiteboard.vue` et ajoutent des handlers distincts dans `App.tsx`. Coordination simple via `git` (merges triviaux attendus).

## Independent Test Criteria

| US | Critère de test autonome |
|----|--------------------------|
| US1 | Ouvrir `/afrolang/session/{id}` (publique ou privée), activer le tableau, attendre 15 min → toolbar Excalidraw visible, aucune erreur console, aucun watermark. |
| US2 | 2 profils, même session. Dessiner côté A, forme apparaît côté B en < 500 ms. Supprimer côté B, disparaît côté A. Zéro boucle d'écho observée. |
| US3 | Modérateur dessine, attend 30 s, ferme l'onglet, rouvre → contenu restauré. Clique « Effacer tout » → vidage immédiat partagé + rien au prochain rechargement. |
| US4 | Ouvrir tableau hors salle active → dessin local OK, 0 erreur console. Session active, couper wifi 10 s, rétablir → resync automatique sur dernier snapshot. |
| Images (FR-001a) | Coller image JPEG 500 Ko → OK. Coller PDF ou JPEG 5 Mo → refusée + toast. |

## Implementation Strategy

**MVP (livraison minimale utilisable)** = Phase 1 + Phase 2 + Phase 3 (US1).

Au terme de l'US1, la production peut déjà être déployée pour valider immédiatement la correction du bug bloquant (barre d'outils persistante) — même sans collaboration ni persistance, ce qui est meilleur que l'état actuel (fonctionnalité totalement inutilisable).

**Incréments suivants** :

1. **Incrément 2** = + Phase 4 (US2) — restaure la collaboration multi-participants.
2. **Incrément 3** = + Phase 5 (US3) — restaure la persistance et l'effacement global.
3. **Incrément 4** = + Phase 6 (US4) — robustesse réseau.
4. **Incrément 5** = + Phase 7 — validation images.
5. **Finalisation** = Phase 8 — nettoyage, déploiement prod, audit.

Chaque incrément est indépendamment testable dans un navigateur et peut être déployé sans dépendre des suivants (les phases 4-7 dégradent proprement si non livrées : FR-008 mode dégradé garantit qu'un tableau sans collab ne crashe pas).

## Validation du format de checklist

Toutes les tâches T001..T053 respectent le format strict : `- [ ] TNNN [P?] [USx?] Description avec chemin absolu`. Les tâches hors US (Setup, Foundational, Polish) n'ont pas de label `[Story]` ; les tâches des phases 3..7 en ont un conformément aux instructions.
