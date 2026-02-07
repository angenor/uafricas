-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Seed : Facultés INUDA (écoles partenaires + facultés)
-- ════════════════════════════════════════════════════════════════════════════


-- Écoles partenaires
INSERT INTO exchange.ecole_partenaire (id, nom, ville, pays_id, type, site_web, email_contact, telephone_contact) VALUES
(
    'a1000000-0000-0000-0000-000000000001',
    'Université Cheikh Anta Diop',
    'Dakar',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'sénégal' LIMIT 1),
    'publique',
    'https://ucad.sn',
    'faseg@ucad.sn',
    '+221 33 825 00 00'
),
(
    'a1000000-0000-0000-0000-000000000002',
    'Institut National Polytechnique',
    'Yamoussoukro',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'côte d''ivoire' LIMIT 1),
    'publique',
    'https://inphb.ci',
    'contact@inphb.ci',
    '+225 30 64 01 00'
),
(
    'a1000000-0000-0000-0000-000000000003',
    'Université de Ouagadougou',
    'Ouagadougou',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'burkina faso' LIMIT 1),
    'publique',
    NULL,
    'esdsp@univ-ouaga.bf',
    NULL
),
(
    'a1000000-0000-0000-0000-000000000004',
    'Université de Lomé',
    'Lomé',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'togo' LIMIT 1),
    'publique',
    NULL,
    'fss@univ-lome.tg',
    '+228 22 21 35 00'
),
(
    'a1000000-0000-0000-0000-000000000005',
    'Institut National des Arts',
    'Bamako',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'mali' LIMIT 1),
    'publique',
    NULL,
    'eaca@ina-mali.ml',
    NULL
);


-- Facultés
INSERT INTO exchange.faculte (
    id, titre, acronyme, slug, description, image_couverture_url,
    ecole_partenaire_id, domaines_etudes,
    programmes_licence, programmes_master, programmes_doctorat, programmes_certificats,
    diplome_minimum, langues_enseignement, frais_scolarite_min, frais_scolarite_max,
    bourses_possibles, periodes_inscription, points_forts,
    accepte_nouveaux_inscrits, statut, nombre_inscrits_total, nombre_inscrits_annee
) VALUES
(
    'b1000000-0000-0000-0000-000000000001',
    'Faculté des Sciences Économiques et de Gestion',
    'FASEG',
    'faculte-des-sciences-economiques-et-de-gestion',
    'La FASEG forme les futurs leaders économiques africains avec des programmes innovants en économie, finance et management adaptés aux réalités du continent.',
    'https://images.unsplash.com/photo-1454165804606-c3d57bc86b40?w=800',
    'a1000000-0000-0000-0000-000000000001',
    ARRAY['Économie', 'Finance', 'Management', 'Comptabilité', 'Marketing'],
    ARRAY['Économie', 'Gestion des entreprises', 'Finance'],
    ARRAY['MBA', 'Finance internationale', 'Économie du développement'],
    ARRAY['Sciences économiques'],
    ARRAY['Gestion de projet', 'Analyse financière'],
    'Baccalauréat',
    ARRAY['Français', 'Anglais'],
    150000, 500000,
    TRUE,
    'Juillet - Septembre',
    ARRAY['Partenariats internationaux', 'Stages en entreprise garantis', 'Formation pratique orientée marché', 'Incubateur de startups'],
    TRUE, 'active', 1250, 320
),
(
    'b1000000-0000-0000-0000-000000000002',
    'Institut Polytechnique de Technologie',
    'IPT',
    'institut-polytechnique-de-technologie',
    'L''IPT est une école d''ingénieurs de premier plan formant des experts en technologie, informatique et télécommunications pour l''Afrique de demain.',
    'https://images.unsplash.com/photo-1581092918056-0c4c3acd3789?w=800',
    'a1000000-0000-0000-0000-000000000002',
    ARRAY['Informatique', 'Génie civil', 'Électronique', 'Télécommunications', 'Énergie'],
    ARRAY['Informatique', 'Génie électrique', 'Génie civil'],
    ARRAY['Ingénierie logicielle', 'Cybersécurité', 'Énergies renouvelables'],
    ARRAY['Sciences de l''ingénieur'],
    ARRAY['Développement web', 'Administration système', 'IoT'],
    'Baccalauréat scientifique',
    ARRAY['Français'],
    200000, 800000,
    TRUE,
    'Juin - Août',
    ARRAY['Laboratoires équipés dernière génération', 'Double diplôme avec universités européennes', 'Taux d''insertion professionnelle de 95%', 'Recherche appliquée'],
    TRUE, 'active', 890, 215
),
(
    'b1000000-0000-0000-0000-000000000003',
    'École Supérieure de Droit et Sciences Politiques',
    'ESDSP',
    'ecole-superieure-de-droit-et-sciences-politiques',
    'L''ESDSP forme les juristes et politologues de demain avec une approche comparative du droit africain et international.',
    'https://images.unsplash.com/photo-1589829545856-d10d557cf95f?w=800',
    'a1000000-0000-0000-0000-000000000003',
    ARRAY['Droit', 'Sciences politiques', 'Relations internationales', 'Administration publique'],
    ARRAY['Droit privé', 'Droit public', 'Sciences politiques'],
    ARRAY['Droit des affaires', 'Droit international', 'Gouvernance publique'],
    ARRAY['Droit', 'Sciences politiques'],
    ARRAY['Médiation', 'Droit OHADA'],
    'Baccalauréat',
    ARRAY['Français'],
    100000, 350000,
    TRUE,
    'Août - Octobre',
    ARRAY['Clinique juridique pour les étudiants', 'Moot courts internationaux', 'Partenariats avec organisations internationales'],
    FALSE, 'active', 670, 0
),
(
    'b1000000-0000-0000-0000-000000000004',
    'Faculté des Sciences de la Santé',
    'FSS',
    'faculte-des-sciences-de-la-sante',
    'La FSS forme les professionnels de santé de qualité pour répondre aux défis sanitaires du continent africain.',
    'https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?w=800',
    'a1000000-0000-0000-0000-000000000004',
    ARRAY['Médecine', 'Pharmacie', 'Sciences infirmières', 'Santé publique'],
    ARRAY['Sciences infirmières', 'Sage-femme'],
    ARRAY['Santé publique', 'Épidémiologie'],
    ARRAY['Médecine', 'Pharmacie'],
    ARRAY['Urgences médicales', 'Gestion hospitalière'],
    'Baccalauréat scientifique',
    ARRAY['Français'],
    300000, 1500000,
    TRUE,
    'Mai - Juillet',
    ARRAY['CHU universitaire intégré', 'Équipements médicaux modernes', 'Programmes de spécialisation'],
    TRUE, 'active', 1100, 180
),
(
    'b1000000-0000-0000-0000-000000000005',
    'École des Arts et de la Culture Africaine',
    'EACA',
    'ecole-des-arts-et-de-la-culture-africaine',
    'L''EACA célèbre et perpétue les arts africains tout en formant les créateurs et managers culturels de demain.',
    'https://images.unsplash.com/photo-1578926288207-a90a5366759d?w=800',
    'a1000000-0000-0000-0000-000000000005',
    ARRAY['Arts visuels', 'Musique', 'Danse', 'Cinéma', 'Management culturel'],
    ARRAY['Arts plastiques', 'Musique', 'Arts du spectacle'],
    ARRAY['Direction artistique', 'Production cinématographique', 'Management culturel'],
    ARRAY[]::TEXT[],
    ARRAY['Photographie', 'Design graphique', 'Sound design'],
    'Baccalauréat ou équivalent artistique',
    ARRAY['Français', 'Bambara'],
    80000, 250000,
    TRUE,
    'Septembre - Novembre',
    ARRAY['Studios et ateliers professionnels', 'Festivals étudiants annuels', 'Résidences d''artistes'],
    TRUE, 'active', 340, 85
);
