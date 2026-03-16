# Contrats API : Notifications et Doublons

## 1. GET /api/notifications/compteur
**Auth** : JWT
**Réponse** : `{ "success": true, "data": { "non_lues": 5 } }`

## 2. GET /api/notifications
**Auth** : JWT | **Query** : `page`, `par_page`, `type` (filtre optionnel)
**Réponse** : Liste paginée de notifications (id, type, message, lien_action, lu, created_at)

## 3. POST /api/notifications/{id}/lire
**Auth** : JWT
**Réponse** : `{ "success": true, "data": { "message": "Notification marquée comme lue" } }`

## 4. POST /api/notifications/tout-lire
**Auth** : JWT
**Réponse** : `{ "success": true, "data": { "nb_marquees": 12 } }`

## 5. GET /api/arbre/doublons
**Auth** : JWT
**Réponse** : Liste de paires de doublons potentiels (personne_a, personne_b, score, score_details)

## 6. POST /api/arbre/doublons/ignorer
**Auth** : JWT | **Body** : `{ "personne_a_id": "uuid", "personne_b_id": "uuid" }`
**Réponse** : `{ "success": true }`

## 7. POST /api/arbre/doublons/fusionner
**Auth** : JWT | **Body** : `FusionDoublonDto` (personne à garder, personne à supprimer, champs choisis)
**Réponse** : `{ "success": true, "data": { "personne_id": "uuid" } }` (ID de la personne conservée)
