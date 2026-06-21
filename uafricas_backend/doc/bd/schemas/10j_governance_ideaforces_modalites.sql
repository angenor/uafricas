-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — governance : IdeaForces — Modalités opérationnelles
-- ════════════════════════════════════════════════════════════════════════════
-- Idempotent : exécutable sur une base déjà initialisée comme en init fraîche.
--
-- Ajoute le champ « Modalités opérationnelles concrètes proposées » : une liste
-- ordonnée de 10 étapes maximum décrivant la mise en œuvre d'une idée-force.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE governance.idea_force
    ADD COLUMN IF NOT EXISTS modalites_operationnelles TEXT[];

ALTER TABLE governance.idea_force DROP CONSTRAINT IF EXISTS idea_force_modalites_max10;
ALTER TABLE governance.idea_force
    ADD CONSTRAINT idea_force_modalites_max10
        CHECK (modalites_operationnelles IS NULL
               OR cardinality(modalites_operationnelles) <= 10);
