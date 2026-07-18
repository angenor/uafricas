-- ============================================================================
-- 09i — media_content : formations enrichies (objectif, présentation,
--       évaluation finale & certification)
-- ----------------------------------------------------------------------------
-- Ajoute à media_content.mooc les champs éditoriaux affichés sur la page
-- détail publique :
--   • objectif        — objectifs pédagogiques de la formation ;
--   • presentation    — texte de présentation détaillée (distinct de la
--                       description courte utilisée dans les listes) ;
--   • a_evaluation    — la formation se termine-t-elle par une évaluation ? ;
--   • est_certifiante — délivre-t-elle une certification ?
--
-- Les intervenants sont gérés dans une itération ultérieure (affichés en
-- placeholder côté front pour l'instant).
--
-- Migration idempotente (ADD COLUMN IF NOT EXISTS).
-- ============================================================================

ALTER TABLE media_content.mooc
    ADD COLUMN IF NOT EXISTS objectif        TEXT,
    ADD COLUMN IF NOT EXISTS presentation    TEXT,
    ADD COLUMN IF NOT EXISTS a_evaluation    BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN IF NOT EXISTS est_certifiante BOOLEAN NOT NULL DEFAULT false;
