-- ============================================================================
-- 11h — country_profile : réactions, signalements et partages des fiches pays
-- ----------------------------------------------------------------------------
-- Permet aux membres authentifiés de :
--   • aimer / ne pas aimer une fiche pays (like/dislike binaire, une réaction
--     par utilisateur, bascule on/off — modèle culture.codimoi_reaction) ;
--   • signaler une fiche (un signalement par utilisateur). Au-delà de
--     SEUIL_SIGNALEMENTS_BLOCAGE (= 10) signalements distincts, la fiche est
--     automatiquement bloquée (retirée du public — modèle factcheck) ;
--   • partager une fiche dans le mur communautaire « /publications » avec une
--     légende facultative (3ᵉ source agrégée du mur).
--
-- Migration idempotente (rétrocompatible) : colonnes nullables / défaut,
-- tables IF NOT EXISTS.
-- ============================================================================

-- 1) Compteurs et drapeau de blocage sur la fiche pays ------------------------
ALTER TABLE country_profile.fiche_pays
    ADD COLUMN IF NOT EXISTS nombre_likes        INT     NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS nombre_dislikes     INT     NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS nombre_signalements INT     NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS bloquee             BOOLEAN NOT NULL DEFAULT FALSE;

-- 2) Réactions like / dislike (une par utilisateur et par fiche) --------------
CREATE TABLE IF NOT EXISTS country_profile.reaction_fiche (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id  UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    utilisateur_id UUID NOT NULL,                       -- [xref] iam.utilisateur
    type_reaction  VARCHAR(10) NOT NULL CHECK (type_reaction IN ('like', 'dislike')),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (fiche_pays_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_reaction_fiche_fiche
    ON country_profile.reaction_fiche (fiche_pays_id);

-- 3) Signalements (un par utilisateur et par fiche) ---------------------------
CREATE TABLE IF NOT EXISTS country_profile.signalement_fiche (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id  UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    utilisateur_id UUID NOT NULL,                       -- [xref] iam.utilisateur
    motif          TEXT CHECK (motif IS NULL OR char_length(motif) <= 1000),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (fiche_pays_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_signalement_fiche_fiche
    ON country_profile.signalement_fiche (fiche_pays_id);

-- 4) Partages vers le mur communautaire (avec légende facultative) ------------
CREATE TABLE IF NOT EXISTS country_profile.partage_fiche (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id  UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    utilisateur_id UUID NOT NULL,                       -- [xref] iam.utilisateur
    legende        TEXT CHECK (legende IS NULL OR char_length(legende) <= 500),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ                          -- soft delete
);

CREATE INDEX IF NOT EXISTS idx_partage_fiche_actifs
    ON country_profile.partage_fiche (created_at DESC)
    WHERE deleted_at IS NULL;
