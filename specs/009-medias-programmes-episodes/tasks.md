---

description: "Plan de tâches — recadrage télé/radio en programmes conteneurs et épisodes"
---

# Tasks: Médias — programmes conteneurs, épisodes, thématiques multiples et couverture panafricaine

**Input**: Documents de conception dans `/specs/009-medias-programmes-episodes/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md),
[data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: Le projet n'a **aucun harnais de test configuré** (constitution, Contraintes Techniques) et la
spécification n'en demande pas. Aucune tâche de test automatisé n'est générée. La validation passe par
`quickstart.md`, `cargo check` et les diagnostics Volar.

**Organization**: Tâches groupées par histoire utilisateur. ⚠️ **Lire d'abord la note ci-dessous** : la
phase Fondations est inhabituellement large, et c'est délibéré.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers distincts, aucune dépendance sur une tâche incomplète)
- **[Story]** : US1 à US5, selon `spec.md`
- Chemins de fichiers exacts dans chaque description

## Path Conventions

Monorepo web : `uafricas_backend/src/`, `uafricas_backend/doc/bd/schemas/`, `uafricas_frontend/app/`.

---

## ⚠️ Note de séquencement — pourquoi les Fondations sont si larges

La spécification pose une **reprise de données en une seule fenêtre, sans cohabitation des deux
modèles**. Concrètement, la migration supprime `media_content.programme_tele` et `programme_radio` : à
cet instant, les 18 fichiers Rust et 24 fichiers frontend qui les référencent cessent de fonctionner.
Il n'existe aucun découpage qui permette de livrer US1 « seule » sans que le reste de la plateforme
soit déjà porté.

La phase 2 contient donc **la migration plus le portage de tout ce qui existe déjà**. Son point de
contrôle est précis et vérifiable : *l'application redémarre, les contenus migrés sont visibles, rien
n'est cassé — et aucune capacité nouvelle n'est encore offerte*. Les histoires ajoutent ensuite les
capacités par-dessus une base saine.

**Exception utile** : US3 (thématiques) et US4 (couverture) reposent sur une **seconde migration
purement additive** (`09r`), indépendante de `09q`. Elles peuvent être menées en parallèle des
Fondations, par une autre personne, sans attendre.

---

## Phase 1: Setup (préparation de la reprise)

**Purpose**: Sécuriser la fenêtre de migration avant d'écrire la moindre ligne de schéma.

- [X] T001 Relever les compteurs de référence avant migration en exécutant les requêtes de contrôle de [quickstart.md](./quickstart.md) §Étape 0 sur la base cible, et consigner le résultat dans la description de la PR (base de comparaison de SC-001)
- [X] T002 Sauvegarder la base de production via `./deploy.sh backup` et vérifier que l'archive est restaurable avant toute application de `09q`
- [X] T003 Inventorier les contenus sans support de rattachement (`SELECT count(*) FROM media_content.programme_tele WHERE chaine_id IS NULL` et l'équivalent radio), trancher leur traitement (chaîne « Sans chaîne » ou mise à l'écart) et consigner la décision en tête de `uafricas_backend/doc/bd/schemas/09q_media_content_emissions_episodes.sql` — inconnue laissée ouverte par [plan.md](./plan.md)

---

## Phase 2: Foundational (prérequis bloquants)

**Purpose**: Basculer le schéma et remettre la plateforme en marche sur le nouveau modèle.

**⚠️ CRITICAL**: Aucune histoire (hors US3/US4) ne peut démarrer avant la fin de cette phase.

### Migration 09q — schéma et reprise

- [X] T004 Créer `uafricas_backend/doc/bd/schemas/09q_media_content_emissions_episodes.sql` §1 : tables `emission_tele`, `emission_radio`, `episode_tele`, `episode_radio` avec leurs CHECK (`ck_episode_*_media_publie`, `ck_episode_*_rejet_motive`, `ck_episode_*_decision_coherente`, `ck_emission_*_cadence`) et leurs index, dont les deux index uniques partiels de mise en avant — voir [data-model.md](./data-model.md) §2.1 et §2.2
- [X] T005 Ajouter à `09q` §2 : `ALTER TABLE creneau_programmation ADD COLUMN emission_id, date_effet`, élargissement des quatre CHECK `ck_*_type_media` à six valeurs, et les quatre `ALTER TYPE type_objet_propose ADD VALUE` — ces derniers **en tête de fichier**, hors de tout bloc les utilisant ([data-model.md](./data-model.md) §3.2 à §3.4)
- [X] T006 Ajouter à `09q` §3 la reprise de données dans l'ordre imposé : une émission par contenu (slug suffixé `-programme`), un épisode par contenu **id et slug conservés**, `UPDATE` du discriminant sur les quatre tables d'interactions, rattachement des créneaux à l'émission avec `date_effet` = date de reprise ([data-model.md](./data-model.md) §4, étapes 2 à 5)
- [X] T007 Ajouter à `09q` §4 : `DROP TABLE media_content.programme_tele, programme_radio CASCADE` puis `ALTER TABLE creneau_programmation DROP COLUMN contenu_id`, et l'index `idx_creneau_emission` en remplacement de `idx_creneau_contenu`
- [X] T008 Appliquer `09q` sur la base locale, la rejouer une seconde fois pour confirmer son idempotence, et vérifier par `SELECT to_regclass('media_content.programme_tele')` que l'ancienne table a bien disparu
- [X] T009 Vérifier la reprise par les requêtes de [quickstart.md](./quickstart.md) §Étape 0 : `episodes_sans_emission`, `creneaux_orphelins` et `slugs_en_collision` doivent valoir **0**, et les décomptes d'interactions correspondre à ceux relevés en T001

### Modèles Rust

- [X] T010 [P] Créer `uafricas_backend/src/models/media_emission.rs` : structs `FromRow` `EmissionTeleRow` / `EmissionRadioRow`, constantes `EMISSION_TELE_COLONNES` / `EMISSION_RADIO_COLONNES`, DTO `EmissionResponse` et `EmissionRequest`, énumération de cadence
- [X] T011 [P] Créer `uafricas_backend/src/models/media_episode.rs` : structs `FromRow` `EpisodeTeleRow` / `EpisodeRadioRow`, constantes `COLONNES`, DTO `EpisodeResponse`, `EpisodeRequest`, `ReordonnancementRequest`, et l'énumération d'état incluant `rejete`
- [X] T012 Modifier `uafricas_backend/src/models/media_detention.rs` : `table_contenu_pour_support` renvoie désormais `media_content.emission_tele` / `emission_radio` (lignes 35-41), et ajouter `table_episode_pour_support`
- [X] T013 [P] Modifier `uafricas_backend/src/models/media_social.rs` : le discriminant `type_media` passe de 4 à 6 valeurs et rejette explicitement `programme_tele` / `programme_radio` ([contracts/api-public.md](./contracts/api-public.md) §4)
- [X] T014 [P] Modifier `uafricas_backend/src/models/media_proposition.rs` : `type_objet` accepte `emission_tele`, `emission_radio`, `episode_tele`, `episode_radio`
- [X] T015 Modifier `uafricas_backend/src/models/television.rs` : `PROGRAMME_TELE_COLONNES` disparaît au profit des colonnes d'émission et d'épisode ; les DTO de section portent `emissions[]` avec `nombre_episodes`, `dernier_episode_at` et `episodes_apercu`
- [X] T016 Modifier `uafricas_backend/src/models/station_radio.rs` et `uafricas_backend/src/models/programme_radio.rs` : mêmes ajustements côté radio ; supprimer les structs devenues sans table
- [X] T017 Déclarer les deux nouveaux modules dans `uafricas_backend/src/models/mod.rs` et retirer les réexports obsolètes

### Portage des handlers existants

- [X] T018 Porter `uafricas_backend/src/handlers/television.rs` : les 13 handlers passent aux émissions et aux épisodes ; `obtenir_programme_par_slug` devient `obtenir_episode_par_slug` (même slug, cf. [research.md](./research.md) R2) ; `obtenir_vedette` lit `episode_tele.a_la_une_globale`
- [X] T019 Porter `uafricas_backend/src/handlers/stations_radio.rs` : mêmes changements côté radio, `audio_url` au lieu de `video_url`
- [X] T020 Porter `uafricas_backend/src/handlers/media_social.rs` : les six cibles sont acceptées, `compteurs_pour` conserve sa forme en deux requêtes, la vérification d'existence de cible interroge la bonne table selon le discriminant
- [X] T021 Porter `uafricas_backend/src/handlers/media_proposition.rs` : proposer une émission ou un épisode ; la soumission d'épisode exige une `target_id` d'émission existante
- [X] T022 Porter `uafricas_backend/src/handlers/admin/media_proposition.rs` : la validation d'une proposition d'émission crée l'émission **et** la ligne de propriété dans la même transaction ; celle d'un épisode crée l'épisode directement en `publie` ([contracts/api-admin.md](./contracts/api-admin.md) §4)
- [X] T023 Porter `uafricas_backend/src/handlers/admin/radio_tele.rs` en **lecture, modification et suppression d'épisodes** ; retirer les routes `programmes-tele` / `programmes-radio` de création — la création exige une émission et relève d'US1
- [X] T024 Porter `uafricas_backend/src/handlers/media_detention.rs` : les listes de contenus d'un support remontent les émissions, et `contacter` reste inchangé
- [X] T025 Modifier `uafricas_backend/src/services/engagement.rs` : `resoudre_beneficiaire` (lignes 638-692) gagne `emission_tele`, `emission_radio`, `episode_tele`, `episode_radio` — un épisode remonte à son émission puis au propriétaire du support, avec repli sur `cree_par` ([research.md](./research.md) R11)
- [X] T026 [P] Modifier `uafricas_backend/src/handlers/engagement_cadeau.rs` et `uafricas_backend/src/models/engagement_cadeau.rs` : le `type_objet` cible d'un cadeau accepte les quatre nouvelles valeurs
- [X] T027 Mettre à jour `uafricas_backend/src/routes.rs` : retirer les routes `programmes-tele` / `programmes-radio` supprimées, déclarer les routes d'épisode par slug, en respectant la règle « segments fixes avant segments dynamiques » déjà commentée dans le fichier
- [X] T028 Exécuter `cargo check` jusqu'au vert, puis redémarrer le backend (`kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run`) et vérifier qu'aucune requête ne remonte `relation … does not exist` dans les logs

### Portage frontend minimal

- [X] T029 [P] Modifier `uafricas_frontend/app/types/admin.ts` : types `Emission` et `Episode`, retrait des types `ProgrammeTele` / `ProgrammeRadio`
- [X] T030 [P] Modifier `uafricas_frontend/app/composables/useMediaSocial.ts` et `useMediaProposition.ts` : six cibles, nouveaux types d'objet proposé
- [X] T031 [P] Modifier les libellés de cible média dans `uafricas_frontend/app/composables/useLecteurMedia.ts`, `uafricas_frontend/app/components/publications/MediaPartageCard.vue`, `uafricas_frontend/app/components/engagement/HistoriquePoints.vue`, `uafricas_frontend/app/components/engagement/MesCadeaux.vue`, `uafricas_frontend/app/composables/useCadeaux.ts` et `uafricas_frontend/app/pages/admin/engagement/regles.vue`
- [X] T032 Porter `uafricas_frontend/app/composables/useTelevision.ts` et `useStationsRadio.ts` sur les nouvelles formes de `sections` et de détail ([contracts/api-public.md](./contracts/api-public.md) §1 et §2)
- [X] T033 Exécuter `pnpm build` depuis `uafricas_frontend/` jusqu'au vert, puis parcourir `/medias/tele`, `/medias/radio/africans`, `/medias/radio/nationales` pour vérifier qu'aucune page n'est vide

**Checkpoint**: L'application redémarre sur le nouveau modèle. Chaque contenu d'avant la migration est
devenu un programme d'un seul épisode, accessible par son ancienne adresse. Aucune capacité nouvelle
n'est encore offerte.

---

## Phase 3: User Story 1 — Regrouper les vidéos et audios sous un programme (Priority: P1) 🎯 MVP

**Goal**: Un co-détenteur crée une émission puis y verse des épisodes successifs, soumis à validation ;
le public découvre une chaîne par ses programmes, et chaque programme par ses épisodes.

**Independent Test**: [quickstart.md](./quickstart.md) §Scénario 1 — créer un programme sans fichier,
y ajouter trois épisodes, les faire valider, vérifier qu'un **seul bloc** apparaît sur la chaîne et
qu'il donne accès aux trois épisodes.

### Backend — gestion par les détenteurs

- [X] T034 [US1] Créer `uafricas_backend/src/handlers/media_emission.rs` : `POST /api/medias/{type_support}/{support_id}/emissions`, `PUT` et `DELETE /api/medias/emissions/{id}`, `GET …/emissions` (vue détenteur avec décompte par état), toutes gardées par `garde_detenteur` au rôle `co_detenteur` — jamais `AdminUtilisateur` ([contracts/api-membre.md](./contracts/api-membre.md) §1)
- [X] T035 [US1] Implémenter dans `uafricas_backend/src/handlers/media_emission.rs` le refus `409` de suppression d'une émission comptant des épisodes publiés, avec le décompte dans le message (FR-010)
- [X] T036 [US1] Créer `uafricas_backend/src/handlers/media_episode.rs` : `POST /api/medias/emissions/{id}/episodes` forçant `etat = 'en_attente'` côté serveur et calculant `ordre = COALESCE(MAX(ordre), -1) + 1` (FR-007, FR-040)
- [X] T037 [US1] Ajouter dans `uafricas_backend/src/handlers/media_episode.rs` : `PUT /api/medias/episodes/{id}` remettant l'état à `en_attente` **si le média change**, effaçant `motif_rejet` sur un épisode rejeté, et `DELETE` en suppression douce ([contracts/api-membre.md](./contracts/api-membre.md) §2)
- [X] T038 [US1] Ajouter dans `uafricas_backend/src/handlers/media_episode.rs` : `PUT /api/medias/emissions/{id}/episodes/reordonner`, réécriture **atomique** sur le modèle de `uafricas_backend/src/handlers/admin/formation_contenu.rs:350`, avec refus `400` si la liste ne couvre pas exactement les épisodes de l'émission
- [X] T039 [US1] Ajouter dans `uafricas_backend/src/handlers/media_episode.rs` : `PATCH /api/medias/episodes/{id}/emission` (déplacement, refus `400` hors du même support) et `PATCH …/a-la-une` dont la bascule et la désignation tiennent dans **une seule transaction** ([research.md](./research.md) R9)

### Backend — modération

- [X] T040 [US1] Créer `uafricas_backend/src/handlers/admin/media_moderation_episode.rs` : `GET /api/admin/medias/episodes` avec filtres `etat`, `type`, `support_id` et tri `echeance | anciennete`, la `prochaine_echeance` étant calculée à la lecture depuis les créneaux de l'émission ([contracts/api-admin.md](./contracts/api-admin.md) §1)
- [X] T041 [US1] Implémenter dans `uafricas_backend/src/handlers/admin/media_moderation_episode.rs` la route `PATCH /api/admin/medias/episodes/{id}/valider` : passage à `publie`, renseignement de `valide_par` et `valide_at`, refus `409` si l'épisode n'est pas `en_attente`
- [X] T042 [US1] Implémenter dans `uafricas_backend/src/handlers/admin/media_moderation_episode.rs` la route `PATCH /api/admin/medias/episodes/{id}/rejeter` : motif d'au moins 10 caractères (même garde que le rejet de proposition), passage à `rejete`
- [X] T043 [US1] Émettre depuis `uafricas_backend/src/handlers/admin/media_moderation_episode.rs` les notifications de décision dans `arbre_genealogique.notifications`, motif inclus en cas de rejet (FR-041, SC-008)
- [X] T044 [US1] Ajouter le CRUD administratif complet des émissions et des épisodes dans `uafricas_backend/src/handlers/admin/radio_tele.rs`, un épisode créé par un administrateur naissant `publie` ([contracts/api-admin.md](./contracts/api-admin.md) §2)
- [X] T045 [US1] Déplacer dans `uafricas_backend/src/handlers/admin/radio_tele.rs` la route `PATCH /api/admin/medias/episodes/{id}/vedette-globale` depuis l'ancienne route `programmes-tele/{id}/vedette-globale` dans `admin/radio_tele.rs`

### Backend — lecture publique

- [X] T046 [US1] Ajouter dans `uafricas_backend/src/handlers/television.rs` : `GET /api/television/emissions/slug/{slug}`, `GET /api/television/emissions/{id}/episodes` (paginé, 24 par défaut, tri `(ordre, created_at, id)`) et `GET /api/television/episodes/slug/{slug}` avec ses `episodes_voisins`
- [X] T047 [US1] Ajouter les routes symétriques dans `uafricas_backend/src/handlers/stations_radio.rs`
- [X] T048 [US1] Enrichir `lister_sections` dans `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs` : chaque chaîne porte ses émissions publiées, avec `nombre_episodes`, `dernier_episode_at` et `episodes_apercu` borné à 12 — **sans requête N+1**, par agrégation en une passe
- [X] T049 [US1] Filtrer dans `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs` toute émission sans épisode publié, tout en la conservant visible du détenteur et de l'administration (FR-011, US1 §6)
- [X] T050 [US1] Déclarer l'ensemble des routes d'US1 dans `uafricas_backend/src/routes.rs`, `audit::log_action` étant appelé sur chaque mutation (FR-045)

### Frontend — public

- [X] T051 [P] [US1] Créer `uafricas_frontend/app/composables/useMediaEmissions.ts` : émissions d'un support, épisodes paginés d'une émission, détail d'épisode
- [X] T052 [P] [US1] Créer `uafricas_frontend/app/components/media/CarteEmission.vue` (Tailwind v4 pur) : vignette de programme annonçant son nombre d'épisodes et sa cadence
- [X] T053 [P] [US1] Créer `uafricas_frontend/app/components/media/ListeEpisodes.vue` : liste paginée ou à chargement progressif, tenant 500 épisodes (SC-009)
- [X] T054 [US1] Porter `uafricas_frontend/app/components/media/SectionChaine.vue` et `SectionStation.vue` : une rangée par **programme**, et non plus une vignette par vidéo
- [X] T055 [US1] Porter `uafricas_frontend/app/components/media/RangeeContenus.vue` et `CarteContenu.vue` sur les épisodes
- [X] T056 [P] [US1] Créer `uafricas_frontend/app/pages/medias/emissions-tele/[slug].vue` et `uafricas_frontend/app/pages/medias/emissions-radio/[slug].vue` : page programme avec SSR et balises Open Graph, sur le modèle des pages de détail existantes
- [X] T057 [US1] Transformer `uafricas_frontend/app/pages/medias/programmes-tele/[slug].vue` et `programmes-radio/[slug].vue` en **pages d'épisode** — emplacement et slug conservés, ce qui préserve les adresses publiques existantes (FR-056)
- [X] T058 [US1] Porter `uafricas_frontend/app/pages/medias/chaines/[slug].vue` et `stations/[slug].vue` : liste des programmes de la chaîne, chacun dépliant ses épisodes

### Frontend — gestion et back-office

- [X] T059 [P] [US1] Créer `uafricas_frontend/app/components/media/GestionEpisodes.vue` : ajout d'un épisode, réordonnancement, état de chaque épisode (en attente / publié / rejeté avec son motif), suivi de FR-042
- [X] T060 [US1] Étendre `uafricas_frontend/app/components/media/MesSupports.vue` : création de programmes et accès à `GestionEpisodes.vue` depuis `/mon-compte/mes-supports`
- [X] T061 [P] [US1] Créer `uafricas_frontend/app/composables/useAdminMediaEmissions.ts` sur la base `useAdmin` (adminFetch, listerPagine, pagination, sort)
- [X] T062 [P] [US1] Créer `uafricas_frontend/app/composables/useAdminMediaModeration.ts` : file, validation, rejet motivé
- [X] T063 [US1] Créer `uafricas_frontend/app/pages/admin/medias/emissions/index.vue` et `[id].vue` (daisyUI) : liste filtrable par support, état et cadence, et gestion des épisodes d'une émission
- [X] T064 [US1] Créer `uafricas_frontend/app/pages/admin/medias/moderation-episodes.vue` : file triée par échéance, affichant ancienneté, émission, support et heures restantes avant diffusion
- [X] T065 [US1] Ajouter les entrées de navigation vers la file de modération et la gestion des émissions dans `uafricas_frontend/app/pages/admin/medias/index.vue`

**Checkpoint**: US1 est complète et testable seule — création d'un programme, versement d'épisodes,
modération, navigation publique à deux niveaux, adresses historiques préservées.

---

## Phase 4: User Story 2 — Programmer une émission au rythme quotidien ou hebdomadaire (Priority: P2)

**Goal**: La grille porte sur des programmes ; l'épisode diffusé se déduit par rotation, sans tâche de
fond ; les détenteurs sont alertés avant l'échéance.

**Independent Test**: [quickstart.md](./quickstart.md) §Scénario 2 — programmer un hebdomadaire,
vérifier le déterminisme, faire avancer la rotation en reculant `date_effet`, contrôler la mention
« rediffusion » au bouclage.

### Backend

- [X] T066 [US2] Modifier `uafricas_backend/src/models/media_programmation.rs` : `CreneauRow` et `CreneauRequest` portent `emission_id` et `date_effet` ; `DiffusionResponse` gagne `emission`, `episode`, `rang_occurrence` et `est_rediffusion`
- [X] T067 [US2] Réécrire `SQL_DIFFUSION_EN_COURS` et `SQL_CRENEAU_SUIVANT` dans `uafricas_backend/src/handlers/media_programmation.rs` (lignes 41-75) : jointure sur l'émission, calcul du rang d'occurrence dans le fuseau du créneau et `JOIN LATERAL` de rotation — expression exacte dans [research.md](./research.md) R3
- [X] T068 [US2] Implémenter dans `uafricas_backend/src/handlers/media_programmation.rs` le double modulo `((rang % total) + total) % total` pour couvrir une `date_effet` future, et `est_rediffusion = rang >= total` (FR-020)
- [X] T069 [US2] Vérifier dans `uafricas_backend/src/handlers/media_programmation.rs` que la `JOIN LATERAL` intérieure suffit à ne rien annoncer quand l'émission n'a aucun épisode publié, sans branche supplémentaire (FR-021)
- [X] T070 [US2] Porter dans `uafricas_backend/src/handlers/media_programmation.rs` la création et la modification de créneau sur `emission_id`, en conservant le verrou `FOR UPDATE` sur le support parent avant détection de chevauchement, et le refus `409` sans écriture (FR-022)
- [X] T071 [US2] Ajouter dans `uafricas_backend/src/handlers/media_programmation.rs` le champ `date_effet` aux requêtes de créneau et renvoyer `episode_actuel` dans la réponse de création et de modification, pour que le détenteur voie l'effet immédiat de son origine de comptage
- [X] T072 [US2] Ajouter dans `uafricas_backend/src/handlers/media_programmation.rs` la vue `GET /api/medias/{type_support}/{support_id}/grille?vue=detenteur` remontant les créneaux en défaut, assortis de `alerte: "aucun_episode_publie"`
- [X] T073 [US2] Implémenter `GET /api/medias/mes-alertes-cadence` dans `uafricas_backend/src/handlers/media_programmation.rs` : calcul à la lecture, seuils `approche` à 2 jours en hebdomadaire et 6 heures en quotidien, champ `episodes_en_attente` pour ne pas accuser un détenteur dont l'épisode attend en file ([contracts/api-membre.md](./contracts/api-membre.md) §5)
- [X] T074 [US2] Ajouter `diffusion_en_cours` et `creneau_suivant` enrichis aux réponses `sections` de `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs`, sans requête supplémentaire par chaîne
- [X] T075 [US2] Déclarer les routes d'US2 dans `uafricas_backend/src/routes.rs`

### Frontend

- [X] T076 [US2] Porter `uafricas_frontend/app/composables/useMediaProgrammation.ts` : `emission_id`, `date_effet`, `est_rediffusion`, alertes de cadence
- [X] T077 [US2] Porter `uafricas_frontend/app/components/media/GrilleProgrammation.vue` : le créneau désigne un **programme**, avec sa cadence et son nombre d'épisodes, et le fuseau reste affiché explicitement (FR-026)
- [X] T078 [US2] Ajouter au formulaire de créneau de `uafricas_frontend/app/components/media/GrilleProgrammation.vue` le champ `date_effet` accompagné d'un aperçu de l'épisode qui passera à la prochaine occurrence
- [X] T079 [US2] Porter `uafricas_frontend/app/components/media/BandeauDiffusion.vue` : nom du programme, titre de l'épisode et badge « Rediffusion » lorsque `est_rediffusion` est vrai
- [X] T080 [P] [US2] Créer un panneau d'alertes de cadence dans `uafricas_frontend/app/components/media/MesSupports.vue`, distinguant `approche`, `depassee` et `aucun_episode`

**Checkpoint**: US1 et US2 fonctionnent ensemble ; la grille annonce le bon programme et le bon épisode,
de façon reproductible.

---

## Phase 5: User Story 3 — Déclarer plusieurs thématiques par chaîne ou station (Priority: P3)

**Goal**: Un support déclare 1..N thématiques du référentiel média, et le public filtre dessus.

**Independent Test**: [quickstart.md](./quickstart.md) §Scénario 3 — attribuer trois thématiques,
filtrer sur chacune, vérifier que le support remonte à chaque fois et une seule fois.

**Note de parallélisme** : la migration `09r` est purement additive et indépendante de `09q`. US3 et US4
peuvent être menées **en parallèle de la phase 2**.

- [X] T081 [P] [US3] Créer `uafricas_backend/doc/bd/schemas/09r_media_content_support_thematiques_territoires.sql` §1 : table `support_thematique` avec `UNIQUE (type_support, support_id, categorie_id)` et ses deux index ([data-model.md](./data-model.md) §2.3)
- [X] T082 [US3] Ajouter à `09r` la reprise : `chaine_tv.categorie` devient la première thématique par correspondance de libellé avec `shared.categorie` en contexte `media`, `ON CONFLICT DO NOTHING`, les chaînes sans correspondance restant sans thématique ([data-model.md](./data-model.md) §4, étape 6)
- [X] T083 [P] [US3] Créer `uafricas_backend/src/models/media_support.rs` : DTO de thématique et de couverture, constantes de colonnes
- [X] T084 [US3] Implémenter dans `uafricas_backend/src/handlers/media_detention.rs` (membre) et `uafricas_backend/src/handlers/admin/radio_tele.rs` (admin) les routes `GET` et `PUT …/thematiques` : remplacement intégral, refus `400` si la liste est vide sur un support publié ou si une catégorie n'est pas du contexte `media`
- [X] T085 [US3] Ajouter le paramètre `thematique` (répétable) aux endpoints `sections` de `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs`, un support multi-thématique ne remontant qu'une fois par résultat (FR-030)
- [X] T086 [US3] Implémenter dans `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs` les routes `GET …/thematiques` : uniquement les thèmes réellement déclarés, avec leur décompte — sur le modèle de `GET /api/experts/specialites`
- [X] T087 [P] [US3] Créer `uafricas_frontend/app/components/media/SelecteurThematiques.vue` : sélection multiple parmi les 44 thèmes, utilisable en public (Tailwind pur) comme en back-office
- [X] T088 [US3] Brancher le sélecteur dans les formulaires de chaîne et de station de `uafricas_frontend/app/pages/admin/medias/`, avec refus d'enregistrement d'un support publié sans thématique
- [X] T089 [US3] Étendre `uafricas_frontend/app/components/media/BarreFiltresTele.vue` et l'équivalent radio au filtre par thématique multiple, et afficher les thématiques sur les fiches `chaines/[slug].vue` et `stations/[slug].vue`

**Checkpoint**: Les trois premières histoires fonctionnent indépendamment.

---

## Phase 6: User Story 4 — Déclarer une couverture multi-territoires ou panafricaine (Priority: P4)

**Goal**: Un support déclare une liste de territoires ou une couverture continentale, exclusives l'une
de l'autre, et le public filtre par territoire.

**Independent Test**: [quickstart.md](./quickstart.md) §Scénario 4 — une chaîne à quatre territoires,
une chaîne continentale, filtrer sur un territoire et vérifier que les deux remontent.

- [X] T090 [US4] Ajouter à `uafricas_backend/doc/bd/schemas/09r_media_content_support_thematiques_territoires.sql` §2 : table `support_territoire`, colonne `couverture_continentale` sur `chaine_tv` et `station_radio`, et la reprise de `pays_id` en unique territoire ([data-model.md](./data-model.md) §2.4 et §4 étape 7)
- [X] T091 [US4] Ajouter à `09r` la fonction `media_content.verifier_couverture_exclusive()` et son trigger `BEFORE INSERT` sur `support_territoire`, qui rend l'ajout d'un territoire à un support continental impossible **en base** (FR-034)
- [X] T092 [US4] Implémenter dans `uafricas_backend/src/handlers/media_detention.rs` et `uafricas_backend/src/handlers/admin/radio_tele.rs` les routes `GET` et `PUT …/couverture` : refus `400` si les deux modes sont renseignés, refus `400` si aucun ne l'est sur un support publié, et suppression des lignes de territoire lors du passage à `couverture_continentale = TRUE` **dans la même transaction**
- [X] T093 [US4] Ajouter le paramètre `territoire` aux endpoints `sections` de `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs`, la clause remontant aussi les supports continentaux (`couverture_continentale = TRUE OR EXISTS (…)`, FR-036)
- [X] T094 [US4] Implémenter dans `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs` les routes `GET …/territoires`, incluant le marqueur `continentales` comptant les supports panafricains
- [X] T095 [P] [US4] Créer `uafricas_frontend/app/components/media/SelecteurCouverture.vue` : bascule « toute l'Afrique » neutralisant la sélection individuelle, terminologie « territoire » à l'écran conformément à la convention du projet
- [X] T096 [US4] Brancher `SelecteurCouverture.vue` dans les formulaires de `uafricas_frontend/app/pages/admin/medias/`, avec refus d'enregistrement d'un support publié sans couverture
- [X] T097 [US4] Ajouter le filtre par territoire à `uafricas_frontend/app/components/media/BarreFiltresTele.vue` et à son équivalent radio, et afficher la couverture sur `uafricas_frontend/app/pages/medias/chaines/[slug].vue` et `stations/[slug].vue`

**Checkpoint**: Les quatre premières histoires fonctionnent indépendamment.

---

## Phase 7: User Story 5 — Réagir à une émission comme à un épisode (Priority: P5)

**Goal**: Émission et épisode portent chacun leurs fils et leurs compteurs, sans agrégation.

**Independent Test**: [quickstart.md](./quickstart.md) §Scénario 5 — commenter un épisode puis son
programme, vérifier que les fils et les compteurs restent distincts et que la suspension de l'un
n'entraîne pas celle de l'autre.

> Le socle SQL (discriminant à six valeurs) a été livré en phase 2 (T005, T013, T020). Cette phase
> ouvre l'usage côté produit.

- [X] T098 [US5] Vérifier dans `uafricas_backend/src/handlers/media_social.rs` que le recompte de signalements et le seuil `SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA` s'appliquent **par cible** : suspendre un épisode ne suspend pas son émission (FR-050)
- [X] T099 [US5] Implémenter dans `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs` la propagation d'affichage : une émission suspendue retire ses épisodes de l'espace public sans les supprimer (FR-011)
- [X] T100 [US5] Servir depuis `uafricas_backend/src/handlers/media_social.rs` les compteurs d'émission et ceux de ses épisodes dans des champs **distincts**, sans aucun total agrégé (FR-048)
- [X] T101 [US5] Monter `uafricas_frontend/app/components/media/MediaReactionsBar.vue` et `MediaCommentaires.vue` sur les pages de programme `emissions-tele/[slug].vue` et `emissions-radio/[slug].vue`
- [X] T102 [US5] Présenter séparément dans `uafricas_frontend/app/pages/medias/emissions-tele/[slug].vue` et `emissions-radio/[slug].vue` les compteurs du programme et ceux de ses épisodes, en levant toute ambiguïté de lecture
- [X] T103 [US5] Brancher `uafricas_frontend/app/components/media/MediaPartagerModal.vue` sur les deux niveaux, le partage d'un épisode menant à l'épisode et celui d'un programme au programme (FR-049)
- [X] T104 [US5] Brancher `uafricas_frontend/app/components/media/MediaSignalerBouton.vue` sur les deux niveaux et étendre `uafricas_frontend/app/composables/useAdminMediaSignalements.ts` ainsi que `/admin/medias/signalements.vue` aux six cibles

**Checkpoint**: Les cinq histoires sont fonctionnelles et indépendamment vérifiables.

---

## Phase 8: Polish & Cross-Cutting Concerns

- [X] T105 Vérifier que `audit::log_action` couvre bien chaque mutation d'émission, d'épisode, d'ordre, de thématique, de couverture, de créneau et de décision de modération, en parcourant `/admin/audit` après avoir joué les scénarios 1 à 6 (SC-012)
- [X] T106 [P] Exécuter l'intégralité de [quickstart.md](./quickstart.md), scénarios 1 à 7, et consigner les écarts
- [X] T107 [P] Contrôler la conformité constitutionnelle : aucune classe daisyUI sur `uafricas_frontend/app/pages/medias/**` (principe VI), aucun nom de fichier accentué, libellés et identifiants en français
- [X] T108 Mesurer les performances du scénario 7 de [quickstart.md](./quickstart.md) avec `RUST_LOG=sqlx=debug` sur `uafricas_backend/` : le nombre de requêtes d'une page de sections ne doit pas croître avec le nombre de programmes, et `…/diffusion` doit rester à **2 requêtes** (SC-009, SC-010)
- [X] T109 Retirer `GET /api/television/categories` et `GET /api/television/pays` de `uafricas_backend/src/routes.rs` une fois le portage frontend confirmé, ainsi que la route jetable `POST /api/admin/medias/rapport-reprise`
- [X] T110 **Décision : conservées, elles ne sont pas muettes.** `chaine_tv.categorie` sert le filtre « Catégorie » du back-office ; `chaine_tv.pays_id` et `station_radio.pays_id` portent le **siège** du support, distinct de sa **couverture** (09r) — le siège dit d'où l'on émet, la couverture où l'on rayonne, et une chaîne panafricaine n'a qu'un siège ; `station_radio.genre` et `genres_liste` décrivent la couleur d'antenne, là où la thématique est déclarée par le support. Aucune migration de nettoyage. Décider du sort des colonnes devenues muettes du schéma `uafricas_backend/doc/bd/schemas/09_media_content.sql` (`chaine_tv.categorie`, `chaine_tv.pays_id`, `station_radio.genre`, `genres_liste`, `pays_id`) : les conserver ou les retirer dans une migration de nettoyage ultérieure, et consigner la décision
- [X] T111 Mettre à jour la section « Recent Changes » de `CLAUDE.md` — **une ligne**, citant les migrations `09q` et `09r` et les modules clés, conformément à la règle d'auto-maintenance du fichier
- [X] T112 Appliquer `09q` et `09r` en production via SSH+psql après `./deploy.sh update`, puis exécuter le rapport de reprise et traiter les supports listés sans thématique ni couverture

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance.
- **Foundational (Phase 2)** : dépend de la phase 1 — **bloque US1, US2 et US5**.
- **US1 (Phase 3)** : dépend de la phase 2.
- **US2 (Phase 4)** : dépend de la phase 2 ; s'appuie sur les émissions d'US1 pour être démontrable, mais son code est indépendant.
- **US3 (Phase 5)** et **US4 (Phase 6)** : ne dépendent **que** de la phase 1 — migration `09r` additive et indépendante de `09q`.
- **US5 (Phase 7)** : dépend de la phase 2 (socle SQL) et d'US1 (pages de programme sur lesquelles monter les composants).
- **Polish (Phase 8)** : dépend des histoires retenues.

### Ordre interne aux tâches

- T004 → T005 → T006 → T007 : sections d'un même fichier de migration, strictement séquentielles.
- T008 → T009 : appliquer avant de vérifier.
- Modèles (T010-T017) avant handlers (T018-T027) ; handlers avant `routes.rs` (T027) ; `routes.rs` avant `cargo check` (T028).
- T034-T039 avant T059-T060 : le composant de gestion consomme les endpoints.
- T066 → T067 → T068 → T069 : la rotation se construit par couches sur le même fichier.
- T081 avant T082 ; T090 avant T091.

### Parallel Opportunities

- Phase 1 : T002 et T003 en parallèle après T001.
- Phase 2 : T010, T011, T013, T014 en parallèle ; T029, T030, T031 en parallèle.
- Phase 3 : T051, T052, T053 en parallèle ; T056 en parallèle de T059 ; T061 et T062 en parallèle.
- **US3 et US4 en parallèle de toute la phase 2**, par une seconde personne — c'est la principale
  opportunité de parallélisme de cette feature.

---

## Parallel Example: Phase 2, modèles Rust

```bash
# Après l'application de la migration (T009), les quatre modèles sont indépendants :
Task: "Créer uafricas_backend/src/models/media_emission.rs"
Task: "Créer uafricas_backend/src/models/media_episode.rs"
Task: "Modifier uafricas_backend/src/models/media_social.rs — six valeurs de discriminant"
Task: "Modifier uafricas_backend/src/models/media_proposition.rs — quatre type_objet"
```

## Parallel Example: US3 et US4 en avance de phase

```bash
# Dès la fin de la phase 1, sans attendre la migration 09q :
Task: "Créer 09r_media_content_support_thematiques_territoires.sql §1 — support_thematique"
Task: "Créer uafricas_frontend/app/components/media/SelecteurThematiques.vue"
Task: "Créer uafricas_frontend/app/components/media/SelecteurCouverture.vue"
```

---

## Implementation Strategy

### MVP (US1 seule)

1. Phase 1 — Setup, sauvegarde et décision sur les contenus sans support.
2. Phase 2 — Fondations : migration `09q` et portage complet. **Point de contrôle impératif** :
   l'application redémarre, les contenus migrés sont visibles, les anciennes adresses résolvent.
3. Phase 3 — US1.
4. **ARRÊT ET VALIDATION** : jouer le scénario 1 de `quickstart.md` de bout en bout.
5. Démonstration possible : le catalogue est devenu navigable à deux niveaux.

### Livraison incrémentale

1. Fondations → base saine, aucune capacité nouvelle.
2. + US1 → programmes et épisodes, modération. **MVP**.
3. + US2 → grille et rotation.
4. + US3 → thématiques multiples.
5. + US4 → couverture territoriale.
6. + US5 → interactions à deux niveaux.

### Stratégie à deux personnes

- **A** : phase 1, puis toute la phase 2 (le chemin critique), puis US1 et US2.
- **B** : dès la fin de la phase 1, US3 puis US4 sur la migration `09r` indépendante, puis US5 une fois
  US1 livrée.

---

## Notes

- Les tâches `[P]` portent sur des fichiers distincts, sans dépendance mutuelle.
- La migration `09q` est **irréversible en pratique** : elle supprime deux tables. T002 (sauvegarde) et
  T009 (vérification) ne sont pas optionnelles.
- Aucune tâche de test automatisé : le projet n'a pas de harnais et la spécification n'en demande pas.
  La vérification passe par `quickstart.md`.
- Commiter après chaque tâche ou groupe logique, messages en français.
- `getDiagnostics` après chaque modification de fichier Vue ou Rust, conformément à `CLAUDE.md`.
