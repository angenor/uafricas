-- ════════════════════════════════════════════════════════════════════════════
-- Schema : iam — Validation admin des Bibliothèques Humaines
-- Feature : 001-admin-biblio-humaine
-- ════════════════════════════════════════════════════════════════════════════

-- ── Enum de statut ───────────────────────────────────────────────────────

CREATE TYPE iam.statut_demande_biblio AS ENUM ('en_attente', 'valide', 'rejete');

-- ── Table principale des demandes ────────────────────────────────────────

CREATE TABLE iam.demande_biblio_humaine (
    id               UUID         DEFAULT gen_random_uuid() PRIMARY KEY,
    utilisateur_id   UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    statut           iam.statut_demande_biblio NOT NULL DEFAULT 'en_attente',
    fonction         VARCHAR(255) NOT NULL,
    biographie       TEXT         NOT NULL CHECK (length(biographie) >= 20),
    pays_origine_id  UUID         REFERENCES shared.pays(id),
    commentaire_admin TEXT,
    traite_par       UUID         REFERENCES iam.utilisateur(id),
    traite_le        TIMESTAMPTZ,
    created_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at       TIMESTAMPTZ
);

-- ── Table de jointure demande–spécialités ────────────────────────────────

CREATE TABLE iam.demande_biblio_specialite (
    demande_id    UUID NOT NULL REFERENCES iam.demande_biblio_humaine(id) ON DELETE CASCADE,
    specialite_id UUID NOT NULL REFERENCES iam.specialite_bibliotheque(id) ON DELETE CASCADE,
    PRIMARY KEY (demande_id, specialite_id)
);

-- ── Index ────────────────────────────────────────────────────────────────

-- Accès rapide aux demandes d'un utilisateur
CREATE INDEX idx_demande_biblio_utilisateur
    ON iam.demande_biblio_humaine(utilisateur_id)
    WHERE deleted_at IS NULL;

-- Filtrage par statut (principal usage admin)
CREATE INDEX idx_demande_biblio_statut
    ON iam.demande_biblio_humaine(statut)
    WHERE deleted_at IS NULL;

-- Contrainte unicité : un seul en_attente ou valide par utilisateur
CREATE UNIQUE INDEX idx_demande_biblio_active_unique
    ON iam.demande_biblio_humaine(utilisateur_id)
    WHERE statut IN ('en_attente', 'valide') AND deleted_at IS NULL;

-- ── Notifications in-app (US5) ───────────────────────────────────────────

CREATE TABLE iam.notification_biblio_humaine (
    id             UUID         DEFAULT gen_random_uuid() PRIMARY KEY,
    utilisateur_id UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    type           VARCHAR(20)  NOT NULL CHECK (type IN ('approuvee', 'rejetee')),
    lu             BOOLEAN      NOT NULL DEFAULT FALSE,
    commentaire    TEXT,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notification_biblio_utilisateur
    ON iam.notification_biblio_humaine(utilisateur_id)
    WHERE lu = FALSE;
