-- ============================================================================
-- Migration de rattrapage PROD — 2026-05-27
-- ============================================================================
-- Couvre les modifications apportées aux fichiers de schéma de BASE qui n'ont
-- PAS de fichier de migration dédié (donc jamais appliquées sur une base déjà
-- initialisée comme la prod). 100 % idempotente : ré-exécutable sans erreur.
--
-- Zones couvertes :
--   1. marketplace.etat_annonce  → valeur 'conclue'           (05_marketplace)
--   2. iam.expertise             → enum 'autre', objectif_expertise,
--                                  colonnes étendues, unicité partielle (04b)
--   3. afrolang                  → session-moderation + ressources-fermeture (08b)
--   4. iam.permission            → 3 permissions (expertise.*, fiche_pays.gerer) (15_seed)
--
-- NB : les colonnes voyage (11→11e) et piste_sous_titre.etat (27→27b) sont
--      couvertes par leurs migrations dédiées et NE sont PAS reprises ici.
-- ============================================================================


-- ── 1. marketplace.etat_annonce : 'conclue' ─────────────────────────────────
ALTER TYPE marketplace.etat_annonce ADD VALUE IF NOT EXISTS 'conclue';


-- ── 2. iam.expertise ────────────────────────────────────────────────────────
ALTER TYPE iam.domaine_expertise ADD VALUE IF NOT EXISTS 'autre';

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'objectif_expertise'
                   AND typnamespace = 'iam'::regnamespace) THEN
        CREATE TYPE iam.objectif_expertise AS ENUM (
            'reseautage', 'consultance', 'recherche_emploi',
            'offre_services_court_terme', 'travail_vacances', 'volontariat', 'benevolat'
        );
    END IF;
END $$;

ALTER TABLE iam.expertise ADD COLUMN IF NOT EXISTS domaine_autre     VARCHAR(120);
ALTER TABLE iam.expertise ADD COLUMN IF NOT EXISTS linkedin_url      VARCHAR(255);
ALTER TABLE iam.expertise ADD COLUMN IF NOT EXISTS cv_url            VARCHAR(500);
ALTER TABLE iam.expertise ADD COLUMN IF NOT EXISTS specialites       TEXT[]  NOT NULL DEFAULT '{}';
ALTER TABLE iam.expertise ADD COLUMN IF NOT EXISTS objectifs         iam.objectif_expertise[] NOT NULL DEFAULT '{}';
ALTER TABLE iam.expertise ADD COLUMN IF NOT EXISTS realisations      TEXT[]  NOT NULL DEFAULT '{}';
ALTER TABLE iam.expertise ADD COLUMN IF NOT EXISTS commentaire_admin TEXT;

-- Unicité totale → unicité partielle "une seule demande active" (autorise re-soumission après refus)
ALTER TABLE iam.expertise DROP CONSTRAINT IF EXISTS expertise_utilisateur_id_key;
CREATE UNIQUE INDEX IF NOT EXISTS idx_expertise_utilisateur_actif
    ON iam.expertise(utilisateur_id)
    WHERE deleted_at IS NULL;


-- ── 3. afrolang : session-moderation + ressources-fermeture ─────────────────

-- 3.1 Types
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname='type_ressource_contribuee' AND typnamespace='afrolang'::regnamespace) THEN
        CREATE TYPE afrolang.type_ressource_contribuee AS ENUM ('document','video_youtube','accompagnateur','lien_web');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname='statut_accompagnateur' AND typnamespace='afrolang'::regnamespace) THEN
        CREATE TYPE afrolang.statut_accompagnateur AS ENUM ('en_attente','acceptee','refusee','retiree');
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname='type_evenement_moderation' AND typnamespace='afrolang'::regnamespace) THEN
        CREATE TYPE afrolang.type_evenement_moderation AS ENUM ('fermeture_admin','reactivation_admin');
    END IF;
END $$;

-- 3.2 Permissions tableau blanc (session-moderation)
CREATE TABLE IF NOT EXISTS afrolang.session_permission_tableau_blanc (
    session_id      UUID         NOT NULL REFERENCES afrolang.session(id) ON DELETE CASCADE,
    utilisateur_id  UUID         NOT NULL,
    accorde_par     UUID         NOT NULL,
    accorde_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    PRIMARY KEY (session_id, utilisateur_id)
);
CREATE INDEX IF NOT EXISTS idx_afrolang_perm_tb_session ON afrolang.session_permission_tableau_blanc(session_id);
CREATE INDEX IF NOT EXISTS idx_afrolang_perm_tb_user    ON afrolang.session_permission_tableau_blanc(utilisateur_id);

-- 3.3 Spotlight session
ALTER TABLE afrolang.session ADD COLUMN IF NOT EXISTS participant_mis_en_evidence_id UUID;
ALTER TABLE afrolang.session ADD COLUMN IF NOT EXISTS mis_en_evidence_par            UUID;
ALTER TABLE afrolang.session ADD COLUMN IF NOT EXISTS mis_en_evidence_at             TIMESTAMPTZ;
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname='ck_session_spotlight_coherent' AND conrelid='afrolang.session'::regclass) THEN
        ALTER TABLE afrolang.session ADD CONSTRAINT ck_session_spotlight_coherent CHECK (
            (participant_mis_en_evidence_id IS NULL AND mis_en_evidence_par IS NULL AND mis_en_evidence_at IS NULL)
            OR
            (participant_mis_en_evidence_id IS NOT NULL AND mis_en_evidence_par IS NOT NULL AND mis_en_evidence_at IS NOT NULL)
        );
    END IF;
END $$;

-- 3.4 Désactivation administrative de salle
ALTER TABLE afrolang.salle ADD COLUMN IF NOT EXISTS desactivee_admin_at      TIMESTAMPTZ;
ALTER TABLE afrolang.salle ADD COLUMN IF NOT EXISTS desactivee_par           UUID;
ALTER TABLE afrolang.salle ADD COLUMN IF NOT EXISTS motif_desactivation      TEXT;
ALTER TABLE afrolang.salle ADD COLUMN IF NOT EXISTS reactivee_at             TIMESTAMPTZ;
ALTER TABLE afrolang.salle ADD COLUMN IF NOT EXISTS reactivee_par            UUID;
ALTER TABLE afrolang.salle ADD COLUMN IF NOT EXISTS commentaire_reactivation TEXT;
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname='ck_salle_desactivation_coherente' AND conrelid='afrolang.salle'::regclass) THEN
        ALTER TABLE afrolang.salle ADD CONSTRAINT ck_salle_desactivation_coherente CHECK (
            (desactivee_admin_at IS NULL AND desactivee_par IS NULL AND motif_desactivation IS NULL)
            OR
            (desactivee_admin_at IS NOT NULL AND desactivee_par IS NOT NULL AND motif_desactivation IS NOT NULL)
        );
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_afrolang_salle_active
    ON afrolang.salle(id) WHERE desactivee_admin_at IS NULL AND deleted_at IS NULL;

-- 3.5 Ressource contribuée
CREATE TABLE IF NOT EXISTS afrolang.ressource_contribuee (
    id                       UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id                 UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    session_origine_id       UUID         REFERENCES afrolang.session(id) ON DELETE SET NULL,
    auteur_id                UUID         NOT NULL,
    type                     afrolang.type_ressource_contribuee NOT NULL,
    titre                    VARCHAR(120) NOT NULL,
    description              VARCHAR(500),
    fichier_url              VARCHAR(500),
    fichier_taille_octets    BIGINT,
    fichier_mime             VARCHAR(120),
    video_url                VARCHAR(500),
    video_id_youtube         VARCHAR(20),
    lien_url                 VARCHAR(1000),
    membre_recommande_id     UUID,
    motif_recommandation     VARCHAR(2000),
    statut_accompagnateur    afrolang.statut_accompagnateur,
    motif_refus              TEXT,
    reponse_at               TIMESTAMPTZ,
    created_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at               TIMESTAMPTZ,
    supprime_par             UUID,
    CONSTRAINT ck_ressource_contribuee_type CHECK (
        (type = 'document'        AND fichier_url IS NOT NULL AND video_url IS NULL AND lien_url IS NULL AND membre_recommande_id IS NULL)
     OR (type = 'video_youtube'   AND video_url IS NOT NULL AND video_id_youtube IS NOT NULL AND fichier_url IS NULL AND lien_url IS NULL AND membre_recommande_id IS NULL)
     OR (type = 'lien_web'        AND lien_url IS NOT NULL AND fichier_url IS NULL AND video_url IS NULL AND membre_recommande_id IS NULL)
     OR (type = 'accompagnateur'  AND membre_recommande_id IS NOT NULL AND statut_accompagnateur IS NOT NULL
                                  AND motif_recommandation IS NOT NULL AND char_length(motif_recommandation) >= 20
                                  AND fichier_url IS NULL AND video_url IS NULL AND lien_url IS NULL)
    ),
    CONSTRAINT ck_ressource_accompagnateur_pas_soi CHECK (
        type <> 'accompagnateur' OR membre_recommande_id <> auteur_id
    )
);
CREATE INDEX IF NOT EXISTS idx_afrolang_ressource_contribuee_salle
    ON afrolang.ressource_contribuee(salle_id, created_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_afrolang_ressource_contribuee_rate_limit
    ON afrolang.ressource_contribuee(auteur_id, salle_id, created_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_afrolang_ressource_recommandations_recues
    ON afrolang.ressource_contribuee(membre_recommande_id, statut_accompagnateur)
    WHERE type = 'accompagnateur' AND deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_afrolang_ressource_contribuee_session_origine
    ON afrolang.ressource_contribuee(session_origine_id) WHERE session_origine_id IS NOT NULL;

-- 3.6 Accès salle privée
CREATE TABLE IF NOT EXISTS afrolang.acces_salle_privee (
    id              UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id UUID         NOT NULL REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    utilisateur_id  UUID         NOT NULL,
    valide_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    revoque_at      TIMESTAMPTZ
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_afrolang_acces_unique_actif
    ON afrolang.acces_salle_privee(salle_privee_id, utilisateur_id) WHERE revoque_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_afrolang_acces_lookup
    ON afrolang.acces_salle_privee(utilisateur_id, salle_privee_id) WHERE revoque_at IS NULL;

-- 3.7 Historique modération salle
CREATE TABLE IF NOT EXISTS afrolang.evenement_moderation_salle (
    id                   UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id             UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    session_concernee_id UUID         REFERENCES afrolang.session(id) ON DELETE SET NULL,
    type_action          afrolang.type_evenement_moderation NOT NULL,
    admin_id             UUID         NOT NULL,
    motif                TEXT,
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    CONSTRAINT ck_moderation_motif_fermeture CHECK (
        (type_action = 'fermeture_admin'    AND motif IS NOT NULL AND char_length(motif) BETWEEN 10 AND 1000)
     OR (type_action = 'reactivation_admin' AND (motif IS NULL OR char_length(motif) <= 1000))
    )
);
CREATE INDEX IF NOT EXISTS idx_afrolang_moderation_salle_chrono
    ON afrolang.evenement_moderation_salle(salle_id, created_at DESC);


-- ── 4. iam.permission : nouvelles permissions ───────────────────────────────
INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    ('Gérer les fiches pays',            'fiche_pays.gerer',   'fiche_pays', 'gerer'),
    ('Voir les demandes d''expertise',   'expertise.voir',     'expertise',  'voir'),
    ('Valider une demande d''expertise', 'expertise.valider',  'expertise',  'valider')
ON CONFLICT (slug) DO NOTHING;
