-- ============================================================================
-- 08i — afrolang : signalement communautaire d'une salle (depuis une session)
-- ----------------------------------------------------------------------------
-- Tout membre connecté présent dans une session live peut SIGNALER la salle
-- hôte (abus, contenu inapproprié, propos haineux…). Les signalements
-- s'accumulent sur la salle PERSISTANTE (pas la session éphémère), à travers
-- les sessions successives. Au-delà de 10 signalements distincts, la salle est
-- automatiquement SUSPENDUE : retirée du listing public et fermée aux nouvelles
-- jointures, jusqu'à réactivation par un administrateur. Jamais de désuspension
-- automatique.
--
-- Pattern calqué sur country_profile.signalement_contribution +
-- governance.factcheck_signalement (insert idempotent + recompte + bascule).
--
-- Migration idempotente.
-- ============================================================================

-- 1) Compteur dénormalisé + état de suspension sur la salle -------------------
ALTER TABLE afrolang.salle
    ADD COLUMN IF NOT EXISTS nombre_signalements INT     NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS suspendu            BOOLEAN NOT NULL DEFAULT FALSE;

-- 2) Table des signalements de salle -----------------------------------------
-- Clé (salle_id, signale_par) → un seul signalement par membre et par salle.
-- session_id : contexte (session où le signalement a été émis), facultatif.
CREATE TABLE IF NOT EXISTS afrolang.signalement_salle (
    id          UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id    UUID        NOT NULL REFERENCES afrolang.salle(id)   ON DELETE CASCADE,
    session_id  UUID        REFERENCES afrolang.session(id)          ON DELETE SET NULL,
    signale_par UUID        NOT NULL REFERENCES iam.utilisateur(id)  ON DELETE CASCADE,
    motif       VARCHAR(50),                     -- catégorie facultative
    description TEXT CHECK (description IS NULL OR char_length(description) <= 1000),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (salle_id, signale_par)               -- un signalement par membre et par salle
);

CREATE INDEX IF NOT EXISTS idx_signalement_salle_salle
    ON afrolang.signalement_salle (salle_id);
