-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : country_profile -- Contributions aux fiches pays
-- ════════════════════════════════════════════════════════════════════════════
--
-- Systeme de contributions collaboratives : les utilisateurs proposent des
-- modifications, ajouts ou suppressions sur les fiches pays. Un admin valide
-- ou rejette chaque proposition. Les contributions approuvees sont appliquees
-- directement sur les donnees de la fiche.
-- ════════════════════════════════════════════════════════════════════════════


-- ── Enums ─────────────────────────────────────────────────────────────────

CREATE TYPE country_profile.etat_contribution AS ENUM (
    'en_attente',
    'approuvee',
    'rejetee'
);

CREATE TYPE country_profile.type_contribution AS ENUM (
    'modification',
    'ajout',
    'suppression'
);


-- ── Table principale des contributions ────────────────────────────────────

CREATE TABLE country_profile.contribution_fiche (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    cree_par            UUID         NOT NULL,  -- [xref] iam.utilisateur

    -- Ce qui est modifie
    section             VARCHAR(100) NOT NULL,  -- champ cible : population, biographie, contexte, slogan, groupe_ethnique, etc.
    type_contribution   country_profile.type_contribution NOT NULL DEFAULT 'modification',

    -- Valeurs
    ancienne_valeur     TEXT,                   -- valeur actuelle au moment de la proposition (NULL pour 'ajout')
    nouvelle_valeur     TEXT         NOT NULL,   -- valeur proposee (pour 'suppression': justification)
    justification       TEXT,                    -- explication optionnelle du contributeur

    -- Moderation
    etat                country_profile.etat_contribution NOT NULL DEFAULT 'en_attente',
    traite_par          UUID,                   -- [xref] iam.utilisateur (admin)
    note_moderation     TEXT,                   -- commentaire de l'admin
    traite_at           TIMESTAMPTZ,

    -- Timestamps
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

-- Index partiels (soft delete)
CREATE INDEX idx_contrib_fiche_fiche   ON country_profile.contribution_fiche(fiche_pays_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_contrib_fiche_etat    ON country_profile.contribution_fiche(etat)          WHERE deleted_at IS NULL;
CREATE INDEX idx_contrib_fiche_cree    ON country_profile.contribution_fiche(cree_par)      WHERE deleted_at IS NULL;
