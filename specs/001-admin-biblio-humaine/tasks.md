# Tasks: Validation Admin des Bibliothèques Humaines

**Feature**: `001-admin-biblio-humaine` | **Date**: 2026-04-22 | **Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)

**Stack**: Rust 2024 / Actix-Web 4 / sqlx (backend), TypeScript / Nuxt 4 / Pinia (frontend)
**Tests**: N/A : pas de CI configuré sur ce projet

## Format: `[ID] [P?] [Story?] Description`

- **[P]** : peut s'exécuter en parallèle (fichiers différents, aucune dépendance incomplète)
- **[Story]** : user story concernée ([US1]…[US5]), absent dans les phases Fondation/Polish
- Chaque description inclut le chemin de fichier depuis la racine du monorepo

---

## Phase 1: Fondation SQL & Modèles Rust

**But** : Poser la base de données et les types Rust. Aucune user story ne peut commencer avant la complétion de cette phase.

**⚠️ CRITIQUE** : Les tâches T001–T006 doivent être terminées avant toute implémentation de user story.

- [X] T001 Créer `uafricas_backend/doc/bd/schemas/04b_iam_biblio_demande.sql`, enum `iam.statut_demande_biblio`, tables `iam.demande_biblio_humaine` et `iam.demande_biblio_specialite`, 3 index (utilisateur, statut, unicité active)
- [X] T002 [P] Ajouter `DemandeCreeeResponse` et `MaDemandeResponse` dans `uafricas_backend/src/models/bibliotheque_humaine.rs`
- [X] T003 [P] Créer `uafricas_backend/src/models/admin/biblio_humaine.rs`, structs `AdminDemandeBiblioRow`, `TraiterDemandeBody`, `AdminDemandeBiblioQueryParams`
- [X] T004 Déclarer `pub mod biblio_humaine;` dans `uafricas_backend/src/models/admin/mod.rs`
- [X] T005 Déclarer `pub mod bibliotheques_humaines;` dans `uafricas_backend/src/handlers/admin/mod.rs`
- [X] T006 Modifier `inscrire_biblio` dans `uafricas_backend/src/handlers/bibliotheques_humaines.rs`, créer une ligne dans `demande_biblio_humaine` (statut `en_attente`) au lieu de `UPDATE iam.utilisateur SET bibliotheque_humain = TRUE` ; retourner `DemandeCreeeResponse` (201) ; retourner 409 si demande active déjà existante

**Checkpoint** : Migration SQL jouée (`psql -f 04b_iam_biblio_demande.sql`), backend compile, `POST /api/bibliotheques-humaines/inscription` retourne 201 avec `{"statut":"en_attente"}`.

---

## Phase 2: User Story 1 : Liste des demandes admin (Priorité: P1) 🎯 MVP

**But** : L'administrateur peut accéder à la liste de toutes les demandes de Bibliothèque Humaine, filtrables par statut.

**Test indépendant** : Se connecter en admin → `GET /api/admin/bibliotheques-humaines` → la demande créée via `inscrire_biblio` apparaît avec `statut: "en_attente"`. La page `/admin/bibliotheques-humaines` s'affiche avec la liste paginée et le badge de comptage.

- [X] T007 [US1] Créer handler `lister_demandes` dans `uafricas_backend/src/handlers/admin/bibliotheques_humaines.rs`, `GET /api/admin/bibliotheques-humaines`, query params `statut`/`recherche`/`page`/`par_page`, JOIN sur `utilisateur` + `pays` + agrégat spécialités, vérification `verifier_permission!` rôle admin
- [X] T008 [US1] Ajouter route `GET /api/admin/bibliotheques-humaines` dans `uafricas_backend/src/routes.rs` (scope admin existant)
- [X] T009 [US1] Créer `uafricas_frontend/app/composables/useAdminBibliosHumaines.ts`, interface `AdminDemandeBiblio`, état réactif `demandes`/`total`/`page`/`chargement`/`erreur`, méthode `listerDemandes(params)`
- [X] T010 [US1] Créer `uafricas_frontend/app/pages/admin/bibliotheques-humaines/index.vue`, `AdminPageHeader` titre, filtres par statut daisyUI, tableau paginé avec colonnes nom/prénom/fonction/statut/date, badge "N en attente" dans l'en-tête, lien vers détail `[id]`

**Checkpoint** : Page admin accessible → liste affichée → filtre "en_attente" fonctionne → état vide géré (message informatif).

---

## Phase 3: User Story 2 : Validation ou rejet d'une demande (Priorité: P1)

**But** : L'administrateur peut consulter le détail d'une demande et l'approuver ou la rejeter, avec possibilité de revenir sur la décision.

**Test indépendant** : Admin ouvre une demande en attente → clique "Approuver" → réponse 200 `statut: "valide"` → `iam.utilisateur.bibliotheque_humain` passe à `TRUE` en base → action tracée dans la table d'audit.

- [X] T011 [P] [US2] Créer handler `obtenir_demande` dans `uafricas_backend/src/handlers/admin/bibliotheques_humaines.rs`, `GET /api/admin/bibliotheques-humaines/{id}`, retourne `AdminDemandeBiblioDetail` avec `biographie`/`pays`/`commentaireAdmin`/`traiteLe`/`traiteParNom` ; 404 si inexistant
- [X] T012 [P] [US2] Créer handler `valider_demande` dans `uafricas_backend/src/handlers/admin/bibliotheques_humaines.rs`, `PATCH /api/admin/bibliotheques-humaines/{id}/valider`, transaction atomique : (1) `demande.statut → valide`, (2) `utilisateur.bibliotheque_humain = TRUE` + `fonction` + `biographie` + `pays_origine_id`, (3) insert `utilisateur_specialite` depuis `demande_biblio_specialite`, (4) `audit::log_action` non-bloquant
- [X] T013 [P] [US2] Créer handler `rejeter_demande` dans `uafricas_backend/src/handlers/admin/bibliotheques_humaines.rs`, `PATCH /api/admin/bibliotheques-humaines/{id}/rejeter`, body `TraiterDemandeBody` : (1) `demande.statut → rejete`, (2) `demande.commentaire_admin`, (3) `utilisateur.bibliotheque_humain = FALSE` si nécessaire, (4) `audit::log_action`
- [X] T014 [US2] Ajouter routes `GET /{id}`, `PATCH /{id}/valider`, `PATCH /{id}/rejeter` dans `uafricas_backend/src/routes.rs` (scope admin biblio-humaine)
- [X] T015 [US2] Ajouter méthodes `obtenirDemande(id)`, `validerDemande(id)`, `rejeterDemande(id, commentaire?)` dans `uafricas_frontend/app/composables/useAdminBibliosHumaines.ts`
- [X] T016 [US2] Créer `uafricas_frontend/app/pages/admin/bibliotheques-humaines/[id].vue`, affichage profil complet (photo, nom, fonction, biographie, pays, spécialités, statut badge, date, traité par), boutons "Approuver"/"Rejeter" avec modale de confirmation (champ commentaire optionnel pour rejet), confirmation visuelle post-action, réversibilité explicite

**Checkpoint** : Admin approuve → `bibliotheque_humain = TRUE` en base → audit loggé → page de détail affiche le nouveau statut. Admin rejette → `bibliotheque_humain = FALSE` → commentaire sauvegardé.

---

## Phase 4: User Story 3 : Visibilité publique conditionnelle (Priorité: P2)

**But** : La page publique `/bibliotheque/humaine` n'affiche que les profils avec une demande en statut `valide`.

**Test indépendant** : Soumettre une demande → elle n'apparaît PAS publiquement → admin valide → elle apparaît publiquement → admin rejette → elle disparaît.

- [X] T017 [US3] Modifier `lister_biblios` dans `uafricas_backend/src/handlers/bibliotheques_humaines.rs`, remplacer le filtre `bibliotheque_humain = TRUE` par une jointure sur `iam.demande_biblio_humaine` avec `statut = 'valide'` et `deleted_at IS NULL`

**Checkpoint** : `GET /api/bibliotheques-humaines` ne retourne aucun profil en attente ou rejeté. Un profil validé par l'admin apparaît immédiatement.

---

## Phase 5: User Story 4 : Suivi de demande par le candidat (Priorité: P2)

**But** : L'utilisateur connecté peut voir le statut de sa demande depuis son profil personnel.

**Test indépendant** : Se connecter en user2 → `GET /api/bibliotheques-humaines/moi/demande` → retourne le statut courant + commentaire admin si rejeté → la page `/profil` affiche l'encart de statut.

- [X] T018 [US4] Créer handler `ma_demande` dans `uafricas_backend/src/handlers/bibliotheques_humaines.rs`, `GET /api/bibliotheques-humaines/moi/demande` (JWT requis), retourne `MaDemandeResponse` pour la demande active de l'utilisateur ; 404 si aucune demande
- [X] T019 [US4] Ajouter route `GET /api/bibliotheques-humaines/moi/demande` dans `uafricas_backend/src/routes.rs` (scope public biblio avec JWT)
- [X] T020 [US4] Ajouter `obtenirMaDemande(): Promise<MaDemandeAPI | null>` dans `uafricas_frontend/app/composables/useBibliothequeHumaine.ts`
- [X] T021 [US4] Modifier `uafricas_frontend/app/pages/profil.vue`, ajouter un encart "Bibliothèque Humaine" conditionnel : badge statut coloré (en attente = orange, validé = vert, rejeté = rouge), date de soumission, commentaire admin si statut rejeté, lien pour resoumettre si rejeté

**Checkpoint** : Utilisateur soumet → profil affiche "En attente de validation" avec la date. Admin valide → profil affiche "Validé". Admin rejette avec commentaire → profil affiche "Rejeté" + commentaire.

---

## Phase 6: User Story 5 : Notification au candidat (Priorité: P3)

**But** : L'utilisateur reçoit une notification in-app lors du traitement de sa demande.

**Test indépendant** : Admin valide → se connecter en tant que l'utilisateur concerné → une notification "Votre demande Bibliothèque Humaine a été acceptée" est visible.

- [X] T022 [US5] Ajouter table `iam.notification_biblio_humaine` dans `uafricas_backend/doc/bd/schemas/04b_iam_biblio_demande.sql`, colonnes : `id UUID PK`, `utilisateur_id UUID FK`, `type VARCHAR` (`approuvee`|`rejetee`), `lu BOOLEAN DEFAULT FALSE`, `commentaire TEXT NULL`, `created_at TIMESTAMPTZ`
- [X] T023 [P] [US5] Intégrer création de notification dans `valider_demande` dans `uafricas_backend/src/handlers/admin/bibliotheques_humaines.rs`, `INSERT INTO iam.notification_biblio_humaine (utilisateur_id, type)` dans la même transaction
- [X] T024 [P] [US5] Intégrer création de notification dans `rejeter_demande` dans `uafricas_backend/src/handlers/admin/bibliotheques_humaines.rs`, `INSERT INTO iam.notification_biblio_humaine (utilisateur_id, type, commentaire)` dans la même transaction
- [ ] T025 [US5] Afficher les notifications in-app non lues dans `uafricas_frontend/app/pages/profil.vue`, badge compteur + liste des notifications avec marquage "lu" au clic

**Checkpoint** : Admin approuve → utilisateur voit notification non lue. Admin rejette avec commentaire → notification inclut le motif. Clic → marquée comme lue.

---

## Phase 7: Polish & Vérifications transversales

**But** : Cohérence, traçabilité et validation end-to-end du workflow complet.

- [X] T026 [P] Mettre à jour `CLAUDE.md`, ajouter les nouvelles routes admin (`/api/admin/bibliotheques-humaines`) et le nouveau schéma SQL dans les sections "API Routes" et "Recent Changes"
- [ ] T027 Valider le quickstart : exécuter les 4 curl de `specs/001-admin-biblio-humaine/quickstart.md` et confirmer les statuts HTTP attendus
- [ ] T028 [P] Vérifier protection anti-doublon, soumettre une seconde demande avec un utilisateur ayant déjà une demande `en_attente` → confirmer réception du 409 Conflict

---

## Dépendances & Ordre d'exécution

### Dépendances entre phases

```
Phase 1 (Fondation)
  ├── Phase 2 (US1) → Phase 3 (US2)
  ├── Phase 4 (US3)
  ├── Phase 5 (US4)
  └── Phase 6 (US5) ← dépend aussi de Phase 3 (T012, T013)
                                         ↓
                              Phase 7 (Polish)
```

- **Phase 1** : aucun prérequis : commencer immédiatement
- **Phase 2** : dépend de Phase 1 complète (T001–T006)
- **Phase 3** : dépend de Phase 1 complète ; T011/T012/T013 parallélisables entre eux
- **Phase 4** : dépend uniquement de T001 (DDL) et T006 (`inscrire_biblio` modifié)
- **Phase 5** : dépend de T001 et T002 (types Rust) ; indépendante de US1/US2
- **Phase 6** : dépend de T012 et T013 (handlers valider/rejeter) ; commence après Phase 3
- **Phase 7** : dépend de toutes les user stories souhaitées

### Dépendances au sein de chaque user story

| Story | Ordre interne |
|-------|--------------|
| US1 | T007 → T008 → T009 → T010 |
| US2 | T011/T012/T013 [P] → T014 → T015 → T016 |
| US3 | T017 (seul) |
| US4 | T018 → T019 → T020 → T021 |
| US5 | T022 → T023/T024 [P] → T025 |

### Opportunités de parallélisme

- **Phase 1** : T002 et T003 peuvent s'exécuter simultanément (fichiers différents)
- **Phase 3** : T011, T012, T013 peuvent s'écrire simultanément dans le même fichier (fonctions distinctes, ajouter séquentiellement pour éviter les conflits)
- **Phase 6** : T023 et T024 peuvent s'exécuter simultanément
- **Phase 7** : T026 et T028 peuvent s'exécuter simultanément

---

## Exemple d'exécution parallèle : Phase 1

```bash
# Lancer simultanément :
Task A: "Ajouter DemandeCreeeResponse et MaDemandeResponse dans src/models/bibliotheque_humaine.rs"
Task B: "Créer src/models/admin/biblio_humaine.rs avec AdminDemandeBiblioRow, TraiterDemandeBody, AdminDemandeBiblioQueryParams"

# Ensuite séquentiellement (dépendent de A et B) :
Task C: "Déclarer pub mod biblio_humaine dans src/models/admin/mod.rs"
Task D: "Déclarer pub mod bibliotheques_humaines dans src/handlers/admin/mod.rs"
Task E: "Modifier inscrire_biblio dans src/handlers/bibliotheques_humaines.rs"
```

---

## Stratégie d'implémentation

### MVP : User Stories 1 + 2 seulement

1. Compléter Phase 1 (Fondation SQL & Modèles)
2. Compléter Phase 2 (US1 : Liste admin) → tester
3. Compléter Phase 3 (US2 : Valider/Rejeter) → tester
4. **ARRÊTER et VALIDER** : workflow admin complet fonctionnel
5. Livrer / démontrer

### Livraison incrémentale

| Étape | Phases | Valeur livrée |
|-------|--------|---------------|
| 1 | Phase 1 | Fondation prête (DDL + modèles) |
| 2 | + Phase 2 | Admin voit les demandes (US1) |
| 3 | + Phase 3 | Admin approuve/rejette (US2), **MVP complet** |
| 4 | + Phase 4 | Liste publique filtrée (US3) |
| 5 | + Phase 5 | Suivi candidat sur profil (US4) |
| 6 | + Phase 6 | Notifications in-app (US5) |

### Risques à surveiller

| Risque | Tâche concernée | Mitigation |
|--------|-----------------|------------|
| Race condition double soumission | T001 | Index UNIQUE `idx_demande_biblio_active_unique` |
| Régression listing public | T017 | Test quickstart étape 4 |
| Transaction validation partielle | T012 | `BEGIN/COMMIT` sqlx explicite avec rollback |
