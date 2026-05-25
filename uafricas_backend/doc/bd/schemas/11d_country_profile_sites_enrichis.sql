-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : country_profile -- Sites touristiques enrichis
-- Feature : 001-sites-touristiques-enrichis (2026-05-25)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Ce fichier étend le schéma country_profile pour l'enrichissement des sites
-- touristiques (emblématiques et privés) :
--   • Nouvel enum : sous_type_site (20 valeurs : 12 emblématiques + 8 privées)
--   • Extension de country_profile.site_touristique (sous_type, gestionnaire,
--     localisation textuelle, info pertinente, contacts, constitution légale,
--     badge verifie)
--   • Nouvelle table : avis_site (notes 1–5 par visiteur, écriture directe)
--   • Index additionnels + trigger updated_at (pattern shared.trigger_set_updated_at())
--
-- Conformité : Principe III (SQL source de vérité). La cohérence famille↔sous-type
-- est validée côté code (Rust + TS), non contrainte en SQL (un seul enum — D1).
-- ════════════════════════════════════════════════════════════════════════════


-- ════════════════════════════════════════════════════════════════════════════
-- SECTION A : NOUVEL ENUM sous-type de site (créé AVANT les ALTER)
-- ════════════════════════════════════════════════════════════════════════════

CREATE TYPE country_profile.sous_type_site AS ENUM (
    -- Emblématiques (famille « emblematique »)
    'plage', 'monument', 'relief_naturel', 'parc_naturel', 'mosquee', 'eglise',
    'pont', 'route', 'service_public', 'immeuble_edifice', 'mer_riviere', 'site_naturel',
    -- Privés (famille « prive »)
    'hotel', 'plage_privee', 'espace_jeux', 'agriculture_touristique',
    'residence_touristique', 'restaurant', 'discotheque', 'bar_maquis'
);


-- ════════════════════════════════════════════════════════════════════════════
-- SECTION B : EXTENSION country_profile.site_touristique
-- ════════════════════════════════════════════════════════════════════════════
-- Colonnes nullables → rétrocompatibilité (FR-018) : les sites existants restent
-- valides. La famille reste portée par la colonne existante `categorie`.

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
    ADD COLUMN IF NOT EXISTS verifie_at           TIMESTAMPTZ,
    -- Galerie : jusqu'à 3 images (la 1re sert de couverture). `image_url` reste
    -- renseignée avec images[1] pour rétrocompatibilité des lectures existantes.
    ADD COLUMN IF NOT EXISTS images               TEXT[] NOT NULL DEFAULT '{}';

ALTER TABLE country_profile.site_touristique
    DROP CONSTRAINT IF EXISTS chk_site_images_max3;
ALTER TABLE country_profile.site_touristique
    ADD CONSTRAINT chk_site_images_max3 CHECK (cardinality(images) <= 3);

CREATE INDEX IF NOT EXISTS idx_site_touristique_sous_type
    ON country_profile.site_touristique (fiche_pays_id, sous_type)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_site_touristique_verifie
    ON country_profile.site_touristique (fiche_pays_id, verifie)
    WHERE deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- SECTION C : NOUVELLE TABLE avis_site (notes 1–5 par visiteur — D2)
-- ════════════════════════════════════════════════════════════════════════════
-- Écriture directe (publication immédiate). Au plus un avis actif par
-- (utilisateur, site). Modération admin = masquage (soft).

CREATE TABLE country_profile.avis_site (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    site_id          UUID NOT NULL REFERENCES country_profile.site_touristique(id) ON DELETE CASCADE,
    utilisateur_id   UUID NOT NULL,   -- [xref] iam.utilisateur
    note             SMALLINT NOT NULL CHECK (note BETWEEN 1 AND 5),
    commentaire      TEXT NOT NULL CHECK (char_length(commentaire) BETWEEN 1 AND 2000),
    masque_par_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at       TIMESTAMPTZ
);

-- Au plus UN avis actif par (utilisateur, site) (FR-015a)
CREATE UNIQUE INDEX uniq_avis_site_actif
    ON country_profile.avis_site (utilisateur_id, site_id)
    WHERE deleted_at IS NULL;

-- Lecture par site (moyenne + liste), exclut masqués/supprimés (FR-015b)
CREATE INDEX idx_avis_site_visible
    ON country_profile.avis_site (site_id, created_at DESC)
    WHERE deleted_at IS NULL AND masque_par_admin = FALSE;


-- Note : le trigger updated_at est créé automatiquement par 14_triggers.sql
-- (génération `trg_<table>_updated_at` pour toute table country_profile ayant
-- une colonne updated_at). Inutile de le déclarer ici (éviter le doublon).
