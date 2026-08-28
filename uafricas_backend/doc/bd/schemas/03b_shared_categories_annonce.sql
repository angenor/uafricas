-- ════════════════════════════════════════════════════════════════════════════
-- 03b — Catégories du marché (Afromarket)
-- ════════════════════════════════════════════════════════════════════════════
--
-- `shared.categorie` ne portait AUCUNE ligne de contexte 'annonce', alors que
-- `GET /api/annonces/categories` la lit et que la catégorie est obligatoire à
-- la publication : le sélecteur du formulaire était vide, et publier une
-- annonce était impossible. La barre de filtres, elle, affichait six
-- catégories — elles venaient d'une constante du frontend, sans contrepartie
-- en base, si bien que tout filtre par catégorie ne renvoyait rien.
--
-- Les six valeurs reprises ici sont EXACTEMENT celles que le frontend
-- annonçait déjà : la base rattrape l'interface, l'inverse aurait retiré des
-- catégories aux utilisateurs.
--
-- Le slug est préfixé `annonce-` : `shared.categorie.slug` est UNIQUE sur
-- TOUS les contextes, et « agriculture » est aussi un secteur d'activité.
-- C'est la convention déjà suivie par les 44 thèmes médias de 09j.
--
-- Idempotent : `ON CONFLICT (slug) DO NOTHING` permet de rejouer le fichier
-- sur une base déjà initialisée sans la casser.
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO shared.categorie (nom, slug, contexte, ordre) VALUES
    ('Agriculture',  'annonce-agriculture',  'annonce', 1),
    ('Informatique', 'annonce-informatique', 'annonce', 2),
    ('Immobilier',   'annonce-immobilier',   'annonce', 3),
    ('Voitures',     'annonce-voitures',     'annonce', 4),
    ('Électronique', 'annonce-electronique', 'annonce', 5),
    ('Formation',    'annonce-formation',    'annonce', 6)
ON CONFLICT (slug) DO NOTHING;
