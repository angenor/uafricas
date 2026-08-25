# Contrat API : Arbre Complet

**Endpoint** : `GET /api/arbre/arbre-complet`
**Authentification** : JWT Bearer (obligatoire)
**Description** : Retourne l'intégralité de l'arbre généalogique de l'utilisateur connecté (toutes les personnes et tous les liens familiaux) en un seul appel.

## Requête

```
GET /api/arbre/arbre-complet
Authorization: Bearer <access_token>
```

**Paramètres** : Aucun.

## Réponse : Succès (200)

```json
{
  "success": true,
  "data": {
    "arbre_id": "uuid-arbre",
    "personnes": [
      {
        "id": "uuid-personne",
        "rattachement_id": "uuid-rattachement",
        "nom": "Diallo",
        "prenoms": "Ibrahim",
        "genre": "masculin",
        "naissance": { "annee": 1850 },
        "deces": null,
        "naissance_lieu": "Conakry",
        "deces_lieu": null,
        "photo_url": "/uploads/personnes/uuid_photo.jpg"
      }
    ],
    "liens": [
      {
        "id": "uuid-lien",
        "rattachement_source_id": "uuid-rattachement-parent",
        "rattachement_cible_id": "uuid-rattachement-enfant",
        "type_lien": "pere"
      }
    ]
  }
}
```

## Réponse : Arbre vide (200)

Si l'utilisateur n'a pas encore créé d'arbre ou n'a aucune personne :

```json
{
  "success": true,
  "data": {
    "arbre_id": null,
    "personnes": [],
    "liens": []
  }
}
```

## Réponse : Non authentifié (401)

```json
{
  "success": false,
  "error": "Token d'accès manquant ou invalide"
}
```

## Types

### PersonneNoeud

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | oui | Identifiant de la personne |
| rattachement_id | UUID | oui | Identifiant du rattachement (utilisé comme clé dans les liens) |
| nom | string | oui | Nom de famille |
| prenoms | string | non | Prénoms |
| genre | enum | oui | `masculin`, `feminin`, `autre`, `non_precise` |
| naissance | DatePartielle | non | Date de naissance (granularité variable) |
| deces | DatePartielle | non | Date de décès (granularité variable) |
| naissance_lieu | string | non | Lieu de naissance |
| deces_lieu | string | non | Lieu de décès |
| photo_url | string | non | URL relative de la photo |

### DatePartielle

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| annee | integer | non | Année (ex: 1850) |
| mois | integer | non | Mois (1-12) |
| jour | integer | non | Jour (1-31) |

### LienArbreResponse

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| id | UUID | oui | Identifiant du lien |
| rattachement_source_id | UUID | oui | Rattachement source (parent ou conjoint A) |
| rattachement_cible_id | UUID | oui | Rattachement cible (enfant ou conjoint B) |
| type_lien | enum | oui | `pere`, `mere`, `parent`, `conjoint` |

## Notes d'implémentation

- L'endpoint suit le pattern `ApiResponse<T>` existant.
- L'arbre est automatiquement créé si l'utilisateur n'en a pas (via `obtenir_ou_creer_arbre`).
- Seules les entrées avec `deleted_at IS NULL` sont retournées.
- Pas de pagination : la taille attendue est < 200 personnes par arbre.
- Les liens conjoint suivent la convention existante : `rattachement_source_id < rattachement_cible_id` (tri UUID).
