# Tasks: Demande pour devenir expert avec validation admin

**Input**: Design documents from `/specs/001-demande-expertise/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api.md, quickstart.md

**Tests**: Aucun framework de test configuré dans le projet (Constitution, « pas de linting, testing ni CI/CD »). Aucune tâche de test automatisé générée ; la validation se fait manuellement via `quickstart.md`.

**Organization**: Tâches regroupées par user story pour une livraison incrémentale indépendante.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : peut s'exécuter en parallèle (fichiers différents, pas de dépendance bloquante)
- **[Story]** : US1, US2, US3
- Chemins de fichiers absolus relatifs à la racine du monorepo

## Path Conventions

- Backend : `uafricas_backend/src/`, schémas SQL `uafricas_backend/doc/bd/schemas/`
- Frontend : `uafricas_frontend/app/`

---

## Phase 1: Setup (Infrastructure partagée)

**Purpose**: Préparation minimale : le projet existe déjà, aucune initialisation lourde.

- [X] T001 [P] Ajouter les icônes FontAwesome requises (`faUserTie`, `faHandHoldingHeart`, `faCheck`, `faXmark`, `faClock`) dans `uafricas_frontend/app/plugins/fontawesome.ts` via `library.add()`

---

## Phase 2: Foundational (Prérequis bloquants)

**Purpose**: Migration SQL et permissions, base partagée par US1, US2 et US3 (SQL source de vérité, Principe III).

**⚠️ CRITICAL**: Aucune user story ne peut être finalisée avant la fin de cette phase.

- [X] T002 Modifier `uafricas_backend/doc/bd/schemas/04b_iam_expertise.sql` : retirer la contrainte `UNIQUE` totale sur `utilisateur_id` (garder `NOT NULL`), ajouter l'index unique partiel `CREATE UNIQUE INDEX idx_expertise_utilisateur_actif ON iam.expertise(utilisateur_id) WHERE deleted_at IS NULL`, et ajouter la colonne `commentaire_admin TEXT`
- [X] T003 Ajouter les permissions `expertise.voir` et `expertise.valider` dans `uafricas_backend/doc/bd/schemas/15_seed.sql` (catalogue `iam.permission`)
- [X] T004 Appliquer la migration en dev (`docker compose down -v && docker compose up -d`) ou exécuter le SQL manuel décrit dans `quickstart.md`, puis vérifier la structure via Adminer

**Checkpoint**: Schéma `iam.expertise` migré + permissions seedées, les user stories peuvent démarrer.

---

## Phase 3: User Story 1 - Soumettre une demande pour devenir expert (Priority: P1) 🎯 MVP

**Goal**: Le lien « Apporter mon expertise » mène à un formulaire où un membre connecté complète son profil de base (photo, fonction, pays) et son expertise, puis soumet une demande au statut `en_attente`, invisible publiquement.

**Independent Test**: Cliquer « Apporter mon expertise » → arriver sur `/devenir-expert`, remplir et soumettre → confirmation affichée, demande non visible sur `/experts` ; non connecté → redirection login puis retour au formulaire.

### Backend (US1)

- [X] T005 [US1] Étendre `ModifierProfilRequest` avec `pays_residence_id: Option<Uuid>` dans `uafricas_backend/src/models/utilisateur.rs`
- [X] T006 [US1] Gérer `pays_residence_id` dans `modifier_profil` (SET conditionnel + validation FK `shared.pays`) dans `uafricas_backend/src/handlers/auth.rs`

### Frontend (US1)

- [X] T007 [US1] Remplacer la cible du lien « Apporter mon expertise » de `/experts` vers `/devenir-expert` dans `uafricas_frontend/app/components/layout/BoutonLateralGauche.vue`
- [X] T008 [US1] Étendre `useExperts.ts` : ajouter la mise à jour du profil de base (appel `PUT /api/auth/profil` + `POST /api/auth/profil/photo`) et conserver `creerCandidature` dans `uafricas_frontend/app/composables/useExperts.ts`
- [X] T009 [US1] Créer la page formulaire publique `uafricas_frontend/app/pages/devenir-expert.vue` (Tailwind v4 pur, Principe VI) : champs profil (photo, fonction, pays) + expertise (domaine, biographie, années, situations, portfolio), garde d'authentification avec redirection `?redirect=/devenir-expert`, validation client, message de confirmation, et état « demande déjà en attente »

**Checkpoint**: US1 livrable : un membre peut soumettre une demande complète ; rien n'apparaît encore sur `/experts`.

---

## Phase 4: User Story 2 - Valider ou refuser une demande (administrateur) (Priority: P1)

**Goal**: Un administrateur liste les demandes, ouvre le détail, valide (l'expert devient visible sur `/experts`) ou refuse avec commentaire ; le candidat est notifié par email ; chaque décision est auditée.

**Independent Test**: En admin sur `/admin/experts`, valider une demande → l'expert apparaît sur `/experts` + email reçu ; refuser sans commentaire → bloqué ; refuser avec commentaire → email de refus + jamais visible.

### Backend (US2)

- [X] T010 [P] [US2] Créer les DTO admin dans `uafricas_backend/src/models/admin/expertise.rs` (`AdminDemandeExpertiseRow` FromRow, `AdminDemandeExpertiseResponse`, `AdminDemandeExpertiseListeResponse`, `AdminDemandeExpertiseQueryParams`, `RejeterExpertiseBody`)
- [X] T011 [US2] Déclarer le sous-module `expertise` dans `uafricas_backend/src/models/admin/mod.rs`
- [X] T012 [P] [US2] Ajouter les fonctions d'email `envoyer_email_expertise_validee` et `envoyer_email_expertise_refusee` (+ wrappers async « fire-and-forget ») dans `uafricas_backend/src/email.rs`
- [X] T013 [US2] Créer le handler `uafricas_backend/src/handlers/admin/expertise.rs` : `lister_demandes` (filtres statut/recherche + pagination, `verifier_permission!(admin, "expertise", "voir")`), `obtenir_demande` (détail)
- [X] T014 [US2] Ajouter dans `uafricas_backend/src/handlers/admin/expertise.rs` : `valider_demande` (transaction `statut='valide'`+`valide_par`+`date_validation`, garde `409` si `statut != 'en_attente'`, email async, `audit::log_action("VALIDATE", "iam", "expertise", id)`, `verifier_permission!(admin, "expertise", "valider")`)
- [X] T015 [US2] Ajouter dans `uafricas_backend/src/handlers/admin/expertise.rs` : `rejeter_demande` (validation `commentaire_admin` non vide, transaction `statut='refuse'`+commentaire+décision, garde `409`, email async, `audit::log_action("REJECT", ...)`)
- [X] T016 [US2] Déclarer le sous-module `expertise` dans `uafricas_backend/src/handlers/admin/mod.rs`
- [X] T017 [US2] Enregistrer les 4 routes admin (`GET /api/admin/experts`, `GET /api/admin/experts/{id}`, `PATCH /api/admin/experts/{id}/valider`, `PATCH /api/admin/experts/{id}/rejeter`) dans `uafricas_backend/src/routes.rs`

### Frontend (US2)

- [X] T018 [P] [US2] Créer le composable `uafricas_frontend/app/composables/useAdminExperts.ts` (`listerDemandes` paginé/filtrable, `obtenirDemande`, `validerDemande`, `rejeterDemande`, interfaces `DemandeExpertiseAPI`)
- [X] T019 [US2] Créer la page admin `uafricas_frontend/app/pages/admin/experts/index.vue` (daisyUI) : liste filtrable par statut + recherche + pagination, lignes cliquables vers le détail
- [X] T020 [US2] Créer la page admin `uafricas_frontend/app/pages/admin/experts/[id].vue` (daisyUI) : détail candidat + expertise, boutons Valider / Refuser (modal avec commentaire obligatoire pour le refus), affichage de la décision si déjà traitée

**Checkpoint**: US2 livrable : workflow de modération complet ; un expert validé apparaît sur `/experts`.

---

## Phase 5: User Story 3 - Suivre le statut de sa demande (Priority: P2)

**Goal**: Un membre consulte le statut de sa demande (en attente / validée / refusée), voit le commentaire en cas de refus, et peut soumettre une nouvelle demande corrigée (ancienne archivée).

**Independent Test**: Après refus, l'onglet « Expertise » de `mon-compte/profil` montre le statut + commentaire ; re-soumettre → ancienne demande soft-deletée, nouvelle `en_attente` (vérif BDD : une seule ligne `deleted_at IS NULL`).

### Backend (US3)

- [X] T021 [US3] Ajouter le DTO « ma candidature » (statut, `commentaire_admin`, `date_validation` + champs expertise) dans `uafricas_backend/src/models/expert.rs`
- [X] T022 [US3] Ajouter le handler `ma_candidature` (`GET /api/experts/moi`, JWT, renvoie la candidature active ou `data: null`) dans `uafricas_backend/src/handlers/experts.rs`
- [X] T023 [US3] Modifier `creer_candidature` dans `uafricas_backend/src/handlers/experts.rs` : bloquer `409` uniquement si demande active `en_attente` **ou** `valide` ; si demande active `refuse`, la soft-deleter puis insérer la nouvelle ligne `en_attente` (même transaction), implémente FR-006 + FR-015
- [X] T024 [US3] Enregistrer la route `GET /api/experts/moi` dans `uafricas_backend/src/routes.rs`

### Frontend (US3)

- [X] T025 [US3] Étendre `useExperts.ts` avec `obtenirMaCandidature` (`GET /api/experts/moi`) dans `uafricas_frontend/app/composables/useExperts.ts`
- [X] T026 [US3] Ajouter un onglet « Expertise » dans `uafricas_frontend/app/pages/mon-compte/profil.vue` : badge de statut coloré, commentaire admin si refusé, lien « Soumettre une nouvelle demande » (→ `/devenir-expert`), lien vers la fiche publique si validé

**Checkpoint**: US3 livrable : transparence et re-soumission opérationnelles.

---

## Phase 6: Polish & Cross-Cutting

**Purpose**: Cohérence, finitions, vérifications transverses.

- [X] T027 [P] Vérifier la cohérence cross-stack des enums domaine/situation (labels frontend ↔ `mapper_domaine_db`/`mapper_domaine_frontend` ↔ `iam.domaine_expertise`/`iam.situation_professionnelle`)
- [X] T028 [P] Mettre à jour `CLAUDE.md` (section Recent Changes + ligne « Admin Expertise » dans le tableau des routes API)
- [X] T029 Exécuter l'ensemble des scénarios de `quickstart.md` (US1, US2, US3 + vérifications audit/permissions/Tailwind v4 pur) et corriger les écarts

---

## Dependencies & Execution Order

### Dépendances entre phases

- **Setup (Phase 1)** : indépendant, peut démarrer immédiatement.
- **Foundational (Phase 2)** : dépend de rien ; **bloque la finalisation** de US2 (permissions + `commentaire_admin`) et US3 (index partiel pour re-soumission). T004 dépend de T002+T003.
- **US1 (Phase 3)** : dépend de Phase 2 appliquée (T004) pour les tests réels, mais le code (T005–T009) peut être écrit dès la fin de T002.
- **US2 (Phase 4)** : dépend de Phase 2 (permissions T003, colonne T002). Indépendante de US1.
- **US3 (Phase 5)** : dépend de Phase 2 (index partiel T002). Indépendante de US1/US2 ; partage `useExperts.ts` avec US1 (séquencer T008 puis T025).
- **Polish (Phase 6)** : après les user stories ciblées.

### Dépendances au sein des user stories

- US1 : T005 → T006 (même domaine profil) ; T007/T008/T009 indépendants entre eux sauf T009 qui consomme T008.
- US2 : T010 → T011 ; T010 → T013/T014/T015 ; T013→T014→T015 (même fichier handler, séquentiels) ; T016 après T013–T015 ; T017 après T016 ; frontend T018 → T019/T020.
- US3 : T021 → T022 ; T022/T023 même fichier (séquentiels) ; T024 après T022 ; T025 → T026.

### Ordre de livraison recommandé

1. Phase 1 + Phase 2 (fondation)
2. **US1** (MVP : soumission)
3. **US2** (modération : complète la boucle de valeur exigée)
4. **US3** (suivi & re-soumission)
5. Phase 6 (polish)

---

## Parallel Execution Examples

### Phase 2 (foundational)
- T002 et T003 modifient des fichiers SQL différents → exécutables en parallèle, puis T004.

### US2 : démarrage parallèle
```
# En parallèle (fichiers différents) :
T010 (models/admin/expertise.rs)
T012 (email.rs)
T018 (composables/useAdminExperts.ts)
# Puis séquentiel sur le handler : T013 → T014 → T015 → T016 → T017
# Puis frontend pages : T019, T020 (après T018)
```

### Inter-stories (après Phase 2)
- US1 (T005–T009), US2 (T010–T020) et US3 backend (T021–T024) touchent des fichiers majoritairement distincts → équipes parallèles possibles. Seul `useExperts.ts` est partagé US1/US3 (T008 avant T025).

---

## Implementation Strategy

- **MVP** = Phase 1 + Phase 2 + **US1** : permet déjà de recueillir des candidatures (valeur immédiate), même si la validation se fait temporairement en base.
- **Incrément 2** = **US2** : ferme la boucle exigée (« l'admin devra valider »), rend les experts visibles.
- **Incrément 3** = **US3** : transparence candidat + re-soumission.
- **Finalisation** = Phase 6.
- Respecter à chaque étape : français partout, `audit::log_action` sur mutations admin, requêtes sqlx paramétrées, site public en Tailwind v4 pur / admin en daisyUI.

---

## Récapitulatif

- **Total** : 29 tâches
- **Setup** : 1 (T001) : **Foundational** : 3 (T002–T004)
- **US1 (P1, MVP)** : 5 (T005–T009), **US2 (P1)** : 11 (T010–T020), **US3 (P2)** : 6 (T021–T026)
- **Polish** : 3 (T027–T029)
- **Opportunités parallèles** : T002/T003 ; T010/T012/T018 ; user stories largement indépendantes
