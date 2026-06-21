-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — governance : IdeaForces — Nouvelles catégories de proposition
-- ════════════════════════════════════════════════════════════════════════════
-- Idempotent : exécutable sur une base déjà initialisée comme en init fraîche.
--
-- Ajoute 7 catégories de proposition (idées-forces panafricaines) :
--   union_africains, infrastructures, retour_cerveaux, union_diaspora,
--   lutte_corruption, urbanisation_durable, acces_energie
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE governance.idea_force DROP CONSTRAINT IF EXISTS idea_force_categorie_proposition_check;
ALTER TABLE governance.idea_force
    ADD CONSTRAINT idea_force_categorie_proposition_check
        CHECK (categorie_proposition IN (
            'amelioration_gouvernance',
            'education_formation',
            'sante_publique',
            'emploi_jeunes',
            'environnement',
            'transport',
            'union_africains',
            'infrastructures',
            'retour_cerveaux',
            'union_diaspora',
            'lutte_corruption',
            'urbanisation_durable',
            'acces_energie',
            'autre'
        ));
