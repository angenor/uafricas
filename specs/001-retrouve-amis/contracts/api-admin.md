# Contrats API : Routes Admin Retrouve Amis

**Base path**: `/api/admin/retrouve-amis`
**Authentification**: JWT Bearer token + rôle `admin` ou `super_admin` requis

---

## Avis de Recherche (Admin)

### `GET /api/admin/retrouve-amis/avis`

Lister tous les avis de recherche avec filtres et pagination.

**Query Parameters** :
| Param | Type | Description |
|-------|------|-------------|
| `recherche` | string | Recherche textuelle (nom, prénom, ville) |
| `etat` | string | Filtrer par état (`actif`, `cloture`, `suspendu`) |
| `auteur_id` | UUID | Filtrer par auteur |
| `pays_id` | UUID | Filtrer par pays |
| `date_debut` | date | Créé après cette date |
| `date_fin` | date | Créé avant cette date |
| `page` | int | Page (défaut: 1) |
| `par_page` | int | Éléments par page (défaut: 20) |
| `tri` | string | Colonne de tri |
| `ordre` | string | `asc` ou `desc` |

**Response 200** :
```json
{
  "success": true,
  "data": {
    "avis": [
      {
        "id": "uuid",
        "auteur": {
          "id": "uuid",
          "nom": "Diallo",
          "prenom": "Fatou",
          "email": "fatou@email.com"
        },
        "nom_recherche": "Kouamé",
        "prenom_recherche": "Jean",
        "ecole": "Lycée Moderne d'Abidjan",
        "ville": "Abidjan",
        "pays": { "id": "uuid", "nom": "Côte d'Ivoire" },
        "etat": "actif",
        "nb_correspondances": 2,
        "nb_signalements": 0,
        "created_at": "2026-02-27T10:00:00Z"
      }
    ],
    "total": 42,
    "page": 1,
    "par_page": 20
  }
}
```

---

### `GET /api/admin/retrouve-amis/avis/{id}`

Détail complet d'un avis (incluant toutes les correspondances et signalements).

**Response 200** :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "auteur": {
      "id": "uuid",
      "nom": "Diallo",
      "prenom": "Fatou",
      "email": "fatou@email.com",
      "etat": "actif"
    },
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
        "id": "uuid",
        "score": 78.50,
        "etat": "mutuelle",
        "type_cible": "profil",
        "cible_utilisateur": {
          "id": "uuid",
          "nom": "Kouamé",
          "prenom": "Jean"
        },
        "created_at": "2026-02-27T10:05:00Z"
      }
    ],
    "signalements": [
      {
        "id": "uuid",
        "signale_par": { "id": "uuid", "nom": "Traoré", "prenom": "Awa" },
        "motif": "usurpation_identite",
        "description": "...",
        "etat": "en_attente",
        "created_at": "2026-02-28T14:00:00Z"
      }
    ],
    "created_at": "2026-02-27T10:00:00Z",
    "updated_at": "2026-02-27T10:00:00Z"
  }
}
```

---

### `PATCH /api/admin/retrouve-amis/avis/{id}/etat`

Changer l'état d'un avis (suspension, réactivation).

**Request Body** :
```json
{ "etat": "suspendu" }
```

**Transitions autorisées** :
- `actif → suspendu` (suspension par admin)
- `suspendu → actif` (réactivation après modération)

**Response 200** :
```json
{
  "success": true,
  "data": { "id": "uuid", "etat": "suspendu" }
}
```

**Audit** : `audit::log_action("UPDATE", "retrouve_amis", "avis_recherche", ...)`

---

## Signalements (Admin)

### `GET /api/admin/retrouve-amis/signalements`

Lister les signalements avec filtres.

**Query Parameters** :
| Param | Type | Description |
|-------|------|-------------|
| `etat` | string | `en_attente`, `approuve`, `rejete` |
| `motif` | string | Filtrer par motif |
| `page` | int | Page |
| `par_page` | int | Éléments par page |
| `tri` | string | Colonne de tri |
| `ordre` | string | `asc` ou `desc` |

**Response 200** :
```json
{
  "success": true,
  "data": {
    "signalements": [
      {
        "id": "uuid",
        "avis": {
          "id": "uuid",
          "nom_recherche": "Kouamé",
          "auteur": { "id": "uuid", "nom": "Diallo", "prenom": "Fatou" }
        },
        "signale_par": { "id": "uuid", "nom": "Traoré", "prenom": "Awa" },
        "motif": "usurpation_identite",
        "description": "...",
        "etat": "en_attente",
        "created_at": "2026-02-28T14:00:00Z"
      }
    ],
    "total": 5,
    "page": 1,
    "par_page": 20
  }
}
```

---

### `GET /api/admin/retrouve-amis/signalements/{id}`

Détail d'un signalement.

---

### `PATCH /api/admin/retrouve-amis/signalements/{id}/moderer`

Modérer un signalement (approuver ou rejeter).

**Request Body** :
```json
{
  "decision": "approuve"
}
```

**Comportement** :
- `approuve` → le signalement est marqué approuvé ET l'avis associé est automatiquement suspendu
- `rejete` → le signalement est marqué rejeté, l'avis reste inchangé

**Response 200** :
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "etat": "approuve",
    "avis_suspendu": true
  }
}
```

**Audit** : `audit::log_action("UPDATE", "retrouve_amis", "signalement", ...)` + éventuellement `audit::log_action("UPDATE", "retrouve_amis", "avis_recherche", ...)`

---

## Statistiques (Admin)

### `GET /api/admin/retrouve-amis/statistiques`

Statistiques globales de la fonctionnalité.

**Response 200** :
```json
{
  "success": true,
  "data": {
    "total_avis": 150,
    "avis_actifs": 85,
    "avis_clotures": 55,
    "avis_suspendus": 10,
    "total_correspondances": 230,
    "correspondances_mutuelles": 45,
    "correspondances_en_attente": 80,
    "correspondances_declinees": 60,
    "correspondances_archivees": 45,
    "utilisateurs_trouvables": 320,
    "signalements_en_attente": 3,
    "signalements_total": 15,
    "blacklists_total": 25
  }
}
```

---

## Pattern Admin

Tous les endpoints admin suivent les conventions existantes :
- `useAdmin()` comme composable de base (adminFetch, listerPagine, pagination, sort)
- `AdminDataTable` pour les listes paginées avec tri
- `AdminFilters` pour les filtres dynamiques
- `AdminDeleteConfirm` pour les actions destructives
- `AdminPageHeader` pour l'en-tête de page avec actions
- Audit systématique sur les mutations via `audit::log_action`

### Colonnes autorisées pour le tri (COLONNES const)

**Avis** : `created_at`, `updated_at`, `nom_recherche`, `etat`
**Signalements** : `created_at`, `motif`, `etat`
