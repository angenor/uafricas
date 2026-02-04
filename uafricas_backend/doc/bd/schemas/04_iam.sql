-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : iam — Identité & Accès
-- ════════════════════════════════════════════════════════════════════════════


-- ── Types ────────────────────────────────────────────────────────────────

CREATE TYPE iam.etat_utilisateur AS ENUM (
    'actif', 'en_attente', 'suspendu', 'bloque', 'supprime'
);

CREATE TYPE iam.genre AS ENUM (
    'homme', 'femme', 'autre', 'non_precise'
);


-- ── Utilisateur ─────────────────────────────────────────────────────────

CREATE TABLE iam.utilisateur (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom                     VARCHAR(100)            NOT NULL,
    prenom                  VARCHAR(100)            NOT NULL,
    email                   CITEXT                  NOT NULL UNIQUE,
    mot_de_passe_hash       VARCHAR(255)            NOT NULL,
    slug                    VARCHAR(220)            UNIQUE,
    telephone               VARCHAR(30),
    photo_url               VARCHAR(500),
    genre                   iam.genre               NOT NULL DEFAULT 'non_precise',
    date_naissance          DATE,
    fonction                VARCHAR(250),
    localite                VARCHAR(250),
    ville                   VARCHAR(200),
    pays_origine_id         UUID,                   -- [xref] shared.pays
    pays_residence_id       UUID,                   -- [xref] shared.pays
    organisation_id         UUID,                   -- ref iam.organisation (même schema)
    biographie              TEXT,
    etat                    iam.etat_utilisateur    NOT NULL DEFAULT 'en_attente',
    email_verifie           BOOLEAN                 NOT NULL DEFAULT FALSE,
    telephone_verifie       BOOLEAN                 NOT NULL DEFAULT FALSE,
    double_facteur_active   BOOLEAN                 NOT NULL DEFAULT FALSE,
    documents_verifie       BOOLEAN                 NOT NULL DEFAULT FALSE,
    bibliotheque_humain     BOOLEAN                 NOT NULL DEFAULT FALSE,
    langue_preferee         VARCHAR(10)             NOT NULL DEFAULT 'fr',
    derniere_connexion      TIMESTAMPTZ,
    created_at              TIMESTAMPTZ             NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ             NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_utilisateur_etat           ON iam.utilisateur(etat)           WHERE deleted_at IS NULL;
CREATE INDEX idx_utilisateur_pays_origine   ON iam.utilisateur(pays_origine_id)   WHERE deleted_at IS NULL;
CREATE INDEX idx_utilisateur_pays_residence ON iam.utilisateur(pays_residence_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_utilisateur_organisation   ON iam.utilisateur(organisation_id)   WHERE deleted_at IS NULL;
CREATE INDEX idx_utilisateur_biblio_humain  ON iam.utilisateur(bibliotheque_humain) WHERE bibliotheque_humain = TRUE;

-- Full-text search
ALTER TABLE iam.utilisateur ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_utilisateur_fts ON iam.utilisateur USING GIN(search_vector);


-- ── Spécialité Bibliothèque Humaine (lookup) ────────────────────────────

CREATE TABLE iam.specialite_bibliotheque (
    id   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom  VARCHAR(200) NOT NULL UNIQUE,
    slug VARCHAR(200) NOT NULL UNIQUE
);

CREATE TABLE iam.utilisateur_specialite (
    utilisateur_id UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    specialite_id  UUID NOT NULL REFERENCES iam.specialite_bibliotheque(id) ON DELETE CASCADE,
    PRIMARY KEY (utilisateur_id, specialite_id)
);


-- ── Rôle ─────────────────────────────────────────────────────────────────

CREATE TABLE iam.role (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom         VARCHAR(100) NOT NULL UNIQUE,
    slug        VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    est_systeme BOOLEAN      NOT NULL DEFAULT FALSE,  -- super_admin, admin…
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Permission ──────────────────────────────────────────────────────────

CREATE TABLE iam.permission (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom             VARCHAR(250) NOT NULL,
    slug            VARCHAR(250) NOT NULL UNIQUE,
    description     TEXT,
    type_ressource  VARCHAR(120),                    -- 'annonce', 'utilisateur', '*'
    action          VARCHAR(60)  NOT NULL,           -- 'voir', 'creer', 'modifier', 'supprimer', '*'
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Rôle ↔ Permission ──────────────────────────────────────────────────

CREATE TABLE iam.role_permission (
    role_id       UUID NOT NULL REFERENCES iam.role(id)       ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES iam.permission(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);


-- ── Utilisateur ↔ Rôle ─────────────────────────────────────────────────

CREATE TABLE iam.utilisateur_role (
    utilisateur_id UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    role_id        UUID NOT NULL REFERENCES iam.role(id)        ON DELETE CASCADE,
    attribue_par   UUID         REFERENCES iam.utilisateur(id),
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    PRIMARY KEY (utilisateur_id, role_id)
);


-- ── Permission spécifique (row-level) ───────────────────────────────────
--    Permet d'attribuer une permission sur un enregistrement précis
--    Ex : modifier l'annonce fjdsbn-efdj-wdn-wef-wdnj-3nj3-efdc

CREATE TABLE iam.permission_specifique (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    permission_id   UUID        NOT NULL REFERENCES iam.permission(id)  ON DELETE CASCADE,
    ressource_type  VARCHAR(120) NOT NULL,           -- ex : 'annonce'
    ressource_id    UUID        NOT NULL,            -- PK de la ressource ciblée
    attribue_par    UUID        REFERENCES iam.utilisateur(id),
    expire_at       TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (utilisateur_id, permission_id, ressource_type, ressource_id)
);

CREATE INDEX idx_perm_spec_ressource ON iam.permission_specifique(ressource_type, ressource_id);


-- ── Organisation ────────────────────────────────────────────────────────

CREATE TABLE iam.organisation (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    denomination        VARCHAR(350) NOT NULL,
    slug                VARCHAR(350) UNIQUE,
    type_organisation   VARCHAR(120),                -- ONG, Entreprise, Association, Coopérative…
    pays_id             UUID,                        -- [xref] shared.pays
    email               CITEXT,
    telephone           VARCHAR(30),
    adresse             TEXT,
    ville               VARCHAR(200),
    site_web            VARCHAR(500),
    logo_url            VARCHAR(500),
    description         TEXT,
    document_legal_url  VARCHAR(500),
    numero_registre     VARCHAR(150),
    etat                VARCHAR(60)  NOT NULL DEFAULT 'en_attente'
                        CHECK (etat IN ('actif','en_attente','suspendu','supprime')),
    cree_par            UUID,                        -- ref iam.utilisateur
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_organisation_pays ON iam.organisation(pays_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_organisation_slug ON iam.organisation(slug);
CREATE INDEX idx_organisation_etat ON iam.organisation(etat)   WHERE deleted_at IS NULL;

-- FK circulaire : utilisateur.organisation_id → organisation
ALTER TABLE iam.utilisateur
    ADD CONSTRAINT fk_utilisateur_organisation
    FOREIGN KEY (organisation_id) REFERENCES iam.organisation(id) ON DELETE SET NULL;

ALTER TABLE iam.organisation
    ADD CONSTRAINT fk_organisation_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;


-- ── Partenariat ─────────────────────────────────────────────────────────
--    Un partenaire EST une organisation (brouillon §7) ;
--    cette table matérialise la relation de partenariat.

CREATE TABLE iam.partenariat (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    organisation_id     UUID         NOT NULL REFERENCES iam.organisation(id) ON DELETE CASCADE,
    type_partenariat    VARCHAR(120),                -- Sponsor, Contributeur, Associé…
    description         TEXT,
    date_debut          DATE,
    date_fin            DATE,
    actif               BOOLEAN      NOT NULL DEFAULT TRUE,
    approuve_par        UUID,                        -- ref iam.utilisateur (admin)
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_partenariat_orga ON iam.partenariat(organisation_id);


-- ── Refresh Token (pour l'authentification JWT) ─────────────────────────

CREATE TABLE iam.refresh_token (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    token_hash      VARCHAR(255) NOT NULL UNIQUE,
    user_agent      TEXT,
    ip_address      INET,
    expire_at       TIMESTAMPTZ  NOT NULL,
    revoque         BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_refresh_token_user ON iam.refresh_token(utilisateur_id) WHERE revoque = FALSE;
