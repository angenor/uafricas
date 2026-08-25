# Phase 0 : Recherche et décisions techniques

**Feature**: 009-medias-programmes-episodes · **Date**: 2026-08-08

Aucun marqueur `NEEDS CLARIFICATION` ne subsistait dans la spécification : les trois décisions de
périmètre ont été tranchées par le commanditaire le 2026-08-08 (section `## Clarifications` de
`spec.md`). Cette phase résout les inconnues **techniques** que ces décisions ouvrent.

---

## R1 : Nommage des tables : `emission_*` / `episode_*`, `programme_*` supprimées

**Decision**: Créer `media_content.emission_tele` et `emission_radio` (conteneurs). Renommer le contenu
des tables `programme_tele` / `programme_radio` vers `episode_tele` / `episode_radio` en **conservant
identifiants et slugs**, puis supprimer les tables `programme_*`. L'interface publique et le back-office
affichent « Programme » pour l'émission et « Épisode » pour l'unité diffusable.

**Rationale**: Le commanditaire appelle « programme » le **groupe**, l'inverse exact du sens que porte
aujourd'hui la table `programme_tele`. Réutiliser l'identifiant `programme_tele` pour le nouveau
conteneur serait le piège le plus coûteux de cette migration : le projet interroge PostgreSQL par
`sqlx::query_as` (requêtes **runtime**, pas de `query!` vérifiée à la compilation), donc toute requête
non portée continuerait de compiler, s'exécuterait sans erreur contre une table homonyme de sens
opposé, et renverrait des résultats silencieusement faux. Avec des noms neufs, chaque référence oubliée
échoue bruyamment (`relation "media_content.programme_tele" does not exist`) au premier appel, 137
occurrences Rust et 24 fichiers frontend rendent cette garantie décisive.

Le décalage libellé↔code n'est pas une nouveauté : le projet affiche déjà « territoire » là où la base
dit `pays` (mémoire projet `terminologie_territoire`). Le principe I porte sur la langue, pas sur
l'identité stricte des termes métier et techniques.

**Alternatives considered**:
- *`ALTER TABLE programme_tele RENAME TO episode_tele` puis `CREATE TABLE programme_tele` (conteneur)* :
  colle au vocabulaire du commanditaire jusque dans le schéma, mais crée exactement le piège décrit 
  une requête non portée lit la nouvelle table vide au lieu d'échouer. Rejeté.
- *Garder `programme_tele` comme conteneur et y ajouter une table `contenu_programme_tele`* : conserve
  les identifiants sur le **conteneur**, ce qui contredit FR-051 (les interactions doivent suivre
  l'épisode) et ferait pointer les URL publiques existantes vers une page de série au lieu de la vidéo
  attendue. Rejeté.

---

## R2 : Conservation des identifiants, des slugs et des interactions

**Decision**: `episode_tele.id` et `episode_tele.slug` reprennent **à l'identique** ceux de la ligne
`programme_tele` d'origine. Les émissions reçoivent des identifiants neufs. Sur les quatre tables
d'interactions, la migration se réduit à un `UPDATE` du discriminant :
`type_media = 'programme_tele' → 'episode_tele'` (idem radio), **sans jamais toucher `media_id`**.

**Rationale**: Les tables `media_reaction`, `media_commentaire`, `partage_media` et `signalement_media`
sont polymorphes par `(type_media, media_id)` et **ne portent aucune clé étrangère** sur `media_id`
(09k, lignes 17-21). Conserver l'identifiant sur l'épisode rend donc la reprise d'interactions
triviale et satisfait FR-051 sans réécriture. Les pages `/medias/programmes-tele/[slug]` restent
valides et deviennent les pages d'épisode : FR-056 est satisfait **sans table de redirection**, ce
qu'aucune autre répartition des identifiants ne permet.

Le discriminant est un `VARCHAR(20)` + CHECK, choisi précisément pour rester extensible sans
`ALTER TYPE` (commentaire de 09k). `'emission_radio'` fait 14 caractères : la largeur suffit.

**Alternatives considered**:
- *Nouveaux identifiants pour les épisodes + table de correspondance* : impose de réécrire `media_id`
  sur quatre tables et de maintenir des redirections d'URL. Coût et surface de bug supérieurs, aucun
  gain. Rejeté.

---

## R3 : Rotation déterministe, calculée à la lecture

**Decision**: Le créneau porte une `date_effet DATE`. Le rang de l'occurrence courante se calcule en SQL
dans le fuseau du créneau, et l'épisode retenu est celui de ce rang **modulo** le nombre d'épisodes
publiés de l'émission, l'ordre étant `(ordre, created_at, id)`.

```sql
-- Rang de l'occurrence courante, dans le référentiel horaire du créneau.
-- Quotidien    : nombre de jours écoulés depuis date_effet.
-- Hebdomadaire : nombre de semaines écoulées depuis la PREMIÈRE occurrence,
--                c'est-à-dire le premier `jour_semaine` à partir de date_effet.
CASE WHEN c.recurrence = 'quotidien' THEN
       ((NOW() AT TIME ZONE c.fuseau)::date - c.date_effet)
     ELSE
       (((NOW() AT TIME ZONE c.fuseau)::date - c.date_effet
         - ((7 + c.jour_semaine - EXTRACT(DOW FROM c.date_effet)::int) % 7)) / 7)
END AS rang
```

L'épisode se sélectionne par jointure latérale, le modulo étant appliqué au numéro de ligne :

```sql
JOIN LATERAL (
    SELECT e.id, e.titre, e.slug, e.media_url, e.image_couverture_url,
           ROW_NUMBER() OVER (ORDER BY e.ordre, e.created_at, e.id) - 1 AS idx,
           COUNT(*)     OVER ()                                         AS total
      FROM media_content.episode_tele e
     WHERE e.emission_id = c.emission_id
       AND e.etat = 'publie' AND e.deleted_at IS NULL
) ep ON ep.idx = (((occ.rang % ep.total) + ep.total) % ep.total)
```

`est_rediffusion` vaut `occ.rang >= ep.total`.

**Rationale**: C'est le prolongement littéral de la résolution paresseuse déjà en production
(`media_programmation.rs:41-75`, patron cité de `rendez_vous.rs`) : aucune tâche de fond, aucune
occurrence matérialisée, aucun état à maintenir. Le déterminisme exigé par FR-017 découle du fait que
**rien n'est stocké** : deux lectures d'une même occurrence recalculent le même rang. La `JOIN LATERAL`
étant intérieure, une émission sans épisode publié (`total = 0`) ne produit aucune ligne, ce qui réalise
FR-021 sans branche supplémentaire.

Le double modulo `((r % t) + t) % t` couvre un `date_effet` postérieur à aujourd'hui : l'opérateur `%`
de PostgreSQL conserve le signe du dividende, un rang négatif produirait sinon un décalage hors borne.

**Alternatives considered**:
- *Date de diffusion saisie par épisode* : c'était l'option A de la clarification Q1, écartée par le
  commanditaire. Elle imposait une saisie par épisode et laissait des trous de grille.
- *Matérialiser les occurrences dans une table* : permettrait de fixer un épisode par date, mais exige
  une tâche de fond ou une génération glissante, contraire au principe posé par 09n et au principe V.
- *Curseur `dernier_episode_diffuse` stocké sur le créneau* : rend la rotation dépendante de l'ordre des
  lectures, donc non déterministe et sensible à la concurrence. Rejeté.

---

## R4 : Origine du comptage : `date_effet DATE`

**Decision**: `date_effet DATE NOT NULL DEFAULT CURRENT_DATE`, interprétée dans le fuseau du créneau.
Les créneaux repris par la migration reçoivent la date de reprise.

**Rationale**: Le rang d'occurrence doit se calculer dans le **même référentiel local** que
`heure_debut` et `jour_semaine`. Un `TIMESTAMPTZ` réintroduirait une conversion de fuseau au moment de
la soustraction de dates et pourrait décaler le rang d'un cran selon l'instant de lecture, exactement
ce que FR-017 interdit. L'écart au principe III est de même nature que celui déjà assumé et documenté
par 09n : « une récurrence n'est pas un instant ».

---

## R5 : Thématiques multiples : table de liaison polymorphe

**Decision**: `media_content.support_thematique (type_support, support_id, categorie_id)`, avec
`categorie_id` en référence **logique** `[xref]` vers `shared.categorie` (contexte `media`), sans clé
étrangère. Unicité sur le triplet. La colonne `chaine_tv.categorie` (enum
`categorie_chaine_tv`) et `station_radio.genre` / `genres_liste` sont conservées en lecture pour la
reprise puis cessent d'être écrites.

**Rationale**: Le référentiel des 44 thèmes média existe déjà (`shared.categorie`, contexte `media`,
seedé par 09j §7) et `theme_phare_id` l'utilise déjà en `[xref]` sans FK, convention explicite du
projet. Le polymorphisme `(type_support, support_id)` est le patron maison, employé identiquement par
`support_detenteur` (09m), `creneau_programmation` (09n) et les quatre tables d'interactions (09k) : une
seule table sert la télé et la radio, une seule requête sert les deux filtres.

**Alternatives considered**:
- *Colonne `UUID[]` sur chaque support* : évite une table, indexable en GIN, mais ne permet aucune
  contrainte d'unicité par thème, complique le filtre par jointure et s'écarte du patron du projet.
  Rejeté.
- *Deux tables dédiées (`chaine_thematique`, `station_thematique`)* : duplique le schéma et les
  handlers pour une différence nulle. Rejeté.

---

## R6 : Couverture territoriale : liaison + drapeau, exclusivité garantie par trigger

**Decision**: `media_content.support_territoire (type_support, support_id, pays_id)` +
`couverture_continentale BOOLEAN NOT NULL DEFAULT FALSE` sur `chaine_tv` et `station_radio`.
L'exclusivité mutuelle (FR-034) est garantie **en base** par un trigger `BEFORE INSERT` sur
`support_territoire` qui refuse l'ajout si le support est continental, et par la suppression des lignes
de territoire lors du passage à `couverture_continentale = TRUE` dans la même transaction.

**Rationale**: Une contrainte `CHECK` ne peut pas porter sur deux tables ; sans trigger, l'exclusivité
resterait une convention applicative : or le projet privilégie systématiquement les invariants
exprimés en SQL (cinq CHECK sur `transaction_cadeau`, quatre sur `proposition_media`). Le trigger est
la seule expression SQL possible de cet invariant, et il reste peu coûteux : il ne se déclenche qu'à
l'écriture d'un territoire.

Le filtre par territoire devient `couverture_continentale = TRUE OR EXISTS (…)`, ce qui réalise FR-036
en une clause.

**Alternatives considered**:
- *Territoire « Afrique » factice dans `shared.pays`* : polluerait le référentiel utilisé par tout le
  reste de la plateforme (annuaire, arbre, marché). Rejeté.
- *Exclusivité laissée au handler* : contraire à la pratique établie du projet, et un import ou un
  correctif SQL manuel la contournerait sans bruit. Rejeté.

---

## R7 : Modération des épisodes : état sur l'objet, pas proposition JSONB

**Decision**: L'épisode versé par un co-détenteur est **créé réellement** avec `etat = 'en_attente'`.
L'état gagne la valeur `'rejete'` et l'épisode une colonne `motif_rejet TEXT`. La file admin lit
directement `episode_* WHERE etat = 'en_attente'`. La table `proposition_media` reste réservée aux
**non-détenteurs** (FR-044), avec deux valeurs d'enum ajoutées : `emission_tele`, `emission_radio`,
`episode_tele`, `episode_radio`.

**Rationale**: `proposition_media` stocke le contenu proposé dans `donnees JSONB` jusqu'à validation,
précisément pour que « rien de non validé n'existe dans les tables publiques » (09l). Cette garantie
vise les contributeurs anonymes ; elle est ici sans objet, puisque le co-détenteur est déjà autorisé
sur le support et que son fichier est déjà téléversé. L'état `'en_attente'` existe **déjà** sur les
tables de contenu depuis 09j §6, introduit pour exactement ce cas. Créer l'objet directement donne
gratuitement : le suivi par le détenteur (FR-042 = lire ses propres épisodes), l'exclusion de la
rotation (FR-018 = le filtre `etat = 'publie'` déjà présent partout), et la resoumission après
correction (FR-041 = repasser `rejete` → `en_attente`).

**Alternatives considered**:
- *Tout passer par `proposition_media`* : impose de sérialiser un épisode en JSONB, de le désérialiser
  à la validation, et prive le détenteur d'un objet à corriger. Deux files de suivi au lieu d'une.
  Rejeté.

---

## R8 : Discriminant des interactions élargi à six valeurs

**Decision**: Les CHECK `type_media` de `media_reaction`, `media_commentaire`, `partage_media` et
`signalement_media` passent de 4 à 6 valeurs : `chaine_tv`, `station_radio`, `emission_tele`,
`emission_radio`, `episode_tele`, `episode_radio`. Les compteurs restent calculés **par cible**, sans
agrégation de l'épisode vers l'émission (FR-048).

**Rationale**: C'est la traduction directe de la clarification Q3. Le CHECK est déjà conçu pour évoluer
sans `ALTER TYPE`. `compteurs_pour` (`media_social.rs`) charge les compteurs d'une page entière en deux
requêtes : le passage à six valeurs ne change ni sa forme ni son coût.

Le seuil de suspension automatique (`SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA = 10`, comparateur `>`)
s'applique tel quel à chaque nouvelle cible : FR-050 est satisfait par le fait que le recompte est déjà
filtré sur `(type_media, media_id)`.

---

## R9 : Mise en avant et vedette : portées sur l'épisode

**Decision**: `a_la_une` (une par chaîne/station) et `a_la_une_globale` (une pour tout l'espace Télé)
migrent de `programme_tele` vers `episode_tele`, avec leurs index uniques partiels. Le pendant radio
conserve `a_la_une` sur `episode_radio`.

**Rationale**: FR-052 exige que la vedette désigne une unité diffusable. Les identifiants étant
conservés sur l'épisode (R2), la reprise est un simple report de colonne. L'index unique sur
l'expression constante `((TRUE))` posé par 09j §3 se transpose sans changement, de même que la règle
déjà documentée : la bascule de l'ancienne vedette et la désignation de la nouvelle doivent tenir dans
**une même transaction**, faute de quoi la seconde requête viole l'index en concurrence.

---

## R10 : Ordre des épisodes : colonne `ordre` + endpoint de réordonnancement

**Decision**: `episode_*.ordre INT NOT NULL DEFAULT 0`, tri `(ordre, created_at, id)`. À la création,
`ordre = COALESCE(MAX(ordre), -1) + 1` sur l'émission. Un endpoint
`PUT /api/medias/emissions/{id}/episodes/reordonner` réécrit l'ordre **dans une transaction unique**.
Pas de contrainte d'unicité sur `(emission_id, ordre)`.

**Rationale**: Patron déjà en production dans `admin/formation_contenu.rs` (chapitres et leçons :
`COALESCE(MAX(ordre), -1) + 1` à la création, réordonnancement atomique commenté « tout réordonner ou
rien »). Le réutiliser à l'identique évite d'inventer une seconde façon d'ordonner dans le même schéma.
L'unicité est volontairement omise : elle imposerait un réordonnancement en deux passes (décalage
temporaire) pour éviter les collisions transitoires, sans bénéfice, le tri secondaire par
`created_at, id` rend l'ordre total et stable même en cas d'ex æquo.

FR-007 (« un épisode ajouté prend rang à la fin ») et FR-019 (« le recalcul ne change pas l'occurrence
en cours ») sont satisfaits ensemble : ajouter en fin de liste n'altère aucun `idx` inférieur, donc
aucun rang déjà atteint par la rotation.

---

## R11 : Périmètre du portage : ce qui casse et où

**Decision**: Recenser explicitement les points de bascule, pour que `/speckit-tasks` n'en omette aucun.

| Point | Fichier | Nature du changement |
|-------|---------|----------------------|
| Table de contenu d'un support | `models/media_detention.rs:35-41` | `table_contenu_pour_support` renvoie désormais `emission_tele` / `emission_radio` |
| Diffusion en cours / suivante | `handlers/media_programmation.rs:41-75` | Jointure sur l'émission + `JOIN LATERAL` de rotation |
| Bénéficiaire des points | `services/engagement.rs:638-692` | 4 `type_objet` : l'épisode remonte à son émission puis au support ; l'émission remonte au support |
| Sections et vedette télé | `handlers/television.rs` (13 handlers) | Sections porteuses d'émissions ; vedette = épisode |
| Sections radio | `handlers/stations_radio.rs` | Idem |
| Cibles sociales | `handlers/media_social.rs`, `models/media_social.rs` | 6 valeurs de discriminant |
| Propositions | `handlers/media_proposition.rs`, `admin/media_proposition.rs` | 4 nouveaux `type_objet`, validation crée émission **ou** épisode |
| Cadeaux virtuels | `handlers/engagement_cadeau.rs`, `models/engagement_cadeau.rs` | `type_objet` cible étendu |
| Règles d'engagement | `pages/admin/engagement/regles.vue`, `types/admin.ts` | Libellés des cibles |

**Rationale**: Les 137 occurrences Rust ne sont pas toutes structurantes ; ces neuf points le sont. Les
autres sont des mentions de chaîne de caractères que la suppression des tables fera échouer bruyamment
au premier appel (R1).
