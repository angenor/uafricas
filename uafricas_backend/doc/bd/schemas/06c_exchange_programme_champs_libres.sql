-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : exchange — Champs libres du programme d'échange
-- ════════════════════════════════════════════════════════════════════════════
-- DDL idempotent (rejouable). Étend exchange.programme :
--   • domaine_libre : précision textuelle quand le domaine choisi est « Autre »
--     (le domaine référentiel domaine_id reste NULL dans ce cas)
--   • statut_legal  : statut légal de l'organisation soumettante (texte libre,
--     ex. SARL, Association loi 1901, ONG…)
-- À charger après 06b_exchange_candidatures.sql.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE exchange.programme
    ADD COLUMN IF NOT EXISTS domaine_libre VARCHAR(250);

ALTER TABLE exchange.programme
    ADD COLUMN IF NOT EXISTS statut_legal  VARCHAR(250);
