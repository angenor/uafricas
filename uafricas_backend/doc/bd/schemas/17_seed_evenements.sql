-- ════════════════════════════════════════════════════════════════════════════
-- SEED : Événements (media_content.evenement + inscriptions)
-- ════════════════════════════════════════════════════════════════════════════

DO $$
DECLARE
    -- Pays
    pays_ci_id   UUID;
    pays_sn_id   UUID;
    pays_ca_id   UUID;
    pays_fr_id   UUID;
    pays_ml_id   UUID;
    pays_cm_id   UUID;
    pays_ke_id   UUID;
    pays_ng_id   UUID;
    pays_gh_id   UUID;
    pays_bf_id   UUID;

    -- Utilisateurs
    user_amadou_id   UUID;
    user_fatou_id    UUID;
    user_kouassi_id  UUID;
    user_mariam_id   UUID;
    user_ousmane_id  UUID;
    user_aminata_id  UUID;
    user_marie_id    UUID;
    user_seydou_id   UUID;
    user_ibrahim_id  UUID;
    user_moussa_id   UUID;

    -- Événements
    evt_id UUID;

BEGIN
    -- ── Récupérer les pays existants ──────────────────────────────
    SELECT id INTO pays_ci_id FROM shared.pays WHERE code_iso2 = 'CI';
    SELECT id INTO pays_sn_id FROM shared.pays WHERE code_iso2 = 'SN';
    SELECT id INTO pays_ca_id FROM shared.pays WHERE code_iso2 = 'CA';
    SELECT id INTO pays_fr_id FROM shared.pays WHERE code_iso2 = 'FR';

    -- Pays africains : récupérer s'ils existent, sinon les créer
    SELECT id INTO pays_ml_id FROM shared.pays WHERE LOWER(nom) = 'mali';
    IF pays_ml_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Mali', 'ML', 'MLI', '+223', 'Bamako', 'Afrique', TRUE)
        RETURNING id INTO pays_ml_id;
    END IF;

    SELECT id INTO pays_cm_id FROM shared.pays WHERE LOWER(nom) = 'cameroun';
    IF pays_cm_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Cameroun', 'CM', 'CMR', '+237', 'Yaoundé', 'Afrique', TRUE)
        RETURNING id INTO pays_cm_id;
    END IF;

    SELECT id INTO pays_ke_id FROM shared.pays WHERE LOWER(nom) = 'kenya';
    IF pays_ke_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Kenya', 'KE', 'KEN', '+254', 'Nairobi', 'Afrique', TRUE)
        RETURNING id INTO pays_ke_id;
    END IF;

    SELECT id INTO pays_ng_id FROM shared.pays WHERE LOWER(nom) = 'nigeria';
    IF pays_ng_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Nigeria', 'NG', 'NGA', '+234', 'Abuja', 'Afrique', TRUE)
        RETURNING id INTO pays_ng_id;
    END IF;

    SELECT id INTO pays_gh_id FROM shared.pays WHERE LOWER(nom) = 'ghana';
    IF pays_gh_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Ghana', 'GH', 'GHA', '+233', 'Accra', 'Afrique', TRUE)
        RETURNING id INTO pays_gh_id;
    END IF;

    SELECT id INTO pays_bf_id FROM shared.pays WHERE LOWER(nom) = 'burkina faso';
    IF pays_bf_id IS NULL THEN
        INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent, actif)
        VALUES ('Burkina Faso', 'BF', 'BFA', '+226', 'Ouagadougou', 'Afrique', TRUE)
        RETURNING id INTO pays_bf_id;
    END IF;

    -- ── Récupérer les utilisateurs existants ──────────────────────
    SELECT id INTO user_amadou_id  FROM iam.utilisateur WHERE email = 'amadou.diallo@uafricas.org';
    SELECT id INTO user_fatou_id   FROM iam.utilisateur WHERE email = 'fatou.traore@uafricas.org';
    SELECT id INTO user_kouassi_id FROM iam.utilisateur WHERE email = 'kouassi.yao@uafricas.org';
    SELECT id INTO user_mariam_id  FROM iam.utilisateur WHERE email = 'mariam.toure@uafricas.org';
    SELECT id INTO user_ousmane_id FROM iam.utilisateur WHERE email = 'ousmane.sow@uafricas.org';
    SELECT id INTO user_aminata_id FROM iam.utilisateur WHERE email = 'aminata.ndiaye@uafricas.org';
    SELECT id INTO user_marie_id   FROM iam.utilisateur WHERE email = 'marie.ndong@uafricas.org';
    SELECT id INTO user_seydou_id  FROM iam.utilisateur WHERE email = 'seydou.ouedraogo@uafricas.org';
    SELECT id INTO user_ibrahim_id FROM iam.utilisateur WHERE email = 'ibrahim.kone@uafricas.org';
    SELECT id INTO user_moussa_id  FROM iam.utilisateur WHERE email = 'moussa.coulibaly@uafricas.org';

    -- ══════════════════════════════════════════════════════════════
    -- ÉVÉNEMENTS
    -- ══════════════════════════════════════════════════════════════

    -- 1. ForAfrica - Forum des Valeurs Africaines (Présentiel, Mali, passé)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville, adresse,
         date_heure_debut, date_heure_fin, image_couverture_url,
         nombre_places, etat, cree_par)
    VALUES (
        'ForAfrica - Forum des Valeurs Africaines',
        'forafrica-forum-valeurs-africaines',
        'Forum annuel de discussion sur les valeurs africaines traditionnelles et leur pertinence dans le monde moderne. Échanges entre experts, chercheurs et acteurs culturels du continent.',
        'presentiel', pays_ml_id, 'Bamako', 'Palais de la Culture Amadou Hampâté Bâ',
        NOW() - INTERVAL '30 days', NOW() - INTERVAL '29 days',
        'https://images.unsplash.com/photo-1540575467063-178a50c2df87?w=800',
        200, 'publie', user_amadou_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    -- Inscriptions pour cet événement
    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_fatou_id, 'inscrit'), (evt_id, user_kouassi_id, 'inscrit'), (evt_id, user_marie_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 2. Webinaire: Langues africaines à l'ère numérique (En ligne, Sénégal, à venir)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville,
         date_heure_debut, date_heure_fin, image_couverture_url,
         lien_en_ligne, nombre_places, etat, cree_par)
    VALUES (
        'Webinaire: Langues africaines à l''ère numérique',
        'webinaire-langues-africaines-numerique',
        'Découvrez comment les technologies numériques peuvent contribuer à la préservation et à la promotion des langues africaines. Session interactive avec démonstrations et ateliers pratiques.',
        'en_ligne', pays_sn_id, 'Dakar',
        NOW() + INTERVAL '15 days', NOW() + INTERVAL '15 days' + INTERVAL '2 hours 30 minutes',
        'https://images.unsplash.com/photo-1591115765373-5207764f72e7?w=800',
        'https://meet.uafricas.org/langues-numerique', 100, 'publie', user_ousmane_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_aminata_id, 'inscrit'), (evt_id, user_amadou_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 3. Atelier de musique traditionnelle Mandingue (Présentiel, Côte d'Ivoire, à venir)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville, adresse,
         date_heure_debut, date_heure_fin, image_couverture_url,
         nombre_places, etat, cree_par)
    VALUES (
        'Atelier de musique traditionnelle Mandingue',
        'atelier-musique-mandingue',
        'Initiation aux instruments traditionnels mandingues: kora, balafon et djembé. Atelier pratique animé par des griots de renommée internationale. Ouvert à tous les niveaux.',
        'presentiel', pays_ci_id, 'Abidjan', 'Centre Culturel UAfricas Abidjan',
        NOW() + INTERVAL '25 days', NOW() + INTERVAL '25 days' + INTERVAL '8 hours',
        'https://images.unsplash.com/photo-1516450360452-9312f5e86fc7?w=800',
        30, 'publie', user_kouassi_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_mariam_id, 'inscrit'), (evt_id, user_moussa_id, 'inscrit'),
           (evt_id, user_amadou_id, 'inscrit'), (evt_id, user_fatou_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 4. Conférence: Économie circulaire en Afrique (Hybride, Kenya, à venir)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville, adresse,
         date_heure_debut, date_heure_fin, image_couverture_url,
         lien_en_ligne, nombre_places, etat, cree_par)
    VALUES (
        'Conférence: Économie circulaire en Afrique',
        'conference-economie-circulaire-afrique',
        'Table ronde sur les modèles économiques durables inspirés des pratiques traditionnelles africaines. Intervenants de 10 pays africains. Sessions en présentiel et en ligne.',
        'hybride', pays_ke_id, 'Nairobi', 'Kenyatta International Convention Centre',
        NOW() + INTERVAL '40 days', NOW() + INTERVAL '41 days',
        'https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=800',
        'https://meet.uafricas.org/economie-circulaire', 300, 'publie', user_seydou_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_amadou_id, 'inscrit'), (evt_id, user_ousmane_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 5. Festival de la Gastronomie Africaine (Présentiel, Cameroun, à venir)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville, adresse,
         date_heure_debut, date_heure_fin, image_couverture_url,
         nombre_places, etat, cree_par)
    VALUES (
        'Festival de la Gastronomie Africaine',
        'festival-gastronomie-africaine',
        'Célébration des saveurs du continent avec des chefs venus de toute l''Afrique. Dégustations, ateliers culinaires et échanges culturels autour de la cuisine africaine.',
        'presentiel', pays_cm_id, 'Douala', 'Palais des Congrès de Douala',
        NOW() + INTERVAL '60 days', NOW() + INTERVAL '62 days',
        'https://images.unsplash.com/photo-1504674900247-0877df9cc836?w=800',
        500, 'publie', user_marie_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_kouassi_id, 'inscrit'), (evt_id, user_mariam_id, 'inscrit'),
           (evt_id, user_fatou_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 6. Hackathon: Solutions IA pour l'Afrique (Hybride, Nigeria, à venir)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville, adresse,
         date_heure_debut, date_heure_fin, image_couverture_url,
         lien_en_ligne, nombre_places, etat, cree_par)
    VALUES (
        'Hackathon: Solutions IA pour l''Afrique',
        'hackathon-ia-afrique',
        'Compétition de développement de solutions d''intelligence artificielle répondant aux défis africains. Prix et mentorat pour les gagnants. 48h de coding intensif.',
        'hybride', pays_ng_id, 'Lagos', 'Landmark Centre Victoria Island',
        NOW() + INTERVAL '75 days', NOW() + INTERVAL '77 days',
        'https://images.unsplash.com/photo-1531482615713-2afd69097998?w=800',
        'https://meet.uafricas.org/hackathon-ia', 150, 'publie', user_ibrahim_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_moussa_id, 'inscrit'), (evt_id, user_seydou_id, 'inscrit'),
           (evt_id, user_aminata_id, 'inscrit'), (evt_id, user_ousmane_id, 'inscrit'),
           (evt_id, user_kouassi_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 7. Séminaire: Leadership féminin en Afrique (En ligne, Ghana, à venir)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville,
         date_heure_debut, date_heure_fin, image_couverture_url,
         lien_en_ligne, nombre_places, etat, cree_par)
    VALUES (
        'Séminaire: Leadership féminin en Afrique',
        'seminaire-leadership-feminin-afrique',
        'Rencontre inspirante avec des femmes leaders africaines. Partage d''expériences et réseautage pour la nouvelle génération de dirigeantes du continent.',
        'en_ligne', pays_gh_id, 'Accra',
        NOW() + INTERVAL '20 days', NOW() + INTERVAL '20 days' + INTERVAL '3 hours',
        'https://images.unsplash.com/photo-1573164713988-8665fc963095?w=800',
        'https://meet.uafricas.org/leadership-feminin', 200, 'publie', user_aminata_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_fatou_id, 'inscrit'), (evt_id, user_mariam_id, 'inscrit'),
           (evt_id, user_marie_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 8. Exposition: Art contemporain africain (Présentiel, France, en cours)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville, adresse,
         date_heure_debut, date_heure_fin, image_couverture_url,
         nombre_places, etat, cree_par)
    VALUES (
        'Exposition: Art contemporain africain',
        'exposition-art-contemporain-africain',
        'Vernissage et exposition d''artistes africains contemporains explorant l''identité et la modernité à travers leurs œuvres. Plus de 50 artistes de 20 pays.',
        'presentiel', pays_fr_id, 'Paris', 'Musée du Quai Branly - Jacques Chirac',
        NOW() - INTERVAL '5 days', NOW() + INTERVAL '25 days',
        'https://images.unsplash.com/photo-1544967082-d9d25d867d66?w=800',
        NULL, 'publie', user_marie_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_seydou_id, 'inscrit'), (evt_id, user_amadou_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 9. Formation: Entrepreneuriat social (Hybride, Burkina Faso, à venir 2026)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville, adresse,
         date_heure_debut, date_heure_fin, image_couverture_url,
         lien_en_ligne, nombre_places, etat, cree_par)
    VALUES (
        'Formation: Entrepreneuriat social',
        'formation-entrepreneuriat-social',
        'Programme intensif de formation pour entrepreneurs sociaux africains. Méthodologie, financement et mise en réseau. 5 jours de formation pratique.',
        'hybride', pays_bf_id, 'Ouagadougou', 'Université de Ouagadougou',
        NOW() + INTERVAL '90 days', NOW() + INTERVAL '95 days',
        'https://images.unsplash.com/photo-1552664730-d307ca884978?w=800',
        'https://meet.uafricas.org/entrepreneuriat-social', 50, 'publie', user_seydou_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_ousmane_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    -- 10. Colloque: Médecine traditionnelle africaine (Présentiel, Canada, à venir)
    INSERT INTO media_content.evenement
        (titre, slug, description, format, pays_id, ville, adresse,
         date_heure_debut, date_heure_fin, image_couverture_url,
         nombre_places, etat, cree_par)
    VALUES (
        'Colloque: Médecine traditionnelle africaine',
        'colloque-medecine-traditionnelle-africaine',
        'Échanges entre praticiens de la médecine traditionnelle et chercheurs modernes sur l''intégration des savoirs ancestraux dans les systèmes de santé contemporains.',
        'presentiel', pays_ca_id, 'Montréal', 'Centre Culturel UAfricas Montréal',
        NOW() + INTERVAL '50 days', NOW() + INTERVAL '52 days',
        'https://images.unsplash.com/photo-1576091160550-2173dba999ef?w=800',
        80, 'publie', user_amadou_id
    )
    ON CONFLICT (slug) DO UPDATE SET titre = EXCLUDED.titre
    RETURNING id INTO evt_id;

    INSERT INTO media_content.evenement_inscription (evenement_id, utilisateur_id, statut)
    VALUES (evt_id, user_fatou_id, 'inscrit'), (evt_id, user_ibrahim_id, 'inscrit'),
           (evt_id, user_marie_id, 'inscrit'), (evt_id, user_aminata_id, 'inscrit')
    ON CONFLICT (evenement_id, utilisateur_id) DO NOTHING;

    RAISE NOTICE '✅ Seed événements : 10 événements + inscriptions insérés avec succès';
END $$;
