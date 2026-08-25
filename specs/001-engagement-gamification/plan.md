# Implementation Plan: Système d'engagement / gamification AFRICANS, Phase 1

**Branch**: `001-engagement-gamification` | **Date**: 2026-07-06 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-engagement-gamification/spec.md`

## Summary

Poser la **fondation** du système d'engagement AFRICANS : un **compte d'engagement** 1‑pour‑1 par utilisateur (solde de points global, solde mensuel, réputation séparée, niveau dérivé), un **journal des points** immuable, un **barème paramétrable en base** (règles, paliers de popularité, seuils de niveaux) et un **service d'attribution non-bloquant** calqué sur `services/audit.rs`. Le barème câblé se limite aux règles **vérifiables côté serveur** : contribution validée par modération (Codimoi, VidAfrica, Ideaforces, BadGoodHabit), FactCheck correct/faux, et paliers de « j'aime » agrégés via une référence unifiée `(type_objet, objet_id)`. Côté UI : une vue publique « Mes points / mon statut / mes badges » (Tailwind pur) et un back-office de gestion des règles + consultation du journal (daisyUI). Anti-abus par idempotence (clé unique), plafonds journaliers/mensuels et interdiction d'auto-attribution.

Nouveau schéma bounded-context **`engagement`** (justifié : domaine transversal distinct, cf. Contraintes Techniques de la constitution).

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 (Vue 3 SSR) (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde (backend) ; Pinia, Tailwind CSS v4 (pur, public) + daisyUI v5 (admin), FontAwesome (frontend). **Aucune dépendance nouvelle.**
**Storage**: PostgreSQL 16 : nouveau schéma `engagement` ; migration idempotente numérotée sous `uafricas_backend/doc/bd/schemas/`
**Testing**: Aucun harnais configuré (constitution : pas de testing/CI). Vérification par `cargo check` + diagnostics LSP + quickstart manuel.
**Target Platform**: Serveur Linux (backend Actix sur :8080/8082), navigateur (frontend Nuxt SSR :3000)
**Project Type**: Web application (monorepo frontend + backend)
**Performance Goals**: Attribution reflétée en < 5 s dans la vue « Mes points » (SC-001) ; l'attribution ne dégrade jamais la latence de l'action métier (non-bloquante).
**Constraints**: Attribution **non-bloquante** (FR-007) ; **idempotence** stricte (FR-008) ; **zéro doublon** (SC-004) ; barème modifiable **sans redéploiement** (FR-022, SC-005).
**Scale/Scope**: 1 compte par utilisateur ; journal en croissance continue (append-only) ; ~4 domaines de contribution + ~5 tables de réactions à instrumenter ; 2 écrans publics (panneau + badge) + 2 écrans admin (règles, journal).

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Justification |
|----------|--------|---------------|
| I. Français d'Abord | ✅ | Tables/colonnes/enums snake_case français, UI et messages en français. |
| II. Monorepo Cohérent | ✅ | Livraison cross-stack (SQL → Rust → TS) dans la même feature ; types alignés. |
| III. SQL Source de Vérité | ✅ | Migration SQL d'abord ; structs `FromRow` + types TS reflètent le schéma. |
| IV. Sécurité par Défaut | ✅ | JWT sur tous les endpoints ; requêtes paramétrées sqlx ; garde anti-auto-attribution ; nouvelle permission admin `engagement` ; aucun secret. |
| V. Simplicité (YAGNI) | ✅ | Pas de table `palier_attribué` séparée (l'unicité de la clé d'idempotence sur le journal suffit) ; un seul module service `engagement.rs` calqué sur `audit.rs` ; pas de nouveau pattern. |
| VI. Tailwind v4 / daisyUI admin | ✅ | Vue publique « Mes points » + badge en Tailwind pur ; écrans admin en daisyUI. |
| VII. Audit & Traçabilité | ✅ | `audit::log_action` sur toute modification de barème et tout ajustement manuel de points. Le journal des points est lui-même une trace métier dédiée. |

**Aucune violation** → *Complexity Tracking* non renseigné.

## Project Structure

### Documentation (this feature)

```text
specs/001-engagement-gamification/
├── plan.md              # Ce fichier
├── research.md          # Phase 0 : décisions techniques
├── data-model.md        # Phase 1 : schéma engagement + entités
├── quickstart.md        # Phase 1 : scénario de vérification manuel
├── contracts/
│   └── engagement-api.md # Phase 1, contrats REST (public + admin)
└── checklists/
    └── requirements.md   # Checklist qualité de la spec (déjà créée)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── NN_engagement.sql              # NOUVEAU, schéma + tables + seed barème (prochain n° libre, ≈33)
├── src/
│   ├── services/
│   │   ├── mod.rs                     # + pub mod engagement;
│   │   └── engagement.rs             # NOUVEAU, moteur non-bloquant (attribuer / retirer / evaluer_popularite / recalculer_niveau)
│   ├── models/
│   │   ├── engagement.rs             # NOUVEAU, Compte, Mouvement, Regle, Palier, Niveau (public DTO)
│   │   └── admin/
│   │       └── engagement.rs         # NOUVEAU, DTO admin (règles/paliers/niveaux/journal, Create/Modifier)
│   ├── handlers/
│   │   ├── engagement.rs             # NOUVEAU, endpoints publics (mon-compte, mon-journal, niveau/{id})
│   │   └── admin/
│   │       └── engagement.rs         # NOUVEAU, endpoints admin (règles, paliers, niveaux, journal, ajustement)
│   └── routes.rs                     # + routes /api/engagement et /api/admin/engagement
│                                     # + points d'appel du service dans les handlers de modération/réaction existants (voir data-model.md §Intégration)

uafricas_frontend/
├── app/
│   ├── composables/
│   │   ├── useEngagement.ts          # NOUVEAU, public (mon compte, mon journal, badge d'un membre)
│   │   └── useAdminEngagement.ts     # NOUVEAU, admin (CRUD barème + journal global)
│   ├── components/
│   │   ├── engagement/
│   │   │   ├── MesPointsPanel.vue     # NOUVEAU, Tailwind pur (solde/statut/réputation/historique)
│   │   │   └── BadgeStatut.vue        # NOUVEAU, Tailwind pur (badge réutilisable profil + sous contenus)
│   │   └── admin/engagement/
│   │       ├── ReglesBaremeTable.vue  # NOUVEAU, daisyUI
│   │       └── JournalPointsTable.vue # NOUVEAU, daisyUI
│   └── pages/
│       ├── mon-compte/profil.vue      # + onglet « Mes points » (dropdown d'onglets existant)
│       └── admin/engagement/
│           ├── regles.vue             # NOUVEAU, daisyUI
│           └── journal.vue            # NOUVEAU, daisyUI
```

**Structure Decision**: Application web monorepo (Option 2). On suit strictement les conventions existantes : un fichier handler/model par domaine, séparation public / `admin/`, service transverse dans `src/services/` calqué sur `audit.rs`, composables `useEngagement` (public) / `useAdminEngagement` (admin), badge et panneau publics en Tailwind pur, écrans admin en daisyUI. Le point d'intégration le plus étendu (appels du service depuis les handlers de modération et de réaction existants) est catalogué dans `data-model.md §Intégration` : il ne crée pas de nouvelle abstraction, seulement des appels non-bloquants ajoutés aux mutations déjà en place.

## Complexity Tracking

> Aucune violation de la constitution à justifier.
