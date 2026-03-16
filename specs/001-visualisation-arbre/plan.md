# Implementation Plan: Visualisation et Navigation de l'Arbre Généalogique

**Branch**: `001-visualisation-arbre` | **Date**: 2026-03-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-visualisation-arbre/spec.md`

## Summary

Affichage graphique interactif de l'arbre généalogique de l'utilisateur connecté sur une page dédiée (`/arbre-genealogique/visualisation`). Utilise **vue-flow** pour le rendu du graphe (zoom/pan/touch natifs) et **relatives-tree** pour le calcul de layout familial (couples, générations). Affichage progressif (3 générations), modes ascendant/descendant, panneau contextuel au clic, responsive mobile. Nécessite un nouvel endpoint backend `GET /api/arbre/arbre-complet`.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (backend) ; @vue-flow/core, @vue-flow/controls, @vue-flow/minimap, relatives-tree (frontend)
**Storage**: PostgreSQL 16 — schema `arbre_genealogique` existant (aucune migration)
**Testing**: Pas de CI/CD configuré — vérification manuelle
**Target Platform**: Web (SSR Nuxt 4, rendu graphe client-only via `<ClientOnly>`)
**Project Type**: Web application (monorepo frontend + backend)
**Performance Goals**: Chargement interactif < 3s pour 50 personnes, fluide jusqu'à 200 personnes
**Constraints**: Responsive 320px–2560px, touch mobile, JWT authentification obligatoire
**Scale/Scope**: 5-200 personnes par arbre, un arbre par utilisateur

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Justification |
|----------|--------|---------------|
| I. Français d'Abord | PASS | Tous les noms de composants, variables, endpoints, commentaires en français |
| II. Monorepo Cohérent | PASS | Modifications dans `uafricas_backend/` + `uafricas_frontend/` du même monorepo |
| III. SQL Source de Vérité | PASS | Aucune modification SQL. Nouveau DTO backend reflète fidèlement le schema existant |
| IV. Sécurité par Défaut | PASS | Endpoint protégé JWT, utilisateur ne voit que son propre arbre (isolation par `utilisateur_id`) |
| V. Simplicité (YAGNI) | PASS | Deux bibliothèques ciblées (vue-flow + relatives-tree) au lieu d'une solution custom. Pas de sur-abstraction |
| VI. Tailwind CSS v4 | PASS | Page publique (derrière auth mais pas admin) → Tailwind CSS v4 pur, pas de daisyUI |
| VII. Audit & Traçabilité | PASS | Endpoint en lecture seule → pas de mutation → pas d'audit nécessaire |

**Post-Phase 1 re-check** : Tous les principes restent respectés. Le nouveau composable `useLayoutArbre.ts` est un composable par domaine (Principe V). Les composants suivent le pattern existant (Principe V).

## Project Structure

### Documentation (this feature)

```text
specs/001-visualisation-arbre/
├── plan.md              # Ce fichier
├── spec.md              # Spécification fonctionnelle
├── research.md          # Recherche technique (5 décisions)
├── data-model.md        # Modèle de données (DTO + structures frontend)
├── quickstart.md        # Guide de démarrage rapide
├── contracts/
│   └── api-arbre-complet.md  # Contrat API nouvel endpoint
├── checklists/
│   └── requirements.md  # Checklist qualité spec
└── tasks.md             # (à générer via /speckit.tasks)
```

### Source Code (repository root)

```text
uafricas_backend/
├── src/
│   ├── models/
│   │   └── arbre_genealogique.rs    # + ArbreCompletResponse, PersonneNoeud, LienArbreResponse
│   ├── handlers/
│   │   └── arbre_genealogique.rs    # + obtenir_arbre_complet handler
│   └── routes.rs                    # + route /arbre-complet

uafricas_frontend/
├── app/
│   ├── pages/
│   │   └── arbre-genealogique/
│   │       ├── index.vue            # Modifier: ajouter lien vers visualisation
│   │       └── visualisation.vue    # NOUVEAU: page de visualisation
│   ├── components/
│   │   └── arbre-genealogique/
│   │       ├── PersonneCard.vue     # Existant (Feature 1)
│   │       ├── PersonneForm.vue     # Existant (Feature 1)
│   │       ├── LienFamilialForm.vue # Existant (Feature 1)
│   │       ├── ArbreGraphe.vue      # NOUVEAU: wrapper vue-flow
│   │       ├── NoeudPersonne.vue    # NOUVEAU: custom node vue-flow
│   │       ├── PanneauPersonne.vue  # NOUVEAU: panneau contextuel
│   │       └── BarreOutils.vue      # NOUVEAU: modes + controls
│   ├── composables/
│   │   ├── useArbreGenealogique.ts  # Modifier: + obtenirArbreComplet()
│   │   └── useLayoutArbre.ts        # NOUVEAU: conversion données → layout vue-flow
│   └── mocks/
│       └── arbre-genealogique.ts    # Modifier: + types/mock arbre complet
```

**Structure Decision** : Web application (Option 2). Modifications dans les deux couches du monorepo existant. Backend : 3 fichiers modifiés (models, handlers, routes). Frontend : 4 nouveaux composants + 1 nouveau composable + 1 nouvelle page + 3 fichiers modifiés.

## Complexity Tracking

Aucune violation de la constitution — pas de justification de complexité nécessaire.
