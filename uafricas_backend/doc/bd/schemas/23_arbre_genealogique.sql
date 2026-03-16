-- ════════════════════════════════════════════════════════════════════════════
-- SCHEMA : arbre_genealogique
-- Bounded context : Arbre généalogique et liens familiaux
-- Créé : 2026-03-15
-- ────────────────────────────────────────────────────────────────────────────
-- Dépendances cross-schema : iam.utilisateur (FK dans 13_contraintes_inter_schemas.sql)
-- ────────────────────────────────────────────────────────────────────────────
-- Architecture clé : séparation Personne réelle / Rattachement
--   - personnes   : entité partageable (plusieurs arbres peuvent référencer la même)
--   - arbres       : conteneur logique propre à un utilisateur (1 par utilisateur)
--   - rattachements : lie une Personne réelle à un Arbre (le "point de vue" d'un utilisateur)
--   - liens_familiaux : relation typée entre deux Rattachements du même Arbre
--
-- Fondation du futur matching inter-arbres :
--   WITH personnes_communes AS (
--       SELECT personne_id FROM arbre_genealogique.rattachements
--       WHERE arbre_id IN ($arbre_a_id, $arbre_b_id) AND deleted_at IS NULL
--       GROUP BY personne_id HAVING COUNT(DISTINCT arbre_id) = 2
--   )
--   SELECT p.* FROM arbre_genealogique.personnes p
--   JOIN personnes_communes pc ON pc.personne_id = p.id;
-- ════════════════════════════════════════════════════════════════════════════

CREATE SCHEMA IF NOT EXISTS arbre_genealogique;

-- ────────────────────────────────────────────────────────────────────────────
-- TABLE : personnes
-- Représente une personne réelle, indépendante de tout arbre.
-- Les dates sont stockées en composantes séparées (annee/mois/jour)
-- pour permettre une saisie à granularité variable (ex : année seule = 1850).
-- La validation de cohérence (deces >= naissance) est appliquée applicativement.
-- ────────────────────────────────────────────────────────────────────────────
CREATE TABLE arbre_genealogique.personnes (
    id                  UUID            NOT NULL DEFAULT uuid_generate_v4(),
    nom                 VARCHAR(255)    NOT NULL,
    prenoms             VARCHAR(500),
    genre               VARCHAR(20)     CHECK (genre IN ('masculin', 'feminin', 'autre', 'non_precise')),
    -- Date de naissance (granularité variable : année seule, mois+année, ou complète)
    naissance_annee     SMALLINT        CHECK (naissance_annee BETWEEN 1 AND 9999),
    naissance_mois      SMALLINT        CHECK (naissance_mois BETWEEN 1 AND 12),
    naissance_jour      SMALLINT        CHECK (naissance_jour BETWEEN 1 AND 31),
    naissance_lieu      VARCHAR(500),
    -- Date de décès (mêmes règles de granularité)
    deces_annee         SMALLINT        CHECK (deces_annee BETWEEN 1 AND 9999),
    deces_mois          SMALLINT        CHECK (deces_mois BETWEEN 1 AND 12),
    deces_jour          SMALLINT        CHECK (deces_jour BETWEEN 1 AND 31),
    deces_lieu          VARCHAR(500),
    photo_url           VARCHAR(1000),
    cree_par            UUID,           -- [xref] iam.utilisateur (FK dans contraintes_inter_schemas)
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,

    CONSTRAINT pk_personnes PRIMARY KEY (id),
    -- Contrainte de cohérence basique sur les années (validation fine = applicative)
    CONSTRAINT chk_coherence_dates_annees
        CHECK (naissance_annee IS NULL OR deces_annee IS NULL OR deces_annee >= naissance_annee)
);

CREATE INDEX idx_personnes_nom
    ON arbre_genealogique.personnes (nom)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_personnes_cree_par
    ON arbre_genealogique.personnes (cree_par)
    WHERE deleted_at IS NULL;

-- ────────────────────────────────────────────────────────────────────────────
-- TABLE : arbres
-- Conteneur logique propre à un utilisateur.
-- Créé automatiquement lors du premier ajout de personne.
-- UNIQUE sur utilisateur_id : 1 arbre par utilisateur maximum.
-- ────────────────────────────────────────────────────────────────────────────
CREATE TABLE arbre_genealogique.arbres (
    id                  UUID            NOT NULL DEFAULT uuid_generate_v4(),
    utilisateur_id      UUID            NOT NULL, -- [xref] iam.utilisateur (UNIQUE : 1 arbre / utilisateur)
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,

    CONSTRAINT pk_arbres PRIMARY KEY (id),
    CONSTRAINT uq_arbres_utilisateur UNIQUE (utilisateur_id)
);

CREATE INDEX idx_arbres_utilisateur
    ON arbre_genealogique.arbres (utilisateur_id)
    WHERE deleted_at IS NULL;

-- ────────────────────────────────────────────────────────────────────────────
-- TABLE : rattachements
-- Lie une Personne réelle à un Arbre spécifique.
-- UNIQUE (arbre_id, personne_id) : une personne ne peut apparaître qu'une
-- seule fois par arbre.
-- Suppression : soft delete appliqué par le handler Rust (avec cascade
-- sur la personne si c'est le dernier rattachement).
-- ────────────────────────────────────────────────────────────────────────────
CREATE TABLE arbre_genealogique.rattachements (
    id                  UUID            NOT NULL DEFAULT uuid_generate_v4(),
    arbre_id            UUID            NOT NULL REFERENCES arbre_genealogique.arbres (id) ON DELETE CASCADE,
    personne_id         UUID            NOT NULL REFERENCES arbre_genealogique.personnes (id) ON DELETE CASCADE,
    ajoute_le           TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,

    CONSTRAINT pk_rattachements PRIMARY KEY (id),
    CONSTRAINT uq_rattachement_arbre_personne UNIQUE (arbre_id, personne_id)
);

CREATE INDEX idx_rattachements_arbre
    ON arbre_genealogique.rattachements (arbre_id)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_rattachements_personne
    ON arbre_genealogique.rattachements (personne_id)
    WHERE deleted_at IS NULL;

-- ────────────────────────────────────────────────────────────────────────────
-- TABLE : liens_familiaux
-- Relation typée entre deux Rattachements du même Arbre.
-- Types parent-enfant : 'pere', 'mere', 'parent' (non précisé)
--   → rattachement_source_id = le parent
--   → rattachement_cible_id  = l'enfant
-- Type conjoint : 'conjoint' (symétrique)
--   → Convention : source < cible (UUID string) pour garantir l'unicité.
-- Détection de cycle : vérifiée applicativement (CTE récursive) avant INSERT.
-- ────────────────────────────────────────────────────────────────────────────
CREATE TABLE arbre_genealogique.liens_familiaux (
    id                      UUID        NOT NULL DEFAULT uuid_generate_v4(),
    arbre_id                UUID        NOT NULL REFERENCES arbre_genealogique.arbres (id) ON DELETE CASCADE,
    rattachement_source_id  UUID        NOT NULL REFERENCES arbre_genealogique.rattachements (id) ON DELETE CASCADE,
    rattachement_cible_id   UUID        NOT NULL REFERENCES arbre_genealogique.rattachements (id) ON DELETE CASCADE,
    type_lien               VARCHAR(20) NOT NULL CHECK (type_lien IN ('pere', 'mere', 'parent', 'conjoint')),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ,

    CONSTRAINT pk_liens_familiaux PRIMARY KEY (id),
    CONSTRAINT uq_lien_familial UNIQUE (arbre_id, rattachement_source_id, rattachement_cible_id, type_lien),
    CONSTRAINT chk_lien_non_reflexif CHECK (rattachement_source_id <> rattachement_cible_id)
);

CREATE INDEX idx_liens_arbre
    ON arbre_genealogique.liens_familiaux (arbre_id)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_liens_source
    ON arbre_genealogique.liens_familiaux (rattachement_source_id)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_liens_cible
    ON arbre_genealogique.liens_familiaux (rattachement_cible_id)
    WHERE deleted_at IS NULL;
