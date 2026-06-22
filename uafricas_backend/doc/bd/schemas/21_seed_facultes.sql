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


-- ════════════════════════════════════════════════════════════════════════════
-- Extension : 7 écoles partenaires + 9 facultés supplémentaires (idempotent)
-- ════════════════════════════════════════════════════════════════════════════

-- Écoles partenaires supplémentaires
INSERT INTO exchange.ecole_partenaire (id, nom, ville, pays_id, type, site_web, email_contact, telephone_contact, whatsapp_contact) VALUES
(
    'a1000000-0000-0000-0000-000000000006',
    'Université de Yaoundé I', 'Yaoundé',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'cameroun' LIMIT 1),
    'publique', 'https://uy1.cm', 'contact@uy1.cm', '+237 222 23 44 95', NULL
),
(
    'a1000000-0000-0000-0000-000000000007',
    'Université Mohammed V', 'Rabat',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'maroc' LIMIT 1),
    'publique', 'https://um5.ac.ma', 'contact@um5.ac.ma', '+212 537 27 27 27', NULL
),
(
    'a1000000-0000-0000-0000-000000000008',
    'University of Ghana', 'Accra',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'ghana' LIMIT 1),
    'publique', 'https://ug.edu.gh', 'pad@ug.edu.gh', '+233 302 500 381', NULL
),
(
    'a1000000-0000-0000-0000-000000000009',
    'Université d''Antananarivo', 'Antananarivo',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'madagascar' LIMIT 1),
    'publique', NULL, 'contact@univ-antananarivo.mg', NULL, NULL
),
(
    'a1000000-0000-0000-0000-00000000000a',
    'Kenyatta University', 'Nairobi',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'kenya' LIMIT 1),
    'publique', 'https://ku.ac.ke', 'vc@ku.ac.ke', '+254 20 870 3000', NULL
),
(
    'a1000000-0000-0000-0000-00000000000b',
    'Université Internationale de Rabat', 'Rabat',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'maroc' LIMIT 1),
    'privee', 'https://www.uir.ac.ma', 'contact@uir.ac.ma', '+212 530 10 30 00', '+212 661 00 00 00'
),
(
    'a1000000-0000-0000-0000-00000000000c',
    'Institut Supérieur de Gestion de Tunis', 'Tunis',
    (SELECT id FROM shared.pays WHERE LOWER(nom) = 'tunisie' LIMIT 1),
    'publique', 'https://www.isg.rnu.tn', 'contact@isg.rnu.tn', '+216 71 560 313', NULL
)
ON CONFLICT (id) DO NOTHING;


-- Facultés supplémentaires (9)
INSERT INTO exchange.faculte (
    id, titre, acronyme, slug, description, image_couverture_url,
    ecole_partenaire_id, domaines_etudes,
    programmes_licence, programmes_master, programmes_doctorat, programmes_certificats,
    diplome_minimum, langues_enseignement, frais_scolarite_min, frais_scolarite_max,
    bourses_possibles, periodes_inscription, points_forts,
    accepte_nouveaux_inscrits, statut, nombre_inscrits_total, nombre_inscrits_annee
) VALUES
(
    'b1000000-0000-0000-0000-000000000006',
    'Faculté d''Agronomie et des Sciences Agricoles', 'FASA',
    'faculte-d-agronomie-et-des-sciences-agricoles',
    'La FASA forme des agronomes et ingénieurs agricoles capables de relever les défis de la souveraineté alimentaire et de l''agriculture durable en Afrique.',
    'https://images.unsplash.com/photo-1500382017468-9049fed747ef?w=800',
    'a1000000-0000-0000-0000-000000000006',
    ARRAY['Agronomie', 'Sciences du sol', 'Élevage', 'Agroéconomie', 'Foresterie'],
    ARRAY['Agronomie générale', 'Production végétale', 'Production animale'],
    ARRAY['Agroécologie', 'Gestion des ressources naturelles', 'Sécurité alimentaire'],
    ARRAY['Sciences agronomiques'],
    ARRAY['Agriculture de précision', 'Transformation agroalimentaire'],
    'Baccalauréat scientifique',
    ARRAY['Français', 'Anglais'],
    120000, 450000,
    TRUE, 'Juin - Septembre',
    ARRAY['Fermes-écoles expérimentales', 'Partenariats avec coopératives agricoles', 'Recherche sur les semences locales'],
    TRUE, 'active', 760, 190
),
(
    'b1000000-0000-0000-0000-000000000007',
    'École Nationale Supérieure d''Ingénieurs', 'ENSI',
    'ecole-nationale-superieure-d-ingenieurs',
    'L''ENSI forme des ingénieurs polyvalents en génie industriel, mécanique et procédés, au service de l''industrialisation du continent.',
    'https://images.unsplash.com/photo-1581094794329-c8112a89af12?w=800',
    'a1000000-0000-0000-0000-000000000007',
    ARRAY['Génie industriel', 'Génie mécanique', 'Génie des procédés', 'Logistique', 'Mécatronique'],
    ARRAY['Génie industriel', 'Génie mécanique'],
    ARRAY['Ingénierie des systèmes industriels', 'Supply chain management'],
    ARRAY['Sciences pour l''ingénieur'],
    ARRAY['Lean management', 'CAO / DAO'],
    'Baccalauréat scientifique',
    ARRAY['Français', 'Arabe', 'Anglais'],
    250000, 900000,
    TRUE, 'Mai - Juillet',
    ARRAY['Ateliers industriels intégrés', 'Stages en multinationales', 'Double diplôme franco-marocain'],
    TRUE, 'active', 940, 240
),
(
    'b1000000-0000-0000-0000-000000000008',
    'Faculté des Sciences de l''Éducation', 'FSE',
    'faculte-des-sciences-de-l-education',
    'La FSE forme enseignants, conseillers pédagogiques et cadres de l''éducation pour renforcer les systèmes éducatifs africains.',
    'https://images.unsplash.com/photo-1503676260728-1c00da094a0b?w=800',
    'a1000000-0000-0000-0000-000000000008',
    ARRAY['Sciences de l''éducation', 'Pédagogie', 'Psychologie', 'Didactique'],
    ARRAY['Sciences de l''éducation', 'Enseignement primaire', 'Enseignement secondaire'],
    ARRAY['Ingénierie de la formation', 'Politiques éducatives'],
    ARRAY['Sciences de l''éducation'],
    ARRAY['Pédagogie numérique', 'Gestion d''établissement'],
    'Baccalauréat',
    ARRAY['Anglais', 'Français'],
    90000, 300000,
    TRUE, 'Août - Octobre',
    ARRAY['Écoles d''application partenaires', 'Formation continue des enseignants', 'Recherche en éducation inclusive'],
    TRUE, 'active', 580, 145
),
(
    'b1000000-0000-0000-0000-000000000009',
    'École Supérieure de Journalisme et de Communication', 'ESJC',
    'ecole-superieure-de-journalisme-et-de-communication',
    'L''ESJC forme journalistes, communicants et créateurs de contenus aux métiers des médias et de l''information en Afrique.',
    'https://images.unsplash.com/photo-1504711434969-e33886168f5c?w=800',
    'a1000000-0000-0000-0000-000000000004',
    ARRAY['Journalisme', 'Communication', 'Médias numériques', 'Relations publiques'],
    ARRAY['Journalisme', 'Communication des organisations'],
    ARRAY['Communication politique', 'Médias et société'],
    ARRAY[]::TEXT[],
    ARRAY['Production audiovisuelle', 'Community management', 'Data journalisme'],
    'Baccalauréat',
    ARRAY['Français'],
    110000, 380000,
    TRUE, 'Septembre - Octobre',
    ARRAY['Studio radio et TV intégré', 'Rédaction-école en ligne', 'Réseau d''anciens dans les grands médias'],
    TRUE, 'active', 410, 120
),
(
    'b1000000-0000-0000-0000-00000000000a',
    'Faculté de l''Environnement et du Développement Durable', 'FEDD',
    'faculte-de-l-environnement-et-du-developpement-durable',
    'La FEDD forme des spécialistes de la gestion environnementale, de la biodiversité et des politiques climatiques adaptées aux écosystèmes africains.',
    'https://images.unsplash.com/photo-1542601906990-b4d3fb778b09?w=800',
    'a1000000-0000-0000-0000-000000000009',
    ARRAY['Environnement', 'Écologie', 'Climat', 'Gestion des déchets', 'Énergies renouvelables'],
    ARRAY['Sciences de l''environnement', 'Gestion des écosystèmes'],
    ARRAY['Changement climatique', 'Aménagement durable du territoire'],
    ARRAY['Sciences de l''environnement'],
    ARRAY['Évaluation d''impact environnemental', 'Économie circulaire'],
    'Baccalauréat scientifique',
    ARRAY['Français', 'Malgache'],
    95000, 320000,
    TRUE, 'Juillet - Septembre',
    ARRAY['Station de terrain en aire protégée', 'Projets de reforestation étudiants', 'Partenariats avec ONG environnementales'],
    TRUE, 'active', 360, 95
),
(
    'b1000000-0000-0000-0000-00000000000b',
    'École de Commerce International et d''Entrepreneuriat', 'ECIE',
    'ecole-de-commerce-international-et-d-entrepreneuriat',
    'L''ECIE forme des entrepreneurs et managers du commerce international, avec un focus sur les marchés panafricains et la ZLECAf.',
    'https://images.unsplash.com/photo-1521737604893-d14cc237f11d?w=800',
    'a1000000-0000-0000-0000-00000000000a',
    ARRAY['Commerce international', 'Entrepreneuriat', 'Marketing', 'Logistique', 'Finance'],
    ARRAY['Commerce international', 'Gestion'],
    ARRAY['Business development Afrique', 'Entrepreneuriat & innovation'],
    ARRAY[]::TEXT[],
    ARRAY['Création d''entreprise', 'Négoce international', 'E-commerce'],
    'Baccalauréat',
    ARRAY['Anglais', 'Français'],
    180000, 700000,
    TRUE, 'Juin - Août',
    ARRAY['Incubateur d''entreprises intégré', 'Réseau d''investisseurs partenaires', 'Immersion sur les marchés régionaux'],
    TRUE, 'active', 720, 200
),
(
    'b1000000-0000-0000-0000-00000000000c',
    'Institut des Technologies Numériques et de l''Intelligence Artificielle', 'ITNIA',
    'institut-des-technologies-numeriques-et-de-l-intelligence-artificielle',
    'L''ITNIA forme les talents du numérique africain : data science, intelligence artificielle, cybersécurité et développement logiciel.',
    'https://images.unsplash.com/photo-1518770660439-4636190af475?w=800',
    'a1000000-0000-0000-0000-00000000000b',
    ARRAY['Intelligence artificielle', 'Data science', 'Cybersécurité', 'Développement logiciel', 'Cloud'],
    ARRAY['Informatique', 'Sciences des données'],
    ARRAY['Intelligence artificielle', 'Cybersécurité', 'Ingénierie des données'],
    ARRAY['Informatique'],
    ARRAY['Machine learning', 'DevOps', 'Sécurité offensive'],
    'Baccalauréat scientifique',
    ARRAY['Français', 'Anglais'],
    300000, 1200000,
    TRUE, 'Mai - Septembre',
    ARRAY['Cluster GPU pour l''IA', 'Certifications professionnelles incluses', 'Hackathons et partenariats tech'],
    TRUE, 'active', 1020, 310
),
(
    'b1000000-0000-0000-0000-00000000000d',
    'Faculté d''Agroalimentaire et de Nutrition', 'FAN',
    'faculte-d-agroalimentaire-et-de-nutrition',
    'La FAN forme des experts en transformation alimentaire, qualité et nutrition pour valoriser les filières agricoles africaines.',
    'https://images.unsplash.com/photo-1490818387583-1baba5e638af?w=800',
    'a1000000-0000-0000-0000-00000000000c',
    ARRAY['Agroalimentaire', 'Nutrition', 'Qualité', 'Biotechnologie', 'Sécurité sanitaire'],
    ARRAY['Sciences alimentaires', 'Nutrition humaine'],
    ARRAY['Technologie agroalimentaire', 'Qualité et sécurité des aliments'],
    ARRAY['Sciences alimentaires'],
    ARRAY['HACCP', 'Analyse sensorielle'],
    'Baccalauréat scientifique',
    ARRAY['Français', 'Arabe'],
    140000, 520000,
    TRUE, 'Juin - Septembre',
    ARRAY['Halle technologique agroalimentaire', 'Laboratoire d''analyse accrédité', 'Partenariats avec l''industrie alimentaire'],
    TRUE, 'active', 480, 130
),
(
    'b1000000-0000-0000-0000-00000000000e',
    'École Supérieure de Tourisme et d''Hôtellerie', 'ESTH',
    'ecole-superieure-de-tourisme-et-d-hotellerie',
    'L''ESTH forme les cadres du tourisme, de l''hôtellerie et de la valorisation du patrimoine, secteurs clés de l''économie africaine.',
    'https://images.unsplash.com/photo-1551882547-ff40c63fe5fa?w=800',
    'a1000000-0000-0000-0000-00000000000c',
    ARRAY['Tourisme', 'Hôtellerie', 'Restauration', 'Patrimoine', 'Événementiel'],
    ARRAY['Management hôtelier', 'Tourisme durable'],
    ARRAY['Management du tourisme', 'Ingénierie touristique'],
    ARRAY[]::TEXT[],
    ARRAY['Accueil et conciergerie', 'Œnologie & arts de la table', 'Guide-conférencier'],
    'Baccalauréat',
    ARRAY['Français', 'Anglais', 'Arabe'],
    130000, 480000,
    FALSE, 'Juillet - Octobre',
    ARRAY['Hôtel-école d''application', 'Stages dans des établissements 5 étoiles', 'Réseau international de partenaires'],
    TRUE, 'active', 390, 0
)
ON CONFLICT (id) DO NOTHING;
