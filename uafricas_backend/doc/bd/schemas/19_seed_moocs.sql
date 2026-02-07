-- ════════════════════════════════════════════════════════════════════════════
-- SEED : Formations MOOC/CLOM (media_content.mooc + inscriptions)
-- ════════════════════════════════════════════════════════════════════════════

DO $$
DECLARE
    -- Pays
    pays_sn_id   UUID;
    pays_ci_id   UUID;
    pays_gh_id   UUID;
    pays_ml_id   UUID;
    pays_ke_id   UUID;
    pays_bf_id   UUID;

    -- Utilisateurs (formateurs)
    user_fatou_id    UUID;
    user_kouassi_id  UUID;
    user_aminata_id  UUID;
    user_ibrahim_id  UUID;
    user_ousmane_id  UUID;
    user_mariam_id   UUID;
    user_amadou_id   UUID;
    user_seydou_id   UUID;
    user_marie_id    UUID;
    user_moussa_id   UUID;

    -- MOOC IDs
    v_mooc_id UUID;

BEGIN
    -- ── Recuperer les pays existants ──────────────────────────────
    SELECT id INTO pays_sn_id FROM shared.pays WHERE code_iso2 = 'SN';
    SELECT id INTO pays_ci_id FROM shared.pays WHERE code_iso2 = 'CI';
    SELECT id INTO pays_gh_id FROM shared.pays WHERE code_iso2 = 'GH';
    SELECT id INTO pays_ml_id FROM shared.pays WHERE LOWER(nom) = 'mali';
    SELECT id INTO pays_ke_id FROM shared.pays WHERE LOWER(nom) = 'kenya';
    SELECT id INTO pays_bf_id FROM shared.pays WHERE LOWER(nom) = 'burkina faso';

    -- ── Recuperer les utilisateurs existants ──────────────────────
    SELECT id INTO user_fatou_id   FROM iam.utilisateur WHERE email = 'fatou.traore@uafricas.org';
    SELECT id INTO user_kouassi_id FROM iam.utilisateur WHERE email = 'kouassi.yao@uafricas.org';
    SELECT id INTO user_aminata_id FROM iam.utilisateur WHERE email = 'aminata.ndiaye@uafricas.org';
    SELECT id INTO user_ibrahim_id FROM iam.utilisateur WHERE email = 'ibrahim.kone@uafricas.org';
    SELECT id INTO user_ousmane_id FROM iam.utilisateur WHERE email = 'ousmane.sow@uafricas.org';
    SELECT id INTO user_mariam_id  FROM iam.utilisateur WHERE email = 'mariam.toure@uafricas.org';
    SELECT id INTO user_amadou_id  FROM iam.utilisateur WHERE email = 'amadou.diallo@uafricas.org';
    SELECT id INTO user_seydou_id  FROM iam.utilisateur WHERE email = 'seydou.ouedraogo@uafricas.org';
    SELECT id INTO user_marie_id   FROM iam.utilisateur WHERE email = 'marie.ndong@uafricas.org';
    SELECT id INTO user_moussa_id  FROM iam.utilisateur WHERE email = 'moussa.coulibaly@uafricas.org';

    -- ══════════════════════════════════════════════════════════════
    -- FORMATIONS MOOC / CLOM
    -- ══════════════════════════════════════════════════════════════

    -- 1. Introduction a l'economie africaine (MOOC, publie, inscriptions ouvertes)
    INSERT INTO media_content.mooc
        (titre, slug, description, type, pays_id, ville,
         date_heure_debut, date_heure_fin,
         image_couverture_url, format, lien_en_ligne,
         langue, nombre_places, prerequis, etat, cree_par)
    VALUES (
        'Introduction à l''économie africaine',
        'introduction-economie-africaine',
        'Ce MOOC offre une vue d''ensemble complète de l''économie africaine, de ses défis et opportunités. Vous découvrirez les principales économies du continent, les secteurs porteurs et les enjeux de l''intégration régionale à travers la ZLECAF.',
        'mooc', pays_sn_id, 'Dakar',
        NOW() + INTERVAL '15 days', NOW() + INTERVAL '75 days',
        'https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?w=800',
        'en_ligne', 'https://learn.uafricas.org/economie-africaine',
        'Français', 500,
        'Aucun prérequis particulier. Ouvert à tous les niveaux.',
        'publie', user_fatou_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO v_mooc_id;

    INSERT INTO media_content.mooc_inscription (mooc_id, utilisateur_id, statut)
    VALUES (v_mooc_id,user_amadou_id, 'inscrit'), (v_mooc_id, user_kouassi_id, 'inscrit'),
           (v_mooc_id, user_marie_id, 'inscrit'), (v_mooc_id, user_moussa_id, 'inscrit'),
           (v_mooc_id, user_seydou_id, 'inscrit')
    ON CONFLICT (mooc_id, utilisateur_id) DO NOTHING;

    -- 2. Leadership et gouvernance en Afrique (CLOM, publie, inscriptions ouvertes)
    INSERT INTO media_content.mooc
        (titre, slug, description, type, pays_id, ville,
         date_heure_debut, date_heure_fin,
         image_couverture_url, format, lien_en_ligne,
         langue, nombre_places, prerequis, etat, cree_par)
    VALUES (
        'Leadership et gouvernance en Afrique',
        'leadership-gouvernance-afrique',
        'Un programme intensif pour développer un leadership authentique et efficace, ancré dans les valeurs africaines tout en intégrant les meilleures pratiques internationales de management et de gouvernance.',
        'clom', pays_ci_id, 'Abidjan',
        NOW() + INTERVAL '30 days', NOW() + INTERVAL '120 days',
        'https://images.unsplash.com/photo-1552664730-d307ca884978?w=800',
        'en_ligne', 'https://learn.uafricas.org/leadership-afrique',
        'Français', 50,
        'Expérience professionnelle de 3 ans minimum recommandée.',
        'publie', user_kouassi_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO v_mooc_id;

    INSERT INTO media_content.mooc_inscription (mooc_id, utilisateur_id, statut)
    VALUES (v_mooc_id,user_fatou_id, 'inscrit'), (v_mooc_id, user_aminata_id, 'inscrit'),
           (v_mooc_id, user_ibrahim_id, 'inscrit')
    ON CONFLICT (mooc_id, utilisateur_id) DO NOTHING;

    -- 3. Atelier pratique : Entrepreneuriat social (Atelier, publie, hybride)
    INSERT INTO media_content.mooc
        (titre, slug, description, type, pays_id, ville,
         date_heure_debut, date_heure_fin,
         image_couverture_url, format, lien_en_ligne,
         langue, nombre_places, prerequis, etat, cree_par)
    VALUES (
        'Atelier pratique : Entrepreneuriat social',
        'atelier-entrepreneuriat-social',
        'Un atelier pratique de 3 jours pour transformer votre idée en projet d''entreprise sociale viable. Coaching individuel et collectif inclus avec des entrepreneurs confirmés du continent.',
        'atelier', pays_bf_id, 'Ouagadougou',
        NOW() + INTERVAL '20 days', NOW() + INTERVAL '23 days',
        'https://images.unsplash.com/photo-1559136555-9303baea8ebd?w=800',
        'hybride', 'https://meet.uafricas.org/entrepreneuriat-social',
        'Français', 25,
        'Avoir une idée de projet d''entreprise sociale.',
        'publie', user_aminata_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO v_mooc_id;

    INSERT INTO media_content.mooc_inscription (mooc_id, utilisateur_id, statut)
    VALUES (v_mooc_id,user_ousmane_id, 'inscrit'), (v_mooc_id, user_mariam_id, 'inscrit'),
           (v_mooc_id, user_amadou_id, 'inscrit')
    ON CONFLICT (mooc_id, utilisateur_id) DO NOTHING;

    -- 4. Concertation : L'avenir de l'education en Afrique (Concertation, publie, a venir)
    INSERT INTO media_content.mooc
        (titre, slug, description, type, pays_id, ville,
         date_heure_debut, date_heure_fin,
         image_couverture_url, format, lien_en_ligne,
         langue, nombre_places, prerequis, etat, cree_par)
    VALUES (
        'Concertation : L''avenir de l''éducation en Afrique',
        'concertation-avenir-education-afrique',
        'Une série de discussions en ligne réunissant experts, éducateurs et décideurs pour repenser l''éducation africaine. Thèmes abordés : curricula adaptés, technologies éducatives, langues locales dans l''enseignement.',
        'concertation', pays_ml_id, 'Bamako',
        NOW() + INTERVAL '45 days', NOW() + INTERVAL '47 days',
        'https://images.unsplash.com/photo-1503676260728-1c00da094a0b?w=800',
        'en_ligne', 'https://meet.uafricas.org/education-afrique',
        'Français', 200,
        NULL,
        'publie', user_ibrahim_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO v_mooc_id;

    -- 5. Developpement web moderne (MOOC, en_cours, complet)
    INSERT INTO media_content.mooc
        (titre, slug, description, type, pays_id, ville,
         date_heure_debut, date_heure_fin,
         image_couverture_url, format, lien_en_ligne,
         langue, nombre_places, prerequis, etat, cree_par)
    VALUES (
        'Développement web moderne',
        'developpement-web-moderne',
        'Formation complète au développement web : HTML, CSS, JavaScript, Vue.js, Node.js. Projets pratiques et déploiement inclus. Accompagnement personnalisé par des développeurs senior africains.',
        'mooc', pays_ke_id, 'Nairobi',
        NOW() - INTERVAL '20 days', NOW() + INTERVAL '40 days',
        'https://images.unsplash.com/photo-1498050108023-c5249f4df085?w=800',
        'en_ligne', 'https://learn.uafricas.org/dev-web',
        'Français', 100,
        'Connaissances de base en informatique.',
        'en_cours', user_ousmane_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO v_mooc_id;

    INSERT INTO media_content.mooc_inscription (mooc_id, utilisateur_id, statut)
    VALUES (v_mooc_id,user_amadou_id, 'en_cours'), (v_mooc_id, user_fatou_id, 'en_cours'),
           (v_mooc_id, user_kouassi_id, 'en_cours'), (v_mooc_id, user_mariam_id, 'en_cours'),
           (v_mooc_id, user_seydou_id, 'en_cours'), (v_mooc_id, user_marie_id, 'en_cours'),
           (v_mooc_id, user_ibrahim_id, 'en_cours'), (v_mooc_id, user_moussa_id, 'en_cours'),
           (v_mooc_id, user_aminata_id, 'en_cours')
    ON CONFLICT (mooc_id, utilisateur_id) DO NOTHING;

    -- 6. Agriculture durable et agroecologie (MOOC, publie, inscriptions ouvertes)
    INSERT INTO media_content.mooc
        (titre, slug, description, type, pays_id, ville,
         date_heure_debut, date_heure_fin,
         image_couverture_url, format, lien_en_ligne,
         langue, nombre_places, prerequis, etat, cree_par)
    VALUES (
        'Agriculture durable et agroécologie',
        'agriculture-durable-agroecologie',
        'Découvrez les méthodes d''agriculture durable, la permaculture et l''agroforesterie pour une production alimentaire résiliente adaptée au climat africain. Études de cas et retours d''expérience du terrain.',
        'mooc', pays_gh_id, 'Accra',
        NOW() + INTERVAL '35 days', NOW() + INTERVAL '95 days',
        'https://images.unsplash.com/photo-1416879595882-3373a0480b5b?w=800',
        'en_ligne', 'https://learn.uafricas.org/agriculture-durable',
        'Français', 300,
        NULL,
        'publie', user_mariam_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO v_mooc_id;

    INSERT INTO media_content.mooc_inscription (mooc_id, utilisateur_id, statut)
    VALUES (v_mooc_id,user_seydou_id, 'inscrit'), (v_mooc_id, user_moussa_id, 'inscrit'),
           (v_mooc_id, user_aminata_id, 'inscrit'), (v_mooc_id, user_ousmane_id, 'inscrit')
    ON CONFLICT (mooc_id, utilisateur_id) DO NOTHING;

    RAISE NOTICE '✅ Seed MOOC : 6 formations + inscriptions insérées avec succès';
END $$;
