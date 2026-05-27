# Implementation Plan: Modération de session Afrolang — mise en évidence et permissions tableau blanc

**Branch**: `001-session-moderation` | **Date**: 2026-05-10 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-session-moderation/spec.md`

## Summary

Ajouter à toute session Afrolang (publique ou privée) deux leviers de modération en temps réel :
1. **Permissions tableau blanc** — par défaut seuls les modérateurs de session écrivent ; un admin peut accorder/retirer individuellement le droit d'écriture aux participants.
2. **Mise en évidence (spotlight)** — uniquement dans les sessions publiques livestreamées, un admin plateforme ou admin de salle peut désigner un participant comme « en vedette ».

L'approche technique repose sur :
- **Persistance** : 1 nouvelle table `afrolang.session_permission_tableau_blanc` (état session-scoped) + 3 colonnes sur `afrolang.session` (`participant_mis_en_evidence_id`, `mis_en_evidence_par`, `mis_en_evidence_at`). L'`ON DELETE CASCADE` existant nettoie automatiquement à la clôture.
- **Enforcement** : utiliser le mécanisme natif LiveKit `RoomService::update_participant` (via `livekit-api` déjà en dépendance) pour basculer `can_publish_data` à chaque mutation de permission → le SFU rejette lui-même les data packets non autorisés (FR-015 satisfait sans relais applicatif custom).
- **Propagation temps réel** : message data LiveKit `{type: 'moderation', subtype: 'permission_update'|'spotlight'}` publié par le backend après chaque mutation, écouté par tous les clients (panneau et UI tableau blanc/grille vidéo se mettent à jour < 2 s, FR-014/FR-023).
- **Frontend** : extension de `useAfrolang`, nouveau composant `SalleModerationPanel.vue` (Tailwind v4 pur, principe VI), adaptation de `AfrolangWhiteboard.vue` (état lecture-seule visuel) et `AfrolangVideoGrid.vue` (mise en évidence visuelle).

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) + TypeScript / Nuxt 4 / Vue 3 SSR (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde, **livekit-api** (déjà présent — utilisé pour token signing aujourd'hui, étendu pour `RoomServiceClient::update_participant`) ; Pinia, $fetch, FontAwesome, **livekit-client** (déjà présent côté Vue pour `DataPacket_Kind.RELIABLE`)
**Storage**: PostgreSQL 16 — schema `afrolang` existant (1 nouvelle table + 3 colonnes sur `session`)
**Testing**: aucun (pas de CI/CD configuré — section « Pas de linting, testing ni CI/CD » de la constitution) ; validation manuelle via quickstart
**Target Platform**: backend Linux server (Actix-Web port 8080) + frontend SSR Nuxt 4 (port 3000) ; livestream via LiveKit (serveur Docker local en dev, Cloud LiveKit en prod)
**Project Type**: monorepo web (frontend + backend, principe II)
**Performance Goals**: propagation modération ≤ 2 s p95 (SC-002) ; aucun impact mesurable sur le débit vidéo LiveKit existant
**Constraints**: pas de relais applicatif des opérations whiteboard (rester sur le canal data LiveKit existant) ; pas de cache Redis ni broker tiers (principe V) ; permissions purement session-scoped (Q3 clarification)
**Scale/Scope**: ~50 participants max par session (champ `max_participants` existant) ; ~5-10 endpoints REST nouveaux ; 1 nouveau composant Vue + extension de 3 composants existants

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Vérification | Statut |
|---|---|---|
| I. Français d'abord | Noms SQL, structs Rust, composants Vue, messages UI, audit → tous en français | Conforme |
| II. Monorepo cohérent | Modifications backend + frontend dans le même PR ; types TS ↔ structs Rust ↔ SQL alignés sur DTO `PermissionTableauBlanc`, `SpotlightSession` | Conforme |
| III. SQL source de vérité | DDL en premier (1 nouvelle table + 3 colonnes), puis backend, puis frontend ; conventions UUID v4 + TIMESTAMPTZ + snake_case français respectées ; pas de `deleted_at` car données session-scoped supprimées par CASCADE | Conforme |
| IV. Sécurité par défaut | JWT inchangé ; sqlx paramétré ; enforcement LiveKit `can_publish_data` server-side (pas de filtrage client-only) ; FR-015 garanti par le SFU | Conforme |
| V. Simplicité (YAGNI) | Pas de Redis/broker ; pas d'abstraction Repository ; 5 endpoints REST simples ; pas de persistance multi-session (Q3 deferred) | Conforme |
| VI. Tailwind v4 / daisyUI back-office | Panneau modération inclus dans `AfrolangRoom.vue` (public) = Tailwind v4 pur ; aucun composant admin créé (modération gérée en session, pas dans `/admin`) | Conforme |
| VII. Audit & traçabilité | `audit::log_action` appelé sur chaque mutation (accord/retrait permission, spotlight on/off) ; before/after JSONB capturent participant cible + auteur | Conforme |

**Résultat** : aucune violation. Aucune entrée dans Complexity Tracking.

## Project Structure

### Documentation (this feature)

```text
specs/001-session-moderation/
├── plan.md              # Ce fichier
├── research.md          # Phase 0 — choix d'architecture (LiveKit enforcement, état session)
├── spec.md              # Feature specification
├── data-model.md        # Phase 1 — DDL nouvelle table + colonnes session
├── quickstart.md        # Phase 1 — scénarios de validation manuelle
├── contracts/
│   └── api-rest.md      # Phase 1 — 5 endpoints REST + format data packets LiveKit
└── checklists/
    └── requirements.md  # Spec quality checklist
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 08b_afrolang.sql              # +1 table, +3 colonnes sur session
├── src/
│   ├── models/
│   │   └── afrolang.rs               # +structs PermissionTableauBlanc, SpotlightInfo, AccorderPermissionPayload
│   ├── handlers/
│   │   └── afrolang.rs               # +5 handlers : lister/accorder/retirer permissions, spotlight on/off
│   ├── services/
│   │   └── livekit_moderation.rs     # NEW — wrapper livekit-api : update_participant(can_publish_data), publish_data(moderation message)
│   └── routes.rs                     # +5 routes scoped sous /api/afrolang/sessions/{id}/...

uafricas_frontend/
├── app/
│   ├── composables/
│   │   └── useAfrolang.ts            # +interfaces + 5 méthodes : listerPermissionsTableauBlanc, accorderPermissionTableauBlanc, retirerPermissionTableauBlanc, mettreEnEvidence, retirerMiseEnEvidence + listener data packet 'moderation'
│   └── components/
│       └── afrolang/
│           ├── SalleModerationPanel.vue     # NEW — panneau permissions + spotlight (Tailwind v4 pur)
│           ├── AfrolangRoom.vue             # +intégration panneau (visible pour modérateurs uniquement)
│           ├── AfrolangWhiteboard.vue       # +état visuel lecture-seule + désactivation barre d'outils
│           └── AfrolangVideoGrid.vue        # +affichage spotlight (mise en avant centrale + bordure + libellé)
```

**Structure Decision** : extension chirurgicale du domaine `afrolang` existant — aucun nouveau dossier, réutilisation maximale des chemins établis par les features 005 et 006. Le nouveau module `services/livekit_moderation.rs` côté backend est isolé pour confiner l'usage de `livekit-api::RoomServiceClient` (déjà utilisé pour la génération de token).

## Phase 0 — Research

Voir [research.md](./research.md). Sujets traités :
- Enforcement serveur des permissions data (LiveKit `RoomService::update_participant` vs canal applicatif custom)
- Persistance de l'état spotlight (colonnes sur `session` vs table dédiée)
- Propagation temps réel des mutations (DataPacket vs WebSocket applicatif vs polling)
- Cible « tous les participants connectés » du spotlight (jointure session_participant vs liste LiveKit live)

## Phase 1 — Design & Contracts

- **Data model** : [data-model.md](./data-model.md) — DDL complet de `session_permission_tableau_blanc`, ALTER sur `session`, contraintes et indexes.
- **Contracts** : [contracts/api-rest.md](./contracts/api-rest.md) — 5 endpoints REST (request/response JSON) + 2 formats de DataPacket LiveKit (`moderation.permission_update`, `moderation.spotlight`).
- **Quickstart** : [quickstart.md](./quickstart.md) — 7 scénarios manuels alignés sur les Acceptance Scenarios de la spec.
- **Agent context** : `CLAUDE.md` mis à jour via `.specify/scripts/bash/update-agent-context.sh claude`.

## Complexity Tracking

> Aucune violation de la constitution. Section non remplie.
