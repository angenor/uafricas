---
description: "Tasks : Refonte salles Afrolang (streaming direct & code secret)"
---

# Tasks: Refonte salles Afrolang : streaming direct & salles privées par code secret

**Input** : Design documents from `/specs/001-afrolang-salles-refonte/`
**Prerequisites** : [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/salles-privees-public-api.md](./contracts/salles-privees-public-api.md), [quickstart.md](./quickstart.md)

**Tests** : aucun framework de test configuré (cf. CLAUDE.md). La validation est manuelle, scénarisée par [quickstart.md](./quickstart.md). Aucune tâche test automatisée n'est générée.

**Organisation** : tâches groupées par user story (US1 → US4) pour livraison incrémentale et test indépendant.

## Format : `[ID] [P?] [Story] Description avec chemin de fichier`

- **[P]** : parallélisable (fichiers différents, aucune dépendance sur tâche en cours)
- **[Story]** : appartenance user story (US1, US2, US3, US4)
- Tous les chemins sont absolus depuis la racine du dépôt

## Path Conventions

- **Backend** : `uafricas_backend/src/...`, `uafricas_backend/doc/bd/schemas/...`
- **Frontend** : `uafricas_frontend/app/...`
- Web app monorepo (cf. plan.md, Constitution Principe II)

---

## Phase 1 : Setup

**Purpose** : aucun nouveau projet à initialiser (refonte d'une feature existante). Cette phase se limite à la préparation de l'environnement de travail.

- [X] T001 Vérifier que la branche `001-afrolang-salles-refonte` est checkée out et à jour : `git status` doit afficher la branche courante ; `git pull origin 001-afrolang-salles-refonte` si elle existe sur le remote
- [ ] T002 Démarrer l'environnement Docker : `docker compose up -d` puis vérifier que `postgres`, `adminer` et `livekit` sont `Up` via `docker compose ps`
- [X] T003 [P] Tuer tout backend résiduel sur le port 8080 : `kill $(lsof -i :8080 -t) 2>/dev/null || true`

---

## Phase 2 : Foundational (Blocking Prerequisites)

**Purpose** : migration BDD table rase et squelette backend/frontend nettoyé. Cette phase DOIT être complète avant d'attaquer toute user story, car les modèles Rust et les composants frontend partagés sont restructurés.

**⚠️ CRITICAL** : aucun travail US ne commence avant la fin de cette phase.

### Migration SQL (table rase legacy)

- [X] T004 Réécrire `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` selon [data-model.md](./data-model.md) : retirer `CREATE TABLE afrolang.salle_privee_adhesion`, `afrolang.proposition_salle` ; retirer les `CREATE TYPE` `motif_salle_privee`, `visibilite_salle_privee`, `type_adhesion`, `etat_adhesion`, `etat_proposition` ; modifier `CREATE TABLE afrolang.salle_privee` (retirer `motif`, `declaration_adulte_at`, `visibilite`, `code_acces` ; ajouter `code_acces_hash CHAR(60) NOT NULL`) ; ajouter `CREATE TABLE afrolang.tentative_code_acces` + ses 2 indexes
- [ ] T005 Réinitialiser la BDD pour appliquer le schéma refondu : `docker compose down -v && docker compose up -d`, attendre ~10 s puis vérifier dans Adminer (`http://localhost:8088`) que `afrolang.salle_privee_adhesion` n'existe plus, que `afrolang.salle_privee` contient `code_acces_hash` et que `afrolang.tentative_code_acces` existe

### Backend : suppression code legacy

- [X] T006 [P] Supprimer `uafricas_backend/src/handlers/admin/propositions_afrolang.rs` (création publique = admin uniquement)
- [X] T007 [P] Supprimer `uafricas_backend/src/models/admin/propositions_afrolang.rs`
- [X] T008 Mettre à jour `uafricas_backend/src/handlers/admin/mod.rs` et `uafricas_backend/src/models/admin/mod.rs` pour retirer les `pub mod propositions_afrolang;`
- [X] T009 Nettoyer `uafricas_backend/src/handlers/admin/salles_privees.rs` : retirer toutes les fonctions liées à adhésion (`lister_adhesions`, `decider_adhesion`, `inviter`), à visibilité (`changer_visibilite`), à modération salle privée (`gerer_moderateur`). Conserver le CRUD admin de base si pertinent (lecture / archivage admin)
- [X] T010 Mettre à jour `uafricas_backend/src/models/admin/salle_privee.rs` : struct `SallePrivee` retire `motif`, `declaration_adulte_at`, `visibilite`, `code_acces` ; ajoute `code_acces_hash: String`
- [X] T011 Mettre à jour `uafricas_backend/src/routes.rs` pour retirer toutes les routes legacy listées dans [contracts/salles-privees-public-api.md](./contracts/salles-privees-public-api.md) section « Endpoints SUPPRIMÉS » (adhesions, inviter, visibilite, propositions-salle, moderateurs salle privée)

### Backend : utilitaires partagés

- [X] T012 [P] Ajouter dans `uafricas_backend/src/handlers/afrolang.rs` les fonctions helper `hasher_code_acces(code: &str) -> Result<String>` et `verifier_code_acces_plain(code: &str, hash: &str) -> Result<bool>` utilisant `bcrypt` cost 10 (cf. [research.md](./research.md) R3)
- [X] T013 [P] Ajouter dans `uafricas_backend/src/handlers/afrolang.rs` la fonction `valider_format_code_acces(code: &str) -> Result<()>` appliquant la regex `^[A-Za-z0-9!@#$%&*?-]{4,16}$` (cf. R2)

### Backend : module rate limit

- [X] T014 Créer `uafricas_backend/src/services/afrolang_rate_limit.rs` exposant `est_verrouillee(pool, salle_privee_id, utilisateur_id) -> Result<bool>` (5 échecs / 60 s + dernière < 5 min) et `enregistrer_tentative(pool, salle_privee_id, utilisateur_id, succes, ip, user_agent) -> Result<()>` (cf. R4) ; déclarer le module dans `uafricas_backend/src/services/mod.rs`

### Backend : token d'accès court

- [X] T015 Étendre `uafricas_backend/src/jwt.rs` (ou créer `uafricas_backend/src/handlers/afrolang_acces_jeton.rs`) avec `creer_acces_jeton(salle_privee_id, utilisateur_id, ttl_secondes) -> String` et `valider_acces_jeton(jeton, salle_privee_id, utilisateur_id) -> Result<()>` (claim `salle_privee_id`, exp 4 h, signature avec `JWT_SECRET` existant)

### Frontend : suppression code legacy

- [X] T016 [P] Supprimer le dossier `uafricas_frontend/app/pages/afrolang/salle-privee/` (FR-006)
- [X] T017 [P] Supprimer `uafricas_frontend/app/pages/afrolang/proposer.vue`
- [X] T018 [P] Supprimer `uafricas_frontend/app/components/afrolang/AnnuaireGroupesEthniques.vue`
- [X] T019 [P] Supprimer `uafricas_frontend/app/components/afrolang/ProposerSalleModal.vue`
- [X] T020 [P] Supprimer `uafricas_frontend/app/components/afrolang/PropositionCard.vue`
- [X] T021 [P] Supprimer `uafricas_frontend/app/components/afrolang/SalleModerationPanel.vue`
- [X] T022 [P] Supprimer `uafricas_frontend/app/components/afrolang/SallePriveeVisibilitePanel.vue`
- [X] T023 [P] Supprimer `uafricas_frontend/app/components/afrolang/DemandeAdhesionCard.vue`
- [X] T024 [P] Supprimer `uafricas_frontend/app/components/afrolang/InvitationBanner.vue`
- [X] T025 Mettre à jour `uafricas_frontend/app/composables/useAfrolang.ts` : retirer les types et helpers `AdhesionSallePriveeAPI`, `listerAdhesions`, `accepterAdhesion`, `refuserAdhesion`, `inviterMembre`, tous les `Proposition*`, `*Moderateur*` (sur salles privées) ; conserver le reste de la surface
- [X] T026 Ajouter middleware Nuxt `uafricas_frontend/app/middleware/afrolang-redirect-legacy.global.ts` : si `to.path` matche `/^/afrolang/salle-privee//` → `navigateTo('/afrolang')` (FR-006, SC-007)

**Checkpoint** : fondation prête (BDD nouveau schéma, code legacy purgé, helpers code secret + rate limit + token disponibles). Implémentation des US peut commencer.

---

## Phase 3 : User Story 1 : Lancer/rejoindre un livestream public en un clic (Priority: P1) 🎯 MVP

**Goal** : depuis `/afrolang`, le clic sur « Démarrer / Rejoindre » d'une salle publique entre directement dans la session live LiveKit. La section « Annuaire des groupes ethniques » disparaît.

**Independent Test** : scénario 1 du quickstart, connecté en `user2`, cliquer sur « Démarrer » sur une carte salle publique → entrée directe dans `/afrolang/session/{salleId}` avec LiveKit chargé. Vérifier en parallèle l'absence de la section annuaire dans le DOM de `/afrolang`.

### Backend US1

- [X] T027 [P] [US1] Vérifier que l'endpoint « démarrer/rejoindre session salle publique » existe et accepte n'importe quel utilisateur connecté (pas seulement `salle.cree_par`) dans `uafricas_backend/src/handlers/afrolang.rs` ; si la logique restreint encore au créateur, retirer la garde et adapter `moderateur_id = utilisateur courant si nouvelle session, sinon inchangé` (FR-005b)
- [X] T028 [US1] S'assurer que `audit::log_action("demarrer_session_salle_publique", …)` et `audit::log_action("rejoindre_session_salle_publique", …)` sont appelés dans le handler de T027 (Constitution Principe VII)

### Frontend US1

- [X] T029 [P] [US1] Modifier `uafricas_frontend/app/pages/afrolang/index.vue` : retirer l'import et l'usage du composant `AnnuaireGroupesEthniques` ainsi que sa section englobante (FR-001)
- [X] T030 [P] [US1] Modifier `uafricas_frontend/app/components/afrolang/SalleCard.vue` : remplacer l'action du bouton « Démarrer / Rejoindre » par `navigateTo(`/afrolang/session/${salle.id}`)` ; libellé dynamique « Rejoindre » si `salle.session_en_cours`, « Démarrer » sinon (FR-002, FR-003)
- [X] T031 [US1] Si la page `uafricas_frontend/app/pages/afrolang/[id].vue` (fiche salle publique) impose un détour avant le live, soit la supprimer, soit y ajouter en `onMounted` `navigateTo(`/afrolang/session/${route.params.id}`, { replace: true })` pour respecter FR-003 (1 navigation max)
- [X] T032 [US1] Vérifier que `uafricas_frontend/app/pages/afrolang/session/[id].vue` (livestream LiveKit) gère correctement le cas « session non existante → démarrage à la volée par n'importe quel utilisateur connecté » : ajuster l'appel composable pour cibler l'endpoint backend de T027

**Checkpoint US1 livré** : MVP fonctionnel, n'importe qui démarre/rejoint un live public en 1 clic, page d'accueil épurée.

---

## Phase 4 : User Story 2 : Créer sa salle privée depuis un live en un modale (Priority: P1)

**Goal** : depuis l'interface de session livestream publique, l'utilisateur ouvre un modale, saisit titre + code secret, valide, et sa salle privée est créée et listée.

**Independent Test** : scénario 2 du quickstart, depuis une session live publique, ouvrir le modale « Créer ma salle privée », saisir titre et code, soumettre, vérifier `POST /api/afrolang/salles-privees → 201` puis présence dans le widget Canal privé.

### Backend US2

- [X] T033 [US2] Implémenter `POST /api/afrolang/salles-privees` dans `uafricas_backend/src/handlers/afrolang.rs` (Endpoint 1 du contrat) : validation `titre` (5-350), `description` (≤1000), `code_acces` (regex via T013), vérification existence et `actif=true` de `salle_id`, vérification non-existence d'une salle privée pour `(salle_id, utilisateur)` non archivée → 409 si conflit avec `data.salle_privee_existante_id` (FR-009, FR-010, SC-005)
- [X] T034 [US2] Dans le handler T033, hasher le code via T012 puis INSERT ; appeler `audit::log_action("creer_salle_privee", …)` (Principe VII)
- [X] T035 [US2] Déclarer la route dans `uafricas_backend/src/routes.rs` (`POST /api/afrolang/salles-privees`)
- [X] T036 [US2] Ajouter dans `uafricas_backend/src/models/afrolang.rs` les DTOs `SallePriveeCreatePayload { salle_id, titre, description, code_acces }` et `SallePriveeAPI { id, salle_id, titre, description, auteur_id, auteur_nom, session_en_cours, est_auteur, created_at }` (NE JAMAIS exposer `code_acces_hash`)

### Frontend US2

- [X] T037 [P] [US2] Refactorer `uafricas_frontend/app/components/afrolang/SallePriveeCreateModal.vue` : champs `titre` (text required 5-350), `code_acces` (text required, attribut `pattern="^[A-Za-z0-9!@#$%&*?-]{4,16}$"`), `description` (textarea optional 0-1000) ; soumission via `useAfrolang().creerSallePrivee()` ; gestion du 409 (afficher message + rediriger vers « Ouvrir ma salle privée » via émission d'event `existante`)
- [X] T038 [US2] Ajouter dans `uafricas_frontend/app/composables/useAfrolang.ts` les types `SallePriveeCreatePayload` / `SallePriveeAPI` alignés sur T036 et la fonction `creerSallePrivee(payload): Promise<SallePriveeAPI>` (POST `$fetch`)
- [X] T039 [US2] Brancher l'ouverture du modale `SallePriveeCreateModal` depuis `uafricas_frontend/app/pages/afrolang/session/[id].vue` via un bouton « Créer ma salle privée » dans la barre de contrôles de la session (FR-008 a)
- [X] T040 [US2] Sur succès création, fermer le modale et afficher un toast « Salle privée créée » avec rappel du code secret saisi (purement UI, pas de re-fetch BDD)

**Checkpoint US2 livré** : un utilisateur en live peut créer sa salle privée en quelques secondes.

---

## Phase 5 : User Story 3 : Accéder à une salle privée par code secret (Priority: P1)

**Goal** : depuis le widget Canal privé sur une carte salle publique, choisir une salle privée, saisir le code secret correct, entrer dans la session live privée. Code incorrect → message clair. Auteur entre sans code.

**Independent Test** : scénarios 4, 5, 6 du quickstart, listing widget, saisie code (correct/incorrect), court-circuit auteur, rate limit après 5 échecs.

### Backend US3

- [X] T041 [US3] Implémenter `GET /api/afrolang/salles/{salle_id}/salles-privees` dans `uafricas_backend/src/handlers/afrolang.rs` (Endpoint 2) : liste filtrée `archivee_at IS NULL AND deleted_at IS NULL`, jointure sur `iam.utilisateur` pour `auteur_nom`, calcul `est_auteur = (cree_par = utilisateur_courant)`, calcul `session_en_cours` via `EXISTS (SELECT 1 FROM afrolang.session WHERE salle_privee_id=$id AND etat='en_cours')` (FR-012)
- [X] T042 [US3] Déclarer la route GET ci-dessus dans `uafricas_backend/src/routes.rs`
- [X] T043 [US3] Implémenter `POST /api/afrolang/salles-privees/{id}/verifier-code` dans `uafricas_backend/src/handlers/afrolang.rs` (Endpoint 3) : (a) charger salle, 404 si introuvable/archivée ; (b) si `cree_par == utilisateur` → 200 avec `acces_jeton` immédiat (FR-014) ; (c) sinon vérifier rate limit via T014 → 429 si verrouillé ; (d) bcrypt verify via T012, INSERT tentative, 403 « Code incorrect » si échec (FR-015), 200 + `acces_jeton` via T015 si succès
- [X] T044 [US3] Déclarer la route POST verifier-code dans `uafricas_backend/src/routes.rs`
- [X] T045 [US3] Audit : appeler `audit::log_action("verifier_code_salle_privee_echec", …)` uniquement sur échec dans T043 (Principe VII, sans saturer sur succès)
- [X] T046 [US3] Implémenter `POST /api/afrolang/salles-privees/{id}/sessions/demarrer-ou-rejoindre` dans `uafricas_backend/src/handlers/afrolang.rs` (Endpoint 4) : exiger header `X-Afrolang-Acces-Jeton`, valider via T015, charger salle (410 si archivée), `SELECT … FOR UPDATE` la session en cours ou INSERT nouvelle (`etat='en_cours'`, `moderateur_id=salle_privee.cree_par`, `cree_par=utilisateur courant`), INSERT `session_participant`, émettre token LiveKit via crate existant (FR-018)
- [X] T047 [US3] Déclarer la route ci-dessus dans `uafricas_backend/src/routes.rs` ; audit `rejoindre_session_salle_privee`

### Frontend US3

- [X] T048 [P] [US3] Ajouter dans `uafricas_frontend/app/composables/useAfrolang.ts` les fonctions `listerSallesPriveesParSallePublique(salleId)`, `verifierCodeAcces(sallePriveeId, code) → { acces_jeton, expires_at }`, `demarrerOuRejoindreSallePrivee(sallePriveeId, accesJeton)` ; mémoriser le `acces_jeton` en mémoire (Map `{ sallePriveeId → jeton }`) pour la session applicative (A2)
- [X] T049 [US3] Refactorer `uafricas_frontend/app/components/afrolang/SallePriveeJoinModal.vue` : champ unique `code_acces` (required) ; sur soumission, appel `verifierCodeAcces` ; si succès, navigation `navigateTo(`/afrolang/session/privee/${id}?jeton=${jeton}`)` (ou stockage mémoire + navigation simple) ; si échec 403 « Code incorrect » affiché ; si 429 « Trop de tentatives, réessayez dans quelques minutes »
- [X] T050 [US3] Modifier `uafricas_frontend/app/components/afrolang/SallePriveeCard.vue` : afficher état (`session_en_cours` badge « En direct ») ; si `est_auteur` → action « Ouvrir ma salle privée » qui court-circuite la saisie de code (appel direct `verifierCodeAcces` qui renvoie le jeton sans saisie) ; sinon « Rejoindre » qui ouvre `SallePriveeJoinModal` (FR-014)
- [X] T051 [US3] Brancher dans le widget Canal privé (dropdown existant sur `SalleCard` ou composant dédié) l'appel à `listerSallesPriveesParSallePublique(salle.id)` au déploiement et le rendu d'une liste de `SallePriveeCard`
- [X] T052 [US3] Adapter `uafricas_frontend/app/pages/afrolang/session/[id].vue` (ou créer une variante `session/privee/[id].vue` selon choix d'architecture) pour appeler `demarrerOuRejoindreSallePrivee` avec le `acces_jeton` récupéré ; gérer 410 (salle archivée) avec message UX et retour `/afrolang`

**Checkpoint US3 livré** : le mécanisme code secret fonctionne bout en bout, l'auteur entre sans code, le rate limit protège.

---

## Phase 6 : User Story 4 : Créer/ouvrir sa salle privée depuis le widget Canal privé (Priority: P2)

**Goal** : depuis le dropdown Canal privé sur une carte salle publique (sans entrer en live), bouton « Créer ma salle privée » ouvre le même modale qu'en live. Si l'utilisateur en a déjà une, le bouton devient « Ouvrir ma salle privée ».

**Independent Test** : scénario 2 du quickstart adapté, depuis `/afrolang` (sans entrer en session), ouvrir le dropdown, cliquer « Créer ma salle privée », vérifier création ; recharger, vérifier que le bouton bascule vers « Ouvrir ma salle privée ».

### Frontend US4

- [X] T053 [US4] Ajouter dans le widget Canal privé (dropdown existant sur `SalleCard.vue`) un bouton conditionnel : si la liste des salles privées contient une entrée avec `est_auteur=true` → « Ouvrir ma salle privée » (action = même que `SallePriveeCard` auteur de T050) ; sinon « Créer ma salle privée » qui ouvre `SallePriveeCreateModal` avec `salle_id` pré-rempli (FR-008 b, US4 acceptance scenarios 1 & 2)
- [X] T054 [P] [US4] Sur fermeture réussie de `SallePriveeCreateModal` depuis ce point d'entrée, rafraîchir la liste du widget (`listerSallesPriveesParSallePublique`) pour faire apparaître la salle nouvellement créée et basculer le bouton vers « Ouvrir ma salle privée »
- [X] T055 [US4] Cohérence visuelle : s'assurer que le widget est bien en Tailwind v4 pur (aucune classe daisyUI `btn`, `card`, `dropdown`, etc.), Constitution Principe VI

**Checkpoint US4 livré** : double point d'entrée création (live + widget) opérationnel.

---

## Phase 7 : Endpoints additionnels (FR-011 modification code, FR-006 archivage par auteur)

**Purpose** : compléter le contrat API avec les endpoints utiles non strictement liés à un parcours d'US (modification du code secret, archivage par l'auteur).

### Backend

- [X] T056 [P] Implémenter `PATCH /api/afrolang/salles-privees/{id}/code-acces` (Endpoint 5) dans `uafricas_backend/src/handlers/afrolang.rs` : auth = auteur (403 sinon), validation `nouveau_code_acces` via T013, hash via T012, UPDATE, audit `modifier_code_salle_privee` (before/after = hashes uniquement, jamais plaintext)
- [X] T057 [P] Implémenter `POST /api/afrolang/salles-privees/{id}/archiver` (Endpoint 6) dans `uafricas_backend/src/handlers/afrolang.rs` : auth = auteur, UPDATE `archivee_at = NOW()`, terminer la session live en cours si présente, audit `archiver_salle_privee`
- [X] T058 Déclarer les routes T056 et T057 dans `uafricas_backend/src/routes.rs`

### Frontend

- [X] T059 [P] Ajouter dans `uafricas_frontend/app/composables/useAfrolang.ts` les fonctions `modifierCodeAcces(id, nouveauCode)` et `archiverSallePriveeParAuteur(id)`
- [X] T060 [P] Ajouter une action discrète « Modifier le code secret » et « Archiver ma salle » sur `SallePriveeCard.vue` quand `est_auteur=true` (modale ou inline minimaliste, Tailwind v4 pur)

---

## Phase 8 : Polish & Cross-Cutting Concerns

- [X] T061 [P] Re-évaluer Constitution Check post-implémentation : confirmer (a) aucun `code_acces` plaintext en BDD ni audit (Principe IV), (b) Tailwind v4 pur sur composants publics (Principe VI), (c) `audit::log_action` présent sur toutes les nouvelles mutations (Principe VII)
- [X] T062 [P] Exécuter le scénario 9 du quickstart (endpoints legacy retournent 404/405) avec `curl` pour verrouiller la suppression effective des routes
- [X] T063 Exécuter l'intégralité de [quickstart.md](./quickstart.md) (scénarios 1 → 10) sur les comptes test `admin@test.com` et `user2@test.com`, et un troisième compte créé pour valider le 409 d'unicité
- [X] T064 [P] Vérifier la compilation backend : `cd uafricas_backend && cargo check` (zéro warning sur le code touché)
- [X] T065 [P] Vérifier la compilation/build frontend : `cd uafricas_frontend && pnpm build` (zéro erreur TypeScript)
- [X] T066 Audit `git status` et `git diff` final : confirmer l'absence de fichier legacy oublié dans `uafricas_backend/src/handlers/admin/`, `uafricas_backend/src/models/admin/`, `uafricas_frontend/app/components/afrolang/`, `uafricas_frontend/app/pages/afrolang/`
- [X] T067 Mettre à jour [CLAUDE.md](../../CLAUDE.md) section « Recent Changes » avec un résumé de la refonte et la suppression des artefacts legacy

---

## Dependencies & Execution Order

```text
Phase 1 (Setup T001→T003)
  ↓
Phase 2 (Foundational T004→T026)
  ├─ T004 → T005 (BDD réécrite puis appliquée)
  ├─ T006-T010 dépendent de T004 (modèles alignés sur nouveau schéma)
  ├─ T011 dépend de T009 (routes retirées après nettoyage handler)
  ├─ T012, T013, T014, T015 [P] entre eux
  └─ T016-T024 [P] entre eux ; T025 dépend de T016-T024 ; T026 indépendant
  ↓
┌─────────────────────────┬─────────────────────────┬─────────────────────────┐
│ Phase 3 : US1 (P1) MVP  │ Phase 4, US2 (P1)      │ Phase 5, US3 (P1)      │
│ T027→T032               │ T033→T040               │ T041→T052               │
│ Indépendante            │ Dépend de T012, T013    │ Dépend de T012, T014,   │
│                         │ (Foundational helpers)  │ T015 (Foundational)     │
└─────────────────────────┴─────────────────────────┴─────────────────────────┘
  ↓
Phase 6 : US4 (P2) T053→T055   (dépend de US2 + US3 frontend modales/composables)
  ↓
Phase 7 : Endpoints additionnels T056→T060   (peuvent démarrer en parallèle de US3 si T012/T013 ok)
  ↓
Phase 8 : Polish T061→T067
```

### Story dependencies

- **US1 ↔ US2/US3/US4** : US1 est indépendante (purement publique). US2 et US3 peuvent être livrées dans n'importe quel ordre l'une vs l'autre (US2 = créer, US3 = accéder), elles sont décorrélées côté backend.
- **US4 dépend fonctionnellement de US2** (réutilise le modale de création) et **de US3** (réutilise la liste/ouverture salle privée auteur).

### Parallel execution opportunities

**Phase 2 : frontend cleanup** : T016 à T024 peuvent être exécutées en parallèle (suppressions de fichiers indépendants).

**Phase 2 : backend helpers** : T012 / T013 / T014 / T015 sont [P] entre elles.

**Phase 3 (US1) en parallèle de Phase 4 (US2)** une fois Foundational terminé : les fichiers touchés sont disjoints (T029-T032 frontend US1 vs T037-T040 frontend US2 ; T027-T028 backend US1 vs T033-T036 backend US2).

**Phase 7 (T056 // T057 // T059 // T060)** : indépendants, parallélisables.

**Phase 8 (T061, T062, T064, T065)** : tous [P].

---

## Implementation Strategy

### MVP scope

**MVP = US1 seule (Phase 3 après Phase 1+2)**. Livre immédiatement :

1. Page `/afrolang` épurée (sans annuaire).
2. Bouton « Démarrer / Rejoindre » qui marche (entrée directe en session LiveKit).

C'est déjà le cœur de la promesse produit selon la spec (US1 = P1 « cœur de la promesse produit »). Les salles privées peuvent suivre dans une seconde livraison incrémentale.

### Incrémental delivery

| Itération | Phases | Valeur livrée |
|---|---|---|
| 1 (MVP) | 1 + 2 + 3 | Streaming public en 1 clic + nettoyage page |
| 2 | + 4 + 5 | Création + accès salle privée par code secret (cœur refonte) |
| 3 | + 6 | Double point d'entrée widget + live |
| 4 | + 7 + 8 | Modification code, archivage par auteur, audit final |

### Risques / points d'attention

- **T005 (reset BDD)** : action destructive locale ; valider que l'environnement Docker n'héberge pas de données utiles avant `down -v`.
- **T031** : décision à clarifier en cours de route, supprimer `/afrolang/[id].vue` ou la garder en redirecteur. Trancher avant l'implémentation.
- **T046 (verrou session)** : utiliser `SELECT … FOR UPDATE` pour éviter la création concurrente de 2 sessions live sur la même salle privée.
- **T049 (mémorisation acces_jeton)** : conserver en mémoire JS (Pinia ou Map du composable), ne PAS écrire dans `localStorage` (le jeton ne doit pas survivre à la fermeture du navigateur, A2).

---

## Récapitulatif

- **Total tâches** : 67 (T001 → T067)
- **Setup** : 3 (T001-T003)
- **Foundational** : 23 (T004-T026)
- **US1** (MVP P1) : 6 tâches (T027-T032)
- **US2** (P1) : 8 tâches (T033-T040)
- **US3** (P1) : 12 tâches (T041-T052)
- **US4** (P2) : 3 tâches (T053-T055)
- **Endpoints additionnels** : 5 (T056-T060)
- **Polish** : 7 (T061-T067)
- **Tâches parallélisables [P]** : 27
