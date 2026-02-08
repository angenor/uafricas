-- ════════════════════════════════════════════════════════════════════════════
-- Migration : Ajout de la table chaine_tv au schema media_content
-- A exécuter sur une base de données déjà initialisée
-- ════════════════════════════════════════════════════════════════════════════

-- Créer le type enum pour la catégorie de chaîne TV
CREATE TYPE media_content.categorie_chaine_tv AS ENUM (
    'generaliste', 'info', 'sport', 'culture',
    'divertissement', 'religieux', 'education', 'musique'
);

-- Créer la table chaine_tv
CREATE TABLE media_content.chaine_tv (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom                  VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT,
    stream_url           VARCHAR(500) NOT NULL,
    image_couverture_url VARCHAR(500),
    categorie            media_content.categorie_chaine_tv NOT NULL DEFAULT 'generaliste',
    pays_id              UUID,                       -- [xref] shared.pays
    langue               VARCHAR(80)  NOT NULL DEFAULT 'Français',
    est_en_direct        BOOLEAN      NOT NULL DEFAULT TRUE,
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'publie'
                         CHECK (etat IN ('brouillon','publie','suspendu','supprime')),
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_chaine_tv_categorie ON media_content.chaine_tv(categorie) WHERE deleted_at IS NULL;
CREATE INDEX idx_chaine_tv_pays      ON media_content.chaine_tv(pays_id)   WHERE deleted_at IS NULL;
CREATE INDEX idx_chaine_tv_etat      ON media_content.chaine_tv(etat)      WHERE deleted_at IS NULL;

-- Contraintes inter-schemas
ALTER TABLE media_content.chaine_tv
    ADD CONSTRAINT fk_chaine_tv_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE media_content.chaine_tv
    ADD CONSTRAINT fk_chaine_tv_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;
