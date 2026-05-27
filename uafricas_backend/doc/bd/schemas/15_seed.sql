-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Données de référence (seed)
-- ════════════════════════════════════════════════════════════════════════════


-- ── Rôles système ───────────────────────────────────────────────────────

INSERT INTO iam.role (nom, slug, description, est_systeme) VALUES
    ('Super Administrateur', 'super_admin',  'Accès total à la plateforme',           TRUE),
    ('Administrateur',       'admin',         'Gestion courante de la plateforme',     TRUE),
    ('Modérateur',           'moderateur',    'Modération des contenus et utilisateurs', TRUE),
    ('Utilisateur',          'utilisateur',   'Utilisateur standard de la plateforme', TRUE);


-- ── Domaines / Secteurs ─────────────────────────────────────────────────

INSERT INTO shared.domaine_secteur (nom, slug) VALUES
    ('Éducation',                              'education'),
    ('Infrastructure',                         'infrastructure'),
    ('Santé',                                  'sante'),
    ('Eau',                                    'eau'),
    ('Développement des Localités',            'developpement-localites'),
    ('Agriculture',                            'agriculture'),
    ('Technologie & Innovation',               'technologie-innovation'),
    ('Environnement',                          'environnement'),
    ('Gouvernance',                            'gouvernance'),
    ('Culture & Patrimoine',                   'culture-patrimoine'),
    ('Économie & Entrepreneuriat',             'economie-entrepreneuriat'),
    ('Énergie',                                'energie');


-- ── Spécialités Bibliothèque Humaine ────────────────────────────────────

INSERT INTO iam.specialite_bibliotheque (nom, slug) VALUES
    ('Immigration africaine',                  'immigration-africaine'),
    ('Colonisation',                           'colonisation'),
    ('Société africaine',                      'societe-africaine'),
    ('Histoire',                               'histoire'),
    ('Éducation et conseils à l''africaine',   'education-conseils-africaine'),
    ('Mariage en Afrique',                     'mariage-afrique'),
    ('Contes et proverbes',                    'contes-proverbes'),
    ('Migrations',                             'migrations'),
    ('Alliances entre peuples',                'alliances-peuples'),
    ('Totems et interdits',                    'totems-interdits'),
    ('Rites et initiations de peuples',        'rites-initiations'),
    ('Bonnes pratiques africaines',            'bonnes-pratiques-africaines'),
    ('Hommes historiques populaires',          'hommes-historiques-populaires'),
    ('Culture générale',                       'culture-generale'),
    ('Savoirs et innovations',                 'savoirs-innovations'),
    ('Spirituel et religions',                 'spirituel-religions');


-- ── Permissions de base ─────────────────────────────────────────────────

INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    -- Wildcard
    ('Accès total',                    'all.all',               '*',           '*'),
    -- Utilisateurs
    ('Voir les utilisateurs',          'utilisateur.voir',      'utilisateur', 'voir'),
    ('Modifier les utilisateurs',      'utilisateur.modifier',  'utilisateur', 'modifier'),
    ('Supprimer les utilisateurs',     'utilisateur.supprimer', 'utilisateur', 'supprimer'),
    ('Bloquer les utilisateurs',       'utilisateur.bloquer',   'utilisateur', 'bloquer'),
    -- Annonces
    ('Créer une annonce',              'annonce.creer',         'annonce',     'creer'),
    ('Modifier une annonce',           'annonce.modifier',      'annonce',     'modifier'),
    ('Supprimer une annonce',          'annonce.supprimer',     'annonce',     'supprimer'),
    ('Valider une annonce',            'annonce.valider',       'annonce',     'valider'),
    -- Programmes d'échange
    ('Créer un programme d''échange',  'programme.creer',       'programme',   'creer'),
    ('Valider un programme d''échange','programme.valider',     'programme',   'valider'),
    -- Innovations
    ('Suspendre une innovation',       'innovation.suspendre',  'innovation',  'suspendre'),
    -- Projets
    ('Approuver un projet',            'projet.approuver',      'projet',      'approuver'),
    -- Contenus médias
    ('Gérer les événements',           'evenement.gerer',       'evenement',   'gerer'),
    ('Gérer les MOOC',                 'mooc.gerer',            'mooc',        'gerer'),
    ('Gérer les livres',               'livre.gerer',           'livre',       'gerer'),
    ('Gérer les radios/télés',         'radio_tele.gerer',      'radio_tele',  'gerer'),
    -- Centres culturels
    ('Gérer les centres culturels',    'centre_culturel.gerer', 'centre_culturel', 'gerer'),
    -- Afrolang
    ('Créer salle Afrolang publique',  'afrolang.creer_pub',    'afrolang',    'creer_publique'),
    -- Gouvernance
    ('Modérer le FactCheck',           'factcheck.moderer',     'factcheck',   'moderer'),
    ('Modérer les BadHabits',          'bad_habit.moderer',     'bad_habit',   'moderer'),
    ('Modérer les IdeaForces',         'idea_force.moderer',    'idea_force',  'moderer'),
    -- Fiches pays
    ('Gérer les fiches pays',          'fiche_pays.gerer',      'fiche_pays',  'gerer'),
    -- Expertise (demandes pour devenir expert)
    ('Voir les demandes d''expertise',  'expertise.voir',       'expertise',   'voir'),
    ('Valider une demande d''expertise','expertise.valider',    'expertise',   'valider');


-- Attribution des permissions au rôle super_admin
INSERT INTO iam.role_permission (role_id, permission_id)
SELECT r.id, p.id
FROM iam.role r, iam.permission p
WHERE r.slug = 'super_admin' AND p.slug = 'all.all';
