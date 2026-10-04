-- ============================================================================
--  SEED LOCAL — ÉPREUVES CARTE, ORDRE ET PAIRES (feature 014)
-- ============================================================================
--
--  ⚠️  POSTE DE DÉVELOPPEMENT UNIQUEMENT (hors de `schemas/`, jamais déployé).
--  Suppose 37_jeu.sql et 38_jeu_concours.sql.
--
--  Douze épreuves JOUABLES des trois nouveaux types, pour recetter le
--  scénario 1 du quickstart sans passer par la saisie.
--
--  ⚠️ Le contenu est écrit DANS L'ORDRE ATTENDU, et les fonctions ci-dessous le
--  rangent AU HASARD avant l'insertion. Stocker les éléments dans l'ordre
--  attendu ferait des clés servies la solution en clair (point de conception
--  PC4 du plan).
--
--  Idempotent : une épreuve dont l'énoncé existe déjà n'est pas réinsérée.
--
--    docker exec -i uafricas_postgres psql -U uafricas -d africans_db \
--      -v ON_ERROR_STOP=1 < uafricas_backend/doc/bd/seeds-locaux/94_seed_local_jeu_types.sql
-- ============================================================================

-- ── Ordre : `elements` et `vals` dans l'ordre attendu ───────────────────────
CREATE OR REPLACE FUNCTION pg_temp.inserer_ordre(
    p_module text, p_enonce text, elements text[], vals text[], p_explication text,
    p_theme text, p_difficulte smallint
) RETURNS void LANGUAGE plpgsql AS $$
DECLARE
    perm int[];   -- perm[k] = rang ATTENDU de l'élément stocké en position k
BEGIN
    IF EXISTS (SELECT 1 FROM jeu.epreuve WHERE module_code = p_module AND enonce = p_enonce) THEN
        RETURN;
    END IF;
    SELECT array_agg(i ORDER BY random()) INTO perm FROM generate_series(1, cardinality(elements)) i;
    INSERT INTO jeu.epreuve
        (module_code, type_reponse, enonce, propositions, valeurs, solution, explication,
         difficulte, theme, origine, etat, valide_at)
    VALUES (
        p_module, 'ordre', p_enonce,
        ARRAY(SELECT elements[perm[k]] FROM generate_series(1, cardinality(perm)) k ORDER BY k),
        ARRAY(SELECT vals[perm[k]]     FROM generate_series(1, cardinality(perm)) k ORDER BY k),
        -- solution[j] = position de stockage de l'élément attendu en j-ième
        ARRAY(SELECT array_position(perm, j)::smallint FROM generate_series(1, cardinality(perm)) j ORDER BY j),
        p_explication, p_difficulte, p_theme, 'saisie', 'jouable', NOW());
END $$;

-- ── Paires : `gauche[i]` va avec `droite[i]` ─────────────────────────────────
CREATE OR REPLACE FUNCTION pg_temp.inserer_paires(
    p_module text, p_enonce text, gauche text[], droite text[], p_explication text,
    p_theme text, p_difficulte smallint
) RETURNS void LANGUAGE plpgsql AS $$
DECLARE
    perm int[];   -- perm[k] = indice, dans `droite`, du texte stocké en position k
BEGIN
    IF EXISTS (SELECT 1 FROM jeu.epreuve WHERE module_code = p_module AND enonce = p_enonce) THEN
        RETURN;
    END IF;
    SELECT array_agg(i ORDER BY random()) INTO perm FROM generate_series(1, cardinality(droite)) i;
    INSERT INTO jeu.epreuve
        (module_code, type_reponse, enonce, propositions, appariements, solution, explication,
         difficulte, theme, origine, etat, valide_at)
    VALUES (
        p_module, 'paires', p_enonce, gauche,
        ARRAY(SELECT droite[perm[k]] FROM generate_series(1, cardinality(perm)) k ORDER BY k),
        -- solution[i] = position de stockage du correspondant de gauche[i]
        ARRAY(SELECT array_position(perm, i)::smallint FROM generate_series(1, cardinality(perm)) i ORDER BY i),
        p_explication, p_difficulte, p_theme, 'saisie', 'jouable', NOW());
END $$;

-- ── Carte ────────────────────────────────────────────────────────────────────
CREATE OR REPLACE FUNCTION pg_temp.inserer_carte(
    p_module text, p_enonce text, iso text, p_explication text, p_theme text, p_difficulte smallint
) RETURNS void LANGUAGE plpgsql AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM jeu.epreuve WHERE module_code = p_module AND enonce = p_enonce) THEN
        RETURN;
    END IF;
    INSERT INTO jeu.epreuve
        (module_code, type_reponse, enonce, propositions, reponse_pays_id, pays_id, explication,
         difficulte, theme, origine, etat, valide_at)
    SELECT p_module, 'carte', p_enonce, '{}', p.id, p.id, p_explication, p_difficulte, p_theme,
           'saisie', 'jouable', NOW()
      FROM shared.pays p WHERE LOWER(p.code_iso2) = iso;
END $$;


-- ════════════════════════════════════════════════════════════════════════════
--  CARTE
-- ════════════════════════════════════════════════════════════════════════════
SELECT pg_temp.inserer_carte('afripulse', 'Désignez sur la carte le pays dont la capitale est Ouagadougou.', 'bf',
    'Ouagadougou est la capitale du Burkina Faso.', 'Capitales', 1::smallint);
SELECT pg_temp.inserer_carte('afripulse', 'Désignez sur la carte le pays dont la capitale est Windhoek.', 'na',
    'Windhoek est la capitale de la Namibie.', 'Capitales', 2::smallint);
SELECT pg_temp.inserer_carte('afripulse', 'Désignez sur la carte le pays où se trouvent les pyramides de Méroé.', 'sd',
    'Les pyramides de Méroé, héritées du royaume de Koush, se trouvent au Soudan ; il y en a plus qu''en Égypte.', 'Patrimoine', 2::smallint);
SELECT pg_temp.inserer_carte('afripulse', 'Désignez sur la carte le pays le plus peuplé d''Afrique.', 'ng',
    'Le Nigeria compte plus de 220 millions d''habitants.', 'Population', 1::smallint);

-- ════════════════════════════════════════════════════════════════════════════
--  ORDRE
-- ════════════════════════════════════════════════════════════════════════════
SELECT pg_temp.inserer_ordre('afripulse', 'Classez ces pays du plus peuplé au moins peuplé.',
    ARRAY['Nigeria', 'Éthiopie', 'Kenya', 'Gabon'],
    ARRAY['≈ 230 millions d''habitants', '≈ 130 millions d''habitants', '≈ 56 millions d''habitants', '≈ 2,5 millions d''habitants'],
    'Le Nigeria est de loin le pays le plus peuplé du continent ; le Gabon compte moins de trois millions d''habitants.',
    'Population', 1::smallint);
SELECT pg_temp.inserer_ordre('afripulse', 'Classez ces pays du plus vaste au moins vaste.',
    ARRAY['Algérie', 'République démocratique du Congo', 'Soudan', 'Libye'],
    ARRAY['2 381 741 km²', '2 344 858 km²', '1 886 068 km²', '1 759 540 km²'],
    'L''Algérie est le plus vaste pays d''Afrique depuis la partition du Soudan en 2011.',
    'Géographie', 3::smallint);
SELECT pg_temp.inserer_ordre('afripulse', 'Classez ces indépendances de la plus ancienne à la plus récente.',
    ARRAY['Ghana', 'Sénégal', 'Algérie', 'Angola'],
    ARRAY['6 mars 1957', '1960', '5 juillet 1962', '11 novembre 1975'],
    'Le Ghana ouvre la voie en 1957 ; l''Angola n''obtient la sienne qu''en 1975, après une longue guerre.',
    'Histoire', 2::smallint);
SELECT pg_temp.inserer_ordre('afrolang', 'Classez ces nombres swahili du plus petit au plus grand.',
    ARRAY['moja', 'mbili', 'tatu', 'nne'],
    ARRAY['un', 'deux', 'trois', 'quatre'],
    'Moja, mbili, tatu, nne : un, deux, trois, quatre.',
    'Vocabulaire', 1::smallint);

-- ════════════════════════════════════════════════════════════════════════════
--  PAIRES
-- ════════════════════════════════════════════════════════════════════════════
SELECT pg_temp.inserer_paires('afripulse', 'Associez chaque pays à sa capitale.',
    ARRAY['Mali', 'Kenya', 'Maroc', 'Ghana'],
    ARRAY['Bamako', 'Nairobi', 'Rabat', 'Accra'],
    'Bamako, Nairobi, Rabat et Accra : Casablanca est la plus grande ville du Maroc, mais pas sa capitale.',
    'Capitales', 1::smallint);
SELECT pg_temp.inserer_paires('afrolang', 'Associez chaque mot swahili à sa traduction.',
    ARRAY['maji', 'chakula', 'rafiki', 'simba'],
    ARRAY['eau', 'nourriture', 'ami', 'lion'],
    'Maji : eau ; chakula : nourriture ; rafiki : ami ; simba : lion.',
    'Vocabulaire', 1::smallint);
SELECT pg_temp.inserer_paires('afrolang', 'Associez chaque salutation à sa langue.',
    ARRAY['Jambo', 'Sawubona', 'Mbote', 'Nanga def'],
    ARRAY['Swahili', 'Zoulou', 'Lingala', 'Wolof'],
    'Jambo (swahili), Sawubona (zoulou), Mbote (lingala), Nanga def (wolof).',
    'Salutations', 2::smallint);
SELECT pg_temp.inserer_paires('codimoi', 'Associez chaque citation à son auteur.',
    ARRAY['« L''éducation est l''arme la plus puissante pour changer le monde. »',
          '« Seule la lutte libère. »',
          '« En Afrique, quand un vieillard meurt, c''est une bibliothèque qui brûle. »',
          '« Nous ne regardons ni vers l''Est ni vers l''Ouest : nous regardons vers l''avant. »'],
    ARRAY['Nelson Mandela', 'Thomas Sankara', 'Amadou Hampâté Bâ', 'Kwame Nkrumah'],
    'Mandela, Sankara, Hampâté Bâ (à l''Unesco, en 1960) et Nkrumah.',
    'Citations', 2::smallint);


-- ════════════════════════════════════════════════════════════════════════════
--  CONTRÔLE
-- ════════════════════════════════════════════════════════════════════════════
SELECT '   ' || rpad(module_code || ' / ' || type_reponse, 22) || ': ' || count(*)
  FROM jeu.epreuve WHERE type_reponse <> 'choix' AND etat = 'jouable'
 GROUP BY module_code, type_reponse ORDER BY 1;
-- Aucune solution ne doit être l'identité POUR TOUTES (PC4) : sur douze
-- tirages, quelques identités isolées sont du hasard, pas un défaut.
SELECT '   solutions identité : ' || count(*) FILTER (WHERE solution = ARRAY(SELECT generate_series(1, cardinality(solution))::smallint))
       || ' / ' || count(*)
  FROM jeu.epreuve WHERE type_reponse IN ('ordre', 'paires');
