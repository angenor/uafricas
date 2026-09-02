# Implementation Plan: Édition Interactive de l'Arbre Généalogique

**Branch**: `001-edition-arbre` | **Date**: 2026-03-16 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-edition-arbre/spec.md`

## Summary

Ajout de l'édition interactive directement depuis la page de visualisation de l'arbre (Feature 2). Boutons d'action dans le panneau contextuel existant (ajouter parent/enfant/conjoint, modifier, supprimer). Formulaire guidé dans le panneau latéral/bottom sheet. Indicateurs visuels de branches incomplètes sur les nœuds + compteur global. **Feature purement frontend** : aucun nouvel endpoint backend, aucune migration SQL. Réutilise les API CRUD existantes (Feature 1) et les composants de visualisation (Feature 2).

## Technical Context

**Language/Version**: TypeScript / Nuxt 4 / Vue 3 (frontend uniquement)
**Primary Dependencies**: @vue-flow/core (existant), composants Feature 2 (existants)
**Storage**: Aucune modification : utilise les endpoints existants
**Testing**: Pas de CI/CD configuré : vérification manuelle
**Target Platform**: Web (SSR Nuxt 4, rendu graphe client-only)
**Project Type**: Web application (monorepo, modifications frontend uniquement)
**Performance Goals**: Mise à jour arbre < 1s après mutation, formulaire ouvert < 200ms
**Constraints**: Responsive 320px–2560px, Tailwind CSS v4 pur (pas de daisyUI)
**Scale/Scope**: 5 fichiers frontend modifiés, 0 fichier backend modifié

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Justification |
|----------|--------|---------------|
| I. Français d'Abord | PASS | Variables, labels, messages en français |
| II. Monorepo Cohérent | PASS | Modifications dans `uafricas_frontend/` uniquement |
| III. SQL Source de Vérité | PASS | Aucune modification SQL, lecture seule du modèle existant |
| IV. Sécurité par Défaut | PASS | Utilise les endpoints existants protégés par JWT |
| V. Simplicité (YAGNI) | PASS | Réutilise PersonneForm.vue existant, pas de nouveau composant de formulaire. Pas de nouvel endpoint. |
| VI. Tailwind CSS v4 | PASS | Page derrière auth mais publique → Tailwind CSS v4 pur |
| VII. Audit & Traçabilité | PASS | Les mutations passent par les endpoints existants déjà auditées |

## Project Structure

### Documentation (this feature)

```text
specs/001-edition-arbre/
├── plan.md              # Ce fichier
├── spec.md              # Spécification fonctionnelle
├── research.md          # 5 décisions techniques
├── data-model.md        # Structures frontend + flux de données
├── quickstart.md        # Guide de vérification
├── checklists/
│   └── requirements.md  # Checklist qualité spec
└── tasks.md             # (à générer via /speckit.tasks)
```

### Source Code (repository root)

```text
uafricas_frontend/
├── app/
│   ├── pages/
│   │   └── arbre-genealogique/
│   │       └── visualisation.vue    # MODIFIER: orchestrer mutations + rechargement
│   ├── components/
│   │   └── arbre-genealogique/
│   │       ├── PanneauPersonne.vue  # MODIFIER: boutons actions + mode formulaire
│   │       ├── NoeudPersonne.vue    # MODIFIER: badge incomplétude
│   │       ├── BarreOutils.vue      # MODIFIER: compteur branches incomplètes
│   │       ├── ArbreGraphe.vue      # INCHANGÉ
│   │       └── PersonneForm.vue     # EXISTANT (Feature 1), réutilisé tel quel
│   └── composables/
│       └── useLayoutArbre.ts        # MODIFIER: + calculerIncompletude()
```

**Structure Decision** : Modifications frontend uniquement dans 5 fichiers existants. Aucun nouveau fichier, aucun nouveau composant, principe YAGNI respecté. `PersonneForm.vue` est importé et utilisé directement dans `PanneauPersonne.vue`.

## Complexity Tracking

Aucune violation de la constitution, pas de justification de complexité nécessaire.
