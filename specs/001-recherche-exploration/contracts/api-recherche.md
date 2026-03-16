# Contrat API : Recherche Publique

## GET /api/arbre/recherche-publique

Recherche de personnes dans tous les arbres (sauf celui de l'utilisateur connecté).

**Auth** : JWT Bearer
**Query params** : `q` (string, min 2 caractères, max 100)

**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "resultats": [
      {
        "nom": "Kouyaté",
        "prenoms": "Fatoumata",
        "naissance_annee": 1885,
        "naissance_lieu": "Kankan, Guinée",
        "genre": "feminin",
        "membre_id_anonymise": "Membre #a1b2",
        "score_similarite": 0.85,
        "source": "autre_arbre"
      }
    ],
    "total": 3
  }
}
```

**Réponse 400** : Terme trop court (< 2 caractères)
**Réponse 401** : Non authentifié
