# Data Model: Matching et Découverte de Parents

**Feature Branch**: `001-matching-arbres`
**Date**: 2026-03-16

## Modifications du schema existant

### Table `arbre_genealogique.personnes`, colonnes ajoutées

| Colonne | Type | Description |
|---------|------|-------------|
| `nom_normalise` | VARCHAR(255) | Nom normalisé phonétiquement (lowercase, variantes africaines) |
| `prenoms_normalise` | VARCHAR(500) | Prénoms normalisés phonétiquement |

Index GIN trigram sur `nom_normalise` et `prenoms_normalise` (WHERE deleted_at IS NULL).

Extension PostgreSQL requise : `pg_trgm`.

## Nouvelles tables

### `arbre_genealogique.suggestions_correspondance`

Stocke les correspondances potentielles détectées par l'algorithme de matching.

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK, DEFAULT uuid_generate_v4() | Identifiant unique |
| `rattachement_a_id` | UUID | FK → rattachements(id), NOT NULL | Rattachement de la personne dans l'arbre A |
| `rattachement_b_id` | UUID | FK → rattachements(id), NOT NULL | Rattachement de la personne dans l'arbre B |
| `score` | REAL | NOT NULL, CHECK (0..1) | Score de confiance composite (0-100%) |
| `score_nom` | REAL | | Sous-score similarité nom |
| `score_prenoms` | REAL | | Sous-score similarité prénoms |
| `score_date` | REAL | | Sous-score compatibilité dates |
| `score_lieu` | REAL | | Sous-score similarité lieu |
| `score_genre` | REAL | | Sous-score genre |
| `statut` | VARCHAR(20) | NOT NULL, DEFAULT 'en_attente' | en_attente, confirmee_a, confirmee_b, confirmee, rejetee_a, rejetee_b |
| `confirmee_par_a` | BOOLEAN | DEFAULT FALSE | L'utilisateur propriétaire de l'arbre A a confirmé |
| `confirmee_par_b` | BOOLEAN | DEFAULT FALSE | L'utilisateur propriétaire de l'arbre B a confirmé |
| `detectee_le` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Date de détection |
| `confirmee_le` | TIMESTAMPTZ | | Date de confirmation mutuelle |
| `deleted_at` | TIMESTAMPTZ | | Soft delete |

**Contraintes** :
- UNIQUE sur `(LEAST(rattachement_a_id, rattachement_b_id), GREATEST(rattachement_a_id, rattachement_b_id))`, une seule suggestion par paire, quel que soit l'ordre
- CHECK `rattachement_a_id != rattachement_b_id`, non-réflexif
- Les rattachements doivent appartenir à des arbres différents (vérifié côté applicatif)

**Cycle de vie** :
```
en_attente → confirmee_a (A confirme) → confirmee (B confirme aussi)
en_attente → confirmee_b (B confirme) → confirmee (A confirme aussi)
en_attente → rejetee_a (A rejette) : définitif
en_attente → rejetee_b (B rejette) : définitif
confirmee_a → rejetee_b (B rejette après que A a confirmé), annulée
confirmee_b → rejetee_a (A rejette après que B a confirmé), annulée
```

### `arbre_genealogique.demandes_contact`

Stocke les demandes de contact entre utilisateurs après confirmation mutuelle.

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK, DEFAULT uuid_generate_v4() | Identifiant unique |
| `suggestion_id` | UUID | FK → suggestions_correspondance(id), NOT NULL | Correspondance source |
| `demandeur_id` | UUID | FK → iam.utilisateur(id), NOT NULL | Utilisateur qui demande le contact |
| `destinataire_id` | UUID | FK → iam.utilisateur(id), NOT NULL | Utilisateur qui reçoit la demande |
| `statut` | VARCHAR(20) | NOT NULL, DEFAULT 'en_attente' | en_attente, acceptee, refusee |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | |
| `traitee_le` | TIMESTAMPTZ | | |

## Structures de réponse API (DTOs)

### SuggestionCorrespondanceResponse

```
SuggestionCorrespondanceResponse
├── id: UUID
├── ma_personne: PersonneResumeResponse (nom, prenoms, naissance, lieu)
├── personne_matchee: PersonneResumeResponse (nom, prenoms, naissance, lieu, de l'autre arbre)
├── score: f32 (0-100%)
├── details_score: DetailsScoreResponse
│   ├── nom: f32
│   ├── prenoms: f32
│   ├── date: f32
│   ├── lieu: f32
│   └── genre: f32
├── statut: String (en_attente, confirmee_de_mon_cote, confirmee, rejetee)
├── membre_id_anonymise: String ("Membre #XXXX")
├── detectee_le: DateTime
└── confirmee_le: Option<DateTime>
```

### ArbreDecouvertResponse

```
ArbreDecouvertResponse
├── suggestion_id: UUID
├── personne_commune: PersonneResumeResponse
├── personnes: Vec<PersonneNoeudResponse> (toutes les personnes de l'autre arbre)
├── liens: Vec<LienArbreResponse> (tous les liens de l'autre arbre)
└── membre_id_anonymise: String
```

### DemandeContactResponse

```
DemandeContactResponse
├── id: UUID
├── suggestion_id: UUID
├── statut: String (en_attente, acceptee, refusee)
├── profil_membre: Option<ProfilPublicResponse> (null tant que non accepté, {nom, prenom, email} si accepté)
└── created_at: DateTime
```

## Diagramme de relations

```
┌──────────────────────┐     ┌──────────────────────┐
│   rattachements      │     │   rattachements      │
│   (arbre A)          │     │   (arbre B)          │
└──────────┬───────────┘     └──────────┬───────────┘
           │                            │
           │    ┌─────────────────────┐  │
           └───►│ suggestions_        │◄─┘
                │ correspondance      │
                │                     │
                │ statut: en_attente  │
                │ → confirmee        │
                └─────────┬───────────┘
                          │
                ┌─────────▼───────────┐
                │ demandes_contact    │
                │                     │
                │ demandeur_id ──► iam.utilisateur
                │ destinataire_id ─► iam.utilisateur
                └─────────────────────┘
```
