-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : iam — Validation manuelle d'un compte par un admin
-- ════════════════════════════════════════════════════════════════════════════
-- Dépend de : 04_iam.sql (iam.utilisateur)
--
-- Un administrateur peut valider manuellement un compte (gage de crédibilité
-- de l'annonceur sur le marché). Distinct de `etat` (cycle de vie du compte) et
-- de `documents_verifie` (pièce d'identité). Sert de badge « Compte validé ».
-- Migration idempotente.

ALTER TABLE iam.utilisateur
    ADD COLUMN IF NOT EXISTS compte_verifie_admin     BOOLEAN     NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS compte_verifie_admin_par UUID        REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    ADD COLUMN IF NOT EXISTS compte_verifie_admin_at  TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_utilisateur_compte_verifie_admin
    ON iam.utilisateur (compte_verifie_admin);
