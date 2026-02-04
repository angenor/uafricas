-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : innovation — Innovations, Projets, Africantives
-- ════════════════════════════════════════════════════════════════════════════


CREATE TYPE innovation.etat_contenu AS ENUM (
    'brouillon', 'publie', 'suspendu', 'supprime'
);

CREATE TYPE innovation.etat_projet AS ENUM (
    'soumis', 'en_revue', 'approuve', 'en_cours', 'termine', 'suspendu', 'rejete'
);


-- ── Innovation ──────────────────────────────────────────────────────────

CREATE TABLE innovation.innovation (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL,
    image_couverture_url VARCHAR(500),
    domaine_id           UUID,                       -- [xref] shared.domaine_secteur
    organisation_id      UUID,                       -- [xref] iam.organisation
    pays_id              UUID,                       -- [xref] shared.pays
    ville                VARCHAR(200),
    etat                 innovation.etat_contenu NOT NULL DEFAULT 'publie',
    nombre_vues          INT          NOT NULL DEFAULT 0,
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_innovation_pays    ON innovation.innovation(pays_id)    WHERE deleted_at IS NULL;
CREATE INDEX idx_innovation_domaine ON innovation.innovation(domaine_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_innovation_etat    ON innovation.innovation(etat)       WHERE deleted_at IS NULL;

ALTER TABLE innovation.innovation ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_innovation_fts ON innovation.innovation USING GIN(search_vector);


-- ── Médias d'innovation ─────────────────────────────────────────────────

CREATE TABLE innovation.innovation_media (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    innovation_id   UUID         NOT NULL REFERENCES innovation.innovation(id) ON DELETE CASCADE,
    media_url       VARCHAR(1000) NOT NULL,
    type_mime       VARCHAR(150),
    ordre           INT          NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Projet ──────────────────────────────────────────────────────────────

CREATE TABLE innovation.projet (
    id                           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                        VARCHAR(350) NOT NULL,
    slug                         VARCHAR(400) UNIQUE,
    -- Informations de l'organisation soumettant
    nom_organisation             VARCHAR(350),
    description_organisation     TEXT,
    site_web                     VARCHAR(500),
    pays_id                      UUID,                    -- [xref] shared.pays
    ville                        VARCHAR(200),
    contact_email                CITEXT,
    contact_telephone            VARCHAR(30),
    -- Finances & durée
    cout_total                   DECIMAL(15,2),
    devise                       VARCHAR(5) DEFAULT 'XOF',
    duree_mois                   INT,
    date_commencement_souhaitee  DATE,
    -- Présentation du projet
    description                  TEXT         NOT NULL,
    objectifs                    TEXT         NOT NULL,
    resultats_attendus           TEXT,                    -- liste à puces (markdown)
    activites_programmees        TEXT,
    echeanciers                  TEXT,
    contribution_autonomisation  TEXT,
    difficultes_risques          TEXT,
    -- Statut
    etat                         innovation.etat_projet NOT NULL DEFAULT 'soumis',
    cree_par                     UUID         NOT NULL,  -- [xref] iam.utilisateur
    traite_par                   UUID,                   -- [xref] iam.utilisateur (admin)
    created_at                   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at                   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at                   TIMESTAMPTZ
);

CREATE INDEX idx_projet_pays  ON innovation.projet(pays_id)  WHERE deleted_at IS NULL;
CREATE INDEX idx_projet_etat  ON innovation.projet(etat)     WHERE deleted_at IS NULL;

ALTER TABLE innovation.projet ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_projet_fts ON innovation.projet USING GIN(search_vector);


-- ── Documents complémentaires de projet ─────────────────────────────────

CREATE TABLE innovation.projet_document (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    projet_id   UUID         NOT NULL REFERENCES innovation.projet(id) ON DELETE CASCADE,
    nom         VARCHAR(300) NOT NULL,
    url         VARCHAR(1000) NOT NULL,
    type_mime   VARCHAR(150),
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Africantive (initiatives africaines) ────────────────────────────────

CREATE TABLE innovation.africantive (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL,
    image_couverture_url VARCHAR(500),
    domaine_id           UUID,                       -- [xref] shared.domaine_secteur
    pays_id              UUID,                       -- [xref] shared.pays
    ville                VARCHAR(200),
    etat                 innovation.etat_contenu NOT NULL DEFAULT 'publie',
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_africantive_pays    ON innovation.africantive(pays_id)    WHERE deleted_at IS NULL;
CREATE INDEX idx_africantive_domaine ON innovation.africantive(domaine_id) WHERE deleted_at IS NULL;
