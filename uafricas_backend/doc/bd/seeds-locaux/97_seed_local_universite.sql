-- ============================================================================
-- Seed LOCAL : écoles partenaires et facultés (INUDA)
-- ============================================================================
--
-- Hors de `schemas/` : ce fichier n'est PAS inclus par `schema.sql`, il n'est
-- donc jamais déployé. Il sert à juger `/universite/facultes`, qui n'affichait
-- rien parce que `exchange.faculte` est réellement vide en base.
--
-- Idempotent : chaque insertion est gardée par un `NOT EXISTS` sur le nom, la
-- table n'ayant pas de contrainte d'unicité dessus.
-- ============================================================================

INSERT INTO exchange.ecole_partenaire (nom, ville, pays_id, type, site_web, email_contact, telephone_contact)
SELECT v.nom, v.ville, p.id, v.type, v.site_web, v.email, v.tel
FROM (VALUES
    ('Université Félix Houphouët-Boigny', 'Abidjan',  'Côte d''Ivoire', 'publique',
     'https://www.univ-fhb.edu.ci', 'contact@univ-fhb.edu.ci', '+225 27 22 44 08 95'),
    ('Université Cheikh Anta Diop',       'Dakar',    'Sénégal',       'publique',
     'https://www.ucad.sn',        'contact@ucad.sn',        '+221 33 825 05 30'),
    ('Université Kwame Nkrumah',          'Kumasi',   'Ghana',         'publique',
     'https://www.knust.edu.gh',   'contact@knust.edu.gh',   '+233 32 206 0331'),
    ('Institut Africain de Management',   'Dakar',    'Sénégal',       'privee',
     'https://www.iam.sn',         'contact@iam.sn',         '+221 33 869 61 61')
) AS v(nom, ville, pays_nom, type, site_web, email, tel)
JOIN shared.pays p ON p.nom = v.pays_nom
WHERE NOT EXISTS (SELECT 1 FROM exchange.ecole_partenaire e WHERE e.nom = v.nom);

INSERT INTO exchange.faculte (
    titre, acronyme, slug, description, ecole_partenaire_id,
    domaines_etudes, programmes_licence, programmes_master, programmes_doctorat,
    diplome_minimum, langues_enseignement, frais_scolarite_min, frais_scolarite_max,
    bourses_possibles, periodes_inscription, points_forts, accepte_nouveaux_inscrits
)
SELECT v.titre, v.acronyme, v.slug, v.description, e.id,
       v.domaines, v.licence, v.master, v.doctorat,
       v.diplome, v.langues, v.frais_min, v.frais_max,
       v.bourses, v.periodes, v.forts, v.ouvert
FROM (VALUES
    ('Faculté des Sciences Économiques et de Gestion', 'FASEG', 'faseg-abidjan',
     'Formation aux métiers de l''économie, de la gestion et de la finance, avec un ancrage sur les enjeux de développement du continent.',
     'Université Félix Houphouët-Boigny',
     ARRAY['Économie', 'Gestion', 'Finance'],
     ARRAY['Licence en Sciences Économiques', 'Licence en Gestion'],
     ARRAY['Master Finance et Banque', 'Master Management des Organisations'],
     ARRAY['Doctorat en Sciences Économiques'],
     'Baccalauréat', ARRAY['Français', 'Anglais'], 150000, 450000,
     TRUE, 'Juillet à septembre',
     ARRAY['Partenariats bancaires', 'Stages en entreprise garantis'], TRUE),

    ('Faculté des Lettres, Arts et Sciences Humaines', 'FLASH', 'flash-dakar',
     'Langues africaines, histoire, philosophie et anthropologie, au service de la transmission des patrimoines.',
     'Université Cheikh Anta Diop',
     ARRAY['Lettres', 'Histoire', 'Anthropologie'],
     ARRAY['Licence en Lettres Modernes', 'Licence en Histoire'],
     ARRAY['Master Langues et Cultures Africaines'],
     ARRAY['Doctorat en Anthropologie'],
     'Baccalauréat', ARRAY['Français', 'Wolof'], 80000, 250000,
     TRUE, 'Juin à août',
     ARRAY['Fonds documentaire sur les langues nationales', 'Terrain de recherche'], TRUE),

    ('College of Engineering', 'COE', 'coe-kumasi',
     'Génie civil, génie électrique et informatique, avec des laboratoires ouverts aux projets étudiants.',
     'Université Kwame Nkrumah',
     ARRAY['Ingénierie', 'Informatique'],
     ARRAY['BSc Civil Engineering', 'BSc Computer Engineering'],
     ARRAY['MSc Renewable Energy'],
     ARRAY['PhD Engineering'],
     'Baccalauréat scientifique', ARRAY['Anglais'], 300000, 900000,
     FALSE, 'Mai à juillet',
     ARRAY['Laboratoires équipés', 'Incubateur intégré'], TRUE),

    ('École de Management et de Commerce', 'EMC', 'emc-dakar',
     'Formation professionnalisante en commerce international, marketing et entrepreneuriat.',
     'Institut Africain de Management',
     ARRAY['Management', 'Commerce', 'Entrepreneuriat'],
     ARRAY['Licence Commerce International', 'Licence Marketing'],
     ARRAY['MBA Entrepreneuriat'],
     ARRAY[]::text[],
     'Baccalauréat', ARRAY['Français', 'Anglais'], 900000, 2500000,
     TRUE, 'Toute l''année',
     ARRAY['Réseau d''anciens actif', 'Double diplôme européen'], FALSE)
) AS v(titre, acronyme, slug, description, ecole_nom, domaines, licence, master,
       doctorat, diplome, langues, frais_min, frais_max, bourses, periodes, forts, ouvert)
JOIN exchange.ecole_partenaire e ON e.nom = v.ecole_nom
WHERE NOT EXISTS (SELECT 1 FROM exchange.faculte f WHERE f.slug = v.slug);
