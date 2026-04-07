# Research: Ajout de personne ludique

**Branch**: `001-ajout-personne-ludique` | **Date**: 2026-04-06

## Décision 1 : Pattern de transition entre étapes

**Décision** : Timeline GSAP par transition (mini-timeline créée à chaque changement d'étape), avec `gsap.fromTo` pour des états déterministes.

**Raisonnement** : Un wizard est piloté par l'utilisateur, pas auto-jouant. Une master timeline unique avec `.play()`/`.reverse()` gère mal la navigation libre (retour, saut d'étapes). `fromTo` est déterministe — il définit explicitement l'état initial et final, ce qui évite les incohérences si l'animation précédente n'est pas terminée.

**Alternatives considérées** :
- Master timeline unique : rejetée car combat la navigation non-linéaire
- `gsap.to` simple : rejeté car dépend de l'état courant, imprévisible lors de clics rapides
- CSS transitions Vue `<Transition>` : rejeté car contrôle insuffisant sur le séquençage et l'interruption

## Décision 2 : Gestion de l'interruption (clics rapides)

**Décision** : Appeler `tl.kill()` sur la timeline en cours puis `gsap.set()` pour snapper instantanément l'ancien step à son état final avant de lancer la nouvelle animation.

**Raisonnement** : Empêche l'empilement d'animations. L'utilisateur peut cliquer rapidement "Suivant" sans artefact visuel.

**Alternatives considérées** :
- `overwrite: "auto"` sur les tweens : fonctionne mais moins explicite
- Bloquer les boutons pendant l'animation : rejeté, mauvaise UX

## Décision 3 : Intégration GSAP + Vue 3

**Décision** : Utiliser `gsap.context()` créé dans `onMounted` avec le template ref comme scope. Nettoyage via `ctx.revert()` dans `onBeforeUnmount`. Timelines stockées dans `shallowRef`.

**Raisonnement** : Pattern officiellement recommandé par GSAP pour les frameworks. `gsap.context()` collecte automatiquement toutes les animations, et `revert()` les nettoie proprement en restaurant les styles inline. `shallowRef` évite la réactivité profonde sur les objets GSAP complexes.

**Alternatives considérées** :
- Plugin `useGSAP` communautaire : pas de version officielle Vue, pattern `context()` suffit
- Gestion manuelle des tweens : plus verbeux, risque de fuites mémoire

## Décision 4 : Réactivité des changements d'étape

**Décision** : `watch(etapeCourante)` + `await nextTick()` pour déclencher les animations après le rendu DOM.

**Raisonnement** : Sépare proprement la logique d'animation des handlers de clic. `nextTick` est nécessaire car le contenu des étapes est conditionnellement rendu (`v-if`), l'élément DOM doit exister avant que GSAP puisse le cibler.

**Alternatives considérées** :
- Animation dans les handlers de clic : mélange logique métier et animation
- `v-show` au lieu de `v-if` : garderait tous les steps dans le DOM, plus lourd

## Décision 5 : Animation de célébration

**Décision** : Checkmark SVG animé (stroke-dashoffset) + confettis DOM (30-50 divs avec positions/rotations aléatoires via `gsap.utils.random()`).

**Raisonnement** : Zéro dépendance externe, moins de 20 lignes de GSAP. Le SVG checkmark utilise `stroke-dasharray` + `stroke-dashoffset` natifs (pas besoin de DrawSVGPlugin). Les confettis DOM sont suffisants pour 30-50 particules.

**Alternatives considérées** :
- `canvas-confetti` (37KB) : surdimensionné pour un wizard
- `@tsparticles` : trop lourd
- Canvas custom : surcodé pour 30-50 particules

## Décision 6 : Performance mobile

**Décision** : Animer uniquement les propriétés GPU-composées (`x`, `y`, `scale`, `rotation`, `opacity`). Ne pas ajouter `will-change` manuellement. Supporter `prefers-reduced-motion` via check media query.

**Raisonnement** : GSAP gère automatiquement l'accélération GPU via `transform: translate3d()`. Ajouter `will-change` manuellement peut nuire aux performances mobiles en promouvant trop de couches. Un wizard n'anime que 2 éléments à la fois (step sortant + step entrant), naturellement léger.

**Alternatives considérées** :
- `gsap.matchMedia()` pour reduced motion : viable mais un simple check `window.matchMedia` suffit pour ce cas
- Désactiver complètement les animations sur mobile : rejeté, les animations transform/opacity sont performantes même sur mobile

## Décision 7 : Indicateur de progression

**Décision** : Dots avec animation `scale` + `backgroundColor` sur le dot actif, segment de ligne entre dots animé via `scaleX`. Animation parallèle avec la transition d'étape.

**Raisonnement** : Les dots sont le pattern le plus naturel pour un wizard en 6 étapes. L'animation parallèle (pas séquentielle) avec la transition de step donne un rendu cohérent.

**Alternatives considérées** :
- Barre de progression : moins précise visuellement pour 6 étapes
- Numéros d'étapes : plus formel, moins ludique
- Animation CSS variable : viable mais les dots avec scaleX sont plus visuels

## Décision 8 : Style UI

**Décision** : Tailwind CSS v4 pur, zéro classe daisyUI. Overlay plein écran avec fond opaque/semi-transparent.

**Raisonnement** : Constitution VI impose daisyUI uniquement pour le back-office admin. Les pages arbre-généalogique sont publiques. L'identité visuelle sur mesure utilise les couleurs du thème (`custom-chocolat`, `custom-green`, `custom-gray`), les polices Oswald/Open Sans.

**Alternatives considérées** :
- daisyUI steps/modal : rejeté, interdit sur les pages publiques

## Décision 9 : Structure du composant

**Décision** : Un composant principal `AssistantAjoutPersonne.vue` dans `app/components/arbre-genealogique/` qui encapsule tout le wizard. Le composant reçoit un contexte optionnel (type de lien + personne liée pour la visualisation) et émet les mêmes données que `PersonneForm.vue`.

**Raisonnement** : Principe V (Simplicité). Un seul composant évite la fragmentation. Le PersonneForm existant reste intact comme alternative "formulaire rapide".

**Alternatives considérées** :
- Composant par étape : sur-fragmenté pour 6 étapes simples
- Remplacement de PersonneForm : rejeté, le formulaire classique reste comme fallback
