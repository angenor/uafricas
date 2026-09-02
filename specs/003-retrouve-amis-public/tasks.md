# Tasks: Avis de Recherche Publics par Défaut

**Input**: Design documents from `/specs/003-retrouve-amis-public/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api-publique.md

**Tests**: Non demandés : pas de CI/CD configuré. Vérification manuelle uniquement.

**Organization**: Tasks groupées par user story pour permettre l'implémentation et le test indépendants de chaque story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut s'exécuter en parallèle (fichiers différents, pas de dépendances)
- **[Story]**: US1, US2, US3, US4 (correspondant aux user stories du spec.md)

## Path Conventions

- **Backend**: `uafricas_backend/`
- **Frontend**: `uafricas_frontend/`

---

## Phase 1: Setup (Schéma SQL)

**Purpose**: Modifications du schéma PostgreSQL, fondation pour tout le reste

- [x] T001 Ajouter les 2 nouveaux enums (`genre_personne`, `type_relation_recherche`) dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T002 Ajouter les 14 nouvelles colonnes à la table `avis_recherche` dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T003 Modifier `est_public DEFAULT TRUE` et ajouter `UPDATE SET est_public = TRUE` pour les avis existants dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T004 Ajouter les contraintes CHECK (`chk_lieu_ou_jamais`, `chk_coordonnees_requises`) dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T005 Ajouter les 4 index (type_relation, localite_trgm, ecole_rencontre_trgm, ville_rencontre_trgm) dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T006 Mettre à jour le calcul du `search_vector` (trigger/fonction) pour inclure les nouveaux champs dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T007 Recréer la BDD de dev (`docker compose down -v && docker compose up -d`) et vérifier que le schéma s'applique sans erreur

**Checkpoint**: Schéma SQL validé : le backend peut commencer

---

## Phase 2: Foundational (Backend Models + Interfaces TS)

**Purpose**: Structs Rust et interfaces TypeScript mises à jour, BLOQUE toutes les user stories

**⚠️ CRITICAL**: Aucun travail sur les user stories ne peut commencer avant la fin de cette phase

- [x] T008 [P] Ajouter les nouveaux champs aux structs `AvisRecherche`, `CreerAvisRecherche`, `ModifierAvisRecherche` et les structs de réponse dans `uafricas_backend/src/models/retrouve_amis.rs`
- [x] T009 [P] Mettre à jour les interfaces TypeScript (`AvisRecherche`, `CreerAvisRecherche`, `AvisPublic`, etc.) et ajouter les types pour les nouveaux enums dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T010 Supprimer la route `publier_avis` (PATCH `/avis/{id}/publier`) dans `uafricas_backend/src/routes.rs`
- [x] T011 Compiler le backend (`cargo check`) et vérifier qu'il n'y a pas d'erreurs de typage

**Checkpoint**: Fondation prête : l'implémentation des user stories peut commencer

---

## Phase 3: User Story 1 : Consulter les avis publics sans connexion (Priority: P1) 🎯 MVP

**Goal**: Tous les avis de recherche actifs sont visibles sur `/retrouve-amis` par n'importe quel visiteur, sans connexion requise.

**Independent Test**: Ouvrir `http://localhost:3000/retrouve-amis` en navigation privée → la liste des avis actifs est visible avec nom, type de relation, lieu, photo et description.

### Implementation for User Story 1

- [x] T012 [US1] Mettre à jour le handler `lister_avis_publics` pour inclure les nouveaux champs (genre_recherche, type_relation, localite_rencontre, ecole_rencontre, ville_rencontre, photo_url, description_physique, auteur_anonyme, auteur_pseudonyme) dans `uafricas_backend/src/handlers/retrouve_amis_public.rs`
- [x] T013 [US1] Mettre à jour le handler `detail_avis_public` (par slug) pour inclure les nouveaux champs (surnom, comment_connu, jamais_rencontre, description, compteur_partages) dans `uafricas_backend/src/handlers/retrouve_amis_public.rs`
- [x] T014 [P] [US1] Mettre à jour le composant `CarteAvisPublic.vue` pour afficher les nouveaux champs (type de relation, lieu de rencontre, photo, description physique, auteur anonyme/pseudonyme) dans `uafricas_frontend/app/components/retrouve-amis/CarteAvisPublic.vue`
- [x] T015 [US1] Transformer la page `index.vue` pour afficher les avis publics en premier plan (hero + listing paginé), accessibles sans connexion, avec le dashboard utilisateur en section secondaire pour les connectés dans `uafricas_frontend/app/pages/retrouve-amis/index.vue`
- [x] T016 [US1] Mettre à jour la méthode `listerAvisPublics` du composable pour mapper les nouveaux champs de la réponse API dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T017 [US1] Ajouter l'état vide (aucun avis disponible) avec message d'invitation à créer le premier avis dans `uafricas_frontend/app/pages/retrouve-amis/index.vue`

**Checkpoint**: La page `/retrouve-amis` affiche les avis publics pour tous les visiteurs

---

## Phase 4: User Story 2 : Remplir le formulaire de recherche (Priority: P1)

**Goal**: Un utilisateur connecté peut créer un avis via un formulaire 6 étapes et l'avis est publié immédiatement.

**Independent Test**: Se connecter, créer un avis avec le formulaire complet (6 étapes + photo), vérifier qu'il apparaît immédiatement sur `/retrouve-amis`.

### Implementation for User Story 2

- [x] T018 [US2] Passer le handler `creer_avis` en multipart/form-data avec parsing des nouveaux champs, upload photo (JPEG/PNG/WebP, max 5 Mo), publication automatique (`est_public = true`, slug auto-généré) et validation dans `uafricas_backend/src/handlers/retrouve_amis.rs`
- [x] T019 [US2] Créer le dossier d'upload `uploads/retrouve-amis/` au runtime et implémenter la logique de sauvegarde/suppression de photo dans `uafricas_backend/src/handlers/retrouve_amis.rs`
- [x] T020 [US2] Ajouter la validation backend : nom_recherche obligatoire, au moins un critère (type_relation OU localite/ecole/ville OU jamais_rencontre), max 10 avis actifs par utilisateur, coordonnées valides si partage activé dans `uafricas_backend/src/handlers/retrouve_amis.rs`
- [x] T021 [US2] Réécrire le composant `AvisRechercheForm.vue` avec 6 étapes : (1) Préférences anonymat/coordonnées, (2) Identité genre/noms/surnom/comment_connu, (3) Relation type_relation, (4) Lieu de rencontre localité/école/ville/jamais_rencontre, (5) Photo upload + description physique, (6) Récapitulatif dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue`
- [x] T022 [US2] Mettre à jour la méthode `creerAvis` du composable pour envoyer en multipart/form-data (FormData) au lieu de JSON dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T023 [US2] Adapter la page `nouveau.vue` au nouveau composant formulaire et ajouter le message de succès avec lien vers l'avis public dans `uafricas_frontend/app/pages/retrouve-amis/nouveau.vue`
- [x] T024 [US2] Ajouter la validation frontend : champs obligatoires, messages d'erreur clairs par étape, empêcher la soumission si incomplet dans `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue`

**Checkpoint**: Le formulaire 6 étapes fonctionne et publie les avis automatiquement

---

## Phase 5: User Story 3 : Filtrer et rechercher parmi les avis publics (Priority: P2)

**Goal**: Les visiteurs peuvent filtrer par type de relation et rechercher par texte sur la page `/retrouve-amis`.

**Independent Test**: Accéder à `/retrouve-amis`, filtrer par "amis d'école" → seuls les avis correspondants s'affichent. Saisir un nom dans la recherche → résultats filtrés.

### Implementation for User Story 3

- [x] T025 [US3] Ajouter le paramètre de requête `type_relation` au handler `rechercher_avis_publics` et intégrer dans la requête SQL dans `uafricas_backend/src/handlers/retrouve_amis_public.rs`
- [x] T026 [US3] Ajouter les composants de filtre (sélecteur type_relation, barre de recherche textuelle, bouton réinitialiser) sur la page `index.vue` dans `uafricas_frontend/app/pages/retrouve-amis/index.vue`
- [x] T027 [US3] Mettre à jour la méthode `rechercherAvisPublics` du composable pour passer le paramètre `type_relation` et gérer les états vides (aucun résultat) dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T028 [US3] Gérer l'état "aucun résultat" avec un message invitant à modifier les critères de recherche dans `uafricas_frontend/app/pages/retrouve-amis/index.vue`

**Checkpoint**: Filtrage par type de relation et recherche textuelle fonctionnels

---

## Phase 6: User Story 4 : Gérer ses avis publiés (Priority: P2)

**Goal**: L'auteur peut modifier et clôturer ses avis. Les coordonnées ne sont jamais exposées publiquement.

**Independent Test**: Créer un avis, le modifier (changer la description), le clôturer → vérifier les changements sur la page publique et le bandeau "Personne retrouvée !".

### Implementation for User Story 4

- [x] T029 [US4] Passer le handler `modifier_avis` en multipart/form-data avec gestion photo (remplacement si nouvelle, conservation si absente) et relance du recoupement automatique dans `uafricas_backend/src/handlers/retrouve_amis.rs`
- [x] T030 [US4] Vérifier que les champs `coordonnees_email`, `coordonnees_telephone`, `coordonnees_whatsapp` ne sont JAMAIS inclus dans les réponses des handlers publics dans `uafricas_backend/src/handlers/retrouve_amis_public.rs`
- [x] T031 [US4] Ajouter le bandeau "Personne retrouvée !" sur les avis clôturés dans le composant `CarteAvisPublic.vue` dans `uafricas_frontend/app/components/retrouve-amis/CarteAvisPublic.vue`
- [x] T032 [US4] Mettre à jour la méthode `modifierAvis` du composable pour envoyer en multipart/form-data dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T033 [US4] Afficher le pseudonyme (prénom + initiale) ou "Anonyme" selon le choix de l'auteur sur la carte publique dans `uafricas_frontend/app/components/retrouve-amis/CarteAvisPublic.vue`

**Checkpoint**: Modification et clôture des avis fonctionnels, coordonnées protégées

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: SEO, sécurité, optimisations transversales

- [x] T034 [P] Ajouter les balises Open Graph (og:title, og:description, og:image, og:url) et métadonnées SEO sur la page de détail de chaque avis dans `uafricas_frontend/app/pages/retrouve-amis/[slug].vue`
- [x] T035 [P] Ajouter le CTA "Connectez-vous pour contacter l'auteur" pour les visiteurs non connectés sur la page de détail d'un avis dans `uafricas_frontend/app/pages/retrouve-amis/[slug].vue`
- [x] T036 Vérifier la sécurité : sanitisation du nom de fichier uploadé, validation MIME type, pas de path traversal dans `uafricas_backend/src/handlers/retrouve_amis.rs`
- [x] T037 Exécuter la validation complète du quickstart.md (créer un avis en navigation privée, vérifier listing public, filtres, modification, clôture)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Pas de dépendances, commencer immédiatement
- **Foundational (Phase 2)**: Dépend de la Phase 1 (schéma SQL appliqué), BLOQUE toutes les user stories
- **US1 (Phase 3)**: Dépend de Phase 2 : peut commencer dès que les models/interfaces sont prêts
- **US2 (Phase 4)**: Dépend de Phase 2 : peut commencer en parallèle avec US1
- **US3 (Phase 5)**: Dépend de US1 (la page index.vue doit exister avec le listing)
- **US4 (Phase 6)**: Dépend de US2 (le handler multipart doit exister pour modifier_avis)
- **Polish (Phase 7)**: Dépend de US1 + US2

### User Story Dependencies

```
Phase 1 (SQL) → Phase 2 (Models/Interfaces)
                    ├── US1 (Listing public)  ──→ US3 (Filtres)
                    └── US2 (Formulaire)      ──→ US4 (Gestion)
                                                     ↓
                                              Phase 7 (Polish)
```

### Within Each User Story

- Backend avant Frontend (le frontend consomme l'API)
- Models avant Handlers
- Handlers avant Composables
- Composables avant Pages/Composants

### Parallel Opportunities

- **Phase 2**: T008 (Rust models) et T009 (TS interfaces) en parallèle
- **Phase 3**: T014 (CarteAvisPublic) en parallèle avec T012/T013 (handlers backend)
- **Phase 3 + Phase 4**: US1 et US2 en parallèle après Phase 2
- **Phase 7**: T034 et T035 en parallèle

---

## Parallel Example: US1 + US2

```bash
# Après Phase 2, lancer US1 et US2 en parallèle :

# Agent 1 : US1 (Listing public) :
Task: T012 "Mettre à jour lister_avis_publics dans retrouve_amis_public.rs"
Task: T013 "Mettre à jour detail_avis_public dans retrouve_amis_public.rs"
Task: T014 "Mettre à jour CarteAvisPublic.vue" (parallèle avec T012/T013)
Task: T015 "Transformer index.vue"
Task: T016 "Mettre à jour composable listerAvisPublics"
Task: T017 "État vide sur index.vue"

# Agent 2 : US2 (Formulaire) :
Task: T018 "Handler creer_avis multipart"
Task: T019 "Upload photo retrouve-amis/"
Task: T020 "Validation backend"
Task: T021 "Réécrire AvisRechercheForm.vue 6 étapes"
Task: T022 "Composable creerAvis multipart"
Task: T023 "Adapter nouveau.vue"
Task: T024 "Validation frontend"
```

---

## Implementation Strategy

### MVP First (US1 + US2)

1. Compléter Phase 1: Setup SQL
2. Compléter Phase 2: Foundational (CRITICAL)
3. Compléter Phase 3: US1 : Listing public
4. Compléter Phase 4: US2 : Formulaire de création
5. **STOP et VALIDER**: Créer un avis → vérifier qu'il apparaît sur `/retrouve-amis` sans connexion
6. Déployer/démo si prêt

### Incremental Delivery

1. Phase 1 + Phase 2 → Fondation prête
2. US1 → Listing public visible → Démo (MVP partiel)
3. US2 → Formulaire fonctionnel → Démo (MVP complet!)
4. US3 → Filtres et recherche → Démo
5. US4 → Gestion des avis → Démo
6. Phase 7 → SEO + Polish → Déploiement

---

## Notes

- [P] tasks = fichiers différents, pas de dépendances
- [Story] label associe chaque tâche à sa user story pour la traçabilité
- Pas de tests automatisés : vérification manuelle via quickstart.md
- Commiter après chaque tâche ou groupe logique
- S'arrêter à chaque checkpoint pour valider la story indépendamment
- Les coordonnées (email, téléphone, WhatsApp) ne doivent JAMAIS apparaître dans les réponses publiques
