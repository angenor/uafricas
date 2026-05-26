---
description: "Task list — Marché Africain : acheter, vendre, troquer, donner"
---

# Tasks: Marché Africain — acheter, vendre, troquer, donner

**Input**: Design documents from `/specs/001-marche-achat-vente-troc-don/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Aucun framework de test configuré (Contrainte constitution) et aucune demande de TDD dans la spec → **pas de tâches de test**. Validation via `quickstart.md` (manuelle).

**Organization**: Tâches groupées par user story pour une livraison incrémentale indépendante.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers différents, pas de dépendance bloquante)
- **[Story]** : US1..US4 (phases user story uniquement)

## Path Conventions

- Backend : `uafricas_backend/src/...`, schémas `uafricas_backend/doc/bd/schemas/...`
- Frontend : `uafricas_frontend/app/...`

---

## Phase 1: Setup (Infrastructure partagée)

**Purpose**: Préparer le schéma et l'environnement avant tout développement.

- [X] T001 [P] Ajouter la valeur `'conclue'` à l'enum `marketplace.etat_annonce` dans `uafricas_backend/doc/bd/schemas/05_marketplace.sql` (liste : `'brouillon', 'publiee', 'en_attente', 'expiree', 'suspendue', 'supprimee', 'conclue'`)
- [X] T002 [P] Créer la migration `uafricas_backend/doc/bd/schemas/30_social_conversation_annonce.sql` (colonne `annonce_id UUID REFERENCES marketplace.annonce(id) ON DELETE SET NULL` + index partiel) et la câbler dans l'orchestrateur `uafricas_backend/doc/bd/schema.sql` via `\ir` (après `29_social.sql`)
- [X] T003 Documenter/garantir la création du dossier d'upload `./uploads/marketplace/annonces/` au runtime (suivre le pattern des handlers existants ; ajouter le sous-dossier à la section Upload de `CLAUDE.md`)

**Checkpoint**: Schéma à jour (re-init dev via `docker compose down -v && up -d`, ou `ALTER TYPE`/migration manuelle sur base existante).

---

## Phase 2: Foundational (Prérequis bloquants)

**Purpose**: Plomberie backend/frontend partagée par toutes les user stories.

**⚠️ CRITICAL**: Aucune user story ne peut démarrer avant cette phase.

- [X] T004 Ajouter le helper d'auth membre `utilisateur_courant(req: &HttpRequest) -> Result<Uuid, ApiErreur>` (extraction JWT Bearer) dans `uafricas_backend/src/handlers/annonces.rs`, sur le modèle de `handlers/amitie.rs` (D1 — un JWT valide implique un compte `actif`)
- [X] T005 [P] Ajouter les DTOs membre dans `uafricas_backend/src/models/annonce.rs` : `CreerAnnonceMembreRequest`, `ModifierAnnonceMembreRequest`, `ContacterAuteurRequest`, `MesAnnoncesItemResponse` (avec `etat`), `FavoriResponse` (réutiliser `AnnonceResponse`/`AnnonceDetailResponse` et `mapper_type_operation`/`mapper_condition`)
- [X] T006 [P] Étendre `uafricas_frontend/app/composables/useMarcheAfricain.ts` avec les signatures des fonctions d'écriture (stubs `creerAnnonce`, `modifierAnnonce`, `supprimerAnnonce`, `conclureAnnonce`, `mesAnnonces`, `ajouterFavori`, `retirerFavori`, `listerFavoris`, `contacterAuteur`) + l'interface `CreerAnnonceForm` et le mapping inverse `mapperTypeVersDb`

**Checkpoint**: Auth membre, DTOs et squelette composable prêts.

---

## Phase 3: User Story 1 — Publier une annonce (Priority: P1) 🎯 MVP

**Goal**: Un membre connecté publie une annonce (vente/troc/don) avec photos, visible immédiatement.

**Independent Test**: Se connecter, publier une annonce de chaque type avec ≥1 photo, vérifier l'apparition immédiate dans la liste et le détail (quickstart Scénario 1).

### Implementation

- [X] T007 [US1] Implémenter le handler `creer_annonce_membre` (POST `/api/annonces`) dans `uafricas_backend/src/handlers/annonces.rs` : multipart, validation (titre/description/catégorie obligatoires, `type_operation ∈ {vente,troc,don}`, `prix` requis si vente), `cree_par = utilisateur_courant`, `type_contact='messagerie_plateforme'`, `etat='publiee'`, génération `slug`, insertion `annonce_pays` (pays_ids)
- [X] T008 [US1] Upload des photos dans `creer_annonce_membre` : ≤ 5 fichiers, ≤ 3 Mo, MIME JPEG/PNG/WebP via le service `image_validation` + `sanitize_filename`, stockage `./uploads/marketplace/annonces/<uuid>.<ext>`, insertion `annonce_media` (1ʳᵉ = `est_principale`, `ordre`)
- [X] T009 [US1] Auditer la création via `audit::log_action` (table `marketplace.annonce`) dans `uafricas_backend/src/handlers/annonces.rs`
- [X] T010 [US1] Enregistrer la route `POST /annonces` (scope membre, auth) dans `uafricas_backend/src/routes.rs`
- [X] T011 [P] [US1] Créer le composant `uafricas_frontend/app/components/marche/MarcheAnnonceForm.vue` (Tailwind v4 pur, **sans daisyUI** — Principe VI) : champs annonce + sélection catégorie/territoires/devise + upload multi-photos (preview, limites 5/3 Mo/formats), validation client, émission du `FormData`
- [X] T012 [US1] Implémenter `creerAnnonce(form)` (POST multipart `/api/annonces`) dans `uafricas_frontend/app/composables/useMarcheAfricain.ts`
- [X] T013 [US1] Brancher le bouton « Publier » dans `uafricas_frontend/app/pages/marche-africain/index.vue` : remplacer le modal placeholder par `MarcheAnnonceForm`, garde d'auth (`isAuthenticated` sinon redirection `/login`), rafraîchir la liste après succès

**Checkpoint**: US1 pleinement fonctionnelle et testable seule (MVP).

---

## Phase 4: User Story 2 — Contacter l'auteur via messagerie (Priority: P1)

**Goal**: Un membre intéressé ouvre une conversation privée (rattachée à l'annonce) avec l'auteur via la messagerie existante.

**Independent Test**: Avec 2 comptes non amis, contacter l'auteur d'une annonce, vérifier réception côté auteur + réponse possible (quickstart Scénario 2).

### Implementation

- [X] T014 [US2] Assouplir `envoyer_message` dans `uafricas_backend/src/handlers/messagerie.rs` : autoriser l'envoi si **amitié active OU conversation déjà existante** entre les deux membres (blocage toujours bloquant) — D2
- [X] T015 [US2] Implémenter le handler `contacter_auteur` (POST `/api/annonces/{id}/contacter`) dans `uafricas_backend/src/handlers/annonces.rs` : vérifs (annonce `publiee`, `cree_par <> courant` → FR-013, pas de blocage réciproque), `obtenir_ou_creer_conversation` **sans exiger l'amitié**, renseigner `conversation.annonce_id` (COALESCE) à la création, insérer le message initial, MAJ `dernier_message_at`, push SSE (`evt_message`), notification auteur (FR-012), audit
- [X] T016 [US2] Enregistrer la route `POST /annonces/{id}/contacter` (auth) dans `uafricas_backend/src/routes.rs`
- [X] T017 [US2] Implémenter `contacterAuteur(annonceId, message)` dans `uafricas_frontend/app/composables/useMarcheAfricain.ts`
- [X] T018 [US2] Brancher le bouton « Contacter / Je suis intéressé(e) » dans `uafricas_frontend/app/pages/marche-africain/[id].vue` : remplacer l'`alert()`, garde d'auth, masquer le bouton sur sa propre annonce (FR-013), après succès rediriger vers la messagerie sur `conversation_id`
- [X] T019 [P] [US2] Afficher le contexte « À propos de l'annonce : <titre> » dans l'UI de messagerie (`uafricas_frontend/app/components/social/...`) via `conversation.annonce_id` (jointure côté backend dans la liste/détail des conversations)

**Checkpoint**: US1 + US2 fonctionnelles indépendamment — acheter/prendre/troquer possible via contact.

---

## Phase 5: User Story 3 — Gérer ses propres annonces (Priority: P2)

**Goal**: L'auteur consulte « Mes annonces », modifie, marque conclue ou supprime ses annonces.

**Independent Test**: Publier, modifier, conclure (disparaît du public, reste dans Mes annonces), supprimer ; échec `403` sur annonce d'autrui (quickstart Scénario 3).

### Implementation

- [X] T020 [US3] Implémenter `mes_annonces` (GET `/api/annonces/mes-annonces`) dans `uafricas_backend/src/handlers/annonces.rs` : pagination, filtre `cree_par = courant`, tous états visibles à l'auteur, `deleted_at IS NULL`, item avec `etat`
- [X] T021 [US3] Implémenter `modifier_annonce_membre` (PUT `/api/annonces/{id}`) dans `uafricas_backend/src/handlers/annonces.rs` : garde propriétaire (`403` sinon), MAJ partielle, régénération `slug` si titre, audit
- [X] T022 [US3] Implémenter `conclure_annonce` (PATCH `/api/annonces/{id}/conclure`) dans `uafricas_backend/src/handlers/annonces.rs` : garde propriétaire, `publiee → conclue`, audit (FR-018)
- [X] T023 [US3] Implémenter `supprimer_annonce_membre` (DELETE `/api/annonces/{id}`) dans `uafricas_backend/src/handlers/annonces.rs` : garde propriétaire, soft delete (`etat='supprimee'`, `deleted_at=NOW()`), audit
- [X] T024 [US3] Implémenter la gestion des photos par le propriétaire (POST `/api/annonces/{id}/medias`, DELETE `/api/annonces/{id}/medias/{media_id}`) dans `uafricas_backend/src/handlers/annonces.rs` : garde propriétaire, plafond 5, promotion d'une nouvelle principale si la principale est retirée
- [X] T025 [US3] Enregistrer les routes US3 dans `uafricas_backend/src/routes.rs` — **`/annonces/mes-annonces` AVANT `/annonces/{id}`** (D8)
- [X] T026 [US3] Implémenter `mesAnnonces()`, `modifierAnnonce()`, `conclureAnnonce()`, `supprimerAnnonce()` dans `uafricas_frontend/app/composables/useMarcheAfricain.ts`
- [X] T027 [US3] Créer la page `uafricas_frontend/app/pages/marche-africain/mes-annonces.vue` (Tailwind pur, garde d'auth) : liste avec état, actions modifier/conclure/supprimer
- [X] T028 [US3] Réutiliser `MarcheAnnonceForm.vue` en **mode édition** (pré-remplissage + gestion ajout/retrait photos) depuis `mes-annonces.vue`

**Checkpoint**: US1 + US2 + US3 fonctionnelles.

---

## Phase 6: User Story 4 — Favoris (Priority: P3)

**Goal**: Un membre sauvegarde des annonces et consulte ses favoris.

**Independent Test**: Ajouter/retirer des favoris, consulter la liste (quickstart Scénario 4).

### Implementation

- [X] T029 [US4] Implémenter `ajouter_favori` (POST `/api/annonces/{id}/favori`, idempotent `ON CONFLICT DO NOTHING`) et `retirer_favori` (DELETE) dans `uafricas_backend/src/handlers/annonces.rs` + audit
- [X] T030 [US4] Implémenter `mes_favoris` (GET `/api/annonces/favoris`, paginé, annonces favorites encore publiées) dans `uafricas_backend/src/handlers/annonces.rs`
- [X] T031 [US4] Enregistrer les routes favoris dans `uafricas_backend/src/routes.rs` — **`/annonces/favoris` AVANT `/annonces/{id}`** (D8)
- [X] T032 [P] [US4] Créer le composant `uafricas_frontend/app/components/marche/MarcheFavoriBouton.vue` (Tailwind pur, état actif/inactif, garde d'auth)
- [X] T033 [US4] Implémenter `ajouterFavori()`, `retirerFavori()`, `listerFavoris()` dans `uafricas_frontend/app/composables/useMarcheAfricain.ts`
- [X] T034 [US4] Créer la page `uafricas_frontend/app/pages/marche-africain/favoris.vue` et insérer `MarcheFavoriBouton` dans `AnnonceCard.vue` et `[id].vue`

**Checkpoint**: Les 4 user stories sont fonctionnelles indépendamment.

---

## Phase 7: Polish & Cross-Cutting Concerns

- [X] T035 [P] Généraliser le libellé « Vendeur » → « Annonceur / Auteur » et appliquer la terminologie « territoire » dans `uafricas_frontend/app/pages/marche-africain/[id].vue` et `app/components/marche/` (hypothèse spec)
- [X] T036 [P] Vérifier l'absence de classes daisyUI sur toutes les pages/composants `marche-africain` (Principe VI)
- [X] T037 [P] Vérifier la couverture d'audit de toutes les mutations marketplace dans l'admin (Principe VII)
- [X] T038 [P] Mettre à jour la section « Recent Changes » de `CLAUDE.md` (feature marché membre)
- [ ] T039 Exécuter la validation `quickstart.md` (5 scénarios) et corriger les écarts
- [X] T040 [P] Vérifier la cohérence des types interface TS ↔ struct Rust ↔ schéma SQL (Principe II/III)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance — démarrer immédiatement
- **Foundational (Phase 2)** : dépend de Setup — BLOQUE toutes les user stories
- **User Stories (Phase 3-6)** : dépendent de Foundational
  - US1 et US2 sont toutes deux P1 ; US2 (T014) modifie `messagerie.rs` (fichier distinct d'US1) → parallélisables
  - US3, US4 indépendantes l'une de l'autre
- **Polish (Phase 7)** : après les user stories visées

### User Story Dependencies

- **US1 (P1)** : après Foundational. Aucune dépendance inter-story. **MVP**
- **US2 (P1)** : après Foundational. Indépendante d'US1 (touche `messagerie.rs` + endpoint contact). Testable seule sur une annonce existante.
- **US3 (P2)** : après Foundational. Réutilise `MarcheAnnonceForm` (T011/US1) pour l'édition — si US1 non faite, créer le formulaire dans US3.
- **US4 (P3)** : après Foundational. Indépendante.

### Within Each User Story

- Backend modèles/DTOs (Phase 2) → handlers → enregistrement des routes
- Backend endpoint → composable → page/composant frontend

### Parallel Opportunities

- T001, T002 en parallèle (fichiers SQL distincts)
- T005, T006 en parallèle (backend models vs frontend composable)
- US1 et US2 par deux développeurs après la Phase 2 (fichiers majoritairement distincts ; coordination sur `routes.rs`)
- Composants frontend marqués [P] (T011, T032) en parallèle de leur backend respectif

---

## Parallel Example: Phase 2 (Foundational)

```text
# Lancer en parallèle :
Task T005: "Ajouter les DTOs membre dans uafricas_backend/src/models/annonce.rs"
Task T006: "Étendre useMarcheAfricain.ts avec les stubs de fonctions d'écriture"
```

## Parallel Example: User Story 1

```text
# Backend et composant UI en parallèle après T010 :
Task T011: "Créer MarcheAnnonceForm.vue (Tailwind pur)"
# (le handler T007/T008 peut avancer côté backend simultanément)
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Phase 1 (Setup) → 2. Phase 2 (Foundational) → 3. Phase 3 (US1)
4. **STOP & VALIDATE** : publier une annonce et la voir apparaître (quickstart Scénario 1)
5. Démo / déploiement possible

### Incrémental

1. Setup + Foundational → socle prêt
2. US1 (publier) → MVP
3. US2 (contacter) → marché « vivant » (acheter/troquer/donner)
4. US3 (gérer) → qualité et cycle de vie
5. US4 (favoris) → confort

### Note migration BDD

Les changements de schéma (T001/T002) nécessitent en production une migration manuelle (SSH+psql) : `ALTER TYPE marketplace.etat_annonce ADD VALUE IF NOT EXISTS 'conclue';` puis exécution de `30_social_conversation_annonce.sql`. En dev : `docker compose down -v && docker compose up -d`.

---

## Notes

- [P] = fichiers différents, sans dépendance bloquante
- Coordination requise sur `uafricas_backend/src/routes.rs` (enregistrements multiples — respecter l'ordre statique avant dynamique, D8)
- Coordination sur `uafricas_frontend/app/composables/useMarcheAfricain.ts` (étendu par plusieurs stories)
- Pas de tâche de test (aucun framework configuré) — validation manuelle via `quickstart.md`
- Commit après chaque tâche ou groupe logique ; messages en français
