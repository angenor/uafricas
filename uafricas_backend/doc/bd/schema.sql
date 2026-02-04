-- ============================================================================
-- PLATEFORME AFRICANS-WORLD
-- Schéma de Base de Données PostgreSQL
-- Architecture : Monolith-First, Microservice-Ready
-- ============================================================================
--
-- PRINCIPE D'ARCHITECTURE
-- ───────────────────────
-- Chaque SCHEMA PostgreSQL représente un bounded context (futur microservice).
-- En phase monolithe, les FK inter-schemas garantissent l'intégrité.
-- Lors du découpage, ces FK (regroupées en §14) seront remplacées par des
-- appels API ; chaque schema deviendra une base indépendante.
--
-- SCHEMAS (BOUNDED CONTEXTS)
-- ──────────────────────────
--   shared          → Référentiels partagés (pays, domaines, catégories, tags, médias)
--   iam             → Identité & Accès (utilisateurs, rôles, permissions, organisations)
--   marketplace     → Marché Africain (annonces)
--   exchange        → Programmes d'échange
--   innovation      → Innovations, projets, africantives
--   culture         → Centres culturels, Afrolang, Codi-Moi
--   media_content   → Radio/Télé, livres, événements, MOOC
--   governance      → Gouvernance citoyenne (FactCheck, BadHabits, IdeaForces)
--   country_profile → Fiches pays
--
-- CONVENTIONS
-- ───────────
--   • PK        : UUID v4 générée côté base (uuid_generate_v4)
--   • Temps     : TIMESTAMPTZ partout (fuseaux horaires)
--   • Soft del. : colonne deleted_at nullable
--   • Audit     : created_at / updated_at sur toute table mutable
--   • Nommage   : snake_case, vocabulaire français aligné sur le domaine métier
--   • [xref]    : commentaire signalant une référence inter-schemas
--                  (→ deviendra un appel API lors du split microservice)
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- §1  EXTENSIONS
-- ════════════════════════════════════════════════════════════════════════════

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";   -- uuid_generate_v4()
CREATE EXTENSION IF NOT EXISTS "citext";      -- emails insensibles à la casse
CREATE EXTENSION IF NOT EXISTS "pg_trgm";     -- recherche par similarité / trigrammes
CREATE EXTENSION IF NOT EXISTS "unaccent";    -- recherche sans accents


-- ════════════════════════════════════════════════════════════════════════════
-- §2  CRÉATION DES SCHEMAS
-- ════════════════════════════════════════════════════════════════════════════

CREATE SCHEMA IF NOT EXISTS shared;
CREATE SCHEMA IF NOT EXISTS iam;
CREATE SCHEMA IF NOT EXISTS marketplace;
CREATE SCHEMA IF NOT EXISTS exchange;
CREATE SCHEMA IF NOT EXISTS innovation;
CREATE SCHEMA IF NOT EXISTS culture;
CREATE SCHEMA IF NOT EXISTS media_content;
CREATE SCHEMA IF NOT EXISTS governance;
CREATE SCHEMA IF NOT EXISTS country_profile;


-- ════════════════════════════════════════════════════════════════════════════
-- §3  FONCTION UTILITAIRE : updated_at automatique
-- ════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION shared.trigger_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


-- ════════════════════════════════════════════════════════════════════════════
-- §4  SCHEMA : shared — Référentiels partagés
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


-- ════════════════════════════════════════════════════════════════════════════
-- §5  SCHEMA : iam — Identité & Accès
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


-- ════════════════════════════════════════════════════════════════════════════
-- §6  SCHEMA : marketplace — Marché Africain
-- ════════════════════════════════════════════════════════════════════════════

CREATE TYPE marketplace.type_operation AS ENUM (
    'vente', 'troc', 'don', 'association', 'opportunite'
);

CREATE TYPE marketplace.etat_annonce AS ENUM (
    'brouillon', 'publiee', 'en_attente', 'expiree', 'suspendue', 'supprimee'
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


-- ════════════════════════════════════════════════════════════════════════════
-- §7  SCHEMA : exchange — Programmes d'échange
-- ════════════════════════════════════════════════════════════════════════════

CREATE TYPE exchange.etat_programme AS ENUM (
    'brouillon', 'en_attente_validation', 'publie', 'en_cours', 'termine', 'suspendu', 'annule'
);

CREATE TYPE exchange.duree_programme AS ENUM (
    '1_semaine', '2_semaines', '3_semaines', '6_semaines',
    '1_mois', '2_mois', '3_mois', '6_mois', '1_an'
);

CREATE TYPE exchange.etat_candidature AS ENUM (
    'soumise', 'en_revue', 'acceptee', 'refusee', 'retiree'
);


-- ── Programme d'échange ─────────────────────────────────────────────────

CREATE TABLE exchange.programme (
    id                          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                       VARCHAR(350) NOT NULL,
    slug                        VARCHAR(400) UNIQUE,
    description                 TEXT         NOT NULL,
    image_couverture_url        VARCHAR(500),
    document_legal_url          VARCHAR(500),
    pays_id                     UUID         NOT NULL,       -- [xref] shared.pays
    ville                       VARCHAR(200),
    adresse                     TEXT,
    -- Prise en charge
    prise_en_charge_billet      BOOLEAN      NOT NULL DEFAULT FALSE,
    prise_en_charge_hebergement BOOLEAN      NOT NULL DEFAULT FALSE,
    prise_en_charge_subsistance BOOLEAN      NOT NULL DEFAULT FALSE,
    prise_en_charge_details     TEXT,
    -- Durée & planning
    duree                       exchange.duree_programme NOT NULL,
    domaine_id                  UUID,                        -- [xref] shared.domaine_secteur
    date_debut                  DATE         NOT NULL,
    date_fin                    DATE,
    -- Capacité & pré-requis
    nombre_places               INT,
    prerequis                   TEXT,
    langues_requises            VARCHAR(250),
    -- Statut
    etat                        exchange.etat_programme NOT NULL DEFAULT 'brouillon',
    cree_par                    UUID         NOT NULL,       -- [xref] iam.utilisateur
    valide_par                  UUID,                        -- [xref] iam.utilisateur (admin)
    valide_at                   TIMESTAMPTZ,
    created_at                  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at                  TIMESTAMPTZ
);

CREATE INDEX idx_programme_pays     ON exchange.programme(pays_id)    WHERE deleted_at IS NULL;
CREATE INDEX idx_programme_etat     ON exchange.programme(etat)       WHERE deleted_at IS NULL;
CREATE INDEX idx_programme_domaine  ON exchange.programme(domaine_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_programme_cree_par ON exchange.programme(cree_par);
CREATE INDEX idx_programme_dates    ON exchange.programme(date_debut, date_fin);


-- ── Candidature à un programme ──────────────────────────────────────────

CREATE TABLE exchange.candidature (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    programme_id        UUID NOT NULL REFERENCES exchange.programme(id) ON DELETE CASCADE,
    candidat_id         UUID NOT NULL,               -- [xref] iam.utilisateur
    lettre_motivation   TEXT,
    cv_url              VARCHAR(500),
    statut              exchange.etat_candidature NOT NULL DEFAULT 'soumise',
    notes_internes      TEXT,
    traite_par          UUID,                        -- [xref] iam.utilisateur (admin)
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (programme_id, candidat_id)
);

CREATE INDEX idx_candidature_candidat   ON exchange.candidature(candidat_id);
CREATE INDEX idx_candidature_programme  ON exchange.candidature(programme_id);
CREATE INDEX idx_candidature_statut     ON exchange.candidature(statut);


-- ════════════════════════════════════════════════════════════════════════════
-- §8  SCHEMA : innovation — Innovations, Projets, Africantives
-- ════════════════════════════════════════════════════════════════════════════

CREATE TYPE innovation.etat_contenu AS ENUM (
    'brouillon', 'publie', 'suspendu', 'supprime'
);

CREATE TYPE innovation.etat_projet AS ENUM (
    'soumis', 'en_revue', 'approuve', 'en_cours', 'termine', 'suspendu', 'rejete'
);


-- ── Innovation ──────────────────────────────────────────────────────────

CREATE TABLE innovation.innovation (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL,
    image_couverture_url VARCHAR(500),
    domaine_id           UUID,                       -- [xref] shared.domaine_secteur
    organisation_id      UUID,                       -- [xref] iam.organisation
    pays_id              UUID,                       -- [xref] shared.pays
    ville                VARCHAR(200),
    etat                 innovation.etat_contenu NOT NULL DEFAULT 'publie',
    nombre_vues          INT          NOT NULL DEFAULT 0,
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_innovation_pays    ON innovation.innovation(pays_id)    WHERE deleted_at IS NULL;
CREATE INDEX idx_innovation_domaine ON innovation.innovation(domaine_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_innovation_etat    ON innovation.innovation(etat)       WHERE deleted_at IS NULL;

ALTER TABLE innovation.innovation ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_innovation_fts ON innovation.innovation USING GIN(search_vector);


-- ── Médias d'innovation ─────────────────────────────────────────────────

CREATE TABLE innovation.innovation_media (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    innovation_id   UUID         NOT NULL REFERENCES innovation.innovation(id) ON DELETE CASCADE,
    media_url       VARCHAR(1000) NOT NULL,
    type_mime       VARCHAR(150),
    ordre           INT          NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Projet ──────────────────────────────────────────────────────────────

CREATE TABLE innovation.projet (
    id                           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                        VARCHAR(350) NOT NULL,
    slug                         VARCHAR(400) UNIQUE,
    -- Informations de l'organisation soumettant
    nom_organisation             VARCHAR(350),
    description_organisation     TEXT,
    site_web                     VARCHAR(500),
    pays_id                      UUID,                    -- [xref] shared.pays
    ville                        VARCHAR(200),
    contact_email                CITEXT,
    contact_telephone            VARCHAR(30),
    -- Finances & durée
    cout_total                   DECIMAL(15,2),
    devise                       VARCHAR(5) DEFAULT 'XOF',
    duree_mois                   INT,
    date_commencement_souhaitee  DATE,
    -- Présentation du projet
    description                  TEXT         NOT NULL,
    objectifs                    TEXT         NOT NULL,
    resultats_attendus           TEXT,                    -- liste à puces (markdown)
    activites_programmees        TEXT,
    echeanciers                  TEXT,
    contribution_autonomisation  TEXT,
    difficultes_risques          TEXT,
    -- Statut
    etat                         innovation.etat_projet NOT NULL DEFAULT 'soumis',
    cree_par                     UUID         NOT NULL,  -- [xref] iam.utilisateur
    traite_par                   UUID,                   -- [xref] iam.utilisateur (admin)
    created_at                   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at                   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at                   TIMESTAMPTZ
);

CREATE INDEX idx_projet_pays  ON innovation.projet(pays_id)  WHERE deleted_at IS NULL;
CREATE INDEX idx_projet_etat  ON innovation.projet(etat)     WHERE deleted_at IS NULL;

ALTER TABLE innovation.projet ADD COLUMN search_vector TSVECTOR;
CREATE INDEX idx_projet_fts ON innovation.projet USING GIN(search_vector);


-- ── Documents complémentaires de projet ─────────────────────────────────

CREATE TABLE innovation.projet_document (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    projet_id   UUID         NOT NULL REFERENCES innovation.projet(id) ON DELETE CASCADE,
    nom         VARCHAR(300) NOT NULL,
    url         VARCHAR(1000) NOT NULL,
    type_mime   VARCHAR(150),
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Africantive (initiatives africaines) ────────────────────────────────

CREATE TABLE innovation.africantive (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL,
    image_couverture_url VARCHAR(500),
    domaine_id           UUID,                       -- [xref] shared.domaine_secteur
    pays_id              UUID,                       -- [xref] shared.pays
    ville                VARCHAR(200),
    etat                 innovation.etat_contenu NOT NULL DEFAULT 'publie',
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_africantive_pays    ON innovation.africantive(pays_id)    WHERE deleted_at IS NULL;
CREATE INDEX idx_africantive_domaine ON innovation.africantive(domaine_id) WHERE deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- §9  SCHEMA : culture — Centres culturels, Afrolang, Codi-Moi
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


-- ════════════════════════════════════════════════════════════════════════════
-- §10  SCHEMA : media_content — Radio/Télé, Livres, Événements, MOOC
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


-- ════════════════════════════════════════════════════════════════════════════
-- §11  SCHEMA : governance — Gouvernance Citoyenne
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


-- ════════════════════════════════════════════════════════════════════════════
-- §12  SCHEMA : country_profile — Fiches pays
-- ════════════════════════════════════════════════════════════════════════════

-- ── Fiche Pays ──────────────────────────────────────────────────────────

CREATE TABLE country_profile.fiche_pays (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pays_id              UUID         NOT NULL UNIQUE,   -- [xref] shared.pays (1-to-1)
    image_couverture_url VARCHAR(500),
    slogan               VARCHAR(500),
    superficie_km2       DECIMAL(12,2),
    population           BIGINT,
    biographie           TEXT,
    contexte             TEXT,
    contexte_historique   TEXT,
    image_drapeau_url    VARCHAR(500),
    image_embleme_url    VARCHAR(500),
    image_devise_url     VARCHAR(500),
    hymne_national       VARCHAR(300),
    langue_officielle    VARCHAR(250),
    langues_populaires   TEXT,                        -- liste séparée par virgules ou JSON
    monnaie              VARCHAR(120),
    fuseau_horaire       VARCHAR(60),
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Régions ─────────────────────────────────────────────────────────────

CREATE TABLE country_profile.region (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id  UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    nom            VARCHAR(250) NOT NULL,
    chef_lieu      VARCHAR(250),
    description    TEXT,
    population     BIGINT,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_region_fiche ON country_profile.region(fiche_pays_id);


-- ── Groupes Ethniques ───────────────────────────────────────────────────

CREATE TABLE country_profile.groupe_ethnique (
    id                          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id               UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    nom                         VARCHAR(250) NOT NULL,
    description                 TEXT,
    objets_culturels_distinctifs TEXT,
    population_estimee          VARCHAR(120),
    langues                     VARCHAR(350),
    region_id                   UUID         REFERENCES country_profile.region(id) ON DELETE SET NULL,
    created_at                  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_groupe_ethno_fiche ON country_profile.groupe_ethnique(fiche_pays_id);


-- ── Alliances Inter-ethniques ───────────────────────────────────────────

CREATE TABLE country_profile.alliance_interethnique (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    nom                 VARCHAR(350) NOT NULL,
    description         TEXT,
    groupes_impliques   TEXT,                        -- description textuelle des groupes
    signification       TEXT,                        -- origine / signification de l'alliance
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_alliance_fiche ON country_profile.alliance_interethnique(fiche_pays_id);


-- ── Contes & Histoires ──────────────────────────────────────────────────

CREATE TABLE country_profile.conte_histoire (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    titre               VARCHAR(350) NOT NULL,
    contenu             TEXT         NOT NULL,
    type                VARCHAR(60)  CHECK (type IN ('conte', 'histoire_drole', 'legende', 'mythe')),
    groupe_ethnique_id  UUID         REFERENCES country_profile.groupe_ethnique(id) ON DELETE SET NULL,
    image_url           VARCHAR(500),
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_conte_fiche ON country_profile.conte_histoire(fiche_pays_id);


-- ── Sites Touristiques ──────────────────────────────────────────────────

CREATE TABLE country_profile.site_touristique (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id   UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    nom             VARCHAR(350) NOT NULL,
    description     TEXT,
    image_url       VARCHAR(500),
    longitude       DECIMAL(10,7),
    latitude        DECIMAL(10,7),
    region_id       UUID         REFERENCES country_profile.region(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_site_touristique_fiche ON country_profile.site_touristique(fiche_pays_id);


-- ── Secteurs de Développement ───────────────────────────────────────────

CREATE TABLE country_profile.secteur_developpement (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id   UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    nom             VARCHAR(250) NOT NULL,
    description     TEXT,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_secteur_dev_fiche ON country_profile.secteur_developpement(fiche_pays_id);


-- ── Saisons ─────────────────────────────────────────────────────────────

CREATE TABLE country_profile.saison (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id   UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    nom             VARCHAR(120) NOT NULL,
    description     TEXT,
    mois_debut      INT          CHECK (mois_debut BETWEEN 1 AND 12),
    mois_fin        INT          CHECK (mois_fin   BETWEEN 1 AND 12),
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_saison_fiche ON country_profile.saison(fiche_pays_id);


-- ── Liens Inter-ethniques (entre pays / diasporas) ──────────────────────

CREATE TABLE country_profile.lien_interethnique (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID         NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    pays_lie_id         UUID,                        -- [xref] shared.pays (autre pays lié)
    description         TEXT         NOT NULL,
    type_lien           VARCHAR(100),                -- migration, parenté, commerce…
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_lien_interethno_fiche ON country_profile.lien_interethnique(fiche_pays_id);


-- ════════════════════════════════════════════════════════════════════════════
-- §13  SCHEMA : shared — Journal d'audit (transversal)
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE shared.audit_log (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID,                            -- [xref] iam.utilisateur (nullable = système)
    action          VARCHAR(60)  NOT NULL,            -- 'CREATE', 'UPDATE', 'DELETE', 'LOGIN'…
    schema_name     VARCHAR(60)  NOT NULL,
    table_name      VARCHAR(120) NOT NULL,
    record_id       UUID,
    ancien_etat     JSONB,                           -- snapshot avant modification
    nouvel_etat     JSONB,                           -- snapshot après modification
    ip_address      INET,
    user_agent      TEXT,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_utilisateur ON shared.audit_log(utilisateur_id);
CREATE INDEX idx_audit_table       ON shared.audit_log(schema_name, table_name);
CREATE INDEX idx_audit_record      ON shared.audit_log(record_id);
CREATE INDEX idx_audit_date        ON shared.audit_log(created_at);


-- ════════════════════════════════════════════════════════════════════════════
-- §14  CONTRAINTES INTER-SCHEMAS (FK cross-boundary)
-- ════════════════════════════════════════════════════════════════════════════
-- Ces contraintes seront SUPPRIMÉES lors du découpage en microservices.
-- Chaque service conservera l'UUID et résoudra la référence via API.

-- ── shared.media ────────────────────────────────────────────────────────
ALTER TABLE shared.media
    ADD CONSTRAINT fk_media_uploaded_by
    FOREIGN KEY (uploaded_by) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

-- ── iam → shared ────────────────────────────────────────────────────────
ALTER TABLE iam.utilisateur
    ADD CONSTRAINT fk_utilisateur_pays_origine
    FOREIGN KEY (pays_origine_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE iam.utilisateur
    ADD CONSTRAINT fk_utilisateur_pays_residence
    FOREIGN KEY (pays_residence_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE iam.organisation
    ADD CONSTRAINT fk_organisation_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

-- ── marketplace → shared & iam ──────────────────────────────────────────
ALTER TABLE marketplace.annonce
    ADD CONSTRAINT fk_annonce_categorie
    FOREIGN KEY (categorie_id) REFERENCES shared.categorie(id) ON DELETE SET NULL;

ALTER TABLE marketplace.annonce
    ADD CONSTRAINT fk_annonce_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE marketplace.annonce_pays
    ADD CONSTRAINT fk_annonce_pays_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE CASCADE;

ALTER TABLE marketplace.annonce_favori
    ADD CONSTRAINT fk_annonce_favori_utilisateur
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

-- ── exchange → shared & iam ─────────────────────────────────────────────
ALTER TABLE exchange.programme
    ADD CONSTRAINT fk_programme_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE RESTRICT;

ALTER TABLE exchange.programme
    ADD CONSTRAINT fk_programme_domaine
    FOREIGN KEY (domaine_id) REFERENCES shared.domaine_secteur(id) ON DELETE SET NULL;

ALTER TABLE exchange.programme
    ADD CONSTRAINT fk_programme_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE exchange.programme
    ADD CONSTRAINT fk_programme_valide_par
    FOREIGN KEY (valide_par) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

ALTER TABLE exchange.candidature
    ADD CONSTRAINT fk_candidature_candidat
    FOREIGN KEY (candidat_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE exchange.candidature
    ADD CONSTRAINT fk_candidature_traite_par
    FOREIGN KEY (traite_par) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

-- ── innovation → shared & iam ───────────────────────────────────────────
ALTER TABLE innovation.innovation
    ADD CONSTRAINT fk_innovation_domaine
    FOREIGN KEY (domaine_id) REFERENCES shared.domaine_secteur(id) ON DELETE SET NULL;

ALTER TABLE innovation.innovation
    ADD CONSTRAINT fk_innovation_organisation
    FOREIGN KEY (organisation_id) REFERENCES iam.organisation(id) ON DELETE SET NULL;

ALTER TABLE innovation.innovation
    ADD CONSTRAINT fk_innovation_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE innovation.innovation
    ADD CONSTRAINT fk_innovation_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE innovation.projet
    ADD CONSTRAINT fk_projet_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE innovation.projet
    ADD CONSTRAINT fk_projet_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE innovation.projet
    ADD CONSTRAINT fk_projet_traite_par
    FOREIGN KEY (traite_par) REFERENCES iam.utilisateur(id) ON DELETE SET NULL;

ALTER TABLE innovation.africantive
    ADD CONSTRAINT fk_africantive_domaine
    FOREIGN KEY (domaine_id) REFERENCES shared.domaine_secteur(id) ON DELETE SET NULL;

ALTER TABLE innovation.africantive
    ADD CONSTRAINT fk_africantive_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE innovation.africantive
    ADD CONSTRAINT fk_africantive_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

-- ── culture → shared & iam ──────────────────────────────────────────────
ALTER TABLE culture.centre_culturel
    ADD CONSTRAINT fk_centre_culturel_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE culture.centre_culturel
    ADD CONSTRAINT fk_centre_culturel_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.programmation_centre
    ADD CONSTRAINT fk_prog_centre_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.afrolang_salle_publique
    ADD CONSTRAINT fk_afrolang_pub_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.afrolang_salle_privee
    ADD CONSTRAINT fk_afrolang_priv_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.afrolang_participant
    ADD CONSTRAINT fk_afrolang_participant_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE culture.codimoi
    ADD CONSTRAINT fk_codimoi_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE culture.codimoi
    ADD CONSTRAINT fk_codimoi_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.codimoi_tag
    ADD CONSTRAINT fk_codimoi_tag_tag
    FOREIGN KEY (tag_id) REFERENCES shared.tag(id) ON DELETE CASCADE;

ALTER TABLE culture.codimoi_commentaire
    ADD CONSTRAINT fk_codimoi_comm_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE culture.codimoi_reaction
    ADD CONSTRAINT fk_codimoi_react_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

-- ── media_content → shared & iam ────────────────────────────────────────
ALTER TABLE media_content.programme_radio_tele
    ADD CONSTRAINT fk_radio_tele_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE media_content.programme_radio_tele
    ADD CONSTRAINT fk_radio_tele_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE media_content.evenement
    ADD CONSTRAINT fk_evenement_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE media_content.evenement
    ADD CONSTRAINT fk_evenement_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE media_content.evenement_inscription
    ADD CONSTRAINT fk_evt_inscription_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE media_content.mooc
    ADD CONSTRAINT fk_mooc_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE media_content.mooc
    ADD CONSTRAINT fk_mooc_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE media_content.mooc_inscription
    ADD CONSTRAINT fk_mooc_inscription_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE media_content.livre
    ADD CONSTRAINT fk_livre_categorie
    FOREIGN KEY (categorie_id) REFERENCES shared.categorie(id) ON DELETE SET NULL;

ALTER TABLE media_content.livre
    ADD CONSTRAINT fk_livre_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE media_content.livre_tag
    ADD CONSTRAINT fk_livre_tag_tag
    FOREIGN KEY (tag_id) REFERENCES shared.tag(id) ON DELETE CASCADE;

-- ── governance → shared & iam ───────────────────────────────────────────
ALTER TABLE governance.factcheck
    ADD CONSTRAINT fk_factcheck_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE SET NULL;

ALTER TABLE governance.factcheck
    ADD CONSTRAINT fk_factcheck_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE governance.factcheck_commentaire
    ADD CONSTRAINT fk_fc_comm_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE governance.factcheck_reaction
    ADD CONSTRAINT fk_fc_react_user
    FOREIGN KEY (utilisateur_id) REFERENCES iam.utilisateur(id) ON DELETE CASCADE;

ALTER TABLE governance.bad_habit
    ADD CONSTRAINT fk_bad_habit_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE RESTRICT;

ALTER TABLE governance.bad_habit
    ADD CONSTRAINT fk_bad_habit_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE governance.idea_force
    ADD CONSTRAINT fk_idea_force_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE RESTRICT;

ALTER TABLE governance.idea_force
    ADD CONSTRAINT fk_idea_force_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

-- ── country_profile → shared & iam ──────────────────────────────────────
ALTER TABLE country_profile.fiche_pays
    ADD CONSTRAINT fk_fiche_pays_pays
    FOREIGN KEY (pays_id) REFERENCES shared.pays(id) ON DELETE RESTRICT;

ALTER TABLE country_profile.fiche_pays
    ADD CONSTRAINT fk_fiche_pays_cree_par
    FOREIGN KEY (cree_par) REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;

ALTER TABLE country_profile.lien_interethnique
    ADD CONSTRAINT fk_lien_interethno_pays_lie
    FOREIGN KEY (pays_lie_id) REFERENCES shared.pays(id) ON DELETE SET NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- §15  TRIGGERS : mise à jour automatique de updated_at
-- ════════════════════════════════════════════════════════════════════════════

DO $$
DECLARE
    t RECORD;
BEGIN
    FOR t IN
        SELECT schemaname, tablename
        FROM pg_tables
        WHERE schemaname IN (
            'shared','iam','marketplace','exchange',
            'innovation','culture','media_content',
            'governance','country_profile'
        )
        AND tablename NOT IN ('audit_log')
    LOOP
        -- Vérifie que la table a bien une colonne updated_at
        IF EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_schema = t.schemaname
              AND table_name   = t.tablename
              AND column_name  = 'updated_at'
        ) THEN
            EXECUTE format(
                'CREATE TRIGGER trg_%I_updated_at
                 BEFORE UPDATE ON %I.%I
                 FOR EACH ROW
                 EXECUTE FUNCTION shared.trigger_set_updated_at()',
                t.tablename, t.schemaname, t.tablename
            );
        END IF;
    END LOOP;
END;
$$;


-- ════════════════════════════════════════════════════════════════════════════
-- §16  DONNÉES DE RÉFÉRENCE (seed)
-- ════════════════════════════════════════════════════════════════════════════

-- ── Rôles système ───────────────────────────────────────────────────────

INSERT INTO iam.role (nom, slug, description, est_systeme) VALUES
    ('Super Administrateur', 'super_admin',  'Accès total à la plateforme',           TRUE),
    ('Administrateur',       'admin',         'Gestion courante de la plateforme',     TRUE),
    ('Modérateur',           'moderateur',    'Modération des contenus et utilisateurs', TRUE),
    ('Utilisateur',          'utilisateur',   'Utilisateur standard de la plateforme', TRUE);


-- ── Domaines / Secteurs ─────────────────────────────────────────────────

INSERT INTO shared.domaine_secteur (nom, slug) VALUES
    ('Éducation',                              'education'),
    ('Infrastructure',                         'infrastructure'),
    ('Santé',                                  'sante'),
    ('Eau',                                    'eau'),
    ('Développement des Localités',            'developpement-localites'),
    ('Agriculture',                            'agriculture'),
    ('Technologie & Innovation',               'technologie-innovation'),
    ('Environnement',                          'environnement'),
    ('Gouvernance',                            'gouvernance'),
    ('Culture & Patrimoine',                   'culture-patrimoine'),
    ('Économie & Entrepreneuriat',             'economie-entrepreneuriat'),
    ('Énergie',                                'energie');


-- ── Spécialités Bibliothèque Humaine ────────────────────────────────────

INSERT INTO iam.specialite_bibliotheque (nom, slug) VALUES
    ('Immigration africaine',                  'immigration-africaine'),
    ('Colonisation',                           'colonisation'),
    ('Société africaine',                      'societe-africaine'),
    ('Histoire',                               'histoire'),
    ('Éducation et conseils à l''africaine',   'education-conseils-africaine'),
    ('Mariage en Afrique',                     'mariage-afrique'),
    ('Contes et proverbes',                    'contes-proverbes'),
    ('Migrations',                             'migrations'),
    ('Alliances entre peuples',                'alliances-peuples'),
    ('Totems et interdits',                    'totems-interdits'),
    ('Rites et initiations de peuples',        'rites-initiations'),
    ('Bonnes pratiques africaines',            'bonnes-pratiques-africaines'),
    ('Hommes historiques populaires',          'hommes-historiques-populaires'),
    ('Culture générale',                       'culture-generale'),
    ('Savoirs et innovations',                 'savoirs-innovations'),
    ('Spirituel et religions',                 'spirituel-religions');


-- ── Permissions de base ─────────────────────────────────────────────────

INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    -- Wildcard
    ('Accès total',                    'all.all',               '*',           '*'),
    -- Utilisateurs
    ('Voir les utilisateurs',          'utilisateur.voir',      'utilisateur', 'voir'),
    ('Modifier les utilisateurs',      'utilisateur.modifier',  'utilisateur', 'modifier'),
    ('Supprimer les utilisateurs',     'utilisateur.supprimer', 'utilisateur', 'supprimer'),
    ('Bloquer les utilisateurs',       'utilisateur.bloquer',   'utilisateur', 'bloquer'),
    -- Annonces
    ('Créer une annonce',              'annonce.creer',         'annonce',     'creer'),
    ('Modifier une annonce',           'annonce.modifier',      'annonce',     'modifier'),
    ('Supprimer une annonce',          'annonce.supprimer',     'annonce',     'supprimer'),
    ('Valider une annonce',            'annonce.valider',       'annonce',     'valider'),
    -- Programmes d'échange
    ('Créer un programme d''échange',  'programme.creer',       'programme',   'creer'),
    ('Valider un programme d''échange','programme.valider',     'programme',   'valider'),
    -- Innovations
    ('Suspendre une innovation',       'innovation.suspendre',  'innovation',  'suspendre'),
    -- Projets
    ('Approuver un projet',            'projet.approuver',      'projet',      'approuver'),
    -- Contenus médias
    ('Gérer les événements',           'evenement.gerer',       'evenement',   'gerer'),
    ('Gérer les MOOC',                 'mooc.gerer',            'mooc',        'gerer'),
    ('Gérer les livres',               'livre.gerer',           'livre',       'gerer'),
    ('Gérer les radios/télés',         'radio_tele.gerer',      'radio_tele',  'gerer'),
    -- Centres culturels
    ('Gérer les centres culturels',    'centre_culturel.gerer', 'centre_culturel', 'gerer'),
    -- Afrolang
    ('Créer salle Afrolang publique',  'afrolang.creer_pub',    'afrolang',    'creer_publique'),
    -- Gouvernance
    ('Modérer le FactCheck',           'factcheck.moderer',     'factcheck',   'moderer'),
    ('Modérer les BadHabits',          'bad_habit.moderer',     'bad_habit',   'moderer'),
    ('Modérer les IdeaForces',         'idea_force.moderer',    'idea_force',  'moderer'),
    -- Fiches pays
    ('Gérer les fiches pays',          'fiche_pays.gerer',      'fiche_pays',  'gerer');


-- Attribution des permissions au rôle super_admin
INSERT INTO iam.role_permission (role_id, permission_id)
SELECT r.id, p.id
FROM iam.role r, iam.permission p
WHERE r.slug = 'super_admin' AND p.slug = 'all.all';


-- ════════════════════════════════════════════════════════════════════════════
-- FIN DU SCHÉMA
-- ════════════════════════════════════════════════════════════════════════════
--
-- NOTES POUR LE FUTUR
-- ───────────────────
-- 1. PostGIS : remplacer les colonnes longitude/latitude par des colonnes
--    GEOMETRY(Point, 4326) pour les requêtes géospatiales avancées.
--
-- 2. Messagerie interne : ajouter un schema « messaging » quand le module
--    de messagerie plateforme sera spécifié (conversations, messages, pièces
--    jointes, statut lu/non-lu).
--
-- 3. Notifications : ajouter un schema « notification » pour les alertes
--    temps réel (WebSocket) et les emails différés.
--
-- 4. Full-text search : les colonnes search_vector doivent être alimentées
--    via des triggers applicatifs utilisant to_tsvector('french', ...).
--
-- 5. Partitionnement : si le volume de données le justifie, partitionner
--    shared.audit_log par mois (RANGE sur created_at).
--
-- 6. Découpage microservices : supprimer les contraintes du §14, chaque
--    service gardera les UUID et résoudra via API / événements.
-- ════════════════════════════════════════════════════════════════════════════
