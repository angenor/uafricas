# Tasks : Administrateurs de salle publique & propositions communautaires

**Feature** : `001-admin-salles-publiques`
**Branch** : `001-admin-salles-publiques`
**Inputs** : `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/public.md`, `contracts/admin.md`, `quickstart.md`

> Tests automatisés : **non requis** (CLAUDE.md : aucune infra de test configurée). Validation manuelle via `quickstart.md` à chaque fin de phase US.

---

## Phase 1 : Setup

- [X] T001 Vérifier l'état de la branche (`git status`) et que `docker compose up -d` lance bien PostgreSQL/Adminer ; relancer le backend proprement (`kill $(lsof -i :8080 -t) 2>/dev/null; cd uafricas_backend && RUST_LOG=info cargo run`), pas de modification de fichier.

---

## Phase 2 : Foundational (BLOQUE toutes les user stories)

### Schéma SQL (Principe III : SQL source de vérité)

- [X] T002 Étendre `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` : ajouter le `CREATE TYPE afrolang.statut_proposition_salle` (en_attente | validee | rejetee | retiree) en tête du fichier (après les autres enums existants).
- [X] T003 Étendre `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` : ajouter la table `afrolang.proposition_salle` avec les 4 CHECK contraintes et les 3 index (idx_proposition_salle_unique_attente, idx_proposition_salle_statut, idx_proposition_salle_auteur) tels que décrits dans `data-model.md` §2.
- [X] T004 Étendre `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` : ajouter la table `afrolang.salle_administrateur` avec le CHECK ck_admin_revocation_coherente et les 3 index (idx_salle_admin_unique_actif, idx_salle_admin_par_salle, idx_salle_admin_par_user) tels que décrits dans `data-model.md` §3.
- [X] T005 Étendre `uafricas_backend/doc/bd/schemas/13_contraintes_inter_schemas.sql` : ajouter les FK cross-schema sur `proposition_salle.auteur_id`, `proposition_salle.decideur`, `proposition_salle.groupe_ethnique_id`, `salle_administrateur.utilisateur_id`, `salle_administrateur.nomme_par`, `salle_administrateur.revoque_par` (toutes vers `iam.utilisateur` et `country_profile.groupe_ethnique`).
- [X] T006 Recréer la base : `docker compose down -v && docker compose up -d` puis vérifier dans Adminer que les nouvelles tables existent dans le schéma `afrolang`.

### Backend : modèles communs

- [X] T007 Étendre `uafricas_backend/src/models/afrolang.rs` : ajouter les structs `PropositionSalle` (FromRow, snake_case français), `PropositionStatut` (enum mappé sur `afrolang.statut_proposition_salle`), `SalleAdministrateur`, `AdministrateurLight`, et leurs DTO `Response` correspondants tels que décrits dans `contracts/public.md` et `contracts/admin.md`. Ajouter la `const COLONNES_PROPOSITION` et `const COLONNES_SALLE_ADMIN`.
- [X] T008 Étendre `uafricas_backend/src/models/afrolang.rs` : ajouter le DTO `SoumettrePropositionRequest`, `DecisionRequest` (champ `commentaire`), `NommerAdministrateurRequest` (`utilisateur_id`), `RevoquerAdministrateurRequest` (`motif`).
- [X] T009 Étendre la struct `SalleResponse` / `SalleDetailResponse` / `AdminSalleDetailResponse` existantes dans `uafricas_backend/src/models/afrolang.rs` avec un champ `pub administrateurs: Vec<AdministrateurLight>`.

### Backend : helper d'autorisation centralisé (FR-019)

- [X] T010 Ajouter dans `uafricas_backend/src/handlers/afrolang.rs` la fonction publique `pub async fn est_administrateur_salle(pool: &PgPool, salle_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error>` qui retourne `EXISTS(SELECT 1 FROM afrolang.salle_administrateur WHERE salle_id=$1 AND utilisateur_id=$2 AND actif=TRUE)`. Ajouter un commentaire de file pointant FR-019.

### Frontend : composables communs

- [X] T011 [P] Étendre `uafricas_frontend/app/composables/useAfrolang.ts` : ajouter les interfaces TS `PropositionSalle`, `StatutProposition`, `AdministrateurLight` (alignées avec `models/afrolang.rs` et `contracts/public.md`).
- [X] T012 [P] Étendre l'interface `SalleAPI` dans `uafricas_frontend/app/composables/useAfrolang.ts` avec `administrateurs: AdministrateurLight[]`.

**⚠️ Checkpoint** : T002–T010 doivent être terminées avant **toutes** les phases US. T011–T012 peuvent démarrer en parallèle dès T009 terminée.

---

## Phase 3 : User Story 1 (P1) : Proposer une salle publique en tant que membre

**Goal** : un utilisateur authentifié peut soumettre une proposition complète, la voir dans son espace perso, et est bloqué sur les doublons.

**Independent Test** : suivre `quickstart.md` Scénario 1 (création + tentative de doublon + accès non connecté).

### Backend

- [X] T013 [US1] Implémenter `pub async fn soumettre_proposition` dans `uafricas_backend/src/handlers/afrolang.rs` : valide les champs, vérifie qu'aucune `salle` active n'existe pour ce `groupe_ethnique_id` (409), vérifie le rate-limit anti-spam (≥ 5 rejets sur 7j → 429), vérifie que tous les `pays_origine_ids` existent et sont actifs, INSERT dans `proposition_salle`, appelle `audit::log_action('CREATE','afrolang','proposition_salle', id)`. Retourne `PropositionResponse` (201).
- [X] T014 [US1] Implémenter `pub async fn lister_mes_propositions` dans `uafricas_backend/src/handlers/afrolang.rs` : pagination + filtre `statut?`, restreint à `auteur_id = JWT current_user.id`, jointures auteur + groupe_ethnique + pays_origine + decideur, tri `created_at DESC`. Retourne `{ items, total, page, taille }`.
- [X] T015 [US1] Implémenter `pub async fn retirer_ma_proposition` dans `uafricas_backend/src/handlers/afrolang.rs` : `SELECT FOR UPDATE` ; refuse 403 si pas auteur, 409 si statut ≠ `en_attente` ; UPDATE statut=`retiree` ; `audit::log_action('UPDATE','afrolang','proposition_salle', id)`. Retourne 200.
- [X] T016 [US1] Étendre `uafricas_backend/src/routes.rs` : enregistrer les 3 routes `POST /api/afrolang/propositions`, `GET /api/afrolang/propositions/moi`, `PATCH /api/afrolang/propositions/{id}/retirer` derrière `auth_middleware`.

### Frontend : composable

- [X] T017 [US1] Étendre `uafricas_frontend/app/composables/useAfrolang.ts` avec les 3 méthodes : `proposerSalle(payload)`, `listerMesPropositions(filtres)`, `retirerProposition(id)`, mappant la réponse API vers les interfaces TS. Gérer les codes 409 et 429 avec messages utilisateur clairs.

### Frontend : composants & page (Tailwind v4 pur, principe VI)

- [X] T018 [P] [US1] Créer `uafricas_frontend/app/components/afrolang/PropositionSalleForm.vue` : formulaire (titre, description, justification, langue cible, langue code, sélection groupe ethnique, multi-sélection pays d'origine), validation côté client, états submitting/error/success, Tailwind v4 pur (pas de classes `btn`/`card` daisyUI).
- [X] T019 [P] [US1] Créer `uafricas_frontend/app/components/afrolang/PropositionSalleCard.vue` : carte affichant titre, statut (badge coloré), date, commentaire de décision si présent, bouton « Retirer » si `en_attente`. Tailwind v4 pur.
- [X] T020 [US1] Créer la page `uafricas_frontend/app/pages/afrolang/proposer.vue` : middleware auth requis ; deux sections, formulaire en haut (`PropositionSalleForm`), liste de mes propositions paginée en dessous (`PropositionSalleCard`). Si non connecté : redirect `/login?redirect=/afrolang/proposer`.
- [X] T021 [US1] Ajouter un lien « Proposer une salle » dans `uafricas_frontend/app/components/layout/NavBar.vue` (ou la zone Afrolang appropriée), visible uniquement pour les utilisateurs connectés.

**Checkpoint US1** : exécuter `quickstart.md` Scénario 1. ✅ FR-001..FR-007.

---

## Phase 4 : User Story 2 (P1) : Valider ou rejeter une proposition (admin plateforme)

**Goal** : l'administrateur de la plateforme peut traiter la file d'attente, valider (création atomique de la salle) ou rejeter (commentaire obligatoire).

**Independent Test** : `quickstart.md` Scénario 2.

### Backend : handlers admin

- [X] T022 [US2] Créer `uafricas_backend/src/handlers/admin/propositions_salle.rs` (nouveau fichier) ; déclarer `pub mod propositions_salle;` dans `uafricas_backend/src/handlers/admin/mod.rs`.
- [X] T023 [US2] Implémenter `pub async fn lister_propositions_admin` dans `propositions_salle.rs` : pagination + filtres (`statut`, `langue_code`, `groupe_ethnique_id`, `auteur_id`, `date_debut`, `date_fin`, `tri`), bornage `taille ≤ 100`, jointures complètes pour `PropositionResponse`.
- [X] T024 [US2] Implémenter `pub async fn obtenir_proposition_admin` dans `propositions_salle.rs` : 404 si introuvable, sinon `PropositionResponse` complet.
- [X] T025 [US2] Implémenter `pub async fn valider_proposition` dans `propositions_salle.rs` : transaction sqlx atomique conforme research.md Décision 3 : `SELECT FOR UPDATE` proposition, vérifs (`statut='en_attente'` 409, pas de `salle` active sur ce groupe ethnique 409), INSERT `salle` (cree_par = auteur de la proposition), INSERT `salle_pays_origine` pour chaque pays, UPDATE proposition (`statut='validee'`, `decideur`, `decide_at`, `salle_id_creee`, `commentaire_decision` si fourni). Hors transaction : `audit::log_action('VALIDATE','afrolang','proposition_salle', id)` + `audit::log_action('CREATE','afrolang','salle', salle_id)` + notification auteur (in-app + e-mail best-effort). Retourne `{ proposition, salle_id }`.
- [X] T026 [US2] Implémenter `pub async fn rejeter_proposition` dans `propositions_salle.rs` : valider `commentaire` ≥ 10 caractères (400 sinon), `SELECT FOR UPDATE`, refuser 409 si statut ≠ `en_attente`, UPDATE (`statut='rejetee'`, `decideur`, `decide_at`, `commentaire_decision`), `audit::log_action('REJECT','afrolang','proposition_salle', id)`, notification auteur.
- [X] T027 [US2] Étendre `uafricas_backend/src/routes.rs` : enregistrer les 4 routes admin (`GET /api/admin/afrolang/propositions`, `GET /api/admin/afrolang/propositions/{id}`, `PATCH .../valider`, `PATCH .../rejeter`) derrière `admin_middleware`.

### Frontend : composable admin

- [X] T028 [US2] Créer `uafricas_frontend/app/composables/useAdminPropositionsSalle.ts` (basé sur le pattern `useAdmin` avec `adminFetch`/`listerPagine`/`pagination`/`sort`) : `listerPropositions(filtres)`, `obtenirProposition(id)`, `validerProposition(id, commentaire?)`, `rejeterProposition(id, commentaire)`.

### Frontend : composants & pages admin (daisyUI v5 autorisé)

- [X] T029 [P] [US2] Créer `uafricas_frontend/app/components/admin/afrolang/PropositionRow.vue` : ligne de tableau avec titre, auteur, groupe ethnique, statut (badge daisyUI), date, lien « Détail ».
- [X] T030 [P] [US2] Créer `uafricas_frontend/app/components/admin/afrolang/PropositionDetail.vue` : panneau détail avec toutes les infos + 2 actions (modal Valider avec commentaire facultatif, modal Rejeter avec commentaire obligatoire ≥ 10 car.). Vue lecture seule si statut ≠ `en_attente` (afficher décideur + date + commentaire).
- [X] T031 [US2] Créer la page `uafricas_frontend/app/pages/admin/afrolang/propositions/index.vue` : tableau filtrable (statut, groupe ethnique, auteur, dates) + pagination, utilisant `PropositionRow`.
- [X] T032 [US2] Créer la page `uafricas_frontend/app/pages/admin/afrolang/propositions/[id].vue` : détail + actions, utilisant `PropositionDetail`.
- [X] T033 [US2] Ajouter une entrée « Propositions de salles » dans le menu admin Afrolang (`uafricas_frontend/app/components/admin/...` selon emplacement existant) pointant vers `/admin/afrolang/propositions`.

**Checkpoint US2** : exécuter `quickstart.md` Scénario 2. ✅ FR-008..FR-012, SC-003.

> **MVP atteint** après T033 : la boucle proposition/décision est fonctionnelle de bout en bout.

---

## Phase 5 : User Story 3 (P2) : Nommer un administrateur de salle publique

**Goal** : l'admin plateforme nomme/révoque des administrateurs sur une salle publique ; la liste apparaît publiquement sur la fiche de la salle ; un audit complet est conservé.

**Independent Test** : `quickstart.md` Scénario 3.

### Backend : extension `GET salles` (visibilité publique)

- [X] T034 [US3] Modifier `uafricas_backend/src/handlers/afrolang.rs` `lister_salles` et `obtenir_salle` : ajouter un `LEFT JOIN LATERAL` ou un sous-`SELECT json_agg` peuplant le champ `administrateurs` (filtre `actif=TRUE`, projection `AdministrateurLight`). Réutiliser le même pattern que `pays_origine` (feature 001-afrolang-pays-origine) pour cohérence.

### Backend : handlers nomination/révocation

- [X] T035 [US3] Implémenter dans `uafricas_backend/src/handlers/admin/salles.rs` (fichier existant, vérifier le nom exact du module gérant `salle_publique`/`salle` côté admin) la fonction `pub async fn nommer_administrateur_salle` : 404 si salle inactive/supprimée ou utilisateur introuvable/inactif, 409 si nomination active déjà existante, INSERT `salle_administrateur` (`actif=TRUE`, `nomme_par`), `audit::log_action('CREATE','afrolang','salle_administrateur', id)`, notification utilisateur. Retourne 201 avec `SalleAdministrateurResponse`.
- [X] T036 [US3] Implémenter `pub async fn revoquer_administrateur_salle` : `SELECT FOR UPDATE` sur la ligne `actif=TRUE` du couple (404 sinon), UPDATE (`actif=FALSE`, `revoque_at`, `revoque_par`, `motif_revocation`), `audit::log_action('UPDATE','afrolang','salle_administrateur', id)`, notification utilisateur.
- [X] T037 [US3] Implémenter `pub async fn lister_administrateurs_salle` : retourne historique complet (actif + inactif), tri `nomme_at DESC`, jointures `iam.utilisateur` pour `utilisateur`/`nomme_par`/`revoque_par`.
- [X] T038 [US3] Étendre `uafricas_backend/src/routes.rs` : enregistrer `POST /api/admin/afrolang/salles/{salle_id}/administrateurs`, `DELETE /api/admin/afrolang/salles/{salle_id}/administrateurs/{utilisateur_id}`, `GET /api/admin/afrolang/salles/{salle_id}/administrateurs` derrière `admin_middleware`.

### Frontend : composable admin

- [X] T039 [US3] Étendre `uafricas_frontend/app/composables/useAdminAfrolangSalles.ts` : `listerAdministrateurs(salleId)`, `nommerAdministrateur(salleId, utilisateurId)`, `revoquerAdministrateur(salleId, utilisateurId, motif)`.

### Frontend : composants & pages

- [X] T040 [P] [US3] Créer `uafricas_frontend/app/components/admin/afrolang/SalleAdministrateursPanel.vue` (daisyUI) : liste actuelle + recherche utilisateur + bouton « Nommer » + bouton « Révoquer » par ligne (modal motif), vue historique (lignes inactives grisées).
- [X] T041 [US3] Étendre `uafricas_frontend/app/pages/admin/salles/[id].vue` (ou `pages/admin/afrolang/salles/[id].vue` selon l'emplacement existant) : ajouter un onglet « Administrateurs » utilisant `SalleAdministrateursPanel`. Distinguer visuellement du panneau « Modérateurs attitrés » existant (FR-018).
- [X] T042 [P] [US3] Créer `uafricas_frontend/app/components/afrolang/SalleAdministrateursWidget.vue` (Tailwind v4 pur) : badge « Administrateurs de la salle » + chips utilisateur (nom, prénom, photo). Distinct visuellement de tout badge « Admin plateforme ».
- [X] T043 [US3] Intégrer `SalleAdministrateursWidget` dans la fiche publique de salle (`uafricas_frontend/app/pages/afrolang/salle/[id].vue` ou `pages/afrolang/index.vue` selon emplacement existant), section visible si `salle.administrateurs.length > 0`.

**Checkpoint US3** : exécuter `quickstart.md` Scénario 3. ✅ FR-013..FR-018, FR-020.

---

## Phase 6 : Polish & Cross-cutting (Cascades, anti-spam, audit, doc)

### Cascades automatiques (FR-021, FR-022, SC-008)

- [X] T044 Étendre le handler admin existant qui désactive/archive une salle Afrolang (probablement dans `uafricas_backend/src/handlers/admin/salles.rs` ou `afrolang.rs` admin) : après mise à jour `salle.actif=FALSE`, exécuter `UPDATE afrolang.salle_administrateur SET actif=FALSE, suspendu_at=NOW(), motif_suspension='salle_archivee', updated_at=NOW() WHERE salle_id=$1 AND actif=TRUE` ; pour chaque ligne affectée, `audit::log_action('UPDATE','afrolang','salle_administrateur', id)` (motif dans le before/after).
- [X] T045 Étendre le handler admin IAM existant qui change l'`etat` d'un utilisateur (`uafricas_backend/src/handlers/admin/utilisateurs.rs`) : si nouveau `etat ∈ {suspendu, supprime, inactif}`, exécuter `UPDATE afrolang.salle_administrateur SET actif=FALSE, suspendu_at=NOW(), motif_suspension='compte_desactive', updated_at=NOW() WHERE utilisateur_id=$1 AND actif=TRUE` + audit par ligne.

### Anti-spam (Décision 6 research.md)

- [X] T046 Vérifier (et implémenter si manquante) la requête anti-spam dans `soumettre_proposition` (T013) : `SELECT COUNT(*) FROM afrolang.proposition_salle WHERE auteur_id=$1 AND statut='rejetee' AND decide_at > NOW() - INTERVAL '7 days'` ; si ≥ 5 → 429 avec champ `reessayer_apres = max(decide_at) + 7 jours`. Si déjà fait dans T013, juste cocher.

### Distinction visuelle (FR-018, SC-006)

- [X] T047 Auditer toutes les pages publiques et admin où apparaissent des badges « Administrateur » : vérifier que le badge « Admin de la salle » utilise une couleur/libellé distincts du badge « Admin plateforme ». Mettre à jour la légende/tooltip si nécessaire.

### Documentation

- [X] T048 Mettre à jour `CLAUDE.md` (section *Recent Changes*) avec une entrée résumant la feature : tables ajoutées, endpoints publics/admin, composants frontend, cascades.
- [ ] T049 Vérifier que `specs/001-admin-salles-publiques/quickstart.md` est exécuté de bout en bout (5 scénarios) sans bug ; consigner toute correction nécessaire dans une nouvelle PR de hotfix si besoin.

---

## Dependencies

```
Phase 1 (T001)
   ↓
Phase 2 (T002→T003→T004→T005→T006) : SQL
   ↓
Phase 2 (T007, T008, T009, T010), Backend foundations [T011, T012 en parallèle après T009]
   ↓
   ├──→ Phase 3 US1 (T013→T014→T015→T016→T017→[T018, T019, T020 // T020 dépend de T018+T019]→T021)
   │
   └──→ Phase 4 US2 (T022→T023→T024→T025→T026→T027→T028→[T029, T030 //]→T031→T032→T033)
                                                                              ↓
                                                                  ── MVP livré ──
                                                                              ↓
                                              Phase 5 US3 (T034→T035→T036→T037→T038→T039→[T040, T042 //]→T041→T043)
                                                                              ↓
                                                                  Phase 6 Polish (T044→T045 // T046 // T047 // T048→T049)
```

**Indépendance des stories** : US1 et US2 forment un couple obligatoire (US1 sans US2 ne livre rien d'utile : les propositions resteraient bloquées en file d'attente). US3 est strictement indépendante de US1/US2 *sur le plan technique* (table dédiée), mais elle suppose qu'au moins une salle publique existe pour être nommable : donc en pratique, dérouler après US1+US2.

---

## Parallel Execution Examples

### Phase 2 (après T009)
```
T010 (helper Rust) ⇄ T011 (interfaces TS) ⇄ T012 (extension SalleAPI)
```

### Phase 3 (après T017)
```
T018 PropositionSalleForm.vue ⇄ T019 PropositionSalleCard.vue
```

### Phase 4 (après T028)
```
T029 PropositionRow.vue ⇄ T030 PropositionDetail.vue
```

### Phase 5 (après T039)
```
T040 SalleAdministrateursPanel.vue (admin, daisyUI) ⇄ T042 SalleAdministrateursWidget.vue (public, Tailwind v4 pur)
```

### Phase 6
```
T044 cascade salle ⇄ T045 cascade compte ⇄ T046 anti-spam ⇄ T047 distinction visuelle
```

---

## Implementation Strategy

1. **MVP (US1 + US2)** : T001 → T033. À ce stade, la boucle proposer→valider/rejeter est complète et utilisable en production. SC-001 à SC-004 et SC-007 sont mesurables.
2. **Itération 2 (US3)** : T034 → T043. Ajoute la nomination d'administrateurs de salle (sans pouvoirs effectifs : c'est volontaire, FR-019).
3. **Polish** : T044 → T049. Cascades, anti-spam confirmé, audit visuel des badges, documentation.

---

## Validation Checklist

- [x] Toutes les tâches respectent le format `- [ ] T### [P?] [US?] description avec chemin de fichier`.
- [x] Chaque US a au minimum : modèle SQL (Phase 2), handler backend, composable, composants UI, intégration dans une page.
- [x] `Independent Test` documenté pour chaque US (renvoi vers `quickstart.md`).
- [x] Aucun test automatisé requis (cf. CLAUDE.md « no testing or CI/CD configured »).
- [x] Principes constitutionnels respectés tout du long (FR : I, monorepo : II, SQL d'abord : III, sécurité auth : IV, simplicité : V, daisyUI back-office uniquement : VI, audit systématique : VII).
