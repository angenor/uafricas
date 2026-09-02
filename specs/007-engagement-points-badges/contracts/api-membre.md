# Contrat : API membre (JWT requis)

**Feature** : `007-engagement-points-badges`
Authentification : `Authorization: Bearer <access_token>` → `extraire_utilisateur_id(&req)` (`handlers/engagement.rs`).
**Ne jamais utiliser l'extracteur `AdminUtilisateur`** sur ces routes : ce sont des routes de membre ordinaire.
Enveloppe de réponse : `ApiResponse<T>` (`{ success, data, error }`).
Scope Actix : `web::scope("/engagement")`, monté dans `routes.rs` à côté des 3 routes existantes.

## Récapitulatif

| Méthode | Chemin | État | Objet |
|---|---|---|---|
| GET | `/api/engagement/mon-compte` | existante, inchangée | Soldes, réputation, niveau, prochain niveau |
| GET | `/api/engagement/mon-journal` | **modifiée** | Historique paginé, nouveaux filtres |
| GET | `/api/engagement/niveau/{utilisateur_id}` | existante, inchangée | Badge de niveau public |
| GET | `/api/engagement/mes-categories` | **NEW** | Ventilation des points par catégorie |
| GET | `/api/engagement/mes-badges` | **NEW** | Badges obtenus + catalogue à débloquer |
| GET | `/api/engagement/badges/{utilisateur_id}` | **NEW** | Badges **publics** d'un membre |
| GET | `/api/engagement/actions-recompensees` | **NEW** | Barème public (état vide pédagogique) |
| POST | `/api/engagement/partages-externes` | **NEW** | Traçage d'un partage externe |

---

## 1. `GET /mon-journal` : filtres ajoutés (FR-012)

Paramètres : `page` (défaut 1), `taille` (défaut 20, max 100), `type_action` *(existant)*, **`categorie`** *(code de catégorie)*, **`depuis`** / **`jusqu_a`** *(dates ISO `YYYY-MM-DD`)*.

Les filtres nuls sont neutralisés par cast paramétré (`$n::text IS NULL OR …`), comme `admin/engagement::lister_journal`, jamais de concaténation de fragments SQL.

`MouvementResponse` gagne deux champs :

```jsonc
{
  "id": "…", "type_action": "media_a_la_une",
  "libelle": "Contenu média mis à la une",      // libellé de la règle (jamais de texte figé côté front)
  "categorie_code": "medias",                    // NEW, null si mouvement antérieur au rattrapage
  "categorie_libelle": "Médias (télé & radio)",  // NEW
  "type_objet": "programme_tele", "objet_id": "…",
  "points": 8, "reputation_delta": 1, "solde_apres": 258,
  "plafond_atteint": false,
  "created_at": "2026-07-29T10:12:00Z"
}
```

`plafond_atteint = true` avec `points = 0` signifie « plafond atteint, aucun point crédité » ; avec `points > 0`, « écrêté à N points » (R14).

> **Asymétrie assumée** : `libelle` est lu dans la **règle courante** (renommer une règle renomme donc l'action dans tout l'historique affiché), tandis que `categorie_code` / `categorie_libelle` viennent de la **catégorie figée à l'écriture** (une re-catégorisation ne réécrit pas le passé, R1). C'est délibéré : un libellé est un texte d'affichage qu'on veut pouvoir corriger partout, une catégorie est une donnée d'agrégation dont la stabilité conditionne la ventilation.

## 2. `GET /mes-categories` (FR-011)

```jsonc
{ "success": true, "data": {
  "solde_points": 258,             // compte.solde_points, le solde COURANT
  "total_gagne": 271,              // SUM(points) du journal, toutes catégories
  "categories": [
    { "code": "medias", "libelle": "Médias (télé & radio)", "couleur": "amber",
      "icone": "tv", "ordre": 3, "points": 128, "nombre_mouvements": 14 },
    { "code": null, "libelle": "Autres", "couleur": null, "icone": null,
      "ordre": 99, "points": 6, "nombre_mouvements": 3 }
  ] } }
```

- Une seule requête : `SUM(points)`, `COUNT(*)` `GROUP BY categorie_id` joint sur `categorie_points`, `ORDER BY ordre`.
- Les catégories à 0 mouvement **ne sont pas** renvoyées (l'écran affiche l'état vide pédagogique).
- `total_gagne` peut dépasser `solde_points` à cause du plancher 0 : les deux sont exposés séparément et libellés distinctement côté UI (R2).

## 3. `GET /mes-badges` (FR-013, FR-018)

Effet de bord assumé : appelle `evaluer_badges` **avant** de répondre (R7), ce qui rattrape les conditions devenues vraies sans mouvement.

```jsonc
{ "success": true, "data": {
  "obtenus": [
    { "code": "conteur", "libelle": "Conteur", "description": "10 contributions validées",
      "couleur": "green", "icone": "feather", "origine": "automatique",
      "obtenu_at": "2026-07-20T08:00:00Z" }
  ],
  "a_debloquer": [
    { "code": "pilier", "libelle": "Pilier", "description": "50 contributions validées",
      "couleur": "chocolat", "icone": "landmark",
      "progression_actuelle": 12, "progression_cible": 50 }   // null/null si non chiffrable
  ] } }
```

- `a_debloquer` exclut les badges `actif = FALSE` et les badges `manuel = TRUE` (une distinction éditoriale ne se « débloque » pas).
- Un badge `actif = FALSE` déjà obtenu **reste** dans `obtenus` (FR-020).
- `progression_actuelle` est calculée par le même SQL que la condition, borné à `progression_cible`.

## 4. `GET /badges/{utilisateur_id}` (FR-014)

Public (aucun JWT requis), comme `GET /niveau/{utilisateur_id}` : renvoie **uniquement** les badges obtenus (code, libellé, description, couleur, icône, date). **Jamais** de solde, de réputation ni de mouvement, le détail chiffré reste privé.

Consommé par `pages/profil/[id].vue` à côté de `EngagementBadgeStatut`.

## 5. `GET /actions-recompensees` (FR-015, FR-016)

Liste des règles `actif = TRUE` : `type_action`, `libelle`, `points`, `categorie_code`, `categorie_libelle`, `plafond_journalier`, `seuil_declencheur`. Sert l'état vide (« voici ce qui rapporte des points ») et garantit qu'**aucun libellé ni montant du barème n'est écrit en dur dans le front**.

Public : le barème n'est pas une donnée sensible, et l'afficher aux visiteurs sert l'engagement.

## 6. `POST /partages-externes` (FR-027)

```jsonc
// requête
{ "type_objet": "programme_tele", "objet_id": "uuid", "reseau": "telegram" }
```

- `utilisateur_id` **toujours** pris du JWT (jamais du corps).
- `reseau` ∈ `whatsapp | facebook | x | linkedin | telegram | email`, toute autre valeur → 400.
- `type_objet` validé contre la liste des familles partageables (littéraux fixes).
- Séquence : `INSERT … ON CONFLICT DO NOTHING` → `COUNT(DISTINCT reseau)` → si `>= regle.seuil_declencheur`, `attribuer(...)` **après** la réponse préparée, en non-bloquant.

```jsonc
// réponse
{ "success": true, "data": { "reseaux_distincts": 5, "seuil": 5, "bonus_attribue": true } }
```

`bonus_attribue` reflète le franchissement du seuil, **pas** le crédit effectif : le plafond journalier peut écrêter à 0 point (l'écrêtage est alors visible dans le journal). Le front ne doit rien promettre à l'utilisateur sur la base de ce champ : il sert au libellé « partagé sur 5 réseaux ✓ ».

**Best-effort côté front** (`usePartageExterne`) : l'appel part **après** l'ouverture de la fenêtre de partage et son échec est silencieux (`.catch(() => {})`). Un traçage raté ne doit jamais empêcher un partage (scénario 5 de l'US5).

---

## Notes d'implémentation

- Les 5 routes nouvelles vont dans `handlers/engagement.rs`, leurs DTO dans `models/engagement.rs`, aucun fichier supplémentaire (Principe V).
- Le composable `useEngagement.ts` est étendu, pas dupliqué ; `usePartageExterne.ts` est créé à part parce qu'il est consommé par 6 modales qui n'ont rien à voir avec l'espace membre.
- Aucun de ces endpoints ne mute le barème : ils sont exempts d'audit administratif (le journal `mouvement_points` suffit à la traçabilité métier).
