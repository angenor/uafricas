-- ============================================================================
-- Jeu de démonstration de la feature 009 (programmes conteneurs et épisodes)
--
-- Écrit en SQL et non par l'API : le seul compte administrateur de production
-- appartient à l'utilisateur, et on ne se sert pas de ses identifiants. Les
-- invariants que l'API défend sont donc reproduits ici explicitement — état,
-- ordre, unicité des mises en avant, exclusivité de la couverture.
--
-- Les vidéos sont de VRAIES URL YouTube, chacune vérifiée par l'endpoint
-- oEmbed de YouTube avant d'être écrite : une seule des candidates refusait
-- l'intégration, et elle a été écartée. Les couvertures sont les vignettes
-- officielles des mêmes vidéos, ce qui évite des cartes grises.
--
-- Idempotent : rejouable sans doublon (ON CONFLICT sur les slugs, et un garde
-- en tête qui sort si le jeu est déjà en place).
-- ============================================================================

\set ON_ERROR_STOP on

BEGIN;

DO $seed$
DECLARE
    v_auteur   UUID;
    v_chaine   UUID;
    v_emission UUID;
    v_episode  UUID;
    v_theme    UUID;
    v_samedi   DATE;
    v_mercredi DATE;
    v_dimanche DATE;
BEGIN
    -- L'administrateur de la plateforme porte la création : `cree_par` est NOT
    -- NULL et sert de repli au bénéficiaire des points d'engagement.
    -- Le compte nommé d'abord, un administrateur ensuite : ce seed echouait
    -- sur TOUTE base ne portant pas cette adresse precise, alors qu'il ne lui
    -- faut qu'un porteur pour `cree_par`, NOT NULL.
    SELECT id INTO v_auteur FROM iam.utilisateur
    WHERE email = 'angenor99@gmail.com' AND deleted_at IS NULL;

    IF v_auteur IS NULL THEN
        SELECT u.id INTO v_auteur
        FROM iam.utilisateur u
        JOIN iam.utilisateur_role ur ON ur.utilisateur_id = u.id
        JOIN iam.role r ON r.id = ur.role_id
        WHERE r.nom IN ('Administrateur', 'Super Administrateur')
          AND u.deleted_at IS NULL
        ORDER BY u.created_at
        LIMIT 1;
    END IF;

    IF v_auteur IS NULL THEN
        RAISE EXCEPTION 'Aucun compte administrateur : seed interrompu';
    END IF;

    IF EXISTS (SELECT 1 FROM media_content.chaine_tv
                WHERE slug = 'africans-histoire' AND deleted_at IS NULL) THEN
        RAISE NOTICE 'Jeu de démonstration déjà en place — rien à faire.';
        RETURN;
    END IF;

    -- Dates d'effet reculées : la rotation a déjà tourné, on voit donc de
    -- vraies rediffusions plutôt qu'un rang 0 partout.
    v_samedi   := CURRENT_DATE - ((7 + EXTRACT(DOW FROM CURRENT_DATE)::int - 6) % 7) - 14;
    v_mercredi := CURRENT_DATE - ((7 + EXTRACT(DOW FROM CURRENT_DATE)::int - 3) % 7) - 7;
    v_dimanche := CURRENT_DATE - ((7 + EXTRACT(DOW FROM CURRENT_DATE)::int - 0) % 7);

    -- ========================================================================
    -- 1. Chaîne « Africans Histoire » — production de la plateforme
    -- ========================================================================
    INSERT INTO media_content.chaine_tv
        (id, nom, slug, description, image_couverture_url, categorie, langue,
         est_en_direct, etat, origine_publication, couverture_continentale,
         contact_email, contact_site_web, cree_par, created_at, updated_at)
    VALUES (gen_random_uuid(), 'Africans Histoire', 'africans-histoire',
            'Les grands récits du continent : empires, routes commerciales, figures oubliées. Une chaîne de la plateforme, diffusée sur toute l''Afrique.',
            'https://i.ytimg.com/vi/ecdabz94_Co/hqdefault.jpg',
            'culture', 'Français', FALSE, 'publie', 'africans', TRUE,
            'histoire@africans-world.org', 'https://www.africans-world.org',
            v_auteur, NOW(), NOW())
    RETURNING id INTO v_chaine;

    FOR v_theme IN
        SELECT id FROM shared.categorie
         WHERE contexte = 'media' AND actif
           AND nom IN ('Culture', 'Investigations', 'Grandes interviews')
    LOOP
        INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
        VALUES ('chaine_tv', v_chaine, v_theme) ON CONFLICT DO NOTHING;
    END LOOP;

    INSERT INTO media_content.emission_tele
        (id, chaine_id, titre, slug, description, image_couverture_url,
         info_animateur, info_producteur, langue, cadence, etat, cree_par,
         theme_phare_id, created_at, updated_at)
    VALUES (gen_random_uuid(), v_chaine, 'Mémoires d''Empires', 'memoires-d-empires',
            'Chaque semaine, un empire africain raconté par ses sources : le Ghana, le Mali, le Songhaï.',
            'https://i.ytimg.com/vi/tuCIq9NPvQ4/hqdefault.jpg',
            'Aminata Diallo', 'Africans Histoire', 'Français', 'hebdomadaire', 'publie', v_auteur,
            (SELECT id FROM shared.categorie WHERE contexte='media' AND nom='Culture'),
            NOW(), NOW())
    RETURNING id INTO v_emission;

    -- L'ORDRE décide de la rotation : deux épisodes au même rang la rendraient
    -- non déterministe. Il est donc explicite et contigu, à partir de 0.
    INSERT INTO media_content.episode_tele
        (id, emission_id, titre, slug, description, image_couverture_url, video_url,
         numero_episode, ordre, duree_minutes, a_la_une, a_la_une_globale, etat,
         valide_par, valide_at, cree_par, created_at, updated_at)
    VALUES
      (gen_random_uuid(), v_emission, 'Le Ghana, première puissance ouest-africaine', 'le-ghana-premiere-puissance-ouest-africaine',
       'Aux sources du royaume de Wagadou, fondé par les Soninkés, et de sa domination sur les routes de l''or.',
       'https://i.ytimg.com/vi/tuCIq9NPvQ4/hqdefault.jpg', 'https://www.youtube.com/watch?v=tuCIq9NPvQ4',
       1, 0, 24, TRUE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'Comment l''empire du Mali a éclipsé le Ghana', 'comment-l-empire-du-mali-a-eclipse-le-ghana',
       'Le basculement du XIIIᵉ siècle : Soundiata, la charte du Manden et le déplacement des routes caravanières.',
       'https://i.ytimg.com/vi/ecdabz94_Co/hqdefault.jpg', 'https://www.youtube.com/watch?v=ecdabz94_Co',
       2, 1, 18, FALSE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'L''empire du Mali, épisode 1', 'l-empire-du-mali-episode-1',
       'Première partie d''une série consacrée au Mali impérial, de Soundiata à Mansa Moussa.',
       'https://i.ytimg.com/vi/Cm5yOJc_NLo/hqdefault.jpg', 'https://www.youtube.com/watch?v=Cm5yOJc_NLo',
       3, 2, 21, FALSE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'Le grand empire africain du Mali', 'le-grand-empire-africain-du-mali',
       'Synthèse : administration, commerce transsaharien et rayonnement de Tombouctou.',
       'https://i.ytimg.com/vi/NbZaaAdf5Aw/hqdefault.jpg', 'https://www.youtube.com/watch?v=NbZaaAdf5Aw',
       4, 3, 27, FALSE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW());

    -- Créneau hebdomadaire. `date_effet` reculée de 14 jours ⇒ deux occurrences
    -- écoulées : la grille montre le 3ᵉ épisode, pas le premier.
    INSERT INTO media_content.creneau_programmation
        (id, type_support, support_id, emission_id, recurrence, jour_semaine,
         heure_debut, duree_minutes, fuseau, date_effet, actif, cree_par, created_at, updated_at)
    VALUES (gen_random_uuid(), 'chaine_tv', v_chaine, v_emission, 'hebdomadaire', 6,
            '20:00', 90, 'Africa/Abidjan', v_samedi, TRUE, v_auteur, NOW(), NOW());

    -- ========================================================================
    -- 2. Chaîne « Africans Innovation »
    -- ========================================================================
    INSERT INTO media_content.chaine_tv
        (id, nom, slug, description, image_couverture_url, categorie, langue,
         est_en_direct, etat, origine_publication, couverture_continentale,
         contact_email, cree_par, created_at, updated_at)
    VALUES (gen_random_uuid(), 'Africans Innovation', 'africans-innovation',
            'Ceux qui fabriquent le continent de demain : fintech, énergie, agriculture, santé.',
            'https://i.ytimg.com/vi/9z52xavACQY/hqdefault.jpg',
            'education', 'Français', TRUE, 'publie', 'africans', TRUE,
            'innovation@africans-world.org', v_auteur, NOW(), NOW())
    RETURNING id INTO v_chaine;

    FOR v_theme IN
        SELECT id FROM shared.categorie
         WHERE contexte = 'media' AND actif
           AND nom IN ('Magazine Innovation', 'Émissions économiques',
                       'Émissions jeunesse', 'Éducation')
    LOOP
        INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
        VALUES ('chaine_tv', v_chaine, v_theme) ON CONFLICT DO NOTHING;
    END LOOP;

    INSERT INTO media_content.emission_tele
        (id, chaine_id, titre, slug, description, image_couverture_url,
         info_animateur, info_producteur, langue, cadence, etat, cree_par,
         theme_phare_id, created_at, updated_at)
    VALUES (gen_random_uuid(), v_chaine, 'Labo Africain', 'labo-africain',
            'Le magazine hebdomadaire de l''innovation africaine, du prototype au déploiement.',
            'https://i.ytimg.com/vi/OvTMkEYu6l8/hqdefault.jpg',
            'Kwame Mensah', 'Africans Innovation', 'Français', 'hebdomadaire', 'publie', v_auteur,
            (SELECT id FROM shared.categorie WHERE contexte='media' AND nom='Magazine Innovation'),
            NOW(), NOW())
    RETURNING id INTO v_emission;

    -- Un seul épisode occupe la tête de /medias/tele : l'index unique global le
    -- garantit. Rétrograder l'ancienne vedette AVANT de promouvoir la nouvelle,
    -- dans la même transaction, est exactement ce que fait
    -- `definir_vedette_globale` — sans cela l'insertion suivante échoue.
    UPDATE media_content.episode_tele
       SET a_la_une_globale = FALSE, updated_at = NOW()
     WHERE a_la_une_globale = TRUE;

    INSERT INTO media_content.episode_tele
        (id, emission_id, titre, slug, description, image_couverture_url, video_url,
         numero_episode, ordre, duree_minutes, a_la_une, a_la_une_globale, etat,
         valide_par, valide_at, cree_par, created_at, updated_at)
    VALUES
      (gen_random_uuid(), v_emission, 'Les pionniers de la tech africaine', 'les-pionniers-de-la-tech-africaine',
       'Dix des économies les plus dynamiques du monde sont africaines : portrait de ceux qui les outillent.',
       'https://i.ytimg.com/vi/9z52xavACQY/hqdefault.jpg', 'https://www.youtube.com/watch?v=9z52xavACQY',
       1, 0, 42, FALSE, TRUE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'Fintech : connecter des millions de personnes', 'fintech-connecter-des-millions-de-personnes',
       'La course des start-up africaines pour réduire la fracture numérique et bancariser le continent.',
       'https://i.ytimg.com/vi/OvTMkEYu6l8/hqdefault.jpg', 'https://www.youtube.com/watch?v=OvTMkEYu6l8',
       2, 1, 23, TRUE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'Ce que l''Afrique vient d''inventer', 'ce-que-l-afrique-vient-d-inventer',
       'Kenya, Afrique du Sud, Congo : les inventions qui changent la donne.',
       'https://i.ytimg.com/vi/W_hQj5mkvaI/hqdefault.jpg', 'https://www.youtube.com/watch?v=W_hQj5mkvaI',
       3, 2, 15, FALSE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'L''entrepreneuriat au cœur du développement', 'l-entrepreneuriat-au-coeur-du-developpement',
       'Six jeunes sur dix ont moins de 25 ans en Afrique subsaharienne : l''entreprise comme réponse.',
       'https://i.ytimg.com/vi/RV6lvELxBuo/hqdefault.jpg', 'https://www.youtube.com/watch?v=RV6lvELxBuo',
       4, 3, 26, FALSE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW());

    INSERT INTO media_content.creneau_programmation
        (id, type_support, support_id, emission_id, recurrence, jour_semaine,
         heure_debut, duree_minutes, fuseau, date_effet, actif, cree_par, created_at, updated_at)
    VALUES (gen_random_uuid(), 'chaine_tv', v_chaine, v_emission, 'hebdomadaire', 3,
            '21:00', 60, 'Africa/Abidjan', v_mercredi, TRUE, v_auteur, NOW(), NOW());

    -- ========================================================================
    -- 3. Chaîne « Terrain Afrique » — chaîne de territoire, couverture ciblée
    -- ========================================================================
    INSERT INTO media_content.chaine_tv
        (id, nom, slug, description, image_couverture_url, categorie, langue,
         est_en_direct, etat, origine_publication, couverture_continentale,
         pays_id, contact_email, contact_telephone, cree_par, created_at, updated_at)
    VALUES (gen_random_uuid(), 'Terrain Afrique', 'terrain-afrique',
            'Le reportage au ras du sol : ce que vivent les villes et les campagnes, sans commentaire de plateau.',
            'https://i.ytimg.com/vi/Ofn31if1Fac/hqdefault.jpg',
            'info', 'Français', FALSE, 'publie', 'territoire', FALSE,
            (SELECT id FROM shared.pays WHERE nom = 'Côte d''Ivoire'),
            'terrain@africans-world.org', '+225 07 00 00 00', v_auteur, NOW(), NOW())
    RETURNING id INTO v_chaine;

    FOR v_theme IN
        SELECT id FROM shared.categorie
         WHERE contexte = 'media' AND actif
           AND nom IN ('Investigations', 'Émissions citoyennes', 'Grandes interviews')
    LOOP
        INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
        VALUES ('chaine_tv', v_chaine, v_theme) ON CONFLICT DO NOTHING;
    END LOOP;

    -- Couverture ciblée : le trigger d'exclusivité l'accepte, la chaîne n'étant
    -- pas déclarée continentale.
    INSERT INTO media_content.support_territoire (type_support, support_id, pays_id)
    SELECT 'chaine_tv', v_chaine, id FROM shared.pays
     WHERE nom IN ('Côte d''Ivoire', 'Sénégal', 'Nigeria', 'Kenya')
    ON CONFLICT DO NOTHING;

    -- Programme quotidien
    INSERT INTO media_content.emission_tele
        (id, chaine_id, titre, slug, description, image_couverture_url,
         info_animateur, langue, cadence, etat, cree_par, theme_phare_id, created_at, updated_at)
    VALUES (gen_random_uuid(), v_chaine, 'Grand Reportage', 'grand-reportage',
            'Un terrain, chaque jour. Des villes qui débordent, des économies qui s''inventent.',
            'https://i.ytimg.com/vi/KzjEhgcBvSE/hqdefault.jpg',
            'Fatou Bensouda', 'Français', 'quotidienne', 'publie', v_auteur,
            (SELECT id FROM shared.categorie WHERE contexte='media' AND nom='Investigations'),
            NOW(), NOW())
    RETURNING id INTO v_emission;

    INSERT INTO media_content.episode_tele
        (id, emission_id, titre, slug, description, image_couverture_url, video_url,
         numero_episode, ordre, duree_minutes, a_la_une, a_la_une_globale, etat,
         valide_par, valide_at, cree_par, created_at, updated_at)
    VALUES
      (gen_random_uuid(), v_emission, 'Lagos, entre villas de luxe et bidonvilles', 'lagos-entre-villas-de-luxe-et-bidonvilles',
       'La plus grande ville d''Afrique, vue depuis ses deux extrémités.',
       'https://i.ytimg.com/vi/Ofn31if1Fac/hqdefault.jpg', 'https://www.youtube.com/watch?v=Ofn31if1Fac',
       1, 0, 52, TRUE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'Kenya, les rois de la débrouille', 'kenya-les-rois-de-la-debrouille',
       'L''économie informelle de Nairobi, racontée par ceux qui la font tenir.',
       'https://i.ytimg.com/vi/KzjEhgcBvSE/hqdefault.jpg', 'https://www.youtube.com/watch?v=KzjEhgcBvSE',
       2, 1, 48, FALSE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'La face sombre du marché du jeans', 'la-face-sombre-du-marche-du-jeans',
       'Comment la friperie importée a défait les filières textiles du continent.',
       'https://i.ytimg.com/vi/r7AaktS648I/hqdefault.jpg', 'https://www.youtube.com/watch?v=r7AaktS648I',
       3, 2, 51, FALSE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW());

    -- Quotidien : la rotation avance d'un cran par jour. `date_effet` d'il y a
    -- 4 jours ⇒ rang 4 sur 3 épisodes, donc une rediffusion visible.
    INSERT INTO media_content.creneau_programmation
        (id, type_support, support_id, emission_id, recurrence, jour_semaine,
         heure_debut, duree_minutes, fuseau, date_effet, actif, cree_par, created_at, updated_at)
    VALUES (gen_random_uuid(), 'chaine_tv', v_chaine, v_emission, 'quotidien', NULL,
            '19:00', 55, 'Africa/Abidjan', CURRENT_DATE - 4, TRUE, v_auteur, NOW(), NOW());

    -- Second programme de la même chaîne — la section en montre donc deux.
    INSERT INTO media_content.emission_tele
        (id, chaine_id, titre, slug, description, image_couverture_url,
         info_animateur, langue, cadence, etat, cree_par, theme_phare_id, created_at, updated_at)
    VALUES (gen_random_uuid(), v_chaine, 'Regards', 'regards',
            'Des films au long cours, diffusés au fil des acquisitions. Aucune périodicité annoncée.',
            'https://i.ytimg.com/vi/u9uf-cd63Po/hqdefault.jpg',
            'Ngozi Okoro', 'Français', 'ponctuelle', 'publie', v_auteur,
            (SELECT id FROM shared.categorie WHERE contexte='media' AND nom='Culture'),
            NOW(), NOW())
    RETURNING id INTO v_emission;

    INSERT INTO media_content.episode_tele
        (id, emission_id, titre, slug, description, image_couverture_url, video_url,
         numero_episode, ordre, duree_minutes, a_la_une, a_la_une_globale, etat,
         valide_par, valide_at, cree_par, created_at, updated_at)
    VALUES
      (gen_random_uuid(), v_emission, 'L''Afrique moderne', 'l-afrique-moderne',
       'Le continent tel qu''il ne se donne pas à voir dans les télévisions occidentales.',
       'https://i.ytimg.com/vi/u9uf-cd63Po/hqdefault.jpg', 'https://www.youtube.com/watch?v=u9uf-cd63Po',
       1, 0, 44, TRUE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW()),
      (gen_random_uuid(), v_emission, 'Cultures numériques en Afrique de l''Ouest', 'cultures-numeriques-en-afrique-de-l-ouest',
       'Enquête du Cirad sur l''appropriation du numérique dans les sociétés ouest-africaines.',
       'https://i.ytimg.com/vi/jx_FiRs39s8/hqdefault.jpg', 'https://www.youtube.com/watch?v=jx_FiRs39s8',
       2, 1, 31, FALSE, FALSE, 'publie', v_auteur, NOW(), v_auteur, NOW(), NOW());

    INSERT INTO media_content.creneau_programmation
        (id, type_support, support_id, emission_id, recurrence, jour_semaine,
         heure_debut, duree_minutes, fuseau, date_effet, actif, cree_par, created_at, updated_at)
    VALUES (gen_random_uuid(), 'chaine_tv', v_chaine, v_emission, 'hebdomadaire', 0,
            '18:00', 60, 'Africa/Abidjan', v_dimanche, TRUE, v_auteur, NOW(), NOW());

    -- ========================================================================
    -- 4. Correction de « Test1 » — chaîne publiée sans thématique
    -- ========================================================================
    SELECT id INTO v_chaine FROM media_content.chaine_tv WHERE nom = 'Test1' AND deleted_at IS NULL;
    IF v_chaine IS NOT NULL THEN
        FOR v_theme IN
            SELECT id FROM shared.categorie
             WHERE contexte = 'media' AND actif
               AND nom IN ('Journal télévisé', 'Émissions citoyennes')
        LOOP
            INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
            VALUES ('chaine_tv', v_chaine, v_theme) ON CONFLICT DO NOTHING;
        END LOOP;

        -- La chaîne porte déjà un pays de siège : sa couverture reprend ce
        -- territoire, faute de quoi elle resterait absente du filtre.
        INSERT INTO media_content.support_territoire (type_support, support_id, pays_id)
        SELECT 'chaine_tv', v_chaine, pays_id
          FROM media_content.chaine_tv WHERE id = v_chaine AND pays_id IS NOT NULL
        ON CONFLICT DO NOTHING;

        RAISE NOTICE 'Test1 complétée : thématiques et couverture posées.';
    END IF;

    RAISE NOTICE 'Seed 009 posé : 3 chaînes, 4 programmes, 13 épisodes, 4 créneaux.';
END
$seed$;

COMMIT;
