# Phase 1 : Modèle de données

Feature : Enrichissement des sites touristiques · Schéma `country_profile` (Principe III, SQL SoT)

Migration cible : `uafricas_backend/doc/bd/schemas/11d_country_profile_sites_enrichis.sql`
(orchestrée via `\ir` dans `schema.sql`, après `11c_country_profile_afripulse.sql`).

---

## 1. Nouvel enum : sous-type de site

```sql
CREATE TYPE country_profile.sous_type_site AS ENUM (
    -- Emblématiques
    'plage', 'monument', 'relief_naturel', 'parc_naturel', 'mosquee', 'eglise',
    'pont', 'route', 'service_public', 'immeuble_edifice', 'mer_riviere', 'site_naturel',
    -- Privés
    'hotel', 'plage_privee', 'espace_jeux', 'agriculture_touristique',
    'residence_touristique', 'restaurant', 'discotheque', 'bar_maquis'
);
```

Mapping famille↔sous-type (validé en code Rust/TS, non contraint en SQL) :

| Famille (`categorie`) | Sous-types autorisés |
|-----------------------|----------------------|
| `emblematique` | plage, monument, relief_naturel, parc_naturel, mosquee, eglise, pont, route, service_public, immeuble_edifice, mer_riviere, site_naturel |
| `prive` | hotel, plage_privee, espace_jeux, agriculture_touristique, residence_touristique, restaurant, discotheque, bar_maquis |

---

## 2. Extension de `country_profile.site_touristique`

Colonnes existantes conservées : `id, fiche_pays_id, nom, description, image_url, longitude,
latitude, region_id, categorie, deleted_at, created_at, updated_at`.

```sql
ALTER TABLE country_profile.site_touristique
    ADD COLUMN IF NOT EXISTS sous_type            country_profile.sous_type_site,
    ADD COLUMN IF NOT EXISTS gestionnaire         VARCHAR(250),
    ADD COLUMN IF NOT EXISTS ville                VARCHAR(150),
    ADD COLUMN IF NOT EXISTS village              VARCHAR(150),
    ADD COLUMN IF NOT EXISTS info_pertinente      TEXT,
    ADD COLUMN IF NOT EXISTS contact_telephone    VARCHAR(40),
    ADD COLUMN IF NOT EXISTS contact_courriel     VARCHAR(254),
    ADD COLUMN IF NOT EXISTS contact_adresse      VARCHAR(500),
    ADD COLUMN IF NOT EXISTS constitution_statut_juridique VARCHAR(250),
    ADD COLUMN IF NOT EXISTS constitution_numero  VARCHAR(120),
    ADD COLUMN IF NOT EXISTS constitution_document_url VARCHAR(500),
    ADD COLUMN IF NOT EXISTS verifie              BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS verifie_par          UUID,        -- [xref] iam.utilisateur
    ADD COLUMN IF NOT EXISTS verifie_at           TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_site_touristique_sous_type
    ON country_profile.site_touristique (fiche_pays_id, sous_type)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_site_touristique_verifie
    ON country_profile.site_touristique (fiche_pays_id, verifie)
    WHERE deleted_at IS NULL;
```

**Règles de validation (code)** :
- Requis à la création/édition (FR-005) : `nom`, `gestionnaire`, `ville`, `info_pertinente`,
  `latitude`, `longitude`, `sous_type`.
- Si `categorie = 'prive'` (FR-006) : au moins un de `contact_telephone`, `contact_courriel`,
  `contact_adresse`.
- `sous_type` ∈ ensemble autorisé pour `categorie` (FR-003).
- `contact_courriel` : format courriel ; `constitution_*` : facultatifs (FR-013).
- Rétrocompatibilité (FR-018) : colonnes nullables → les sites existants restent valides.

---

## 3. Nouvelle table : `country_profile.avis_site`

Avis noté 1–5 par visiteur sur un site (D2, écriture directe, publication immédiate).

```sql
CREATE TABLE country_profile.avis_site (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    site_id         UUID NOT NULL REFERENCES country_profile.site_touristique(id) ON DELETE CASCADE,
    utilisateur_id  UUID NOT NULL,   -- [xref] iam.utilisateur
    note            SMALLINT NOT NULL CHECK (note BETWEEN 1 AND 5),
    commentaire     TEXT NOT NULL CHECK (char_length(commentaire) BETWEEN 1 AND 2000),
    masque_par_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

-- Au plus UN avis actif par (utilisateur, site)  (FR-015a)
CREATE UNIQUE INDEX uniq_avis_site_actif
    ON country_profile.avis_site (utilisateur_id, site_id)
    WHERE deleted_at IS NULL;

-- Lecture par site (moyenne + liste), exclut masqués/supprimés
CREATE INDEX idx_avis_site_visible
    ON country_profile.avis_site (site_id, created_at DESC)
    WHERE deleted_at IS NULL AND masque_par_admin = FALSE;

CREATE TRIGGER trg_avis_site_updated
    BEFORE UPDATE ON country_profile.avis_site
    FOR EACH ROW EXECUTE FUNCTION shared.trigger_set_updated_at();
```

Note moyenne et nombre d'avis (FR-015b) calculés par agrégat :
`SELECT AVG(note)::float8, COUNT(*)::bigint FROM country_profile.avis_site
 WHERE site_id = $1 AND deleted_at IS NULL AND masque_par_admin = FALSE`.

---

## 4. Entités (vue logique)

### Site touristique (étendu)
- **Identité** : `id`, `fiche_pays_id` (territoire).
- **Classification** : `categorie` (famille), `sous_type`.
- **Descriptif** : `nom`, `description`, `info_pertinente`, `image_url`.
- **Localisation** : `ville`, `village`, `latitude`, `longitude`, `region_id`.
- **Gestion** : `gestionnaire`, `contact_telephone`, `contact_courriel`, `contact_adresse`.
- **Constitution légale** (facultatif) : `constitution_statut_juridique`, `constitution_numero`,
  `constitution_document_url`.
- **Fiabilité** : `verifie`, `verifie_par`, `verifie_at`.
- **Cycle de vie** : `deleted_at` (soft), `created_at`, `updated_at`.
- **Agrégats dérivés** (non stockés) : `note_moyenne`, `nombre_avis`.

### Avis site
- `id`, `site_id`, `utilisateur_id`, `note` (1–5), `commentaire` (1–2000), `masque_par_admin`,
  `deleted_at`, `created_at`, `updated_at`.
- Invariant : un avis actif au plus par (utilisateur, site).

---

## 5. Mapping cross-stack (Principe II)

| SQL (`country_profile`) | Rust (`models/afripulse.rs`, `models/admin/profils_pays.rs`) | TS (`useOpportuniteAfrique.ts`) |
|--------------------------|--------------------------------------------------------------|---------------------------------|
| enum `sous_type_site` | `enum SousTypeSite` (`#[sqlx(type_name=...)]`) | `type SousTypeSite = '…'` + libellés |
| `site_touristique` (+colonnes) | `SiteTouristiqueResponse`, `Admin*`/`Creer*`/`Modifier*SiteTouristiqueRequest` étendus | `SiteTouristiqueAPI` étendu |
| `avis_site` | `AvisSiteRow` + DTO `AvisSiteResponse`, `AvisSiteListeResponse` | `AvisSiteAPI`, `AvisSiteListe` |
| `verifie`/`verifie_par`/`verifie_at` | DTO toggle `VerificationSiteBody { verifie: bool }` | méthode `definirVerificationSite` (admin) |
