---

description: "Liste de tâches : feature 013, activités interactives, ludiques et participatives"
---

# Tasks: Activités interactives, ludiques et participatives

**Input**: Documents de conception dans `/specs/013-activites-ludiques/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md)

**Tests**: aucune tâche de test automatisé. Le projet n'a **ni linting, ni testing, ni CI/CD** (contrainte constitutionnelle assumée) et la spécification n'en réclame pas. La validation passe par les scénarios manuels de [quickstart.md](./quickstart.md), appelés explicitement en fin de chaque phase. sqlx étant vérifié à l'exécution, **chaque requête écrite doit avoir tourné une fois** avant de cocher sa tâche.

**Avancement au 2026-10-04** : les 12 phases sont terminées (T001 à T123). Phase 12 : `CLAUDE.md` annonçait le backend sur le port 8082 alors qu'il écoute sur 8080 (valeur par défaut, et cible du frontend) ; corrigé. La description de PR (T123) est dans `pr-description.md` ; elle demande aussi de vérifier en production la présence des tables d'engagement de `35c`–`35g`, que `37_jeu.sql` lit. Historique des phases précédentes : phases 1 à 7 terminées (T001 à T080) : on saisit des épreuves, on en tire du contenu publié, on les passe en revue, on les joue, on les signale, on relève le défi du jour et celui de la semaine, le Championship classe les membres et les pays par saisons, on défie un ami en duel différé, la carte de l'Afrique situe chaque pays et permet de jouer sur lui, et les accomplissements du jeu se lisent en réputation et en distinctions, la triche se sanctionnant par annulation de gains ; deux amis peuvent enfin s'affronter en direct, sur la même épreuve au même instant. Les quatre modules pilotes sont jouables en local (327 candidates dérivées : 292 Afripulse, 21 Codimoi, 14 FactCheck). Recette déroulée par l'API et dans un navigateur piloté (scénarios 1 et 2 du quickstart). Trois ajustements par rapport au plan, tous répercutés dans les contrats : `POST …/repondre` accepte `cle: null` pour un temps écoulé (la correction s'affiche avant que l'horloge ne reparte) ; lancer une partie libre clôt la précédente (sinon on pouvait lire une question, abandonner, et la retrouver « neuve ») ; l'entrée de navigation est dans `navigation-africans.ts` (T019). En phase 5 : la bascule vers `a_revoir` se fait à toute lecture du vivier par un administrateur (liste des épreuves et décompte des modules), pas seulement au filtre `a_revoir` ; la forme `devise_nationale` écarte les valeurs qui sont de vraies URL ; les notifications du jeu mènent à `/activites` tant que `/mon-compte/activites` n'existe pas (à rebasculer en T077). En phase 6 : `GET /defis/courants` et `…/resultats` sont publics avec jeton facultatif (le hub se consulte sans connexion) ; `POST /defis/{id}/jouer` renvoie la partie existante au lieu d'un 409 quand le membre a déjà commencé (une seule participation, mais on doit pouvoir la reprendre) ; les routes `GET`/`PUT /admin/jeu/regles` de T116 sont livrées en avance, l'écran de programmation des défis en lit les tailles (reste de T116 : la page des règles). En phase 7 : un début de saison dans le passé est ramené à maintenant (les gains antérieurs ne portent pas la saison) ; le classement de tous les membres affiche le pays de rattachement ACTUEL du membre, les classements par pays celui du gain ; les notifications du jeu mènent désormais à `/mon-compte/activites`. Mesure SC-011 : 10 000 membres classés, les trois classements répondent en 10 à 40 ms en local. En phase 8 : à l'échéance d'un duel, une partie COMMENCÉE est close en l'état et compte avec ce qui a été répondu (seul celui qui n'a rien commencé perd par forfait) ; le mode direct est refusé en 400 (« arrive bientôt ») jusqu'à la phase 11 ; un duel annulé (amitié rompue, blocage, compte suspendu) n'émet qu'un signal SSE, pas de notification. En phase 9 : la carte est colorée par RANG du pays et non par score (un score brut dépend de l'affluence de la saison) ; la Bascule Classement / Carte vaut pour toutes les largeurs, la liste restant la vue par défaut ; le composant déplacé est `common/CarteAfriqueValeurs.vue` (Africonnect n'a changé que de nom de balise). Limite connue, héritée de la géométrie : la Gambie, enclavée et minuscule, est difficile à viser au clic. Avec 10 épreuves par partie, « jouer sur ce pays » n'est possible que pour les pays riches en contenu : une fiche pays ne produit que 5 épreuves dérivées. En phase 10 : l'annulation de gains a son propre écran `/admin/activites/joueurs` (recherche par nom ou e-mail) plutôt qu'une action greffée sur la page des saisons, et une route `GET /admin/jeu/joueurs` en plus du contrat ; T105 n'affiche les distinctions que dans « Mes activités », pas dans le bilan de partie (une distinction nouvelle y arrive déjà par la notification `engagement.badge_debloque`). En phase 11 : la première manche démarre 3 s après que les deux joueurs sont présents (le temps que le second écran relise), compte à rebours commun calé sur l'horloge du serveur ; accepter un duel direct vaut présence ; l'ordre des propositions d'une manche est STABLE d'une relecture à l'autre (graine par duel, manche et joueur), sinon il changerait toutes les 3 s ; en plus du signal et du sondage de 3 s, chaque écran relit pile aux instants charnières qu'il connaît (départ, fin de question, manche suivante) : mesuré, 1 ms d'écart d'apparition entre les deux écrans et 0 ms pour la correction. Le duel direct est résolu par le même `resoudre_duel` que le différé (branche `resoudre_direct`) ; la conclusion (vainqueur, primes) est factorisée en `conclure` / `terminer_duel`.

**Organization**: les tâches sont groupées par user story, dans l'ordre de livraison en sept paliers de [plan.md](./plan.md) : socle (US1, US2), vivier (US8), défis (US3), Championship (US5), duel différé (US4), carte et réputation (US6, US9), duel direct (US7).

**Décision reprise telle quelle** : le rattachement territorial suit la lecture littérale de la décision du client, `COALESCE(pays_origine_id, pays_residence_id)`. Le repli sur la résidence ne joue que si l'origine est **vide**, pas si elle est hors d'Afrique ([research.md D7](./research.md)).

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers distincts, aucune dépendance sur une tâche inachevée)
- **[Story]** : rattachement à une user story de [spec.md](./spec.md) (US1 à US9)
- Chaque description porte le chemin exact du fichier

## Path Conventions

Monorepo web : `uafricas_backend/src/`, `uafricas_backend/doc/bd/`, `uafricas_frontend/app/`. Arborescence complète dans « Project Structure » de [plan.md](./plan.md).

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: poser le schéma `jeu` et le jeu de données sans lesquels rien n'est observable.

- [X] T001 Écrire la migration `uafricas_backend/doc/bd/schemas/37_jeu.sql`, partie tables : `CREATE SCHEMA IF NOT EXISTS jeu`, puis les dix tables de [data-model.md §1 à §13](./data-model.md) (`module`, `epreuve`, `saison`, `defi`, `duel`, `partie`, `reponse`, `gain`, `score_saison`, `joueur`, `signalement_epreuve`, `regles`) dans un ordre qui respecte les clés étrangères, avec tous les CHECK, index et index uniques partiels nommés dans le modèle. États en `VARCHAR` + `CHECK`, aucun enum. Contrainte d'exclusion `ex_saison_chevauchement` en GiST sur `tstzrange`. Unicité `NULLS NOT DISTINCT` sur `score_saison`. Migration **idempotente** (`CREATE … IF NOT EXISTS`, `DROP CONSTRAINT IF EXISTS` puis `ADD`), en-tête de commentaire sur le modèle de `36_social_africanite.sql`.
- [X] T002 Compléter `uafricas_backend/doc/bd/schemas/37_jeu.sql` avec la vue `jeu.v_source` (`CREATE OR REPLACE VIEW`, six branches `UNION ALL`) : colonnes `type_source, source_id, visible, empreinte, pays_id`. Conditions de visibilité et champs de l'empreinte par branche en [data-model.md §3](./data-model.md). L'empreinte est un `md5` des seuls champs listés, **jamais** de `updated_at` ni des compteurs de J'aime ([research.md D5](./research.md)).
- [X] T003 Compléter `uafricas_backend/doc/bd/schemas/37_jeu.sql` avec les données de référence, toutes en `ON CONFLICT DO NOTHING` : 4 lignes `jeu.module` (`afrolang`, `codimoi`, `afripulse`, `factcheck`, avec leur route et une icône déjà enregistrée dans `fontawesome.ts`) ; le singleton `jeu.regles` ; la catégorie `jeux` dans `engagement.categorie_points` ; les 6 règles `jeu_*` à `points = 0` dans `engagement.regle_points` ; les 5 badges `jeu_*` en condition `actions_comptees` dans `engagement.badge` ; la permission `jeu.gerer` rattachée **explicitement** aux rôles `super_admin` et `admin`. Valeurs en [data-model.md §13 et §14](./data-model.md).
- [X] T004 Ajouter `\ir schemas/37_jeu.sql` à `uafricas_backend/doc/bd/schema.sql`, **en phase 5 après `35g_engagement_cadeaux.sql`** (la migration insère une permission et des règles d'engagement), et citer le schéma `jeu` dans le commentaire d'en-tête.
- [X] T005 Écrire le seed local `uafricas_backend/doc/bd/seeds-locaux/95_seed_local_jeu.sql`, idempotent et sans UUID en dur : amitié entre `test-admin@test.com` et `martialdjezou@gmail.com` (paire canonique, plus la `social.conversation` si `creer_amitie` en crée une) ; deux pays d'origine différents pour ces deux comptes ; une saison en cours ; 12 proverbes Codimoi **avec `pays_id`** et 8 citations avec `nom_auteur_originel` d'au moins 5 auteurs distincts ; 12 factchecks publiés à `verdict` `vrai` ou `faux` avec `prejuge_titre` et `realite_description` ; 12 épreuves Afrolang saisies et `jouable`. Hors de `schemas/`, donc jamais déployé ([research.md D14](./research.md)).
- [X] T006 Appliquer `37_jeu.sql` puis les seeds `99`, `98`, `95` sur la base locale (commandes de la section « Mise en place » de [quickstart.md](./quickstart.md)), contrôler `\dt jeu.*`, `SELECT type_source, count(*) FROM jeu.v_source GROUP BY 1`, puis **rejouer** la migration et le seed 95 une seconde fois : aucune erreur, aucun doublon.

**Checkpoint**: le schéma existe, la vue répond, la base locale contient de quoi jouer dans les quatre modules.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: le moteur partagé par tous les cadres de jeu (épreuve servable, série, réponse, gain), le garde du joueur, les types et l'ossature des routes et du frontend.

**⚠️ CRITICAL**: aucune user story ne peut démarrer avant la fin de cette phase.

### Backend

- [X] T007 Créer `uafricas_backend/src/models/jeu.rs` avec les types de base : `ModuleJeu`, `ReglesJeu` (FromRow sur le singleton), `EpreuveRow` (interne, complète), `EpreuveServie` et `PropositionServie { cle, texte }` (**sans** `bonne_reponse` ni `explication`), `Correction`, `PartieRow`, `PartieResponse`. `EpreuveServie` et `Correction` sont deux types distincts : la bonne réponse n'existe dans aucun type servi avant la réponse (FR-027, [data-model.md §15](./data-model.md)). Déclarer le module dans `uafricas_backend/src/models/mod.rs`.
- [X] T008 Créer `uafricas_backend/src/services/jeu.rs` avec : `charger_regles(pool)` ; la constante `SERVABLE_SQL`, **unique** fragment `FROM … JOIN jeu.module … LEFT JOIN jeu.v_source … WHERE …` de [data-model.md §2](./data-model.md) réutilisé par tout tirage ; `compter_servables(pool, module, pays_id)` ; `composer_serie(conn, utilisateur_id, module, pays_id, theme, taille, neuves_seulement) -> Vec<Uuid>` en une requête (anti-jointure sur `jeu.reponse` quand `neuves_seulement`, `ORDER BY random()`). Déclarer le module dans `uafricas_backend/src/services/mod.rs`.
- [X] T009 Ajouter dans `uafricas_backend/src/services/jeu.rs` la fonction `crediter(tx, utilisateur_id, montant, origine, reference_id, module_code, cle_idempotence) -> bool` : lit `COALESCE(pays_origine_id, pays_residence_id)` et la saison telle que `debut_at <= NOW() < fin_at`, fait `INSERT INTO jeu.gain … ON CONFLICT (cle_idempotence) DO NOTHING`, et **seulement si une ligne est insérée** upserte `jeu.score_saison` (si saison) et `jeu.joueur`. C'est le seul point d'écriture de ces trois tables ([research.md D2](./research.md), FR-024, FR-025, FR-053, FR-054).
- [X] T010 Ajouter dans `uafricas_backend/src/services/jeu.rs` le déroulé d'une partie, commun aux trois cadres : `presenter_suivante(tx, partie) -> Presentation` (enregistre `sans_reponse` sur l'épreuve courante laissée sans réponse, écrit `presentee_at`, mélange les propositions, clôt la partie s'il n'y a plus d'épreuve) et `enregistrer_reponse(tx, partie, rang, cle) -> Correction` (délai = `temps_epreuve_s` + 2 s de tolérance, + 5 s si média ; `INSERT … ON CONFLICT (partie_id, rang) DO NOTHING` et renvoi de la correction déjà enregistrée en cas de rejeu ; incrémente `nombre_servie` / `nombre_bonnes` ; appelle `crediter` avec la clé `reponse:{id}` **uniquement** en cadre `libre` ou `defi`). Règles en [research.md D8](./research.md) et [api-membre.md §3](./contracts/api-membre.md).
- [X] T011 Créer `uafricas_backend/src/handlers/jeu.rs` avec `garde_joueur(pool, req) -> Result<Uuid, ApiErreur>` (JWT par le patron `utilisateur_courant` du dépôt, puis lecture en base : `etat = 'actif' AND deleted_at IS NULL`, 403 sinon) et `joueur_optionnel(req) -> Option<Uuid>`, puis le handler public `lister_modules` (`GET /api/jeu/modules`, `disponible` calculé contre `taille_partie`). Déclarer le module dans `uafricas_backend/src/handlers/mod.rs` ([research.md D11](./research.md), FR-001, FR-002, FR-005).
- [X] T012 Déclarer dans `uafricas_backend/src/routes.rs` le `web::scope("/jeu")` avec `/modules`, ajouter les handlers au `use` de tête, et poser en commentaire la règle d'ordre du scope (littéraux avant motifs). Vérifier que le backend compile et que `GET /api/jeu/modules` répond avec les quatre modules.
- [X] T013 [P] Ajouter les six `type_action` `jeu_*` à `ACTIONS_INSTRUMENTEES` dans `uafricas_backend/src/handlers/admin/engagement.rs`, pour qu'ils ne s'affichent pas « non instrumentée » dans le barème ([research.md D3](./research.md)).
- [X] T014 [P] Ajouter un sous-module `pub mod jeu` à `uafricas_backend/src/models/notification.rs` avec les constantes de type et de lien de [temps-reel.md §4](./contracts/temps-reel.md).

### Frontend

- [X] T015 [P] Créer `uafricas_frontend/app/composables/useJeu.ts` : types `ModuleJeuAPI`, `EpreuveServieAPI`, `CorrectionAPI`, `PartieAPI`, `ReglesJeuAPI` ; `authHeaders()` lisant `useUserStore().accessToken` (**jamais** `localStorage`) ; un `appelAuth` local qui, sur 401, appelle une fois `useAuth().refreshAccessToken()` puis rejoue la requête ; `listerModules()`. Patron : `useEngagement.ts` ([research.md D11](./research.md)).
- [X] T016 [P] Enregistrer les icônes `gamepad` et `ranking-star` dans `uafricas_frontend/app/plugins/fontawesome.ts` (import **et** `library.add`, sinon elles s'affichent vides sans erreur).
- [X] T017 [P] Créer `uafricas_frontend/app/components/jeu/CarteModule.vue` (Tailwind pur, jetons `af-*`) : libellé, icône, nombre d'épreuves, état « bientôt disponible » quand `disponible` est faux, lien vers `/activites/{code}`.
- [X] T018 Créer la page `uafricas_frontend/app/pages/activites/index.vue` sur le gabarit `africans` (`definePageMeta({ layout: false })` + `<NuxtLayout name="africans">`, bandeau, fil d'Ariane) : pour l'instant la grille des modules ; accessible sans connexion. Les blocs défis, duels et classement s'y ajoutent dans leurs phases.
- [X] T019 [P] Ajouter l'entrée « Activités » vers `/activites` dans la navigation latérale, `uafricas_frontend/app/utils/navigation-africans.ts`. **Écart assumé** : la tâche visait `navigation-modules.ts`, mais ce fichier décrit des univers et leurs applications ; les activités sont transversales et n'appartiennent à aucun univers, d'où une entrée plate. L'ancienne barre du gabarit `default` ne la porte donc pas.

**Checkpoint**: `/activites` affiche les quatre modules et leur disponibilité ; le moteur de partie et de gain existe, sans route encore.

---

## Phase 3: User Story 1 - Jouer une partie dans un module (Priority: P1) 🎯 MVP

**Goal**: un membre lance une partie de dix épreuves dans un module, voit la correction après chaque réponse et un bilan à la fin.

**Independent Test**: [quickstart.md, scénario 2](./quickstart.md), sur les épreuves du seed.

- [X] T020 [US1] Implémenter dans `uafricas_backend/src/handlers/jeu.rs` `creer_partie` (`POST /api/jeu/parties`) : 409 si le module est fermé ou indisponible ; 409 « seule une partie d'entraînement est possible » si les épreuves neuves ne suffisent pas et que `entrainement` est faux ; cadre `libre` ou `entrainement` ; création paresseuse de `jeu.joueur`. Tableau des cas en [api-membre.md §3](./contracts/api-membre.md) (FR-014 à FR-016).
- [X] T021 [US1] Implémenter dans `uafricas_backend/src/handlers/jeu.rs` `partie_suivante`, `partie_repondre` et `partie_injouable` (`POST /api/jeu/parties/{id}/suivante|repondre|injouable`), minces au-dessus de `services::jeu`, sous `SELECT … FOR UPDATE` de la partie ; 404 si la partie n'est pas celle du membre. `injouable` : 409 si l'épreuve n'a pas de média, issue `injouable`, aucun compteur d'épreuve touché (FR-017, FR-018, FR-027 à FR-030).
- [X] T022 [US1] Implémenter dans `uafricas_backend/src/handlers/jeu.rs` `obtenir_partie` (`GET /api/jeu/parties/{id}`) : passe `close` une partie `en_cours` de plus de 24 h ; renvoie le bilan (bonnes, score gagné, total, rang de saison s'il existe, réponses) pour une partie terminée ou close (FR-019, FR-020).
- [X] T023 [US1] Ajouter dans `uafricas_backend/src/services/jeu.rs` la construction du `lien` de la correction à partir de `type_source` et `source_id` (page du proverbe, du factcheck, de la fiche pays, du site, de la recette, de la personnalité), en lisant les routes réelles des pages de détail existantes (FR-007, FR-018).
- [X] T024 [US1] Déclarer les 5 routes de parties dans `uafricas_backend/src/routes.rs` et exécuter chacune une fois par curl.
- [X] T025 [P] [US1] Étendre `uafricas_frontend/app/composables/useJeu.ts` : `creerPartie`, `obtenirPartie`, `suivante`, `repondre`, `declarerInjouable`, tous par `appelAuth`.
- [X] T026 [P] [US1] Créer `uafricas_frontend/app/components/jeu/Minuteur.vue` : compte à rebours calé sur l'horloge du serveur (`expire_a` et `maintenant` du payload, jamais `Date.now()` seul), émet `expire`.
- [X] T027 [P] [US1] Créer `uafricas_frontend/app/components/jeu/CarteEpreuve.vue` : énoncé, image ou lecteur audio réécoutable (FR-010), propositions en boutons, état verrouillé après réponse ; sur erreur de chargement du média, propose « Je ne peux pas l'écouter » qui émet `injouable`.
- [X] T028 [P] [US1] Créer `uafricas_frontend/app/components/jeu/Correction.vue` : bonne proposition, choix du membre, explication, lien vers le contenu d'origine, emplacement pour le bouton de signalement (branché en US8).
- [X] T029 [P] [US1] Créer `uafricas_frontend/app/components/jeu/BilanPartie.vue` : bonnes réponses, score gagné, total, rang, actions « Rejouer » et « Retour au module ».
- [X] T030 [US1] Créer la page de jeu `uafricas_frontend/app/pages/activites/partie/[id].vue` (`middleware: 'auth'`, gabarit `africans`, sans rail) : enchaîne `suivante` → `CarteEpreuve` + `Minuteur` → `repondre` → `Correction` → `suivante` ; à l'expiration du minuteur, appelle `suivante` ; au rechargement, reprend par `suivante` ; affiche `BilanPartie` quand `terminee`. Page commune aux cadres libre, défi et duel différé.
- [X] T031 [US1] Créer la page `uafricas_frontend/app/pages/activites/[module].vue` : présentation du module, bouton « Jouer » ; sur 409 d'entraînement, une modale `AfricansModale` **annonce l'entraînement avant de commencer** et relance avec `entrainement: true` (FR-016) ; visiteur renvoyé à la connexion par `useAuth().redirigerVersConnexion` (FR-001).
- [X] T032 [P] [US1] Créer `uafricas_frontend/app/components/jeu/PanneauActivites.vue` (prop `module`) : un `AfricansPanneau` « Activités » avec le nombre d'épreuves et un `AfricansBouton pleine-largeur` vers `/activites/{module}`.
- [X] T033 [US1] Monter `<JeuPanneauActivites>` en tête du slot `#rail` des quatre pages de module : `uafricas_frontend/app/pages/codi-moi/index.vue`, `afrolang/index.vue`, `opportunite-afrique/index.vue`, `universite/gouvernance/factcheck.vue` (FR-003).
- [X] T034 [US1] Dérouler le [scénario 2 du quickstart](./quickstart.md) en entier, dont : inspection du payload de `suivante` (aucune fuite), rejeu de `repondre` par curl (aucun second gain), épuisement du vivier, compte suspendu sans déconnexion (403 immédiat), et contrôle que `engagement.compte` n'a pas bougé (SC-004 à SC-006).

**Checkpoint**: un membre joue une partie complète dans chacun des quatre modules et gagne du score de jeu.

---

## Phase 4: User Story 2 - Constituer et tenir le vivier d'épreuves (Priority: P1)

**Goal**: un administrateur saisit, publie, corrige et retire des épreuves, et ouvre ou ferme un module.

**Independent Test**: [quickstart.md, scénario 1](./quickstart.md), étapes 1, 5 et 6, puis scénario 2 étape 9.

- [X] T035 [US2] Créer `uafricas_backend/src/models/admin/jeu.rs` : `EpreuveAdmin` (FromRow, avec `taux_reussite` et `source_etat`), `EpreuveRequest` + `valider_publication() -> Result<(), String>` qui **nomme le manque** (messages de [api-admin.md §2](./contracts/api-admin.md)), `FiltresEpreuves`, `ModuleAdmin`. Déclarer le module dans `uafricas_backend/src/models/admin/mod.rs`.
- [X] T036 [US2] Créer `uafricas_backend/src/handlers/admin/jeu.rs` avec `lister_epreuves` (pagination de `models/pagination.rs`, filtres `module`, `etat`, `origine`, `difficulte`, `pays`, `recherche`, `anomalie` ; `source_etat` calculé par jointure sur `jeu.v_source`), `creer_epreuve`, `obtenir_epreuve`, `modifier_epreuve`. Première ligne de chaque handler : `verifier_permission!(admin, "jeu", "gerer")` ; chaque mutation appelle `audit::log_action` sur le schéma `jeu` avec avant et après. Déclarer le module dans `uafricas_backend/src/handlers/admin/mod.rs` (FR-073, FR-074, FR-080, FR-085).
- [X] T037 [US2] Ajouter dans `uafricas_backend/src/handlers/admin/jeu.rs` `publier_epreuve` (`candidate` ou `a_revoir` → `jouable`, validation avant écriture, 400 nommant le manque) et `retirer_epreuve` (`jouable` → `retiree`, sans toucher aux réponses ni aux gains), audités (FR-009, FR-013, FR-074).
- [X] T038 [US2] Ajouter dans `uafricas_backend/src/handlers/admin/jeu.rs` `lister_modules_admin` (décompte par état) et `modifier_module` (`PATCH`, `ouvert`), audités (FR-081).
- [X] T039 [US2] Déclarer dans `uafricas_backend/src/routes.rs`, à plat dans le scope `/admin`, les routes `/jeu/modules`, `/jeu/modules/{code}`, `/jeu/epreuves`, `/jeu/epreuves/{id}`, `/jeu/epreuves/{id}/publier`, `/jeu/epreuves/{id}/retirer`. Laisser un commentaire réservant la place des littéraux `/jeu/epreuves/revue|derivation|formes` **avant** `/jeu/epreuves/{id}` (ajoutés en US8).
- [X] T040 [P] [US2] Créer `uafricas_frontend/app/composables/useAdminJeu.ts`, bâti sur `useAdmin()` : `listerPagine` pour les épreuves avec `filtres` réactifs, `creerEpreuve`, `obtenirEpreuve`, `modifierEpreuve`, `publierEpreuve`, `retirerEpreuve`, `listerModules`, `modifierModule`. Patron : `useAdminFactcheck.ts`.
- [X] T041 [P] [US2] Créer `uafricas_frontend/app/components/admin/jeu/EpreuveFormulaire.vue` (daisyUI) : module, énoncé, média (dépôt audio par `useAdminMediaUpload`, image par le champ d'upload d'image admin existant, à identifier dans `app/components/admin/`), de 2 à 6 propositions avec un bouton radio pour la bonne, explication, difficulté, thème, pays.
- [X] T042 [US2] Créer `uafricas_frontend/app/pages/admin/activites/epreuves/index.vue` (`layout: 'admin'`, `middleware: ['admin']`) : `AdminPageHeader`, `AdminFilters`, `AdminDataTable` avec colonnes énoncé, module, état, origine, servie, taux de réussite, état de la source ; filtre « anomalies » ; bloc d'en-tête listant les modules avec leur interrupteur d'ouverture.
- [X] T043 [US2] Créer `uafricas_frontend/app/pages/admin/activites/epreuves/[id].vue` (et le cas `nouvelle`) : `EpreuveFormulaire`, boutons Enregistrer, Publier, Retirer ; le refus de publication affiche le message du serveur tel quel.
- [X] T044 [US2] Ajouter la section « Activités » dans `uafricas_frontend/app/components/admin/AdminSidebar.vue` (enfant « Épreuves » ; les autres enfants s'ajoutent dans leurs phases) et le titre `activites` dans la table `titles` de `uafricas_frontend/app/layouts/admin.vue`.
- [X] T045 [US2] Dérouler le [scénario 1 du quickstart](./quickstart.md), étapes 1, 5 et 6, et le scénario 2 étape 9 ; contrôler une ligne `shared.audit_log` par mutation.

**Checkpoint**: MVP complet. On saisit des épreuves, on les joue, on les retire.

---

## Phase 5: User Story 8 - Tirer des épreuves du contenu déjà publié (Priority: P2)

**Goal**: l'administrateur fait proposer des candidates à partir du contenu publié et les passe en revue ; un membre signale une épreuve douteuse.

**Independent Test**: [quickstart.md, scénario 1](./quickstart.md) étapes 2 à 4, et scénario 3.

- [X] T046 [US8] Créer `uafricas_backend/src/services/jeu_derivation.rs` : struct `Forme { code, module, type_source, libelle }`, const `FORMES` (onze formes de [research.md D5](./research.md)), `tirer_distracteurs` (trois valeurs **distinctes** et différentes de la bonne, sinon aucune candidate), et l'insertion `INSERT … ON CONFLICT (type_source, source_id, forme) WHERE origine = 'derivee' DO NOTHING` enregistrant `source_empreinte` lue dans `jeu.v_source`. Déclarer le module dans `uafricas_backend/src/services/mod.rs`.
- [X] T047 [P] [US8] Implémenter dans `uafricas_backend/src/services/jeu_derivation.rs` les cinq formes sur `fiche_pays` (`capitale`, `pays_de_capitale`, `monnaie`, `drapeau` avec `media_type = 'image'`, `devise_nationale` lue dans `image_devise_url` qui contient du texte), bornées aux 55 pays par `PAYS_AFRICAINS_ISO2` et à `bloquee = FALSE`. Explication générée à partir des champs de la fiche ; `pays_id` de l'épreuve renseigné.
- [X] T048 [P] [US8] Implémenter dans `uafricas_backend/src/services/jeu_derivation.rs` les trois formes `pays_du_site`, `pays_de_recette`, `pays_de_personnalite` (sources `deleted_at IS NULL AND suspendu = FALSE`), puis `pays_du_proverbe` et `auteur_de_citation` sur `culture.codimoi` (`explication` de l'épreuve reprise de `codimoi.explication` quand elle existe), puis `vrai_ou_faux` sur `governance.factcheck` (deux propositions « Vrai » / « Faux », explication = `realite_description`, uniquement `verdict IN ('vrai','faux')`).
- [X] T049 [US8] Ajouter dans `uafricas_backend/src/handlers/admin/jeu.rs` `lister_formes` (par module : sources éligibles, candidates déjà produites ; liste vide pour Afrolang) et `deriver` (`POST /jeu/epreuves/derivation`, réponse `creees / deja_proposees / sans_distracteurs / par_forme`, **une** ligne d'audit `DERIVATION`) (FR-075).
- [X] T050 [US8] Ajouter dans `uafricas_backend/src/handlers/admin/jeu.rs` `revue` (`POST /jeu/epreuves/revue`, lot : `accepter` avec les validations de publication et réenregistrement de l'empreinte courante, `rejeter` avec motif obligatoire ; renvoie `acceptees / rejetees / refus[]`, une ligne d'audit pour le lot), et la **bascule paresseuse** dans `lister_epreuves` : avant de servir `etat = a_revoir`, passer en `a_revoir` les `jouable` dérivées dont l'empreinte ne correspond plus (FR-008, FR-012, FR-076).
- [X] T051 [US8] Déclarer dans `uafricas_backend/src/routes.rs` `/jeu/epreuves/formes`, `/jeu/epreuves/derivation`, `/jeu/epreuves/revue` **avant** `/jeu/epreuves/{id}`, et vérifier par curl qu'aucune ne tombe en 404 « UUID parsing failed ».
- [X] T052 [US8] Implémenter dans `uafricas_backend/src/handlers/jeu.rs` `signaler_epreuve` (`POST /api/jeu/epreuves/{id}/signaler`) : 403 sans réponse du membre sur l'épreuve, 409 si déjà signalée, classement d'office si l'épreuve est déjà retirée ; route déclarée dans `routes.rs` (FR-082).
- [X] T053 [US8] Ajouter dans `uafricas_backend/src/handlers/admin/jeu.rs` `lister_signalements` (regroupés par épreuve) et `decider_signalement` (`confirmer` : après le COMMIT, `engagement::attribuer(…, "jeu_signalement_confirme", …, "jeu:signalement:{id}")` et notification `jeu.signalement_traite` ; confirme d'office les autres signalements en attente de la même épreuve ; `retirer_epreuve` facultatif ; `classer` : notification seule). Aucun gain repris. Routes déclarées, mutations auditées (FR-083, FR-084).
- [X] T054 [P] [US8] Étendre `uafricas_frontend/app/composables/useAdminJeu.ts` (`listerFormes`, `deriver`, `revue`, `listerSignalements`, `deciderSignalement`) et `uafricas_frontend/app/composables/useJeu.ts` (`signalerEpreuve`).
- [X] T055 [US8] Créer `uafricas_frontend/app/pages/admin/activites/revue.vue` : bloc « Proposer des épreuves » par module et par forme avec le récapitulatif de dérivation (pour Afrolang, un texte dit qu'il n'y a pas de dérivation, pas de bouton) ; liste des `candidate` et `a_revoir` avec sélection multiple, « Accepter », « Rejeter » (motif obligatoire), lien « Corriger » vers `epreuves/[id]` ; affichage des refus du lot avec leur raison.
- [X] T056 [P] [US8] Créer `uafricas_frontend/app/pages/admin/activites/signalements.vue` : file par épreuve, nombre de signaleurs, motifs, décisions ; et ajouter « Revue » et « Signalements » à `uafricas_frontend/app/components/admin/AdminSidebar.vue`.
- [X] T057 [US8] Brancher le signalement dans `uafricas_frontend/app/components/jeu/Correction.vue` avec `AfricansModaleSignalement` (motifs de [data-model.md §12](./data-model.md)), visible quand `signalable` est vrai.
- [X] T058 [US8] Dérouler le [scénario 1](./quickstart.md) étapes 2 à 4 et le [scénario 3](./quickstart.md) en entier : rejeu de la dérivation sans doublon, rejetée jamais reproposée, fiche bloquée → épreuve non servie, J'aime sans effet, capitale modifiée → `a_revoir` (SC-003, FR-011, FR-012).

**Checkpoint**: le vivier se remplit par dérivation et se tient par la revue et les signalements.

---

## Phase 6: User Story 3 - Relever le défi du jour et le défi de la semaine (Priority: P2)

**Goal**: un défi identique pour tous chaque jour et chaque semaine, une participation par membre, une série de jours.

**Independent Test**: [quickstart.md, scénario 4](./quickstart.md).

- [X] T059 [US3] Ajouter dans `uafricas_backend/src/services/jeu.rs` `defi_courant(pool, periodicite) -> Option<Defi>` : période en UTC (`(NOW() AT TIME ZONE 'UTC')::date`, lundi par `date_trunc('week', …)`), composition automatique écartant les épreuves des défis des 30 derniers jours, `INSERT … ON CONFLICT (periodicite, periode_debut) DO NOTHING` puis `SELECT` : le premier lecteur crée, tous lisent la même ligne ; `None` si le vivier ne suffit pas ([research.md D6, D13](./research.md), FR-032, FR-033, FR-035).
- [X] T060 [US3] Ajouter dans `uafricas_backend/src/services/jeu.rs` la clôture d'une partie de défi, appelée par `presenter_suivante` quand la série est finie : prime (`crediter`, clé `defi:{defi_id}:{uid}`) ; mise à jour de `joueur.serie_jours` et `serie_dernier_jour` pour un défi du jour, datée de `defi.periode_debut` ; puis, **après le COMMIT**, `engagement::attribuer` pour `jeu_defi_termine` (clé `jeu:defi:{defi_id}:{uid}`) et, quand la série atteint un multiple de 7, `jeu_serie_7_jours` (clé `jeu:serie:{uid}:{jour}`). Ajouter au même endroit, pour tout cadre, `jeu_premiere_partie` (clé `jeu:premiere:{uid}`) (FR-036, FR-037, [data-model.md §8 et §14](./data-model.md)).
- [X] T061 [US3] Créer `uafricas_backend/src/handlers/jeu_defi.rs` : `defis_courants` (`GET /api/jeu/defis/courants`, avec `ma_partie` et la série **affichée**, ramenée à 0 si le dernier jour est antérieur à la veille), `jouer_defi` (`POST /api/jeu/defis/{id}/jouer` : 409 si le défi est passé ; sur violation de `uq_partie_defi`, 409 portant l'identifiant de la partie existante), `resultats_defi` (`GET /api/jeu/defis/{id}/resultats`, tri `bonnes DESC, temps_total_ms ASC`, rang du membre). Déclarer le module et les routes, `/defis/courants` **avant** `/defis/{id}/…` (FR-034, FR-038).
- [X] T062 [US3] Ajouter dans `uafricas_backend/src/handlers/admin/jeu.rs` `lister_defis`, `programmer_defi` (`PUT /jeu/defis/{periodicite}/{date}` : date à venir seulement, lundi pour `semaine`, exactement la taille réglée, toutes servables) et `deprogrammer_defi` (`DELETE`), audités ; routes déclarées (FR-079).
- [X] T063 [P] [US3] Étendre `uafricas_frontend/app/composables/useJeu.ts` (`defisCourants`, `jouerDefi`, `resultatsDefi`) et `uafricas_frontend/app/composables/useAdminJeu.ts` (`listerDefis`, `programmerDefi`, `deprogrammerDefi`).
- [X] T064 [P] [US3] Créer `uafricas_frontend/app/components/jeu/CarteDefi.vue` : titre, nombre d'épreuves, temps restant affiché en durée (« se termine dans 5 h 12 », jamais une heure d'horloge), états « Jouer », « Reprendre », « Voir mon résultat », et la série de jours.
- [X] T065 [US3] Ajouter le bloc des deux défis en tête de `uafricas_frontend/app/pages/activites/index.vue` ; « Jouer » appelle `jouerDefi` et mène à `/activites/partie/{id}` ; un 409 « déjà commencé » mène à la partie existante. Adapter `BilanPartie.vue` pour afficher la prime et le lien vers les résultats quand le cadre est `defi`.
- [X] T066 [P] [US3] Créer `uafricas_frontend/app/pages/activites/defis/[id].vue` : résultats d'un défi, rang du membre, podium.
- [X] T067 [P] [US3] Créer `uafricas_frontend/app/pages/admin/activites/defis.vue` : défis passés avec leur participation, calendrier des périodes à venir, programmation par sélection d'épreuves ; entrée « Défis » dans `AdminSidebar.vue`.
- [X] T068 [US3] Dérouler le [scénario 4 du quickstart](./quickstart.md) : même série pour deux comptes, refus du second essai, série de jours après changement de jour simulé, programmation refusée pour aujourd'hui, ligne « réputation +1 » à 0 point dans `/mon-compte/engagement` (SC-007).

**Checkpoint**: les membres ont une raison de revenir chaque jour.

---

## Phase 7: User Story 5 - Suivre le Championship panafricain (Priority: P2)

**Goal**: des saisons, trois classements, un pays de rattachement figé au gain, une clôture sans intervention.

**Independent Test**: [quickstart.md, scénario 6](./quickstart.md), étapes 1 à 3, 6 et 7.

- [X] T069 [US5] Ajouter dans `uafricas_backend/src/models/jeu.rs` `SaisonResponse` (état **dérivé** des dates), `LigneClassement`, `LignePays`, `MonJeu`, et dans `uafricas_backend/src/services/jeu.rs` `cloturer_saisons_echues(pool)` : `UPDATE jeu.saison SET cloturee_at = NOW() WHERE cloturee_at IS NULL AND fin_at <= NOW() RETURNING id`, puis pour chaque saison rendue, `engagement::attribuer(…, "jeu_podium_saison", …, "jeu:podium:{saison_id}:{uid}")` aux trois premiers de tous les membres et au premier de chaque pays africain ([research.md D6](./research.md), FR-062, FR-063).
- [X] T070 [US5] Créer `uafricas_backend/src/handlers/jeu_classement.rs` : `lister_saisons` (appelle `cloturer_saisons_echues`), `classement_membres` (`saison`, `pays`, pagination ; filtre `u.etat = 'actif' AND u.deleted_at IS NULL` ; tri `score DESC, atteint_at ASC` ; bloc `moi` avec rang et deux voisins de chaque côté si un jeton valide est présent, via `joueur_optionnel`), `classement_pays` (les 55 pays de `PAYS_AFRICAINS_ISO2`, **tous**, score = somme des `joueurs_par_pays` meilleurs par `ROW_NUMBER() OVER (PARTITION BY pays_id …)`, rang `null` à score 0). Ne sérialiser que nom, prénom, slug, photo, pays, `niveau_code`, score. Requêtes en [data-model.md §7](./data-model.md) (FR-056 à FR-061).
- [X] T071 [US5] Ajouter dans `uafricas_backend/src/handlers/jeu.rs` `mon_jeu` (`GET /api/jeu/moi` : total, score et rang de saison, pays de rattachement ou `null`, série, répartition par module et par origine) et `mes_parties` (`GET /api/jeu/moi/parties`, paginé). Compléter `obtenir_partie` avec `rang_saison` (FR-021, FR-026, FR-055).
- [X] T072 [US5] Déclarer dans `uafricas_backend/src/routes.rs` `/jeu/saisons`, `/jeu/classements/membres`, `/jeu/classements/pays`, `/jeu/moi`, `/jeu/moi/parties` ; déclarer le module `jeu_classement`.
- [X] T073 [US5] Ajouter dans `uafricas_backend/src/handlers/admin/jeu.rs` `lister_saisons_admin`, `creer_saison`, `modifier_saison`, `clore_saison` : traduire la violation de `ex_saison_chevauchement` en 409 lisible ; refuser en 400 la modification de `debut_at` d'une saison commencée et de `fin_at` d'une saison close ; audités ; routes déclarées (FR-051, FR-078).
- [X] T074 [P] [US5] Créer `uafricas_frontend/app/composables/useChampionship.ts` : `saisons`, `classementMembres`, `classementPays` (le jeton est joint s'il existe, pour obtenir `moi`), types `SaisonAPI`, `LigneClassementAPI`, `LignePaysAPI` ; étendre `useJeu.ts` avec `monJeu` et `mesParties`, et `useAdminJeu.ts` avec les quatre fonctions de saison.
- [X] T075 [P] [US5] Créer `uafricas_frontend/app/components/jeu/TableauClassement.vue` (Tailwind pur) : rang, `AfricansAvatar`, nom lié au profil, pays, `EngagementBadgeStatut`, score ; ligne du membre mise en évidence ; bloc « ma position » avec ses voisins quand il est hors de la page affichée.
- [X] T076 [US5] Créer `uafricas_frontend/app/pages/activites/championship.vue` : sélecteur de saison (courante et archives), `AfricansOnglets` « Tous les membres / Mon pays / Les pays », pagination, message « aucune saison en cours » hors saison, invitation à compléter son profil quand `pays_rattachement` est `null`. La bascule vers la carte s'ajoute en US6.
- [X] T077 [P] [US5] Créer `uafricas_frontend/app/components/jeu/ResumeJoueur.vue` et la page `uafricas_frontend/app/pages/mon-compte/activites.vue` (`middleware: 'auth'`, rail `ComptePanneauNavigation`) : score total et de saison, rang, série, répartition par module et par origine, historique paginé des parties ; ajouter « Mes activités » à `uafricas_frontend/app/utils/navigation-compte.ts`.
- [X] T078 [P] [US5] Créer `uafricas_frontend/app/pages/admin/activites/saisons.vue` : liste avec état et nombre de joueurs, création, modification, « Clore » ; entrée « Saisons » dans `AdminSidebar.vue`.
- [X] T079 [US5] Ajouter un aperçu du classement (cinq premiers et lien) dans `uafricas_frontend/app/pages/activites/index.vue`, et le rang dans `BilanPartie.vue`.
- [X] T080 [US5] Dérouler le [scénario 6 du quickstart](./quickstart.md) étapes 1 à 3, 6 et 7 ; puis charger 10 000 lignes dans `jeu.score_saison` par `generate_series` et mesurer les trois classements sous 2 s (SC-011, SC-012, SC-013).

**Checkpoint**: le score a un sens collectif ; une saison se clôt seule.

---

## Phase 8: User Story 4 - Défier un ami en duel différé (Priority: P2)

**Goal**: deux amis affrontent la même série, chacun quand il veut, dans les 48 heures.

**Independent Test**: [quickstart.md, scénario 5](./quickstart.md).

- [X] T081 [US4] Ajouter dans `uafricas_backend/src/models/jeu.rs` `DuelRow`, `DuelResponse` (le résultat de l'autre joueur n'est sérialisé que si le duel est terminé, FR-042) et les sept constructeurs `evt_duel_*` de [temps-reel.md §2](./contracts/temps-reel.md), sur le modèle de `models/appel.rs`, avec `MembreLight` pour le proposant.
- [X] T082 [US4] Ajouter dans `uafricas_backend/src/services/jeu.rs` `resoudre_duel(tx, duel_id) -> DuelRow`, sous `SELECT … FOR UPDATE`, branches du mode différé de [data-model.md §10](./data-model.md) : expiration d'une proposition ; annulation si l'amitié a disparu, s'il y a blocage ou si un compte n'est plus actif ; clôture quand les deux parties sont terminées (vainqueur : plus de `bonnes`, puis moins de `temps_total_ms`, sinon nul) ; forfait ou expiration à l'échéance. Chaque transition par `UPDATE … WHERE etat = <attendu>`. Si `compte` : primes par `crediter` (clé `duel:{duel_id}:{uid}`) ; renvoyer la liste des effets à produire **après le COMMIT** (`engagement::attribuer` `jeu_duel_gagne`, notifications, signaux) (FR-040, FR-043, FR-047 à FR-049).
- [X] T083 [US4] Créer `uafricas_backend/src/handlers/jeu_duel.rs` avec une copie locale de `verifier_relation` (ami et non bloqué, patron de `handlers/appels.rs:74`), `proposer_duel` (`POST /api/jeu/duels` : 403 hors amitié, 409 si le module n'a pas assez d'épreuves servables ou si un duel non terminé existe déjà entre les deux sur ce module, `sera_compte` calculé contre les deux plafonds du jour), `accepter_duel` (fige la série, tirée en priorité parmi les épreuves qu'**aucun** des deux n'a jouées ; fixe `compte` ; calcule l'échéance), `refuser_duel`, `annuler_duel`. Notification et signal SSE à chaque étape (`web::Data<RegistreSse>`). Toute action hors de l'état attendu : 409 (FR-039, FR-041, FR-048, FR-050).
- [X] T084 [US4] Ajouter dans `uafricas_backend/src/handlers/jeu_duel.rs` `lister_duels` (`GET /api/jeu/duels` : **résout chaque duel non terminé avant de le lister**, répartit en `a_repondre`, `a_jouer`, `en_attente`, `termines`, porte `quotas`), `obtenir_duel` (résolu), `jouer_duel` (`POST …/jouer` : crée ou renvoie la partie `cadre = 'duel'`, passe le duel `en_cours` au premier joueur).
- [X] T085 [US4] Compléter `uafricas_backend/src/services/jeu.rs` : à la fin d'une partie de cadre `duel`, appeler `resoudre_duel` dans la même transaction et, après le COMMIT, publier `duel_a_vous` (plus la notification `jeu.duel_a_vous`) si l'autre n'a pas encore joué, ou `duel_termine` aux deux.
- [X] T086 [US4] Déclarer dans `uafricas_backend/src/routes.rs` `/jeu/duels`, `/jeu/duels/{id}`, `/jeu/duels/{id}/accepter|refuser|annuler|jouer`, et le module `jeu_duel` ; exécuter chaque route une fois.
- [X] T087 [P] [US4] Créer `uafricas_frontend/app/composables/useDuels.ts` : `listerDuels`, `proposerDuel`, `obtenirDuel`, `accepter`, `refuser`, `annuler`, `jouer` ; état `useState('duel:invitation')` ; `gererEvenement(evt)` qui, pour l'instant, rafraîchit la liste et le compteur de notifications. Patron : `useAppels.ts`.
- [X] T088 [US4] Ajouter dans `uafricas_frontend/app/plugins/messagerie.client.ts` la branche `else if (evt.type.startsWith('duel_'))` → `useDuels().gererEvenement(evt)`, **avant** le `else` final (le dispatch n'a pas de cas par défaut).
- [X] T089 [P] [US4] Ajouter les sept types `jeu.*` aux quatre endroits prévus : union `TypeNotification`, `iconeNotification`, `couleurNotification` dans `uafricas_frontend/app/mocks/notifications.ts`, et table `TYPES` de `uafricas_frontend/app/pages/notifications.vue`.
- [X] T090 [P] [US4] Créer `uafricas_frontend/app/components/jeu/ProposerDuelModal.vue` (`AfricansModale`) : choix d'un ami (liste de `useAmis`), du module, du mode (le mode « direct » reste désactivé jusqu'à US7) ; affiche avant l'envoi que le duel sera **amical** quand `sera_compte` est faux.
- [X] T091 [P] [US4] Créer `uafricas_frontend/app/components/jeu/ListeDuels.vue` : quatre groupes, adversaire, module, échéance en durée, actions selon l'état.
- [X] T092 [US4] Créer `uafricas_frontend/app/pages/activites/duels/index.vue` (`middleware: 'auth'`) et `uafricas_frontend/app/pages/activites/duels/[id].vue` : état du duel, boutons Accepter, Refuser, Annuler, Jouer (vers `/activites/partie/{id}`), résultat comparé une fois terminé ; mention « duel amical, sans gain » quand `compte` est faux. Adapter `BilanPartie.vue` au cadre `duel` (pas de score, « en attente de l'adversaire » ou lien vers le duel).
- [X] T093 [US4] Ajouter « Mes duels » (duels à répondre et à jouer) et le bouton « Défier un ami » dans `uafricas_frontend/app/pages/activites/index.vue` et `uafricas_frontend/app/pages/activites/[module].vue`.
- [X] T094 [US4] Dérouler le [scénario 5 du quickstart](./quickstart.md) avec deux navigateurs : résultat masqué tant que l'autre n'a pas joué, forfait après échéance avancée, expiration, quatrième duel du jour annoncé amical, annulation à la rupture d'amitié, refus hors amitié (SC-009).

**Checkpoint**: le jeu devient social, sans exiger que deux membres soient connectés ensemble.

---

## Phase 9: User Story 6 - Explorer la carte interactive de l'Afrique (Priority: P3)

**Goal**: la carte teinte chaque pays selon son rang et sert d'entrée par pays.

**Independent Test**: [quickstart.md, scénario 6](./quickstart.md), étapes 4, 5 et 8.

- [X] T095 [US6] Déplacer `uafricas_frontend/app/components/retrouve-amis/CarteAfrique.vue` vers `uafricas_frontend/app/components/common/CarteAfriqueValeurs.vue` et lui ajouter trois props facultatives dont les défauts reproduisent le comportement actuel : `couleur?: (valeur: number) => string` (défaut `couleurChaleurAvis`), `libelleBulle?: (valeur: number) => string`, `cliquableAZero?: boolean` (défaut `false`). Mettre à jour l'unique point d'appel dans `uafricas_frontend/app/pages/retrouve-amis/index.vue` ([research.md D10](./research.md)).
- [X] T096 [US6] Ajouter dans `uafricas_backend/src/handlers/jeu_classement.rs` `fiche_pays_jeu` (`GET /api/jeu/pays/{pays_id}` : rang, score, joueurs, cinq meilleurs, et par module le nombre d'épreuves servables portant sur ce pays avec `disponible`) ; route déclarée (FR-065).
- [X] T097 [P] [US6] Ajouter une échelle de teintes par rang dans `uafricas_frontend/app/utils/carteAfrique.ts` (`PALIERS_RANG`, `couleurRangPays`, gris pour un pays sans score), sans toucher à `PALIERS_CHALEUR` ; étendre `useChampionship.ts` avec `fichePays`.
- [X] T098 [P] [US6] Créer `uafricas_frontend/app/components/jeu/FichePaysClassement.vue` : rang, score, meilleurs joueurs, boutons « Jouer sur ce pays » par module disponible ; pour un pays sans score, le message « aucun joueur encore, soyez le premier » (FR-067).
- [X] T099 [US6] Ajouter dans `uafricas_frontend/app/pages/activites/championship.vue` une `AfricansBascule` « Classement / Carte » : la carte (`CommonCarteAfriqueValeurs` avec `cliquable-a-zero`, valeurs = scores des pays, légende dans le rail) ; la sélection d'un pays ouvre `FichePaysClassement` dans le rail avec `scrollIntoView` sous 1280 px ; **vue liste par défaut sous 1024 px** (FR-064, FR-068). « Jouer sur ce pays » appelle `creerPartie` avec `pays_id` (FR-066).
- [X] T100 [US6] Dérouler le [scénario 6 du quickstart](./quickstart.md) étapes 4, 5 et 8, dont la **non-régression de la carte d'Africonnect** sur `/retrouve-amis`.

**Checkpoint**: le Championship se lit d'un coup d'œil.

---

## Phase 10: User Story 9 - Gagner en réputation et en distinctions (Priority: P3)

**Goal**: les accomplissements du jeu se voient sur le profil ; la triche se sanctionne.

**Independent Test**: [quickstart.md, scénario 8](./quickstart.md).

- [X] T101 [US9] Vérifier, mouvement par mouvement, que les six appels à `engagement::attribuer` posés en T053, T060, T069 et T082 écrivent bien une ligne à 0 point avec le bon `reputation_delta` et la bonne catégorie, et que `engagement.compte.solde_points` et `niveau_code` ne bougent pas ; corriger les clés ou les règles semées dans `uafricas_backend/doc/bd/schemas/37_jeu.sql` si un écart apparaît (FR-069, FR-070, SC-006).
- [X] T102 [US9] Ajouter dans `uafricas_backend/src/handlers/admin/jeu.rs` `annuler_gains` (`POST /jeu/joueurs/{utilisateur_id}/annuler-gains`) : dans une transaction, marquer les gains (`annule_at`, `annule_par`, `motif_annulation`) puis **recalculer** `score_saison` et `joueur.score_total` du membre depuis les gains restants ; après le COMMIT, `engagement::ajuster(pool, uid, 0, -retrait)` si demandé, notification `jeu.gains_annules`, audit `GAINS_ANNULES`. Motif obligatoire ; route déclarée (FR-031, FR-072).
- [X] T103 [P] [US9] Ajouter à la page d'un classement administrateur ou à `uafricas_frontend/app/pages/admin/activites/saisons.vue` une action « Annuler les gains » par joueur (modale : motif obligatoire, date de départ, retrait de réputation), et `annulerGains` dans `useAdminJeu.ts`.
- [X] T104 [P] [US9] Ajouter dans `uafricas_frontend/app/pages/admin/engagement/regles.vue` une mention sur les règles `jeu_*` : « à 0 point par construction ; la désactiver fige aussi le badge associé » ([research.md D3](./research.md)).
- [X] T105 [P] [US9] Afficher dans `uafricas_frontend/app/components/jeu/BilanPartie.vue` et `uafricas_frontend/app/pages/mon-compte/activites.vue` les distinctions du jeu obtenues (`useEngagement().obtenirBadgesPublics` filtré sur les codes `jeu_*`), avec un lien vers `/mon-compte/engagement`.
- [X] T106 [US9] Dérouler le [scénario 8 du quickstart](./quickstart.md) : badges obtenus une seule fois, règles `jeu_*` « instrumentées » dans le barème, signalement confirmé, annulation de gains avec égalité journal / agrégat.

**Checkpoint**: le jeu laisse une trace durable sur le profil, sans toucher aux points d'engagement.

---

## Phase 11: User Story 7 - Affronter un ami en duel direct (Priority: P3)

**Goal**: deux amis jouent la même épreuve au même instant.

**Independent Test**: [quickstart.md, scénario 7](./quickstart.md), deux navigateurs.

- [X] T107 [US7] Compléter `resoudre_duel` dans `uafricas_backend/src/services/jeu.rs` avec les branches du mode direct de [data-model.md §10](./data-model.md) : passage `accepte` → `en_cours` quand les deux présences sont fraîches ; boucle « tant que la manche courante est close (deux réponses, ou `manche_debut_at + temps` dépassé), enregistrer `sans_reponse` pour qui n'a pas répondu, puis ouvrir la suivante après `pause_revelation_s` ou terminer » ; forfait d'un joueur absent au-delà de `grace_direct_s` ; fin sans issue si les deux le sont ; expiration si les deux ne sont pas présents sous `delai_direct_min` ([research.md D1, D6](./research.md), FR-044, FR-045).
- [X] T108 [US7] Ajouter dans `uafricas_backend/src/models/jeu.rs` `EtatDuelDirect` (phases `attente`, `question`, `revelation`, `termine`) et dans `uafricas_backend/src/handlers/jeu_duel.rs` `etat_direct` (`GET /api/jeu/duels/{id}/direct`) : écrit le battement de présence du demandeur, appelle `resoudre_duel`, sert l'épreuve **sans correction** en phase `question`, `adversaire_a_repondu` en simple booléen, la correction et les deux réponses en phase `revelation`, `maintenant` pour caler l'horloge. Crée les deux parties `cadre = 'duel'` au démarrage.
- [X] T109 [US7] Ajouter dans `uafricas_backend/src/handlers/jeu_duel.rs` `repondre_direct` (`POST /api/jeu/duels/{id}/direct/repondre`) : mêmes règles de délai et d'idempotence que `enregistrer_reponse`, **ne renvoie pas la correction**, publie `duel_manche` aux deux ; et `convertir_duel` (`POST …/convertir` : le proposant d'un duel direct `expire` ouvre un duel différé). Routes déclarées (FR-046).
- [X] T110 [US7] Adapter `proposer_duel` et `accepter_duel` dans `uafricas_backend/src/handlers/jeu_duel.rs` au mode `direct` : échéance `delai_direct_min`, pas de notification `duel_a_vous`, signal `duel_propose` portant `expire_a`.
- [X] T111 [P] [US7] Étendre `uafricas_frontend/app/composables/useDuels.ts` : `etatDirect`, `repondreDirect`, `convertir` ; `gererEvenement` pose `invitation` sur un `duel_propose` de mode `direct` (sauf si un duel direct est déjà en cours) et la retire sur `duel_annule` / `duel_termine` ; fonction `suivreDuelDirect(id)` qui relit l'état à chaque signal `duel_*` de ce duel **et** toutes les 3 secondes, et s'arrête à la phase `termine` ou au démontage.
- [X] T112 [P] [US7] Créer `uafricas_frontend/app/components/jeu/InvitationDuelPrompt.vue` (Tailwind pur, patron visuel de `SocialAppelEntrantPrompt.vue`, sans sonnerie) : proposant, module, temps restant, Accepter (mène à `/activites/duels/{id}`), Refuser ; le monter dans `uafricas_frontend/app/components/social/MessagerieFlottante.vue` à côté de `SocialAppelEntrantPrompt`.
- [X] T113 [US7] Créer `uafricas_frontend/app/components/jeu/DuelDirectSalle.vue` : phases `attente` (présence de l'adversaire), `question` (`CarteEpreuve` + `Minuteur` calé sur `manche_fin_at` et `maintenant`, « l'adversaire a répondu », « en attente de l'adversaire » après sa propre réponse), `revelation` (`Correction` avec les deux réponses, scores), `termine` (issue, gain) ; bandeau « adversaire déconnecté » quand `adversaire_present` est faux.
- [X] T114 [US7] Brancher le mode direct dans `uafricas_frontend/app/pages/activites/duels/[id].vue` (`DuelDirectSalle` + `suivreDuelDirect`, bouton « Transformer en duel différé » sur un duel direct expiré) et activer le choix « direct » dans `ProposerDuelModal.vue`.
- [X] T115 [US7] Dérouler le [scénario 7 du quickstart](./quickstart.md) : écart d'affichage sous une seconde, aucune correction avant la révélation dans les payloads, coupure réseau de 15 s, forfait après le délai de grâce, **flux SSE bloqué** (le duel se joue quand même), expiration et conversion (SC-010).

**Checkpoint**: toutes les histoires de la spec sont livrées.

---

## Phase 12: Polish & Cross-Cutting Concerns

- [X] T116 [P] Créer `uafricas_frontend/app/pages/admin/activites/regles.vue` et les handlers `obtenir_regles` / `modifier_regles` dans `uafricas_backend/src/handlers/admin/jeu.rs` (`GET`/`PUT /jeu/regles`, bornes des CHECK rappelées en 400 lisible, audit `REGLES_MODIFIEES` avec avant et après), routes déclarées, entrée « Règles » dans `AdminSidebar.vue`, lien vers `/admin/engagement/regles` pour les montants de réputation (FR-077).
- [X] T117 [P] Créer une modale de découverte « C'est quoi les Activités ? » avec `AfricansModaleDecouverte` dans `uafricas_frontend/app/components/jeu/DecouverteModale.vue` et la brancher sur l'aide du bandeau de `uafricas_frontend/app/pages/activites/index.vue`.
- [X] T118 Relire les quatre chemins de tirage (partie libre, défi, duel, carte par pays) et vérifier qu'ils passent **tous** par `SERVABLE_SQL` de `uafricas_backend/src/services/jeu.rs` ; relire toutes les routes qui lisent un duel et vérifier qu'elles appellent `resoudre_duel` (points 1 et 2 de « Points de conception » dans [plan.md](./plan.md)).
- [X] T119 Parcourir toutes les pages `/activites/**` à 375 px, 768 px et 1280 px : aucune colonne à 0 px, aucun débordement horizontal, rail empilé sous le contenu ; vérifier qu'aucune classe daisyUI n'apparaît hors de `/admin/**` (principe VI).
- [X] T120 Lancer `getDiagnostics` sur les fichiers Rust et Vue touchés, corriger avertissements et erreurs de types ; vérifier que `cargo build` et `pnpm build` passent.
- [X] T121 Contrôler les critères de sortie de [quickstart.md](./quickstart.md) : aucune clé d'idempotence en double dans `jeu.gain`, agrégats égaux au journal pour tout membre, une ligne d'audit par mutation d'administration, migration rejouée sans erreur, non-régression de la messagerie et des appels entre amis.
- [X] T122 Mettre à jour `CLAUDE.md` : schéma `jeu` dans la liste des schémas, `useJeu` / `useDuels` / `useChampionship` / `useAdminJeu`, seed `95_seed_local_jeu.sql` dans le tableau des seeds locaux, port réel du backend en développement, et **une ligne** d'index dans « Recent Changes ».
- [X] T123 Rédiger dans la description de la PR la marche d'ouverture en production de [quickstart.md](./quickstart.md) (« Avant l'ouverture en production ») : migration **avant** le déploiement du code, mesure du contenu réel par `deploy.sh psql`, dérivation et revue, saisie Afrolang, première saison.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance.
- **Foundational (Phase 2)** : dépend de la Phase 1. **Bloque toutes les histoires.**
- **US1 (Phase 3)** et **US2 (Phase 4)** : dépendent de la Phase 2 seulement. Indépendantes l'une de l'autre grâce aux épreuves du seed.
- **US8 (Phase 5)** : dépend de US2 (liste des épreuves, publication, routes admin) et de US1 pour le signalement dans l'écran de correction (T052, T057).
- **US3 (Phase 6)** : dépend de US1 (déroulé de partie, page de jeu).
- **US5 (Phase 7)** : dépend de US1 (il faut des gains). Indépendante de US3 et US8.
- **US4 (Phase 8)** : dépend de US1. Indépendante de US3 et US5 ; le rang affiché dans le bilan profite de US5 s'il est là.
- **US6 (Phase 9)** : dépend de US5 (classement des pays).
- **US9 (Phase 10)** : dépend de US3, US4, US5 et US8, qui posent chacune un appel à `engagement::attribuer`. T102 (annulation de gains) ne dépend que de US5.
- **US7 (Phase 11)** : dépend de US4 (cycle du duel, signaux, composable).
- **Polish (Phase 12)** : après les histoires retenues. T116 (écran des règles) peut être fait dès la Phase 4.

### User Story Dependencies

```text
Phase 1 → Phase 2 ─┬─> US1 ─┬─> US3 ─────────────┐
                   │        ├─> US5 ──> US6      ├─> US9
                   │        └─> US4 ──> US7 ─────┘
                   └─> US2 ──> US8 ───────────────┘
```

### Within Each User Story

- Modèles avant services, services avant handlers, handlers avant routes, routes avant composable, composable avant composants et pages.
- Chaque route déclarée est exécutée une fois (sqlx est vérifié à l'exécution).
- La dernière tâche de chaque phase déroule le scénario du quickstart : ne pas passer à la phase suivante avant.

### Parallel Opportunities

- **Phase 1** : T001 à T003 touchent le même fichier, donc en séquence ; T005 peut s'écrire en parallèle de T001.
- **Phase 2** : T013, T014, T015, T016, T017, T019 en parallèle une fois T007 posé.
- **Entre histoires** : après la Phase 2, US1 et US2 peuvent avancer de front. Après US1, US3, US5 et US4 peuvent avancer de front côté frontend ; côté backend elles touchent toutes `services/jeu.rs` et `routes.rs`, donc à coordonner.
- **Dans une histoire** : les composants Vue marqués [P] sont des fichiers distincts.

---

## Parallel Example: User Story 1

```text
# Backend en séquence (même fichier handlers/jeu.rs) : T020 → T021 → T022 → T023 → T024

# Pendant ce temps, frontend en parallèle (fichiers distincts) :
T025  useJeu.ts : fonctions de partie
T026  jeu/Minuteur.vue
T027  jeu/CarteEpreuve.vue
T028  jeu/Correction.vue
T029  jeu/BilanPartie.vue
T032  jeu/PanneauActivites.vue

# Puis assemblage : T030 (page de jeu) → T031 (page du module) → T033 (rail des 4 modules) → T034 (recette)
```

---

## Implementation Strategy

### MVP First (US1 + US2)

1. Phase 1 puis Phase 2.
2. Phase 3 (US1) : on joue sur les épreuves du seed.
3. Phase 4 (US2) : on saisit et on tient les épreuves.
4. **Arrêt et recette** : scénarios 1 (étapes 1, 5, 6) et 2 du quickstart. À ce stade la feature est déjà utile : un jeu complet sur quatre modules, avec un score.

### Incremental Delivery

| Palier | Phases | Ce que le membre gagne |
|---|---|---|
| 1. Socle | 1 à 4 | jouer, avec des épreuves saisies |
| 2. Vivier | 5 | du volume, tiré du contenu publié |
| 3. Retour | 6 | un défi par jour et par semaine |
| 4. Collectif | 7 | des saisons et des classements par pays |
| 5. Social | 8 | les duels entre amis |
| 6. Lecture | 9, 10 | la carte, la réputation, les distinctions |
| 7. Direct | 11 | le duel en direct |

Chaque palier est déployable seul : la migration est posée une fois pour toutes au palier 1, et les routes non livrées n'existent simplement pas encore.

### Ce qui peut glisser sans rien casser

- **Phase 11 (duel direct)** : c'est le seul morceau dont le risque n'est pas maîtrisé d'avance. Le mode « direct » reste désactivé dans la modale de proposition tant qu'elle n'est pas livrée.
- **Phase 9 (carte)** : le classement des pays existe sans elle, en liste.

---

## Notes

- 123 tâches. Aucune n'ajoute de dépendance Cargo ou pnpm.
- Les tâches [P] touchent des fichiers distincts ; `services/jeu.rs`, `handlers/admin/jeu.rs` et `routes.rs` sont partagés par plusieurs histoires et ne se parallélisent pas.
- Commiter après chaque tâche ou groupe logique, messages en français.
- Le volume d'épreuves de production (SC-002) n'est pas une tâche de développement : il se traite à l'ouverture (T123).
