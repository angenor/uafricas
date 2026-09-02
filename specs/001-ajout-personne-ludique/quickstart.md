# Quickstart: Ajout de personne ludique

**Branch**: `001-ajout-personne-ludique` | **Date**: 2026-04-06

## Prérequis

- Node.js 18+, pnpm
- Frontend démarré : `cd uafricas_frontend && pnpm dev`
- GSAP 3.14.2 déjà installé (dépendance existante)

## Fichiers à créer

```
uafricas_frontend/app/components/arbre-genealogique/
└── AssistantAjoutPersonne.vue    # Composant wizard principal
```

## Fichiers à modifier

```
uafricas_frontend/app/pages/arbre-genealogique/
├── index.vue                      # Remplacer le modal PersonneForm par le wizard
└── visualisation.vue              # Remplacer l'ajout contextuel par le wizard
```

## Vérification rapide

1. `pnpm dev` : le serveur démarre sans erreur
2. Naviguer vers `/arbre-genealogique/`
3. Cliquer "Ajouter une personne" → l'overlay plein écran wizard s'ouvre
4. Parcourir les 6 étapes + récapitulatif
5. Valider → animation de célébration → personne créée
6. Cliquer "Formulaire rapide" → le modal classique PersonneForm s'ouvre
7. Naviguer vers `/arbre-genealogique/visualisation`
8. Sélectionner un noeud → "Ajouter un parent" → le wizard s'ouvre avec contexte

## Points d'attention

- Aucune modification backend nécessaire
- Le DTO en sortie du wizard est identique à `CreerPersonneForm` existant
- Tailwind CSS v4 pur : pas de classes daisyUI
- Tester sur mobile (320px min) et vérifier `prefers-reduced-motion`
