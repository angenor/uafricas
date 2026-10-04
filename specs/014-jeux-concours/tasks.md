---

description: "Liste de tâches : feature 014, jeux variés et concours communautaires"
---

# Tasks: Jeux variés et concours communautaires

**Input** : documents de conception dans `/specs/014-jeux-concours/`

**Prerequisites** : [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/](./contracts/), [quickstart.md](./quickstart.md). La feature 013 (schéma `jeu`, `37_jeu.sql`) doit être en place.

**Tests** : aucune tâche de test automatisé. Le projet n'a ni linting, ni testing, ni CI/CD, et la spec n'en réclame pas. La validation passe par les scénarios de [quickstart.md](./quickstart.md), appelés en fin de chaque phase.

**Organization** : groupement par user story, dans l'ordre de livraison en six paliers de [plan.md](./plan.md) :
1. US1 en partie libre ;
2. US1 en duel et en défi ;
3. US2 et US3 ;
4. US6 puis US4 ;
5. US5 et US7 ;
6. US8 et US9.

**Avancement au 2026-10-04** : premier livrable terminé (T001 à T020). Carte, ordre et paires se jouent en partie libre, vérifiés par l'API et dans un navigateur à 375 et 1280 px. Écarts au plan :
- `38_jeu_concours.sql` porte deux CHECK de plus que le modèle de données : `solution` n'existe que pour l'ordre et les paires, `appariements` que pour les paires.
- `evaluer` prend le pays déjà résolu ; la résolution du code ISO (`resoudre_pays_joue`) est une fonction à part, parce qu'elle lit la base.
- Au doigt, l'ordre se règle par les flèches de chaque ligne : le glisser-déposer natif du navigateur ne répond pas au toucher.
- Dans les paires, toucher l'élément de gauche déjà actif ne le désactive plus. La recette l'a révélé : après une paire, l'élément suivant est sélectionné automatiquement, et le toucher, geste naturel, le désélectionnait et verrouillait la colonne de droite.
- Le seed `94` mélange les éléments par des fonctions SQL, pas à la main (PC4) ; 0 solution sur 8 n'est l'identité.

Étape 2 terminée (T021 à T024). Duel direct sur carte, ordre et paires, vérifié dans deux navigateurs (1280 et 390 px) : même épreuve, temps majoré à 45 s, révélation « juste / faux » pour les deux, victoire 3 à 1. Écarts :
- T021 était déjà couvert par la phase 2 : le duel direct répond par `moteur::repondre`, donc par `evaluer`.
- `ReponseOrdre` et `ReponsePaires` surveillent la liste des CLÉS, pas le tableau : l'état du duel direct est relu toutes les 3 s dans un objet neuf, et surveiller l'objet aurait effacé l'ordre en cours de construction. La recette le vérifie (ordre intact après 4,5 s).
- `EtatDuelDirect` gagne `ma_reponse` : l'ordre et les paires n'ont pas de clé unique, `ma_cle` ne suffisait plus à savoir qu'on avait répondu.
- `PUT /admin/jeu/regles` écrit aussi les primes de concours (avec leurs bornes) : la page renvoie l'objet lu, leur champ d'écran viendra avec les concours.

US2 terminée (T025 à T030). Écarts :
- La 013 déposait déjà ses médias, mais par les routes d'AUTRES modules (photos Afripulse, médias radio et télé avec des sons jusqu'à 80 Mo). `ChampMediaEpreuve` les remplace par la route du jeu.
- **Défaut préexistant corrigé dans `useAdmin.ts`** : `adminFetch` posait `Content-Type: application/json` même sur un `FormData`, donc tout envoi de fichier par `adminFetch` échouait en 400 (« ContentTypeIncompatible »). Les composables d'upload existants le contournaient par un `$fetch` direct ; l'en-tête n'est plus posé quand le corps est un `FormData`.
- T028 : pas de champ `copier_image` sur `Forme`. Toute forme à image copie son média, drapeaux compris. Une image EXTERNE (`https://…`) est gardée telle quelle, car la copier demanderait un client HTTP, donc une dépendance. Une copie orpheline (insertion refusée par l'unicité) est supprimée.
- Recette : la photo stockée n'a plus ni bloc EXIF ni marqueur GPS (injecté dans l'original) ; un PNG renommé `.mp3` est refusé par sa signature binaire ; un extrait de 40 s est refusé par l'écran ; un site dont le fichier n'existe pas ne produit rien (`sans_media`).

US3 terminée (T031 à T039) : 10 formes typées produisent 338 candidates en local (Afripulse 309, Codimoi 21, FactCheck 8). Écarts :
- **Deux défauts du back-office corrigés au passage** : `EpreuveAdmin` et `SignalementRow` lisaient `bonne_reponse` en `i16` non optionnel. `query_as` étant vérifié à l'exécution, la liste des épreuves, la revue et la file des signalements auraient échoué dès la première épreuve carte, ordre ou paires.
- La carte se saisit par **code ISO** (`reponse_pays_iso`, résolu par le serveur), sur la même liste de 55 pays que le jeu ; l'identifiant reste accepté.
- `EpreuveAdmin` renvoie `elements_attendus` et `paires_attendues` : la base garde ordre et paires mélangés, l'administrateur les relit dans l'ordre de saisie.
- **Trois formes de plus que le plan** (T034 à T037 n'en prévoyaient aucune hors Afripulse) : `paires_proverbes`, `paires_citations` (Codimoi) et `paires_idees_recues` (FactCheck, idée reçue ↔ réalité). Sans elles, Codimoi et FactCheck n'auraient eu aucune épreuve typée. Ils restent sous les 30 de SC-005 en local, faute de volume (12 proverbes, 8 idées reçues), comme Afrolang, qui ne s'enrichit que par la saisie.
- Contrôles : 0 paire de monnaies en double, 0 peuple partagé en carte, 243 paires d'éléments consécutifs toutes à plus de 15 % d'écart dans le bon sens ; 8 solutions sur 191 égales à l'identité, soit le hasard attendu (1 chance sur 24).

US6 terminée (T040 à T048). Programmation et modération vérifiées par l'API et dans le navigateur. Écarts :
- La liste des concours résout CHAQUE concours (PC1) et filtre la phase en Rust : la phase n'est pas une colonne, elle ne se filtre pas en SQL. Volumes attendus : quelques dizaines de concours.
- Les modules de rattachement viennent de `utils/modulesPlateforme.ts` (les 20 modules du document client), pas de `navigation-africans.ts`, qui ne nomme pas les modules par un code.
- « Retirer du concours » une photo publiée est un REJET avec motif (`publiee → rejetee`), comme le permet le contrat : le membre sait pourquoi.
- Recette : annulation automatique sous le seuil à la première lecture, une seule prime de participation après trois lectures (PC5) ; titre gelé et thème modifiable une fois l'appel ouvert ; suppression refusée d'un concours ouvert ; motif de rejet obligatoire.

US4 terminée (T049 à T055), avec T073 en avance. Dépôt, remplacement, retrait, galerie anonyme, hub, panneau de module et « Mes activités » vérifiés à 375 et 1280 px. Écarts :
- **Défaut trouvé à la recette** : la page d'un concours relisait le bloc `moi` au montage seulement si le jeton était déjà là. Or après un rechargement, le jeton d'accès (en mémoire) est restauré APRÈS le montage, et le `refresh()` de `useAsyncData` est ignoré pendant l'hydratation : un membre connecté se voyait invité à se connecter. La page relit désormais le concours elle-même dès que le jeton apparaît.
- « Remplacer » n'est proposé sur une photo refusée que s'il reste de la place : la remettre en attente la rend active, et le serveur refuserait au-delà du plafond.
- Les concours à venir ne sont pas listés publiquement, et leur page répond 404 : on n'annonce pas un appel qui n'est pas ouvert.
- Le panneau d'activités affiche les concours du module, mais il n'est monté que sur les quatre modules jouables. Un concours rattaché à Afroculture est visible sur l'espace Activités, pas encore sur la page d'Afroculture (aucun panneau n'y est monté).
- T073 est livrée avec la page : `jeu/ReglesVote.vue` affiche les règles d'admissibilité des voix sur tout concours en appel ou en vote.

US5 terminée (T056 à T060). Vérifié par l'API (huit votants, 48 votes) et dans le navigateur à 375 px : même paire au rechargement ; 10 paires sur 10 possibles pour un participant, sa photo jamais présentée ; zéro auto-vote ; voix trop rapide et voix d'un compte récent enregistrées mais écartées, sans que la réponse le dise ; présentations de 18 à 19 par photo (écart 1, pour 2 visés) ; aucun champ d'auteur, de décompte ou d'admissibilité dans les réponses ; 15 votes en 16 s, dont 1 s d'attente par vote imposée par le test. Écarts :
- Pour la recette, des jetons d'accès sont fabriqués avec le secret LOCAL du backend (`jwtlocal.py` dans le répertoire de travail, hors dépôt), afin de voter avec les comptes de démonstration sans toucher à leurs mots de passe.
- Le tirage de B réutilise la condition « paire jamais vue » écrite pour A, en renommant les alias plutôt qu'en la dupliquant.

US7 terminée (T061 à T065) : premier concours complet de bout en bout. Vérifié :
- résultats établis à la première lecture en 72 ms (SC-011) ;
- après 1 000 relectures, 6 primes de participation versées une fois, aucune clé en double (SC-012) ;
- gains d'origine `concours`, rattachés à la saison et au pays ;
- ex aequo forcés (50 % et 50 %) au même rang, avec la même prime ;
- distinction `jeu_laureat` au premier ;
- notifications « podium » et « résultats » ;
- aucun point d'engagement, statut inchangé (SC-013), seule la réputation bouge.

Écarts :
- `classement()` est l'unique calcul du classement : les résultats l'appellent, le jury (T066) et le suivi (T069) l'appelleront.
- Sur « Tenue traditionnelle », aucune photo n'a atteint les 10 duels comptés (les votes de démonstration, trop rapides, ont été écartés) : pas de podium, classement entièrement « hors seuil ». C'est la règle FR-043 qui s'applique, pas un défaut ; un seuil adapté au volume attendu se règle par concours.
- **Défaut trouvé à la recette** : le podium plaçait ses cartes d'après le RANG, si bien que deux 2ᵉ ex aequo délogeaient le 1er du centre. L'ordre suit désormais la position dans le podium, la hauteur de marche le rang.
- La première comparaison SC-013 a échoué à tort : elle incluait les comptes d'engagement CRÉÉS au versement de la réputation (0 point, statut « membre », soit l'état implicite d'un membre sans compte).

US8 et US9 terminées (T066 à T074). Vérifié par l'API et dans le navigateur :
- **jury** : délibération sans résultats publiés, galerie anonyme, choix parmi les finalistes, podium du jury en tête, repli sur la communauté passé le délai ;
- **suivi** : le compte à 31 votes dans la même minute est signalé (« rythme ») ; ses voix s'écartent et se rétablissent, avec une ligne d'audit à chaque fois ;
- **signalement** : refusé sur sa propre photo ; suspension au 11ᵉ signalement distinct (un doublon ne compte pas), auteur notifié, photo retirée de la galerie et du classement ; rétablissement avec compteur remis à zéro ;
- **compte suspendu** : sa photo sort du classement.

Écarts :
- Le suivi et les finalistes appellent `classement()`, le calcul des résultats : il n'y en a toujours qu'un.
- Le signalement réutilise la modale commune `AfricansModaleSignalement` ; le motif enregistré est le libellé choisi, suivi du commentaire.
- Avec des voix insérées en SQL pour la recette, les « présentations par photo » du suivi restent à 0 : elles ne sont comptées que par le tirage. Le test de l'étape 5, mené par l'API, les avait vérifiées (18 à 19 par photo).

Finitions terminées (T075 à T080) : **les 80 tâches sont faites**.
- **Relecture PC1** : un trou fermé. `retablir_participation` pouvait republier une photo suspendue après les résultats ; elle réapparaissait dans une galerie figée, sans rang. Elle résout désormais son concours et refuse s'il est terminé.
- **PC4** : environ 4 % de solutions égales à l'identité parmi les épreuves dérivées, soit le hasard attendu (1 sur 24 pour quatre éléments), et aucune parmi les épreuves saisies.
- **Avertissements** : `ParticipationRow` et `PARTICIPATION_COLONNES`, jamais employés, sont supprimés ; plus aucun avertissement dans les fichiers du jeu.
- **Recette finale** :
  - 18 combinaisons de pages et de largeurs sans débordement ; aucune classe daisyUI hors du back-office ;
  - `pnpm build` et `cargo build` passent ;
  - migration `38` rejouée deux fois, 1 120 servables avant et après ;
  - journal des gains cohérent ; non-régression de la 013.
- **Écart de lecture à la recette** : les épreuves typées dérivées étaient jouables, alors qu'elles naissent candidates. L'audit montre une revue faite à la main dans Safari avec le compte test-admin : ce n'est pas un défaut.

**Les cinq points de conception de [plan.md](./plan.md)**, rappelés dans les tâches qu'ils concernent (repère ⚠️ PCn) :
- **PC1** : `resoudre_concours` est appelée par toute route qui lit un concours.
- **PC2** : toutes les requêtes qui remplissent `EpreuveRow` lisent les nouvelles colonnes.
- **PC3** : `servir_stable` mélange aussi `appariements`.
- **PC4** : les éléments d'un « ordre » et la colonne de droite d'une « paires » sont stockés dans un ordre aléatoire, à la saisie comme à la dérivation.
- **PC5** : les primes de participation d'un concours annulé partent sous la même clé que celles d'un concours mené à terme.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers distincts, aucune dépendance sur une tâche inachevée)
- **[Story]** : US1 à US9 de [spec.md](./spec.md)

## Path Conventions

Monorepo web : `uafricas_backend/src/`, `uafricas_backend/doc/bd/`, `uafricas_frontend/app/`.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose** : poser la migration `38` complète. Elle sert les deux familles, mais elle est écrite une seule fois.

- [X] T001 Écrire `uafricas_backend/doc/bd/schemas/38_jeu_concours.sql`, partie épreuves ([data-model.md §1 à §4](./data-model.md)) :
  - la fonction `jeu.est_permutation(smallint[]) RETURNS boolean IMMUTABLE` ;
  - sur `jeu.epreuve` : `ADD COLUMN IF NOT EXISTS type_reponse VARCHAR(10) NOT NULL DEFAULT 'choix'`, `solution SMALLINT[]`, `appariements TEXT[]`, `valeurs TEXT[]`, `reponse_pays_id UUID REFERENCES shared.pays(id)` ;
  - `ALTER COLUMN bonne_reponse DROP NOT NULL` ;
  - `DROP CONSTRAINT IF EXISTS ck_epreuve_propositions, ck_epreuve_bonne_reponse`, puis les 8 CHECK de §1, chacun précédé de son `DROP CONSTRAINT IF EXISTS` pour rester rejouable ;
  - sur `jeu.reponse` : `reponse_detail SMALLINT[]` et `pays_choisi_id UUID REFERENCES shared.pays(id)` ;
  - sur `jeu.regles` : `majoration_ordre_paires_s` (15, 0..60), `prime_concours_participation` (2), `prime_concours_podium SMALLINT[]` (`{30,20,10}`, CHECK 3 valeurs ≥ 0 décroissantes) ;
  - `ck_gain_origine` élargi à `'concours'`.
- [X] T002 Compléter `uafricas_backend/doc/bd/schemas/38_jeu_concours.sql`, partie concours ([data-model.md §5 à §9](./data-model.md)) :
  - les tables `jeu.concours`, `jeu.participation`, `jeu.confrontation`, `jeu.resultat_concours`, `jeu.signalement_participation`, en `CREATE TABLE IF NOT EXISTS`, avec toutes leurs CHECK et leurs index ;
  - les deux index uniques de `confrontation` : `(concours_id, votant_id, a_id, b_id)`, et l'index partiel `(concours_id, votant_id) WHERE choix_id IS NULL` ;
  - CHECK `a_id < b_id`, `choix_id IN (a_id, b_id)`, `(choix_id IS NULL) = (vote_at IS NULL)`.
- [X] T003 Compléter `uafricas_backend/doc/bd/schemas/38_jeu_concours.sql`, partie engagement ([data-model.md §10](./data-model.md)), en `ON CONFLICT DO NOTHING` :
  - 3 règles à 0 point, catégorie `jeux` : `jeu_concours_participation` (réputation 1), `jeu_concours_podium` (5), `jeu_concours_vote` (1) ;
  - 2 badges `actions_comptees` : `jeu_laureat` (1 × podium, icône `medal`) et `jeu_jure_populaire` (5 × vote, icône `scale-balanced`).

  Vérifier que les icônes sont enregistrées dans `uafricas_frontend/app/plugins/fontawesome.ts` et les y ajouter sinon.
- [X] T004 Ajouter `\ir schemas/38_jeu_concours.sql` après `37_jeu.sql` dans `uafricas_backend/doc/bd/schema.sql`. Appliquer en local deux fois de suite : aucune erreur, et le nombre d'épreuves servables est le même avant et après (toutes de type `choix`).

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose** : faire comprendre au moteur de la 013 les nouveaux types d'épreuve, sans rien changer pour le choix multiple. US1, US2 et US3 en dépendent toutes.

**⚠️ CRITICAL** : aucune histoire de la famille A ne commence avant la fin de cette phase.

- [X] T005 Étendre `EpreuveRow` dans `uafricas_backend/src/models/jeu.rs` (`type_reponse`, `solution: Option<Vec<i16>>`, `appariements: Option<Vec<String>>`, `valeurs: Option<Vec<String>>`, `reponse_pays_id: Option<Uuid>`, `bonne_reponse: Option<i16>`) et sa constante de colonnes. ⚠️ **PC2** : relire **chaque** `query_as::<_, EpreuveRow>` de `services/jeu.rs`, `handlers/jeu.rs`, `handlers/jeu_duel.rs`, `handlers/admin/jeu.rs` et vérifier qu'elle passe par la constante ; corriger celles qui listent leurs colonnes à la main.
- [X] T006 Ajouter dans `uafricas_backend/src/models/jeu.rs` :
  - `ReponseJoueur`, avec les champs facultatifs `cle`, `ordre`, `paires` et `pays` : désérialisation de la forme de [api-membre.md §1](./contracts/api-membre.md) ;
  - `PropositionServie` réutilisée pour `appariements` ;
  - `EpreuveServie`, qui gagne `type_reponse` et `appariements: Option<Vec<PropositionServie>>` ;
  - `Correction` et `CorrectionManche`, qui gagnent `type_reponse`, `solution`, `valeurs` (clé → texte), `bon_pays { iso, nom }` et `jouee`, avec `bonne_cle` passé en `Option<i16>`.
- [X] T007 Écrire `evaluer(epreuve: &EpreuveRow, reponse: &ReponseJoueur, pays_iso_vers_id) -> Result<Option<bool>, ApiErreur>` dans `uafricas_backend/src/services/jeu.rs` (research D2) :
  - `None` = sans réponse ;
  - 400 si la forme ne correspond pas au type, si `ordre` ou `paires` n'est pas une permutation exacte des clés `1..n`, ou si le pays n'est pas l'un des 55 ;
  - juste si et seulement si tout est juste.

  Le code ISO2 se résout en `pays_id` par `shared.pays`, borné à `PAYS_AFRICAINS_ISO2`.
- [X] T008 Dans `uafricas_backend/src/services/jeu.rs`, faire passer `inscrire_reponse` par `evaluer` et écrire `reponse_detail` ou `pays_choisi_id` dans `jeu.reponse`. `correction_existante` les relit pour reconstruire la correction à l'identique (idempotence). Le choix multiple garde exactement son comportement : même issue, même gain.
- [X] T009 Dans `uafricas_backend/src/services/jeu.rs`, étendre `servir` : `appariements` mélangés indépendamment des propositions, avec des clés égales à leur rang stocké, et rien de la solution dans `EpreuveServie`. ⚠️ **PC3** : `servir_stable` mélange aussi `appariements`, avec une graine dérivée (`graine ^ 0x9E37_79B9`), et trie d'abord par clé pour que l'ordre soit stable entre deux relectures.
- [X] T010 Dans `uafricas_backend/src/services/jeu.rs`, ajouter `majoration_ordre_paires_s` à `ReglesJeu` (models) et à `delai_ms` pour `ordre` et `paires`, cumulable avec `MARGE_MEDIA_MS`. Exposer `delai_ms` dans `EpreuveServie` si ce n'est pas déjà le cas.
- [X] T011 Construire la correction étendue dans `uafricas_backend/src/services/jeu.rs` (fonction `construire_correction`, partagée par `inscrire_reponse` et `correction_existante`) :
  - `ordre` : `solution` + `valeurs` ;
  - `paires` : `solution` réindexée **par ordre croissant de clé de gauche** ;
  - `carte` : `bon_pays` lu dans `shared.pays` ;
  - `choix` : `bonne_cle`.
- [X] T012 [P] Étendre les types de `uafricas_frontend/app/composables/useJeu.ts` (`TypeReponse`, `EpreuveServieAPI.type_reponse` et `appariements`, `ReponseJoueur`, `CorrectionAPI` étendue) et faire passer `repondre(partieId, rang, reponse: ReponseJoueur)` à la place de `cle`. Garder un appel compatible `{ cle }` pour le choix multiple.
- [X] T013 Lancer `cargo build`. Rejouer en API une partie de la 013 sur des épreuves `choix` : mêmes issues, mêmes gains, même correction, plus le champ `type_reponse: "choix"` (non-régression).

**Checkpoint** : le moteur sait corriger les quatre types ; le choix multiple n'a pas bougé.

---

## Phase 3: User Story 1 — Répondre autrement (Priority: P1) 🎯 MVP

**Goal** : carte, ordre et paires jouables dans tous les cadres de la 013.

**Independent Test** : [quickstart.md, scénario 1](./quickstart.md).

### Palier 1 — partie libre

- [X] T014 [P] [US1] Écrire le seed local `uafricas_backend/doc/bd/seeds-locaux/94_seed_local_jeu_types.sql`, idempotent :
  - 4 épreuves `carte` (Afripulse) ;
  - 4 épreuves `ordre` (Afripulse : populations, avec `valeurs` ; Afrolang : 3 langues par nombre de locuteurs) ;
  - 4 épreuves `paires` (Codimoi : proverbe ↔ pays ; Afrolang : mot ↔ traduction) ;
  - toutes `jouable`, avec une explication.

  ⚠️ **PC4** : les éléments et la colonne de droite sont **insérés dans un ordre mélangé**, et `solution` est calculée en conséquence. Ne jamais insérer dans l'ordre attendu.
- [X] T015 [P] [US1] Créer `uafricas_frontend/app/components/jeu/ReponseCarte.vue`, en Tailwind pur :
  - `CommonCarteAfriqueValeurs` cliquable, avec mise en évidence du pays survolé et du pays choisi, et agrandissement possible sur téléphone ;
  - **une liste de pays accessible** (champ de recherche + liste sur `NOMS_PAYS_FR`, entièrement au clavier, FR-010) ;
  - bouton « Valider » ; émet `{ pays: iso }` ;
  - à la correction, colore en vert le bon pays et en rouge le pays joué.
- [X] T016 [P] [US1] Créer `uafricas_frontend/app/components/jeu/ReponseOrdre.vue` :
  - liste réordonnable au glisser-déposer (API Drag and Drop natif, plus la gestion du toucher sur téléphone) ;
  - boutons « monter » et « descendre » sur chaque ligne, pour le clavier ;
  - affichage du critère tiré de l'énoncé, bouton « Valider » ; émet `{ ordre: [cles] }` ;
  - à la correction, affiche l'ordre attendu avec les `valeurs` et marque les positions fausses.
- [X] T017 [P] [US1] Créer `uafricas_frontend/app/components/jeu/ReponsePaires.vue` :
  - deux colonnes ; on touche un élément de gauche, puis un élément de droite, pour les relier ;
  - une couleur par paire ; retoucher une paire la défait ;
  - accessible au clavier (Tab, puis Entrée) ;
  - « Valider » actif quand toutes les paires sont faites ; émet `{ paires: [...] }` dans l'ordre croissant des clés de gauche ;
  - à la correction, montre les bonnes paires.
- [X] T018 [US1] Dans `uafricas_frontend/app/components/jeu/CarteEpreuve.vue`, aiguiller selon `type_reponse` (le choix multiple est inchangé). Le minuteur et l'envoi `cle: null` à l'expiration restent communs. Dans `uafricas_frontend/app/components/jeu/Correction.vue`, rendre la solution par type.
- [X] T019 [US1] Dans `uafricas_frontend/app/pages/activites/partie/[id].vue`, transmettre la réponse typée à `useJeu().repondre`. Vérifier le bilan de partie.
- [X] T020 [US1] Dérouler [quickstart.md, scénario 1](./quickstart.md), points 1 à 7, en partie libre ; corriger les écarts. Le point 6 (fuite de la solution) est bloquant.

### Palier 2 — défi, duel différé, duel direct

- [X] T021 [US1] Dans `uafricas_backend/src/handlers/jeu_duel.rs`, faire passer `repondre_direct` par `ReponseJoueur` et `evaluer`, et enregistrer `reponse_detail` et `pays_choisi_id`. `etat_direct` sert `EpreuveServie` par `servir_stable` (PC3) et la `CorrectionManche` étendue en phase de révélation. Le délai de manche passe par `delai_epreuve_ms`, majoration comprise.
- [X] T022 [US1] Dans `uafricas_frontend/app/components/jeu/DuelDirectSalle.vue`, monter les mêmes composants de réponse que `CarteEpreuve` et envoyer la réponse typée. La révélation montre la solution par type pour les deux joueurs.
- [X] T023 [US1] Vérifier défi du jour et duel différé sur des épreuves des nouveaux types : ils passent par `inscrire_reponse`, donc aucun code ne devrait changer. Dérouler [quickstart.md, scénario 1](./quickstart.md), point 8, avec deux navigateurs.
- [X] T024 [US1] Ajouter le champ « Temps majoré pour l'ordre et les paires » à `uafricas_frontend/app/pages/admin/activites/regles.vue` et au `PUT /admin/jeu/regles` de `uafricas_backend/src/handlers/admin/jeu.rs` (bornes 0 à 60, audit `REGLES_MODIFIEES`).

**Checkpoint** : US1 livrée. Les nouveaux types se jouent partout ; tirage, score, défis et duels sont inchangés.

---

## Phase 4: User Story 2 — Reconnaître une photo ou un son (Priority: P1)

**Goal** : déposer des médias d'épreuve, et tirer des épreuves à photo du contenu publié.

**Independent Test** : [quickstart.md, scénario 2](./quickstart.md).

- [X] T025 [US2] Créer `POST /api/admin/jeu/medias` (multipart) dans `uafricas_backend/src/handlers/admin/jeu.rs` (research D4, [api-admin.md §1](./contracts/api-admin.md)) :
  - `image` : `image_validation::normaliser_photo`, écrite dans `uploads/jeu/images/<uuid>.<ext>` ;
  - `audio` : signature binaire MP3 (`ID3` ou synchronisation `0xFFE`), OGG (`OggS`), M4A (`ftyp` à l'octet 4) ou WAV (`RIFF….WAVE`), 1 Mo au plus, écrite dans `uploads/jeu/audios/` ;
  - créer les dossiers au besoin ; audit `MEDIA_EPREUVE_DEPOSE` ;
  - route déclarée dans `uafricas_backend/src/routes.rs` **avant** `/jeu/epreuves/{id}`.
- [X] T026 [P] [US2] Créer `uafricas_frontend/app/components/admin/jeu/ChampMediaEpreuve.vue` (daisyUI) :
  - choix image ou son, dépôt, aperçu (`<img>` ou `<audio controls>`) ;
  - pour un son, lecture de la durée par un élément `Audio` **avant l'envoi**, et refus au-delà de 30 s ;
  - émet `{ media_type, media_url }`.

  Ajouter `deposerMedia(fichier, type)` à `uafricas_frontend/app/composables/useAdminJeu.ts`.
- [X] T027 [US2] Monter `ChampMediaEpreuve` dans `uafricas_frontend/app/components/admin/jeu/EpreuveFormulaire.vue`, à la place de la saisie d'URL. Une URL existante reste affichée et remplaçable.
- [X] T028 [US2] Dans `uafricas_backend/src/services/jeu_derivation.rs`, permettre à une forme de **copier** l'image source :
  - champ `copier_image: bool` sur `Forme` ;
  - à l'insertion de la candidate, copier le fichier référencé par `media_url` (chemin `/uploads/…` résolu sous `UPLOAD_DIR`) vers `uploads/jeu/images/<uuid>.<ext>`, et enregistrer la copie ;
  - une source dont l'image est introuvable ne produit rien (compter `sans_media` dans `BilanForme`).
- [X] T029 [US2] Ajouter les formes `photo_recette` (source `recette_culinaire`, image = `images[1]`) et `photo_site` (source `site_touristique`, image = `image_url`) à `FORMES` dans `uafricas_backend/src/services/jeu_derivation.rs` : énoncés « De quel pays vient ce plat ? » et « Dans quel pays se trouve ce site ? », sans nommer le plat ni le site ; distracteurs `PaysAfricains`.
- [X] T030 [US2] Dérouler [quickstart.md, scénario 2](./quickstart.md), dont la vérification de la suppression des EXIF et le refus d'un PNG renommé en `.mp3`.

**Checkpoint** : US2 livrée.

---

## Phase 5: User Story 3 — Constituer le vivier des nouveaux types (Priority: P1)

**Goal** : saisir et dériver des épreuves carte, ordre et paires, revues comme les autres.

**Independent Test** : [quickstart.md, scénario 3](./quickstart.md).

- [X] T031 [US3] Étendre `CreerEpreuveRequest` et `ModifierEpreuveRequest` dans `uafricas_backend/src/models/admin/jeu.rs` avec `type_reponse`, `reponse_pays_id`, `elements: [{ texte, valeur? }]` et `paires: [{ gauche, droite }]` ([api-admin.md §1](./contracts/api-admin.md)).
- [X] T032 [US3] Dans `uafricas_backend/src/handlers/admin/jeu.rs` (`creer_epreuve`, `modifier_epreuve`), valider par type, avec des 400 qui nomment le champ : bornes, doublons, `valeur` sur tous les éléments ou sur aucun, pays parmi les 55.

  ⚠️ **PC4** : **mélanger** les éléments (et la colonne de droite) avant l'insertion et calculer `solution` en conséquence. `obtenir_epreuve` renvoie l'épreuve **remise dans l'ordre attendu** pour l'édition.
- [X] T033 [P] [US3] Étendre `uafricas_frontend/app/components/admin/jeu/EpreuveFormulaire.vue` (daisyUI, labels `flex flex-col`) :
  - sélecteur de type ;
  - `carte` : sélecteur de pays sur `NOMS_PAYS_FR` ;
  - `ordre` : 3 à 6 lignes réordonnables avec valeur facultative et un rappel « saisissez dans l'ordre attendu » ;
  - `paires` : 3 à 5 lignes gauche/droite.

  Mettre à jour les types de `uafricas_frontend/app/composables/useAdminJeu.ts`.
- [X] T034 [US3] Étendre `Forme` dans `uafricas_backend/src/services/jeu_derivation.rs` : champ `type_reponse`, colonnes facultatives `solution`, `appariements`, `valeurs` et `reponse_pays_id` dans `SourceEligible` (`#[sqlx(default)]`) ; `deriver_forme` écrit selon le type. Ajouter `type_reponse` à `EtatForme` (écran de revue).

  ⚠️ **PC4** : pour `ordre` et `paires`, la requête rend les éléments **dans l'ordre attendu**, et c'est le Rust qui les mélange avant l'insertion et calcule `solution`. Un seul endroit, testé une fois.
- [X] T035 [US3] Ajouter les formes `ordre_population` et `ordre_superficie` (pivot : fiche ; 3 autres fiches tirées au hasard ; **écart d'au moins 15 % entre valeurs consécutives**, sinon rien ; `valeurs` via `jeu.fmt_habitants` et `jeu.fmt_km2`) dans `uafricas_backend/src/services/jeu_derivation.rs`.
- [X] T036 [US3] Ajouter les formes `paires_capitales` et `paires_monnaies` (pivot + 3 fiches ; pour les monnaies, **4 valeurs toutes distinctes** sans tenir compte de la casse, sinon rien) dans `uafricas_backend/src/services/jeu_derivation.rs`.
- [X] T037 [US3] Ajouter les formes `carte_capitale` (« Désignez le pays dont la capitale est X »), `carte_site` et `carte_peuple` dans `uafricas_backend/src/services/jeu_derivation.rs`. `carte_peuple` ne retient qu'un peuple déclaré par **un seul** pays et cité dans les langues d'aucun autre ; `reponse_pays_id` = pays du pivot.
- [X] T038 [US3] Dans `uafricas_frontend/app/pages/admin/activites/revue.vue`, afficher le type de réponse de chaque forme et de chaque candidate, avec un aperçu de la solution (ordre attendu, paires, pays) pour que la revue se fasse sans ouvrir l'épreuve.
- [X] T039 [US3] Dérouler [quickstart.md, scénario 3](./quickstart.md) : dérivation, trois requêtes SQL d'ambiguïté (aucun résultat), revue, saisie refusée, et le décompte SC-005 (au moins 30 épreuves hors `choix` par module pilote ; Afrolang attendu en dessous).

**Checkpoint** : famille A complète (US1 à US3).

---

## Phase 6: User Story 6 — Programmer et modérer un concours (Priority: P2)

**Goal** : un administrateur crée un concours et modère les participations ; les phases avancent seules.

**Independent Test** : [quickstart.md, scénario 4](./quickstart.md), points 1, 3, 5 et 7.

- [X] T040 [P] [US6] Créer `uafricas_backend/src/models/jeu_concours.rs` : `ConcoursRow`, `ParticipationRow`, `ConfrontationRow`, `ResultatRow`, l'enum `Phase` (`a_venir`, `appel`, `vote`, `deliberation`, `resultats`, `annule`) avec `fn phase(c, maintenant) -> Phase` (data-model §5), et les DTO publics de [api-membre.md §2](./contracts/api-membre.md). La galerie a deux formes : **sans auteur** pendant appel et vote, **avec** auteur après. Déclarer le module dans `uafricas_backend/src/models/mod.rs`.
- [X] T041 [P] [US6] Créer `uafricas_backend/src/models/admin/jeu_concours.rs` : requêtes de création et de modification, file de modération, suivi. Déclarer dans `models/admin/mod.rs`.
- [X] T042 [US6] Créer `uafricas_backend/src/services/jeu_concours.rs` avec `resoudre_concours(pool, id) -> Result<ConcoursRow, ApiErreur>` (research D6) :
  - lecture ; si la phase calculée exige une transition **écrite** (annulation au seuil du vote, ou résultats), transaction avec `SELECT … FOR UPDATE`, puis revérification de `etat = 'actif'` ;
  - annulation : `etat = 'annule'`, motif « participations insuffisantes » ; puis, **après le COMMIT**, notifications `jeu.concours_annule` et primes de participation des publiées.

  ⚠️ **PC5** : la clé est `concours:{id}:participation:{pid}`, la même que celle des résultats.

  Laisser un appel `etablir_resultats` vide (`todo` explicite remplacé en T061). Déclarer dans `services/mod.rs`.
- [X] T043 [US6] Créer `uafricas_backend/src/handlers/admin/jeu_concours.rs` ([api-admin.md §2 et §3](./contracts/api-admin.md)), sous `verifier_permission!(…, "jeu.gerer")`, chaque mutation avec `audit::log_action` :
  - `lister_concours`, `creer_concours` (règles de dates FR-022), `modifier_concours` (FR-024 : champs permis selon la phase, 409 qui nomme le champ), `supprimer_concours` (phase `a_venir`), `annuler_concours` (avec motif, primes PC5) ;
  - `lister_participations` (file transversale), `accepter`, `rejeter` (motif obligatoire), `moderation_groupee`, `retablir`, `signalements_participation` ;
  - notifications `jeu.participation_acceptee` et `jeu.participation_rejetee`.

  ⚠️ **PC1** : `lister_concours` et toute lecture d'un concours passent par `resoudre_concours`.
- [X] T044 [US6] Déclarer les routes admin dans `uafricas_backend/src/routes.rs`. **Littéraux d'abord** : `/jeu/concours/participations…` avant `/jeu/concours/{id}…`.
- [X] T045 [P] [US6] Créer `uafricas_frontend/app/composables/useAdminConcours.ts` sur `useAdmin` (`adminFetch`, `listerPagine`) : concours, modération, suivi, jury.
- [X] T046 [P] [US6] Créer `uafricas_frontend/app/components/admin/jeu/ConcoursFormulaire.vue` (daisyUI) :
  - titre, thème, règlement, rattachement (liste des codes de modules de `navigation-africans.ts`), visuel ;
  - les trois dates, avec un contrôle de cohérence immédiat ;
  - réglages (participations par membre, minimum, plafond de votes, seuil de présentations, jury, finalistes, délai) et primes facultatives ;
  - les champs non modifiables après l'ouverture sont désactivés, avec la raison.
- [X] T047 [US6] Créer `uafricas_frontend/app/pages/admin/activites/concours/index.vue` (liste par phase, création) `uafricas_frontend/app/pages/admin/activites/concours/[id].vue` (fiche du concours : réglages, phase, onglet « Participations » de ce concours ; les onglets « Jury » et « Suivi du vote » viennent en T067 et T072) et `uafricas_frontend/app/pages/admin/activites/participations.vue` (file de modération transversale : photo en grand, accepter, rejeter avec motif, sélection multiple). Ajouter « Concours » et « Participations » dans la section Activités de `uafricas_frontend/app/components/admin/AdminSidebar.vue`, et les titres dans `uafricas_frontend/app/layouts/admin.vue`.
- [X] T048 [US6] Ajouter les 6 types `jeu.participation_*` et `jeu.concours_*` dans `uafricas_frontend/app/mocks/notifications.ts` et `uafricas_frontend/app/pages/notifications.vue` (libellé, icône, lien vers `/activites/concours/{id}`).

**Checkpoint** : un concours se programme et se modère ; les phases avancent seules.

---

## Phase 7: User Story 4 — Participer à une bataille de photos (Priority: P2)

**Goal** : un membre découvre un concours et y dépose sa photo.

**Independent Test** : [quickstart.md, scénario 4](./quickstart.md), points 2, 4 et 6.

- [X] T049 [US4] Créer `uafricas_backend/src/handlers/jeu_concours.rs`, partie lecture et dépôt ([api-membre.md §2 et §3](./contracts/api-membre.md)) :
  - `lister_concours` (public, filtres `phase` et `rattachement`) ;
  - `obtenir_concours` (jeton facultatif, bloc `moi`) ;
  - `galerie` (ordre **aléatoire** et sans auteur pendant appel et vote, classée avec auteurs après les résultats) ;
  - `deposer` (multipart, `garde_joueur`, phase `appel`, plafond de participations **sous verrou de la ligne du concours**, `normaliser_photo`, écriture dans `uploads/jeu/concours/{id}/`) ;
  - `remplacer`, `retirer`, `mes_participations`.

  ⚠️ **PC1** : chaque handler commence par `resoudre_concours`, liste comprise (une résolution par concours listé).
- [X] T050 [US4] Déclarer les routes membre et publiques dans `uafricas_backend/src/routes.rs` : `/concours/mes-participations` **avant** `/concours/{id}`.
- [X] T051 [P] [US4] Créer `uafricas_frontend/app/composables/useConcours.ts` (types de [api-membre.md](./contracts/api-membre.md), `appelAuth` pour les routes membre, `$fetch` pour le public).
- [X] T052 [P] [US4] Créer `uafricas_frontend/app/components/jeu/CarteConcours.vue` (visuel, titre, phase en clair avec compte à rebours jusqu'à la prochaine échéance, appel à l'action selon la phase) `uafricas_frontend/app/components/jeu/GalerieConcours.vue` (grille de photos ; mode anonyme en ordre aléatoire pendant appel et vote, mode classé en T063) et `uafricas_frontend/app/components/jeu/DeposerParticipation.vue` (choix et aperçu de la photo, légende de 200 signes avec compteur, rappel de la modération préalable, messages d'erreur du serveur affichés tels quels).
- [X] T053 [US4] Créer `uafricas_frontend/app/pages/activites/concours/index.vue` (concours en cours, puis terminés) et `uafricas_frontend/app/pages/activites/concours/[id].vue` (vue par phase : à venir, appel = règlement + dépôt + « ma participation » et son état ou son motif de rejet, vote et résultats en T059 et T063), sur le gabarit `africans` (`layout: false` + `NuxtLayout`), en Tailwind pur, avec un SSR et un Open Graph pour le partage de la galerie.
- [X] T054 [US4] Ajouter le bloc « Concours en cours » dans `uafricas_frontend/app/pages/activites/index.vue`, et le concours rattaché au module dans `uafricas_frontend/app/components/jeu/PanneauActivites.vue` (`GET /concours?rattachement=<code>&phase=en_cours`). Ajouter « Mes participations » dans `uafricas_frontend/app/pages/mon-compte/activites.vue`.
- [X] T055 [US4] Dérouler [quickstart.md, scénario 4](./quickstart.md) en entier, dont l'annulation automatique d'un concours sous le seuil (PC5 : une seule prime après dix relectures).

**Checkpoint** : on dépose et on modère ; le concours s'arrête à l'appel (attendu, voir le palier 5).

---

## Phase 8: User Story 5 — Voter à l'aveugle (Priority: P2)

**Goal** : départager les participations par confrontations de paires, anonymes et équilibrées.

**Independent Test** : [quickstart.md, scénario 5](./quickstart.md).

- [X] T056 [US5] Dans `uafricas_backend/src/services/jeu_concours.rs`, écrire `tirer_confrontation(conn, concours, votant)` (research D7), en transaction :
  1. rendre la confrontation ouverte s'il y en a une ;
  2. sinon, appliquer le plafond `votes_max` ;
  3. choisir A, la moins présentée (`ORDER BY nombre_presentations, random()`), parmi les publiées qui ne sont pas au votant et qui ont un partenaire non encore vu par lui ;
  4. choisir B de même parmi les partenaires de A ;
  5. insérer `(a<b)` avec `gauche_est_a` aléatoire, et `presentee_at = NOW()` ;
  6. `nombre_presentations + 1` sur les deux ;
  7. si aucune paire n'est possible, `Termine { raison }`.

  Le conflit sur l'index partiel (deux onglets) se résout en relisant la confrontation ouverte.
- [X] T057 [US5] Dans `uafricas_backend/src/services/jeu_concours.rs`, écrire `voter(conn, confrontation, votant, cote)` :
  - idempotent si déjà votée ;
  - refus si la confrontation n'est pas au votant, ou si le votant est l'auteur d'un des côtés (défense en profondeur) ;
  - `choix_id` et `vote_at = NOW()` ;
  - `comptee` et `motif_ecart` selon research D9 : `trop_rapide` à moins de 1 000 ms de `presentee_at`, `compte_recent` si `created_at > vote_debut`, `non_verifie` si `email_verifie = false`.

  Puis enchaîner `tirer_confrontation`.
- [X] T058 [US5] Ajouter les handlers `confrontation_courante` et `voter` à `uafricas_backend/src/handlers/jeu_concours.rs` (`garde_joueur`, phase `vote` sinon 409, PC1), avec les routes. La réponse ne contient **jamais** d'auteur, de décompte, ni `comptee` (FR-039).
- [X] T059 [P] [US5] Créer `uafricas_frontend/app/components/jeu/Confrontation.vue` :
  - deux photos côte à côte (empilées sous 640 px), légende ;
  - choix par clic, par toucher, ou par les touches ← et → ;
  - transition immédiate vers la paire suivante renvoyée par le vote ;
  - compteur « N votes » du votant seulement ;
  - écran de fin « vous avez tout vu » ou « plafond atteint ».

  Intégrer dans `uafricas_frontend/app/pages/activites/concours/[id].vue` (phase vote), avec la galerie anonyme en ordre aléatoire.
- [X] T060 [US5] Dérouler [quickstart.md, scénario 5](./quickstart.md), dont l'équilibre des présentations (écart ≤ 2 après 40 votes) et la vérification **réseau** de l'absence d'auteur et de décompte.

**Checkpoint** : on vote ; aucun résultat n'est encore publié.

---

## Phase 9: User Story 7 — Résultats et récompenses (Priority: P2)

**Goal** : à la clôture, classement figé, auteurs révélés, récompenses versées une fois.

**Independent Test** : [quickstart.md, scénario 6](./quickstart.md).

- [X] T061 [US7] Écrire `etablir_resultats(conn, concours)` dans `uafricas_backend/src/services/jeu_concours.rs` (research D8), appelée par `resoudre_concours` (remplace l'appel vide de T042) :
  - une seule requête d'agrégation sur les confrontations **comptées** et votées, sans celles qui impliquent une participation retirée ou suspendue, ni celles dont le votant est suspendu ;
  - calcul de `taux` et de `sous_seuil` (< `presentations_min`) ;
  - tri ; ex aequo à 0,1 point près (même rang) ; podium du jury s'il existe (`place_jury`, en tête) ;
  - insertion dans `jeu.resultat_concours`, `etat = 'resultats'`, `resultats_at` ;
  - **dans la même transaction**, `crediter` (origine `concours`) : prime de participation pour chaque publiée (clé PC5), prime de podium selon le rang (`prime_podium` du concours ou des règles ; les ex aequo touchent la prime de leur rang).
- [X] T062 [US7] Après le COMMIT, dans `uafricas_backend/src/services/jeu_concours.rs` :
  - `engagement::attribuer` pour `jeu_concours_participation`, `jeu_concours_podium` et `jeu_concours_vote` (votants avec au moins 10 voix comptées), avec les clés de data-model §10 ;
  - notifications `jeu.concours_resultats` (participants) et `jeu.concours_laureat` (podium).

  Ajouter les 3 codes à `ACTIONS_INSTRUMENTEES` dans `uafricas_backend/src/handlers/admin/engagement.rs`.
- [X] T063 [P] [US7] Créer `uafricas_frontend/app/components/jeu/PodiumConcours.vue` (trois premières places, auteur, taux de victoire en %, distinction) et passer `uafricas_frontend/app/components/jeu/GalerieConcours.vue` en mode classé (rang, taux, duels, « hors classement » pour les participations sous le seuil). Brancher sur la phase `resultats` de `uafricas_frontend/app/pages/activites/concours/[id].vue`.
- [X] T064 [US7] Dans `uafricas_frontend/app/pages/mon-compte/activites.vue`, afficher pour chaque participation le rang, le taux et la récompense une fois les résultats établis.
- [X] T065 [US7] Dérouler [quickstart.md, scénario 6](./quickstart.md) :
  - 1 000 relectures, un seul versement ;
  - **comparaison de `engagement.compte` avant et après (SC-013)** ;
  - ex aequo forcés en SQL.

**Checkpoint** : premier concours complet de bout en bout (MVP de la famille B).

---

## Phase 10: User Story 8 — Jury (Priority: P3)

**Goal** : le jury fixe le podium parmi les finalistes ; repli sur le vote passé le délai.

**Independent Test** : [quickstart.md, scénario 7](./quickstart.md).

- [X] T066 [US8] Dans `uafricas_backend/src/handlers/admin/jeu_concours.rs`, ajouter :
  - `finalistes`, en phase `deliberation` : classement provisoire calculé comme `etablir_resultats` **sans écrire**, puis les `jury_finalistes` premiers hors participations sous le seuil ;
  - `deliberer` : 1 à 3 participations distinctes parmi les finalistes ; écrit `podium_jury`, `delibere_par` et `delibere_at` ; audit `JURY_DELIBERATION`.

  Factoriser le calcul du classement dans `services/jeu_concours.rs` pour que `finalistes` et `etablir_resultats` partagent **la même** fonction.
- [X] T067 [P] [US8] Ajouter l'onglet « Jury » à `uafricas_frontend/app/pages/admin/activites/concours/[id].vue` : finalistes en grand, choix des 3 places, confirmation.
- [X] T068 [US8] Dérouler [quickstart.md, scénario 7](./quickstart.md), dont le repli sur le classement communautaire une fois `jury_delai_jours` passé (dates en SQL).

---

## Phase 11: User Story 9 — Protéger le vote et les photos (Priority: P3)

**Goal** : écarter la fraude organisée et retirer le contenu inapproprié.

**Independent Test** : [quickstart.md, scénario 8](./quickstart.md).

- [X] T069 [US9] Dans `uafricas_backend/src/handlers/admin/jeu_concours.rs`, ajouter `suivi` ([api-admin.md §4](./contracts/api-admin.md)) : volumes par motif d'écart, votants, présentations (min, max, moyenne), classement provisoire (même fonction que T066), et `comptes_signales` calculés à la lecture (plus de 30 votes en une minute, plus de 200 votes, préférence de 90 % ou plus sur au moins 10 confrontations). Ajouter aussi `ecarter_votant` et `retablir_votant` (409 après les résultats ; audits `VOIX_ECARTEES` et `VOIX_RETABLIES`).
- [X] T070 [US9] Ajouter `signaler` à `uafricas_backend/src/handlers/jeu_concours.rs` :
  - une fois par membre (`ON CONFLICT DO NOTHING`), pas sur sa propre participation ;
  - recompte, puis `suspendue` **au-delà de 10** (même constante et même comparateur que `SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA`) ;
  - notification `jeu.participation_suspendue` à l'auteur.

  `retablir` (T043) remet `nombre_signalements` à 0.
- [X] T071 [P] [US9] Créer `uafricas_frontend/app/components/jeu/SignalerParticipation.vue` (motif, confirmation, Tailwind pur) et le monter sur la galerie et la confrontation.
- [X] T072 [P] [US9] Ajouter les onglets « Suivi du vote » (chiffres, comptes signalés, écarter ou rétablir les voix) et « Participations » (avec signalements) à `uafricas_frontend/app/pages/admin/activites/concours/[id].vue`.
- [X] T073 [US9] Rendre visible dans le règlement public de chaque concours (`uafricas_frontend/app/pages/activites/concours/[id].vue`) les règles d'admissibilité des voix (FR-040), en texte fixe ajouté au règlement saisi.
- [X] T074 [US9] Dérouler [quickstart.md, scénario 8](./quickstart.md).

---

## Phase 12: Polish & Cross-Cutting Concerns

- [X] T075 Relire les points de conception PC1 à PC5. PC1 : lister toutes les routes qui lisent un concours, et vérifier que chacune appelle `resoudre_concours`. PC2 : vérifier toutes les lectures de `EpreuveRow`. PC4 : vérifier sur 20 épreuves saisies et 20 dérivées que la solution n'est pas la permutation identité **pour toutes**.
- [X] T076 [P] Parcourir `/activites/concours/**` et les nouvelles épreuves à 375, 768 et 1280 px : aucun débordement, carte et paires utilisables au doigt. Aucune classe daisyUI hors `/admin/**`.
- [X] T077 Lancer `cargo build` et `pnpm build` ; aucun avertissement dans les fichiers touchés ; diagnostics VS Code propres.
- [X] T078 Critères de sortie de [quickstart.md](./quickstart.md) : migration rejouée, aucune clé d'idempotence en double, agrégats égaux au journal, non-régression de la 013 (partie, défi, duel différé et direct, Championship sur des épreuves `choix`).
- [X] T079 Mettre à jour `CLAUDE.md` : composables `useConcours` et `useAdminConcours`, seed `94_seed_local_jeu_types.sql` dans le tableau des seeds, et **une ligne** d'index « Recent Changes » citant `38_jeu_concours.sql`.
- [X] T080 Rédiger `specs/014-jeux-concours/pr-description.md` : contenu, migration `38` **avant** le code (elle suppose `37`), dérivation et revue des nouvelles formes, premier concours, hypothèses H1 à H4 à faire confirmer par le client.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)** : aucune dépendance, sinon la 013.
- **Foundational (Phase 2)** : après le Setup. Bloque US1, US2 et US3.
- **US1 (Phase 3)** : après la Phase 2. Le palier 2 (T021 à T024) suit le palier 1.
- **US2 (Phase 4)** et **US3 (Phase 5)** : après la Phase 2. Indépendantes de US1 côté backend, mais US3 s'appuie sur le formulaire étendu par US2 (T027 avant T033).
- **US6 (Phase 6)** : après le Setup seulement (tables de concours). **Indépendante de la famille A** : elle peut démarrer en parallèle des phases 3 à 5.
- **US4 (Phase 7)** : après US6.
- **US5 (Phase 8)** : après US4.
- **US7 (Phase 9)** : après US5.
- **US8 (Phase 10)** et **US9 (Phase 11)** : après US7, indépendantes entre elles.
- **Polish** : en dernier.

### User Story Dependencies

```
Setup ─▶ Foundational ─▶ US1 ─▶ (palier 2)
                     ├─▶ US2 ─▶ US3
Setup ─▶ US6 ─▶ US4 ─▶ US5 ─▶ US7 ─┬─▶ US8
                                    └─▶ US9
```

### Parallel Opportunities

- Phase 2 : T012 (front) en parallèle de T005 à T011 (back).
- US1 : T014, T015, T016 et T017 en parallèle (fichiers distincts), puis T018 qui les assemble.
- US6 : T040, T041, T045 et T046 en parallèle.
- Les deux familles en parallèle : US6, US4, US5 et US7 n'utilisent rien des phases 2 à 5.

## Parallel Example: User Story 1

```text
T014 Seed des épreuves carte, ordre et paires         (SQL)
T015 ReponseCarte.vue                                 (Vue)
T016 ReponseOrdre.vue                                 (Vue)
T017 ReponsePaires.vue                                (Vue)
→ puis T018 (aiguillage dans CarteEpreuve) et T019 (page de partie)
```

## Implementation Strategy

### MVP First

1. Phases 1 et 2, puis Phase 3 palier 1 (T014 à T020) : **les nouveaux types se jouent en partie libre**. C'est la réponse la plus visible à « les jeux sont trop uniformes ».
2. Démontrer, puis enchaîner le palier 2 (duels et défis).

### Incremental Delivery

| Palier | Phases | Démontrable |
|---|---|---|
| 1 | 1, 2, 3 (T014–T020) | carte, ordre, paires en partie libre |
| 2 | 3 (T021–T024) | nouveaux types en défi et en duel |
| 3 | 4, 5 | photo et son, vivier dérivé |
| 4 | 6, 7 | concours ouvert, dépôts modérés |
| 5 | 8, 9 | premier concours complet, récompenses |
| 6 | 10, 11, 12 | jury, anti-fraude, finitions |

---

## Notes

- Requêtes sqlx vérifiées à l'exécution : chaque requête neuve passe au moins une fois en recette.
- Après l'ajout d'un composable, redémarrer le serveur Nuxt.
- Rien n'est commité automatiquement ; proposer « commite et pousse » à chaque palier.
