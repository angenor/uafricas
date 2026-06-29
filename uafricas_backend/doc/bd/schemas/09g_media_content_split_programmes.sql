-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : séparation radio / télé jusqu'aux tables
-- ════════════════════════════════════════════════════════════════════════════
-- La table partagée `programme_radio_tele` (colonne `type` radio/tele) est scindée
-- en DEUX tables indépendantes, pour que radio et télé évoluent séparément :
--
--   • media_content.programme_radio — émissions RADIO : `audio_url` (fichier/lien),
--     rattachées à une station via `station_id`, une « à la une » par station.
--   • media_content.programme_tele  — programmes TÉLÉ : `video_url` (fichier/lien),
--     rattachés à une chaîne via `chaine_id`, une « à la une » par chaîne.
--
-- Migration idempotente. Placée APRÈS 09/09d/13 : sur une init fraîche, l'ancienne
-- table est créée puis migrée/supprimée ici (0 ligne) ; sur une base existante,
-- les lignes sont recopiées par `type` avant suppression.
-- ════════════════════════════════════════════════════════════════════════════

-- ── Programmes RADIO ───────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS media_content.programme_radio (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom_emission         VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT NOT NULL DEFAULT '',
    image_couverture_url VARCHAR(500),
    audio_url            VARCHAR(500),                 -- fichier (/uploads/...) ou lien
    info_animateur       TEXT,
    info_producteur      TEXT,
    pays_id              UUID,                          -- [xref] shared.pays
    est_international     BOOLEAN NOT NULL DEFAULT FALSE,
    langue               VARCHAR(80) NOT NULL DEFAULT 'Français',
    categorie_radio      media_content.categorie_radio,
    station_id           UUID,                          -- [xref] station_radio (regroupement)
    a_la_une             BOOLEAN NOT NULL DEFAULT FALSE,
    etat                 VARCHAR(50) NOT NULL DEFAULT 'brouillon'
                         CHECK (etat IN ('brouillon','publie','suspendu','supprime')),
    cree_par             UUID NOT NULL,                 -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_programme_radio_station
    ON media_content.programme_radio(station_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_programme_radio_categorie
    ON media_content.programme_radio(categorie_radio);
CREATE INDEX IF NOT EXISTS idx_programme_radio_etat
    ON media_content.programme_radio(etat) WHERE deleted_at IS NULL;
-- Une seule émission « à la une » par station
CREATE UNIQUE INDEX IF NOT EXISTS uq_programme_radio_a_la_une_par_station
    ON media_content.programme_radio(station_id)
    WHERE a_la_une = TRUE AND deleted_at IS NULL;

-- ── Programmes TÉLÉ ────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS media_content.programme_tele (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom_emission         VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT NOT NULL DEFAULT '',
    image_couverture_url VARCHAR(500),
    video_url            VARCHAR(500),                 -- fichier (/uploads/...) ou lien
    info_animateur       TEXT,
    info_producteur      TEXT,
    pays_id              UUID,                          -- [xref] shared.pays
    est_international     BOOLEAN NOT NULL DEFAULT FALSE,
    langue               VARCHAR(80) NOT NULL DEFAULT 'Français',
    chaine_id            UUID,                          -- [xref] chaine_tv
    a_la_une             BOOLEAN NOT NULL DEFAULT FALSE,
    etat                 VARCHAR(50) NOT NULL DEFAULT 'brouillon'
                         CHECK (etat IN ('brouillon','publie','suspendu','supprime')),
    cree_par             UUID NOT NULL,                 -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ,
    -- Vidéo obligatoire pour un programme télé publié
    CONSTRAINT chk_video_tele CHECK (etat != 'publie' OR video_url IS NOT NULL)
);

CREATE INDEX IF NOT EXISTS idx_programme_tele_chaine
    ON media_content.programme_tele(chaine_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_programme_tele_etat
    ON media_content.programme_tele(etat) WHERE deleted_at IS NULL;
-- Une seule émission « à la une » par chaîne
CREATE UNIQUE INDEX IF NOT EXISTS uq_programme_tele_a_la_une_par_chaine
    ON media_content.programme_tele(chaine_id)
    WHERE a_la_une = TRUE AND deleted_at IS NULL;

-- ── Clés étrangères ────────────────────────────────────────────────────────
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_programme_radio_pays') THEN
        ALTER TABLE media_content.programme_radio
            ADD CONSTRAINT fk_programme_radio_pays FOREIGN KEY (pays_id)
            REFERENCES shared.pays(id) ON DELETE SET NULL;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_programme_radio_cree_par') THEN
        ALTER TABLE media_content.programme_radio
            ADD CONSTRAINT fk_programme_radio_cree_par FOREIGN KEY (cree_par)
            REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_programme_radio_station') THEN
        ALTER TABLE media_content.programme_radio
            ADD CONSTRAINT fk_programme_radio_station FOREIGN KEY (station_id)
            REFERENCES media_content.station_radio(id) ON DELETE SET NULL;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_programme_tele_pays') THEN
        ALTER TABLE media_content.programme_tele
            ADD CONSTRAINT fk_programme_tele_pays FOREIGN KEY (pays_id)
            REFERENCES shared.pays(id) ON DELETE SET NULL;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_programme_tele_cree_par') THEN
        ALTER TABLE media_content.programme_tele
            ADD CONSTRAINT fk_programme_tele_cree_par FOREIGN KEY (cree_par)
            REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_programme_tele_chaine') THEN
        ALTER TABLE media_content.programme_tele
            ADD CONSTRAINT fk_programme_tele_chaine FOREIGN KEY (chaine_id)
            REFERENCES media_content.chaine_tv(id) ON DELETE SET NULL;
    END IF;
END $$;

-- ── Reprise des données existantes puis suppression de l'ancienne table ─────
DO $$
BEGIN
    IF to_regclass('media_content.programme_radio_tele') IS NOT NULL THEN
        INSERT INTO media_content.programme_tele
            (id, nom_emission, slug, description, image_couverture_url, video_url,
             info_animateur, info_producteur, pays_id, est_international, langue,
             chaine_id, a_la_une, etat, cree_par, created_at, updated_at, deleted_at)
        SELECT id, nom_emission, slug, description, image_couverture_url, video_url,
               info_animateur, info_producteur, pays_id, est_international, langue,
               chaine_id, a_la_une, etat, cree_par, created_at, updated_at, deleted_at
        FROM media_content.programme_radio_tele
        WHERE type = 'tele'
        ON CONFLICT (id) DO NOTHING;

        INSERT INTO media_content.programme_radio
            (id, nom_emission, slug, description, image_couverture_url,
             info_animateur, info_producteur, pays_id, est_international, langue,
             categorie_radio, a_la_une, etat, cree_par, created_at, updated_at, deleted_at)
        SELECT id, nom_emission, slug, description, image_couverture_url,
               info_animateur, info_producteur, pays_id, est_international, langue,
               categorie_radio, a_la_une, etat, cree_par, created_at, updated_at, deleted_at
        FROM media_content.programme_radio_tele
        WHERE type = 'radio'
        ON CONFLICT (id) DO NOTHING;

        DROP TABLE media_content.programme_radio_tele CASCADE;
    END IF;
END $$;
