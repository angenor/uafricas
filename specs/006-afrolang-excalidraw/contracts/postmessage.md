# Phase 1 : Contract: postMessage iframe ↔ Vue

**Feature** : `006-afrolang-excalidraw`
**Date** : 2026-04-24

Ce contrat définit les messages échangés entre l'iframe React `whiteboard/` et le composant Vue hôte `AfrolangWhiteboard.vue`. Il **remplace** le précédent contrat tldraw. Le transport est `window.postMessage` avec ciblage de `contentWindow` (iframe → Vue utilise `window.parent.postMessage`, Vue → iframe utilise `iframe.contentWindow.postMessage`). Aucun backend n'est impliqué directement dans ces échanges : ils sont purement front.

---

## Règles transverses

- **Origine** : `targetOrigin` = origine courante de la page hôte (même origine en dev et en prod grâce au service statique via `public/whiteboard/`). Les messages d'origine non attendue sont ignorés (log console, pas d'erreur utilisateur).
- **Forme** : tout message est un objet sérialisable `{ type: string, ...payload }`. Aucun `Function`, `DOM Element`, `undefined` explicite ne doit être transmis.
- **Langue** : les identifiants de type (`type`) sont en anglais, sans accent (ex: `excalidraw-operation`). C'est un protocole technique inter-processus, exception autorisée par le principe I de la constitution.
- **Anti-écho** : l'iframe maintient un flag `remote` (booléen) mis à `true` avant d'appliquer `updateScene` en réponse à `apply-operation`, puis remis à `false` à l'événement `onChange` suivant. Pendant `remote === true`, aucun `excalidraw-operation` n'est ré-émis.
- **Idempotence** : un message reçu deux fois identiques doit produire le même état final sans corruption (conséquence de la stratégie last-write-wins).

---

## Messages iframe → Vue

### `excalidraw-ready`

Émis une fois par l'iframe à l'initialisation, après que l'`excalidrawAPI` soit disponible.

```json
{ "type": "excalidraw-ready" }
```

Comportement Vue : déclenche l'envoi initial de `load-snapshot` avec le contenu obtenu par `obtenirTableauBlanc(sessionId)`.

### `excalidraw-operation`

Émis par l'iframe lorsque l'utilisateur local modifie la scène (callback `onChange` débouncé ~80 ms), si et seulement si `remote === false`.

```json
{
  "type": "excalidraw-operation",
  "payload": {
    "elements": [ /* ExcalidrawElement[] filtré, voir data-model.md */ ],
    "appState": { /* Partial<AppState> sans champs volatils */ }
  }
}
```

Comportement Vue : si `room?.state === 'connected'`, broadcast via `room.localParticipant.publishData(payload, { reliable: false })`. Sinon (mode dégradé FR-008), ignoré silencieusement.

### `excalidraw-snapshot`

Réponse au message `get-snapshot`. Émise uniquement par l'iframe du modérateur, après réception de la demande.

```json
{
  "type": "excalidraw-snapshot",
  "payload": {
    "elements": [ /* ... */ ],
    "appState": { /* ... */ },
    "files": { /* BinaryFiles, peut être {} */ }
  }
}
```

Comportement Vue : appel `sauvegarderTableauBlanc(sessionId, { type: 'excalidraw', version: 1, ...payload })`. Erreur : log console, pas d'interruption utilisateur.

### `excalidraw-image-rejected`

Émis par l'iframe quand la garde FR-001a refuse une image (taille > 2 Mo ou MIME non supporté).

```json
{
  "type": "excalidraw-image-rejected",
  "payload": {
    "raison": "taille",
    "nomFichier": "logo_ecole.jpg"
  }
}
```

Comportement Vue : afficher un toast localisé (Tailwind v4 pur, cf. principe VI de la constitution) « Image refusée : formats acceptés JPEG/PNG, taille max 2 Mo ». Pas de diffusion LiveKit, pas de persistance.

---

## Messages Vue → iframe

### `apply-operation`

Émis par la Vue lorsqu'un message LiveKit `DataReceived` a été reçu d'un pair distant. L'iframe applique la scène sans ré-émettre.

```json
{
  "type": "apply-operation",
  "payload": {
    "elements": [ /* ... */ ],
    "appState": { /* ... */ }
  }
}
```

Comportement iframe : `remote = true; excalidrawAPI.updateScene(payload); /* remote rebasculé à false au onChange suivant */`.

### `load-snapshot`

Émis par la Vue pour injecter un état persisté, à l'ouverture initiale, ou après reconnexion LiveKit (FR-016).

```json
{
  "type": "load-snapshot",
  "snapshot": {
    "elements": [ /* ... */ ],
    "appState": { /* ... */ },
    "files": { /* ... */ }
  }
}
```

Comportement iframe :
1. Exécuter `estSnapshotExcalidrawValide(snapshot)` (cf. data-model.md).
2. Si invalide (snapshot tldraw legacy), substituer par `{ elements: [], appState: {}, files: {} }`.
3. Appliquer `remote = true; excalidrawAPI.updateScene(snapshotValide); if (snapshot.files) excalidrawAPI.addFiles(Object.values(snapshot.files))`.

### `get-snapshot`

Émis périodiquement (30 s) par la Vue si `props.estModerateur === true`. L'iframe doit répondre par un `excalidraw-snapshot`.

```json
{ "type": "get-snapshot" }
```

Comportement iframe : construire `{ elements: excalidrawAPI.getSceneElements(), appState: filterAppState(excalidrawAPI.getAppState()), files: excalidrawAPI.getFiles() }` et poster en retour.

### `clear`

Émis par la Vue lorsque le modérateur clique sur « Effacer tout », ET lorsqu'un pair distant envoie un signal `clear` via LiveKit.

```json
{ "type": "clear" }
```

Comportement iframe : `remote = true; excalidrawAPI.resetScene()`.

Comportement Vue additionnel côté modérateur émetteur : après l'envoi local, broadcast LiveKit de `{ __clear: true }` (préfixe distinct des payloads d'opération pour éviter toute confusion côté récepteur) + appel `effacerTableauBlanc(sessionId)`.

---

## Séquences de référence

### Ouverture normale

1. Vue monte l'iframe → iframe émet `excalidraw-ready`.
2. Vue appelle `obtenirTableauBlanc(sessionId)` → reçoit `{ donnees, version }`.
3. Vue envoie `load-snapshot` avec `donnees` (même si invalide, l'iframe gère la lecture défensive).
4. Utilisateurs commencent à dessiner.

### Collaboration live (2 participants, A et B)

1. A dessine → iframe A émet `excalidraw-operation`.
2. Vue A broadcast via LiveKit.
3. Vue B reçoit `DataReceived` → envoie `apply-operation` à iframe B.
4. Iframe B met à jour la scène avec `remote = true`, pas de ré-émission.

### Snapshot modérateur (toutes les 30 s)

1. Vue modérateur envoie `get-snapshot` à iframe modérateur.
2. Iframe répond par `excalidraw-snapshot`.
3. Vue appelle `sauvegarderTableauBlanc(sessionId, { type: 'excalidraw', version: 1, ...payload })`.

### Effacement global (modérateur déclenche)

1. Modérateur clique « Effacer tout ».
2. Vue envoie `clear` à iframe locale.
3. Vue broadcast LiveKit `{ __clear: true }`.
4. Vue appelle `effacerTableauBlanc(sessionId)`.
5. Chaque Vue distante reçoit `DataReceived` avec `{ __clear: true }` et relaie `clear` à son iframe.

### Resync après reconnexion LiveKit

1. `room.state` passe `Connected` → `Disconnected` → `Connected`.
2. Watcher Vue détecte la transition de retour.
3. Vue appelle `obtenirTableauBlanc(sessionId)`.
4. Vue envoie `load-snapshot` à iframe.
5. Les `onChange` locaux reprennent normalement la diffusion.

---

## Erreurs de protocole tolérées

| Cas | Comportement attendu |
|-----|-----------------------|
| Message d'origine inattendue | Log console, ignoré |
| `type` inconnu | Log console `debug`, ignoré |
| Payload manquant ou mal formé | Log console, ignoré côté récepteur ; l'émetteur n'est pas notifié |
| `load-snapshot` avec snapshot legacy tldraw | Substitué par scène vide, pas d'erreur utilisateur (FR-009) |
| `apply-operation` sans `elements` | Ignoré |
| `get-snapshot` reçu par un non-modérateur | Ignoré côté iframe (cas défensif, ne devrait pas arriver) |
