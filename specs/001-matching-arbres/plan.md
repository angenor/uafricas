# Implementation Plan: Matching et Découverte de Parents

**Branch**: `001-matching-arbres` | **Date**: 2026-03-16 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-matching-arbres/spec.md`

## Summary

Feature clé de la plateforme. Algorithme de matching inter-arbres comparant les personnes par similarité de nom (pg_trgm + normalisation phonétique africaine), lieu et date. Exécution hybride : vérification synchrone rapide (nom exact) + tâche de fond profonde (tokio::spawn). Score composite pondéré (nom 35%, prénoms 20%, date 15%, lieu 20%, genre 10%, seuil 55%). Nouveau schema SQL (suggestions_correspondance + demandes_contact + colonnes normalisées). 7 endpoints API. Page frontend Découvertes (3 sections). Branches découvertes en lecture seule dans la visualisation.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) + TypeScript / Nuxt 4 / Vue 3 (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx, pg_trgm (PostgreSQL extension), tokio::spawn, frontend: @vue-flow/core (existant)
**Storage**: PostgreSQL 16 : schema `arbre_genealogique` étendu (2 nouvelles tables + 2 colonnes + 3 indexes)
**Testing**: Pas de CI/CD : vérification manuelle avec 2 comptes test
**Target Platform**: Web (SSR Nuxt 4)
**Project Type**: Web application (monorepo frontend + backend)
**Performance Goals**: Matching rapide <10ms, matching profond <30s pour 10k personnes, branches visibles <5s après confirmation
**Constraints**: Confidentialité (anonymat avant confirmation mutuelle), extensions PostgreSQL (pg_trgm)
**Scale/Scope**: 10 000 personnes max initialement, 7 endpoints API, 1 page frontend, ~10 fichiers backend, ~7 fichiers frontend

## Constitution Check

| Principe | Statut | Justification |
|----------|--------|---------------|
| I. Français d'Abord | PASS | Code, variables, messages, colonnes SQL en français |
| II. Monorepo Cohérent | PASS | Backend + frontend dans le même monorepo, contrats cohérents |
| III. SQL Source de Vérité | PASS | Nouveau SQL d'abord (24_matching.sql), puis modèles Rust, puis types TS |
| IV. Sécurité par Défaut | PASS | JWT sur tous les endpoints, anonymat avant confirmation, isolation par arbre |
| V. Simplicité (YAGNI) | PASS | tokio::spawn au lieu de job queue, pg_trgm au lieu de moteur de recherche externe |
| VI. Tailwind CSS v4 | PASS | Page Découvertes en Tailwind v4 pur |
| VII. Audit & Traçabilité | PASS | Confirmations/rejets auditées via log_action existant |

## Project Structure

### Documentation

```text
specs/001-matching-arbres/
├── plan.md
├── spec.md
├── research.md          # 6 décisions techniques
├── data-model.md        # 2 nouvelles tables + colonnes + DTOs
├── quickstart.md
├── contracts/
│   └── api-matching.md  # 7 endpoints
├── checklists/
│   └── requirements.md
└── tasks.md             # (à générer via /speckit.tasks)
```

### Source Code

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                    # MODIFIER: ajouter \ir schemas/24_matching.sql
│   └── schemas/
│       └── 24_matching.sql           # NOUVEAU: tables + indexes + extension pg_trgm
├── src/
│   ├── models/
│   │   ├── mod.rs                    # MODIFIER: pub mod matching
│   │   ├── arbre_genealogique.rs     # MODIFIER: + nom_normalise, prenoms_normalise
│   │   └── matching.rs              # NOUVEAU: structs + DTOs
│   ├── handlers/
│   │   ├── mod.rs                    # MODIFIER: pub mod matching
│   │   ├── arbre_genealogique.rs     # MODIFIER: normalisation + tokio::spawn matching
│   │   └── matching.rs              # NOUVEAU: 7 handlers
│   ├── services/
│   │   ├── mod.rs                    # MODIFIER: pub mod matching
│   │   └── matching.rs              # NOUVEAU: normalisation + scoring + matching
│   └── routes.rs                    # MODIFIER: scope /decouvertes

uafricas_frontend/
├── app/
│   ├── pages/
│   │   └── arbre-genealogique/
│   │       ├── decouvertes.vue       # NOUVEAU: page Découvertes
│   │       └── visualisation.vue     # MODIFIER: branches découvertes
│   ├── components/
│   │   └── arbre-genealogique/
│   │       ├── CarteSuggestion.vue   # NOUVEAU: carte de suggestion
│   │       ├── SectionDecouvertes.vue# NOUVEAU: section paginée
│   │       └── NoeudPersonne.vue     # MODIFIER: style branche découverte
│   ├── composables/
│   │   └── useDecouvertes.ts         # NOUVEAU: API wrapper
│   └── mocks/
│       └── matching.ts              # NOUVEAU: types + mock
```

**Structure Decision** : Full-stack. Backend : 1 nouveau fichier SQL, 3 nouveaux modules Rust (models, handlers, services), 4 fichiers modifiés. Frontend : 1 nouvelle page, 2 nouveaux composants, 1 nouveau composable, 2 fichiers modifiés.

## Complexity Tracking

Aucune violation de la constitution.
