-- ════════════════════════════════════════════════════════════════════════════
-- 09b — media_content : Événements en streaming direct (feature 001-evenements-streaming)
-- ════════════════════════════════════════════════════════════════════════════
-- Sessions de diffusion temps réel rattachées à un événement « en ligne » / « hybride ».
-- Modèle webinaire (diffusion 1→N) calqué sur afrolang.session, allégé :
--   - une seule session « en_cours » par événement (index unique partiel) ;
--   - rôle du participant => droit de diffuser (can_publish côté token LiveKit) ;
--   - aucun média persisté (flux via SFU LiveKit) ; chat/réactions éphémères (DataPackets).
-- Migration IDEMPOTENTE (CREATE TABLE IF NOT EXISTS / ADD COLUMN IF NOT EXISTS) —
-- réexécutable sans erreur (Principe III, parité avec les migrations existantes).
-- À inclure dans schema.sql via \ir, après 09_media_content.sql et avant 12_audit.sql.

-- ── Session de direct ───────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS media_content.evenement_session (
    id                       UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    evenement_id             UUID        NOT NULL
                             REFERENCES media_content.evenement(id) ON DELETE CASCADE,
    etat                     VARCHAR(30) NOT NULL DEFAULT 'en_cours'
                             CHECK (etat IN ('en_cours', 'terminee')),
    organisateur_id          UUID        NOT NULL,           -- [xref] iam.utilisateur (= evenement.cree_par)
    demarre_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    termine_at               TIMESTAMPTZ,
    duree_secondes           INT,                            -- calculé à la clôture
    max_participants         INT         NOT NULL DEFAULT 100,  -- capacité (D8 ; >= SC-004)
    nombre_participants_pic  INT         NOT NULL DEFAULT 0,
    arret_securite_at        TIMESTAMPTZ NOT NULL,           -- arrêt de sécurité absolu (D6), figé à l'ouverture
    noeud_id                 VARCHAR(120),                   -- nœud SFU (routage multi-VPS, parité afrolang)
    created_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at               TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Colonnes additives idempotentes (réexécution sur une table préexistante)
ALTER TABLE media_content.evenement_session
    ADD COLUMN IF NOT EXISTS arret_securite_at TIMESTAMPTZ;
ALTER TABLE media_content.evenement_session
    ADD COLUMN IF NOT EXISTS noeud_id VARCHAR(120);

-- Une seule session active par événement (FR-015)
CREATE UNIQUE INDEX IF NOT EXISTS uq_evenement_session_active
    ON media_content.evenement_session(evenement_id) WHERE etat = 'en_cours';
CREATE INDEX IF NOT EXISTS idx_evenement_session_evenement
    ON media_content.evenement_session(evenement_id);
CREATE INDEX IF NOT EXISTS idx_evenement_session_etat
    ON media_content.evenement_session(etat) WHERE etat = 'en_cours';

-- ── Participant à une session ───────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS media_content.evenement_session_participant (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id      UUID        NOT NULL
                    REFERENCES media_content.evenement_session(id) ON DELETE CASCADE,
    utilisateur_id  UUID        NOT NULL,                    -- [xref] iam.utilisateur
    role            VARCHAR(30) NOT NULL DEFAULT 'spectateur'
                    CHECK (role IN ('organisateur', 'intervenant', 'spectateur')),
    main_levee      BOOLEAN     NOT NULL DEFAULT FALSE,      -- demande de parole en cours (FR-022)
    main_levee_at   TIMESTAMPTZ,                             -- horodatage de la demande (ordre d'affichage)
    rejoint_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    quitte_at       TIMESTAMPTZ,                             -- NULL = présent ; recompté à la reconnexion
    duree_secondes  INT,                                     -- cumul à la sortie
    UNIQUE (session_id, utilisateur_id)
);

ALTER TABLE media_content.evenement_session_participant
    ADD COLUMN IF NOT EXISTS main_levee BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE media_content.evenement_session_participant
    ADD COLUMN IF NOT EXISTS main_levee_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_evenement_session_participant_session
    ON media_content.evenement_session_participant(session_id);
CREATE INDEX IF NOT EXISTS idx_evenement_session_participant_main
    ON media_content.evenement_session_participant(session_id) WHERE main_levee = TRUE;
