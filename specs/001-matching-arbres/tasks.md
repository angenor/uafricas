# Tasks: Matching et Découverte de Parents

**Input**: Design documents from `/specs/001-matching-arbres/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api-matching.md

**Tests**: Non demandés : pas de tâches de test.

**Organization**: Feature full-stack. Tâches groupées par user story. Backend (Rust/SQL) + Frontend (Nuxt 4/Vue 3).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut tourner en parallèle (fichiers différents, pas de dépendance)
- **[Story]**: User story associée (US1, US2, US3, US4, US5)

## Path Conventions

- **Backend**: `uafricas_backend/`
- **Frontend**: `uafricas_frontend/app/`

---

## Phase 1: Setup (Schema SQL + Extensions + Types)

**Purpose**: Migration SQL, extension pg_trgm, colonnes normalisées, types partagés

- [x] T001 Créer le fichier de migration `uafricas_backend/doc/bd/schemas/24_matching.sql`, activer l'extension `pg_trgm`, ajouter les colonnes `nom_normalise VARCHAR(255)` et `prenoms_normalise VARCHAR(500)` à `arbre_genealogique.personnes`, créer les indexes GIN trigram sur `nom_normalise` et `prenoms_normalise` (WHERE deleted_at IS NULL), créer la table `arbre_genealogique.suggestions_correspondance` (id, rattachement_a_id FK, rattachement_b_id FK, score REAL, score_nom, score_prenoms, score_date, score_lieu, score_genre, statut VARCHAR(20) DEFAULT 'en_attente', confirmee_par_a BOOLEAN DEFAULT FALSE, confirmee_par_b BOOLEAN DEFAULT FALSE, detectee_le TIMESTAMPTZ DEFAULT NOW(), confirmee_le TIMESTAMPTZ, deleted_at TIMESTAMPTZ, UNIQUE(LEAST(rattachement_a_id,rattachement_b_id),GREATEST(rattachement_a_id,rattachement_b_id)), CHECK rattachement_a_id != rattachement_b_id), créer la table `arbre_genealogique.demandes_contact` (id, suggestion_id FK, demandeur_id FK iam.utilisateur, destinataire_id FK iam.utilisateur, statut VARCHAR(20) DEFAULT 'en_attente', created_at, traitee_le)
- [x] T002 Ajouter `\ir schemas/24_matching.sql` dans `uafricas_backend/doc/bd/schema.sql`
- [x] T003 [P] Créer les types TypeScript dans `uafricas_frontend/app/mocks/matching.ts`, interfaces `SuggestionCorrespondance` (id, maPersonne, personneMatchee, score, detailsScore, statut, membreIdAnonimise, detecteeLe, confirmeeLe), `DetailsScore` (nom, prenoms, date, lieu, genre), `DemandeContact` (id, suggestionId, statut, profilMembre), `ArbreDecouvert` (suggestionId, personneCommune, personnes, liens, membreIdAnonimise). Types : `StatutSuggestion = 'en_attente' | 'confirmee_de_mon_cote' | 'confirmee' | 'rejetee'`, `StatutDemande = 'en_attente' | 'acceptee' | 'refusee'`. Données mock : 3 suggestions exemple + helpers async.

---

## Phase 2: Foundational (Service matching + Modèles Rust)

**Purpose**: Algorithme de normalisation, scoring, matching, DOIT être terminé avant les user stories

**⚠️ CRITICAL**: Aucune tâche de user story ne peut commencer avant la fin de cette phase

- [x] T004 [P] Créer le service de matching `uafricas_backend/src/services/matching.rs`, implémenter la fonction `normaliser_nom(nom: &str) -> String` (lowercase, suppression diacritiques, collapse phonétiques africaines : ou→u, dy→di, ll→l, ss→s, h final muet supprimé, double consonnes simplifiées). Implémenter `calculer_score_date(annee_a: Option<i16>, annee_b: Option<i16>) -> f32` avec gaussienne sigma=5. Implémenter `executer_matching_rapide(pool, personne_id, arbre_id) -> Vec<Uuid>` pour correspondances exactes sur nom+prenoms normalisés dans d'autres arbres. Implémenter `executer_matching_profond(pool, personne_id, arbre_id) -> Result<()>` : requête pg_trgm avec score composite (nom 35%, prenoms 20%, date 15%, lieu 20%, genre 10%), seuil 55%, INSERT dans suggestions_correspondance en excluant les paires existantes.
- [x] T005 [P] Créer les modèles Rust `uafricas_backend/src/models/matching.rs`, structs `SuggestionCorrespondance` (FromRow), `DemandeContact` (FromRow), DTOs réponse : `SuggestionResponse`, `DetailsScoreResponse`, `PersonneResumeResponse`, `ArbreDecouvertResponse`, `DemandeContactResponse`. Constante `SUGGESTION_COLONNES`.
- [x] T006 Ajouter `pub mod matching;` dans `uafricas_backend/src/services/mod.rs` et `uafricas_backend/src/models/mod.rs`
- [x] T007 Modifier le handler `creer_personne` dans `uafricas_backend/src/handlers/arbre_genealogique.rs`, après l'insertion de la personne, appeler `normaliser_nom()` pour stocker `nom_normalise` et `prenoms_normalise`, puis `tokio::spawn` pour lancer `executer_matching_profond()` en tâche de fond (fire-and-forget, log erreur sans affecter la réponse)

**Checkpoint**: Algorithme de matching fonctionnel, scoring opérationnel, normalisation en place

---

## Phase 3: User Story 1 : Recevoir des suggestions (Priority: P1) 🎯 MVP

**Goal**: L'utilisateur voit ses suggestions de correspondance sur la page Découvertes avec score et critères

**Independent Test**: Créer 2 comptes avec personnes similaires → vérifier apparition des suggestions

### Implementation for User Story 1

- [x] T008 [P] [US1] Implémenter le handler `lister_decouvertes` dans `uafricas_backend/src/handlers/matching.rs`, GET /api/arbre/decouvertes, retourne les suggestions groupées par statut (en_attente, en_cours de confirmation, confirmees) pour l'arbre de l'utilisateur connecté. Jointure sur rattachements + personnes pour obtenir les données affichables. Pagination par section. Anonymisation du membre (4 derniers caractères UUID de l'utilisateur propriétaire de l'autre arbre).
- [x] T009 [US1] Ajouter la route `/decouvertes` (GET) dans le scope `/arbre` de `uafricas_backend/src/routes.rs` + `pub mod matching` dans `uafricas_backend/src/handlers/mod.rs`
- [x] T010 [P] [US1] Créer le composable `useDecouvertes` dans `uafricas_frontend/app/composables/useDecouvertes.ts`, méthode `listerDecouvertes(section?)` appelant GET /api/arbre/decouvertes avec header JWT
- [x] T011 [P] [US1] Créer le composant `CarteSuggestion.vue` dans `uafricas_frontend/app/components/arbre-genealogique/CarteSuggestion.vue`, affiche : nom personne locale, nom personne matchée, score de confiance (barre de progression colorée), sous-scores détaillés (nom, prénoms, date, lieu, genre), identifiant anonymisé "Membre #XXXX", date de détection. Boutons Confirmer/Rejeter (émettent vers parent). Tailwind CSS v4 pur.
- [x] T012 [P] [US1] Créer le composant `SectionDecouvertes.vue` dans `uafricas_frontend/app/components/arbre-genealogique/SectionDecouvertes.vue`, section paginée avec titre, compteur, liste de `CarteSuggestion`. Props : suggestions[], titre, emptyMessage. Pagination (précédent/suivant).
- [x] T013 [US1] Créer la page `decouvertes.vue` dans `uafricas_frontend/app/pages/arbre-genealogique/decouvertes.vue`, page avec hero section (titre "Mes Découvertes", sous-titre, icône), 3 sections via `SectionDecouvertes` (en_attente, en_cours, confirmees), état vide encourageant à enrichir l'arbre, état de chargement. Import explicite des composants. Layout default. `mt-28` pour éviter le chevauchement navbar.

**Checkpoint**: Page Découvertes affiche les suggestions avec score, US1 testable

---

## Phase 4: User Story 2 : Confirmer / Rejeter (Priority: P1)

**Goal**: L'utilisateur peut confirmer ou rejeter les suggestions, avec gestion du flux mutuel

**Independent Test**: Confirmer des 2 côtés → statut "confirmée"

### Implementation for User Story 2

- [x] T014 [P] [US2] Implémenter les handlers `confirmer_suggestion` et `rejeter_suggestion` dans `uafricas_backend/src/handlers/matching.rs`, POST /api/arbre/decouvertes/{id}/confirmer : vérifie que l'utilisateur est propriétaire d'un des deux rattachements, met à jour `confirmee_par_a` ou `confirmee_par_b`, si les deux sont true → statut='confirmee' + `confirmee_le=NOW()`. POST .../rejeter : met statut='rejetee_a' ou 'rejetee_b', annule toute confirmation de l'autre côté. Audit log pour les deux actions.
- [x] T015 [US2] Ajouter les routes `/decouvertes/{id}/confirmer` (POST) et `/decouvertes/{id}/rejeter` (POST) dans `uafricas_backend/src/routes.rs`
- [x] T016 [US2] Ajouter les méthodes `confirmerSuggestion(id)` et `rejeterSuggestion(id)` dans `uafricas_frontend/app/composables/useDecouvertes.ts`
- [x] T017 [US2] Intégrer les actions confirmer/rejeter dans `uafricas_frontend/app/pages/arbre-genealogique/decouvertes.vue`, quand CarteSuggestion émet @confirmer ou @rejeter : appeler l'API, mettre à jour la liste, afficher toast de succès/erreur, déplacer la suggestion dans la bonne section

**Checkpoint**: Flux confirmer/rejeter fonctionnel avec mise à jour mutuelle, US2 testable

---

## Phase 5: User Story 3 : Branches découvertes (Priority: P1)

**Goal**: Après confirmation mutuelle, voir l'arbre complet de l'autre utilisateur dans sa propre visualisation

**Independent Test**: Après confirmation mutuelle → branches de l'autre arbre visibles dans la visualisation

### Implementation for User Story 3

- [x] T018 [P] [US3] Implémenter le handler `obtenir_branches_decouvertes` dans `uafricas_backend/src/handlers/matching.rs`, GET /api/arbre/decouvertes/{id}/branches : vérifie confirmation mutuelle, retourne l'arbre complet de l'autre utilisateur (toutes personnes + tous liens via le même pattern que `obtenir_arbre_complet`), avec identifiant anonymisé. Erreur 403 si non confirmée mutuellement.
- [x] T019 [US3] Ajouter la route `/decouvertes/{id}/branches` (GET) dans `uafricas_backend/src/routes.rs`
- [x] T020 [US3] Ajouter la méthode `obtenirBranchesDecouvertes(suggestionId)` dans `uafricas_frontend/app/composables/useDecouvertes.ts`
- [x] T021 [US3] Modifier `NoeudPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/NoeudPersonne.vue`), ajouter une prop `estDecouvert: boolean` (défaut false). Si true, appliquer un style visuel distinct : bordure pointillée, opacité légèrement réduite (opacity-80), badge petit "D" en bas à gauche du nœud (couleur custom-green). Le nœud reste cliquable mais le panneau n'affiche pas les actions d'édition.
- [x] T022 [US3] Modifier `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), après chargement de l'arbre, appeler un nouvel endpoint pour récupérer les correspondances confirmées de l'utilisateur, puis pour chaque correspondance, charger les branches découvertes et les fusionner dans le graphe avec le flag `estDecouvert=true`. Les nœuds découverts sont en lecture seule (pas de boutons d'action dans le panneau). Ajouter un lien "Mes Découvertes" dans la barre d'outils.

**Checkpoint**: Branches d'autres arbres visibles dans la visualisation, US3 testable

---

## Phase 6: User Story 4 : Page historique (Priority: P2)

**Goal**: Organisation en 3 sections avec compteurs, pagination, lien "Voir dans l'arbre"

**Independent Test**: Mélange de suggestions dans les 3 états → sections correctes

### Implementation for User Story 4

- [x] T023 [US4] Enrichir la page `decouvertes.vue` (`uafricas_frontend/app/pages/arbre-genealogique/decouvertes.vue`), ajouter dans la section "Correspondances confirmées" : date de confirmation, nombre de nouvelles personnes découvertes (obtenu via l'endpoint branches), et un NuxtLink « Voir dans l'arbre » qui redirige vers `/arbre-genealogique/visualisation?centre={rattachement_id}` pour centrer la vue sur la personne commune.
- [x] T024 [US4] Modifier `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`), supporter un query param `?centre={rattachement_id}` pour centrer automatiquement la vue sur une personne spécifique au chargement (utilisé par le lien "Voir dans l'arbre" de la page Découvertes).

**Checkpoint**: Page Découvertes complète avec navigation vers l'arbre, US4 testable

---

## Phase 7: User Story 5 : Matching automatique + Demande de contact (Priority: P2)

**Goal**: Matching déclenché automatiquement + demande de contact après confirmation

### Implementation for User Story 5

- [x] T025 [P] [US5] Implémenter les handlers de demande de contact dans `uafricas_backend/src/handlers/matching.rs`, POST /api/arbre/decouvertes/{suggestion_id}/demande-contact (crée une demande), POST /api/arbre/demandes-contact/{id}/accepter (révèle les profils mutuellement), POST /api/arbre/demandes-contact/{id}/refuser. Vérifier que la correspondance est confirmée avant de permettre la demande. Audit log.
- [x] T026 [US5] Ajouter les 3 routes de demande de contact dans `uafricas_backend/src/routes.rs`
- [x] T027 [US5] Ajouter les méthodes `demanderContact(suggestionId)`, `accepterDemande(id)`, `refuserDemande(id)` dans `uafricas_frontend/app/composables/useDecouvertes.ts`
- [x] T028 [US5] Ajouter la section "Demande de contact" dans la page `decouvertes.vue` pour les correspondances confirmées, bouton "Demander le contact", affichage du statut de la demande (en_attente/acceptée/refusée), affichage du profil (nom, prénom, email) si la demande est acceptée
- [x] T029 [US5] Ajouter un badge de notification "Nouvelles suggestions" dans la navbar principale, compter les suggestions récentes (< 7 jours, statut en_attente) et afficher un badge numérique à côté du lien "Découvertes" dans le menu de navigation

**Checkpoint**: Demande de contact + notification suggestions, US5 testable

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Backfill, robustesse, finitions

- [x] T030 [P] Écrire un script SQL de backfill pour normaliser les noms/prénoms de toutes les personnes existantes dans `uafricas_backend/doc/bd/schemas/24_matching.sql`, UPDATE arbre_genealogique.personnes SET nom_normalise = LOWER(nom), prenoms_normalise = LOWER(prenoms) WHERE nom_normalise IS NULL (version simplifiée, la normalisation phonétique complète sera appliquée par le backend au prochain UPDATE)
- [x] T031 [P] Gérer la suppression cascade dans le handler `supprimer_personne` de `uafricas_backend/src/handlers/arbre_genealogique.rs`, avant de soft-delete le rattachement, soft-delete aussi les suggestions_correspondance liées (et notifier l'autre utilisateur si possible via statut)
- [x] T032 Vérification de cohérence Tailwind CSS v4, s'assurer qu'aucune classe daisyUI n'est utilisée dans la page Découvertes, CarteSuggestion, SectionDecouvertes
- [x] T033 Exécuter le scénario de validation quickstart.md, parcourir les 10 étapes avec 2 comptes test

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Aucune dépendance, démarrage immédiat
- **Foundational (Phase 2)**: Dépend de Phase 1, BLOQUE toutes les user stories
- **US1 (Phase 3)**: Dépend de Phase 2 : MVP
- **US2 (Phase 4)**: Dépend de Phase 3 (US1 fournit la page avec les suggestions)
- **US3 (Phase 5)**: Dépend de Phase 4 (US2 fournit le flux de confirmation mutuelle)
- **US4 (Phase 6)**: Dépend de Phase 5 (US3 fournit les branches découvertes)
- **US5 (Phase 7)**: Dépend de Phase 4 (US2 fournit les confirmations pour la demande de contact)
- **Polish (Phase 8)**: Dépend de toutes les phases

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
            Phase 5 (US3) Phase 7 (US5) [parallélisable]
                   │         │
                   ▼         │
            Phase 6 (US4)    │
                   │         │
                   └────┬────┘
                        ▼
                   Phase 8 (Polish)
```

### Parallel Opportunities

- **Phase 1** : T001 + T003 en parallèle (SQL + types TS dans des fichiers différents)
- **Phase 2** : T004 + T005 en parallèle (service + modèles dans des fichiers différents)
- **Phase 3** : T008 + T010 + T011 + T012 en parallèle (handler + composable + 2 composants)
- **Phase 5** : US3 ∥ US5 (Phase 5 et Phase 7 parallélisables)
- **Phase 8** : T030 + T031 en parallèle

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1: Setup (T001–T003)
2. Phase 2: Foundational (T004–T007)
3. Phase 3: US1 (T008–T013)
4. **STOP et VALIDER** : Suggestions visibles avec score

### Incremental Delivery

1. Setup + Foundational → Matching opérationnel
2. US1 → Page Découvertes avec suggestions → **MVP**
3. US2 → Confirmer/rejeter → Flux complet
4. US3 → Branches découvertes dans l'arbre → La promesse de valeur
5. US4 → Historique structuré
6. US5 → Demande de contact + notifications
7. Polish → Backfill + cascade + validation

### Estimation de charge

| Phase | Tâches | Fichiers touchés | Priorité |
|-------|--------|-----------------|----------|
| Setup | 3 | 3 | : |
| Foundational | 4 | 5 | : |
| US1 (P1) | 6 | 6 | MVP |
| US2 (P1) | 4 | 3 | MVP+ |
| US3 (P1) | 5 | 4 | Core |
| US4 (P2) | 2 | 2 | Incrémental |
| US5 (P2) | 5 | 4 | Incrémental |
| Polish | 4 | 3 | Final |
| **Total** | **33** | **~17 uniques** |, |

---

## Notes

- Feature full-stack : ~10 fichiers backend + ~7 fichiers frontend
- Extension PostgreSQL `pg_trgm` requise (à activer via le script SQL)
- La normalisation phonétique africaine est la pièce maîtresse de la qualité du matching
- Le `tokio::spawn` pour le matching profond est fire-and-forget, ne bloque jamais la création de personne
- Anonymat strict : "Membre #XXXX" jusqu'à acceptation de la demande de contact
- Commit après chaque tâche ou groupe logique
