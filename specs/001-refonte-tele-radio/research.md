# Phase 0 : Recherche : Refonte des pages Télé et Radio Africans

**Feature** : `001-refonte-tele-radio` | **Date** : 2026-07-19 | **Spec** : [spec.md](./spec.md)

Toutes les inconnues techniques de la Technical Context sont résolues ci-dessous. Chaque décision est
adossée à un précédent vérifié dans le dépôt (chemin + ligne), conformément au Principe V (ne pas
réinventer) et au Principe III (le SQL fait foi).

---

## Constat liminaire : l'état des lieux invalide trois suppositions courantes

Avant les décisions, trois faits établis par l'exploration, qui changent la nature du travail :

1. **Il n'y a pas de contrôle d'accès à la publication.** `POST /api/stations-radio`
   (`handlers/stations_radio.rs:263`), `POST /api/television/chaines` (`handlers/television.rs:207`) et
   `POST /api/television/programmes-vedettes` (`handlers/television.rs:428`) insèrent `etat = 'publie'`
   **en dur**, derrière un simple `extraire_utilisateur_id`. Tout membre connecté publie sans validation.
   L'US4 n'est donc pas un ajout : c'est la fermeture d'une faille ouverte.
2. **La page Télé contient un bug latent masqué.** `tele.vue:141` injecte `programmeActif.videoUrl` dans
   `<video :src>` (`:219`), alors que les données peuvent contenir des URL YouTube `watch`
   (`mocks/tele.ts:45,54,63,72`) qu'une balise `<video>` ne sait pas lire. L'iframe provisoire en `z-30`
   (`tele.vue:204`) recouvre l'écran et masque le défaut. Retirer le provisoire (FR-010) **expose le bug** :
   le routage média de FR-056 n'est pas une option, c'est un prérequis.
3. **Le filtre `type_station` n'est pas cassé.** `mapper_type_station_db` (`models/station_radio.rs:109-116`)
   fait bien l'aller-retour `Nationales` ↔ `nationale`. Le défaut est ailleurs : `africans.vue:28` initialise
   le filtre à `'Tous les types'`, sentinelle que le composable retire avant l'appel
   (`useStationsRadio.ts:137`). Radio Africans n'envoie donc **aucun** critère. Il ne faut pas réparer un
   mapping, il faut introduire un axe de répartition qui n'existe pas.

---

## R1 : Porter la distinction Radio Africans / Radio Nationales

**Décision** : ajouter une colonne dédiée `origine_publication VARCHAR(20) NOT NULL DEFAULT 'territoire'`
sur `media_content.station_radio`, avec `CHECK (origine_publication IN ('africans','territoire'))` et un
index partiel `WHERE deleted_at IS NULL`.

**Rationale** : FR-014 répartit **les stations** selon une **origine éditoriale**. Aucun porteur n'existe :
`station_radio` n'a ni booléen, ni catégorie d'origine, et `cree_par` n'est pas exploitable comme proxy
puisque la route publique de création accepte tout membre. Le `VARCHAR + CHECK` est le patron des
migrations incrémentales du projet (`10h:16-24`, `10i:11-29`) ; il évite le piège `ALTER TYPE ADD VALUE`
inutilisable dans la même transaction (documenté `05c:10`) et reste idempotent. Le défaut `'territoire'`
qualifie l'existant du bon côté : les soumissions membres portent sur des stations de territoire (FR-036),
la bannière Africans étant une décision éditoriale.

**Alternatives écartées** :
- *Réutiliser `type_station`* (`nationale|locale|internationale`) : il exprime une **portée géographique**,
  axe orthogonal à l'origine. Les fusionner rendrait « station Africans à portée locale » inexprimable et
  casserait le filtre de portée que les deux pages proposent déjà (`africans.vue:35`, `nationales.vue:35`).
- *Réutiliser `categorie_radio`* (`radio_africans_*` / `radio_nationale_*`), porte la bonne sémantique mais
  sur `programme_radio` (l'émission), pas sur la station, et croise déjà les deux axes. Le déplacer créerait
  deux sources de vérité contradictoires (`radio_africans_local` vs `type_station='locale'`) sans contrainte
  pour les tenir cohérentes. `categorie_radio` reste inchangé, utile en back-office
  (`admin/radio_tele.rs:802-805`).

**Effet de bord retenu** : profiter de la migration pour poser les FK manquantes de `station_radio`
(`pays_id`, `cree_par`), la table n'en a **aucune** aujourd'hui, contrairement à `programme_radio`
(`09g:87-119`).

---

## R2 : Porter la vedette générale de la page Télé

**Décision** : colonne `a_la_une_globale BOOLEAN NOT NULL DEFAULT FALSE` sur `media_content.programme_tele`,
assortie d'un index unique partiel garantissant l'unicité **à l'échelle de la table** :

```sql
CREATE UNIQUE INDEX uq_programme_tele_a_la_une_globale
    ON media_content.programme_tele ((TRUE))
    WHERE a_la_une_globale = TRUE AND deleted_at IS NULL;
```

**Rationale** : FR-001 exige **une** vedette pour toute la page, distincte des mises en avant par chaîne.
Le projet porte déjà l'exclusivité par chaîne avec exactement ce mécanisme, 
`uq_programme_tele_a_la_une_par_chaine` (`09g:82-84`) : l'index unique partiel sur une expression constante
étend le même patron à une portée globale, sans table de configuration ni singleton applicatif.

**Alternatives écartées** :
- *Table de configuration `media_content.mise_en_avant`*, une table pour une ligne ; sur-ingénierie
  contraire au Principe V.
- *Réutiliser `a_la_une` avec `chaine_id IS NULL`*, collision sémantique avec les programmes orphelins,
  qu'aucune contrainte n'empêche aujourd'hui.

**Point d'attention** : l'exclusivité applicative existante (bascule de l'ancien à `FALSE` avant l'INSERT,
`admin/radio_tele.rs:1256-1265`) est **hors transaction**. Le nouveau code doit envelopper les deux requêtes
dans une transaction, sans quoi l'index unique fera échouer la seconde en concurrence.

---

## R3 : Réactions, commentaires, partages et signalements sur les entités médias

**Décision** : créer **des tables dédiées dans `media_content`**, avec un discriminant local
`type_media VARCHAR(20) + CHECK IN ('chaine_tv','station_radio','programme_tele','programme_radio')`, 
et non étendre `country_profile.reaction_element` / `partage_element`.

**Rationale** : trois blocages factuels, pas une préférence de style :

1. **Le « générique » est câblé sur `country_profile`.** `element_social.rs:282-296` fait un
   `JOIN country_profile.fiche_pays fp ON fp.id = el.fiche_pays_id` **inconditionnel**, et filtre
   `fp.bloquee = FALSE`. Les quatre tables médias n'ont pas de `fiche_pays_id` (elles ont `pays_id`
   directement, `09_media_content.sql:31` et `:236`). Le SQL ne compilerait pas.
2. **`verifier_element` (`element_social.rs:53-55`) lit `SELECT suspendu FROM {table}`.** Les tables médias
   n'ont pas de colonne `suspendu` : elles portent `etat VARCHAR(50) CHECK IN
   ('brouillon','publie','suspendu','supprime')`. Le prédicat de suspension diffère structurellement.
3. **`ALTER TYPE country_profile.type_objet_contribution ADD VALUE 'chaine_tv'`** est sémantiquement absurde
   et pollue durablement `signalement_contribution` (`11j:43`) et `contribution_fiche` (`11c:89`), qui
   consomment le même enum.

**L'économie espérée n'existe pas.** Le mur `/publications` n'agrège pas en SQL : il fait **7 appels HTTP
parallèles** (`publications/index.vue:742-878`) et discrimine par `source` dans un `v-else-if` (`:150-175`).
Les 6 modifications frontend (type, filtre, styles, compteurs, chargement, carte) sont dues **quelle que
soit** l'option backend. Étendre le générique n'économiserait que ~15 lignes de DDL, au prix d'un refactor
à risque sur du code en production servant 4 types afripulse.

**Suspension** : ne pas ajouter de colonne `suspendu` aux tables médias : elles ont `etat`. Le handler doit
basculer `etat = 'suspendu'` (patron factcheck, `gouvernance.rs:769`). Ajouter en revanche
`nombre_signalements INT NOT NULL DEFAULT 0`, comme partout ailleurs.

**Seuil** : `SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA = 10`, comparateur `>`, aligné sur les deux mécanismes les
plus récents (`contribution_signalement.rs:23,131` ; `session_signalement.rs:23,135`) et sur H-007. Le
codebase mélange `>` et `>=` : trancher explicitement évite de reproduire l'incohérence.

**Commentaires** : aucun socle générique n'existe, les quatre implémentations
(`iam.biblio_commentaire`, `culture.codimoi_commentaire`, `governance.factcheck_commentaire`,
`country_profile.recommandation_visiteur`) sont toutes dédiées. Partir de `iam.biblio_commentaire`
(`04g:41-55`, liste plate) plutôt que des variantes arborescentes : FR-024 ne demande pas de fil de réponses.
Aucun précédent de modération de commentaires n'existe dans le projet ; le signalement porte sur le contenu
média, pas sur les commentaires.

---

## R4 : Référentiel des 43 thèmes phares

**Décision** : table de référence + couple `theme_phare_id UUID` / `theme_phare_autre VARCHAR(200)` sur
`programme_tele` et `programme_radio`. Réutiliser `shared.categorie` avec `contexte = 'media'`
(`03_shared.sql:42-60`) plutôt que créer une table.

**Rationale** : 43 valeurs à libellés riches et accentués (« Éducation, Les carrés de l'instruction en
Afrique ») sont hors de proportion avec les enums du projet, qui plafonnent à 8 valeurs
(`categorie_chaine_tv`) et sont tous en `snake_case` ASCII, un enum imposerait 43 lignes de mapping Rust
bidirectionnel. `shared.categorie` prévoit nommément le contexte `'radio'` (commentaire `03_shared.sql:47`),
possède déjà `idx_categorie_contexte`, `ordre` et `actif`, et est en production sur `marketplace.annonce`
et `media_content.livre`.

Le couple `<ref>_id` + `<ref>_autre` reprend littéralement `secteur_id` / `secteur_autre` de
`05c:16-17` : le patron maison pour « liste de référence + option Autre », qui est exactement ce que
demande FR-030. Ajouter un thème devient un `INSERT … ON CONFLICT (slug) DO NOTHING` (`05c:25-38`),
sans DDL.

**Contre-exemple qui tranche** : `10i:11-29`, ajouter 7 catégories a obligé à `DROP CONSTRAINT` puis
réécrire les 14 valeurs d'un CHECK. C'est le coût de maintenance à éviter sur une liste vouée à s'allonger.

**Rôle de partie prenante** (9 valeurs, stable, structurant) : à l'inverse, un `VARCHAR + CHECK` suffit,
avec le même couple `role_partie_prenante` / `role_partie_prenante_autre`.

---

## R5 : Workflow de soumission et de modération

**Décision** : une table polymorphe unique `media_content.proposition_media`, bâtie sur l'ossature
d'`afrolang.proposition_salle` (`08b:359-403`) et le polymorphisme de `country_profile.contribution_fiche`
(`11c:86-113`).

**Rationale** : comparaison des trois patrons disponibles :

| Critère | Vidafrica (27*) | contribution_fiche (11b/11c) | proposition_salle (08b) |
|---|---|---|---|
| Motif de refus stocké | **non** | oui | oui (+ CHECK) |
| Traçabilité modérateur | **non** | `traite_par`/`traite_at` | `decideur`/`decide_at` |
| Endpoint « mes soumissions » | **non** | oui | oui |
| Retrait par l'auteur | non | non | **oui** |
| Notification auteur | **non** | oui (in-tx) | oui |
| Polymorphe | non | **oui** | non |
| Intégrité garantie en SQL | faible | moyenne | **forte (4 CHECK)** |

Vidafrica est **muet côté contributeur** : le membre propose une vidéo, reçoit `{"etat":"brouillon"}` et
elle disparaît de sa vue : aucun endpoint « mes vidéos », aucun motif, aucune notification. Reproduire ce
patron violerait FR-034. `contribution_fiche` est couplé à `fiche_pays_id NOT NULL` et à une sémantique
d'édition de champ. `proposition_salle` apporte ce qui manque : quatre `CHECK` rendent le workflow
**inviolable au niveau SQL** : impossible de valider sans créer l'objet, impossible de rejeter sans motif.

Le polymorphisme `(type_objet, target_id, donnees JSONB)` permet **une seule file d'attente admin** et
**un seul composable membre** pour chaîne / station / programme télé / programme radio, et absorbe
ensuite la demande d'animation de programme (FR-045) par simple ajout d'une valeur d'enum, sans quatrième
table.

**Notification** : suivre le style **transactionnel** d'`admin/profils_pays.rs:2545-2559` plutôt que le
fire-and-forget d'`admin/propositions_salle.rs:369`, une décision de publication ne doit pas pouvoir être
commitée sans que l'auteur en soit averti.

**Correction indissociable** : basculer les trois INSERT `'publie'` en dur (R-constat liminaire n°1). Sans
cela, la modération reste contournable par les anciennes routes.

---

## R6 : Co-détention d'une chaîne ou d'une station

**Décision** : `media_content.support_detenteur`, calquée sur `afrolang.salle_moderateur` (`08b:224-242`),
enrichie d'une colonne `role` ; plus `media_content.invitation_detenteur`, calquée sur
`arbre_genealogique.invitations` (`25:16-38`).

**Rationale** : `salle_moderateur` fournit exactement la bonne forme, `UNIQUE (objet, utilisateur)` sans
filtre (jamais de doublon historique), `actif` + `retire_at` (retrait en soft-delete réversible, ajout en
upsert-réactivation), `designe_par` + `designe_at` (traçabilité, indispensable pour FR-055). Sa logique
d'ajout à trois branches (`admin/moderateurs_afrolang.rs:59-190`) est copiable telle quelle.

Il lui manque deux choses : le **rôle** (afrolang est mono-rôle) et un chemin d'**invitation par le
propriétaire** (afrolang ne connaît que la désignation par un admin). FR-045 exige que l'acceptation d'une
demande d'animation ajoute le demandeur aux co-détenteurs : d'où l'emprunt du modèle d'invitation de l'arbre
généalogique, qui apporte l'expiration à 30 jours et l'acceptation explicite.

**Alternatives écartées** :
- *`iam.permission_specifique`* (`04_iam.sql:126-138`), dispositif générique row-level, mais **table morte** :
  aucune référence dans `src/`, et le middleware ne la lit pas (`middleware/admin.rs:108-125`). Elle est de
  surcroît réservée aux admins, alors que les co-détenteurs sont des membres.
- *`afrolang.acces_salle_privee`* : droit binaire sans rôle, hors sujet.

**Garde d'autorisation** : ne **pas** utiliser l'extracteur `AdminUtilisateur`, qui rejette tout non-admin
(`middleware/admin.rs:100-105`). Écrire un helper `garde_detenteur(pool, type_support, support_id, moi,
roles_admis)` sur le modèle de `garde_proprietaire` (`handlers/annonces.rs:111`).

---

## R7 : Grille de programmation : worker ou résolution paresseuse ?

**Décision** : **résolution paresseuse en SQL à la lecture**, sans tâche de fond. Le créneau courant est
calculé à chaque requête à partir de la grille et de `NOW()`.

**Rationale** : le projet n'a **aucun scheduler**, `main.rs` ne contient aucun `spawn` périodique,
`Cargo.toml` aucune dépendance de planification. Tous les `tokio::spawn` existants sont du fire-and-forget
ponctuel (envois d'e-mails, keep-alive SSE). À l'inverse, l'écoulement du temps est déjà géré paresseusement
en SQL à deux endroits : `rendez_vous.rs:184,190`
(`rv.date_heure + make_interval(mins => rv.duree_minutes::int) >= NOW()`) et `afrolang.rs:422,518,689`
(`(demande_passation_at + interval '60 seconds') <= NOW()`). `CLAUDE.md:121` nomme explicitement ce patron
« résolution paresseuse ».

Cette approche satisfait SC-010 : le contenu prévu est servi dès la première requête suivant l'échéance,
donc à moins d'une minute pour une page consultée. Elle évite une dépendance nouvelle, un processus à
superviser en production et toute dérive d'état entre worker et base.

**Fuseau** : le projet impose `TIMESTAMPTZ` partout (`schema.sql:32`) et n'a **aucun** `TIME` nu. La grille
introduit nécessairement heure-du-jour + jour-de-semaine ; FR-042 exige un référentiel explicite. Décision :
stocker `heure_debut TIME`, `duree_minutes INT`, `jour_semaine SMALLINT NULL` (NULL = quotidien) et
`fuseau VARCHAR(60) NOT NULL DEFAULT 'Africa/Abidjan'` (UTC+0, sans heure d'été), la résolution se faisant
via `(NOW() AT TIME ZONE fuseau)`.

**Détection de chevauchement (FR-040)** : vérification applicative en transaction, précédée d'un **verrou
sur la ligne du support parent** (`SELECT id FROM media_content.chaine_tv WHERE id = $1 FOR UPDATE`). Ce
verrou sérialise toutes les modifications de grille d'un même support et couvre l'edge case
« co-détenteurs en concurrence », y compris les insertions concurrentes qu'un `FOR UPDATE` sur les créneaux
existants ne verrouillerait pas : le patron est déjà employé par `valider_proposition`
(`admin/propositions_salle.rs:204`).

*Une contrainte d'exclusion GiST serait plus élégante, mais imposerait l'extension `btree_gist`, un type
range sur `TIME` et le traitement du franchissement de minuit, pour une garantie qu'un verrou de ligne
apporte déjà à cette volumétrie. Le projet n'utilise aucune contrainte d'exclusion aujourd'hui.*

---

## R8 : Barre de lecture audio persistante

**Décision** : monter le lecteur **une seule fois dans `app/layouts/default.vue`**, hors du `<slot/>`, avec
un état partagé par `useState`. Réécrire `useAudioPlayer.ts`.

**Rationale** : le mécanisme de persistance du projet est la **persistance du layout Nuxt**, pas Teleport.
`layouts/default.vue:5-7` place `<slot/>` dans `<main>` : à la navigation, Nuxt ne détruit pas l'instance du
layout, seul le contenu du slot est échangé. C'est ainsi que `SocialMessagerieFlottante` survit
(`default.vue:11-13`), avec état en `useState` (`useAppels.ts:57-65`, `useMessagerie.ts:63-70`).
`AfrolangParticipantAudio.vue:1-6` énonce la règle applicable mot pour mot : découpler l'audio du layout
« évite que le son ne se coupe lors des changements d'affichage ».

`useAudioPlayer.ts` existant est **du code mort** (aucun import dans `app/`) et structurellement inadapté :
`ref()` locaux sans partage, stations passées en **argument tableau non réactif** (figé avant le chargement
async), et surtout `onUnmounted` qui fait `pause()` + `src = ''` (`:128-133`), activement hostile à la
persistance. Il importe de surcroît `RadioStation` depuis `~/mocks/radios` alors que les pages utilisent le
type du composable.

**Contrainte de placement** : aucun `fixed bottom-0` n'existe dans le codebase : c'est un pattern neuf. Le
FAB messagerie occupe `fixed bottom-6 right-6` (z-50) et l'invite d'appel `fixed bottom-24 right-6`
(z-[75]). La barre pleine largeur les recouvrirait : prévoir un décalage vertical de ces éléments quand la
barre est active, et un z-index sous l'invite d'appel.

---

## R9 : Vedette plein écran et barre de navigation

**Décision** : réserver la hauteur de la barre par `pt-24` / `top-24` sur le contenu, la vedette occupant
`h-[100svh]`. Aucune marge négative.

**Rationale** : contre-intuitivement, `NavBar.vue:2` est `absolute`, **ni `fixed` ni `sticky`**, et aucun
ancêtre n'est positionné (`app.vue:2`, `default.vue:2`). Elle est donc ancrée au document et **défile hors
de l'écran**. Elle est hors-flux et recouvre `<main>` : chaque page gère son propre offset haut. La
convention dominante est `pt-28` (8 pages publiques) ; `tele.vue` utilise déjà `top-24` / `mt-24` / `top-32`.
Aucun `-mt-*` n'existe dans le projet.

`100svh` plutôt que `100vh` : sur mobile, `100vh` déborde sous la barre d'URL des navigateurs, ce qui
contredirait SC-013. FR-011 impose par ailleurs de servir la vedette sur mobile, le `v-if="!isMobile"`
actuel (`tele.vue:216`) qui prive les visiteurs mobiles de tout hero doit disparaître.

---

## R10 : Rangée horizontale défilante

**Décision** : composant maison en Tailwind pur, sur la base
`flex flex-nowrap gap-4 overflow-x-auto scrollbar-none snap-x snap-mandatory -mx-1 px-1`, chaque carte en
`shrink-0 snap-start`.

**Rationale** : aucun composant de rangée catalogue n'existe et `snap-x` n'apparaît nulle part. En revanche
l'utilitaire `scrollbar-none` est déjà défini (`main.css:77-83`) et le patron de rangée scrollable est
établi à l'identique en deux endroits (`africantives/index.vue:69`, `bibliotheque/humaine.vue:118`), y
compris l'astuce `-mx-1 px-1` qui évite de rogner les anneaux de focus.

`vue3-carousel` (utilisé par `CentreCulturelCarousel.vue`) est écarté : c'est un carrousel
une-slide-à-la-fois (`itemsToShow: 1`, `autoplay`, `wrapAround`), pas une rangée catalogue, et il
introduirait une dépendance CSS tierce sur des pages publiques tenues au Tailwind pur (Principe VI).

---

## R11 : Routage média : fichier hébergé vs lien externe

**Décision** : un composant lecteur unique qui **route selon la source**, `youtubeEmbedUrl(url)` non nul →
`<iframe>` ; sinon `<video>` / `<audio>` natif. Repli explicite si aucun des deux ne s'applique.

**Rationale** : FR-056 autorise les deux sans restriction. La fonction de conversion **existe déjà**, 
`youtubeEmbedUrl` (`useEvenements.ts:285-301`) gère `watch` / `youtu.be` / `embed` / `live` / `shorts` et
renvoie `null` si non reconnu ; elle est en production sur `evenements/[id].vue:387` et
`FormationCurriculum.vue:195`. Une regex de validation existe aussi
(`RessourceContribueeForm.vue:41`). `VidafricaLecteur.vue` ne gère que les fichiers (`<video :src>`,
`:251-265`) : il n'est pas réutilisable tel quel pour l'externe, mais sa barre de lecture (`:291-338`, avec
gestion du scrub par flag `scrubbing`) et son plein écran cross-navigateur (`:209-230`, incluant le repli
iOS `webkitEnterFullscreen`) sont à reprendre.

**Limite assumée** : `mute=1` est obligatoire pour l'autoplay d'un iframe YouTube : ce qui coïncide avec
FR-003 (son coupé par défaut). En revanche le démarrage à l'échéance exacte n'est pas garantissable sur un
lecteur tiers, ce que SC-010 acte déjà.

---

## R12 : Pages de détail et aperçus sociaux

**Décision** : créer quatre pages de détail SSR, `/medias/chaines/[slug]`, `/medias/stations/[slug]`,
`/medias/programmes-tele/[slug]`, `/medias/programmes-radio/[slug]`, au lot P2, avec le pattern
`await useAsyncData` + `useHead(() => …)`.

**Rationale** : FR-026 exige un aperçu social avec titre, description et image. Un aperçu suppose une **URL
propre au contenu**, or aucune page de détail média n'existe : tout passe par des modales. Les colonnes
`slug VARCHAR(400) UNIQUE` existent déjà sur les quatre tables, les routes sont donc créables sans
migration. Le pattern OG du projet est établi et **rendu côté serveur**, 
`opportunite-afrique/[id]/sites/[siteId].vue:297-370` : `await useAsyncData` au niveau racine, puis
`useHead()` avec fonction réactive produisant `og:type/title/description/url/image` + `twitter:card` +
`canonical`. Il est répliqué à l'identique sur 6 pages.

**Régression à ne pas reproduire** : `vidafrica/[slug].vue` n'a **aucune** balise Open Graph alors que le
partage de vidéos y est implémenté : l'aperçu social y est donc cassé.

**Conséquence de périmètre** : ces pages ne figurent pas dans la spec. Elles sont un **prérequis technique
de l'US3**, pas une extension du besoin, et restent hors du MVP P1.

---

## R13 : Chargement différé des médias

**Décision** : implémenter un `useObservateurVisibilite` (IntersectionObserver) et ne monter le lecteur
d'une section qu'à son entrée dans le viewport ; vignettes en `loading="lazy"`, vidéos en `preload="none"`.

**Rationale** : SC-011 et FR-054 imposent de ne pas précharger 50 sections. Aucun `IntersectionObserver`
n'existe dans `app/` et `useAOS` ne fait qu'initialiser la bibliothèque d'animation AOS : il ne diffère
aucun chargement. C'est donc un pattern neuf, mais sans conflit avec l'existant. `preload="none"` est déjà
la valeur retenue sur `tele.vue:225`.

---

## R14 : Notifications au contributeur

**Décision** : réutiliser `arbre_genealogique.notifications` via `models::notification::creer_notification`,
en ajoutant un `pub mod media { … }` de constantes. Élargir au préalable `type VARCHAR(30)` → `VARCHAR(80)`.

**Rationale** : le mécanisme est complet et transverse malgré le nom de son schéma, table
(`26_notifications.sql:5-21`), helper d'émission (`models/notification.rs:83-101`), 4 endpoints
(`routes.rs:887-890`), composable `useNotifications.ts`. Les constantes de type sont centralisées
(`models/notification.rs:15-46`) précisément « pour éviter les typos entre handlers producteurs et UI
consommatrice ».

**Défaut préexistant à corriger** : `type VARCHAR(30)` est déjà dépassé par les constantes actuelles, 
`"afrolang.accompagnateur.recommandation_recue"` fait 43 caractères. Tout type média
(`"media.proposition_validee"`, 25 car.) frôle la limite. L'élargissement est un prérequis, pas un confort.

---

## R15 : Permission administrative

**Décision** : `verifier_permission!(admin, "media", <action>)`, et **seeder les permissions manquantes**.

**Rationale** : deux ressources voisines coexistent et sont facilement confondues, `"media"` couvre
radio/télé (`admin/radio_tele.rs`, 21 occurrences) tandis que `"media_content"` couvre vidafrica
(`admin/vidafrica.rs`, 17 occurrences). `"programme"` désigne les programmes d'échange, pas les programmes
radio/TV.

**Défaut préexistant** : `15_seed.sql:55-99` ne contient **aucune** ligne pour `media` : il déclare
`('Gérer les radios/télés','radio_tele.gerer','radio_tele','gerer')`, couple **jamais interrogé** par le
code. En pratique, seul `super_admin` (wildcard `all.all` → `('*','*')`) franchit les gardes ; tout rôle
`admin` non-super est bloqué. La migration doit insérer `('media','voir'|'modifier'|'supprimer')` et les
lier au rôle `admin`, sans quoi la file de modération sera inaccessible aux administrateurs ordinaires.

---

## R16 : Recherche de réalisateurs et producteurs

**Décision** : ajouter « Réalisateur », « Producteur » (et métiers voisins) à `iam.specialite_bibliotheque`
(`04_iam.sql:63-67`), et ajouter un filtre `ANY(e.specialites)` à `lister_experts`.

**Rationale** : `iam.domaine_expertise` est un ENUM sans valeur audiovisuelle, le repli serait
`'autre'` + texte libre, non filtrable. `iam.expertise.specialites TEXT[]` accepte le texte libre mais
**aucun endpoint ne le filtre** : `lister_experts` (`handlers/experts.rs:46-102`) filtre `domaine`, `pays`,
`situation` et `recherche`, jamais `specialites`. Le filtre territoire existe déjà
(`LOWER(p.nom) = LOWER($n)` via `u.pays_residence_id`), ce qui couvre la moitié de FR-046.
`specialite_bibliotheque` est une table de lookup libre : ajouter un métier est un `INSERT`, sans migration
de type. L'ajout du filtre est ~8 lignes calquées sur le pattern `ANY(…)` déjà utilisé pour
`situations_professionnelles` (`:76-86`).

---

## R17 : Mise en relation avec un co-détenteur

**Décision** : dupliquer `contacter_auteur` (`handlers/annonces.rs:893`) et
`obtenir_ou_creer_conversation_annonce` (`:146-166`) avec une colonne de contexte propre aux médias.

**Rationale** : il n'existe **aucun** endpoint générique « démarrer une conversation avec le membre X ». La
règle d'accès de la messagerie (`handlers/messagerie.rs:291-302`) autorise l'envoi si amitié active **ou si
une conversation existe déjà** : le seul moyen d'ouvrir un canal vers un non-ami est donc qu'un handler
métier crée lui-même la conversation, ce que fait le Marché Africain. La contrainte
`ck_conversation_ordre (a_id < b_id)` + `uq_conversation_paire` impose de passer par le helper
`paire_canonique`.

---

## R18 : Stratégie de rendu

**Décision** : pages de liste (`/medias/tele`, `/medias/radio/*`) chargées **côté client** ;
pages de détail (R12) en **SSR**.

**Rationale** : c'est la règle implicite déjà suivie par le projet, 64 des 87 pages publiques chargent en
`onMounted` + `$fetch`, et les 9 pages en `await useAsyncData` sont **toutes** des pages de détail sensibles
au SEO. Les composables métier (`useStationsRadio.ts`, `useTelevision.ts`) sont de simples enveloppes
`$fetch` non SSR-aware ; les rendre SSR serait un chantier transverse hors périmètre. Les pages de liste
n'ont pas d'enjeu d'aperçu social, contrairement aux pages de détail dont c'est la raison d'être.

---

## Dette connexe relevée, hors périmètre mais à signaler

Ces points ne bloquent pas la feature et ne sont pas traités ici, mais sont documentés pour ne pas être
redécouverts :

- `13_contraintes_inter_schemas.sql:227-233` référence encore `media_content.programme_radio_tele`,
  supprimée par `09g:147` : l'ordre d'exécution de `schema.sql` masque le problème.
- Les commentaires de `models/television.rs` et `handlers/television.rs:245` évoquent toujours
  `programme_radio_tele`.
- `useTelevision.ts:185-192` code en dur 2 des 4 statistiques (`'24/7'`, `'HD+'`) ; `nombre_programmes` et
  `nombre_chaines_en_direct` ne sont jamais affichés.
- Deux définitions concurrentes de `RadioStation` (`mocks/radios.ts:2` vs `useStationsRadio.ts:31`) et de
  `TvChannel`/`TvProgram`, les composants `media/*` étant liés aux mocks.
- `MediaAddProgramModal` (354 lignes) est un formulaire complet **branché nulle part**, dont `handleSubmit`
  simule l'envoi par un `setTimeout` (D-006).
- `uploader_media` (`admin/radio_tele.rs:1494`) ne persiste rien dans `shared.media` : aucun ramasse-miettes
  des fichiers orphelins.
- Syntaxe de dégradé incohérente : `bg-linear-to-*` (v4) vs `bg-gradient-to-*` (v3), les pages du périmètre
  utilisent la forme v3, à migrer au passage (Principe VI).
