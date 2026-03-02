# Contrats API Publics (sans authentification)

**Branch**: `002-partage-avis-recherche` | **Date**: 2026-03-02

Ces endpoints sont accessibles sans JWT. Ils sont enregistrés hors du scope d'authentification dans `routes.rs`.

---

## GET /api/retrouve-amis/public/{slug}

**Description**: Récupérer les détails d'un avis public par son slug.

**Paramètres path**:
| Param | Type | Description |
|-------|------|-------------|
| `slug` | String | Slug unique de l'avis (ex: `keita-fatou-a3f8b2c1`) |

**Réponse 200**:
```json
{
  "succes": true,
  "donnees": {
    "id": "uuid",
    "slug": "keita-fatou-a3f8b2c1",
    "nom_recherche": "Keita",
    "prenom_recherche": "Fatou",
    "ecole": "Lycée de Bamako",
    "ville": "Bamako",
    "pays": { "id": "uuid", "nom": "Mali" },
    "periode_debut": 2000,
    "periode_fin": 2005,
    "description": "Nous étions dans la même classe...",
    "auteur_anonyme": "Amadou D.",
    "etat": "actif",
    "compteur_partages": 12,
    "date_publication_publique": "2026-03-01T10:00:00Z",
    "created_at": "2026-02-28T08:00:00Z"
  }
}
```

**Réponse si avis non actif** (200 avec état):
```json
{
  "succes": true,
  "donnees": {
    "slug": "keita-fatou-a3f8b2c1",
    "etat": "cloture",
    "message": "Cette personne a été retrouvée !"
  }
}
```

**Réponse si avis suspendu** (200 avec état):
```json
{
  "succes": true,
  "donnees": {
    "slug": "keita-fatou-a3f8b2c1",
    "etat": "suspendu",
    "message": "Cet avis a été temporairement retiré."
  }
}
```

**Réponse 404** (slug inexistant ou avis dépublié):
```json
{
  "succes": false,
  "erreur": "Avis non disponible."
}
```

**Headers spéciaux**:
- Si `etat != actif` : ajouter `X-Robots-Tag: noindex, nofollow`

---

## GET /api/retrouve-amis/public/rechercher

**Description**: Lister et rechercher parmi les avis publics actifs.

**Paramètres query**:
| Param | Type | Défaut | Description |
|-------|------|--------|-------------|
| `page` | u32 | 1 | Numéro de page |
| `par_page` | u32 | 12 | Résultats par page (max 50) |
| `recherche` | String | — | Recherche full-text (TSVECTOR) |
| `pays_id` | UUID | — | Filtrer par pays |
| `ville` | String | — | Filtrer par ville (ILIKE) |
| `ecole` | String | — | Filtrer par école (ILIKE) |
| `tri` | String | "created_at" | Champ de tri (created_at, compteur_partages) |
| `ordre` | String | "desc" | Ordre (asc, desc) |

**Réponse 200**:
```json
{
  "succes": true,
  "donnees": {
    "avis": [
      {
        "slug": "keita-fatou-a3f8b2c1",
        "nom_recherche": "Keita",
        "prenom_recherche": "Fatou",
        "ville": "Bamako",
        "pays": { "id": "uuid", "nom": "Mali" },
        "periode_debut": 2000,
        "periode_fin": 2005,
        "compteur_partages": 12,
        "created_at": "2026-02-28T08:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "par_page": 12,
      "total": 45,
      "pages": 4
    }
  }
}
```

**Filtre implicite**: Seuls les avis avec `est_public = TRUE AND etat = 'actif' AND deleted_at IS NULL` sont retournés.

---

## POST /api/retrouve-amis/public/{slug}/partage

**Description**: Incrémenter le compteur de partages d'un avis.

**Paramètres path**:
| Param | Type | Description |
|-------|------|-------------|
| `slug` | String | Slug unique de l'avis |

**Corps**: Aucun (POST vide)

**Réponse 200**:
```json
{
  "succes": true,
  "donnees": {
    "compteur_partages": 13
  }
}
```

**Réponse 404** (slug inexistant ou avis non public/non actif):
```json
{
  "succes": false,
  "erreur": "Avis non disponible."
}
```

---

## GET /api/retrouve-amis/pays

**Description**: Lister les pays actifs (endpoint existant, utilisé aussi pour les filtres publics).

**Note**: Cet endpoint existe déjà et est public. Il sera réutilisé pour le dropdown de filtres sur la page de recherche.

**Réponse 200**:
```json
{
  "succes": true,
  "donnees": [
    { "id": "uuid", "nom": "Mali" },
    { "id": "uuid", "nom": "Sénégal" }
  ]
}
```
