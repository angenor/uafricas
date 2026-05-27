-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Seed : TOUS les pays africains (référentiel + fiches pays)
-- ════════════════════════════════════════════════════════════════════════════
-- Périmètre = liste figée Afripulse (54 États + Sahara occidental « eh »),
-- miroir de `src/constants/afripulse_pays_autorises.rs`.
--
-- Idempotent :
--   • shared.pays          → ON CONFLICT (code_iso2) DO UPDATE
--   • country_profile.fiche_pays → créée seulement si absente (NOT EXISTS),
--     donc les 12 fiches riches de 20_seed_fiches_pays.sql sont préservées.
--
-- Doit être chargé APRÈS 20_seed_fiches_pays.sql (réutilise son créateur seedé).
-- ════════════════════════════════════════════════════════════════════════════

-- ── Table de travail : données canoniques des 55 codes ──────────────────────
DROP TABLE IF EXISTS tmp_pays_afrique;
CREATE TEMP TABLE tmp_pays_afrique (
    iso2      CHAR(2)      NOT NULL,
    nom       VARCHAR(150) NOT NULL,
    iso3      CHAR(3)      NOT NULL,
    indicatif VARCHAR(10)  NOT NULL,
    capitale  VARCHAR(150) NOT NULL,
    langue    VARCHAR(250) NOT NULL,
    monnaie   VARCHAR(120) NOT NULL
);

INSERT INTO tmp_pays_afrique (iso2, nom, iso3, indicatif, capitale, langue, monnaie) VALUES
    ('dz', 'Algérie',                          'DZA', '+213', 'Alger',         'Arabe, Amazigh',                       'Dinar algérien (DZD)'),
    ('ao', 'Angola',                           'AGO', '+244', 'Luanda',        'Portugais',                            'Kwanza (AOA)'),
    ('bj', 'Bénin',                            'BEN', '+229', 'Porto-Novo',    'Français',                             'Franc CFA (XOF)'),
    ('bw', 'Botswana',                         'BWA', '+267', 'Gaborone',      'Anglais, Tswana',                      'Pula (BWP)'),
    ('bf', 'Burkina Faso',                     'BFA', '+226', 'Ouagadougou',   'Français',                             'Franc CFA (XOF)'),
    ('bi', 'Burundi',                          'BDI', '+257', 'Gitega',        'Kirundi, Français',                    'Franc burundais (BIF)'),
    ('cv', 'Cap-Vert',                         'CPV', '+238', 'Praia',         'Portugais',                            'Escudo cap-verdien (CVE)'),
    ('cm', 'Cameroun',                         'CMR', '+237', 'Yaoundé',       'Français, Anglais',                    'Franc CFA (XAF)'),
    ('cf', 'République centrafricaine',        'CAF', '+236', 'Bangui',        'Français, Sango',                      'Franc CFA (XAF)'),
    ('km', 'Comores',                          'COM', '+269', 'Moroni',        'Comorien, Arabe, Français',            'Franc comorien (KMF)'),
    ('cg', 'République du Congo',              'COG', '+242', 'Brazzaville',   'Français',                             'Franc CFA (XAF)'),
    ('cd', 'République Démocratique du Congo', 'COD', '+243', 'Kinshasa',      'Français',                             'Franc congolais (CDF)'),
    ('ci', 'Côte d''Ivoire',                   'CIV', '+225', 'Yamoussoukro',  'Français',                             'Franc CFA (XOF)'),
    ('dj', 'Djibouti',                         'DJI', '+253', 'Djibouti',      'Français, Arabe',                      'Franc djiboutien (DJF)'),
    ('eg', 'Égypte',                           'EGY', '+20',  'Le Caire',      'Arabe',                                'Livre égyptienne (EGP)'),
    ('gq', 'Guinée équatoriale',               'GNQ', '+240', 'Malabo',        'Espagnol, Français, Portugais',        'Franc CFA (XAF)'),
    ('er', 'Érythrée',                         'ERI', '+291', 'Asmara',        'Tigrinya, Arabe, Anglais',             'Nakfa (ERN)'),
    ('sz', 'Eswatini',                         'SWZ', '+268', 'Mbabane',       'Swati, Anglais',                       'Lilangeni (SZL)'),
    ('et', 'Éthiopie',                         'ETH', '+251', 'Addis-Abeba',   'Amharique',                            'Birr éthiopien (ETB)'),
    ('ga', 'Gabon',                            'GAB', '+241', 'Libreville',    'Français',                             'Franc CFA (XAF)'),
    ('gm', 'Gambie',                           'GMB', '+220', 'Banjul',        'Anglais',                              'Dalasi (GMD)'),
    ('gh', 'Ghana',                            'GHA', '+233', 'Accra',         'Anglais',                              'Cedi ghanéen (GHS)'),
    ('gn', 'Guinée',                           'GIN', '+224', 'Conakry',       'Français',                             'Franc guinéen (GNF)'),
    ('gw', 'Guinée-Bissau',                    'GNB', '+245', 'Bissau',        'Portugais',                            'Franc CFA (XOF)'),
    ('ke', 'Kenya',                            'KEN', '+254', 'Nairobi',       'Swahili, Anglais',                     'Shilling kényan (KES)'),
    ('ls', 'Lesotho',                          'LSO', '+266', 'Maseru',        'Sesotho, Anglais',                     'Loti (LSL)'),
    ('lr', 'Libéria',                          'LBR', '+231', 'Monrovia',      'Anglais',                              'Dollar libérien (LRD)'),
    ('ly', 'Libye',                            'LBY', '+218', 'Tripoli',       'Arabe',                                'Dinar libyen (LYD)'),
    ('mg', 'Madagascar',                       'MDG', '+261', 'Antananarivo',  'Malgache, Français',                   'Ariary (MGA)'),
    ('mw', 'Malawi',                           'MWI', '+265', 'Lilongwe',      'Anglais, Chichewa',                    'Kwacha malawien (MWK)'),
    ('ml', 'Mali',                             'MLI', '+223', 'Bamako',        'Français',                             'Franc CFA (XOF)'),
    ('ma', 'Maroc',                            'MAR', '+212', 'Rabat',         'Arabe, Amazigh',                       'Dirham marocain (MAD)'),
    ('mr', 'Mauritanie',                       'MRT', '+222', 'Nouakchott',    'Arabe',                                'Ouguiya (MRU)'),
    ('mu', 'Maurice',                          'MUS', '+230', 'Port-Louis',    'Anglais, Français, Créole',            'Roupie mauricienne (MUR)'),
    ('mz', 'Mozambique',                       'MOZ', '+258', 'Maputo',        'Portugais',                            'Metical (MZN)'),
    ('na', 'Namibie',                          'NAM', '+264', 'Windhoek',      'Anglais',                              'Dollar namibien (NAD)'),
    ('ne', 'Niger',                            'NER', '+227', 'Niamey',        'Français',                             'Franc CFA (XOF)'),
    ('ng', 'Nigeria',                          'NGA', '+234', 'Abuja',         'Anglais',                              'Naira (NGN)'),
    ('ug', 'Ouganda',                          'UGA', '+256', 'Kampala',       'Anglais, Swahili',                     'Shilling ougandais (UGX)'),
    ('rw', 'Rwanda',                           'RWA', '+250', 'Kigali',        'Kinyarwanda, Français, Anglais, Swahili', 'Franc rwandais (RWF)'),
    ('st', 'Sao Tomé-et-Principe',             'STP', '+239', 'São Tomé',      'Portugais',                            'Dobra (STN)'),
    ('sn', 'Sénégal',                          'SEN', '+221', 'Dakar',         'Français',                             'Franc CFA (XOF)'),
    ('sc', 'Seychelles',                       'SYC', '+248', 'Victoria',      'Créole seychellois, Anglais, Français', 'Roupie seychelloise (SCR)'),
    ('sl', 'Sierra Leone',                     'SLE', '+232', 'Freetown',      'Anglais',                              'Leone (SLE)'),
    ('so', 'Somalie',                          'SOM', '+252', 'Mogadiscio',    'Somali, Arabe',                        'Shilling somalien (SOS)'),
    ('sd', 'Soudan',                           'SDN', '+249', 'Khartoum',      'Arabe, Anglais',                       'Livre soudanaise (SDG)'),
    ('ss', 'Soudan du Sud',                    'SSD', '+211', 'Djouba',        'Anglais',                              'Livre sud-soudanaise (SSP)'),
    ('za', 'Afrique du Sud',                   'ZAF', '+27',  'Pretoria',      'Anglais, Zoulou, Xhosa, Afrikaans',    'Rand sud-africain (ZAR)'),
    ('tz', 'Tanzanie',                         'TZA', '+255', 'Dodoma',        'Swahili, Anglais',                     'Shilling tanzanien (TZS)'),
    ('td', 'Tchad',                            'TCD', '+235', 'N''Djaména',    'Français, Arabe',                      'Franc CFA (XAF)'),
    ('tg', 'Togo',                             'TGO', '+228', 'Lomé',          'Français',                             'Franc CFA (XOF)'),
    ('tn', 'Tunisie',                          'TUN', '+216', 'Tunis',         'Arabe',                                'Dinar tunisien (TND)'),
    ('zm', 'Zambie',                           'ZMB', '+260', 'Lusaka',        'Anglais',                              'Kwacha zambien (ZMW)'),
    ('zw', 'Zimbabwe',                         'ZWE', '+263', 'Harare',        'Anglais',                              'Dollar zimbabwéen (ZWL)'),
    ('eh', 'Sahara occidental',                'ESH', '+212', 'Laâyoune',      'Arabe',                                'Dirham marocain (MAD)');

-- ── 1. Référentiel : upsert des 55 codes dans shared.pays ───────────────────
INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
SELECT nom, UPPER(iso2), iso3, indicatif, capitale, 'Afrique'
FROM tmp_pays_afrique
ON CONFLICT (code_iso2) DO UPDATE SET
    nom           = EXCLUDED.nom,
    code_iso3     = EXCLUDED.code_iso3,
    indicatif_tel = EXCLUDED.indicatif_tel,
    capitale      = EXCLUDED.capitale,
    continent     = EXCLUDED.continent,
    actif         = TRUE,
    updated_at    = NOW();

-- ── 2. Fiches pays minimales pour tout pays africain sans fiche ─────────────
DO $$
DECLARE
    u_createur_id UUID;
    nb_creees     INTEGER;
BEGIN
    -- Réutilise le créateur seedé par 20_seed_fiches_pays.sql, sinon repli
    -- sur n'importe quel utilisateur existant (sinon abandon propre).
    SELECT id INTO u_createur_id
    FROM iam.utilisateur
    WHERE email = 'moussa.ndiaye@uafricas.org';

    IF u_createur_id IS NULL THEN
        SELECT id INTO u_createur_id
        FROM iam.utilisateur
        ORDER BY created_at
        LIMIT 1;
    END IF;

    IF u_createur_id IS NULL THEN
        RAISE NOTICE 'Aucun utilisateur disponible comme créateur — fiches pays non créées';
        RETURN;
    END IF;

    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, image_drapeau_url,
        langue_officielle, monnaie, cree_par
    )
    SELECT
        p.id,
        'https://flagcdn.com/w640/' || LOWER(p.code_iso2) || '.png',
        'https://flagcdn.com/w320/' || LOWER(p.code_iso2) || '.png',
        t.langue,
        t.monnaie,
        u_createur_id
    FROM shared.pays p
    JOIN tmp_pays_afrique t ON LOWER(p.code_iso2) = t.iso2
    WHERE NOT EXISTS (
        SELECT 1 FROM country_profile.fiche_pays f WHERE f.pays_id = p.id
    );

    GET DIAGNOSTICS nb_creees = ROW_COUNT;
    RAISE NOTICE 'Seed pays africains : 55 codes référencés, % nouvelles fiches créées', nb_creees;
END $$;

DROP TABLE IF EXISTS tmp_pays_afrique;
