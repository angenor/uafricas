-- ============================================================================
-- 11i — country_profile : recettes culinaires populaires (objet Afripulse)
-- ----------------------------------------------------------------------------
-- Nouvel objet contribué/modéré rattaché à une fiche pays, sur le même canal de
-- contribution modérée que les sites touristiques / secteurs / personnalités.
-- Contenu : titre, territoires de consommation, petite histoire/présentation,
-- ingrédients nécessaires, mode de préparation (étapes 1 à 10), images
-- illustratives (jusqu'à 5).
--
-- Migration idempotente. NB : `ALTER TYPE ... ADD VALUE` s'exécute hors
-- transaction explicite (autocommit psql) — même pattern que 11c (etat_contribution).
-- ============================================================================

-- 1) Extension des enums Afripulse -------------------------------------------
ALTER TYPE country_profile.type_objet_contribution ADD VALUE IF NOT EXISTS 'recette_culinaire';
ALTER TYPE country_profile.section_afripulse        ADD VALUE IF NOT EXISTS 'recettes_culinaires';

-- 2) Table des recettes culinaires -------------------------------------------
CREATE TABLE IF NOT EXISTS country_profile.recette_culinaire (
    id                       UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id            UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    titre                    VARCHAR(250) NOT NULL,
    territoires_consommation TEXT,                       -- territoires où la recette est consommée
    histoire                 TEXT,                       -- petite histoire / présentation
    ingredients              TEXT[] NOT NULL DEFAULT '{}',
    etapes_preparation       TEXT[] NOT NULL DEFAULT '{}',  -- étapes 1 à 10
    images                   TEXT[] NOT NULL DEFAULT '{}',  -- images illustratives (≤ 5)
    cree_par                 UUID NOT NULL,              -- [xref] iam.utilisateur
    created_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at               TIMESTAMPTZ                 -- soft delete
);

-- Plafonds de cardinalité (10 étapes max, 5 images max), idempotents
ALTER TABLE country_profile.recette_culinaire
    DROP CONSTRAINT IF EXISTS chk_recette_etapes_max10;
ALTER TABLE country_profile.recette_culinaire
    ADD CONSTRAINT chk_recette_etapes_max10 CHECK (cardinality(etapes_preparation) <= 10);

ALTER TABLE country_profile.recette_culinaire
    DROP CONSTRAINT IF EXISTS chk_recette_images_max5;
ALTER TABLE country_profile.recette_culinaire
    ADD CONSTRAINT chk_recette_images_max5 CHECK (cardinality(images) <= 5);

CREATE INDEX IF NOT EXISTS idx_recette_culinaire_fiche
    ON country_profile.recette_culinaire (fiche_pays_id)
    WHERE deleted_at IS NULL;

-- 3) Trigger updated_at (idempotent) -----------------------------------------
DROP TRIGGER IF EXISTS trg_recette_culinaire_updated ON country_profile.recette_culinaire;
CREATE TRIGGER trg_recette_culinaire_updated
    BEFORE UPDATE ON country_profile.recette_culinaire
    FOR EACH ROW EXECUTE FUNCTION shared.trigger_set_updated_at();
