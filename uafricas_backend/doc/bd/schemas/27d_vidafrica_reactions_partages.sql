-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : Vidafrica — Réactions (like/dislike) & partages
-- ════════════════════════════════════════════════════════════════════════════
-- Permet à tout membre authentifié de :
--   • aimer / ne pas aimer une vidéo (like/dislike binaire, une réaction par
--     utilisateur et par vidéo, bascule on/off — modèle iam.biblio_reaction) ;
--   • partager une vidéo dans le mur communautaire « /publications » avec une
--     légende facultative (source agrégée supplémentaire du mur — modèle
--     social.partage_profil / country_profile.partage_fiche).
--
-- Aucun compteur dénormalisé : les totaux sont calculés à la lecture.
-- Migration idempotente (rétrocompatible) : tables IF NOT EXISTS.
-- ════════════════════════════════════════════════════════════════════════════

-- ── Réactions like / dislike (une par utilisateur et par vidéo) ───────────────
CREATE TABLE IF NOT EXISTS media_content.video_reaction (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    video_id       UUID NOT NULL REFERENCES media_content.video(id) ON DELETE CASCADE,
    utilisateur_id UUID NOT NULL,                       -- [xref] iam.utilisateur
    type_reaction  VARCHAR(10) NOT NULL CHECK (type_reaction IN ('like', 'dislike')),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (video_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_video_reaction_video
    ON media_content.video_reaction (video_id);

-- ── Partages vers le mur communautaire (avec légende facultative) ─────────────
CREATE TABLE IF NOT EXISTS media_content.partage_video (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    video_id       UUID NOT NULL REFERENCES media_content.video(id) ON DELETE CASCADE,
    utilisateur_id UUID NOT NULL,                       -- [xref] iam.utilisateur
    legende        TEXT CHECK (legende IS NULL OR char_length(legende) <= 500),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ                          -- soft delete
);

CREATE INDEX IF NOT EXISTS idx_partage_video_actifs
    ON media_content.partage_video (created_at DESC)
    WHERE deleted_at IS NULL;
