# Research: Nouveau avis de recherche ludique et anime

**Date**: 2026-04-07

## Decision 1: Bibliotheque d'animation

**Decision**: Utiliser GSAP 3.14.2 (deja installe dans le projet)

**Rationale**:
- GSAP est deja present dans `package.json` (version ^3.14.2)
- Un pattern complet existe dans `AssistantAjoutPersonne.vue` : context, timeline, cleanup, reduced-motion, direction tracking
- GSAP offre des performances superieures aux CSS transitions pour les animations complexes (timelines, stagger, interruptions)
- Compatible SSR Nuxt 4 (import dynamique ou guard `process.client` si necessaire)

**Alternatives considerees**:
- CSS transitions/animations natives : insuffisantes pour les timelines sequentielles, le stagger et les confettis
- Vue `<Transition>` / `<TransitionGroup>` : limites pour l'orchestration de timelines complexes et les animations imperatives
- Framer Motion (via `motion-v`) : dependance supplementaire non justifiee quand GSAP est deja disponible
- canvas-confetti (npm) : dependance supplementaire pour un seul effet, realisable avec GSAP seul

## Decision 2: Pattern d'architecture des animations

**Decision**: Composable `useAnimationsFormulaire.ts` centralisant toute la logique GSAP

**Rationale**:
- Separe la logique d'animation de la logique metier du formulaire (SRP)
- Reutilisable si d'autres formulaires multi-etapes sont ajoutes
- Facilite le testing (mock du composable) et la maintenance
- Suit le pattern des composables existants (`useAuth`, `useRetrouvAmis`, etc.)

**Alternatives considerees**:
- Animations inline dans le composant (comme AssistantAjoutPersonne) : fonctionnel mais alourdit un composant deja complexe (~400 lignes)
- Directives Vue custom (`v-gsap-slide`, `v-gsap-fade`) : sur-abstraction pour ce cas d'usage, YAGNI
- Plugin Nuxt GSAP : dependance supplementaire non necessaire, import direct suffit

## Decision 3: Strategie de confettis

**Decision**: Elements DOM animes avec GSAP (pas de canvas)

**Rationale**:
- Pattern identique a `lancerCelebration()` existant dans AssistantAjoutPersonne
- 30-40 elements DOM sont performants pour un effet temporaire de 3-4s
- Pas besoin d'un canvas pour un nombre aussi faible de particules
- Plus simple a nettoyer (remove des elements) qu'un canvas a detruire

**Alternatives considerees**:
- Canvas 2D + requestAnimationFrame : plus performant pour >100 particules, mais surdimensionne ici
- CSS @keyframes avec elements pre-places : pas assez flexible pour la randomisation (positions, rotations, tailles)

## Decision 4: Gestion du SSR Nuxt 4

**Decision**: Guard `import.meta.client` ou `onMounted` pour tout code GSAP

**Rationale**:
- GSAP accede au DOM (`window`, `document`) qui n'existe pas cote serveur
- `onMounted` est deja le point d'entree naturel pour les animations (DOM pret)
- Le pattern existant dans AssistantAjoutPersonne utilise deja `onMounted` pour l'init GSAP

**Alternatives considerees**:
- Import dynamique conditionnel (`if (import.meta.client) { const gsap = await import('gsap') }`) : fonctionnel mais complexifie le code inutilement quand `onMounted` suffit

## Decision 5: Strategie d'interruptibilite

**Decision**: `timeline.kill()` avant chaque nouvelle animation + ref `enTransition`

**Rationale**:
- Pattern eprouve dans AssistantAjoutPersonne (`timelineCourante.value.kill()`)
- `enTransition` empeche les clics multiples de creer des animations concurrentes
- GSAP gere nativement le kill/override de tweens sur un meme element

**Alternatives considerees**:
- `gsap.killTweensOf(element)` : kill par element plutot que par timeline — moins precis pour les animations sequentielles
- Debounce sur les boutons suivant/precedent : masque le probleme au lieu de le resoudre, degrade l'UX
