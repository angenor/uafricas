# Contrats API : Système d'engagement (Phase 1)

Toutes les réponses suivent le wrapper projet `ApiResponse<T>` (`{ success, data, message }`). Auth : JWT (`Authorization: Bearer …`). Erreurs standard : 401 (non authentifié), 403 (permission manquante / auto-attribution interdite), 404, 409 (conflit), 422 (validation).

> **Aucun endpoint ne « donne » de points directement.** Les gains proviennent d'actions métier existantes (validation modération, jugement factcheck, like) qui appellent le service en interne. Les endpoints ci-dessous exposent la **lecture** (public) et l'**administration du barème** (admin).

---

## Public : `/api/engagement` (JWT requis sauf mention)

### `GET /api/engagement/mon-compte`
Compte d'engagement du membre connecté.
```json
{
  "success": true,
  "data": {
    "solde_points": 250,
    "solde_points_mensuel": 40,
    "reputation": 12,
    "niveau": { "code": "premium", "libelle": "Membre Premium", "badge_couleur": "amber", "badge_icone": "star" },
    "prochain_niveau": { "code": "platinum", "libelle": "Influenceur Platinum", "seuil_min": 1000, "points_restants": 750 },
    "dernier_mouvement_at": "2026-07-06T10:12:00Z"
  }
}
```
`prochain_niveau` = `null` si déjà au niveau maximal.

### `GET /api/engagement/mon-journal?page={n}&taille={m}&type_action={?}`
Historique paginé des mouvements du membre connecté (le plus récent d'abord). Réutilise la pagination projet (`listerPagine`).
```json
{
  "success": true,
  "data": {
    "elements": [
      { "id": "…", "type_action": "contribution_validee", "libelle": "Contribution validée par modération",
        "type_objet": "codimoi", "objet_id": "…", "points": 2, "reputation_delta": 0,
        "solde_apres": 250, "plafond_atteint": false, "created_at": "2026-07-06T10:12:00Z" }
    ],
    "total": 37, "page": 1, "taille": 20
  }
}
```

### `GET /api/engagement/niveau/{utilisateur_id}`  *(public, léger, badge)*
Niveau + code d'un membre, pour afficher le **badge** sur son profil public et **sous ses contenus**. N'expose **pas** le solde exact ni le journal (FR-019).
```json
{ "success": true, "data": { "code": "platinum", "libelle": "Influenceur Platinum", "badge_couleur": "slate", "badge_icone": "crown" } }
```

---

## Admin : `/api/admin/engagement` (JWT + permission `engagement`)

### Règles de barème
- `GET /api/admin/engagement/regles` : liste des règles.
- `PUT /api/admin/engagement/regles/{id}`, modifier `points`, `reputation_delta`, `plafond_journalier`, `plafond_mensuel`, `actif`, `libelle`. Audité (`log_action`, schema `engagement`).

```json
// PUT body
{ "libelle": "Contribution validée par modération", "points": 3,
  "reputation_delta": 0, "plafond_journalier": 50, "plafond_mensuel": 500, "actif": true }
```

### Paliers de popularité
- `GET /api/admin/engagement/paliers`
- `POST /api/admin/engagement/paliers`, `{ "seuil_likes": 2000, "points": 80, "actif": true }`
- `PUT /api/admin/engagement/paliers/{id}`, modifier `points` / `actif` (le seuil est unique).
- `DELETE /api/admin/engagement/paliers/{id}`, désactivation (`actif=false`) plutôt que suppression physique (référencé par le journal).

### Seuils de niveaux
- `GET /api/admin/engagement/niveaux`
- `PUT /api/admin/engagement/niveaux/{id}`, modifier `seuil_min`, `libelle`, `badge_couleur`, `badge_icone`. Audité. (Le `code` et l'`ordre` restent fixes.)

### Journal global
- `GET /api/admin/engagement/journal?utilisateur_id={?}&type_action={?}&depuis={?}&jusqu_a={?}&page&taille`
  Journal filtrable (membre, type d'action, période). Réponse paginée identique à `mon-journal`, enrichie du nom du membre.

### Ajustement manuel
- `POST /api/admin/engagement/ajustement`
  Crédit/débit manuel motivé (litige, geste commercial). Crée un mouvement `type_action = 'ajustement_admin'`, audité.
```json
{ "utilisateur_id": "…", "points": -10, "reputation_delta": 0, "motif": "Correction double comptage palier" }
```
Réponse : le mouvement créé + le nouveau solde.

---

## Invariants vérifiables (rappels contractuels)

- Rejouer une action déjà créditée (même `cle_idempotence`) ⇒ **aucun** nouveau mouvement, solde inchangé (SC-004).
- Un membre ne peut jamais lire le **journal détaillé** d'un autre membre via le public (seul `niveau/{id}` est ouvert) (FR-019).
- Un gain au-delà d'un plafond actif ⇒ mouvement avec `plafond_atteint=true` et `points` écrêté (SC-007).
- Toute modification de barème ou ajustement admin ⇒ une entrée d'audit (FR-023 / Principe VII).
