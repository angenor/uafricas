-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — governance : FactCheck — Type de publication + preuve
-- ════════════════════════════════════════════════════════════════════════════
-- Idempotent : exécutable sur une base déjà initialisée comme en init fraîche.
--
-- 1. Chaque factcheck porte un « type de publication » :
--      • on_dit         → « On dit / entendu quelque part »
--      • adage_legende  → « Adage / Légende »
--      • fait_vecu      → « Fait vécu ou observé » (lieu, territoire, date, preuve)
-- 2. Pour un fait vécu, l'auteur peut joindre une preuve (photo ou PDF). En son
--    absence, l'interface affiche « Pas de preuve » dans un coin de la publication.
--    Le territoire réutilise `pays_id`, la date réutilise `created_at`.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE governance.factcheck
    ADD COLUMN IF NOT EXISTS type_publication VARCHAR(20) NOT NULL DEFAULT 'on_dit',
    ADD COLUMN IF NOT EXISTS preuve_url       VARCHAR(500),
    ADD COLUMN IF NOT EXISTS preuve_type      VARCHAR(10);

-- Contrainte sur le type de publication
ALTER TABLE governance.factcheck DROP CONSTRAINT IF EXISTS factcheck_type_publication_chk;
ALTER TABLE governance.factcheck
    ADD CONSTRAINT factcheck_type_publication_chk
        CHECK (type_publication IN ('on_dit', 'adage_legende', 'fait_vecu'));

-- Contrainte sur le type de preuve (image | pdf | NULL)
ALTER TABLE governance.factcheck DROP CONSTRAINT IF EXISTS factcheck_preuve_type_chk;
ALTER TABLE governance.factcheck
    ADD CONSTRAINT factcheck_preuve_type_chk
        CHECK (preuve_type IS NULL OR preuve_type IN ('image', 'pdf'));
