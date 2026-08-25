# Data Model: Modèle de données des personnes et liens familiaux

**Branch**: `001-personnes-arbre` | **Date**: 2026-03-15

## Vue d'ensemble

```
iam.utilisateurs (existant)
        │
        │ 1
        ▼
arbre_genealogique.arbres       ← 1 arbre par utilisateur
        │
        │ 1..*
        ▼
arbre_genealogique.rattachements   ← lie une Personne à un Arbre
        │ *                              │ *
        │                               │
        ▼                               ▼
arbre_genealogique.personnes   arbre_genealogique.liens_familiaux
  (entité partageable)           (entre 2 rattachements du même arbre)
```

## Schema SQL : `23_arbre_genealogique.sql`

```sql
-- =============================================================
-- SCHEMA : arbre_genealogique
-- Bounded context : Arbre généalogique et liens familiaux
-- Créé : 2026-03-15
-- =============================================================

CREATE SCHEMA IF NOT EXISTS arbre_genealogique;

-- -------------------------------------------------------------
-- TABLE : personnes
-- Représente une personne réelle, indépendante de tout arbre.
-- Entité partageable (plusieurs arbres pourront la référencer
-- via un rattachement). Chaque composante de date est stockée
-- séparément pour permettre une saisie à granularité variable.
-- -------------------------------------------------------------
CREATE TABLE arbre_genealogique.personnes (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom                 VARCHAR(255) NOT NULL,
    prenoms             VARCHAR(500),
    genre               VARCHAR(20) CHECK (genre IN ('masculin', 'feminin', 'autre', 'non_precise')),
    -- Date de naissance (granularité variable : année seule, mois+année, ou complète)
    naissance_annee     SMALLINT CHECK (naissance_annee BETWEEN 1 AND 9999),
    naissance_mois      SMALLINT CHECK (naissance_mois BETWEEN 1 AND 12),
    naissance_jour      SMALLINT CHECK (naissance_jour BETWEEN 1 AND 31),
    naissance_lieu      VARCHAR(500),
    -- Date de décès (mêmes règles)
    deces_annee         SMALLINT CHECK (deces_annee BETWEEN 1 AND 9999),
    deces_mois          SMALLINT CHECK (deces_mois BETWEEN 1 AND 12),
    deces_jour          SMALLINT CHECK (deces_jour BETWEEN 1 AND 31),
    deces_lieu          VARCHAR(500),
    photo_url           VARCHAR(1000),
    cree_par            UUID REFERENCES iam.utilisateurs(id) ON DELETE SET NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

-- Contrainte cohérence dates : décès >= naissance (sur les années connues)
ALTER TABLE arbre_genealogique.personnes ADD CONSTRAINT chk_coherence_dates
    CHECK (
        naissance_annee IS NULL OR deces_annee IS NULL OR
        deces_annee > naissance_annee OR
        (deces_annee = naissance_annee AND (
            naissance_mois IS NULL OR deces_mois IS NULL OR
            deces_mois > naissance_mois OR
            (deces_mois = naissance_mois AND (
                naissance_jour IS NULL OR deces_jour IS NULL OR
                deces_jour >= naissance_jour
            ))
        ))
    );

CREATE INDEX idx_personnes_nom ON arbre_genealogique.personnes(nom) WHERE deleted_at IS NULL;
CREATE INDEX idx_personnes_cree_par ON arbre_genealogique.personnes(cree_par) WHERE deleted_at IS NULL;

-- -------------------------------------------------------------
-- TABLE : arbres
-- Un arbre généalogique par utilisateur (UNIQUE sur utilisateur_id).
-- Créé automatiquement lors du premier ajout de personne.
-- -------------------------------------------------------------
CREATE TABLE arbre_genealogique.arbres (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id      UUID NOT NULL UNIQUE REFERENCES iam.utilisateurs(id) ON DELETE CASCADE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_arbres_utilisateur ON arbre_genealogique.arbres(utilisateur_id) WHERE deleted_at IS NULL;

-- -------------------------------------------------------------
-- TABLE : rattachements
-- Lie une Personne réelle à un Arbre spécifique.
-- Une Personne ne peut apparaître qu'une seule fois par arbre.
-- Suppression soft : deleted_at positionné par le handler Rust.
-- -------------------------------------------------------------
CREATE TABLE arbre_genealogique.rattachements (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    arbre_id            UUID NOT NULL REFERENCES arbre_genealogique.arbres(id) ON DELETE CASCADE,
    personne_id         UUID NOT NULL REFERENCES arbre_genealogique.personnes(id) ON DELETE CASCADE,
    ajoute_le           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,
    UNIQUE (arbre_id, personne_id)
);

CREATE INDEX idx_rattachements_arbre ON arbre_genealogique.rattachements(arbre_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_rattachements_personne ON arbre_genealogique.rattachements(personne_id) WHERE deleted_at IS NULL;

-- -------------------------------------------------------------
-- TABLE : liens_familiaux
-- Relation typée entre deux Rattachements du même Arbre.
-- Types parent-enfant : 'pere', 'mere', 'parent' (non précisé)
--   → rattachement_source_id = le parent
--   → rattachement_cible_id = l'enfant
-- Type conjoint : 'conjoint' (symétrique)
--   → Convention : rattachement_source_id < rattachement_cible_id (UUID)
--     garantit l'unicité sans doublon inverse.
-- Détection cycle : vérifiée applicativement (CTE récursive) avant INSERT.
-- -------------------------------------------------------------
CREATE TABLE arbre_genealogique.liens_familiaux (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    arbre_id                UUID NOT NULL REFERENCES arbre_genealogique.arbres(id) ON DELETE CASCADE,
    rattachement_source_id  UUID NOT NULL REFERENCES arbre_genealogique.rattachements(id) ON DELETE CASCADE,
    rattachement_cible_id   UUID NOT NULL REFERENCES arbre_genealogique.rattachements(id) ON DELETE CASCADE,
    type_lien               VARCHAR(20) NOT NULL CHECK (type_lien IN ('pere', 'mere', 'parent', 'conjoint')),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ,
    -- Unicité : même lien ne peut pas être créé deux fois
    UNIQUE (arbre_id, rattachement_source_id, rattachement_cible_id, type_lien),
    -- Un rattachement ne peut pas être lié à lui-même
    CHECK (rattachement_source_id <> rattachement_cible_id)
);

CREATE INDEX idx_liens_arbre ON arbre_genealogique.liens_familiaux(arbre_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_liens_source ON arbre_genealogique.liens_familiaux(rattachement_source_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_liens_cible ON arbre_genealogique.liens_familiaux(rattachement_cible_id) WHERE deleted_at IS NULL;
```

## Structs Rust : `src/models/arbre_genealogique.rs`

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ─── Colonnes SELECT ──────────────────────────────────────────────────────

pub const PERSONNE_COLONNES: &str = r#"
    p.id,
    p.nom,
    p.prenoms,
    p.genre,
    p.naissance_annee,
    p.naissance_mois,
    p.naissance_jour,
    p.naissance_lieu,
    p.deces_annee,
    p.deces_mois,
    p.deces_jour,
    p.deces_lieu,
    p.photo_url,
    p.cree_par,
    p.created_at,
    p.updated_at
"#;

// ─── Struct BDD (FromRow) ─────────────────────────────────────────────────

#[derive(Debug, sqlx::FromRow)]
pub struct Personne {
    pub id: Uuid,
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance_annee: Option<i16>,
    pub naissance_mois: Option<i16>,
    pub naissance_jour: Option<i16>,
    pub naissance_lieu: Option<String>,
    pub deces_annee: Option<i16>,
    pub deces_mois: Option<i16>,
    pub deces_jour: Option<i16>,
    pub deces_lieu: Option<String>,
    pub photo_url: Option<String>,
    pub cree_par: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Arbre {
    pub id: Uuid,
    pub utilisateur_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Rattachement {
    pub id: Uuid,
    pub arbre_id: Uuid,
    pub personne_id: Uuid,
    pub ajoute_le: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct LienFamilial {
    pub id: Uuid,
    pub arbre_id: Uuid,
    pub rattachement_source_id: Uuid,
    pub rattachement_cible_id: Uuid,
    pub type_lien: String,
    pub created_at: DateTime<Utc>,
}

// ─── DTOs de réponse (Serialize) ──────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PersonneResponse {
    pub id: Uuid,
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance: Option<DatePartielle>,
    pub naissance_lieu: Option<String>,
    pub deces: Option<DatePartielle>,
    pub deces_lieu: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Représente une date à granularité variable
#[derive(Debug, Serialize)]
pub struct DatePartielle {
    pub annee: Option<i16>,
    pub mois: Option<i16>,
    pub jour: Option<i16>,
}

impl PersonneResponse {
    pub fn from_row(p: Personne) -> Self {
        let naissance = if p.naissance_annee.is_some() || p.naissance_mois.is_some() || p.naissance_jour.is_some() {
            Some(DatePartielle { annee: p.naissance_annee, mois: p.naissance_mois, jour: p.naissance_jour })
        } else { None };
        let deces = if p.deces_annee.is_some() || p.deces_mois.is_some() || p.deces_jour.is_some() {
            Some(DatePartielle { annee: p.deces_annee, mois: p.deces_mois, jour: p.deces_jour })
        } else { None };
        Self {
            id: p.id, nom: p.nom, prenoms: p.prenoms, genre: p.genre,
            naissance, naissance_lieu: p.naissance_lieu,
            deces, deces_lieu: p.deces_lieu,
            photo_url: p.photo_url, created_at: p.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PersonneDetailResponse {
    pub personne: PersonneResponse,
    pub parents: Vec<LienResumeResponse>,
    pub enfants: Vec<LienResumeResponse>,
    pub conjoints: Vec<LienResumeResponse>,
}

#[derive(Debug, Serialize)]
pub struct LienResumeResponse {
    pub lien_id: Uuid,
    pub type_lien: String,
    pub personne: PersonneResponse,
}

#[derive(Debug, Serialize)]
pub struct PersonneListeResponse {
    pub personnes: Vec<PersonneResponse>,
    pub total: i64,
    pub page: i64,
    pub par_page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct LienFamilialResponse {
    pub id: Uuid,
    pub type_lien: String,
    pub personne_source_id: Uuid,
    pub personne_cible_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// ─── DTOs de requête (Deserialize) ────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreerPersonneDto {
    pub nom: String,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance: Option<DatePartielleDto>,
    pub naissance_lieu: Option<String>,
    pub deces: Option<DatePartielleDto>,
    pub deces_lieu: Option<String>,
    // photo_url : géré par upload multipart séparé
}

#[derive(Debug, Deserialize)]
pub struct DatePartielleDto {
    pub annee: Option<i16>,
    pub mois: Option<i16>,
    pub jour: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct ModifierPersonneDto {
    pub nom: Option<String>,
    pub prenoms: Option<String>,
    pub genre: Option<String>,
    pub naissance: Option<DatePartielleDto>,
    pub naissance_lieu: Option<String>,
    pub deces: Option<DatePartielleDto>,
    pub deces_lieu: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreerLienDto {
    pub rattachement_source_id: Uuid,   // parent ou premier conjoint
    pub rattachement_cible_id: Uuid,    // enfant ou second conjoint
    pub type_lien: String,              // 'pere'|'mere'|'parent'|'conjoint'
}

#[derive(Debug, Deserialize)]
pub struct PersonneQueryParams {
    pub page: Option<i64>,
    pub par_page: Option<i64>,
    pub recherche: Option<String>,
}
```

## Interfaces TypeScript : `app/mocks/arbre-genealogique.ts`

```typescript
// ─── Types ──────────────────────────────────────────────────────────────

export type Genre = 'masculin' | 'feminin' | 'autre' | 'non_precise'
export type TypeLien = 'pere' | 'mere' | 'parent' | 'conjoint'

// ─── Interfaces ──────────────────────────────────────────────────────────

export interface DatePartielle {
  annee?: number
  mois?: number
  jour?: number
}

export interface Personne {
  id: string
  nom: string
  prenoms?: string
  genre?: Genre
  naissance?: DatePartielle
  naissance_lieu?: string
  deces?: DatePartielle
  deces_lieu?: string
  photo_url?: string
  created_at: string
}

export interface LienResume {
  lien_id: string
  type_lien: TypeLien
  personne: Personne
}

export interface PersonneDetail extends Personne {
  parents: LienResume[]
  enfants: LienResume[]
  conjoints: LienResume[]
}

export interface PersonneListe {
  personnes: Personne[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

export interface LienFamilial {
  id: string
  type_lien: TypeLien
  personne_source_id: string
  personne_cible_id: string
  created_at: string
}

// ─── DTOs (formulaires) ──────────────────────────────────────────────────

export interface CreerPersonneForm {
  nom: string
  prenoms?: string
  genre?: Genre
  naissance?: DatePartielle
  naissance_lieu?: string
  deces?: DatePartielle
  deces_lieu?: string
}

export interface ModifierPersonneForm {
  nom?: string
  prenoms?: string
  genre?: Genre
  naissance?: DatePartielle
  naissance_lieu?: string
  deces?: DatePartielle
  deces_lieu?: string
}

export interface CreerLienForm {
  rattachement_source_id: string
  rattachement_cible_id: string
  type_lien: TypeLien
}

export interface PersonneQueryParams {
  page?: number
  par_page?: number
  recherche?: string
}

// ─── Données mock ────────────────────────────────────────────────────────

export const personnesMock: PersonneDetail[] = [
  {
    id: '11111111-0000-0000-0000-000000000001',
    nom: 'Diallo',
    prenoms: 'Ibrahim',
    genre: 'masculin',
    naissance: { annee: 1850 },
    naissance_lieu: 'Ségou, Mali',
    created_at: '2026-03-15T10:00:00Z',
    parents: [],
    enfants: [],
    conjoints: [],
  },
  {
    id: '11111111-0000-0000-0000-000000000002',
    nom: 'Diallo',
    prenoms: 'Ousmane Ibrahim',
    genre: 'masculin',
    naissance: { annee: 1880, mois: 3 },
    naissance_lieu: 'Ségou, Mali',
    deces: { annee: 1955, mois: 7, jour: 12 },
    deces_lieu: 'Bamako, Mali',
    created_at: '2026-03-15T10:05:00Z',
    parents: [],
    enfants: [],
    conjoints: [],
  },
]

// ─── Helpers ─────────────────────────────────────────────────────────────

const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

export const formaterDate = (date?: DatePartielle): string => {
  if (!date) return 'Inconnu'
  if (date.jour && date.mois && date.annee) {
    return `${String(date.jour).padStart(2, '0')}/${String(date.mois).padStart(2, '0')}/${date.annee}`
  }
  if (date.mois && date.annee) return `${String(date.mois).padStart(2, '0')}/${date.annee}`
  if (date.annee) return `${date.annee}`
  return 'Inconnu'
}

export const getPersonneMockById = async (id: string): Promise<PersonneDetail | undefined> => {
  await delay(100)
  return personnesMock.find(p => p.id === id)
}

export const listerPersonnesMock = async (params?: PersonneQueryParams): Promise<PersonneListe> => {
  await delay(150)
  const page = params?.page ?? 1
  const par_page = params?.par_page ?? 12
  const recherche = params?.recherche?.toLowerCase()
  const filtrees = recherche
    ? personnesMock.filter(p => p.nom.toLowerCase().includes(recherche) || p.prenoms?.toLowerCase().includes(recherche))
    : personnesMock
  const debut = (page - 1) * par_page
  return {
    personnes: filtrees.slice(debut, debut + par_page),
    total: filtrees.length,
    page,
    par_page,
    total_pages: Math.ceil(filtrees.length / par_page),
  }
}

export const formeVide = (): CreerPersonneForm => ({
  nom: '',
  prenoms: undefined,
  genre: undefined,
  naissance: undefined,
  naissance_lieu: undefined,
  deces: undefined,
  deces_lieu: undefined,
})
```
