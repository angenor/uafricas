-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : shared — Référentiels partagés
-- ════════════════════════════════════════════════════════════════════════════


-- ── Pays ─────────────────────────────────────────────────────────────────

CREATE TABLE shared.pays (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom             VARCHAR(150) NOT NULL,
    code_iso2       CHAR(2)      UNIQUE,            -- ISO 3166-1 alpha-2
    code_iso3       CHAR(3)      UNIQUE,            -- ISO 3166-1 alpha-3
    indicatif_tel   VARCHAR(10),
    capitale        VARCHAR(150),
    continent       VARCHAR(50)  NOT NULL DEFAULT 'Afrique',
    longitude       DECIMAL(10,7),
    latitude        DECIMAL(10,7),
    actif           BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX uq_pays_nom ON shared.pays (LOWER(nom));


-- ── Domaine / Secteur ───────────────────────────────────────────────────

CREATE TABLE shared.domaine_secteur (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom         VARCHAR(200) NOT NULL,
    slug        VARCHAR(200) NOT NULL UNIQUE,
    description TEXT,
    icone       VARCHAR(100),                       -- nom d'icône ou classe CSS
    actif       BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Catégorie (hiérarchique, multi-contexte) ────────────────────────────

CREATE TABLE shared.categorie (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom         VARCHAR(200) NOT NULL,
    slug        VARCHAR(200) NOT NULL UNIQUE,
    parent_id   UUID         REFERENCES shared.categorie(id) ON DELETE SET NULL,
    contexte    VARCHAR(60)  NOT NULL,              -- 'annonce', 'livre', 'radio', etc.
    description TEXT,
    icone       VARCHAR(100),
    ordre       INT          NOT NULL DEFAULT 0,
    actif       BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_categorie_contexte ON shared.categorie(contexte);
CREATE INDEX idx_categorie_parent   ON shared.categorie(parent_id);


-- ── Tag (hashtags réutilisables) ────────────────────────────────────────

CREATE TABLE shared.tag (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom        VARCHAR(100) NOT NULL,
    slug       VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Média (fichiers uploadés — registre central) ────────────────────────

CREATE TABLE shared.media (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom_original    VARCHAR(500) NOT NULL,
    chemin_stockage VARCHAR(1000) NOT NULL,          -- chemin S3 / disque
    url_publique    VARCHAR(1000),
    type_mime       VARCHAR(150) NOT NULL,
    taille_octets   BIGINT,
    largeur         INT,                             -- pixels (images/vidéos)
    hauteur         INT,
    duree_secondes  INT,                             -- audio/vidéo
    uploaded_by     UUID,                            -- [xref] iam.utilisateur
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);
