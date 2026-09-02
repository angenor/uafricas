# Tasks: Visualisation et Navigation de l'Arbre Généalogique

**Input**: Design documents from `/specs/001-visualisation-arbre/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Non demandés : pas de tâches de test.

**Organization**: Tâches groupées par user story pour implémentation et test indépendants.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut tourner en parallèle (fichiers différents, pas de dépendance)
- **[Story]**: User story associée (US1, US2, US3, US4, US5, US6)
- Chemins exacts inclus dans les descriptions

## Path Conventions

- **Backend**: `uafricas_backend/src/`
- **Frontend**: `uafricas_frontend/app/`

---

## Phase 1: Setup (Infrastructure partagée)

**Purpose**: Installation des dépendances et types de base pour la visualisation

- [x] T001 Installer les dépendances npm : `@vue-flow/core`, `@vue-flow/controls`, `@vue-flow/minimap`, `relatives-tree` dans `uafricas_frontend/package.json`
- [x] T002 Ajouter les types TypeScript pour l'arbre complet (interfaces `ArbreComplet`, `PersonneNoeud`, `LienArbreResponse`) dans `uafricas_frontend/app/mocks/arbre-genealogique.ts`
- [x] T003 Ajouter les données mock pour l'arbre complet (8 personnes, 7 liens sur 3 générations avec couples et multi-enfants) dans `uafricas_frontend/app/mocks/arbre-genealogique.ts`

---

## Phase 2: Foundational (Prérequis bloquants)

**Purpose**: Endpoint backend + composable frontend + moteur de layout, DOIT être terminé avant toute user story

**⚠️ CRITICAL**: Aucune tâche de user story ne peut commencer avant la fin de cette phase

- [x] T004 [P] Ajouter les structs `ArbreCompletResponse`, `PersonneNoeud`, `LienArbreResponse` dans `uafricas_backend/src/models/arbre_genealogique.rs` (voir contrat `contracts/api-arbre-complet.md`)
- [x] T005 [P] Implémenter le handler `obtenir_arbre_complet` dans `uafricas_backend/src/handlers/arbre_genealogique.rs`, requête SQL joignant `rattachements` + `personnes` et `liens_familiaux`, filtrées par `arbre_id` de l'utilisateur connecté, sans pagination
- [x] T006 Ajouter la route `.route("/arbre-complet", web::get().to(arbre_genealogique::obtenir_arbre_complet))` dans le scope `/arbre` de `uafricas_backend/src/routes.rs` (dépend de T005)
- [x] T007 Ajouter la méthode `obtenirArbreComplet()` dans `uafricas_frontend/app/composables/useArbreGenealogique.ts`, appel `GET /api/arbre/arbre-complet` avec header JWT
- [x] T008 Créer le composable `useLayoutArbre` dans `uafricas_frontend/app/composables/useLayoutArbre.ts`, conversion des données API (`PersonneNoeud[]` + `LienArbreResponse[]`) en entrée `relatives-tree`, puis mapping des positions calculées vers des nodes/edges vue-flow. Inclure : fonction `calculerLayout(personnes, liens, centreId, mode)`, construction du graphe adjacence (`parents[]`, `enfants[]`, `conjoints[]` par rattachement_id), calcul de génération par BFS, filtrage par mode (complet 3 générations / ascendant / descendant)

**Checkpoint**: Backend endpoint fonctionnel + composables frontend prêts, l'implémentation des user stories peut commencer

---

## Phase 3: User Story 1 : Visualiser son arbre sous forme graphique (Priority: P1) 🎯 MVP

**Goal**: L'utilisateur voit un rendu graphique de toutes ses personnes et liens familiaux sur une page dédiée `/arbre-genealogique/visualisation`

**Independent Test**: Ajouter 5-10 personnes avec liens via Feature 1, accéder à `/arbre-genealogique/visualisation`, vérifier que tous les nœuds et connexions apparaissent

### Implementation for User Story 1

- [x] T009 [P] [US1] Créer le composant `NoeudPersonne.vue` dans `uafricas_frontend/app/components/arbre-genealogique/NoeudPersonne.vue`, custom node vue-flow affichant : photo (ou initiales via `getInitiales`), nom complet, années naissance/décès (via `formaterDate`). Tailwind CSS v4 pur (pas de daisyUI). Props : données PersonneNoeud. Style : carte compacte avec bordure colorée selon genre
- [x] T010 [P] [US1] Créer le composant `ArbreGraphe.vue` dans `uafricas_frontend/app/components/arbre-genealogique/ArbreGraphe.vue`, wrapper `<VueFlow>` avec : custom node type `personne` → `NoeudPersonne.vue`, edges avec styles différents parent-enfant (verticaux, trait plein) vs conjoint (horizontaux, trait pointillé), props `nodes` et `edges` réactifs, événement `@node-click` émis vers le parent. Envelopper dans `<ClientOnly>` avec skeleton de chargement
- [x] T011 [US1] Créer la page `visualisation.vue` dans `uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`, page pleine hauteur (h-screen moins navbar), appel `obtenirArbreComplet()` au montage, passage des données à `useLayoutArbre.calculerLayout()` puis à `ArbreGraphe.vue`. État vide si aucune personne (message + bouton « Ajouter ma première personne » redirigeant vers `/arbre-genealogique`). État de chargement avec skeleton. Middleware auth (redirection si non connecté)
- [x] T012 [US1] Modifier la page index dans `uafricas_frontend/app/pages/arbre-genealogique/index.vue`, ajouter un bouton/lien « Voir mon arbre » redirigeant vers `/arbre-genealogique/visualisation`, placé à côté du bouton « Ajouter une personne » existant

**Checkpoint**: L'arbre est visible graphiquement avec nœuds et connexions, US1 fonctionnelle et testable

---

## Phase 4: User Story 2 : Naviguer en vue centrée sur une personne (Priority: P1)

**Goal**: Un clic sur un nœud recentre la vue avec transition animée et ouvre un panneau contextuel (mini-fiche) avec bouton « Voir détail »

**Independent Test**: Cliquer sur différentes personnes, vérifier recentrage animé + panneau avec informations correctes + navigation vers fiche détail

### Implementation for User Story 2

- [x] T013 [P] [US2] Créer le composant `PanneauPersonne.vue` dans `uafricas_frontend/app/components/arbre-genealogique/PanneauPersonne.vue`, panneau latéral (desktop: droite, fixé) affichant : photo/initiales, nom complet, genre, dates naissance/décès avec lieux, nombre de parents/enfants/conjoints. Bouton « Voir détail » (NuxtLink vers `/arbre-genealogique/{id}`). Bouton fermer. Props : `PersonneNoeud | null`. Tailwind CSS v4 pur. Animation slide-in
- [x] T014 [US2] Intégrer la navigation centrée dans `uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`, gérer `@node-click` de `ArbreGraphe.vue` : appeler `setCenter(x, y, { duration: 800 })` de vue-flow pour transition animée, mettre à jour la personne sélectionnée (ref réactive), afficher `PanneauPersonne.vue` avec les données de la personne cliquée, recalculer le layout avec la nouvelle personne comme centre (via `useLayoutArbre`)
- [x] T015 [US2] Ajouter dans `ArbreGraphe.vue` (`uafricas_frontend/app/components/arbre-genealogique/ArbreGraphe.vue`) la mise en surbrillance du nœud sélectionné, classe CSS distincte sur le nœud actif (bordure plus épaisse ou couleur `custom-chocolat`), transmission de `selectedId` comme prop

**Checkpoint**: Navigation par clic fonctionnelle avec recentrage animé + panneau contextuel, US2 testable

---

## Phase 5: User Story 3 + 4 : Navigation ascendante et descendante (Priority: P2)

**Goal**: L'utilisateur peut basculer entre 3 modes de vue : complet (3 générations), ascendant (ancêtres), descendant (descendants)

**Independent Test**: Sélectionner une personne avec 3+ générations d'ancêtres, activer mode ascendant → seuls ancêtres visibles. Activer mode descendant → seuls descendants visibles. Revenir en mode complet → 3 générations autour du centre

### Implementation for User Stories 3 & 4

- [x] T016 [P] [US3] Créer le composant `BarreOutils.vue` dans `uafricas_frontend/app/components/arbre-genealogique/BarreOutils.vue`, barre horizontale en haut de la visualisation avec 3 boutons radio/toggle : « Vue complète » (défaut, icône arbre), « Ancêtres ↑ » (icône flèche haut), « Descendants ↓ » (icône flèche bas). Émet `@mode-change(mode: 'complet' | 'ascendant' | 'descendant')`. Tailwind CSS v4 pur. Style : boutons groupés avec état actif visuellement distinct (fond `custom-green`)
- [x] T017 [US3] Implémenter le filtre ascendant dans `uafricas_frontend/app/composables/useLayoutArbre.ts`, fonction `filtrerAncetres(graphe, centreId)` : DFS remontant via `parents[]` depuis la personne centrée, retourne uniquement les nœuds ancêtres + la personne. Si aucun parent renseigné : retourner uniquement la personne centrée. Recalculer les edges filtrés
- [x] T018 [US4] Implémenter le filtre descendant dans `uafricas_frontend/app/composables/useLayoutArbre.ts`, fonction `filtrerDescendants(graphe, centreId)` : DFS descendant via `enfants[]` depuis la personne centrée, retourne uniquement les nœuds descendants + la personne. Si aucun enfant : retourner uniquement la personne centrée. Recalculer les edges filtrés
- [x] T019 [US3] Intégrer `BarreOutils.vue` et les modes dans `uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`, ajouter `BarreOutils` au-dessus de `ArbreGraphe`, gérer `@mode-change` : passer le mode à `useLayoutArbre.calculerLayout()`, recalculer nodes/edges réactivement, animer la transition avec `fitView({ duration: 500 })`. Afficher indicateur « Aucun ancêtre renseigné » ou « Aucun descendant renseigné » si le filtre retourne uniquement la personne

**Checkpoint**: 3 modes de navigation fonctionnels et basculement < 1 seconde, US3 et US4 testables

---

## Phase 6: User Story 5 : Utilisation sur mobile (Priority: P2)

**Goal**: La visualisation est fonctionnelle et lisible sur smartphone (320px+), avec gestes tactiles (pinch zoom, pan, tap)

**Independent Test**: Accéder à `/arbre-genealogique/visualisation` depuis un smartphone ou émulateur mobile, arbre de 10+ personnes, vérifier : zoom pinch, pan glisser, tap pour sélectionner, lisibilité des nœuds

### Implementation for User Story 5

- [x] T020 [P] [US5] Adapter `NoeudPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/NoeudPersonne.vue`) pour mobile, réduire la taille des nœuds sur petit écran (taille de police, padding), cacher les détails secondaires (lieux) sous 768px, augmenter la zone tactile (min 44x44px), utiliser les breakpoints Tailwind CSS v4 (`@max-md:`)
- [x] T021 [P] [US5] Adapter `PanneauPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/PanneauPersonne.vue`) pour mobile, transformer en bottom sheet sur écran < 768px (position fixed en bas, hauteur 40% max, glissable pour fermer), conserver le panneau latéral sur desktop. Transition CSS pour l'animation
- [x] T022 [P] [US5] Adapter `BarreOutils.vue` (`uafricas_frontend/app/components/arbre-genealogique/BarreOutils.vue`) pour mobile, boutons plus compacts sur petit écran (icônes seules sans texte sous 640px), barre fixée en haut sans chevauchement avec la navbar
- [x] T023 [US5] Vérifier et ajuster les paramètres tactiles de vue-flow dans `ArbreGraphe.vue` (`uafricas_frontend/app/components/arbre-genealogique/ArbreGraphe.vue`), s'assurer que `panOnDrag`, `zoomOnPinch`, `zoomOnScroll` sont activés, ajuster `minZoom`/`maxZoom` pour mobile (0.3 à 3), désactiver le scroll de la page quand le geste est dans le canvas (prevent default sur touch events)

**Checkpoint**: Visualisation pleinement fonctionnelle sur mobile, US5 testable

---

## Phase 7: User Story 6 : Zoom et déplacement desktop (Priority: P3)

**Goal**: Contrôles de zoom explicites (boutons +/-, molette) et bouton de réinitialisation de la vue

**Independent Test**: Arbre de 20+ personnes, zoom molette, pan drag, clic bouton réinitialiser → retour vue initiale

### Implementation for User Story 6

- [x] T024 [US6] Intégrer `@vue-flow/controls` dans `ArbreGraphe.vue` (`uafricas_frontend/app/components/arbre-genealogique/ArbreGraphe.vue`), ajouter le composant `<Controls>` avec boutons zoom in/out. Ajouter un bouton custom « Réinitialiser la vue » qui appelle `fitView({ duration: 500 })` pour revenir à la position et zoom initiaux. Positionner les contrôles en bas à droite. Optionnel : ajouter `<MiniMap>` de `@vue-flow/minimap` en bas à gauche pour les grands arbres
- [x] T025 [US6] Ajouter le bouton de réinitialisation dans `BarreOutils.vue` (`uafricas_frontend/app/components/arbre-genealogique/BarreOutils.vue`), bouton « Réinitialiser » (icône home/reset) qui émet `@reset-view`, géré dans `visualisation.vue` pour appeler `fitView()` sur l'instance vue-flow

**Checkpoint**: Zoom/pan/reset fonctionnels desktop et mobile, US6 testable

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Améliorations transversales à toutes les user stories

- [x] T026 [P] Ajouter les boutons d'expansion « Voir plus » aux extrémités de l'arbre dans `ArbreGraphe.vue` (`uafricas_frontend/app/components/arbre-genealogique/ArbreGraphe.vue`), nœuds fantômes cliquables sur les bords de l'arbre (là où des générations supplémentaires existent mais ne sont pas affichées), au clic : étendre le nombre de générations visibles et recalculer le layout
- [x] T027 [P] Gérer l'état de chargement et les erreurs réseau dans `uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`, skeleton animé pendant le chargement, message d'erreur avec bouton « Réessayer » en cas d'échec API, gestion du token expiré (redirection login)
- [x] T028 Vérification de cohérence des styles Tailwind v4 dans tous les nouveaux composants, s'assurer qu'aucune classe daisyUI n'est utilisée, migrer d'éventuels résidus v3 détectés, vérifier les couleurs du thème (`custom-chocolat`, `custom-green`, `custom-gray`)
- [x] T029 Exécuter le scénario de validation quickstart.md, créer 5+ personnes, liens parent-enfant et conjoint, naviguer vers visualisation, tester les 3 modes, tester mobile, vérifier les performances

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Aucune dépendance, démarrage immédiat
- **Foundational (Phase 2)**: Dépend de Phase 1, BLOQUE toutes les user stories
- **US1 (Phase 3)**: Dépend de Phase 2 : aucune dépendance inter-story
- **US2 (Phase 4)**: Dépend de Phase 3 (US1 fournit le graphe sur lequel naviguer)
- **US3+US4 (Phase 5)**: Dépend de Phase 4 (US2 fournit le recentrage + la BarreOutils est nouvelle)
- **US5 (Phase 6)**: Dépend de Phase 5 (adapte les composants créés en US1-4)
- **US6 (Phase 7)**: Dépend de Phase 3 (US1 fournit le graphe), peut tourner en parallèle avec US5
- **Polish (Phase 8)**: Dépend de toutes les phases précédentes

### User Story Dependencies

```
Phase 1 (Setup) → Phase 2 (Foundational)
                        │
                        ▼
                   Phase 3 (US1) 🎯 MVP
                        │
                        ▼
                   Phase 4 (US2)
                        │
                   ┌────┴────┐
                   ▼         ▼
            Phase 5 (US3+4)  Phase 7 (US6) [parallélisable]
                   │         │
                   ▼         │
            Phase 6 (US5)    │
                   │         │
                   └────┬────┘
                        ▼
                   Phase 8 (Polish)
```

### Within Each User Story

- Composants [P] avant intégration dans la page
- Composable/logique avant composants qui l'utilisent
- Commit après chaque tâche ou groupe logique

### Parallel Opportunities

- **Phase 2** : T004 + T005 en parallèle (models et handler dans des fichiers différents)
- **Phase 3** : T009 + T010 en parallèle (NoeudPersonne + ArbreGraphe dans des fichiers différents)
- **Phase 4** : T013 en parallèle avec T015 (PanneauPersonne + highlight dans des fichiers différents)
- **Phase 5** : T016 en parallèle avec T017 + T018 (BarreOutils et filtres dans des fichiers différents)
- **Phase 6** : T020 + T021 + T022 en parallèle (adaptations mobile de 3 composants différents)
- **Phase 7** : Peut tourner en parallèle avec Phase 6
- **Phase 8** : T026 + T027 en parallèle

---

## Parallel Example: Phase 2 (Foundational)

```bash
# Lancer en parallèle (fichiers différents) :
Task T004: "Ajouter structs ArbreCompletResponse dans models/arbre_genealogique.rs"
Task T005: "Implémenter handler obtenir_arbre_complet dans handlers/arbre_genealogique.rs"

# Puis séquentiellement :
Task T006: "Ajouter route /arbre-complet dans routes.rs" (dépend de T005)
Task T007: "Ajouter obtenirArbreComplet() dans useArbreGenealogique.ts"
Task T008: "Créer useLayoutArbre.ts"
```

## Parallel Example: Phase 3 (US1 : MVP)

```bash
# Lancer en parallèle (composants indépendants) :
Task T009: "Créer NoeudPersonne.vue"
Task T010: "Créer ArbreGraphe.vue"

# Puis séquentiellement :
Task T011: "Créer visualisation.vue" (dépend de T009, T010)
Task T012: "Modifier index.vue" (indépendant mais logiquement après T011)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Compléter Phase 1: Setup (T001-T003)
2. Compléter Phase 2: Foundational (T004-T008), CRITIQUE
3. Compléter Phase 3: User Story 1 (T009-T012)
4. **STOP et VALIDER** : Arbre graphique visible avec nœuds et connexions
5. Démo possible avec un arbre de test

### Incremental Delivery

1. Setup + Foundational → Infrastructure prête
2. US1 → Arbre visible → **MVP déployable**
3. US2 → Navigation centrée + panneau → Exploration fonctionnelle
4. US3+US4 → Modes ascendant/descendant → Navigation complète
5. US5 → Mobile → Audience élargie
6. US6 → Zoom/controls → Finitions desktop
7. Polish → Expansion progressive + robustesse

### Estimation de charge

| Phase | Tâches | Fichiers touchés | Priorité |
|-------|--------|-----------------|----------|
| Setup | 3 | 2 | : |
| Foundational | 5 | 5 | : |
| US1 (P1) | 4 | 4 | MVP |
| US2 (P1) | 3 | 3 | MVP+ |
| US3+US4 (P2) | 4 | 3 | Incrémental |
| US5 (P2) | 4 | 4 | Incrémental |
| US6 (P3) | 2 | 2 | Finition |
| Polish | 4 | 3 | Final |
| **Total** | **29** | **~12 uniques** |, |

---

## Notes

- [P] = fichiers différents, pas de dépendances
- [Story] = traçabilité vers la user story de la spec
- Tailwind CSS v4 pur sur tous les composants (pas de daisyUI, page publique)
- Noms de variables, composants et commentaires en français
- Commit après chaque tâche ou groupe logique
- Arrêter à tout checkpoint pour valider la story indépendamment
