# Quickstart: Nouveau avis de recherche ludique et anime

**Date**: 2026-04-07

## Prerequis

- Node.js + pnpm installes
- Le frontend tourne (`pnpm dev` sur le port 3000)
- GSAP 3.14.2 est deja dans `package.json` (rien a installer)

## Demarrage rapide

```bash
# 1. Checkout de la branche
git checkout 001-nouveau-avis-ludique

# 2. Installer les dependances (si pas deja fait)
cd uafricas_frontend && pnpm install

# 3. Lancer le frontend
pnpm dev

# 4. Ouvrir dans le navigateur
# http://localhost:3000/retrouve-amis/nouveau
# (Necessite d'etre connecte — utiliser admin@test.com / Test1234)
```

## Fichiers a modifier

| Fichier | Action | Description |
|---------|--------|-------------|
| `app/composables/useAnimationsFormulaire.ts` | CREER | Composable GSAP centralise |
| `app/components/retrouve-amis/AvisRechercheForm.vue` | MODIFIER | Ajouter animations transitions/stagger/progression |
| `app/pages/retrouve-amis/nouveau.vue` | MODIFIER | Ajouter animations ecran succes + confettis + shake erreur |

## Verification

1. Naviguer entre les 6 etapes → transitions slide directionnelles fluides
2. Observer les champs → apparition echelonnee (stagger)
3. Observer les indicateurs d'etapes → pulse sur actif, check anime
4. Soumettre un avis → confettis chocolat/vert, bounce-in, compteur anime
5. Tester avec `prefers-reduced-motion: reduce` → animations desactivees
6. Double-clic rapide sur "Suivant" → pas d'accumulation d'animations

## Pattern de reference

Voir `app/components/arbre-genealogique/AssistantAjoutPersonne.vue` pour le pattern GSAP complet (context, timeline, cleanup, reduced-motion, confettis).
