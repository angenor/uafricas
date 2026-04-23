# Data Model: Validation Admin des Bibliothèques Humaines

**Date**: 2026-04-22 | **Feature**: 001-admin-biblio-humaine

## Nouveau schéma SQL

### Enum de statut

```sql
CREATE TYPE iam.statut_demande_biblio AS ENUM ('en_attente', 'valide', 'rejete');
```

### Table principale des demandes

```sql
CREATE TABLE iam.demande_biblio_humaine (
    id               UUID         DEFAULT gen_random_uuid() PRIMARY KEY,
    utilisateur_id   UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    statut           iam.statut_demande_biblio NOT NULL DEFAULT 'en_attente',
    fonction         VARCHAR(255) NOT NULL,
    biographie       TEXT         NOT NULL CHECK (length(biographie) >= 20),
    pays_origine_id  UUID         REFERENCES shared.pays(id),
    commentaire_admin TEXT,
    traite_par       UUID         REFERENCES iam.utilisateur(id),
    traite_le        TIMESTAMPTZ,
    created_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at       TIMESTAMPTZ
);
```

### Table de jointure demande–spécialités

```sql
CREATE TABLE iam.demande_biblio_specialite (
    demande_id    UUID NOT NULL REFERENCES iam.demande_biblio_humaine(id) ON DELETE CASCADE,
    specialite_id UUID NOT NULL REFERENCES iam.specialite_bibliotheque(id) ON DELETE CASCADE,
    PRIMARY KEY (demande_id, specialite_id)
);
```

### Index

```sql
-- Accès rapide aux demandes d'un utilisateur
CREATE INDEX idx_demande_biblio_utilisateur
    ON iam.demande_biblio_humaine(utilisateur_id)
    WHERE deleted_at IS NULL;

-- Filtrage par statut (principal usage admin)
CREATE INDEX idx_demande_biblio_statut
    ON iam.demande_biblio_humaine(statut)
    WHERE deleted_at IS NULL;

-- Contrainte unicité : un seul en_attente ou valide par utilisateur
CREATE UNIQUE INDEX idx_demande_biblio_active_unique
    ON iam.demande_biblio_humaine(utilisateur_id)
    WHERE statut IN ('en_attente', 'valide') AND deleted_at IS NULL;
```

---

## Tables existantes (non modifiées)

```sql
-- Flag activé lors de la VALIDATION admin (pas à l'inscription)
iam.utilisateur.bibliotheque_humain  BOOLEAN NOT NULL DEFAULT FALSE

-- Spécialités actives de l'utilisateur (remplies lors de la validation)
iam.specialite_bibliotheque (id UUID, nom VARCHAR, slug VARCHAR)
iam.utilisateur_specialite  (utilisateur_id UUID, specialite_id UUID)
```

---

## Diagramme des relations

```
iam.utilisateur (1) ──────────────── (N) iam.demande_biblio_humaine
      │                                           │
      │                                           │ (N)
      │                                           ▼
      │                               iam.demande_biblio_specialite
      │                                           │ (N)
      │                                           ▼
      └──── iam.utilisateur_specialite ◄── iam.specialite_bibliotheque
                 (activé lors validation)
```

---

## Transitions d'état

```
                    submit
[aucune demande] ──────────► [en_attente]
                                  │
                      valider │   │ rejeter
                               ▼  ▼
                   [valide]  [rejete]
                      │          │
              rejeter │          │ submit nouvelle demande
                      ▼          ▼
                   [rejete]  [en_attente]
```

**Règles** :
- Nouvelle demande possible uniquement si `statut = rejete` ou aucune demande active
- La validation admin applique les changements de profil en transaction atomique
- `valide` → `rejete` possible (décision réversible)

---

## Types Rust

### Ajouts dans `src/models/bibliotheque_humaine.rs`

```rust
#[derive(Debug, Serialize)]
pub struct DemandeCreeeResponse {
    pub id: Uuid,
    pub statut: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct MaDemandeResponse {
    pub id: Uuid,
    pub statut: String,
    pub fonction: String,
    pub biographie: String,
    pub pays: Option<String>,
    pub specialites: Vec<String>,
    pub commentaire_admin: Option<String>,
    pub created_at: DateTime<Utc>,
    pub traite_le: Option<DateTime<Utc>>,
}
```

### Nouveau fichier `src/models/admin/biblio_humaine.rs`

```rust
#[derive(Debug, FromRow)]
pub struct AdminDemandeBiblioRow {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub fonction: String,
    pub biographie: String,
    pub pays_nom: Option<String>,
    pub statut: String,
    pub specialites_noms: String,
    pub commentaire_admin: Option<String>,
    pub traite_par_nom: Option<String>,
    pub traite_le: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct TraiterDemandeBody {
    pub commentaire: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdminDemandeBiblioQueryParams {
    pub statut: Option<String>,
    pub recherche: Option<String>,
    pub page: Option<i64>,
    pub par_page: Option<i64>,
}
```

---

## Types TypeScript

```typescript
// Ajouts dans useBibliothequeHumaine.ts
export interface DemandeCreeeAPI {
  id: string
  statut: 'en_attente'
  createdAt: string
}

export interface MaDemandeAPI {
  id: string
  statut: 'en_attente' | 'valide' | 'rejete'
  fonction: string
  biographie: string
  pays: string | null
  specialites: string[]
  commentaireAdmin: string | null
  createdAt: string
  traiteLe: string | null
}

// Dans useAdminBibliosHumaines.ts
export interface AdminDemandeBiblio {
  id: string
  utilisateurId: string
  nom: string
  prenom: string
  email: string
  photoUrl: string | null
  fonction: string
  statut: 'en_attente' | 'valide' | 'rejete'
  specialites: string[]
  createdAt: string
}

export interface AdminDemandeBiblioDetail extends AdminDemandeBiblio {
  biographie: string
  pays: string | null
  commentaireAdmin: string | null
  traiteLe: string | null
  traiteParNom: string | null
}
```
