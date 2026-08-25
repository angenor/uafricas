---
description: "Tâches d'implémentation, Modération de session Afrolang"
---

# Tasks: Modération de session Afrolang, mise en évidence et permissions tableau blanc

**Input**: Design documents from `/specs/001-session-moderation/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api-rest.md, quickstart.md

**Tests**: Aucun test automatisé (le projet n'a pas de CI/CD configurée, constitution). Validation par les scénarios manuels du quickstart.

**Organization**: Tâches regroupées par user story pour permettre une livraison incrémentale (US1 + US2 = MVP P1, US3 = P2 incrémentale).

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers distincts, sans dépendance sur tâches incomplètes)
- **[Story]** : [US1], [US2], [US3] ; pas de label pour Setup, Foundational, Polish
- Chemins de fichiers absolus depuis la racine du repo

## Path Conventions

Monorepo : `uafricas_backend/` (Rust/Actix-Web) + `uafricas_frontend/` (Nuxt 4). DDL : `uafricas_backend/doc/bd/schemas/`.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Préparation environnement, aucune nouvelle dépendance à ajouter (livekit-api et livekit-client déjà présents).

- [X] T001 Vérifier que Docker (postgres + livekit) est lancé et que les utilisateurs de test mentionnés dans `specs/001-session-moderation/quickstart.md` existent (`admin@test.com`, `user2@test.com`, plus créer `alice@test.com`, `bob@test.com`, `carole@test.com` via inscription)
- [X] T002 Vérifier que la dépendance `livekit-api` est bien dans `uafricas_backend/Cargo.toml` avec les features `services` activées (pour `RoomServiceClient::update_participant` et `send_data`) ; ajouter la feature manquante si nécessaire

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: DDL, modèles, helpers d'autorisation et wrappers LiveKit, tout ce dont les trois stories dépendent.

**CRITIQUE** : aucune user story ne peut commencer tant que cette phase n'est pas complète.

### Schéma SQL (constitution III : SQL source de vérité)

- [X] T003 Ajouter à `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (à la fin du fichier, après la table `salle_administrateur`) la nouvelle table `afrolang.session_permission_tableau_blanc` (PK composite `(session_id, utilisateur_id)`, FK CASCADE sur `session`, 2 indexes), DDL exact dans `specs/001-session-moderation/data-model.md` section 1
- [X] T004 Ajouter dans le même fichier l'`ALTER TABLE afrolang.session ADD COLUMN participant_mis_en_evidence_id UUID NULL, ADD COLUMN mis_en_evidence_par UUID NULL, ADD COLUMN mis_en_evidence_at TIMESTAMPTZ NULL` + contrainte `CHECK ck_session_spotlight_coherent`, DDL exact dans data-model.md section 2
- [X] T005 Appliquer la migration sur la BD Docker locale : `docker compose exec -T postgres psql -U uafricas -d africans_db < uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (ou appliquer manuellement les nouveaux DDL si réexécution non idempotente) ; vérifier via Adminer que `session_permission_tableau_blanc` existe et que `afrolang.session` a les 3 nouvelles colonnes

### Modèles Rust

- [X] T006 [P] Étendre `uafricas_backend/src/models/afrolang.rs` avec les structs : `PermissionTableauBlanc` (FromRow), `PermissionTableauBlancResponse` (DTO avec jointure user), `AccorderPermissionPayload`, `SpotlightInfo`, `MettreEnEvidencePayload`, et l'enum `NiveauModerateur` avec méthode `peut_spotlight()`, signatures exactes dans data-model.md
- [X] T007 [P] Ajouter constante `COLONNES_PERMISSION_TB` dans `uafricas_backend/src/models/afrolang.rs` listant les colonnes pour `SELECT` (suivant le pattern existant `COLONNES`)

### Helpers et services partagés

- [X] T008 Créer la fonction publique `est_moderateur_session(pool: &PgPool, session_id: Uuid, utilisateur_id: Uuid) -> Result<Option<NiveauModerateur>, AppError>` dans `uafricas_backend/src/handlers/afrolang.rs`, logique exacte dans `research.md` section R6 (vérifie : rôle global admin plateforme → admin_salle via `salle_administrateur` actif → moderateur_attitre via `salle_moderateur` actif → createur_salle_privee via `salle_privee.cree_par`)
- [X] T009 Créer le nouveau module `uafricas_backend/src/services/livekit_moderation.rs` exposant deux fonctions : (1) `update_participant_can_publish_data(room_name: &str, identity: &str, autorise: bool) -> Result<(), AppError>` qui wrappe `RoomServiceClient::update_participant` ; (2) `publier_evenement_moderation(room_name: &str, payload: &serde_json::Value) -> Result<(), AppError>` qui wrappe `RoomServiceClient::send_data` en RELIABLE
- [X] T010 Déclarer le nouveau module dans `uafricas_backend/src/services/mod.rs` (`pub mod livekit_moderation;`)

### Frontend foundational

- [X] T011 [P] Étendre `uafricas_frontend/app/composables/useAfrolang.ts` avec les interfaces TypeScript `PermissionTableauBlancAPI`, `SpotlightInfoAPI`, et le type `NiveauModerateur`, signatures exactes dans data-model.md
- [X] T012 [P] Ajouter dans `useAfrolang.ts` un listener sur l'évènement LiveKit `dataReceived` dédié au type `'moderation'` ; brancher deux callbacks vides à compléter dans US1 (`onPermissionUpdate`) et US3 (`onSpotlight`), exposer un état réactif `monNiveauModerateurSession: Ref<NiveauModerateur>`

**Checkpoint** : Foundation prête : DDL en place, modèles Rust et TS définis, helper d'autorisation testable, wrappers LiveKit isolés. US1, US2 et US3 peuvent commencer.

---

## Phase 3: User Story 1 : Contrôler qui peut écrire sur le tableau blanc (Priority: P1) 🎯 MVP

**Goal** : un modérateur de session (admin plateforme, admin salle, modérateur attitré, ou créateur de salle privée) peut accorder/retirer individuellement le droit d'écrire sur le tableau blanc. L'enforcement est effectif côté SFU LiveKit.

**Independent Test** : exécuter le **Scénario 1** et le **Scénario 2** du quickstart.md (création salle privée + permissions individuelles avec propagation < 2 s + refus serveur des packets non autorisés).

### Backend : Handlers et routes

- [X] T013 [US1] Implémenter dans `uafricas_backend/src/handlers/afrolang.rs` le handler `lister_permissions_tableau_blanc(session_id: Path<Uuid>)` → `GET /api/afrolang/sessions/{id}/permissions-tableau-blanc` (réponse : modérateurs d'office + permissions individuelles + `mon_niveau_moderateur`), contrat exact dans `contracts/api-rest.md`
- [X] T014 [US1] Implémenter dans `uafricas_backend/src/handlers/afrolang.rs` le handler `accorder_permission_tableau_blanc(session_id, body: AccorderPermissionPayload)` → `POST /api/afrolang/sessions/{id}/permissions-tableau-blanc` : (1) vérifier `est_moderateur_session(auteur)`, (2) refuser si cible est déjà modérateur (409), (3) `INSERT ON CONFLICT DO NOTHING`, (4) appeler `livekit_moderation::update_participant_can_publish_data(true)`, (5) `audit::log_action("CREATE", ...)`, (6) `livekit_moderation::publier_evenement_moderation` avec payload `permission_update action=accordee`
- [X] T015 [US1] Implémenter dans `uafricas_backend/src/handlers/afrolang.rs` le handler `retirer_permission_tableau_blanc(session_id, utilisateur_id: Path<Uuid>)` → `DELETE /api/afrolang/sessions/{id}/permissions-tableau-blanc/{user_id}` : (1) vérifier modérateur auteur, (2) **refuser 409 si cible est elle-même modérateur** (FR-013), (3) `DELETE`, (4) `update_participant_can_publish_data(false)`, (5) audit `DELETE`, (6) publier DataPacket `permission_update action=retiree`
- [X] T016 [US1] Enregistrer les 3 routes dans `uafricas_backend/src/routes.rs` (sous `/api/afrolang/sessions/{session_id}`) avec middleware JWT auth

### Backend : Initialisation des modérateurs d'office au démarrage de session

- [X] T017 [US1] Localiser le handler qui démarre une session Afrolang (probablement `demarrer_session` ou équivalent dans `uafricas_backend/src/handlers/afrolang.rs`) et, immédiatement après la création de la session, appeler `update_participant_can_publish_data(false)` pour tous les participants non modérateurs lors de leur jointure (la liste est connue via `est_moderateur_session` à la jointure de chaque participant : voir T018)
- [X] T018 [US1] Localiser le handler qui gère la jointure d'un participant (probablement `rejoindre_session` ou via webhook LiveKit `participant_joined`) et appliquer la logique de permission initiale dans cet ordre : (1) si `est_moderateur_session` retourne `Some(_)` → `can_publish_data=true` ; sinon (2) `SELECT 1 FROM afrolang.session_permission_tableau_blanc WHERE session_id=$1 AND utilisateur_id=$2` : si une ligne existe → `can_publish_data=true` (FR-edge case « permission accordée à un participant absent puis appliquée à la jointure ») ; sinon (3) `can_publish_data=false`. **Note d'exploration préalable** : si le webhook LiveKit n'est pas câblé dans le projet, faire un `grep -r "participant_joined\|participant_left\|webhook" uafricas_backend/src/` et utiliser le handler REST `rejoindre_session` à la place

### Frontend : Composable et UI

- [X] T019 [P] [US1] Étendre `uafricas_frontend/app/composables/useAfrolang.ts` avec les méthodes : `listerPermissionsTableauBlanc(sessionId)`, `accorderPermissionTableauBlanc(sessionId, utilisateurId)`, `retirerPermissionTableauBlanc(sessionId, utilisateurId)` ; appel via `$fetch` sur l'API REST définie dans contracts/api-rest.md
- [X] T020 [P] [US1] Étendre `useAfrolang.ts` avec un état réactif `permissionsTableauBlanc: Ref<PermissionTableauBlancAPI[]>` et `monEcritureAutorisee: Ref<boolean>` (calculé : `monNiveauModerateurSession !== null || permissionsTableauBlanc inclut moi`) ; implémenter le callback `onPermissionUpdate(payload)` (rebranché depuis T012) qui patch localement la liste et recalcule `monEcritureAutorisee`
- [X] T021 [US1] Créer le nouveau composant `uafricas_frontend/app/components/afrolang/SalleModerationPanel.vue` (Tailwind v4 pur, principe VI, pas de daisyUI) : section « Permissions tableau blanc » avec liste des participants + toggle par participant + indicateur "modérateur" non-toggleable + affichage erreur 409 ; le composant n'est rendu que si `monNiveauModerateurSession.value !== null`
- [X] T022 [US1] Adapter `uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue` : ajouter une prop `ecritureAutorisee: boolean` ; quand `false` → désactiver la barre d'outils (CSS `pointer-events-none opacity-50`), afficher un libellé « Lecture seule » en overlay, et passer un message `{action: 'set-readonly', value: true}` à l'iframe Excalidraw via postMessage ; quand `true` → l'inverse (FR-018)
- [X] T023 [US1] Adapter `uafricas_frontend/app/components/afrolang/AfrolangRoom.vue` : (1) appeler `useAfrolang().listerPermissionsTableauBlanc(sessionId)` au mount + au rejoint LiveKit pour récupérer l'état initial (FR-024 nouvelle connexion), (2) intégrer `<SalleModerationPanel>` dans un drawer/sidebar conditionnel (visible uniquement si modérateur), (3) passer `ecritureAutorisee` à `<AfrolangWhiteboard>`

### Nettoyage à la clôture de session (FR-017)

- [X] T023b [US1] Localiser le handler de clôture de session Afrolang (handler qui met `afrolang.session.etat='terminee'` et `termine_at=NOW()`) et y ajouter, **en transaction avec la mise à jour d'état**, `DELETE FROM afrolang.session_permission_tableau_blanc WHERE session_id = $1` afin de réinitialiser explicitement les permissions à la clôture (FR-017, le `ON DELETE CASCADE` ne se déclenche que sur suppression de la ligne `session`, pas sur transition d'état). Ne **pas** appeler `update_participant_can_publish_data` à ce moment (les participants partent eux-mêmes à la clôture).

### Validation manuelle US1

- [ ] T024 [US1] Dérouler manuellement les **Scénarios 1, 2, 3** du `quickstart.md` ; vérifier propagation < 2 s, refus serveur LiveKit côté DevTools (Scénario 2), reconnexion préservée (Scénario 3) ; vérifier aussi dans Adminer la table `session_permission_tableau_blanc` peuplée puis **vidée à la transition `etat='terminee'`** (T023b, FR-017) ainsi que dans le cas de suppression de session (CASCADE)
- [ ] T024b [US1] Test additionnel : couverture C2/edge-case « permission différée » : (1) en tant que modérateur, accorder une permission à un utilisateur qui **n'est pas encore connecté** à la session, (2) faire rejoindre cet utilisateur, (3) vérifier qu'à la jointure ses outils tableau blanc sont **actifs d'emblée** sans nouvelle action du modérateur (FR-edge case)

**Checkpoint** : US1 livré : permissions tableau blanc fonctionnelles en salle publique et privée pour tous les niveaux de modérateurs. Constitue déjà un MVP utilisable.

---

## Phase 4: User Story 2 : Créateur salle privée = admin (Priority: P1)

**Goal** : un utilisateur qui crée une salle privée est automatiquement reconnu comme modérateur de session dans toutes les sessions de cette salle, sans nomination explicite.

**Independent Test** : Scénario 1 du quickstart en partant d'un utilisateur sans aucun rôle global, qui crée une salle privée, vérifier que le panneau apparaît.

> Cette story est largement absorbée par le helper `est_moderateur_session` (branch `createur_salle_privee`) déjà implémenté en Phase 2. Les tâches ci-dessous garantissent que ce cas est correctement testé et que l'UX reste cohérente.

- [X] T025 [US2] Vérifier que la branche `if salle_privee_id.is_some() && salle_privee.cree_par == utilisateur_id` du helper `est_moderateur_session` (T008) retourne `Some(NiveauModerateur::CreateurSallePrivee)`, ajouter le cas dans le helper si oublié
- [X] T026 [US2] Confirmer dans `SalleModerationPanel.vue` (T021) que le libellé du niveau modérateur affiché est différencié pour `createur_salle_privee` (ex. "Créateur de salle") afin que l'UX soit cohérente avec l'admin de salle publique
- [ ] T027 [US2] Valider manuellement les Acceptance Scenarios 1, 2 et 3 de US2 (cf. spec.md US2) : (1) créateur voit le panneau, (2) participant invité (via code d'accès) ne le voit pas, (3) un second utilisateur entré avec le code d'accès n'obtient pas de droits modérateur (helper retourne `None`)

**Checkpoint** : US1 + US2 livrés = MVP complet (P1) déployable.

---

## Phase 5: User Story 3 : Mettre en évidence un intervenant (Priority: P2)

**Goal** : dans une session publique livestreamée, un admin plateforme ou admin de salle peut mettre en évidence un participant (mono-spotlight) ; les modérateurs attitrés n'ont pas ce pouvoir.

**Independent Test** : Scénarios 4, 5, 6, 7 du quickstart.md (spotlight on/off + transfert + masqué en privé + refusé pour modérateur attitré + cascade au départ).

### Backend : Handlers et routes

- [X] T028 [US3] Implémenter dans `uafricas_backend/src/handlers/afrolang.rs` le handler `mettre_en_evidence(session_id, body: MettreEnEvidencePayload)` → `POST /api/afrolang/sessions/{id}/spotlight` : (1) vérifier `est_moderateur_session(auteur).peut_spotlight()` (sinon 403), (2) refuser 422 si `salle_privee_id IS NOT NULL` (FR-027), (3) vérifier cible présente dans `session_participant` avec `quitte_at IS NULL`, (4) `UPDATE session SET participant_mis_en_evidence_id=$cible, mis_en_evidence_par=$auteur, mis_en_evidence_at=NOW()`, (5) audit `UPDATE` avec before/after JSONB, (6) publier DataPacket `moderation.spotlight` avec `SpotlightInfo`
- [X] T029 [US3] Implémenter le handler `retirer_mise_en_evidence(session_id)` → `DELETE /api/afrolang/sessions/{id}/spotlight` : autorisation identique, `UPDATE ... SET ... NULL`, audit, DataPacket `spotlight` payload `null`
- [X] T030 [US3] Enregistrer les 2 routes dans `uafricas_backend/src/routes.rs`
- [X] T031 [US3] Étendre le handler `obtenir_session` (GET `/api/afrolang/sessions/{id}`) pour inclure dans la réponse JSON un champ `spotlight: SpotlightInfo | null` (jointure sur les 3 nouvelles colonnes + `iam.utilisateur` pour le nom/avatar), FR-024
- [X] T032 [US3] Étendre le handler de départ de session (celui qui set `session_participant.quitte_at = NOW()` ou écoute le webhook LiveKit `participant_left`) pour exécuter en transaction `UPDATE afrolang.session SET participant_mis_en_evidence_id=NULL, mis_en_evidence_par=NULL, mis_en_evidence_at=NULL WHERE id=$session_id AND participant_mis_en_evidence_id=$utilisateur_qui_part` ; si la mise à jour affecte 1 ligne → publier DataPacket `spotlight: null` (FR-025) : voir research.md R5

### Frontend : Composable et UI

- [X] T033 [P] [US3] Étendre `uafricas_frontend/app/composables/useAfrolang.ts` avec les méthodes `mettreEnEvidence(sessionId, utilisateurId)` et `retirerMiseEnEvidence(sessionId)` ; ajouter l'état `spotlightActif: Ref<SpotlightInfoAPI | null>` ; implémenter le callback `onSpotlight(payload)` (rebranché depuis T012) qui met à jour `spotlightActif`
- [X] T034 [P] [US3] Au mount de `AfrolangRoom.vue`, après réception du `GET /sessions/{id}` (T031), initialiser `spotlightActif` à partir du champ `spotlight` de la réponse (FR-024)
- [X] T035 [US3] Étendre `uafricas_frontend/app/components/afrolang/SalleModerationPanel.vue` avec une section « Mise en évidence » conditionnelle : **rendue uniquement si** `monNiveauModerateurSession ∈ {admin_plateforme, admin_salle}` **ET** la session est publique (pas de `salle_privee_id`), FR-001b + FR-027 ; liste des participants connectés (depuis `session_participant`) + bouton "Mettre en évidence" par participant + bouton "Désactiver" si une mise en évidence est active ; mise à jour optimiste de l'UI suivie de la confirmation via DataPacket
- [X] T036 [US3] Adapter `uafricas_frontend/app/components/afrolang/AfrolangVideoGrid.vue` : quand `useAfrolang().spotlightActif.value !== null`, basculer en disposition spotlight (tuile cible agrandie au centre, bordure `border-2 border-custom-chocolat`, libellé « En vedette » via FontAwesome `faStar`) ; autres tuiles en miniature en bas ; transition CSS douce (≤ 300 ms) ; quand `spotlightActif === null` → disposition mosaïque par défaut

### Validation manuelle US3

- [ ] T037 [US3] Dérouler manuellement les **Scénarios 4, 5, 6, 7** du quickstart.md : spotlight + transfert + désactivation, masqué en privé (Scénario 5), modérateur attitré sans pouvoir spotlight (Scénario 6), cascade au départ (Scénario 7) ; vérifier dans Adminer que les 3 colonnes de `session` reviennent à NULL après désactivation/clôture

**Checkpoint** : US3 livré : fonctionnalité complète (P1 + P2).

---

## Phase 6: Polish & Cross-Cutting Concerns

- [ ] T038 [P] Vérifier dans `/admin/audit` (frontend admin) que chaque mutation de la feature a généré une ligne d'audit : action (`CREATE`, `DELETE`, `UPDATE`), ressource (`session_permission_tableau_blanc` ou `session`), `before`/`after` JSONB cohérents ; corriger les `audit::log_action` manquants si nécessaire
- [ ] T039 [P] Relire les composants frontend nouveaux/modifiés (`SalleModerationPanel.vue`, `AfrolangWhiteboard.vue`, `AfrolangVideoGrid.vue`, `AfrolangRoom.vue`) pour vérifier l'absence de classes daisyUI (`btn`, `card`, `modal`, etc., principe VI) : ces composants sont du domaine public donc Tailwind v4 pur uniquement
- [X] T040 Mettre à jour `CLAUDE.md` (section « Recent Changes ») avec une entrée résumant la feature : nouvelle table `session_permission_tableau_blanc`, 3 colonnes spotlight sur `session`, 5 endpoints REST, panneau `SalleModerationPanel.vue`, enforcement LiveKit `can_publish_data`
- [ ] T041 Relancer la validation complète des **7 scénarios** du quickstart.md de bout en bout dans une nouvelle session (sanity check), confirmer SC-001 (aucun packet non autorisé propagé), SC-002 (propagation < 2 s, mesurée DevTools Network), SC-006 (contenu existant préservé après retrait)

---

## Dependencies

```
Phase 1 (Setup, T001-T002)
   ↓
Phase 2 (Foundational, T003-T012), BLOQUANT
   ├─ T003 → T004 → T005 (DDL séquentiel)
   ├─ T006, T007 (P, parallèles entre eux après T005)
   ├─ T008 → T009 → T010 (helper puis services)
   └─ T011, T012 (P, frontend foundational, parallèles)
   ↓
Phase 3 (US1, T013-T024) ──┐
   ├─ T013, T014, T015 → T016 (handlers puis routes)             │
   ├─ T017, T018 (joining/start hooks)                            │
   └─ T019, T020 (P) → T021 → T022, T023 → T024                   │
                                                                  │
Phase 4 (US2, T025-T027), peut chevaucher US1 ──────────────────┤
   ├─ T025 (vérif helper)                                         │
   ├─ T026 (libellé UI dans le panneau)                           │
   └─ T027 (validation manuelle)                                  │
                                                                  ▼
Phase 5 (US3, T028-T037), démarrable après Phase 2 mais bénéfice fort à attendre US1 (UI partagée SalleModerationPanel.vue)
   ├─ T028, T029, T030, T031, T032 (backend, en grande partie parallèles entre eux)
   └─ T033, T034 (P) → T035 → T036 → T037
   ↓
Phase 6 (Polish, T038-T041)
```

## Parallel Execution Examples

### Au sein de la Phase 2 (foundational)

Après T005, lancer en parallèle :
- T006 (structs Rust dans models/afrolang.rs)
- T007 (constante COLONNES_PERMISSION_TB)
- T011 (interfaces TS dans useAfrolang.ts)
- T012 (listener moderation dans useAfrolang.ts) *(séquentiel avec T011, même fichier)*

### Au sein de la Phase 3 (US1)

Backend (séquentiel sur le même fichier `afrolang.rs`): T013 → T014 → T015 → T016.
Frontend en parallèle du backend : T019, T020 (séquentiels entre eux, même fichier composable).
T021 dépend de T019/T020, T022 et T023 peuvent ensuite être faits en parallèle.

### Polish

T038, T039 sont totalement indépendants (audit DB vs. relecture composants Vue), à exécuter en parallèle.

## Implementation Strategy

**MVP scope (P1 = US1 + US2)** : Phases 1 → 2 → 3 → 4 → validation.
Livraison utile dès la fin de la Phase 3 (US1) : les sessions sont déjà protégeables. La Phase 4 (US2) ne modifie que la garantie d'identification du créateur de salle privée, elle est très légère.

**Increment P2 (US3)** : Phases 5 → 6. Indépendant du MVP, peut être livré dans une PR séparée ou la même selon préférence.

**Risque principal** : T017/T018 (initialisation des permissions LiveKit au démarrage et à la jointure) nécessitent de bien comprendre le wiring existant des handlers de session Afrolang (feature 005). Réserver un sous-agent d'exploration backend dédié si le code de jointure n'est pas trivial à localiser.

## Validation Checklist

- [x] Toutes les tâches commencent par `- [ ]` (checkbox)
- [x] Chaque tâche a un ID séquentiel `T###`
- [x] Toutes les tâches de phase US ont un label `[US1]`, `[US2]` ou `[US3]`
- [x] Aucun label `[Story]` sur Setup, Foundational et Polish
- [x] Chaque tâche cite un chemin de fichier explicite
- [x] Tâches `[P]` portent sur des fichiers distincts
- [x] Couverture FR : FR-001/FR-001b (T008), FR-010/FR-013 (T015/T017), FR-014/FR-018 (T020/T022), FR-015 (T009/T014/T015), FR-017 (T003 CASCADE + **T023b clôture par état**), FR-edge « permission différée » (T018 + T024b), FR-020-026 (T028/T029/T036), FR-025 (T032), FR-027 (T035), FR-030 (T014/T015/T028/T029)
- [x] Couverture SC : SC-001 (T024/T037/T041), SC-002 (T024/T037), SC-006 (T041)
