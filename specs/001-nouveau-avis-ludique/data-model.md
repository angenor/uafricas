# Data Model: Nouveau avis de recherche ludique et anime

**Date**: 2026-04-07

## Aucune modification du modele de donnees

Cette feature est purement frontend (animations UI). Aucune entite, table, colonne ou relation n'est ajoutee, modifiee ou supprimee.

Le schema PostgreSQL `retrouve_amis` et les interfaces TypeScript existantes restent inchanges.

## Structures internes au composable

Les structures suivantes sont internes au composable `useAnimationsFormulaire.ts` et ne representent pas des donnees persistees :

### ConfigAnimation

Options passees au composable pour parametrer les animations :

- `dureeTransition` : nombre (ms), defaut 400
- `dureeStagger` : nombre (ms par champ), defaut 100
- `dureeConfettis` : nombre (ms), defaut 3500
- `couleursConfettis` : tableau de chaines (hex), defaut ['#A54A1C', '#228B22']
- `nombreConfettis` : nombre, defaut 35

### EtatAnimation

Etat reactif expose par le composable :

- `enTransition` : booleen — true pendant une animation de transition d'etape
- `prefereReducedMotion` : booleen — preference systeme detectee
- `timelineCourante` : reference GSAP Timeline ou null
- `gsapCtx` : contexte GSAP pour cleanup
