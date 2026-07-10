-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schéma « engagement » (gamification, Phase 1)
-- ════════════════════════════════════════════════════════════════════════════
-- Fondation du système d'engagement : compte de points par utilisateur,
-- journal des mouvements (append-only, idempotent), barème paramétrable
-- (règles, paliers de popularité, seuils de niveaux).
-- Migration idempotente (CREATE ... IF NOT EXISTS, seed ON CONFLICT DO NOTHING).
-- Voir specs/001-engagement-gamification/data-model.md
-- ════════════════════════════════════════════════════════════════════════════

CREATE SCHEMA IF NOT EXISTS engagement;


-- ── Compte d'engagement (1‑1 utilisateur) ───────────────────────────────────

CREATE TABLE IF NOT EXISTS engagement.compte (
    utilisateur_id       UUID PRIMARY KEY REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    solde_points         INTEGER     NOT NULL DEFAULT 0 CHECK (solde_points >= 0),
    solde_points_mensuel INTEGER     NOT NULL DEFAULT 0,
    mois_courant         DATE        NOT NULL DEFAULT date_trunc('month', NOW())::date,
    reputation           INTEGER     NOT NULL DEFAULT 0,
    niveau_code          VARCHAR(30) NOT NULL DEFAULT 'membre',
    dernier_mouvement_at TIMESTAMPTZ,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- ── Journal des mouvements de points (append-only, immuable) ────────────────

CREATE TABLE IF NOT EXISTS engagement.mouvement_points (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id    UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    type_action       VARCHAR(50) NOT NULL,           -- réfère engagement.regle_points.type_action
    type_objet        VARCHAR(40),                    -- codimoi / factcheck / video / ideaforce / bad_habit / biblio_humaine / fiche_pays
    objet_id          UUID,
    points            INTEGER     NOT NULL,           -- delta réellement appliqué (signé, après plancher/écrêtage)
    reputation_delta  INTEGER     NOT NULL DEFAULT 0,
    solde_apres       INTEGER     NOT NULL,           -- snapshot du solde global après application
    plafond_atteint   BOOLEAN     NOT NULL DEFAULT FALSE,
    cle_idempotence   TEXT        NOT NULL UNIQUE,    -- empêche le doublon (idempotence structurelle)
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_mouvement_utilisateur ON engagement.mouvement_points(utilisateur_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_mouvement_type_action ON engagement.mouvement_points(type_action, created_at);


-- ── Règles de barème (paramétrables sans redéploiement) ─────────────────────

CREATE TABLE IF NOT EXISTS engagement.regle_points (
    id                 UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type_action        VARCHAR(50)  NOT NULL UNIQUE,
    libelle            VARCHAR(150) NOT NULL,
    points             INTEGER      NOT NULL,          -- signé (négatif = malus)
    reputation_delta   INTEGER      NOT NULL DEFAULT 0,
    plafond_journalier INTEGER,                        -- NULL = illimité
    plafond_mensuel    INTEGER,
    actif              BOOLEAN      NOT NULL DEFAULT TRUE,
    updated_at         TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ── Paliers de popularité (nombre de « j'aime » → points) ───────────────────

CREATE TABLE IF NOT EXISTS engagement.palier_popularite (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    seuil_likes INTEGER NOT NULL UNIQUE CHECK (seuil_likes > 0),
    points      INTEGER NOT NULL,
    actif       BOOLEAN NOT NULL DEFAULT TRUE
);


-- ── Seuils de niveaux (statuts dérivés du solde) ────────────────────────────

CREATE TABLE IF NOT EXISTS engagement.niveau (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code          VARCHAR(30) NOT NULL UNIQUE,        -- membre / premium / platinum
    libelle       VARCHAR(80) NOT NULL,
    seuil_min     INTEGER     NOT NULL,               -- solde minimal pour entrer dans le niveau
    ordre         SMALLINT    NOT NULL,
    badge_couleur VARCHAR(20),
    badge_icone   VARCHAR(40)
);


-- ════════════════════════════════════════════════════════════════════════════
-- SEED — barème par défaut (valeurs indicatives, paramétrables ensuite)
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO engagement.regle_points (type_action, libelle, points, reputation_delta, plafond_journalier, plafond_mensuel) VALUES
    ('contribution_validee',        'Contribution validée par modération',      2,  0, NULL, NULL),
    ('contribution_mise_en_avant',  'Contribution mise en avant par l''équipe', 5,  0, NULL, NULL),
    ('factcheck_valide',            'FactCheck jugé correct',                   3,  1, NULL, NULL),
    ('factcheck_faux',              'FactCheck jugé faux / abusif',            -2, -3, NULL, NULL),
    ('popularite_palier',           'Palier de popularité franchi',             0,  0, NULL, NULL),
    ('ajustement_admin',            'Correction manuelle (administration)',     0,  0, NULL, NULL)
ON CONFLICT (type_action) DO NOTHING;

INSERT INTO engagement.palier_popularite (seuil_likes, points) VALUES
    (100,  10),
    (500,  30),
    (1000, 50)
ON CONFLICT (seuil_likes) DO NOTHING;

INSERT INTO engagement.niveau (code, libelle, seuil_min, ordre, badge_couleur, badge_icone) VALUES
    ('membre',   'Membre',               0,    1, 'gray',  'user'),
    ('premium',  'Membre Premium',       200,  2, 'amber', 'star'),
    ('platinum', 'Influenceur Platinum', 1000, 3, 'slate', 'crown')
ON CONFLICT (code) DO NOTHING;


-- ── Permission d'administration du barème (idempotent) ──────────────────────

INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    ('Gérer l''engagement (points)', 'engagement.gerer', 'engagement', 'gerer')
ON CONFLICT (slug) DO NOTHING;

-- Attribuer la permission au rôle super_admin
INSERT INTO iam.role_permission (role_id, permission_id)
SELECT r.id, p.id
FROM iam.role r, iam.permission p
WHERE r.slug = 'super_admin' AND p.slug = 'engagement.gerer'
ON CONFLICT (role_id, permission_id) DO NOTHING;
