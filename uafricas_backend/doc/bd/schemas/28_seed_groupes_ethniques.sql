-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Seed groupes ethniques / langues locales par pays
-- Référentiel ethno-linguistique africain (54 pays, ~280 entrées)
-- ════════════════════════════════════════════════════════════════════════════
-- Idempotent : ON CONFLICT DO NOTHING sur pays.code_iso2 et fiche_pays.pays_id ;
-- WHERE NOT EXISTS sur (fiche_pays_id, lower(nom)) pour groupe_ethnique.

BEGIN;

-- ── 1) Pays : insérer les manquants (UNIQUE sur code_iso2) ────────────────
INSERT INTO shared.pays (nom, code_iso2, continent, actif) VALUES
    ('Algérie',                          'DZ', 'Afrique', TRUE),
    ('Angola',                           'AO', 'Afrique', TRUE),
    ('Bénin',                            'BJ', 'Afrique', TRUE),
    ('Botswana',                         'BW', 'Afrique', TRUE),
    ('Burundi',                          'BI', 'Afrique', TRUE),
    ('Cap-Vert',                         'CV', 'Afrique', TRUE),
    ('République centrafricaine',        'CF', 'Afrique', TRUE),
    ('Tchad',                            'TD', 'Afrique', TRUE),
    ('Comores',                          'KM', 'Afrique', TRUE),
    ('République du Congo',              'CG', 'Afrique', TRUE),
    ('Djibouti',                         'DJ', 'Afrique', TRUE),
    ('Guinée équatoriale',               'GQ', 'Afrique', TRUE),
    ('Érythrée',                         'ER', 'Afrique', TRUE),
    ('Eswatini',                         'SZ', 'Afrique', TRUE),
    ('Gabon',                            'GA', 'Afrique', TRUE),
    ('Gambie',                           'GM', 'Afrique', TRUE),
    ('Guinée-Bissau',                    'GW', 'Afrique', TRUE),
    ('Lesotho',                          'LS', 'Afrique', TRUE),
    ('Libéria',                          'LR', 'Afrique', TRUE),
    ('Libye',                            'LY', 'Afrique', TRUE),
    ('Madagascar',                       'MG', 'Afrique', TRUE),
    ('Malawi',                           'MW', 'Afrique', TRUE),
    ('Mauritanie',                       'MR', 'Afrique', TRUE),
    ('Maurice',                          'MU', 'Afrique', TRUE),
    ('Mozambique',                       'MZ', 'Afrique', TRUE),
    ('Namibie',                          'NA', 'Afrique', TRUE),
    ('Niger',                            'NE', 'Afrique', TRUE),
    ('Sao Tomé-et-Principe',             'ST', 'Afrique', TRUE),
    ('Seychelles',                       'SC', 'Afrique', TRUE),
    ('Sierra Leone',                     'SL', 'Afrique', TRUE),
    ('Somalie',                          'SO', 'Afrique', TRUE),
    ('Soudan du Sud',                    'SS', 'Afrique', TRUE),
    ('Soudan',                           'SD', 'Afrique', TRUE),
    ('Togo',                             'TG', 'Afrique', TRUE),
    ('Tunisie',                          'TN', 'Afrique', TRUE),
    ('Ouganda',                          'UG', 'Afrique', TRUE),
    ('Zambie',                           'ZM', 'Afrique', TRUE),
    ('Zimbabwe',                         'ZW', 'Afrique', TRUE)
ON CONFLICT (code_iso2) DO NOTHING;

-- ── 2) Fiche pays minimale (UNIQUE sur pays_id) ───────────────────────────
INSERT INTO country_profile.fiche_pays (pays_id, cree_par)
SELECT p.id, (SELECT id FROM iam.utilisateur ORDER BY created_at LIMIT 1)
FROM shared.pays p
WHERE p.actif = TRUE
  AND p.code_iso2 IN (
      'DZ','AO','BJ','BW','BF','BI','CV','CM','CF','TD','KM','CG','CD','CI',
      'DJ','EG','GQ','ER','ET','SZ','GA','GM','GH','GN','GW','KE','LS','LR',
      'LY','MG','MW','ML','MR','MU','MA','MZ','NA','NE','NG','RW','ST','SN',
      'SC','SL','SO','ZA','SS','SD','TZ','TG','TN','UG','ZM','ZW'
  )
ON CONFLICT (pays_id) DO NOTHING;

-- ── 3) Groupes ethniques / langues (idempotent) ───────────────────────────
INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom, langues)
SELECT fp.id, src.nom, NULLIF(src.langues, '')
FROM (VALUES
    -- Algérie
    ('DZ', 'Kabyle', ''), ('DZ', 'Chaoui', ''), ('DZ', 'Chenoui', ''),
    ('DZ', 'Mozabite', ''), ('DZ', 'Touareg', 'Tamasheq'),
    ('DZ', 'Tamahaq', ''), ('DZ', 'Tamazight de l''Atlas saharien', ''),

    -- Angola
    ('AO', 'Umbundu', ''), ('AO', 'Kimbundu', ''), ('AO', 'Kikongo', ''),
    ('AO', 'Chokwe', ''), ('AO', 'Kwanyama', ''), ('AO', 'Ngangela', ''),
    ('AO', 'Fiote', ''), ('AO', 'Lunda', ''),

    -- Bénin
    ('BJ', 'Fon', ''), ('BJ', 'Yoruba', ''), ('BJ', 'Bariba', ''),
    ('BJ', 'Dendi', ''), ('BJ', 'Gun', ''), ('BJ', 'Ditammari', ''),
    ('BJ', 'Yom', ''), ('BJ', 'Fulfulde', ''), ('BJ', 'Idaasha', ''),
    ('BJ', 'Anii', ''),

    -- Botswana
    ('BW', 'Setswana', ''), ('BW', 'Kalanga', ''), ('BW', 'Herero', ''),
    ('BW', 'Subiya', ''), ('BW', 'Yeyi', ''), ('BW', 'Mbukushu', ''),
    ('BW', '!Xóõ', ''), ('BW', 'Naro', ''),

    -- Burkina Faso
    ('BF', 'Mooré', ''), ('BF', 'Dioula', ''), ('BF', 'Fulfulde', ''),
    ('BF', 'Gourmantchéma', ''), ('BF', 'Bissa', ''), ('BF', 'Lobi', ''),
    ('BF', 'Dagara', ''), ('BF', 'Sénoufo', ''), ('BF', 'Bwamu', ''),

    -- Burundi
    ('BI', 'Kirundi', ''), ('BI', 'Swahili', ''), ('BI', 'Ha', ''), ('BI', 'Twa', ''),

    -- Cap-Vert
    ('CV', 'Créole capverdien', 'Santiago, São Vicente, Fogo, Santo Antão'),

    -- Cameroun
    ('CM', 'Ewondo', ''), ('CM', 'Duala', ''), ('CM', 'Bassa', ''),
    ('CM', 'Bamileke', 'Ghomala, Fe''fe'', Medumba'),
    ('CM', 'Fulfulde', ''), ('CM', 'Tikar', ''), ('CM', 'Toupouri', ''),
    ('CM', 'Fang', ''), ('CM', 'Mafa', ''), ('CM', 'Gbaya', ''),

    -- République centrafricaine
    ('CF', 'Sango', ''), ('CF', 'Banda', ''), ('CF', 'Gbaya', ''),
    ('CF', 'Ngbaka', ''), ('CF', 'Zande', ''), ('CF', 'Kare', ''),

    -- Tchad
    ('TD', 'Sara', ''), ('TD', 'Kanembu', ''), ('TD', 'Maba', ''),
    ('TD', 'Teda', ''), ('TD', 'Daza', ''), ('TD', 'Fulfulde', ''),
    ('TD', 'Masa', ''), ('TD', 'Ngambay', ''),

    -- Comores
    ('KM', 'Shikomori', 'Ngazidja, Ndzuwani, Mwali, Maore'),

    -- République du Congo
    ('CG', 'Lingala', ''), ('CG', 'Kituba', ''), ('CG', 'Teke', ''),
    ('CG', 'Lari', ''), ('CG', 'Mboshi', ''), ('CG', 'Kongo', ''),

    -- République démocratique du Congo
    ('CD', 'Lingala', ''), ('CD', 'Swahili', ''), ('CD', 'Kikongo', ''),
    ('CD', 'Tshiluba', ''), ('CD', 'Luba-Katanga', ''), ('CD', 'Mongo', ''),
    ('CD', 'Zande', ''), ('CD', 'Ngbaka', ''), ('CD', 'Tetela', ''),
    ('CD', 'Hemba', ''),

    -- Côte d'Ivoire
    ('CI', 'Baoulé', ''), ('CI', 'Bété', ''), ('CI', 'Sénoufo', ''),
    ('CI', 'Dioula', ''), ('CI', 'Agni', ''), ('CI', 'Attié', ''),
    ('CI', 'Yacouba', 'Dan'), ('CI', 'Gouro', ''), ('CI', 'Dida', ''),

    -- Djibouti
    ('DJ', 'Somali', ''), ('DJ', 'Afar', ''), ('DJ', 'Saho', ''),

    -- Égypte
    ('EG', 'Nobiin', ''), ('EG', 'Kenuzi-Dongola', ''),
    ('EG', 'Beja', ''), ('EG', 'Siwi', ''),

    -- Guinée équatoriale
    ('GQ', 'Fang', ''), ('GQ', 'Bubi', ''), ('GQ', 'Ndowe', ''),
    ('GQ', 'Annobonese', ''), ('GQ', 'Benga', ''),

    -- Érythrée
    ('ER', 'Tigré', ''), ('ER', 'Afar', ''), ('ER', 'Saho', ''),
    ('ER', 'Bilen', ''), ('ER', 'Kunama', ''), ('ER', 'Nara', ''),
    ('ER', 'Tigre', ''),

    -- Éthiopie
    ('ET', 'Oromo', ''), ('ET', 'Amharique', ''), ('ET', 'Somali', ''),
    ('ET', 'Tigrinya', ''), ('ET', 'Afar', ''), ('ET', 'Sidama', ''),
    ('ET', 'Wolaytta', ''), ('ET', 'Gurage', ''), ('ET', 'Hadiyya', ''),

    -- Eswatini
    ('SZ', 'Siswati', ''),

    -- Gabon
    ('GA', 'Fang', ''), ('GA', 'Myene', ''), ('GA', 'Punu', ''),
    ('GA', 'Teke', ''), ('GA', 'Nzebi', ''), ('GA', 'Kota', ''),
    ('GA', 'Obamba', ''),

    -- Gambie
    ('GM', 'Wolof', ''), ('GM', 'Mandinka', ''), ('GM', 'Fula', ''),
    ('GM', 'Jola', ''), ('GM', 'Serer', ''), ('GM', 'Manjak', ''),

    -- Ghana
    ('GH', 'Akan', 'Twi, Fante'), ('GH', 'Ewe', ''), ('GH', 'Dagbani', ''),
    ('GH', 'Ga', ''), ('GH', 'Nzema', ''), ('GH', 'Gonja', ''),
    ('GH', 'Kasem', ''), ('GH', 'Dagaare', ''),

    -- Guinée
    ('GN', 'Malinké', ''), ('GN', 'Susu', ''), ('GN', 'Pular', ''),
    ('GN', 'Kissi', ''), ('GN', 'Kpelle', ''), ('GN', 'Toma', ''),
    ('GN', 'Baga', ''),

    -- Guinée-Bissau
    ('GW', 'Créole bissau-guinéen', ''), ('GW', 'Balanta', ''), ('GW', 'Fula', ''),
    ('GW', 'Mandingue', ''), ('GW', 'Papel', ''), ('GW', 'Manjaco', ''),
    ('GW', 'Bijago', ''),

    -- Kenya
    ('KE', 'Kikuyu', ''), ('KE', 'Luhya', ''), ('KE', 'Luo', ''),
    ('KE', 'Kamba', ''), ('KE', 'Kalenjin', ''), ('KE', 'Maasai', ''),
    ('KE', 'Meru', ''), ('KE', 'Turkana', ''), ('KE', 'Samburu', ''),

    -- Lesotho
    ('LS', 'Sesotho', ''),

    -- Libéria
    ('LR', 'Kpelle', ''), ('LR', 'Bassa', ''), ('LR', 'Gio', ''),
    ('LR', 'Kru', ''), ('LR', 'Grebo', ''), ('LR', 'Vai', ''),
    ('LR', 'Gola', ''), ('LR', 'Mano', ''),

    -- Libye
    ('LY', 'Tamazight', ''), ('LY', 'Nafusi', ''), ('LY', 'Tebu', ''),
    ('LY', 'Tamasheq', ''),

    -- Madagascar
    ('MG', 'Merina', ''), ('MG', 'Betsimisaraka', ''), ('MG', 'Sakalava', ''),
    ('MG', 'Betsileo', ''), ('MG', 'Antandroy', ''), ('MG', 'Bara', ''),
    ('MG', 'Tsimihety', ''),

    -- Malawi
    ('MW', 'Chichewa', ''), ('MW', 'Tumbuka', ''), ('MW', 'Lomwe', ''),
    ('MW', 'Yao', ''), ('MW', 'Sena', ''), ('MW', 'Tonga', ''),
    ('MW', 'Ngoni', ''),

    -- Mali
    ('ML', 'Bambara', ''), ('ML', 'Fulfulde', ''), ('ML', 'Songhaï', ''),
    ('ML', 'Soninké', ''), ('ML', 'Tamasheq', ''), ('ML', 'Dogon', ''),
    ('ML', 'Minianka', ''), ('ML', 'Bozo', ''),

    -- Mauritanie
    ('MR', 'Pulaar', ''), ('MR', 'Soninké', ''), ('MR', 'Wolof', ''),
    ('MR', 'Imraguen', ''),

    -- Maurice
    ('MU', 'Créole mauricien', ''), ('MU', 'Bhojpuri', ''),
    ('MU', 'Rodriguan', ''),

    -- Maroc
    ('MA', 'Tarifit', ''), ('MA', 'Tamazight du Moyen Atlas', ''),
    ('MA', 'Tachelhit', ''), ('MA', 'Hassaniya', ''),

    -- Mozambique
    ('MZ', 'Makhuwa', ''), ('MZ', 'Sena', ''), ('MZ', 'Tsonga', ''),
    ('MZ', 'Lomwe', ''), ('MZ', 'Chopi', ''), ('MZ', 'Makonde', ''),
    ('MZ', 'Yao', ''), ('MZ', 'Ndau', ''),

    -- Namibie
    ('NA', 'Oshiwambo', ''), ('NA', 'Herero', ''), ('NA', 'Nama', ''),
    ('NA', 'Damara', ''), ('NA', 'Kavango', ''), ('NA', 'Caprivian', ''),
    ('NA', 'San', 'Ju|''hoan'),

    -- Niger
    ('NE', 'Hausa', ''), ('NE', 'Zarma', ''), ('NE', 'Fulfulde', ''),
    ('NE', 'Kanuri', ''), ('NE', 'Tamasheq', ''), ('NE', 'Gourmantchéma', ''),
    ('NE', 'Toubou', ''),

    -- Nigeria
    ('NG', 'Hausa', ''), ('NG', 'Yoruba', ''), ('NG', 'Igbo', ''),
    ('NG', 'Fulfulde', ''), ('NG', 'Kanuri', ''), ('NG', 'Tiv', ''),
    ('NG', 'Edo', ''), ('NG', 'Ibibio', ''), ('NG', 'Nupe', ''),
    ('NG', 'Ijaw', ''), ('NG', 'Urhobo', ''),

    -- Rwanda
    ('RW', 'Kinyarwanda', ''),

    -- Sao Tomé-et-Principe
    ('ST', 'Forro', ''), ('ST', 'Angolar', ''), ('ST', 'Lunguyê', ''),
    ('ST', 'Principense', ''),

    -- Sénégal
    ('SN', 'Wolof', ''), ('SN', 'Pulaar', ''), ('SN', 'Sérère', ''),
    ('SN', 'Jola', ''), ('SN', 'Soninké', ''), ('SN', 'Mandingue', ''),
    ('SN', 'Balante', ''), ('SN', 'Mancagne', ''),

    -- Seychelles
    ('SC', 'Créole seychellois', ''),

    -- Sierra Leone
    ('SL', 'Mende', ''), ('SL', 'Temne', ''), ('SL', 'Krio', ''),
    ('SL', 'Limba', ''), ('SL', 'Kono', ''), ('SL', 'Kissi', ''),

    -- Somalie
    ('SO', 'Somali', 'Maay, Maxaa'), ('SO', 'Afar', ''), ('SO', 'Garre', ''),

    -- Afrique du Sud
    ('ZA', 'Zulu', ''), ('ZA', 'Xhosa', ''), ('ZA', 'Sesotho', ''),
    ('ZA', 'Tswana', ''), ('ZA', 'Ndebele', ''), ('ZA', 'Venda', ''),
    ('ZA', 'Tsonga', ''), ('ZA', 'Pedi', ''), ('ZA', 'Swati', ''),
    ('ZA', 'Khoisan', 'Nama'),

    -- Soudan du Sud
    ('SS', 'Dinka', ''), ('SS', 'Nuer', ''), ('SS', 'Shilluk', ''),
    ('SS', 'Bari', ''), ('SS', 'Zande', ''), ('SS', 'Toposa', ''),

    -- Soudan
    ('SD', 'Nubien', ''), ('SD', 'Beja', ''), ('SD', 'Fur', ''),
    ('SD', 'Dinka', ''), ('SD', 'Nuba', 'plusieurs langues kordofaniennes'),

    -- Tanzanie
    ('TZ', 'Sukuma', ''), ('TZ', 'Chaga', ''), ('TZ', 'Haya', ''),
    ('TZ', 'Nyamwezi', ''), ('TZ', 'Makonde', ''), ('TZ', 'Hehe', ''),
    ('TZ', 'Gogo', ''), ('TZ', 'Nyakyusa', ''),

    -- Togo
    ('TG', 'Ewe', ''), ('TG', 'Kabye', ''), ('TG', 'Mina', ''),
    ('TG', 'Tem', ''), ('TG', 'Moba', ''), ('TG', 'Ifè', ''),

    -- Tunisie
    ('TN', 'Chelha', ''), ('TN', 'Djerbi', ''), ('TN', 'Matmata', ''),
    ('TN', 'Sened', 'berbères'),

    -- Ouganda
    ('UG', 'Luganda', ''), ('UG', 'Runyankole', ''), ('UG', 'Acholi', ''),
    ('UG', 'Lusoga', ''), ('UG', 'Langi', ''), ('UG', 'Ateso', ''),
    ('UG', 'Lugbara', ''),

    -- Zambie
    ('ZM', 'Bemba', ''), ('ZM', 'Nyanja', ''), ('ZM', 'Tonga', ''),
    ('ZM', 'Lozi', ''), ('ZM', 'Kaonde', ''), ('ZM', 'Lunda', ''),
    ('ZM', 'Luvale', ''),

    -- Zimbabwe
    ('ZW', 'Shona', ''), ('ZW', 'Ndebele', ''), ('ZW', 'Tonga', ''),
    ('ZW', 'Venda', ''), ('ZW', 'Kalanga', ''), ('ZW', 'Sotho', ''),
    ('ZW', 'Shangani', '')
) AS src(code_iso2, nom, langues)
JOIN shared.pays p ON p.code_iso2 = src.code_iso2
JOIN country_profile.fiche_pays fp ON fp.pays_id = p.id
WHERE NOT EXISTS (
    SELECT 1 FROM country_profile.groupe_ethnique g
    WHERE g.fiche_pays_id = fp.id
      AND lower(g.nom) = lower(src.nom)
);

COMMIT;
