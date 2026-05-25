-- ════════════════════════════════════════════════════════════════════════════
-- SCHEMA : social — Demandes d'amitié, amitiés, blocages & messagerie privée
-- Bounded context : relations sociales entre membres + messagerie temps réel 1-1
-- Créé : 2026-05-25 (feature 001-demande-amitie)
-- ────────────────────────────────────────────────────────────────────────────
-- Dépendances cross-schema : iam.utilisateur, shared.pays (résolues inline ici,
--   comme pour le schema arbre_genealogique / 26_notifications).
--
-- Conventions : PK UUID (gen_random_uuid), TIMESTAMPTZ, soft delete deleted_at
--   où pertinent, snake_case français, enums PostgreSQL.
--
-- Ordre canonique des paires (Décision 4 research.md) : pour amitie et
--   conversation, utilisateur_a_id < utilisateur_b_id ⇒ une seule ligne par paire.
--   Le blocage reste orienté (bloqueur_id, bloque_id).
-- ════════════════════════════════════════════════════════════════════════════


-- ── Schema ───────────────────────────────────────────────────────────────────

CREATE SCHEMA IF NOT EXISTS social;


-- ── Types (Enums) ────────────────────────────────────────────────────────────

CREATE TYPE social.statut_demande_amitie AS ENUM (
    'en_attente', 'acceptee', 'refusee', 'annulee'
);

CREATE TYPE social.type_notification_social AS ENUM (
    'demande_recue', 'demande_acceptee'
);


-- ── demande_amitie ───────────────────────────────────────────────────────────
-- Sollicitation orientée d'un membre vers un autre.

CREATE TABLE social.demande_amitie (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    demandeur_id    UUID                            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    destinataire_id UUID                            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    statut          social.statut_demande_amitie    NOT NULL DEFAULT 'en_attente',
    created_at      TIMESTAMPTZ                     NOT NULL DEFAULT NOW(),
    traite_at       TIMESTAMPTZ,
    deleted_at      TIMESTAMPTZ,

    -- FR-002 : on ne peut pas se solliciter soi-même
    CONSTRAINT ck_demande_pas_soi CHECK (demandeur_id <> destinataire_id)
);

-- FR-003 : pas deux demandes en attente identiques (sens orienté)
CREATE UNIQUE INDEX uq_demande_active
    ON social.demande_amitie (demandeur_id, destinataire_id)
    WHERE statut = 'en_attente' AND deleted_at IS NULL;

-- Liste des demandes reçues / envoyées en attente
CREATE INDEX idx_demande_destinataire_attente
    ON social.demande_amitie (destinataire_id)
    WHERE statut = 'en_attente' AND deleted_at IS NULL;

CREATE INDEX idx_demande_demandeur_attente
    ON social.demande_amitie (demandeur_id)
    WHERE statut = 'en_attente' AND deleted_at IS NULL;

-- FR-014 : rate-limit (COUNT sur 24 h glissantes par demandeur)
CREATE INDEX idx_demande_demandeur_date
    ON social.demande_amitie (demandeur_id, created_at);


-- ── amitie ───────────────────────────────────────────────────────────────────
-- Relation mutuelle symétrique. Ordre canonique a < b.

CREATE TABLE social.amitie (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    utilisateur_a_id  UUID            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    utilisateur_b_id  UUID            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    created_at        TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_amitie_ordre CHECK (utilisateur_a_id < utilisateur_b_id),
    CONSTRAINT uq_amitie_paire UNIQUE (utilisateur_a_id, utilisateur_b_id)
);

-- Recherche d'amitiés par l'un ou l'autre membre
CREATE INDEX idx_amitie_b ON social.amitie (utilisateur_b_id);


-- ── blocage ──────────────────────────────────────────────────────────────────
-- Relation orientée empêchant sollicitation et communication.

CREATE TABLE social.blocage (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bloqueur_id  UUID            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    bloque_id    UUID            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    created_at   TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_blocage_pas_soi CHECK (bloqueur_id <> bloque_id),
    CONSTRAINT uq_blocage_paire UNIQUE (bloqueur_id, bloque_id)
);

-- Vérifier « suis-je bloqué par X »
CREATE INDEX idx_blocage_bloque ON social.blocage (bloque_id);


-- ── conversation ─────────────────────────────────────────────────────────────
-- Fil privé entre deux membres. Ordre canonique. Conservée après rupture
-- d'amitié (historique lisible) mais verrouillée pour de nouveaux messages.

CREATE TABLE social.conversation (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    utilisateur_a_id    UUID            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    utilisateur_b_id    UUID            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    dernier_message_at  TIMESTAMPTZ,

    CONSTRAINT ck_conversation_ordre CHECK (utilisateur_a_id < utilisateur_b_id),
    CONSTRAINT uq_conversation_paire UNIQUE (utilisateur_a_id, utilisateur_b_id)
);

CREATE INDEX idx_conversation_dernier_message
    ON social.conversation (dernier_message_at DESC);

CREATE INDEX idx_conversation_b ON social.conversation (utilisateur_b_id);


-- ── message ──────────────────────────────────────────────────────────────────
-- Message texte dans une conversation.

CREATE TABLE social.message (
    id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    conversation_id   UUID            NOT NULL REFERENCES social.conversation(id) ON DELETE CASCADE,
    expediteur_id     UUID            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    contenu           TEXT            NOT NULL,
    created_at        TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    lu_at             TIMESTAMPTZ,
    deleted_at        TIMESTAMPTZ,

    -- FR-027 : longueur du contenu 1..2000
    CONSTRAINT ck_message_contenu_longueur CHECK (char_length(contenu) BETWEEN 1 AND 2000)
);

-- Pagination de l'historique
CREATE INDEX idx_message_conversation_date
    ON social.message (conversation_id, created_at DESC);

-- Comptage des non-lus
CREATE INDEX idx_message_non_lus
    ON social.message (conversation_id, lu_at)
    WHERE lu_at IS NULL;


-- ── notification ─────────────────────────────────────────────────────────────
-- Notification relationnelle (Décision 5). Seuls les évènements de relation
-- (demande reçue, demande acceptée) en créent ; pas les nouveaux messages.

CREATE TABLE social.notification (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    destinataire_id UUID                            NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    type            social.type_notification_social NOT NULL,
    demande_id      UUID                            REFERENCES social.demande_amitie(id) ON DELETE CASCADE,
    acteur_id       UUID                            REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    lu              BOOLEAN                         NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ                     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notification_destinataire
    ON social.notification (destinataire_id, lu);

CREATE INDEX idx_notification_date
    ON social.notification (created_at DESC);
