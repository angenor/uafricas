-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : culture -- Inscription aux programmations
-- Feature : centres-culturels-inscription-programmation (2026-06-21)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Permet à un utilisateur connecté de s'inscrire à une programmation de centre
-- culturel (remplace l'ancien bouton « Je suis intéressé » mock).
-- Pattern calqué sur media_content.evenement_inscription. Idempotent.
--
-- Conformité : Principe III (SQL source de vérité).
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS culture.programmation_inscription (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    programmation_id    UUID NOT NULL REFERENCES culture.programmation_centre(id) ON DELETE CASCADE,
    utilisateur_id      UUID NOT NULL,                   -- [xref] iam.utilisateur
    statut              VARCHAR(30)  NOT NULL DEFAULT 'inscrit'
                        CHECK (statut IN ('inscrit','confirme','annule','present','absent')),
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    UNIQUE (programmation_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_prog_inscription_prog ON culture.programmation_inscription(programmation_id);
CREATE INDEX IF NOT EXISTS idx_prog_inscription_user ON culture.programmation_inscription(utilisateur_id);
