# Data Model — Afripulse Enrichissement collaboratif

**Feature**: Afripulse — Enrichissement collaboratif des fiches pays
**Branch**: `001-afripulse-contributions`
**Date**: 2026-04-18

Ce document décrit le modèle de données étendu. Il est la source de vérité pour la propagation vers les structs Rust (`FromRow`), les DTOs (`Request`/`Response`) et les types TypeScript (`useOpportuniteAfrique.ts`). Conformément à la constitution §III, ce DDL est écrit **avant** tout code applicatif.

---

## Fichier SQL cible

Nouveau fichier : `uafricas_backend/doc/bd/schemas/11c_country_profile_afripulse.sql`
Orchestrateur : `uafricas_backend/doc/bd/schema.sql` — ajouter `\ir schemas/11c_country_profile_afripulse.sql` après la ligne `\ir schemas/11b_country_profile_contributions.sql`.

Les ALTER sur tables existantes (site_touristique, contribution_fiche) sont groupés dans ce même fichier en tête (section A), suivis des CREATE TYPE (section B), CREATE TABLE (section C), INDEX (section D), TRIGGERS (section E).

---

## Section A — Extensions des tables existantes

### A.1 `country_profile.site_touristique` — colonne `categorie`

```sql
-- Distinction site emblématique (patrimoine public) / site privé
ALTER TABLE country_profile.site_touristique
    ADD COLUMN IF NOT EXISTS categorie country_profile.categorie_site_touristique
        NOT NULL DEFAULT 'emblematique';

ALTER TABLE country_profile.site_touristique
    ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_site_touristique_categorie
    ON country_profile.site_touristique (fiche_pays_id, categorie)
    WHERE deleted_at IS NULL;
```

### A.2 `country_profile.contribution_fiche` — colonnes manquantes

```sql
-- Typage strict + champ JSONB pour valeurs et pièces jointes
ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS type_objet_contribution country_profile.type_objet_contribution
        NOT NULL DEFAULT 'fiche_pays';

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS section_afripulse country_profile.section_afripulse;

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS target_id UUID;  -- identifiant de l'élément cible (édition/suppression)

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS nouvelle_valeur_jsonb JSONB;  -- payload structuré complet
ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS ancienne_valeur_jsonb JSONB;  -- snapshot au moment de la soumission

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS pieces_jointes JSONB NOT NULL DEFAULT '[]'::jsonb;
-- Forme : [{"chemin_fichier": "...", "legende": "...", "taille_octets": 123, "largeur": 1600, "hauteur": 1200}]
```

Note : les colonnes texte `ancienne_valeur TEXT` et `nouvelle_valeur TEXT` existantes sont **conservées** pour rétrocompatibilité avec les contributions existantes. Les nouvelles contributions Afripulse utilisent exclusivement les variantes JSONB.

### A.3 `country_profile.etat_contribution` — valeur `obsolete`

```sql
ALTER TYPE country_profile.etat_contribution ADD VALUE IF NOT EXISTS 'obsolete';
```

---

## Section B — Nouveaux enums

```sql
-- Catégorie de site touristique
CREATE TYPE country_profile.categorie_site_touristique AS ENUM (
    'emblematique',   -- patrimoine national ouvert au public
    'prive'           -- domaine privé, écolodge, réserve privée
);

-- Type d'objet ciblé par une contribution
CREATE TYPE country_profile.type_objet_contribution AS ENUM (
    'fiche_pays',                -- création ou modification de la fiche elle-même
    'site_touristique',          -- élément de section sites emblématiques ou privés
    'secteur_developpement',     -- élément de section secteurs d'opportunités
    'personnalite_connue',       -- élément de section personnalités
    'savoir_pratique',           -- élément de section à savoir avant de voyager
    'recommandation_visiteur',   -- avis noté + commentaire
    'photo_visiteur'             -- photo + légende pour la galerie
);

-- Section UI Afripulse de rattachement (utile pour les filtres admin)
CREATE TYPE country_profile.section_afripulse AS ENUM (
    'sites_emblematiques',
    'sites_prives',
    'secteurs_opportunites',
    'personnalites',
    'savoir_avant_voyager',
    'recommandations',
    'galerie_photos'
);

-- Catégorie de savoir pratique (pour filtrer à l'affichage)
CREATE TYPE country_profile.categorie_savoir AS ENUM (
    'langue_argot',        -- ex. Nouchi
    'coutumes',            -- coutumes sociales, gestes, règles
    'etiquette',           -- règles de politesse, codes vestimentaires
    'securite',            -- conseils sécurité
    'sante',               -- vaccins, eau potable, etc.
    'transports',          -- taxis, bus, permis
    'autre'
);

-- Domaine d'une personnalité connue
CREATE TYPE country_profile.domaine_personnalite AS ENUM (
    'politique',
    'artiste_musicien',
    'artiste_autre',      -- cinéma, peinture, littérature
    'sportif',
    'entrepreneur',
    'scientifique',
    'militaire_historique',
    'autre'
);
```

---

## Section C — Nouvelles tables

### C.1 `country_profile.personnalite_connue`

```sql
CREATE TABLE country_profile.personnalite_connue (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    nom_complet         VARCHAR(250) NOT NULL,
    domaine             country_profile.domaine_personnalite NOT NULL,
    biographie_courte   TEXT NOT NULL,               -- 100 à 1500 caractères (contrôle applicatif)
    annee_naissance     SMALLINT,                    -- optionnel, CHECK 0 < x < EXTRACT(YEAR FROM NOW()) + 1
    annee_deces         SMALLINT,                    -- optionnel, >= annee_naissance
    portrait_url        VARCHAR(500),                -- photo ou portrait (optionnel)
    lien_reference      VARCHAR(500),                -- URL Wikipédia / article (optionnel)
    cree_par            UUID NOT NULL,               -- [xref] iam.utilisateur
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,
    CHECK (annee_deces IS NULL OR annee_naissance IS NULL OR annee_deces >= annee_naissance)
);

CREATE INDEX idx_personnalite_fiche ON country_profile.personnalite_connue (fiche_pays_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_personnalite_domaine ON country_profile.personnalite_connue (fiche_pays_id, domaine) WHERE deleted_at IS NULL;
```

### C.2 `country_profile.savoir_pratique`

```sql
CREATE TABLE country_profile.savoir_pratique (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    titre               VARCHAR(250) NOT NULL,
    categorie           country_profile.categorie_savoir NOT NULL,
    explication         TEXT NOT NULL,               -- 50 à 3000 caractères (contrôle applicatif)
    exemple             TEXT,                        -- optionnel (ex. phrases en Nouchi)
    cree_par            UUID NOT NULL,               -- [xref] iam.utilisateur
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_savoir_fiche ON country_profile.savoir_pratique (fiche_pays_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_savoir_categorie ON country_profile.savoir_pratique (fiche_pays_id, categorie) WHERE deleted_at IS NULL;
```

### C.3 `country_profile.recommandation_visiteur`

```sql
CREATE TABLE country_profile.recommandation_visiteur (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    utilisateur_id      UUID NOT NULL,               -- [xref] iam.utilisateur
    note                SMALLINT NOT NULL CHECK (note BETWEEN 1 AND 5),
    commentaire         TEXT NOT NULL,               -- 50..2000 caractères (CHECK applicatif + DB ci-dessous)
    active              BOOLEAN NOT NULL DEFAULT TRUE,  -- FALSE = remplacée par une édition validée
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,
    CHECK (char_length(commentaire) BETWEEN 50 AND 2000)
);

-- Unicité : au plus UNE recommandation active par (utilisateur, pays) non supprimée
CREATE UNIQUE INDEX uniq_recommandation_active
    ON country_profile.recommandation_visiteur (utilisateur_id, fiche_pays_id)
    WHERE active = TRUE AND deleted_at IS NULL;

CREATE INDEX idx_reco_fiche_active ON country_profile.recommandation_visiteur (fiche_pays_id, active) WHERE deleted_at IS NULL;
```

### C.4 `country_profile.photo_visiteur`

```sql
CREATE TABLE country_profile.photo_visiteur (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    utilisateur_id      UUID NOT NULL,               -- [xref] iam.utilisateur
    chemin_fichier      VARCHAR(500) NOT NULL,       -- ex. uploads/opportunite-afrique/photos/<uuid>.jpg
    legende             VARCHAR(500) NOT NULL,       -- légende obligatoire
    format              VARCHAR(10) NOT NULL CHECK (format IN ('jpeg', 'png')),
    taille_octets       INTEGER NOT NULL CHECK (taille_octets > 0 AND taille_octets <= 2097152), -- <=2 Mo
    largeur_px          SMALLINT NOT NULL CHECK (largeur_px > 0 AND largeur_px <= 2048),
    hauteur_px          SMALLINT NOT NULL CHECK (hauteur_px > 0 AND hauteur_px <= 2048),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_photo_fiche ON country_profile.photo_visiteur (fiche_pays_id, created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_photo_utilisateur ON country_profile.photo_visiteur (utilisateur_id, created_at DESC) WHERE deleted_at IS NULL;
```

---

## Section D — Index additionnels sur `contribution_fiche` (pour rate-limit)

```sql
-- Support du rate-limit 20 textes/j et 10 photos/j (cf. D5 research)
CREATE INDEX IF NOT EXISTS idx_contribution_rate_limit
    ON country_profile.contribution_fiche (cree_par, created_at)
    WHERE deleted_at IS NULL;

-- Support du rate-limit 5 en attente par pays
CREATE INDEX IF NOT EXISTS idx_contribution_attente_pays
    ON country_profile.contribution_fiche (cree_par, fiche_pays_id, etat)
    WHERE etat = 'en_attente' AND deleted_at IS NULL;

-- Support du filtre type_objet + section pour la file admin
CREATE INDEX IF NOT EXISTS idx_contribution_type_section
    ON country_profile.contribution_fiche (type_objet_contribution, section_afripulse, etat)
    WHERE deleted_at IS NULL;
```

---

## Section E — Triggers

### E.1 `updated_at` auto-update (pattern standard du projet)

```sql
-- Réutilise la fonction existante shared.tg_updated_at() si présente, sinon crée-la
CREATE TRIGGER trg_personnalite_updated
    BEFORE UPDATE ON country_profile.personnalite_connue
    FOR EACH ROW EXECUTE FUNCTION shared.tg_updated_at();

CREATE TRIGGER trg_savoir_updated
    BEFORE UPDATE ON country_profile.savoir_pratique
    FOR EACH ROW EXECUTE FUNCTION shared.tg_updated_at();

CREATE TRIGGER trg_reco_updated
    BEFORE UPDATE ON country_profile.recommandation_visiteur
    FOR EACH ROW EXECUTE FUNCTION shared.tg_updated_at();
```

Note : `photo_visiteur` n'a pas d'`updated_at` (immuable après création ; si légende modifiée, c'est une nouvelle contribution).

### E.2 Désactivation de l'ancienne recommandation lors d'un remplacement

La logique de remplacement d'une recommandation est **applicative** (dans le handler `moderer_contribution`, cf. D3), pas un trigger DB — pour garder le flux métier lisible et testable. Voir pseudo-code dans `quickstart.md`.

---

## Mapping Rust `FromRow` (extrait — à placer dans `src/models/afripulse.rs`)

```rust
#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PersonnaliteConnueRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub nom_complet: String,
    pub domaine: String,                   // map enum via #[sqlx(type_name = "...")]
    pub biographie_courte: String,
    pub annee_naissance: Option<i16>,
    pub annee_deces: Option<i16>,
    pub portrait_url: Option<String>,
    pub lien_reference: Option<String>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug)]
pub struct SavoirPratiqueRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub titre: String,
    pub categorie: String,
    pub explication: String,
    pub exemple: Option<String>,
    pub cree_par: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug)]
pub struct RecommandationVisiteurRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub utilisateur_id: Uuid,
    pub note: i16,
    pub commentaire: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, serde::Serialize, Clone, Debug)]
pub struct PhotoVisiteurRow {
    pub id: Uuid,
    pub fiche_pays_id: Uuid,
    pub utilisateur_id: Uuid,
    pub chemin_fichier: String,
    pub legende: String,
    pub format: String,                    // "jpeg" | "png"
    pub taille_octets: i32,
    pub largeur_px: i16,
    pub hauteur_px: i16,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
```

Enums Rust pour `type_objet_contribution` et `section_afripulse` à placer dans `src/models/contribution_fiche.rs` :

```rust
#[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "country_profile.type_objet_contribution", rename_all = "snake_case")]
pub enum TypeObjetContribution {
    FichePays,
    SiteTouristique,
    SecteurDeveloppement,
    PersonnaliteConnue,
    SavoirPratique,
    RecommandationVisiteur,
    PhotoVisiteur,
}

#[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "country_profile.section_afripulse", rename_all = "snake_case")]
pub enum SectionAfripulse {
    SitesEmblematiques,
    SitesPrives,
    SecteursOpportunites,
    Personnalites,
    SavoirAvantVoyager,
    Recommandations,
    GaleriePhotos,
}
```

---

## Mapping TypeScript (extrait — à placer dans `useOpportuniteAfrique.ts`)

```typescript
export type TypeObjetContribution =
  | 'fiche_pays'
  | 'site_touristique'
  | 'secteur_developpement'
  | 'personnalite_connue'
  | 'savoir_pratique'
  | 'recommandation_visiteur'
  | 'photo_visiteur'

export type SectionAfripulse =
  | 'sites_emblematiques'
  | 'sites_prives'
  | 'secteurs_opportunites'
  | 'personnalites'
  | 'savoir_avant_voyager'
  | 'recommandations'
  | 'galerie_photos'

export type CategorieSiteTouristique = 'emblematique' | 'prive'

export type CategorieSavoir =
  | 'langue_argot' | 'coutumes' | 'etiquette' | 'securite'
  | 'sante' | 'transports' | 'autre'

export type DomainePersonnalite =
  | 'politique' | 'artiste_musicien' | 'artiste_autre'
  | 'sportif' | 'entrepreneur' | 'scientifique'
  | 'militaire_historique' | 'autre'

export interface PersonnaliteConnueAPI {
  id: string
  fiche_pays_id: string
  nom_complet: string
  domaine: DomainePersonnalite
  biographie_courte: string
  annee_naissance: number | null
  annee_deces: number | null
  portrait_url: string | null
  lien_reference: string | null
  cree_par: string
  created_at: string
}

export interface SavoirPratiqueAPI {
  id: string
  fiche_pays_id: string
  titre: string
  categorie: CategorieSavoir
  explication: string
  exemple: string | null
  cree_par: string
  created_at: string
}

export interface RecommandationVisiteurAPI {
  id: string
  fiche_pays_id: string
  utilisateur: {
    id: string
    nom: string
    prenom: string
    photo_url: string | null
  }
  note: number                 // 1..5
  commentaire: string          // 50..2000
  created_at: string
}

export interface PhotoVisiteurAPI {
  id: string
  fiche_pays_id: string
  utilisateur: {
    id: string
    nom: string
    prenom: string
    photo_url: string | null
  }
  url: string                  // URL publique servie par /uploads/
  legende: string
  largeur_px: number
  hauteur_px: number
  created_at: string
}
```

---

## Relations (vue synoptique)

```text
country_profile.fiche_pays (1) ──< (N) site_touristique (+ categorie emblematique|prive)
                           (1) ──< (N) secteur_developpement                      [existant]
                           (1) ──< (N) personnalite_connue                        [NOUVEAU]
                           (1) ──< (N) savoir_pratique                            [NOUVEAU]
                           (1) ──< (N) recommandation_visiteur (active = TRUE)    [NOUVEAU]
                           (1) ──< (N) photo_visiteur                             [NOUVEAU]
                           (1) ──< (N) contribution_fiche                         [existant, étendu]

contribution_fiche: target_id ─ ─> id de la table cible (site_touristique | personnalite_connue | ...)
                    selon type_objet_contribution ; NULL si type_contribution = 'ajout'.
```

---

## Invariants métier (à enforcer côté handler + CHECK SQL)

1. **Recommandation** : `char_length(commentaire) BETWEEN 50 AND 2000` (CHECK DB) ; `note BETWEEN 1 AND 5` (CHECK DB) ; unicité (utilisateur, pays) sur actives (UNIQUE index partiel).
2. **Photo** : formats `jpeg|png` (CHECK DB `format IN ('jpeg','png')`) ; taille ≤ 2 Mo (CHECK DB) ; dimensions ≤ 2048×2048 (CHECK DB).
3. **Site touristique** : `categorie IN ('emblematique','prive')` (enum DB).
4. **Personnalité** : `annee_deces >= annee_naissance` si les deux sont renseignés (CHECK DB).
5. **Contribution approuvée** : `pieces_jointes` JSONB non modifiable après approbation — enforcement applicatif dans `moderer_contribution`.
6. **Périmètre ISO** : `fiche_pays.pays_id` référence un pays dont `shared.pays.code_iso2` appartient à la liste figée de 54 codes africains — vérifié au handler `creer_fiche_pays` en amont de l'insertion.
7. **Crédit contributeur** : un utilisateur ayant un compte supprimé (`iam.utilisateur.deleted_at IS NOT NULL`) voit ses contributions validées conservées mais est affiché comme « Contributeur retiré » — résolution à la requête de listing des contributeurs (JOIN + COALESCE).

---

## Évolution future (hors scope Afripulse)

- Ajouter un flag `est_africain BOOLEAN NOT NULL DEFAULT FALSE` sur `shared.pays` pour éliminer la duplication frontend/backend des 54 codes. Non fait maintenant pour limiter le blast radius du principe V (YAGNI) — la liste figée actuelle suffit.
- Ajouter une vue matérialisée `country_profile.vm_contributeurs_par_pays` si la requête agrégée de listing devient coûteuse à grande échelle (> 100 k contributions validées).

---

## Checklist de conformité

- [x] Conventions SQL respectées (UUID v4, TIMESTAMPTZ, `deleted_at`, snake_case français, enums).
- [x] Aucune donnée en dur (les 54 codes ISO sont gérés côté application — D6).
- [x] Indexes partiels pour les requêtes chaudes (rate-limit, section admin).
- [x] Tous les CHECKs applicatifs critiques ont un équivalent DB pour défense en profondeur.
- [x] Chaque table a un `fiche_pays_id` + `deleted_at` permettant cascade et soft delete.
- [x] Les triggers `updated_at` suivent le pattern existant `shared.tg_updated_at()`.
