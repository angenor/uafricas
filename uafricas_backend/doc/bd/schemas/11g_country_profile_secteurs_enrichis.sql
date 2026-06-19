-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : country_profile -- Secteurs d'opportunités enrichis
-- Feature : secteurs-opportunites-enrichis (2026-06-10)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Ce fichier enrichit country_profile.secteur_developpement avec, en plus du
-- nom et de la description : localité, contacts (téléphone/courriel/adresse),
-- références utiles, lien de site web (validé http/https côté application) et
-- une image illustrative.
--
-- Conformité : Principe III (SQL source de vérité). Colonnes nullables →
-- rétrocompatibilité (les secteurs existants restent valides). Idempotent.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE country_profile.secteur_developpement
    ADD COLUMN IF NOT EXISTS localite           VARCHAR(200),
    ADD COLUMN IF NOT EXISTS contact_telephone  VARCHAR(40),
    ADD COLUMN IF NOT EXISTS contact_courriel   VARCHAR(254),
    ADD COLUMN IF NOT EXISTS contact_adresse    VARCHAR(500),
    -- `references` est un mot réservé SQL → on nomme la colonne `references_utiles`.
    ADD COLUMN IF NOT EXISTS references_utiles  TEXT,
    ADD COLUMN IF NOT EXISTS site_web_url       TEXT,
    ADD COLUMN IF NOT EXISTS image_url          VARCHAR(500);
