-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : media_content — Vidafrica (sous-titrage vidéo)
-- ════════════════════════════════════════════════════════════════════════════

-- ── Enum : langues de sous-titres ────────────────────────────────────────

CREATE TYPE media_content.langue_sous_titre AS ENUM (
    'francais', 'anglais', 'arabe', 'portugais', 'swahili',
    'wolof', 'haoussa', 'amharique', 'zoulou', 'lingala',
    'bambara', 'yoruba', 'peul', 'espagnol', 'mandarin'
);

-- ── Table : video ────────────────────────────────────────────────────────

CREATE TABLE media_content.video (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(300) NOT NULL,
    slug                 VARCHAR(350) NOT NULL,
    description          TEXT,
    fichier_video_url    VARCHAR(500) NOT NULL,
    vignette_url         VARCHAR(500),
    duree_secondes       INTEGER,
    taille_octets        BIGINT,
    format_video         VARCHAR(20),
    etat                 VARCHAR(50) NOT NULL DEFAULT 'brouillon'
                         CHECK (etat IN ('brouillon', 'publie', 'suspendu', 'supprime')),
    cree_par             UUID NOT NULL,  -- [xref] iam.utilisateur
    search_vector        TSVECTOR,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_video_slug ON media_content.video (slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_video_etat ON media_content.video (etat) WHERE deleted_at IS NULL;
CREATE INDEX idx_video_search ON media_content.video USING GIN (search_vector);
CREATE INDEX idx_video_cree_par ON media_content.video (cree_par);

-- Trigger search_vector
CREATE OR REPLACE FUNCTION media_content.video_search_vector_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.search_vector := to_tsvector('french', COALESCE(NEW.titre, '') || ' ' || COALESCE(NEW.description, ''));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_video_search_vector
    BEFORE INSERT OR UPDATE OF titre, description
    ON media_content.video
    FOR EACH ROW EXECUTE FUNCTION media_content.video_search_vector_trigger();

-- ── Table : piste_sous_titre ─────────────────────────────────────────────

CREATE TABLE media_content.piste_sous_titre (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    video_id             UUID NOT NULL REFERENCES media_content.video(id),
    langue               media_content.langue_sous_titre NOT NULL,
    est_complete         BOOLEAN NOT NULL DEFAULT false,
    cree_par             UUID NOT NULL,  -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

CREATE INDEX idx_piste_sous_titre_video ON media_content.piste_sous_titre (video_id) WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX idx_piste_sous_titre_unique_langue ON media_content.piste_sous_titre (video_id, langue) WHERE deleted_at IS NULL;

-- ── Table : segment_sous_titre ───────────────────────────────────────────

CREATE TABLE media_content.segment_sous_titre (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    piste_id             UUID NOT NULL REFERENCES media_content.piste_sous_titre(id),
    position             INTEGER NOT NULL,
    texte                TEXT NOT NULL,
    debut_ms             INTEGER NOT NULL CHECK (debut_ms >= 0),
    fin_ms               INTEGER NOT NULL,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_segment_debut_fin CHECK (debut_ms < fin_ms),
    CONSTRAINT uq_segment_piste_position UNIQUE (piste_id, position)
);

CREATE INDEX idx_segment_piste_position ON media_content.segment_sous_titre (piste_id, position);
CREATE INDEX idx_segment_piste_debut ON media_content.segment_sous_titre (piste_id, debut_ms);

-- ── Table : timing_mot ───────────────────────────────────────────────────

CREATE TABLE media_content.timing_mot (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    segment_id           UUID NOT NULL REFERENCES media_content.segment_sous_titre(id) ON DELETE CASCADE,
    position             INTEGER NOT NULL,
    mot                  VARCHAR(200) NOT NULL,
    debut_ms             INTEGER NOT NULL CHECK (debut_ms >= 0),
    fin_ms               INTEGER NOT NULL,
    CONSTRAINT chk_timing_mot_debut_fin CHECK (debut_ms < fin_ms),
    CONSTRAINT uq_timing_mot_segment_position UNIQUE (segment_id, position)
);

CREATE INDEX idx_timing_mot_segment_position ON media_content.timing_mot (segment_id, position);
