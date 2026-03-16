# Tasks: Notifications et Suggestions Intelligentes

**Input**: Design documents from `/specs/001-notifications-suggestions/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Non demandés.

**Organization**: Full-stack. 2 nouvelles tables, 7 endpoints, cloche navbar, suggestions client, doublons.

## Format: `[ID] [P?] [Story] Description`

## Path Conventions

- **Backend**: `uafricas_backend/`
- **Frontend**: `uafricas_frontend/app/`

---

## Phase 1: Setup (SQL + Types)

- [x] T001 Créer `uafricas_backend/doc/bd/schemas/26_notifications.sql` — table `arbre_genealogique.notifications` (id UUID PK, destinataire_id FK iam.utilisateur, type VARCHAR(30), message TEXT, lien_action VARCHAR(500), lu BOOLEAN DEFAULT FALSE, created_at TIMESTAMPTZ DEFAULT NOW()), table `arbre_genealogique.doublons_ignores` (id UUID PK, arbre_id FK, personne_a_id FK, personne_b_id FK, created_at, UNIQUE INDEX sur LEAST/GREATEST par arbre), indexes sur destinataire_id + lu
- [x] T002 Ajouter `\ir schemas/26_notifications.sql` dans `uafricas_backend/doc/bd/schema.sql`
- [x] T003 [P] Créer types TS dans `uafricas_frontend/app/mocks/notifications.ts` — interfaces `Notification`, `DoublonPotentiel`, `SuggestionProactive`, `FusionDoublonDto`. Types `TypeNotification`, helpers `iconeNotification()`, `couleurNotification()`

---

## Phase 2: Foundational (Backend models + handlers + routes)

- [x] T004 [P] Créer `uafricas_backend/src/models/notification.rs` — structs FromRow : `NotificationRow`, `DoublonIgnoreRow`. DTOs : `NotificationResponse`, `DoublonResponse`, `FusionDoublonDto`, `IgnorerDoublonDto`. Helper : `creer_notification(pool, destinataire_id, type, message, lien_action)` — INSERT non-bloquant.
- [x] T005 [P] Créer `uafricas_backend/src/handlers/notification.rs` — 7 handlers : `compteur_notifications` (GET, COUNT WHERE lu=false), `lister_notifications` (GET, paginé), `marquer_lue` (POST), `tout_marquer_lu` (POST), `detecter_doublons` (GET, pg_trgm intra-arbre seuil 70%), `ignorer_doublon` (POST), `fusionner_doublons` (POST transactionnel : update personne A avec champs choisis, transférer liens de B vers A, soft-delete B)
- [x] T006 Ajouter `pub mod notification` dans `uafricas_backend/src/handlers/mod.rs` et `uafricas_backend/src/models/mod.rs`
- [x] T007 Ajouter 7 routes dans `uafricas_backend/src/routes.rs` — scope `/notifications` : compteur (GET), liste (GET), `/{id}/lire` (POST), `/tout-lire` (POST). Scope `/arbre` : `/doublons` (GET), `/doublons/ignorer` (POST), `/doublons/fusionner` (POST)
- [x] T008 Modifier `uafricas_backend/src/handlers/matching.rs` — dans `matching_profond`, après INSERT des suggestions, INSERT notification pour chaque utilisateur concerné : "Nouvelle correspondance détectée pour [NomPersonne]"
- [x] T009 Modifier `uafricas_backend/src/handlers/collaboration.rs` — dans `accepter_invitation`, INSERT notification pour le propriétaire : "[User] a accepté votre invitation". Dans les handlers d'édition (creer_personne, modifier_personne, supprimer_personne via Feature 3), INSERT notification pour le propriétaire si l'auteur est un collaborateur
- [x] T010 [P] Créer `uafricas_frontend/app/composables/useNotifications.ts` — méthodes : `compteurNonLues()`, `listerNotifications(page, type?)`, `marquerLue(id)`, `toutMarquerLu()`, `detecterDoublons()`, `ignorerDoublon(a, b)`, `fusionnerDoublons(dto)`. Exporter ref réactif `nbNonLues`.

**Checkpoint**: Backend complet + composable prêt

---

## Phase 3: User Story 1 — Notifications in-app (Priority: P1) 🎯 MVP

**Goal**: Cloche dans la navbar avec badge, panneau déroulant, notifications cliquables

- [x] T011 [P] [US1] Créer `uafricas_frontend/app/components/layout/ClocheNotifications.vue` — icône cloche (FontAwesome `bell`), badge rouge avec compteur si > 0, clic ouvre un panneau déroulant (dropdown position absolute, max-height scrollable). Liste des 20 dernières notifications : icône par type, message, date relative, statut lu/non-lu. Clic sur une notification → marquer lue + naviguer vers lien_action. Bouton "Tout marquer comme lu" en haut. Lien "Voir toutes" en bas. Tailwind v4 pur.
- [x] T012 [US1] Modifier `uafricas_frontend/app/layouts/default.vue` — importer et ajouter `ClocheNotifications` dans la navbar (à côté du profil utilisateur). Appeler `compteurNonLues()` au montage du layout pour initialiser le badge.
- [x] T013 [US1] Ajouter l'icône `faBell` dans `uafricas_frontend/app/plugins/fontawesome.ts` si elle n'existe pas déjà

**Checkpoint**: Cloche avec badge + panneau fonctionnel — US1 testable

---

## Phase 4: User Story 2 — Suggestions proactives (Priority: P2)

**Goal**: Suggestions de complétion d'arbre calculées côté client

- [x] T014 [P] [US2] Créer `uafricas_frontend/app/composables/useSuggestions.ts` — fonction `calculerSuggestions(graphe, liens): SuggestionProactive[]` qui analyse le graphe en mémoire et retourne max 10 suggestions triées par priorité : personnes sans parents (< 2, les plus connectées en premier), personnes sans date de naissance, branches avec une seule personne. Chaque suggestion a un type, message lisible, et action (URL vers visualisation avec paramètre).
- [x] T015 [US2] Ajouter une section "Suggestions" dans la page index `uafricas_frontend/app/pages/arbre-genealogique/index.vue` — sous "Mon arbre", afficher les 5 premières suggestions dans des cartes compactes : icône (ampoule), message, bouton "Compléter". Clic redirige vers `/arbre-genealogique/visualisation?centre={rattachement_id}`. Afficher uniquement si l'utilisateur a un arbre avec au moins 3 personnes.

**Checkpoint**: Suggestions visibles et cliquables — US2 testable

---

## Phase 5: User Story 3 — Détection de doublons (Priority: P2)

**Goal**: Détecter et gérer les doublons dans son propre arbre

- [x] T016 [P] [US3] Créer `uafricas_frontend/app/components/arbre-genealogique/PanneauDoublons.vue` — liste des doublons potentiels : pour chaque paire, afficher les deux personnes côte à côte (nom, dates, lieu), score de similarité, boutons "Ignorer" et "Fusionner". Le bouton "Fusionner" ouvre un formulaire de choix champ par champ (nom de A ou B, dates de A ou B, etc.) avec preview du résultat. Tailwind v4 pur.
- [x] T017 [US3] Ajouter un onglet/section "Doublons" dans la page de gestion `uafricas_frontend/app/pages/arbre-genealogique/gestion.vue` — appeler `detecterDoublons()`, afficher `PanneauDoublons` si des doublons sont détectés. Après fusion, recharger l'arbre.

**Checkpoint**: Doublons détectés, ignorés ou fusionnés — US3 testable

---

## Phase 6: User Story 4 — Page notifications complète (Priority: P3)

- [x] T018 [US4] Créer `uafricas_frontend/app/pages/notifications.vue` — page avec hero section "Mes Notifications", liste paginée (10/page), filtres par type (tabs : Tout, Matching, Collaboration, Suggestions), bouton "Tout marquer comme lu". Chaque notification affiche : icône type, message, date, statut lu/non-lu, lien d'action. `mt-28` pour navbar.

**Checkpoint**: Page notifications complète — US4 testable

---

## Phase 7: Polish

- [x] T019 [P] Vérification Tailwind CSS v4 dans tous les nouveaux composants
- [x] T020 Exécuter le scénario de validation quickstart.md — 8 étapes

---

## Dependencies & Execution Order

```
Phase 1 (SQL) → Phase 2 (Backend)
                     │
                ┌────┴────────────┐
                ▼                  ▼
         Phase 3 (US1)      Phase 4 (US2) [parallélisable]
         🎯 MVP                    │
                │              ┌───┴───┐
                ▼              ▼       ▼
         Phase 6 (US4)   Phase 5 (US3)│
                │              │       │
                └────┬─────────┘───────┘
                     ▼
                Phase 7 (Polish)
```

## Implementation Strategy

### MVP First
Phase 1 + 2 + 3 = **13 tâches** → Cloche avec notifications

### Estimation

| Phase | Tâches | Priorité |
|-------|--------|----------|
| Setup | 3 | — |
| Foundational | 7 | — |
| US1 (P1) | 3 | MVP |
| US2 (P2) | 2 | Incrémental |
| US3 (P2) | 2 | Incrémental |
| US4 (P3) | 1 | Finition |
| Polish | 2 | Final |
| **Total** | **20** | — |
