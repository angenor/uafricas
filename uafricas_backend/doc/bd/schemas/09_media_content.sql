-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : media_content — Radio/Télé, Livres, Événements, MOOC
-- ════════════════════════════════════════════════════════════════════════════


CREATE TYPE media_content.type_programme_media AS ENUM ('radio', 'tele');

CREATE TYPE media_content.categorie_radio AS ENUM (
    'radio_africans_international',
    'radio_africans_national',
    'radio_africans_local',
    'radio_nationale_national',
    'radio_nationale_local'
);

CREATE TYPE media_content.format_evenement AS ENUM ('presentiel', 'en_ligne', 'hybride');

CREATE TYPE media_content.acces_livre AS ENUM ('lecture_seule', 'lecture_telechargement');


CREATE TYPE media_content.type_station AS ENUM ('nationale', 'locale', 'internationale');

-- ── Station Radio ─────────────────────────────────────────────────────

CREATE TABLE media_content.station_radio (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom                  VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT,
    stream_url           VARCHAR(500) NOT NULL,
    image_couverture_url VARCHAR(500),
    genre                VARCHAR(350),
    genres_liste         TEXT[] NOT NULL DEFAULT '{}',
    pays_id              UUID,                       -- [xref] shared.pays
    ville                VARCHAR(200),
    type_station         media_content.type_station NOT NULL DEFAULT 'nationale',
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'publie'
                         CHECK (etat IN ('brouillon','publie','suspendu','supprime')),
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_station_radio_type ON media_content.station_radio(type_station) WHERE deleted_at IS NULL;
CREATE INDEX idx_station_radio_pays ON media_content.station_radio(pays_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_station_radio_etat ON media_content.station_radio(etat) WHERE deleted_at IS NULL;


-- ── Programme Radio / Télé ──────────────────────────────────────────────

CREATE TABLE media_content.programme_radio_tele (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom_emission         VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    type                 media_content.type_programme_media NOT NULL,
    description          TEXT         NOT NULL,
    image_couverture_url VARCHAR(500),
    video_url            VARCHAR(500),
    info_animateur       TEXT,
    info_producteur      TEXT,
    pays_id              UUID,                       -- [xref] shared.pays
    est_international    BOOLEAN      NOT NULL DEFAULT FALSE,
    langue               VARCHAR(80)  NOT NULL DEFAULT 'Français',
    categorie_radio      media_content.categorie_radio,
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'brouillon'
                         CHECK (etat IN ('brouillon','publie','suspendu','supprime')),
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ,
    -- Vidéo obligatoire pour une émission télé publiée
    CONSTRAINT chk_video_tele CHECK (
        type != 'tele' OR etat != 'publie' OR video_url IS NOT NULL
    )
);

CREATE INDEX idx_radio_tele_type ON media_content.programme_radio_tele(type)   WHERE deleted_at IS NULL;
CREATE INDEX idx_radio_tele_pays ON media_content.programme_radio_tele(pays_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_radio_tele_cat  ON media_content.programme_radio_tele(categorie_radio);


-- ── Événement Africans-World ────────────────────────────────────────────

CREATE TABLE media_content.evenement (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL,
    type                 VARCHAR(120),
    pays_id              UUID,                       -- [xref] shared.pays
    ville                VARCHAR(200),
    adresse              TEXT,
    date_heure_debut     TIMESTAMPTZ  NOT NULL,
    date_heure_fin       TIMESTAMPTZ,
    image_couverture_url VARCHAR(500),
    format               media_content.format_evenement NOT NULL DEFAULT 'presentiel',
    lien_en_ligne        VARCHAR(500),
    langue               VARCHAR(80)  NOT NULL DEFAULT 'Français',
    nombre_places        INT,
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'brouillon'
                         CHECK (etat IN ('brouillon','publie','annule','termine','suspendu')),
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_evenement_pays ON media_content.evenement(pays_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_evenement_date ON media_content.evenement(date_heure_debut);
CREATE INDEX idx_evenement_etat ON media_content.evenement(etat) WHERE deleted_at IS NULL;

ALTER TABLE media_content.evenement ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_evenement_fts ON media_content.evenement USING GIN(search_vector);


-- ── Inscription à un événement ──────────────────────────────────────────

CREATE TABLE media_content.evenement_inscription (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    evenement_id    UUID NOT NULL REFERENCES media_content.evenement(id) ON DELETE CASCADE,
    utilisateur_id  UUID NOT NULL,                   -- [xref] iam.utilisateur
    statut          VARCHAR(30)  NOT NULL DEFAULT 'inscrit'
                    CHECK (statut IN ('inscrit','confirme','annule','present','absent')),
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    UNIQUE (evenement_id, utilisateur_id)
);


-- ── CLOM / MOOC ─────────────────────────────────────────────────────────

CREATE TABLE media_content.mooc (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL,
    type                 VARCHAR(120),
    pays_id              UUID,                       -- [xref] shared.pays
    ville                VARCHAR(200),
    date_heure_debut     TIMESTAMPTZ  NOT NULL,
    date_heure_fin       TIMESTAMPTZ,
    image_couverture_url VARCHAR(500),
    format               media_content.format_evenement NOT NULL DEFAULT 'en_ligne',
    lien_en_ligne        VARCHAR(500),
    langue               VARCHAR(80)  NOT NULL DEFAULT 'Français',
    nombre_places        INT,
    prerequis            TEXT,
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'brouillon'
                         CHECK (etat IN ('brouillon','publie','en_cours','termine','annule','suspendu')),
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_mooc_date ON media_content.mooc(date_heure_debut);
CREATE INDEX idx_mooc_etat ON media_content.mooc(etat) WHERE deleted_at IS NULL;


-- ── Inscription à un MOOC ───────────────────────────────────────────────

CREATE TABLE media_content.mooc_inscription (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mooc_id         UUID NOT NULL REFERENCES media_content.mooc(id) ON DELETE CASCADE,
    utilisateur_id  UUID NOT NULL,                   -- [xref] iam.utilisateur
    progression     DECIMAL(5,2) DEFAULT 0.00,       -- % complété
    statut          VARCHAR(30)  NOT NULL DEFAULT 'inscrit'
                    CHECK (statut IN ('inscrit','en_cours','complete','abandonne')),
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    UNIQUE (mooc_id, utilisateur_id)
);


-- ── Livre (Bibliothèque numérique) ─────────────────────────────────────

CREATE TABLE media_content.livre (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                   VARCHAR(500) NOT NULL,
    slug                    VARCHAR(550) UNIQUE,
    description             TEXT         NOT NULL,
    image_couverture_url    VARCHAR(500),
    document_pdf_url        VARCHAR(500) NOT NULL,
    type_document           VARCHAR(120) NOT NULL,   -- Article de revue, Rapport, etc.
    categorie_id            UUID,                    -- [xref] shared.categorie
    acces                   media_content.acces_livre NOT NULL DEFAULT 'lecture_seule',
    info_auteur             TEXT         NOT NULL,
    date_publication        DATE,
    rapport_auteur          TEXT,                    -- rapport de l'auteur avec le document
    condition_diffusion     TEXT,
    acceptation_diffusion   BOOLEAN      NOT NULL DEFAULT FALSE,
    langue                  VARCHAR(80)  DEFAULT 'Français',
    nombre_pages            INT,
    isbn                    VARCHAR(25),
    nombre_telechargements  INT          NOT NULL DEFAULT 0,
    nombre_vues             INT          NOT NULL DEFAULT 0,
    etat                    VARCHAR(50)  NOT NULL DEFAULT 'brouillon'
                            CHECK (etat IN ('brouillon','publie','suspendu','supprime')),
    cree_par                UUID         NOT NULL,   -- [xref] iam.utilisateur
    created_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_livre_type_doc ON media_content.livre(type_document);
CREATE INDEX idx_livre_etat     ON media_content.livre(etat) WHERE deleted_at IS NULL;

ALTER TABLE media_content.livre ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_livre_fts ON media_content.livre USING GIN(search_vector);


-- ── Livre ↔ Tags ────────────────────────────────────────────────────────

CREATE TABLE media_content.livre_tag (
    livre_id UUID NOT NULL REFERENCES media_content.livre(id) ON DELETE CASCADE,
    tag_id   UUID NOT NULL,                          -- [xref] shared.tag
    PRIMARY KEY (livre_id, tag_id)
);
