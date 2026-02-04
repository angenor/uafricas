-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : culture — Centres culturels, Afrolang, Codi-Moi
-- ════════════════════════════════════════════════════════════════════════════


CREATE TYPE culture.mode_evenement AS ENUM ('en_ligne', 'presentiel', 'hybride');

CREATE TYPE culture.type_codimoi AS ENUM (
    'proverbe_adage', 'citation', 'ressource_historique', 'bonne_pratique'
);


-- ── Centre Culturel Africain et Afro-Descendant (CCAD) ──────────────────

CREATE TABLE culture.centre_culturel (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom                  VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT,
    image_couverture_url VARCHAR(500),
    pays_id              UUID,                       -- [xref] shared.pays
    ville                VARCHAR(200),
    adresse              TEXT,
    longitude            DECIMAL(10,7),
    latitude             DECIMAL(10,7),
    actif                BOOLEAN      NOT NULL DEFAULT TRUE,
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_centre_culturel_pays ON culture.centre_culturel(pays_id);


-- ── Programmation d'un Centre Culturel ──────────────────────────────────

CREATE TABLE culture.programmation_centre (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    centre_culturel_id  UUID NOT NULL REFERENCES culture.centre_culturel(id) ON DELETE CASCADE,
    titre               VARCHAR(350) NOT NULL,
    description         TEXT,
    lieu                VARCHAR(350),
    mode                culture.mode_evenement NOT NULL DEFAULT 'presentiel',
    lien_en_ligne       VARCHAR(500),
    date_heure_debut    TIMESTAMPTZ  NOT NULL,
    date_heure_fin      TIMESTAMPTZ,
    nombre_places       INT,
    cree_par            UUID         NOT NULL,       -- [xref] iam.utilisateur
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_prog_centre_centre ON culture.programmation_centre(centre_culturel_id);
CREATE INDEX idx_prog_centre_date   ON culture.programmation_centre(date_heure_debut);


-- ── Afrolang — Salle publique (admin) ───────────────────────────────────

CREATE TABLE culture.afrolang_salle_publique (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT,
    image_couverture_url VARCHAR(500),
    langue_cible         VARCHAR(100),               -- langue africaine enseignée
    actif                BOOLEAN      NOT NULL DEFAULT TRUE,
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur (admin)
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Afrolang — Salle privée (tout utilisateur) ─────────────────────────

CREATE TABLE culture.afrolang_salle_privee (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_publique_id    UUID NOT NULL REFERENCES culture.afrolang_salle_publique(id) ON DELETE CASCADE,
    titre                VARCHAR(350) NOT NULL,
    description          TEXT,
    code_acces           VARCHAR(100),
    image_couverture_url VARCHAR(500),
    max_participants     INT          DEFAULT 50,
    actif                BOOLEAN      NOT NULL DEFAULT TRUE,
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_afrolang_privee_publique ON culture.afrolang_salle_privee(salle_publique_id);


-- ── Afrolang — Participants ─────────────────────────────────────────────

CREATE TABLE culture.afrolang_participant (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id UUID        NOT NULL REFERENCES culture.afrolang_salle_privee(id) ON DELETE CASCADE,
    utilisateur_id  UUID        NOT NULL,            -- [xref] iam.utilisateur
    role_salle      VARCHAR(30) NOT NULL DEFAULT 'participant'
                    CHECK (role_salle IN ('animateur', 'participant', 'observateur')),
    rejoint_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    quitte_at       TIMESTAMPTZ,
    UNIQUE (salle_privee_id, utilisateur_id)
);


-- ── Codi-Moi (publication sociale — tous types unifiés) ─────────────────

CREATE TABLE culture.codimoi (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type                    culture.type_codimoi NOT NULL,
    contenu                 TEXT         NOT NULL,
    explication             TEXT,                     -- proverbe & citation
    nom_auteur_originel     VARCHAR(250),             -- citation
    pays_id                 UUID,                     -- [xref] shared.pays
    groupe_ethnique         VARCHAR(250),
    couleur_fond            VARCHAR(7),               -- hex (#RRGGBB)
    image_couverture_url    VARCHAR(500),             -- ressource_historique & bonne_pratique
    image_arriere_plan_url  VARCHAR(500),             -- citation (optionnel)
    etat                    VARCHAR(50)  NOT NULL DEFAULT 'publie'
                            CHECK (etat IN ('publie','brouillon','suspendu','supprime')),
    nombre_likes            INT          NOT NULL DEFAULT 0,
    nombre_dislikes         INT          NOT NULL DEFAULT 0,
    cree_par                UUID         NOT NULL,   -- [xref] iam.utilisateur
    created_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_codimoi_type     ON culture.codimoi(type)     WHERE deleted_at IS NULL;
CREATE INDEX idx_codimoi_pays     ON culture.codimoi(pays_id)  WHERE deleted_at IS NULL;
CREATE INDEX idx_codimoi_cree_par ON culture.codimoi(cree_par);
CREATE INDEX idx_codimoi_etat     ON culture.codimoi(etat)     WHERE deleted_at IS NULL;

ALTER TABLE culture.codimoi ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_codimoi_fts ON culture.codimoi USING GIN(search_vector);


-- ── Codi-Moi ↔ Tags (hashtags) ─────────────────────────────────────────

CREATE TABLE culture.codimoi_tag (
    codimoi_id UUID NOT NULL REFERENCES culture.codimoi(id) ON DELETE CASCADE,
    tag_id     UUID NOT NULL,                        -- [xref] shared.tag
    PRIMARY KEY (codimoi_id, tag_id)
);


-- ── Codi-Moi — Commentaires (arborescent) ──────────────────────────────

CREATE TABLE culture.codimoi_commentaire (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    codimoi_id  UUID NOT NULL REFERENCES culture.codimoi(id)              ON DELETE CASCADE,
    parent_id   UUID          REFERENCES culture.codimoi_commentaire(id)  ON DELETE CASCADE,
    contenu     TEXT NOT NULL,
    cree_par    UUID NOT NULL,                       -- [xref] iam.utilisateur
    nombre_likes INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ
);

CREATE INDEX idx_codimoi_comm_post   ON culture.codimoi_commentaire(codimoi_id);
CREATE INDEX idx_codimoi_comm_parent ON culture.codimoi_commentaire(parent_id);


-- ── Codi-Moi — Réactions (like / dislike, 1 par user par post) ──────────

CREATE TABLE culture.codimoi_reaction (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    codimoi_id      UUID        NOT NULL REFERENCES culture.codimoi(id) ON DELETE CASCADE,
    utilisateur_id  UUID        NOT NULL,            -- [xref] iam.utilisateur
    type_reaction   VARCHAR(10) NOT NULL CHECK (type_reaction IN ('like', 'dislike')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (codimoi_id, utilisateur_id)
);

CREATE INDEX idx_codimoi_react_post ON culture.codimoi_reaction(codimoi_id);
