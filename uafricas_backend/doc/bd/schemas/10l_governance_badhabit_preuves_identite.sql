-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — governance : Bad Habit — Preuves photos, solutions, identité
-- ════════════════════════════════════════════════════════════════════════════
-- Idempotent : exécutable sur une base déjà initialisée comme en init fraîche.
--
-- Pour une mauvaise pratique, le signalement n'est jamais anonyme : l'auteur
-- partage obligatoirement son identité réelle (nom/prénom à l'état civil,
-- courriel, contact). On distingue désormais :
--   • les témoignages (texte) → colonne existante `preuves_temoignages`
--   • les preuves (photos)     → nouvelle colonne `preuves_photos TEXT[]`
-- Les solutions proposées deviennent une liste (10 propositions maximum).
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE governance.bad_habit
    ADD COLUMN IF NOT EXISTS preuves_photos         TEXT[],
    ADD COLUMN IF NOT EXISTS solutions_propositions TEXT[],
    ADD COLUMN IF NOT EXISTS identite_nom           VARCHAR(150),
    ADD COLUMN IF NOT EXISTS identite_prenom        VARCHAR(150),
    ADD COLUMN IF NOT EXISTS identite_courriel      VARCHAR(255),
    ADD COLUMN IF NOT EXISTS identite_contact       VARCHAR(50);

ALTER TABLE governance.bad_habit DROP CONSTRAINT IF EXISTS bad_habit_preuves_photos_max;
ALTER TABLE governance.bad_habit
    ADD CONSTRAINT bad_habit_preuves_photos_max
        CHECK (preuves_photos IS NULL OR cardinality(preuves_photos) <= 5);

ALTER TABLE governance.bad_habit DROP CONSTRAINT IF EXISTS bad_habit_solutions_max;
ALTER TABLE governance.bad_habit
    ADD CONSTRAINT bad_habit_solutions_max
        CHECK (solutions_propositions IS NULL OR cardinality(solutions_propositions) <= 10);
