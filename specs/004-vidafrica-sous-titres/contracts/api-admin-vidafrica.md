# Contrats API — Admin Vidafrica

**Base**: `/api/admin/vidafrica`  
**Auth**: JWT Bearer (rôle admin requis)

## Vidéos

### `GET /videos` — Lister les vidéos (paginé)

**Query params** :

| Param | Type | Défaut | Description |
|-------|------|--------|-------------|
| `page` | integer | 1 | Page courante |
| `par_page` | integer | 20 | Éléments par page |
| `tri_par` | string | "created_at" | Colonne de tri |
| `tri_dir` | string | "desc" | Direction (asc/desc) |
| `recherche` | string | | Recherche titre/description |
| `etat` | string | | Filtre par état |

**Response** : `200 OK`
```json
{
  "succes": true,
  "donnees": [
    {
      "id": "uuid",
      "titre": "string",
      "slug": "string",
      "vignette_url": "string|null",
      "duree_secondes": "integer|null",
      "etat": "string",
      "nombre_pistes": "integer",
      "created_at": "datetime"
    }
  ],
  "pagination": {
    "page": 1,
    "par_page": 20,
    "total": 42
  }
}
```

### `GET /videos/{id}` — Détail d'une vidéo

**Response** : `200 OK`
```json
{
  "succes": true,
  "donnees": {
    "id": "uuid",
    "titre": "string",
    "slug": "string",
    "description": "string|null",
    "fichier_video_url": "string",
    "vignette_url": "string|null",
    "duree_secondes": "integer|null",
    "taille_octets": "integer|null",
    "format_video": "string|null",
    "etat": "string",
    "cree_par": "uuid",
    "cree_par_nom": "string",
    "pistes": [
      {
        "id": "uuid",
        "langue": "string",
        "est_complete": "boolean",
        "nombre_segments": "integer"
      }
    ],
    "created_at": "datetime",
    "updated_at": "datetime"
  }
}
```

### `POST /videos` — Créer une vidéo (multipart)

**Content-Type**: `multipart/form-data`

| Champ | Type | Requis | Description |
|-------|------|--------|-------------|
| `titre` | string | oui | Titre de la vidéo |
| `description` | string | non | Description |
| `fichier_video` | file | oui | Fichier vidéo (MP4, WebM, max 500Mo) |
| `vignette` | file | non | Vignette (JPG, PNG, WebP, max 5Mo) |

**Response** : `201 Created` — même format que GET détail

### `PUT /videos/{id}` — Modifier une vidéo (multipart)

**Content-Type**: `multipart/form-data`

| Champ | Type | Requis | Description |
|-------|------|--------|-------------|
| `titre` | string | non | Nouveau titre |
| `description` | string | non | Nouvelle description |
| `vignette` | file | non | Nouvelle vignette |

Note : le fichier vidéo ne peut pas être remplacé — supprimer et recréer la vidéo.

**Response** : `200 OK`

### `PATCH /videos/{id}/etat` — Changer l'état

**Body** :
```json
{ "etat": "publie" }
```

**Response** : `200 OK`

### `DELETE /videos/{id}` — Supprimer une vidéo (soft delete)

**Response** : `200 OK`

---

## Pistes de sous-titres

### `GET /videos/{video_id}/pistes` — Lister les pistes d'une vidéo

**Response** : `200 OK`
```json
{
  "succes": true,
  "donnees": [
    {
      "id": "uuid",
      "langue": "string",
      "est_complete": "boolean",
      "nombre_segments": "integer",
      "created_at": "datetime"
    }
  ]
}
```

### `POST /videos/{video_id}/pistes` — Créer une piste

**Body** :
```json
{ "langue": "francais" }
```

**Response** : `201 Created`
**Erreur** : `409 Conflict` si une piste dans cette langue existe déjà

### `DELETE /pistes/{id}` — Supprimer une piste (soft delete)

**Response** : `200 OK`

---

## Segments de sous-titres

### `GET /pistes/{piste_id}/segments` — Lister les segments (ordonnés par position)

**Response** : `200 OK`
```json
{
  "succes": true,
  "donnees": [
    {
      "id": "uuid",
      "position": 1,
      "texte": "Bonjour tout le monde",
      "debut_ms": 1500,
      "fin_ms": 4200,
      "timings_mot": [
        { "position": 1, "mot": "Bonjour", "debut_ms": 1500, "fin_ms": 2300 },
        { "position": 2, "mot": "tout", "debut_ms": 2300, "fin_ms": 2800 },
        { "position": 3, "mot": "le", "debut_ms": 2800, "fin_ms": 3100 },
        { "position": 4, "mot": "monde", "debut_ms": 3100, "fin_ms": 4200 }
      ]
    }
  ]
}
```

### `POST /pistes/{piste_id}/segments` — Créer un segment

**Body** :
```json
{
  "texte": "Bonjour tout le monde",
  "debut_ms": 1500,
  "fin_ms": 4200
}
```

**Response** : `201 Created` (position auto-incrémentée)

### `PUT /segments/{id}` — Modifier un segment

**Body** :
```json
{
  "texte": "string (optionnel)",
  "debut_ms": "integer (optionnel)",
  "fin_ms": "integer (optionnel)"
}
```

**Response** : `200 OK`

### `DELETE /segments/{id}` — Supprimer un segment

**Response** : `200 OK` — suppression physique + CASCADE sur timings_mot

### `PUT /pistes/{piste_id}/segments/reordonner` — Réordonner les segments

**Body** :
```json
{
  "ordre": ["uuid-segment-3", "uuid-segment-1", "uuid-segment-2"]
}
```

**Response** : `200 OK`

---

## Timings mot (tap-to-mark)

### `POST /segments/{segment_id}/timings-mot` — Enregistrer les timings mot (batch)

Enregistre tous les timings mot d'un segment en une seule requête (résultat du tap-to-mark).
Remplace les timings existants s'il y en a.

**Body** :
```json
{
  "timings": [
    { "position": 1, "mot": "Bonjour", "debut_ms": 1500, "fin_ms": 2300 },
    { "position": 2, "mot": "tout", "debut_ms": 2300, "fin_ms": 2800 },
    { "position": 3, "mot": "le", "debut_ms": 2800, "fin_ms": 3100 },
    { "position": 4, "mot": "monde", "debut_ms": 3100, "fin_ms": 4200 }
  ]
}
```

**Response** : `201 Created`

### `DELETE /segments/{segment_id}/timings-mot` — Supprimer tous les timings mot d'un segment

**Response** : `200 OK`
