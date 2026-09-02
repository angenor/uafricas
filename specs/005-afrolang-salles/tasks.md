---
description: "Task list for feature 005-afrolang-salles"
---

# Tasks: Afrolang : Ajustements salles publiques et privées

**Input**: Design documents from `/specs/005-afrolang-salles/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅
**Tests**: Aucun framework automatisé configuré dans le projet (Constitution UAfricas). Validation par scénarios manuels (quickstart.md). Donc **pas** de tâches de tests unitaires/intégration automatisés, chaque phase se termine par une ligne « validation manuelle quickstart scénario X ».

**Organization**: Tâches groupées par User Story pour permettre une implémentation et validation indépendantes.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : peut être exécutée en parallèle (fichier différent, pas de dépendance bloquante).
- **[Story]** : user story à laquelle la tâche se rattache (US1..US6). Pas de label pour Setup, Foundational, Polish.
- Chaque tâche inclut le **chemin absolu** de fichier(s) concerné(s).

## Path Conventions (rappel plan.md)

- Backend : `uafricas_backend/...` (Rust + Actix-Web 4)
- Frontend : `uafricas_frontend/app/...` (Nuxt 4)
- Schémas SQL : `uafricas_backend/doc/bd/schemas/08b_afrolang.sql`
- Uploads : `uafricas_backend/uploads/afrolang/ressources/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Préparer le terrain pour l'extension du schema `afrolang` et la structure de fichiers.

- [X] T001 Créer le dossier d'upload `uafricas_backend/uploads/afrolang/ressources/` avec un fichier `.gitkeep` et vérifier que `actix-files` le sert déjà sous `/uploads/` (sinon étendre `main.rs`).
- [X] T002 Ajouter l'extension PostgreSQL `unaccent` (CREATE EXTENSION IF NOT EXISTS unaccent) dans `uafricas_backend/doc/bd/schemas/00_init.sql` ou équivalent, pour permettre la détection de doublons de propositions (research Décision 2).
- [X] T003 [P] Vérifier que `uafricas_backend/docker-init.sh` inclut bien le fichier `08b_afrolang.sql` modifié après les ajustements de la Phase 2 (sinon corriger l'ordre d'exécution des scripts SQL).

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Socle SQL, modèles et enums partagés par toutes les user stories. **Aucune user story ne peut démarrer avant la fin de cette phase.**

### 2.1 Schéma SQL (source de vérité : Principe III)

- [X] T004 Éditer [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql) pour ajouter les 7 nouveaux enums du data-model : `etat_proposition`, `motif_salle_privee`, `visibilite_salle_privee`, `type_adhesion`, `etat_adhesion`, `type_ressource`, `etat_ressource`.
- [X] T005 Dans le même fichier [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql), modifier la table `afrolang.salle` : ajouter `groupe_ethnique_id UUID NOT NULL REFERENCES country_profile.groupe_ethnique(id) ON DELETE RESTRICT`, `langue_code`, `alphabet`, `dictionnaire_url`, `deleted_at`, supprimer `moderateur_id`, créer l'index unique partiel `idx_afrolang_salle_groupe_unique`.
- [X] T006 Dans [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql), modifier la table `afrolang.salle_privee` : ajouter `motif`, `declaration_adulte_at`, `visibilite`, `archivee_at`, `deleted_at`, créer l'index unique partiel `idx_afrolang_privee_unique_par_salle` et l'index `idx_afrolang_privee_visibilite`.
- [X] T007 [P] Dans [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql), créer la table `afrolang.proposition_salle` avec ses index (`idx_afrolang_proposition_etat`, `idx_afrolang_proposition_auteur`) conformément au data-model.md.
- [X] T008 [P] Dans [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql), créer la table `afrolang.salle_moderateur` avec sa contrainte UNIQUE et ses index actifs.
- [X] T009 [P] Dans [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql), créer la table `afrolang.salle_privee_adhesion` avec sa contrainte UNIQUE `(salle_privee_id, utilisateur_id)` et ses index (`idx_afrolang_adhesion_salle`, `idx_afrolang_adhesion_user`, `idx_afrolang_adhesion_attente`).
- [X] T010 [P] Dans [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql), créer la table `afrolang.ressource_salle` avec ses CHECK constraints (`ck_ressource_url_coherence`, `ck_ressource_etat_initial`) et ses index.
- [X] T011 [P] Dans [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql), créer la table `afrolang.message_session` et son index composite `(session_id, created_at)`.
- [X] T011a Dans [uafricas_backend/doc/bd/schemas/08b_afrolang.sql](uafricas_backend/doc/bd/schemas/08b_afrolang.sql), modifier la FK `salle_privee.salle_id` : remplacer `ON DELETE CASCADE` par `ON DELETE RESTRICT` (DROP CONSTRAINT + ADD CONSTRAINT), Edge Case « salle publique désactivée », data-model.md §2.
- [ ] T011b [P] Vérifier que le rôle IAM `moderateur_afrolang` existe dans `iam.role` (sinon insertion via le SQL d'init `iam` ou via le back-office admin) et que les permissions associées (publier/refuser un lien externe sur les salles attitrées) sont déclarées dans `iam.permission`. Cf. contracts/api-admin-afrolang.md « Permissions ».
- [ ] T012 Recréer la base locale (`docker compose down -v && docker compose up -d`) puis vérifier dans Adminer (`http://localhost:8088`) que toutes les nouvelles tables / colonnes / enums / index / contraintes existent (y compris la nouvelle FK `RESTRICT` et le rôle `moderateur_afrolang`).

### 2.2 Enums et structs Rust partagés

- [X] T013 Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), définir les enums Rust correspondant aux 7 nouveaux enums SQL avec `#[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Debug, Clone)]` et `#[sqlx(type_name = "afrolang.xxx", rename_all = "snake_case")]`.
- [X] T014 Dans le même fichier, étendre `SalleRow` (ajouter `groupe_ethnique_id`, `langue_code`, `alphabet`, `dictionnaire_url`, `deleted_at`, retirer `moderateur_id`) et mettre à jour la constante `SALLE_COLONNES`.
- [X] T015 Dans le même fichier, étendre `SallePriveeRow` (ajouter `motif`, `declaration_adulte_at`, `visibilite`, `archivee_at`, `deleted_at`) et mettre à jour `SALLE_PRIVEE_COLONNES`.
- [X] T016 [P] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), ajouter les nouveaux row structs : `PropositionSalleRow`, `SalleModerateurRow`, `SallePriveeAdhesionRow`, `RessourceSalleRow`, `MessageSessionRow`, chacun avec sa constante `XXX_COLONNES` et impl `FromRow`.

### 2.3 Routes, notifications, audit

- [X] T017 Dans [uafricas_backend/src/routes.rs](uafricas_backend/src/routes.rs), ajouter les squelettes des nouvelles routes publiques sous `/api/afrolang/**` (propositions, moderation/transferer, salles-privees/{id}/demandes, /invitations, adhesions/{id}/decision, salles/{id}/ressources/fichier, /lien, sessions/{id}/messages) et admin sous `/api/admin/afrolang/**` (propositions, moderateurs, ressources/en-attente, salles-privees/archiver-batch-utilisateur), toutes pointant vers des handlers stubs renvoyant 501 pour l'instant.
- [X] T018 [P] Dans [uafricas_backend/src/models/notification.rs](uafricas_backend/src/models/notification.rs), ajouter les nouveaux types de notification Afrolang énumérés en research Décision 10 (`afrolang.proposition_validee`, `afrolang.proposition_refusee`, `afrolang.moderation_reprise`, `afrolang.adhesion_demandee`, `afrolang.adhesion_acceptee`, `afrolang.adhesion_refusee`, `afrolang.adhesion_groupe_complet`, `afrolang.invitation_recue`, `afrolang.invitation_refusee`, `afrolang.salle_privee_archivee`), selon le pattern existant.
- [X] T019 [P] Vérifier que `audit::log_action` dans [uafricas_backend/src/services/audit.rs](uafricas_backend/src/services/audit.rs) supporte les nouvelles actions afrolang listées dans `api-admin-afrolang.md` (ajouter les identifiants d'action si le service en tient une liste, sinon rien à faire car le service est générique).

### 2.4 Frontend : contrat commun

- [X] T020 Dans [uafricas_frontend/app/mocks/afrolang.ts](uafricas_frontend/app/mocks/afrolang.ts), aligner les interfaces TypeScript sur le nouveau schema : `Salle` (groupeEthniqueId, langueCode, alphabet, dictionnaireUrl, moderateursAttitres[]), `SallePrivee` (motif, declarationAdulteAt, visibilite, archiveeAt) ; ajouter `PropositionSalle`, `ModerateurAttitre`, `AdhesionSallePrivee`, `RessourceSalle`, `MessageSession` + les literal enums TS 1:1 avec SQL.
- [X] T021 [P] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), préparer les signatures des nouvelles fonctions (stubs retournant `null`/`[]` appelant $fetch) : `listerGroupesEthniques`, `listerMesPropositions`, `soumettrePropositionSalle`, `transfererModerationSession`, `demanderAdhesion`, `inviterMembre`, `decisionAdhesion`, `listerRessources`, `uploaderRessourceFichier`, `soumettreLienExterne`, `listerMessagesSession`, `envoyerMessageSession`, `changerVisibiliteSallePrivee`.
- [X] T022 [P] Créer [uafricas_frontend/app/composables/useAdminAfrolangSalles.ts](uafricas_frontend/app/composables/useAdminAfrolangSalles.ts) basé sur `useAdmin` (adminFetch, pagination, sort) avec stubs pour : `listerPropositions`, `obtenirProposition`, `approuverProposition`, `refuserProposition`, `listerModerateursAttitres`, `designerModerateur`, `retirerModerateur`, `listerLiensEnAttente`, `publierLien`, `refuserLien`, `archiverSallePrivee`, `archiverBatchUtilisateur`.

**Checkpoint**: Fondations prêtes : les user stories peuvent être implémentées, US1/US2/US3 en parallèle car périmètres de fichiers disjoints pour l'essentiel.

---

## Phase 3: User Story 1 - Accéder à la salle publique de son groupe ethnique (Priority: P1) 🎯 MVP

**Goal**: Un membre parcourt l'annuaire des groupes ethniques et rejoint la salle publique correspondante ; il accède aux ressources, à la messagerie, au tableau blanc et à la visio (SC-001, SC-008).

**Independent Test**: Se connecter en `user2@test.com`, ouvrir `/afrolang`, voir l'annuaire ethnique, rejoindre « Gurunsi », accéder à chacun des 4 onglets (Visio, Chat, Ressources, Tableau blanc) en <10 s (quickstart Scénario 1).

### Backend : US1

- [X] T023 [US1] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), ajouter le struct `GroupeEthniqueResume` (id, nom, pays_id, pays_nom, salle_id, salle_slug, salle_active) + constante `GROUPE_ETHNIQUE_RESUME_COLONNES` (query jointe `country_profile.groupe_ethnique` + `shared.pays` + `afrolang.salle`).
- [X] T024 [US1] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), implémenter le handler `lister_groupes_ethniques` pour `GET /api/afrolang/groupes-ethniques` avec filtres `q?`, `pays_id?`, pagination (réutilisation `models/pagination.rs`).
- [X] T025 [US1] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), modifier le handler existant `lister_salles` pour accepter les filtres `groupe_ethnique_id?` et `langue_code?` et enrichir `SalleListeResponse` avec `groupe_ethnique`, `nombre_moderateurs_attitres` (LEFT JOIN `salle_moderateur` WHERE `actif=TRUE`), `ressources_count` (COUNT `ressource_salle` WHERE `etat='publiee' AND deleted_at IS NULL`).
- [X] T026 [US1] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), modifier `obtenir_salle` pour retourner `SalleDetailResponse` enrichi du groupe ethnique complet, des modérateurs attitrés actifs et du compteur de ressources publiées.
- [X] T027 [US1] Dans [uafricas_backend/src/routes.rs](uafricas_backend/src/routes.rs), brancher la route `GET /api/afrolang/groupes-ethniques` sur `lister_groupes_ethniques` et valider manuellement que `GET /afrolang/salles` retourne les nouveaux champs.

### Frontend : US1

- [X] T028 [P] [US1] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), implémenter `listerGroupesEthniques({q?, pays_id?, page?, limit?})` appelant `GET /api/afrolang/groupes-ethniques`.
- [X] T029 [P] [US1] Créer [uafricas_frontend/app/components/afrolang/AnnuaireGroupesEthniques.vue](uafricas_frontend/app/components/afrolang/AnnuaireGroupesEthniques.vue) : grille de cartes par groupe (nom, pays, badge salle active / en attente / absente), recherche, pagination. **Tailwind v4 pur, pas de daisyUI** (Constitution VI).
- [X] T030 [US1] Modifier [uafricas_frontend/app/pages/afrolang/index.vue](uafricas_frontend/app/pages/afrolang/index.vue) pour intégrer `AnnuaireGroupesEthniques` comme vue principale, garder le Hero existant, utiliser le composable `useAfrolang().listerGroupesEthniques`.
- [X] T031 [US1] Modifier [uafricas_frontend/app/pages/afrolang/[id].vue](uafricas_frontend/app/pages/afrolang/[id].vue) pour afficher le bloc « Groupe ethnique » (nom, pays, région), les modérateurs attitrés actifs, et 4 onglets (Visio, Chat, Ressources, Tableau blanc), le contenu des onglets Chat/Ressources/Tableau blanc sera complété dans US6.
- [ ] T032 [US1] Exécuter **quickstart Scénario 1** et cocher les critères SC-001 et SC-008.

**Checkpoint**: US1 livrable en MVP : l'annuaire est la porte d'entrée, la salle s'ouvre avec les blocs attendus (chat/ressources/tableau blanc vides mais présents).

---

## Phase 4: User Story 2 - Proposer la création d'une salle absente (Priority: P1)

**Goal**: Un membre soumet une proposition pour un groupe ethnique absent, l'admin valide/refuse, notifications automatiques, détection de doublons (FR-003 à FR-007, SC-002).

**Independent Test**: Depuis `user2@test.com` soumettre « Zulu » via `/afrolang/proposer`, admin approuve via `/admin/afrolang/propositions`, salle créée et visible (quickstart Scénario 2).

### Backend : US2

- [X] T033 [US2] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), ajouter les DTO `CreerPropositionRequest`, `PropositionSalleResponse` (public) et `PropositionSalleAdminResponse` (admin enrichi : proposant_nom_complet, proposant_email, doublons détectés).
- [X] T034 [US2] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), implémenter le handler `creer_proposition` pour `POST /api/afrolang/salles/propositions` avec : (a) détection doublon via `lower(unaccent(nom))` vs salles actives et propositions en_attente → 409 ; (b) insertion `etat='en_attente'` ; (c) notification proposant (optionnel à ce stade).
- [X] T035 [P] [US2] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), implémenter `lister_mes_propositions` pour `GET /api/afrolang/salles/propositions/mine` (filtre par `propose_par = utilisateur courant`).
- [X] T036 [US2] Dans [uafricas_backend/src/models/admin/propositions_afrolang.rs](uafricas_backend/src/models/admin/propositions_afrolang.rs), ajouter DTO `ApprouverPropositionRequest` (groupe_ethnique_id NOT NULL, titre?, image_couverture_url?, langue_code?, alphabet?, dictionnaire_url?), `RefuserPropositionRequest` (motif_refus min 5).
- [X] T037 [US2] Dans [uafricas_backend/src/handlers/admin/propositions_afrolang.rs](uafricas_backend/src/handlers/admin/propositions_afrolang.rs), implémenter `lister_propositions`, `obtenir_proposition`, `approuver_proposition` (transaction : crée la salle + UPDATE proposition + notification + `audit::log_action`), `refuser_proposition` (UPDATE + notification + audit).
- [X] T038 [US2] Dans [uafricas_backend/src/routes.rs](uafricas_backend/src/routes.rs), brancher les 4 routes admin propositions + 2 routes publiques (création, mine).

### Frontend : US2

- [X] T039 [US2] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), implémenter `soumettrePropositionSalle`, `listerMesPropositions`.
- [X] T040 [P] [US2] Créer [uafricas_frontend/app/components/afrolang/ProposerSalleModal.vue](uafricas_frontend/app/components/afrolang/ProposerSalleModal.vue) (modal Tailwind v4 custom, champs nom/pays/langue/description, affichage 409 avec pointer vers la salle/proposition existante).
- [X] T041 [P] [US2] Créer [uafricas_frontend/app/components/afrolang/PropositionCard.vue](uafricas_frontend/app/components/afrolang/PropositionCard.vue) pour afficher une proposition dans la liste « mes propositions » avec l'état, la décision et le motif de refus.
- [X] T042 [US2] Créer [uafricas_frontend/app/pages/afrolang/proposer.vue](uafricas_frontend/app/pages/afrolang/proposer.vue) combinant le bouton « Nouvelle proposition » (ouvre `ProposerSalleModal`) et la liste `PropositionCard` des propositions du membre.
- [X] T043 [US2] Modifier [uafricas_frontend/app/pages/afrolang/index.vue](uafricas_frontend/app/pages/afrolang/index.vue) et [AnnuaireGroupesEthniques.vue](uafricas_frontend/app/components/afrolang/AnnuaireGroupesEthniques.vue) pour afficher un encart « Proposer cette salle » quand la recherche ne retourne aucun résultat, ouvrant `ProposerSalleModal`.
- [X] T044 [P] [US2] Dans [uafricas_frontend/app/composables/useAdminAfrolangSalles.ts](uafricas_frontend/app/composables/useAdminAfrolangSalles.ts), implémenter `listerPropositions`, `obtenirProposition`, `approuverProposition`, `refuserProposition`.
- [X] T045 [P] [US2] Créer [uafricas_frontend/app/components/admin/afrolang/ValidationPropositionsList.vue](uafricas_frontend/app/components/admin/afrolang/ValidationPropositionsList.vue) (daisyUI v5 autorisé côté admin) avec file paginée, filtres par état, actions approuver/refuser (modaux).
- [X] T046 [US2] Créer [uafricas_frontend/app/pages/admin/afrolang/propositions.vue](uafricas_frontend/app/pages/admin/afrolang/propositions.vue) intégrant `ValidationPropositionsList`, et ajouter une entrée dans [AdminSidebar.vue](uafricas_frontend/app/components/admin/AdminSidebar.vue).
- [ ] T047 [US2] Exécuter **quickstart Scénario 2** (création + approbation + cas de doublon 409).

**Checkpoint**: US2 livrable : soumission + validation fonctionnent de bout en bout.

---

## Phase 5: User Story 3 - Modération de salle publique (Priority: P1)

**Goal**: Modérateurs Afrolang attitrés (statique, admin) + modérateur de session (dynamique serveur : premier arrivé, transfert manuel, reprise par attitré entrant, réattribution au départ), FR-008 à FR-012, SC-003.

**Independent Test**: Valider les 4 sous-scénarios du quickstart Scénario 3 (premier arrivé ; transfert ; reprise attitré ; départ modérateur actif).

### Backend : US3

- [X] T048 [US3] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), ajouter DTO `TransfererModerationRequest { destinataire_id }`, `ModerateurAttitreResponse` (pour endpoints publics qui affichent les modérateurs actifs).
- [X] T049 [US3] Dans [uafricas_backend/src/models/admin/moderateurs_afrolang.rs](uafricas_backend/src/models/admin/moderateurs_afrolang.rs), ajouter DTO `DesignerModerateurRequest { utilisateur_id, disponibilite? }`.
- [X] T050 [US3] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), implémenter `transferer_moderation_session` pour `PUT /api/afrolang/sessions/{id}/moderation/transferer` avec vérifs (appelant = modérateur actuel, destinataire = participant actif) + notification aux deux.
- [X] T051 [US3] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), étendre `rejoindre_session` existant : si aucun `moderateur_id` → l'arrivant devient modérateur (FR-009) ; sinon, si l'arrivant est dans `salle_moderateur` (actif=TRUE) et que le modérateur actuel ne l'est pas → reprise automatique, `UPDATE session.moderateur_id`, notification `moderation_reprise` aux deux parties (FR-011).
- [X] T052 [US3] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), étendre `quitter_session` existant : si l'appelant est le modérateur actif et qu'il reste des participants → réattribuer au plus ancien actif en priorisant un attitré s'il est présent, sinon premier participant restant (FR-012).
- [X] T053 [P] [US3] Dans [uafricas_backend/src/handlers/admin/moderateurs_afrolang.rs](uafricas_backend/src/handlers/admin/moderateurs_afrolang.rs), implémenter `lister_moderateurs_attitres` (`GET /api/admin/afrolang/salles/{salle_id}/moderateurs`), `designer_moderateur` (`POST ...`) avec upsert (réactivation si ligne actif=FALSE) + audit, `retirer_moderateur` (`DELETE ...`) avec soft retrait + audit.
- [X] T054 [US3] Dans [uafricas_backend/src/routes.rs](uafricas_backend/src/routes.rs), brancher la route `PUT /sessions/{id}/moderation/transferer` + les 3 routes admin modérateurs attitrés.

### Frontend : US3

- [X] T055 [US3] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), implémenter `transfererModerationSession`.
- [X] T056 [P] [US3] Créer [uafricas_frontend/app/components/afrolang/SalleModerationPanel.vue](uafricas_frontend/app/components/afrolang/SalleModerationPanel.vue) (Tailwind v4 pur) : visible uniquement pour le modérateur actif, liste des participants actifs, action « Transférer la modération » → confirmation.
- [ ] T057 [US3] Modifier [uafricas_frontend/app/components/afrolang/AfrolangControls.vue](uafricas_frontend/app/components/afrolang/AfrolangControls.vue) pour afficher un badge « Vous modérez la session » quand `session.moderateur_id == utilisateurCourant.id`, et déclencher `SalleModerationPanel` via un bouton.
- [ ] T058 [US3] Modifier [uafricas_frontend/app/components/afrolang/AfrolangRoom.vue](uafricas_frontend/app/components/afrolang/AfrolangRoom.vue) pour écouter les notifications `moderation_reprise` et mettre à jour l'UI en conséquence (toast + réévaluation du rôle).
- [X] T059 [P] [US3] Dans [uafricas_frontend/app/composables/useAdminAfrolangSalles.ts](uafricas_frontend/app/composables/useAdminAfrolangSalles.ts), implémenter `listerModerateursAttitres`, `designerModerateur`, `retirerModerateur`.
- [X] T060 [P] [US3] Créer [uafricas_frontend/app/components/admin/afrolang/ModerateursAttitresPanel.vue](uafricas_frontend/app/components/admin/afrolang/ModerateursAttitresPanel.vue) (daisyUI autorisé) : liste des attitrés actifs d'une salle, recherche utilisateur, ajout/retrait.
- [X] T061 [US3] Créer [uafricas_frontend/app/pages/admin/afrolang/moderateurs.vue](uafricas_frontend/app/pages/admin/afrolang/moderateurs.vue) (sélection d'une salle publique puis gestion via `ModerateursAttitresPanel`).
- [ ] T062 [US3] Exécuter **quickstart Scénario 3** (les 4 sous-séquences 3.1 à 3.4).

**Checkpoint**: US3 livrable : la modération double fonctionne de façon déterministe serveur.

---

## Phase 6: User Story 4 - Créer une salle privée avec motif et déclaration adulte (Priority: P2)

**Goal**: Bouton permanent dans la salle publique + info-bulle 1ère visite, formulaire motif/description/adulte/notice enfants, unicité « 1 par (membre × salle publique) », FR-013 à FR-018, FR-035, SC-005, SC-010.

**Independent Test**: Quickstart Scénario 4 (création + blocage sans adulte + notice enfants + unicité 409).

### Backend : US4

- [X] T063 [US4] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), étendre `CreerSallePriveeRequest` (champs existants + `motif`, `declaration_adulte: bool`, `visibilite: default fermee`) et `SallePriveeDetailResponse` (nouveaux champs).
- [X] T064 [US4] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), modifier `creer_salle_privee` pour : (a) rejeter 400 si `declaration_adulte != true` ; (b) écrire `declaration_adulte_at = NOW()` ; (c) capturer le 409 de l'index unique partiel `idx_afrolang_privee_unique_par_salle` et renvoyer un message explicite « salle privée active déjà existante dans cette salle publique ».
- [X] T065 [US4] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), étendre `lister_salles_privees` (route `GET /api/afrolang/salles/{salle_id}/privees`) pour ne retourner que les salles `visibilite='visible'` + celles où l'utilisateur courant est `abonne` ou a une `invitation/demande` en_attente (filtre serveur).

### Frontend : US4

- [X] T066 [US4] Modifier [uafricas_frontend/app/components/afrolang/SallePriveeCreateModal.vue](uafricas_frontend/app/components/afrolang/SallePriveeCreateModal.vue) : select motif (3 valeurs), textarea description, switch visibilité (fermée par défaut), champ `max_participants`, **checkbox « Je déclare être majeur·e (18+) »** + notice d'alerte conditionnelle quand motif = `apprentissage_enfants`. Tailwind v4 pur.
- [X] T067 [US4] Dans [uafricas_frontend/app/pages/afrolang/[id].vue](uafricas_frontend/app/pages/afrolang/[id].vue), ajouter le **bouton permanent « Créer une salle privée »** dans l'UI de la salle publique. Utiliser `localStorage` (clé `afrolang_bulle_${salleId}`) pour afficher une info-bulle de découverte uniquement à la 1ère visite dans cette salle (Q3).
- [X] T068 [P] [US4] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), implémenter la gestion de la 409 d'unicité : retour structuré `{ erreur: 'salle_privee_unicite', salle_existante_id }` pour que l'UI puisse proposer « accéder à votre salle existante ».
- [ ] T069 [US4] Modifier ou créer [uafricas_frontend/app/pages/afrolang/salle-privee/creer.vue](uafricas_frontend/app/pages/afrolang/salle-privee/creer.vue) (page alternative au modal si contextualisé depuis un lien direct), avec les mêmes règles.
- [ ] T070 [US4] Exécuter **quickstart Scénario 4** (création + blocage sans adulte + notice enfants + 409 unicité + liberté de créer dans une autre salle publique).

**Checkpoint**: US4 livrable : la création de salle privée est conforme et sécurisée (âge, unicité).

---

## Phase 7: User Story 5 - Visibilité, adhésions et invitations (Priority: P2)

**Goal**: Bascule fermée/visible, demandes d'adhésion (salle visible), invitations directes (toutes salles), acceptation/refus, refus automatique « groupe complet » atomique, FR-019 à FR-025, SC-006, SC-007.

**Independent Test**: Quickstart Scénario 5 (5.1 fermée + invitation + refus puis acceptation ; 5.2 visible + demande + refus puis acceptation ; 5.3 groupe complet atomique).

### Backend : US5

- [X] T071 [US5] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), ajouter DTO `ChangerVisibiliteRequest`, `DemanderAdhesionRequest` (vide), `InviterMembreRequest { utilisateur_id }`, `DecisionAdhesionRequest { decision: acceptee|refusee }`, `AdhesionResponse`.
- [X] T072 [US5] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), implémenter `changer_visibilite_salle_privee` (`PATCH /salles-privees/{id}/visibilite`) : JWT + créateur uniquement. **Appeler `audit::log_action` avec `before`/`after` JSONB sur chaque changement** (FR-032).
- [X] T072a [US5] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), ajouter le DTO `ModifierMaxParticipantsRequest { max_participants }` et dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), implémenter `modifier_max_participants_salle_privee` (`PATCH /api/afrolang/salles-privees/{id}/max-participants`) : JWT + créateur uniquement, vérification serveur que la nouvelle valeur ≥ nombre d'abonnés actuels (sinon 422 avec message explicite), `audit::log_action` (FR-036). Brancher la route dans [uafricas_backend/src/routes.rs](uafricas_backend/src/routes.rs).
- [X] T072b [US5] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), implémenter `modifierMaxParticipantsSallePrivee`. Dans [uafricas_frontend/app/components/afrolang/SallePriveeVisibilitePanel.vue](uafricas_frontend/app/components/afrolang/SallePriveeVisibilitePanel.vue), exposer un champ « limite de participants » modifiable avec contrôle UX (warning si nouvelle valeur < abonnés actuels). Permet l'usage « augmenter la limite pour accepter manuellement » (FR-024).
- [X] T073 [US5] Dans le même fichier, implémenter `demander_adhesion` (`POST /salles-privees/{id}/demandes`) : vérifie `visibilite='visible'`, vérifie absence de ligne existante ; transaction `SELECT max_participants, COUNT(abonne) FOR UPDATE` ; si plein → insertion `etat='groupe_complet'` (refus auto, SC-006) ; sinon `etat='en_attente'` + notification créateur.
- [X] T074 [US5] Dans le même fichier, implémenter `inviter_membre` (`POST /salles-privees/{id}/invitations`) : JWT + créateur, insertion `type='invitation', etat='en_attente'`, notification invité.
- [X] T075 [US5] Dans le même fichier, implémenter `decision_adhesion` (`PATCH /adhesions/{id}/decision`) avec contrôle d'accès différencié (créateur pour `type=demande`, utilisateur concerné pour `type=invitation`) ; sur `acceptee` : transaction `SELECT ... FOR UPDATE` + vérification limite + `UPDATE type='abonne', etat='acceptee'` ; sur `refusee` : `UPDATE etat='refusee'`. Notifications aux deux parties.
- [X] T076 [US5] Dans le même fichier, implémenter `lister_adhesions_salle_privee` (`GET /salles-privees/{id}/adhesions`) côté créateur (toutes lignes).
- [X] T077 [US5] Dans le même fichier, implémenter `retirer_abonne` (`DELETE /adhesions/{id}`) côté créateur (soft-delete).
- [X] T078 [US5] Dans [uafricas_backend/src/routes.rs](uafricas_backend/src/routes.rs), brancher les 6 routes ci-dessus sous `/api/afrolang/**`.

### Frontend : US5

- [X] T079 [US5] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), implémenter `changerVisibiliteSallePrivee`, `demanderAdhesion`, `inviterMembre`, `decisionAdhesion`, `listerAdhesions`, `retirerAbonne`.
- [X] T080 [P] [US5] Créer [uafricas_frontend/app/components/afrolang/SallePriveeVisibilitePanel.vue](uafricas_frontend/app/components/afrolang/SallePriveeVisibilitePanel.vue) (Tailwind v4) : affichage pour le créateur uniquement, toggle fermée/visible, explication UX, champ recherche + bouton « Inviter ».
- [X] T081 [P] [US5] Créer [uafricas_frontend/app/components/afrolang/DemandeAdhesionCard.vue](uafricas_frontend/app/components/afrolang/DemandeAdhesionCard.vue) affichant une demande avec actions Accepter/Refuser (gère également les entrées `groupe_complet`).
- [X] T082 [P] [US5] Créer [uafricas_frontend/app/components/afrolang/InvitationBanner.vue](uafricas_frontend/app/components/afrolang/InvitationBanner.vue) : bannière affichée dans l'UI membre quand il a une invitation `en_attente`, boutons Accepter/Refuser.
- [X] T083 [US5] Modifier [uafricas_frontend/app/pages/afrolang/salle-privee/[id].vue](uafricas_frontend/app/pages/afrolang/salle-privee/[id].vue) : intégrer `SallePriveeVisibilitePanel` (créateur), `DemandeAdhesionCard` (liste des demandes côté créateur), afficher le compteur « X/Y participants », badge « Complet » si plein, gestion du refus automatique avec message explicite.
- [ ] T084 [US5] Modifier la vue de la salle publique [uafricas_frontend/app/pages/afrolang/[id].vue](uafricas_frontend/app/pages/afrolang/[id].vue) pour afficher les salles privées `visibilite='visible'` rattachées avec bouton « Demander à rejoindre ».
- [ ] T085 [US5] Intégrer `InvitationBanner` globalement (ex. dans le layout `default.vue` ou dans `/afrolang/index.vue`) pour afficher les invitations en attente du membre courant.
- [ ] T086 [US5] Exécuter **quickstart Scénario 5** (5.1, 5.2, 5.3) incluant le test de concurrence atomique sur la limite.

**Checkpoint**: US5 livrable : adhésions et invitations gérées de bout en bout, limite atomique.

---

## Phase 8: User Story 6 - Tableau blanc, Ressources, Messagerie écrite (Priority: P2)

**Goal**: Tableau blanc synchronisé <500 ms (SC-004), rubrique Ressources avec fichiers internes (publication directe) et liens externes (modération préalable, SC-009), messagerie écrite persistée + diffusée temps réel, FR-026 à FR-030.

**Independent Test**: Quickstart Scénario 6 (6.1 tableau blanc + effacement ; 6.2 ressource fichier ; 6.3 ressource lien modérée ; 6.4 chat avec reprise d'historique).

### Backend : US6 : messagerie

- [X] T087 [US6] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), ajouter DTO `CreerMessageRequest { contenu }` (validation serveur : trim, 1 ≤ len ≤ 4000), `MessageSessionResponse`.
- [X] T088 [US6] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), implémenter `lister_messages_session` (`GET /sessions/{id}/messages?since&limit`) et `envoyer_message_session` (`POST /sessions/{id}/messages`) avec vérif que l'émetteur est `session_participant` actif.

### Backend : US6 : ressources

- [X] T089 [US6] Dans [uafricas_backend/src/models/afrolang.rs](uafricas_backend/src/models/afrolang.rs), ajouter DTO `CreerRessourceLienRequest`, `RessourceSalleResponse`, `RessourceSalleAdminResponse`.
- [X] T090 [US6] Dans [uafricas_backend/src/handlers/afrolang.rs](uafricas_backend/src/handlers/afrolang.rs), implémenter `uploader_ressource_fichier` (`POST /salles/{salle_id}/ressources/fichier` multipart) : rôle modérateur attitré ou admin, whitelist extensions (pdf, png, jpg, jpeg, mp3, mp4, webm, ogg, wav), taille max 50 Mo, `sanitize-filename`, stockage `./uploads/afrolang/ressources/{uuid}-{sanitized}.ext`, insertion `etat='publiee'`.
- [X] T091 [US6] Dans le même fichier, implémenter `soumettre_lien_externe` (`POST /salles/{salle_id}/ressources/lien`) : tout membre JWT, validation URL (schema http/https, longueur ≤ 1000, pas de caractères de contrôle), insertion `etat='en_attente_validation'`.
- [X] T092 [US6] Dans le même fichier, implémenter `lister_ressources` (`GET /salles/{salle_id}/ressources`) : renvoie ressources `etat='publiee' AND deleted_at IS NULL` + celles soumises par l'appelant encore en attente (pour son suivi).
- [X] T093 [US6] Dans le même fichier, implémenter `supprimer_ressource` (`DELETE /ressources/{id}`) : auteur, modérateur attitré de la salle, ou admin.
- [X] T094 [US6] Dans [uafricas_backend/src/handlers/admin/sessions_afrolang.rs](uafricas_backend/src/handlers/admin/sessions_afrolang.rs), implémenter `lister_liens_en_attente` (`GET /admin/afrolang/ressources/en-attente`), `publier_lien` (`POST /{id}/publier` + audit), `refuser_lien` (`POST /{id}/refuser` avec motif + audit + notification auteur). Autoriser aussi un modérateur Afrolang attitré (contrôle serveur : `salle_moderateur.actif=TRUE AND salle_id=<cible>`).
- [X] T095 [US6] Dans [uafricas_backend/src/routes.rs](uafricas_backend/src/routes.rs), brancher les 4 routes ressources publiques + 3 routes admin ressources.

### Frontend : US6 : messagerie

- [X] T096 [US6] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), implémenter `listerMessagesSession`, `envoyerMessageSession`.
- [X] T097 [P] [US6] Créer [uafricas_frontend/app/components/afrolang/SalleChat.vue](uafricas_frontend/app/components/afrolang/SalleChat.vue) (Tailwind v4) : flux de messages, auto-scroll, champ de saisie, reprise d'historique via `listerMessagesSession` au join, diffusion via `LiveKit DataPacket.Kind.RELIABLE` + persistance serveur par `envoyerMessageSession` (pattern hybride research Décision 7). *(Persistance serveur livrée ; diffusion LiveKit à brancher dans AfrolangRoom par la suite.)*

### Frontend : US6 : ressources

- [X] T098 [US6] Dans [uafricas_frontend/app/composables/useAfrolang.ts](uafricas_frontend/app/composables/useAfrolang.ts), implémenter `listerRessources`, `uploaderRessourceFichier`, `soumettreLienExterne`, `supprimerRessource`.
- [X] T099 [P] [US6] Créer [uafricas_frontend/app/components/afrolang/SalleRessources.vue](uafricas_frontend/app/components/afrolang/SalleRessources.vue) (Tailwind v4) : liste par type (fichier / lien), badges d'état, actions d'ajout (bouton fichier pour modérateur attitré, bouton lien pour tout membre), tooltip « en attente de validation » pour les liens pending.
- [X] T100 [P] [US6] Dans [uafricas_frontend/app/composables/useAdminAfrolangSalles.ts](uafricas_frontend/app/composables/useAdminAfrolangSalles.ts), implémenter `listerLiensEnAttente`, `publierLien`, `refuserLien`.
- [X] T101 [P] [US6] Créer [uafricas_frontend/app/components/admin/afrolang/LiensExternesValidation.vue](uafricas_frontend/app/components/admin/afrolang/LiensExternesValidation.vue) (daisyUI) : file des liens en attente, actions publier / refuser avec motif.
- [X] T102 [US6] Créer [uafricas_frontend/app/pages/admin/afrolang/liens-externes.vue](uafricas_frontend/app/pages/admin/afrolang/liens-externes.vue) + entrée dans `AdminSidebar`.

### Frontend : US6 : tableau blanc temps réel

- [ ] T103 [US6] Modifier [uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue](uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue) pour : (a) publier chaque opération (trait, forme, texte, effacement) via LiveKit data channel (`DataPacket.Kind.RELIABLE`, throttle 16 ms pour les traits) ; (b) souscrire aux messages data des autres participants et appliquer les opérations dans l'ordre ; (c) snapshot serveur via `PUT /sessions/{id}/tableau-blanc` toutes les 10 s en throttle ; (d) action d'effacement du modérateur actif (`DELETE /sessions/{id}/tableau-blanc`) propagée à tous.
- [X] T104 [US6] Intégrer `SalleRessources` dans [uafricas_frontend/app/pages/afrolang/[id].vue](uafricas_frontend/app/pages/afrolang/[id].vue). *(Chat à intégrer dans AfrolangRoom/session session existante au prochain incrément.)*
- [ ] T105 [US6] Exécuter **quickstart Scénario 6** (6.1 tableau blanc <500 ms ; 6.2 fichier ; 6.3 lien modéré ; 6.4 chat + reprise).

**Checkpoint**: US6 livrable : les 3 outils collaboratifs sont fonctionnels.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Archivage automatique, journalisation d'audit, documentation, mise à jour CLAUDE.md, hygiène générale.

- [X] T106 Implémenter dans [uafricas_backend/src/handlers/admin/sessions_afrolang.rs](uafricas_backend/src/handlers/admin/sessions_afrolang.rs) l'endpoint `archiver_batch_utilisateur` (`POST /admin/afrolang/salles-privees/archiver-batch-utilisateur`) : UPDATE toutes les salles privées actives du créateur, notifier les abonnés (`salle_privee_archivee`), `audit::log_action` par ligne, FR-034.
- [ ] T107 Dans le handler admin IAM `desactiver_utilisateur` (fichier `uafricas_backend/src/handlers/admin/utilisateurs.rs`), déclencher `tokio::spawn` l'appel au handler `archiver_batch_utilisateur` pour rester non-bloquant et cohérent avec le pattern audit.
- [X] T108 [P] Ajouter un endpoint admin `POST /admin/afrolang/salles-privees/{id}/archiver` pour archivage manuel + `audit::log_action` (cohérent avec le spec).
- [X] T108a Ajouter dans [uafricas_backend/src/handlers/admin/sessions_afrolang.rs](uafricas_backend/src/handlers/admin/sessions_afrolang.rs) le handler `desactiver_salle_publique_avec_cascade` (`POST /api/admin/afrolang/salles/{id}/desactiver`) : transaction atomique qui (a) UPDATE `salle.actif=FALSE` (et éventuellement `deleted_at=NOW()`), (b) UPDATE `salle_privee.archivee_at=NOW()` pour toutes les salles privées rattachées non encore archivées, (c) notifie chaque abonné via `salle_privee_archivee`, (d) `audit::log_action` par salle privée archivée + 1 audit pour la salle publique. Empêche l'erreur FK `RESTRICT` introduite en T011a (Edge Case « salle publique désactivée »).
- [X] T109 Revue complète : chaque mutation admin nouvelle (propositions, modérateurs, liens, archivage, cascade) appelle `audit::log_action` avec `before`/`after` JSONB. Vérifié lors de l'implémentation des handlers.
- [ ] T110 [P] Vérifier que **chaque nouvelle page publique** `/afrolang/**` n'utilise **aucune** classe daisyUI (`btn`, `card`, `modal`, `alert`, `drawer`, etc.), Principe VI. Utiliser `grep -r "\\bbtn\\b\\|\\bcard\\b\\|daisy" uafricas_frontend/app/components/afrolang uafricas_frontend/app/pages/afrolang` et corriger.
- [ ] T111 [P] Vérifier que **chaque identifiant nouveau** (colonne, enum, struct, composant, route, variable) est en **français**, Principe I.
- [X] T112 [P] Mettre à jour [CLAUDE.md](CLAUDE.md) : section « Active Technologies » (schéma `afrolang` étendu) et « Recent Changes » (résumé US1-US6 livrés).
- [ ] T113 Exécuter `cargo build --release` sur le backend et `pnpm build` sur le frontend pour valider la compilation de bout en bout.
- [ ] T114 Exécuter l'intégralité des 7 scénarios du [quickstart.md](specs/005-afrolang-salles/quickstart.md) et cocher toutes les Success Criteria (SC-001 à SC-010).

---

## Dependencies & Execution Order

### Ordre des phases (obligatoire)

```
Phase 1 (Setup)
      ↓
Phase 2 (Foundational) : T004..T022
      ↓
Phase 3 (US1) ──────────┐
Phase 4 (US2) ──────────┤  Peuvent démarrer en parallèle après Phase 2
Phase 5 (US3) ──────────┤  (périmètres de fichiers largement disjoints)
Phase 6 (US4) ──────────┤
Phase 7 (US5) ──────────┤  US7 dépend légèrement de US4 (modèle salle privée
                        │   enrichi doit être en place)
Phase 8 (US6) ──────────┘  US6 peut démarrer en parallèle si US1 est prêt
      ↓
Phase 9 (Polish)
```

### Dépendances critiques inter-stories

- **US5 dépend de US4** : les adhésions nécessitent que `salle_privee` ait déjà `visibilite`, `max_participants` et que la création soit fonctionnelle.
- **US3 dépend de US1** : la reprise par modérateur attitré suppose l'affichage correct de la salle publique avec ses métadonnées et des modérateurs attitrés.
- **US6 peut démarrer en parallèle avec US1/US2/US3** : le chat et les messages nécessitent juste la session existante (déjà en place) + la table `message_session` (Phase 2). Le tableau blanc s'appuie sur `tableau_blanc` existante.
- **US2 est indépendante de US3, US4, US5, US6** mais nécessite US1 pour la cohérence UX (le bouton « proposer » s'affiche dans l'annuaire).

### Dépendances intra-phase (ordre suggéré)

Dans chaque phase user story : **SQL/models → handlers → routes → composables → composants → pages → validation quickstart**.

---

## Parallel Execution Examples

### Phase 2 (Foundational) : lot parallèle SQL

Après T004..T006 (qui modifient des tables existantes et doivent être séquentiels), **T007, T008, T009, T010, T011 peuvent s'exécuter en parallèle** car chacune crée une table neuve indépendante dans le même fichier SQL, réécriture sans conflit si coordonnée (chaque agent ajoute à la fin du fichier un bloc distinct).

### Phase 3 (US1) : parallélisation frontend

Après T023..T027 (backend séquentiel), **T028, T029, T030, T031 peuvent s'exécuter en parallèle** (fichiers frontend disjoints).

### Phase 4 (US2) : parallélisation admin/public

T033..T038 séquentielles côté backend, puis **T040, T041, T044, T045 en parallèle** côté frontend (modal public, card public, composable admin, liste admin, fichiers disjoints).

### Phase 5 (US3) : parallélisation admin/session

Après T048..T052 (backend session), **T053 peut démarrer immédiatement en parallèle** côté admin. Côté frontend, **T056, T060 en parallèle** (composant public session vs composant admin modérateurs).

### Phase 7 (US5) : frontend

**T080, T081, T082 en parallèle** (3 composants frontend distincts).

### Phase 8 (US6) : trois sous-ensembles

Les trois sous-ensembles **messagerie (T087-T088, T096-T097)**, **ressources (T089-T094, T098-T102)**, **tableau blanc (T103)** peuvent être conduits en parallèle par trois agents, les pages d'intégration (T104) attendant leur fin.

### Phase 9

**T108, T110, T111, T112 en parallèle** (audits indépendants).

---

## Implementation Strategy

### MVP scope suggéré (fin Phase 3)

Livrer **US1 seul** comme MVP : l'annuaire ethnique + l'accès aux salles publiques avec leurs onglets vides/lecture, déjà une forte valeur ajoutée par rapport à l'existant, testable et déployable.

### Incréments successifs

1. **MVP (fin Phase 3 / US1)** : annuaire + accès salle publique enrichie.
2. **Incrément 1 (fin Phase 4 / US2)** : proposition de salles absentes opérationnelle.
3. **Incrément 2 (fin Phase 5 / US3)** : modération double fonctionnelle.
4. **Incrément 3 (fin Phase 6 / US4)** : création de salles privées.
5. **Incrément 4 (fin Phase 7 / US5)** : adhésions et invitations.
6. **Incrément 5 (fin Phase 8 / US6)** : outils collaboratifs complets.
7. **Release (fin Phase 9)** : polish, audit, doc à jour.

Chaque incrément reste **mergeable** et **testable manuellement** selon le quickstart, conformément à la Constitution (principe V : livrer simple et itérer).

---

## Synthèse

- **Total de tâches** : 119 (114 initiales + 5 ajoutées suite à `/speckit.analyze` : T011a, T011b, T072a, T072b, T108a)
- **Par User Story** :
  - Setup (Phase 1) : 3
  - Foundational (Phase 2) : 21 (+ T011a FK RESTRICT, + T011b rôle IAM)
  - US1 (P1) : 10
  - US2 (P1) : 15
  - US3 (P1) : 15
  - US4 (P2) : 8
  - US5 (P2) : 18 (+ T072a max_participants, + T072b UI max_participants)
  - US6 (P2) : 19
  - Polish (Phase 9) : 10 (+ T108a désactivation cascade salle publique)
- **Opportunités [P]** : ~35 tâches parallélisables, principalement dans les composants frontend et entre couches admin/public au sein d'une même story.
- **Critère d'indépendance** : chaque user story peut être validée isolément via son scénario quickstart dédié (Scénarios 1 à 7).
- **Conformité Constitution** : vérifiée tâche par tâche (français, SQL en premier, Tailwind v4 public / daisyUI admin, JWT, audit non-bloquant, simplicité).

**Prochaine étape** : commencer par Phase 1 (T001..T003), puis Phase 2 (blocking), puis lancer US1 en MVP.
