# Contrats API — Public Vidafrica

**Base**: `/api/vidafrica`  
**Auth**: Aucune (endpoints publics)

## Vidéos

### `GET /videos` — Lister les vidéos publiées (paginé)

Retourne uniquement les vidéos avec `etat = 'publie'`.

**Query params** :

| Param | Type | Défaut | Description |
|-------|------|--------|-------------|
| `page` | integer | 1 | Page courante |
| `par_page` | integer | 20 | Éléments par page |
| `recherche` | string | | Recherche full-text (titre + description) |
| `langue` | string | | Filtre par langue de sous-titres disponible |

**Response** : `200 OK`
```json
{
  "succes": true,
  "donnees": [
    {
      "id": "uuid",
      "titre": "string",
      "slug": "string",
      "description": "string|null",
      "vignette_url": "string|null",
      "duree_secondes": "integer|null",
      "langues_disponibles": ["francais", "anglais"],
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

### `GET /videos/{slug}` — Détail d'une vidéo publiée (par slug)

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
    "langues_disponibles": ["francais", "anglais", "wolof"],
    "created_at": "datetime"
  }
}
```

### `GET /videos/{video_id}/sous-titres/{langue}` — Récupérer les sous-titres d'une vidéo dans une langue

Retourne tous les segments avec leurs timings mot pour une piste donnée.

**Response** : `200 OK`
```json
{
  "succes": true,
  "donnees": {
    "langue": "francais",
    "segments": [
      {
        "position": 1,
        "texte": "Bonjour tout le monde",
        "debut_ms": 1500,
        "fin_ms": 4200,
        "mots": [
          { "position": 1, "mot": "Bonjour", "debut_ms": 1500, "fin_ms": 2300 },
          { "position": 2, "mot": "tout", "debut_ms": 2300, "fin_ms": 2800 },
          { "position": 3, "mot": "le", "debut_ms": 2800, "fin_ms": 3100 },
          { "position": 4, "mot": "monde", "debut_ms": 3100, "fin_ms": 4200 }
        ]
      }
    ]
  }
}
```

Note : si `mots` est un tableau vide pour un segment, le frontend affiche le segment entier sans effet karaoké.

### `GET /langues-sous-titres` — Lister les langues disponibles

Retourne la liste des langues pour lesquelles au moins une vidéo publiée a une piste de sous-titres.

**Response** : `200 OK`
```json
{
  "succes": true,
  "donnees": [
    { "code": "francais", "label": "Français", "nombre_videos": 12 },
    { "code": "anglais", "label": "Anglais", "nombre_videos": 8 },
    { "code": "wolof", "label": "Wolof", "nombre_videos": 3 }
  ]
}
```
