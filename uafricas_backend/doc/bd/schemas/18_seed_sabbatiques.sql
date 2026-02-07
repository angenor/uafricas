-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Seed : Programmes d'échange sabbatique (données de test)
-- ════════════════════════════════════════════════════════════════════════════

-- Ajouter la colonne interafricain si elle n'existe pas
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'exchange' AND table_name = 'programme' AND column_name = 'interafricain'
    ) THEN
        ALTER TABLE exchange.programme ADD COLUMN interafricain BOOLEAN NOT NULL DEFAULT TRUE;
    END IF;
END $$;

DO $$
DECLARE
    -- Pays
    pays_senegal_id    UUID;
    pays_civ_id        UUID;
    pays_ghana_id      UUID;
    pays_mali_id       UUID;
    pays_kenya_id      UUID;
    pays_rwanda_id     UUID;
    pays_cameroun_id   UUID;

    -- Domaines
    domaine_edu_id     UUID;
    domaine_infra_id   UUID;
    domaine_sante_id   UUID;
    domaine_eau_id     UUID;
    domaine_dev_id     UUID;
    domaine_energie_id UUID;
    domaine_agri_id    UUID;

    -- Utilisateur createur
    u_createur_id      UUID;
    role_utilisateur_id UUID;

BEGIN

    -- ── 1. Pays (INSERT ON CONFLICT) ──────────────────────────────────────

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Sénégal', 'SN', 'SEN', '+221', 'Dakar', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_senegal_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Côte d''Ivoire', 'CI', 'CIV', '+225', 'Yamoussoukro', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_civ_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Ghana', 'GH', 'GHA', '+233', 'Accra', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_ghana_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Mali', 'ML', 'MLI', '+223', 'Bamako', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_mali_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Kenya', 'KE', 'KEN', '+254', 'Nairobi', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_kenya_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Rwanda', 'RW', 'RWA', '+250', 'Kigali', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_rwanda_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Cameroun', 'CM', 'CMR', '+237', 'Yaoundé', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_cameroun_id;

    -- ── 2. Domaines (récupérer les IDs existants du seed principal) ───────

    SELECT id INTO domaine_edu_id     FROM shared.domaine_secteur WHERE slug = 'education';
    SELECT id INTO domaine_infra_id   FROM shared.domaine_secteur WHERE slug = 'infrastructure';
    SELECT id INTO domaine_sante_id   FROM shared.domaine_secteur WHERE slug = 'sante';
    SELECT id INTO domaine_eau_id     FROM shared.domaine_secteur WHERE slug = 'eau';
    SELECT id INTO domaine_dev_id     FROM shared.domaine_secteur WHERE slug = 'developpement-localites';
    SELECT id INTO domaine_energie_id FROM shared.domaine_secteur WHERE slug = 'energie';
    SELECT id INTO domaine_agri_id    FROM shared.domaine_secteur WHERE slug = 'agriculture';

    -- ── 3. Utilisateur créateur (créer un utilisateur de test) ────────────

    SELECT id INTO role_utilisateur_id FROM iam.role WHERE slug = 'utilisateur';

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, etat, email_verifie)
    VALUES ('Diallo', 'Aminata', 'aminata.diallo@uafricas.org',
            '$2b$12$LJ3m4ys4yKq8q0k.ABC123fakeHashForSeedOnly00000000000000', 'actif', TRUE)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_createur_id;

    INSERT INTO iam.utilisateur_role (utilisateur_id, role_id)
    VALUES (u_createur_id, role_utilisateur_id)
    ON CONFLICT DO NOTHING;

    -- ── 4. Programmes sabbatiques ─────────────────────────────────────────

    -- Programme 1 : Interafricain - Agriculture / Sénégal
    INSERT INTO exchange.programme (
        titre, slug, description,
        image_couverture_url, pays_id, ville,
        prise_en_charge_billet, prise_en_charge_hebergement, prise_en_charge_subsistance,
        duree, domaine_id, date_debut, date_fin,
        nombre_places, interafricain, etat, cree_par
    ) VALUES (
        'Programme d''échange en ingénierie agricole',
        'programme-echange-ingenierie-agricole',
        '<p>Venez partager votre expertise en techniques agricoles modernes avec les agriculteurs locaux du Sénégal. Ce programme de 6 mois vous permettra de former des coopératives agricoles aux méthodes durables et à l''utilisation efficace des ressources en eau.</p><p>Vous travaillerez avec des équipes locales pour améliorer les rendements tout en préservant l''environnement.</p>',
        'https://images.unsplash.com/photo-1574943320219-553eb213f72d?w=800',
        pays_senegal_id, 'Thiès',
        TRUE, TRUE, TRUE,
        '6_mois'::exchange.duree_programme, domaine_agri_id,
        '2026-03-15', '2026-09-15',
        5, TRUE, 'publie', u_createur_id
    );

    -- Programme 2 : Interafricain - Santé / Côte d'Ivoire
    INSERT INTO exchange.programme (
        titre, slug, description,
        image_couverture_url, pays_id, ville,
        prise_en_charge_billet, prise_en_charge_hebergement, prise_en_charge_subsistance,
        duree, domaine_id, date_debut, date_fin,
        nombre_places, interafricain, etat, cree_par
    ) VALUES (
        'Mission santé communautaire en zone rurale',
        'mission-sante-communautaire-zone-rurale',
        '<p>Programme de formation des agents de santé communautaires en Côte d''Ivoire. Vous participerez à la mise en place de centres de santé de proximité et formerez les personnels locaux aux premiers soins et à la prévention des maladies.</p><p>Une expérience enrichissante au service des populations rurales.</p>',
        'https://images.unsplash.com/photo-1576091160550-2173dba999ef?w=800',
        pays_civ_id, 'Bouaké',
        FALSE, TRUE, TRUE,
        '3_semaines'::exchange.duree_programme, domaine_sante_id,
        '2026-04-01', '2026-04-22',
        8, TRUE, 'publie', u_createur_id
    );

    -- Programme 3 : Interafricain - Éducation / Ghana
    INSERT INTO exchange.programme (
        titre, slug, description,
        image_couverture_url, pays_id, ville,
        prise_en_charge_billet, prise_en_charge_hebergement, prise_en_charge_subsistance,
        duree, domaine_id, date_debut, date_fin,
        nombre_places, interafricain, etat, cree_par
    ) VALUES (
        'Développement de programmes éducatifs au Ghana',
        'developpement-programmes-educatifs-ghana',
        '<p>Rejoignez notre équipe pour développer des programmes éducatifs innovants dans les écoles primaires du Ghana. Vous travaillerez sur l''intégration des technologies numériques dans l''enseignement et formerez les enseignants locaux aux nouvelles méthodes pédagogiques.</p>',
        'https://images.unsplash.com/photo-1503676260728-1c00da094a0b?w=800',
        pays_ghana_id, 'Accra',
        TRUE, TRUE, FALSE,
        '2_mois'::exchange.duree_programme, domaine_edu_id,
        '2026-05-10', '2026-07-10',
        4, TRUE, 'publie', u_createur_id
    );

    -- Programme 4 : Interafricain - Infrastructures / Mali
    INSERT INTO exchange.programme (
        titre, slug, description,
        image_couverture_url, pays_id, ville,
        prise_en_charge_billet, prise_en_charge_hebergement, prise_en_charge_subsistance,
        duree, domaine_id, date_debut, date_fin,
        nombre_places, interafricain, etat, cree_par
    ) VALUES (
        'Construction d''infrastructures scolaires au Mali',
        'construction-infrastructures-scolaires-mali',
        '<p>Participez à un projet de construction et rénovation d''écoles dans la région de Ségou. Ce programme combine travail manuel et transfert de compétences en techniques de construction durable.</p><p>Vous travaillerez avec des artisans locaux et contribuerez à améliorer l''accès à l''éducation.</p>',
        'https://images.unsplash.com/photo-1541888946425-d81bb19240f5?w=800',
        pays_mali_id, 'Ségou',
        FALSE, TRUE, TRUE,
        '1_mois'::exchange.duree_programme, domaine_infra_id,
        '2026-06-01', '2026-07-01',
        10, TRUE, 'publie', u_createur_id
    );

    -- Programme 5 : Hors Afrique - Développement local / Sénégal
    INSERT INTO exchange.programme (
        titre, slug, description,
        image_couverture_url, pays_id, ville,
        prise_en_charge_billet, prise_en_charge_hebergement, prise_en_charge_subsistance,
        duree, domaine_id, date_debut, date_fin,
        nombre_places, interafricain, etat, cree_par
    ) VALUES (
        'Expertise en gestion municipale - France vers Sénégal',
        'expertise-gestion-municipale-france-senegal',
        '<p>Programme d''échange destiné aux experts en gestion municipale souhaitant partager leur expérience avec les collectivités locales sénégalaises.</p><p>Vous accompagnerez les équipes municipales dans la modernisation de leurs services publics et la gestion des ressources.</p>',
        'https://images.unsplash.com/photo-1517245386807-bb43f82c33c4?w=800',
        pays_senegal_id, 'Saint-Louis',
        TRUE, TRUE, TRUE,
        '6_mois'::exchange.duree_programme, domaine_dev_id,
        '2026-02-01', '2026-08-01',
        3, FALSE, 'publie', u_createur_id
    );

    -- Programme 6 : Hors Afrique - Énergie / Kenya
    INSERT INTO exchange.programme (
        titre, slug, description,
        image_couverture_url, pays_id, ville,
        prise_en_charge_billet, prise_en_charge_hebergement, prise_en_charge_subsistance,
        duree, domaine_id, date_debut, date_fin,
        nombre_places, interafricain, etat, cree_par
    ) VALUES (
        'Formation en énergies renouvelables - USA vers Kenya',
        'formation-energies-renouvelables-usa-kenya',
        '<p>Programme d''échange pour ingénieurs spécialisés en énergies renouvelables. Vous formerez des techniciens kényans à l''installation et la maintenance de systèmes solaires dans les zones rurales non raccordées au réseau électrique.</p>',
        'https://images.unsplash.com/photo-1509391366360-2e959784a276?w=800',
        pays_kenya_id, 'Nairobi',
        TRUE, TRUE, FALSE,
        '3_semaines'::exchange.duree_programme, domaine_energie_id,
        '2026-03-10', '2026-03-31',
        6, FALSE, 'publie', u_createur_id
    );

    -- Programme 7 : Hors Afrique - Santé / Rwanda
    INSERT INTO exchange.programme (
        titre, slug, description,
        image_couverture_url, pays_id, ville,
        prise_en_charge_billet, prise_en_charge_hebergement, prise_en_charge_subsistance,
        duree, domaine_id, date_debut, date_fin,
        nombre_places, interafricain, etat, cree_par
    ) VALUES (
        'Échange médical Canada-Rwanda',
        'echange-medical-canada-rwanda',
        '<p>Programme d''échange pour médecins et infirmiers souhaitant contribuer au renforcement du système de santé rwandais.</p><p>Vous travaillerez dans des hôpitaux régionaux et participerez à la formation continue du personnel médical local.</p>',
        'https://images.unsplash.com/photo-1551601651-2a8555f1a136?w=800',
        pays_rwanda_id, 'Kigali',
        FALSE, TRUE, TRUE,
        '2_mois'::exchange.duree_programme, domaine_sante_id,
        '2026-04-15', '2026-06-15',
        4, FALSE, 'publie', u_createur_id
    );

    RAISE NOTICE 'Seed sabbatiques : 7 programmes insérés avec succès';

END $$;
