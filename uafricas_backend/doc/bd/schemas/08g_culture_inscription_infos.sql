-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : culture -- Infos d'inscription aux programmations
-- Feature : centres-culturels-inscription-infos (2026-06-21)
-- ════════════════════════════════════════════════════════════════════════════
--
-- L'inscription à une programmation collecte désormais des informations
-- saisies par l'inscrit : nom, prénom, pays, lieu de résidence, titre.
-- Colonnes nullables → rétrocompatibilité (anciennes inscriptions conservées).
-- Idempotent.
--
-- Conformité : Principe III (SQL source de vérité).
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE culture.programmation_inscription
    ADD COLUMN IF NOT EXISTS nom            VARCHAR(200),
    ADD COLUMN IF NOT EXISTS prenom         VARCHAR(200),
    ADD COLUMN IF NOT EXISTS pays           VARCHAR(150),
    ADD COLUMN IF NOT EXISTS lieu_residence VARCHAR(250),
    ADD COLUMN IF NOT EXISTS titre          VARCHAR(200);
