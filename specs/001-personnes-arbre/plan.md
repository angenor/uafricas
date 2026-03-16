# Implementation Plan: Modèle de données des personnes et liens familiaux

**Branch**: `001-personnes-arbre` | **Date**: 2026-03-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/001-personnes-arbre/spec.md`

## Summary

Implémenter le modèle de données fondateur de l'arbre généalogique : CRUD des personnes (données biographiques à granularité partielle), liens familiaux (parent-enfant avec rôle, conjoint), et l'architecture Personne réelle / Rattachement qui permettra le futur matching inter-arbres. Chaque utilisateur travaille exclusivement sur ses propres fiches dans cette feature — pas de partage cross-users exposé.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL async), uuid, chrono, serde — frontend : Nuxt 4, Pinia, $fetch
**Storage**: PostgreSQL 16 — nouveau schema `arbre_genealogique` (11e schema bounded-context)
**Testing**: Pas de CI/CD configuré — validation manuelle via Adminer + appels API directs
**Target Platform**: Linux server (Docker), navigateur web (SSR + CSR Nuxt 4)
**Project Type**: Web service (Actix-Web REST API) + Web application (Nuxt 4 SSR)
**Performance Goals**: Liste 500 personnes < 1 seconde (SC-003) ; création fiche < 2 min UX (SC-001)
**Constraints**: UUID v4 PKs, soft delete (`deleted_at`), TIMESTAMPTZ, snake_case français, JWT requis (access 15min), audit toutes mutations
**Scale/Scope**: 1 arbre par utilisateur, illimité en taille, pagination standard projet

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Vérification |
|----------|--------|--------------|
| I. Français d'abord | ✅ PASS | Toutes les variables, colonnes SQL, routes, messages UI en français |
| II. Monorepo cohérent | ✅ PASS | Ajout dans `uafricas_backend/` + `uafricas_frontend/` — même commit cross-stack |
| III. SQL source de vérité | ✅ PASS | Schema SQL défini en premier ; structs Rust + interfaces TS dérivent du SQL |
| IV. Sécurité par défaut | ✅ PASS | JWT obligatoire sur toutes les routes ; validation backend de toutes les entrées |
| V. Simplicité (YAGNI) | ✅ PASS | Handlers directs sans Repository pattern ; composable unique `useArbreGenealogique` |
| VI. Tailwind v4 (daisyUI admin) | ✅ PASS | Pages publiques en Tailwind v4 pur ; daisyUI réservé aux composants admin si nécessaire |
| VII. Audit & Traçabilité | ✅ PASS | `audit::log_action` sur toutes les mutations (5 handlers : créer/modifier/supprimer personne, créer/supprimer lien) |

**Complexité ajoutée justifiée** :
- Nouveau schema `arbre_genealogique` : domaine distinct (généalogie), aucun des 10 schemas existants ne couvre ce contexte. Justification conforme au Principe III.
- Détection de cycle (requête récursive CTE) : obligatoire pour FR-009 (aucun lien circulaire). Aucune alternative plus simple ne couvre le cas général sur N générations.

## Project Structure

### Documentation (this feature)

```text
specs/001-personnes-arbre/
├── plan.md              ← Ce fichier
├── research.md          ← Décisions techniques Phase 0
├── data-model.md        ← Schéma SQL + structs Rust + interfaces TS
├── quickstart.md        ← Guide démarrage développeur
├── contracts/
│   └── api.md           ← Contrats endpoints REST
├── checklists/
│   └── requirements.md  ← Checklist qualité spec
└── tasks.md             ← Généré par /speckit.tasks (non créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 23_arbre_genealogique.sql    ← Nouveau schema SQL (source de vérité)
├── src/
│   ├── handlers/
│   │   └── arbre_genealogique.rs    ← CRUD personnes + rattachements + liens
│   ├── models/
│   │   └── arbre_genealogique.rs    ← Structs FromRow, DTOs, COLONNES
│   └── routes.rs                   ← Ajout scope /api/arbre

uafricas_frontend/
└── app/
    ├── mocks/
    │   └── arbre-genealogique.ts    ← Interfaces TS + données mock
    ├── composables/
    │   └── useArbreGenealogique.ts  ← Composable public (CRUD + liens)
    ├── pages/
    │   └── arbre-genealogique/
    │       ├── index.vue            ← Liste des personnes de l'arbre
    │       └── [id].vue             ← Fiche détail d'une personne
    └── components/
        └── arbre-genealogique/
            ├── PersonneCard.vue     ← Carte personne dans la liste
            ├── PersonneForm.vue     ← Formulaire création/édition
            └── LienFamilialForm.vue ← Formulaire ajout lien
```

**Structure Decision** : Option 2 (Web application) — backend Rust dans `uafricas_backend/` et frontend Nuxt dans `uafricas_frontend/`, reflétant l'architecture monorepo existante.

## Phase 0 — Research

→ Voir [research.md](./research.md) pour les décisions et rationales détaillées.

**Résumé des décisions clés** :

1. **Schema PostgreSQL** : Nouveau schema `arbre_genealogique` (numéro 23). Justification : domaine généalogique sans équivalent dans les 10 schemas existants.
2. **Dates à granularité variable** : 3 colonnes séparées par date (annee, mois, jour), toutes SMALLINT nullable. Validation partielle possible côté SQL + backend.
3. **Détection de cycle** : Recursive CTE `WITH RECURSIVE ancetres AS (...)` au moment de la création d'un lien parent-enfant.
4. **Soft delete en cascade** : Trigger PostgreSQL ou logique applicative en Rust lors de la suppression du dernier rattachement d'une Personne.
5. **Arbre auto-créé** : L'arbre de l'utilisateur est créé automatiquement lors du premier ajout de personne (pas d'endpoint séparé de création d'arbre).

## Phase 1 — Design & Contracts

→ Voir [data-model.md](./data-model.md) pour le schéma SQL complet, structs Rust et interfaces TypeScript.
→ Voir [contracts/api.md](./contracts/api.md) pour les contrats d'API REST.
→ Voir [quickstart.md](./quickstart.md) pour le guide de démarrage.

### Résumé des entités

| Entité SQL | Table | Clé |
|-----------|-------|-----|
| Personne réelle | `arbre_genealogique.personnes` | UUID, soft delete |
| Arbre généalogique | `arbre_genealogique.arbres` | UUID, 1 par utilisateur |
| Rattachement | `arbre_genealogique.rattachements` | UUID, UNIQUE(arbre_id, personne_id) |
| Lien familial | `arbre_genealogique.liens_familiaux` | UUID, UNIQUE(arbre_id, ratt_a, ratt_b, type_lien) |

### Résumé des endpoints

| Méthode | Route | Description |
|---------|-------|-------------|
| GET | `/api/arbre/personnes` | Liste paginée des personnes de l'arbre |
| POST | `/api/arbre/personnes` | Créer une personne (+ rattachement auto) |
| GET | `/api/arbre/personnes/:id` | Détail + liens directs |
| PUT | `/api/arbre/personnes/:id` | Modifier les infos d'une personne |
| DELETE | `/api/arbre/personnes/:id` | Supprimer rattachement (+ cascade si dernier) |
| POST | `/api/arbre/liens` | Créer un lien familial |
| DELETE | `/api/arbre/liens/:id` | Supprimer un lien familial |
