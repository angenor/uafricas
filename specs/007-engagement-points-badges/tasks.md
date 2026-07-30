---

description: "Liste de tâches — récompenses par points, barème paramétrable, espace « Mon engagement »"
---

# Tasks: Récompenses par points — barème 100 % paramétrable & espace « Mon engagement »

**Input**: Documents de conception dans `/specs/007-engagement-points-badges/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: **aucune tâche de test automatisé** — le projet n'a ni harnais ni CI (contrainte constitutionnelle assumée) et la spec n'en demande pas. La validation est manuelle et scénarisée dans [quickstart.md](./quickstart.md) (campagnes S1 → S6), exécutée en Phase 8.

**Organization**: tâches groupées par user story, chaque story étant livrable et démontrable seule.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers différents, aucune dépendance sur une tâche incomplète)
- **[Story]** : US1 → US5, en correspondance avec [spec.md](./spec.md)

## Path Conventions

Monorepo web : `uafricas_backend/src/…` (Rust/Actix), `uafricas_backend/doc/bd/schemas/…` (SQL), `uafricas_frontend/app/…` (Nuxt 4). Chemins complets donnés dans chaque tâche.

## Rappels non négociables

- **Le SQL d'abord** (Principe III) : aucune struct Rust ni interface TS avant la migration correspondante.
- **Attribution hors transaction métier** : `services::engagement::attribuer` s'appelle **après le `COMMIT`**, jamais dedans.
- **`audit::log_action` sur chaque mutation admin** (Principe VII).
- **Tailwind v4 pur** côté membre, **daisyUI** autorisé uniquement sous `pages/admin/` (Principe VI).
- **`getDiagnostics`** après chaque fichier modifié (rust-analyzer / Volar).

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: vérifier le socle existant et déclarer les migrations à venir

- [X] T001 Vérifier l'état du socle en base avant toute écriture : les 6 tables de `engagement` existent, `regle_points` contient les 6 règles seedées, la permission `engagement.gerer` est attribuée à `super_admin` — sinon appliquer d'abord `uafricas_backend/doc/bd/schemas/35_engagement.sql` et `35b_engagement_mise_en_avant.sql`
- [X] T002 Déclarer les 3 nouvelles migrations dans `uafricas_backend/doc/bd/schema.sql`, à la suite de la ligne `\ir schemas/35b_engagement_mise_en_avant.sql` (ordre imposé : `35c`, puis `35d`, puis `35e`)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: catégories de points et catégorie portée par le mouvement — socle commun à US1 (ventilation) et US2 (paramétrage)

**⚠️ CRITICAL**: aucune user story ne peut démarrer avant la fin de cette phase

- [X] T003 Écrire `uafricas_backend/doc/bd/schemas/35c_engagement_categories_bareme.sql` : table `engagement.categorie_points`, colonnes `regle_points.categorie_id` + `regle_points.seuil_declencheur`, colonne `mouvement_points.categorie_id` + index `idx_mouvement_categorie`, colonne `palier_popularite.type_objet` avec remplacement de l'unicité par `idx_uq_palier_seuil_famille … NULLS NOT DISTINCT`, index `idx_uq_niveau_seuil` — migration **idempotente** (`IF NOT EXISTS`, `DO $$ … duplicate_object`), conforme à [data-model.md](./data-model.md#migration-35c_engagement_categories_baremesql)
- [X] T004 Compléter `35c…sql` par les seeds et le rattrapage : 6 catégories (`contributions`, `popularite`, `medias`, `factcheck`, `partages`, `ajustements`), rattachement des 6 règles existantes, seed des 4 règles neuves (`proposition_media_validee` +5, `media_a_la_une` +8, `animation_support_acceptee` +15, `partage_externe_5reseaux` +10 / plafond **30 points** / `seuil_declencheur` 5), puis `UPDATE mouvement_points … FROM regle_points` pour rattacher l'historique existant
- [X] T005 Appliquer `35c` en développement et exécuter la requête de contrôle de [quickstart.md](./quickstart.md#prérequis) — `regles_sans_categorie` doit valoir **0**
- [X] T006 Dans `uafricas_backend/src/services/engagement.rs`, faire porter la catégorie par le mouvement : `charger_regle` renvoie aussi `categorie_id` et `seuil_declencheur`, et l'`INSERT INTO engagement.mouvement_points` de `appliquer` renseigne `categorie_id` (recopie, jamais de jointure à la lecture — R1)
- [X] T007 [P] Dans `uafricas_backend/src/models/engagement.rs`, ajouter `categorie_code` / `categorie_libelle` à `MouvementResponse` et créer les DTO `CategorieVentilation` et `VentilationResponse` (`solde_points`, `total_gagne`, `categories`)
- [X] T008 [P] Dans `uafricas_backend/src/models/admin/engagement.rs`, ajouter `categorie_id` / `seuil_declencheur` / `instrumentee` / `nombre_mouvements` à `RegleAdmin`, les champs correspondants à `ModifierRegleRequest`, et créer `CreerRegleRequest`, `CategorieAdmin`, `CreerCategorieRequest`, `ModifierCategorieRequest`

**Checkpoint**: le journal porte désormais sa catégorie ; US1 et US2 peuvent démarrer en parallèle

---

## Phase 3: User Story 1 - Consulter mon engagement (Priority: P1) 🎯 MVP

**Goal**: le membre voit ses soldes, sa progression vers le niveau suivant, la ventilation de ses points par catégorie et son historique filtrable, depuis un espace dédié atteignable en 2 clics.

**Independent Test**: se connecter avec un membre ayant un historique, ouvrir `/mon-compte/engagement` et vérifier soldes, distance au niveau suivant, ventilation (somme réconciliable avec le total du journal) et filtrage de l'historique par catégorie et par période. Aucune dépendance sur US2 à US5 : la story se démontre sur les données déjà produites par le socle.

### Implementation for User Story 1

- [X] T009 [US1] Implémenter `GET /api/engagement/mes-categories` dans `uafricas_backend/src/handlers/engagement.rs` : une seule requête `SUM(points)` / `COUNT(*)` `GROUP BY categorie_id` jointe sur `categorie_points`, triée par `ordre`, avec le regroupement « Autres » pour `categorie_id IS NULL` ; exposer `solde_points` **et** `total_gagne` séparément (R2)
- [X] T010 [US1] Étendre `GET /api/engagement/mon-journal` dans `uafricas_backend/src/handlers/engagement.rs` : filtres `categorie`, `depuis`, `jusqu_a` par paramètres castés neutralisables (`$n::text IS NULL OR …`, jamais de concaténation), et renseigner `libelle` / `categorie_code` / `categorie_libelle` dans chaque ligne
- [X] T011 [US1] Implémenter `GET /api/engagement/actions-recompensees` (public) dans `uafricas_backend/src/handlers/engagement.rs` : règles `actif = TRUE` avec libellé, points, catégorie, plafond journalier et seuil déclencheur — c'est la source unique des libellés du barème côté front (FR-016)
- [X] T012 [US1] Déclarer les 3 routes dans le scope `/engagement` de `uafricas_backend/src/routes.rs`
- [X] T013 [P] [US1] Étendre `uafricas_frontend/app/composables/useEngagement.ts` : `obtenirMesCategories`, `obtenirActionsRecompensees`, et paramètres `categorie` / `depuis` / `jusqu_a` sur `listerMonJournal` ; interfaces TS alignées sur les DTO Rust de T007
- [X] T014 [P] [US1] Créer `uafricas_frontend/app/components/engagement/ResumeEngagement.vue` (Tailwind v4 pur) : solde total, solde du mois, réputation, badge de niveau via `EngagementBadgeStatut`, barre de progression + « N points avant *niveau* », et mention « niveau maximal atteint » quand `prochain_niveau` est nul
- [X] T015 [P] [US1] Créer `uafricas_frontend/app/components/engagement/VentilationCategories.vue` : une carte par catégorie (icône, libellé, total), distinction explicite entre « points gagnés » et « solde courant » pour rendre l'écart du plancher 0 compréhensible
- [X] T016 [P] [US1] Créer `uafricas_frontend/app/components/engagement/HistoriquePoints.vue` : liste paginée, filtres catégorie + période, points signés, impact réputation, et mention d'écrêtage distinguant « écrêté à N points » de « plafond atteint, aucun point crédité » (R14)
- [X] T017 [US1] Créer la page `uafricas_frontend/app/pages/mon-compte/engagement.vue` assemblant T014–T016, avec `definePageMeta` cohérent avec les pages sœurs `/mon-compte/*` et un **état vide pédagogique** alimenté par `obtenirActionsRecompensees` quand le membre n'a aucun point (FR-015) ; **aucune section badges à ce stade** — l'emplacement est prévu et renseigné par T046 (US3)
- [X] T018 [US1] Ramener `uafricas_frontend/app/components/engagement/MesPointsPanel.vue` à un résumé + lien proéminent « Voir tout mon engagement » vers `/mon-compte/engagement`, l'onglet « Mes points » de `uafricas_frontend/app/pages/mon-compte/profil.vue` restant la porte d'entrée (R13 — ne pas retirer cet onglet)

**Checkpoint**: US1 fonctionne seule — le membre voit et comprend ses points

---

## Phase 4: User Story 2 - Paramétrer intégralement le barème (Priority: P1)

**Goal**: l'administrateur crée, modifie, désactive règles, catégories, paliers et niveaux sans redéploiement, et voit immédiatement quelles règles sont réellement instrumentées.

**Independent Test**: créer une règle avec montant, plafond et catégorie, déclencher l'action, constater le crédit ; la désactiver et constater l'arrêt sans casser l'action métier ; insérer un niveau intermédiaire et vérifier la bascule de tous les comptes concernés ; vérifier le refus des doublons et le 403 pour un compte sans `engagement.gerer`.

### Implementation for User Story 2

- [X] T019 [US2] Déclarer le catalogue const des actions instrumentées dans `uafricas_backend/src/handlers/admin/engagement.rs` (`type_action`, libellé par défaut, `types_objet`, module d'origine) et exposer `GET /api/admin/engagement/actions-disponibles` ; enrichir `lister_regles` de `instrumentee` et `nombre_mouvements` afin qu'une règle orpheline soit visible sans enquête (R3)
- [X] T020 [US2] Implémenter `POST /api/admin/engagement/regles` et `DELETE /api/admin/engagement/regles/{id}` dans `uafricas_backend/src/handlers/admin/engagement.rs` : validation `^[a-z0-9_]{3,50}$`, **409** explicite sur `type_action` déjà pris, **409** sur suppression d'une règle référencée par un mouvement (« désactivez-la »), `type_action` immuable en modification
- [X] T021 [US2] Étendre `modifier_regle` à `categorie_id` et `seuil_declencheur` dans `uafricas_backend/src/handlers/admin/engagement.rs`
- [X] T022 [US2] Implémenter le CRUD des catégories (`GET`, `POST`, `PUT /{id}`, `DELETE /{id}`) dans `uafricas_backend/src/handlers/admin/engagement.rs` : `code` immuable, `nombre_regles` en lecture, **409** si une règle référence la catégorie visée par la suppression
- [X] T023 [US2] Implémenter `POST /api/admin/engagement/niveaux` et `DELETE /api/admin/engagement/niveaux/{id}` dans `uafricas_backend/src/handlers/admin/engagement.rs`, plus la fonction `recalculer_niveaux` (`UPDATE engagement.compte … SET niveau_code = (SELECT … ORDER BY seuil_min DESC LIMIT 1)`) appelée **dans la même transaction** par les trois mutations de niveau ; garde-fous : refus de retirer le niveau plancher (`seuil_min = 0`) ou le dernier niveau, recalcul de `ordre` d'après `seuil_min`, retour de `comptes_recalcules` (R5)
- [X] T024 [US2] Accepter `type_objet` sur `creer_palier` / `modifier_palier` (`uafricas_backend/src/handlers/admin/engagement.rs`) et implémenter la **règle de substitution** dans `evaluer_popularite` (`uafricas_backend/src/services/engagement.rs`) : les paliers d'une famille remplacent les paliers globaux pour cette famille, jamais d'union (R4)
- [X] T025 [US2] Ajouter le filtre `categorie` et les colonnes `categorie_code` / `categorie_libelle` à `lister_journal` dans `uafricas_backend/src/handlers/admin/engagement.rs`
- [X] T026 [US2] Instrumenter `audit::log_action` sur les 15 mutations nouvelles (règles, catégories, niveaux) dans `uafricas_backend/src/handlers/admin/engagement.rs`, avec état avant/après en JSONB
- [X] T027 [US2] Déclarer les nouvelles routes admin dans `uafricas_backend/src/routes.rs`, derrière `AdminUtilisateur` + `verifier_permission!(admin, "engagement", "gerer")` — **y compris les `GET`**
- [X] T028 [P] [US2] Étendre `uafricas_frontend/app/composables/useAdminEngagement.ts` : `listerActionsDisponibles`, `creerRegle`, `supprimerRegle`, CRUD catégories, `creerNiveau` / `supprimerNiveau`, `type_objet` sur les paliers, filtre `categorie` sur le journal
- [X] T029 [P] [US2] Refondre `uafricas_frontend/app/pages/admin/engagement/regles.vue` (daisyUI) : création depuis le catalogue d'actions, désactivation, colonne catégorie, colonne seuil déclencheur, mention **« non instrumentée — aucun point ne sera attribué »** visible sans survol, et **aide explicite « les plafonds sont exprimés en points, pas en nombre d'actions »**
- [X] T030 [P] [US2] Créer `uafricas_frontend/app/pages/admin/engagement/categories.vue` (daisyUI) : CRUD, ordre, couleur, icône, avertissement chiffré avant suppression
- [X] T031 [P] [US2] Créer `uafricas_frontend/app/pages/admin/engagement/niveaux.vue` (daisyUI) : CRUD des niveaux extrait de `regles.vue`, affichage du nombre de comptes recalculés après chaque mutation
- [X] T032 [P] [US2] Grouper les paliers **par famille** dans l'écran des paliers avec le rappel « les paliers d'une famille remplacent les paliers globaux » (`uafricas_frontend/app/pages/admin/engagement/regles.vue` ou l'écran dédié issu de T031)
- [X] T033 [US2] Ajouter les entrées « Catégories », « Niveaux » (et « Badges » préparée pour US3) sous la section Engagement de `uafricas_frontend/app/components/admin/AdminSidebar.vue`

**Checkpoint**: US1 **et** US2 fonctionnent — la promesse « points paramétrables + consultables » est tenue

---

## Phase 5: User Story 3 - Débloquer des badges et succès (Priority: P2)

**Goal**: des badges paramétrables s'attribuent automatiquement, se voient dans l'espace membre et sur le profil public, et déclenchent une notification.

**Independent Test**: définir un badge conditionné à 2 contributions validées, atteindre le seuil, vérifier l'attribution unique, la notification, l'absence de doublon après réévaluations répétées, puis désactiver le badge et vérifier qu'il reste chez son détenteur.

### Implementation for User Story 3

- [X] T034 [US3] Écrire `uafricas_backend/doc/bd/schemas/35d_engagement_badges.sql` : enums `type_condition_badge` et `origine_badge`, tables `engagement.badge` (avec le `CHECK ck_badge_condition` complet) et `engagement.badge_obtenu` (`UNIQUE (utilisateur_id, badge_id)`, `ON DELETE CASCADE`), index, seed des 10 badges, puis la **rétro-évaluation** unique `INSERT … origine = 'retroactif' … ON CONFLICT DO NOTHING` **sans notification** (R9)
- [X] T035 [US3] Appliquer `35d` en développement et vérifier qu'aucun `badge_obtenu` n'est en doublon
- [X] T036 [US3] Implémenter `evaluer_badges(pool, utilisateur_id)` dans `uafricas_backend/src/services/engagement.rs` : une requête par badge actif non encore obtenu, pour les 5 types de condition (`actions_comptees`, `points_categorie`, `solde_total`, `niveau_atteint` comparé sur `ordre`, `palier_popularite`), badges `manuel` exclus, insertion `ON CONFLICT DO NOTHING`, erreurs loguées et jamais propagées
- [X] T037 [US3] Appeler `evaluer_badges` **après le commit** dans `appliquer` (`uafricas_backend/src/services/engagement.rs`) et y détecter le changement de `niveau_code` (comparaison ancien/nouveau) pour émettre la notification de niveau
- [X] T038 [US3] Ajouter `pub mod engagement { NIVEAU_ATTEINT, BADGE_DEBLOQUE }` à `uafricas_backend/src/models/notification.rs` et émettre dans `arbre_genealogique.notifications` avec `lien_action = "/mon-compte/engagement"` — **uniquement si l'insertion a créé une ligne** (`rows_affected() == 1`), ce qui interdit la notification répétée (R7, R8)
- [X] T039 [US3] Implémenter `GET /api/engagement/mes-badges` (obtenus + à débloquer + progression chiffrée bornée, appelle `evaluer_badges` avant de répondre) et `GET /api/engagement/badges/{utilisateur_id}` (public, badges obtenus **seulement**) dans `uafricas_backend/src/handlers/engagement.rs`, DTO dans `uafricas_backend/src/models/engagement.rs`
- [X] T040 [US3] Implémenter le CRUD des badges + `POST /badges/{id}/attribuer` + `DELETE /badges/{id}/attribuer/{utilisateur_id}` dans `uafricas_backend/src/handlers/admin/engagement.rs` : validation applicative **miroir du CHECK SQL** (message français plutôt que violation de contrainte brute), `code` immuable, **409** sur suppression d'un badge détenu, audit avec motif, notification à l'attribution manuelle et **aucune** au retrait
- [X] T041 [US3] Déclarer les routes badges (membre + admin) dans `uafricas_backend/src/routes.rs`
- [X] T042 [P] [US3] Créer `uafricas_frontend/app/components/engagement/BadgeSucces.vue` (vignette unitaire : icône, libellé, couleur, état obtenu/verrouillé, date) — Tailwind v4 pur
- [X] T043 [P] [US3] Créer `uafricas_frontend/app/components/engagement/MesBadges.vue` : section « obtenus » et section « à débloquer » avec condition en clair et progression (aucune barre quand la condition n'est pas chiffrable)
- [X] T044 [P] [US3] Étendre `uafricas_frontend/app/composables/useEngagement.ts` (`obtenirMesBadges`, `obtenirBadgesPublics`) et `uafricas_frontend/app/composables/useAdminEngagement.ts` (CRUD badges, attribution, retrait)
- [X] T045 [P] [US3] Créer `uafricas_frontend/app/pages/admin/engagement/badges.vue` (daisyUI) : CRUD, formulaire de condition dont les champs de paramètres s'adaptent au `type_condition` choisi, `nombre_detenteurs`, attribution/retrait manuels avec motif
- [X] T046 [US3] Monter `MesBadges` dans `uafricas_frontend/app/pages/mon-compte/engagement.vue` (dépend de T043) — **repli si US1 n'est pas encore livrée** : monter le composant dans l'onglet « Mes points » de `uafricas_frontend/app/pages/mon-compte/profil.vue`, de sorte qu'US3 reste démontrable seule
- [X] T047 [P] [US3] Afficher les badges obtenus sur `uafricas_frontend/app/pages/profil/[id].vue`, à côté de `EngagementBadgeStatut`, **sans** exposer soldes ni journal (FR-014)
- [X] T048 [P] [US3] Donner une icône et une couleur aux 2 nouveaux types de notification dans `uafricas_frontend/app/mocks/notifications.ts` — c'est là que résident `iconeNotification` / `couleurNotification`, consommées par `ClocheNotifications.vue` ; leur union `TypeNotification` ne couvre **aucun** type pointé (`afrolang.*`, `media.*` retombent sur `'bell'`) : ajouter `engagement.niveau_atteint` (`medal`) et `engagement.badge_debloque` (`award`) plutôt que d'hériter du défaut générique

**Checkpoint**: US1 + US2 + US3 — les badges vivent, se paramètrent et se notifient

---

## Phase 6: User Story 4 - Récompenser les actions non encore couvertes (Priority: P2)

**Goal**: proposition média validée, mise à la une, animation acceptée et popularité télé/radio créditent leur bénéficiaire.

**Independent Test**: dérouler chacun des 4 parcours réels et vérifier le crédit unique au bon bénéficiaire, la catégorie « Médias », l'absence de crédit au rejeu et l'absence de crédit en auto-attribution. Les 4 branchements sont indépendants entre eux.

### Implementation for User Story 4

- [X] T049 [US4] Créditer `proposition_media_validee` dans `uafricas_backend/src/handlers/admin/media_proposition.rs::valider_proposition`, **après le `COMMIT`**, clé `prop_media:{proposition_id}`, bénéficiaire `auteur_id`, ignoré si `auteur_id == admin.id` (R11)
- [X] T050 [US4] Créditer `animation_support_acceptee` sur les **deux** chemins de décision — `uafricas_backend/src/handlers/admin/media_proposition.rs` (file admin) et `uafricas_backend/src/handlers/media_proposition.rs::accepter_engagement` (co-détenteurs) — après commit, clé identique `animation:{proposition_id}` de sorte qu'un seul crédit soit possible quel que soit le chemin, ignoré si `auteur_id == decideur` ; ne **pas** insérer l'appel dans `appliquer_acceptation_engagement`, qui travaille dans une transaction (R11)
- [X] T051 [P] [US4] Créditer `media_a_la_une` dans `uafricas_backend/src/handlers/admin/radio_tele.rs`, à la création **et** à la modification, pour les 4 tables portant `a_la_une` (`chaine_tv`, `station_radio`, `programme_tele`, `programme_radio`) : bénéficiaire `cree_par`, clé `alaune:{type_objet}:{objet_id}` (retirer puis reposer la mise à la une ne recrédite pas), ignoré si `cree_par == admin.id`
- [X] T052 [P] [US4] Brancher la popularité des médias dans `uafricas_backend/src/handlers/media_social.rs::reagir_media` : résoudre l'auteur par `match` sur littéraux fixes (`chaine_tv|station_radio|programme_tele|programme_radio → cree_par`), compter les likes **en excluant celui de l'auteur** (décompte distinct de celui affiché dans la réponse), puis appeler `evaluer_popularite` uniquement lors de la pose d'un `like` (R12)

**Checkpoint**: les domaines télé/radio récompensent enfin leurs contributeurs

---

## Phase 7: User Story 5 - Récompenser le partage vers les réseaux externes (Priority: P3)

**Goal**: partager un contenu vers 5 réseaux distincts crédite un bonus plafonné, sans jamais gêner le partage lui-même.

**Independent Test**: partager un contenu vers 5 réseaux différents (bonus unique), répéter un réseau (aucun bonus), dépasser le plafond journalier (écrêtage visible), et couper le backend pour vérifier que le partage fonctionne quand même.

### Implementation for User Story 5

- [X] T053 [US5] Écrire `uafricas_backend/doc/bd/schemas/35e_engagement_partage_externe.sql` : enum `engagement.reseau_social` (`whatsapp`, `facebook`, `x`, `linkedin`, `telegram`, `email`), table `engagement.partage_externe` avec `UNIQUE (utilisateur_id, type_objet, objet_id, reseau)` et index `idx_partage_externe_contenu`, puis l'appliquer en développement
- [X] T054 [US5] Implémenter `enregistrer_partage_externe` dans `uafricas_backend/src/services/engagement.rs` : `INSERT … ON CONFLICT DO NOTHING`, `COUNT(DISTINCT reseau)` pour ce couple (membre, contenu), crédit de `partage_externe_5reseaux` quand le compte atteint le `seuil_declencheur` **lu dans la règle** (jamais 5 en dur), clé `partage5:{type_objet}:{objet_id}:{utilisateur_id}`. **Ne pas filtrer les contenus dont le membre est l'auteur** : le bénéficiaire est le partageur, et promouvoir sa propre contribution à l'extérieur est le comportement recherché (FR-030 amendé)
- [X] T055 [US5] Implémenter `POST /api/engagement/partages-externes` dans `uafricas_backend/src/handlers/engagement.rs` (identité prise du JWT, `reseau` validé contre l'enum, `type_objet` validé contre la liste des familles partageables, réponse `{ reseaux_distincts, seuil, bonus_attribue }`) et déclarer la route dans `uafricas_backend/src/routes.rs`
- [X] T056 [P] [US5] Créer `uafricas_frontend/app/composables/usePartageExterne.ts` : traçage **best-effort** appelé **après** l'ouverture de la fenêtre du réseau, échec silencieux (`.catch(() => {})`) — un traçage raté ne doit jamais empêcher un partage
- [X] T057 [P] [US5] Ajouter **Telegram** et **E-mail** puis brancher le traçage dans `uafricas_frontend/app/components/media/MediaPartagerModal.vue`, `uafricas_frontend/app/components/opportunite-afrique/PartagerElementModal.vue` et `uafricas_frontend/app/components/opportunite-afrique/PartagerFicheModal.vue` — sans ces 2 réseaux, le seuil de 5 est inatteignable (R10) ; « copier le lien » n'est pas un réseau et ne se trace pas
- [X] T058 [P] [US5] Même traitement pour `uafricas_frontend/app/components/evenements/EvenementPartage.vue`, `uafricas_frontend/app/components/universite/gouvernance/PartagePublication.vue` et `uafricas_frontend/app/components/retrouve-amis/BoutonsPartage.vue`

**Checkpoint**: les 5 user stories sont livrées et démontrables indépendamment

---

## Phase 8: Polish & Cross-Cutting Concerns

- [X] T059 Exécuter les campagnes S1 → S6 de [quickstart.md](./quickstart.md), y compris les 2 requêtes de contrôle de doublons (`cle_idempotence`, `badge_obtenu`) qui doivent renvoyer 0 ligne
- [X] T060 Vérifier la **non-régression** du barème existant (SC-011) : contribution Codimoi validée, factcheck correct/faux, mise en avant et paliers de popularité créditent exactement les mêmes montants qu'avant la feature
- [X] T061 Vérifier le caractère non-bloquant (SC-007) en désactivant toutes les règles puis en déroulant les 4 parcours médias : chaque action métier doit réussir
- [X] T062 [P] Vérifier l'absence de toute classe daisyUI dans `uafricas_frontend/app/pages/mon-compte/engagement.vue` et `uafricas_frontend/app/components/engagement/` (Principe VI), et migrer les résidus Tailwind v3 rencontrés dans les fichiers touchés
- [X] T063 [P] Vérifier qu'aucun libellé, montant ou seuil du barème n'est écrit en dur dans le frontend : tout provient de `actions-recompensees`, `mes-categories`, `mes-badges` ou `niveau`
- [X] T064 [P] Passer `getDiagnostics` sur l'ensemble des fichiers touchés (rust-analyzer + Volar) et corriger avant commit
- [X] T065 Contrôler la piste d'audit (SC-009) : une entrée par mutation de barème, de catégorie, de niveau et de badge, avec son auteur. Vérifier séparément qu'un compte dépourvu de `engagement.gerer` reçoit un **403 nommant la permission requise** — ce refus n'apparaît **pas** dans `/admin/audit` (`verifier_permission!` n'y écrit rien) et **ne doit pas** y être ajouté : la macro est partagée par toutes les routes d'administration de la plateforme
- [X] T066 Mettre à jour `CLAUDE.md` : une ligne dans « Recent Changes (index) » citant les migrations `35c`/`35d`/`35e` et les modules clés, plus les entrées « Active Technologies » de la feature

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance
- **Foundational (Phase 2)** : dépend de Setup — **bloque US1 et US2** (la catégorie du mouvement est le socle commun)
- **US1 (Phase 3)** et **US2 (Phase 4)** : parallélisables dès la fin de Phase 2
- **US3 (Phase 5)** : dépend de Phase 2 ; T046 dépend de T017 (la page d'accueil de l'espace membre doit exister pour y monter la section badges) ; le reste est indépendant d'US1/US2
- **US4 (Phase 6)** : dépend de Phase 2 uniquement (les 4 règles sont seedées par T004). Livrable **avant** US1 si besoin — les points s'accumulent, seule leur consultation attend US1
- **US5 (Phase 7)** : dépend de Phase 2 ; indépendante d'US1 à US4
- **Polish (Phase 8)** : dépend des stories livrées

### Chaînes internes (à ne pas paralléliser)

- **T003 → T004 → T005 → T006** : la migration précède toujours le code qui la consomme (Principe III)
- **T009 → T010 → T011** : même fichier (`handlers/engagement.rs`)
- **T019 → T020 → T021 → T022 → T023 → T024 → T025 → T026** : même fichier (`handlers/admin/engagement.rs`)
- **T034 → T035 → T036 → T037** : migration, puis service, puis branchement post-commit
- **T036 → T039** et **T038 → T040** : le service et les constantes précèdent leurs consommateurs
- **T049 → T050** : même fichier (`handlers/admin/media_proposition.rs`)
- **T053 → T054 → T055** : migration → service → endpoint
- **T014, T015, T016 → T017** et **T043 → T046** : les composants précèdent la page qui les assemble
- **routes.rs (T012, T027, T041, T055)** : un seul fichier, à sérialiser entre stories

### Parallel Opportunities

- Phase 2 : **T007 ∥ T008** (deux fichiers de modèles distincts)
- Phase 3 : **T013 ∥ T014 ∥ T015 ∥ T016** (composable + 3 composants)
- Phase 4 : **T028 ∥ T029 ∥ T030 ∥ T031 ∥ T032 ∥ T033** (composable, 3 pages, sidebar) une fois le backend de la story terminé
- Phase 5 : **T042 ∥ T043 ∥ T044 ∥ T045 ∥ T047 ∥ T048**
- Phase 6 : **T051 ∥ T052** (et T049 en parallèle des deux)
- Phase 7 : **T056 ∥ T057 ∥ T058**
- Phase 8 : **T062 ∥ T063 ∥ T064**
- Entre équipes : US1, US2, US4 et US5 peuvent être menées par 4 développeurs distincts dès la fin de Phase 2, la seule contention étant `routes.rs`

---

## Parallel Example: User Story 1

```bash
# Frontend d'US1, une fois T009–T012 livrés :
Tâche T013 : "Étendre useEngagement.ts (catégories, actions récompensées, filtres de journal)"
Tâche T014 : "Créer ResumeEngagement.vue"
Tâche T015 : "Créer VentilationCategories.vue"
Tâche T016 : "Créer HistoriquePoints.vue"
# puis T017 assemble la page, T018 rebranche l'onglet du profil
```

## Parallel Example: User Story 4

```bash
# Les 3 branchements indépendants (fichiers différents) :
Tâche T049 : "Crédit sur validation d'une proposition média (admin/media_proposition.rs)"
Tâche T051 : "Crédit sur mise à la une (admin/radio_tele.rs, 4 tables)"
Tâche T052 : "Popularité des médias (media_social.rs, auto-like exclu)"
# T050 attend T049 : même fichier
```

---

## Implementation Strategy

### MVP (US1 seule)

1. Phase 1 Setup → Phase 2 Foundational.
2. Phase 3 US1 → **valider S1** de quickstart.
3. Démo : un membre ouvre « Mon engagement », comprend d'où viennent ses points et retrouve son historique filtré. **STOP & VALIDATE.**

À ce stade, le barème reste celui de la phase 1 (6 règles) : la valeur livrée est la **lisibilité**, qui est la moitié la plus visible de la demande.

### Incrément recommandé (la promesse complète)

4. Phase 4 US2 → valider S2. Les deux exigences explicites de la demande (« paramétrable » + « consultable ») sont alors tenues.
5. Phase 5 US3 → valider S3. Les badges, cités par la demande, deviennent réels.
6. Phase 6 US4 → valider S4. La couverture s'étend aux domaines médias.
7. Phase 7 US5 → valider S5. La règle la moins vérifiable arrive en dernier, avec ses garde-fous.
8. Phase 8 → non-régression, audit, documentation.

### Ordre alternatif si l'objectif est l'engagement immédiat

US4 avant US1 : les points s'accumulent dès le branchement des actions médias, et l'espace membre les révèle ensuite d'un coup. À réserver au cas où la mise en service de l'espace membre serait retardée — la spec interdit toute rétroactivité, donc **tout jour sans branchement est un jour de points définitivement perdus** pour les membres.

---

## Notes

- `[P]` = fichiers différents, aucune dépendance sur une tâche incomplète.
- Aucune tâche ne modifie `mouvement_points` autrement qu'en insertion : le journal reste immuable, ventilation et badges se recalculent toujours à partir de lui.
- Toute nouvelle règle doit définir sa **clé d'idempotence** avant d'être branchée : c'est elle, et non un contrôle applicatif, qui interdit le double crédit.
- Les plafonds sont en **points**, pas en occurrences — à rappeler dans l'UI (T029) et à vérifier au moindre ajustement.
- Commit par tâche ou par groupe logique, message en français (Principe I).
