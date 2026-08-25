# Data Model: Notifications et Suggestions Intelligentes

**Feature Branch**: `001-notifications-suggestions`
**Date**: 2026-03-16

## Nouvelles tables

### `arbre_genealogique.notifications`

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK | Identifiant |
| `destinataire_id` | UUID | FK → iam.utilisateur, NOT NULL | Utilisateur qui reçoit la notification |
| `type` | VARCHAR(30) | NOT NULL | `matching`, `collaboration`, `invitation`, `contact`, `systeme` |
| `message` | TEXT | NOT NULL | Message lisible |
| `lien_action` | VARCHAR(500) | | URL relative vers la page pertinente |
| `lu` | BOOLEAN | NOT NULL, DEFAULT FALSE | Statut de lecture |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |

### `arbre_genealogique.doublons_ignores`

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK | Identifiant |
| `arbre_id` | UUID | FK → arbres, NOT NULL | Arbre concerné |
| `personne_a_id` | UUID | FK → personnes, NOT NULL | Première personne |
| `personne_b_id` | UUID | FK → personnes, NOT NULL | Deuxième personne |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |

UNIQUE INDEX sur `(LEAST(personne_a_id, personne_b_id), GREATEST(personne_a_id, personne_b_id))` par arbre.

## Endpoints API

| Endpoint | Méthode | Description |
|----------|---------|-------------|
| `GET /api/notifications/compteur` | GET | Nombre de notifications non lues |
| `GET /api/notifications` | GET | Liste paginée des notifications |
| `POST /api/notifications/{id}/lire` | POST | Marquer une notification comme lue |
| `POST /api/notifications/tout-lire` | POST | Marquer toutes comme lues |
| `GET /api/arbre/doublons` | GET | Détecter les doublons dans son arbre |
| `POST /api/arbre/doublons/ignorer` | POST | Marquer une paire comme non-doublon |
| `POST /api/arbre/doublons/fusionner` | POST | Fusionner deux personnes |

## Structures côté client

### Suggestions proactives (calculées côté client)

```
SuggestionProactive
├── type: 'parents_manquants' | 'date_manquante' | 'branche_courte'
├── personneId: string (rattachement_id)
├── personneNom: string
├── message: string
├── action: string (URL ou action interne)
└── priorite: number (nb liens de la personne, plus connectée = plus prioritaire)
```

### Formulaire de fusion

```
FusionDoublonDto
├── personne_a_garder_id: UUID (personne conservée)
├── personne_a_supprimer_id: UUID (personne soft-deleted)
├── nom: string (choix A ou B)
├── prenoms: string | null
├── genre: string | null
├── naissance: DatePartielle | null
├── naissance_lieu: string | null
├── deces: DatePartielle | null
├── deces_lieu: string | null
```
