-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : afrolang — Visioconférence WebRTC
-- ════════════════════════════════════════════════════════════════════════════
--
-- Bounded context dédié, conçu pour être scalé indépendamment sur
-- plusieurs VPS (chaque instance hébergeant un SFU : mediasoup, Janus,
-- LiveKit…).  Lors du split microservice, ce schema deviendra sa propre
-- base de données avec son propre service de signaling.
--
-- IMPORTANT : Seules les métadonnées sont stockées ici.
-- Le signaling temps réel (SDP, ICE candidates, état des tracks) transite
-- par WebSocket et n'est PAS persisté en base.
-- ════════════════════════════════════════════════════════════════════════════


-- ── Types ─────────────────────────────────────────────────────────────────

CREATE TYPE afrolang.etat_session AS ENUM (
    'planifiee',      -- session programmée mais pas encore démarrée
    'en_cours',       -- visioconférence active
    'terminee',       -- terminée normalement
    'annulee'         -- annulée avant démarrage ou interrompue
);


-- ── Salle (canal par langue africaine) ────────────────────────────────────
-- Anciennement culture.afrolang_salle_publique — créée par un admin.

CREATE TABLE afrolang.salle (
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


-- ── Salle privée (sous-salle créée par un utilisateur) ────────────────────
-- Anciennement culture.afrolang_salle_privee.

CREATE TABLE afrolang.salle_privee (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id             UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    titre                VARCHAR(350) NOT NULL,
    description          TEXT,
    code_acces           VARCHAR(100),               -- code pour rejoindre
    image_couverture_url VARCHAR(500),
    max_participants     INT          DEFAULT 50,
    actif                BOOLEAN      NOT NULL DEFAULT TRUE,
    cree_par             UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_afrolang_privee_salle ON afrolang.salle_privee(salle_id);


-- ── Session de visioconférence (éphémère) ─────────────────────────────────
-- Chaque session représente UNE conférence WebRTC dans une salle privée.
-- C'est cette table que le load-balancer / orchestrateur interroge pour
-- router les participants vers le bon VPS.

CREATE TABLE afrolang.session (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id         UUID                   NOT NULL REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    titre                   VARCHAR(350),
    etat                    afrolang.etat_session   NOT NULL DEFAULT 'planifiee',

    -- Planification
    date_debut_prevue       TIMESTAMPTZ,            -- si session programmée à l'avance

    -- Cycle de vie réel
    demarre_at              TIMESTAMPTZ,            -- instant du vrai démarrage
    termine_at              TIMESTAMPTZ,            -- instant de fin
    duree_secondes          INT,                    -- calculé à la fermeture

    -- Capacité & usage
    max_participants        INT          DEFAULT 50,
    nombre_participants_pic INT          DEFAULT 0, -- pic de participants simultanés

    -- Identification du serveur média (pour routage multi-VPS)
    noeud_id                VARCHAR(120),           -- identifiant du VPS / pod gérant la session

    cree_par                UUID         NOT NULL,  -- [xref] iam.utilisateur
    created_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_afrolang_session_salle  ON afrolang.session(salle_privee_id);
CREATE INDEX idx_afrolang_session_etat   ON afrolang.session(etat) WHERE etat IN ('planifiee', 'en_cours');
CREATE INDEX idx_afrolang_session_noeud  ON afrolang.session(noeud_id) WHERE noeud_id IS NOT NULL;
CREATE INDEX idx_afrolang_session_date   ON afrolang.session(date_debut_prevue);


-- ── Participant par session ───────────────────────────────────────────────
-- Tracking de présence par session (pas par salle).
-- Permet de calculer : durée de participation, pics, analytics.

CREATE TABLE afrolang.session_participant (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id      UUID        NOT NULL REFERENCES afrolang.session(id) ON DELETE CASCADE,
    utilisateur_id  UUID        NOT NULL,           -- [xref] iam.utilisateur
    role_session    VARCHAR(30) NOT NULL DEFAULT 'participant'
                    CHECK (role_session IN ('animateur', 'participant', 'observateur')),
    rejoint_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    quitte_at       TIMESTAMPTZ,
    duree_secondes  INT,                            -- calculé au départ
    UNIQUE (session_id, utilisateur_id)
);

CREATE INDEX idx_afrolang_participant_session ON afrolang.session_participant(session_id);
CREATE INDEX idx_afrolang_participant_user    ON afrolang.session_participant(utilisateur_id);
