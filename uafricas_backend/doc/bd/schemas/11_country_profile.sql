-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : country_profile — Fiches pays
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
    -- Bloc « À savoir avant de voyager » (infos pratiques uniques par territoire,
    -- contribuables par tous via le canal de contribution scalaire fiche_pays)
    voyage_langue_internationale     TEXT,
    voyage_langue_locale             TEXT,
    voyage_infos_visa                TEXT,
    voyage_infos_sanitaires          TEXT,
    voyage_meteo                     TEXT,
    voyage_prises_electriques        TEXT,
    voyage_contacts_tourisme         TEXT,
    voyage_recommandations_securite  TEXT,
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
