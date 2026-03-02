# Implementation Plan: Partage Public des Avis de Recherche

**Branch**: `002-partage-avis-recherche` | **Date**: 2026-03-02 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/002-partage-avis-recherche/spec.md`

## Summary

Rendre les avis de recherche "Retrouve Amis" publiquement accessibles et partageables sur les réseaux sociaux (WhatsApp, Facebook, X/Twitter, LinkedIn) afin d'augmenter les chances de retrouvailles au-delà des utilisateurs de la plateforme. La fonctionnalité inclut : pages publiques avec Open Graph/Twitter Card, boutons de partage avec compteur intégré, formulaire de réponse structuré créant des correspondances, page de listing/recherche publique, et protections anti-harcèlement (anonymisation, signalement, demande de retrait avec arbitrage admin 72h).

## Technical Context

**Language/Version**: Rust (Edition 2024) + TypeScript (Nuxt 4 / Vue 3 SSR)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), Nuxt 4, Pinia, Tailwind CSS v4
**Storage**: PostgreSQL 16, schema `retrouve_amis` existant (6 tables + 1 fonction PL/pgSQL)
**Testing**: Pas de framework de test configuré (ni frontend ni backend)
**Target Platform**: Web SSR (Nuxt 4 server-side rendering), domaine www.africans-world.org
**Project Type**: Web application (monorepo frontend + backend)
**Performance Goals**: Pages publiques < 2s de chargement, partage social instantané, listing paginé fluide
**Constraints**: Site public = Tailwind CSS v4 pur (pas de daisyUI), français obligatoire, audit de toutes les mutations
**Scale/Scope**: ~500 avis actifs initialement, croissance attendue avec le partage social

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Détail |
|----------|--------|--------|
| I. Français d'Abord | ✅ PASS | Tout le code, variables, UI, SQL en français |
| II. Monorepo Cohérent | ✅ PASS | Modifications dans `uafricas_frontend/` et `uafricas_backend/` du même repo |
| III. SQL Source de Vérité | ✅ PASS | Modifications SQL d'abord → backend → frontend. Extension du schema `retrouve_amis` existant |
| IV. Sécurité par Défaut | ✅ PASS | Anonymisation auteur (prénom+initiale), JWT pour mutations, signalement connecté uniquement, retrait avec suspension immédiate |
| V. Simplicité (YAGNI) | ✅ PASS | Extension des tables existantes (ALTER TABLE), réutilisation du système de correspondances, pas de nouvelle abstraction |
| VI. Tailwind CSS v4 (daisyUI back-office) | ✅ PASS | Pages publiques = Tailwind CSS v4 pur (pas de daisyUI). Admin = daisyUI autorisé |
| VII. Audit & Traçabilité | ✅ PASS | Toutes les nouvelles mutations (partage, réponse, retrait, modération) auditées via `audit::log_action` |

**Gate Result (pré-Phase 0)**: ✅ ALL PASS — Aucune violation. Procéder à Phase 0.

### Re-check post-Phase 1 Design

| Principe | Statut | Vérification post-design |
|----------|--------|--------------------------|
| I. Français d'Abord | ✅ PASS | Tables `reponse_publique`, `demande_retrait`, enums `type_reponse_publique`, `etat_demande_retrait`, colonnes `est_public`, `compteur_partages` — tout en français |
| II. Monorepo Cohérent | ✅ PASS | data-model.md (SQL) + contracts/ (REST API) + quickstart.md (frontend+backend) cohérents |
| III. SQL Source de Vérité | ✅ PASS | data-model.md définit le schema en premier → contrats API dérivés → types TS à dériver |
| IV. Sécurité par Défaut | ✅ PASS | Anonymisation (FR-008), JWT pour mutations, rate limits (1/avis + 10/jour), UNIQUE constraints anti-spam, suspension immédiate retrait |
| V. Simplicité (YAGNI) | ✅ PASS | 3 colonnes + 2 tables. Compteur simple (pas d'analytics par réseau). Réutilisation correspondances existantes |
| VI. Tailwind CSS v4 | ✅ PASS | Composants publics spécifiés sans daisyUI. Admin demandes-retrait = daisyUI autorisé |
| VII. Audit & Traçabilité | ✅ PASS | 7 nouvelles mutations identifiées : publier, répondre, signaler, retrait, partage, statuer_retrait×2 sens |

**Gate Result (post-Phase 1)**: ✅ ALL PASS — Design conforme à la constitution.

## Project Structure

### Documentation (this feature)

```text
specs/002-partage-avis-recherche/
├── spec.md              # Spécification fonctionnelle (complétée)
├── plan.md              # Ce fichier
├── research.md          # Phase 0 — recherche et décisions techniques
├── data-model.md        # Phase 1 — modèle de données (extensions SQL)
├── quickstart.md        # Phase 1 — guide de démarrage rapide
├── contracts/           # Phase 1 — contrats d'API (endpoints REST)
│   ├── public-api.md    # Endpoints publics (sans auth)
│   └── auth-api.md      # Endpoints authentifiés (réponse, signalement, retrait)
└── tasks.md             # Phase 2 — tâches ordonnées (/speckit.tasks)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 16_retrouve_amis.sql          # ALTER TABLE + nouvelles tables (demande_retrait, reponse_publique)
├── src/
│   ├── models/
│   │   ├── retrouve_amis.rs           # Nouveaux structs : ReponsePublique, DemandeRetrait, DTOs publics
│   │   └── admin/retrouve_amis.rs     # Admin DTOs pour demandes de retrait
│   ├── handlers/
│   │   ├── retrouve_amis.rs           # Nouveaux handlers publics (page publique, réponse, listing)
│   │   ├── retrouve_amis_public.rs    # Handlers sans auth (page publique, listing, partage)
│   │   └── admin/retrouve_amis.rs     # Nouveaux handlers admin (demande retrait, modération)
│   └── routes.rs                      # Nouvelles routes publiques et authentifiées

uafricas_frontend/
├── app/
│   ├── pages/retrouve-amis/
│   │   ├── public/
│   │   │   └── [slug].vue             # Page publique d'un avis (SSR, Open Graph)
│   │   └── rechercher.vue             # Page de listing/recherche publique
│   ├── components/retrouve-amis/
│   │   ├── PagePublique.vue           # Composant page publique (affichage avis)
│   │   ├── BoutonsPartage.vue         # Boutons WhatsApp/Facebook/X/LinkedIn/Copier
│   │   ├── FormulaireReponse.vue      # Formulaire réponse structurée
│   │   ├── CarteAvisPublic.vue        # Carte résumé pour le listing
│   │   └── DemandeRetrait.vue         # Bouton + formulaire demande de retrait
│   └── composables/
│       └── useRetrouvAmis.ts          # Extension avec fonctions publiques
```

**Structure Decision**: Extension du code existant dans le monorepo. Séparation des handlers publics (sans auth) dans un fichier dédié `retrouve_amis_public.rs` pour clarifier la distinction auth/no-auth. Les pages publiques utilisent le routing Nuxt 4 SSR pour le SEO (server-side rendering des balises Open Graph).

## Complexity Tracking

> Aucune violation de constitution détectée. Pas de justification nécessaire.
