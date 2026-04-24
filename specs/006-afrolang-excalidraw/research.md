# Phase 0 — Research: Migration du tableau blanc Afrolang vers Excalidraw

**Feature** : `006-afrolang-excalidraw`
**Date** : 2026-04-24

Ce document consolide les décisions techniques prises pour lever tous les `NEEDS CLARIFICATION` potentiels avant la phase de design. Aucune clarification fonctionnelle ne reste ouverte (cf. `spec.md` § Clarifications, session 2026-04-24).

---

## Décision 1 — Librairie cible du moteur de tableau blanc

- **Decision** : `@excalidraw/excalidraw` (npm), licence MIT, self-hosted dans l'iframe React `whiteboard/`.
- **Rationale** :
  - Licence MIT sans watermark ni mécanisme anti-tamper en production, contrairement à tldraw v3/v4 qui désactive l'UI après quelques secondes sans `licenseKey` commercial valide (cause directe du bug prod AC-1).
  - API imperative `excalidrawAPI` compatible avec un bridge `postMessage` (méthodes `updateScene`, `resetScene`, `getSceneElements`, `getAppState`).
  - Callback `onChange(elements, appState, files)` adapté à une stratégie « last-write-wins global » (Q2) sans nécessiter de protocole de diffs.
  - `langCode="fr-FR"` supporté nativement (FR-010).
  - Taille du bundle plus élevée que tldraw, mais acceptable car l'iframe n'est chargée qu'à la demande (lazy mount lorsque `tableauBlancOuvert === true` dans `AfrolangRoom.vue`).
- **Alternatives considered** :
  - *tldraw v3/v4 avec licenseKey* : écarté — coût récurrent incompatible avec une plateforme panafricaine associative (NFR-1).
  - *tldraw v2 (MIT historique)* : écarté — abandonné par l'amont, non maintenu, régressions potentielles et écosystème figé.
  - *Fabric.js / Konva.js* : écarté — pas d'UI de tableau blanc clé en main, il faudrait reconstruire toute la toolbar.
  - *Excalidraw Plus (hosted)* : écarté — service tiers payant, contrevient à NFR-1.
  - *Whiteboard maison (canvas brut)* : écarté — explosion du coût, principe V (YAGNI) de la constitution.

## Décision 2 — Stratégie de diffusion temps réel

- **Decision** : broadcast de la scène complète débouncée ~80 ms via `Room.localParticipant.publishData` ; réception via `RoomEvent.DataReceived` ; garde anti-écho par flag `remote` maintenu le temps de l'application de `updateScene`.
- **Rationale** :
  - Déjà la tuyauterie en place pour le tableau blanc tldraw ; l'architecture LiveKit DataChannel reste valable sans nouvelle infrastructure.
  - Le débouncing 80 ms absorbe les rafales d'événements Excalidraw pendant les tracés continus (stylo libre) sans dégrader la latence perçue (SC-002 < 500 ms).
  - Scène complète (au lieu de deltas) = cohérent avec la stratégie last-write-wins (Q2 / FR-014) : un récepteur n'a jamais à fusionner.
  - Payload typique pour une scène modeste (< 200 éléments, pas d'images lourdes grâce à FR-001a) : quelques dizaines de Ko, compatible avec la capacité d'un DataChannel LiveKit non fiable.
- **Alternatives considered** :
  - *Diffs par élément* : écarté — complexité accrue, nécessite une couche de réconciliation, incompatible avec la simplicité YAGNI.
  - *CRDT (Yjs / Automerge)* : écarté — usine à gaz pour une fonctionnalité pédagogique à faible concurrence réelle.
  - *WebSocket serveur relay* : écarté — LiveKit DataChannel existe déjà ; ajouter un canal doublerait l'infrastructure.

## Décision 3 — Fréquence et forme du snapshot persistant

- **Decision** : un timer `setInterval` de 30 s dans `AfrolangWhiteboard.vue` pour les modérateurs uniquement ; envoi de `{ elements, appState, files }` (filtrage des champs volatils comme `collaborators`, `selectedElementIds`) via `sauvegarderTableauBlanc(sessionId, donnees)` ; snapshot final tenté sur `beforeunload` et dans `onBeforeUnmount` du composant.
- **Rationale** :
  - Parité stricte avec l'implémentation tldraw actuelle (FR-005).
  - 30 s = compromis entre fraîcheur et charge serveur (assumption explicite dans la spec).
  - Filtrer les champs volatils réduit la taille JSONB et évite les faux conflits.
- **Alternatives considered** :
  - *Sauvegarde par événement* : écarté — trafic trop élevé, surcharge du handler Rust de PUT.
  - *Sauvegarde uniquement à la fermeture* : écarté — perd tout en cas de crash navigateur.

## Décision 4 — Lecture défensive des snapshots legacy tldraw

- **Decision** : à l'ouverture, si le document JSONB récupéré par `obtenirTableauBlanc` contient une clé `document`, `store` ou `records` (marqueurs tldraw), ou à défaut une clé `elements` non-array, considérer l'état comme incompatible et initialiser Excalidraw avec un état vide sans erreur utilisateur.
- **Rationale** :
  - FR-009 impose l'ouverture sur état vide plutôt qu'une erreur.
  - La prod n'a jamais été exploitable (User Story 1) → aucun contenu pédagogique à préserver, l'effort de conversion serait gaspillé.
  - Logique isolée côté iframe (fonction utilitaire `estSnapshotExcalidrawValide`).
- **Alternatives considered** :
  - *Conversion automatique tldraw → Excalidraw* : écarté — impossible en pratique (modèles graphiques très différents) et sans bénéfice utilisateur vu l'inutilisabilité prod.
  - *Suppression SQL en masse des lignes tldraw* : écarté — contrevient à NFR-3 (aucune migration SQL) et le backend doit rester intouché.

## Décision 5 — Mode dégradé et resynchronisation à la reconnexion

- **Decision** : observer la propriété `room.state` de LiveKit ; si la room devient `Disconnected` puis `Connected`, déclencher un `obtenirTableauBlanc(sessionId)` suivi d'un `postMessage({ type: 'load-snapshot', snapshot })` à l'iframe. Hors connexion, les `onChange` locaux ne sont ni broadcastés ni persistés.
- **Rationale** : couvre FR-008 (mode dégradé) et FR-016 (resync automatique). Implémentation simple : un seul `watch` sur `room?.state` + un flag booléen `estConnecte`.
- **Alternatives considered** :
  - *Resync par réplication d'opérations locales en attente* : écarté — crée des boucles d'écho si un autre participant a déjà modifié entre-temps, contrevient à Q2 last-write-wins.
  - *Rechargement complet de la page* : écarté — dégrade l'UX de la session (audio/vidéo LiveKit coupés).

## Décision 6 — Permissions de dessin et bouton « Effacer tout »

- **Decision** : l'iframe Excalidraw est toujours montée en mode édition complète (aucun passage du prop `viewModeEnabled`) ; seul le bouton « Effacer tout » est conditionné côté Vue par `props.estModerateur` (`v-if` dans le template). Le `clear` broadcasté est accepté par tout participant (la garde côté émetteur suffit).
- **Rationale** : cohérent avec Q1 (« tous les participants peuvent dessiner, seul l'effacement est réservé au modérateur »).
- **Alternatives considered** :
  - *Masquer la toolbar pour les non-modérateurs* : écarté — contredit Q1.
  - *Double garde serveur pour « Effacer tout »* : non nécessaire, la route `DELETE /api/afrolang/sessions/:id/tableau-blanc` existe déjà et son autorisation relève du backend (non modifié).

## Décision 7 — Validation images (FR-001a)

- **Decision** : intercepter l'événement `onPaste` et le handler de fichier de l'API Excalidraw côté iframe ; pour chaque image, vérifier `file.type` appartient à `['image/jpeg','image/png']` et `file.size <= 2 * 1024 * 1024` ; en cas d'échec, afficher un toast localisé « Image refusée : formats acceptés JPEG/PNG, taille max 2 Mo » et annuler l'insertion.
- **Rationale** :
  - Prévention du gonflement JSONB (limite pratique PostgreSQL) et du DoS canal LiveKit.
  - Cohérent avec la pratique Afripulse (validation 2 Mo côté client déjà en place ailleurs dans le projet).
- **Alternatives considered** :
  - *Validation serveur uniquement* : écarté — le spec FR-013 interdit de modifier le backend, et le coût de transit jusqu'à l'erreur serveur est élevé.
  - *Limite plus large (5 Mo)* : écarté — cohérence avec les autres modules du projet (2 Mo est la norme UAfricas).

## Décision 8 — Agent context (mise à jour CLAUDE.md)

- **Decision** : ajouter une ligne dans la section `## Recent Changes` de `CLAUDE.md` pour signaler la nouvelle feature et l'ajout de la dépendance `@excalidraw/excalidraw` au projet `whiteboard/`. Laisser le reste inchangé.
- **Rationale** : script `update-agent-context.sh claude` attendu ; maintient la traçabilité historique.
- **Alternatives considered** :
  - *Pas de mise à jour* : écarté — contrevient au workflow speckit.
