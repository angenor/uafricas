-- ════════════════════════════════════════════════════════════════════════════
-- 24 — Matching inter-arbres et découvertes
-- ════════════════════════════════════════════════════════════════════════════

-- Extension pour la similarité trigram (fuzzy matching)
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- ─── Colonnes normalisées pour le matching ──────────────────────────────

ALTER TABLE arbre_genealogique.personnes
    ADD COLUMN IF NOT EXISTS nom_normalise VARCHAR(255),
    ADD COLUMN IF NOT EXISTS prenoms_normalise VARCHAR(500);

-- Index GIN trigram pour matching fuzzy rapide
CREATE INDEX IF NOT EXISTS idx_personnes_nom_trgm
    ON arbre_genealogique.personnes USING GIN (nom_normalise gin_trgm_ops)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_personnes_prenoms_trgm
    ON arbre_genealogique.personnes USING GIN (prenoms_normalise gin_trgm_ops)
    WHERE deleted_at IS NULL;

-- Backfill simplifié des noms normalisés (la normalisation phonétique complète se fait côté Rust)
UPDATE arbre_genealogique.personnes
SET nom_normalise = LOWER(nom),
    prenoms_normalise = LOWER(COALESCE(prenoms, ''))
WHERE nom_normalise IS NULL AND deleted_at IS NULL;

-- ─── Table : suggestions de correspondance ──────────────────────────────

CREATE TABLE IF NOT EXISTS arbre_genealogique.suggestions_correspondance (
    id                      UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    rattachement_a_id       UUID NOT NULL REFERENCES arbre_genealogique.rattachements(id),
    rattachement_b_id       UUID NOT NULL REFERENCES arbre_genealogique.rattachements(id),

    -- Score composite et sous-scores
    score                   REAL NOT NULL CHECK (score BETWEEN 0 AND 1),
    score_nom               REAL,
    score_prenoms           REAL,
    score_date              REAL,
    score_lieu              REAL,
    score_genre             REAL,

    -- Statut et confirmations
    statut                  VARCHAR(20) NOT NULL DEFAULT 'en_attente'
                            CHECK (statut IN ('en_attente', 'confirmee_a', 'confirmee_b', 'confirmee', 'rejetee_a', 'rejetee_b')),
    confirmee_par_a         BOOLEAN NOT NULL DEFAULT FALSE,
    confirmee_par_b         BOOLEAN NOT NULL DEFAULT FALSE,

    -- Timestamps
    detectee_le             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmee_le            TIMESTAMPTZ,
    deleted_at              TIMESTAMPTZ,

    -- Contraintes
    CONSTRAINT chk_suggestion_non_reflexive CHECK (rattachement_a_id != rattachement_b_id)
);

-- Index unique pour empêcher les doublons de paires (quelle que soit l'orientation)
CREATE UNIQUE INDEX IF NOT EXISTS idx_uq_suggestion_paire
    ON arbre_genealogique.suggestions_correspondance (
        LEAST(rattachement_a_id, rattachement_b_id),
        GREATEST(rattachement_a_id, rattachement_b_id)
    )
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_suggestions_rattachement_a
    ON arbre_genealogique.suggestions_correspondance (rattachement_a_id)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_suggestions_rattachement_b
    ON arbre_genealogique.suggestions_correspondance (rattachement_b_id)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_suggestions_statut
    ON arbre_genealogique.suggestions_correspondance (statut)
    WHERE deleted_at IS NULL;

-- ─── Table : demandes de contact ────────────────────────────────────────

CREATE TABLE IF NOT EXISTS arbre_genealogique.demandes_contact (
    id                      UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    suggestion_id           UUID NOT NULL REFERENCES arbre_genealogique.suggestions_correspondance(id),
    demandeur_id            UUID NOT NULL REFERENCES iam.utilisateur(id),
    destinataire_id         UUID NOT NULL REFERENCES iam.utilisateur(id),

    statut                  VARCHAR(20) NOT NULL DEFAULT 'en_attente'
                            CHECK (statut IN ('en_attente', 'acceptee', 'refusee')),

    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    traitee_le              TIMESTAMPTZ,

    CONSTRAINT chk_contact_non_reflexif CHECK (demandeur_id != destinataire_id)
);

CREATE INDEX IF NOT EXISTS idx_demandes_contact_destinataire
    ON arbre_genealogique.demandes_contact (destinataire_id);
