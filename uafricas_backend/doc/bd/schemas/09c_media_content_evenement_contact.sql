-- ─────────────────────────────────────────────────────────────────────────
-- 09c — media_content.evenement : type d'organisateur & contact
-- ─────────────────────────────────────────────────────────────────────────
-- L'organisateur publie soit EN SON NOM PROPRE (personnel), soit AU NOM D'UNE
-- ORGANISATION. Le type est important pour les statistiques de la plateforme.
-- Quand type_organisateur = 'organisation', `contact_nom` porte le nom de
-- l'organisation. Les autres champs de contact (email, telephone, site web)
-- sont des coordonnees facultatives dans les deux cas.
-- Toutes les colonnes sont nullables/avec defaut (retrocompatibilite).
-- Migration idempotente.

DO $$ BEGIN
    CREATE TYPE media_content.type_organisateur AS ENUM ('personnel', 'organisation');
EXCEPTION WHEN duplicate_object THEN NULL; END $$;

ALTER TABLE media_content.evenement
    ADD COLUMN IF NOT EXISTS type_organisateur  media_content.type_organisateur NOT NULL DEFAULT 'personnel',
    ADD COLUMN IF NOT EXISTS contact_nom        VARCHAR(250),
    ADD COLUMN IF NOT EXISTS contact_email      VARCHAR(320),
    ADD COLUMN IF NOT EXISTS contact_telephone  VARCHAR(50),
    ADD COLUMN IF NOT EXISTS contact_site_web   VARCHAR(500);
