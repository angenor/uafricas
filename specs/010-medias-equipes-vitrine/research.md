# Phase 0 — Recherche et décisions de conception

**Feature**: 010-medias-equipes-vitrine · **Date**: 2026-08-10

Aucun marqueur `NEEDS CLARIFICATION` n'est entré en Phase 0 : les trois ambiguïtés de la spec ont été levées par le commanditaire avant le plan (Q1→A, Q2→B, Q3→A). Ce document consigne les décisions **techniques** issues du relevé de l'existant.

---

## D1 — Forme de la table d'équipe

**Décision** : une table unique `media_content.membre_equipe`, polymorphe par `(type_porteur, porteur_id)`, avec `type_porteur VARCHAR(20)` contraint par un **CHECK à 4 valeurs** (`chaine_tv`, `station_radio`, `emission_tele`, `emission_radio`).

**Rationale** :
- Les colonnes et toutes les règles métier sont identiques au niveau support et au niveau programme. Deux tables dupliqueraient requêtes, DTO, handlers, validations et audit.
- Le patron est déjà employé trois fois dans le schéma : `support_thematique` et `support_territoire` par `(type_support, support_id)` (09r), et les quatre tables d'interactions par `(type_media, media_id)` (09k), dont le CHECK a été porté à 6 valeurs par 09q.
- **CHECK plutôt qu'ENUM** : aucun enum existant ne décrit ce jeu précis. `type_support_media` (09m) n'a que 2 valeurs — supports seuls, par construction (« un contenu n'a pas de détenteur propre »). Le CHECK des interactions en a 6, dont 2 (`episode_*`) qui n'ont pas de sens ici. Créer un cinquième enum pour 4 valeurs coûterait, à chaque extension, un `ALTER TYPE … ADD VALUE` qui ne peut cohabiter avec son usage dans la même transaction — la contrainte qui a imposé le préambule de 09q. Un CHECK s'étend par un `DROP`/`ADD CONSTRAINT` ordinaire.

**Alternatives écartées** :
- *Deux tables (`support_equipe`, `emission_equipe`)* — offrirait de vraies FK, mais quadruple les chemins de code sur une entité qu'aucune autre table ne référence.
- *Réutiliser `support_detenteur`* — impossible : cette table porte des **droits** (`proprietaire`, `co_detenteur`, `programmateur`), exige un `utilisateur_id NOT NULL`, et ne couvre pas les programmes. Attention au piège de vocabulaire : `MesSupports.vue:505` intitule déjà « Équipe du support » le panneau des **co-détenteurs**. Les deux notions doivent rester distinctes à l'écran (voir D7).
- *Étendre `info_animateur`/`info_producteur`* — deux `TEXT` scalaires sans multiplicité, sans rang, sans champs séparés. Ils sont conservés en base (aucune saisie perdue) et cessent d'être la source d'affichage.

---

## D2 — Rattachement facultatif à un compte (Q2 → B)

**Décision** : `utilisateur_id UUID NULL REFERENCES iam.utilisateur(id) ON DELETE SET NULL`. Le lien vers le profil public n'est produit que si le compte existe **et** n'est pas supprimé — résolu à la lecture par `LEFT JOIN iam.utilisateur u ON u.id = m.utilisateur_id AND u.deleted_at IS NULL`.

**Rationale** : `ON DELETE SET NULL` réalise en SQL l'exigence du cas limite « membre rattaché à un compte devenu indisponible → texte simple, sans lien mort ». Le `deleted_at IS NULL` dans la condition de jointure couvre la suppression douce, que la FK ignore. La fiche survit dans les deux cas, puisque nom/prénom/fonction sont saisis, jamais dérivés du compte.

**Point de sécurité** : le champ `contact` est **saisi à la main** par le gestionnaire. Il n'est jamais rempli depuis `iam.utilisateur.email`, même quand le rattachement existe. Un rattachement ne doit pas transformer une adresse de compte en donnée publique.

---

## D3 — Fonction dynamique et déduplication des suggestions (FR-015)

**Décision** : `fonction VARCHAR(120) NOT NULL`, texte libre. Deux traitements complémentaires :

1. **À l'écriture** — normalisation minimale : `btrim` + réduction des espaces internes (`regexp_replace(fonction, '\s+', ' ', 'g')`). La casse saisie est conservée telle quelle.
2. **À la lecture des suggestions** — regroupement par clé insensible à la casse et aux espaces, et restitution de l'orthographe **la plus employée** pour chaque clé :

```sql
SELECT fonction FROM (
    SELECT DISTINCT ON (cle) fonction
      FROM (
        SELECT lower(btrim(regexp_replace(fonction, '\s+', ' ', 'g'))) AS cle,
               btrim(regexp_replace(fonction, '\s+', ' ', 'g'))        AS fonction,
               COUNT(*)                                                AS n
          FROM media_content.membre_equipe
         WHERE deleted_at IS NULL AND btrim(fonction) <> ''
         GROUP BY 1, 2
      ) v
     ORDER BY cle, n DESC, fonction ASC
) f ORDER BY fonction ASC
```

**Rationale** : c'est le patron canonique du projet, `GET /api/experts/specialites` (`handlers/experts.rs:616`), dont la justification écrite tient toujours : on ne propose que des valeurs **réellement déclarées**. Trois clones existent déjà dans le domaine médias (`lister_genres_stations`, `lister_pays_stations`, `lister_pays_television`). L'écart avec ces clones — le `DISTINCT ON` — est exigé par FR-015, que le simple `SELECT DISTINCT` ne satisfait pas : « Directeur » et « directeur » y remonteraient tous deux.

**Alternative écartée** : une table de référentiel `fonction_equipe` avec CRUD admin (patron `iam.specialite_bibliotheque`). Elle contredit « toute fonction nouvelle est acceptée » (FR-015) en imposant une création préalable, et ajoute une surface d'administration que la demande ne réclame pas.

---

## D4 — Périodicité : étendre les libellés, pas les clés (Q1 → A)

**Décision** : conserver les trois clés stockées (`ponctuelle`, `quotidienne`, `hebdomadaire`) et n'ajouter que `mensuelle`. Seuls les **libellés** changent :

| Clé stockée | Libellé actuel | Libellé cible |
|---|---|---|
| `ponctuelle` | Au fil des publications | **Non périodique** |
| `quotidienne` | Tous les jours | **Journalier** |
| `hebdomadaire` | Chaque semaine | **Hebdomadaire** |
| `mensuelle` | — | **Mensuel** (neuf) |

**Rationale** : FR-043 interdit d'altérer les périodicités déjà déclarées. Renommer les clés imposerait un `UPDATE` de données, un basculement de CHECK, et la reprise de six gardes `v-if="cadence !== 'ponctuelle'"` disséminées dans le frontend — pour un gain nul, la clé n'étant jamais montrée à l'utilisateur. La migration se réduit à deux `DROP`/`ADD CONSTRAINT`.

**Conséquence obligatoire, à ne pas oublier** : `mes_alertes_cadence` (`handlers/media_programmation.rs:825-847`) calcule aujourd'hui `periode_heures = 24 si "quotidienne", sinon 24×7`. Tel quel, un programme **mensuel** déclencherait une alerte de retard chaque semaine. La période et l'anticipation deviennent des fonctions de la cadence, dans `models/media_emission.rs` :

| Cadence | `periode_heures` | `heures_anticipation_alerte` |
|---|---|---|
| `quotidienne` | 24 | 6 |
| `hebdomadaire` | 168 | 48 |
| `mensuelle` | 720 (30 j) | 168 (7 j) |
| `ponctuelle` | — | `None` → aucune alerte |

**Corollaire d'affichage** : FR-041 impose des libellés identiques à la saisie et en public. Aujourd'hui l'admin entretient son propre `CADENCES`/`libelleCadence` dans `useAdminMediaEmissions.ts`, distinct de `LIBELLES_CADENCE` (`useMediaEmissions.ts:109`). Les deux surfaces lisent désormais la même constante. Et le scénario US5-3 impose de retirer les gardes qui masquent `ponctuelle` sur les **deux pages de détail** — « non périodique » est une information, pas une absence.

---

## D5 — Ce que les sections cessent de renvoyer

**Décision** : dans `television::lister_sections` et `stations_radio::lister_sections_stations` —

1. **Ne plus appeler `greffer_apercus_et_compteurs`.** Les sections ne rendent plus d'épisode : jusqu'à 12 épisodes × 12 programmes × 6 chaînes de JSON deviennent du poids mort. Les compteurs d'interaction du **support** (`compteurs_pour("chaine_tv", …)`) restent : la barre de réactions du support n'est pas retirée.
2. **Lever la condition d'existence d'un épisode publié.** Côté télé, l'`EXISTS` du WHERE (`television.rs:376-381`) disparaît. Côté radio, c'est le `sections.retain(|s| !s.emissions.is_empty() || s.direct_disponible)` de `stations_radio.rs:468` — dont la disparition corrige au passage l'incohérence entre le `total` compté en SQL et le nombre de sections servies.
3. **Ouvrir la latérale de `emissions_publiees_par_supports`** : `JOIN LATERAL (…) agg ON agg.nombre_episodes > 0` devient `ON TRUE` (`handlers/media_emission.rs:75-99`).
4. **Greffer l'équipe du support** (et elle seule — la carte de programme n'affiche pas d'équipe, FR-004).

**Rationale** : FR-005 (« un programme reste listé même sans épisode publié ») et le cas limite « chaîne sans programme : la section reste affichée avec l'identité et l'équipe » sont incompatibles avec ces trois filtres. Sous le modèle neuf, la vitrine annonce une **offre éditoriale**, pas un catalogue de fichiers.

**Effet de bord à annoncer au recettage** : des chaînes et stations publiées mais dépourvues d'épisodes vont **apparaître** sur les vitrines, où elles étaient jusqu'ici invisibles. C'est voulu et exigé par la spec, mais c'est un changement visible du contenu servi, pas seulement de sa mise en forme. Le décompte affiché sur `/medias/tele` bougera d'autant.

**Décision liée** : `obtenir_emission_par_slug` (`handlers/media_emission.rs:278`) renvoie aujourd'hui 404 pour une émission publiée **sans épisode publié**. FR-033 exige qu'elle reste consultable. Le 404 ne subsiste que si l'émission elle-même n'est pas publiée.

---

## D5 bis — Le plafond de programmes par section devient visible

**Constat** : `contenus_par_section.unwrap_or(12).clamp(1, 30)` (`television.rs:369`, et son jumeau radio) borne le nombre de programmes rendus par section, et **aucune page front ne transmet ce paramètre** — vérifié, zéro occurrence dans `app/pages/`. Une chaîne de 13 programmes en cache donc un, une chaîne de 40 en cache dix au minimum, sans le dire.

Tant que la section montrait des épisodes, ce plafond bornait un **aperçu**. Sous le modèle neuf, la liste de programmes **est** le contenu de la section : le même plafond devient une troncature silencieuse de l'information principale, en contradiction avec FR-001, SC-008 et le cas limite « plus de 30 programmes ».

**Décision** : plafond porté à **30 par défaut, 60 au maximum**, et la section **annonce le total** avec un lien vers la page de la chaîne dès que `emissions.len() < total_emissions` (FR-008).

**Rationale** :
- Le budget de payload a changé de nature. Un programme rendu pesait, aperçus compris, jusqu'à 12 épisodes sérialisés ; il ne pèse plus que ses métadonnées. Trente programmes coûtent aujourd'hui moins que douze hier.
- `total_emissions` est **déjà** porté par `TeleSectionResponse` et `StationSectionResponse` : la détection de troncature ne demande aucun champ neuf.
- Un plafond conservé plutôt que supprimé : une section reste une vitrine, pas un catalogue, et une borne haute protège la page d'un support pathologique.

**Alternatives écartées** :
- *Supprimer toute borne* — expose la page à une chaîne au catalogue démesuré, sans gain pour le cas courant.
- *Paginer les programmes dans la section* — deux niveaux de pagination sur la même page (chaînes **et** programmes), pour un gain nul : la page de la chaîne existe déjà et les liste tous.

---

## D5 ter — Les mentions héritées « Animation » et « Production » cessent d'être affichées

**Constat** : les deux pages de programme rendent une ligne « Animation : … · Production : … » depuis `info_animateur` / `info_producteur` (`emissions-tele/[slug].vue:128-135` et son pendant radio). Conservée à côté du nouveau bloc d'équipe, elle donnerait **deux sources concurrentes** pour la même information, l'une structurée et l'autre non.

**Décision** (FR-034) : l'affichage est retiré des deux pages. Les colonnes restent en base et les deux champs restent **visibles en lecture** dans les formulaires d'édition, explicitement libellés « hérité — reporter dans l'équipe ».

**Rationale** : supprimer les colonnes perdrait des saisies que personne n'a encore reportées ; les masquer entièrement des formulaires priverait le gestionnaire de la seule trace de ce qu'il doit recopier. Les garder affichées au public reviendrait à livrer la contradiction que la feature vient corriger.

---

## D6 — Écriture par remplacement intégral

**Décision** : `PUT …/equipe` reçoit la liste complète et ordonnée ; le handler exécute `DELETE` puis `INSERT` dans une transaction, `ordre` valant l'index reçu.

**Rationale** : copie conforme de `media_support::appliquer_thematiques` (`handlers/media_support.rs:217`), déjà éprouvé sur le même domaine. L'équipe est éditée comme un tout dans un formulaire ; le remplacement rend le réordonnancement (FR-016) trivial et évite trois routes supplémentaires (POST/PATCH/DELETE par membre). Aucune table ne référence un membre d'équipe : la rotation des identifiants est sans conséquence.

**Audit (Principe VII, FR-018)** : une action `equipe_modifiee` par `PUT`, avec l'instantané avant et après en JSONB. Un `PUT` couvre indifféremment création, modification, suppression et réordonnancement — la diff est lisible dans le journal, ce qu'un flux d'événements unitaires ne donnerait pas mieux.

**Gardes** : côté membre `garde_detenteur(≥ co_detenteur)` (`handlers/media_detention.rs:56`), **jamais** `AdminUtilisateur` — ces routes sont membres, erreur déjà commise et corrigée en 009. Quand le porteur est une émission, le support est résolu par `contexte_emission` (`handlers/media_emission.rs:675`) avant la garde. Côté admin, `verifier_permission!(admin, "media", "modifier")`.

---

## D7 — Lecture publique : greffer, ne pas ajouter de route

**Décision** : l'équipe voyage dans les DTO existants — `ChaineTvResponse.equipe`, `StationRadioResponse.equipe`, `EmissionResponse.equipe` — sérialisées avec `skip_serializing_if = "Vec::is_empty"`. **Aucune route publique neuve.**

**Rationale** : le frontend affiche l'équipe partout où il affiche déjà le support ou le programme. Une route dédiée ajouterait un aller-retour par page et une cascade de chargement, pour une donnée de quelques lignes. C'est exactement le choix fait pour `thematiques` et `couverture` en 09r.

**Chargement sans N+1** : `equipes_par_porteurs(pool, type_porteur, &[Uuid]) -> HashMap<Uuid, Vec<MembreEquipeResponse>>`, une requête `WHERE porteur_id = ANY($1)`, patron de `thematiques_par_supports` (`handlers/media_support.rs:37`). La page de détail d'une chaîne en appelle deux : une pour le support, une pour ses programmes.

**Volume** : l'équipe complète est servie, sans troncature serveur. Le seuil de repli de FR-024 est une décision d'affichage, tranchée côté client — une équipe compte quelques unités de personnes, jamais des milliers.

**Piège de vocabulaire à trancher à l'écran** : `MesSupports.vue:505` appelle déjà « Équipe du support » le panneau des co-détenteurs (des **droits**). La nouvelle section s'intitule « Équipe éditoriale » et l'ancienne devient « Gestion des accès » — sans quoi le détenteur trouvera deux panneaux « équipe » qui ne parlent pas de la même chose.

---

## D8 — Composants frontend : ce qui existe, ce qui manque

Relevé exhaustif effectué sur `app/` :

| Besoin | Existant | Décision |
|---|---|---|
| Texte tronqué avec « voir plus / voir moins » | **Rien.** Zéro occurrence de « voir moins / lire moins / réduire ». La troncature est partout du `line-clamp` CSS irréversible, dupliqué à l'identique dans 5 `<style scoped>`. Les seuls « Voir plus » sont des paginations. | Créer `common/TexteRepliable.vue` (5 points d'appel). |
| Champ combo (saisie libre + suggestions) | **Rien de réutilisable.** Aucun `<datalist>` dans tout `app/`. `arbre-genealogique/ChampRecherche.vue` est le seul « input + liste déroulante », mais spécialisé (props `graphe: Map<…>`) et **il ne valide pas la saisie libre** : la sélection vide le champ. Le contournement actuel est un `<select>` avec option `AUTRE` révélant un `<input>` (`ProposerMediaModal.vue:412`, `admin/medias/emissions/[id].vue:247`). | Créer `common/ChampCombo.vue` (4 points d'appel). |
| Carte de programme pour la vitrine | `media/CarteEmission.vue` existe et rend déjà couverture + titre + badge — mais c'est **du code mort** (zéro usage) et il n'affiche pas la description tronquée exigée par FR-004. | Créer `media/CarteProgramme.vue` et supprimer `CarteEmission.vue`. Le ressusciter en le réécrivant à 80 % n'aurait aucun intérêt. |
| Rendu public d'une équipe | Rien. | Créer `media/EquipeMedia.vue`. |
| Édition d'une équipe, membre **et** admin | Précédent direct : `media/GestionEpisodes.vue`, « un seul composant membre+admin, l'autorité seule diffère ». Vérifié : ce composant, `MesSupports.vue` et `GestionCoDetenteurs.vue` n'emploient **aucune** classe daisyUI. | Créer `media/GestionEquipe.vue` en Tailwind pur, monté des deux côtés. |

**Anomalie rencontrée, réparée en passant** : `CommonFilAriane` est monté en `emissions-tele/[slug].vue:94` et `emissions-radio/[slug].vue:94` — **ce composant n'existe pas**. Le fil d'Ariane est donc mort sur les deux pages que cette feature réécrit de toute façon. Il est remplacé par le `<nav>` écrit à la main que les autres pages médias emploient. Correction d'une ligne, sur un fichier déjà ouvert ; le signaler sans le corriger serait laisser un bug connu dans du code touché.

---

## Récapitulatif des points de vigilance au recettage

1. **sqlx est vérifié au runtime.** Chaque requête modifiée doit être exécutée au moins une fois — une colonne oubliée compile.
2. **Ordre des routes actix.** Deux 404 « UUID parsing failed » ont été livrés en 009 pour cette raison. Les segments littéraux neufs (`/equipe/fonctions`) passent avant les motifs à deux paramètres.
3. **Chaînes et stations sans épisode vont apparaître** sur les vitrines (D5). Vérifier le décompte affiché avant et après.
4. **L'alerte de cadence mensuelle** doit se déclencher au bout d'un mois, pas d'une semaine (D4).
5. **Le champ `contact`** ne doit jamais être pré-rempli depuis le compte rattaché (D2).
