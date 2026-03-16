-- ════════════════════════════════════════════════════════════════════════════
-- 25 — Collaboration et partage d'arbres
-- ════════════════════════════════════════════════════════════════════════════

-- ─── Colonnes de confidentialité ────────────────────────────────────────

ALTER TABLE arbre_genealogique.arbres
    ADD COLUMN IF NOT EXISTS arbre_prive BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE arbre_genealogique.personnes
    ADD COLUMN IF NOT EXISTS visible_matching BOOLEAN NOT NULL DEFAULT TRUE;

-- ─── Table : invitations ────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS arbre_genealogique.invitations (
    id                      UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    arbre_id                UUID NOT NULL REFERENCES arbre_genealogique.arbres(id),
    email_invite            VARCHAR(255) NOT NULL,
    utilisateur_invite_id   UUID REFERENCES iam.utilisateur(id),
    permission              VARCHAR(20) NOT NULL CHECK (permission IN ('lecture_seule', 'edition')),
    statut                  VARCHAR(20) NOT NULL DEFAULT 'en_attente'
                            CHECK (statut IN ('en_attente', 'acceptee', 'refusee', 'expiree')),
    invite_par              UUID NOT NULL REFERENCES iam.utilisateur(id),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expire_at               TIMESTAMPTZ DEFAULT (NOW() + INTERVAL '30 days'),
    traitee_le              TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_invitations_email
    ON arbre_genealogique.invitations (email_invite)
    WHERE statut = 'en_attente';

CREATE INDEX IF NOT EXISTS idx_invitations_arbre
    ON arbre_genealogique.invitations (arbre_id);

-- ─── Table : collaborateurs ─────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS arbre_genealogique.collaborateurs (
    id                      UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    arbre_id                UUID NOT NULL REFERENCES arbre_genealogique.arbres(id),
    utilisateur_id          UUID NOT NULL REFERENCES iam.utilisateur(id),
    permission              VARCHAR(20) NOT NULL CHECK (permission IN ('lecture_seule', 'edition')),
    invitation_id           UUID REFERENCES arbre_genealogique.invitations(id),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_collaborateur_arbre UNIQUE (arbre_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_collaborateurs_utilisateur
    ON arbre_genealogique.collaborateurs (utilisateur_id);

CREATE INDEX IF NOT EXISTS idx_collaborateurs_arbre
    ON arbre_genealogique.collaborateurs (arbre_id);
