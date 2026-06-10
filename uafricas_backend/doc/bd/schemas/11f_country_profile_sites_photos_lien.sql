-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : country_profile -- Sites touristiques : photos + lien web
-- Feature : sites-touristiques-photos-lien (2026-06-09)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Ce fichier enrichit country_profile.site_touristique pour :
--   • Relever la galerie de 3 → 5 images (contrainte CHECK)
--   • Ajouter un lien de site web officiel (site_web_url), facultatif, validé
--     http(s) côté application (Rust), affiché en redirection externe.
--
-- Conformité : Principe III (SQL source de vérité). Idempotent.
-- ════════════════════════════════════════════════════════════════════════════


-- ────────────────────────────────────────────────────────────────────────────
-- SECTION A : Lien de site web officiel (facultatif)
-- ────────────────────────────────────────────────────────────────────────────
-- Validation du schéma http/https effectuée côté application (canal de
-- contribution modéré + back-office), comme la cohérence famille↔sous-type.

ALTER TABLE country_profile.site_touristique
    ADD COLUMN IF NOT EXISTS site_web_url TEXT;


-- ────────────────────────────────────────────────────────────────────────────
-- SECTION B : Galerie portée à 5 images (la 1re sert toujours de couverture)
-- ────────────────────────────────────────────────────────────────────────────
-- On supprime les deux noms de contrainte possibles (max3 historique, max5
-- éventuel d'un re-run) avant de (re)créer la contrainte à 5 — idempotence.

ALTER TABLE country_profile.site_touristique
    DROP CONSTRAINT IF EXISTS chk_site_images_max3;
ALTER TABLE country_profile.site_touristique
    DROP CONSTRAINT IF EXISTS chk_site_images_max5;
ALTER TABLE country_profile.site_touristique
    ADD CONSTRAINT chk_site_images_max5 CHECK (cardinality(images) <= 5);
