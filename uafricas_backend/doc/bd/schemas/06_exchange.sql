-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : exchange — Programmes d'échange
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
