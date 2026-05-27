-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : country_profile -- Infos pratiques voyage
-- Section « À savoir avant de voyager » (bloc structuré)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Ajoute à country_profile.fiche_pays 8 champs texte uniques par territoire,
-- contribuables par tous via le canal de contribution scalaire existant
-- (voir SECTIONS_VALIDES, obtenir_valeur_actuelle, appliquer_fiche_scalaire).
--
-- Idempotent (ADD COLUMN IF NOT EXISTS) : exécutable sur une base déjà initialisée.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE country_profile.fiche_pays
    ADD COLUMN IF NOT EXISTS voyage_langue_internationale    TEXT,
    ADD COLUMN IF NOT EXISTS voyage_langue_locale            TEXT,
    ADD COLUMN IF NOT EXISTS voyage_infos_visa               TEXT,
    ADD COLUMN IF NOT EXISTS voyage_infos_sanitaires         TEXT,
    ADD COLUMN IF NOT EXISTS voyage_meteo                    TEXT,
    ADD COLUMN IF NOT EXISTS voyage_prises_electriques       TEXT,
    ADD COLUMN IF NOT EXISTS voyage_contacts_tourisme        TEXT,
    ADD COLUMN IF NOT EXISTS voyage_recommandations_securite TEXT;
