# Contrat : API back-office

**Feature** : `007-engagement-points-badges`
Extracteur : `AdminUtilisateur` · Garde : **`verifier_permission!(admin, "engagement", "gerer")`**
La permission `engagement.gerer` est **déjà seedée** par `35_engagement.sql` et attribuée à `super_admin`. Aucun seed de permission supplémentaire, mais tout autre rôle devant administrer le barème devra la recevoir explicitement (`/admin/roles`), sinon seul `super_admin` (wildcard `all.all`) franchira ces gardes.
Audit : **`audit::log_action` sur chacune des 15 routes mutantes** (Principe VII), avec état avant/après en JSONB.
Scope Actix : à la suite des 13 routes existantes du scope `/api/admin`.

## Récapitulatif

| Méthode | Chemin | État |
|---|---|---|
| GET | `/api/admin/engagement/actions-disponibles` | **NEW** |
| GET | `/api/admin/engagement/regles` | existante |
| POST | `/api/admin/engagement/regles` | **NEW** |
| PUT | `/api/admin/engagement/regles/{id}` | **modifiée** (+ catégorie, + seuil) |
| DELETE | `/api/admin/engagement/regles/{id}` | **NEW** (refus si référencée) |
| GET · POST | `/api/admin/engagement/categories` | **NEW** |
| PUT · DELETE | `/api/admin/engagement/categories/{id}` | **NEW** |
| GET | `/api/admin/engagement/paliers` | existante |
| POST · PUT | `/api/admin/engagement/paliers` · `/{id}` | **modifiées** (+ `type_objet`) |
| DELETE | `/api/admin/engagement/paliers/{id}` | existante (désactivation) |
| GET | `/api/admin/engagement/niveaux` | existante |
| POST | `/api/admin/engagement/niveaux` | **NEW** |
| PUT | `/api/admin/engagement/niveaux/{id}` | **modifiée** (+ recalcul) |
| DELETE | `/api/admin/engagement/niveaux/{id}` | **NEW** |
| GET · POST | `/api/admin/engagement/badges` | **NEW** |
| PUT · DELETE | `/api/admin/engagement/badges/{id}` | **NEW** |
| POST | `/api/admin/engagement/badges/{id}/attribuer` | **NEW** |
| DELETE | `/api/admin/engagement/badges/{id}/attribuer/{utilisateur_id}` | **NEW** |
| GET | `/api/admin/engagement/journal` | **modifiée** (+ filtre catégorie) |
| POST | `/api/admin/engagement/ajustement` | existante |
| GET·POST·DELETE | `/api/admin/engagement/mise-en-avant…` | existantes |

**15 routes nouvelles**, 5 modifiées, 8 inchangées.

---

## 1. `GET /actions-disponibles` (R3) : l'antidote à la règle orpheline

Catalogue **déclaré par le code** (const Rust dans `handlers/admin/engagement.rs`), pas une table :

```jsonc
{ "success": true, "data": [
  { "type_action": "contribution_validee", "libelle_defaut": "Contribution validée par modération",
    "types_objet": ["codimoi", "video", "idea_force", "bad_habit", "fiche_pays"],
    "module": "codimoi_admin, vidafrica, gouvernance", "regle_existante": true },
  { "type_action": "proposition_media_validee", "libelle_defaut": "Proposition de média validée",
    "types_objet": ["chaine_tv", "station_radio", "programme_tele", "programme_radio"],
    "module": "admin/media_proposition", "regle_existante": true }
] }
```

L'écran de création propose ce catalogue en premier ; toute règle dont le `type_action` **n'y figure pas** est affichée avec la mention « non instrumentée, aucun point ne sera attribué tant que le code n'émet pas cette action ». La liste des règles renvoie donc aussi, par règle, `instrumentee: bool` et `nombre_mouvements: i64` : une règle active, instrumentée et à 0 mouvement depuis des semaines est un signal de branchement cassé.

## 2. Règles de points

**`POST /regles`** : corps : `type_action`, `libelle`, `points`, `reputation_delta`, `plafond_journalier?`, `plafond_mensuel?`, `seuil_declencheur?`, `categorie_id?`, `actif?`.

- `type_action` : trim, `^[a-z0-9_]{3,50}$` (c'est une clé, pas une phrase) ; **409** si déjà pris (FR-003), message explicite, pas une erreur SQL brute.
- `points` peut être négatif (malus).
- Effet immédiat : la règle est lue à chaque attribution (`charger_regle`), sans cache ni redéploiement (FR-007).

**`PUT /regles/{id}`** : accepte en plus `categorie_id` et `seuil_declencheur`. `type_action` **immuable** (le modifier orphelinerait tous les mouvements passés qui le référencent par chaîne).

**`DELETE /regles/{id}`** : **409** si `EXISTS(SELECT 1 FROM mouvement_points WHERE type_action = …)` (FR-002), avec le message « Cette règle a déjà attribué des points : désactivez-la au lieu de la supprimer ». Sinon suppression réelle (règle créée par erreur, jamais utilisée).

> ⚠️ **Piège de paramétrage à documenter dans l'UI** : `plafond_journalier` / `plafond_mensuel` sont exprimés **en points**, pas en nombre d'actions (le moteur compare `SUM(points)`). « 3 bonus de 10 points par jour » = plafond **30**.

## 3. Catégories de points (FR-004)

- **`GET /categories`** : toutes, avec `nombre_regles` (pour savoir ce qui est supprimable).
- **`POST /categories`** : `code` (`^[a-z0-9_]{3,30}$`, unique, **immuable** ensuite), `libelle`, `description?`, `ordre?`, `couleur?`, `icone?`.
- **`PUT /categories/{id}`** : tout sauf `code`.
- **`DELETE /categories/{id}`** : **409** si au moins une règle la référence (`ON DELETE RESTRICT` en filet de sécurité). Les mouvements passés qui la portent ne bloquent pas la suppression (`ON DELETE SET NULL` → ils basculent en « Autres »), mais l'UI avertit du nombre de mouvements concernés avant confirmation.

## 4. Paliers de popularité (FR-005)

`POST` et `PUT` acceptent `type_objet` (`null` = palier global). **409** si `(seuil_likes, type_objet)` existe déjà (index `NULLS NOT DISTINCT`).

L'écran affiche les paliers **groupés par famille**, avec un rappel de la règle de substitution : « les paliers d'une famille remplacent les paliers globaux pour cette famille ». Sans ce rappel, un administrateur croira que les deux se cumulent.

## 5. Niveaux (FR-006)

**`POST /niveaux`** : `code` (`^[a-z0-9_]{3,30}$`, unique), `libelle`, `seuil_min`, `badge_couleur?`, `badge_icone?`.
**`PUT /niveaux/{id}`** : `libelle`, `seuil_min`, apparence. `code` immuable (`compte.niveau_code` le référence par valeur).
**`DELETE /niveaux/{id}`** : **409** si `seuil_min = 0` (niveau plancher) ou s'il ne resterait qu'un niveau.

Les trois routes exécutent, **dans la même transaction**, le recalcul ensembliste de `ordre` (d'après `seuil_min` croissant) **et** de `compte.niveau_code` pour tous les comptes (R5). Réponse : `{ niveaux: [...], comptes_recalcules: 1423 }`, le nombre est affiché à l'administrateur, qui voit ainsi l'effet réel de son geste.

## 6. Badges (FR-017, FR-019)

**`GET /badges`** : catalogue complet + `nombre_detenteurs` par badge.

**`POST /badges`** / **`PUT /badges/{id}`**, corps :

```jsonc
{ "code": "batisseur_medias", "libelle": "Bâtisseur de médias",
  "description": "50 points gagnés dans la catégorie Médias",
  "couleur": "amber", "icone": "tower-broadcast", "ordre": 6, "actif": true,
  "manuel": false, "type_condition": "points_categorie",
  "parametre_categorie_id": "uuid", "parametre_action": null,
  "parametre_niveau_code": null, "seuil": 50 }
```

Validation applicative **miroir du CHECK SQL** `ck_badge_condition` (400 avec un message en français plutôt qu'une violation de contrainte remontée telle quelle). `code` immuable après création.

**`DELETE /badges/{id}`** : **409** si des `badge_obtenu` existent (« désactivez-le : les membres qui l'ont obtenu doivent le conserver », FR-020). Sinon suppression réelle.

**`POST /badges/{id}/attribuer`** : corps `{ "utilisateur_id": "uuid", "motif": "…" }` :
`INSERT … origine = 'manuel', attribue_par = admin.id … ON CONFLICT DO NOTHING`, notification `engagement.badge_debloque` **si** une ligne a été créée, audit avec le motif.

**`DELETE /badges/{id}/attribuer/{utilisateur_id}`**, retrait, audité. **Aucune** notification (on n'annonce pas un retrait à un membre ; c'est un geste de correction).

## 7. Journal global : filtre ajouté

`GET /journal` accepte en plus **`categorie`** (code), aux côtés de `utilisateur_id`, `type_action`, `depuis`, `jusqu_a`, `page`, `taille`. Chaque ligne gagne `categorie_code` / `categorie_libelle` (FR-009).

---

## Codes de réponse

| Code | Quand |
|---|---|
| 200 | Lecture ou mutation réussie |
| 201 | Création (`POST` de règle, catégorie, niveau, badge) |
| 400 | Corps invalide : `code`/`type_action` malformé, condition de badge incohérente, réseau inconnu |
| 401 | JWT absent ou expiré |
| 403 | Permission `engagement.gerer` manquante, message nommant la permission requise |
| 404 | Identifiant inexistant |
| 409 | Doublon (`type_action`, `code`, `(seuil_likes, type_objet)`, `seuil_min`) ou suppression refusée (règle/catégorie/badge/niveau encore utilisés) |

> **Ce que le 403 ne fait pas** : `verifier_permission!` (`middleware/admin.rs:142-150`) renvoie `AccesInterdit` **sans écrire dans la piste d'audit**, l'audit ne consigne que les opérations appliquées. Un refus se constate donc dans les journaux techniques du serveur, pas dans `/admin/audit`. **Ne pas** instrumenter les refus dans la macro pour cette feature : elle est partagée par la totalité des routes d'administration de la plateforme (une centaine), et ce serait un changement transverse hors périmètre.

## Invariants de sécurité

1. Aucune route de ce contrat n'est accessible sans `AdminUtilisateur` **et** `engagement.gerer`, y compris les `GET` (le catalogue des règles révèle la mécanique anti-abus).
2. Aucune interpolation de valeur utilisateur dans du SQL : les filtres sont des `$n` castés, les noms de table proviennent de `match` sur littéraux.
3. L'attribution manuelle de badge et l'ajustement de points ne peuvent pas être auto-appliqués silencieusement : `attribue_par` / l'audit conservent l'identité de l'administrateur, y compris s'il se cible lui-même.
4. Aucune route n'écrit dans `mouvement_points` autrement que par `services::engagement`, pas de crédit « à la main » en SQL depuis un handler.
