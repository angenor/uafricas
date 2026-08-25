---
description: "Task list : Rendez-vous en visioconférence entre membres amis"
---

# Tasks: Rendez-vous en visioconférence entre membres amis

**Input**: Design documents from `/specs/001-rendez-vous-visio/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/rendez-vous.md, quickstart.md

**Tests**: Aucune tâche de test automatisé, le projet n'a pas de framework de test configuré (Constitution) et la spec ne demande pas de TDD. La validation se fait manuellement via `quickstart.md` (phase Polish).

**Organization**: Tâches groupées par user story (P1→P4) pour une implémentation et une validation incrémentales.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Parallélisable (fichiers différents, sans dépendance bloquante)
- **[Story]**: US1/US2/US3/US4 (référence spec.md)

## Path Conventions

- Backend Rust : `uafricas_backend/src/`, schémas SQL : `uafricas_backend/doc/bd/`
- Frontend Nuxt : `uafricas_frontend/app/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Dépendances et configuration nécessaires à la visioconférence (US4), n'impactent pas le MVP (US1).

- [X] T001 [P] Ajouter la dépendance PeerJS : `cd uafricas_frontend && pnpm add peerjs` (vérifier l'entrée dans `uafricas_frontend/package.json`)
- [X] T002 [P] Déclarer la configuration runtime WebRTC dans `uafricas_frontend/nuxt.config.ts` → `runtimeConfig.public` : `peerjsHost` (vide = cloud public), `peerjsPort` (443), `peerjsPath` ('/'), `peerjsSecure` (true), `iceServers` (défaut `[{ urls: 'stun:stun.l.google.com:19302' }]`), surchargeables via `NUXT_PUBLIC_*`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Schéma SQL, squelettes backend/frontend et plomberie temps réel partagés par toutes les stories.

**⚠️ CRITICAL**: Aucune user story ne peut commencer avant la fin de cette phase.

- [X] T003 Créer le schéma SQL `uafricas_backend/doc/bd/schemas/31_social_rendez_vous.sql` (idempotent) : enum `social.statut_rendez_vous` (`propose`/`accepte`/`refuse`/`annule`), table `social.rendez_vous` (colonnes + contraintes `ck_rdv_pas_soi`, `duree_minutes IN (15,30,45,60)`, `sujet` 1..150, `deleted_at`) et index (`idx_rdv_initiateur`, `idx_rdv_destinataire`, `idx_rdv_tour`, `idx_rdv_date`), conformément à `data-model.md`
- [X] T004 Intégrer la migration à l'orchestrateur : ajouter `\ir schemas/31_social_rendez_vous.sql` dans `uafricas_backend/doc/bd/schema.sql` juste après la ligne `30_social_conversation_annonce.sql` (dépend de T003)
- [X] T005 Appliquer la migration à la BDD dev : `docker compose exec -T postgres psql -U uafricas -d africans_db < uafricas_backend/doc/bd/schemas/31_social_rendez_vous.sql` (ou `docker compose down -v && docker compose up -d`) (dépend de T004)
- [X] T006 [P] Créer le squelette du model `uafricas_backend/src/models/rendez_vous.rs` : struct `RendezVous` (`FromRow`), const `COLONNES`, DTO `RendezVousResponse` (avec `autre: MembreLight`, `mon_tour`, `suis_initiateur`, `etat_derive`, `peut_rejoindre`), helpers de calcul (`etat_derive`, fenêtre `peut_rejoindre` = `[date_heure−5min, date_heure+duree+15min]`), réutilisation de `obtenir_membre_light` (de `models/amitie.rs`) ; déclarer `pub mod rendez_vous;` dans `uafricas_backend/src/models/mod.rs`
- [X] T007 [P] Créer le squelette du composable `uafricas_frontend/app/composables/useRendezVous.ts` : types TS (mapping `data-model.md`), état partagé `useState` (liste, filtre courant), wrapper `$fetch` + en-tête `Authorization: Bearer` (store user), méthodes `lister(filtre, page)` et `obtenir(id)`, stub `gererEvenement(evt)`
- [X] T008 Créer le squelette du handler `uafricas_backend/src/handlers/rendez_vous.rs` : module + helpers réutilisés (`utilisateur_courant`, `amitie_existe`, `blocage_existe`), endpoints lecture `GET /api/rendez-vous` (lister + filtres `attente_moi/attente_autre/a_venir/passes` + pagination) et `GET /api/rendez-vous/{id}` (détail participant) renvoyant `ApiResponse<RendezVousResponse>` ; déclarer `pub mod rendez_vous;` dans `uafricas_backend/src/handlers/mod.rs` (dépend de T006)
- [X] T009 Enregistrer le scope `web::scope("/rendez-vous")` (routes lecture) dans `uafricas_backend/src/routes.rs`, calqué sur le scope `/amities` (dépend de T008)
- [X] T010 Étendre `uafricas_frontend/app/plugins/messagerie.client.ts` : sur réception d'un événement SSE dont `type` commence par `rdv_`, appeler `useRendezVous().gererEvenement(evt)` ET `useNotifications().compteurNonLues()` (rafraîchissement du badge cloche) (dépend de T007)

**Checkpoint**: Schéma en base, lecture des rendez-vous opérationnelle, temps réel branché, les user stories peuvent démarrer.

---

## Phase 3: User Story 1 - Proposer un rendez-vous (Priority: P1) 🎯 MVP

**Goal**: Depuis le profil d'un ami, proposer un entretien vidéo (sujet/description/date/heure/durée) ; le destinataire est notifié (temps réel + cloche).

**Independent Test**: Connecté en A (ami de B), ouvrir `/profil/<id B>`, soumettre un RDV valide → 201, B reçoit la cloche (+1) et un événement temps réel < 5 s ; les cas invalides (date passée, soi-même, sujet/durée manquant, non-ami/bloqué) sont rejetés avec message clair.

### Implementation for User Story 1

- [X] T011 [US1] Implémenter `proposer` (`POST /api/rendez-vous`) dans `uafricas_backend/src/handlers/rendez_vous.rs` : validations (FR-006..FR-010 : pas soi-même, amis+non bloqués, sujet 1..150, durée ∈ {15,30,45,60}, `date_heure > NOW()`), transaction `INSERT` (`statut='propose'`, `tour_id=destinataire_id`), `creer_notification(pool, destinataire, "rdv_propose", message, lien_action)`, push SSE `{type:"rdv_propose", rendez_vous_id}` via `RegistreSse.publier`, `audit::log_action` (`social`/`rendez_vous`, **sans** sujet/description) ; ajouter le constructeur `evt_rdv_propose` dans `models/rendez_vous.rs` et la route dans `routes.rs`
- [X] T012 [P] [US1] Ajouter `proposer(payload)` dans `uafricas_frontend/app/composables/useRendezVous.ts` (POST `/api/rendez-vous`, gestion erreurs FR)
- [X] T013 [P] [US1] Créer `uafricas_frontend/app/components/social/RendezVousProposerModal.vue` (Tailwind v4 pur) : formulaire sujet (obligatoire ≤150), description (option), date+heure (future), durée (boutons 15/30/45/60), validation client + messages d'erreur en français
- [X] T014 [US1] Modifier `uafricas_frontend/app/pages/profil/[id].vue` : ajouter le bouton « Proposer un rendez-vous » dans la carte « Entrer en contact » (visible ssi `etatRelation === 'amis'`), ouvrant `RendezVousProposerModal` et appelant `useRendezVous().proposer` (dépend de T012, T013)

**Checkpoint**: US1 fonctionnelle : proposition + notification opérationnelles.

---

## Phase 4: User Story 2 - Répondre à une proposition (Priority: P2)

**Goal**: Le destinataire (partie dont c'est le tour) peut accepter, refuser ou contre-proposer ; chaque réponse notifie l'autre et, pour la contre-proposition, fait basculer l'initiative.

**Independent Test**: À partir d'une proposition existante, vérifier via l'API/UI : accepter → `accepte` (créneau figé) + initiateur notifié ; refuser → `refuse` + notifié ; contre-proposer → reste `propose`, `tour_id` basculé + notifié ; action hors tour ou contre-proposition sur `accepte` → rejet 409.

### Implementation for User Story 2

- [X] T015 [US2] Implémenter `accepter` (`POST /{id}/accepter`) et `refuser` (`POST /{id}/refuser`) dans `uafricas_backend/src/handlers/rendez_vous.rs` : verrouillage optimiste (`UPDATE … WHERE id=$1 AND statut='propose' AND tour_id=$moi AND deleted_at IS NULL` ; 0 ligne → 409), revérif amitié/blocage, `creer_notification` (`rdv_accepte`/`rdv_refuse`), push SSE, `audit::log_action` ; constructeurs `evt_rdv_accepte`/`evt_rdv_refuse` dans `models/rendez_vous.rs` + routes dans `routes.rs`
- [X] T016 [US2] Implémenter `contre_proposer` (`POST /{id}/contre-proposer`) dans `uafricas_backend/src/handlers/rendez_vous.rs` : validations (`date_heure > NOW()`, durée ∈ {15,30,45,60}), verrouillage optimiste (`statut='propose' AND tour_id=$moi`), `UPDATE` date/heure/durée + bascule `tour_id` vers l'autre (reste `propose`), refus si `accepte` (409, FR-018), `creer_notification("rdv_contre_propose")`, push SSE, audit ; constructeur `evt_rdv_contre_propose` + route
- [X] T017 [P] [US2] Ajouter `accepter(id)`, `refuser(id)`, `contreProposer(id, payload)` dans `uafricas_frontend/app/composables/useRendezVous.ts`
- [X] T018 [P] [US2] Créer `uafricas_frontend/app/components/social/RendezVousCarte.vue` (Tailwind v4 pur) : affiche l'autre membre (photo/nom/fonction/pays via MembreLight), sujet, date/heure, durée, statut + badge `etat_derive` ; calcule les actions disponibles et émet les événements (`accepter`/`refuser`/`contre`/`annuler`/`rejoindre`) ; boutons accepter/refuser/contre visibles ssi `statut==='propose' && mon_tour`
- [X] T019 [US2] Créer `uafricas_frontend/app/components/social/RendezVousContreModal.vue` (date+heure future, durée 15/30/45/60) câblée sur `useRendezVous().contreProposer` (dépend de T017)

**Checkpoint**: US2 fonctionnelle : accepter/refuser/contre-proposer opérationnels (testables via API ; surfacés dans l'UI dès US3).

---

## Phase 5: User Story 3 - Gérer ses rendez-vous (Priority: P3)

**Goal**: Vue listant ses rendez-vous (4 filtres) intégrée au panneau messagerie flottant, avec annulation et accès direct à la messagerie privée.

**Independent Test**: Créer des RDV dans divers états, ouvrir l'onglet « Rendez-vous » du panneau, vérifier les 4 filtres, l'affichage MembreLight, l'annulation (depuis l'une ou l'autre partie → `annule` + autre notifié) et le lien vers la conversation privée.

### Implementation for User Story 3

- [X] T020 [US3] Implémenter `annuler` (`POST /{id}/annuler`) dans `uafricas_backend/src/handlers/rendez_vous.rs` : autorisé à l'un OU l'autre participant si `statut IN ('propose','accepte')` (verrouillage optimiste, 409 sinon), `UPDATE statut='annule'`, `creer_notification("rdv_annule")`, push SSE `evt_rdv_annule`, audit ; constructeur SSE + route
- [X] T021 [P] [US3] Ajouter `annuler(id)` dans `uafricas_frontend/app/composables/useRendezVous.ts` et exposer la gestion des 4 filtres (`attente_moi/attente_autre/a_venir/passes`)
- [X] T022 [US3] Créer `uafricas_frontend/app/components/social/RendezVousListe.vue` (Tailwind v4 pur) : onglets/filtres (4), liste de `RendezVousCarte`, gestion des événements émis (annuler → `useRendezVous().annuler` ; lien messagerie → `useMessagerie().demanderOuverture(autre)`), états vide/chargement (dépend de T018, T021)
- [X] T023 [US3] Ajouter un 3e onglet « Rendez-vous » (avec pastille) dans `uafricas_frontend/app/components/social/MessagerieFlottante.vue` montant `RendezVousListe` aux côtés de « Discussions » et « Membres » (dépend de T022)

**Checkpoint**: US3 fonctionnelle : vue de gestion complète + annulation + lien messagerie.

---

## Phase 6: User Story 4 - Rejoindre la visioconférence (Priority: P4)

**Goal**: Pour un RDV `accepté` dans la fenêtre, rejoindre une salle vidéo P2P (PeerJS) avec flux local/distant, contrôles micro/caméra/quitter, gestion des états et repli messagerie en cas d'échec.

**Independent Test**: Sur un RDV accepté dont l'heure approche, vérifier l'activation du bouton « Rejoindre » dans la fenêtre, l'établissement de la connexion entre deux navigateurs, les contrôles, les états (attente/connecté/parti/échec) et le repli messagerie ; hors fenêtre → bouton inactif et endpoint salle en 409.

### Implementation for User Story 4

- [X] T024 [US4] Implémenter `salle` (`GET /{id}/salle`) dans `uafricas_backend/src/handlers/rendez_vous.rs` : autorise seulement `statut='accepte'` + participant + amis/non bloqués + `NOW() ∈ fenêtre` (sinon 403/409), calcule `mon_peer_id`/`pair_peer_id` = `uafr-{hex(sha256(rendez_vous_id ‖ participant_id))}` (helper dans `models/rendez_vous.rs`, crate `sha2` existante), `suis_appelant = (moi_id < autre_id)`, renvoie aussi l'`autre` (MembreLight) ; route dans `routes.rs`
- [X] T025 [P] [US4] Ajouter `obtenirSalle(id)` dans `uafricas_frontend/app/composables/useRendezVous.ts` (GET `/{id}/salle`)
- [X] T026 [US4] Créer `uafricas_frontend/app/components/social/RendezVousSalle.vue` (Tailwind v4 pur) : initialisation PeerJS depuis `runtimeConfig.public` (host/ice), `getUserMedia` (aperçu local), appel/réception selon `suis_appelant`, affichage flux distant, contrôles micro/caméra/quitter, états (attente que l'autre rejoigne / connexion en cours / connecté / l'autre a quitté / échec) avec messages clairs et bouton repli « Ouvrir la messagerie » (`useMessagerie().demanderOuverture`) (dépend de T025, T001, T002)
- [X] T027 [US4] Activer le bouton « Rejoindre » dans `uafricas_frontend/app/components/social/RendezVousCarte.vue` selon `peut_rejoindre` (fenêtre) et ouvrir `RendezVousSalle` (modal/overlay) au clic (dépend de T018, T026)

**Checkpoint**: Toutes les user stories sont fonctionnelles.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, validation transverse, vérifications de conformité.

- [X] T028 [P] Mettre à jour `CLAUDE.md` (sections « Recent Changes » + « Active Technologies » : feature rendez-vous visio, dépendance `peerjs`, schéma `social.rendez_vous`, endpoints `/api/rendez-vous`)
- [X] T029 Exécuter la validation manuelle de `specs/001-rendez-vous-visio/quickstart.md` (parcours US1→US4)
- [X] T030 Vérifier la conformité : aucune `sujet`/`description` dans `shared.audit_log` pour `social/rendez_vous` (FR-033) et confirmer le média P2P (DevTools `chrome://webrtc-internals`, SC-005)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : sans dépendance, peut démarrer immédiatement (n'impacte que US4).
- **Foundational (Phase 2)** : T003→T004→T005 (chaîne SQL) ; T006/T007 en parallèle ; T008 dépend de T006 ; T009 dépend de T008 ; T010 dépend de T007. **Bloque toutes les user stories.**
- **User Stories (Phases 3–6)** : dépendent de Foundational. Ordre de priorité P1→P2→P3→P4.
- **Polish (Phase 7)** : après les stories visées.

### User Story Dependencies

- **US1 (P1)** : après Foundational. Indépendante. **MVP.**
- **US2 (P2)** : après Foundational. Backend indépendant (testable via API) ; l'UI des cartes est surfacée par US3.
- **US3 (P3)** : après Foundational ; réutilise `RendezVousCarte` (T018, US2) et l'ouverture de conversation existante.
- **US4 (P4)** : après Foundational + Setup (T001/T002) ; réutilise `RendezVousCarte` (T018, US2).

### Within Each User Story

- Backend (model→handler→route) avant ou en parallèle du composable ; composant UI après le composable ; intégration (profil/panneau) en dernier.

### Parallel Opportunities

- **Setup** : T001, T002 en parallèle.
- **Foundational** : T006 (backend model) ‖ T007 (composable). T010 dès T007.
- **US1** : T012 ‖ T013 (puis T014).
- **US2** : T017 ‖ T018 (backend T015/T016 en parallèle du frontend).
- **US4** : T025 en parallèle du backend T024.

---

## Parallel Example: User Story 1

```bash
# Après T011 (backend proposer), lancer en parallèle :
Task: "T012 [US1] Ajouter proposer() dans useRendezVous.ts"
Task: "T013 [US1] Créer RendezVousProposerModal.vue"
# puis T014 (câblage profil) une fois T012 et T013 terminées
```

---

## Implementation Strategy

### MVP First (User Story 1 uniquement)

1. Phase 2 Foundational (T003–T010) : le Setup (T001/T002) peut être différé (US4).
2. Phase 3 US1 (T011–T014).
3. **STOP & VALIDATE** : proposer un RDV depuis un profil ami, vérifier la cloche du destinataire.
4. Démo/déploiement possible.

### Incremental Delivery

1. Foundational → lecture des RDV opérationnelle.
2. + US1 → proposition + notification (MVP).
3. + US2 → réponse/négociation (testable API).
4. + US3 → vue de gestion complète dans le panneau messagerie (surface US2).
5. + Setup + US4 → visioconférence P2P.

---

## Notes

- [P] = fichiers différents, sans dépendance bloquante.
- Couplage assumé : `RendezVousCarte.vue` (T018, US2) est réutilisé par US3 (T022) et US4 (T027) ; US2 reste testable via l'API avant que l'UI ne soit surfacée par US3.
- Réutilisation maximale du domaine social : `obtenir_membre_light`, `amitie_existe`/`blocage_existe`, `RegistreSse`, `creer_notification`, `demanderOuverture`, pas de réécriture (Principe V).
- Conformité Constitution à chaque mutation : `audit::log_action` sans contenu sensible (FR-033), revérif amitié/blocage (FR-034), Tailwind v4 pur côté membre (Principe VI), snake_case français en SQL/Rust.
- `getDiagnostics` après chaque modification (rust-analyzer / Volar) ; relancer le backend proprement (`kill $(lsof -i :8082 -t)`).
- Commit par tâche ou groupe logique, messages en français.
