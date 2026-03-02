# Tasks: Partage Public des Avis de Recherche

**Input**: Design documents from `/specs/002-partage-avis-recherche/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/
**Tests**: Non inclus (pas de framework de test configuré — cf. plan.md)
**Organization**: Tasks groupées par user story. Chaque story est indépendamment implémentable et testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut être exécuté en parallèle (fichiers différents, pas de dépendances)
- **[Story]**: User story concernée (US1–US5)
- Chemins relatifs depuis la racine du monorepo

## Path Conventions

- **Backend**: `uafricas_backend/src/` (handlers, models, routes)
- **Frontend**: `uafricas_frontend/app/` (pages, components, composables)
- **SQL**: `uafricas_backend/doc/bd/schemas/`

---

## Phase 1: Setup — Schema SQL (Source de Vérité)

**Purpose**: Étendre le schema `retrouve_amis` existant avec les nouvelles structures de données conformément à `data-model.md`

- [x] T001 Ajouter les 3 nouveaux enums (`type_reponse_publique`, `etat_demande_retrait`, `source_signalement`) et ALTER TABLE `avis_recherche` (+`est_public` BOOLEAN DEFAULT FALSE, +`slug` VARCHAR(400) UNIQUE, +`date_publication_publique` TIMESTAMPTZ, +`compteur_partages` INTEGER DEFAULT 0 CHECK >= 0) dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T002 ALTER TABLE `signalement` (+`source` source_signalement DEFAULT 'correspondance') et ALTER TYPE `type_notification` (+`reponse_publique`, +`demande_retrait`) dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T003 CREATE TABLE `reponse_publique` (id UUID PK, avis_id FK, repondeur_id FK, type_reponse, message TEXT, correspondance_id FK nullable, created_at) avec UNIQUE(avis_id, repondeur_id) et index dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T004 CREATE TABLE `demande_retrait` (id UUID PK, avis_id FK, demandeur_id FK, motif TEXT, etat etat_demande_retrait DEFAULT 'en_attente', date_suspension, decide_par FK nullable, decision_at, commentaire_admin, created_at) avec UNIQUE(avis_id, demandeur_id) et index dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T005 Créer l'index partiel `idx_avis_public_actif` (WHERE est_public = TRUE AND etat = 'actif' AND deleted_at IS NULL) dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
- [x] T006 Réinitialiser la base de données avec le nouveau schema : `docker compose down -v && docker compose up -d`

**Checkpoint**: Le schema SQL est à jour. Vérifier avec Adminer (http://localhost:8088) que les nouvelles tables et colonnes existent.

---

## Phase 2: Foundational — Backend Types & Module Setup

**Purpose**: Structs Rust, DTOs et module public handler — bloque toutes les user stories

**CRITICAL**: Aucune user story ne peut commencer avant la fin de cette phase

- [x] T007 [P] Ajouter les structs et DTOs publics dans `uafricas_backend/src/models/retrouve_amis.rs` : `AvisPublicDetail` (avec auteur_anonyme, pays nom), `AvisPublicResume` (pour listing), `AvisPublicEtatReponse` (pour avis non-actifs), `PublierAvisRequest` (est_public bool), `PublierAvisReponse`, `ReponsePubliqueRequest` (type_reponse + message), `ReponsePubliqueReponse`, `SignalerPublicRequest` (motif + description), `DemandeRetraitRequest` (motif), `DemandeRetraitReponse`, `PartageReponse` (compteur_partages), `RecherchePubliqueParams` (page, par_page, recherche, pays_id, ville, ecole, tri, ordre), `ListeAvisPublicsReponse` (avis + pagination)
- [x] T008 [P] Ajouter les structs admin dans `uafricas_backend/src/models/admin/retrouve_amis.rs` : `DemandeRetraitAdmin` (avec nom_recherche, demandeur anonymisé, auteur anonymisé), `ListeDemandesRetraitParams` (page, par_page, etat, tri_par, tri_dir), `ListeDemandesRetraitReponse`, `StatuerDemandeRequest` (decision + commentaire), `StatuerDemandeReponse`
- [x] T009 Créer le module `uafricas_backend/src/handlers/retrouve_amis_public.rs` avec les imports nécessaires (actix_web, sqlx, models) et l'enregistrer dans `uafricas_backend/src/handlers/mod.rs`

**Checkpoint**: `cargo check` compile sans erreur. Le module public handler est déclaré.

---

## Phase 3: User Story 1 — Rendre un avis public (Priority: P1) MVP

**Goal**: Permettre à l'auteur d'activer la visibilité publique et créer une page publique accessible sans authentification avec affichage conditionnel selon l'état

**Independent Test**: Créer un avis, activer "Rendre public", accéder à l'URL `/retrouve-amis/public/{slug}` en navigation privée (non connecté) et vérifier l'affichage anonymisé

### Implementation for User Story 1

- [x] T010 [P] [US1] Implémenter le handler `publier_avis` (PATCH `/api/retrouve-amis/avis/{id}/publier`) dans `uafricas_backend/src/handlers/retrouve_amis.rs` : vérifier auteur + etat actif, générer slug `{nom}-{prenom}-{uuid8}` à la première activation, set `date_publication_publique` une seule fois, toggle `est_public`, audit via `audit::log_action`. Contrat: `contracts/auth-api.md` section PATCH publier
- [x] T011 [P] [US1] Implémenter le handler `detail_avis_public` (GET `/api/retrouve-amis/public/{slug}`) dans `uafricas_backend/src/handlers/retrouve_amis_public.rs` : retourner `AvisPublicDetail` si actif (avec `auteur_anonyme` = prénom + initiale nom + ".", pays.nom via JOIN), `AvisPublicEtatReponse` si cloturé/suspendu, 404 si dépublié (`est_public = FALSE`). Header `X-Robots-Tag: noindex, nofollow` si etat != actif. Contrat: `contracts/public-api.md` section GET {slug}
- [x] T012 [US1] Enregistrer les routes US1 dans `uafricas_backend/src/routes.rs` : GET `/api/retrouve-amis/public/{slug}` (hors scope JWT), PATCH `/api/retrouve-amis/avis/{id}/publier` (dans scope JWT retrouve-amis)
- [x] T013 [US1] Compiler et tester le backend : `cargo check` puis `cargo run`, tester avec curl les 2 endpoints US1
- [x] T014 [P] [US1] Ajouter les fonctions API `publierAvis(id, estPublic)` et `detailAvisPublic(slug)` avec les interfaces TypeScript correspondantes dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T015 [P] [US1] Créer le composant `PagePublique.vue` dans `uafricas_frontend/app/components/retrouve-amis/PagePublique.vue` : affichage du nom recherché, prénom, école, ville, pays, période, description, auteur anonyme, compteur partages, date publication. Tailwind CSS v4 pur (PAS de daisyUI). Affichage conditionnel par etat (actif: contenu complet, cloturé: "Cette personne a été retrouvée !", suspendu: "Cet avis a été temporairement retiré", dépublié: "Cet avis n'est plus disponible")
- [x] T016 [US1] Créer la page `uafricas_frontend/app/pages/retrouve-amis/public/[slug].vue` : SSR avec `useSeoMeta()` pour les balises de base (title, description, og:url), `useFetch` pour charger les données via `detailAvisPublic(slug)`, intégrer `PagePublique.vue`, ajouter `noindex/nofollow` si etat != actif via `useHead()`. Layout default
- [x] T017 [US1] Modifier `uafricas_frontend/app/pages/retrouve-amis/mes-recherches.vue` : ajouter un interrupteur (toggle switch) "Rendre public" par avis qui appelle `publierAvis()`, afficher le lien public cliquable + compteur de partages quand `est_public = true`. Tailwind CSS v4 pur

**Checkpoint**: Un avis peut être rendu public et sa page est accessible sans connexion. L'auteur est anonymisé.

---

## Phase 4: User Story 4 — Protections anti-harcèlement (Priority: P1)

**Goal**: Permettre le signalement depuis la page publique (avec auto-suspension à 3), la demande de retrait avec suspension immédiate, et l'arbitrage admin

**Independent Test**: Signaler un avis 3 fois avec des comptes différents → vérification suspension auto. Demander un retrait → vérification suspension immédiate. Admin statue → vérification réactivation ou retrait définitif

### Implementation for User Story 4

- [x] T018 [P] [US4] Implémenter le handler `signaler_avis_public` (POST `/api/retrouve-amis/public/{slug}/signaler`) dans `uafricas_backend/src/handlers/retrouve_amis.rs` : vérifier JWT + avis public+actif + pas l'auteur + pas déjà signalé (UNIQUE), insérer signalement avec `source = 'page_publique'`, compter signalements distincts et si >= 3 → UPDATE etat = 'suspendu', audit. Contrat: `contracts/auth-api.md` section POST signaler
- [x] T019 [P] [US4] Implémenter le handler `demander_retrait` (POST `/api/retrouve-amis/public/{slug}/demande-retrait`) dans `uafricas_backend/src/handlers/retrouve_amis.rs` : vérifier JWT + avis public + pas l'auteur + pas déjà demandé (UNIQUE), insérer `demande_retrait`, UPDATE avis `etat = 'suspendu'`, créer notifications (auteur + admins), audit. Contrat: `contracts/auth-api.md` section POST demande-retrait
- [x] T020 [P] [US4] Implémenter le handler `lister_demandes_retrait` (GET `/api/admin/retrouve-amis/demandes-retrait`) dans `uafricas_backend/src/handlers/admin/retrouve_amis.rs` : pagination + filtre par etat + tri + JOIN pour nom_recherche/demandeur/auteur anonymisés. Contrat: `contracts/auth-api.md` section GET demandes-retrait
- [x] T021 [US4] Implémenter le handler `statuer_demande_retrait` (PATCH `/api/admin/retrouve-amis/demandes-retrait/{id}/statuer`) dans `uafricas_backend/src/handlers/admin/retrouve_amis.rs` : vérifier permission admin `retrouve_amis/modifier` + demande en_attente. Si approuvee: avis reste suspendu + `est_public = FALSE`. Si rejetee: avis `etat = 'actif'` + `est_public = TRUE`. Notifications + audit. Contrat: `contracts/auth-api.md` section PATCH statuer
- [x] T022 [US4] Enregistrer les routes US4 dans `uafricas_backend/src/routes.rs` : POST signaler + POST demande-retrait (scope JWT), GET + PATCH demandes-retrait (scope admin)
- [x] T023 [US4] Compiler et tester le backend US4 : `cargo check` puis tester avec curl les 4 endpoints
- [x] T024 [P] [US4] Ajouter les fonctions API `signalerAvisPublic(slug, data)`, `demanderRetrait(slug, motif)` dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T025 [P] [US4] Ajouter les fonctions admin `listerDemandesRetrait(params)`, `statuerDemandeRetrait(id, data)` dans `uafricas_frontend/app/composables/useAdminRetrouvAmis.ts`
- [x] T026 [US4] Créer le composant `DemandeRetrait.vue` dans `uafricas_frontend/app/components/retrouve-amis/DemandeRetrait.vue` : bouton "Cet avis me concerne — demander le retrait" + formulaire motif + bouton "Signaler cet avis" avec formulaire motif/description. Redirection vers connexion si non connecté. Tailwind CSS v4 pur
- [x] T027 [US4] Intégrer `DemandeRetrait.vue` dans `uafricas_frontend/app/pages/retrouve-amis/public/[slug].vue` : afficher uniquement quand etat = actif et utilisateur connecté (pas l'auteur)

**Checkpoint**: Les signalements et demandes de retrait fonctionnent. L'admin peut statuer sur les demandes.

---

## Phase 5: User Story 2 — Partage social (Priority: P2)

**Goal**: Proposer des boutons de partage (WhatsApp, Facebook, X/Twitter, LinkedIn, copier lien) avec compteur et enrichir les balises Open Graph/Twitter Card pour un aperçu riche

**Independent Test**: Cliquer sur "Partager sur WhatsApp" → vérifier message pré-formaté. Partager le lien sur Facebook → vérifier aperçu Open Graph. Le compteur s'incrémente à chaque clic.

### Implementation for User Story 2

- [x] T028 [US2] Implémenter le handler `incrementer_partage` (POST `/api/retrouve-amis/public/{slug}/partage`) dans `uafricas_backend/src/handlers/retrouve_amis_public.rs` : UPDATE atomique `compteur_partages = compteur_partages + 1` WHERE slug + est_public + actif, retourner nouveau compteur. Contrat: `contracts/public-api.md` section POST partage
- [x] T029 [US2] Enregistrer la route POST partage (hors scope JWT) dans `uafricas_backend/src/routes.rs`
- [x] T030 [P] [US2] Ajouter la fonction API `incrementerPartage(slug)` dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T031 [US2] Créer le composant `BoutonsPartage.vue` dans `uafricas_frontend/app/components/retrouve-amis/BoutonsPartage.vue` : 5 boutons (WhatsApp avec message pré-formaté `whatsapp://send?text=...`, Facebook `https://www.facebook.com/sharer/sharer.php?u=...`, X/Twitter `https://twitter.com/intent/tweet?url=...&text=...`, LinkedIn `https://www.linkedin.com/sharing/share-offsite/?url=...`, Copier le lien avec `navigator.clipboard`). Chaque clic appelle `incrementerPartage()`. Afficher compteur total. Tailwind CSS v4 pur
- [x] T032 [US2] Intégrer `BoutonsPartage.vue` dans `uafricas_frontend/app/pages/retrouve-amis/public/[slug].vue` (visible uniquement si etat = actif) et enrichir les balises `useSeoMeta()` avec og:title, og:description, og:image (image par défaut UAfricas), og:type, twitter:card, twitter:title, twitter:description

**Checkpoint**: Les boutons de partage fonctionnent et le compteur s'incrémente. L'aperçu Open Graph est visible lors du partage.

---

## Phase 6: User Story 5 — Parcourir les avis publics (Priority: P2)

**Goal**: Créer une page publique de listing/recherche avec pagination, filtres (pays, ville, école) et recherche full-text

**Independent Test**: Accéder à `/retrouve-amis/rechercher` sans connexion, filtrer par pays, vérifier que seuls les avis publics actifs apparaissent avec pagination

### Implementation for User Story 5

- [x] T033 [US5] Implémenter le handler `rechercher_avis_publics` (GET `/api/retrouve-amis/public/rechercher`) dans `uafricas_backend/src/handlers/retrouve_amis_public.rs` : filtre `est_public = TRUE AND etat = 'actif' AND deleted_at IS NULL`, filtres optionnels (pays_id UUID, ville ILIKE, ecole ILIKE), recherche full-text via `search_vector @@ plainto_tsquery('french', ...)`, pagination (page/par_page max 50), tri (created_at ou compteur_partages), JOIN pays pour nom. Contrat: `contracts/public-api.md` section GET rechercher
- [x] T034 [US5] Enregistrer la route GET rechercher (hors scope JWT) dans `uafricas_backend/src/routes.rs`
- [x] T035 [P] [US5] Ajouter la fonction API `rechercherAvisPublics(params)` et l'interface `RecherchePubliqueParams` dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T036 [P] [US5] Créer le composant `CarteAvisPublic.vue` dans `uafricas_frontend/app/components/retrouve-amis/CarteAvisPublic.vue` : carte résumé (nom recherché, ville, pays, période, compteur partages) avec lien vers `/retrouve-amis/public/{slug}`. Tailwind CSS v4 pur
- [x] T037 [US5] Créer la page `uafricas_frontend/app/pages/retrouve-amis/rechercher.vue` : listing paginé avec `CarteAvisPublic.vue`, filtres (dropdown pays via endpoint existant `/api/retrouve-amis/pays`, champs ville et école), barre de recherche full-text, pagination, message "Aucun avis ne correspond" si vide. Tailwind CSS v4 pur. `useSeoMeta()` pour SEO de base

**Checkpoint**: La page de listing affiche les avis publics avec filtres et recherche fonctionnels.

---

## Phase 7: User Story 3 — Répondre à un avis public (Priority: P3)

**Goal**: Permettre aux utilisateurs connectés de répondre à un avis public via un formulaire structuré, créant automatiquement une correspondance

**Independent Test**: Se connecter, accéder à un avis public, remplir le formulaire de réponse, vérifier que l'auteur reçoit une notification et qu'une correspondance est créée

### Implementation for User Story 3

- [x] T038 [US3] Implémenter le handler `repondre_avis_public` (POST `/api/retrouve-amis/public/{slug}/repondre`) dans `uafricas_backend/src/handlers/retrouve_amis.rs` : vérifier JWT + avis public+actif + pas l'auteur + pas déjà répondu (UNIQUE) + pas dans blacklist + rate limit 10/jour (COUNT reponse_publique WHERE repondeur_id AND created_at > now()-1day), insérer `reponse_publique`, créer `correspondance` (type_cible='profil', score=70, details_score={"source":"reponse_publique","type_reponse":"..."}), créer notification `reponse_publique` pour l'auteur, audit. Contrat: `contracts/auth-api.md` section POST repondre
- [x] T039 [US3] Enregistrer la route POST repondre (dans scope JWT) dans `uafricas_backend/src/routes.rs`
- [x] T040 [P] [US3] Ajouter la fonction API `repondreAvisPublic(slug, data)` dans `uafricas_frontend/app/composables/useRetrouvAmis.ts`
- [x] T041 [US3] Créer le composant `FormulaireReponse.vue` dans `uafricas_frontend/app/components/retrouve-amis/FormulaireReponse.vue` : sélection type de réponse (radio: "Je suis cette personne", "Je la connais", "J'ai des informations"), champ message textarea, bouton envoyer. Redirection vers connexion si non connecté (avec retour automatique vers l'avis après auth — FR-012). Gestion erreur 409 "Vous avez déjà répondu" + erreur 429 "Limite atteinte". Tailwind CSS v4 pur
- [x] T042 [US3] Intégrer `FormulaireReponse.vue` dans `uafricas_frontend/app/pages/retrouve-amis/public/[slug].vue` : afficher uniquement quand etat = actif, masquer si l'utilisateur connecté est l'auteur de l'avis

**Checkpoint**: Les réponses publiques créent des correspondances visibles dans l'espace de l'auteur.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Vérifications finales, intégration et validation

- [ ] T043 Vérifier que toutes les mutations sont auditées via `audit::log_action` (publier, signaler, demander_retrait, statuer_retrait, repondre, incrementer_partage) — revue de code dans `uafricas_backend/src/handlers/retrouve_amis.rs`, `retrouve_amis_public.rs` et `admin/retrouve_amis.rs`
- [ ] T044 Vérifier que toutes les pages publiques utilisent Tailwind CSS v4 pur (aucune classe daisyUI) — revue de `[slug].vue`, `rechercher.vue`, `PagePublique.vue`, `BoutonsPartage.vue`, `FormulaireReponse.vue`, `CarteAvisPublic.vue`, `DemandeRetrait.vue`
- [ ] T045 Exécuter la validation quickstart.md complète : compiler backend (`cargo run`), démarrer frontend (`pnpm dev`), test E2E manuel (créer avis → rendre public → accéder en privé → partager → répondre → signaler → demander retrait → admin statuer)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Aucune dépendance — commence immédiatement
- **Foundational (Phase 2)**: Dépend de Phase 1 (schema SQL doit exister) — **BLOQUE toutes les user stories**
- **US1 (Phase 3)**: Dépend de Phase 2 — **MVP, doit être complété en premier**
- **US4 (Phase 4)**: Dépend de Phase 3 (la page publique doit exister pour signaler/demander retrait)
- **US2 (Phase 5)**: Dépend de Phase 3 (les boutons de partage sont sur la page publique)
- **US5 (Phase 6)**: Dépend de Phase 2 seulement (le listing est une page indépendante)
- **US3 (Phase 7)**: Dépend de Phase 3 (le formulaire de réponse est sur la page publique)
- **Polish (Phase 8)**: Dépend de toutes les phases précédentes

### User Story Dependencies

```
Phase 1 (Setup) → Phase 2 (Foundational)
                        │
                        ▼
                   Phase 3 (US1) ←── MVP
                   ╱    │    ╲
                  ╱     │     ╲
                 ▼      ▼      ▼
           Phase 4  Phase 5  Phase 7
           (US4)    (US2)    (US3)
                 ╲     │    ╱
                  ╲    │   ╱
                   ▼   ▼  ▼
                Phase 6 (US5) ← peut aussi démarrer après Phase 2
                       │
                       ▼
                  Phase 8 (Polish)
```

**Note**: US5 (listing) peut techniquement démarrer dès Phase 2 car c'est une page indépendante. Cependant, l'implémentation séquentielle P1→P2→P3 est recommandée.

### Within Each User Story

- Backend handlers avant routes registration
- Routes registration avant tests curl
- Composable (API functions) avant composants frontend
- Composants avant intégration dans les pages

### Parallel Opportunities

**Phase 2** (après Phase 1):
```
T007 (models publics) ║ T008 (models admin) — fichiers différents
```

**Phase 3 — US1** (après Phase 2):
```
T010 (handler publier) ║ T011 (handler detail) — fichiers différents
T014 (composable)      ║ T015 (PagePublique.vue) — fichiers différents
```

**Phase 4 — US4** (après Phase 3):
```
T018 (signaler)      ║ T020 (admin lister)  — fichiers différents
T024 (composable)    ║ T025 (admin composable) — fichiers différents
```

**Phase 5 — US2** (après Phase 3):
```
T030 (composable) ║ (aucun autre parallélisable dans cette phase)
```

**Phase 6 — US5** (après Phase 2):
```
T035 (composable) ║ T036 (CarteAvisPublic) — fichiers différents
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup SQL
2. Complete Phase 2: Backend types + module
3. Complete Phase 3: User Story 1 (page publique + toggle)
4. **STOP and VALIDATE**: Tester la page publique en navigation privée
5. Démo possible: un avis est visible publiquement avec anonymisation

### Incremental Delivery

1. Setup + Foundational → Schema et types prêts
2. **US1** (P1) → Page publique fonctionnelle → **MVP deployable**
3. **US4** (P1) → Protections anti-harcèlement → **Sécurité complète**
4. **US2** (P2) → Partage social → **Viralité activée**
5. **US5** (P2) → Listing public → **Découvrabilité SEO**
6. **US3** (P3) → Réponses publiques → **Boucle de retrouvailles complète**
7. Polish → Audit + validation finale

### Estimation par phase

| Phase | Tasks | Fichiers touchés |
|-------|-------|-----------------|
| Phase 1: Setup | 6 | 1 SQL + docker |
| Phase 2: Foundational | 3 | 3 backend |
| Phase 3: US1 | 8 | 5 backend + 4 frontend |
| Phase 4: US4 | 10 | 4 backend + 4 frontend |
| Phase 5: US2 | 5 | 2 backend + 3 frontend |
| Phase 6: US5 | 5 | 2 backend + 3 frontend |
| Phase 7: US3 | 5 | 2 backend + 3 frontend |
| Phase 8: Polish | 3 | revue transversale |
| **Total** | **45** | |

---

## Notes

- [P] tasks = fichiers différents, pas de dépendances — exécutables en parallèle
- [Story] label = lien vers la user story pour traçabilité
- **Constitution VI**: Pages publiques = Tailwind CSS v4 pur. AUCUNE classe daisyUI (btn, card, modal, etc.)
- **Constitution VII**: Toute mutation doit appeler `audit::log_action` (7 mutations identifiées)
- **SEO**: `useSeoMeta()` DOIT être dans le `setup()` (pas `onMounted()`) pour le SSR
- **Robots**: `noindex, nofollow` pour les pages non-actives via `useHead()`
- Commit après chaque task ou groupe logique
- S'arrêter à chaque checkpoint pour valider la story indépendamment
