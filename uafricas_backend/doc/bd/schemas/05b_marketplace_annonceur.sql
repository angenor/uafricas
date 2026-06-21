-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : marketplace — Type d'annonceur & coordonnées publiques
-- ════════════════════════════════════════════════════════════════════════════
-- Dépend de : 05_marketplace.sql (marketplace.annonce)
--
-- Une annonce est publiée soit au NOM PROPRE (particulier) → le contact reste
-- privé (révélé sur demande via la messagerie), soit au nom d'une ENTREPRISE →
-- les coordonnées (téléphone, email, adresse) sont affichées publiquement.
-- Champ optionnel commun : site web ou page réseau social.
-- Migration idempotente.

DO $$ BEGIN
    CREATE TYPE marketplace.type_annonceur AS ENUM ('particulier', 'entreprise');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

ALTER TABLE marketplace.annonce
    ADD COLUMN IF NOT EXISTS type_annonceur    marketplace.type_annonceur NOT NULL DEFAULT 'particulier',
    ADD COLUMN IF NOT EXISTS nom_entreprise    VARCHAR(200),
    ADD COLUMN IF NOT EXISTS contact_telephone VARCHAR(30),
    ADD COLUMN IF NOT EXISTS contact_email     VARCHAR(255),
    ADD COLUMN IF NOT EXISTS contact_adresse   VARCHAR(300),
    ADD COLUMN IF NOT EXISTS site_web_url      VARCHAR(500);
