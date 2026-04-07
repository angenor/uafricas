# Tasks: Ajout de personne ludique

**Input**: Design documents from `/specs/001-ajout-personne-ludique/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

**Tests**: Non demandés (pas de testing configuré dans le projet).

**Organization**: Tasks groupées par user story pour implémentation et test indépendants.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut tourner en parallèle (fichiers différents, pas de dépendance)
- **[Story]**: User story associée (US1, US2, US3, US4, US5)
- Chemins exacts inclus dans les descriptions

---

## Phase 1: Setup

**Purpose**: Aucune dépendance à installer — GSAP 3.14.2 et Tailwind v4 déjà présents. Vérification uniquement.

- [x] T001 Vérifier que GSAP est importable dans le projet : `import gsap from 'gsap'` dans un composant test temporaire sous `uafricas_frontend/`

---

## Phase 2: Foundational (Squelette du composant)

**Purpose**: Structure de base du composant wizard qui sera enrichie par chaque user story.

**⚠️ CRITICAL**: Les phases US ne peuvent commencer qu'après cette phase.

- [x] T002 Créer le squelette du composant `AssistantAjoutPersonne.vue` dans `uafricas_frontend/app/components/arbre-genealogique/AssistantAjoutPersonne.vue` — overlay plein écran (`fixed inset-0 z-50`), props (`typeLien`, `personneLiee`, `loading`), emits (`submit`, `annuler`, `formulaire-classique`), state local (`etapeCourante: ref(1)`, `formulaire: reactive<CreerPersonneForm>`)
- [x] T003 Implémenter la structure des 7 étapes dans `AssistantAjoutPersonne.vue` — définir le tableau des étapes (champ, obligatoire, ordre) avec rendu conditionnel `v-if="etapeCourante === n"` pour chaque étape, layout centré Tailwind v4 pur (pas de daisyUI)
- [x] T004 Implémenter la navigation entre étapes dans `AssistantAjoutPersonne.vue` — boutons Suivant/Retour/Passer, logique `allerSuivant()` / `allerPrecedent()` / `passerEtape()`, conservation des réponses dans l'objet `formulaire` réactif
- [x] T005 Implémenter la validation par étape dans `AssistantAjoutPersonne.vue` — nom obligatoire (étape 1 bloquante), année de naissance valide (1-année courante), affichage erreur inline avec style Tailwind v4

**Checkpoint**: Le squelette du wizard navigue entre les étapes, collecte et conserve les données, valide les champs. Pas d'animation ni de textes enrichis.

---

## Phase 3: User Story 1 — Parcours guidé pas-à-pas (Priority: P1) 🎯 MVP

**Goal**: L'utilisateur peut ajouter une personne via le parcours étape par étape depuis les deux pages (index et visualisation).

**Independent Test**: Cliquer "Ajouter une personne" sur index.vue → parcourir les 7 étapes → valider → personne créée. Idem depuis visualisation.vue avec contexte de lien.

### Implementation

- [x] T006 Implémenter l'écran récapitulatif (étape 7) dans `AssistantAjoutPersonne.vue` — afficher toutes les réponses collectées (nom, prénoms, genre, naissance, lieu, statut vital) avec mise en page claire, bouton "Valider" qui émet `submit` avec le DTO `CreerPersonneForm`, bouton "Modifier" pour revenir à une étape spécifique
- [x] T007 Implémenter la gestion du contexte de lien dans `AssistantAjoutPersonne.vue` — si `typeLien` et `personneLiee` sont fournis (depuis visualisation), adapter le titre de l'étape 1 pour mentionner le lien ("Qui est la mère de {nom} ?"), stocker le contexte pour les textes conditionnels
- [x] T008 Intégrer le wizard dans `uafricas_frontend/app/pages/arbre-genealogique/index.vue` — remplacer l'ouverture du modal PersonneForm par l'ouverture de `AssistantAjoutPersonne.vue` comme comportement par défaut du bouton "Ajouter une personne", gérer les emits `submit` (appel `ajouterPersonne`), `annuler` (fermer), `formulaire-classique` (basculer vers PersonneForm)
- [x] T009 Intégrer le wizard dans `uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue` — remplacer le flux d'ajout contextuel (parent/enfant/conjoint) par l'ouverture de `AssistantAjoutPersonne.vue` avec les props `typeLien` et `personneLiee` pré-remplies depuis le noeud sélectionné, gérer les emits de la même manière que index.vue
- [x] T010 Implémenter la gestion d'erreur réseau dans `AssistantAjoutPersonne.vue` — si la soumission échoue, afficher un message d'erreur sur l'écran récapitulatif avec bouton "Réessayer" sans perdre les données, utiliser la prop `loading` pour l'état de chargement

**Checkpoint**: US1 complète. Le parcours fonctionne de bout en bout sur les deux pages, collecte les données et crée la personne via l'API existante.

---

## Phase 4: User Story 2 — Textes engageants et contextuels (Priority: P1)

**Goal**: Chaque étape affiche un texte d'accroche chaleureux qui s'adapte au contexte des réponses précédentes.

**Independent Test**: Parcourir toutes les étapes et vérifier que les textes sont engageants, pas de simples labels. Indiquer "décédé" et vérifier que le ton change.

### Implementation

- [x] T011 Créer le système de textes dynamiques dans `AssistantAjoutPersonne.vue` — fonction `obtenirTexteAccroche(etape, formulaire, contexte)` qui retourne le texte principal et le texte de transition pour chaque étape, avec variantes selon : le genre saisi, le statut vital (décédé → ton respectueux), le type de lien (depuis visualisation), le nom/prénoms déjà saisis
- [x] T012 Rédiger les textes d'accroche pour les 7 étapes dans `AssistantAjoutPersonne.vue` — étape 1: "Comment s'appelle ce membre de votre famille ?", étape 2: "Magnifique ! Et quel est le prénom de {nom} ?", étape 3: "Pour mieux connaître {prenoms || nom}…", étape 4: "{prenoms || nom} est-il/elle toujours parmi nous ?", étape 5: "Savez-vous quand {prenoms || nom} est né(e) ?" (ou "a vu le jour" si décédé), étape 6: "Et où {prenoms || nom} a-t-il/elle vu le jour ?", étape 7: "Voici le portrait de {prenoms || nom} !" — variantes contextuelles pour le lien (ex: "Qui est la mère de {personneLiee.nom} ?")
- [x] T013 Ajouter les textes de transition entre étapes dans `AssistantAjoutPersonne.vue` — petits textes encourageants affichés brièvement lors du passage à l'étape suivante ("Parfait !", "Très bien !", "Continuons…", "Presque terminé !"), adaptés au ton si personne décédée ("Merci pour ce souvenir", "Honorons sa mémoire")
- [x] T014 Ajouter des icônes FontAwesome contextuelles à chaque étape dans `AssistantAjoutPersonne.vue` — étape 1: fa-user, étape 2: fa-signature, étape 3: fa-venus-mars, étape 4: fa-heart (ou fa-dove si décédé), étape 5: fa-calendar, étape 6: fa-map-marker-alt, étape 7: fa-check-circle — importer via `app/plugins/fontawesome.ts`

**Checkpoint**: US2 complète. Les textes sont chaleureux, contextuels et adaptés au ton selon les réponses.

---

## Phase 5: User Story 3 — Animations GSAP fluides (Priority: P2)

**Goal**: Transitions animées fluides entre étapes, animation d'entrée, animation de célébration.

**Independent Test**: Parcourir le flux complet, vérifier fluidité, tester les clics rapides, vérifier la célébration.

### Implementation

- [x] T015 Mettre en place le cycle de vie GSAP dans `AssistantAjoutPersonne.vue` — `gsap.context()` dans `onMounted` avec template ref comme scope, `ctx.revert()` dans `onBeforeUnmount`, `shallowRef<gsap.core.Timeline>` pour la timeline courante, check `prefers-reduced-motion` pour mode accessibilité
- [x] T016 Implémenter l'animation d'entrée de l'overlay dans `AssistantAjoutPersonne.vue` — fond `opacity: 0 → 1`, contenu central `scale: 0.9, opacity: 0 → scale: 1, opacity: 1` avec ease `back.out(1.4)`, durée 0.4s
- [x] T017 Implémenter les transitions entre étapes dans `AssistantAjoutPersonne.vue` — `watch(etapeCourante)` + `nextTick`, step sortant `gsap.fromTo` slide+fade (`x: 0 → -80, opacity: 1 → 0`), step entrant `gsap.fromTo` slide+fade (`x: 80 → 0, opacity: 0 → 1`), direction inversée pour le retour, durée 0.4s ease `power2.out`
- [x] T018 Implémenter la gestion d'interruption dans `AssistantAjoutPersonne.vue` — avant chaque nouvelle transition, `timelineCourante.value?.kill()` puis `gsap.set()` pour snapper l'ancien step à son état final, empêcher l'empilement d'animations lors de clics rapides
- [x] T019 Implémenter l'animation de célébration dans `AssistantAjoutPersonne.vue` — checkmark SVG animé (`stroke-dashoffset` de pathLength à 0, avec `scale: 0 → 1` ease `back.out(1.7)`), burst de confettis DOM (30-50 divs colorés, positions/rotations aléatoires via `gsap.utils.random()`, nettoyage DOM dans `onComplete`), se déclenche après succès de la soumission
- [x] T020 Ajouter les micro-animations sur les éléments interactifs dans `AssistantAjoutPersonne.vue` — boutons hover/press (`scale: 1.05` / `scale: 0.95`), champ input focus (ring animé), apparition des messages d'erreur (fade+slideY)

**Checkpoint**: US3 complète. Toutes les animations sont fluides, interruptibles, performantes sur mobile.

---

## Phase 6: User Story 4 — Indicateur de progression (Priority: P2)

**Goal**: L'utilisateur voit sa progression dans le parcours via un indicateur visuel animé.

**Independent Test**: Vérifier que l'indicateur reflète l'étape courante, s'anime lors des transitions avant/arrière.

### Implementation

- [x] T021 Implémenter l'indicateur de progression dots dans `AssistantAjoutPersonne.vue` — barre de 6 dots (1 par étape de saisie, excluant le récapitulatif) + segments de liaison entre dots, positionnée en haut de l'overlay, dot actif visuellement distinct (`bg-custom-chocolat`, plus grand), dots complétés en `bg-custom-green`, dots restants en `bg-gray-300`, Tailwind v4 pur
- [x] T022 Animer l'indicateur de progression avec GSAP dans `AssistantAjoutPersonne.vue` — dot actif `gsap.to` scale `1 → 1.3 → 1` + changement `backgroundColor`, segment de liaison `scaleX: 0 → 1` (`transformOrigin: left`), animation parallèle avec la transition d'étape (même timing), animation inverse lors du retour

**Checkpoint**: US4 complète. L'indicateur est clair, animé, cohérent avec la navigation.

---

## Phase 7: User Story 5 — Compatibilité formulaire existant (Priority: P3)

**Goal**: Le formulaire classique PersonneForm reste accessible comme alternative.

**Independent Test**: Cliquer "Formulaire rapide" → le modal classique s'ouvre. Compléter le wizard → données identiques au formulaire classique.

### Implementation

- [x] T023 Ajouter le lien "Formulaire rapide" dans `AssistantAjoutPersonne.vue` — lien discret en bas de l'overlay ("Vous préférez un formulaire classique ? Formulaire rapide"), émet `formulaire-classique` au clic, style texte petit avec underline, couleur `custom-gray`
- [x] T024 Implémenter la bascule wizard/formulaire classique dans `uafricas_frontend/app/pages/arbre-genealogique/index.vue` — état `modeAjout: ref<'wizard' | 'classique'>('wizard')`, sur emit `formulaire-classique` : fermer le wizard et ouvrir le modal PersonneForm existant, les deux modes appellent la même fonction `ajouterPersonne`
- [x] T025 Implémenter la bascule wizard/formulaire classique dans `uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue` — même logique que index.vue avec le contexte de lien, le PersonneForm s'ouvre en modal classique si l'utilisateur bascule

**Checkpoint**: US5 complète. Les deux modes coexistent, les données produites sont identiques.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Améliorations finales affectant plusieurs user stories.

- [x] T026 Optimiser le responsive mobile dans `AssistantAjoutPersonne.vue` — vérifier le rendu à 320px minimum, adapter padding/spacing/font-size pour mobile, s'assurer que les boutons sont assez grands pour le tactile (min 44px), tester l'orientation paysage
- [x] T027 Implémenter le support `prefers-reduced-motion` dans `AssistantAjoutPersonne.vue` — si activé, remplacer tous les `gsap.to`/`gsap.fromTo` par `gsap.set` (instantané), désactiver confettis, garder le checkmark statique
- [x] T028 Migrer les classes daisyUI résiduelles dans `uafricas_frontend/app/components/arbre-genealogique/PersonneForm.vue` — remplacer `radio radio-sm radio-primary` par des styles Tailwind v4 purs (custom radio avec appearance-none + checked states)
- [x] T029 Vérification finale quickstart.md — parcourir les 8 points de vérification du fichier `specs/001-ajout-personne-ludique/quickstart.md` et valider chaque scénario manuellement

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Pas de dépendance — vérification immédiate
- **Foundational (Phase 2)**: Dépend de Phase 1 — **BLOQUE toutes les user stories**
- **US1 (Phase 3)**: Dépend de Phase 2 — MVP, priorité absolue
- **US2 (Phase 4)**: Dépend de Phase 2 — Peut démarrer en parallèle de US1 (même fichier, mais sections différentes), recommandé après US1 pour avoir le squelette
- **US3 (Phase 5)**: Dépend de Phase 2 + US1 (besoin du wizard fonctionnel pour y ajouter les animations)
- **US4 (Phase 6)**: Dépend de Phase 2 + US1 (besoin du wizard pour y intégrer l'indicateur), parallélisable avec US3
- **US5 (Phase 7)**: Dépend de US1 (besoin du wizard intégré dans les pages pour ajouter la bascule)
- **Polish (Phase 8)**: Dépend de toutes les US complétées

### User Story Dependencies

```
Phase 1 (Setup)
    ↓
Phase 2 (Foundational: T002-T005)
    ↓
Phase 3 (US1: T006-T010) ← MVP
    ↓ ↘
Phase 4 (US2: T011-T014)   Phase 5 (US3: T015-T020) ← parallélisables
    ↓                         ↓
Phase 6 (US4: T021-T022) ← parallélisable avec US3
    ↓
Phase 7 (US5: T023-T025)
    ↓
Phase 8 (Polish: T026-T029)
```

### Within Each User Story

- Logique métier avant intégration dans les pages
- Textes/contenu avant animations
- Animations basiques avant micro-animations

### Parallel Opportunities

- **Phase 2** : T002 et T003 séquentiels (même fichier), T004 et T005 séquentiels (même fichier)
- **Phase 3** : T008 et T009 parallélisables (fichiers différents : index.vue et visualisation.vue)
- **Phase 4** : T011 et T014 parallélisables (logique texte vs icônes FontAwesome)
- **Phase 5** : T015, T016, T017, T018 séquentiels (même fichier, dépendances GSAP), T019 et T020 parallélisables après T018
- **Phase 6** : T021 et T022 séquentiels (même fichier)
- **Phase 7** : T024 et T025 parallélisables (fichiers différents)
- **Phase 8** : T026, T027, T028 parallélisables (fichiers différents)

---

## Parallel Example: Phase 3 (US1)

```bash
# Séquentiels (même fichier AssistantAjoutPersonne.vue) :
Task T006: Écran récapitulatif
Task T007: Gestion contexte de lien
Task T010: Gestion erreur réseau

# Parallélisables (fichiers différents) :
Task T008: Intégration index.vue
Task T009: Intégration visualisation.vue
```

## Parallel Example: Phase 8 (Polish)

```bash
# Tous parallélisables (fichiers différents) :
Task T026: Responsive mobile (AssistantAjoutPersonne.vue)
Task T027: prefers-reduced-motion (AssistantAjoutPersonne.vue) — séquentiel avec T026
Task T028: Migration daisyUI PersonneForm.vue
Task T029: Vérification quickstart.md
```

---

## Implementation Strategy

### MVP First (US1 seule — Phase 1-3)

1. Phase 1 : Vérification setup
2. Phase 2 : Squelette wizard (T002-T005)
3. Phase 3 : Parcours complet fonctionnel (T006-T010)
4. **STOP et VALIDER** : Le wizard fonctionne de bout en bout, crée des personnes, sur les deux pages
5. Démontrer au client si OK

### Incremental Delivery

1. Phase 2 → Squelette navigable
2. + US1 → **MVP fonctionnel** (parcours complet)
3. + US2 → Textes engageants (le parcours devient ludique)
4. + US3 → Animations GSAP (le parcours devient beau)
5. + US4 → Indicateur progression (l'utilisateur se repère)
6. + US5 → Formulaire classique en fallback (compatibilité)
7. + Polish → Mobile, accessibilité, nettoyage

---

## Notes

- Tous les fichiers sont dans `uafricas_frontend/app/` — aucune modification backend
- Tailwind CSS v4 pur — zéro classe daisyUI (page publique, Constitution VI)
- Le DTO `CreerPersonneForm` existant dans `app/mocks/arbre-genealogique.ts` est réutilisé tel quel
- GSAP importé directement : `import gsap from 'gsap'` (déjà dans les dépendances)
- Textes en français avec accents (Constitution I)
- Noms de variables/fonctions en français camelCase (etapeCourante, allerSuivant, etc.)
