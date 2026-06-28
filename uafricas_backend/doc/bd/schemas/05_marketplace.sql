-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : marketplace — Marché Africain
-- ════════════════════════════════════════════════════════════════════════════


CREATE TYPE marketplace.type_operation AS ENUM (
    'vente', 'troc', 'don', 'association', 'opportunite', 'opportunite_investissement'
);

CREATE TYPE marketplace.etat_annonce AS ENUM (
    'brouillon', 'publiee', 'en_attente', 'expiree', 'suspendue', 'supprimee', 'conclue'
);

CREATE TYPE marketplace.type_contact AS ENUM (
    'email', 'telephone', 'messagerie_plateforme'
);

CREATE TYPE marketplace.condition_article AS ENUM (
    'neuf', 'occasion', 'reconditionne', 'non_applicable'
);


-- ── Annonce ─────────────────────────────────────────────────────────────

CREATE TABLE marketplace.annonce (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre               VARCHAR(350) NOT NULL,
    slug                VARCHAR(400) UNIQUE,
    description         TEXT         NOT NULL,
    type_operation      marketplace.type_operation      NOT NULL,
    categorie_id        UUID,                            -- [xref] shared.categorie
    secteur_id          UUID,                            -- [xref] shared.domaine_secteur
    secteur_autre       VARCHAR(200),                    -- libellé libre si « Autre »
    condition_article   marketplace.condition_article    NOT NULL DEFAULT 'non_applicable',
    prix                DECIMAL(15,2),
    devise              VARCHAR(5)   DEFAULT 'XOF',
    prix_negociable     BOOLEAN      NOT NULL DEFAULT FALSE,
    ville               VARCHAR(200),
    adresse             TEXT,
    longitude           DECIMAL(10,7),
    latitude            DECIMAL(10,7),
    type_contact        marketplace.type_contact         NOT NULL DEFAULT 'messagerie_plateforme',
    contact_info        VARCHAR(300),
    quantite            INT          DEFAULT 1,
    etat                marketplace.etat_annonce         NOT NULL DEFAULT 'brouillon',
    nombre_vues         INT          NOT NULL DEFAULT 0,
    cree_par            UUID         NOT NULL,           -- [xref] iam.utilisateur
    expire_at           TIMESTAMPTZ,
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_annonce_etat        ON marketplace.annonce(etat)           WHERE deleted_at IS NULL;
CREATE INDEX idx_annonce_type_op     ON marketplace.annonce(type_operation)  WHERE deleted_at IS NULL;
CREATE INDEX idx_annonce_categorie   ON marketplace.annonce(categorie_id)    WHERE deleted_at IS NULL;
CREATE INDEX idx_annonce_secteur     ON marketplace.annonce(secteur_id)      WHERE deleted_at IS NULL;
CREATE INDEX idx_annonce_cree_par    ON marketplace.annonce(cree_par);
CREATE INDEX idx_annonce_expire      ON marketplace.annonce(expire_at)       WHERE etat = 'publiee';

ALTER TABLE marketplace.annonce ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_annonce_fts ON marketplace.annonce USING GIN(search_vector);


-- ── Annonce ↔ Pays (une annonce peut cibler plusieurs pays) ─────────────

CREATE TABLE marketplace.annonce_pays (
    annonce_id UUID NOT NULL REFERENCES marketplace.annonce(id) ON DELETE CASCADE,
    pays_id    UUID NOT NULL,                        -- [xref] shared.pays
    PRIMARY KEY (annonce_id, pays_id)
);


-- ── Médias d'annonce ────────────────────────────────────────────────────

CREATE TABLE marketplace.annonce_media (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    annonce_id      UUID         NOT NULL REFERENCES marketplace.annonce(id) ON DELETE CASCADE,
    media_url       VARCHAR(1000) NOT NULL,
    type_mime       VARCHAR(150),
    est_principale  BOOLEAN      NOT NULL DEFAULT FALSE,
    ordre           INT          NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_annonce_media_annonce ON marketplace.annonce_media(annonce_id);


-- ── Favoris d'annonce ───────────────────────────────────────────────────

CREATE TABLE marketplace.annonce_favori (
    utilisateur_id UUID NOT NULL,                    -- [xref] iam.utilisateur
    annonce_id     UUID NOT NULL REFERENCES marketplace.annonce(id) ON DELETE CASCADE,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (utilisateur_id, annonce_id)
);
