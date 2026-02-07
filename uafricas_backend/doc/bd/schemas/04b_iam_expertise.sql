-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : iam — Expertise (profils experts)
-- ════════════════════════════════════════════════════════════════════════════
-- Dépend de : 04_iam.sql (iam.utilisateur)


-- ── Types ────────────────────────────────────────────────────────────────

CREATE TYPE iam.statut_expertise AS ENUM (
    'en_attente', 'valide', 'refuse'
);

CREATE TYPE iam.domaine_expertise AS ENUM (
    'agriculture', 'informatique', 'electronique', 'immobilier',
    'mecanique', 'sante', 'education', 'finance'
);

CREATE TYPE iam.situation_professionnelle AS ENUM (
    'recherche_emploi', 'en_emploi', 'consultance',
    'volontariat_expertise', 'recherche_nouvelles_opportunites'
);


-- ── Table expertise (1-to-1 avec utilisateur) ────────────────────────────

CREATE TABLE iam.expertise (
    id                          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id              UUID         NOT NULL UNIQUE
                                REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    domaine                     iam.domaine_expertise NOT NULL,
    biographie                  TEXT         NOT NULL,
    nb_annees_experience        INT          NOT NULL CHECK (nb_annees_experience >= 0),
    rating                      NUMERIC(2,1) NOT NULL DEFAULT 0.0
                                CHECK (rating >= 0 AND rating <= 5),
    portfolio                   VARCHAR(500),
    situations_professionnelles iam.situation_professionnelle[] NOT NULL DEFAULT '{}',
    statut                      iam.statut_expertise NOT NULL DEFAULT 'en_attente',
    valide_par                  UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    date_validation             TIMESTAMPTZ,
    created_at                  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at                  TIMESTAMPTZ
);


-- ── Indexes ──────────────────────────────────────────────────────────────

CREATE INDEX idx_expertise_statut
    ON iam.expertise(statut)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_expertise_domaine
    ON iam.expertise(domaine)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_expertise_utilisateur
    ON iam.expertise(utilisateur_id)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_expertise_rating
    ON iam.expertise(rating DESC)
    WHERE deleted_at IS NULL AND statut = 'valide';


-- ── Full-text search ─────────────────────────────────────────────────────

ALTER TABLE iam.expertise ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_expertise_fts ON iam.expertise USING GIN(search_vector);
