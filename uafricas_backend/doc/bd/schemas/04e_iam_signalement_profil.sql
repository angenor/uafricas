-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : iam — Signalement de profil (faux profils / arnaques)
-- ════════════════════════════════════════════════════════════════════════════
-- Dépend de : 04_iam.sql (iam.utilisateur, iam.etat_utilisateur)
--
-- Tout membre connecté peut signaler le profil d'un autre membre (1 fois).
-- Au-delà de SEUIL_SIGNALEMENTS_PROFIL_SUSPENSION (20 signalements distincts),
-- le compte passe automatiquement en etat='suspendu' (invisible publiquement).
-- Calqué sur governance.factcheck_signalement. Migration idempotente.


CREATE TABLE IF NOT EXISTS iam.signalement_profil (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- Profil signalé
    utilisateur_id  UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    -- Auteur du signalement
    signale_par     UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    motif           VARCHAR(50),                         -- ex: 'faux_profil', 'arnaque', 'usurpation', 'harcelement', 'autre'
    description     TEXT CHECK (description IS NULL OR char_length(description) <= 1000),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (utilisateur_id, signale_par)                 -- idempotence : 1 signalement par membre
);

-- Compteur dénormalisé sur l'utilisateur signalé
ALTER TABLE iam.utilisateur
    ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_signalement_profil_cible
    ON iam.signalement_profil (utilisateur_id);

CREATE INDEX IF NOT EXISTS idx_signalement_profil_auteur
    ON iam.signalement_profil (signale_par);
