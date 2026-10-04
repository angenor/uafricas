-- ============================================================================
--  SEED LOCAL — ACTIVITÉS LUDIQUES (feature 013)
-- ============================================================================
--
--  ⚠️  POSTE DE DÉVELOPPEMENT UNIQUEMENT. Ce fichier vit dans `seeds-locaux/`,
--  hors de `schemas/` : `schema.sql` ne l'inclut pas, il n'est donc jamais
--  déployé. Il suppose les deux comptes de test de `99_seed_local_tests.sql`.
--
--  Ordre : 99 (comptes), 98 (Afripulse), puis 95.
--
--    docker exec -i uafricas_postgres psql -U uafricas -d africans_db \
--      -v ON_ERROR_STOP=1 < uafricas_backend/doc/bd/seeds-locaux/95_seed_local_jeu.sql
--
--  Pourquoi ce seed existe : hors Afripulse, le contenu dont on peut tirer des
--  épreuves est presque absent des seeds (un proverbe sans pays, deux
--  factchecks). Sans lui, la dérivation de Codimoi et de FactCheck ne produit
--  rien et ne peut pas être recettée (research.md, D14).
--
--  Idempotent : rejouable sans doublon. Aucun UUID en dur : comptes et pays sont
--  résolus par leur clé naturelle (e-mail, code ISO).
-- ============================================================================

-- ─────────────────────────────────────────────────────────────────────────────
--  LES DEUX JOUEURS
--
--  Deux pays d'origine DIFFÉRENTS, pour que les classements par pays et la
--  carte aient deux lignes à montrer. Amis, pour que le duel soit possible.
-- ─────────────────────────────────────────────────────────────────────────────

UPDATE iam.utilisateur u
   SET pays_origine_id = p.id
  FROM (VALUES
    ('martialdjezou@gmail.com', 'CI'),
    ('test-admin@test.com',     'SN')
  ) AS v(email, iso2)
  JOIN shared.pays p ON p.code_iso2 = v.iso2
 WHERE u.email = v.email AND u.pays_origine_id IS NULL;

INSERT INTO social.amitie (utilisateur_a_id, utilisateur_b_id)
SELECT LEAST(a.id, b.id), GREATEST(a.id, b.id)
  FROM iam.utilisateur a
  JOIN iam.utilisateur b ON b.email = 'test-admin@test.com'
 WHERE a.email = 'martialdjezou@gmail.com'
ON CONFLICT DO NOTHING;

-- ─────────────────────────────────────────────────────────────────────────────
--  UNE SAISON EN COURS
--
--  Sans saison, on joue mais aucun classement de Championship ne se remplit.
--  La contrainte d'exclusion interdit le chevauchement : on n'insère que s'il
--  n'existe aucune saison couvrant maintenant.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO jeu.saison (nom, debut_at, fin_at)
SELECT 'Saison d''ouverture', date_trunc('month', NOW()), date_trunc('month', NOW()) + interval '3 months'
 WHERE NOT EXISTS (
    SELECT 1 FROM jeu.saison s
     WHERE tstzrange(s.debut_at, s.fin_at, '[)')
        && tstzrange(date_trunc('month', NOW()), date_trunc('month', NOW()) + interval '3 months', '[)'));

-- ─────────────────────────────────────────────────────────────────────────────
--  CODIMOI — 12 proverbes AVEC pays, 8 citations AVEC auteur
--
--  Le proverbe sans pays est le cas courant en base (le formulaire ne l'exige
--  pas) : la forme « de quel pays vient ce proverbe » n'en tire rien. Douze
--  pays distincts donnent des distracteurs en nombre.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO culture.codimoi (type, contenu, explication, pays_id, couleur_fond, etat, cree_par)
SELECT 'proverbe_adage'::culture.type_codimoi, v.contenu, v.explication, p.id, v.couleur, 'publie', u.id
  FROM (VALUES
    ('Le mensonge donne des fleurs mais pas de fruits.', 'Ce qui est faux séduit un temps, puis ne produit rien.', 'NG', '#2D5A27'),
    ('Quand les racines sont profondes, il n''y a aucune raison de craindre le vent.', 'Qui connaît ses origines tient debout dans l''épreuve.', 'SN', '#1E3A5F'),
    ('L''eau chaude n''oublie pas qu''elle a été froide.', 'On ne renie pas ce qu''on a été.', 'CI', '#6B2C5B'),
    ('Celui qui pose une question ne s''égare pas.', 'Demander son chemin n''a jamais perdu personne.', 'CM', '#8B4513'),
    ('La pirogue ne se moque pas du crocodile avant d''avoir traversé le fleuve.', 'On ne nargue pas le danger tant qu''on y est exposé.', 'ML', '#2D5A27'),
    ('Un seul bracelet ne tinte pas.', 'Rien d''important ne se fait seul.', 'CD', '#1E3A5F'),
    ('La patience est la clé du bien-être.', 'Ce qui se fait dans la hâte se défait de même.', 'MA', '#6B2C5B'),
    ('Qui apprend, enseigne.', 'Le savoir reçu oblige à le transmettre.', 'ET', '#8B4513'),
    ('La pluie ne tombe pas sur un seul toit.', 'Les épreuves comme les chances sont partagées.', 'KE', '#2D5A27'),
    ('Le bâton que tu tiens est celui qui tue le serpent.', 'On agit avec ce qu''on a sous la main, pas avec ce qu''on espère.', 'GH', '#1E3A5F'),
    ('L''arbre ne tombe pas au premier coup de hache.', 'La persévérance vient à bout de ce que l''élan ne fait pas céder.', 'BF', '#6B2C5B'),
    ('La langue qui fourche fait plus de mal que le pied qui trébuche.', 'Une parole de travers blesse plus qu''un faux pas.', 'MG', '#8B4513')
  ) AS v(contenu, explication, iso2, couleur)
  JOIN shared.pays p ON p.code_iso2 = v.iso2
  JOIN iam.utilisateur u ON u.email = 'martialdjezou@gmail.com'
 WHERE NOT EXISTS (SELECT 1 FROM culture.codimoi c WHERE c.contenu = v.contenu);

INSERT INTO culture.codimoi (type, contenu, explication, nom_auteur_originel, couleur_fond, etat, cree_par)
SELECT 'citation'::culture.type_codimoi, v.contenu, v.explication, v.auteur, v.couleur, 'publie', u.id
  FROM (VALUES
    ('Cela semble toujours impossible, jusqu''à ce que ce soit fait.', 'Sur ce que l''on tient pour infaisable tant que personne ne l''a fait.', 'Nelson Mandela', '#1E3A5F'),
    ('L''esclave qui n''est pas capable d''assumer sa révolte ne mérite pas que l''on s''apitoie sur son sort.', 'Discours sur la responsabilité de sa propre libération.', 'Thomas Sankara', '#6B2C5B'),
    ('En Afrique, quand un vieillard meurt, c''est une bibliothèque qui brûle.', 'Sur la tradition orale et l''urgence de la recueillir.', 'Amadou Hampâté Bâ', '#8B4513'),
    ('Il n''y a pas de frontière raciale, ethnique ou culturelle à l''intelligence.', 'Sur l''unité du genre humain face au savoir.', 'Cheikh Anta Diop', '#2D5A27'),
    ('Quand nous plantons des arbres, nous plantons les graines de la paix et de l''espoir.', 'Sur le lien entre l''environnement et la paix.', 'Wangari Maathai', '#1E3A5F'),
    ('L''émotion est nègre, comme la raison est hellène.', 'Formule célèbre, et très discutée, de la négritude.', 'Léopold Sédar Senghor', '#6B2C5B'),
    ('Nous devons oser inventer l''avenir.', 'Sur le refus de reproduire des modèles importés.', 'Thomas Sankara', '#8B4513'),
    ('Tant que les lions n''auront pas leurs propres historiens, les histoires de chasse glorifieront toujours le chasseur.', 'Sur qui écrit l''histoire, et pour qui.', 'Chinua Achebe', '#2D5A27')
  ) AS v(contenu, explication, auteur, couleur)
  JOIN iam.utilisateur u ON u.email = 'martialdjezou@gmail.com'
 WHERE NOT EXISTS (SELECT 1 FROM culture.codimoi c WHERE c.contenu = v.contenu);

-- ─────────────────────────────────────────────────────────────────────────────
--  FACTCHECK — 12 idées reçues TRANCHÉES (verdict vrai ou faux)
--
--  Seuls les verdicts `vrai` et `faux` donnent une épreuve « vrai ou faux » :
--  un « partiellement vrai » n'a pas de bonne réponse à deux propositions.
--  Six de chaque, sinon répondre toujours « Faux » suffirait à tout gagner.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO governance.factcheck (contenu, prejuge_titre, prejuge_description, realite_titre, realite_description, couleur_fond, verdict, etat, cree_par)
SELECT v.pt, v.pt, v.pd, v.rt, v.rd, '#1E3A5F', v.verdict, 'publie', u.id
  FROM (VALUES
    ('L''Afrique est un pays', 'Le raccourci est fréquent dans le langage courant.', 'C''est un continent de 55 États', 'L''Union africaine compte 55 États membres, aux langues, monnaies et institutions distinctes.', 'faux'),
    ('Le Sahara couvre la majeure partie de l''Afrique', 'L''image du désert domine les représentations.', 'Il en couvre environ un tiers', 'Le Sahara s''étend sur un peu plus de 9 millions de km², pour un continent de 30 millions.', 'faux'),
    ('Le Nil coule du sud vers le nord', 'Beaucoup pensent qu''un fleuve descend « vers le bas de la carte ».', 'Oui, il se jette dans la Méditerranée', 'Né dans la région des Grands Lacs, le Nil traverse le Soudan et l''Égypte jusqu''à la Méditerranée.', 'vrai'),
    ('Le swahili n''est parlé qu''au Kenya', 'On l''associe souvent à un seul pays.', 'C''est une langue de plus de dix pays', 'Le swahili est parlé en Tanzanie, au Kenya, en Ouganda, en RDC, au Rwanda, au Burundi et au-delà.', 'faux'),
    ('L''Éthiopie n''a jamais été colonisée durablement', 'On suppose que tout le continent l''a été.', 'Exact, hormis une occupation italienne de cinq ans', 'Victorieuse à Adoua en 1896, l''Éthiopie n''a connu qu''une occupation italienne de 1936 à 1941.', 'vrai'),
    ('Le Nigeria est le pays le plus peuplé d''Afrique', 'L''ordre de grandeur est souvent ignoré.', 'Oui, avec plus de 200 millions d''habitants', 'Le Nigeria devance largement l''Éthiopie et l''Égypte.', 'vrai'),
    ('On ne trouve pas de neige en Afrique', 'Le continent est associé à la chaleur.', 'Il neige sur plusieurs massifs', 'Le Kilimandjaro, le mont Kenya, le Rwenzori et l''Atlas marocain connaissent la neige.', 'faux'),
    ('L''Afrique compte plus de 2 000 langues', 'Le chiffre paraît exagéré.', 'Oui, c''est le continent le plus divers linguistiquement', 'Les inventaires recensent plus de 2 000 langues, soit près d''un tiers des langues du monde.', 'vrai'),
    ('Tombouctou est une ville légendaire qui n''existe pas', 'Son nom sert d''image pour « le bout du monde ».', 'C''est une ville du Mali, inscrite au patrimoine mondial', 'Tombouctou fut un centre savant majeur ; ses manuscrits se comptent en centaines de milliers.', 'faux'),
    ('Le lac Victoria est le plus grand lac d''Afrique', 'On hésite souvent avec le Tanganyika.', 'Oui, par sa superficie', 'Avec près de 69 000 km², le lac Victoria est le plus étendu ; le Tanganyika est le plus profond.', 'vrai'),
    ('L''Afrique est le continent le plus jeune par sa population', 'L''idée est parfois contestée.', 'Oui, l''âge médian y est d''environ 19 ans', 'Plus de la moitié des Africains ont moins de vingt ans.', 'vrai'),
    ('Les pyramides n''existent qu''en Égypte', 'Celles de Gizeh éclipsent les autres.', 'Le Soudan en compte davantage', 'Le royaume de Koush a laissé plus de 200 pyramides à Méroé et Nuri, plus que l''Égypte.', 'faux')
  ) AS v(pt, pd, rt, rd, verdict)
  JOIN iam.utilisateur u ON u.email = 'martialdjezou@gmail.com'
 WHERE NOT EXISTS (SELECT 1 FROM governance.factcheck f WHERE f.contenu = v.pt);

-- ─────────────────────────────────────────────────────────────────────────────
--  AFROLANG — 12 épreuves SAISIES, déjà jouables
--
--  Afrolang n'a aucune forme de dérivation (pas de référentiel de langues) :
--  ses épreuves sont de la saisie. Douze, pour qu'une partie de dix soit
--  possible dès le premier lancement.
-- ─────────────────────────────────────────────────────────────────────────────

INSERT INTO jeu.epreuve (module_code, enonce, propositions, bonne_reponse, explication, difficulte, theme, origine, etat, cree_par, valide_par, valide_at)
SELECT 'afrolang', v.enonce, v.propositions, v.bonne, v.explication, v.difficulte, v.theme, 'saisie', 'jouable', u.id, u.id, NOW()
  FROM (VALUES
    ('Comment dit-on « bonjour » en swahili ?', ARRAY['Jambo', 'Sawubona', 'Nanga def', 'Akwaba'], 1, '« Jambo » est le salut swahili le plus connu ; « Sawubona » est zoulou, « Nanga def » wolof, « Akwaba » baoulé.', 1, 'Salutations'),
    ('« Akwaba » signifie « bienvenue ». De quelle langue vient ce mot ?', ARRAY['Wolof', 'Baoulé', 'Lingala', 'Haoussa'], 2, '« Akwaba » appartient aux langues akan, dont le baoulé, parlé en Côte d''Ivoire.', 1, 'Salutations'),
    ('Dans quelle langue dit-on « Nanga def ? » pour demander « comment vas-tu ? »', ARRAY['Peul', 'Bambara', 'Wolof', 'Swahili'], 3, 'C''est la salutation wolof ; on y répond « Maa ngi fi rekk ».', 1, 'Salutations'),
    ('Que signifie « Ubuntu » dans les langues nguni ?', ARRAY['La terre des ancêtres', 'Je suis parce que nous sommes', 'Le chemin du retour', 'La parole donnée'], 2, 'Ubuntu désigne l''humanité partagée : on n''est une personne que par les autres.', 2, 'Vocabulaire'),
    ('« Hakuna matata » est une expression de quelle langue ?', ARRAY['Swahili', 'Zoulou', 'Amharique', 'Yoruba'], 1, 'Expression swahili qui signifie « il n''y a pas de problème ».', 1, 'Vocabulaire'),
    ('Quelle langue s''écrit avec l''alphasyllabaire guèze ?', ARRAY['Le somali', 'L''amharique', 'Le haoussa', 'Le malgache'], 2, 'L''amharique, langue officielle de l''Éthiopie, utilise le fidäl hérité du guèze.', 2, 'Écritures'),
    ('Le n''ko est un alphabet créé en 1949 pour quelles langues ?', ARRAY['Les langues mandingues', 'Les langues bantoues', 'Les langues berbères', 'Les langues nilotiques'], 1, 'Solomana Kanté l''a conçu en Guinée pour le maninka, le bambara et le dioula.', 3, 'Écritures'),
    ('Comment s''appelle l''alphabet des langues amazighes ?', ARRAY['L''ajami', 'Le tifinagh', 'Le vaï', 'Le bamoun'], 2, 'Le tifinagh est l''écriture amazighe, officielle au Maroc depuis 2003 dans sa forme moderne.', 2, 'Écritures'),
    ('Le lingala est une langue véhiculaire de quelle région ?', ARRAY['Le bassin du Congo', 'La vallée du Nil', 'Le golfe de Guinée', 'La Corne de l''Afrique'], 1, 'Le lingala est parlé des deux côtés du fleuve, à Kinshasa comme à Brazzaville.', 1, 'Géographie des langues'),
    ('Quelle est la langue africaine qui compte le plus de locuteurs, langue seconde comprise ?', ARRAY['Le yoruba', 'Le zoulou', 'Le swahili', 'L''amharique'], 3, 'Le swahili dépasse les 150 millions de locuteurs si l''on compte ceux qui l''ont comme langue seconde.', 2, 'Géographie des langues'),
    ('À quelle famille appartient le haoussa ?', ARRAY['Nigéro-congolaise', 'Afro-asiatique', 'Nilo-saharienne', 'Khoïsan'], 2, 'Le haoussa est une langue tchadique, branche de la famille afro-asiatique, comme l''arabe et l''amharique.', 3, 'Familles de langues'),
    ('Le malgache est apparenté à des langues parlées…', ARRAY['en Afrique australe', 'en Asie du Sud-Est', 'au Moyen-Orient', 'en Afrique de l''Ouest'], 2, 'Le malgache est une langue austronésienne, proche de langues de Bornéo.', 3, 'Familles de langues')
  ) AS v(enonce, propositions, bonne, explication, difficulte, theme)
  JOIN iam.utilisateur u ON u.email = 'test-admin@test.com'
 WHERE NOT EXISTS (SELECT 1 FROM jeu.epreuve e WHERE e.enonce = v.enonce);

-- ─────────────────────────────────────────────────────────────────────────────
--  CONTRÔLE
-- ─────────────────────────────────────────────────────────────────────────────

SELECT '   joueurs avec pays d''origine : ' || count(*) FROM iam.utilisateur
 WHERE email IN ('martialdjezou@gmail.com', 'test-admin@test.com') AND pays_origine_id IS NOT NULL;
SELECT '   saisons en cours            : ' || count(*) FROM jeu.saison WHERE debut_at <= NOW() AND NOW() < fin_at;
SELECT '   sources visibles ' || rpad(type_source, 20) || ': ' || count(*) FILTER (WHERE visible)
  FROM jeu.v_source GROUP BY type_source ORDER BY type_source;
SELECT '   épreuves ' || rpad(module_code || ' / ' || etat, 24) || ': ' || count(*)
  FROM jeu.epreuve GROUP BY module_code, etat ORDER BY 1;
