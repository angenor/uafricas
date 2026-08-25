# Contrats API: Validation Admin des Bibliothèques Humaines

**Date**: 2026-04-22 | **Feature**: 001-admin-biblio-humaine

## Endpoints modifiés

### `POST /api/bibliotheques-humaines/inscription`, modifié

**Avant** : créait directement `bibliotheque_humain = TRUE`.
**Après** : crée une `demande_biblio_humaine` en statut `en_attente`.

**Auth** : JWT Bearer requis

**Body** (inchangé) :
```json
{
  "specialites": ["Histoire", "Griot"],
  "biographie": "Passionné d'histoire africaine... (min 20 car.)",
  "fonction": "Historien et conteur",
  "pays": "Sénégal"
}
```

**Réponse 201** (nouveau format) :
```json
{
  "success": true,
  "data": {
    "id": "uuid-demande",
    "statut": "en_attente",
    "createdAt": "2026-04-22T10:00:00Z"
  },
  "error": null
}
```

**Erreurs** :
- `409 Conflict` : demande déjà active (en_attente ou valide)
- `401 Unauthorized` : non authentifié
- `422 Unprocessable` : biographie < 20 car., fonction vide, aucune spécialité

---

### `GET /api/bibliotheques-humaines` : signature inchangée, filtrage interne modifié

La liste publique ne retourne que les utilisateurs avec une demande `valide`. La réponse reste identique au format actuel.

---

## Nouveaux endpoints publics (JWT)

### `GET /api/bibliotheques-humaines/moi/demande`

**Auth** : JWT Bearer requis

**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "id": "uuid-demande",
    "statut": "en_attente",
    "fonction": "Historien",
    "biographie": "Texte de biographie...",
    "pays": "Sénégal",
    "specialites": ["Histoire", "Griot"],
    "commentaireAdmin": null,
    "createdAt": "2026-04-22T10:00:00Z",
    "traiteLe": null
  },
  "error": null
}
```

**Réponse 404** : aucune demande soumise par cet utilisateur

---

## Nouveaux endpoints admin

### `GET /api/admin/bibliotheques-humaines`

**Auth** : JWT + rôle `admin`

**Query params** :
- `statut` : `en_attente` | `valide` | `rejete` (optionnel)
- `recherche` : filtre nom / prénom / email
- `page` : défaut 1
- `par_page` : défaut 20, max 100

**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "demandes": [
      {
        "id": "uuid",
        "utilisateurId": "uuid",
        "nom": "Diallo",
        "prenom": "Amadou",
        "email": "amadou@example.com",
        "photoUrl": null,
        "fonction": "Historien",
        "statut": "en_attente",
        "specialites": ["Histoire"],
        "createdAt": "2026-04-22T10:00:00Z"
      }
    ],
    "total": 42,
    "page": 1,
    "par_page": 20,
    "total_pages": 3
  },
  "error": null
}
```

---

### `GET /api/admin/bibliotheques-humaines/{id}`

**Auth** : JWT + rôle `admin`

**Réponse 200** : même structure que l'item de liste + `biographie`, `pays`, `commentaireAdmin`, `traiteLe`, `traiteParNom`

**Réponse 404** : demande inexistante

---

### `PATCH /api/admin/bibliotheques-humaines/{id}/valider`

**Auth** : JWT + rôle `admin`

**Body** : `{}` (aucun champ requis)

**Actions atomiques** :
1. `demande.statut → valide`
2. `utilisateur.bibliotheque_humain = TRUE`
3. `utilisateur.fonction`, `biographie`, `pays_origine_id` mis à jour
4. `utilisateur_specialite` peuplé depuis `demande_biblio_specialite`
5. `audit::log_action` enregistré

**Réponse 200** :
```json
{ "success": true, "data": { "id": "uuid", "statut": "valide" }, "error": null }
```

---

### `PATCH /api/admin/bibliotheques-humaines/{id}/rejeter`

**Auth** : JWT + rôle `admin`

**Body** :
```json
{ "commentaire": "Biographie insuffisante." }
```

**Actions** :
1. `demande.statut → rejete`
2. `demande.commentaire_admin` sauvegardé
3. `utilisateur.bibliotheque_humain` repassé à `FALSE` si nécessaire
4. `audit::log_action` enregistré

**Réponse 200** :
```json
{ "success": true, "data": { "id": "uuid", "statut": "rejete" }, "error": null }
```
