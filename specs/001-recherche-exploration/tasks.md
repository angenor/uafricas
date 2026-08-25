# Tasks: Recherche et Exploration de l'Arbre

**Input**: Design documents from `/specs/001-recherche-exploration/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Non demandés.

**Organization**: Majoritairement frontend. 1 endpoint backend. Composable central + 3 composants + intégration.

## Format: `[ID] [P?] [Story] Description`

## Path Conventions

- **Backend**: `uafricas_backend/src/`
- **Frontend**: `uafricas_frontend/app/`

---

## Phase 1: Setup

**Purpose**: Composable central avec les algorithmes de recherche, chemin et filtres

- [x] T001 Créer le composable `useRechercheArbre` dans `uafricas_frontend/app/composables/useRechercheArbre.ts`, exporter les fonctions : `rechercherLocal(graphe, terme): ResultatRecherche[]` (filtre côté client sur nom/prenoms/lieu/date, normalisation casse/diacritiques, retourne max 20 résultats), `calculerCheminParente(graphe, sourceId, cibleId): CheminParente | null` (BFS bidirectionnel pour trouver le plus court chemin, calcul du LCA), `decrireRelation(genMontantes, genDescendantes, genreSource): string` (terminologie française : père/mère/grand-père/cousin au Nème degré selon la table de recherche research.md), `filtrerParLieu(graphe, terme): Set<string>`, `filtrerParGeneration(graphe, centreId, plage): Set<string>`, `filtrerParBranche(graphe, centreId, parentId): Set<string>`. Aussi exporter les types `ResultatRecherche`, `CheminParente`, `FiltreArbre`.

---

## Phase 2: Foundational (Endpoint recherche publique)

**Purpose**: Endpoint backend pour la recherche dans tous les arbres

- [x] T002 Ajouter le handler `recherche_publique` dans `uafricas_backend/src/handlers/matching.rs`, GET /api/arbre/recherche-publique?q=... : valider q >= 2 caractères, requête pg_trgm sur `nom_normalise` et `prenoms_normalise` de toutes les personnes hors de l'arbre de l'utilisateur, retourner max 20 résultats avec score_similarite, nom, prenoms, naissance_annee, naissance_lieu, genre, membre_id_anonymise. Trier par score décroissant.
- [x] T003 Ajouter la route `/recherche-publique` (GET) dans le scope `/arbre` de `uafricas_backend/src/routes.rs`
- [x] T004 Ajouter la méthode `rechercherPublique(terme)` dans `uafricas_frontend/app/composables/useDecouvertes.ts`, appel GET /api/arbre/recherche-publique?q=... avec header JWT

**Checkpoint**: Endpoint recherche publique fonctionnel

---

## Phase 3: User Story 1 : Recherche dans son propre arbre (Priority: P1) 🎯 MVP

**Goal**: Champ de recherche dans la barre d'outils, résultats instantanés, clic pour centrer

**Independent Test**: Taper "Diallo" → résultats instantanés → cliquer → vue centrée

### Implementation for User Story 1

- [x] T005 [P] [US1] Créer le composant `ChampRecherche.vue` dans `uafricas_frontend/app/components/arbre-genealogique/ChampRecherche.vue`, champ de recherche avec : icône loupe, placeholder "Rechercher une personne...", toggle segmenté "Mon arbre / Tous les arbres" (défaut: Mon arbre), dropdown des résultats sous le champ (position absolute, max-height scrollable), chaque résultat affiche nom complet + date + lieu + badge source. Emit `@resultat-selectionne(rattachementId)` et `@recherche-publique-resultat(personne)`. Debounce 300ms pour recherche locale, 500ms pour publique. Props : `graphe` (pour recherche locale). Tailwind CSS v4 pur.
- [x] T006 [US1] Intégrer `ChampRecherche` dans `BarreOutils.vue` (`uafricas_frontend/app/components/arbre-genealogique/BarreOutils.vue`), ajouter le champ de recherche entre les boutons de mode et le bouton réinitialiser. Passer le graphe comme prop. Émettre `@recherche-selectionne(rattachementId)` vers la page parent. Sur mobile (max-sm), le champ s'affiche en pleine largeur sous les boutons de mode.
- [x] T007 [US1] Intégrer la recherche dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), quand un résultat de recherche est sélectionné (`@recherche-selectionne`), centrer la vue sur la personne correspondante (même logique que le clic nœud : setCenter + ouvrir panneau).

**Checkpoint**: Recherche locale fonctionnelle avec centrage, US1 testable

---

## Phase 4: User Story 2 : Recherche publique (Priority: P2)

**Goal**: Basculer le toggle sur "Tous les arbres" pour chercher dans la base publique

**Independent Test**: Toggle "Tous les arbres" + taper un nom → résultats d'autres arbres avec anonymisation

### Implementation for User Story 2

- [x] T008 [US2] Implémenter la recherche publique dans `ChampRecherche.vue` (`uafricas_frontend/app/components/arbre-genealogique/ChampRecherche.vue`), quand le toggle est sur "Tous les arbres", appeler `rechercherPublique(terme)` depuis le composable `useDecouvertes`. Afficher les résultats avec badge "Autre arbre" + "Membre #XXXX". Cliquer sur un résultat public affiche un message invitant à consulter la page Découvertes. Distinguer visuellement les résultats locaux (fond blanc) des résultats publics (fond vert léger).

**Checkpoint**: Recherche publique avec anonymisation, US2 testable

---

## Phase 5: User Story 3 : Chemin de parenté (Priority: P2)

**Goal**: Sélectionner deux personnes et voir le lien familial en langage naturel

**Independent Test**: Sélectionner grand-père et petit-fils → "X est le grand-père de Y"

### Implementation for User Story 3

- [x] T009 [P] [US3] Créer le composant `PanneauChemin.vue` dans `uafricas_frontend/app/components/arbre-genealogique/PanneauChemin.vue`, panneau latéral (remplace le panneau personne quand 2 personnes sont sélectionnées) affichant : noms des 2 personnes, description de la relation en langage naturel (ex : "Ibrahim est le grand-père de Aminata"), chemin visualisé comme une liste verticale de nœuds avec flèches, bouton "Fermer". Si aucun lien trouvé, afficher "Aucun lien de parenté trouvé". Bouton "Mettre en surbrillance" pour colorer le chemin dans l'arbre.
- [x] T010 [US3] Intégrer le mode "chemin de parenté" dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), ajouter un bouton "Chemin de parenté" dans la barre d'outils (ou dans le panneau personne). Quand actif, le premier clic sélectionne la personne source, le second clic sélectionne la cible. Appeler `calculerCheminParente()` et afficher `PanneauChemin`. Mettre en surbrillance les nœuds et liens du chemin (classe CSS spéciale, couleur `custom-chocolat`).

**Checkpoint**: Chemin de parenté avec terminologie française, US3 testable

---

## Phase 6: User Story 4 : Filtres (Priority: P3)

**Goal**: Filtrer l'arbre par lieu, génération et branche familiale

**Independent Test**: Filtre "Mali" → seules les personnes du Mali visibles

### Implementation for User Story 4

- [x] T011 [P] [US4] Créer le composant `PanneauFiltres.vue` dans `uafricas_frontend/app/components/arbre-genealogique/PanneauFiltres.vue`, panneau déroulant depuis la barre d'outils avec 3 filtres : champ texte "Lieu" (filtre géographique), slider/input "Générations ±N" (défaut: ±3), sélecteur "Branche" (liste des parents de la personne centrée, clic = filtre branche paternelle/maternelle). Bouton "Réinitialiser les filtres". Émet `@filtres-change(filtres: FiltreArbre[])`. Compteur de filtres actifs visible dans la barre d'outils.
- [x] T012 [US4] Intégrer les filtres dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), ajouter un bouton "Filtres" avec compteur dans BarreOutils. Quand les filtres changent, appliquer les fonctions `filtrerParLieu`, `filtrerParGeneration`, `filtrerParBranche` du composable. Combiner les résultats (intersection) pour obtenir les nœuds visibles. Mettre à jour les nodes/edges de vue-flow en conséquence. Ajouter un bouton dans BarreOutils pour toggle le panneau filtres.

**Checkpoint**: Filtres combinables fonctionnels, US4 testable

---

## Phase 7: Polish

- [x] T013 [P] Vérification Tailwind CSS v4 dans tous les nouveaux composants
- [x] T014 Exécuter le scénario de validation quickstart.md, 9 étapes

---

## Dependencies & Execution Order

```
Phase 1 (Setup) → Phase 2 (Foundational/Backend)
                        │
                        ▼
                   Phase 3 (US1) 🎯 MVP
                        │
                   ┌────┴────────┐
                   ▼              ▼
            Phase 4 (US2)   Phase 5 (US3) [parallélisable]
                   │              │
                   └────┬─────────┘
                        ▼
                   Phase 6 (US4)
                        │
                        ▼
                   Phase 7 (Polish)
```

### Parallel Opportunities

- Phase 3: T005 (composant) en parallèle avec le développement
- Phase 4 ∥ Phase 5: US2 et US3 indépendants
- Phase 7: T013 + T014 en parallèle

---

## Implementation Strategy

### MVP First (US1)

1. Phase 1 + 2 (T001-T004)
2. Phase 3 (T005-T007)
3. **STOP** : Recherche locale fonctionnelle

### Estimation

| Phase | Tâches | Priorité |
|-------|--------|----------|
| Setup | 1 | : |
| Foundational | 3 | : |
| US1 (P1) | 3 | MVP |
| US2 (P2) | 1 | Incrémental |
| US3 (P2) | 2 | Incrémental |
| US4 (P3) | 2 | Finition |
| Polish | 2 | Final |
| **Total** | **14** | : |
