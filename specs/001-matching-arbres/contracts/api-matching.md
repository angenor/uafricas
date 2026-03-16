# Contrats API : Matching et Découvertes

## Endpoints

### 1. GET /api/arbre/decouvertes

Liste les suggestions de correspondance de l'utilisateur connecté (toutes sections).

**Auth** : JWT Bearer
**Query params** : `section` (optionnel : `en_attente` | `en_cours` | `confirmees`), `page`, `par_page`

**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "en_attente": { "suggestions": [...], "total": 5 },
    "en_cours": { "suggestions": [...], "total": 1 },
    "confirmees": { "suggestions": [...], "total": 2 }
  }
}
```

### 2. POST /api/arbre/decouvertes/{id}/confirmer

Confirme une suggestion de correspondance.

**Auth** : JWT Bearer
**Réponse 200** : `{ "success": true, "data": { "statut": "confirmee_de_mon_cote", "message": "En attente de confirmation de l'autre membre" } }`
**Réponse 200 (mutuelle)** : `{ "success": true, "data": { "statut": "confirmee", "message": "Correspondance confirmée ! Vous pouvez maintenant voir les branches de l'autre arbre." } }`
**Réponse 404** : Suggestion introuvable
**Réponse 409** : Déjà traitée

### 3. POST /api/arbre/decouvertes/{id}/rejeter

Rejette une suggestion définitivement.

**Auth** : JWT Bearer
**Réponse 200** : `{ "success": true, "data": { "message": "Suggestion rejetée" } }`

### 4. GET /api/arbre/decouvertes/{id}/branches

Retourne l'arbre complet de l'autre utilisateur (après confirmation mutuelle uniquement).

**Auth** : JWT Bearer
**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "suggestion_id": "uuid",
    "personne_commune": { "nom": "Diallo", "prenoms": "Ibrahim", ... },
    "personnes": [...],
    "liens": [...],
    "membre_id_anonymise": "Membre #a1b2"
  }
}
```
**Réponse 403** : Correspondance non confirmée mutuellement

### 5. POST /api/arbre/decouvertes/{suggestion_id}/demande-contact

Envoie une demande de contact à l'autre utilisateur (après confirmation mutuelle).

**Auth** : JWT Bearer
**Réponse 201** : `{ "success": true, "data": { "id": "uuid", "statut": "en_attente" } }`
**Réponse 403** : Correspondance non confirmée
**Réponse 409** : Demande déjà envoyée

### 6. POST /api/arbre/demandes-contact/{id}/accepter

Accepte une demande de contact reçue.

**Auth** : JWT Bearer
**Réponse 200** : `{ "success": true, "data": { "profil": { "nom": "...", "prenom": "...", "email": "..." } } }`

### 7. POST /api/arbre/demandes-contact/{id}/refuser

Refuse une demande de contact.

**Auth** : JWT Bearer
**Réponse 200** : `{ "success": true, "data": { "message": "Demande refusée" } }`
