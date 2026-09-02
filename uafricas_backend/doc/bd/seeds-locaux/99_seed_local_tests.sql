-- ============================================================================
--  SEED LOCAL — POSTE DE DÉVELOPPEMENT UNIQUEMENT
-- ============================================================================
--
--  ⚠️  CE FICHIER NE DOIT JAMAIS ÊTRE EXÉCUTÉ EN PRODUCTION.
--
--  Il crée des données de démonstration et suppose l'existence de deux comptes
--  aux mots de passe connus et publiés dans `CLAUDE.md` — dont une ADRESSE RÉELLE.
--  Les faire exister sur
--  un environnement en service serait une porte d'entrée offerte.
--
--  C'est pourquoi il vit dans `doc/bd/seeds-locaux/` et NON dans `doc/bd/schemas/` :
--  `schema.sql` inclut les fichiers de `schemas/` par `\ir`, et `docker-init.sh`
--  exécute `schema.sql`. Un fichier posé dans `schemas/` finirait tôt ou tard
--  déployé. Ici, il ne peut être lancé qu'à la main.
--
--  Lancement :
--    PGPASSWORD=… psql -h 127.0.0.1 -p 5433 -U uafricas -d africans_db \
--                      -v ON_ERROR_STOP=1 -f doc/bd/seeds-locaux/99_seed_local_tests.sql
--
--  Idempotent : relançable sans doublon. Les comptes eux-mêmes ne sont PAS créés
--  ici — leur mot de passe doit être haché par le backend (bcrypt coût 12), pas
--  par un hash recopié dans un fichier. Voir l'en-tête « COMPTES » ci-dessous.
-- ============================================================================

-- ─────────────────────────────────────────────────────────────────────────────
--  COMPTES
--
--  À créer AVANT ce script, par l'API, pour que le hachage soit celui du
--  backend :
--
--    curl -X POST http://localhost:8080/api/auth/inscription \
--         -H 'Content-Type: application/json' \
--         -d '{"nom":"Test","prenom":"Membre","email":"martialdjezou@gmail.com",
--              "mot_de_passe":"Test1234","confirmation_mot_de_passe":"Test1234"}'
--
--    (idem pour test-admin@test.com, prénom « Admin »)
--
--  Le bloc suivant fait ce que le clic sur le lien de vérification aurait fait —
--  aucun SMTP n'écoute en local — puis attribue les rôles.
-- ─────────────────────────────────────────────────────────────────────────────

UPDATE iam.utilisateur
   SET etat = 'actif', email_verifie = TRUE
 WHERE email IN ('martialdjezou@gmail.com', 'test-admin@test.com');

INSERT INTO iam.utilisateur_role (utilisateur_id, role_id)
SELECT u.id, r.id
  FROM iam.utilisateur u
  JOIN iam.role r ON r.nom = 'Utilisateur'
 WHERE u.email IN ('martialdjezou@gmail.com', 'test-admin@test.com')
ON CONFLICT DO NOTHING;

-- L'admin reçoit les DEUX rôles : sans « Utilisateur », il perdrait les
-- permissions communes à tout compte et se retrouverait admin sans être membre.
INSERT INTO iam.utilisateur_role (utilisateur_id, role_id)
SELECT u.id, r.id
  FROM iam.utilisateur u
  JOIN iam.role r ON r.nom = 'Administrateur'
 WHERE u.email = 'test-admin@test.com'
ON CONFLICT DO NOTHING;

-- ─────────────────────────────────────────────────────────────────────────────
--  FONCTIONS DES MEMBRES
--
--  Le panneau « Membres à découvrir » du fil affiche la fonction sous le nom.
--  Elle est nulle sur tous les comptes semés, si bien que le panneau ne montrait
--  que des noms : la structure de la carte restait invisible.
-- ─────────────────────────────────────────────────────────────────────────────

UPDATE iam.utilisateur u
   SET fonction = v.fonction
  FROM (VALUES
    ('ibrahim.kone@uafricas.org',        'Photographe'),
    ('marie.ndong@uafricas.org',         'Cuisine & patrimoine'),
    ('seydou.ouedraogo@uafricas.org',    'Mode & Culture'),
    ('aissatou.bamba@uafricas.org',      'Agronome'),
    ('kouassi.yao@uafricas.org',         'Historien'),
    ('fatou.traore@uafricas.org',        'Conteuse')
  ) AS v(email, fonction)
 WHERE u.email = v.email AND u.fonction IS NULL;

-- ─────────────────────────────────────────────────────────────────────────────
--  AMITIÉS DU COMPTE DE TEST
--
--  Sans ami(e)s, la rangée en tête du fil n'affiche que le cercle « + » et le
--  panneau des ami(e)s de Codimoi reste vide. Trois suffisent à voir la forme.
--
--  `amitie` est symétrique par convention (une seule ligne par paire) : on range
--  toujours le plus petit identifiant en `utilisateur_a_id`, sinon la même
--  amitié pourrait exister deux fois dans les deux ordres.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO social.amitie (utilisateur_a_id, utilisateur_b_id)
SELECT LEAST(t.id, a.id), GREATEST(t.id, a.id)
  FROM iam.utilisateur t
  JOIN iam.utilisateur a
    ON a.email IN ('ibrahim.kone@uafricas.org',
                   'marie.ndong@uafricas.org',
                   'seydou.ouedraogo@uafricas.org')
 WHERE t.email = 'martialdjezou@gmail.com'
ON CONFLICT DO NOTHING;

-- ─────────────────────────────────────────────────────────────────────────────
--  PUBLICATIONS CODIMOI
--
--  Quatre publications, une par type, pour que le fil montre ses deux formes de
--  carte : le bloc coloré des proverbes et citations, et la carte à image des
--  ressources et bonnes pratiques.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO culture.codimoi (type, contenu, explication, nom_auteur_originel, couleur_fond, image_couverture_url, etat, nombre_likes, nombre_dislikes, cree_par)
SELECT v.type::culture.type_codimoi, v.contenu, v.explication, v.auteur, v.couleur, v.image, 'publie', v.likes, v.dislikes, u.id
  FROM (VALUES
    ('proverbe_adage',
     'Seul on va plus vite, ensemble on va plus loin.',
     'Proverbe répandu de l''Afrique de l''Ouest au Sahel. Il oppose la rapidité de l''individu à la portée du collectif — et donne raison au second dès qu''il s''agit de durer.',
     NULL, '#2D5A27', NULL, 42, 1),
    ('citation',
     'L''éducation est l''arme la plus puissante pour changer le monde.',
     'Prononcée par Nelson Mandela, elle place le savoir avant la force dans l''ordre des leviers de transformation.',
     'Nelson Mandela', '#1E3A5F', NULL, 128, 0),
    ('ressource_historique',
     'Les manuscrits de Tombouctou',
     'Des centaines de milliers de manuscrits rédigés entre le XIIIᵉ et le XVIIᵉ siècle : astronomie, droit, médecine, poésie. Ils démentent à eux seuls l''idée d''une Afrique sans écrit.',
     NULL, NULL, 'https://images.unsplash.com/photo-1461360370896-922624d12aa1?w=1200', 76, 2),
    ('bonne_pratique',
     'Le tontine, une épargne collective avant la banque',
     'Chacun verse à tour de rôle, chacun reçoit à son tour. Le mécanisme finance des projets là où le crédit bancaire n''arrive pas, et repose sur une garantie que nulle banque ne sait produire : la réputation dans le groupe.',
     NULL, NULL, 'https://images.unsplash.com/photo-1521791136064-7986c2920216?w=1200', 55, 0)
  ) AS v(type, contenu, explication, auteur, couleur, image, likes, dislikes)
  JOIN iam.utilisateur u ON u.email = 'martialdjezou@gmail.com'
 WHERE NOT EXISTS (SELECT 1 FROM culture.codimoi c WHERE c.contenu = v.contenu);

-- ─────────────────────────────────────────────────────────────────────────────
--  MOTS-DIÈSE
--
--  Le panneau « Tendances » du rail les compte sur le fil chargé. Sans eux, il
--  ne s'affiche pas du tout — c'est ce qui le rendait invisible en local.
--  Les répétitions sont voulues : un décompte où tout vaut 1 ne classe rien.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO shared.tag (nom, slug)
SELECT v.nom, v.slug
  FROM (VALUES
    ('Sagesse', 'sagesse'), ('Transmission', 'transmission'), ('Histoire', 'histoire'),
    ('Education', 'education'), ('Sahel', 'sahel'), ('Economie', 'economie')
  ) AS v(nom, slug)
 WHERE NOT EXISTS (SELECT 1 FROM shared.tag t WHERE t.slug = v.slug);

INSERT INTO culture.codimoi_tag (codimoi_id, tag_id)
SELECT c.id, t.id
  FROM (VALUES
    ('Seul on va plus vite, ensemble on va plus loin.',            'sagesse'),
    ('Seul on va plus vite, ensemble on va plus loin.',            'transmission'),
    ('Seul on va plus vite, ensemble on va plus loin.',            'sahel'),
    ('L''éducation est l''arme la plus puissante pour changer le monde.', 'sagesse'),
    ('L''éducation est l''arme la plus puissante pour changer le monde.', 'education'),
    ('Les manuscrits de Tombouctou',                               'histoire'),
    ('Les manuscrits de Tombouctou',                               'transmission'),
    ('Les manuscrits de Tombouctou',                               'sahel'),
    ('Le tontine, une épargne collective avant la banque',         'economie'),
    ('Le tontine, une épargne collective avant la banque',         'transmission')
  ) AS v(contenu, slug)
  JOIN culture.codimoi c ON c.contenu = v.contenu
  JOIN shared.tag t      ON t.slug    = v.slug
ON CONFLICT DO NOTHING;

-- ─────────────────────────────────────────────────────────────────────────────
--  SESSION AFROLANG EN DIRECT
--
--  Neuvième source du fil d'actualité, et pastille « Live en cours » sur la
--  page Afrolang. Les deux lisent `sessions_en_cours`, renvoyé par la liste des
--  salles.
--
--  ⚠️  ATTENTION, PIÈGE : ce compteur ne compte QUE les sessions des salles
--  PRIVÉES. Sa sous-requête (handlers/afrolang.rs) joint `salle_privee` :
--
--      SELECT COUNT(*) FROM afrolang.session ses2
--      JOIN afrolang.salle_privee sp3 ON sp3.id = ses2.salle_privee_id
--      WHERE sp3.salle_id = s.id AND ses2.etat = 'en_cours'
--
--  Une session ouverte directement sur une salle publique — ce que crée
--  `demarrer_ou_rejoindre_session_salle_publique` — n'est donc jamais comptée.
--  La salle reste affichée « Non démarrée » alors qu'elle est en direct.
--
--  Le seed contourne le problème plutôt que de le masquer : il ouvre la session
--  sur une salle PRIVÉE, seul cas que le compteur voit. La salle privée doit
--  exister au préalable — son code d'accès est haché par le backend, comme les
--  mots de passe :
--
--    curl -X POST http://localhost:8080/api/afrolang/salles-privees \
--         -H "Authorization: Bearer <jeton>" -H 'Content-Type: application/json' \
--         -d '{"salle_id":"<id>","titre":"Cours du soir — Baoulé débutants",
--              "description":"…","code_acces":"Test1234"}'
-- ─────────────────────────────────────────────────────────────────────────────

-- Nettoyage AVANT l'insertion, et non après : la garde de l'INSERT compte les
-- sessions en cours, et une session invisible du compteur la bloquerait sans
-- que rien ne s'affiche. L'ordre n'est pas cosmétique.
DELETE FROM afrolang.session
 WHERE etat = 'en_cours' AND salle_privee_id IS NULL
   AND titre = 'Atelier du soir — prononciation';

INSERT INTO afrolang.session (salle_privee_id, titre, etat, moderateur_id, demarre_at, cree_par)
SELECT sp.id,
       'Atelier du soir — prononciation',
       'en_cours',
       u.id,
       NOW() - INTERVAL '25 minutes',
       u.id
  FROM afrolang.salle_privee sp
  JOIN iam.utilisateur u ON u.email = 'test-admin@test.com'
 WHERE sp.deleted_at IS NULL
   -- Garde GLOBALE et non par salle : « une session en direct » est l'intention.
   -- Une garde par salle rejouerait le seed sur la salle suivante à chaque
   -- passe, et le fil finirait par afficher autant de directs que de salles.
   AND NOT EXISTS (SELECT 1 FROM afrolang.session x WHERE x.etat = 'en_cours')
 ORDER BY sp.created_at
 LIMIT 1;

-- ─────────────────────────────────────────────────────────────────────────────
--  CONTRIBUTIONS CITOYENNES — la source « gouvernance » du fil
--
--  Trois formes, et la carte du fil en rend trois variantes différentes :
--  FactCheck affiche le préjugé ET sa réfutation côte à côte, IdeaForces son
--  objectif, BadGoodhabits sa catégorie et son urgence. Sans les trois, deux
--  branches de la carte restent invisibles.
--
--  Le préjugé et la réalité vont PAR PAIRE : semer un préjugé sans sa
--  réfutation ferait circuler le préjugé tout seul.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO governance.factcheck (contenu, prejuge_titre, prejuge_description, realite_titre, realite_description, couleur_fond, verdict, etat, nombre_likes, cree_par)
SELECT v.contenu, v.pt, v.pd, v.rt, v.rd, v.couleur, v.verdict, 'publie', v.likes, u.id
  FROM (VALUES
    ('L''Afrique n''aurait pas connu l''écriture avant la colonisation',
     'L''Afrique n''avait pas d''écriture avant l''arrivée des Européens',
     'Une idée reçue tenace, qui sert à présenter la colonisation comme un apport de civilisation.',
     'Plusieurs systèmes d''écriture y sont nés ou y circulaient depuis des siècles',
     'Le guèze en Éthiopie dès le IVᵉ siècle, le tifinagh amazigh, l''ajami en caractères arabes pour le wolof, le peul et le haoussa, le n''ko en Guinée. Les manuscrits de Tombouctou se comptent en centaines de milliers.',
     '#1E3A5F', 'faux', 128),
    ('Le continent africain serait un bloc homogène',
     'On peut parler de « la culture africaine » au singulier',
     'Le raccourci gomme 54 pays, plus de 2 000 langues et des trajectoires historiques sans rapport entre elles.',
     'L''Afrique compte plus de diversité génétique et linguistique que tout le reste du monde réuni',
     'Deux populations d''Afrique subsaharienne peuvent être génétiquement plus éloignées l''une de l''autre qu''un Européen ne l''est d''un Asiatique.',
     '#6B2C5B', 'faux', 94)
  ) AS v(contenu, pt, pd, rt, rd, couleur, verdict, likes)
  JOIN iam.utilisateur u ON u.email = 'martialdjezou@gmail.com'
 WHERE NOT EXISTS (SELECT 1 FROM governance.factcheck f WHERE f.contenu = v.contenu);

INSERT INTO governance.idea_force (titre, pays_id, description_generale, details_proposition, categorie_proposition, urgence, impact_attendu, etat, nombre_soutiens, cree_par)
SELECT v.titre, p.id, v.descr, v.details, v.categorie, v.urgence::governance.niveau_gravite, v.impact, 'publie', v.soutiens, u.id
  FROM (VALUES
    ('Former les enseignants aux langues maternelles avant le français',
     'Les premières années de scolarité se jouent dans une langue que l''enfant ne parle pas encore. Le décrochage commence là.',
     'Former les enseignants du primaire à conduire les deux premières années dans la langue de l''élève, puis introduire le français progressivement. Les évaluations existantes montrent un gain durable en lecture, y compris en français.',
     'education_formation', 'elevee',
     'Réduction du redoublement au primaire et meilleure maîtrise du français en fin de cycle.', 47)
  ) AS v(titre, descr, details, categorie, urgence, impact, soutiens)
  JOIN iam.utilisateur u ON u.email = 'test-admin@test.com'
  JOIN shared.pays p ON p.nom = 'Sénégal'
 WHERE NOT EXISTS (SELECT 1 FROM governance.idea_force i WHERE i.titre = v.titre);

INSERT INTO governance.bad_habit (titre, pays_id, description_generale, details_problematique, categorie_probleme, gravite, solutions_proposees, etat, nombre_soutiens, cree_par)
SELECT v.titre, p.id, v.descr, v.details, v.categorie, v.gravite::governance.niveau_gravite, v.solutions, 'publie', v.soutiens, u.id
  FROM (VALUES
    ('Les ordures déversées dans les canaux de drainage',
     'Chaque saison des pluies, les mêmes quartiers sont inondés. Les canaux sont bouchés bien avant la première averse.',
     'Le ramassage ne passe pas dans les quartiers non lotis, et les canaux deviennent le dépôt par défaut. L''inondation qui suit n''est pas un aléa climatique, c''est le résultat prévisible de neuf mois d''accumulation.',
     'insalubrite', 'elevee',
     'Points de collecte de proximité et curage avant saison, plutôt que l''évacuation d''urgence une fois les maisons sous l''eau.', 63)
  ) AS v(titre, descr, details, categorie, gravite, solutions, soutiens)
  JOIN iam.utilisateur u ON u.email = 'martialdjezou@gmail.com'
  JOIN shared.pays p ON p.nom = 'Côte d''Ivoire'
 WHERE NOT EXISTS (SELECT 1 FROM governance.bad_habit b WHERE b.titre = v.titre);

-- ─────────────────────────────────────────────────────────────────────────────
--  PARTAGES — quatre sources de plus dans le fil
--
--  Un partage relaie un objet existant. La carte du fil est la même pour les
--  six sources de partage ; ce qui change est le verbe, le badge et l'aperçu.
-- ─────────────────────────────────────────────────────────────────────────────

-- Territoire partagé
INSERT INTO country_profile.partage_fiche (fiche_pays_id, utilisateur_id, legende)
SELECT f.id, u.id, v.legende
  FROM (VALUES
    ('Sénégal',        'La Teranga n''est pas un slogan touristique : c''est une obligation sociale, et elle se mesure à ce qu''on doit à celui qui arrive.'),
    ('Côte d''Ivoire', 'Premier producteur mondial de cacao, et pourtant l''essentiel de la valeur se crée ailleurs. La fiche dit où.')
  ) AS v(nom, legende)
  -- La fiche ne porte PAS le nom du territoire : il vit dans `shared.pays`,
  -- la fiche n'en tient que la clé.
  JOIN shared.pays pa ON pa.nom = v.nom
  JOIN country_profile.fiche_pays f ON f.pays_id = pa.id
  JOIN iam.utilisateur u ON u.email = 'martialdjezou@gmail.com'
 WHERE NOT EXISTS (SELECT 1 FROM country_profile.partage_fiche pf WHERE pf.fiche_pays_id = f.id AND pf.utilisateur_id = u.id);

-- Profil partagé
INSERT INTO social.partage_profil (profil_id, utilisateur_id, legende)
SELECT p.id, u.id, v.legende
  FROM (VALUES
    ('marie.ndong@uafricas.org',      'Elle documente les recettes de sa grand-mère avant qu''elles ne se perdent. Suivez-la.'),
    ('kouassi.yao@uafricas.org',      'Historien, il remet les royaumes précoloniaux à leur place dans le récit.')
  ) AS v(email, legende)
  JOIN iam.utilisateur p ON p.email = v.email
  JOIN iam.utilisateur u ON u.email = 'test-admin@test.com'
 WHERE NOT EXISTS (SELECT 1 FROM social.partage_profil sp WHERE sp.profil_id = p.id AND sp.utilisateur_id = u.id);

-- Contribution partagée : relaie l'une des contributions semées plus haut.
INSERT INTO governance.partage_contribution (type_contribution, contribution_id, utilisateur_id, legende)
SELECT 'factcheck', f.id, u.id, 'À faire lire à tous ceux qui répètent que l''écriture est arrivée avec les bateaux.'
  FROM governance.factcheck f
  JOIN iam.utilisateur u ON u.email = 'test-admin@test.com'
 WHERE f.contenu = 'L''Afrique n''aurait pas connu l''écriture avant la colonisation'
   AND NOT EXISTS (SELECT 1 FROM governance.partage_contribution pc WHERE pc.contribution_id = f.id AND pc.utilisateur_id = u.id);

-- ─────────────────────────────────────────────────────────────────────────────
--  AFRICANITÉS — publications éphémères en tête du fil (spec 012)
--
--  Trois formes, pour que la visionneuse montre ses trois rendus : le texte sur
--  fond coloré, l'image, et — laissée de côté ici — la vidéo, qu'aucun fichier
--  d'exemple n'accompagne dans le dépôt.
--
--  L'échéance est REPOUSSÉE à chaque exécution du seed. Sans cela, relancer le
--  script le lendemain laisserait des africanités échues que rien n'affiche, et
--  la rangée paraîtrait cassée alors qu'elle applique sa règle.
--
--  Une des trois est semée pour un(e) ami(e) : sans elle, on ne verrait jamais
--  l'anneau « non vue » d'autrui, seulement sa propre pastille.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO social.africanite (auteur_id, forme, texte, couleur_fond, media_url, legende, expire_at)
SELECT u.id, v.forme, v.texte, v.couleur, v.media, v.legende, NOW() + INTERVAL '24 hours'
  FROM (VALUES
    ('martialdjezou@gmail.com', 'texte',
     'Le soleil se lève sur la corniche. Bonne journée à toutes et à tous.',
     '#A74916', NULL, 'Dakar, 6 h 40'),
    ('martialdjezou@gmail.com', 'texte',
     'On ne mange pas le fruit d''un arbre qu''on n''a pas arrosé.',
     '#1C8C1C', NULL, 'Ce que ma grand-mère répétait'),
    ('martialdjezou@gmail.com', 'image',
     NULL, NULL,
     'https://images.unsplash.com/photo-1516026672322-bc52d61a55d5?w=1200',
     'Le marché, juste avant l''ouverture'),
    ('ibrahim.kone@uafricas.org', 'texte',
     'Trois heures de lumière rasante. Les meilleures photos de la semaine.',
     '#1E3A5F', NULL, 'En repérage')
  ) AS v(email, forme, texte, couleur, media, legende)
  JOIN iam.utilisateur u ON u.email = v.email
 WHERE NOT EXISTS (
   SELECT 1 FROM social.africanite a
    WHERE a.auteur_id = u.id
      AND a.legende IS NOT DISTINCT FROM v.legende
      AND a.deleted_at IS NULL
 );

-- Repousse l'échéance des africanités semées : elles doivent rester actives à
-- chaque relance, sinon le seed produirait un jeu invisible.
UPDATE social.africanite a
   SET expire_at = NOW() + INTERVAL '24 hours'
  FROM iam.utilisateur u
 WHERE u.id = a.auteur_id
   AND u.email IN ('martialdjezou@gmail.com', 'ibrahim.kone@uafricas.org')
   AND a.deleted_at IS NULL
   AND a.expire_at <= NOW() + INTERVAL '1 hour';

-- ─────────────────────────────────────────────────────────────────────────────
--  CONTRÔLE
-- ─────────────────────────────────────────────────────────────────────────────

\echo ''
\echo 'Récapitulatif du seed local :'
-- Les deux adresses sont énumérées et non filtrées par motif : le compte
-- membre porte une adresse réelle, aucun `LIKE 'test-%'` ne la reconnaîtrait.
SELECT '  comptes de test actifs      : ' || count(*) FROM iam.utilisateur
 WHERE email IN ('martialdjezou@gmail.com', 'test-admin@test.com') AND etat = 'actif';
SELECT '  ami(e)s du compte membre    : ' || count(*) FROM social.amitie a
  JOIN iam.utilisateur u ON u.id IN (a.utilisateur_a_id, a.utilisateur_b_id)
 WHERE u.email = 'martialdjezou@gmail.com';
SELECT '  publications Codimoi        : ' || count(*) FROM culture.codimoi WHERE deleted_at IS NULL;
SELECT '  associations mot-dièse      : ' || count(*) FROM culture.codimoi_tag;
SELECT '  membres avec une fonction   : ' || count(*) FROM iam.utilisateur WHERE fonction IS NOT NULL;
SELECT '  contributions citoyennes    : ' || ((SELECT count(*) FROM governance.factcheck) + (SELECT count(*) FROM governance.idea_force) + (SELECT count(*) FROM governance.bad_habit));
SELECT '  partages (fiche/profil/contr): ' || (SELECT count(*) FROM country_profile.partage_fiche) || ' / ' || (SELECT count(*) FROM social.partage_profil) || ' / ' || (SELECT count(*) FROM governance.partage_contribution);
SELECT '  africanités actives         : ' || count(*) FROM social.africanite WHERE deleted_at IS NULL AND expire_at > NOW();
-- Deux nombres, et l'écart entre eux est INSTRUCTIF : la première ligne compte
-- toutes les sessions ouvertes, la seconde celles que le listing sait montrer.
-- Un écart signale des sessions de salle PUBLIQUE, que `sessions_en_cours`
-- ignore (voir l'avertissement plus haut).
SELECT '  sessions ouvertes (toutes)  : ' || count(*) FROM afrolang.session WHERE etat = 'en_cours';
SELECT '  salles vues en direct       : ' || count(*) FROM afrolang.salle s
 WHERE EXISTS (SELECT 1 FROM afrolang.session ses JOIN afrolang.salle_privee sp ON sp.id = ses.salle_privee_id
                WHERE sp.salle_id = s.id AND ses.etat = 'en_cours');

-- ═══════════════════════════════════════════════════════════════════════
-- Symboles nationaux (migration 11l) — Afrique du Sud
-- ═══════════════════════════════════════════════════════════════════════
--
-- La maquette Figma de la fiche territoire montre SEPT symboles renseignés.
-- Sans données, la colonne de droite reste presque vide et le rendu ne se
-- compare à rien. Ces valeurs sont FACTUELLES (dates d'adoption officielles,
-- emblèmes reconnus), pas du texte de remplissage — mais elles restent dans
-- le seed LOCAL : en production, ce sont les administrateurs qui saisissent
-- les 54 fiches, chacune avec ses propres symboles.
--
-- Idempotent : le WHERE ne repasse que sur les colonnes encore vides, une
-- saisie manuelle ultérieure n'est donc jamais écrasée.

UPDATE country_profile.fiche_pays fp
   SET hymne_national      = COALESCE(fp.hymne_national, 'Nkosi Sikelel'' iAfrika / Die Stem van Suid-Afrika'),
       hymne_description   = COALESCE(fp.hymne_description, 'Un chant combinant le Nkosi Sikelel'' iAfrika et l''ancien Die Stem van Suid-Afrika, chanté en plusieurs langues officielles.'),
       drapeau_description = COALESCE(fp.drapeau_description, 'Adopté le 27 avril 1994, il possède un « Y » horizontal qui unit les couleurs rouge, blanc, bleu, vert, noir et jaune.'),
       embleme_description = COALESCE(fp.embleme_description, 'Adoptées le 27 avril 2000, elles montrent un soleil levant, une protée royale, des défenses d''éléphant et la devise « !ke e: ǀxarra ǁke » (Peuples divers, unis).'),
       fleur_nationale     = COALESCE(fp.fleur_nationale, 'Protée royal (Protea cynaroides)'),
       fleur_description   = COALESCE(fp.fleur_description, 'Elle représente la beauté et l''épanouissement.'),
       animal_national     = COALESCE(fp.animal_national, 'Springbok'),
       animal_description  = COALESCE(fp.animal_description, 'Une antilope qui symbolise la grâce et la vitesse, et qui donne son nom à l''équipe nationale de rugby.'),
       oiseau_national     = COALESCE(fp.oiseau_national, 'Grue bleue (Anthropoides paradiseus)'),
       oiseau_description  = COALESCE(fp.oiseau_description, 'Un oiseau élégant des prairies du pays.')
  FROM shared.pays p
 WHERE p.id = fp.pays_id
   AND LOWER(p.code_iso2) = 'za';

SELECT '  symboles Afrique du Sud     : ' || CASE WHEN count(*) > 0 THEN 'renseignés' ELSE 'fiche absente' END
  FROM country_profile.fiche_pays fp
  JOIN shared.pays p ON p.id = fp.pays_id
 WHERE LOWER(p.code_iso2) = 'za' AND fp.fleur_nationale IS NOT NULL;
