# Tasks: Retrouve Amis

**Input**: Design documents from `/specs/001-retrouve-amis/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/api-publique.md, contracts/api-admin.md

**Tests**: Non demandés : aucun framework de test configuré (conformément aux contraintes techniques du projet).

**Organization**: Tasks groupées par user story pour permettre l'implémentation et le test indépendants de chaque story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut s'exécuter en parallèle (fichiers différents, pas de dépendances)
- **[Story]**: User story associée (US1, US2, US3, US4, US5)
- Chemins exacts inclus dans les descriptions

## Path Conventions

- **Backend**: `uafricas_backend/src/`, `uafricas_backend/doc/bd/schemas/`
- **Frontend**: `uafricas_frontend/app/`

---

## Phase 1: Setup (Infrastructure Partagée)

**Purpose**: Création du schema SQL et enregistrement dans l'orchestrateur. Source de vérité pour tout le reste.

- [x] T001 Créer le fichier schema SQL `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`, créer le schema `retrouve_amis`, les 7 enums (`etat_avis`, `etat_correspondance`, `type_cible`, `motif_signalement`, `etat_signalement`, `type_parcours`, `type_notification`), les 6 tables (`avis_recherche`, `correspondance`, `parcours_trouvable`, `blacklist`, `signalement`, `notification_retrouve`) avec tous les index (GIN trigramme, GIN FTS, B-tree avec filtres partiels), CHECK constraints, index d'unicité, et un trigger `tsvector_update_trigger` sur `avis_recherche` pour populer `search_vector` à partir de `nom_recherche`, `prenom_recherche`, `surnom`, `ecole`, `ville`, `description` (configuration `'french'`), conformément à `data-model.md`
- [x] T002 Créer la fonction de matching `retrouve_amis.calculer_correspondances(p_avis_id UUID)` dans `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`, scoring multi-critères (nom 40pts via `pg_trgm`+`unaccent`, école 20pts, ville 15pts, période 15pts chevauchement, pays 10pts), exclusion auto-correspondance/blacklist/correspondances actives. **Deux branches de matching** : 1) avis↔avis (comparer les champs de l'avis source avec les champs des autres avis actifs), 2) avis↔profil trouvable (jointure `iam.utilisateur` WHERE `est_trouvable=true` + `retrouve_amis.parcours_trouvable` : scorer nom_recherche↔nom/prenom utilisateur 40pts, ecole↔parcours.nom WHERE type_entree='ecole' 20pts, ville↔parcours.nom WHERE type_entree='ville_residence' OU parcours.ville 15pts, pays_id↔parcours.pays_id 10pts, période↔parcours.periode_debut/fin 15pts, prendre le meilleur score parmi les entrées de parcours), conformément à `research.md` R2
- [x] T003 Ajouter `\ir schemas/16_retrouve_amis.sql` dans `uafricas_backend/doc/bd/schema.sql` (orchestrateur), après le fichier 15 et avant `13_contraintes_inter_schemas.sql`
- [x] T004 Modifier `uafricas_backend/doc/bd/schemas/13_contraintes_inter_schemas.sql`, ajouter les FK cross-schema : `avis_recherche.auteur_id → iam.utilisateur`, `avis_recherche.pays_id → shared.pays`, `parcours_trouvable.utilisateur_id → iam.utilisateur`, `parcours_trouvable.pays_id → shared.pays`, `correspondance.cible_utilisateur_id → iam.utilisateur`, `blacklist.utilisateur_a_id/b_id → iam.utilisateur`, `signalement.signale_par/modere_par → iam.utilisateur`, `notification_retrouve.utilisateur_id → iam.utilisateur`
- [x] T005 Ajouter la colonne `est_trouvable BOOLEAN NOT NULL DEFAULT FALSE` sur `iam.utilisateur`, dans le fichier SQL IAM existant ou via ALTER dans `16_retrouve_amis.sql`
- [x] T006 Ajouter `'retrouve_amis'` à la liste des schemas dans le trigger auto-update `uafricas_backend/doc/bd/schemas/14_triggers.sql`

**Validation** : `docker compose down -v && docker compose up -d` puis vérifier via Adminer (http://localhost:8088) que les 6 tables, 7 enums, la fonction de matching et la colonne `est_trouvable` existent.

---

## Phase 2: Fondations (Prérequis Bloquants)

**Purpose**: Models et handlers Rust de base + composables frontend de base. DOIT être complété avant toute user story.

**⚠️ CRITIQUE**: Aucune user story ne peut commencer avant la fin de cette phase.

### Backend : Models

- [x] T007 [P] Créer `uafricas_backend/src/models/retrouve_amis.rs`, structs publiques : `AvisRecherche` (FromRow), `AvisRechercheDetail` (avec correspondances imbriquées), `AvisRechercheResponse`, `Correspondance` (FromRow), `CorrespondanceDetail`, `CorrespondanceResponse` (avec `resume_anonymise`, `mon_role`), `NotificationRetrouve` (FromRow), `ParcoursTrouvable` (FromRow), `TableauDeBord`, DTOs : `CreerAvisRecherche`, `ModifierAvisRecherche`, `CreerParcours`, `ModifierParcours`, `AccepterCorrespondance`, `SignalerAvis`, `CoordonneesChoix`, conformément à `contracts/api-publique.md`
- [x] T008 [P] Créer `uafricas_backend/src/models/admin/retrouve_amis.rs`, structs admin : `AdminAvisRecherche` (FromRow avec jointure auteur), `AdminAvisRechercheDetail` (avec correspondances + signalements), `AdminSignalement` (FromRow avec jointures), `AdminSignalementDetail`, `AdminStatistiques`, DTOs : `ChangerEtatAvis`, `ModererSignalement`, conformément à `contracts/api-admin.md`
- [x] T009 Déclarer les modules dans `uafricas_backend/src/models/mod.rs` (`pub mod retrouve_amis;`) et `uafricas_backend/src/models/admin/mod.rs` (`pub mod retrouve_amis;`)
- [x] T010 Ajouter le champ `est_trouvable: bool` à la struct `Utilisateur` existante dans `uafricas_backend/src/models/` (fichier utilisateur existant)

### Backend : Structure routes

- [x] T011 Déclarer les modules handlers dans `uafricas_backend/src/handlers/mod.rs` (`pub mod retrouve_amis;`) et `uafricas_backend/src/handlers/admin/mod.rs` (`pub mod retrouve_amis;`)
- [x] T012 Ajouter les scopes de routes dans `uafricas_backend/src/routes.rs`, scope `/api/retrouve-amis` (avis CRUD, correspondances, notifications, tableau de bord) + scope `/api/profil` (trouvable, parcours CRUD) + scope `/api/admin/retrouve-amis` (avis admin, signalements, statistiques), conformément aux deux contrats API

### Frontend : Composables

- [x] T013 [P] Créer `uafricas_frontend/app/composables/useRetrouvAmis.ts`, types TypeScript (`AvisRecherche`, `Correspondance`, `NotificationRetrouve`, `ParcoursTrouvable`, `TableauDeBord`, DTOs, enums), constantes, composable avec méthodes : `creerAvis`, `listerAvis`, `detailAvis`, `modifierAvis`, `cloturerAvis`, `listerCorrespondances`, `detailCorrespondance`, `accepterCorrespondance`, `refuserCorrespondance`, `signalerAvis`, `listerNotifications`, `marquerLu`, `toutMarquerLu`, `tableauDeBord`, `basculerTrouvable`, `listerParcours`, `ajouterParcours`, `modifierParcours`, `supprimerParcours`, pattern $fetch existant
- [x] T014 [P] Créer `uafricas_frontend/app/composables/useAdminRetrouvAmis.ts`, étend `useAdmin()` comme base, méthodes : `listerAvis`, `detailAvis`, `changerEtatAvis`, `listerSignalements`, `detailSignalement`, `modererSignalement`, `statistiques`, conformément à `contracts/api-admin.md`

**Checkpoint**: Fondations prêtes : l'implémentation des user stories peut commencer.

---

## Phase 3: User Story 1 : Déposer un avis de recherche (Priority: P1) 🎯 MVP

**Goal**: Un utilisateur connecté peut créer, modifier, consulter et clôturer des avis de recherche. Le matching est déclenché automatiquement à la création/modification.

**Independent Test**: Créer un avis de recherche, vérifier qu'il est enregistré, visible dans la liste, modifiable et clôturable. Vérifier la validation (champs obligatoires, limite 10 avis actifs).

### Implementation for User Story 1

- [x] T015 [US1] Créer `uafricas_backend/src/handlers/retrouve_amis.rs`, implémenter les handlers CRUD avis de recherche : `creer_avis` (POST, validation nom_recherche + au moins 1 critère supplémentaire parmi prenom/ecole/ville/pays/periode, vérification limite 10 avis actifs, insertion + appel `calculer_correspondances` + insertion correspondances score >= 60 + création notifications, retourne `correspondances_trouvees`), `lister_avis` (GET, filtré par auteur JWT, pagination/tri/filtre état), `detail_avis` (GET par id, vérification auteur, jointure correspondances avec résumé anonymisé), `modifier_avis` (PUT, vérification auteur + état actif, suppression correspondances `en_attente` existantes, relance matching), `cloturer_avis` (PATCH, transition actif→cloture), audit::log_action sur mutations
- [x] T016 [P] [US1] Créer `uafricas_frontend/app/components/retrouve-amis/AvisRechercheForm.vue`, formulaire multi-étapes (5 étapes) : 1) nom/prénom/surnom, 2) école/université, 3) ville/pays (sélection pays via API existante), 4) période début/fin, 5) description + récapitulatif, validation progressive, mode création et modification, Tailwind CSS v4 pur (pas de daisyUI)
- [x] T017 [P] [US1] Créer `uafricas_frontend/app/components/retrouve-amis/AvisRechercheCard.vue`, carte résumée d'un avis : nom recherché, ville, période, état (badge couleur), nombre de correspondances, dates, actions : modifier, clôturer, Tailwind CSS v4 pur
- [x] T018 [P] [US1] Créer `uafricas_frontend/app/components/retrouve-amis/RetrouvAmisHero.vue`, section hero de la page d'accueil fonctionnalité : titre, description, illustration, CTA "Créer un avis de recherche" et "Activer trouvable", Tailwind CSS v4 pur
- [x] T019 [US1] Créer `uafricas_frontend/app/pages/retrouve-amis/index.vue`, page d'accueil fonctionnalité : hero (RetrouvAmisHero), explication du fonctionnement, statistiques simples (si connecté : mini tableau de bord), CTA vers nouveau.vue et mes-recherches.vue, layout default
- [x] T020 [US1] Créer `uafricas_frontend/app/pages/retrouve-amis/nouveau.vue`, page de création d'avis : utilise AvisRechercheForm en mode création, redirection vers mes-recherches.vue après succès avec message de confirmation (nombre de correspondances trouvées), middleware auth
- [x] T021 [US1] Créer `uafricas_frontend/app/pages/retrouve-amis/mes-recherches.vue`, liste des avis de l'utilisateur : AvisRechercheCard en grille, filtre par état (actif/clôturé/suspendu), tri par date, pagination, message vide si pas d'avis, CTA vers nouveau.vue, middleware auth

**Checkpoint**: US1 complète : un utilisateur peut créer, consulter, modifier et clôturer ses avis de recherche.

---

## Phase 4: User Story 2 : Recevoir une notification de correspondance (Priority: P1)

**Goal**: Le système détecte les correspondances (score >= 60%), crée des notifications, et l'utilisateur peut consulter un résumé anonymisé de chaque correspondance.

**Independent Test**: Créer deux avis complémentaires (A cherche B, B cherche A) et vérifier que les deux reçoivent une notification. Vérifier le résumé anonymisé (initiales, ville, période, score %).

### Implementation for User Story 2

- [x] T022 [US2] Ajouter les handlers notifications dans `uafricas_backend/src/handlers/retrouve_amis.rs`, `lister_notifications` (GET, filtré par utilisateur JWT, pagination, filtre lu/non-lu, retourne `non_lues`), `marquer_lu` (PATCH par id), `tout_marquer_lu` (PATCH, retourne `mises_a_jour`), la logique de création de notifications est déjà dans le handler `creer_avis` (T015)
- [x] T023 [US2] Ajouter les handlers correspondances (listing) + signalement + archivage dans `uafricas_backend/src/handlers/retrouve_amis.rs`, `lister_correspondances` (GET, correspondances où l'utilisateur est auteur de l'avis OU cible trouvable, pagination/filtre état/avis_id, résumé anonymisé avec initiales+ville+période+critères communs, champ `mon_role`, **lazy archival** : UPDATE SET etat='archivee' WHERE etat IN ('en_attente','acceptee_a','acceptee_b') AND created_at < NOW() - INTERVAL '30 days' + création notification `correspondance_archivee`), `detail_correspondance` (GET par id, vérification participation, `details_score` détaillé, `coordonnees_partagees` si état mutuelle), `signaler_avis` (POST, signalement d'un avis via une correspondance reçue, l'utilisateur doit avoir une correspondance avec l'avis pour pouvoir le signaler, validation motif, insertion signalement, unicité avis+signaleur), audit::log_action sur signalement
- [x] T024 [P] [US2] Créer `uafricas_frontend/app/components/retrouve-amis/CorrespondanceCard.vue`, carte anonymisée : initiales (pas le nom complet), ville, période, score % (badge couleur), état, type cible (avis/profil), rôle (auteur/cible), date, expiration, Tailwind CSS v4 pur
- [x] T025 [P] [US2] Créer `uafricas_frontend/app/components/retrouve-amis/ScoreBadge.vue`, badge visuel du score de correspondance : couleur verte >= 80%, jaune >= 60%, affichage pourcentage, Tailwind CSS v4 pur
- [x] T026 [US2] Créer `uafricas_frontend/app/pages/retrouve-amis/correspondances.vue`, liste des correspondances de l'utilisateur : CorrespondanceCard en grille, filtres (état, avis source), tri par score décroissant ou date, pagination, compteur de notifications non lues, polling 60s pour nouvelles notifications, middleware auth

**Checkpoint**: US2 complète : les correspondances sont détectées, notifiées, et consultables avec résumé anonymisé.

---

## Phase 5: User Story 3 : Accepter le contact et partager ses coordonnées (Priority: P2)

**Goal**: Flux de consentement mutuel (double opt-in) : accepter/refuser une correspondance, partage de coordonnées après consentement mutuel, blacklist automatique après refus.

**Independent Test**: Avec deux utilisateurs ayant une correspondance active, tester le flux complet : A accepte → notification à B → B accepte → état mutuelle → coordonnées partagées. Tester aussi le refus → blacklist.

### Implementation for User Story 3

- [x] T027 [US3] Ajouter les handlers accepter/refuser dans `uafricas_backend/src/handlers/retrouve_amis.rs`, `accepter_correspondance` (POST par id, vérification participation, transition état en_attente→acceptee_a/acceptee_b selon rôle, si les deux ont accepté→mutuelle + stocker coordonnées JSONB, création notification `acceptation_contact` ou `coordonnees_partagees`), `refuser_correspondance` (POST par id, transition→declinee, insertion blacklist symétrique LEAST/GREATEST, création notification), audit::log_action sur mutations, Note : le handler `signaler_avis` et l'archivage automatique sont déjà implémentés dans T023 (US2)
- [x] T028 [P] [US3] Créer `uafricas_frontend/app/components/retrouve-amis/CorrespondanceDetail.vue`, détail d'une correspondance : résumé anonymisé étendu, détails du score par critère (barres de progression), état actuel avec timeline visuelle des transitions, boutons accepter/refuser (si en_attente ou acceptée par l'autre), formulaire choix coordonnées (email/téléphone/messagerie), affichage coordonnées partagées si état mutuelle, Tailwind CSS v4 pur
- [x] T029 [US3] Créer `uafricas_frontend/app/pages/retrouve-amis/correspondances/[id].vue`, page détail correspondance : utilise CorrespondanceDetail, gestion des états (en_attente, acceptée, mutuelle, déclinée, archivée), messages de confirmation après action, redirection si accès non autorisé, middleware auth

**Checkpoint**: US3 complète : le flux complet de consentement mutuel fonctionne, avec blacklist automatique et archivage.

---

## Phase 6: User Story 4 : Gérer la visibilité de son profil (Priority: P2)

**Goal**: Un utilisateur peut activer/désactiver "je suis trouvable" et gérer son parcours (écoles, villes passées) pour améliorer le matching.

**Independent Test**: Activer trouvable, ajouter des entrées de parcours, vérifier que le profil apparaît dans les résultats de matching. Désactiver et vérifier que les correspondances basées sur le profil sont annulées.

### Implementation for User Story 4

- [x] T030 [US4] Ajouter les handlers profil trouvable dans `uafricas_backend/src/handlers/retrouve_amis.rs`, `basculer_trouvable` (PATCH, toggle `est_trouvable` sur `iam.utilisateur`, si activation → appeler `calculer_correspondances` pour le profil contre tous les avis actifs, si désactivation → annuler correspondances en_attente basées sur ce profil), `lister_parcours` (GET, par utilisateur JWT), `ajouter_parcours` (POST, validation type_entree + nom + période), `modifier_parcours` (PUT par id, vérification propriétaire), `supprimer_parcours` (DELETE par id, vérification propriétaire), audit::log_action sur mutations
- [x] T031 [P] [US4] Créer `uafricas_frontend/app/components/retrouve-amis/ProfilTrouvableForm.vue`, formulaire parcours : toggle "je suis trouvable" avec explication, liste des entrées de parcours (écoles/villes), formulaire ajout/modification d'une entrée (type, nom, ville, pays, période début/fin), suppression avec confirmation, Tailwind CSS v4 pur
- [x] T032 [US4] Modifier `uafricas_frontend/app/pages/profil.vue`, ajouter une section "Retrouve Amis" avec le composant ProfilTrouvableForm, afficher le statut trouvable actuel, nombre d'entrées de parcours

**Checkpoint**: US4 complète : l'utilisateur peut gérer sa visibilité et son parcours trouvable.

---

## Phase 7: User Story 5 : Consulter et gérer son tableau de bord (Priority: P3)

**Goal**: Tableau de bord dédié avec résumé global (compteurs avis, correspondances, notifications) et navigation rapide.

**Independent Test**: Vérifier que les compteurs reflètent correctement le nombre d'avis actifs/clôturés, correspondances par état, et notifications non lues.

### Implementation for User Story 5

- [x] T033 [US5] Ajouter le handler tableau de bord dans `uafricas_backend/src/handlers/retrouve_amis.rs`, `tableau_de_bord` (GET, requêtes agrégées : COUNT avis par état, COUNT correspondances par état, COUNT notifications non lues, est_trouvable, COUNT parcours), conformément à `contracts/api-publique.md` section Tableau de bord
- [x] T034 [P] [US5] Créer `uafricas_frontend/app/components/retrouve-amis/TableauDeBord.vue`, dashboard résumé : cartes compteurs (avis actifs, clôturés, correspondances en attente, mutuelles, notifications non lues), statut trouvable, liens rapides vers mes-recherches, correspondances, profil, Tailwind CSS v4 pur
- [x] T035 [US5] Intégrer le composant TableauDeBord dans `uafricas_frontend/app/pages/retrouve-amis/index.vue`, afficher le tableau de bord si l'utilisateur est connecté (en dessous du hero), masquer si non connecté

**Checkpoint**: US5 complète : le tableau de bord résume toute l'activité Retrouve Amis de l'utilisateur.

---

## Phase 8: Admin : Modération et Signalements

**Purpose**: Pages d'administration pour la modération des avis et signalements. Concerne FR-011, FR-016, SC-006.

### Backend Admin

- [x] T036 Créer `uafricas_backend/src/handlers/admin/retrouve_amis.rs`, handlers admin : `lister_avis_admin` (GET, filtres recherche/état/auteur/pays/dates, pagination/tri, jointures auteur + compteurs correspondances/signalements), `detail_avis_admin` (GET par id, jointures complètes auteur + correspondances + signalements), `changer_etat_avis` (PATCH, transitions actif↔suspendu), `lister_signalements` (GET, filtres état/motif, pagination/tri, jointures avis + signaleur), `detail_signalement` (GET par id), `moderer_signalement` (PATCH, decision approuve→suspend avis / rejete→inchangé, `modere_par` = admin JWT, `modere_at` = NOW()), `statistiques` (GET, compteurs globaux), audit::log_action sur toutes les mutations, conformément à `contracts/api-admin.md`

### Frontend Admin

- [x] T037 [P] Créer `uafricas_frontend/app/pages/admin/retrouve-amis/index.vue`, liste paginée des avis (AdminDataTable), colonnes : auteur, nom recherché, ville, état, nb correspondances, nb signalements, date, filtres : recherche texte, état, pays, tri sur colonnes, actions : voir détail, suspendre/réactiver, daisyUI
- [x] T038 [P] Créer `uafricas_frontend/app/pages/admin/retrouve-amis/[id].vue`, détail admin d'un avis : informations complètes, auteur (lien profil), liste correspondances associées, liste signalements, actions modération (suspendre/réactiver), daisyUI
- [x] T039 [P] Créer `uafricas_frontend/app/pages/admin/retrouve-amis/signalements.vue`, liste paginée des signalements (AdminDataTable), colonnes : avis concerné, auteur signalement, motif, état, date, filtres : état, motif, actions : voir détail, approuver (suspend l'avis), rejeter, daisyUI

---

## Phase 9: Polish & Intégration Cross-Cutting

**Purpose**: Navigation, intégration dans le layout existant, vérifications finales.

- [x] T040 Ajouter le lien "Retrouve Amis" dans la NavBar `uafricas_frontend/app/components/layout/NavBar.vue`, icône FontAwesome appropriée (fa-users ou fa-search), lien vers `/retrouve-amis`, badge notifications non lues (polling 60s si connecté)
- [x] T041 Ajouter la section "Retrouve Amis" dans le sidebar admin `uafricas_frontend/app/components/admin/AdminSidebar.vue`, liens vers : Avis de recherche (`/admin/retrouve-amis`), Signalements (`/admin/retrouve-amis/signalements`)
- [x] T042 Ajouter les icônes FontAwesome nécessaires dans `uafricas_frontend/app/plugins/fontawesome.ts`, ajouter dans `library.add()` les icônes utilisées par les composants retrouve-amis
- [x] T043 Vérifier la cohérence des types cross-stack, s'assurer que les interfaces TypeScript dans `useRetrouvAmis.ts` et `useAdminRetrouvAmis.ts` correspondent exactement aux structs Rust et au schema SQL
- [x] T044 Valider le quickstart.md : suivre les étapes de `specs/001-retrouve-amis/quickstart.md` de bout en bout pour vérifier que tout fonctionne

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup SQL)**: Pas de dépendances, commencer immédiatement
- **Phase 2 (Fondations)**: Dépend de Phase 1 (schema SQL doit exister), BLOQUE toutes les user stories
- **Phase 3 (US1)**: Dépend de Phase 2 : MVP minimum
- **Phase 4 (US2)**: Dépend de Phase 2 + la logique de matching de T015 (Phase 3), implémentable en parallèle de US1 côté frontend si le backend US1 est prêt
- **Phase 5 (US3)**: Dépend de Phase 4 (correspondances doivent exister pour accepter/refuser)
- **Phase 6 (US4)**: Dépend de Phase 2 : indépendante des autres US côté profil
- **Phase 7 (US5)**: Dépend de Phase 3 + Phase 4 (compteurs requièrent données)
- **Phase 8 (Admin)**: Dépend de Phase 2, peut commencer en parallèle des user stories
- **Phase 9 (Polish)**: Dépend de toutes les phases précédentes

### User Story Dependencies

- **US1 (P1)**: Fondations → implémentable immédiatement
- **US2 (P1)**: Fondations + backend US1 (matching) → frontend parallélisable
- **US3 (P2)**: US2 complète (correspondances doivent exister)
- **US4 (P2)**: Fondations → indépendante, parallélisable avec US1/US2
- **US5 (P3)**: US1 + US2 (données nécessaires pour compteurs)

### Within Each User Story

- Backend handler avant frontend pages (API doit exister)
- Composants réutilisables avant pages (les pages utilisent les composants)
- Core implémentation avant intégration

### Parallel Opportunities

- T007 + T008 (models publics et admin) en parallèle
- T013 + T014 (composables public et admin) en parallèle
- T016 + T017 + T018 (composants frontend US1) en parallèle
- T024 + T025 (composants frontend US2) en parallèle
- T037 + T038 + T039 (pages admin) en parallèle
- US4 en parallèle de US1/US2 (domaines indépendants)
- Phase 8 (Admin) en parallèle des user stories

---

## Parallel Example: User Story 1

```bash
# Lancer les composants frontend US1 en parallèle :
Task: "Créer AvisRechercheForm.vue"     # T016
Task: "Créer AvisRechercheCard.vue"      # T017
Task: "Créer RetrouvAmisHero.vue"        # T018

# Puis les pages séquentiellement (utilisent les composants) :
Task: "Créer index.vue"                  # T019
Task: "Créer nouveau.vue"               # T020
Task: "Créer mes-recherches.vue"         # T021
```

## Parallel Example: Phase Admin

```bash
# Lancer les 3 pages admin en parallèle (fichiers différents) :
Task: "Créer admin/retrouve-amis/index.vue"        # T037
Task: "Créer admin/retrouve-amis/[id].vue"          # T038
Task: "Créer admin/retrouve-amis/signalements.vue"  # T039
```

---

## Implementation Strategy

### MVP First (US1 + US2 : les deux P1)

1. Compléter Phase 1: Setup SQL
2. Compléter Phase 2: Fondations (CRITIQUE, bloque tout)
3. Compléter Phase 3: US1 : Créer des avis de recherche
4. Compléter Phase 4: US2 : Recevoir des notifications de correspondance
5. **STOP et VALIDER**: Tester le flux A crée un avis → B crée un avis complémentaire → notification de correspondance → résumé anonymisé visible
6. Déployer/démo si prêt

### Incremental Delivery

1. Setup SQL + Fondations → Base prête
2. US1 → Créer/modifier/clôturer des avis → Démo MVP minimal
3. US2 → Correspondances détectées et notifiées → Démo MVP complet
4. US3 → Consentement mutuel et partage coordonnées → Démo fonctionnalité complète
5. US4 → Profil trouvable → Démo enrichissement base
6. US5 → Tableau de bord → Démo expérience utilisateur améliorée
7. Admin → Modération → Démo back-office
8. Polish → Intégration NavBar/Sidebar → Release

### Parallel Strategy (2 développeurs)

1. Équipe complète Setup SQL + Fondations ensemble
2. Une fois les fondations prêtes :
   - Dev A : US1 backend → US2 backend → US3 backend
   - Dev B : US4 (indépendant) → US1 frontend → US2 frontend
3. Admin en fin de cycle (un seul dev)

---

## Notes

- [P] tasks = fichiers différents, pas de dépendances
- [Story] label associe chaque tâche à sa user story pour traçabilité
- Chaque user story est indépendamment complétable et testable
- Commiter après chaque tâche ou groupe logique
- Arrêter à chaque checkpoint pour valider la story indépendamment
- Frontend pages publiques : Tailwind CSS v4 pur (pas de daisyUI)
- Frontend pages admin : daisyUI v5 (conformément Constitution VI)
- Audit (`audit::log_action`) requis sur TOUTES les mutations backend
- Schema SQL est la source de vérité (Constitution III)
