-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Seed : Fiches Pays (données de test)
-- ════════════════════════════════════════════════════════════════════════════

DO $$
DECLARE
    -- Pays IDs
    pays_senegal_id     UUID;
    pays_civ_id         UUID;
    pays_cameroun_id    UUID;
    pays_rdc_id         UUID;
    pays_kenya_id       UUID;
    pays_ethiopie_id    UUID;
    pays_egypte_id      UUID;
    pays_maroc_id       UUID;
    pays_za_id          UUID;
    pays_nigeria_id     UUID;
    pays_ghana_id       UUID;
    pays_tanzanie_id    UUID;

    -- Fiche IDs
    fiche_senegal_id    UUID;
    fiche_civ_id        UUID;
    fiche_cameroun_id   UUID;
    fiche_rdc_id        UUID;
    fiche_kenya_id      UUID;
    fiche_ethiopie_id   UUID;
    fiche_egypte_id     UUID;
    fiche_maroc_id      UUID;
    fiche_za_id         UUID;
    fiche_nigeria_id    UUID;
    fiche_ghana_id      UUID;
    fiche_tanzanie_id   UUID;

    -- Createur
    u_createur_id       UUID;
    role_utilisateur_id UUID;

BEGIN

    -- ── 1. Insérer/récupérer les pays dans shared.pays ─────────────────────

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Sénégal', 'SN', 'SEN', '+221', 'Dakar', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_senegal_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Côte d''Ivoire', 'CI', 'CIV', '+225', 'Yamoussoukro', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_civ_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Cameroun', 'CM', 'CMR', '+237', 'Yaoundé', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_cameroun_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('République Démocratique du Congo', 'CD', 'COD', '+243', 'Kinshasa', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_rdc_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Kenya', 'KE', 'KEN', '+254', 'Nairobi', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_kenya_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Éthiopie', 'ET', 'ETH', '+251', 'Addis-Abeba', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_ethiopie_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Égypte', 'EG', 'EGY', '+20', 'Le Caire', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_egypte_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Maroc', 'MA', 'MAR', '+212', 'Rabat', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_maroc_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Afrique du Sud', 'ZA', 'ZAF', '+27', 'Pretoria', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_za_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Nigeria', 'NG', 'NGA', '+234', 'Abuja', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_nigeria_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Ghana', 'GH', 'GHA', '+233', 'Accra', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_ghana_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Tanzanie', 'TZ', 'TZA', '+255', 'Dodoma', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_tanzanie_id;

    -- ── 2. Créer un utilisateur créateur ────────────────────────────────────

    SELECT id INTO role_utilisateur_id FROM iam.role WHERE slug = 'utilisateur';

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, email_verifie)
    VALUES ('Ndiaye', 'Moussa', 'moussa.ndiaye@uafricas.org',
            '$2b$12$LJ3m4ys4yKq8q0k.ABC123fakeHashForSeedOnly00000000000000', 'actif', TRUE)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_createur_id;

    IF role_utilisateur_id IS NOT NULL THEN
        INSERT INTO iam.utilisateur_role (utilisateur_id, role_id)
        VALUES (u_createur_id, role_utilisateur_id)
        ON CONFLICT DO NOTHING;
    END IF;

    -- ── 3. Insérer les fiches pays ──────────────────────────────────────────

    -- Sénégal
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_senegal_id,
        'https://images.unsplash.com/photo-1589556264800-08ae9e129a8c?w=800',
        'Un Peuple, Un But, Une Foi',
        196722.00, 18380000,
        'https://flagcdn.com/sn.svg',
        'Un Peuple, Un But, Une Foi',
        'Français',
        'Wolof, Sérère, Diola, Mandingue, Peul',
        'Franc CFA (XOF)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_senegal_id;

    -- Côte d'Ivoire
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_civ_id,
        'https://images.unsplash.com/photo-1591117207239-788bf8de6c3b?w=800',
        'Union, Discipline, Travail',
        322463.00, 31170000,
        'https://flagcdn.com/ci.svg',
        'Union, Discipline, Travail',
        'Français',
        'Dioula, Baoulé, Bété, Sénoufo',
        'Franc CFA (XOF)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_civ_id;

    -- Cameroun
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_cameroun_id,
        'https://images.unsplash.com/photo-1596005554384-d293674c91d7?w=800',
        'Paix, Travail, Patrie',
        475442.00, 29120000,
        'https://flagcdn.com/cm.svg',
        'Paix, Travail, Patrie',
        'Français, Anglais',
        'Fulfulde, Ewondo, Bamiléké',
        'Franc CFA (XAF)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_cameroun_id;

    -- RDC
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_rdc_id,
        'https://images.unsplash.com/photo-1580746738099-78d6833b3471?w=800',
        'Justice, Paix, Travail',
        2345409.00, 105800000,
        'https://flagcdn.com/cd.svg',
        'Justice, Paix, Travail',
        'Français',
        'Lingala, Swahili, Kikongo, Tshiluba',
        'Franc congolais (CDF)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_rdc_id;

    -- Kenya
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_kenya_id,
        'https://images.unsplash.com/photo-1489392191049-fc10c97e64b6?w=800',
        'Harambee (Travaillons ensemble)',
        580367.00, 56430000,
        'https://flagcdn.com/ke.svg',
        'Harambee',
        'Swahili, Anglais',
        'Kikuyu, Luo, Kamba',
        'Shilling kényan (KES)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_kenya_id;

    -- Éthiopie
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_ethiopie_id,
        'https://images.unsplash.com/photo-1523805009345-7448845a9e53?w=800',
        'L''Éthiopie d''abord',
        1104300.00, 128700000,
        'https://flagcdn.com/et.svg',
        'L''Éthiopie d''abord',
        'Amharique',
        'Oromo, Tigrinya, Somali, Afar',
        'Birr éthiopien (ETB)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_ethiopie_id;

    -- Égypte
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_egypte_id,
        'https://images.unsplash.com/photo-1539650116574-8efeb43e2750?w=800',
        'Liberté, Socialisme, Unité',
        1001449.00, 114500000,
        'https://flagcdn.com/eg.svg',
        'Liberté, Socialisme, Unité',
        'Arabe',
        'Arabe égyptien',
        'Livre égyptienne (EGP)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_egypte_id;

    -- Maroc
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_maroc_id,
        'https://images.unsplash.com/photo-1493246507139-91e8fad9978e?w=800',
        'Dieu, la Patrie, le Roi',
        446550.00, 37840000,
        'https://flagcdn.com/ma.svg',
        'Dieu, la Patrie, le Roi',
        'Arabe, Amazigh',
        'Darija (arabe marocain), Tamazight, Français',
        'Dirham marocain (MAD)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_maroc_id;

    -- Afrique du Sud
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_za_id,
        'https://images.unsplash.com/photo-1580060839134-75a5edca2e99?w=800',
        '!ke e: /xarra //ke (L''unité dans la diversité)',
        1221037.00, 63210000,
        'https://flagcdn.com/za.svg',
        '!ke e: /xarra //ke',
        'Anglais, Afrikaans, Zoulou, Xhosa, Sotho, Tswana, Pedi, Tsonga, Swati, Venda, Ndebele',
        'Zoulou, Xhosa, Afrikaans, Sotho du Nord',
        'Rand sud-africain (ZAR)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_za_id;

    -- Nigeria
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_nigeria_id,
        'https://images.unsplash.com/photo-1618828665011-0abd973f7bb8?w=800',
        'Unité et Foi, Paix et Progrès',
        923768.00, 229150000,
        'https://flagcdn.com/ng.svg',
        'Unité et Foi, Paix et Progrès',
        'Anglais',
        'Haoussa, Yoruba, Igbo, Fulani',
        'Naira nigérian (NGN)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_nigeria_id;

    -- Ghana
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_ghana_id,
        'https://images.unsplash.com/photo-1577948000111-9c970dfe3743?w=800',
        'Liberté et Justice',
        238535.00, 34120000,
        'https://flagcdn.com/gh.svg',
        'Liberté et Justice',
        'Anglais',
        'Twi, Fante, Ga, Ewe, Dagbani',
        'Cedi ghanéen (GHS)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_ghana_id;

    -- Tanzanie
    INSERT INTO country_profile.fiche_pays (
        pays_id, image_couverture_url, slogan, superficie_km2, population,
        image_drapeau_url, image_devise_url, langue_officielle, langues_populaires,
        monnaie, cree_par
    ) VALUES (
        pays_tanzanie_id,
        'https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?w=800',
        'Uhuru na Umoja (Liberté et Unité)',
        945087.00, 67440000,
        'https://flagcdn.com/tz.svg',
        'Uhuru na Umoja',
        'Swahili, Anglais',
        'Sukuma, Chagga, Haya',
        'Shilling tanzanien (TZS)',
        u_createur_id
    )
    ON CONFLICT (pays_id) DO UPDATE SET
        image_couverture_url = EXCLUDED.image_couverture_url,
        slogan = EXCLUDED.slogan,
        superficie_km2 = EXCLUDED.superficie_km2,
        population = EXCLUDED.population,
        image_drapeau_url = EXCLUDED.image_drapeau_url,
        image_devise_url = EXCLUDED.image_devise_url,
        langue_officielle = EXCLUDED.langue_officielle,
        langues_populaires = EXCLUDED.langues_populaires,
        monnaie = EXCLUDED.monnaie,
        updated_at = NOW()
    RETURNING id INTO fiche_tanzanie_id;

    -- ── 4. Insérer les groupes ethniques ────────────────────────────────────

    -- Sénégal
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_senegal_id, 'Wolof'),
        (fiche_senegal_id, 'Peul'),
        (fiche_senegal_id, 'Sérère'),
        (fiche_senegal_id, 'Diola'),
        (fiche_senegal_id, 'Mandingue'),
        (fiche_senegal_id, 'Soninké')
    ON CONFLICT DO NOTHING;

    -- Côte d'Ivoire
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_civ_id, 'Baoulé'),
        (fiche_civ_id, 'Bété'),
        (fiche_civ_id, 'Sénoufo'),
        (fiche_civ_id, 'Malinké'),
        (fiche_civ_id, 'Dan'),
        (fiche_civ_id, 'Lobi')
    ON CONFLICT DO NOTHING;

    -- Cameroun
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_cameroun_id, 'Bamiléké'),
        (fiche_cameroun_id, 'Béti'),
        (fiche_cameroun_id, 'Fulani'),
        (fiche_cameroun_id, 'Kirdi'),
        (fiche_cameroun_id, 'Bassa'),
        (fiche_cameroun_id, 'Douala')
    ON CONFLICT DO NOTHING;

    -- RDC
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_rdc_id, 'Luba'),
        (fiche_rdc_id, 'Kongo'),
        (fiche_rdc_id, 'Mongo'),
        (fiche_rdc_id, 'Mangbetu-Azande'),
        (fiche_rdc_id, 'Lunda')
    ON CONFLICT DO NOTHING;

    -- Kenya
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_kenya_id, 'Kikuyu'),
        (fiche_kenya_id, 'Luhya'),
        (fiche_kenya_id, 'Kalenjin'),
        (fiche_kenya_id, 'Luo'),
        (fiche_kenya_id, 'Kamba'),
        (fiche_kenya_id, 'Kisii'),
        (fiche_kenya_id, 'Meru')
    ON CONFLICT DO NOTHING;

    -- Éthiopie
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_ethiopie_id, 'Oromo'),
        (fiche_ethiopie_id, 'Amhara'),
        (fiche_ethiopie_id, 'Somali'),
        (fiche_ethiopie_id, 'Tigréens'),
        (fiche_ethiopie_id, 'Sidama'),
        (fiche_ethiopie_id, 'Gurage')
    ON CONFLICT DO NOTHING;

    -- Égypte
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_egypte_id, 'Égyptiens'),
        (fiche_egypte_id, 'Bédouins'),
        (fiche_egypte_id, 'Nubiens'),
        (fiche_egypte_id, 'Berbères')
    ON CONFLICT DO NOTHING;

    -- Maroc
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_maroc_id, 'Arabes'),
        (fiche_maroc_id, 'Berbères'),
        (fiche_maroc_id, 'Gnawa'),
        (fiche_maroc_id, 'Haratin')
    ON CONFLICT DO NOTHING;

    -- Afrique du Sud
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_za_id, 'Zoulou'),
        (fiche_za_id, 'Xhosa'),
        (fiche_za_id, 'Métis'),
        (fiche_za_id, 'Blancs'),
        (fiche_za_id, 'Pedi'),
        (fiche_za_id, 'Sotho'),
        (fiche_za_id, 'Tswana')
    ON CONFLICT DO NOTHING;

    -- Nigeria
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_nigeria_id, 'Haoussa'),
        (fiche_nigeria_id, 'Yoruba'),
        (fiche_nigeria_id, 'Igbo'),
        (fiche_nigeria_id, 'Fulani'),
        (fiche_nigeria_id, 'Ijaw'),
        (fiche_nigeria_id, 'Kanuri'),
        (fiche_nigeria_id, 'Ibibio'),
        (fiche_nigeria_id, 'Tiv')
    ON CONFLICT DO NOTHING;

    -- Ghana
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_ghana_id, 'Akan'),
        (fiche_ghana_id, 'Mole-Dagbani'),
        (fiche_ghana_id, 'Ewe'),
        (fiche_ghana_id, 'Ga-Dangme'),
        (fiche_ghana_id, 'Gurma'),
        (fiche_ghana_id, 'Guan')
    ON CONFLICT DO NOTHING;

    -- Tanzanie
    INSERT INTO country_profile.groupe_ethnique (fiche_pays_id, nom) VALUES
        (fiche_tanzanie_id, 'Sukuma'),
        (fiche_tanzanie_id, 'Chagga'),
        (fiche_tanzanie_id, 'Haya'),
        (fiche_tanzanie_id, 'Makonde'),
        (fiche_tanzanie_id, 'Nyamwezi'),
        (fiche_tanzanie_id, 'Zanzibar')
    ON CONFLICT DO NOTHING;

    RAISE NOTICE 'Seed fiches pays : 12 fiches + groupes ethniques insérés avec succès';

END $$;
