-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : country_profile -- Afripulse enrichissement
-- Feature : 001-afripulse-contributions (2026-04-18)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Ce fichier étend le schéma country_profile pour la plateforme Afripulse :
--   • Extensions de country_profile.site_touristique (catégorie emblematique/prive)
--   • Extensions de country_profile.contribution_fiche (type_objet, section,
--     target_id, nouvelle/ancienne valeur JSONB, pièces jointes)
--   • Nouveaux enums : categorie_site_touristique, type_objet_contribution,
--     section_afripulse, categorie_savoir, domaine_personnalite
--   • Nouvelles tables : personnalite_connue, savoir_pratique,
--     recommandation_visiteur, photo_visiteur
--   • Index additionnels pour rate-limit et file admin
--   • Triggers updated_at (pattern shared.trigger_set_updated_at())
-- ════════════════════════════════════════════════════════════════════════════


-- ════════════════════════════════════════════════════════════════════════════
-- SECTION B : NOUVEAUX ENUMS (créés AVANT les ALTER / CREATE TABLE)
-- ════════════════════════════════════════════════════════════════════════════

CREATE TYPE country_profile.categorie_site_touristique AS ENUM (
    'emblematique',
    'prive'
);

CREATE TYPE country_profile.type_objet_contribution AS ENUM (
    'fiche_pays',
    'site_touristique',
    'secteur_developpement',
    'personnalite_connue',
    'savoir_pratique',
    'recommandation_visiteur',
    'photo_visiteur'
);

CREATE TYPE country_profile.section_afripulse AS ENUM (
    'sites_emblematiques',
    'sites_prives',
    'secteurs_opportunites',
    'personnalites',
    'savoir_avant_voyager',
    'recommandations',
    'galerie_photos'
);

CREATE TYPE country_profile.categorie_savoir AS ENUM (
    'langue_argot',
    'coutumes',
    'etiquette',
    'securite',
    'sante',
    'transports',
    'autre'
);

CREATE TYPE country_profile.domaine_personnalite AS ENUM (
    'politique',
    'artiste_musicien',
    'artiste_autre',
    'sportif',
    'entrepreneur',
    'scientifique',
    'militaire_historique',
    'autre'
);


-- ════════════════════════════════════════════════════════════════════════════
-- SECTION A : EXTENSIONS DES TABLES EXISTANTES
-- ════════════════════════════════════════════════════════════════════════════

-- A.1 : country_profile.site_touristique — colonne catégorie + soft delete
ALTER TABLE country_profile.site_touristique
    ADD COLUMN IF NOT EXISTS categorie country_profile.categorie_site_touristique
        NOT NULL DEFAULT 'emblematique';

ALTER TABLE country_profile.site_touristique
    ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_site_touristique_categorie
    ON country_profile.site_touristique (fiche_pays_id, categorie)
    WHERE deleted_at IS NULL;


-- A.2 : country_profile.contribution_fiche — colonnes Afripulse (JSONB + typage)
ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS type_objet_contribution country_profile.type_objet_contribution
        NOT NULL DEFAULT 'fiche_pays';

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS section_afripulse country_profile.section_afripulse;

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS target_id UUID;

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS nouvelle_valeur_jsonb JSONB;

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS ancienne_valeur_jsonb JSONB;

ALTER TABLE country_profile.contribution_fiche
    ADD COLUMN IF NOT EXISTS pieces_jointes JSONB NOT NULL DEFAULT '[]'::jsonb;

-- Les colonnes TEXT ancienne_valeur / nouvelle_valeur sont CONSERVÉES pour
-- rétrocompatibilité avec les contributions legacy (section=texte scalaire).


-- A.3 : etat_contribution — valeur 'obsolete' (marquage automatique lors d'une
-- approbation surplantant des contributions concurrentes)
ALTER TYPE country_profile.etat_contribution ADD VALUE IF NOT EXISTS 'obsolete';


-- ════════════════════════════════════════════════════════════════════════════
-- SECTION C : NOUVELLES TABLES
-- ════════════════════════════════════════════════════════════════════════════

-- C.1 : Personnalités connues du pays
CREATE TABLE country_profile.personnalite_connue (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    nom_complet         VARCHAR(250) NOT NULL,
    domaine             country_profile.domaine_personnalite NOT NULL,
    biographie_courte   TEXT NOT NULL,
    annee_naissance     SMALLINT,
    annee_deces         SMALLINT,
    portrait_url        VARCHAR(500),
    lien_reference      VARCHAR(500),
    cree_par            UUID NOT NULL,  -- [xref] iam.utilisateur
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,
    CHECK (annee_deces IS NULL OR annee_naissance IS NULL OR annee_deces >= annee_naissance)
);

CREATE INDEX idx_personnalite_fiche
    ON country_profile.personnalite_connue (fiche_pays_id)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_personnalite_domaine
    ON country_profile.personnalite_connue (fiche_pays_id, domaine)
    WHERE deleted_at IS NULL;


-- C.2 : Savoir pratique (conseils à connaître avant de voyager)
CREATE TABLE country_profile.savoir_pratique (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    titre               VARCHAR(250) NOT NULL,
    categorie           country_profile.categorie_savoir NOT NULL,
    explication         TEXT NOT NULL,
    exemple             TEXT,
    cree_par            UUID NOT NULL,  -- [xref] iam.utilisateur
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_savoir_fiche
    ON country_profile.savoir_pratique (fiche_pays_id)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_savoir_categorie
    ON country_profile.savoir_pratique (fiche_pays_id, categorie)
    WHERE deleted_at IS NULL;


-- C.3 : Recommandation d'un visiteur (note 1-5 + commentaire)
-- Invariant : au plus UNE recommandation active par (utilisateur, pays)
CREATE TABLE country_profile.recommandation_visiteur (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    utilisateur_id      UUID NOT NULL,  -- [xref] iam.utilisateur
    note                SMALLINT NOT NULL CHECK (note BETWEEN 1 AND 5),
    commentaire         TEXT NOT NULL,
    active              BOOLEAN NOT NULL DEFAULT TRUE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,
    CHECK (char_length(commentaire) BETWEEN 50 AND 2000)
);

CREATE UNIQUE INDEX uniq_recommandation_active
    ON country_profile.recommandation_visiteur (utilisateur_id, fiche_pays_id)
    WHERE active = TRUE AND deleted_at IS NULL;

CREATE INDEX idx_reco_fiche_active
    ON country_profile.recommandation_visiteur (fiche_pays_id, active)
    WHERE deleted_at IS NULL;


-- C.4 : Photo visiteur (galerie photos légendées)
CREATE TABLE country_profile.photo_visiteur (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fiche_pays_id       UUID NOT NULL REFERENCES country_profile.fiche_pays(id) ON DELETE CASCADE,
    utilisateur_id      UUID NOT NULL,  -- [xref] iam.utilisateur
    chemin_fichier      VARCHAR(500) NOT NULL,
    legende             VARCHAR(500) NOT NULL,
    format              VARCHAR(10) NOT NULL CHECK (format IN ('jpeg', 'png')),
    taille_octets       INTEGER NOT NULL CHECK (taille_octets > 0 AND taille_octets <= 2097152),
    largeur_px          SMALLINT NOT NULL CHECK (largeur_px > 0 AND largeur_px <= 2048),
    hauteur_px          SMALLINT NOT NULL CHECK (hauteur_px > 0 AND hauteur_px <= 2048),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_photo_fiche
    ON country_profile.photo_visiteur (fiche_pays_id, created_at DESC)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_photo_utilisateur
    ON country_profile.photo_visiteur (utilisateur_id, created_at DESC)
    WHERE deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- SECTION D : INDEX ADDITIONNELS SUR contribution_fiche (rate-limit + file admin)
-- ════════════════════════════════════════════════════════════════════════════

CREATE INDEX IF NOT EXISTS idx_contribution_rate_limit
    ON country_profile.contribution_fiche (cree_par, created_at)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_contribution_attente_pays
    ON country_profile.contribution_fiche (cree_par, fiche_pays_id, etat)
    WHERE etat = 'en_attente' AND deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_contribution_type_section
    ON country_profile.contribution_fiche (type_objet_contribution, section_afripulse, etat)
    WHERE deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- SECTION E : TRIGGERS updated_at (pattern shared.trigger_set_updated_at())
-- ════════════════════════════════════════════════════════════════════════════

CREATE TRIGGER trg_personnalite_updated
    BEFORE UPDATE ON country_profile.personnalite_connue
    FOR EACH ROW EXECUTE FUNCTION shared.trigger_set_updated_at();

CREATE TRIGGER trg_savoir_updated
    BEFORE UPDATE ON country_profile.savoir_pratique
    FOR EACH ROW EXECUTE FUNCTION shared.trigger_set_updated_at();

CREATE TRIGGER trg_reco_updated
    BEFORE UPDATE ON country_profile.recommandation_visiteur
    FOR EACH ROW EXECUTE FUNCTION shared.trigger_set_updated_at();

-- Note : photo_visiteur n'a pas d'updated_at (immuable après création).
-- ════════════════════════════════════════════════════════════════════════════
