-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : governance — Gouvernance Citoyenne
-- ════════════════════════════════════════════════════════════════════════════


CREATE TYPE governance.niveau_gravite AS ENUM ('faible', 'elevee', 'critique');


-- ── FactCheck ───────────────────────────────────────────────────────────

CREATE TABLE governance.factcheck (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    contenu              TEXT         NOT NULL,
    pays_id              UUID,                       -- [xref] shared.pays
    image_couverture_url VARCHAR(500),
    couleur_fond         VARCHAR(7),
    source_originale     VARCHAR(500),               -- URL / référence de la source vérifiée
    verdict              VARCHAR(50)
                         CHECK (verdict IN ('vrai','faux','partiellement_vrai','trompeur','non_verifie')),
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'publie'
                         CHECK (etat IN ('brouillon','publie','suspendu','supprime')),
    nombre_likes         INT          NOT NULL DEFAULT 0,
    nombre_dislikes      INT          NOT NULL DEFAULT 0,
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_factcheck_pays    ON governance.factcheck(pays_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_factcheck_verdict ON governance.factcheck(verdict);

ALTER TABLE governance.factcheck ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_factcheck_fts ON governance.factcheck USING GIN(search_vector);


-- ── FactCheck — Commentaires (soutien vs contradiction) ─────────────────

CREATE TABLE governance.factcheck_commentaire (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    factcheck_id      UUID        NOT NULL REFERENCES governance.factcheck(id) ON DELETE CASCADE,
    parent_id         UUID        REFERENCES governance.factcheck_commentaire(id) ON DELETE CASCADE,
    contenu           TEXT        NOT NULL,
    type_commentaire  VARCHAR(20) NOT NULL CHECK (type_commentaire IN ('soutien', 'contradiction')),
    cree_par          UUID        NOT NULL,          -- [xref] iam.utilisateur
    nombre_likes      INT         NOT NULL DEFAULT 0,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at        TIMESTAMPTZ
);

CREATE INDEX idx_fc_comm_factcheck ON governance.factcheck_commentaire(factcheck_id);
CREATE INDEX idx_fc_comm_type      ON governance.factcheck_commentaire(type_commentaire);


-- ── FactCheck — Réactions ───────────────────────────────────────────────

CREATE TABLE governance.factcheck_reaction (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    factcheck_id    UUID        NOT NULL REFERENCES governance.factcheck(id) ON DELETE CASCADE,
    utilisateur_id  UUID        NOT NULL,            -- [xref] iam.utilisateur
    type_reaction   VARCHAR(10) NOT NULL CHECK (type_reaction IN ('like', 'dislike')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (factcheck_id, utilisateur_id)
);


-- ── BadHabits (Mauvaises pratiques) ─────────────────────────────────────

CREATE TABLE governance.bad_habit (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                   VARCHAR(350) NOT NULL,
    slug                    VARCHAR(400) UNIQUE,
    pays_id                 UUID         NOT NULL,   -- [xref] shared.pays
    region                  VARCHAR(250),
    ville_quartier_zone     VARCHAR(350),
    description_generale    TEXT         NOT NULL,
    details_problematique   TEXT         NOT NULL,
    categorie_probleme      VARCHAR(150) NOT NULL
                            CHECK (categorie_probleme IN (
                                'corruption',
                                'service_public_defaillant',
                                'infrastructure_degradee',
                                'acces_services_limite',
                                'insalubrite',
                                'probleme_securite',
                                'autre'
                            )),
    categorie_probleme_detail VARCHAR(200),           -- précision si "autre"
    gravite                 governance.niveau_gravite NOT NULL DEFAULT 'faible',
    preuves_temoignages     TEXT,
    solutions_proposees     TEXT,
    -- Options de publication
    publication_anonyme         BOOLEAN NOT NULL DEFAULT FALSE,
    geolocalisation_autorisee   BOOLEAN NOT NULL DEFAULT FALSE,
    longitude               DECIMAL(10,7),
    latitude                DECIMAL(10,7),
    -- Statut
    etat                    VARCHAR(50) NOT NULL DEFAULT 'en_attente'
                            CHECK (etat IN ('en_attente','publie','suspendu','supprime')),
    nombre_soutiens         INT         NOT NULL DEFAULT 0,
    cree_par                UUID        NOT NULL,    -- [xref] iam.utilisateur
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_bad_habit_pays      ON governance.bad_habit(pays_id)            WHERE deleted_at IS NULL;
CREATE INDEX idx_bad_habit_categorie ON governance.bad_habit(categorie_probleme) WHERE deleted_at IS NULL;
CREATE INDEX idx_bad_habit_gravite   ON governance.bad_habit(gravite)            WHERE deleted_at IS NULL;
CREATE INDEX idx_bad_habit_etat      ON governance.bad_habit(etat)              WHERE deleted_at IS NULL;


-- ── BadHabits — Médias (photos / vidéos de preuve) ──────────────────────

CREATE TABLE governance.bad_habit_media (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    bad_habit_id UUID         NOT NULL REFERENCES governance.bad_habit(id) ON DELETE CASCADE,
    media_url    VARCHAR(1000) NOT NULL,
    type_mime    VARCHAR(150),
    ordre        INT          NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── IdeaForces (Propositions positives) ─────────────────────────────────

CREATE TABLE governance.idea_force (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                   VARCHAR(350) NOT NULL,
    slug                    VARCHAR(400) UNIQUE,
    pays_id                 UUID         NOT NULL,   -- [xref] shared.pays
    region                  VARCHAR(250),
    ville_quartier_zone     VARCHAR(350),
    description_generale    TEXT         NOT NULL,
    details_proposition     TEXT         NOT NULL,
    categorie_proposition   VARCHAR(150) NOT NULL
                            CHECK (categorie_proposition IN (
                                'amelioration_gouvernance',
                                'education_formation',
                                'sante_publique',
                                'emploi_jeunes',
                                'environnement',
                                'transport',
                                'autre'
                            )),
    categorie_proposition_detail VARCHAR(200),        -- précision si "autre"
    urgence                 governance.niveau_gravite NOT NULL DEFAULT 'faible',
    plan_implementation     TEXT,
    ressources_necessaires  TEXT,
    impact_attendu          TEXT,
    -- Statut
    etat                    VARCHAR(50) NOT NULL DEFAULT 'en_attente'
                            CHECK (etat IN ('en_attente','publie','suspendu','supprime')),
    nombre_soutiens         INT         NOT NULL DEFAULT 0,
    cree_par                UUID        NOT NULL,    -- [xref] iam.utilisateur
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_idea_force_pays      ON governance.idea_force(pays_id)              WHERE deleted_at IS NULL;
CREATE INDEX idx_idea_force_categorie ON governance.idea_force(categorie_proposition) WHERE deleted_at IS NULL;
CREATE INDEX idx_idea_force_urgence   ON governance.idea_force(urgence)               WHERE deleted_at IS NULL;


-- ── IdeaForces — Médias ─────────────────────────────────────────────────

CREATE TABLE governance.idea_force_media (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    idea_force_id  UUID         NOT NULL REFERENCES governance.idea_force(id) ON DELETE CASCADE,
    media_url      VARCHAR(1000) NOT NULL,
    type_mime      VARCHAR(150),
    ordre          INT          NOT NULL DEFAULT 0,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
