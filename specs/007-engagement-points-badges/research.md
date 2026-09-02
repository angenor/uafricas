# Phase 0 : Recherche & décisions techniques

**Feature** : `007-engagement-points-badges` | **Date** : 2026-07-29

Toutes les décisions ci-dessous sont prises **après lecture du code existant** (`services/engagement.rs`, `handlers/engagement.rs`, `handlers/admin/engagement.rs`, `35_engagement.sql`, `35b_engagement_mise_en_avant.sql`, les handlers médias et les 6 modales de partage). Aucun `NEEDS CLARIFICATION` ne subsiste.

---

## R1 : Catégories de points : table dédiée + catégorie figée sur le mouvement

**Décision** : nouvelle table `engagement.categorie_points (code, libelle, ordre, couleur, icone, actif)` ; `regle_points.categorie_id` (FK, nullable) ; **et** `mouvement_points.categorie_id` (FK, nullable) renseignée **à l'écriture** par recopie de la catégorie de la règle.

**Rationale** : la spec impose que « la ventilation reflète la catégorie **au moment du mouvement** ; une re-catégorisation n'est pas rétroactive » (edge case explicite). Seule une copie sur la ligne de journal garantit ça. Le surcoût est d'une colonne et d'un `JOIN` évité à la lecture.

**Alternatives rejetées** :
- *Joindre `regle_points` à la lecture*, plus normalisé, mais toute re-catégorisation réécrirait l'historique affiché. Interdit par la spec.
- *Catégorie déduite du `type_objet`* : le `type_objet` est le domaine du contenu (codimoi, chaine_tv…), pas la nature de l'effort récompensé ; « contribution validée » et « palier de popularité » sur un même codimoi doivent tomber dans deux catégories différentes.

**Nullable, pourquoi** : les mouvements déjà journalisés en production n'ont pas de catégorie. La migration les rattache par `UPDATE … FROM regle_points` (rattrapage unique), et les rares reliquats s'affichent sous « Autres ».

---

## R2 : Ventilation calculée à la lecture, pas de soldes persistés par catégorie

**Décision** : la ventilation est une agrégation `SUM(points) … GROUP BY categorie_id` sur `mouvement_points` du membre. Aucune colonne de solde par catégorie sur `engagement.compte`.

**Rationale** : Principe V. L'index `idx_mouvement_utilisateur (utilisateur_id, created_at DESC)` couvre déjà le filtre ; le volume par membre est de l'ordre de quelques centaines de lignes. Des soldes persistés créeraient un second état à maintenir cohérent avec le journal (donc à réparer un jour).

**Réconciliation avec le solde** (SC-005) : `SUM(points)` toutes catégories confondues peut **dépasser** `compte.solde_points` à cause du **plancher 0** (`nouveau_solde = (solde + points).max(0)` dans `appliquer`). L'espace membre affiche donc deux notions distinctes et libellées comme telles : « points gagnés par catégorie » (cumul du journal) et « solde courant » (`compte.solde_points`). C'est exactement l'écart que la spec demande de rendre compréhensible.

**Alternative rejetée** : *vue matérialisée*, rafraîchissement à ordonnancer, donc une tâche de fond, interdite par les contraintes.

---

## R3 : Création de règles : un catalogue des actions réellement instrumentées

**Problème découvert** : les points ne sont attribués que là où le code appelle `attribuer("<type_action>", …)`. Une règle créée pour un `type_action` que personne n'émet ne crédite jamais rien. « Barème entièrement paramétrable » ne peut donc pas signifier « je crée une action et elle se met à rapporter des points ».

**Décision** :
1. Le code publie un **catalogue const** des actions instrumentées (`type_action`, libellé par défaut, types d'objet concernés, module d'origine) exposé par `GET /api/admin/engagement/actions-disponibles`.
2. L'écran de création propose ce catalogue en priorité et **signale les règles orphelines** (`actif = TRUE`, aucun mouvement, action absente du catalogue) par la mention « aucun point n'a jamais été attribué par cette règle ».
3. La saisie d'un `type_action` libre reste permise (préparer une règle avant son branchement) mais est explicitement étiquetée « non instrumentée ».

**Rationale** : l'administrateur garde la maîtrise totale de ce qui est paramétrable (montants, réputation, plafonds, catégorie, libellé, activation, seuil) : donc SC-002 est tenu, sans illusion sur ce qui exige une livraison de code. Mentir ici produirait exactement le bug le plus coûteux à diagnostiquer : « j'ai créé la règle, elle ne donne rien ».

**Alternative rejetée** : *table `action_disponible` en base alimentée par les migrations*, dédouble la vérité (le code reste seul juge de ce qu'il émet) et se désynchronise au premier branchement oublié.

---

## R4 : Paliers de popularité par famille de contenus

**Décision** : `palier_popularite.type_objet VARCHAR(40) NULL` (NULL = palier global). L'unicité passe de `UNIQUE(seuil_likes)` à `UNIQUE NULLS NOT DISTINCT (seuil_likes, type_objet)` (PostgreSQL 15+, disponible en 16). Résolution dans `evaluer_popularite` : **si au moins un palier actif existe pour la famille, il remplace intégralement les paliers globaux ; sinon les globaux s'appliquent** (règle de substitution, pas d'union).

**Rationale** : la substitution est la seule sémantique prévisible, une union ferait cumuler « 100 likes global » et « 100 likes télé » sur le même contenu (deux clés d'idempotence distinctes → double crédit). La spec dit « à défaut de palier spécifique, les paliers globaux s'appliquent », ce qui est exactement une substitution.

**Impact sur l'idempotence** : la clé reste `popularite:{type_objet}:{objet_id}:{seuil}`, déjà porteuse du `type_objet`, donc inchangée. Un contenu ayant déjà déclenché un palier global ne le re-déclenche pas si un palier spécifique de même seuil apparaît plus tard.

**Migration de l'existant** : la contrainte `palier_popularite_seuil_likes_key` est remplacée ; les 3 paliers seedés (100/500/1000) restent globaux (`type_objet IS NULL`).

---

## R5 : CRUD des niveaux + recalcul ensembliste

**Problème** : `compte.niveau_code` est un `VARCHAR(30)` **sans FK** vers `niveau.code`, recalculé uniquement lors d'un mouvement (`niveau_pour_solde`). Créer, déplacer ou retirer un niveau laisserait donc des comptes sur un code périmé jusqu'à leur prochain mouvement : or le scénario 5 de l'US2 exige la bascule « sans opération manuelle membre par membre ».

**Décision** : toute mutation de `engagement.niveau` (création, modification de seuil, retrait) est suivie, **dans la même transaction**, d'un recalcul ensembliste :

```sql
UPDATE engagement.compte c
   SET niveau_code = COALESCE((SELECT n.code FROM engagement.niveau n
                                WHERE n.seuil_min <= c.solde_points
                                ORDER BY n.seuil_min DESC LIMIT 1), 'membre'),
       updated_at = NOW();
```

Une seule requête pour toute la table (≈ 15 000 lignes) : quelques dizaines de millisecondes, exécutée à une fréquence de l'ordre de la modification annuelle du barème.

**Garde-fous** : refus de retirer le niveau plancher (`seuil_min = 0`) ou le dernier niveau restant ; refus de deux niveaux au même `seuil_min` (contrainte `UNIQUE`) ; `ordre` recalculé d'après `seuil_min` croissant à chaque mutation, pour que `ordre` et `seuil_min` ne puissent pas se contredire (edge case « niveaux mal ordonnés »).

**Pas de FK ajoutée** sur `compte.niveau_code` : elle empêcherait le retrait d'un niveau encore porté par des comptes, alors que la spec veut ce retrait possible (les membres retombent au niveau inférieur). `charger_niveau` retombe déjà sur « Membre » si le code est introuvable.

---

## R6 : Conditions de badge : enum fermé, pas de moteur d'expressions

**Décision** : `engagement.type_condition_badge` = enum PostgreSQL à 5 valeurs, avec 3 colonnes de paramètres sur `badge` :

| `type_condition` | Paramètres utilisés | Sens |
|---|---|---|
| `actions_comptees` | `parametre_action` + `seuil` | N mouvements du `type_action` donné |
| `points_categorie` | `parametre_categorie_id` + `seuil` | N points cumulés dans une catégorie |
| `solde_total` | `seuil` | Solde global ≥ N |
| `niveau_atteint` | `parametre_niveau_code` | Niveau courant ≥ ce niveau (comparaison sur `ordre`) |
| `palier_popularite` | `seuil` | Un palier ≥ N franchi (au moins une fois) |
| *(aucune)* | `manuel = TRUE` | Badge éditorial, attribué à la main uniquement |

**Rationale** : Principe V et exigence de spec (« l'administration paramètre les valeurs, pas des expressions libres »). Chaque type se traduit en **une** requête `EXISTS`/`SUM` connue à l'avance : donc auditable, indexable, et insensible à l'injection.

**Alternative rejetée** : *condition en JSONB avec mini-langage*, flexibilité dont personne n'a besoin aujourd'hui, au prix d'un évaluateur à écrire, tester et sécuriser ; c'est précisément le cas d'école du Principe V.

---

## R7 : Quand évaluer les badges : post-commit du mouvement + à la lecture de l'espace

**Décision** : `services::engagement::evaluer_badges(pool, utilisateur_id)` est appelée à deux endroits, jamais dans une transaction métier :
1. **Après le commit** d'un mouvement réussi, depuis `appliquer` (donc pour `attribuer` / `retirer` / `evaluer_popularite` / `ajuster` d'un coup), les erreurs sont loguées, jamais propagées (FR-028).
2. **À la lecture** de `GET /api/engagement/mes-badges`, rattrape les conditions devenues vraies autrement qu'à la suite d'un mouvement (création d'un badge par l'administration, condition assouplie), sans aucune tâche de fond.

L'insertion est `INSERT … ON CONFLICT (utilisateur_id, badge_id) DO NOTHING` : l'idempotence est structurelle, donc réévaluer est inoffensif (FR-034). La notification n'est émise **que si l'insertion a réellement créé une ligne** (`rows_affected() == 1`), ce qui interdit la notification répétée (SC-010).

**Coût** : une requête par badge actif non encore obtenu, sur ≈ 10–20 badges, et l'ensemble se réduit à mesure que le membre les collectionne.

**Alternative rejetée** : *trigger PostgreSQL sur `mouvement_points`*, attribuerait les badges dans la transaction du mouvement (donc dans celle de l'action métier), en contradiction directe avec le caractère non-bloquant du moteur.

---

## R8 : Notifications : `arbre_genealogique.notifications`, pas `social.notification`

**Décision** : émettre dans `arbre_genealogique.notifications (destinataire_id, type, message, lien_action)` avec deux nouvelles constantes dans `models/notification.rs` :

```rust
pub mod engagement {
    pub const NIVEAU_ATTEINT: &str = "engagement.niveau_atteint";
    pub const BADGE_DEBLOQUE: &str = "engagement.badge_debloque";
}
```

`lien_action = "/mon-compte/engagement"`.

**Rationale** : malgré son nom, `arbre_genealogique.notifications` est la table **générique de fait** de la plateforme, `type` est un `VARCHAR` libre (élargi par `09j`), et les domaines afrolang, médias, profils pays y écrivent déjà via les constantes centralisées de `models/notification.rs`. La cloche `ClocheNotifications.vue` et `useNotifications.ts` la consomment déjà : zéro travail d'affichage supplémentaire, hors libellé/icône des deux nouveaux types.

**Alternative rejetée** : *`social.notification`*, son `type` est un **enum PostgreSQL** (`social.type_notification_social`) et sa colonne `demande_id` référence `social.demande_amitie` : il faudrait un `ALTER TYPE` et une colonne de plus pour un domaine qui n'est pas le sien. Le document source évoquait « raccrocher au domaine social » ; l'inspection du code montre que la table réellement générique est l'autre.

---

## R9 : Rétro-évaluation des badges au lancement, sans notification

**Décision** : les migrations `35d` terminent par un `INSERT INTO badge_obtenu … SELECT … ON CONFLICT DO NOTHING` qui attribue les badges seedés dont la condition est **déjà** satisfaite par l'état courant des comptes (`solde_total`, `niveau_atteint`, `actions_comptees`). **Aucune notification n'est émise** pour ce lot.

**Rationale** : la spec accepte la rétro-évaluation des badges (et refuse celle des points). Notifier ce lot enverrait des dizaines de notifications simultanées à des membres qui n'ont rien fait à cet instant, bruit pur. Les badges apparaissent simplement dans l'espace « Mon engagement ».

**Cohérence** : `origine = 'retroactif'` sur ces lignes, ce qui les distingue de `'automatique'` et `'manuel'` dans le back-office.

---

## R10 : Partage externe : seuls **4** réseaux existent aujourd'hui, le seuil de 5 est inatteignable

**Constat** : les 6 modales de partage (`MediaPartagerModal`, `PartagerElementModal`, `PartagerFicheModal`, `EvenementPartage`, `PartagePublication`, `BoutonsPartage`) proposent exactement **WhatsApp, Facebook, X/Twitter, LinkedIn**, 4 réseaux. La règle du document source (« +10 pts / **5 partages sur réseaux distincts** ») serait donc **structurellement inatteignable**, et le barème afficherait une règle qui ne crédite jamais personne.

**Décision** :
1. Ajouter **Telegram** et **E-mail** (`mailto:`) aux modales, portant le catalogue à **6 réseaux**, le seuil de 5 devient atteignable sans être automatique.
2. Rendre le seuil **paramétrable** : nouvelle colonne `regle_points.seuil_declencheur INTEGER NULL` (5 par défaut pour cette règle, `NULL` pour toutes les autres). Sans cela, « 5 » serait une valeur figée dans le code, en contradiction avec SC-002.
3. Nouvelle table `engagement.partage_externe (utilisateur_id, type_objet, objet_id, reseau, created_at)` avec `UNIQUE (utilisateur_id, type_objet, objet_id, reseau)` : c'est cette contrainte qui rend « distincts » structurel : répéter un réseau ne crée pas de ligne.
4. `reseau` = enum PostgreSQL `engagement.reseau_social` (`whatsapp`, `facebook`, `x`, `linkedin`, `telegram`, `email`), convention BDD du Principe III ; un réseau futur s'ajoute par `ALTER TYPE … ADD VALUE`.
5. Le crédit est déclenché quand `COUNT(DISTINCT reseau) >= seuil_declencheur` pour ce couple (membre, contenu), avec la clé `partage5:{type_objet}:{objet_id}:{utilisateur_id}`, une seule fois par contenu, quel que soit le nombre de réseaux au-delà du seuil.
6. « Copier le lien » n'est **pas** un réseau (invérifiable, trivialement répétable) et ne compte pas.

**Rationale** : le partage externe est la règle la moins vérifiable du barème ; les garde-fous doivent être structurels (contrainte d'unicité, plafond journalier de la règle) plutôt que déclaratifs.

**Note d'honnêteté** : le système enregistre une **intention** de partage (le clic), pas une publication effective. C'est indiqué dans la spec et rappelé dans l'UI du back-office à côté de la règle.

---

## R11 : Les quatre branchements médias : points d'accroche exacts

Tous les crédits sont émis **après le `COMMIT`** de la transaction métier (pattern déjà utilisé par `admin/codimoi_admin.rs` et `admin/vidafrica.rs`), avec anti-auto-attribution explicite.

| Règle | Point d'accroche | Bénéficiaire | Clé d'idempotence | Anti-auto |
|---|---|---|---|---|
| `proposition_media_validee` | `handlers/admin/media_proposition.rs::valider_proposition` (après commit) | `auteur_id` de la proposition | `prop_media:{proposition_id}` | ignorer si `auteur_id == admin.id` |
| `animation_support_acceptee` | **2 sites** : `handlers/admin/media_proposition.rs` (file admin) **et** `handlers/media_proposition.rs::accepter_engagement` (décision des co-détenteurs) | `auteur_id` de la proposition | `animation:{proposition_id}` | ignorer si `auteur_id == decideur` |
| `media_a_la_une` | `handlers/admin/radio_tele.rs`, création et modification, sur les 4 tables portant `a_la_une` (`chaine_tv`, `station_radio`, `programme_tele`, `programme_radio`) | `cree_par` du contenu | `alaune:{type_objet}:{objet_id}` | ignorer si `cree_par == admin.id` |
| `popularite_palier` (familles médias) | `handlers/media_social.rs::reagir_media`, après le recompte des likes déjà présent | `cree_par` du contenu | `popularite:{type_media}:{media_id}:{seuil}` (existante) | likes de l'auteur exclus du décompte |

**Pourquoi deux sites pour l'animation et non un hook dans `appliquer_acceptation_engagement`** : cette fonction partagée reçoit une `&mut Transaction`, alors que `attribuer` prend un `&PgPool` et doit s'exécuter **hors** transaction pour rester non-bloquant. La clé d'idempotence identique (`animation:{proposition_id}`) garantit qu'une demande acceptée par l'un ou l'autre chemin ne crédite qu'une fois : ce qui préserve l'invariant de la fonction partagée (« les deux chemins produisent exactement le même effet »).

**`a_la_une` est un drapeau exclusif par support** (index unique partiel `(chaine_id, a_la_une)` / `(station_id, a_la_une)`) : le retirer puis le reposer ne re-crédite pas, la clé étant portée par le contenu et non par l'événement. C'est le comportement demandé (scénario 2 de l'US4) et cohérent avec « pas de clawback ».

---

## R12 : Popularité des médias : exclure l'auto-like et où prendre le décompte

**Décision** : `reagir_media` recompte déjà `nombre_likes` pour sa réponse ; on ajoute, **uniquement pour l'évaluation des paliers**, un décompte excluant l'auteur :

```sql
SELECT COUNT(*) FROM media_content.media_reaction
 WHERE type_media = $1 AND media_id = $2 AND type_reaction = 'like'
   AND utilisateur_id <> $3   -- l'auteur/détenteur
```

L'auteur est résolu via un `match` sur littéraux fixes (comme `table_pour_type`) : `chaine_tv|station_radio|programme_tele|programme_radio → cree_par`.

**Rationale** : la spec exclut l'auto-like du calcul des paliers, mais le compteur **affiché** doit rester le total réel (sinon l'auteur verrait son propre like disparaître de l'interface). Deux décomptes, deux usages : c'est explicitement le sens de la contrainte.

**Coût** : une requête supplémentaire par réaction, uniquement quand la réaction est un `like` posé (pas sur les retraits ni les dislikes).

---

## R13 : Espace membre : page dédiée, l'onglet existant devient la porte d'entrée

**Décision** : nouvelle page `/mon-compte/engagement`. L'onglet « Mes points » de `/mon-compte/profil` (déjà présent dans le groupe « Ce que j'anime ») **est conservé** et affiche le résumé (`MesPointsPanel` allégé) + un lien proéminent « Voir tout mon engagement ».

**Rationale** :
- SC-003 (« au plus 2 clics depuis le profil ») est tenu : profil → onglet → page.
- Une page dédiée est partageable, indexable dans l'historique du navigateur et peut porter des filtres dans l'URL, impossible pour un onglet interne piloté par un `ref`.
- Ne **pas** retirer l'onglet : il existe, des membres l'utilisent, et le retirer serait une régression d'UX non demandée.
- Le pattern est déjà celui de la plateforme : `/mon-compte/{mes-supports,invitations-medias,propositions-medias,contributions}` sont des pages sœurs du profil.

**Composants** : `ResumeEngagement` (soldes + niveau + progression), `VentilationCategories`, `MesBadges` (+ `BadgeSucces` unitaire), `HistoriquePoints` (filtres catégorie/période + pagination). Découpage aligné sur le pattern Hero/Card/Filters existant, Tailwind v4 pur (Principe VI).

---

## R14 : Écrêtage : rien à changer, tout à afficher

**Constat** : `appliquer` gère déjà l'écrêtage (`plafond_atteint = TRUE`, `points_effectifs` réduit au résiduel, y compris à **0**) et insère la ligne de journal même quand le résiduel est nul. La donnée nécessaire à FR-012 / SC-008 existe donc déjà : elle n'est simplement pas affichée : `MesPointsPanel` ignore `plafond_atteint`.

**Décision** : aucune modification du moteur d'écrêtage. `HistoriquePoints` affiche une mention « plafond atteint » sur ces lignes, et le libellé distingue les deux cas (« écrêté à N points » / « plafond atteint, aucun point crédité »).

---

## Synthèse des impacts

| Axe | Migrations | Backend | Frontend |
|---|---|---|---|
| Paramétrage (US2) | `35c` | CRUD règles/catégories/niveaux + `actions-disponibles` + `recalculer_niveaux` | `regles.vue` (refonte), `categories.vue`, `niveaux.vue` |
| Espace membre (US1) | `35c` (catégorie du mouvement) | `mes-categories`, `mon-journal` filtré | `engagement.vue` + 5 composants |
| Badges (US3) | `35d` | `badge` CRUD, `evaluer_badges`, 2 notifications | `badges.vue`, `MesBadges`, profil public |
| Couverture médias (US4) | : | 4 branchements (`media_proposition` ×2, `radio_tele`, `media_social`) |, |
| Partage externe (US5) | `35e` | `POST /partages-externes` + comptage distinct | `usePartageExterne` + 6 modales (dont +2 réseaux) |
