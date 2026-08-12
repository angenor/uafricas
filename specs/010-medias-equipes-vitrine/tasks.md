---

description: "Liste de tâches — feature 010, équipes éditoriales et recentrage des vitrines Télé & Radio"
---

# Tasks: Médias — équipes éditoriales et recentrage des vitrines Télé & Radio

**Input**: Documents de conception dans `/specs/010-medias-equipes-vitrine/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: aucune tâche de test automatisé. Le projet n'a **ni linting, ni testing, ni CI/CD** (contrainte constitutionnelle assumée) et la spécification n'en réclame pas. La validation passe par les scénarios manuels de [quickstart.md](./quickstart.md), appelés explicitement en fin de chaque phase.

**Organization**: les tâches sont groupées par user story, pour que chaque histoire soit implémentable et recettable seule.

**Révision du 2026-08-10** — cette liste intègre les corrections issues de `/speckit-analyze` : plafond de programmes par section (FR-008, T023 et T029), retrait des mentions héritées « Animation / Production » (FR-034, T048/T049/T051), remontée du nettoyage d'équipes orphelines en Phase 2 (T019), audit explicité sur le chemin admin (T010), et quatre reformulations (T003, T004, T016, T058).

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers distincts, aucune dépendance sur une tâche inachevée)
- **[Story]** : rattachement à une user story de [spec.md](./spec.md) (US1 à US5)
- Chaque description porte le chemin exact du fichier

## Path Conventions

Monorepo web : `uafricas_backend/src/`, `uafricas_backend/doc/bd/schemas/`, `uafricas_frontend/app/`. Aucune arborescence neuve — voir « Structure Decision » dans [plan.md](./plan.md).

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: poser le schéma et le jeu de données sans lesquels rien n'est observable.

- [X] T001 Écrire la migration `uafricas_backend/doc/bd/schemas/09t_media_content_equipes_periodicite.sql` : table `media_content.membre_equipe`, ses 3 CHECK (`type_porteur` à 4 valeurs, `nom` et `fonction` non vides après `btrim`), ses 2 index partiels, ses commentaires de colonne, puis le remplacement des CHECK `ck_emission_{tele,radio}_cadence` par la liste à 4 valeurs. DDL intégrale en [data-model.md §1-§2](./data-model.md). Migration **idempotente** (`CREATE … IF NOT EXISTS`, `DROP CONSTRAINT IF EXISTS` puis `ADD`).
- [X] T002 Ajouter `\ir schemas/09t_media_content_equipes_periodicite.sql` à l'orchestrateur `uafricas_backend/doc/bd/schema.sql`, à la suite de `09s`.
- [X] T003 Relever le décompte des cadences **avant** application (`SELECT cadence, COUNT(*) …` sur `emission_tele` et `emission_radio`), appliquer la migration sur la base locale, puis rejouer le décompte et vérifier qu'aucune ligne n'a changé de valeur (FR-043). Contrôler aussi `\d media_content.membre_equipe` et la relecture des CHECK de cadence — section « Prérequis » de [quickstart.md](./quickstart.md).
- [X] T004 [P] Créer les cinq cas creux du [§1 du quickstart](./quickstart.md) : une chaîne sans programme, un programme sans épisode publié, une chaîne à description > 900 caractères, un programme à description > 400 caractères, et un programme **mensuel**. Les quatre premiers se posent en back-office ; le cinquième **par SQL direct** (`UPDATE … SET cadence = 'mensuelle'`), le sélecteur ne proposant cette valeur qu'après T056 en Phase 7. Sans ces cinq objets, la moitié des exigences (FR-005, FR-007, cas limites) est invérifiable.

**Checkpoint**: le schéma est en place et la base contient les cas qui font échouer une implémentation naïve.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: l'entité « membre d'équipe » de bout en bout — modèle, lecture groupée, écriture, routes, cycle de vie, types et composants génériques. Toutes les user stories en dépendent.

**⚠️ CRITICAL**: aucune user story ne peut démarrer avant la fin de cette phase.

### Backend — modèle et règles

- [X] T005 Créer `uafricas_backend/src/models/media_equipe.rs` : `TYPES_PORTEUR`, `MEMBRE_EQUIPE_COLONNES`, `MembreEquipeRow` (FromRow), `MembreEquipeResponse` (tous champs facultatifs en `skip_serializing_if`), `MembreEquipeRequest`, `EquipeRequest` + `valider()`, et les deux fonctions d'aiguillage `table_porteur` / `type_support_du_porteur`. Signatures en [data-model.md §3](./data-model.md). Déclarer le module dans `uafricas_backend/src/models/mod.rs`.
- [X] T006 Créer `uafricas_backend/src/handlers/media_equipe.rs` avec `equipes_par_porteurs(pool, type_porteur, &[Uuid]) -> HashMap<Uuid, Vec<MembreEquipeResponse>>` : **une seule requête** `WHERE porteur_id = ANY($1) AND deleted_at IS NULL ORDER BY ordre`, avec `LEFT JOIN iam.utilisateur u ON u.id = m.utilisateur_id AND u.deleted_at IS NULL` — c'est cette condition de jointure qui fait qu'un compte supprimé n'expose pas de lien mort (FR-014). Patron : `media_support::thematiques_par_supports` (`handlers/media_support.rs:37`). Déclarer le module dans `uafricas_backend/src/handlers/mod.rs`.
- [X] T007 Ajouter dans `uafricas_backend/src/handlers/media_equipe.rs` la fonction de règles partagée `appliquer_equipe(tx, type_porteur, porteur_id, membres, auteur)` : `DELETE` des membres du porteur puis `INSERT` de la liste, `ordre` = index reçu, `fonction` normalisée par `btrim` + `regexp_replace('\s+',' ','g')` avant insertion (FR-015). Modèle : `media_support::appliquer_thematiques` (`handlers/media_support.rs:217`).

### Backend — endpoints

- [X] T008 Implémenter les 3 handlers membres dans `uafricas_backend/src/handlers/media_equipe.rs` — `obtenir_equipe`, `definir_equipe`, `lister_fonctions`. Contrat : [contracts/api-membre.md](./contracts/api-membre.md). `definir_equipe` résout d'abord le support (via `media_emission::contexte_emission` quand le porteur est une émission) puis appelle `garde_detenteur(…, "co_detenteur")` — **jamais `AdminUtilisateur`**, ce sont des routes membres. Journaliser `equipe_modifiee` par `audit::log_action` **après le commit**, table `media_content.membre_equipe`, instantané avant/après en JSONB (Principe VII, FR-018).
- [X] T009 Implémenter la requête de suggestions de `lister_fonctions` avec le `DISTINCT ON (cle)` de [research.md D3](./research.md), et non un simple `SELECT DISTINCT` : « Directeur », « directeur » et « directeur  » doivent donner **une** entrée, pas trois (FR-015).
- [X] T010 Implémenter les 2 handlers admin `admin_obtenir_equipe` / `admin_definir_equipe` dans `uafricas_backend/src/handlers/media_equipe.rs`, gardés par `AdminUtilisateur` + `verifier_permission!(admin, "media", "voir"|"modifier")`, réutilisant `appliquer_equipe` sans aucune duplication de règles. **Journaliser `equipe_modifiee` sur ce chemin aussi** : le Principe VII est un MUST, et le `PUT` admin est une mutation au même titre que le `PUT` membre. Contrat : [contracts/api-admin.md](./contracts/api-admin.md).
- [X] T011 Déclarer les 5 routes dans `uafricas_backend/src/routes.rs` en respectant l'**ordre imposé** : `/medias/equipe/fonctions` parmi les segments littéraux du scope `/medias`, **avant** les motifs `"/{type_support}/{support_id}/…"` (`:1150+`) ; `/medias/{type_porteur}/{porteur_id}/equipe` (GET+PUT) dans ce même bloc ; côté admin, `/medias/{type_porteur}/{porteur_id}/equipe` (GET+PUT) à la suite du bloc `thematiques`/`couverture` (`:136`), donc avant `/medias/{id}` (`:147`). Un littéral déclaré trop tard produit un 404 « UUID parsing failed » — deux cas livrés en 009 pour cette raison.

### Backend — greffe dans les DTO existants et cycle de vie

- [X] T012 [P] Ajouter `pub equipe: Vec<MembreEquipeResponse>` avec `#[serde(skip_serializing_if = "Vec::is_empty")]` à `ChaineTvResponse` dans `uafricas_backend/src/models/television.rs:63`.
- [X] T013 [P] Ajouter le même champ à `StationRadioResponse` dans `uafricas_backend/src/models/station_radio.rs`.
- [X] T014 [P] Ajouter le même champ à `EmissionResponse` dans `uafricas_backend/src/models/media_emission.rs:188`, et le renseigner à `Vec::new()` dans `EmissionRow::to_response` (`:234`) — la greffe est faite après coup par les appelants, comme `episodes_apercu` et `interactions`.
- [X] T015 Implémenter la suppression douce de l'équipe aux **quatre** points de suppression d'un porteur (FR-019) : `media_emission::supprimer_emission` (`handlers/media_emission.rs:605`), `admin::radio_tele::supprimer_emission_admin` (`:1325`), `supprimer_chaine_tv` (`:936`) et `supprimer_station_radio` (`:524`). `porteur_id` n'a **pas de FK** : sans ce nettoyage explicite, les équipes orphelines survivent et polluent les suggestions de fonctions. **Traité en Phase 2 et non avec l'interface de saisie** : l'écriture d'équipes est ouverte dès T008, donc la fenêtre d'orphelins s'ouvre ici.

### Frontend — socle

- [X] T016 [P] Créer `uafricas_frontend/app/composables/useMediaEquipe.ts` : types `TypePorteurEquipe`, `MembreEquipeAPI`, `MembreEquipeForm` ([data-model.md §5](./data-model.md)), et les fonctions `obtenirEquipe`, `definirEquipe`, `listerFonctions`. Une prop `base: 'membre' | 'admin'` choisit le préfixe d'URL, à l'image du couple `useMediaEmissions`/`useAdminMediaEmissions`.
- [X] T017 [P] Créer `uafricas_frontend/app/components/common/TexteRepliable.vue` — Tailwind v4 pur, props `texte`, `lignes` (défaut 4), `sombre`, et **`repliable` (défaut `true`)**. Avec `repliable`, tronque puis propose « voir plus » / « voir moins » (FR-021) ; sans, tronque et s'arrête à l'ellipse — c'est le mode attendu en vitrine, où FR-003 demande des points de suspension et **non** une commande de dépliage. Dans les deux modes, un texte qui tient entièrement s'affiche sans bouton et sans ellipse (FR-022).
- [X] T018 [P] Créer `uafricas_frontend/app/components/common/ChampCombo.vue` — Tailwind v4 pur, `modelValue` texte libre + liste de suggestions filtrée à la frappe, **valeur hors liste acceptée** (FR-015). Aucun composant réutilisable de ce type n'existe : `arbre-genealogique/ChampRecherche.vue` vide le champ à la sélection, et le contournement actuel est un `<select>` + option « AUTRE » (`ProposerMediaModal.vue:412`).
- [X] T019 Créer `uafricas_frontend/app/components/media/EquipeMedia.vue` — rendu public d'une équipe (Tailwind pur) : une fiche par personne, **seuls les champs renseignés affichés** (FR-007), nom cliquable vers `/profil/{utilisateur_id}` si et seulement si `utilisateur_id` est présent (FR-014), et repli au-delà d'un seuil avec « voir plus » (FR-024) — ce repli est **interne au composant**, il plie des fiches et non du texte, `TexteRepliable` n'y intervient pas. Props : `membres`, `seuil` (0 = pas de repli, pour la vitrine), `sombre`. Dépend de T016.

**Checkpoint**: l'équipe se lit, s'écrit et se nettoie par API ; les composants de rendu et de saisie existent. `curl -X PUT /api/medias/chaine_tv/<id>/equipe` doit fonctionner **avant** toute écriture d'interface — c'est ce qui rend US1 recettable seule.

---

## Phase 3: User Story 1 — La vitrine annonce des chaînes et des programmes (Priority: P1) 🎯 MVP

**Goal**: chaque section de `/medias/tele` et des deux espaces Radio présente une chaîne (nom, extrait de description, équipe) puis ses programmes (couverture, nom, description tronquée), sans aucun média lisible et sans troncature silencieuse de la liste.

**Independent Test**: ouvrir les trois vitrines **déconnecté** et vérifier qu'aucune section ne contient de lecteur ni de vignette d'épisode, que chaque section nomme sa chaîne et liste ses programmes. Une équipe posée par `curl` (Phase 2) suffit à valider le bloc équipe sans attendre US2.

### Backend

- [X] T020 [US1] Dans `uafricas_backend/src/handlers/media_emission.rs:75-99`, remplacer `JOIN LATERAL (…) agg ON agg.nombre_episodes > 0` par `ON TRUE` dans `emissions_publiees_par_supports` : un programme sans épisode publié doit rester listé (FR-005).
- [X] T021 [US1] Dans `uafricas_backend/src/handlers/television.rs`, fonction `lister_sections` (`:360`) — supprimer la condition `EXISTS` sur les épisodes publiés du `WHERE` (`:376-381`), supprimer l'appel à `greffer_apercus_et_compteurs` (`:570`) tout en **conservant** `compteurs_pour("chaine_tv", …)` (`:573`), et greffer `equipes_par_porteurs("chaine_tv", …)` sur les chaînes. Conserver `diffusion_pour_support` : le bandeau de programmation reste en vitrine (Q3 → A).
- [X] T022 [US1] Dans `uafricas_backend/src/handlers/stations_radio.rs`, fonction `lister_sections_stations` (`:277`) — supprimer le filtre a posteriori `sections.retain(…)` (`:468`), qui désaccordait le `total` compté en SQL du nombre de sections servies ; supprimer la greffe des aperçus (`:474`) ; greffer `equipes_par_porteurs("station_radio", …)`.
- [X] T023 [US1] Porter le plafond de programmes par section de `unwrap_or(12).clamp(1, 30)` à **`unwrap_or(30).clamp(1, 60)`** dans `uafricas_backend/src/handlers/television.rs:369` **et** `uafricas_backend/src/handlers/stations_radio.rs` (même ligne, jumelle). Ce plafond bornait un aperçu d'épisodes ; il borne désormais le **contenu principal** de la section, et aucune page front ne le transmet — vérifié, zéro occurrence de `contenus_par_section` dans `app/pages/`. Justification : [research.md D5 bis](./research.md).

### Frontend

- [X] T024 [P] [US1] Ajouter `equipe: MembreEquipeAPI[]` à `TvChannel` et `TvEmission` dans `uafricas_frontend/app/composables/useTelevision.ts` (`:81-155`), avec repli `[]` dans `mapperEmissionVersTv` et le mappeur de chaîne — jamais `undefined`, pour que les gabarits n'aient pas à tester deux formes.
- [X] T025 [P] [US1] Faire de même dans `uafricas_frontend/app/composables/useStationsRadio.ts` pour la station et ses émissions.
- [X] T026 [P] [US1] Créer `uafricas_frontend/app/components/media/CarteProgramme.vue` — carte de programme pour la vitrine : image de couverture, nom, description tronquée par des points de suspension, lien vers `/medias/emissions-{tele,radio}/{slug}` (FR-004, FR-006). Tailwind v4 pur.
- [X] T027 [US1] Réécrire `uafricas_frontend/app/components/media/SectionChaine.vue` : retirer `MediaLecteurMedia`, `useObservateurVisibilite`, l'épisode mis en avant, les `MediaRangeeContenus`/`MediaCarteContenu` et la `MediaReactionsBar` sur épisode. Nouvel ordre — identité → `TexteRepliable` **en mode ellipse** (`:repliable="false"`) sur la description → `EquipeMedia` (`:seuil="0"`) → `MediaBandeauDiffusion` → grille de `CarteProgramme`. Conserver le bouton « Gérer ma chaîne » (`monRole`) et les boutons d'engagement. Dépend de T017, T019, T024, T026.
- [X] T028 [US1] Réécrire `uafricas_frontend/app/components/media/SectionStation.vue` selon le même plan (FR-060), en conservant « Écouter le direct » et `direct_disponible`. Dépend de T017, T019, T025, T026.
- [X] T029 [US1] Dans `SectionChaine.vue` et `SectionStation.vue`, afficher un lien « Voir les N programmes » vers la page du support dès que `emissions.length < totalEmissions` (FR-008). `total_emissions` est **déjà** servi par l'API — aucun champ neuf n'est requis. Sans ce garde-fou, une chaîne dépassant le plafond de T023 masquerait des programmes sans le dire.
- [X] T030 [US1] Traiter les états vides dans `SectionChaine.vue` et `SectionStation.vue` : une chaîne **sans programme** affiche identité et équipe puis signale l'absence de programmes ; une chaîne **sans équipe** n'affiche aucun cadre (FR-007, cas limites).
- [X] T031 [US1] Supprimer `uafricas_frontend/app/components/media/CarteEmission.vue` — code mort (zéro usage dans tout `app/`), remplacé par `CarteProgramme.vue`.
- [X] T032 [US1] Vérifier les trois pages porteuses — `uafricas_frontend/app/pages/medias/tele.vue`, `radio/africans.vue`, `radio/nationales.vue` : les compteurs et messages d'état vide restent justes maintenant que des chaînes sans épisode apparaissent.
- [X] T033 [US1] Recetter le [§2 du quickstart](./quickstart.md) sur les trois vitrines, y compris le contrôle `curl … /sections | grep -c episodes_apercu` ⇒ `0`, le cas d'une chaîne à 40 programmes (FR-008), et **noter le décompte de chaînes avant/après** : leur nombre augmente, c'est voulu ([research.md D5](./research.md)).

**Checkpoint**: US1 est fonctionnelle et recettable seule. SC-001, SC-002 et SC-008 sont mesurables.

---

## Phase 4: User Story 2 — Déclarer l'équipe d'une chaîne et d'un programme (Priority: P1)

**Goal**: un détenteur ou un administrateur saisit, ordonne et retire les personnes d'une équipe, avec une fonction proposée mais jamais imposée.

**Independent Test**: se connecter comme détenteur, ajouter trois personnes dont une avec une fonction inédite, enregistrer, recharger — les trois reviennent dans l'ordre, et la fonction inédite est proposée à la saisie suivante.

- [X] T034 [US2] Créer `uafricas_frontend/app/components/media/GestionEquipe.vue` — **Tailwind v4 pur** (il est monté côté membre, où daisyUI est proscrit : Principe VI ; les voisins `GestionEpisodes.vue`, `MesSupports.vue`, `GestionCoDetenteurs.vue` sont déjà dans ce cas, vérifié à zéro occurrence). Liste de lignes éditables (nom, prénom, `ChampCombo` sur la fonction, territoire, contact), ajout, retrait, réordonnancement, et enregistrement par un `PUT` de la liste complète. Props : `typePorteur`, `porteurId`, `base: 'membre' | 'admin'`. Dépend de T016, T018.
- [X] T035 [US2] Ajouter le rattachement facultatif à un compte dans `GestionEquipe.vue` : recherche d'un membre existant, sélection ⇒ `utilisateur_id`, et possibilité de détacher. L'enregistrement doit aboutir avec **zéro** rattachement (FR-013). Ne jamais pré-remplir `contact` depuis le compte — voir [research.md D2](./research.md).
- [X] T036 [US2] Monter la section « Équipe éditoriale » dans le panneau de support de `uafricas_frontend/app/components/media/MesSupports.vue`, et **renommer** la section existante « Équipe du support » (`:505`, qui gère en réalité les co-détenteurs, donc des droits) en « Gestion des accès » — sans quoi le détenteur trouve deux panneaux « équipe » sans rapport.
- [X] T037 [US2] Monter `GestionEquipe` sur la fiche d'un **programme** dans `MesSupports.vue`, porteur `emission_tele` ou `emission_radio` selon le support : les deux équipes doivent coexister sans recopie ni écrasement (FR-011).
- [X] T038 [P] [US2] Monter `GestionEquipe` (`base="admin"`, porteur `chaine_tv`) dans `uafricas_frontend/app/pages/admin/television/[id].vue`.
- [X] T039 [P] [US2] Monter `GestionEquipe` (`base="admin"`, porteur `station_radio`) dans `uafricas_frontend/app/pages/admin/radio/[id].vue`.
- [X] T040 [P] [US2] Monter `GestionEquipe` (`base="admin"`, porteur déduit de `type_support`) dans `uafricas_frontend/app/pages/admin/medias/emissions/[id].vue`.
- [X] T041 [US2] Recetter le [§3 du quickstart](./quickstart.md), en incluant les contrôles de droits (401 / 403 par `curl`), l'entrée d'audit `equipe_modifiee` **sur les deux chemins, membre et admin**, et la vérification que le `contact` servi est bien celui saisi et non l'e-mail du compte rattaché.

**Checkpoint**: US1 et US2 sont fonctionnelles ensemble — le MVP annoncé par la spec est livré.

---

## Phase 5: User Story 3 — Page de détail d'une chaîne / station (Priority: P2)

**Goal**: la page d'une chaîne déplie description et équipe à la demande, et liste ses programmes avec périodicité, description, équipe propre et vidéos — sans couverture de programme.

**Independent Test**: ouvrir `/medias/chaines/<slug>` et vérifier le pliage de la description, le pliage de l'équipe, la présence de la périodicité et de l'équipe pour chaque programme, la liste des vidéos, et l'absence de couverture de programme.

- [X] T042 [US3] Dans `uafricas_backend/src/handlers/television.rs`, fonction `obtenir_chaine_par_slug` (`:600`) — greffer l'équipe du support et **l'équipe de chacune de ses émissions** (deux appels à `equipes_par_porteurs`, un par discriminant, donc deux requêtes et aucun N+1). Conserver `episodes_apercu` : c'est la page qui liste les vidéos (FR-027).
- [X] T043 [US3] Faire de même dans `uafricas_backend/src/handlers/stations_radio.rs`, fonction `obtenir_station_par_slug` (`:209`).
- [X] T044 [US3] Remanier `uafricas_frontend/app/pages/medias/chaines/[slug].vue` : `TexteRepliable` **en mode repliable** sur la description (`:187-189`, aujourd'hui servie entière), `EquipeMedia` avec seuil de repli sous la description, et pour chaque programme — périodicité, nom, description, `EquipeMedia` du programme, liste des vidéos. **Retirer toute image de couverture de programme** (FR-026). Vérifier qu'aucun bloc ni libellé vide ne subsiste sur les cas creux de T004 (FR-007).
- [X] T045 [US3] Remanier `uafricas_frontend/app/pages/medias/stations/[slug].vue` selon le même plan (FR-060). Attention : cette page rend aujourd'hui ses programmes en `RangeeContenus` là où la page chaîne emploie une grille manuelle — les deux doivent converger sur le même rendu.
- [X] T046 [US3] Recetter le [§4 du quickstart](./quickstart.md), dont le cas « équipe de 11 personnes » (FR-024) et le cas « description courte, aucun bouton » (FR-022).

**Checkpoint**: SC-005 est mesurable.

---

## Phase 6: User Story 4 — Page de détail d'un programme (Priority: P2)

**Goal**: la page d'un programme montre périodicité, nom, image de couverture, description, équipe propre et vidéos — et une seule source d'information sur les personnes.

**Independent Test**: ouvrir `/medias/emissions-tele/<slug>` et vérifier les six blocs, dont l'image de couverture — absente de la page chaîne mais présente ici et sur la vitrine — et l'absence de la ligne héritée « Animation / Production ».

- [X] T047 [US4] Dans `uafricas_backend/src/handlers/media_emission.rs`, fonction `obtenir_emission_par_slug` (`:246`) — greffer l'équipe du programme, et **lever le 404 posé quand l'émission publiée n'a aucun épisode publié** (`:278`) : le 404 ne subsiste que si l'émission elle-même n'est pas publiée (FR-033).
- [X] T048 [US4] Remanier `uafricas_frontend/app/pages/medias/emissions-tele/[slug].vue` : conserver l'image de couverture (FR-031) ; afficher la périodicité **sans la masquer quand elle vaut « non périodique »** (le `v-if="cadence !== 'ponctuelle'"` de `:120` disparaît) ; `TexteRepliable` en mode repliable sur la description ; `EquipeMedia` du programme — jamais celle de la chaîne en repli (FR-032) ; **retirer la ligne héritée « Animation : … · Production : … » (`:128-135`)**, qui donnerait sinon une seconde source concurrente de l'équipe (FR-034) ; message explicite quand aucune vidéo n'est publiée (FR-033) ; aucun bloc ni libellé vide (FR-007).
- [X] T049 [US4] Remanier `uafricas_frontend/app/pages/medias/emissions-radio/[slug].vue` selon le même plan, retrait de la ligne héritée compris (FR-060, FR-034).
- [X] T050 [P] [US4] Réparer le fil d'Ariane des deux pages : `CommonFilAriane` y est monté (`:94`) mais **ce composant n'existe pas** — le remplacer par le `<nav>` écrit à la main qu'emploient les autres pages médias, ou par `CommonBreadcrumbNav`.
- [X] T051 [P] [US4] Dans `uafricas_frontend/app/pages/admin/medias/emissions/[id].vue` (`:272-277`), regrouper « Animateur » et « Producteur » sous un libellé explicite « Champs hérités — reporter dans l'équipe », en lecture. Les colonnes restent en base et servies par l'API : les masquer entièrement priverait le gestionnaire de la seule trace de ce qu'il doit recopier ([research.md D5 ter](./research.md)).
- [X] T052 [US4] Recetter le [§5 du quickstart](./quickstart.md), dont le contrôle `curl` prouvant que l'émission sans épisode renvoie `200` avec `nombre_episodes: 0` et non `404`.

**Checkpoint**: les quatre pages de détail publiques sont conformes, et l'information sur les personnes n'a plus qu'une source.

---

## Phase 7: User Story 5 — Périodicité enrichie et déclarée (Priority: P3)

**Goal**: quatre périodicités déclarables (non périodique, journalier, hebdomadaire, mensuel), avec des libellés identiques à la saisie et en public.

**Independent Test**: créer un programme mensuel, vérifier que « Mensuel » est proposé à la saisie et lisible sur les deux pages publiques.

- [X] T053 [US5] Dans `uafricas_backend/src/models/media_emission.rs:26-55` — porter `CADENCES_AUTORISEES` à `["quotidienne","hebdomadaire","mensuelle","ponctuelle"]`, ajouter `periode_heures_cadence` (24 / 168 / 720 / `None`), étendre `heures_anticipation_alerte` (6 / 48 / 168 / `None`), et mettre à jour le message d'erreur de `valider_cadence`, qui énumère aujourd'hui trois valeurs.
- [X] T054 [US5] Dans `uafricas_backend/src/handlers/media_programmation.rs:829`, remplacer le calcul en dur `if cadence == "quotidienne" { 24 } else { 24 * 7 }` par un appel à `periode_heures_cadence`. **Sans cette reprise, un programme mensuel serait signalé en retard dès le 8ᵉ jour** — c'est le piège de cette histoire.
- [X] T055 [P] [US5] Dans `uafricas_frontend/app/composables/useMediaEmissions.ts:109-113`, remplacer `LIBELLES_CADENCE` par les quatre libellés cibles (`ponctuelle` → « Non périodique », `quotidienne` → « Journalier », `hebdomadaire` → « Hebdomadaire », `mensuelle` → « Mensuel ») et exporter `CADENCES_ORDONNEES`, « non périodique » en tête (défaut, FR-042).
- [X] T056 [US5] Dans `uafricas_frontend/app/composables/useAdminMediaEmissions.ts`, supprimer `CADENCES` et `libelleCadence` propres au back-office et réexporter ceux de `useMediaEmissions` : FR-041 exige un libellé identique des deux côtés, ce que deux tables séparées ne garantissent pas dans la durée. Adapter le sélecteur de `uafricas_frontend/app/pages/admin/medias/emissions/[id].vue:221` et son pendant à la création pour qu'ils proposent exactement les quatre valeurs de `CADENCES_ORDONNEES`.
- [X] T057 [US5] Retirer les gardes `v-if="… cadence !== 'ponctuelle'"` sur les deux pages de support — `uafricas_frontend/app/pages/medias/chaines/[slug].vue:222` et `stations/[slug].vue:244` — pour que « Non périodique » s'affiche comme une information et non comme un blanc (FR-044, US5-3).
- [X] T058 [US5] Vérifier que la périodicité **n'est pas réintroduite** dans les sections de vitrine : les gardes qu'elles portaient (`SectionChaine.vue:274`, `SectionStation.vue:233`) ont disparu avec les `RangeeContenus` supprimées par T027/T028, et l'hypothèse de la spec exclut cet affichage en vitrine.
- [X] T059 [US5] Recetter le [§6 du quickstart](./quickstart.md), dont le contrôle chiffré de l'alerte de cadence mensuelle : aucune alerte à 10 jours, alerte `depassee` à 31 jours.

**Checkpoint**: toutes les user stories sont fonctionnelles.

---

## Phase 8: Polish & Cross-Cutting Concerns

- [X] T060 Exécuter le [§7 du quickstart](./quickstart.md) — parité Radio complète et non-régression : vedette plein écran intacte, signalement / partage / propositions toujours accessibles, adresses indexées qui résolvent (SC-009), grille et thématiques inchangées.
- [X] T061 Exécuter la requête d'équipes orphelines du [§7 du quickstart](./quickstart.md) et vérifier qu'elle renvoie `0` pour les quatre discriminants — c'est le seul contrôle du nettoyage de T015, l'absence de FK sur `porteur_id` interdisant à la base de le garantir.
- [X] T062 Parcourir l'ensemble des pages touchées avec `RUST_LOG=info cargo run` sous les yeux : **sqlx est vérifié au runtime**, une colonne oubliée compile sans broncher et n'échoue qu'à l'exécution. Aucune erreur SQL ne doit apparaître.
- [X] T063 [P] Ajouter la ligne de la feature 010 à l'index « Recent Changes » de `CLAUDE.md` — une ligne, citant la migration `09t` et les modules clés, conformément à la consigne d'auto-maintenance du fichier.
- [X] T064 Vérifier la conformité au Principe VI sur les cinq composants neufs (`TexteRepliable`, `ChampCombo`, `EquipeMedia`, `CarteProgramme`, `GestionEquipe`) : zéro classe daisyUI, tous étant montés sur des pages publiques ou membres.
- [ ] T065 Appliquer la migration `09t` en production via SSH + psql, comme toute migration du projet, et rejouer les contrôles de la section « Prérequis » du quickstart sur la base cible.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance.
- **Foundational (Phase 2)** : dépend de Phase 1 — **bloque toutes les user stories**.
- **US1 (Phase 3)** et **US2 (Phase 4)** : dépendent de Phase 2, indépendantes l'une de l'autre.
- **US3 (Phase 5)** et **US4 (Phase 6)** : dépendent de Phase 2, indépendantes entre elles et d'US1/US2.
- **US5 (Phase 7)** : dépend de Phase 1 (le CHECK SQL) et de Phase 2. T057 touche les mêmes fichiers que T044/T045 (US3) — les séquencer si US3 est livrée. T004 crée son cas de test en SQL faute de sélecteur avant T056.
- **Polish (Phase 8)** : dépend des histoires effectivement livrées. T065 clôt la livraison.

### User Story Dependencies

- **US1 (P1)** : autonome après Phase 2. L'API d'écriture livrée en Phase 2 permet de poser une équipe par `curl` et donc de recetter le bloc équipe **sans attendre US2**.
- **US2 (P1)** : autonome après Phase 2. Ne dépend pas d'US1 — un `PUT` réussi se vérifie par `GET`.
- **US3 (P2)**, **US4 (P2)** : autonomes après Phase 2.
- **US5 (P3)** : autonome. Une seule collision de fichiers, avec US3 (T057 ↔ T044/T045).

### Within Each User Story

Backend avant frontend (le gabarit ne peut pas afficher un champ que l'API ne sert pas), types avant composants, composants avant pages, recette en dernier.

### Parallel Opportunities

- **Phase 2** : T012, T013, T014 (trois fichiers de modèles distincts) ; T016, T017, T018 (trois fichiers frontend distincts). T005 → T006 → T007 → T008/T010 sont séquentiels, même fichier.
- **Phase 3** : T024, T025, T026 en parallèle ; T027 et T028 ensuite, en parallèle entre eux.
- **Phase 4** : T038, T039, T040 (trois pages admin distinctes) en parallèle, une fois T034 fait.
- **Phase 6** : T050 et T051 en parallèle de T048/T049.
- **Phase 7** : T055 en parallèle de T053/T054 (backend).
- Entre équipes : une fois la Phase 2 close, US1, US2, US3+US4 et US5 peuvent être menées par quatre personnes distinctes.

---

## Parallel Example: Phase 2 (Foundational)

```bash
# Les trois greffes de DTO, sur trois fichiers de modèles distincts :
Task: "T012 ChaineTvResponse.equipe dans uafricas_backend/src/models/television.rs"
Task: "T013 StationRadioResponse.equipe dans uafricas_backend/src/models/station_radio.rs"
Task: "T014 EmissionResponse.equipe dans uafricas_backend/src/models/media_emission.rs"

# Le socle frontend, sur trois fichiers neufs sans dépendance mutuelle :
Task: "T016 useMediaEquipe.ts"
Task: "T017 common/TexteRepliable.vue"
Task: "T018 common/ChampCombo.vue"
```

## Parallel Example: Phase 4 (US2)

```bash
# Une fois T034 (GestionEquipe.vue) livré, les trois montages back-office :
Task: "T038 GestionEquipe dans app/pages/admin/television/[id].vue"
Task: "T039 GestionEquipe dans app/pages/admin/radio/[id].vue"
Task: "T040 GestionEquipe dans app/pages/admin/medias/emissions/[id].vue"
```

---

## Implementation Strategy

### MVP (US1 + US2)

La spécification est explicite : « US1 et US2 sont les deux moitiés d'un même MVP ». La vitrine recentrée sans saisie d'équipe afficherait des blocs vides ; la saisie sans vitrine recentrée ne se verrait nulle part.

1. Phase 1 : Setup — migration et cas creux
2. Phase 2 : Foundational — **critique, bloque tout**
3. Phase 3 : US1 → recette du §2
4. Phase 4 : US2 → recette du §3
5. **STOP et VALIDER** : les trois vitrines et les deux surfaces de saisie
6. Démonstration possible

### Livraison incrémentale

Chaque phase suivante ajoute une valeur observable sans casser la précédente :

- \+ US3 → les pages de chaîne et de station déplient description et équipe
- \+ US4 → les pages de programme sont complètes, sans double source sur les personnes
- \+ US5 → la périodicité couvre les quatre cadences demandées
- \+ Polish → non-régression, audit d'orphelins, migration en production

### Quatre pièges à ne pas manquer

1. **T011 — ordre des routes.** Un segment littéral déclaré après un motif à paramètre produit un 404 « UUID parsing failed ». Deux cas ont été livrés en 009 pour cette raison exacte.
2. **T023/T029 — troncature silencieuse.** Le plafond de programmes par section bornait un aperçu ; il borne désormais le contenu principal. Le relever sans poser le lien « Voir les N programmes » ne ferait que déplacer le seuil auquel l'information disparaît sans le dire.
3. **T054 — période d'alerte.** Ajouter `mensuelle` sans reprendre `periode_heures_cadence` signalerait un programme mensuel en retard au bout d'une semaine. Le symptôme n'apparaît qu'après coup, jamais à la compilation.
4. **T015 — nettoyage sans FK.** `porteur_id` n'a pas de clé étrangère. Oublier l'un des quatre points de suppression ne casse rien de visible : les équipes orphelines restent simplement dans le référentiel de suggestions. T061 est le seul filet.

### Effet visible à annoncer avant la recette

Des chaînes et stations publiées mais dépourvues d'épisode **vont apparaître** sur les vitrines, où trois filtres les masquaient. C'est exigé par FR-005 et par le cas limite « chaîne sans programme », mais c'est un changement du **contenu servi**, pas seulement de sa présentation : le décompte affiché sur `/medias/tele` augmentera. Noter la valeur avant et après (T033).

---

## Notes

- `[P]` = fichiers distincts, aucune dépendance sur une tâche inachevée
- Le label `[Story]` assure la traçabilité vers [spec.md](./spec.md)
- Commiter par tâche ou par groupe logique, message en français (Principe I)
- S'arrêter à chaque checkpoint pour valider l'histoire seule
- Aucune tâche de test automatisé : le projet n'a pas de harnais, la validation est celle de [quickstart.md](./quickstart.md)
