-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : iam — Notes d'expertise (notation des experts)
-- ════════════════════════════════════════════════════════════════════════════
-- Dépend de : 04b_iam_expertise.sql (iam.expertise), 04_iam.sql (iam.utilisateur)
--
-- Tout membre connecté peut attribuer une note (1–5) à un expert (hors son
-- propre profil). Une seule note active par (auteur, expertise) — modifiable.
-- Le champ iam.expertise.rating reflète la moyenne, recalculé par le backend
-- (handler experts::noter_expert) à chaque insertion / mise à jour.
-- Migration idempotente.


-- ── Table note_expertise ──────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS iam.note_expertise (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    expertise_id    UUID        NOT NULL
                    REFERENCES iam.expertise(id) ON DELETE CASCADE,
    auteur_id       UUID        NOT NULL
                    REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    note            SMALLINT    NOT NULL CHECK (note BETWEEN 1 AND 5),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- ── Indexes ────────────────────────────────────────────────────────────────

-- Une seule note par (auteur, expert) : l'upsert s'appuie dessus (ON CONFLICT)
CREATE UNIQUE INDEX IF NOT EXISTS uniq_note_expertise
    ON iam.note_expertise (expertise_id, auteur_id);

-- Lecture / agrégation par expert
CREATE INDEX IF NOT EXISTS idx_note_expertise_expertise
    ON iam.note_expertise (expertise_id);
