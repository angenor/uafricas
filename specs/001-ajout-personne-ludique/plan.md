# Implementation Plan: Ajout de personne ludique

**Branch**: `001-ajout-personne-ludique` | **Date**: 2026-04-06 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-ajout-personne-ludique/spec.md`

## Summary

Transformer l'ajout de personnes à l'arbre généalogique en parcours ludique pas-à-pas avec animations GSAP. Un overlay plein écran remplace le modal actuel, posant une question par étape avec des textes engageants et des transitions animées. Le composant `AssistantAjoutPersonne.vue` fonctionne sur les pages liste et visualisation (avec contexte de lien familial). Le formulaire classique `PersonneForm.vue` reste accessible comme alternative.

## Technical Context

**Language/Version**: TypeScript (Nuxt 4 / Vue 3 SSR)
**Primary Dependencies**: GSAP 3.14.2 (existant), Tailwind CSS v4 (existant)
**Storage**: N/A — aucune modification backend/BDD
**Testing**: Non configuré (pas de CI/CD)
**Target Platform**: Web responsive (desktop + mobile 320px+)
**Project Type**: Web application — frontend uniquement
**Performance Goals**: Transitions < 600ms, 60fps sur mobile
**Constraints**: Tailwind CSS v4 pur (pas de daisyUI), même DTO `CreerPersonneForm` en sortie
**Scale/Scope**: 1 nouveau composant, 2 pages modifiées

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Notes |
|----------|--------|-------|
| I. Français d'Abord | ✅ Conforme | Variables, textes UI, commentaires en français |
| II. Monorepo Cohérent | ✅ Conforme | Modification frontend uniquement, pas de cross-stack |
| III. SQL Source de Vérité | ✅ N/A | Aucune modification du modèle de données |
| IV. Sécurité par Défaut | ✅ N/A | Pas de nouvelle surface d'attaque (même API) |
| V. Simplicité (YAGNI) | ✅ Conforme | 1 composant unique, pas d'abstraction prématurée |
| VI. Tailwind CSS v4 | ✅ Conforme | Tailwind v4 pur, zéro daisyUI (page publique) |
| VII. Audit & Traçabilité | ✅ N/A | Pas de nouvelle mutation backend |

**Re-check post-Phase 1** : Tous les principes restent respectés. Aucune violation.

## Project Structure

### Documentation (this feature)

```text
specs/001-ajout-personne-ludique/
├── spec.md              # Spécification
├── plan.md              # Ce fichier
├── research.md          # Recherche GSAP + patterns
├── data-model.md        # Pas de changement — DTOs réutilisés
├── quickstart.md        # Guide de démarrage rapide
└── tasks.md             # (Phase 2 — /speckit.tasks)
```

### Source Code (repository root)

```text
uafricas_frontend/app/
├── components/arbre-genealogique/
│   ├── AssistantAjoutPersonne.vue    # NOUVEAU — Composant wizard principal
│   ├── PersonneForm.vue              # EXISTANT — Formulaire classique (inchangé)
│   ├── PersonneCard.vue              # EXISTANT — Inchangé
│   └── LienFamilialForm.vue          # EXISTANT — Inchangé
├── pages/arbre-genealogique/
│   ├── index.vue                     # MODIFIÉ — Intégration du wizard
│   └── visualisation.vue             # MODIFIÉ — Intégration du wizard contextuel
└── mocks/arbre-genealogique.ts       # EXISTANT — Inchangé (DTOs réutilisés)
```

**Structure Decision** : Un seul nouveau composant `AssistantAjoutPersonne.vue` dans le dossier feature existant. Pas de nouveau composable — la logique d'animation est locale au composant (principe V Simplicité). Les deux pages consommatrices l'intègrent via import direct.

## Architecture du composant AssistantAjoutPersonne

### Props

| Prop | Type | Défaut | Description |
|------|------|--------|-------------|
| `typeLien` | `'pere' \| 'mere' \| 'parent' \| 'conjoint' \| 'enfant'` | `undefined` | Type de lien (depuis visualisation) |
| `personneLiee` | `{ id: string, nom: string, prenoms?: string }` | `undefined` | Personne liée (depuis visualisation) |
| `loading` | `boolean` | `false` | État de chargement lors de la soumission |

### Emits

| Event | Payload | Description |
|-------|---------|-------------|
| `submit` | `CreerPersonneForm` | Données du formulaire (même DTO qu'avant) |
| `annuler` | — | Fermeture du wizard |
| `formulaire-classique` | — | Bascule vers PersonneForm |

### Étapes du parcours

| # | Champ | Obligatoire | Texte d'accroche (exemple) |
|---|-------|-------------|---------------------------|
| 1 | nom | ✅ Oui | "Comment s'appelle ce membre de votre famille ?" |
| 2 | prenoms | Non | "Magnifique ! Et quel est son prénom ?" |
| 3 | genre | Non | "Pour mieux le/la connaître…" |
| 4 | est_decede | Non | "Cette personne est-elle toujours parmi nous ?" |
| 5 | naissance (année) | Non | "Savez-vous quand il/elle est né(e) ?" |
| 6 | naissance_lieu | Non | "Et où a-t-il/elle vu le jour ?" |
| 7 | récapitulatif | — | "Voici le portrait de votre proche !" |

> Les textes s'adaptent au contexte : si `typeLien` est fourni, l'étape 1 mentionne le lien ("Qui est la mère de {nom} ?"). Si `est_decede` est vrai, les textes suivants adoptent un ton respectueux.

### Patterns GSAP

- **Cycle de vie** : `gsap.context()` dans `onMounted`, `ctx.revert()` dans `onBeforeUnmount`
- **Transitions** : Mini-timeline par changement d'étape via `gsap.fromTo`. Step sortant slide+fade out, step entrant slide+fade in. Durée : 0.4-0.5s, ease `power2.out`
- **Interruption** : `shallowRef<gsap.core.Timeline>` pour la timeline courante. `tl.kill()` + `gsap.set()` snap avant nouvelle transition
- **Célébration** : Checkmark SVG animé (stroke-dashoffset) + burst de confettis DOM (30-50 divs, `gsap.utils.random()`)
- **Progression** : Dots animés `scale` + `backgroundColor`, segment de liaison `scaleX`, parallèle avec transition d'étape
- **Accessibilité** : Check `prefers-reduced-motion` → `gsap.set()` (instant) au lieu de `gsap.to()` (animé)
- **Performance** : Uniquement `x`, `y`, `scale`, `rotation`, `opacity` (GPU-composé). Pas de `will-change` manuel.

## Complexity Tracking

Aucune violation de la constitution à justifier.
