# Tasks: Édition Interactive de l'Arbre Généalogique

**Input**: Design documents from `/specs/001-edition-arbre/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md

**Tests**: Non demandés : pas de tâches de test.

**Organization**: Tâches groupées par user story. Feature purement frontend : 5 fichiers existants modifiés, 0 nouveau fichier.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut tourner en parallèle (fichiers différents, pas de dépendance)
- **[Story]**: User story associée (US1, US2, US3, US4, US5)
- Chemins exacts inclus dans les descriptions

## Path Conventions

- **Frontend**: `uafricas_frontend/app/`

---

## Phase 1: Setup

**Purpose**: Aucune dépendance à installer. Préparation du composable.

- [x] T001 Ajouter la fonction `calculerIncompletude(graphe)` dans `uafricas_frontend/app/composables/useLayoutArbre.ts`, parcourt tous les nœuds du graphe, retourne un `Map<string, { estIncomplet: boolean, messageManquant: string | null }>` basé sur `noeud.parents.length < 2`. Si 0 parents → "Parents manquants", si 1 parent de type père → "Mère manquante", si 1 parent de type mère → "Père manquant", sinon "Parent manquant". Exporter aussi `compterBranchesIncompletes(graphe): number`.

---

## Phase 2: Foundational (Prérequis bloquants)

**Purpose**: Transformer `PanneauPersonne.vue` en composant multi-mode (fiche / ajout / modifier), DOIT être terminé avant les user stories

**⚠️ CRITICAL**: Aucune tâche de user story ne peut commencer avant la fin de cette phase

- [x] T002 Refactorer `PanneauPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/PanneauPersonne.vue`) pour supporter 3 modes : `'fiche'` (mini-fiche existante), `'ajout'` (formulaire de création), `'modifier'` (formulaire de modification). Ajouter une prop `mode` (défaut: `'fiche'`) et un emit `@mode-change`. En mode `'fiche'` : afficher le contenu actuel (photo, nom, dates, stats liens). En mode `'ajout'` ou `'modifier'` : afficher un slot/composant formulaire à la place. Ajouter un en-tête avec bouton « Retour » quand mode ≠ `'fiche'`. Conserver l'animation slide-in et le responsive desktop/mobile existants.

- [x] T003 Ajouter les types `ModePanneau` et `ContexteAjout` dans `uafricas_frontend/app/mocks/arbre-genealogique.ts`, `ModePanneau = 'fiche' | 'ajout' | 'modifier'` et `ContexteAjout = { personneSourceId: string, typeAction: 'parent' | 'enfant' | 'conjoint', typeLienSuggere: TypeLien }`. Ajouter aussi une fonction `suggererTypeLien(typeAction, parentsExistants): TypeLien` qui retourne le type de lien le plus approprié.

**Checkpoint**: PanneauPersonne supporte les 3 modes avec navigation, les user stories peuvent commencer

---

## Phase 3: User Story 1 + 2 : Ajouter un membre + Validation guidée (Priority: P1) 🎯 MVP

**Goal**: L'utilisateur peut ajouter un parent, enfant ou conjoint depuis la vue arbre via le panneau latéral. Le système valide les liens (cycles, doublons, types adaptés).

**Independent Test**: Cliquer un nœud → bouton "Ajouter un enfant" → formulaire → valider → nouveau nœud visible dans l'arbre

### Implementation for User Stories 1 & 2

- [x] T004 [US1] Ajouter les boutons d'action dans `PanneauPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/PanneauPersonne.vue`) en mode `'fiche'`, sous le bouton "Voir le détail complet" existant, ajouter une section "Actions" avec 5 boutons : « Ajouter un parent » (icône `user-plus`), « Ajouter un enfant » (icône `user-plus`), « Ajouter un conjoint(e) » (icône `heart`), « Modifier » (icône `pen-to-square`), « Supprimer » (icône `trash`, rouge). Chaque bouton émet un événement vers le parent avec le type d'action. Tailwind CSS v4 pur.

- [x] T005 [US1] Implémenter le mode `'ajout'` dans `PanneauPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/PanneauPersonne.vue`), quand mode = `'ajout'`, afficher : titre contextuel (ex: "Ajouter un enfant de [NomPersonne]"), le composant `PersonneForm.vue` (importé depuis `~/components/arbre-genealogique/PersonneForm.vue`), un sélecteur de type de lien pré-rempli (père/mère/parent/conjoint selon `ContexteAjout.typeLienSuggere`) avec possibilité de changer. Bouton "Retour" en haut. Props : `contexteAjout: ContexteAjout`, emit `@personne-ajoutee(form, typeLien)`.

- [x] T006 [US1] Gérer le flux d'ajout dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), quand l'utilisateur clique un bouton d'action "Ajouter parent/enfant/conjoint" dans le panneau : 1) calculer le `ContexteAjout` (personne source, type action, type lien suggéré via `suggererTypeLien`), 2) passer le panneau en mode `'ajout'`, 3) quand le formulaire est soumis (`@personne-ajoutee`) : appeler `creerPersonne(form)` → puis `creerLien({ rattachement_source_id, rattachement_cible_id, type_lien })` → puis `obtenirArbreComplet()` pour recharger → recalculer le layout. Gérer les inversions source/cible selon le type (pour "ajouter un parent" : la nouvelle personne est source, le nœud existant est cible).

- [x] T007 [US2] Ajouter la gestion des erreurs de lien dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), intercepter les erreurs de l'API `creerLien` (422 cycle détecté, 409 doublon) et les afficher dans le panneau. Mapper les messages d'erreur backend en messages français lisibles : "Lien circulaire détecté" → "Ce lien créerait un cycle dans l'arbre (une personne ne peut pas être son propre ancêtre)", "Ce lien familial existe déjà" → "Un lien de ce type existe déjà entre ces deux personnes". En cas d'erreur de création du lien, supprimer la personne qui vient d'être créée (rollback).

- [x] T008 [US2] Implémenter le filtrage intelligent des types de lien dans la fonction `suggererTypeLien` de `uafricas_frontend/app/mocks/arbre-genealogique.ts`, analyser les parents existants de la personne cible : si un père existe → suggérer `'mere'`, si une mère existe → suggérer `'pere'`, si les deux existent → suggérer `'parent'` avec avertissement. Pour les conjoints : toujours `'conjoint'`. Pour les enfants depuis un homme → `'pere'`, depuis une femme → `'mere'`, sinon → `'parent'`.

- [x] T009 [US1] Ajouter la détection d'homonymes dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), quand le formulaire d'ajout est soumis, avant l'appel API, vérifier si un nœud dans le graphe a les mêmes nom+prénoms (comparaison insensible à la casse). Si oui, afficher un avertissement discret (toast ou texte dans le panneau) : « Une personne portant le même nom existe déjà dans votre arbre ». Ne pas bloquer la soumission.

**Checkpoint**: Ajout contextuel fonctionnel avec validation guidée, US1 et US2 testables

---

## Phase 4: User Story 3 : Modifier depuis la vue arbre (Priority: P2)

**Goal**: L'utilisateur peut modifier les informations d'une personne directement dans le panneau latéral

**Independent Test**: Cliquer un nœud → "Modifier" → changer le prénom → valider → nœud mis à jour

### Implementation for User Story 3

- [x] T010 [US3] Implémenter le mode `'modifier'` dans `PanneauPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/PanneauPersonne.vue`), quand mode = `'modifier'`, afficher : titre "Modifier [NomPersonne]", le composant `PersonneForm.vue` pré-rempli avec les données actuelles de la personne (convertir `NoeudArbre` en `CreerPersonneForm` compatible). Bouton "Retour" en haut. Emit `@personne-modifiee(form)`.

- [x] T011 [US3] Gérer le flux de modification dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), quand "Modifier" est cliqué : passer le panneau en mode `'modifier'`. Quand le formulaire est soumis (`@personne-modifiee`) : appeler `modifierPersonne(personne_id, form)` → recharger arbre-complet → recalculer layout. En cas d'erreur (validation dates), afficher le message dans le panneau. Après succès, revenir en mode `'fiche'` avec les données mises à jour.

**Checkpoint**: Modification en place fonctionnelle, US3 testable

---

## Phase 5: User Story 4 : Supprimer depuis la vue arbre (Priority: P2)

**Goal**: L'utilisateur peut supprimer une personne avec confirmation et cascade

**Independent Test**: Cliquer un nœud → "Supprimer" → confirmer → nœud disparaît

### Implementation for User Story 4

- [x] T012 [US4] Implémenter la confirmation de suppression dans `PanneauPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/PanneauPersonne.vue`), quand "Supprimer" est cliqué : calculer le nombre de liens de la personne (à partir de `parents.length + enfants.length + conjoints.length`), afficher un dialogue de confirmation dans le panneau : "Supprimer [NomPersonne] ?" + "X liens familiaux seront supprimés" + boutons "Confirmer la suppression" (rouge) et "Annuler". Emit `@personne-supprimee`.

- [x] T013 [US4] Gérer le flux de suppression dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), quand la suppression est confirmée (`@personne-supprimee`) : appeler `supprimerPersonne(personne_id)` → recharger arbre-complet → recalculer layout. Si la personne supprimée était le centre (`centreId`), recentrer sur le nœud le plus connecté (réutiliser la logique existante). Si l'arbre est vide, afficher l'état vide. Fermer le panneau après suppression.

**Checkpoint**: Suppression avec confirmation fonctionnelle, US4 testable

---

## Phase 6: User Story 5 : Indicateurs de branches incomplètes (Priority: P3)

**Goal**: Indicateurs visuels sur les nœuds incomplets + compteur global

**Independent Test**: Arbre avec personnes sans parents → badges visibles + compteur dans la barre d'outils

### Implementation for User Story 5

- [x] T014 [P] [US5] Ajouter le badge d'incomplétude dans `NoeudPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/NoeudPersonne.vue`), recevoir une nouvelle prop `incompletude: { estIncomplet: boolean, messageManquant: string | null }`. Si `estIncomplet`, afficher un petit badge en haut à droite du nœud (cercle orange/ambre avec icône `exclamation-triangle` ou `+`). Le badge affiche un tooltip au survol avec `messageManquant`. Cliquer sur le badge émet `@ajout-parent-demande` avec l'id du nœud.

- [x] T015 [P] [US5] Ajouter le compteur dans `BarreOutils.vue` (`uafricas_frontend/app/components/arbre-genealogique/BarreOutils.vue`), recevoir une nouvelle prop `nbBranchesIncompletes: number`. Si > 0, afficher un badge à côté des boutons de mode : "[N] branche(s) à compléter" avec un petit cercle ambre. Sur mobile (`max-sm:`), afficher uniquement le nombre avec icône.

- [x] T016 [US5] Intégrer les indicateurs dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), après chaque `calculerLayout`, appeler `calculerIncompletude(graphe)` pour obtenir la map d'incomplétude et le compteur. Passer les données d'incomplétude aux nœuds via le `data` de chaque node vue-flow. Passer `nbBranchesIncompletes` à `BarreOutils`. Quand un nœud émet `@ajout-parent-demande`, ouvrir le panneau en mode `'ajout'` avec `typeAction: 'parent'` pour cette personne.

**Checkpoint**: Badges incomplétude + compteur fonctionnels, US5 testable

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Robustesse et finitions

- [x] T017 [P] Gérer les états de chargement pendant les mutations dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), ajouter un ref `mutationEnCours` (boolean), désactiver le bouton "Valider" du formulaire pendant les appels API (FR-013), afficher un spinner dans le panneau pendant le rechargement de l'arbre
- [x] T018 [P] Gérer la résilience réseau dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), en cas d'erreur réseau (fetch failed), conserver les données du formulaire (FR-014), afficher un message "Connexion perdue. Veuillez réessayer." avec un bouton "Réessayer" qui relance la dernière action
- [x] T019 Vérification de cohérence Tailwind CSS v4 dans les composants modifiés, s'assurer qu'aucune classe daisyUI n'est utilisée dans les boutons d'action, la confirmation de suppression, les badges d'incomplétude
- [x] T020 Exécuter le scénario de validation quickstart.md, parcourir les 9 étapes de vérification

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Aucune dépendance, démarrage immédiat
- **Foundational (Phase 2)**: Dépend de Phase 1, BLOQUE toutes les user stories
- **US1+US2 (Phase 3)**: Dépend de Phase 2, MVP
- **US3 (Phase 4)**: Dépend de Phase 2 (pas de Phase 3 strictement, mais logiquement après)
- **US4 (Phase 5)**: Dépend de Phase 2
- **US5 (Phase 6)**: Dépend de Phase 1 (utilise `calculerIncompletude`)
- **Polish (Phase 7)**: Dépend de toutes les phases précédentes

### User Story Dependencies

```
Phase 1 (Setup) → Phase 2 (Foundational)
                        │
                   ┌────┴────────────┐
                   ▼                  ▼
            Phase 3 (US1+US2)   Phase 6 (US5) [parallélisable]
            🎯 MVP                    │
                   │                  │
              ┌────┴────┐            │
              ▼         ▼            │
        Phase 4 (US3) Phase 5 (US4)  │
              │         │            │
              └────┬────┘            │
                   └────┬────────────┘
                        ▼
                   Phase 7 (Polish)
```

### Parallel Opportunities

- **Phase 3** : T004 peut commencer pendant que T003 finit (fichiers différents, mais T005 dépend de T004)
- **Phase 6** : T014 + T015 en parallèle (NoeudPersonne + BarreOutils = fichiers différents)
- **Phase 6 ∥ Phase 4 ∥ Phase 5** : US3, US4 et US5 peuvent tourner en parallèle après Phase 2
- **Phase 7** : T017 + T018 en parallèle (même fichier mais sections indépendantes)

---

## Implementation Strategy

### MVP First (User Stories 1+2 Only)

1. Compléter Phase 1: Setup (T001)
2. Compléter Phase 2: Foundational (T002–T003)
3. Compléter Phase 3: US1+US2 (T004–T009)
4. **STOP et VALIDER** : Ajout contextuel fonctionnel avec validation guidée

### Incremental Delivery

1. Setup + Foundational → Panneau multi-mode prêt
2. US1+US2 → Ajout contextuel → **MVP**
3. US3 → Modification en place
4. US4 → Suppression avec confirmation
5. US5 → Indicateurs d'incomplétude
6. Polish → Robustesse + validation

### Estimation de charge

| Phase | Tâches | Fichiers touchés | Priorité |
|-------|--------|-----------------|----------|
| Setup | 1 | 1 | : |
| Foundational | 2 | 2 | : |
| US1+US2 (P1) | 6 | 3 | MVP |
| US3 (P2) | 2 | 2 | Incrémental |
| US4 (P2) | 2 | 2 | Incrémental |
| US5 (P3) | 3 | 3 | Finition |
| Polish | 4 | 2 | Final |
| **Total** | **20** | **5 uniques** | : |

---

## Notes

- Feature purement frontend : 0 fichier backend modifié
- 5 fichiers touchés : `useLayoutArbre.ts`, `PanneauPersonne.vue`, `visualisation.vue`, `NoeudPersonne.vue`, `BarreOutils.vue` + `arbre-genealogique.ts` (types)
- Tailwind CSS v4 pur sur tous les composants
- Noms de variables, composants et commentaires en français
- Commit après chaque tâche ou groupe logique
