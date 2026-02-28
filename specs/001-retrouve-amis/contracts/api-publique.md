# Contrats API — Routes Publiques Retrouve Amis

**Base path**: `/api/retrouve-amis`
**Authentification**: JWT Bearer token requis sur tous les endpoints

---

## Avis de Recherche

### `POST /api/retrouve-amis/avis`

Créer un nouvel avis de recherche. Déclenche le matching synchrone.

**Request Body** (JSON):
```json
{
  "nom_recherche": "Kouamé",
  "prenom_recherche": "Jean",
  "surnom": "Jeannot",
  "ecole": "Lycée Moderne d'Abidjan",
  "ville": "Abidjan",
  "pays_id": "uuid-pays",
  "periode_debut": 2005,
  "periode_fin": 2008,
  "description": "On était dans la même classe de terminale"
}
```

**Validations** :
- `nom_recherche` : obligatoire, max 100 caractères
- Au moins un critère supplémentaire parmi : `prenom_recherche`, `ecole`, `ville`, `pays_id`, `periode_debut`
- `periode_debut <= periode_fin` si les deux renseignés
- Max 10 avis actifs par utilisateur (sinon erreur 422)

**Response 201** :
```json
{
  "success": true,
  "data": {
    "id": "uuid-avis",
    "etat": "actif",
    "correspondances_trouvees": 2
  }
}
```

**Erreurs** :
- `400` — Champs obligatoires manquants
- `401` — Non authentifié
- `422` — Limite de 10 avis actifs atteinte

---

### `GET /api/retrouve-amis/avis`

Lister les avis de recherche de l'utilisateur connecté.

**Query Parameters** :
| Param | Type | Description |
|-------|------|-------------|
| `etat` | string | Filtrer par état (`actif`, `cloture`, `suspendu`) |
| `page` | int | Page (défaut: 1) |
| `par_page` | int | Éléments par page (défaut: 20, max: 100) |
| `tri` | string | Colonne de tri (défaut: `created_at`) |
| `ordre` | string | `asc` ou `desc` (défaut: `desc`) |

**Response 200** :
```json
{
  "success": true,
  "data": {
    "avis": [
      {
        "id": "uuid",
        "nom_recherche": "Kouamé",
        "prenom_recherche": "Jean",
        "surnom": "Jeannot",
        "ecole": "Lycée Moderne d'Abidjan",
        "ville": "Abidjan",
        "pays": { "id": "uuid", "nom": "Côte d'Ivoire" },
        "periode_debut": 2005,
        "periode_fin": 2008,
        "etat": "actif",
        "nb_correspondances": 2,
        "created_at": "2026-02-27T10:00:00Z",
        "updated_at": "2026-02-27T10:00:00Z"
      }
    ],
    "total": 5,
    "page": 1,
    "par_page": 20
  }
}
```

---

### `GET /api/retrouve-amis/avis/{id}`

Détail d'un avis de recherche (uniquement si auteur).

**Response 200** :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "nom_recherche": "Kouamé",
    "prenom_recherche": "Jean",
    "surnom": "Jeannot",
    "ecole": "Lycée Moderne d'Abidjan",
    "ville": "Abidjan",
    "pays": { "id": "uuid", "nom": "Côte d'Ivoire" },
    "periode_debut": 2005,
    "periode_fin": 2008,
    "description": "On était dans la même classe de terminale",
    "etat": "actif",
    "correspondances": [
      {
        "id": "uuid-correspondance",
        "score": 78.50,
        "etat": "en_attente",
        "resume_anonymise": {
          "initiales": "J.K.",
          "ville": "Abidjan",
          "periode": "2005-2009",
          "criteres_communs": ["ville", "ecole", "periode"]
        },
        "created_at": "2026-02-27T10:05:00Z"
      }
    ],
    "created_at": "2026-02-27T10:00:00Z",
    "updated_at": "2026-02-27T10:00:00Z"
  }
}
```

**Erreurs** :
- `403` — Pas l'auteur de cet avis
- `404` — Avis introuvable

---

### `PUT /api/retrouve-amis/avis/{id}`

Modifier un avis de recherche actif. Relance le matching.

**Request Body** (JSON) : mêmes champs que POST (partiels autorisés).

**Response 200** :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "correspondances_trouvees": 3
  }
}
```

**Erreurs** :
- `403` — Pas l'auteur
- `404` — Avis introuvable
- `422` — Avis clôturé ou suspendu (non modifiable)

---

### `PATCH /api/retrouve-amis/avis/{id}/cloturer`

Clôturer un avis de recherche (ami retrouvé ou abandon).

**Response 200** :
```json
{
  "success": true,
  "data": { "id": "uuid", "etat": "cloture" }
}
```

---

## Correspondances

### `GET /api/retrouve-amis/correspondances`

Lister les correspondances de l'utilisateur (en tant qu'auteur d'avis OU cible trouvable).

**Query Parameters** :
| Param | Type | Description |
|-------|------|-------------|
| `etat` | string | Filtrer par état |
| `avis_id` | UUID | Filtrer par avis source |
| `page` | int | Page (défaut: 1) |
| `par_page` | int | Éléments par page (défaut: 20) |

**Response 200** :
```json
{
  "success": true,
  "data": {
    "correspondances": [
      {
        "id": "uuid",
        "avis_id": "uuid",
        "score": 78.50,
        "etat": "en_attente",
        "type_cible": "avis",
        "resume_anonymise": {
          "initiales": "J.K.",
          "ville": "Abidjan",
          "periode": "2005-2009",
          "criteres_communs": ["ville", "ecole", "periode"]
        },
        "mon_role": "auteur",
        "created_at": "2026-02-27T10:05:00Z",
        "expire_at": "2026-03-29T10:05:00Z"
      }
    ],
    "total": 2,
    "page": 1,
    "par_page": 20
  }
}
```

**Note** : `mon_role` indique si l'utilisateur est "auteur" (de l'avis source) ou "cible" (trouvé par un avis).

---

### `GET /api/retrouve-amis/correspondances/{id}`

Détail d'une correspondance.

**Response 200** :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "avis_id": "uuid",
    "score": 78.50,
    "details_score": {
      "nom": 35.2,
      "ville": 15.0,
      "pays": 10.0,
      "ecole": 12.5,
      "periode": 5.8
    },
    "etat": "en_attente",
    "type_cible": "avis",
    "mon_role": "auteur",
    "resume_anonymise": {
      "initiales": "J.K.",
      "ville": "Abidjan",
      "periode": "2005-2009",
      "criteres_communs": ["ville", "ecole", "periode"]
    },
    "coordonnees_partagees": null,
    "created_at": "2026-02-27T10:05:00Z",
    "expire_at": "2026-03-29T10:05:00Z"
  }
}
```

**Après consentement mutuel** (`etat = "mutuelle"`) :
```json
{
  "coordonnees_partagees": {
    "email": "jean.kouame@email.com",
    "telephone": null,
    "messagerie": true
  }
}
```

---

### `POST /api/retrouve-amis/correspondances/{id}/accepter`

Accepter le contact pour une correspondance.

**Request Body** :
```json
{
  "coordonnees": {
    "email": true,
    "telephone": false,
    "messagerie": true
  }
}
```

**Response 200** :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "etat": "acceptee_a",
    "consentement_mutuel": false
  }
}
```

Si les deux parties ont accepté :
```json
{
  "etat": "mutuelle",
  "consentement_mutuel": true,
  "coordonnees_partagees": { "email": "...", "messagerie": true }
}
```

---

### `POST /api/retrouve-amis/correspondances/{id}/refuser`

Refuser le contact. Crée une blacklist automatique.

**Response 200** :
```json
{
  "success": true,
  "data": { "id": "uuid", "etat": "declinee" }
}
```

---

### `POST /api/retrouve-amis/avis/{id}/signaler`

Signaler un avis de recherche comme abusif.

**Request Body** :
```json
{
  "motif": "usurpation_identite",
  "description": "Cette personne cherche mon ex sans mon consentement"
}
```

**Response 201** :
```json
{
  "success": true,
  "data": { "id": "uuid-signalement" }
}
```

**Note** : L'utilisateur ne signale pas directement l'avis d'un autre (les avis sont privés). Le signalement porte sur une correspondance reçue — le `avis_id` correspond à l'avis de l'autre partie dans la correspondance.

---

## Notifications

### `GET /api/retrouve-amis/notifications`

Lister les notifications non lues de l'utilisateur.

**Query Parameters** :
| Param | Type | Description |
|-------|------|-------------|
| `lu` | bool | Filtrer par statut lu/non lu (défaut: false) |
| `page` | int | Page |
| `par_page` | int | Éléments par page |

**Response 200** :
```json
{
  "success": true,
  "data": {
    "notifications": [
      {
        "id": "uuid",
        "type": "nouvelle_correspondance",
        "correspondance_id": "uuid",
        "lu": false,
        "created_at": "2026-02-27T10:05:00Z"
      }
    ],
    "total": 3,
    "non_lues": 2,
    "page": 1,
    "par_page": 20
  }
}
```

---

### `PATCH /api/retrouve-amis/notifications/{id}/lire`

Marquer une notification comme lue.

**Response 200** :
```json
{ "success": true, "data": null }
```

---

### `PATCH /api/retrouve-amis/notifications/tout-lire`

Marquer toutes les notifications comme lues.

**Response 200** :
```json
{ "success": true, "data": { "mises_a_jour": 5 } }
```

---

## Tableau de bord

### `GET /api/retrouve-amis/tableau-de-bord`

Résumé global pour l'utilisateur connecté.

**Response 200** :
```json
{
  "success": true,
  "data": {
    "avis_actifs": 3,
    "avis_clotures": 1,
    "correspondances_en_attente": 2,
    "correspondances_mutuelles": 1,
    "notifications_non_lues": 4,
    "est_trouvable": true,
    "nb_parcours": 3
  }
}
```

---

## Profil Trouvable

**Base path**: `/api/profil` (extension du profil existant)

### `PATCH /api/profil/trouvable`

Activer ou désactiver le mode "trouvable". Si activation, déclenche le matching.

**Request Body** :
```json
{ "est_trouvable": true }
```

**Response 200** :
```json
{
  "success": true,
  "data": {
    "est_trouvable": true,
    "correspondances_trouvees": 1
  }
}
```

---

### `GET /api/profil/parcours`

Lister le parcours de l'utilisateur (écoles, villes passées).

**Response 200** :
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "type_entree": "ecole",
      "nom": "Université Félix Houphouët-Boigny",
      "ville": "Abidjan",
      "pays": { "id": "uuid", "nom": "Côte d'Ivoire" },
      "periode_debut": 2008,
      "periode_fin": 2012
    },
    {
      "id": "uuid",
      "type_entree": "ville_residence",
      "nom": "Dakar",
      "ville": null,
      "pays": { "id": "uuid", "nom": "Sénégal" },
      "periode_debut": 2012,
      "periode_fin": 2015
    }
  ]
}
```

---

### `POST /api/profil/parcours`

Ajouter une entrée de parcours.

**Request Body** :
```json
{
  "type_entree": "ecole",
  "nom": "Université Cheikh Anta Diop",
  "ville": "Dakar",
  "pays_id": "uuid-senegal",
  "periode_debut": 2012,
  "periode_fin": 2016
}
```

**Response 201** :
```json
{
  "success": true,
  "data": { "id": "uuid" }
}
```

---

### `PUT /api/profil/parcours/{id}`

Modifier une entrée de parcours.

---

### `DELETE /api/profil/parcours/{id}`

Supprimer une entrée de parcours.

**Response 200** :
```json
{ "success": true, "data": null }
```

---

## Wrapper `ApiResponse<T>`

Toutes les réponses suivent le pattern existant du backend :
```json
{
  "success": true|false,
  "data": T | null,
  "error": "message" | null
}
```
