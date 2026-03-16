-- ════════════════════════════════════════════════════════════════════════════
-- 26 — Notifications et détection de doublons
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS arbre_genealogique.notifications (
    id                  UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    destinataire_id     UUID NOT NULL REFERENCES iam.utilisateur(id),
    type                VARCHAR(30) NOT NULL,
    message             TEXT NOT NULL,
    lien_action         VARCHAR(500),
    lu                  BOOLEAN NOT NULL DEFAULT FALSE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_notifications_destinataire
    ON arbre_genealogique.notifications (destinataire_id, lu)
    WHERE lu = FALSE;

CREATE INDEX IF NOT EXISTS idx_notifications_date
    ON arbre_genealogique.notifications (created_at DESC);

CREATE TABLE IF NOT EXISTS arbre_genealogique.doublons_ignores (
    id                  UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    arbre_id            UUID NOT NULL REFERENCES arbre_genealogique.arbres(id),
    personne_a_id       UUID NOT NULL REFERENCES arbre_genealogique.personnes(id),
    personne_b_id       UUID NOT NULL REFERENCES arbre_genealogique.personnes(id),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_uq_doublon_ignore
    ON arbre_genealogique.doublons_ignores (
        arbre_id,
        LEAST(personne_a_id, personne_b_id),
        GREATEST(personne_a_id, personne_b_id)
    );
