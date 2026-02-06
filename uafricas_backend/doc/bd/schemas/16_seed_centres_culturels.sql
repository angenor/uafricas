-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Seed : Centres Culturels (données de test)
-- ════════════════════════════════════════════════════════════════════════════

DO $$
DECLARE
    -- Pays
    pays_canada_id   UUID;
    pays_france_id   UUID;
    pays_civ_id      UUID;
    pays_senegal_id  UUID;

    -- Rôle utilisateur
    role_utilisateur_id UUID;

    -- Utilisateurs Montréal
    u_montreal_pres_id  UUID;
    u_montreal_vp_id    UUID;
    u_montreal_comm_id  UUID;

    -- Utilisateurs Paris
    u_paris_pres_id     UUID;
    u_paris_vp_id       UUID;
    u_paris_comm_id     UUID;

    -- Utilisateurs Abidjan
    u_abidjan_pres_id   UUID;
    u_abidjan_vp_id     UUID;
    u_abidjan_comm_id   UUID;

    -- Utilisateurs Dakar
    u_dakar_pres_id     UUID;
    u_dakar_vp_id       UUID;
    u_dakar_comm_id     UUID;

    -- Centres
    centre_montreal_id  UUID;
    centre_paris_id     UUID;
    centre_abidjan_id   UUID;
    centre_dakar_id     UUID;
BEGIN

    -- ── 1. Pays (INSERT ON CONFLICT pour ne pas dupliquer) ─────────────────

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Canada', 'CA', 'CAN', '+1', 'Ottawa', 'Amérique du Nord')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_canada_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('France', 'FR', 'FRA', '+33', 'Paris', 'Europe')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_france_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Côte d''Ivoire', 'CI', 'CIV', '+225', 'Yamoussoukro', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_civ_id;

    INSERT INTO shared.pays (nom, code_iso2, code_iso3, indicatif_tel, capitale, continent)
    VALUES ('Sénégal', 'SN', 'SEN', '+221', 'Dakar', 'Afrique')
    ON CONFLICT (code_iso2) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO pays_senegal_id;


    -- ── 2. Récupérer le rôle "utilisateur" ────────────────────────────────

    SELECT id INTO role_utilisateur_id FROM iam.role WHERE slug = 'utilisateur';


    -- ── 3. Utilisateurs de test ────────────────────────────────────────────
    -- Mot de passe : hash bcrypt de "Test1234!" (cost 12)

    -- Montréal
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Diallo', 'Amadou', 'amadou.diallo@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+1-514-555-0101', 'actif', TRUE, pays_canada_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_montreal_pres_id;

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Traoré', 'Fatou', 'fatou.traore@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+1-514-555-0102', 'actif', TRUE, pays_canada_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_montreal_vp_id;

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Koné', 'Ibrahim', 'ibrahim.kone@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+1-514-555-0103', 'actif', TRUE, pays_canada_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_montreal_comm_id;

    -- Paris
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Ndong', 'Marie', 'marie.ndong@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+33-6-55-00-01-04', 'actif', TRUE, pays_france_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_paris_pres_id;

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Ouédraogo', 'Seydou', 'seydou.ouedraogo@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+33-6-55-00-01-05', 'actif', TRUE, pays_france_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_paris_vp_id;

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Bamba', 'Aissatou', 'aissatou.bamba@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+33-6-55-00-01-06', 'actif', TRUE, pays_france_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_paris_comm_id;

    -- Abidjan
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Yao', 'Kouassi', 'kouassi.yao@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+225-07-55-00-07', 'actif', TRUE, pays_civ_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_abidjan_pres_id;

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Touré', 'Mariam', 'mariam.toure@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+225-07-55-00-08', 'actif', TRUE, pays_civ_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_abidjan_vp_id;

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Coulibaly', 'Moussa', 'moussa.coulibaly@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+225-07-55-00-09', 'actif', TRUE, pays_civ_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_abidjan_comm_id;

    -- Dakar
    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Sow', 'Ousmane', 'ousmane.sow@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+221-77-555-00-10', 'actif', TRUE, pays_senegal_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_dakar_pres_id;

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Ndiaye', 'Aminata', 'aminata.ndiaye@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+221-77-555-00-11', 'actif', TRUE, pays_senegal_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_dakar_vp_id;

    INSERT INTO iam.utilisateur (nom, prenom, email, mot_de_passe_hash, telephone, etat, email_verifie, pays_residence_id)
    VALUES ('Fall', 'Ibrahima', 'ibrahima.fall@uafricas.org', '$2b$12$LJ3m4ys4Dz/8hNdMBqZzJOqXFGxRJv5pK1vK0w7vC5/YJ1TyLxEHy', '+221-77-555-00-12', 'actif', TRUE, pays_senegal_id)
    ON CONFLICT (email) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO u_dakar_comm_id;


    -- ── 4. Attribution du rôle "utilisateur" ───────────────────────────────

    INSERT INTO iam.utilisateur_role (utilisateur_id, role_id, attribue_par) VALUES
        (u_montreal_pres_id, role_utilisateur_id, u_montreal_pres_id),
        (u_montreal_vp_id,   role_utilisateur_id, u_montreal_pres_id),
        (u_montreal_comm_id, role_utilisateur_id, u_montreal_pres_id),
        (u_paris_pres_id,    role_utilisateur_id, u_paris_pres_id),
        (u_paris_vp_id,      role_utilisateur_id, u_paris_pres_id),
        (u_paris_comm_id,    role_utilisateur_id, u_paris_pres_id),
        (u_abidjan_pres_id,  role_utilisateur_id, u_abidjan_pres_id),
        (u_abidjan_vp_id,    role_utilisateur_id, u_abidjan_pres_id),
        (u_abidjan_comm_id,  role_utilisateur_id, u_abidjan_pres_id),
        (u_dakar_pres_id,    role_utilisateur_id, u_dakar_pres_id),
        (u_dakar_vp_id,      role_utilisateur_id, u_dakar_pres_id),
        (u_dakar_comm_id,    role_utilisateur_id, u_dakar_pres_id)
    ON CONFLICT DO NOTHING;


    -- ── 5. Centres culturels ───────────────────────────────────────────────

    INSERT INTO culture.centre_culturel (nom, slug, description, pays_id, ville, adresse, latitude, longitude, cree_par)
    VALUES (
        'Montréal', 'montreal',
        'Le Centre Culturel Africain et Afro-Descendant de Montréal est un espace de rassemblement et de promotion des cultures africaines au Canada. Il organise des événements culturels, des ateliers et des conférences.',
        pays_canada_id, 'Montréal', '1234 Rue Saint-Denis, Montréal, QC H2X 3J6, Canada',
        45.5152559, -73.5611381,
        u_montreal_pres_id
    )
    ON CONFLICT (slug) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO centre_montreal_id;

    INSERT INTO culture.centre_culturel (nom, slug, description, pays_id, ville, adresse, latitude, longitude, cree_par)
    VALUES (
        'Paris', 'paris',
        'Le Centre Culturel Africain et Afro-Descendant de Paris est un lieu de référence pour la diaspora africaine en France. Il propose des expositions, des spectacles et des rencontres interculturelles.',
        pays_france_id, 'Paris', '45 Rue de Rivoli, 75004 Paris, France',
        48.8566, 2.3522,
        u_paris_pres_id
    )
    ON CONFLICT (slug) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO centre_paris_id;

    INSERT INTO culture.centre_culturel (nom, slug, description, pays_id, ville, adresse, latitude, longitude, cree_par)
    VALUES (
        'Abidjan', 'abidjan',
        'Le Centre Culturel Africain et Afro-Descendant d''Abidjan célèbre la richesse culturelle ivoirienne et panafricaine. Il accueille des artistes, des conteurs et des chercheurs.',
        pays_civ_id, 'Abidjan', 'Boulevard de la République, Plateau, Abidjan, Côte d''Ivoire',
        5.3599517, -4.0082563,
        u_abidjan_pres_id
    )
    ON CONFLICT (slug) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO centre_abidjan_id;

    INSERT INTO culture.centre_culturel (nom, slug, description, pays_id, ville, adresse, latitude, longitude, cree_par)
    VALUES (
        'Dakar', 'dakar',
        'Le Centre Culturel Africain et Afro-Descendant de Dakar est un carrefour des cultures ouest-africaines. Il met en avant la tradition orale, la musique et les arts visuels du Sénégal.',
        pays_senegal_id, 'Dakar', 'Avenue Léopold Sédar Senghor, Dakar, Sénégal',
        14.6928, -17.4467,
        u_dakar_pres_id
    )
    ON CONFLICT (slug) DO UPDATE SET nom = EXCLUDED.nom
    RETURNING id INTO centre_dakar_id;


    -- ── 6. Membres des centres ─────────────────────────────────────────────

    INSERT INTO culture.membre_centre (centre_culturel_id, utilisateur_id, role) VALUES
        -- Montréal
        (centre_montreal_id, u_montreal_pres_id, 'president'),
        (centre_montreal_id, u_montreal_vp_id,   'vice_president'),
        (centre_montreal_id, u_montreal_comm_id,  'resp_communication'),
        -- Paris
        (centre_paris_id,    u_paris_pres_id,     'president'),
        (centre_paris_id,    u_paris_vp_id,       'vice_president'),
        (centre_paris_id,    u_paris_comm_id,     'resp_communication'),
        -- Abidjan
        (centre_abidjan_id,  u_abidjan_pres_id,   'president'),
        (centre_abidjan_id,  u_abidjan_vp_id,     'vice_president'),
        (centre_abidjan_id,  u_abidjan_comm_id,   'resp_communication'),
        -- Dakar
        (centre_dakar_id,    u_dakar_pres_id,     'president'),
        (centre_dakar_id,    u_dakar_vp_id,       'vice_president'),
        (centre_dakar_id,    u_dakar_comm_id,     'resp_communication')
    ON CONFLICT DO NOTHING;


    -- ── 7. Programmations ──────────────────────────────────────────────────

    -- Montréal
    INSERT INTO culture.programmation_centre (centre_culturel_id, titre, description, lieu, mode, date_heure_debut, date_heure_fin, nombre_places, cree_par) VALUES
    (centre_montreal_id,
     'Festival des Arts Africains de Montréal',
     'Un festival célébrant la diversité des arts africains : musique, danse, arts visuels et gastronomie. Venez découvrir les talents de la diaspora montréalaise dans une ambiance festive et conviviale.',
     'Place des Arts, Montréal', 'presentiel',
     NOW() + INTERVAL '30 days', NOW() + INTERVAL '33 days', 200,
     u_montreal_pres_id),
    (centre_montreal_id,
     'Conférence : L''Afrique et le numérique',
     'Une conférence en ligne explorant les opportunités numériques pour le continent africain. Des intervenants de renom partageront leur expertise sur l''innovation technologique en Afrique.',
     NULL, 'en_ligne',
     NOW() + INTERVAL '15 days', NOW() + INTERVAL '15 days' + INTERVAL '3 hours', 500,
     u_montreal_comm_id);

    -- Paris
    INSERT INTO culture.programmation_centre (centre_culturel_id, titre, description, lieu, mode, date_heure_debut, date_heure_fin, nombre_places, cree_par) VALUES
    (centre_paris_id,
     'Exposition : Masques et Traditions',
     'Une exposition immersive présentant les masques traditionnels de différentes cultures africaines. Chaque pièce raconte une histoire, un rituel, une croyance ancestrale transmise de génération en génération.',
     'Centre Culturel de Paris, Salle d''exposition', 'presentiel',
     NOW() + INTERVAL '20 days', NOW() + INTERVAL '50 days', 150,
     u_paris_pres_id),
    (centre_paris_id,
     'Atelier de danse Sabar',
     'Initiez-vous à la danse Sabar, danse traditionnelle sénégalaise rythmée par les percussions. Un atelier ouvert à tous les niveaux pour découvrir l''énergie et la joie de cette danse populaire.',
     'Studio Danse Paris, 15e arrondissement', 'presentiel',
     NOW() + INTERVAL '10 days', NOW() + INTERVAL '10 days' + INTERVAL '2 hours', 30,
     u_paris_comm_id);

    -- Abidjan
    INSERT INTO culture.programmation_centre (centre_culturel_id, titre, description, lieu, mode, date_heure_debut, date_heure_fin, nombre_places, cree_par) VALUES
    (centre_abidjan_id,
     'Nuit des Contes Africains',
     'Une soirée magique de contes et de récits traditionnels africains. Les griots et conteurs vous transporteront dans l''univers fascinant des légendes du continent.',
     'Jardin botanique de Bingerville, Abidjan', 'presentiel',
     NOW() + INTERVAL '25 days', NOW() + INTERVAL '25 days' + INTERVAL '4 hours', 100,
     u_abidjan_pres_id),
    (centre_abidjan_id,
     'Webinaire : Entrepreneuriat et Culture',
     'Comment valoriser le patrimoine culturel africain à travers l''entrepreneuriat ? Des success stories inspirantes et des pistes concrètes pour entreprendre dans le secteur culturel.',
     NULL, 'en_ligne',
     NOW() + INTERVAL '12 days', NOW() + INTERVAL '12 days' + INTERVAL '2 hours', 300,
     u_abidjan_comm_id);

    -- Dakar
    INSERT INTO culture.programmation_centre (centre_culturel_id, titre, description, lieu, mode, date_heure_debut, date_heure_fin, nombre_places, cree_par) VALUES
    (centre_dakar_id,
     'Concert : Musiques du Sahel',
     'Un concert exceptionnel réunissant des musiciens du Mali, du Niger, du Burkina Faso et du Sénégal pour célébrer les sonorités uniques du Sahel. Kora, balafon et djembé au rendez-vous.',
     'Institut Français de Dakar', 'presentiel',
     NOW() + INTERVAL '18 days', NOW() + INTERVAL '18 days' + INTERVAL '3 hours', 250,
     u_dakar_pres_id),
    (centre_dakar_id,
     'Atelier hybride : Initiation au Wolof',
     'Apprenez les bases de la langue wolof dans un atelier interactif. Disponible en présentiel à Dakar et en ligne pour la diaspora.',
     'Centre Culturel de Dakar', 'hybride',
     NOW() + INTERVAL '8 days', NOW() + INTERVAL '8 days' + INTERVAL '2 hours', 50,
     u_dakar_comm_id);


    RAISE NOTICE 'Seed centres culturels : 4 pays, 12 utilisateurs, 4 centres, 12 membres, 8 programmations inserees.';

END $$;
