-- Migration : Ajout de la table station_radio au schema media_content
-- À exécuter sur une base de données déjà initialisée

-- Créer le type enum pour le type de station
CREATE TYPE media_content.type_station AS ENUM ('nationale', 'locale', 'internationale');

-- Créer la table station_radio
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
