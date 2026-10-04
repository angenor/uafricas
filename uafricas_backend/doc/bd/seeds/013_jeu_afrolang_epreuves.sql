-- ============================================================================
--  CONTENU — ÉPREUVES AFROLANG À REVOIR (feature 013)
-- ============================================================================
--
--  Afrolang n'a aucune forme de dérivation (la plateforme n'a pas de
--  référentiel de langues) : ses épreuves sont de la saisie. Ce lot en propose
--  44, sur cinq thèmes : salutations, vocabulaire, géographie des langues,
--  écritures, familles de langues.
--
--  Elles naissent `candidate` : AUCUNE n'est jouable avant d'être acceptée dans
--  /admin/activites/revue. C'est voulu — un contenu linguistique se relit par
--  un locuteur avant d'être servi, et la revue permet de corriger un énoncé.
--
--  Applicable en production comme en local (aucun compte, aucun UUID en dur) ;
--  idempotent : une épreuve dont l'énoncé existe déjà n'est pas réinsérée.
--
--    ./deploy.sh migrate uafricas_backend/doc/bd/seeds/013_jeu_afrolang_epreuves.sql
--    docker exec -i uafricas_postgres psql -U uafricas -d africans_db \
--      -v ON_ERROR_STOP=1 < uafricas_backend/doc/bd/seeds/013_jeu_afrolang_epreuves.sql
-- ============================================================================

INSERT INTO jeu.epreuve (module_code, enonce, propositions, bonne_reponse, explication, difficulte, theme, origine, etat)
SELECT 'afrolang', v.enonce, v.propositions, v.bonne, v.explication, v.difficulte, v.theme, 'saisie', 'candidate'
  FROM (VALUES
    -- ── Salutations ─────────────────────────────────────────────────────────
    ('Que veut dire « Asante » en swahili ?', ARRAY['Bonjour', 'Merci', 'Au revoir', 'Bienvenue'], 2,
     '« Asante » veut dire merci ; « asante sana », merci beaucoup.', 1, 'Salutations'),
    ('Que veut dire « Karibu » en swahili ?', ARRAY['Bienvenue', 'Bonne nuit', 'Pardon', 'À bientôt'], 1,
     '« Karibu » accueille le visiteur : bienvenue, entre.', 1, 'Salutations'),
    ('« Jërëjëf » est le mot wolof pour…', ARRAY['Bonjour', 'Bienvenue', 'Merci', 'Au revoir'], 3,
     '« Jërëjëf » veut dire merci en wolof, langue la plus parlée au Sénégal.', 1, 'Salutations'),
    ('« Mbote » est une salutation de quelle langue ?', ARRAY['Wolof', 'Yoruba', 'Zoulou', 'Lingala'], 4,
     '« Mbote » est le bonjour lingala, de Kinshasa à Brazzaville.', 1, 'Salutations'),
    ('Que veut dire littéralement « Sawubona », la salutation zouloue ?', ARRAY['Je te vois', 'Que la paix soit avec toi', 'Tu es arrivé', 'Le jour est levé'], 1,
     '« Sawubona » signifie « je te vois » ; on répond « Yebo, sawubona » : « oui, je te vois aussi ».', 2, 'Salutations'),
    ('Que veut dire « Ngiyabonga » en zoulou ?', ARRAY['Bonjour', 'Merci', 'Pardon', 'Je t''aime'], 2,
     '« Ngiyabonga » veut dire merci en zoulou.', 2, 'Salutations'),
    ('Dans quelle langue « Murakoze » veut-il dire merci ?', ARRAY['Lingala', 'Amharique', 'Kinyarwanda', 'Haoussa'], 3,
     '« Murakoze » est le merci du kinyarwanda, au Rwanda (et du kirundi voisin).', 2, 'Salutations'),
    ('Que veut dire « Misaotra » en malgache ?', ARRAY['Merci', 'Bonjour', 'Bienvenue', 'Au revoir'], 1,
     '« Misaotra » veut dire merci en malgache.', 2, 'Salutations'),
    ('« Salama » est une salutation courante dans quelle langue ?', ARRAY['Zoulou', 'Malgache', 'Wolof', 'Lingala'], 2,
     '« Salama » est le bonjour malgache le plus courant.', 2, 'Salutations'),
    ('« Selam », salutation courante en Éthiopie, signifie littéralement…', ARRAY['Le soleil', 'L''ami', 'La paix', 'Le matin'], 3,
     'En amharique, « selam » veut dire paix ; c''est aussi la salutation du quotidien.', 2, 'Salutations'),
    ('« Medaase » veut dire merci dans quelle langue ?', ARRAY['Twi', 'Éwé', 'Yoruba', 'Fon'], 1,
     '« Medaase » est le merci du twi, langue akan du Ghana.', 3, 'Salutations'),
    ('« Sannu » est une salutation de quelle langue ?', ARRAY['Swahili', 'Haoussa', 'Peul', 'Amharique'], 2,
     '« Sannu » est le salut haoussa, au Nigeria et au Niger.', 2, 'Salutations'),
    ('« E kaaro » est une salutation du matin dans quelle langue ?', ARRAY['Igbo', 'Éwé', 'Yoruba', 'Fon'], 3,
     '« E kaaro » (« ẹ káàárọ̀ ») souhaite le bonjour du matin en yoruba.', 2, 'Salutations'),
    ('« I ni ce » est une salutation de quelle langue ?', ARRAY['Wolof', 'Bambara', 'Peul', 'Haoussa'], 2,
     'Salutation bambara, qui sert aussi à remercier ; on la retrouve en dioula.', 2, 'Salutations'),

    -- ── Vocabulaire ─────────────────────────────────────────────────────────
    ('Que veut dire « Rafiki » en swahili ?', ARRAY['Le lion', 'L''ami', 'La maison', 'Le chef'], 2,
     '« Rafiki » veut dire ami.', 1, 'Vocabulaire'),
    ('Quel animal le mot swahili « Simba » désigne-t-il ?', ARRAY['L''éléphant', 'La girafe', 'Le lion', 'Le léopard'], 3,
     '« Simba » veut dire lion ; l''éléphant se dit « tembo ».', 1, 'Vocabulaire'),
    ('Que voulait dire à l''origine le mot swahili « safari » ?', ARRAY['La chasse', 'Le voyage', 'La savane', 'Le campement'], 2,
     '« Safari » veut dire voyage ; le sens de « chasse » ou d''« expédition animalière » est venu ensuite.', 2, 'Vocabulaire'),
    ('Que désigne le mot wolof « teranga » ?', ARRAY['L''hospitalité', 'Le courage', 'La famille', 'Le marché'], 1,
     'La « teranga » est l''hospitalité, valeur que le Sénégal revendique comme sienne.', 1, 'Vocabulaire'),
    ('Que veut dire « Uhuru » en swahili ?', ARRAY['L''unité', 'Le travail', 'La liberté', 'La terre'], 3,
     '« Uhuru » veut dire liberté ; le mot a porté les indépendances d''Afrique de l''Est.', 2, 'Vocabulaire'),
    ('Que veut dire « Baraka » en swahili ?', ARRAY['La chance au jeu', 'La bénédiction', 'La richesse', 'La force'], 2,
     '« Baraka », venu de l''arabe, désigne la bénédiction.', 2, 'Vocabulaire'),
    ('Le mot « swahili » vient d''un mot arabe qui signifie…', ARRAY['les marchands', 'la parole', 'les côtes', 'les îles'], 3,
     '« Sawāḥil » désigne les côtes : le swahili est né sur le littoral de l''océan Indien.', 3, 'Vocabulaire'),

    -- ── Géographie des langues ──────────────────────────────────────────────
    ('Quelle langue africaine est aussi une langue de travail de l''Union africaine ?', ARRAY['Le yoruba', 'Le zoulou', 'Le swahili', 'Le wolof'], 3,
     'Le swahili est langue de travail de l''Union africaine, aux côtés de l''arabe, de l''anglais, de l''espagnol, du français et du portugais.', 2, 'Géographie des langues'),
    ('Le kinyarwanda est la langue nationale de quel pays ?', ARRAY['Le Burundi', 'Le Rwanda', 'L''Ouganda', 'La Tanzanie'], 2,
     'Le kinyarwanda est parlé par presque toute la population du Rwanda.', 1, 'Géographie des langues'),
    ('Le kirundi est la langue nationale de quel pays ?', ARRAY['Le Burundi', 'Le Rwanda', 'Le Malawi', 'La Zambie'], 1,
     'Le kirundi est la langue nationale du Burundi, très proche du kinyarwanda.', 1, 'Géographie des langues'),
    ('Le sango est la langue nationale de quel pays ?', ARRAY['Le Tchad', 'Le Gabon', 'La République centrafricaine', 'Le Congo'], 3,
     'Le sango est langue officielle de la Centrafrique, aux côtés du français.', 2, 'Géographie des langues'),
    ('Le setswana est la langue nationale de quel pays ?', ARRAY['La Namibie', 'Le Botswana', 'Le Lesotho', 'L''Eswatini'], 2,
     'Le setswana, langue des Tswana, a donné son nom au Botswana.', 2, 'Géographie des langues'),
    ('Le chichewa est la langue nationale de quel pays ?', ARRAY['Le Malawi', 'Le Mozambique', 'L''Angola', 'Le Zimbabwe'], 1,
     'Le chichewa (ou nyanja) est la langue nationale du Malawi.', 2, 'Géographie des langues'),
    ('Le sesotho est la langue nationale de quel pays ?', ARRAY['La Zambie', 'Le Malawi', 'La Namibie', 'Le Lesotho'], 4,
     'Le sesotho est langue officielle du Lesotho, et l''une des langues officielles de l''Afrique du Sud.', 2, 'Géographie des langues'),
    ('Le shona est la langue la plus parlée de quel pays ?', ARRAY['La Zambie', 'Le Zimbabwe', 'Le Botswana', 'Madagascar'], 2,
     'Le shona est la première langue de la majorité des Zimbabwéens.', 2, 'Géographie des langues'),
    ('Le fon est surtout parlé dans quel pays ?', ARRAY['Le Bénin', 'Le Ghana', 'Le Cameroun', 'Le Mali'], 1,
     'Le fon est la langue la plus parlée au Bénin, notamment dans le Sud.', 2, 'Géographie des langues'),
    ('Le yoruba est surtout parlé…', ARRAY['au Kenya et en Tanzanie', 'au Nigeria et au Bénin', 'au Mali et au Niger', 'en Angola et en Namibie'], 2,
     'Le yoruba est parlé dans le sud-ouest du Nigeria et au Bénin voisin.', 1, 'Géographie des langues'),
    ('Le haoussa est surtout parlé…', ARRAY['au Nigeria et au Niger', 'au Kenya et en Tanzanie', 'en Angola et en Namibie', 'au Maroc et en Algérie'], 1,
     'Le haoussa est la grande langue véhiculaire du nord du Nigeria et du Niger.', 1, 'Géographie des langues'),
    ('Le tigrinya est surtout parlé…', ARRAY['au Soudan', 'en Somalie', 'en Érythrée et dans le nord de l''Éthiopie', 'au Kenya'], 3,
     'Le tigrinya est la langue la plus parlée d''Érythrée et la langue du Tigré éthiopien.', 2, 'Géographie des langues'),
    ('Le tamazight (berbère) est langue officielle dans quels pays ?', ARRAY['En Égypte et au Soudan', 'En Tunisie et en Libye', 'Au Mali et au Niger', 'Au Maroc et en Algérie'], 4,
     'Le Maroc (2011) et l''Algérie (2016) ont fait du tamazight une langue officielle.', 2, 'Géographie des langues'),
    ('Depuis 2023, combien de langues officielles l''Afrique du Sud reconnaît-elle ?', ARRAY['4', '9', '12', '2'], 3,
     'Onze langues depuis 1996, et douze depuis l''ajout de la langue des signes sud-africaine en 2023.', 3, 'Géographie des langues'),

    -- ── Langues créoles et de contact ───────────────────────────────────────
    ('L''afrikaans dérive principalement de quelle langue ?', ARRAY['L''anglais', 'L''allemand', 'Le néerlandais', 'Le portugais'], 3,
     'L''afrikaans est né du néerlandais parlé au Cap à partir du XVIIᵉ siècle.', 2, 'Familles de langues'),
    ('Le créole mauricien est à base lexicale…', ARRAY['française', 'anglaise', 'portugaise', 'néerlandaise'], 1,
     'Le kreol morisien tire l''essentiel de son vocabulaire du français.', 2, 'Familles de langues'),
    ('Le créole du Cap-Vert est à base lexicale…', ARRAY['française', 'espagnole', 'anglaise', 'portugaise'], 4,
     'Le kabuverdianu est un créole à base portugaise.', 2, 'Familles de langues'),
    ('« Pulaar » et « fulfulde » sont deux noms de quelle langue ?', ARRAY['Le wolof', 'Le peul', 'Le haoussa', 'Le soninké'], 2,
     'Le peul porte des noms différents selon les régions : pulaar à l''ouest, fulfulde à l''est.', 2, 'Familles de langues'),
    ('Les consonnes à clic sont typiques de quelles langues ?', ARRAY['Les langues berbères', 'Les langues mandingues', 'Les langues tchadiques', 'Les langues khoïsan'], 4,
     'Les clics viennent des langues khoïsan ; le xhosa et le zoulou les ont empruntés.', 2, 'Familles de langues'),

    -- ── Écritures ───────────────────────────────────────────────────────────
    ('L''écriture bamoun, au Cameroun, a été inventée vers 1900 par…', ARRAY['Solomana Kanté', 'le roi Njoya', 'Cheikh Anta Diop', 'Samuel Ajayi Crowther'], 2,
     'Le sultan Ibrahim Njoya l''a conçue pour noter la langue bamoun, l''histoire et les coutumes du royaume.', 3, 'Écritures'),
    ('Dans quel pays le syllabaire vaï a-t-il été créé, au XIXᵉ siècle ?', ARRAY['Le Ghana', 'Le Nigeria', 'Le Libéria', 'Le Sénégal'], 3,
     'Le vaï, l''une des plus anciennes écritures indigènes d''Afrique de l''Ouest, est né au Libéria vers 1830.', 3, 'Écritures'),
    ('Qu''appelle-t-on « ajami » ?', ARRAY['Un tambour d''Afrique de l''Ouest', 'L''écriture de langues africaines en caractères arabes', 'Un alphabet créé au Mali', 'Une langue du Sahel'], 2,
     'L''ajami note en caractères arabes le haoussa, le wolof, le peul ou le swahili, depuis des siècles.', 2, 'Écritures'),
    ('Depuis 1972, le somali s''écrit officiellement avec…', ARRAY['l''alphabet arabe', 'l''alphabet latin', 'le fidäl éthiopien', 'l''osmanya'], 2,
     'La Somalie a adopté l''alphabet latin en 1972, après un long débat entre écritures.', 3, 'Écritures')
  ) AS v(enonce, propositions, bonne, explication, difficulte, theme)
 WHERE NOT EXISTS (SELECT 1 FROM jeu.epreuve e WHERE e.module_code = 'afrolang' AND e.enonce = v.enonce);

SELECT '   épreuves afrolang ' || rpad(etat, 10) || ': ' || count(*)
  FROM jeu.epreuve WHERE module_code = 'afrolang' GROUP BY etat ORDER BY etat;
