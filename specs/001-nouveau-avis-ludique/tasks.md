# Tasks: Nouveau avis de recherche ludique et anime

**Input**: Design documents from `/specs/001-nouveau-avis-ludique/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: Non demandes dans la spec. Verification manuelle via quickstart.md.

**Organization**: Taches groupees par user story pour permettre l'implementation et le test independants de chaque story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut tourner en parallele (fichiers differents, pas de dependances)
- **[Story]**: User story concernee (US1, US2, US3, US4, US5)
- Chemins relatifs depuis la racine du monorepo

---

## Phase 1: Setup

**Purpose**: Creation du composable d'animation GSAP et preparation des refs template

- [X] T001 Creer le composable `useAnimationsFormulaire.ts` dans `uafricas_frontend/app/composables/useAnimationsFormulaire.ts` avec la structure de base : import gsap, interface ConfigAnimation (dureeTransition, dureeStagger, dureeConfettis, couleursConfettis, nombreConfettis avec valeurs par defaut), interface EtatAnimation (enTransition, prefereReducedMotion, timelineCourante, gsapCtx). Exposer une fonction factory qui retourne les refs reactives et les fonctions d'animation (stubs vides pour l'instant). Inclure la detection `prefers-reduced-motion` via `window.matchMedia` dans `onMounted`, la creation du `gsap.context()` scope au conteneur, et le cleanup dans `onBeforeUnmount` (`timeline.kill()` + `gsapCtx.revert()`). Pattern de reference : `uafricas_frontend/app/components/arbre-genealogique/AssistantAjoutPersonne.vue`.

**Checkpoint**: Le composable est importable et le cycle de vie GSAP (init/cleanup) fonctionne sans erreur console.

---

## Phase 2: Foundational (Prerequisites bloquants)

**Purpose**: Fonctions d'animation core dans le composable, utilisees par toutes les user stories

**Warning**: Les phases US ne peuvent pas commencer avant la completion de cette phase.

- [X] T002 Implementer `animerTransitionEtape(direction: 'avant' | 'arriere', cibleRef: Ref<HTMLElement | null>)` dans `uafricas_frontend/app/composables/useAnimationsFormulaire.ts`. Animation : `gsap.fromTo` avec slide directionnel (x: ±80 → 0) + fade (opacity: 0 → 1), duree 400ms, ease `power2.out`. Tuer `timelineCourante` avant chaque nouvelle animation. Respecter la garde `prefereReducedMotion` (return immediat si true).
- [X] T003 [P] Implementer `animerChampsEtape(conteneurRef: Ref<HTMLElement | null>)` dans `uafricas_frontend/app/composables/useAnimationsFormulaire.ts`. Animation : `gsap.fromTo` sur les enfants directs du conteneur avec stagger de 100ms, fade-in (opacity: 0 → 1) + translateY (y: 20 → 0), ease `power1.out`. Garde `prefereReducedMotion`.
- [X] T004 [P] Implementer `animerProgression(nouvelleEtape: number, ancienneEtape: number, dotsRef: Ref<HTMLElement | null>)` dans `uafricas_frontend/app/composables/useAnimationsFormulaire.ts`. Animation : `gsap.to` sur les elements `.dot-indicateur` (scale pulse 1.3 sur actif avec `back.out(1.7)`, scale 1 sur les autres), `gsap.to` sur les `.segment-indicateur` (scaleX 0/1 avec transformOrigin directionnel). Garde `prefereReducedMotion`.
- [X] T005 [P] Implementer `lancerConfettis(conteneurRef: Ref<HTMLElement | null>, couleurs?: string[])` dans `uafricas_frontend/app/composables/useAnimationsFormulaire.ts`. Creer 35 divs avec position absolute, tailles aleatoires (6-12px), formes variees (border-radius 0/50%), couleurs alternees (#A54A1C, #228B22). Animer avec gsap : position initiale en haut, y aleatoire vers le bas + rotation aleatoire, opacity fade-out, duree 3-4s, nettoyage DOM apres completion. Conteneur overlay `pointer-events: none`. Garde `prefereReducedMotion`.
- [X] T006 [P] Implementer `animerCompteur(cibleRef: Ref<HTMLElement | null>, valeurFinale: number)` dans `uafricas_frontend/app/composables/useAnimationsFormulaire.ts`. Animation : `gsap.to` sur un objet proxy `{ val: 0 }`, duree 1500ms, ease `power2.out`, mise a jour du `textContent` de la cible a chaque frame via `onUpdate`. Garde `prefereReducedMotion`.
- [X] T007 [P] Implementer `animerErreur(cibleRef: Ref<HTMLElement | null>)` dans `uafricas_frontend/app/composables/useAnimationsFormulaire.ts`. Animation : `gsap.to` avec keyframes translateX [0, -10, 10, -6, 6, 0], duree 400ms, ease `power2.out`. Garde `prefereReducedMotion`.

**Checkpoint**: Toutes les fonctions d'animation sont implementees et exportees par le composable. Pret pour l'integration dans les composants.

---

## Phase 3: User Story 1 : Navigation fluide et animee entre les etapes (Priority: P1) MVP

**Goal**: Chaque transition entre les 6 etapes du formulaire est accompagnee d'animations slide directionnelles + fade.

**Independent Test**: Naviguer entre les etapes du formulaire via les boutons Suivant/Precedent et les indicateurs d'etapes. Observer les animations slide gauche/droite avec fade. Tester le double-clic rapide (l'animation precedente doit etre interrompue sans accumulation).

### Implementation

- [X] T008 [US1] Modifier `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` : importer `useAnimationsFormulaire` et `gsap`, creer les refs template (`etapeRef` sur le conteneur de l'etape active, `conteneurFormRef` pour le scope GSAP). Initialiser le composable avec `onMounted`/`onBeforeUnmount` pour le cycle de vie GSAP. Ajouter une ref `enTransition` (booleen) pour empecher les clics multiples.
- [X] T009 [US1] Modifier `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` : remplacer les `v-if="etapeCourante === N"` par un conteneur unique avec ref `etapeRef` qui wrap le contenu de l'etape active. Ajouter un `watch` sur `etapeCourante` qui : (1) determine la direction (avant/arriere), (2) met `enTransition = true`, (3) `await nextTick()`, (4) appelle `animerTransitionEtape(direction, etapeRef)`, (5) remet `enTransition = false` apres completion. Garder les boutons Suivant/Precedent avec `:disabled="enTransition"`.
- [X] T010 [US1] Modifier les fonctions `suivant()` et `precedent()` dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` pour verifier `if (enTransition.value) return` en debut de fonction. Modifier le gestionnaire de clic sur les indicateurs d'etape pour le meme guard.

**Checkpoint**: Les transitions entre etapes sont animees avec slide directionnel. Le double-clic rapide est bloque. Le formulaire fonctionne normalement.

---

## Phase 4: User Story 2 : Barre de progression animee (Priority: P1)

**Goal**: La barre de progression (indicateurs d'etapes + connecteurs) s'anime fluidement a chaque changement d'etape.

**Independent Test**: Observer les indicateurs circulaires et les segments de connexion lors de la navigation. Le dot actif doit pulser (scale), les etapes completees doivent afficher un check anime, les segments doivent s'animer en scaleX avec direction.

### Implementation

- [X] T011 [US2] Modifier le template des indicateurs d'etapes dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` : ajouter la classe CSS `dot-indicateur` sur chaque `<button>` d'etape, ajouter la classe CSS `segment-indicateur` sur chaque `<div>` connecteur, et envelopper le tout dans un conteneur avec ref `dotsRef`.
- [X] T012 [US2] Enrichir le `watch` sur `etapeCourante` dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` pour appeler `animerProgression(nouvelleEtape, ancienneEtape, dotsRef)` en plus de `animerTransitionEtape`. L'animation de progression doit se declencher en meme temps que la transition d'etape.
- [X] T013 [US2] Ajouter un style initial CSS sur les `.segment-indicateur` dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` : `transform: scaleX(0)` pour les segments non actifs et `scaleX(1)` pour les segments actifs, via une classe dynamique ou un style inline reactif. Cela permet a GSAP d'animer le scaleX de maniere fluide.

**Checkpoint**: La barre de progression s'anime fluidement. Les dots pulsent, les segments s'etendent/retractent avec direction. Fonctionne avec les boutons et les indicateurs cliquables.

---

## Phase 5: User Story 3 : Animations d'apparition des champs (Priority: P2)

**Goal**: Les champs de chaque etape apparaissent de maniere echelonnee (stagger) avec fade-in + translateY.

**Independent Test**: Arriver sur chaque etape et observer les champs apparaitre un par un avec un decalage. Tester les champs conditionnels (checkbox reseaux sociaux a l'etape 4).

### Implementation

- [X] T014 [US3] Enrichir le `watch` sur `etapeCourante` dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` pour appeler `animerChampsEtape(etapeRef)` apres `animerTransitionEtape`, avec un leger delai (200ms apres le debut de la transition pour que les champs apparaissent pendant le slide).
- [X] T015 [US3] Ajouter un `watch` sur `form.rencontre_reseaux_sociaux` dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue`. Quand la valeur passe a true : `await nextTick()` puis `gsap.from` sur le conteneur des checkboxes reseaux avec height auto-expand (from `height: 0, opacity: 0` to `height: auto, opacity: 1`), duree 350ms, ease `power2.out`. Quand false : animation inverse (collapse).

**Checkpoint**: Les champs apparaissent en stagger a chaque etape. Les champs conditionnels s'expandent/collapsent fluidement.

---

## Phase 6: User Story 4 : Ecran de succes celebratoire (Priority: P2)

**Goal**: L'ecran de succes affiche des animations de celebration : bounce-in icone, fade-in titre, confettis chocolat/vert 3-4s, compteur de correspondances, stagger boutons.

**Independent Test**: Soumettre un avis avec succes (via le formulaire ou en forcant `succes.value = true` en dev). Observer la sequence : icone bounce → titre fade → confettis retombent → boutons apparaissent. Si des correspondances existent, le compteur s'incremente de 0 a N.

### Implementation

- [X] T016 [US4] Modifier `uafricas_frontend/app/pages/retrouve-amis/nouveau.vue` : importer `useAnimationsFormulaire`, ajouter des refs template sur les elements de l'ecran de succes (`iconeSuccesRef`, `titreSuccesRef`, `messageCorrespondancesRef`, `boutonsSuccesRef`, `conteneurConfettisRef`). Ajouter un conteneur div `position: relative; overflow: hidden` autour de l'ecran de succes pour les confettis.
- [X] T017 [US4] Ajouter un `watch` sur `succes` dans `uafricas_frontend/app/pages/retrouve-amis/nouveau.vue`. Quand `succes` passe a true : `await nextTick()` puis lancer une timeline GSAP sequentielle : (1) `gsap.fromTo` bounce-in sur icone (scale 0→1, ease `back.out(1.4)`, 500ms), (2) `gsap.fromTo` fade-in titre (y: 20→0, opacity 0→1, 400ms), (3) `lancerConfettis(conteneurConfettisRef)`, (4) si `correspondancesTrouvees > 0` appeler `animerCompteur`, (5) stagger fade-in + bounce sur les boutons (150ms/bouton, ease `back.out(1.2)`).
- [X] T018 [US4] Ajouter un `watch` sur `erreur` dans `uafricas_frontend/app/pages/retrouve-amis/nouveau.vue`. Quand `erreur` change et n'est pas vide : `await nextTick()` puis appeler `animerErreur` sur le div d'erreur (ref `erreurRef` a ajouter sur le div existant `v-if="erreur"`).

**Checkpoint**: L'ecran de succes est celebratoire avec confettis aux couleurs du site. Le compteur s'anime. L'erreur shake. Les animations durent 3-4s pour les confettis.

---

## Phase 7: User Story 5 : Micro-interactions (Priority: P3)

**Goal**: Les boutons, selections radio et upload photo ont des micro-animations au hover/clic.

**Independent Test**: Survoler les boutons Suivant/Precedent (scale-up leger). Cliquer sur un type de relation (pulse). Uploader une photo (zoom-in de la preview).

### Implementation

- [X] T019 [US5] Ajouter des animations CSS Tailwind sur les boutons Suivant/Precedent dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` : classes `hover:scale-105 active:scale-95 transition-transform duration-200`. Pas besoin de GSAP pour ces micro-interactions (CSS natif suffit, principe YAGNI).
- [X] T020 [US5] Ajouter une micro-animation sur la selection des radio buttons (type de relation, genre) dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` : au `@click`, appeler `gsap.fromTo` sur l'element selectionne avec un pulse rapide (scale 1→1.05→1, duree 200ms, ease `power1.out`). Garde `prefereReducedMotion`.
- [X] T021 [US5] Ajouter une animation sur l'apparition de la preview photo dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` : watcher sur la ref de preview, quand elle change (photo uploadee), `await nextTick()` puis `gsap.fromTo` sur l'image (scale 0.8→1, opacity 0→1, duree 300ms, ease `back.out(1.2)`). Garde `prefereReducedMotion`.

**Checkpoint**: Les interactions avec le formulaire sont satisfaisantes avec des retours visuels subtils. Les animations CSS et GSAP cohabitent.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Accessibilite, performance, nettoyage

- [X] T022 Verifier le support `prefers-reduced-motion` dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` et `uafricas_frontend/app/pages/retrouve-amis/nouveau.vue` : activer la preference dans les DevTools du navigateur et confirmer que TOUTES les animations GSAP sont desactivees (transitions instantanees, pas de confettis, pas de stagger). Les transitions CSS (hover scale) doivent aussi etre desactivees via une media query `@media (prefers-reduced-motion: reduce)`.
- [X] T023 [P] Tester la reactivite mobile dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` : verifier que les animations ne causent pas de saccades sur un appareil mobile (utiliser le throttling CPU des DevTools Chrome). Ajuster les durees si necessaire (reduire de 20% sur mobile si perceptiblement lent).
- [X] T024 [P] Verifier qu'aucun residu Tailwind v3 n'existe dans les fichiers modifies (`uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue` et `uafricas_frontend/app/pages/retrouve-amis/nouveau.vue`). Si detecte, migrer vers la syntaxe Tailwind CSS v4.
- [X] T025 Validation finale via `specs/001-nouveau-avis-ludique/quickstart.md` : executer tous les scenarios de verification documentes.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: Pas de dependances, demarrage immediat
- **Phase 2 (Foundational)**: Depend de Phase 1, BLOQUE toutes les user stories
- **Phase 3 (US1)**: Depend de Phase 2 (T002 obligatoire)
- **Phase 4 (US2)**: Depend de Phase 3 (enrichit le meme watch, meme fichier)
- **Phase 5 (US3)**: Depend de Phase 3 (enrichit le meme watch)
- **Phase 6 (US4)**: Depend de Phase 2 (T005, T006, T007 requis), peut tourner en **parallele** de Phase 3/4/5 car fichier different (`nouveau.vue`)
- **Phase 7 (US5)**: Depend de Phase 3 (meme fichier, ajout de micro-interactions)
- **Phase 8 (Polish)**: Depend de toutes les phases precedentes

### User Story Dependencies

- **US1 (P1)**: Depend de Foundational : aucune dependance inter-stories
- **US2 (P1)**: Depend de US1 (meme fichier, enrichit le watch existant)
- **US3 (P2)**: Depend de US1 (enrichit le watch, meme fichier)
- **US4 (P2)**: **INDEPENDANT** : fichier different (`nouveau.vue`), peut tourner en parallele de US1/US2/US3
- **US5 (P3)**: Depend de US1 (meme fichier)

### Within Each User Story

- Implementation directe (pas de tests automatises demandes)
- Chaque tache doit etre commitee apres verification manuelle

### Parallel Opportunities

- T003, T004, T005, T006, T007 (Phase 2) : tous en parallele (fonctions independantes dans le meme fichier)
- US4 (Phase 6) en parallele de US1+US2+US3 (fichiers differents)
- T022, T023, T024 (Phase 8) : T023 et T024 en parallele

---

## Parallel Example: Phase 2 (Foundational)

```bash
# Toutes les fonctions d'animation en parallele (meme fichier, sections independantes) :
T003: animerChampsEtape dans useAnimationsFormulaire.ts
T004: animerProgression dans useAnimationsFormulaire.ts
T005: lancerConfettis dans useAnimationsFormulaire.ts
T006: animerCompteur dans useAnimationsFormulaire.ts
T007: animerErreur dans useAnimationsFormulaire.ts
```

## Parallel Example: US4 independant

```bash
# US4 peut tourner en parallele de US1/US2/US3 :
# Agent A: T008→T009→T010 (US1, AvisRechercheForm.vue)
# Agent B: T016→T017→T018 (US4, nouveau.vue)
```

---

## Implementation Strategy

### MVP First (User Story 1 + 2 : transitions + progression)

1. Completer Phase 1 : Setup composable
2. Completer Phase 2 : Fonctions d'animation core
3. Completer Phase 3 : US1 : transitions entre etapes
4. Completer Phase 4 : US2 : barre de progression animee
5. **STOP et VALIDER** : le formulaire a des animations fluides et engageantes
6. L'experience est deja significativement amelioree a ce stade

### Incremental Delivery

1. Setup + Foundational → composable pret
2. US1 + US2 → transitions + progression (MVP)
3. US3 → stagger des champs (amelioration percue)
4. US4 → ecran de succes celebratoire (moment fort)
5. US5 → micro-interactions (polish final)
6. Chaque ajout enrichit l'experience sans casser les precedents

---

## Notes

- Tous les fichiers modifies sont dans `uafricas_frontend/app/`
- GSAP 3.14.2 est deja installe : aucun `pnpm add` necessaire
- Pattern de reference : `app/components/arbre-genealogique/AssistantAjoutPersonne.vue`
- Couleurs confettis : chocolat #A54A1C + vert #228B22 (couleurs du site)
- Pas de daisyUI (site public) : Tailwind CSS v4 pur
- Commiter apres chaque phase ou user story completee
