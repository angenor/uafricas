# Contrats API REST : Arbre généalogique

**Branch**: `001-personnes-arbre` | **Date**: 2026-03-15
**Base URL**: `http://localhost:8080/api/arbre`
**Auth**: JWT Bearer token requis sur tous les endpoints
**Format**: JSON `{ "success": bool, "data": T | null, "error": string | null }`

---

## Personnes

### `GET /api/arbre/personnes`

Liste paginée des personnes de l'arbre de l'utilisateur connecté.

**Query params**:

| Paramètre | Type | Défaut | Description |
|-----------|------|--------|-------------|
| `page` | int | 1 | Numéro de page (≥ 1) |
| `par_page` | int | 12 | Résultats par page (1–50) |
| `recherche` | string | : | Filtre sur nom / prénoms (ILIKE) |

**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "personnes": [
      {
        "id": "uuid",
        "nom": "Diallo",
        "prenoms": "Ibrahim",
        "genre": "masculin",
        "naissance": { "annee": 1850, "mois": null, "jour": null },
        "naissance_lieu": "Ségou, Mali",
        "deces": null,
        "deces_lieu": null,
        "photo_url": null,
        "created_at": "2026-03-15T10:00:00Z"
      }
    ],
    "total": 42,
    "page": 1,
    "par_page": 12,
    "total_pages": 4
  },
  "error": null
}
```

---

### `POST /api/arbre/personnes`

Crée une personne et la rattache automatiquement à l'arbre de l'utilisateur. Crée l'arbre s'il n'existe pas encore.

**Corps** :
```json
{
  "nom": "Diallo",
  "prenoms": "Ibrahim",
  "genre": "masculin",
  "naissance": { "annee": 1850, "mois": null, "jour": null },
  "naissance_lieu": "Ségou, Mali",
  "deces": null,
  "deces_lieu": null
}
```

**Validations** :
- `nom` : obligatoire, non vide
- `genre` : si fourni, l'une de `masculin | feminin | autre | non_precise`
- Cohérence des dates : si `naissance.annee` et `deces.annee` fournis → `deces.annee >= naissance.annee`
- Photo : non incluse dans ce body : à uploader séparément via `POST /api/arbre/personnes/:id/photo`

**Réponse 201** :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "rattachement_id": "uuid",
    "nom": "Diallo",
    ...
  },
  "error": null
}
```

**Erreurs** :
- `400` : `nom` manquant ou vide
- `422` : incohérence de dates (décès avant naissance)
- `401` : token manquant ou expiré

---

### `GET /api/arbre/personnes/:id`

Détail d'une personne avec ses liens familiaux directs (parents, enfants, conjoints).

**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "personne": { "id": "uuid", "nom": "Diallo", ... },
    "parents": [
      { "lien_id": "uuid", "type_lien": "mere", "personne": { ... } }
    ],
    "enfants": [
      { "lien_id": "uuid", "type_lien": "enfant", "personne": { ... } }
    ],
    "conjoints": [
      { "lien_id": "uuid", "type_lien": "conjoint", "personne": { ... } }
    ]
  },
  "error": null
}
```

**Erreurs** :
- `404` : personne introuvable ou n'appartient pas à l'arbre de l'utilisateur
- `401` : non authentifié

---

### `PUT /api/arbre/personnes/:id`

Modifie les informations biographiques d'une personne.

**Corps** (tous les champs sont optionnels, seuls les champs présents sont mis à jour) :
```json
{
  "nom": "Diallo",
  "naissance": { "annee": 1852 }
}
```

**Validations** : idem `POST`, appliquées aux champs fournis.

**Réponse 200** : même structure que `GET /api/arbre/personnes/:id` (données mises à jour).

**Erreurs** :
- `400` : `nom` fourni mais vide
- `422` : incohérence de dates
- `403` : personne n'appartient pas à l'arbre de l'utilisateur
- `404` : personne introuvable

---

### `DELETE /api/arbre/personnes/:id`

Supprime le rattachement de la personne dans l'arbre. Si c'est le dernier rattachement de cette Personne réelle, supprime également la Personne et ses liens familiaux (soft delete en cascade). Tout dans une transaction atomique.

**Réponse 200** :
```json
{
  "success": true,
  "data": { "message": "Personne supprimée de l'arbre" },
  "error": null
}
```

**Erreurs** :
- `403` : n'appartient pas à l'arbre de l'utilisateur
- `404` : personne introuvable

---

### `POST /api/arbre/personnes/:id/photo`

Upload de la photo d'une personne (multipart/form-data).

**Corps** : `multipart/form-data`, champ `photo` (image JPEG/PNG/WebP, max 5 Mo).

**Réponse 200** :
```json
{
  "success": true,
  "data": { "photo_url": "/uploads/personnes/uuid.jpg" },
  "error": null
}
```

---

## Liens familiaux

### `POST /api/arbre/liens`

Crée un lien familial entre deux personnes de l'arbre de l'utilisateur.

**Corps** :
```json
{
  "rattachement_source_id": "uuid-du-parent",
  "rattachement_cible_id": "uuid-de-l-enfant",
  "type_lien": "pere"
}
```

**Valeurs de `type_lien`** :

| Valeur | Sens |
|--------|------|
| `pere` | source est le père de cible |
| `mere` | source est la mère de cible |
| `parent` | source est le parent (non précisé) de cible |
| `conjoint` | source et cible sont conjoints (symétrique) |

**Validations** :
- Les deux rattachements doivent appartenir à l'arbre de l'utilisateur
- `rattachement_source_id ≠ rattachement_cible_id`
- Pas de doublons (même triplet source + cible + type)
- Pour les types `pere|mere|parent` : vérification de l'absence de cycle (recursive CTE)

**Réponse 201** :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "type_lien": "pere",
    "personne_source_id": "uuid",
    "personne_cible_id": "uuid",
    "created_at": "2026-03-15T11:00:00Z"
  },
  "error": null
}
```

**Erreurs** :
- `400` : type_lien invalide ou source = cible
- `403` : rattachement n'appartient pas à l'arbre de l'utilisateur
- `409` : lien identique déjà existant
- `422` : lien circulaire détecté

---

### `DELETE /api/arbre/liens/:id`

Supprime un lien familial (soft delete).

**Réponse 200** :
```json
{
  "success": true,
  "data": { "message": "Lien familial supprimé" },
  "error": null
}
```

**Erreurs** :
- `403` : lien n'appartient pas à l'arbre de l'utilisateur
- `404` : lien introuvable

---

## Codes d'erreur utilisés

| Code HTTP | Signification dans ce contexte |
|-----------|-------------------------------|
| 200 | Succès |
| 201 | Ressource créée |
| 400 | Validation échouée (champ obligatoire manquant, valeur invalide) |
| 401 | Non authentifié (JWT absent ou expiré) |
| 403 | Accès refusé (ressource appartenant à un autre utilisateur) |
| 404 | Ressource introuvable |
| 409 | Conflit (doublon de lien) |
| 422 | Règle métier violée (incohérence de dates, cycle détecté) |
| 500 | Erreur serveur interne |
