-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : afrolang — Visioconférence WebRTC + ressources
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
-- MODÉRATION (feature 005 : double modération)
-- ────────────────────────────────────────────
-- • Modération attitrée  : table salle_moderateur (N-N admin-désigné,
--                          disponibilité libre-texte, actif/retire_at).
-- • Modération de session : session.moderateur_id est attribué
--                           dynamiquement (1er arrivé ou transfert ou
--                           reprise par attitré entrant).
-- • Salle privée   : le modérateur est TOUJOURS le créateur (cree_par).
-- • Démarrage session privée : seul le modérateur peut démarrer.
-- ════════════════════════════════════════════════════════════════════════════


-- ── Types ─────────────────────────────────────────────────────────────────

CREATE TYPE afrolang.etat_session AS ENUM (
    'planifiee',      -- session programmée mais pas encore démarrée
    'en_cours',       -- visioconférence active
    'terminee',       -- terminée normalement
    'annulee'         -- annulée avant démarrage ou interrompue
);

-- ── Nouveaux enums (feature 005) ────────────────────────────────────────

CREATE TYPE afrolang.etat_proposition AS ENUM (
    'en_attente',
    'approuvee',
    'refusee'
);

CREATE TYPE afrolang.motif_salle_privee AS ENUM (
    'apprentissage_enfants',
    'reseautage_adulte',
    'echanges_groupe'
);

CREATE TYPE afrolang.visibilite_salle_privee AS ENUM (
    'fermee',
    'visible'
);

CREATE TYPE afrolang.type_adhesion AS ENUM (
    'demande',
    'invitation',
    'abonne'
);

CREATE TYPE afrolang.etat_adhesion AS ENUM (
    'en_attente',
    'acceptee',
    'refusee',
    'groupe_complet'
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


-- ── Salle publique (canal par groupe ethnique) ────────────────────────────
-- Créée par un admin, ou issue d'une proposition validée (proposition_salle).
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


-- ── Salle privée (sous-salle créée par un utilisateur) ────────────────────
-- Le modérateur est TOUJOURS le créateur (cree_par). Lui seul peut
-- démarrer une session dans cette salle.
-- Feature 005 : motif obligatoire, déclaration d'adulte (18+) obligatoire,
-- visibilité fermée/visible, archivage automatique (créateur supprimé,
-- salle publique désactivée en cascade) + unicité 1 par (membre × salle).
-- FK salle_id : ON DELETE RESTRICT (pas CASCADE) pour éviter la perte
-- silencieuse de salles privées ; archivage piloté par handler dédié.

CREATE TABLE afrolang.salle_privee (
    id                    UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id              UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE RESTRICT,
    titre                 VARCHAR(350) NOT NULL,
    description           TEXT,
    code_acces            VARCHAR(100),               -- code pour rejoindre (legacy, optionnel)
    image_couverture_url  VARCHAR(500),
    max_participants      INT          DEFAULT 50,
    motif                 afrolang.motif_salle_privee NOT NULL,
    declaration_adulte_at TIMESTAMPTZ  NOT NULL,      -- capture de la case cochée (FR-033)
    visibilite            afrolang.visibilite_salle_privee NOT NULL DEFAULT 'fermee',
    archivee_at           TIMESTAMPTZ,                -- cascade créateur supprimé / salle publique désactivée
    actif                 BOOLEAN      NOT NULL DEFAULT TRUE,
    cree_par              UUID         NOT NULL,      -- [xref] iam.utilisateur
    created_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at            TIMESTAMPTZ
);

CREATE INDEX idx_afrolang_privee_salle ON afrolang.salle_privee(salle_id);

-- 1 salle privée active par (membre × salle publique) — FR-035, SC-010
CREATE UNIQUE INDEX idx_afrolang_privee_unique_par_salle
    ON afrolang.salle_privee(salle_id, cree_par)
    WHERE archivee_at IS NULL AND deleted_at IS NULL;

CREATE INDEX idx_afrolang_privee_visibilite
    ON afrolang.salle_privee(salle_id, visibilite)
    WHERE archivee_at IS NULL AND deleted_at IS NULL;


-- ── Session de visioconférence (éphémère) ─────────────────────────────────
-- Chaque session représente UNE conférence WebRTC dans une salle privée.
-- C'est cette table que le load-balancer / orchestrateur interroge pour
-- router les participants vers le bon VPS.
--
-- Modérateur effectif :
--   • Salle privée → toujours salle_privee.cree_par
--   • Salle publique → feature 005 : 1er arrivé, puis transfert manuel
--     possible ou reprise automatique par un modérateur attitré entrant.

-- Une session est rattachée EXCLUSIVEMENT à une salle publique OU à une salle
-- privée (XOR). Les salles publiques hébergent leur propre session live
-- communautaire (spec FR-026→FR-029, SC-001, SC-003) ; les salles privées
-- hébergent les sessions de groupes restreints.
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


-- ── Proposition de salle publique (feature 005) ───────────────────────────
-- Un membre propose la création d'une salle pour un groupe ethnique absent.
-- L'admin valide ou refuse. À l'approbation, la salle est créée et le champ
-- salle_id_creee est renseigné. Détection de doublons applicative via
-- lower(unaccent(nom)) contre les salles actives et les propositions en_attente.

CREATE TABLE afrolang.proposition_salle (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom_groupe_ethnique     VARCHAR(250) NOT NULL,
    pays_id                 UUID,                          -- [xref] shared.pays (facultatif)
    groupe_ethnique_id      UUID,                          -- [xref] country_profile.groupe_ethnique
    langue_cible            VARCHAR(100),
    description             TEXT,
    etat                    afrolang.etat_proposition NOT NULL DEFAULT 'en_attente',
    motif_refus             TEXT,
    salle_id_creee          UUID REFERENCES afrolang.salle(id) ON DELETE SET NULL,
    propose_par             UUID         NOT NULL,         -- [xref] iam.utilisateur
    decide_par              UUID,                          -- [xref] iam.utilisateur (admin)
    decide_at               TIMESTAMPTZ,
    created_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_afrolang_proposition_etat
    ON afrolang.proposition_salle(etat) WHERE deleted_at IS NULL;

CREATE INDEX idx_afrolang_proposition_auteur
    ON afrolang.proposition_salle(propose_par) WHERE deleted_at IS NULL;


-- ── Modérateurs Afrolang attitrés (feature 005) ───────────────────────────
-- Affectation N-N admin-désignée entre une salle publique et un utilisateur.
-- Retrait = actif=FALSE + retire_at (conserve l'historique pour audit).

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


-- ── Adhésion à une salle privée (demande / invitation / abonné) ──────────
-- Modèle unifié :
--   • type=demande     : initié par l'utilisateur, décidé par le créateur.
--   • type=invitation  : initié par le créateur, décidé par l'utilisateur.
--   • type=abonne      : état terminal après acceptation.
-- Atomicité de la limite : SELECT ... FOR UPDATE sur salle_privee puis
-- comparaison avec COUNT(*) des abonnés actifs (gestion « groupe_complet »).

CREATE TABLE afrolang.salle_privee_adhesion (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id   UUID        NOT NULL REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    utilisateur_id    UUID        NOT NULL,                -- [xref] iam.utilisateur (destinataire/demandeur)
    type              afrolang.type_adhesion NOT NULL,
    etat              afrolang.etat_adhesion NOT NULL DEFAULT 'en_attente',
    initiateur_id     UUID        NOT NULL,                -- [xref] iam.utilisateur
    decideur_id       UUID,                                -- [xref] iam.utilisateur
    decided_at        TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at        TIMESTAMPTZ,
    UNIQUE (salle_privee_id, utilisateur_id)
);

CREATE INDEX idx_afrolang_adhesion_salle
    ON afrolang.salle_privee_adhesion(salle_privee_id) WHERE deleted_at IS NULL;

CREATE INDEX idx_afrolang_adhesion_user
    ON afrolang.salle_privee_adhesion(utilisateur_id) WHERE deleted_at IS NULL;

CREATE INDEX idx_afrolang_adhesion_attente
    ON afrolang.salle_privee_adhesion(salle_privee_id, etat)
    WHERE etat = 'en_attente' AND deleted_at IS NULL;


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
