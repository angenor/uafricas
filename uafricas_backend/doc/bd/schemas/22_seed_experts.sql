-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Seed : Profils experts (12 experts de démonstration)
-- ════════════════════════════════════════════════════════════════════════════

DO $$
DECLARE
    -- Pays IDs
    pays_sn_id UUID;
    pays_ci_id UUID;
    pays_ml_id UUID;
    pays_bf_id UUID;
    pays_gh_id UUID;
    pays_gn_id UUID;
    pays_cm_id UUID;

    -- User IDs
    user_mamadou_id   UUID;
    user_awa_id       UUID;
    user_ibrahima_id  UUID;
    user_fatoumata_id UUID;
    user_pascal_id    UUID;
    user_kofi_id      UUID;
    user_mariama_id   UUID;
    user_amadou_t_id  UUID;
    user_aicha_id     UUID;
    user_seydou_id    UUID;
    user_ousmane_d_id UUID;
    user_aminata_id   UUID;

BEGIN

    -- ── Récupérer ou créer les pays nécessaires ─────────────────────

    SELECT id INTO pays_sn_id FROM shared.pays WHERE LOWER(nom) = 'sénégal';
    IF pays_sn_id IS NULL THEN
        SELECT id INTO pays_sn_id FROM shared.pays WHERE code_iso2 = 'SN';
    END IF;
    IF pays_sn_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Sénégal', 'SN', 'SEN', '+221', 'Dakar', 'Afrique', TRUE)
        RETURNING id INTO pays_sn_id;
    END IF;

    SELECT id INTO pays_ci_id FROM shared.pays WHERE code_iso2 = 'CI';
    IF pays_ci_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Côte d''Ivoire', 'CI', 'CIV', '+225', 'Yamoussoukro', 'Afrique', TRUE)
        RETURNING id INTO pays_ci_id;
    END IF;

    SELECT id INTO pays_ml_id FROM shared.pays WHERE LOWER(nom) = 'mali';
    IF pays_ml_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Mali', 'ML', 'MLI', '+223', 'Bamako', 'Afrique', TRUE)
        RETURNING id INTO pays_ml_id;
    END IF;

    SELECT id INTO pays_bf_id FROM shared.pays WHERE LOWER(nom) = 'burkina faso';
    IF pays_bf_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Burkina Faso', 'BF', 'BFA', '+226', 'Ouagadougou', 'Afrique', TRUE)
        RETURNING id INTO pays_bf_id;
    END IF;

    SELECT id INTO pays_gh_id FROM shared.pays WHERE LOWER(nom) = 'ghana';
    IF pays_gh_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Ghana', 'GH', 'GHA', '+233', 'Accra', 'Afrique', TRUE)
        RETURNING id INTO pays_gh_id;
    END IF;

    SELECT id INTO pays_gn_id FROM shared.pays WHERE LOWER(nom) = 'guinée';
    IF pays_gn_id IS NULL THEN
        SELECT id INTO pays_gn_id FROM shared.pays WHERE code_iso2 = 'GN';
    END IF;
    IF pays_gn_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Guinée', 'GN', 'GIN', '+224', 'Conakry', 'Afrique', TRUE)
        RETURNING id INTO pays_gn_id;
    END IF;

    SELECT id INTO pays_cm_id FROM shared.pays WHERE LOWER(nom) = 'cameroun';
    IF pays_cm_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Cameroun', 'CM', 'CMR', '+237', 'Yaoundé', 'Afrique', TRUE)
        RETURNING id INTO pays_cm_id;
    END IF;


    -- ── Créer les 12 utilisateurs experts ───────────────────────────
    -- Mot de passe : "Expert2024!" hashé avec bcrypt (cost 12)
    -- ON CONFLICT (email) DO NOTHING pour idempotence

    -- 1. Mamadou Diallo — Informatique — Sénégal
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Diallo', 'Mamadou', 'mamadou.diallo@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Dakar', pays_sn_id,
            'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=400&fit=crop',
            '2023-01-15'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_mamadou_id;
    IF user_mamadou_id IS NULL THEN
        SELECT id INTO user_mamadou_id FROM iam.utilisateur WHERE email = 'mamadou.diallo@example.com';
    END IF;

    -- 2. Awa Kouassi — Finance — Côte d'Ivoire
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Kouassi', 'Awa', 'awa.kouassi@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Abidjan', pays_ci_id,
            'https://images.unsplash.com/photo-1531123897727-8f129e1688ce?w=400&h=400&fit=crop',
            '2023-03-20'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_awa_id;
    IF user_awa_id IS NULL THEN
        SELECT id INTO user_awa_id FROM iam.utilisateur WHERE email = 'awa.kouassi@example.com';
    END IF;

    -- 3. Ibrahima Ndiaye — Agriculture — Sénégal
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Ndiaye', 'Ibrahima', 'ibrahima.ndiaye@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Saint-Louis', pays_sn_id,
            'https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=400&h=400&fit=crop',
            '2022-06-10'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_ibrahima_id;
    IF user_ibrahima_id IS NULL THEN
        SELECT id INTO user_ibrahima_id FROM iam.utilisateur WHERE email = 'ibrahima.ndiaye@example.com';
    END IF;

    -- 4. Fatoumata Kamara — Santé — Mali
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Kamara', 'Fatoumata', 'fatoumata.kamara@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Bamako', pays_ml_id,
            'https://images.unsplash.com/photo-1489424731084-a5d8b219a5bb?w=400&h=400&fit=crop',
            '2023-02-28'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_fatoumata_id;
    IF user_fatoumata_id IS NULL THEN
        SELECT id INTO user_fatoumata_id FROM iam.utilisateur WHERE email = 'fatoumata.kamara@example.com';
    END IF;

    -- 5. Pascal Ouedraogo — Éducation — Burkina Faso
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Ouedraogo', 'Pascal', 'pascal.ouedraogo@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Ouagadougou', pays_bf_id,
            'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=400&h=400&fit=crop',
            '2022-09-01'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_pascal_id;
    IF user_pascal_id IS NULL THEN
        SELECT id INTO user_pascal_id FROM iam.utilisateur WHERE email = 'pascal.ouedraogo@example.com';
    END IF;

    -- 6. Kofi Mensah — Électronique — Ghana
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Mensah', 'Kofi', 'kofi.mensah@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Accra', pays_gh_id,
            'https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=400&h=400&fit=crop',
            '2023-05-15'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_kofi_id;
    IF user_kofi_id IS NULL THEN
        SELECT id INTO user_kofi_id FROM iam.utilisateur WHERE email = 'kofi.mensah@example.com';
    END IF;

    -- 7. Mariama Bah — Immobilier — Guinée
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Bah', 'Mariama', 'mariama.bah@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Conakry', pays_gn_id,
            'https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=400&h=400&fit=crop',
            '2024-01-10'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_mariama_id;
    IF user_mariama_id IS NULL THEN
        SELECT id INTO user_mariama_id FROM iam.utilisateur WHERE email = 'mariama.bah@example.com';
    END IF;

    -- 8. Amadou Toure — Mécanique — Mali
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Toure', 'Amadou', 'amadou.toure.expert@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Bamako', pays_ml_id,
            'https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=400&h=400&fit=crop',
            '2022-11-20'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_amadou_t_id;
    IF user_amadou_t_id IS NULL THEN
        SELECT id INTO user_amadou_t_id FROM iam.utilisateur WHERE email = 'amadou.toure.expert@example.com';
    END IF;

    -- 9. Aicha Ndongo — Informatique — Cameroun
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Ndongo', 'Aicha', 'aicha.ndongo@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Douala', pays_cm_id,
            'https://images.unsplash.com/photo-1580489944761-15a19d654956?w=400&h=400&fit=crop',
            '2023-07-05'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_aicha_id;
    IF user_aicha_id IS NULL THEN
        SELECT id INTO user_aicha_id FROM iam.utilisateur WHERE email = 'aicha.ndongo@example.com';
    END IF;

    -- 10. Seydou Kone — Agriculture — Côte d'Ivoire
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Kone', 'Seydou', 'seydou.kone@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Yamoussoukro', pays_ci_id,
            'https://images.unsplash.com/photo-1531427186611-ecfd6d936c79?w=400&h=400&fit=crop',
            '2022-04-12'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_seydou_id;
    IF user_seydou_id IS NULL THEN
        SELECT id INTO user_seydou_id FROM iam.utilisateur WHERE email = 'seydou.kone@example.com';
    END IF;

    -- 11. Ousmane Diop — Finance — Sénégal
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Diop', 'Ousmane', 'ousmane.diop.expert@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Dakar', pays_sn_id,
            'https://images.unsplash.com/photo-1560250097-0b93528c311a?w=400&h=400&fit=crop',
            '2022-08-25'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_ousmane_d_id;
    IF user_ousmane_d_id IS NULL THEN
        SELECT id INTO user_ousmane_d_id FROM iam.utilisateur WHERE email = 'ousmane.diop.expert@example.com';
    END IF;

    -- 12. Aminata Sanogo — Santé — Burkina Faso
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, ville, pays_residence_id, photo_url, created_at)
    VALUES ('Sanogo', 'Aminata', 'aminata.sanogo@example.com',
            '$2b$12$LJ3m4ys3Lk0TSwMCfVGZnOvPJXmOHby.YDpMfOKbRbFHbgKZdQEKS',
            'actif', 'Bobo-Dioulasso', pays_bf_id,
            'https://images.unsplash.com/photo-1573496359142-b8d87734a5a2?w=400&h=400&fit=crop',
            '2023-09-18'::timestamptz)
    ON CONFLICT (email) DO NOTHING
    RETURNING id INTO user_aminata_id;
    IF user_aminata_id IS NULL THEN
        SELECT id INTO user_aminata_id FROM iam.utilisateur WHERE email = 'aminata.sanogo@example.com';
    END IF;


    -- ── Créer les 12 profils expertise ──────────────────────────────

    INSERT INTO iam.expertise (utilisateur_id, domaine, biographie, nb_annees_experience, rating, portfolio, situations_professionnelles, statut, created_at, updated_at)
    VALUES
        -- 1. Mamadou Diallo — Informatique
        (user_mamadou_id, 'informatique',
         'Expert en développement web et mobile avec plus de 10 ans d''expérience. Spécialisé dans les technologies modernes comme React, Vue.js et Node.js. Passionné par l''innovation technologique en Afrique.',
         12, 4.8, NULL,
         ARRAY['consultance', 'volontariat_expertise']::iam.situation_professionnelle[],
         'valide', '2023-01-15'::timestamptz, '2024-11-20'::timestamptz),

        -- 2. Awa Kouassi — Finance
        (user_awa_id, 'finance',
         'Experte en finance et gestion d''entreprise. Accompagne les startups africaines dans leur développement financier et leur levée de fonds. MBA de HEC Paris.',
         8, 4.9, NULL,
         ARRAY['en_emploi', 'recherche_nouvelles_opportunites']::iam.situation_professionnelle[],
         'valide', '2023-03-20'::timestamptz, '2024-12-01'::timestamptz),

        -- 3. Ibrahima Ndiaye — Agriculture
        (user_ibrahima_id, 'agriculture',
         'Ingénieur agronome spécialisé dans l''agriculture durable et l''agroécologie. Travaille sur des projets de sécurité alimentaire en Afrique de l''Ouest depuis 15 ans.',
         15, 4.7, NULL,
         ARRAY['en_emploi', 'volontariat_expertise']::iam.situation_professionnelle[],
         'valide', '2022-06-10'::timestamptz, '2024-10-15'::timestamptz),

        -- 4. Fatoumata Kamara — Santé
        (user_fatoumata_id, 'sante',
         'Médecin spécialiste en santé publique. Coordonne des programmes de vaccination et de lutte contre les maladies tropicales. Docteur de l''Université de Genève.',
         10, 4.9, NULL,
         ARRAY['en_emploi']::iam.situation_professionnelle[],
         'valide', '2023-02-28'::timestamptz, '2024-11-30'::timestamptz),

        -- 5. Pascal Ouedraogo — Éducation
        (user_pascal_id, 'education',
         'Expert en pédagogie et formation professionnelle. Développe des programmes éducatifs innovants adaptés au contexte africain. Ancien directeur d''école.',
         20, 4.6, NULL,
         ARRAY['consultance', 'recherche_nouvelles_opportunites']::iam.situation_professionnelle[],
         'valide', '2022-09-01'::timestamptz, '2024-08-20'::timestamptz),

        -- 6. Kofi Mensah — Électronique
        (user_kofi_id, 'electronique',
         'Ingénieur en électronique et systèmes embarqués. Spécialisé dans les solutions IoT pour l''agriculture intelligente. Fondateur d''une startup tech à Accra.',
         7, 4.5, NULL,
         ARRAY['en_emploi', 'consultance']::iam.situation_professionnelle[],
         'valide', '2023-05-15'::timestamptz, '2024-12-05'::timestamptz),

        -- 7. Mariama Bah — Immobilier
        (user_mariama_id, 'immobilier',
         'Experte en développement immobilier et urbanisme durable. Conseille les promoteurs et les collectivités sur les projets de construction écologique en Afrique.',
         9, 4.7, NULL,
         ARRAY['recherche_emploi']::iam.situation_professionnelle[],
         'valide', '2024-01-10'::timestamptz, '2024-11-25'::timestamptz),

        -- 8. Amadou Toure — Mécanique
        (user_amadou_t_id, 'mecanique',
         'Ingénieur mécanique spécialisé dans les énergies renouvelables. Conçoit des solutions solaires et éoliennes adaptées au contexte africain.',
         11, 4.8, NULL,
         ARRAY['en_emploi', 'volontariat_expertise']::iam.situation_professionnelle[],
         'valide', '2022-11-20'::timestamptz, '2024-09-30'::timestamptz),

        -- 9. Aicha Ndongo — Informatique
        (user_aicha_id, 'informatique',
         'Data scientist et experte en intelligence artificielle. Développe des solutions IA pour résoudre des problématiques africaines. PhD en Machine Learning.',
         6, 4.9, NULL,
         ARRAY['consultance', 'recherche_nouvelles_opportunites']::iam.situation_professionnelle[],
         'valide', '2023-07-05'::timestamptz, '2024-12-10'::timestamptz),

        -- 10. Seydou Kone — Agriculture
        (user_seydou_id, 'agriculture',
         'Spécialiste en cultures vivrières et transformation agroalimentaire. Accompagne les coopératives agricoles dans leur modernisation et leur accès aux marchés.',
         14, 4.6, NULL,
         ARRAY['volontariat_expertise']::iam.situation_professionnelle[],
         'valide', '2022-04-12'::timestamptz, '2024-07-18'::timestamptz),

        -- 11. Ousmane Diop — Finance
        (user_ousmane_d_id, 'finance',
         'Expert en microfinance et inclusion financière. Travaille avec des institutions de microfinance pour développer des produits adaptés aux populations rurales.',
         13, 4.7, NULL,
         ARRAY['en_emploi']::iam.situation_professionnelle[],
         'valide', '2022-08-25'::timestamptz, '2024-10-05'::timestamptz),

        -- 12. Aminata Sanogo — Santé
        (user_aminata_id, 'sante',
         'Pharmacienne et experte en médecine traditionnelle africaine. Travaille sur la valorisation des plantes médicinales et leur intégration dans les systèmes de santé.',
         8, 4.8, NULL,
         ARRAY['recherche_emploi', 'consultance']::iam.situation_professionnelle[],
         'valide', '2023-09-18'::timestamptz, '2024-11-12'::timestamptz)
    ON CONFLICT (utilisateur_id) DO NOTHING;

    RAISE NOTICE 'Seed experts : 12 profils experts créés avec succès';

END $$;
