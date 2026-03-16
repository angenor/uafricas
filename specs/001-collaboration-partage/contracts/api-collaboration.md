# Contrats API : Collaboration et Partage

## 1. GET /api/arbre/mes-arbres

Liste l'arbre de l'utilisateur + les arbres partagés avec lui.

**Auth** : JWT Bearer

**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "mon_arbre": { "id": "uuid", "nb_personnes": 15, "created_at": "..." },
    "arbres_partages": [
      {
        "arbre_id": "uuid",
        "proprietaire_nom": "Keita Moussa",
        "permission": "lecture_seule",
        "nb_personnes": 42,
        "partage_depuis": "2026-03-10"
      }
    ]
  }
}
```

## 2. POST /api/arbre/invitations

Envoie une invitation.

**Auth** : JWT Bearer (propriétaire uniquement)
**Body** : `{ "email": "cousin@example.com", "permission": "edition" }`

**Réponse 201** : `{ "success": true, "data": { "id": "uuid", "statut": "en_attente" } }`
**Réponse 400** : Email invalide
**Réponse 403** : Non propriétaire
**Réponse 409** : Déjà invité ou collaborateur
**Réponse 422** : Limite 20 collaborateurs atteinte

## 3. POST /api/arbre/invitations/{id}/accepter

**Auth** : JWT Bearer (destinataire)
**Réponse 200** : `{ "success": true, "data": { "arbre_id": "uuid", "permission": "edition" } }`

## 4. POST /api/arbre/invitations/{id}/refuser

**Auth** : JWT Bearer (destinataire)
**Réponse 200** : `{ "success": true, "data": { "message": "Invitation refusée" } }`

## 5. GET /api/arbre/{arbre_id}/collaborateurs

**Auth** : JWT Bearer (propriétaire)
**Réponse 200** : Liste des collaborateurs avec nom, email, permission, date

## 6. PUT /api/arbre/collaborateurs/{id}

**Auth** : JWT Bearer (propriétaire)
**Body** : `{ "permission": "edition" }`
**Réponse 200** : Collaborateur mis à jour

## 7. DELETE /api/arbre/collaborateurs/{id}

**Auth** : JWT Bearer (propriétaire)
**Réponse 200** : Accès révoqué

## 8. PUT /api/arbre/{arbre_id}/confidentialite

**Auth** : JWT Bearer (propriétaire)
**Body** : `{ "arbre_prive": true }`
**Réponse 200** : Paramètre mis à jour

## 9. PUT /api/arbre/personnes/{id}/confidentialite

**Auth** : JWT Bearer (propriétaire)
**Body** : `{ "visible_matching": false }`
**Réponse 200** : Personne marquée privée

## 10. GET /api/arbre/{arbre_id}/historique

**Auth** : JWT Bearer (propriétaire ou collaborateur édition)
**Query** : `page`, `par_page`, `auteur_id` (filtre optionnel)
**Réponse 200** : Liste paginée d'entrées d'historique avec action, auteur, date, détails
