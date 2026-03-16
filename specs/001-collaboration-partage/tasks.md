# Tasks: Collaboration et Partage de l'Arbre

**Input**: Design documents from `/specs/001-collaboration-partage/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Non demandés.

**Organization**: Feature full-stack. 2 nouvelles tables SQL, 12 endpoints backend, pages frontend.

## Format: `[ID] [P?] [Story] Description`

## Path Conventions

- **Backend**: `uafricas_backend/`
- **Frontend**: `uafricas_frontend/app/`

---

## Phase 1: Setup (SQL + Types)

**Purpose**: Migration SQL, types TypeScript, modèles Rust

- [x] T001 Créer le fichier de migration `uafricas_backend/doc/bd/schemas/25_collaboration.sql` — ajouter colonne `arbre_prive BOOLEAN DEFAULT FALSE` à `arbre_genealogique.arbres`, colonne `visible_matching BOOLEAN DEFAULT TRUE` à `arbre_genealogique.personnes`, créer table `arbre_genealogique.invitations` (id UUID PK, arbre_id FK, email_invite VARCHAR(255), utilisateur_invite_id FK nullable, permission VARCHAR(20), statut VARCHAR(20) DEFAULT 'en_attente', invite_par FK, created_at, expire_at, traitee_le), créer table `arbre_genealogique.collaborateurs` (id UUID PK, arbre_id FK, utilisateur_id FK, permission VARCHAR(20), invitation_id FK, created_at, UNIQUE(arbre_id, utilisateur_id)), indexes appropriés
- [x] T002 Ajouter `\ir schemas/25_collaboration.sql` dans `uafricas_backend/doc/bd/schema.sql`
- [x] T003 [P] Créer les types TypeScript dans `uafricas_frontend/app/mocks/collaboration.ts` — interfaces `Invitation`, `Collaborateur`, `MesArbresResponse`, `HistoriqueEntree`, types `Permission = 'lecture_seule' | 'edition'`, `StatutInvitation`. Données mock + helpers async.

---

## Phase 2: Foundational (Modèles + Handlers Rust)

**Purpose**: Modèles Rust, handlers de base, routes — BLOQUE toutes les user stories

- [x] T004 [P] Créer les modèles Rust `uafricas_backend/src/models/collaboration.rs` — structs FromRow : `Invitation`, `Collaborateur`, `HistoriqueEntree`. DTOs : `InvitationResponse`, `CollaborateurResponse`, `MesArbresResponse`, `CreerInvitationDto`, `ModifierPermissionDto`, `ConfidentialiteArbreDto`, `ConfidentialitePersonneDto`, `HistoriqueResponse`.
- [x] T005 [P] Créer les handlers `uafricas_backend/src/handlers/collaboration.rs` — 12 handlers : `mes_arbres` (GET), `creer_invitation` (POST, vérifie propriétaire + limite 20 + email unique), `lister_invitations_recues` (GET), `accepter_invitation` (POST, crée collaborateur), `refuser_invitation` (POST), `lister_collaborateurs` (GET, propriétaire), `modifier_permission` (PUT, propriétaire), `revoquer_collaborateur` (DELETE, propriétaire), `modifier_confidentialite_arbre` (PUT, propriétaire), `modifier_confidentialite_personne` (PUT, propriétaire), `obtenir_historique` (GET, filtre audit_log par arbre). Helper `verifier_acces_arbre(pool, utilisateur_id, arbre_id) -> Result<Permission>` qui vérifie si l'utilisateur est propriétaire ou collaborateur.
- [x] T006 Ajouter `pub mod collaboration` dans `uafricas_backend/src/handlers/mod.rs` et `uafricas_backend/src/models/mod.rs`
- [x] T007 Ajouter les 12 routes dans le scope `/arbre` de `uafricas_backend/src/routes.rs` — `/mes-arbres` (GET), `/invitations` (POST, GET), `/invitations/{id}/accepter` (POST), `/invitations/{id}/refuser` (POST), `/{arbre_id}/collaborateurs` (GET), `/collaborateurs/{id}` (PUT, DELETE), `/{arbre_id}/confidentialite` (PUT), `/personnes/{id}/confidentialite` (PUT), `/{arbre_id}/historique` (GET). Import `collaboration` dans le use.
- [x] T008 Modifier `obtenir_arbre_complet` dans `uafricas_backend/src/handlers/arbre_genealogique.rs` — accepter un query param optionnel `arbre_id`. Si fourni, vérifier que l'utilisateur est propriétaire ou collaborateur (via `verifier_acces_arbre`). Retourner aussi la permission de l'utilisateur dans la réponse.
- [x] T009 [P] Créer le composable `useCollaboration` dans `uafricas_frontend/app/composables/useCollaboration.ts` — méthodes pour les 12 endpoints : `mesArbres()`, `creerInvitation(email, permission)`, `listerInvitationsRecues()`, `accepterInvitation(id)`, `refuserInvitation(id)`, `listerCollaborateurs(arbreId)`, `modifierPermission(id, permission)`, `revoquerCollaborateur(id)`, `modifierConfidentialiteArbre(arbreId, prive)`, `modifierConfidentialitePersonne(id, visible)`, `obtenirHistorique(arbreId, params)`.

**Checkpoint**: Backend complet + composable frontend prêt

---

## Phase 3: User Story 1 + 5 — Invitations + Navigation multi-arbres (Priority: P1) 🎯 MVP

**Goal**: Inviter un collaborateur, accepter/refuser, naviguer entre ses arbres

**Independent Test**: Inviter user2 → accepter → arbre apparaît dans "Arbres partagés"

- [x] T010 [US1] Modifier la page `index.vue` (`uafricas_frontend/app/pages/arbre-genealogique/index.vue`) — appeler `mesArbres()` au montage. Afficher deux sections : "Mon arbre" (carte avec nb personnes + boutons Voir/Gérer) et "Arbres partagés avec moi" (liste de cartes avec nom propriétaire, permission, nb personnes, bouton Voir). Chaque carte de la section partagés clique vers `/arbre-genealogique/visualisation?arbre_id=...`. Ajouter un bouton "Inviter un collaborateur" dans la section Mon arbre.
- [x] T011 [US1] Créer le composant `CarteInvitation.vue` dans `uafricas_frontend/app/components/arbre-genealogique/CarteInvitation.vue` — carte d'invitation reçue : nom de l'arbre, nom du propriétaire, permission proposée, boutons Accepter/Refuser. Tailwind v4 pur.
- [x] T012 [US1] Ajouter une section "Invitations en attente" dans `index.vue` — si `listerInvitationsRecues()` retourne des invitations, les afficher en haut avec `CarteInvitation`. Actions accepter/refuser mettent à jour la liste.
- [x] T013 [US5] Modifier `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`) — lire le query param `?arbre_id=...`. Si présent, passer ce paramètre à `obtenirArbreComplet({ arbre_id })`. Stocker la permission retournée pour conditionner l'affichage des actions.

**Checkpoint**: Invitations + navigation multi-arbres fonctionnels — US1 + US5 testables

---

## Phase 4: User Story 2 — Permissions (Priority: P1)

**Goal**: Lecture seule vs édition, bandeau visuel, masquage des actions

- [x] T014 [P] [US2] Créer le composant `BandeauLectureSeule.vue` dans `uafricas_frontend/app/components/arbre-genealogique/BandeauLectureSeule.vue` — bandeau fixe en haut de la visualisation : "Arbre de [Propriétaire] — Lecture seule" avec fond ambre/jaune clair, icône cadenas. Visible uniquement quand permission = lecture_seule.
- [x] T015 [US2] Intégrer le bandeau et le masquage des actions dans `visualisation.vue` (`uafricas_frontend/app/pages/arbre-genealogique/visualisation.vue`) — si permission = 'lecture_seule' : afficher `BandeauLectureSeule`, masquer les boutons d'action dans `PanneauPersonne` (passer une prop `editable: boolean`), masquer les badges d'incomplétude (pas pertinent en lecture seule).
- [x] T016 [US2] Modifier `PanneauPersonne.vue` (`uafricas_frontend/app/components/arbre-genealogique/PanneauPersonne.vue`) — ajouter une prop `editable: boolean` (défaut: true). Si false, masquer la section "Actions" (ajouter parent/enfant/conjoint, modifier, supprimer).

**Checkpoint**: Permissions visuellement appliquées — US2 testable

---

## Phase 5: User Story 3 — Confidentialité (Priority: P2)

**Goal**: Marquer des personnes/l'arbre comme privé pour le matching

- [x] T017 [US3] Créer la page `gestion.vue` dans `uafricas_frontend/app/pages/arbre-genealogique/gestion.vue` — page de gestion de l'arbre avec 3 onglets/sections : "Collaborateurs" (liste + inviter), "Confidentialité" (toggle arbre privé + liste personnes avec toggles visible_matching), "Historique" (liste chronologique des modifications). Hero section avec titre "Gestion de mon arbre". Accessible via bouton "Gérer" depuis index.vue.
- [x] T018 [US3] Implémenter la section Confidentialité dans `gestion.vue` — toggle global "Arbre privé" qui appelle `modifierConfidentialiteArbre(arbreId, prive)`. Liste de toutes les personnes avec toggle "Visible pour le matching" qui appelle `modifierConfidentialitePersonne(id, visible)`. Le toggle global désactive les toggles individuels quand l'arbre est privé.
- [x] T019 [US3] Modifier le matching dans `uafricas_backend/src/services/matching.rs` — dans la requête de `matching_profond`, ajouter les conditions : `AND p2.visible_matching = TRUE` et `AND NOT EXISTS (SELECT 1 FROM arbre_genealogique.arbres a WHERE a.id = r2.arbre_id AND a.arbre_prive = TRUE)`. Idem pour `recherche_publique` dans `uafricas_backend/src/handlers/matching.rs`.

**Checkpoint**: Confidentialité appliquée au matching — US3 testable

---

## Phase 6: User Story 4 — Historique (Priority: P2)

**Goal**: Consulter l'historique des modifications avec filtres

- [x] T020 [US4] Implémenter la section Historique dans `gestion.vue` (`uafricas_frontend/app/pages/arbre-genealogique/gestion.vue`) — appeler `obtenirHistorique(arbreId)`. Afficher une liste chronologique inversée avec : icône action (ajout=vert, modification=bleu, suppression=rouge), description formatée ("Ibrahim Diallo ajouté par Admin Test"), date relative ("il y a 2 heures"). Filtre par collaborateur (select). Pagination.

**Checkpoint**: Historique consultable — US4 testable

---

## Phase 7: Polish

- [x] T021 [P] Implémenter la section Collaborateurs dans `gestion.vue` — liste des collaborateurs avec nom, email, permission, date. Boutons : modifier permission (select lecture_seule/edition), révoquer (bouton rouge avec confirmation). Formulaire d'invitation : champ email + select permission + bouton "Inviter".
- [x] T022 [P] Vérification Tailwind CSS v4 dans tous les nouveaux composants
- [x] T023 Exécuter le scénario de validation quickstart.md — 7 étapes avec 2 comptes

---

## Dependencies & Execution Order

```
Phase 1 (SQL + Types) → Phase 2 (Backend + Composable)
                              │
                         ┌────┴─────────┐
                         ▼               ▼
                   Phase 3 (US1+5)  Phase 4 (US2) [parallélisable]
                   🎯 MVP               │
                         │               │
                    ┌────┴────┐          │
                    ▼         ▼          │
             Phase 5 (US3)  Phase 6 (US4)│
                    │         │          │
                    └────┬────┘──────────┘
                         ▼
                    Phase 7 (Polish)
```

## Implementation Strategy

### MVP First

1. Phase 1 + 2 (T001-T009)
2. Phase 3 (T010-T013)
3. **STOP** : Invitations + navigation multi-arbres

### Estimation

| Phase | Tâches | Priorité |
|-------|--------|----------|
| Setup | 3 | — |
| Foundational | 6 | — |
| US1+US5 (P1) | 4 | MVP |
| US2 (P1) | 3 | MVP+ |
| US3 (P2) | 3 | Incrémental |
| US4 (P2) | 1 | Incrémental |
| Polish | 3 | Final |
| **Total** | **23** | — |
