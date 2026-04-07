# Implementation Plan: Nouveau avis de recherche ludique et anime

**Branch**: `001-nouveau-avis-ludique` | **Date**: 2026-04-07 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-nouveau-avis-ludique/spec.md`

## Summary

Enrichir le formulaire wizard 6 etapes de creation d'avis de recherche (`AvisRechercheForm.vue`) et l'ecran de succes (`nouveau.vue`) avec des animations GSAP fluides et modernes. Le projet dispose deja de GSAP 3.14.2 et d'un pattern de reference complet dans `AssistantAjoutPersonne.vue` (context, timeline, cleanup, reduced-motion, direction tracking). Aucune modification backend ou BDD.

## Technical Context

**Language/Version**: TypeScript (Nuxt 4 / Vue 3 SSR)
**Primary Dependencies**: GSAP 3.14.2 (deja installe), Vue 3 Composition API, Tailwind CSS v4
**Storage**: N/A (aucune modification BDD)
**Testing**: Pas de framework de test configure (verification manuelle)
**Target Platform**: Web (desktop + mobile), SSR Nuxt 4
**Project Type**: Web application (frontend uniquement)
**Performance Goals**: Animations < 800ms, 60fps sur mobile recent
**Constraints**: prefers-reduced-motion respecte, pas de dependance supplementaire, pas de daisyUI (site public)
**Scale/Scope**: 2 fichiers principaux modifies, 1 composable cree

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Notes |
|----------|--------|-------|
| I. Francais d'Abord | PASS | Variables, fonctions, commentaires en francais |
| II. Monorepo Coherent | PASS | Modifications frontend uniquement, pas de desynchronisation |
| III. SQL Source de Verite | N/A | Aucune modification de donnees |
| IV. Securite par Defaut | N/A | Pas d'input utilisateur nouveau, pas de mutation backend |
| V. Simplicite (YAGNI) | PASS | Un composable d'animation reutilisable, pas de sur-abstraction. Pattern identique a AssistantAjoutPersonne existant |
| VI. Tailwind CSS v4 (pas daisyUI public) | PASS | Site public, Tailwind CSS v4 pur |
| VII. Audit & Tracabilite | N/A | Pas de mutation de donnees |

## Project Structure

### Documentation (this feature)

```text
specs/001-nouveau-avis-ludique/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── spec.md              # Feature specification
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
uafricas_frontend/
├── app/
│   ├── composables/
│   │   └── useAnimationsFormulaire.ts    # NOUVEAU — composable GSAP reutilisable
│   ├── components/
│   │   └── retrouve-amis/
│   │       └── AvisRechercheForm.vue     # MODIFIE — ajout animations GSAP
│   └── pages/
│       └── retrouve-amis/
│           └── nouveau.vue               # MODIFIE — animations ecran de succes
```

**Structure Decision**: Pas de nouveau dossier. Un composable centralise la logique d'animation GSAP reutilisable (transitions d'etapes, stagger de champs, confettis). Les deux fichiers Vue existants sont enrichis sans changement de structure fonctionnelle.

## Architecture des Animations

### Composable `useAnimationsFormulaire.ts`

Responsabilites :
- **Gestion du contexte GSAP** : creation (`gsap.context()`), cleanup (`revert()`) dans le cycle de vie Vue
- **Detection reduced-motion** : `window.matchMedia('(prefers-reduced-motion: reduce)')` avec garde sur toutes les fonctions
- **Transitions d'etapes** : `animerTransitionEtape(direction: 'avant' | 'arriere', cibleRef)` — slide directionnel + fade
- **Stagger de champs** : `animerChampsEtape(conteneurRef)` — fade-in + translateY echelonne sur les enfants directs
- **Animation progression** : `animerProgression(nouvelleEtape, ancienneEtape, dotsRef)` — scale pulse sur dot actif + scaleX segments
- **Confettis** : `lancerConfettis(conteneurRef, couleurs)` — particules DOM avec gravite GSAP, duree 3-4s, fade-out
- **Compteur anime** : `animerCompteur(cibleRef, valeurFinale)` — incrementation progressive de 0 a N
- **Shake erreur** : `animerErreur(cibleRef)` — translateX oscillant rapide
- **Interruptibilite** : `timelineCourante` tuee avant chaque nouvelle animation

Pattern de reference : `AssistantAjoutPersonne.vue` lignes utilisant `gsap.context()`, `timeline.kill()`, `shallowRef<gsap.core.Timeline>`.

### Modifications `AvisRechercheForm.vue`

1. **Import et setup** : importer `useAnimationsFormulaire`, creer les refs template (`etapeRef`, `dotsRef`, `conteneurRef`)
2. **Watch sur `etapeCourante`** : `watch` avec `await nextTick()` pour declencher `animerTransitionEtape` et `animerProgression` (meme pattern que AssistantAjoutPersonne)
3. **Garde de transition** : ref `enTransition` pour empecher le double-clic rapide d'accumuler des animations
4. **Step indicator** : ajouter classes CSS `dot-indicateur` et `segment-indicateur` sur les boutons et connecteurs existants pour le querySelectorAll GSAP
5. **Champs conditionnels** : watcher sur `form.rencontre_reseaux_sociaux` pour declencher l'animation d'expansion

### Modifications `nouveau.vue`

1. **Ecran de succes** : ajouter refs sur les elements (icone, titre, message correspondances, boutons)
2. **Animation d'entree** : `onMounted` conditionnel quand `succes === true` — timeline sequentielle : bounce-in icone → fade-in titre → confettis → stagger boutons
3. **Compteur correspondances** : `animerCompteur` sur le nombre
4. **Shake erreur** : `animerErreur` sur le div d'erreur quand `erreur` change

### Timings des Animations

| Animation | Duree | Ease | Declencheur |
|-----------|-------|------|-------------|
| Transition etape (slide + fade) | 400ms | power2.out | Changement d'etape |
| Stagger champs | 100ms/champ | power1.out | Apres transition etape |
| Barre progression | 300ms | power2.inOut | Changement d'etape |
| Pulse dot actif | 300ms | back.out(1.7) | Changement d'etape |
| Check etape completee | 300ms | back.out(1.4) | Etape validee |
| Expansion champs conditionnels | 350ms | power2.out | Toggle checkbox |
| Ecran succes bounce-in | 500ms | back.out(1.4) | Soumission reussie |
| Confettis (gravite + fade) | 3000-4000ms | power1.in | Ecran succes |
| Compteur correspondances | 1500ms | power2.out | Ecran succes |
| Stagger boutons succes | 150ms/bouton | back.out(1.2) | Apres confettis |
| Shake erreur | 400ms | power2.out | Erreur soumission |

### Confettis — Implementation

- 30-40 elements `<div>` crees dynamiquement dans un conteneur overlay (`position: absolute`, `overflow: hidden`, `pointer-events: none`)
- Couleurs alternees : `#A54A1C` (chocolat) et `#228B22` (vert)
- Taille : 6-12px, formes variees (carre, rectangle, cercle via `border-radius`)
- Animation GSAP : position initiale en haut, `y` aleatoire vers le bas avec `rotation` aleatoire, `opacity` fade-out en fin
- Duree : 3-4 secondes, nettoyage DOM apres completion
- Pattern identique a `lancerCelebration()` dans AssistantAjoutPersonne

## Complexity Tracking

Aucune violation de constitution detectee. Pas de complexite supplementaire injustifiee.
