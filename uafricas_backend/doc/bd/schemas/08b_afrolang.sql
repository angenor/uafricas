-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : afrolang — Visioconférence WebRTC + ressources
-- Refonte 2026-04 (feature 001-afrolang-salles-refonte)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Bounded context dédié, conçu pour être scalé indépendamment sur
-- plusieurs VPS (chaque instance hébergeant un SFU : mediasoup, Janus,
-- LiveKit…). Lors du split microservice, ce schema deviendra sa propre
-- base de données avec son propre service de signaling.
--
-- IMPORTANT : Seules les métadonnées sont stockées ici.
-- Le signaling temps réel (SDP, ICE candidates, état des tracks) et les
-- opérations du tableau blanc transitent par WebSocket et ne sont PAS
-- persistés en base (seul le snapshot final est sauvegardé).
--
-- REFONTE feature 001-afrolang-salles-refonte
-- ─────────────────────────────────────────────
-- • Salle publique : création admin uniquement. Session démarrée/rejointe
--   en un clic par n'importe quel utilisateur connecté.
-- • Salle privée : créée par n'importe quel utilisateur connecté,
--   durable (dormant ↔ live), protégée par un code secret hashé bcrypt.
--   Une seule salle privée active par (salle publique, créateur).
-- • Contrôle d'accès : code secret en clair saisi par l'utilisateur,
--   vérifié contre `code_acces_hash` (bcrypt cost 10). L'auteur n'a
--   jamais besoin de saisir le code.
-- • Rate limit : table `tentative_code_acces` + règle 5 échecs/minute.
-- ════════════════════════════════════════════════════════════════════════════


-- ── Types ─────────────────────────────────────────────────────────────────

CREATE TYPE afrolang.etat_session AS ENUM (
    'planifiee',      -- session programmée mais pas encore démarrée
    'en_cours',       -- visioconférence active
    'terminee',       -- terminée normalement
    'annulee'         -- annulée avant démarrage ou interrompue
);

CREATE TYPE afrolang.type_ressource AS ENUM (
    'fichier',
    'lien_externe'
);

CREATE TYPE afrolang.etat_ressource AS ENUM (
    'publiee',
    'en_attente_validation',
    'refusee'
);

-- Statut du workflow de proposition de salle publique (feature 001-admin-salles-publiques)
-- en_attente → validee | rejetee | retiree (terminaux)
CREATE TYPE afrolang.statut_proposition_salle AS ENUM (
    'en_attente',
    'validee',
    'rejetee',
    'retiree'
);


-- ── Salle publique (canal par groupe ethnique) ────────────────────────────
-- Créée exclusivement par un admin.
-- Rattachée à un groupe_ethnique du référentiel country_profile.
-- La modération attitrée est gérée par la table salle_moderateur (N-N).

CREATE TABLE afrolang.salle (
    id                   UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT,
    image_couverture_url VARCHAR(500),
    langue_cible         VARCHAR(100),                -- langue africaine enseignée (libellé libre)
    langue_code          VARCHAR(40),                 -- code ISO ou code métier
    alphabet             TEXT,                        -- alphabet affiché dans Ressources
    dictionnaire_url     VARCHAR(500),                -- lien direct dictionnaire intégré
    groupe_ethnique_id   UUID         NOT NULL,       -- [xref] country_profile.groupe_ethnique (FK ajoutée dans 13_contraintes)
    actif                BOOLEAN      NOT NULL DEFAULT TRUE,
    cree_par             UUID         NOT NULL,       -- [xref] iam.utilisateur (admin)
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

-- Au plus une salle active par groupe ethnique
CREATE UNIQUE INDEX idx_afrolang_salle_groupe_unique
    ON afrolang.salle(groupe_ethnique_id)
    WHERE actif = TRUE AND deleted_at IS NULL;

CREATE INDEX idx_afrolang_salle_groupe ON afrolang.salle(groupe_ethnique_id);


-- ── Salle privée (cercle restreint durable, code secret) ─────────────────
-- Créée par n'importe quel utilisateur connecté, rattachée à exactement une
-- salle publique. Objet durable : alterne dormant ↔ session live en cours.
-- Protection : code secret hashé bcrypt (cost 10) ; l'auteur n'a jamais
-- besoin de saisir le code (vérifié par comparaison d'identité).
-- Unicité : 1 salle privée active par (salle publique, créateur).
-- FK salle_id : ON DELETE RESTRICT (pas CASCADE) pour éviter la perte
-- silencieuse ; archivage piloté par handler dédié quand la salle publique
-- est désactivée.

CREATE TABLE afrolang.salle_privee (
    id                    UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id              UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE RESTRICT,
    titre                 VARCHAR(350) NOT NULL,
    description           TEXT,
    code_acces_hash       CHAR(60)     NOT NULL,       -- bcrypt cost 10 (R3)
    image_couverture_url  VARCHAR(500),
    max_participants      INT          DEFAULT 50,
    archivee_at           TIMESTAMPTZ,                 -- cascade désactivation salle publique / archivage auteur
    actif                 BOOLEAN      NOT NULL DEFAULT TRUE,
    cree_par              UUID         NOT NULL,       -- [xref] iam.utilisateur
    created_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at            TIMESTAMPTZ
);

CREATE INDEX idx_afrolang_privee_salle ON afrolang.salle_privee(salle_id);

-- 1 salle privée active par (créateur × salle publique) — FR-010, SC-005
CREATE UNIQUE INDEX idx_afrolang_privee_unique_par_salle
    ON afrolang.salle_privee(salle_id, cree_par)
    WHERE archivee_at IS NULL AND deleted_at IS NULL;


-- ── Session de visioconférence (éphémère) ─────────────────────────────────
-- Chaque session représente UNE conférence WebRTC dans une salle publique
-- OU dans une salle privée (XOR).
--
-- Modérateur effectif :
--   • Salle privée  → toujours salle_privee.cree_par (auteur)
--   • Salle publique → 1er arrivé (FR-005b), puis transfert manuel possible
--     ou reprise automatique par un modérateur attitré entrant.

CREATE TABLE afrolang.session (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id         UUID                   REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    salle_id                UUID                   REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    titre                   VARCHAR(350),
    etat                    afrolang.etat_session   NOT NULL DEFAULT 'planifiee',

    -- Modérateur effectif de cette session
    moderateur_id           UUID,                   -- [xref] iam.utilisateur

    -- Planification
    date_debut_prevue       TIMESTAMPTZ,            -- si session programmée à l'avance

    -- Cycle de vie réel
    demarre_at              TIMESTAMPTZ,            -- instant du vrai démarrage
    termine_at              TIMESTAMPTZ,            -- instant de fin
    duree_secondes          INT,                    -- calculé à la fermeture

    -- Capacité & usage
    max_participants        INT          DEFAULT 50,
    nombre_participants_pic INT          DEFAULT 0, -- pic de participants simultanés

    -- Tableau blanc collaboratif activé ?
    tableau_blanc_actif     BOOLEAN      NOT NULL DEFAULT TRUE,

    -- Identification du serveur média (pour routage multi-VPS)
    noeud_id                VARCHAR(120),           -- identifiant du VPS / pod gérant la session

    cree_par                UUID         NOT NULL,  -- [xref] iam.utilisateur
    created_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    -- XOR : rattachement exclusif salle publique OU salle privée
    CONSTRAINT ck_session_contexte CHECK (
        (salle_id IS NOT NULL AND salle_privee_id IS NULL) OR
        (salle_id IS NULL     AND salle_privee_id IS NOT NULL)
    )
);

CREATE INDEX idx_afrolang_session_salle_privee
    ON afrolang.session(salle_privee_id) WHERE salle_privee_id IS NOT NULL;
CREATE INDEX idx_afrolang_session_salle_publique
    ON afrolang.session(salle_id) WHERE salle_id IS NOT NULL;
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
                    CHECK (role_session IN ('moderateur', 'participant', 'observateur')),
    rejoint_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    quitte_at       TIMESTAMPTZ,
    duree_secondes  INT,                            -- calculé au départ
    UNIQUE (session_id, utilisateur_id)
);

CREATE INDEX idx_afrolang_participant_session ON afrolang.session_participant(session_id);
CREATE INDEX idx_afrolang_participant_user    ON afrolang.session_participant(utilisateur_id);


-- ── Tableau blanc collaboratif ────────────────────────────────────────────
-- Un tableau blanc par session. Les opérations temps réel (traits, formes,
-- texte) transitent par WebSocket ; seul le snapshot est persisté en base
-- (sauvegarde périodique + snapshot final à la fermeture de la session).
-- Tous les participants peuvent dessiner ; le modérateur peut effacer.

CREATE TABLE afrolang.tableau_blanc (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id  UUID  NOT NULL REFERENCES afrolang.session(id) ON DELETE CASCADE,
    donnees     JSONB NOT NULL DEFAULT '{}',  -- snapshot : strokes, formes, texte…
    version     INT   NOT NULL DEFAULT 1,     -- incrémenté à chaque sauvegarde
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (session_id)                       -- 1 tableau blanc par session
);


-- ── Modérateurs Afrolang attitrés (feature 005) ───────────────────────────
-- Affectation N-N admin-désignée entre une salle publique et un utilisateur.
-- Retrait = actif=FALSE + retire_at (conserve l'historique pour audit).
-- Concerne UNIQUEMENT les salles publiques (les salles privées n'ont pas
-- de modérateur attitré : le modérateur effectif est toujours l'auteur).

CREATE TABLE afrolang.salle_moderateur (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id          UUID        NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    utilisateur_id    UUID        NOT NULL,                -- [xref] iam.utilisateur
    designe_par       UUID        NOT NULL,                -- [xref] iam.utilisateur (admin)
    designe_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    disponibilite     TEXT,                                -- libre-texte (horaires, fuseau)
    actif             BOOLEAN     NOT NULL DEFAULT TRUE,
    retire_at         TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (salle_id, utilisateur_id)
);

CREATE INDEX idx_afrolang_moderateur_salle
    ON afrolang.salle_moderateur(salle_id) WHERE actif = TRUE;

CREATE INDEX idx_afrolang_moderateur_user
    ON afrolang.salle_moderateur(utilisateur_id) WHERE actif = TRUE;


-- ── Tentative de saisie du code secret (rate limit, R4) ──────────────────
-- Trace chaque tentative de saisie du code secret d'une salle privée.
-- Sert à appliquer le rate limit 5 tentatives/minute/(utilisateur, salle).
-- Règle : si count(succes=FALSE) sur 60 dernières secondes >= 5 ET
-- dernière tentative < 5 min → refuser sans vérifier le hash.
-- Granularité : par utilisateur connecté (pas par IP).

CREATE TABLE afrolang.tentative_code_acces (
    id              UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id UUID         NOT NULL REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    utilisateur_id  UUID         NOT NULL,                          -- [xref] iam.utilisateur
    tente_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    succes          BOOLEAN      NOT NULL DEFAULT FALSE,
    ip              INET,                                            -- traçabilité audit
    user_agent      TEXT
);

CREATE INDEX idx_afrolang_tentative_lookup
    ON afrolang.tentative_code_acces(salle_privee_id, utilisateur_id, tente_at DESC);

CREATE INDEX idx_afrolang_tentative_purge
    ON afrolang.tentative_code_acces(tente_at);


-- ── Ressource pédagogique d'une salle publique (feature 005) ─────────────
-- • Type fichier      : upload interne, publication directe.
-- • Type lien_externe : modération préalable par un modérateur attitré
--                        ou un admin.
-- CHECK ck_ressource_url_coherence : exclusivité fichier_url / lien_url.
-- CHECK ck_ressource_etat_initial  : les fichiers ne peuvent qu'être publiés
--                                    ou refusés (pas d'état en_attente_validation).

CREATE TABLE afrolang.ressource_salle (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id          UUID        NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    titre             VARCHAR(350) NOT NULL,
    description       TEXT,
    type              afrolang.type_ressource NOT NULL,
    fichier_url       VARCHAR(500),                        -- requis si type=fichier
    lien_url          VARCHAR(1000),                       -- requis si type=lien_externe
    etat              afrolang.etat_ressource NOT NULL DEFAULT 'publiee',
    motif_refus       TEXT,
    ajoute_par        UUID        NOT NULL,                -- [xref] iam.utilisateur
    valide_par        UUID,                                -- [xref] iam.utilisateur (modérateur attitré ou admin)
    valide_at         TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at        TIMESTAMPTZ,
    CONSTRAINT ck_ressource_url_coherence CHECK (
        (type = 'fichier'      AND fichier_url IS NOT NULL AND lien_url IS NULL) OR
        (type = 'lien_externe' AND lien_url    IS NOT NULL AND fichier_url IS NULL)
    ),
    CONSTRAINT ck_ressource_etat_initial CHECK (
        (type = 'fichier'      AND etat IN ('publiee', 'refusee')) OR
        (type = 'lien_externe')
    )
);

CREATE INDEX idx_afrolang_ressource_salle
    ON afrolang.ressource_salle(salle_id) WHERE deleted_at IS NULL;

CREATE INDEX idx_afrolang_ressource_attente
    ON afrolang.ressource_salle(etat)
    WHERE etat = 'en_attente_validation' AND deleted_at IS NULL;


-- ── Message de session (messagerie écrite, feature 005) ──────────────────
-- Message texte horodaté par session. Le flux est émis via LiveKit data
-- channel pour la diffusion temps réel ; la persistance sert uniquement à
-- la reprise d'historique (nouvel arrivant) et à la conformité audit.

CREATE TABLE afrolang.message_session (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id      UUID        NOT NULL REFERENCES afrolang.session(id) ON DELETE CASCADE,
    auteur_id       UUID        NOT NULL,                  -- [xref] iam.utilisateur
    contenu         TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_afrolang_message_session
    ON afrolang.message_session(session_id, created_at)
    WHERE deleted_at IS NULL;


-- ── Pays d'origine d'une salle publique (feature 001-afrolang-pays-origine) ──
--
-- Relation N-N entre une salle publique et les pays où la langue cible est
-- parlée à l'origine. Indépendante du pays implicite via groupe_ethnique →
-- fiche_pays. Enrichie manuellement par les admins (Q1 — aucun pré-remplissage).

CREATE TABLE afrolang.salle_pays_origine (
    salle_id    UUID         NOT NULL REFERENCES afrolang.salle(id)   ON DELETE CASCADE,
    pays_id     UUID         NOT NULL REFERENCES shared.pays(id)      ON DELETE CASCADE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    PRIMARY KEY (salle_id, pays_id)
);

CREATE INDEX idx_afrolang_salle_pays_origine_pays
    ON afrolang.salle_pays_origine (pays_id);

COMMENT ON TABLE  afrolang.salle_pays_origine IS
    'Pays d''origine d''une salle publique Afrolang (feature 001-afrolang-pays-origine).';
COMMENT ON COLUMN afrolang.salle_pays_origine.pays_id IS
    'FK vers shared.pays. Filtré sur actif=TRUE côté API publique (Q3).';


-- ── Proposition communautaire de salle publique (feature 001-admin-salles-publiques) ──
-- Workflow : en_attente → validee (création atomique d'une afrolang.salle)
--                       | rejetee (commentaire admin obligatoire)
--                       | retiree (auteur lui-même)
-- Anti-spam : ≥ 5 rejets sur 7 jours → 429 (vérifié côté handler).
-- Unicité : une seule proposition `en_attente` par (auteur, groupe ethnique).

CREATE TABLE afrolang.proposition_salle (
    id                    UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Données soumises par l'utilisateur
    auteur_id             UUID         NOT NULL,                 -- [xref] iam.utilisateur
    titre                 VARCHAR(350) NOT NULL,
    description           TEXT         NOT NULL,
    justification         TEXT         NOT NULL,
    langue_cible          VARCHAR(100) NOT NULL,
    langue_code           VARCHAR(40),
    groupe_ethnique_id    UUID         NOT NULL,                 -- [xref] country_profile.groupe_ethnique
    pays_origine_ids      UUID[]       NOT NULL DEFAULT '{}',    -- ≥ 1 (CHECK)

    -- Workflow
    statut                afrolang.statut_proposition_salle NOT NULL DEFAULT 'en_attente',
    decideur              UUID,                                  -- [xref] iam.utilisateur (admin plateforme)
    decide_at             TIMESTAMPTZ,
    commentaire_decision  TEXT,                                  -- obligatoire si statut=rejetee
    salle_id_creee        UUID         REFERENCES afrolang.salle(id) ON DELETE SET NULL,

    created_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_proposition_pays_non_vide CHECK (cardinality(pays_origine_ids) >= 1),
    CONSTRAINT ck_proposition_decision_coherente CHECK (
        (statut = 'en_attente' AND decideur IS NULL AND decide_at IS NULL) OR
        (statut IN ('validee', 'rejetee') AND decideur IS NOT NULL AND decide_at IS NOT NULL) OR
        (statut = 'retiree' AND decideur IS NULL)
    ),
    CONSTRAINT ck_proposition_rejet_commente CHECK (
        statut <> 'rejetee' OR commentaire_decision IS NOT NULL
    ),
    CONSTRAINT ck_proposition_validation_a_salle CHECK (
        statut <> 'validee' OR salle_id_creee IS NOT NULL
    )
);

CREATE UNIQUE INDEX idx_proposition_salle_unique_attente
    ON afrolang.proposition_salle(auteur_id, groupe_ethnique_id)
    WHERE statut = 'en_attente';

CREATE INDEX idx_proposition_salle_statut
    ON afrolang.proposition_salle(statut, created_at DESC);

CREATE INDEX idx_proposition_salle_auteur
    ON afrolang.proposition_salle(auteur_id, created_at DESC);

COMMENT ON TABLE afrolang.proposition_salle IS
    'Proposition communautaire de salle publique Afrolang (feature 001-admin-salles-publiques).';


-- ── Administrateurs de salle publique (feature 001-admin-salles-publiques) ──
-- Rôle distinct de salle_moderateur (FR-018) — capacités effectives reportées (FR-019).
-- Cette table est le réceptacle d'autorisation : toute future capacité doit
-- s'appuyer sur le helper Rust est_administrateur_salle(salle_id, user_id).
-- Cascades automatiques (FR-021/FR-022) : suspension par handler Rust quand
-- la salle est archivée OU quand le compte utilisateur est désactivé.

CREATE TABLE afrolang.salle_administrateur (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id            UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    utilisateur_id      UUID         NOT NULL,                 -- [xref] iam.utilisateur
    nomme_par           UUID         NOT NULL,                 -- [xref] iam.utilisateur (admin plateforme)
    nomme_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    actif               BOOLEAN      NOT NULL DEFAULT TRUE,
    revoque_at          TIMESTAMPTZ,
    revoque_par         UUID,                                   -- [xref] iam.utilisateur
    motif_revocation    TEXT,
    suspendu_at         TIMESTAMPTZ,                            -- cascade FR-021/FR-022
    motif_suspension    TEXT,                                   -- 'salle_archivee' | 'compte_desactive'

    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_admin_revocation_coherente CHECK (
        (actif = TRUE  AND revoque_at IS NULL AND revoque_par IS NULL AND motif_revocation IS NULL) OR
        (actif = FALSE)
    )
);

CREATE UNIQUE INDEX idx_salle_admin_unique_actif
    ON afrolang.salle_administrateur(salle_id, utilisateur_id)
    WHERE actif = TRUE;

CREATE INDEX idx_salle_admin_par_salle
    ON afrolang.salle_administrateur(salle_id) WHERE actif = TRUE;

CREATE INDEX idx_salle_admin_par_user
    ON afrolang.salle_administrateur(utilisateur_id) WHERE actif = TRUE;

COMMENT ON TABLE afrolang.salle_administrateur IS
    'Administrateurs scopés à une salle publique Afrolang (feature 001-admin-salles-publiques).';
