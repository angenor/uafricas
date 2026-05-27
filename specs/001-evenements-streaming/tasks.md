---
description: "Liste de tâches — Événements en streaming direct (LiveKit)"
---

# Tasks: Événements en streaming direct sur la plateforme

**Input**: Documents de conception dans `/specs/001-evenements-streaming/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Aucun framework de test n'est configuré (constitution — Principe « pas de testing »). Aucune tâche de test automatisé n'est générée ; la validation se fait manuellement via `quickstart.md`.

**Organization**: Tâches groupées par user story pour une implémentation et une validation indépendantes.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: parallélisable (fichiers différents, pas de dépendance bloquante)
- **[Story]**: user story rattachée (US1–US4)
- Chemins de fichiers absolus depuis la racine du dépôt

## Path Conventions

Monorepo (Principe II) : backend `uafricas_backend/`, frontend `uafricas_frontend/`.

---

## Phase 1: Setup (Infrastructure partagée)

**Purpose**: Préparer l'environnement et les fichiers squelette pour que le projet compile.

- [X] T001 [P] Vérifier que le service LiveKit dev tourne (`docker compose up -d`) et confirmer la dépendance `livekit-client` (^2.17.1) dans `uafricas_frontend/package.json`
- [X] T002 Créer les fichiers squelette backend `uafricas_backend/src/models/evenement_streaming.rs` et `uafricas_backend/src/handlers/evenement_streaming.rs` (stubs vides) et les déclarer (`pub mod evenement_streaming;`) dans `uafricas_backend/src/models/mod.rs` et `uafricas_backend/src/handlers/mod.rs`

---

## Phase 2: Foundational (Prérequis bloquants)

**Purpose**: Cœur de session + génération de token partagé par TOUTES les user stories.

**⚠️ CRITICAL**: Aucune user story ne peut démarrer avant la fin de cette phase.

- [X] T003 Créer la migration idempotente `uafricas_backend/doc/bd/schemas/09b_media_content_evenements_streaming.sql` : tables `media_content.evenement_session` et `media_content.evenement_session_participant` (colonnes + CHECK `etat`/`role` + `arret_securite_at` + `main_levee`), index unique partiel `uq_evenement_session_active … WHERE etat='en_cours'`, index secondaires (DDL dans `data-model.md`), pattern `CREATE TABLE IF NOT EXISTS` / `ADD COLUMN IF NOT EXISTS`
- [X] T004 Inclure `\ir schemas/09b_media_content_evenements_streaming.sql` dans `uafricas_backend/doc/bd/schema.sql` après les tables `media_content` et avant `12_audit.sql`
- [X] T005 Appliquer la migration à la BDD dev via psql et vérifier les tables/index dans Adminer (`http://localhost:8088`)
- [X] T006 [P] Étendre `uafricas_backend/src/services/livekit_moderation.rs` : ajouter `update_participant_can_publish(cfg, room_name, identity, autorise)` (ParticipantPermission `can_publish=autorise`, `can_subscribe=true`, `can_publish_data=true`) et `retirer_participant(cfg, room_name, identity)` (`RoomClient.remove_participant`), erreurs journalisées non bloquantes
- [X] T007 [P] Implémenter le model `uafricas_backend/src/models/evenement_streaming.rs` : structs `FromRow` (`SessionRow`, `ParticipantRow`), `EVENEMENT_SESSION_COLONNES`, DTO `Serialize` (`EtatDirectResponse`, `TokenDirectResponse`, `DemandeParole`), et calculs purs (`statut_direct`, `fenetre_ouverture_at`, `arret_securite_at`, `grants_pour_role`) — voir mapping cross-stack `data-model.md`
- [X] T008 Implémenter les helpers partagés dans `uafricas_backend/src/handlers/evenement_streaming.rs` : `extraire_utilisateur_id` (réutilise le pattern JWT in-handler), `charger_evenement_diffusable` (format `en_ligne`/`hybride`, `etat='publie'`, sinon 404/422), `est_organisateur` (=`cree_par`), `est_inscrit`, `charger_session_active`, `appliquer_arret_securite` (clôture paresseuse si `NOW() > arret_securite_at` OU événement `annule`), `generer_token` (grants scopés par rôle — D2)
- [X] T009 Implémenter `GET /api/evenements/{id}/direct` dans `uafricas_backend/src/handlers/evenement_streaming.rs` : état dérivé (`statut_direct`, `peut_ouvrir`, `peut_rejoindre`, `nombre_participants`, `fenetre_ouverture_at`), `demandes_parole` si organisateur (cf. `contracts/rest-api.md`)
- [X] T010 Implémenter `POST /api/evenements/{id}/direct/rejoindre` dans `uafricas_backend/src/handlers/evenement_streaming.rs` : open-or-join (crée la session `en_cours` si organisateur + fenêtre OK, sinon rejoint l'active), refus 409 si capacité atteinte (D8), upsert participant (`quitte_at=NULL`, FR-014), réponse token+rôle ; à la **création** : `audit::log_action` action `OUVRIR` + `creer_notification("evenement_direct_demarre")` + `RegistreSse::publier({type:"event_stream_demarre"})` à chaque inscrit
- [X] T011 Implémenter `POST /api/evenements/{id}/direct/quitter` dans `uafricas_backend/src/handlers/evenement_streaming.rs` : `quitte_at=NOW()` + cumul `duree_secondes` (idempotent)
- [X] T012 Enregistrer le sous-scope `/evenements/{id}/direct` (routes `GET`, `rejoindre`, `quitter`) dans `uafricas_backend/src/routes.rs` (import du handler + scope sous `/evenements`)

**Checkpoint**: Cœur de session opérationnel — token généré, jointure/état/sortie testables via curl.

---

## Phase 3: User Story 1 - Assister au direct (Priority: P1) 🎯 MVP

**Goal**: Un membre inscrit rejoint et regarde le direct d'un événement en ligne depuis la page de l'événement.

**Independent Test**: Avec une session déjà ouverte (via API/DB) et un compte inscrit, cliquer « Rejoindre le direct » → la vidéo/audio de l'organisateur est visible/audible ; un non-inscrit/non-connecté est refusé.

- [X] T013 [P] [US1] Étendre `uafricas_frontend/app/composables/useEvenements.ts` : types (`StatutDirect`, `RoleDirect`, `EtatDirect`, `TokenDirect`) + fonctions `obtenirEtatDirect(id)`, `rejoindreDirect(id)`, `quitterDirect(id)`, `gererEvenementStream(evt)` (fetch authentifié, gestion d'erreur, `useState`)
- [X] T014 [P] [US1] Créer `uafricas_frontend/app/components/evenements/EvenementDirectRoom.vue` (Tailwind v4 pur) : connexion `livekit-client` (`room.connect(url, token)`), abonnement tracks distants (`TrackSubscribed`), grille vidéo/audio, **rôle-aware** (spectateur = aucune activation caméra/micro), gestion d'erreur de connexion + repli `lien_en_ligne` (FR-023) avec bouton « Réessayer »
- [X] T015 [US1] Créer la page `uafricas_frontend/app/pages/evenements/[id]/direct.vue` : `onMounted` → `rejoindreDirect` → monte `<EvenementDirectRoom>`, écrans chargement/erreur, redirection `/login` si non connecté (dépend de T013, T014)
- [X] T016 [US1] Ajouter le bouton « Rejoindre le direct » sur `uafricas_frontend/app/pages/evenements/[id].vue` (visible si `statut_direct='en_direct'` + éligible), encart « Inscrivez-vous d'abord » / « Connectez-vous » sinon (dépend de T013)
- [X] T017 [US1] Ajouter la branche `evt.type.startsWith('event_stream_')` dans `uafricas_frontend/app/plugins/messagerie.client.ts` (appelle `gererEvenementStream` + `compteurNonLues`) (dépend de T013)

**Checkpoint**: US1 fonctionnelle — un inscrit peut regarder un direct ouvert (MVP démontrable).

---

## Phase 4: User Story 2 - Animer l'événement (Priority: P1)

**Goal**: L'organisateur ouvre le direct, diffuse caméra/micro/écran, puis clôture.

**Independent Test**: Avec un compte organisateur, ouvrir le direct de son événement, activer caméra/micro/partage d'écran (reçus par un 2ᵉ compte), puis clôturer → la salle se ferme pour tous.

- [X] T018 [US2] Implémenter `POST /api/evenements/{id}/direct/cloturer` dans `uafricas_backend/src/handlers/evenement_streaming.rs` : organisateur uniquement (403), `etat='terminee'` + `termine_at` + `duree_secondes`, `livekit_moderation::fermer_session_admin` (best-effort), `audit::log_action` action `CLOTURER` ; enregistrer la route dans `uafricas_backend/src/routes.rs`
- [X] T019 [US2] Étendre `uafricas_frontend/app/composables/useEvenements.ts` : `cloturerDirect(id)` et `ouvrirDirect(id)` (alias de `rejoindreDirect` côté organisateur)
- [X] T020 [P] [US2] Créer `uafricas_frontend/app/components/evenements/EvenementDirectControls.vue` (Tailwind v4 pur) : contrôles diffuseur (micro / caméra / partage écran / clôturer) conditionnés au rôle `organisateur`/`intervenant`
- [X] T021 [US2] Activer la diffusion dans `uafricas_frontend/app/components/evenements/EvenementDirectRoom.vue` : si rôle diffuseur → `setCameraEnabled`/`setMicrophoneEnabled`/`setScreenShareEnabled`, câbler `<EvenementDirectControls>` et l'action clôturer (dépend de T020, T021↔T014 même fichier)
- [X] T022 [US2] Ajouter le bouton « Ouvrir le direct » sur `uafricas_frontend/app/pages/evenements/[id].vue` (organisateur, `statut_direct='en_attente'`, `peut_ouvrir`) → redirige vers la page direct (dépend de T019)

**Checkpoint**: US1 + US2 fonctionnelles — l'organisateur ouvre/diffuse/clôture, les inscrits regardent.

---

## Phase 5: User Story 3 - Interagir (chat & réactions) (Priority: P2)

**Goal**: Les participants échangent par chat texte et réactions emoji pendant le direct (éphémères).

**Independent Test**: Deux comptes dans un direct ; un message texte / une réaction émis par l'un apparaît chez l'autre < 2 s ; rien n'est conservé après clôture.

- [X] T023 [P] [US3] Créer `uafricas_frontend/app/components/evenements/EvenementDirectChat.vue` (Tailwind v4 pur) : envoi/réception DataPacket `{type:'chat'}` via `publishData`, rendu échappé par Vue (anti-XSS), liste éphémère avec nom d'auteur (cf. `contracts/temps-reel.md`)
- [X] T024 [P] [US3] Créer `uafricas_frontend/app/components/evenements/EvenementDirectReactions.vue` (Tailwind v4 pur) : picker emoji → `publishData {type:'reaction'}` + overlay éphémère (réutilise le pattern `AfrolangReactionsOverlay`)
- [X] T025 [US3] Intégrer chat + réactions dans `uafricas_frontend/app/components/evenements/EvenementDirectRoom.vue` : dispatch `DataReceived` (chat/reaction), ignorer ses propres paquets, monter `<EvenementDirectChat>` et `<EvenementDirectReactions>` (dépend de T023, T024)

**Checkpoint**: US1 + US2 + US3 fonctionnelles — interaction temps réel disponible.

---

## Phase 6: User Story 4 - Donner la parole (lever la main + promotion) (Priority: P3)

**Goal**: Un spectateur lève la main ; l'organisateur voit les demandes, promeut/rétrograde/retire un participant.

**Independent Test**: Dans un direct, le spectateur lève la main → l'organisateur voit la demande, le promeut (caméra/micro activés), le rétrograde, puis retire un perturbateur (déconnecté).

- [X] T026 [US4] Implémenter dans `uafricas_backend/src/handlers/evenement_streaming.rs` les endpoints `POST …/lever-main` (toggle `main_levee` + DataPacket `main_levee`), `POST …/participants/{uid}/promouvoir` (`role='intervenant'`, `main_levee=false`, `update_participant_can_publish(true)`, DataPacket `role_update`), `…/retrograder` (`role='spectateur'`, `update_participant_can_publish(false)`, DataPacket), `…/retirer` (`retirer_participant`, `quitte_at=NOW()`, DataPacket `retire`) ; `audit::log_action` `PROMOUVOIR`/`RETROGRADER`/`RETIRER` ; enregistrer les routes dans `uafricas_backend/src/routes.rs`
- [X] T027 [US4] Étendre `uafricas_frontend/app/composables/useEvenements.ts` : `leverMain(id)`, `promouvoir(id, uid)`, `retrograder(id, uid)`, `retirer(id, uid)`
- [X] T028 [P] [US4] Créer `uafricas_frontend/app/components/evenements/EvenementDirectModerationPanel.vue` (Tailwind v4 pur) : liste des `demandes_parole` + boutons promouvoir/rétrograder/retirer (organisateur uniquement)
- [X] T029 [US4] Ajouter le bouton « Lever la main » (spectateur) dans `uafricas_frontend/app/components/evenements/EvenementDirectControls.vue` + émission/écoute du DataPacket `main_levee` (dépend de T027)
- [X] T030 [US4] Gérer les DataPackets `role_update`/`retire` dans `uafricas_frontend/app/components/evenements/EvenementDirectRoom.vue` (activation des contrôles à la promotion du participant ciblé, déconnexion au retrait) + monter `<EvenementDirectModerationPanel>` pour l'organisateur (dépend de T028)

**Checkpoint**: Les 4 user stories sont indépendamment fonctionnelles.

---

## Phase 7: Polish & Cross-Cutting

**Purpose**: Finitions transverses et validation.

- [X] T031 [P] Étendre `uafricas_frontend/app/mocks/notifications.ts` : type/icône/couleur pour `evenement_direct_demarre` (icône `video`, lien `/evenements/{id}`)
- [X] T032 Vérifier l'audit dans `shared.audit_log` (OUVRIR/CLOTURER/PROMOUVOIR/RETROGRADER/RETIRER sur `media_content.evenement_session`, sans contenu de chat ni média)
- [X] T033 Cascade d'annulation (FR-016) : brancher la clôture forcée de session quand `evenement.etat` passe à `'annule'` dans `uafricas_backend/src/handlers/admin/evenements.rs` (`changer_etat_evenement`) et confirmer la clôture paresseuse à la lecture (T008)
- [X] T034 Mettre à jour `CLAUDE.md` (section Recent Changes) et vérifier la cohérence cross-stack des types (`statut_direct`, `role`) entre SQL ↔ DTO Rust ↔ TS (Principe II)
- [X] T035 Exécuter la validation manuelle `quickstart.md` (scénarios A–F) et corriger les écarts

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance — démarrage immédiat.
- **Foundational (Phase 2)** : dépend du Setup — **BLOQUE** toutes les user stories.
- **User Stories (Phases 3–6)** : dépendent de la fin de Foundational.
  - US1 (P1) est le MVP ; US2 (P1) s'appuie sur le cœur de session foundational.
  - US3 (P2) et US4 (P3) sont des enrichissements de la salle (T014/T021).
- **Polish (Phase 7)** : dépend des user stories visées.

### User Story Dependencies

- **US1 (Assister)** : après Foundational. Indépendante (suppose une session ouverte via API/DB pour le test).
- **US2 (Animer)** : après Foundational. Ajoute la clôture + la diffusion organisateur ; partage `EvenementDirectRoom.vue` avec US1.
- **US3 (Chat/réactions)** : après Foundational ; s'intègre dans `EvenementDirectRoom.vue` (T025 après T014).
- **US4 (Lever main/promotion)** : après Foundational ; s'intègre dans la salle + contrôles (T029 après T020, T030 après T014).

### Within Each User Story

- Backend (endpoints) avant frontend qui les consomme.
- Composable (`useEvenements`) avant page/composants qui l'appellent.
- Composants enfants avant intégration dans `EvenementDirectRoom.vue`.

### Parallel Opportunities

- **Setup** : T001 [P].
- **Foundational** : T006 [P] (livekit_moderation) et T007 [P] (model) en parallèle après T005 ; T008→T012 séquentiels (même fichier handler + routes).
- **US1** : T013 [P] et T014 [P] en parallèle ; puis T015/T016/T017.
- **US2** : T020 [P] en parallèle de T018/T019.
- **US3** : T023 [P] et T024 [P] en parallèle ; puis T025.
- **US4** : T028 [P] en parallèle de T026/T027 ; puis T029/T030.
- **Polish** : T031 [P].

---

## Parallel Example: User Story 1

```bash
# Lancer en parallèle (fichiers différents) :
Task: "T013 Étendre useEvenements.ts (types + fonctions direct)"
Task: "T014 Créer EvenementDirectRoom.vue (connexion LiveKit, spectateur)"
# Puis séquentiel :
Task: "T015 Page evenements/[id]/direct.vue"
Task: "T016 Bouton Rejoindre le direct sur evenements/[id].vue"
Task: "T017 Branche SSE event_stream_ dans messagerie.client.ts"
```

---

## Implementation Strategy

### MVP First (User Stories P1)

1. Phase 1 (Setup) → Phase 2 (Foundational, CRITIQUE).
2. Phase 3 (US1 Assister) → **STOP & VALIDATE** : un inscrit regarde un direct ouvert.
3. Phase 4 (US2 Animer) → l'organisateur ouvre/diffuse/clôture. **MVP complet déployable.**

### Incremental Delivery

1. Foundational prêt → cœur de session testable (curl).
2. + US1 → visionnage (démo).
3. + US2 → animation organisateur (démo).
4. + US3 → chat & réactions (démo).
5. + US4 → modération/parole (démo).
6. Polish → audit, cascade annulation, quickstart.

### Notes

- [P] = fichiers différents, sans dépendance bloquante.
- Pages événements = **Tailwind v4 pur** (Principe VI), aucun daisyUI.
- Aucun média stocké ; chat/réactions/lever-main en DataPackets éphémères.
- `getDiagnostics` (rust-analyzer / Volar) après chaque modification de fichier.
- Commit en français après chaque tâche ou groupe logique.
- Migration BD prod manuelle via SSH+psql (cf. `project_deploy`).
