# Contract : API publique

## `GET /api/afrolang/salles`

**Modifié** : la réponse existante est enrichie d'un champ `pays_origine`. Un nouveau paramètre `pays_id` est accepté.

### Query parameters (ajouts uniquement)

| Param      | Type   | Required | Notes                                                                 |
|------------|--------|----------|-----------------------------------------------------------------------|
| `pays_id`  | UUID   | non      | Filtre mono-valué (Q2). Ne renvoie que les salles ayant ce pays parmi leurs pays d'origine **actifs**. |

> Tous les paramètres existants (`page`, `par_page`, `langue`, `langue_code`, `groupe_ethnique_id`, `recherche`) sont conservés à l'identique.

### Response : schéma de chaque élément `salles[]`

Champ ajouté :

```json
{
  "pays_origine": [
    { "id": "uuid-pays-1", "nom": "Sénégal", "code_iso2": "SN" },
    { "id": "uuid-pays-2", "nom": "Gambie",  "code_iso2": "GM" }
  ]
}
```

**Garanties** :

- Toujours présent (jamais omis, jamais `null`). Tableau vide `[]` si aucun pays n'est associé ou si tous les pays associés sont archivés (Q3).
- Trié par `nom` croissant (FR-003), insensible à la casse au niveau PostgreSQL (collation par défaut).
- Les pays archivés (`shared.pays.actif = false`) sont **filtrés** (Q3), invisibles côté public.

### Codes de retour

| Code | Cas |
|------|-----|
| 200  | Succès (avec ou sans résultats)                                                      |
| 400  | `pays_id` mal formé (UUID invalide)                                                  |

> Un `pays_id` valide mais inconnu ou archivé renvoie 200 avec une liste vide, comportement intentionnel et cohérent avec les autres filtres.

## Exemple de requête

```http
GET /api/afrolang/salles?pays_id=11111111-1111-1111-1111-111111111111&par_page=12 HTTP/1.1
```

### Exemple de réponse (extrait)

```json
{
  "success": true,
  "data": {
    "salles": [
      {
        "id": "...",
        "titre": "Wolof",
        "langue_cible": "Wolof",
        "groupe_ethnique": { "id": "...", "nom": "Wolof", "fiche_pays_id": "...", "pays_nom": "Sénégal" },
        "pays_origine": [
          { "id": "...", "nom": "Gambie",     "code_iso2": "GM" },
          { "id": "...", "nom": "Mauritanie", "code_iso2": "MR" },
          { "id": "...", "nom": "Sénégal",    "code_iso2": "SN" }
        ],
        "actif": true,
        "nombre_salles_privees": 2,
        "sessions_en_cours": 0
      }
    ],
    "total": 1,
    "page": 1,
    "par_page": 12,
    "total_pages": 1
  }
}
```

## Non-changements

- Aucun nouvel endpoint public n'est ajouté pour cette feature (la liste des pays disponibles côté filtre frontend est dérivée localement à partir des salles déjà retournées, ou réutilise l'endpoint existant `GET /api/pays` côté frontend si besoin de la liste exhaustive).
- Aucune signature de session, salle privée, modérateur ou ressource n'est touchée.
