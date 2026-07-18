-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Engagement : « mise en avant » d'une contribution (règle +5)
-- ════════════════════════════════════════════════════════════════════════════
-- Câble la règle `contribution_mise_en_avant` (barème 35_engagement.sql), jusque‑là
-- orpheline faute d'un signal « vedette » sur les contributions.
--
-- Table générique (calquée sur country_profile.signalement_contribution) : l'équipe
-- marque n'importe quelle contribution ayant un auteur unique (codimoi, factcheck,
-- bad_habit, idea_force, piste de sous‑titres vidafrica) comme mise en avant ;
-- le back‑office déclenche alors le +5 à l'auteur (idempotent, anti‑auto‑attribution,
-- non‑bloquant). Retirer la mise en avant NE reprend PAS les points (pas de clawback).
-- Migration idempotente.
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS engagement.mise_en_avant (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type_objet   VARCHAR(40) NOT NULL,        -- codimoi / factcheck / bad_habit / idea_force / video
    objet_id     UUID        NOT NULL,
    auteur_id    UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE, -- bénéficiaire du +5
    mis_par      UUID        REFERENCES iam.utilisateur(id) ON DELETE SET NULL,          -- admin ayant mis en avant
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (type_objet, objet_id)
);

CREATE INDEX IF NOT EXISTS idx_mise_en_avant_objet
    ON engagement.mise_en_avant(type_objet, objet_id);
CREATE INDEX IF NOT EXISTS idx_mise_en_avant_auteur
    ON engagement.mise_en_avant(auteur_id);
