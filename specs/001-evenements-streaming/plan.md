# Implementation Plan: Événements en streaming direct sur la plateforme

**Branch**: `001-evenements-streaming` | **Date**: 2026-05-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-evenements-streaming/spec.md`

## Summary

Permettre qu'un événement de format « en ligne » ou « hybride » soit diffusé **en direct sur la plateforme** selon un modèle **webinaire** : l'organisateur (et les participants qu'il promeut à la volée) diffusent caméra/micro/écran ; les inscrits regardent et interagissent par chat texte, réactions et « lever la main ». La diffusion média réutilise l'infrastructure **LiveKit** déjà câblée pour afrolang (SFU + tokens JWT signés côté backend) ; aucun média n'est stocké (pas d'enregistrement au MVP).

Approche technique : extension du schéma `media_content` avec une table `evenement_session` (calquée sur `afrolang.session`) et `evenement_session_participant`, un module backend `evenement_streaming` (handlers + model) exposant des sous-routes sous `/api/evenements/{id}/direct`, et côté frontend un composant `EvenementDirectRoom.vue` (Tailwind v4 pur) consommant `livekit-client`. La différence clé avec afrolang : le **token porte `can_publish: false` pour les spectateurs** (afrolang fait publier tout le monde). Chat / réactions / lever-la-main circulent en **DataPackets LiveKit** (éphémères, pas de stockage), le « lever la main » étant aussi reflété sur le participant en base pour fournir à l'organisateur une liste fiable.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) ; TypeScript / Nuxt 4 (Vue 3 SSR) (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), `livekit-api` 0.4, `livekit-protocol` 0.7, `jsonwebtoken`, `uuid`, `chrono`, `serde` (backend) ; `livekit-client` ^2.17.1 (déjà installé), Pinia, Tailwind CSS v4 (pur), FontAwesome (frontend)
**Storage**: PostgreSQL 16, schéma `media_content` (source de vérité — Principe III). **Aucun stockage de média** (flux via SFU LiveKit). Cloche persistante via `arbre_genealogique.notifications` (système unifié existant).
**Testing**: Aucun framework configuré (constitution) — validation manuelle via `quickstart.md` (2 comptes : organisateur + inscrit). Tests automatisés hors périmètre actuel.
**Target Platform**: Serveur Linux (backend port 8080/8082) + rendu SSR web (frontend port 3000) ; SFU LiveKit (Docker dev : ports 7880/7881/7882 + 50000-50100/udp)
**Project Type**: Application web (monorepo frontend Nuxt + backend Rust — Principe II)
**Performance Goals**: latence diffusion < 5 s (SC-002) ; ≥ 100 spectateurs simultanés sans dégradation (SC-004) ; chat/réaction < 2 s (SC-006) — assurés par le SFU LiveKit et le canal DataPacket
**Constraints**: modèle webinaire (diffusion 1→N, promotion à la volée) ; accès réservé aux inscrits + organisateur ; pas d'enregistrement ; média jamais sur le serveur applicatif ; pas de cron (états dérivés à la lecture, arrêt de sécurité appliqué paresseusement)
**Scale/Scope**: 2 nouvelles tables, 1 enum logique, 1 migration idempotente ; ~8 endpoints REST ; 1 nouvelle page + ~4 composants frontend ; extension `useEvenements` ; 1 branche SSE supplémentaire

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Justification |
|----------|--------|---------------|
| **I. Français d'Abord** | ✅ PASS | Colonnes SQL, structs, composables, UI, messages en français (`evenement_session`, `lever-main`, « Rejoindre le direct »). Termes techniques tolérés : JWT, UUID, LiveKit, SFU, API. |
| **II. Monorepo Cohérent** | ✅ PASS | Changement cross-stack livré ensemble : SQL → struct Rust `FromRow` → DTO → type TS. Contrats cohérents documentés dans `contracts/`. |
| **III. SQL Source de Vérité** | ✅ PASS | Migration SQL idempotente d'abord (`schemas/09b_media_content_evenements_streaming.sql`), puis backend, puis frontend. UUID v4, TIMESTAMPTZ, snake_case français, CHECK + index partiels. |
| **IV. Sécurité par Défaut** | ✅ PASS | JWT in-handler ; revérif inscription/organisateur à chaque action ; token LiveKit **scopé** (`can_publish` false pour spectateurs) ; pas de secret en dur (réutilise `LIVEKIT_*` env) ; requêtes paramétrées sqlx ; échappement Vue natif pour le chat. |
| **V. Simplicité (YAGNI)** | ✅ PASS | Réutilise LiveKit + `livekit_moderation.rs` existants ; chat/réactions/lever-main en DataPackets client (aucun stockage, aucun nouvel endpoint pour le chat) ; pas de cron (arrêt de sécurité paresseux à la lecture) ; pas d'enregistrement ; calque le pattern `afrolang.session` éprouvé. Une seule extension de `livekit_moderation` (toggle `can_publish`). |
| **VI. Tailwind v4 (daisyUI back-office only)** | ✅ PASS | Pages événements = site public → **Tailwind v4 pur**, aucun `btn`/`card`/`modal` daisyUI. Réutilise le pattern Hero/Card/Modal existant. |
| **VII. Audit & Traçabilité** | ✅ PASS | `audit::log_action` (non-bloquant) sur ouvrir/clôturer/promouvoir/rétrograder/retirer (schema `media_content`, table `evenement_session`), **sans** contenu de chat ni média. |

**Résultat** : aucune violation. Section Complexity Tracking vide.

## Project Structure

### Documentation (this feature)

```text
specs/001-evenements-streaming/
├── spec.md              # Spécification (clarifiée)
├── plan.md              # Ce fichier (/speckit.plan)
├── research.md          # Phase 0 : décisions techniques
├── data-model.md        # Phase 1 : DDL + entités + transitions
├── quickstart.md        # Phase 1 : scénario de validation manuelle
├── contracts/           # Phase 1 : contrats REST + DataPackets/SSE
│   ├── rest-api.md
│   └── temps-reel.md
├── checklists/
│   └── requirements.md  # Checklist qualité (déjà validée)
└── tasks.md             # Phase 2 (/speckit.tasks — NON créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                                   # MODIF : insérer \ir schemas/09b_... avant contraintes
│   └── schemas/
│       └── 09b_media_content_evenements_streaming.sql   # NOUVEAU : migration idempotente
├── src/
│   ├── models/
│   │   ├── mod.rs                                   # MODIF : pub mod evenement_streaming;
│   │   └── evenement_streaming.rs                   # NOUVEAU : structs FromRow, DTO, COLONNES, calculs purs
│   ├── handlers/
│   │   ├── mod.rs                                   # MODIF : pub mod evenement_streaming;
│   │   └── evenement_streaming.rs                   # NOUVEAU : ouvrir/rejoindre/cloturer/promouvoir/... + token LiveKit
│   ├── services/
│   │   └── livekit_moderation.rs                    # MODIF : ajouter update_participant_can_publish + retirer_participant
│   └── routes.rs                                    # MODIF : sous-scope /evenements/{id}/direct
│
uafricas_frontend/
├── app/
│   ├── composables/
│   │   └── useEvenements.ts                         # MODIF : etatDirect / rejoindreDirect / cloturer / promouvoir / ...
│   ├── components/
│   │   └── evenements/
│   │       ├── EvenementDirectRoom.vue              # NOUVEAU : conteneur LiveKit (rôle-aware)
│   │       ├── EvenementDirectControls.vue          # NOUVEAU : barre contrôles (diffuseur vs spectateur)
│   │       ├── EvenementDirectChat.vue              # NOUVEAU : chat DataPacket éphémère
│   │       └── EvenementDirectReactions.vue         # NOUVEAU : picker + overlay réactions
│   ├── pages/evenements/
│   │   ├── [id].vue                                 # MODIF : bouton « Rejoindre le direct » / « Ouvrir le direct »
│   │   └── [id]/
│   │       └── direct.vue                           # NOUVEAU : page de la salle de direct
│   ├── plugins/
│   │   └── messagerie.client.ts                     # MODIF : branche dispatch type "event_stream_*"
│   └── mocks/
│       └── notifications.ts                         # MODIF (optionnel) : icône/couleur type "event_*"
```

**Structure Decision**: Application web monorepo (Principe II). Le backend ajoute un module de domaine `evenement_streaming` à plat (pattern identique à `rendez_vous`), branché sous le scope public `/api/evenements` existant. Le frontend ajoute des composants `evenements/` (Tailwind v4 pur) et une page enfant `evenements/[id]/direct.vue`, en réutilisant les patterns LiveKit déjà présents dans `afrolang/` sans modifier ces composants (isolation pour éviter toute régression afrolang).

## Complexity Tracking

> Aucune violation de la Constitution — section vide.
