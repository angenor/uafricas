# Data Model: Partage Public des Avis de Recherche

**Branch**: `002-partage-avis-recherche` | **Date**: 2026-03-02

## Vue d'ensemble

Cette fonctionnalité étend le schema `retrouve_amis` existant (PostgreSQL 16) avec :
- **3 colonnes ajoutées** à la table `avis_recherche` (visibilité publique)
- **2 nouvelles tables** : `reponse_publique` et `demande_retrait`
- **1 colonne ajoutée** à la table `signalement` (source du signalement)
- **1 nouvel enum** : `source_signalement`
- **1 enum étendu** : `type_notification` (2 nouvelles valeurs)
- **Mise à jour du TSVECTOR** trigger pour inclure le slug dans le search_vector

## Modifications de la table existante: `avis_recherche`

### Colonnes ajoutées

| Colonne | Type | Défaut | Nullable | Description |
|---------|------|--------|----------|-------------|
| `est_public` | BOOLEAN | FALSE | NOT NULL | Active/désactive la visibilité publique |
| `slug` | VARCHAR(400) | NULL | YES | URL slug unique (généré à l'activation publique) |
| `date_publication_publique` | TIMESTAMPTZ | NULL | YES | Date de première publication publique |
| `compteur_partages` | INTEGER | 0 | NOT NULL | Nombre total de partages (incrémenté atomiquement) |

### Contraintes ajoutées

- `UNIQUE(slug)` : un seul avis par slug
- `CHECK(compteur_partages >= 0)` : pas de compteur négatif
- Index partiel : `CREATE INDEX idx_avis_public_actif ON avis_recherche(est_public, etat) WHERE est_public = TRUE AND etat = 'actif' AND deleted_at IS NULL`, optimise les requêtes de listing public

### Règles de validation

- `slug` est généré uniquement quand `est_public` passe de FALSE à TRUE
- Format du slug : `{nom_recherche_slugifie}-{uuid8}` (ex: `keita-fatou-a3f8b2c1`)
- `date_publication_publique` est set une seule fois (première publication), pas réinitialisé si dépublié puis republié
- `compteur_partages` ne peut qu'augmenter (pas de réinitialisation)

## Nouvelle table: `reponse_publique`

Représente la réponse d'un visiteur connecté à un avis public.

| Colonne | Type | Défaut | Nullable | Contrainte | Description |
|---------|------|--------|----------|------------|-------------|
| `id` | UUID | gen_random_uuid() | NOT NULL | PK | Identifiant unique |
| `avis_id` | UUID | : | NOT NULL | FK → avis_recherche(id) CASCADE | Avis concerné |
| `repondeur_id` | UUID | : | NOT NULL | FK → iam.utilisateur(id) | Utilisateur qui répond |
| `type_reponse` | type_reponse_publique |, | NOT NULL |, | Type de la réponse |
| `message` | TEXT | : | NOT NULL | : | Message du répondeur |
| `correspondance_id` | UUID | NULL | YES | FK → correspondance(id) SET NULL | Correspondance créée automatiquement |
| `created_at` | TIMESTAMPTZ | NOW() | NOT NULL |, | Date de création |

### Enum: `type_reponse_publique`

```sql
CREATE TYPE retrouve_amis.type_reponse_publique AS ENUM (
    'je_suis_cette_personne',
    'je_la_connais',
    'jai_des_informations'
);
```

### Contraintes

- `UNIQUE(avis_id, repondeur_id)`, un seul message par utilisateur par avis (FR-007)
- L'auteur de l'avis ne peut pas répondre à son propre avis
- Le répondeur ne doit pas être dans la blacklist de l'auteur

### Index

- `CREATE INDEX idx_reponse_avis ON reponse_publique(avis_id)`, recherche par avis
- `CREATE INDEX idx_reponse_repondeur ON reponse_publique(repondeur_id)`, recherche par utilisateur

## Nouvelle table: `demande_retrait`

Représente une demande de retrait d'un avis par une personne qui s'y reconnaît.

| Colonne | Type | Défaut | Nullable | Contrainte | Description |
|---------|------|--------|----------|------------|-------------|
| `id` | UUID | gen_random_uuid() | NOT NULL | PK | Identifiant unique |
| `avis_id` | UUID | : | NOT NULL | FK → avis_recherche(id) CASCADE | Avis concerné |
| `demandeur_id` | UUID | : | NOT NULL | FK → iam.utilisateur(id) | Personne demandant le retrait |
| `motif` | TEXT | : | NOT NULL | : | Raison de la demande |
| `etat` | etat_demande_retrait | 'en_attente' | NOT NULL |, | État de traitement |
| `date_suspension` | TIMESTAMPTZ | NOW() | NOT NULL |, | Date de suspension automatique de l'avis |
| `decide_par` | UUID | NULL | YES | FK → iam.utilisateur(id) | Admin ayant tranché |
| `decision_at` | TIMESTAMPTZ | NULL | YES |, | Date de la décision admin |
| `commentaire_admin` | TEXT | NULL | YES |, | Justification de la décision |
| `created_at` | TIMESTAMPTZ | NOW() | NOT NULL |, | Date de création |

### Enum: `etat_demande_retrait`

```sql
CREATE TYPE retrouve_amis.etat_demande_retrait AS ENUM (
    'en_attente',
    'approuvee',
    'rejetee'
);
```

### Contraintes

- `UNIQUE(avis_id, demandeur_id)`, une seule demande par utilisateur par avis
- Le demandeur ne peut pas être l'auteur de l'avis (l'auteur peut simplement dépublier)

### Transitions d'état

```
en_attente ← (création : avis immédiatement suspendu)
en_attente → approuvee (admin approuve : avis reste suspendu, est_public = FALSE)
en_attente → rejetee (admin rejette : avis réactivé, est_public = TRUE)
```

### Index

- `CREATE INDEX idx_demande_avis ON demande_retrait(avis_id)`, recherche par avis
- `CREATE INDEX idx_demande_etat ON demande_retrait(etat) WHERE etat = 'en_attente'`, filtrer demandes en attente

## Modification de la table existante: `signalement`

### Colonne ajoutée

| Colonne | Type | Défaut | Nullable | Description |
|---------|------|--------|----------|-------------|
| `source` | source_signalement | 'correspondance' | NOT NULL | Origine du signalement |

### Enum: `source_signalement`

```sql
CREATE TYPE retrouve_amis.source_signalement AS ENUM (
    'correspondance',
    'page_publique'
);
```

### Impact

- Les signalements existants reçoivent la valeur par défaut `'correspondance'`
- Les signalements depuis la page publique sont créés avec `source = 'page_publique'`
- Le handler public ne vérifie pas l'existence d'une correspondance (contrairement au handler existant)
- Le workflow de modération admin reste identique

## Extension de l'enum: `type_notification`

Ajout de 2 nouvelles valeurs :

```sql
ALTER TYPE retrouve_amis.type_notification ADD VALUE 'reponse_publique';
ALTER TYPE retrouve_amis.type_notification ADD VALUE 'demande_retrait';
```

Valeurs complètes après modification :
- `nouvelle_correspondance` (existant)
- `acceptation_contact` (existant)
- `coordonnees_partagees` (existant)
- `correspondance_archivee` (existant)
- `avis_suspendu` (existant)
- `reponse_publique` (nouveau : notifie l'auteur d'une réponse)
- `demande_retrait` (nouveau : notifie l'auteur + admins d'une demande)

## Diagramme des relations

```
                    ┌──────────────────┐
                    │  iam.utilisateur │
                    └────────┬─────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
    ┌─────────────┐  ┌──────────────┐  ┌────────────────┐
    │  demande_   │  │ avis_        │  │ reponse_       │
    │  retrait    │──│ recherche    │──│ publique       │
    │             │  │ (+ est_public│  │                │
    │             │  │  + slug      │  │                │
    │             │  │  + compteur) │  │                │
    └─────────────┘  └──────┬───────┘  └───────┬────────┘
                            │                  │
                     ┌──────┼──────┐           │
                     │      │      │           │
                     ▼      ▼      ▼           ▼
              ┌──────────┐ ┌──────────┐ ┌──────────────┐
              │signalement│ │correspon-│ │correspondance│
              │(+source)  │ │ dance    │ │(créée auto)  │
              └──────────┘ └──────────┘ └──────────────┘
```

## Rate limiting (application-level)

- **Réponses** : 1 par avis par utilisateur (UNIQUE constraint) + 10 par jour par utilisateur (vérification handler)
- **Partages** : Pas de rate limit (compteur simple, pas de risque d'abus)
- **Signalements** : 1 par avis par utilisateur (UNIQUE constraint existant)
- **Demandes de retrait** : 1 par avis par utilisateur (UNIQUE constraint)
