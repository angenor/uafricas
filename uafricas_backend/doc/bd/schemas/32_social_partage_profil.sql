-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : social — Partage de profils sur le mur communautaire
-- ════════════════════════════════════════════════════════════════════════════
-- Dépend de : 29_social.sql, 04_iam.sql (iam.utilisateur)
--
-- Un membre connecté peut partager le profil d'un autre membre sur /publications,
-- avec une légende facultative. Calqué sur country_profile.partage_fiche.
-- Migration idempotente.


CREATE TABLE IF NOT EXISTS social.partage_profil (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- Profil partagé
    profil_id      UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    -- Auteur du partage
    utilisateur_id UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    legende        TEXT CHECK (legende IS NULL OR char_length(legende) <= 500),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ                          -- soft delete
);

CREATE INDEX IF NOT EXISTS idx_partage_profil_actifs
    ON social.partage_profil (created_at DESC)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_partage_profil_par_profil
    ON social.partage_profil (profil_id)
    WHERE deleted_at IS NULL;
