-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — governance : FactCheck — Signalements (modération communautaire)
-- ════════════════════════════════════════════════════════════════════════════
-- Idempotent. Chaque utilisateur ne peut signaler un factcheck qu'une seule fois.
-- Au-delà de 20 signalements distincts, le factcheck est suspendu automatiquement
-- (etat = 'suspendu') côté backend, ce qui le retire de la liste publique.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE governance.factcheck
    ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0;

CREATE TABLE IF NOT EXISTS governance.factcheck_signalement (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    factcheck_id    UUID        NOT NULL REFERENCES governance.factcheck(id) ON DELETE CASCADE,
    utilisateur_id  UUID        NOT NULL,            -- [xref] iam.utilisateur
    motif           VARCHAR(50),                     -- catégorie facultative
    commentaire     TEXT,                            -- précision facultative
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (factcheck_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_factcheck_signalement_fc
    ON governance.factcheck_signalement(factcheck_id);
