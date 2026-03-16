# Implementation Plan: Recherche et Exploration de l'Arbre

**Branch**: `001-recherche-exploration` | **Date**: 2026-03-16 | **Spec**: [spec.md](./spec.md)

## Summary

Recherche multi-critères dans son arbre (côté client, instantanée) et dans la base publique (endpoint pg_trgm). Chemin de parenté entre deux personnes avec terminologie française (BFS + LCA côté client). Filtres combinables (géographique, générationnel, branche). Interface unifiée avec toggle "Mon arbre / Tous les arbres". Majoritairement frontend — 1 seul nouvel endpoint backend.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend, 1 endpoint) + TypeScript / Nuxt 4 / Vue 3 (frontend, principal)
**Primary Dependencies**: pg_trgm existant (Feature 4), @vue-flow/core existant
**Storage**: Aucune modification SQL — réutilise colonnes normalisées et indexes de Feature 4
**Testing**: Vérification manuelle
**Target Platform**: Web (SSR Nuxt 4)
**Project Type**: Web application (monorepo)
**Performance Goals**: Recherche locale <500ms, publique <2s, chemin parenté <1s
**Constraints**: Responsive, Tailwind CSS v4 pur
**Scale/Scope**: 1 endpoint backend, 4 nouveaux composants frontend, 1 nouveau composable, 2 fichiers modifiés

## Constitution Check

| Principe | Statut | Justification |
|----------|--------|---------------|
| I. Français d'Abord | PASS | Terminologie familiale en français |
| II. Monorepo Cohérent | PASS | Backend + frontend dans le monorepo |
| III. SQL Source de Vérité | PASS | Aucune modification SQL |
| IV. Sécurité par Défaut | PASS | Endpoint protégé JWT, résultats anonymisés |
| V. Simplicité (YAGNI) | PASS | Recherche locale côté client (données déjà en mémoire), pas de moteur de recherche externe |
| VI. Tailwind CSS v4 | PASS | Tous composants en Tailwind v4 pur |
| VII. Audit & Traçabilité | PASS | Recherche publique = lecture seule, pas d'audit nécessaire |

## Project Structure

### Source Code

```text
uafricas_backend/
├── src/
│   ├── handlers/
│   │   └── matching.rs              # MODIFIER: +handler recherche_publique
│   └── routes.rs                    # MODIFIER: +route /recherche-publique

uafricas_frontend/
├── app/
│   ├── composables/
│   │   └── useRechercheArbre.ts     # NOUVEAU: recherche locale + publique + chemin + filtres
│   ├── components/
│   │   └── arbre-genealogique/
│   │       ├── ChampRecherche.vue   # NOUVEAU: champ unifié + toggle + dropdown
│   │       ├── PanneauChemin.vue    # NOUVEAU: affichage chemin de parenté
│   │       ├── PanneauFiltres.vue   # NOUVEAU: filtres combinables
│   │       └── BarreOutils.vue      # MODIFIER: +champ recherche +bouton filtres
│   └── pages/
│       └── arbre-genealogique/
│           └── visualisation.vue    # MODIFIER: intégrer recherche, chemin, filtres
```

## Complexity Tracking

Aucune violation.
