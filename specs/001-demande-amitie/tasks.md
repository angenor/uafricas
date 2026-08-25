---
description: "Task list : Demande d'amitié & messagerie temps réel"
---

# Tasks: Demande d'amitié entre membres

**Input**: Design documents from `/specs/001-demande-amitie/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api.md, quickstart.md

**Tests**: Aucun framework de test n'est configuré dans le projet (cf. constitution) et la spec ne demande pas de TDD → **aucune tâche de test générée**. Validation manuelle via [quickstart.md](./quickstart.md).

**Organization**: Tâches groupées par user story (US1→US4) pour livraison incrémentale indépendante.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers différents, aucune dépendance bloquante)
- **[Story]** : US1/US2/US3/US4 ; pas de label pour Setup/Foundational/Polish
- Chemins relatifs à la racine du monorepo

## Path Conventions

- Backend : `uafricas_backend/src/`, schéma SQL `uafricas_backend/doc/bd/`
- Frontend : `uafricas_frontend/app/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Squelette de fichiers et déclarations de modules

- [X] T001 Créer les modules backend vides et leurs déclarations : `uafricas_backend/src/models/amitie.rs`, `models/messagerie.rs`, `handlers/amitie.rs`, `handlers/messagerie.rs`, `services/messagerie_sse.rs` + ajouter `pub mod ...;` dans `src/models/mod.rs`, `src/handlers/mod.rs`, `src/services/mod.rs`
- [X] T002 [P] Créer le dossier `uafricas_frontend/app/components/social/` et les composables vides `app/composables/useAmis.ts` et `app/composables/useMessagerie.ts` (signatures exportées, corps à remplir)
- [X] T003 [P] Enregistrer les icônes FontAwesome nécessaires (`faUserPlus`, `faUserCheck`, `faUserClock`, `faUserXmark`, `faComments`, `faPaperPlane`, `faBan`, `faTrash`) dans `uafricas_frontend/app/plugins/fontawesome.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Données et fondations partagées, **bloque toutes les user stories**

**⚠️ CRITICAL**: Aucune user story ne peut démarrer avant la fin de cette phase

- [X] T004 Écrire `uafricas_backend/doc/bd/schemas/29_social.sql` : enums `social.statut_demande_amitie` et `social.type_notification_social` ; tables `demande_amitie`, `amitie`, `blocage`, `conversation`, `message`, `notification` avec PK UUID, FK `iam.utilisateur`, CHECK (`ck_demande_pas_soi`, `ck_amitie_ordre`, `ck_blocage_pas_soi`, `ck_conversation_ordre`, contenu 1..2000), index et index uniques partiels, strictement selon [data-model.md](./data-model.md)
- [X] T005 Ajouter `\ir schemas/29_social.sql` dans `uafricas_backend/doc/bd/schema.sql` après la ligne `\ir schemas/26_notifications.sql` ; recréer le volume Docker en dev (`docker compose down -v && docker compose up -d`) pour appliquer le schéma
- [X] T006 Dans `uafricas_backend/src/models/amitie.rs` : DTO partagé `MembreLight` (champs publics uniquement : id, nom, prenom, slug, photo_url, fonction, pays, jamais d'email/téléphone) + helper `paire_canonique(a, b) -> (min, max)` pour l'ordre canonique des paires (Décision 4)
- [X] T007 Déclarer les scopes de routes vides `/api/amities` et `/api/messagerie` dans `uafricas_backend/src/routes.rs` (configuration prête à recevoir les handlers par story)

**Checkpoint**: Schéma `social` en base, modules câblés, les user stories peuvent démarrer

---

## Phase 3: User Story 1 - Envoyer une demande d'amitié (Priority: P1) 🎯 MVP

**Goal**: Un membre connecté envoie une demande d'amitié depuis `/profil` et `/profil/{id}` ; le destinataire reçoit une notification ; l'état de la relation est reflété.

**Independent Test**: Connecté, ouvrir la fiche d'un autre membre, cliquer « Demander en ami » → bouton « Demande envoyée », ligne `en_attente` en base, notification créée pour le destinataire ; déconnecté → invitation à se connecter.

### Implementation (Backend)

- [X] T008 [P] [US1] Modèle demande d'amitié dans `uafricas_backend/src/models/amitie.rs` : struct `DemandeAmitie` (`FromRow`), mapping enum statut, const `COLONNES`, DTO `DemandeResponse`/`EtatRelationResponse`
- [X] T009 [P] [US1] Modèle notification dans `uafricas_backend/src/models/amitie.rs` : struct `NotificationSociale` + helper d'insertion `creer_notification(tx, destinataire, type, demande_id, acteur)`
- [X] T010 [US1] Handler `creer_demande` (`POST /api/amities/demandes`) dans `uafricas_backend/src/handlers/amitie.rs` : extraction utilisateur courant (JWT), validations FR-002 (pas soi), FR-015 (destinataire actif), FR-003 (pas de doublon/amitié), FR-013 (403 si blocage), rate-limit FR-014 (≤30/24 h → 429), auto-acceptation croisée FR-009 en **transaction**, notification `demande_recue`, `audit::log_action("CREATE","social","demande_amitie",...)`
- [X] T011 [US1] Handler `etat_relation` (`GET /api/amities/etat/{utilisateur_id}`) dans `uafricas_backend/src/handlers/amitie.rs` : renvoie `aucune|demande_envoyee|demande_recue|amis|bloque_par_moi|indisponible` (FR-016)
- [X] T011b [US1] Handler `etats_relation_lot` (`POST /api/amities/etats`) dans `uafricas_backend/src/handlers/amitie.rs` : états relationnels pour une liste d'ids (≤ 50) en **une seule requête**, évite le N+1 sur l'annuaire (FR-016)
- [X] T012 [US1] Brancher les routes US1 (`POST /api/amities/demandes`, `GET /api/amities/etat/{id}`, `POST /api/amities/etats`) dans `uafricas_backend/src/routes.rs`

### Implementation (Frontend)

- [X] T013 [P] [US1] Dans `uafricas_frontend/app/composables/useAmis.ts` : `envoyerDemande(destinataireId)`, `obtenirEtatRelation(utilisateurId)` et `obtenirEtatsRelationLot(ids)` via `$fetch` avec en-tête JWT, gestion des erreurs (409/422/429/403)
- [X] T014 [US1] Composant `uafricas_frontend/app/components/social/BoutonAmitie.vue` (Tailwind v4 pur) : affiche l'état (aucune→« Demander en ami », envoyée→« Demande envoyée » désactivé, reçue→« Répondre », amis→« Amis ») ; clic = `envoyerDemande` ; si non connecté → redirection `/login?redirect=`
- [X] T015 [US1] Intégrer `BoutonAmitie` sur les cartes membres de `uafricas_frontend/app/pages/profil/index.vue` (masqué sur sa propre carte) ; charger les états des cartes visibles en **un seul appel** `obtenirEtatsRelationLot` (anti N+1, FR-016)
- [X] T016 [US1] Intégrer `BoutonAmitie` + chargement de l'état relation sur `uafricas_frontend/app/pages/profil/[id].vue`

**Checkpoint**: US1 fonctionnelle : un membre peut envoyer une demande et voir l'état ; le destinataire a une notification en base

---

## Phase 4: User Story 2 - Répondre à une demande reçue (Priority: P1)

**Goal**: Le destinataire consulte ses demandes reçues et accepte/refuse ; l'acceptation crée l'amitié mutuelle et notifie l'émetteur.

**Independent Test**: Avec une demande `en_attente`, accepter → amitié visible des deux côtés + notification `demande_acceptee` ; refuser une autre → disparaît sans notifier l'émetteur ; demande croisée → amitié directe.

### Implementation (Backend)

- [X] T017 [P] [US2] Modèle amitié dans `uafricas_backend/src/models/amitie.rs` : struct `Amitie` (`FromRow`), DTO `AmiResponse`, helper `creer_amitie(tx, a, b)` (ordre canonique via `paire_canonique`)
- [X] T018 [US2] Handler `accepter_demande` (`POST /api/amities/demandes/{id}/accepter`) dans `uafricas_backend/src/handlers/amitie.rs` : **transaction** statut→`acceptee` + `creer_amitie` + notification `demande_acceptee` à l'émetteur, garde 409 si déjà traitée, 403 si non destinataire, `audit::log_action("UPDATE",...)`
- [X] T019 [US2] Handler `refuser_demande` (`POST /api/amities/demandes/{id}/refuser`) : statut→`refusee`, **pas** de notification (FR-008), gardes 409/403, audit
- [X] T020 [P] [US2] Handlers de liste `lister_demandes_recues` (`GET /api/amities/demandes/recues`) et `lister_demandes_envoyees` (`GET /api/amities/demandes/envoyees`) dans `uafricas_backend/src/handlers/amitie.rs` : pagination, JOIN `MembreLight`
- [X] T021 [P] [US2] Handlers notifications `lister_notifications` (`GET /api/amities/notifications`), `marquer_lu` (`PATCH .../{id}/lu`), `tout_lu` (`PATCH .../tout-lu`) dans `uafricas_backend/src/handlers/amitie.rs` (FR-017)
- [X] T022 [US2] Brancher les routes US2 (accepter/refuser/recues/envoyees/notifications) dans `uafricas_backend/src/routes.rs`

### Implementation (Frontend)

- [X] T023 [P] [US2] Dans `uafricas_frontend/app/composables/useAmis.ts` : `listerDemandesRecues()`, `accepterDemande(id)`, `refuserDemande(id)`, `listerNotifications()`, `marquerNotificationLue(id)`
- [X] T024 [US2] Créer `uafricas_frontend/app/pages/mon-compte/amis.vue` (Tailwind v4 pur) avec un onglet « Demandes reçues » : liste des demandeurs (`MembreLight`) + boutons Accepter/Refuser ; ajouter le lien « Mes amis » dans la NavBar (`app/components/layout/NavBar.vue`)

**Checkpoint**: US1 + US2 : cycle complet envoyer → accepter/refuser → amitié établie, testable de bout en bout

---

## Phase 5: User Story 3 - Chat temps réel via le bouton flottant (Priority: P2)

**Goal**: Bouton flottant global ouvrant une fenêtre listant les amis ; messagerie texte 1-1 en temps réel (SSE), indicateur de non-lus, suppression de ses messages.

**Independent Test**: Deux comptes amis, deux navigateurs : ouvrir le bouton flottant sur n'importe quelle page, sélectionner l'ami, envoyer un message → reçu < 2 s sans rechargement ; badge non-lus fenêtre fermée ; non-ami absent de la liste ; supprimer un message → « message supprimé » des deux côtés.

### Implementation (Backend)

- [X] T025 [US3] Service SSE `uafricas_backend/src/services/messagerie_sse.rs` : registre `Arc<Mutex<HashMap<Uuid, Vec<UnboundedSender<Event>>>>>`, `abonner(user_id) -> Stream`, `publier(user_id, evenement)`, nettoyage à la déconnexion (sans nouvelle dépendance ; `tokio`/`futures-util`)
- [X] T026 [US3] Enregistrer le registre SSE dans l'état applicatif (`web::Data`) au démarrage dans `uafricas_backend/src/main.rs`
- [X] T027 [P] [US3] Modèles conversation/message dans `uafricas_backend/src/models/messagerie.rs` : structs `Conversation`, `Message` (`FromRow`), DTO `ConversationResponse` (avec `non_lus`, `verrouillee`), `MessageResponse` (avec `supprime`), évènements SSE sérialisables
- [X] T028 [US3] Handler flux SSE `flux` (`GET /api/messagerie/flux?token=`) dans `uafricas_backend/src/handlers/messagerie.rs` : auth par token query (Décision 3), `HttpResponse` streaming `text/event-stream`, keep-alive périodique, abonnement au registre
- [X] T029 [US3] Handler `lister_conversations` (`GET /api/messagerie/conversations`) : tri `dernier_message_at` desc, `non_lus`, `verrouillee` (amitié absente/blocage)
- [X] T030 [US3] Handler `lister_messages` (`GET /api/messagerie/conversations/{ami_id}/messages`) : pagination `avant`/`limite≤50`, création conversation à la volée si amis, 403 si non amis/bloqué (FR-022), contenu `null`+`supprime` si soft-deleted
- [X] T031 [US3] Handler `envoyer_message` (`POST /api/messagerie/conversations/{ami_id}/messages`) : validation 1..2000 (FR-027), vérif **amitié active** (FR-022/R5), persistance + MAJ `dernier_message_at`, **push SSE** `message` au destinataire et aux autres connexions de l'expéditeur ; `audit::log_action("CREATE","social","message",...)` avec **métadonnées uniquement** (id message/conversation/expéditeur, longueur, jamais le contenu, Décision 9)
- [X] T032 [P] [US3] Handlers `marquer_conversation_lue` (`POST .../{ami_id}/lu` → push `non_lus`), `supprimer_message` (`DELETE /api/messagerie/messages/{id}` → soft delete + push `message_supprime`, 403 si non expéditeur, `audit::log_action("DELETE","social","message",...)` métadonnées seules), `compteur_non_lus` (`GET /api/messagerie/non-lus`)
- [X] T033 [US3] Étendre `creer_demande`/`accepter_demande` (handlers/amitie.rs) pour **publier via SSE** les évènements `demande_recue` (T010) et `demande_acceptee` (T018) en plus de la notification persistée
- [X] T034 [US3] Brancher toutes les routes `/api/messagerie/*` dans `uafricas_backend/src/routes.rs`

### Implementation (Frontend)

- [X] T035 [US3] Plugin `uafricas_frontend/app/plugins/messagerie.client.ts` : ouvre `EventSource` sur `/api/messagerie/flux?token=` après authentification, reconnexion avec token rafraîchi, dispatch des évènements dans l'état global `useState`
- [X] T035b [US3] À chaque (re)connexion du flux SSE (`messagerie.client.ts` / `useMessagerie.ts`), recharger `listerConversations()` + `obtenirNonLus()` pour **rattraper les messages reçus pendant la coupure** (edge case « resynchronisation sans perte »)
- [X] T036 [US3] Dans `uafricas_frontend/app/composables/useMessagerie.ts` : état global `useState` (conversations, compteur non-lus, conversation ouverte), `listerConversations`, `listerMessages`, `envoyerMessage`, `marquerLue`, `supprimerMessage`, `obtenirNonLus`, handlers d'évènements SSE
- [X] T037 [P] [US3] Composant `uafricas_frontend/app/components/social/ListeAmis.vue` (Tailwind v4 pur) : liste des conversations/amis avec dernier message + badge non-lus ; sélection d'un ami
- [X] T038 [P] [US3] Composant `uafricas_frontend/app/components/social/FenetreConversation.vue` (Tailwind v4 pur) : fil de messages (avec « message supprimé »), saisie (compteur 2000, envoi), suppression de ses messages, état verrouillé (lecture seule)
- [X] T039 [US3] Composant `uafricas_frontend/app/components/social/MessagerieFlottante.vue` (Tailwind v4 pur) : bouton flottant fixe + badge non-lus, ouvre une fenêtre flottante intégrant `ListeAmis` puis `FenetreConversation` ; état vide « faites-vous des amis »
- [X] T040 [US3] Monter `<SocialMessagerieFlottante>` dans `uafricas_frontend/app/layouts/default.vue`, **client-only et seulement si connecté** (présent sur toutes les pages)

**Checkpoint**: US1 + US2 + US3 : les amis discutent en temps réel via le bouton flottant global

---

## Phase 6: User Story 4 - Gérer ses relations & blocage (Priority: P3)

**Goal**: Espace de gestion complet : amis, demandes envoyées, bloqués ; annuler une demande, retirer un ami, bloquer/débloquer.

**Independent Test**: Annuler une demande envoyée en attente ; retirer un ami → disparaît du chat + conversation verrouillée ; bloquer → demandes/amitié rompues + messagerie inaccessible ; débloquer.

### Implementation (Backend)

- [X] T041 [P] [US4] Handler `annuler_demande` (`DELETE /api/amities/demandes/{id}`) dans `uafricas_backend/src/handlers/amitie.rs` : statut→`annulee`, 403 si non émetteur, 409 si déjà traitée, audit (FR-010)
- [X] T042 [P] [US4] Handler `lister_amis` (`GET /api/amities`) : liste paginée filtrée sur l'utilisateur courant (FR-011/FR-026), recherche optionnelle
- [X] T043 [US4] Handler `retirer_ami` (`DELETE /api/amities/{utilisateur_id}`) : suppression `amitie` + **verrouillage** de la conversation (FR-012/FR-025, verrouillage implicite dérivé), audit `DELETE`
- [X] T044 [US4] Handler `bloquer` (`POST /api/blocages`) dans `uafricas_backend/src/handlers/amitie.rs` : **transaction** créer `blocage` + supprimer amitié + annuler demandes actives entre eux + verrouiller conversation (FR-013/R4), audit `CREATE`
- [X] T045 [P] [US4] Handlers `debloquer` (`DELETE /api/blocages/{utilisateur_id}`, audit `DELETE`) et `lister_blocages` (`GET /api/blocages`) dans `uafricas_backend/src/handlers/amitie.rs`
- [X] T046 [US4] Brancher les routes US4 (annuler, amities liste/retrait, blocages) dans `uafricas_backend/src/routes.rs`

### Implementation (Frontend)

- [X] T047 [P] [US4] Dans `uafricas_frontend/app/composables/useAmis.ts` : `annulerDemande(id)`, `listerAmis()`, `retirerAmi(utilisateurId)`, `bloquer(utilisateurId)`, `debloquer(utilisateurId)`, `listerBlocages()` (+ `listerDemandesEnvoyees()`)
- [X] T048 [US4] Enrichir `uafricas_frontend/app/pages/mon-compte/amis.vue` : onglets « Amis » (retrait + blocage), « Demandes envoyées » (annulation), « Bloqués » (déblocage)
- [X] T049 [US4] Refléter le blocage dans `BoutonAmitie.vue` (état `bloque_par_moi` + action Débloquer) et le verrouillage dans `FenetreConversation.vue` (déjà géré en US3)

**Checkpoint**: Les 4 user stories sont fonctionnelles et indépendamment testables

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Cohérence, sécurité, conformité constitution, documentation

- [X] T050 [P] Vérifier que **toutes** les mutations (demande create/accept/refus/annul, amitié retrait, blocage/déblocage **et** message create/delete) appellent `audit::log_action` ; pour les messages, confirmer que le payload d'audit ne contient **que des métadonnées** (jamais le contenu textuel), Principe VII / Décision 9 → **vérifié** : annuler/retirer/bloquer/débloquer audités (UPDATE/DELETE/CREATE/DELETE), message create/delete = métadonnées seules
- [X] T051 [P] Sécurité : confirmer le filtrage de confidentialité FR-026 (aucun endpoint n'expose la liste d'amis d'autrui) et exclure la query string du flux SSE des logs (Décision 3) → **vérifié** : `lister_amis`/`lister_blocages` dérivent `moi` du JWT ; aucun `middleware::Logger` configuré dans `main.rs` (le token query n'est donc pas journalisé)
- [X] T052 [P] Vérifier la cohérence des types cross-stack (structs Rust ↔ DTO ↔ interfaces TS) selon `contracts/api.md` (Principe II) → **vérifié** : `AmiResponse{utilisateur,ami_depuis}`↔`AmiAPI`, `BlocageResponse{utilisateur,depuis}`↔`BlocageAPI`
- [X] T053 Mettre à jour `CLAUDE.md` (section Recent Changes + tableau API Routes : domaines Amitié et Messagerie) et la documentation du schéma `social`
- [ ] T054 Exécuter le parcours de validation complet de [quickstart.md](./quickstart.md) (US1→US4 + vérifications transverses), **validation manuelle requise** (stack lancée + deux navigateurs)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance
- **Foundational (Phase 2)** : dépend de Setup, **bloque toutes les user stories** (schéma `social` requis)
- **User Stories (Phases 3-6)** : dépendent de Foundational
  - US1 (P1) → US2 (P1) : US2 suppose des demandes créées (US1) mais testable via insertion manuelle d'une demande
  - US3 (P2) : suppose une amitié (US1+US2) mais testable via insertion manuelle d'une `amitie`
  - US4 (P3) : agit sur demandes/amitiés/conversations des stories précédentes
- **Polish (Phase 7)** : après les stories souhaitées

### User Story Dependencies

- **US1** : indépendante (après Foundational)
- **US2** : enchaîne logiquement sur US1 (réutilise `demande_amitie`, `MembreLight`) ; indépendamment testable
- **US3** : réutilise `amitie` (US2) et étend les notifications (T033) ; le socle SSE (T025-T026) lui est propre
- **US4** : touche `BoutonAmitie` (US1) et `FenetreConversation` (US3) ; gardes de blocage déjà prévues côté US1/US3

### Within Each User Story

- Modèles avant services/handlers ; handlers avant câblage des routes ; backend avant intégration frontend

### Parallel Opportunities

- Setup : T002, T003 en parallèle
- Foundational : T006 (après T004/T005) ; T004 d'abord (bloquant)
- US1 : T008, T009 en parallèle ; T013 en parallèle du backend
- US2 : T017, T020, T021 en parallèle ; T023 en parallèle
- US3 : T027, T032, T037, T038 en parallèle (fichiers distincts)
- US4 : T041, T042, T045, T047 en parallèle
- Polish : T050, T051, T052 en parallèle

---

## Parallel Example: User Story 1

```bash
# Modèles backend US1 en parallèle :
Task: "Modèle demande d'amitié dans src/models/amitie.rs"        # T008
Task: "Modèle notification dans src/models/amitie.rs"             # T009  (même fichier → séquencer si conflit)

# Composable frontend en parallèle du backend :
Task: "useAmis.envoyerDemande + obtenirEtatRelation"             # T013
```

> Note : T008 et T009 modifient le même fichier `models/amitie.rs`, les exécuter séquentiellement ou en sections distinctes pour éviter les conflits.

---

## Implementation Strategy

### MVP First (US1 + US2)

1. Phase 1 (Setup) → Phase 2 (Foundational)
2. Phase 3 (US1) → **STOP & VALIDATE** : envoi de demande + état
3. Phase 4 (US2) → cycle complet accepter/refuser → **MVP livrable** (relations d'amitié fonctionnelles)

### Incremental Delivery

1. Setup + Foundational → fondation prête
2. US1 + US2 → MVP (amitié) → démo
3. US3 → messagerie temps réel → démo (valeur principale débloquée)
4. US4 → gestion fine + blocage → démo
5. Polish → conformité & validation quickstart

---

## Notes

- `[P]` = fichiers différents, sans dépendance bloquante
- UI **Tailwind v4 pur** partout (public + espace membre), pas de daisyUI (Principe VI)
- Temps réel **mono-instance** (registre SSE en mémoire)
- Schéma SQL d'abord, puis backend, puis frontend (Principe III)
- Commit en français après chaque tâche ou groupe logique
- En production : appliquer `29_social.sql` par migration manuelle SSH+psql (pas de recréation de volume)
