-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : culture -- Image illustrative des programmations
-- Feature : centres-culturels-images (2026-06-21)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Ajoute une image illustrative (couverture) aux programmations de centre
-- culturel. La colonne `image_couverture_url` du centre culturel existe déjà ;
-- seule la programmation en manquait. Colonne nullable → rétrocompatibilité.
-- Idempotent.
--
-- Conformité : Principe III (SQL source de vérité).
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE culture.programmation_centre
    ADD COLUMN IF NOT EXISTS image_couverture_url VARCHAR(500);
