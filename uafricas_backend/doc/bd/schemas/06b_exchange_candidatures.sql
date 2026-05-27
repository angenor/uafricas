-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : exchange — Enrichissement candidatures & proposition
-- ════════════════════════════════════════════════════════════════════════════
-- DDL idempotent (rejouable). Étend exchange.programme et exchange.candidature :
--   • Type d'organisation soumettante (association / entreprise / service public)
--   • Candidat retenu (sélection finale par l'organisateur, affichage public)
--   • Champs de candidature enrichis (état civil, fonction, lieu, statut emploi,
--     adéquation au profil, lien compte expertise)
-- À charger après 06_exchange.sql.
-- ════════════════════════════════════════════════════════════════════════════

-- ── Enums ───────────────────────────────────────────────────────────────────

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type t JOIN pg_namespace n ON n.oid = t.typnamespace
        WHERE t.typname = 'type_organisation_soumettante' AND n.nspname = 'exchange'
    ) THEN
        CREATE TYPE exchange.type_organisation_soumettante
            AS ENUM ('association', 'entreprise', 'service_public');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type t JOIN pg_namespace n ON n.oid = t.typnamespace
        WHERE t.typname = 'statut_emploi_candidat' AND n.nspname = 'exchange'
    ) THEN
        -- Condition d'éligibilité : être en emploi ou retraité (jamais sans emploi)
        CREATE TYPE exchange.statut_emploi_candidat
            AS ENUM ('en_emploi', 'retraite');
    END IF;
END $$;

-- ── Programme : organisation soumettante + candidat retenu ───────────────────

ALTER TABLE exchange.programme
    ADD COLUMN IF NOT EXISTS type_organisation exchange.type_organisation_soumettante;

ALTER TABLE exchange.programme
    ADD COLUMN IF NOT EXISTS candidat_retenu_id UUID;          -- [xref] iam.utilisateur

ALTER TABLE exchange.programme
    ADD COLUMN IF NOT EXISTS candidat_retenu_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_programme_candidat_retenu
    ON exchange.programme(candidat_retenu_id)
    WHERE candidat_retenu_id IS NOT NULL;

-- ── Candidature : champs enrichis ────────────────────────────────────────────

ALTER TABLE exchange.candidature
    ADD COLUMN IF NOT EXISTS nom_etat_civil    VARCHAR(250);

ALTER TABLE exchange.candidature
    ADD COLUMN IF NOT EXISTS fonction_actuelle VARCHAR(250);

ALTER TABLE exchange.candidature
    ADD COLUMN IF NOT EXISTS lieu_residence    VARCHAR(250);

ALTER TABLE exchange.candidature
    ADD COLUMN IF NOT EXISTS statut_emploi     exchange.statut_emploi_candidat;

ALTER TABLE exchange.candidature
    ADD COLUMN IF NOT EXISTS repond_profil     BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE exchange.candidature
    ADD COLUMN IF NOT EXISTS lien_expertise    VARCHAR(500);
