-- ═══════════════════════════════════════════════════════════════════════
-- Seeds Afripulse — les cinq sections enrichies de la fiche territoire
-- ═══════════════════════════════════════════════════════════════════════
--
-- Sans contenu, les cinq accordéons d'une fiche n'affichent que « Aucun … pour
-- l'instant » : impossible de juger la mise en page, l'alignement des cartes ou
-- le comportement des filtres. Ces jeux couvrent la Côte d'Ivoire, le Cameroun
-- et le Sénégal, avec assez de variété pour que les filtres (localité, type de
-- site, domaine de personnalité) aient de quoi trier.
--
-- Idempotent : chaque insertion est gardée par un NOT EXISTS sur le libellé.

DO $$
DECLARE
    v_auteur UUID;
    v_ci UUID; v_cm UUID; v_sn UUID;
BEGIN
    SELECT id INTO v_auteur FROM iam.utilisateur WHERE email = 'test-admin@test.com';
    IF v_auteur IS NULL THEN RAISE NOTICE 'Compte de test absent — seeds Afripulse ignorés.'; RETURN; END IF;

    SELECT fp.id INTO v_ci FROM country_profile.fiche_pays fp JOIN shared.pays p ON p.id=fp.pays_id WHERE lower(p.code_iso2)='ci';
    SELECT fp.id INTO v_cm FROM country_profile.fiche_pays fp JOIN shared.pays p ON p.id=fp.pays_id WHERE lower(p.code_iso2)='cm';
    SELECT fp.id INTO v_sn FROM country_profile.fiche_pays fp JOIN shared.pays p ON p.id=fp.pays_id WHERE lower(p.code_iso2)='sn';

    -- ── Secteurs d'opportunités ──────────────────────────────────────────
    INSERT INTO country_profile.secteur_developpement (fiche_pays_id, nom, description, localite, contact_courriel, site_web_url, references_utiles)
    SELECT v.fiche, v.nom, v.descr, v.loc, v.mail, v.web, v.refs
      FROM (VALUES
        (v_ci, 'Transformation du cacao', 'La Côte d''Ivoire produit près de 40 % du cacao mondial mais n''en transforme qu''une part réduite. Les unités de broyage, de beurre et de chocolat de couverture cherchent des partenaires techniques et financiers.', 'San-Pédro', 'cacao@exemple-ci.org', 'https://www.conseilcafecacao.ci', 'Conseil du Café-Cacao — rapports annuels de campagne.'),
        (v_ci, 'Énergie solaire décentralisée', 'Électrification des localités hors réseau par mini-centrales solaires et kits domestiques. Besoins en installateurs formés et en modèles de paiement échelonné.', 'Korhogo', NULL, NULL, NULL),
        (v_ci, 'Logistique portuaire', 'Le port d''Abidjan dessert plusieurs pays sans littoral. Entreposage sous température dirigée, groupage et dédouanement numérique sont les maillons faibles.', 'Abidjan', 'logistique@exemple-ci.org', NULL, NULL),
        (v_cm, 'Filière bois certifié', 'Deuxième massif forestier d''Afrique. La demande porte sur la seconde transformation et la traçabilité conforme aux exigences européennes.', 'Douala', NULL, 'https://www.minfof.cm', NULL),
        (v_sn, 'Aquaculture continentale', 'Élevage de tilapia et de silure en cages flottantes le long du fleuve Sénégal. Alimentation locale et chaîne du froid restent à construire.', 'Saint-Louis', 'aqua@exemple-sn.org', NULL, NULL)
      ) AS v(fiche, nom, descr, loc, mail, web, refs)
     WHERE v.fiche IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM country_profile.secteur_developpement s WHERE s.fiche_pays_id = v.fiche AND s.nom = v.nom);

    -- ── Recettes culinaires ──────────────────────────────────────────────
    INSERT INTO country_profile.recette_culinaire (fiche_pays_id, titre, territoires_consommation, histoire, ingredients, etapes_preparation, images, cree_par)
    SELECT v.fiche, v.titre, v.terr, v.hist, v.ing, v.etapes, ARRAY[]::text[], v_auteur
      FROM (VALUES
        (v_ci, 'Attiéké poisson braisé', 'Sud et centre du pays, très présent à Abidjan',
         'Semoule de manioc fermentée d''origine ébrié, l''attiéké est devenu le plat de rue le plus répandu du pays. Il a obtenu une indication géographique protégée en 2023.',
         ARRAY['Manioc fermenté (attiéké)','Poisson entier (carpe ou tilapia)','Oignons','Tomates','Piment','Huile','Citron','Cube de bouillon'],
         ARRAY['Vider et inciser le poisson, le mariner deux heures avec ail, gingembre et cube émietté.','Braiser au charbon de bois en retournant à mi-cuisson.','Réchauffer l''attiéké à la vapeur pour l''aérer.','Préparer une sauce claire d''oignons et de tomates crues au citron.','Servir le poisson sur l''attiéké, sauce à part.']),
        (v_ci, 'Sauce graine', 'Ensemble du territoire, plat dominical',
         'Préparée à partir de la pulpe du fruit du palmier à huile, la sauce graine accompagne traditionnellement les repas de fête et les retours de deuil.',
         ARRAY['Noix de palme','Viande ou poisson fumé','Aubergines africaines','Gombo','Piment','Sel'],
         ARRAY['Cuire les noix de palme puis les piler pour en extraire la pulpe.','Filtrer à l''eau chaude pour recueillir le jus épais.','Faire mijoter avec la viande et le poisson fumé.','Ajouter aubergines et gombo en fin de cuisson.','Servir avec du riz ou du foutou banane.']),
        (v_cm, 'Ndolé', 'Littoral et Sud-Ouest',
         'Plat emblématique du Cameroun, à base de feuilles amères longuement rincées. Il figure sur toutes les tables de cérémonie douala.',
         ARRAY['Feuilles de ndolé','Arachides crues','Crevettes séchées','Viande de bœuf','Ail','Oignons','Huile'],
         ARRAY['Blanchir les feuilles plusieurs fois pour en retirer l''amertume.','Griller puis broyer les arachides en pâte.','Cuire la viande et le poisson séparément.','Réunir le tout et laisser mijoter à feu doux.','Servir avec des plantains mûrs ou du miondo.']),
        (v_sn, 'Thiéboudienne', 'Tout le pays, plat national',
         'Riz au poisson né à Saint-Louis au XIXᵉ siècle. Inscrit au patrimoine culturel immatériel de l''UNESCO en 2021.',
         ARRAY['Riz brisé','Thiof (mérou)','Tomate concentrée','Chou','Carotte','Manioc','Aubergine amère','Tamarin','Persil','Ail'],
         ARRAY['Farcir le poisson d''une persillade pilée (roff).','Frire le poisson puis le réserver.','Faire revenir la tomate, ajouter l''eau et les légumes.','Retirer les légumes, cuire le riz dans le bouillon.','Dresser le riz, disposer poisson et légumes par-dessus.'])
      ) AS v(fiche, titre, terr, hist, ing, etapes)
     WHERE v.fiche IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM country_profile.recette_culinaire r WHERE r.fiche_pays_id = v.fiche AND r.titre = v.titre);

    -- ── Sites touristiques ───────────────────────────────────────────────
    INSERT INTO country_profile.site_touristique (fiche_pays_id, nom, description, categorie, sous_type, ville, village, info_pertinente, verifie, images)
    SELECT v.fiche, v.nom, v.descr, v.cat::country_profile.categorie_site_touristique, v.st::country_profile.sous_type_site, v.ville, v.village, v.info, v.verif, ARRAY[]::text[]
      FROM (VALUES
        (v_ci, 'Basilique Notre-Dame de la Paix', 'Plus grand édifice chrétien du monde par sa superficie, consacré en 1990. Sa coupole culmine à 158 mètres et ses vitraux couvrent 7 400 m².', 'emblematique', 'eglise', 'Yamoussoukro', NULL, 'Visites guidées tous les jours sauf pendant les offices. Tenue correcte exigée.', TRUE),
        (v_ci, 'Parc national de Taï', 'Dernier grand massif de forêt primaire d''Afrique de l''Ouest, inscrit au patrimoine mondial. On y observe des chimpanzés utilisateurs d''outils.', 'emblematique', 'parc_naturel', 'Guiglo', NULL, 'Accès réglementé, guide obligatoire. Saison sèche recommandée.', TRUE),
        (v_ci, 'Grand-Bassam, quartier France', 'Première capitale coloniale, classée au patrimoine mondial pour son ensemble architectural de la fin du XIXᵉ siècle.', 'emblematique', 'monument', 'Grand-Bassam', NULL, NULL, FALSE),
        (v_ci, 'Campement d''Assinie', 'Hébergement en bord de lagune avec accès direct à la plage, pirogues et pêche traditionnelle.', 'prive', 'residence_touristique', 'Assinie-Mafia', 'Assouindé', 'Réservation conseillée en fin de semaine.', FALSE),
        (v_ci, 'Maquis du Plateau', 'Table de cuisine ivoirienne : braisés, kedjenou et garba, en terrasse ombragée.', 'prive', 'bar_maquis', 'Abidjan', NULL, NULL, FALSE),
        (v_cm, 'Mont Cameroun', 'Volcan actif culminant à 4 095 mètres, point le plus haut d''Afrique de l''Ouest. Ascension en deux à trois jours.', 'emblematique', 'relief_naturel', 'Buéa', NULL, 'Course annuelle de l''espoir en février. Guides agréés obligatoires.', TRUE),
        (v_cm, 'Chutes de la Lobé', 'Rare cas de fleuve se jetant directement dans l''océan par une série de cascades.', 'emblematique', 'site_naturel', 'Kribi', NULL, NULL, FALSE),
        (v_sn, 'Île de Gorée', 'Mémoire de la traite atlantique, inscrite au patrimoine mondial. Maison des Esclaves et musée historique.', 'emblematique', 'monument', 'Dakar', NULL, 'Chaloupe au départ du port de Dakar, environ vingt minutes.', TRUE),
        (v_sn, 'Lac Rose (Retba)', 'Lac hypersalé dont la teinte varie du rose au mauve selon la lumière et la saison. Récolte artisanale du sel.', 'emblematique', 'mer_riviere', 'Dakar', NULL, NULL, FALSE)
      ) AS v(fiche, nom, descr, cat, st, ville, village, info, verif)
     WHERE v.fiche IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM country_profile.site_touristique s WHERE s.fiche_pays_id = v.fiche AND s.nom = v.nom);

    -- ── Personnalités connues ────────────────────────────────────────────
    INSERT INTO country_profile.personnalite_connue (fiche_pays_id, nom_complet, domaine, biographie_courte, annee_naissance, annee_deces, lien_reference, cree_par)
    SELECT v.fiche, v.nom, v.dom::country_profile.domaine_personnalite, v.bio, v.ne, v.mort, v.lien, v_auteur
      FROM (VALUES
        (v_ci, 'Félix Houphouët-Boigny', 'politique', 'Médecin, syndicaliste agricole puis premier président de la Côte d''Ivoire de 1960 à 1993. Artisan de l''abolition du travail forcé en Afrique occidentale française.', 1905::smallint, 1993::smallint, NULL),
        (v_ci, 'Didier Drogba', 'sportif', 'Attaquant international, meilleur buteur de l''histoire de la sélection ivoirienne. Son appel de 2005 est associé à l''apaisement de la crise politique.', 1978::smallint, NULL::smallint, NULL),
        (v_ci, 'Alpha Blondy', 'artiste_musicien', 'Chanteur de reggae chantant en dioula, français et anglais. Il a porté la musique ivoirienne sur la scène internationale à partir des années 1980.', 1953::smallint, NULL::smallint, NULL),
        (v_ci, 'Ahmadou Kourouma', 'artiste_autre', 'Romancier, auteur des « Soleils des indépendances » et d''« Allah n''est pas obligé ». Son écriture plie le français à la syntaxe malinké.', 1927::smallint, 2003::smallint, NULL),
        (v_cm, 'Samuel Eto''o', 'sportif', 'Quadruple Ballon d''or africain, vainqueur de la Ligue des champions avec le FC Barcelone et l''Inter Milan.', 1981::smallint, NULL::smallint, NULL),
        (v_cm, 'Manu Dibango', 'artiste_musicien', 'Saxophoniste et compositeur, auteur de « Soul Makossa » (1972), morceau fondateur de la rencontre entre funk et musiques camerounaises.', 1933::smallint, 2020::smallint, NULL),
        (v_sn, 'Cheikh Anta Diop', 'scientifique', 'Historien, anthropologue et physicien nucléaire. Ses travaux sur l''antériorité des civilisations africaines ont renouvelé l''historiographie du continent.', 1923::smallint, 1986::smallint, NULL),
        (v_sn, 'Mariama Bâ', 'artiste_autre', 'Romancière et enseignante, autrice d''« Une si longue lettre », première grande œuvre féministe de la littérature ouest-africaine.', 1929::smallint, 1981::smallint, NULL)
      ) AS v(fiche, nom, dom, bio, ne, mort, lien)
     WHERE v.fiche IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM country_profile.personnalite_connue pc WHERE pc.fiche_pays_id = v.fiche AND pc.nom_complet = v.nom);

    -- ── Savoirs pratiques ────────────────────────────────────────────────
    INSERT INTO country_profile.savoir_pratique (fiche_pays_id, titre, categorie, explication, exemple, cree_par)
    SELECT v.fiche, v.titre, v.cat::country_profile.categorie_savoir, v.expl, v.ex, v_auteur
      FROM (VALUES
        (v_ci, 'Saluer avant toute chose', 'etiquette', 'On ne va pas droit au but. Demander des nouvelles de la famille, du travail et de la nuit passée fait partie de la salutation, même dans un cadre professionnel.', 'À l''entrée d''un bureau ou d''une boutique, dire bonjour à toutes les personnes présentes, pas seulement à celle que l''on vient voir.'),
        (v_ci, 'Le nouchi', 'langue_argot', 'Argot né dans les rues d''Abidjan, mêlant français, dioula, baoulé et anglais. Il est compris de tous et employé jusque dans la publicité.', '« Enjaillé » signifie ravi ; « gaou » désigne un naïf ; « djo » veut dire type, gars.'),
        (v_ci, 'Négocier en taxi', 'transports', 'Les taxis communaux (woro-woro) ont des tarifs fixes par trajet ; les taxis compteurs d''Abidjan négocient le prix avant le départ.', 'Annoncer sa destination et convenir du prix avant de monter évite les discussions à l''arrivée.'),
        (v_ci, 'Manger à la main droite', 'coutumes', 'Dans les repas partagés autour d''un plat commun, la main gauche est réservée à l''hygiène et n''entre pas dans le plat.', 'Se laver les mains avant et après le repas fait partie du rituel ; une bassine circule souvent avant de servir.'),
        (v_cm, 'Le mot « on est ensemble »', 'langue_argot', 'Formule de connivence qui clôt une conversation ou scelle un accord. Elle dit l''appartenance à un même groupe plus qu''un rendez-vous.', 'On l''entend en fin d''échange, souvent accompagnée d''une poignée de main.'),
        (v_sn, 'La teranga', 'coutumes', 'Hospitalité érigée en valeur cardinale : un visiteur est nourri avant d''être questionné, et refuser un repas offert peut blesser.', 'Arriver à l''heure du déjeuner vaut invitation ; il est d''usage d''accepter au moins symboliquement.')
      ) AS v(fiche, titre, cat, expl, ex)
     WHERE v.fiche IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM country_profile.savoir_pratique sp WHERE sp.fiche_pays_id = v.fiche AND sp.titre = v.titre);

    -- ── Infos pratiques scalaires (bloc « À savoir avant de voyager ») ───
    UPDATE country_profile.fiche_pays SET
        voyage_langue_internationale    = COALESCE(voyage_langue_internationale, 'Français'),
        voyage_langue_locale            = COALESCE(voyage_langue_locale, 'Dioula, baoulé, bété — le dioula sert de langue véhiculaire des marchés.'),
        voyage_infos_visa               = COALESCE(voyage_infos_visa, 'e-Visa obtenu en ligne avant le départ pour la plupart des nationalités, retiré à l''aéroport d''Abidjan. Ressortissants CEDEAO exemptés.'),
        voyage_infos_sanitaires         = COALESCE(voyage_infos_sanitaires, 'Vaccination contre la fièvre jaune obligatoire, carnet exigé à l''arrivée. Prophylaxie antipaludique fortement recommandée.'),
        voyage_meteo                    = COALESCE(voyage_meteo, 'Climat tropical. Saison sèche de novembre à mars au sud, harmattan en décembre et janvier au nord.'),
        voyage_prises_electriques       = COALESCE(voyage_prises_electriques, 'Type C et E, 230 V / 50 Hz — prises identiques à la France.'),
        voyage_contacts_tourisme        = COALESCE(voyage_contacts_tourisme, 'Côte d''Ivoire Tourisme — +225 27 20 25 16 00'),
        voyage_recommandations_securite = COALESCE(voyage_recommandations_securite, 'Consulter les recommandations officielles avant le départ. Vigilance renforcée dans les zones frontalières du nord.')
     WHERE id = v_ci;

    RAISE NOTICE 'Seeds Afripulse appliqués.';
END $$;

SELECT '  secteurs      : ' || count(*) FROM country_profile.secteur_developpement;
SELECT '  recettes      : ' || count(*) FROM country_profile.recette_culinaire;
SELECT '  sites         : ' || count(*) FROM country_profile.site_touristique;
SELECT '  personnalités : ' || count(*) FROM country_profile.personnalite_connue;
SELECT '  savoirs       : ' || count(*) FROM country_profile.savoir_pratique;
