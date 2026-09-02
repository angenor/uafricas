# Implementation Plan: Migration du tableau blanc Afrolang vers Excalidraw

**Branch**: `006-afrolang-excalidraw` | **Date**: 2026-04-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-afrolang-excalidraw/spec.md`

## Summary

Remplacer le moteur du tableau blanc collaboratif des salles Afrolang, qui utilise actuellement `tldraw@4.3.2` et se désactive en production faute de licence commerciale, par `@excalidraw/excalidraw` (MIT). Le périmètre touche uniquement le projet iframe React `whiteboard/` (refonte complète de `App.tsx` et du `package.json`) et le composant bridge `uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue` (nouveaux messages `postMessage`, resync à la reconnexion LiveKit, validation images côté client). L'API backend, le schéma PostgreSQL `afrolang`, le composable `useAfrolang.ts`, les pages salles publiques/privées et `AfrolangRoom.vue` restent strictement inchangés. Le contenu JSONB persisté évolue de format (tldraw → Excalidraw) avec lecture défensive pour ignorer les anciens snapshots incompatibles.

## Technical Context

**Language/Version** : TypeScript 5.7 (iframe React 19) + TypeScript (Nuxt 4 / Vue 3 SSR côté plateforme), aucune modification Rust backend.
**Primary Dependencies** : `@excalidraw/excalidraw` (MIT, dernière majeure stable 0.18+), React 19, Vite 6 (iframe) ; `livekit-client` existant et `@fortawesome/vue-fontawesome` existant côté Vue, aucun ajout dans `uafricas_frontend/package.json`.
**Storage** : PostgreSQL 16, schema `afrolang`, colonne JSONB existante `donnees` de la table `afrolang.tableau_blanc_session` (endpoints `GET/PUT/DELETE /api/afrolang/sessions/:id/tableau-blanc`), aucune migration SQL.
**Testing** : validation manuelle multi-navigateurs (Chrome, Firefox, Safari, Edge) ; vérification production via `www.africans-world.org` ; pas de framework de tests configuré dans le repo (la constitution, principe V, n'exige pas la mise en place d'un tel cadre pour cette itération).
**Target Platform** : navigateurs desktop modernes (≤ 12 mois) ; tactile/mobile hors périmètre.
**Project Type** : web monorepo : iframe React (`whiteboard/`) + frontend Nuxt 4 (`uafricas_frontend/`) + backend Rust Actix-Web (`uafricas_backend/`, non modifié).
**Performance Goals** : jusqu'à 100 participants actifs simultanés par session avec latence de diffusion < 500 ms dans 95 % des cas ; débouncing des opérations locales ~80 ms pour limiter la charge LiveKit.
**Constraints** : conservation stricte des contrats publics, interface props de `AfrolangWhiteboard.vue` (`sessionId`, `estModerateur`, `room`), signatures `obtenirTableauBlanc` / `sauvegarderTableauBlanc` / `effacerTableauBlanc`, routes backend existantes ; zéro dépendance payante ; images ≤ 2 Mo JPEG/PNG côté client ; barre d'outils jamais masquée.
**Scale/Scope** : 2 fichiers à refondre (`whiteboard/src/App.tsx`, `whiteboard/package.json`), 1 fichier à adapter (`uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue`) ; ~0 ligne backend ; re-build + re-copie d'actif statique dans `uafricas_frontend/public/whiteboard/`.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Conformité | Justification |
|----------|-----------|---------------|
| I. Français d'Abord | Conforme | Code, commentaires, messages UI en français ; `langCode="fr-FR"` côté Excalidraw. Les identifiants techniques `postMessage` (`excalidraw-operation`, `apply-operation`, etc.) restent en anglais car ce sont des identifiants de protocole inter-processus (exception « termes techniques sans équivalent consacré »). |
| II. Monorepo Cohérent | Conforme | Modification intra-monorepo uniquement ; cohérence TS ↔ JSONB serveur préservée, la colonne `donnees` restant un conteneur opaque côté backend. |
| III. SQL Source de Vérité | Conforme | Aucune modification du schéma SQL. La colonne `donnees JSONB` reste un conteneur générique ; seule la forme *interne* du document change côté application. |
| IV. Sécurité par Défaut | Conforme | Authentification JWT inchangée (routes backend non touchées) ; validation des images côté client (taille + format) avant diffusion/persistance (prévention DoS JSONB) ; pas de nouveaux secrets ; isolement iframe préservé via `postMessage` ciblé. |
| V. Simplicité (YAGNI) | Conforme | Aucune nouvelle abstraction : on remplace un moteur par un autre avec une API plus simple. Last-write-wins (décision Q2) évite d'introduire CRDT/OT. Pas de feature flag ni de double-maintenance tldraw/Excalidraw. |
| VI. Tailwind v4 (daisyUI back-office uniquement) | Conforme | `AfrolangWhiteboard.vue` est un composant public ; les ajouts éventuels (bouton « Effacer tout », toasts d'erreur image) utiliseront Tailwind v4 pur, sans classes daisyUI. Excalidraw apporte son propre CSS scoped dans l'iframe, sans conflit avec la feuille Tailwind du frontend. |
| VII. Audit & Traçabilité | Conforme | Pas de nouvelle mutation backend ; les handlers `sauvegarder_tableau_blanc` et `effacer_tableau_blanc` existants, déjà instrumentés via `audit::log_action`, restent les seules voies de persistance. Le changement de format de contenu ne crée pas de nouveau flux à auditer. |

**Résultat** : toutes les gates passent, aucune violation à justifier. Complexity Tracking laissé vide.

## Project Structure

### Documentation (this feature)

```text
specs/006-afrolang-excalidraw/
├── plan.md              # Ce fichier (output /speckit.plan)
├── spec.md              # Specification fonctionnelle (output /speckit.specify + /speckit.clarify)
├── research.md          # Phase 0 : décisions techniques résolues
├── data-model.md        # Phase 1 : forme du snapshot Excalidraw + lecture défensive
├── contracts/
│   └── postmessage.md   # Phase 1 : contrats postMessage iframe ↔ Vue
├── quickstart.md        # Phase 1 : procédure build + validation prod
├── checklists/
│   └── requirements.md  # (déjà généré par /speckit.specify)
└── tasks.md             # Phase 2 : généré par /speckit.tasks (hors scope ici)
```

### Source Code (repository root)

```text
whiteboard/                               # iframe React, projet à refondre
├── package.json                          # [RÉÉCRIT] tldraw retiré, @excalidraw/excalidraw ajouté
├── pnpm-lock.yaml                        # [REGÉNÉRÉ] via pnpm install
├── src/
│   ├── App.tsx                           # [RÉÉCRIT] composant Excalidraw + bridge postMessage
│   └── main.tsx                          # [INCHANGÉ]
├── index.html                            # [INCHANGÉ]
├── vite.config.ts                        # [INCHANGÉ]
└── tsconfig.json                         # [INCHANGÉ]

uafricas_frontend/
├── app/components/afrolang/
│   ├── AfrolangWhiteboard.vue            # [ADAPTÉ] nouveaux messages postMessage, resync reconnexion, validation images
│   └── AfrolangRoom.vue                  # [INCHANGÉ, interdit par spec FR-012]
├── app/composables/
│   └── useAfrolang.ts                    # [INCHANGÉ, interdit par spec FR-013]
├── app/pages/afrolang/session/
│   ├── [id].vue                          # [INCHANGÉ, interdit par spec FR-012]
│   └── privee/[id].vue                   # [INCHANGÉ, interdit par spec FR-012]
└── public/whiteboard/                    # [REGÉNÉRÉ] copie du build Vite depuis whiteboard/dist/

uafricas_backend/                         # [INTOUCHÉ ENTIÈREMENT, spec FR-013]
```

**Structure Decision** : la migration est volontairement chirurgicale. Les seuls trois emplacements à toucher sont (1) le projet iframe `whiteboard/` (rewrite moteur + dépendances + `App.tsx`), (2) le composant pont `AfrolangWhiteboard.vue` (nouveaux messages `postMessage`, handler de reconnexion LiveKit, validation images, lecture défensive des anciens snapshots), et (3) l'actif statique servi par Nuxt (`uafricas_frontend/public/whiteboard/`) régénéré par `pnpm build`. Tout le reste du monorepo (pages Nuxt, layouts, backend Rust, schéma SQL, docker-compose, nginx) reste strictement intact : c'est une exigence forte de la spec (FR-012, FR-013), et la constitution (principe V, simplicité YAGNI) l'autorise et l'encourage.

## Complexity Tracking

Aucune violation de constitution : section laissée vide.

## Phase 2 (aperçu informatif : génération déléguée à `/speckit.tasks`)

Les user stories de la spec se mappent naturellement à des tâches orthogonales :

- **US1 : Barre d'outils persistante en prod** : refonte `whiteboard/` (package.json, App.tsx, build) + redéploiement. Couvre AC-1, AC-5, AC-6.
- **US2 : Collab temps réel** : intégration `onChange` Excalidraw + débouncing 80 ms + bridge `excalidraw-operation` / `apply-operation` + garde anti-écho. Couvre AC-2, FR-003, FR-014.
- **US3 : Persistance + Effacer tout** : snapshots 30 s modérateur uniquement, réponse à `get-snapshot`, effacement global (`clear`) avec broadcast LiveKit + `effacerTableauBlanc`, lecture défensive du format tldraw legacy. Couvre AC-3, AC-4, FR-005→FR-009.
- **US4 + FR-016 : Mode dégradé & resync reconnexion** : détection de l'état `Room` LiveKit (`ConnectionState`), appel `obtenirTableauBlanc` au retour de connexion, injection via `load-snapshot`.
- **FR-001a : Validation images** : garde côté iframe avant toute diffusion/persistance (2 Mo, JPEG/PNG).
- **AC-5/AC-6 : Nettoyage résiduel** : suppression code, CSS, assets, strings tldraw ; `grep` final de validation.

Chaque tâche est indépendamment testable à la main dans un navigateur. La recette finale (AC-7) passe par une session de fumée sur les deux pages Afrolang concernées sans modification de celles-ci.
