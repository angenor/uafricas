# Data Model: Collaboration et Partage de l'Arbre

**Feature Branch**: `001-collaboration-partage`
**Date**: 2026-03-16

## Modifications du schema existant

### Table `arbre_genealogique.arbres` — colonne ajoutée

| Colonne | Type | Description |
|---------|------|-------------|
| `arbre_prive` | BOOLEAN DEFAULT FALSE | Si true, toutes les personnes de l'arbre sont exclues du matching public |

### Table `arbre_genealogique.personnes` — colonne ajoutée

| Colonne | Type | Description |
|---------|------|-------------|
| `visible_matching` | BOOLEAN DEFAULT TRUE | Si false, cette personne est exclue du matching public |

## Nouvelles tables

### `arbre_genealogique.invitations`

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK | Identifiant |
| `arbre_id` | UUID | FK → arbres, NOT NULL | Arbre cible |
| `email_invite` | VARCHAR(255) | NOT NULL | Email du destinataire |
| `utilisateur_invite_id` | UUID | FK → iam.utilisateur, NULLABLE | Rempli si l'email correspond à un compte existant |
| `permission` | VARCHAR(20) | NOT NULL | `lecture_seule` ou `edition` |
| `statut` | VARCHAR(20) | NOT NULL, DEFAULT 'en_attente' | `en_attente`, `acceptee`, `refusee`, `expiree` |
| `invite_par` | UUID | FK → iam.utilisateur, NOT NULL | Propriétaire qui invite |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| `expire_at` | TIMESTAMPTZ | | Date d'expiration (30 jours) |
| `traitee_le` | TIMESTAMPTZ | | Date d'acceptation/refus |

### `arbre_genealogique.collaborateurs`

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK | Identifiant |
| `arbre_id` | UUID | FK → arbres, NOT NULL | Arbre partagé |
| `utilisateur_id` | UUID | FK → iam.utilisateur, NOT NULL | Collaborateur |
| `permission` | VARCHAR(20) | NOT NULL | `lecture_seule` ou `edition` |
| `invitation_id` | UUID | FK → invitations | Invitation source |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |

**Contraintes** :
- UNIQUE(arbre_id, utilisateur_id) — un seul accès par utilisateur par arbre
- CHECK: le collaborateur n'est pas le propriétaire de l'arbre (vérifié côté applicatif)
- Limite 20 collaborateurs par arbre (vérifiée par COUNT avant INSERT)

## Endpoints API

| Endpoint | Méthode | Description |
|----------|---------|-------------|
| `GET /api/arbre/mes-arbres` | GET | Liste mon arbre + arbres partagés avec moi |
| `GET /api/arbre/arbre-complet?arbre_id=...` | GET | Charge un arbre (propre ou partagé, vérification accès) |
| `POST /api/arbre/invitations` | POST | Envoyer une invitation |
| `GET /api/arbre/invitations` | GET | Lister mes invitations reçues (en attente) |
| `POST /api/arbre/invitations/{id}/accepter` | POST | Accepter une invitation |
| `POST /api/arbre/invitations/{id}/refuser` | POST | Refuser une invitation |
| `GET /api/arbre/{arbre_id}/collaborateurs` | GET | Lister les collaborateurs d'un arbre (propriétaire) |
| `PUT /api/arbre/collaborateurs/{id}` | PUT | Modifier la permission d'un collaborateur |
| `DELETE /api/arbre/collaborateurs/{id}` | DELETE | Révoquer l'accès d'un collaborateur |
| `PUT /api/arbre/{arbre_id}/confidentialite` | PUT | Modifier arbre_prive |
| `PUT /api/arbre/personnes/{id}/confidentialite` | PUT | Modifier visible_matching d'une personne |
| `GET /api/arbre/{arbre_id}/historique` | GET | Historique des modifications (filtre audit_log) |

## Diagramme de relations

```
iam.utilisateur
     │
     ├── propriétaire de → arbre_genealogique.arbres (utilisateur_id UNIQUE)
     │                           │
     │                           ├── arbre_prive: boolean
     │                           ├── invitations (arbre_id FK)
     │                           └── collaborateurs (arbre_id FK)
     │
     └── collaborateur via → arbre_genealogique.collaborateurs
                              ├── utilisateur_id FK
                              ├── permission: lecture_seule | edition
                              └── invitation_id FK
```
