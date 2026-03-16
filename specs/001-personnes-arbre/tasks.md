# Tasks: Modèle de données des personnes et liens familiaux

**Input**: Design documents from `specs/001-personnes-arbre/`
**Prerequisites**: plan.md ✅ spec.md ✅ research.md ✅ data-model.md ✅ contracts/ ✅ quickstart.md ✅

**Tests**: Aucun test automatisé — validation manuelle via Adminer, curl et navigateur (pas de CI/CD configuré).

**Organisation**: Tâches groupées par user story pour permettre une implémentation et une validation indépendantes.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut s'exécuter en parallèle (fichiers différents, aucune dépendance incomplète)
- **[Story]**: User story concernée (US1–US4)
- Tous les chemins sont relatifs à la racine du monorepo

---

## Phase 1 : Setup (Infrastructure partagée)

**Objectif** : Créer les fichiers SQL et enregistrer le nouveau module dans le schema orchestrateur.

- [x] T001 Créer le fichier `uafricas_backend/doc/bd/schemas/23_arbre_genealogique.sql` avec le contenu complet du schéma (4 tables + contraintes + index) tel que défini dans `specs/001-personnes-arbre/data-model.md`
- [x] T002 Ajouter `\ir schemas/23_arbre_genealogique.sql` dans `uafricas_backend/doc/bd/schema.sql` (après la dernière ligne `\ir`)

---

## Phase 2 : Fondation (Prérequis bloquants)

**Objectif** : Infrastructure technique complète avant tout travail sur les user stories.

**⚠️ CRITIQUE** : Aucune user story ne peut démarrer avant la fin de cette phase.

- [x] T003 Réinitialiser la base de données (`docker compose down -v && docker compose up -d`) et vérifier dans Adminer (`http://localhost:8088`) que le schema `arbre_genealogique` contient bien les 4 tables : `personnes`, `arbres`, `rattachements`, `liens_familiaux`
- [x] T004 [P] Créer `uafricas_backend/src/models/arbre_genealogique.rs` avec toutes les structs `FromRow` (`Personne`, `Arbre`, `Rattachement`, `LienFamilial`), les DTOs de réponse (`PersonneResponse`, `DatePartielle`, `PersonneDetailResponse`, `PersonneListeResponse`, `LienFamilialResponse`), les DTOs de requête (`CreerPersonneDto`, `ModifierPersonneDto`, `DatePartielleDto`, `CreerLienDto`, `PersonneQueryParams`) et les constantes `PERSONNE_COLONNES`, tels que définis dans `specs/001-personnes-arbre/data-model.md`
- [x] T005 [P] Déclarer le module dans `uafricas_backend/src/models/mod.rs` : ajouter `pub mod arbre_genealogique;`
- [x] T006 Créer `uafricas_backend/src/handlers/arbre_genealogique.rs` avec le squelette du fichier (imports `use` pour actix-web, sqlx, uuid, chrono, crate::errors::ApiErreur, crate::models::arbre_genealogique::*, crate::services::audit, crate::jwt::Claims) et les 8 fonctions publiques vides retournant `todo!()`
- [x] T007 [P] Déclarer le module dans `uafricas_backend/src/handlers/mod.rs` : ajouter `pub mod arbre_genealogique;`
- [x] T008 Ajouter le scope `/arbre` dans `uafricas_backend/src/routes.rs` avec les 8 routes déclarées (GET + POST `/personnes`, GET + PUT + DELETE `/personnes/{id}`, POST `/personnes/{id}/photo`, POST + DELETE `/liens/{id}`) pointant vers les fonctions du handler
- [x] T009 [P] Créer `uafricas_frontend/app/mocks/arbre-genealogique.ts` avec toutes les interfaces TypeScript (`Personne`, `PersonneDetail`, `PersonneListe`, `LienResume`, `LienFamilial`, `DatePartielle`, `CreerPersonneForm`, `ModifierPersonneForm`, `CreerLienForm`, `PersonneQueryParams`), les types (`Genre`, `TypeLien`), les données mock (`personnesMock`), les helpers (`formaterDate`, `getPersonneMockById`, `listerPersonnesMock`, `formeVide`) tels que définis dans `specs/001-personnes-arbre/data-model.md`
- [x] T010 [P] Créer `uafricas_frontend/app/composables/useArbreGenealogique.ts` avec les 7 fonctions (`listerPersonnes`, `obtenirPersonne`, `creerPersonne`, `modifierPersonne`, `supprimerPersonne`, `creerLien`, `supprimerLien`) utilisant `$fetch` + `useRuntimeConfig` + `useUserStore`, tel que défini dans `specs/001-personnes-arbre/quickstart.md`

Vérifier la compilation backend : `cd uafricas_backend && cargo check`

**Checkpoint** : Foundation prête — les user stories peuvent démarrer

---

## Phase 3 : User Story 1 — Créer une fiche personne (Priority: P1) 🎯 MVP

**Objectif** : Un utilisateur authentifié peut créer, consulter, modifier et supprimer une personne dans son arbre.

**Test indépendant** : Appeler `POST /api/arbre/personnes` avec `{"nom":"Diallo"}`, vérifier `201`, appeler `GET /api/arbre/personnes/:id`, modifier le lieu de naissance via `PUT`, supprimer via `DELETE` et vérifier `200`. Aucun lien familial nécessaire.

### Implémentation Backend US1

- [x] T011 [US1] Implémenter `creer_personne` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : créer l'arbre avec `INSERT ... ON CONFLICT DO NOTHING`, valider `nom` non vide (400 si absent), valider cohérence dates (422 si deces_annee < naissance_annee), insérer dans `arbre_genealogique.personnes` puis `arbre_genealogique.rattachements`, appeler `audit::log_action` avec table `"arbre_genealogique.personnes"`, retourner 201 avec `PersonneResponse`
- [x] T012 [US1] Implémenter `obtenir_personne` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : vérifier que le rattachement actif existe pour l'arbre de l'utilisateur (403 sinon), retourner `PersonneDetailResponse` avec `parents`, `enfants` et `conjoints` en tableaux vides pour l'instant (US2 les remplira)
- [x] T013 [US1] Implémenter `modifier_personne` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : vérifier appartenance à l'arbre, valider `nom` non vide si fourni, valider cohérence des dates avec les valeurs fusionnées (existantes + nouvelles), UPDATE `personnes` avec `updated_at = NOW()`, appeler `audit::log_action` avec état avant/après JSON, retourner 200 avec `PersonneDetailResponse`
- [x] T014 [US1] Implémenter `supprimer_personne` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : vérifier appartenance à l'arbre, ouvrir une transaction sqlx, soft-delete le rattachement, soft-delete les `liens_familiaux` impliquant ce rattachement, compter les rattachements actifs restants sur la personne, si 0 soft-delete la personne, committer, appeler `audit::log_action`, retourner 200
- [x] T015 [US1] Implémenter `uploader_photo` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : parser multipart via `actix-multipart`, valider le type MIME (image/jpeg, image/png, image/webp) et la taille (max 5 Mo), sanitiser le nom de fichier, sauvegarder dans `./uploads/personnes/`, mettre à jour `personnes.photo_url`, retourner 200 avec `{ "photo_url": "/uploads/personnes/..." }`

### Implémentation Frontend US1

- [x] T016 [P] [US1] Créer `uafricas_frontend/app/components/arbre-genealogique/PersonneForm.vue` : formulaire Tailwind v4 pur (sans daisyUI) avec les champs nom (requis), prénoms, genre (select), date de naissance partielle (3 inputs séparés : année, mois, jour), lieu de naissance, date de décès partielle, lieu de décès, upload photo ; émettre `@submit` avec `CreerPersonneForm | ModifierPersonneForm` ; afficher les erreurs de validation locales (nom vide) et les erreurs API
- [x] T017 [US1] Créer `uafricas_frontend/app/pages/arbre-genealogique/[id].vue` : charger la personne via `useArbreGenealogique().obtenirPersonne(id)`, afficher la fiche complète (nom, prénoms, genre, dates formatées via `formaterDate`, lieu naissance/décès, photo), sections "Parents", "Enfants", "Conjoints" vides avec message "Aucun lien" (remplies en US2), boutons "Modifier" (ouvre `PersonneForm` en mode édition inline ou modal) et "Supprimer" (confirmation + redirection vers index après suppression), gestion erreur 404 ; Tailwind v4 pur, pas de classes daisyUI

**Checkpoint US1** : `POST` → `GET /:id` → `PUT /:id` → `DELETE /:id` fonctionnels. Fiche affichée en navigateur avec formulaire édition opérationnel.

---

## Phase 4 : User Story 2 — Créer des liens familiaux (Priority: P2)

**Objectif** : Un utilisateur peut relier deux personnes de son arbre par un lien parent-enfant (père/mère/parent) ou conjoint, et supprimer ce lien.

**Test indépendant** : Créer deux personnes A et B, `POST /api/arbre/liens` avec `type_lien:"pere"` (A père de B), vérifier `201`. `GET /api/arbre/personnes/:B.id` → A dans `parents`. Supprimer le lien via `DELETE /api/arbre/liens/:id`, vérifier que `parents` est vide.

### Implémentation Backend US2

- [x] T018 [US2] Implémenter `creer_lien` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : vérifier que les deux rattachements appartiennent à l'arbre de l'utilisateur (403 sinon), vérifier `source_id ≠ cible_id` (400), vérifier absence de doublon avec SELECT (409 si existe), pour `type_lien` dans `['pere','mere','parent']` exécuter la requête CTE récursive de détection de cycle (voir `specs/001-personnes-arbre/research.md` Décision 3) et retourner 422 si cycle détecté, pour `type_lien='conjoint'` appliquer la convention `min/max UUID` (source < cible), insérer dans `liens_familiaux`, appeler `audit::log_action`, retourner 201 avec `LienFamilialResponse`
- [x] T019 [US2] Implémenter `supprimer_lien` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : vérifier que le lien appartient à l'arbre de l'utilisateur (403 sinon), soft-delete, appeler `audit::log_action`, retourner 200
- [x] T020 [US2] Mettre à jour `obtenir_personne` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : remplacer les tableaux vides par de vraies queries SQL — récupérer les parents (liens WHERE cible_id = rattachement ET type_lien IN pere/mere/parent), les enfants (liens WHERE source_id = rattachement ET type_lien IN pere/mere/parent), les conjoints (liens WHERE source_id = rattachement OU cible_id = rattachement ET type_lien = conjoint) — joindre avec `personnes` pour retourner `LienResumeResponse` avec la fiche complète

### Implémentation Frontend US2

- [x] T021 [P] [US2] Créer `uafricas_frontend/app/components/arbre-genealogique/LienFamilialForm.vue` : formulaire Tailwind v4 pur pour créer un lien — sélection de la personne cible (dropdown depuis la liste de l'arbre), sélection du type de lien (select : père / mère / parent / conjoint), bouton soumettre, affichage des erreurs API (doublon 409, cycle 422) ; émettre `@submit` avec `CreerLienForm`
- [x] T022 [US2] Mettre à jour `uafricas_frontend/app/pages/arbre-genealogique/[id].vue` : brancher `useArbreGenealogique().creerLien()` et `supprimerLien()`, afficher les sections Parents / Enfants / Conjoints avec les fiches des personnes liées (nom + dates) + bouton "Supprimer le lien", intégrer `LienFamilialForm.vue` dans un bloc "Ajouter un lien", rafraîchir la fiche après chaque opération

**Checkpoint US2** : Deux personnes reliées, lien visible dans GET detail des deux côtés, suppression fonctionnelle, erreur 422 sur tentative de cycle.

---

## Phase 5 : User Story 3 — Visualiser son arbre (Priority: P3)

**Objectif** : Un utilisateur consulte la liste paginée de son arbre et navigue vers la fiche de chaque personne.

**Test indépendant** : `GET /api/arbre/personnes?page=1&par_page=12` retourne les personnes de l'utilisateur (et uniquement les siennes). Avec 0 personne → `total:0`. Avec `recherche=Diallo` → seules les personnes Diallo.

### Implémentation Backend US3

- [x] T023 [US3] Implémenter `lister_personnes` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` : résoudre l'arbre de l'utilisateur (créer via `ON CONFLICT DO NOTHING` si absent), joindre `rattachements` + `personnes`, appliquer le filtre ILIKE sur `nom || prenoms` si `recherche` fourni, paginer avec `LIMIT` + `OFFSET`, retourner `PersonneListeResponse` avec total, page, par_page, total_pages

### Implémentation Frontend US3

- [x] T024 [P] [US3] Créer `uafricas_frontend/app/components/arbre-genealogique/PersonneCard.vue` : carte Tailwind v4 pur affichant photo (ou initiales si absente), nom complet, dates formatées via `formaterDate`, lieu de naissance ; émettre `@click` pour naviguer vers la fiche, slot optionnel pour actions supplémentaires
- [x] T025 [US3] Créer `uafricas_frontend/app/pages/arbre-genealogique/index.vue` : appel `useArbreGenealogique().listerPersonnes()` au mount, grille `PersonneCard.vue` avec pagination (boutons Précédent / Suivant + info "page X / Y"), champ de recherche avec debounce, état vide explicite ("Votre arbre est vide — ajoutez votre première personne" + bouton "Ajouter"), état chargement (skeleton ou spinner), bouton "Ajouter une personne" ouvrant `PersonneForm.vue` en modal ou panneau latéral ; Tailwind v4 pur

**Checkpoint US3** : Navigation complète — liste → fiche → retour liste, recherche, pagination, état vide.

---

## Phase 6 : User Story 4 — Architecture fondation matching (Priority: P4)

**Objectif** : Valider que la séparation Personne réelle / Rattachement supporte le futur matching inter-arbres sans modification de schéma.

**Test indépendant** : Via Adminer, insérer manuellement un second rattachement pour la même `personne_id` dans un arbre différent (utilisateur B). Vérifier que la contrainte `UNIQUE(arbre_id, personne_id)` est bien présente. Exécuter la requête de recherche de personnes partagées et vérifier qu'elle retourne la personne.

- [x] T026 [US4] Enrichir `uafricas_backend/doc/bd/schemas/23_arbre_genealogique.sql` avec un bloc de commentaires documentant l'architecture de matching futur : expliquer la séparation Personne réelle / Rattachement, fournir en commentaire SQL la requête CTE type pour trouver les personnes partagées entre deux arbres (`WITH personnes_communes AS (SELECT personne_id FROM rattachements WHERE arbre_id IN ($arbre_a, $arbre_b) AND deleted_at IS NULL GROUP BY personne_id HAVING COUNT(DISTINCT arbre_id) = 2)`)
- [x] T027 [P] [US4] Ajouter dans `specs/001-personnes-arbre/research.md` une nouvelle décision (Décision 8) documentant la requête SQL pour identifier les personnes partagées entre deux arbres, avec exemple concret et explication de pourquoi aucune migration ne sera nécessaire pour le futur feature de matching
- [x] T028 [P] [US4] Valider dans Adminer les contraintes clés : vérifier `UNIQUE` sur `arbres.utilisateur_id` (1 arbre / utilisateur), `UNIQUE` sur `rattachements(arbre_id, personne_id)` (1 rattachement / personne / arbre), `CHECK` de cohérence des dates, `CHECK` source ≠ cible dans les liens — documenter le résultat de validation dans un commentaire de commit

**Checkpoint US4** : Architecture validée — la fondation du matching est en place et documentée.

---

## Phase 7 : Polish & Préoccupations transversales

**Objectif** : Conformité audit, mise à jour documentation, validation end-to-end.

- [x] T029 [P] Auditer `uafricas_backend/src/handlers/arbre_genealogique.rs` : vérifier que chaque fonction de mutation (`creer_personne`, `modifier_personne`, `supprimer_personne`, `uploader_photo`, `creer_lien`, `supprimer_lien`) appelle bien `audit::log_action` avec les bons paramètres (action en français, table, ID, before/after JSON si applicable, IP, user-agent)
- [x] T030 [P] Mettre à jour `CLAUDE.md` : ajouter dans "Active Technologies" l'entrée pour la feature `001-personnes-arbre` (schema `arbre_genealogique`, handlers, composable `useArbreGenealogique`) et dans "Recent Changes" un résumé de ce qui a été ajouté
- [ ] T031 Exécuter la validation complète du `specs/001-personnes-arbre/quickstart.md` de bout en bout : réinitialiser la DB, démarrer le backend, créer 3 personnes, créer 2 liens, consulter la liste, naviguer dans les fiches, supprimer un lien, supprimer une personne — noter tout écart par rapport au comportement attendu

---

## Dépendances & Ordre d'exécution

### Dépendances entre phases

- **Phase 1 (Setup)** : Aucune dépendance — peut démarrer immédiatement
- **Phase 2 (Fondation)** : Dépend de Phase 1 — **bloque toutes les user stories**
- **Phase 3 (US1)** : Dépend de Phase 2 — aucune dépendance sur US2/US3/US4
- **Phase 4 (US2)** : Dépend de Phase 2 — nécessite US1 complète pour les tests (personnes existantes)
- **Phase 5 (US3)** : Dépend de Phase 2 — `lister_personnes` handler indépendant ; frontend nécessite US1 pour navigation
- **Phase 6 (US4)** : Dépend de Phase 2 — validation architecturale indépendante
- **Phase 7 (Polish)** : Dépend de toutes les phases précédentes

### Dépendances entre user stories

- **US1 (P1)** : Démarre après Phase 2 — aucune dépendance sur d'autres stories
- **US2 (P2)** : Peut démarrer après Phase 2 ; T020 (mise à jour `obtenir_personne`) requiert T012 (US1) complète
- **US3 (P3)** : Backend (T023) indépendant ; frontend (T025) bénéficie de PersonneCard.vue (T024) et de la fiche [id].vue (US1)
- **US4 (P4)** : Entièrement indépendante après Phase 2

### Au sein de chaque user story

- Backend avant frontend pour chaque story
- Handler avant intégration composable
- Commit après chaque checkpoint

### Opportunités de parallélisme

**Phase 2 (Foundation)** :
- T004 (models Rust) ‖ T005 (mod.rs) ‖ T009 (mock TS) ‖ T010 (composable)
- T006 (handler squelette) après T004 ; T007 (handlers/mod.rs) après T006 en parallèle avec T008 (routes)

**Phase 3 (US1)** :
- T016 (PersonneForm.vue) en parallèle avec T011–T015 (backend)

**Phase 4 (US2)** :
- T021 (LienFamilialForm.vue) en parallèle avec T018–T020 (backend)

**Phase 5 (US3)** :
- T024 (PersonneCard.vue) en parallèle avec T023 (lister_personnes backend)

**Phase 6 (US4)** :
- T026, T027, T028 peuvent s'exécuter en parallèle

**Phase 7 (Polish)** :
- T029 (audit check) ‖ T030 (CLAUDE.md update)

---

## Exemple d'exécution parallèle — Phase 2

```text
Sous-agent A : T004 — models/arbre_genealogique.rs (toutes les structs Rust)
Sous-agent B : T009 — app/mocks/arbre-genealogique.ts (interfaces TS + mock)
Sous-agent C : T010 — app/composables/useArbreGenealogique.ts (composable)

→ Puis en séquence :
T005 (pub mod arbre_genealogique dans models/mod.rs)
T006 (handlers/arbre_genealogique.rs squelette)
T007 (pub mod dans handlers/mod.rs) ‖ T008 (routes.rs scope /arbre)
```

## Exemple d'exécution parallèle — Phase 3 (US1)

```text
Sous-agent A : T011 → T012 → T013 → T014 → T015 (handlers backend séquentiels)
Sous-agent B : T016 (PersonneForm.vue — fichier différent, aucune dépendance backend)

→ Puis T017 ([id].vue) après T016 + T012
```

---

## Stratégie d'implémentation

### MVP (User Story 1 uniquement)

1. Compléter Phase 1 : Setup
2. Compléter Phase 2 : Fondation (CRITIQUE)
3. Compléter Phase 3 : US1 — CRUD une personne
4. **STOP et VALIDER** : créer/éditer/supprimer une personne en navigateur
5. Démontrable à ce stade

### Livraison incrémentale

1. Setup + Fondation → Backend compilable, DB initialisée
2. US1 → CRUD personnes fonctionnel → **MVP démontrable**
3. US2 → Liens familiaux → Arbre avec relations
4. US3 → Liste paginée → Navigation complète
5. US4 → Architecture validée → Prêt pour feature matching
6. Polish → Audit + docs → Prêt pour merge

### Stratégie parallèle (2 développeurs)

```
Développeur A (backend Rust) :
  Phase 2 T004 → Phase 3 T011-T015 → Phase 4 T018-T020 → Phase 5 T023

Développeur B (frontend Nuxt) :
  Phase 2 T009-T010 → Phase 3 T016-T017 → Phase 4 T021-T022 → Phase 5 T024-T025
```

---

## Notes

- `[P]` = fichiers différents, aucune dépendance en attente
- `[US1–US4]` = traçabilité vers la user story de `spec.md`
- Chaque user story est testable indépendamment sans les autres
- Ne pas utiliser daisyUI dans les composants frontend de cette feature (pages publiques — cf. Principe VI constitution)
- Toutes les mutations backend doivent appeler `audit::log_action` (Principe VII constitution)
- Ordre obligatoire : SQL → structs Rust → handlers → routes → mock TS → composable → pages
