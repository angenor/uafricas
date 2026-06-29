-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : formations organisées en chapitres & leçons
-- ════════════════════════════════════════════════════════════════════════════
-- Une formation (`media_content.mooc`) est désormais structurée en CHAPITRES
-- ordonnés ; chaque chapitre regroupe des LEÇONS ordonnées. Une leçon porte un
-- contenu pédagogique : vidéo (fichier /uploads/... OU lien externe/YouTube),
-- notes texte, document PDF téléchargeable optionnel, durée estimée.
--
-- ACCÈS : la STRUCTURE (titres, durées) est publique pour donner envie ; le
-- CONTENU des leçons (vidéo/texte/document) n'est servi qu'aux INSCRITS
-- (gating applicatif côté backend, cf. handlers/moocs.rs).
--
-- PROGRESSION : un inscrit marque chaque leçon « terminée »
-- (`formation_lecon_completion`) ; la colonne `mooc_inscription.progression`
-- (déjà existante) est recalculée à chaque (dé)marquage.
--
-- Migration idempotente.
-- ════════════════════════════════════════════════════════════════════════════

-- ── Chapitre ────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS media_content.formation_chapitre (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mooc_id      UUID         NOT NULL REFERENCES media_content.mooc(id) ON DELETE CASCADE,
    titre        VARCHAR(350) NOT NULL,
    description  TEXT,
    ordre        INT          NOT NULL DEFAULT 0,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at   TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_formation_chapitre_mooc
    ON media_content.formation_chapitre(mooc_id, ordre) WHERE deleted_at IS NULL;

-- ── Leçon ─────────────────────────────────────────────────────────────────--
CREATE TABLE IF NOT EXISTS media_content.formation_lecon (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    chapitre_id   UUID         NOT NULL REFERENCES media_content.formation_chapitre(id) ON DELETE CASCADE,
    titre         VARCHAR(350) NOT NULL,
    contenu       TEXT,                       -- notes / description texte de la leçon
    video_url     VARCHAR(500),               -- fichier (/uploads/...) ou lien externe/YouTube
    document_url  VARCHAR(500),               -- document PDF téléchargeable (optionnel)
    duree_minutes INT,                        -- durée estimée
    ordre         INT          NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_formation_lecon_chapitre
    ON media_content.formation_lecon(chapitre_id, ordre) WHERE deleted_at IS NULL;

-- ── Progression : leçons terminées par utilisateur inscrit ───────────────────
CREATE TABLE IF NOT EXISTS media_content.formation_lecon_completion (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    lecon_id       UUID        NOT NULL REFERENCES media_content.formation_lecon(id) ON DELETE CASCADE,
    utilisateur_id UUID        NOT NULL,       -- [xref] iam.utilisateur
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (lecon_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_formation_lecon_completion_user
    ON media_content.formation_lecon_completion(utilisateur_id);
