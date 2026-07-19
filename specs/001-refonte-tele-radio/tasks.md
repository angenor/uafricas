---
description: "Task list for feature implementation"
---

# Tasks: Refonte des pages Télé et Radio Africans

**Input**: Design documents from `/specs/001-refonte-tele-radio/`
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests** : aucune tâche de test automatisé. Le projet n'a ni harnais ni CI (constitution, « Contraintes
Techniques »), et la spécification n'en demande pas. La validation se fait par les parcours manuels de
[quickstart.md](./quickstart.md), référencés à chaque checkpoint.

**Organization** : tâches groupées par user story, chaque story restant implémentable et démontrable seule.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable — fichiers différents, aucune dépendance sur une tâche inachevée
- **[Story]** : US1 à US7, en correspondance avec les user stories de `spec.md`
- Chemins de fichiers exacts, relatifs à la racine du dépôt

## Path Conventions

Monorepo web : `uafricas_backend/` (Rust/Actix) et `uafricas_frontend/` (Nuxt 4).
Migrations SQL sous `uafricas_backend/doc/bd/schemas/`, orchestrées par `doc/bd/schema.sql`.

---

## Phase 1: Setup (ossature partagée)

**Purpose** : préparer les points d'ancrage pour que les phases suivantes n'entrent pas en conflit.

- [X] T001 Créer les 5 fichiers de migration vides avec en-tête « Migration idempotente » dans `uafricas_backend/doc/bd/schemas/` : `09j_media_content_editorial.sql`, `09k_media_content_interactions.sql`, `09l_media_content_propositions.sql`, `09m_media_content_codetention.sql`, `09n_media_content_programmation.sql`
- [X] T002 Déclarer les 5 migrations par `\ir schemas/09{j..n}_*.sql` dans `uafricas_backend/doc/bd/schema.sql`, immédiatement après la ligne de `09i` (une migration non déclarée casse toute initialisation fraîche)
- [X] T003 [P] Déclarer les nouveaux modules `media_social`, `media_proposition`, `media_detention`, `media_programmation` dans `uafricas_backend/src/handlers/mod.rs` et `uafricas_backend/src/models/mod.rs` (fichiers vides pour l'instant)
- [X] T004 [P] Vérifier le démarrage complet de l'environnement selon `quickstart.md` §0 : `docker compose up -d`, backend sur 8082, frontend sur 3000

---

## Phase 2: Foundational (bloquant — aucune story ne peut démarrer avant)

**Purpose** : socle de données et briques de lecture partagées par US1 et US2.

**⚠️ CRITIQUE** : les tâches T005 à T011 écrivent toutes dans `09j_media_content_editorial.sql` — elles
sont séquentielles, jamais parallèles.

### Migration `09j` — éditorial et corrections de dette

- [X] T005 Ajouter `origine_publication VARCHAR(20) NOT NULL DEFAULT 'territoire'` + `CHECK IN ('africans','territoire')` + index partiel sur `media_content.station_radio` dans `uafricas_backend/doc/bd/schemas/09j_media_content_editorial.sql` (FR-014)
- [X] T006 Ajouter les FK manquantes `fk_station_radio_pays` et `fk_station_radio_cree_par` via bloc `DO $$ … pg_constraint` idempotent dans `09j_media_content_editorial.sql` — la table n'a aucune FK aujourd'hui
- [X] T007 Ajouter `a_la_une_globale BOOLEAN NOT NULL DEFAULT FALSE` sur `media_content.programme_tele` + index unique partiel `uq_programme_tele_a_la_une_globale ON …((TRUE)) WHERE a_la_une_globale AND deleted_at IS NULL` dans `09j_media_content_editorial.sql` (FR-001)
- [X] T008 Ajouter `theme_phare_id` / `theme_phare_autre` sur `programme_tele` et `programme_radio`, `role_partie_prenante` / `_autre` + CHECK sur `chaine_tv` et `station_radio`, avec les CHECK « Autre exige une précision » dans `09j_media_content_editorial.sql` (FR-029, FR-030)
- [X] T009 Élargir le `CHECK etat` des 4 tables médias à `'en_attente'` et ajouter `nombre_signalements INT NOT NULL DEFAULT 0` dans `09j_media_content_editorial.sql` (FR-032, FR-050)
- [X] T010 Insérer les 43 thèmes phares dans `shared.categorie` avec `contexte = 'media'` via `INSERT … ON CONFLICT (slug) DO NOTHING` dans `09j_media_content_editorial.sql` — libellés repris de `spec.md` § Key Entities
- [X] T011 Insérer les permissions `media.voir` / `media.modifier` / `media.supprimer` dans `iam.permission` + liaison au rôle `admin` dans `iam.role_permission`, et élargir `arbre_genealogique.notifications.type` en `VARCHAR(80)`, dans `09j_media_content_editorial.sql` — sans cela seul `super_admin` peut modérer (R15) et les types de notification débordent (R14)
- [X] T012 Jouer `09j` sur la base locale et vérifier l'idempotence en le rejouant : `psql -h localhost -U uafricas -d africans_db -f uafricas_backend/doc/bd/schemas/09j_media_content_editorial.sql`

### Briques Rust partagées

- [X] T013 [P] Ajouter `origine_publication`, `role_partie_prenante`, `role_partie_prenante_autre`, `nombre_signalements` aux structs `StationRadioRow` / `StationRadioResponse` / `CreerStationRadioForm` et à `STATION_RADIO_COLONNES` dans `uafricas_backend/src/models/station_radio.rs`
- [X] T014 [P] Ajouter `a_la_une_globale`, `theme_phare_id`, `theme_phare_autre`, `source_media`, `nombre_signalements` aux structs et constantes `COLONNES` de `uafricas_backend/src/models/television.rs`
- [X] T015 [P] Répercuter les mêmes champs sur les DTO admin dans `uafricas_backend/src/models/admin/radio_tele.rs` (les `*Detail` exposent en plus `etat`, `cree_par`, `updated_at`)

### Briques frontend partagées

- [X] T016 [P] Extraire `youtubeEmbedUrl` de `uafricas_frontend/app/composables/useEvenements.ts:285-301` vers `uafricas_frontend/app/utils/media.ts`, y ajouter `estMediaExterne(url)` et un réexport depuis `useEvenements.ts` pour ne pas casser ses 3 consommateurs actuels
- [X] T017 Créer `uafricas_frontend/app/components/media/LecteurMedia.vue` : route selon la source — `youtubeEmbedUrl()` non nul → `<iframe allow="autoplay; encrypted-media; picture-in-picture; fullscreen">` avec `mute=1` obligatoire, sinon `<video>` / `<audio>` natif ; repli explicite si aucun des deux (FR-056, edge case « média externe défaillant »)
- [X] T018 Créer `uafricas_frontend/app/composables/useLecteurMedia.ts` : état global par `useState` (`'media:lecture'`, `'media:contenu'`, `'media:volume'`), API `lire / pause / basculerSon / definirVolume`, garantie d'un flux unique (FR-018). Supprimer `uafricas_frontend/app/composables/useAudioPlayer.ts` (code mort, `ref()` locaux et `onUnmounted` qui coupe le son — R8)
- [X] T019 [P] Créer `uafricas_frontend/app/composables/useObservateurVisibilite.ts` (IntersectionObserver) — aucun mécanisme de chargement différé n'existe dans le projet (FR-054, SC-011)
- [X] T020 [P] Créer `uafricas_frontend/app/components/media/RangeeContenus.vue` : rangée horizontale `flex flex-nowrap gap-4 overflow-x-auto scrollbar-none snap-x snap-mandatory -mx-1 px-1`, cartes en `shrink-0 snap-start`, flèches au clavier (FR-022, FR-053)

**Checkpoint** : socle prêt — US1 et US2 peuvent démarrer, en parallèle si l'équipe le permet.

---

## Phase 3: User Story 1 — Page Télé, vedette plein écran et chaînes en sections (Priority: P1) 🎯 MVP

**Goal** : remplacer la grille de vignettes par une vedette occupant tout l'écran à l'ouverture, puis une
section par chaîne au défilement, chacune avec son contenu mis en évidence et ses autres programmes.

**Independent Test** : désigner un programme comme vedette générale, ouvrir `/medias/tele`, vérifier qu'il
occupe toute la fenêtre et démarre, puis que le défilement révèle une section par chaîne.

### Backend

- [X] T021 [US1] Implémenter `GET /api/television/vedette` dans `uafricas_backend/src/handlers/television.rs` : programme `a_la_une_globale = TRUE AND etat = 'publie'`, repli sur le programme publié le plus récent avec `est_repli: true`, `data: null` si aucun (FR-001, FR-007)
- [X] T022 [US1] Implémenter `GET /api/television/sections` dans `uafricas_backend/src/handlers/television.rs` : chaînes paginées (`par_page` 6) avec `mis_en_evidence` + `contenus` (12 max) + `total_contenus`, `ORDER BY nom ASC, id ASC` pour un ordre stable, exclusion des chaînes sans contenu publié (FR-004, FR-005, FR-008)
- [X] T023 [US1] Implémenter `GET /api/television/chaines/{slug}` et `GET /api/television/programmes/{slug}` dans `uafricas_backend/src/handlers/television.rs` (résolution par slug, requise par les pages SSR d'US3)
- [X] T024 [US1] Déclarer les 4 routes dans le scope `/api/television` de `uafricas_backend/src/routes.rs`
- [X] T025 [US1] Implémenter `PATCH /api/admin/television/programmes-tele/{id}/vedette-globale` dans `uafricas_backend/src/handlers/admin/radio_tele.rs` : bascule de l'ancienne vedette à `FALSE` puis mise à jour **dans une même transaction**, `verifier_permission!(admin, "media", "modifier")`, `audit::log_action` avec les deux instantanés JSONB
- [X] T026 [US1] Corriger l'exclusivité `a_la_une` par chaîne et par station dans `uafricas_backend/src/handlers/admin/radio_tele.rs:1256-1265` et `:1392-1407` : les deux requêtes hors transaction échoueraient désormais en concurrence face à l'index unique

### Frontend

- [X] T027 [US1] Étendre `uafricas_frontend/app/composables/useTelevision.ts` : `obtenirVedette()`, `listerSections(filtres)`, `obtenirChaineParSlug(slug)`, interfaces `TeleSection` / `ProgrammeVedette` avec `source_media` et `est_repli`
- [X] T028 [US1] Créer `uafricas_frontend/app/components/media/VedettePleinEcran.vue` : `h-[100svh]` (jamais `100vh`, qui déborde sous la barre d'URL mobile), `top-24` pour dégager la NavBar (`absolute`, donc défilante), lecture auto son coupé via `LecteurMedia`, commandes son et pause visibles et atteignables au clavier, repère de défilement (FR-002, FR-003, FR-009)
- [X] T029 [US1] Créer `uafricas_frontend/app/components/media/SectionChaine.vue` : bloc empilé de hauteur naturelle — identité de la chaîne, bandeau du contenu mis en évidence avec titre, description et action de lecture, puis `RangeeContenus` ; montage du lecteur différé par `useObservateurVisibilite` (FR-005, FR-006, FR-022, FR-054)
- [X] T030 [US1] Remanier `uafricas_frontend/app/pages/medias/tele.vue` : retirer `videoProvisoireEmbed` et son bloc iframe (FR-010), retirer le `v-if="!isMobile"` qui prive les mobiles de hero (FR-011), remplacer la grille et les filtres par `VedettePleinEcran` + liste de `SectionChaine` chargée au défilement, conserver `MediaTelePresentationModal`
- [X] T031 [US1] Migrer les résidus Tailwind v3 de `uafricas_frontend/app/pages/medias/tele.vue` : `bg-gradient-to-*` → `bg-linear-to-*` (Principe VI)
- [X] T032 [P] [US1] Ajouter le champ « Vedette générale de la page Télé » aux formulaires `uafricas_frontend/app/pages/admin/television/create.vue` et `[id].vue`, avec avertissement explicite que la vedette précédente sera remplacée
- [X] T033 [P] [US1] Étendre `uafricas_frontend/app/composables/useAdminTelevision.ts` : `definirVedetteGlobale(id)` et champs `theme_phare_id` / `theme_phare_autre` dans les formulaires

**Checkpoint** : parcours 1 à 8 et 14-15 de `quickstart.md` § Lot 1 passent. **MVP démontrable.**

---

## Phase 4: User Story 2 — Pages Radio en sections, réellement distinctes (Priority: P1)

**Goal** : donner aux deux pages Radio la même structure en sections, avec une écoute qui survit au
défilement et à la navigation, et rendre leur différenciation effective par l'origine de publication.

**Independent Test** : ouvrir les deux pages et vérifier qu'aucune station n'apparaît sur les deux, que
chaque section porte son contenu mis en évidence, et que l'écoute survit au changement de page.

### Backend

- [X] T034 [P] [US2] Créer `uafricas_backend/src/models/programme_radio.rs` : `PROGRAMME_RADIO_COLONNES`, `ProgrammeRadioRow`, `ProgrammeRadioResponse`, `ProgrammeRadioQueryParams` — comble D-002, ces contenus n'ont aujourd'hui aucune exposition publique
- [X] T035 [US2] Implémenter `GET /api/programmes-radio` et `GET /api/programmes-radio/{slug}` dans `uafricas_backend/src/handlers/stations_radio.rs`, à parité avec les programmes de télévision (FR-020)
- [X] T036 [US2] Implémenter `GET /api/stations-radio/sections` dans `uafricas_backend/src/handlers/stations_radio.rs` : paramètre `origine` (`africans` | `territoire`) porté par la page et non par l'utilisateur, cumulable avec les filtres `type_station` / `pays` / `genre` (FR-013, FR-014)
- [X] T037 [US2] Implémenter `GET /api/stations-radio/{slug}` et ajouter `origine` à `StationRadioQueryParams` dans `uafricas_backend/src/handlers/stations_radio.rs` + `models/station_radio.rs`
- [X] T038 [US2] Déclarer les nouvelles routes dans les scopes `/api/stations-radio` et `/api/programmes-radio` de `uafricas_backend/src/routes.rs`
- [X] T039 [US2] Corriger la sentinelle désynchronisée `pays != "Tous les pays"` de `uafricas_backend/src/handlers/stations_radio.rs:54` — le frontend envoie « Tous les territoires »
- [X] T040 [P] [US2] Ajouter `PATCH /api/admin/stations-radio/{id}/origine` et le champ `origine_publication` aux CRUD admin dans `uafricas_backend/src/handlers/admin/radio_tele.rs`, avec `audit::log_action` action `CHANGEMENT_ORIGINE`

### Frontend

- [X] T041 [US2] Étendre `uafricas_frontend/app/composables/useStationsRadio.ts` : `listerSections({ origine, … })`, `listerContenusStation(stationId)`, interface `StationSection` ; corriger le cast non contrôlé de `type_station` en `programType` (`:106`)
- [X] T042 [US2] Créer `uafricas_frontend/app/components/media/BarreLecturePersistante.vue` : `fixed bottom-0` pleine largeur, titre écouté, station, lecture/pause, volume ; z-index sous `z-[75]` et décalage du FAB messagerie (`bottom-6 right-6`) et de l'invite d'appel (`bottom-24 right-6`) quand elle est active (FR-017)
- [X] T043 [US2] Monter `<MediaBarreLecturePersistante v-if="lectureEnCours" />` dans `uafricas_frontend/app/layouts/default.vue`, **hors du `<slot/>`**, sous `<ClientOnly>` — c'est ce placement qui fait survivre le son à la navigation (R8)
- [X] T044 [US2] Créer `uafricas_frontend/app/components/media/SectionStation.vue` : identité de la station, contenu mis en évidence, direct proposé au même titre quand `stream_url` existe (FR-016), `RangeeContenus` des autres contenus
- [X] T045 [US2] Remanier `uafricas_frontend/app/pages/medias/radio/africans.vue` : bandeau d'accroche sans lecteur en tête (FR-013), sections par station, `origine: 'africans'` en dur, message d'état vide explicite (FR-019)
- [X] T046 [US2] Remanier `uafricas_frontend/app/pages/medias/radio/nationales.vue` à l'identique avec `origine: 'territoire'` — **les deux pages restent distinctes**, aucune factorisation en une page unique, aucune redirection (FR-012)
- [X] T047 [P] [US2] Remplacer `loading loading-spinner loading-lg` (daisyUI) par `animate-spin rounded-full h-12 w-12 border-b-2 border-yellow-400` dans `africans.vue:188` et `nationales.vue:189`, et migrer `bg-gradient-to-*` → `bg-linear-to-*` (Principe VI)
- [X] T048 [P] [US2] Ajouter le sélecteur « Origine de publication » aux formulaires `uafricas_frontend/app/pages/admin/radio/create.vue` et `[id].vue`, et `definirOrigine()` dans `uafricas_frontend/app/composables/useAdminRadio.ts`
- [X] T049 [US2] Supprimer `uafricas_frontend/app/components/media/AudioPlayer.vue` et `StationCard.vue`, devenus sans usage, ainsi que leurs imports depuis `~/mocks/radios`

**Checkpoint** : parcours 9 à 15 de `quickstart.md` § Lot 1 passent. **Lot 1 complet et déployable.**

---

## Phase 5: User Story 3 — Réagir, commenter et partager (Priority: P2)

**Goal** : ouvrir la participation sur les contenus télé et radio, et les rendre partageables vers l'espace
communautés et les réseaux sociaux.

**Independent Test** : réagir, commenter et partager un contenu, vérifier l'unicité de la réaction, la
persistance du commentaire, l'apparition du partage sur `/publications` et l'aperçu social correct.

### Migration et backend

- [X] T050 [US3] Écrire `uafricas_backend/doc/bd/schemas/09k_media_content_interactions.sql` : `media_reaction`, `media_commentaire`, `partage_media`, `signalement_media`, toutes discriminées par `type_media VARCHAR(20) + CHECK` sur les 4 valeurs, avec les contraintes d'unicité de `data-model.md` §2
- [X] T051 [US3] Jouer `uafricas_backend/doc/bd/schemas/09k_media_content_interactions.sql` et vérifier son idempotence par un second passage
- [X] T052 [P] [US3] Créer `uafricas_backend/src/models/media_social.rs` : `TYPES_MEDIA_AUTORISES` (whitelist de littéraux, jamais d'interpolation de l'entrée brute), `table_pour_type()`, DTO réaction / commentaire / partage / signalement
- [X] T053 [US3] Implémenter réactions et commentaires dans `uafricas_backend/src/handlers/media_social.rs` : `POST …/reaction` en `ON CONFLICT DO UPDATE` avec retrait sur `null`, `POST` / `GET` / `DELETE` commentaires (soft delete réservé à l'auteur) (FR-023, FR-024)
- [X] T054 [US3] Implémenter partages dans `uafricas_backend/src/handlers/media_social.rs` : `POST …/partages` avec légende ≤ 500, `GET /api/medias/partages` paginé pour le mur (FR-025)
- [X] T055 [US3] Enrichir les DTO de détail et de section de `television.rs` et `stations_radio.rs` avec `nombre_likes`, `nombre_dislikes`, `ma_reaction`, `nombre_commentaires`, `nombre_partages` — évite un aller-retour par carte (FR-027)
- [X] T056 [US3] Déclarer le scope `/api/medias` et ses routes dans `uafricas_backend/src/routes.rs`

### Pages de détail — prérequis du partage

- [X] T057 [P] [US3] Créer `uafricas_frontend/app/pages/medias/programmes-tele/[slug].vue` : `await useAsyncData` au niveau racine puis `useHead(() => …)` produisant `og:type/title/description/url/image`, `twitter:card` et `canonical`, sur le modèle de `opportunite-afrique/[id]/sites/[siteId].vue:297-370` — **ne pas reproduire l'absence d'Open Graph de `vidafrica/[slug].vue`**
- [X] T058 [P] [US3] Créer `uafricas_frontend/app/pages/medias/programmes-radio/[slug].vue` sur le même modèle
- [X] T059 [P] [US3] Créer `uafricas_frontend/app/pages/medias/chaines/[slug].vue` : identité de la chaîne, ses contenus, ses interactions
- [X] T060 [P] [US3] Créer `uafricas_frontend/app/pages/medias/stations/[slug].vue` sur le même modèle

### Composants et intégration

- [X] T061 [P] [US3] Créer `uafricas_frontend/app/composables/useMediaSocial.ts` : `reagir`, `listerCommentaires`, `commenter`, `supprimerCommentaire`, `partager`, `listerPartages`
- [X] T062 [P] [US3] Créer `uafricas_frontend/app/components/media/MediaReactionsBar.vue` — mise à jour optimiste resynchronisée par watch, émission de `require-login` hors session (modèle `opportunite-afrique/ReactionsBar.vue`)
- [X] T063 [P] [US3] Créer `uafricas_frontend/app/components/media/MediaCommentaires.vue` : liste plate paginée, formulaire 1–2000 caractères, suppression réservée à l'auteur
- [X] T064 [P] [US3] Créer `uafricas_frontend/app/components/media/MediaPartagerModal.vue` : réseaux sociaux externes + mur communautaire avec légende (modèle `opportunite-afrique/PartagerElementModal.vue`)
- [X] T065 [US3] Brancher `MediaReactionsBar`, `MediaCommentaires` et `MediaPartagerModal` dans les 4 pages de détail et sous le contenu mis en évidence de `SectionChaine.vue` / `SectionStation.vue`
- [X] T066 [US3] Ajouter la 8ᵉ source au mur : interface `PublicationMediaPartage`, entrée de `filtres[]`, `STYLES_PAR_TYPE`, compteur, bloc `Promise.allSettled` et branche `v-else-if` dans `uafricas_frontend/app/pages/publications/index.vue` (6 points de modification, cf. R3)
- [X] T067 [P] [US3] Créer `uafricas_frontend/app/components/publications/MediaPartageCard.vue`
- [X] T068 [US3] Gérer le contenu retiré : renvoyer 404 depuis `uafricas_backend/src/handlers/television.rs` et `stations_radio.rs` quand `etat <> 'publie'`, et afficher un message explicite dans les 4 pages `uafricas_frontend/app/pages/medias/{chaines,stations,programmes-tele,programmes-radio}/[slug].vue` (FR-028)

**Checkpoint** : parcours 1 à 5 et 13 de `quickstart.md` § Lot 2 passent.

---

## Phase 6: User Story 4 — Soumission par les parties prenantes, validée par l'administrateur (Priority: P2)

**Goal** : ouvrir la contribution à tout membre connecté tout en garantissant qu'aucun contenu n'atteint le
public sans validation — ce qui ferme au passage une faille ouverte.

**Independent Test** : soumettre une chaîne depuis un compte membre, constater son absence des pages
publiques, la valider en back-office et la voir apparaître avec son auteur devenu propriétaire.

### Migration et backend

- [X] T069 [US4] Écrire `uafricas_backend/doc/bd/schemas/09l_media_content_propositions.sql` : ENUM `type_objet_propose` (6 valeurs) et `statut_proposition_media`, table `proposition_media` avec ses 4 CHECK d'intégrité et ses 3 index (`data-model.md` §3)
- [X] T070 [US4] Jouer `uafricas_backend/doc/bd/schemas/09l_media_content_propositions.sql` et vérifier par `psql` qu'il est impossible d'insérer une ligne `validee` sans `objet_id_cree`, ni `rejetee` sans `commentaire_decision`
- [X] T071 [P] [US4] Créer `uafricas_backend/src/models/media_proposition.rs` : DTO de soumission, de suivi et de modération, validation par `type_objet` du contenu de `donnees`
- [X] T072 [US4] Implémenter `POST /api/medias/propositions` (multipart) dans `uafricas_backend/src/handlers/media_proposition.rs` : `statut` forcé à `'en_attente'`, `origine_publication` **forcée à `'territoire'`** côté serveur, refus si « Autre » sans précision (FR-029, FR-030, FR-031, FR-036)
- [X] T073 [US4] Implémenter `GET /api/medias/propositions/moi` et `PATCH /api/medias/propositions/{id}/retirer` dans `uafricas_backend/src/handlers/media_proposition.rs` — comble le trou de suivi de vidafrica (FR-034)
- [X] T074 [US4] Ajouter `pub mod media { … }` de constantes de notification dans `uafricas_backend/src/models/notification.rs` : `PROPOSITION_VALIDEE`, `PROPOSITION_REJETEE`, `CODETENTEUR_AJOUTE`, `CONTENU_SUSPENDU`
- [X] T075 [US4] Implémenter la file de modération dans `uafricas_backend/src/handlers/admin/media_proposition.rs` : `GET` liste et détail, `PATCH …/valider`, `PATCH …/rejeter` (motif ≥ 10 caractères), sous `verifier_permission!(admin, "media", …)`
- [X] T076 [US4] Implémenter la transaction de validation dans `uafricas_backend/src/handlers/admin/media_proposition.rs` : `SELECT … FOR UPDATE` → `INSERT` de l'objet métier → `INSERT` du premier co-détenteur `proprietaire` → `UPDATE` de la proposition → `INSERT` de la notification **dans la transaction** → `COMMIT` → audit
- [X] T077 [US4] **Fermer la faille** : retirer les `etat = 'publie'` codés en dur de `uafricas_backend/src/handlers/stations_radio.rs:263`, `handlers/television.rs:207` et `:428` — router vers `proposition_media` ou insérer en `'en_attente'` (FR-032)
- [X] T078 [US4] Implémenter `PATCH /api/medias/contenus/{type_media}/{id}/metadonnees` (publication immédiate) et `PUT …/media` (bascule en `'en_attente'` + proposition de modification) dans `uafricas_backend/src/handlers/media_proposition.rs` (FR-032)
- [X] T079 [US4] Déclarer les routes membre et admin dans `uafricas_backend/src/routes.rs`

### Frontend

- [X] T080 [P] [US4] Créer `uafricas_frontend/app/composables/useMediaProposition.ts` : `soumettre` (FormData), `mesPropositions`, `retirer`, `modifierMetadonnees`, `remplacerMedia`
- [X] T081 [US4] Créer `uafricas_frontend/app/components/media/ProposerMediaModal.vue` : formulaire par `type_objet`, sélecteur de rôle de partie prenante et de thème phare avec « Autre » + précision obligatoire, téléversement ou lien externe. Reprendre le modèle de champs de `MediaAddProgramModal` (D-006) puis **supprimer `uafricas_frontend/app/components/media/AddProgramModal.vue`**, maquette morte dont `handleSubmit` simule l'envoi
- [X] T082 [P] [US4] Créer `uafricas_frontend/app/pages/mon-compte/propositions-medias.vue` : suivi des soumissions avec état et motif de refus (FR-034)
- [X] T083 [P] [US4] Créer `uafricas_frontend/app/composables/useAdminMediaPropositions.ts` et la page `uafricas_frontend/app/pages/admin/medias/propositions/index.vue` (file filtrable par statut et type) plus `[id].vue` (détail, valider, rejeter)
- [X] T084 [US4] Faire apparaître en évidence, dans `[id].vue`, la source du média et l'auteur déclaré — aucune décharge de droits n'étant recueillie, l'administrateur est seul à se prononcer sur la licéité (H-012, FR-033)
- [X] T085 [US4] Ajouter le bouton « Proposer un contenu » ouvrant `ProposerMediaModal` dans `uafricas_frontend/app/pages/medias/tele.vue`, `radio/africans.vue`, `radio/nationales.vue` et les 4 pages `medias/{chaines,stations,programmes-tele,programmes-radio}/[slug].vue`

**Checkpoint** : parcours 6 à 12 de `quickstart.md` § Lot 2 passent. **Lot 2 complet et déployable.**

---

## Phase 7: User Story 5 — Programmation automatique (Priority: P3)

**Goal** : permettre aux co-détenteurs d'établir une grille quotidienne ou hebdomadaire dont les contenus se
diffusent d'eux-mêmes, sans tâche de fond.

**Independent Test** : planifier un contenu à deux minutes d'ici et constater qu'à l'échéance la section le
diffuse et affiche le créneau suivant.

### Migrations et backend

- [X] T086 [US5] Écrire `uafricas_backend/doc/bd/schemas/09m_media_content_codetention.sql` : ENUM `type_support_media` et `role_detenteur`, tables `support_detenteur` (avec `uq_support_un_proprietaire`) et `invitation_detenteur` (`data-model.md` §4)
- [X] T087 [US5] Écrire `uafricas_backend/doc/bd/schemas/09n_media_content_programmation.sql` : `creneau_programmation` avec `heure_debut TIME`, `jour_semaine SMALLINT`, `fuseau`, et les CHECK de cohérence jour/récurrence et de non-franchissement de minuit
- [X] T088 [US5] Jouer `uafricas_backend/doc/bd/schemas/09m_media_content_codetention.sql` puis `09n_media_content_programmation.sql`, et vérifier leur idempotence par un second passage
- [X] T089 [US5] Implémenter `garde_detenteur(pool, type_support, support_id, moi, roles_admis)` dans `uafricas_backend/src/handlers/media_detention.rs` — **ne pas** utiliser l'extracteur `AdminUtilisateur`, qui rejette tout non-admin (modèle `garde_proprietaire`, `handlers/annonces.rs:111`)
- [X] T090 [US5] Implémenter la gestion des co-détenteurs dans `uafricas_backend/src/handlers/media_detention.rs` : listage, invitation par le propriétaire, acceptation et refus, retrait — avec la logique d'ajout à trois branches de `admin/moderateurs_afrolang.rs:59-190`
- [X] T091 [P] [US5] Créer `uafricas_backend/src/models/media_programmation.rs` : DTO de créneau, validation de `recurrence` / `jour_semaine` / `duree_minutes`
- [X] T092 [US5] Implémenter le CRUD des créneaux dans `uafricas_backend/src/handlers/media_programmation.rs` : verrou `SELECT id FROM … FOR UPDATE` sur le **support parent** avant toute écriture, détection de chevauchement, `409` détaillé sans écriture en cas de conflit (FR-040)
- [X] T093 [US5] Implémenter la résolution paresseuse du créneau courant dans `uafricas_backend/src/handlers/media_programmation.rs` : calcul SQL `(NOW() AT TIME ZONE fuseau)` à la lecture, aucune tâche de fond (R7, FR-038, FR-042)
- [X] T094 [US5] Brancher `diffusion_en_cours` et `creneau_suivant` sur les endpoints `sections` de `television.rs` et `stations_radio.rs`, avec repli sur le contenu mis en évidence si le contenu programmé n'est plus publié (FR-041, FR-043)
- [X] T095 [US5] Implémenter `GET /api/medias/{type_support}/{support_id}/grille` et déclarer les routes de co-détention et de programmation dans `uafricas_backend/src/routes.rs`
- [X] T096 [P] [US5] Ajouter la gestion admin des co-détenteurs dans `uafricas_backend/src/handlers/admin/media_proposition.rs` (listage, ajout, retrait) sous `verifier_permission!(admin, "media", …)`

### Frontend

- [X] T097 [P] [US5] Créer `uafricas_frontend/app/composables/useMediaDetention.ts` et `useMediaProgrammation.ts`
- [X] T098 [P] [US5] Créer `uafricas_frontend/app/components/media/GrilleProgrammation.vue` : vue hebdomadaire, création et édition de créneau, affichage du fuseau, signalement des créneaux invalides
- [X] T099 [P] [US5] Créer `uafricas_frontend/app/components/media/GestionCoDetenteurs.vue` : liste, invitation par courriel, retrait, rôles
- [X] T100 [US5] Afficher « En ce moment » et « À suivre » dans `SectionChaine.vue` et `SectionStation.vue` quand une grille est active (FR-039)
- [X] T101 [P] [US5] Créer `uafricas_frontend/app/pages/mon-compte/mes-supports.vue` : supports co-détenus, accès à la grille et aux co-détenteurs
- [X] T102 [P] [US5] Créer `uafricas_frontend/app/pages/mon-compte/invitations-medias.vue` : invitations reçues, acceptation et refus

**Checkpoint** : parcours 1 à 8 de `quickstart.md` § Lot 3 passent.

---

## Phase 8: User Story 6 — Engagement : idées, animation, réalisateurs (Priority: P3)

**Goal** : permettre aux visiteurs de proposer des sujets, aux parties prenantes de demander l'animation
d'un programme, et aux porteurs de projet de trouver des réalisateurs ou producteurs.

**Independent Test** : déposer une idée sur une chaîne, soumettre une demande d'animation et constater
qu'une acceptation ajoute le demandeur aux co-détenteurs, puis rechercher un réalisateur et le contacter.

- [X] T103 [US6] Implémenter les types `idee_contenu` et `animation_programme` dans `uafricas_backend/src/handlers/media_proposition.rs` : `target_id` obligatoire, aucun objet créé pour une idée, création d'une ligne `support_detenteur` à l'acceptation d'une demande d'animation (FR-044, FR-045)
- [X] T104 [US6] Exposer les propositions aux co-détenteurs concernés — et non aux seuls administrateurs — dans `uafricas_backend/src/handlers/media_proposition.rs` (FR-047)
- [X] T105 [P] [US6] Ajouter « Réalisateur », « Producteur », « Cadreur », « Monteur », « Animateur radio » à `iam.specialite_bibliotheque` par `INSERT … ON CONFLICT DO NOTHING` dans `uafricas_backend/doc/bd/schemas/09m_media_content_codetention.sql`
- [X] T106 [US6] Ajouter un filtre `$n = ANY(e.specialites)` à `lister_experts` dans `uafricas_backend/src/handlers/experts.rs:76-86` — le tableau `specialites` n'est aujourd'hui pas filtrable (FR-046)
- [X] T107 [US6] Implémenter `POST /api/medias/{type_support}/{support_id}/contacter` dans `uafricas_backend/src/handlers/media_detention.rs` : dupliquer `contacter_auteur` et `obtenir_ou_creer_conversation_annonce` (`handlers/annonces.rs:146,893`) en respectant `paire_canonique`, aucun endpoint générique d'ouverture de conversation n'existant (R17)
- [X] T108 [P] [US6] Créer `uafricas_frontend/app/components/media/ProposerIdeeModal.vue` et `DemanderAnimationModal.vue`
- [X] T109 [P] [US6] Ajouter le filtre par spécialité à la page `uafricas_frontend/app/pages/experts/index.vue` et à `uafricas_frontend/app/composables/useExperts.ts`
- [X] T110 [US6] Brancher `ProposerIdeeModal` et `DemanderAnimationModal` dans `uafricas_frontend/app/components/media/SectionChaine.vue` et `SectionStation.vue`, et le bouton « Contacter » dans `uafricas_frontend/app/pages/medias/chaines/[slug].vue` et `stations/[slug].vue`

**Checkpoint** : parcours 9 et 10 de `quickstart.md` § Lot 3 passent.

---

## Phase 9: User Story 7 — Signalement des contenus interdits (Priority: P3)

**Goal** : permettre à tout membre de signaler un contenu contraire aux règles et retirer automatiquement
de l'antenne ceux qui franchissent le seuil.

**Independent Test** : signaler depuis 11 comptes distincts et constater le retrait automatique, puis le
rétablissement en back-office avec remise à zéro du compteur.

**Prérequis** : la table `signalement_media` et la colonne `nombre_signalements` sont créées par `09k`
(T050) et `09j` (T009). Si US3 n'a pas été livrée, jouer `09k` avant cette phase.

- [X] T111 [US7] Déclarer `pub const SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA: i64 = 10;` dans `uafricas_backend/src/models/media_social.rs` — comparateur `>` (suspension au 11ᵉ signalement distinct), aligné sur les deux mécanismes les plus récents du projet
- [X] T112 [US7] Implémenter `POST /api/medias/{type_media}/{media_id}/signalement` dans `uafricas_backend/src/handlers/media_social.rs` : `INSERT … ON CONFLICT DO NOTHING` → `COUNT(*)` distinct → bascule `etat = 'suspendu'` au-dessus du seuil (et non une colonne `suspendu`, absente de ces tables) → `audit::log_action` action `SIGNALEMENT` ou `SIGNALEMENT_SUSPENSION` (FR-049, FR-050)
- [X] T113 [US7] Implémenter `GET /api/admin/medias/signalements` et `PATCH /api/admin/medias/{type_media}/{id}/etat` dans `uafricas_backend/src/handlers/admin/media_proposition.rs` : le rétablissement remet `nombre_signalements = 0`, faute de quoi le contenu serait resuspendu au signalement suivant (FR-051)
- [X] T114 [US7] Vérifier le filtre `etat = 'publie'` dans `uafricas_backend/src/handlers/television.rs` (vedette et sections), `stations_radio.rs` (sections) et `media_programmation.rs` (créneau courant) : un contenu suspendu doit disparaître à la requête suivante et faire basculer la page sur son repli (edge case « contenu signalé pendant sa diffusion »)
- [X] T115 [P] [US7] Créer `uafricas_frontend/app/components/media/MediaSignalerModal.vue` et `MediaSignalerBouton.vue` (modèle `opportunite-afrique/SignalerContributionModal.vue`)
- [X] T116 [P] [US7] Créer `uafricas_frontend/app/components/media/ReglesContenuModal.vue` énonçant les contenus interdits — violence, racisme, discrimination, mauvaise gouvernance, corruption — et l'ouvrir depuis les trois pages médias (FR-048)
- [X] T117 [P] [US7] Créer `uafricas_frontend/app/pages/admin/medias/signalements.vue` : file triée par nombre de signalements, rétablissement et suppression
- [X] T118 [US7] Brancher `MediaSignalerBouton` dans les 4 pages `uafricas_frontend/app/pages/medias/{chaines,stations,programmes-tele,programmes-radio}/[slug].vue` et dans `uafricas_frontend/app/components/media/SectionChaine.vue` et `SectionStation.vue`

**Checkpoint** : parcours 11 à 14 de `quickstart.md` § Lot 3 passent. **Lot 3 complet.**

---

## Phase 10: Polish & Cross-Cutting Concerns

- [X] T119 [P] Ajouter une ligne par lot dans « Recent Changes » de `CLAUDE.md`, citant les indices de migration — format imposé par la section Auto-maintenance
- [ ] T120 [P] Retirer la section « Active Technologies » ajoutée automatiquement en fin de `CLAUDE.md`, redondante avec « Tech Stack par feature », si le mainteneur le confirme
- [X] T121 [P] Corriger la dette relevée en Phase 0 : références à `media_content.programme_radio_tele` (table supprimée) dans `uafricas_backend/doc/bd/schemas/13_contraintes_inter_schemas.sql:227-233` et dans les commentaires de `src/models/television.rs` et `src/handlers/television.rs:245`
- [X] T122 [P] Remplacer les 2 statistiques codées en dur (`'24/7'`, `'HD+'`) de `uafricas_frontend/app/composables/useTelevision.ts:185-192` par `nombre_programmes` et `nombre_chaines_en_direct`, déjà renvoyés par l'API et jamais affichés
- [X] T123 [P] Unifier les définitions concurrentes de `RadioStation` (`uafricas_frontend/app/mocks/radios.ts:2` vs `app/composables/useStationsRadio.ts:31`) et de `TvChannel` / `TvProgram` (`app/mocks/tele.ts:2,14` vs `app/composables/useTelevision.ts:64,77`), les composables faisant foi
- [X] T124 Vérifier l'absence de classe daisyUI et de résidu `bg-gradient-to-*` dans `uafricas_frontend/app/pages/medias/` et `app/components/media/` par les greps de `quickstart.md` § Vérifications transverses
- [X] T125 Vérifier que chaque mutation de `uafricas_backend/src/handlers/media_social.rs`, `media_proposition.rs`, `media_detention.rs`, `media_programmation.rs` et `admin/media_proposition.rs` appelle `audit::log_action` avec `ancien_etat` et `nouvel_etat` renseignés — l'existant les passe à `None` (`admin/radio_tele.rs:1473`), ne pas reproduire ce défaut (FR-055)
- [ ] T126 Exécuter `getDiagnostics` (rust-analyzer, Volar) sur `uafricas_backend/src/handlers/media_*.rs`, `src/models/media_*.rs`, `uafricas_frontend/app/components/media/*.vue`, `app/composables/useMedia*.ts` et `app/pages/medias/**`, et corriger les avertissements
- [ ] T127 Dérouler l'intégralité des parcours de `quickstart.md` sur les trois lots, sur mobile et sur bureau
- [ ] T128 Déployer par `./deploy.sh update`, puis jouer les migrations par SSH et exécuter la reprise de données `origine_publication` documentée en fin de `quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance
- **Foundational (Phase 2)** : dépend du Setup — **bloque toutes les user stories**
- **US1 et US2 (Phases 3-4)** : dépendent de Foundational ; **indépendantes entre elles**
- **US3 (Phase 5)** : dépend de Foundational. Ses pages de détail s'ancrent dans les sections d'US1/US2, mais les endpoints et les tables sont autonomes
- **US4 (Phase 6)** : dépend de Foundational. T076 (validation) crée un `support_detenteur` — si US5 n'est pas encore livrée, insérer la ligne sans la table de co-détention n'est pas possible : livrer `09m` (T086) en amont, ou différer cette portion de T076
- **US5 (Phase 7)** : dépend de Foundational ; le flux complet suppose US4 pour l'origine des co-détenteurs, mais la phase est testable avec un co-détenteur créé par un administrateur
- **US6 (Phase 8)** : dépend d'US4 (`proposition_media`) et d'US5 (`support_detenteur`)
- **US7 (Phase 9)** : dépend des migrations `09j` (T009) et `09k` (T050) ; indépendante du reste d'US3
- **Polish (Phase 10)** : après les lots retenus

### Within Each User Story

Migration SQL → modèles Rust → handlers → routes → composables → composants → pages. Les tâches écrivant
dans un même fichier ne sont jamais marquées `[P]`.

### Parallel Opportunities

- Phase 2 : T013, T014, T015 (trois fichiers de modèles distincts) ; T016, T019, T020
- Phase 3 vs Phase 4 : **US1 et US2 en parallèle par deux développeurs**, seule `default.vue` (T043) étant partagée
- Phase 5 : T057 à T060 (quatre pages de détail) ; T061 à T064 (quatre composants)
- Phase 9 : T115, T116, T117

---

## Parallel Example: User Story 3

```bash
# Les quatre pages de détail, indépendantes entre elles :
Task: "Créer uafricas_frontend/app/pages/medias/programmes-tele/[slug].vue"
Task: "Créer uafricas_frontend/app/pages/medias/programmes-radio/[slug].vue"
Task: "Créer uafricas_frontend/app/pages/medias/chaines/[slug].vue"
Task: "Créer uafricas_frontend/app/pages/medias/stations/[slug].vue"

# Les quatre briques d'interaction, indépendantes entre elles :
Task: "Créer uafricas_frontend/app/composables/useMediaSocial.ts"
Task: "Créer uafricas_frontend/app/components/media/MediaReactionsBar.vue"
Task: "Créer uafricas_frontend/app/components/media/MediaCommentaires.vue"
Task: "Créer uafricas_frontend/app/components/media/MediaPartagerModal.vue"
```

---

## Implementation Strategy

### MVP (US1 seule)

1. Phase 1 : Setup — T001 à T004
2. Phase 2 : Foundational — T005 à T020 (**bloquant**)
3. Phase 3 : US1 — T021 à T033
4. **ARRÊT ET VALIDATION** : parcours 1 à 8 de `quickstart.md` § Lot 1
5. Démontrable : la page Télé est passée d'une grille à une vitrine éditorialisée

### Livraison incrémentale

| Étape | Contenu | Démontrable |
|---|---|---|
| 1 | Setup + Foundational | socle prêt |
| 2 | + US1 | **MVP** — page Télé remaniée |
| 3 | + US2 | **Lot 1 déployable** — les trois pages remaniées, distinction Radio effective |
| 4 | + US3, US4 | **Lot 2 déployable** — participation ouverte, faille de publication fermée |
| 5 | + US5, US6, US7 | **Lot 3** — programmation, engagement, modération |

Chaque étape apporte de la valeur sans casser la précédente.

### Ordre imposé par la sécurité

T077 (fermeture de la faille de publication directe) ne peut pas être livrée avant T072 à T076, sous peine
de rendre impossible toute création de chaîne. Inversement, elle **ne doit pas** être différée au-delà du
Lot 2 : tant qu'elle n'est pas appliquée, tout membre connecté publie sans validation, ce qui vide de son
sens l'ensemble d'US4.

### Ordre imposé par le bug latent

T030 (retrait du contenu vedette codé en dur) **expose** un bug jusque-là masqué : les URL YouTube injectées
dans une balise `<video>`. T017 (routage média) doit être terminée avant.

---

## Notes

- `[P]` = fichiers différents, aucune dépendance sur une tâche inachevée
- Un commit par tâche ou par groupe logique, message en français
- S'arrêter à chaque checkpoint pour valider la story de façon indépendante
- Toute migration nouvelle doit être **idempotente** et déclarée dans `schema.sql` : elles sont jouées à la
  main en production, sans runner ni table de versions
