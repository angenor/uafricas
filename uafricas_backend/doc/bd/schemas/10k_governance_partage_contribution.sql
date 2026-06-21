-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — governance : Partage de contributions vers le mur /publications
-- ════════════════════════════════════════════════════════════════════════════
-- Idempotent : exécutable sur une base déjà initialisée comme en init fraîche.
--
-- Un membre peut partager une contribution de gouvernance (factcheck, badhabits,
-- ideaforces) sur le mur communautaire /publications, avec une légende facultative
-- — calqué sur country_profile.partage_fiche et social.partage_profil.
--
-- Table polymorphe : `type_contribution` discrimine la table cible, `contribution_id`
-- pointe l'enregistrement. Pas de FK (cible polymorphe) ; l'existence et l'état
-- « publie » sont vérifiés à l'insertion et à la lecture.
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS governance.partage_contribution (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type_contribution VARCHAR(20) NOT NULL
                      CHECK (type_contribution IN ('factcheck', 'badhabits', 'ideaforces')),
    contribution_id   UUID NOT NULL,
    utilisateur_id    UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    legende           TEXT CHECK (legende IS NULL OR char_length(legende) <= 500),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at        TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_partage_contribution_actifs
    ON governance.partage_contribution (created_at DESC)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_partage_contribution_cible
    ON governance.partage_contribution (type_contribution, contribution_id)
    WHERE deleted_at IS NULL;
