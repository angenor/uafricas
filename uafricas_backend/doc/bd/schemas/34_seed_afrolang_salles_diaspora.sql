-- ════════════════════════════════════════════════════════════════════════
-- Seed : salles Afrolang « diaspora » (langues africaines métamorphosées)
-- ────────────────────────────────────────────────────────────────────────
-- En plus des langues africaines « actuelles » (parlées sur le continent),
-- on amorce des salles pour les langues AFRO-DESCENDANTES nées hors d'Afrique :
-- créoles et parlers rituels issus du contact entre des langues africaines
-- (gbe, kikongo, akan, yorùbá, mandé…) et les langues coloniales, dans les
-- Amériques. Ces salles utilisent `groupe_ethnique_libre` (texte libre) car
-- la communauté n'est pas un groupe ethnique du référentiel `country_profile`.
--
-- Chaque salle est rattachée à son territoire de la diaspora ET à son
-- principal substrat africain (table afrolang.salle_pays_origine).
--
-- IDEMPOTENT : ON CONFLICT (slug) DO NOTHING + ON CONFLICT pour les territoires.
-- Dépendances : shared.pays (seeds 30/33), iam.utilisateur (un créateur admin).
-- ════════════════════════════════════════════════════════════════════════

DO $$
DECLARE
  v_createur UUID;
  v_salle    UUID;
  r          RECORD;
BEGIN
  -- Créateur : l'admin de test en priorité, sinon n'importe quel utilisateur.
  SELECT id INTO v_createur
    FROM iam.utilisateur
   WHERE email = 'test-admin@test.com' AND deleted_at IS NULL
   LIMIT 1;

  IF v_createur IS NULL THEN
    SELECT id INTO v_createur
      FROM iam.utilisateur
     WHERE deleted_at IS NULL
     ORDER BY created_at
     LIMIT 1;
  END IF;

  IF v_createur IS NULL THEN
    RAISE NOTICE 'Aucun utilisateur disponible — seed des salles diaspora ignoré.';
    RETURN;
  END IF;

  FOR r IN
    SELECT * FROM (VALUES
      ('creole-haitien', 'Créole haïtien',
       'Communauté afro-haïtienne (Fon, Ewe, Kongo)',
       'Langue créole née en Haïti du contact des langues gbe (Fon, Ewe) et kikongo avec le français.',
       ARRAY['HT','BJ']),

      ('patois-jamaicain', 'Patois jamaïcain',
       'Diaspora akan & igbo (Jamaïque)',
       'Créole jamaïcain (Patwa) porté par un fort substrat akan (Ghana) et igbo (Nigeria).',
       ARRAY['JM','GH']),

      ('gullah', 'Gullah',
       'Communauté Gullah-Geechee (Sierra Leone, Sénégambie)',
       'Créole anglais des îles de la côte sud-est des États-Unis, héritier des langues de Sierra Leone et de Sénégambie.',
       ARRAY['US','SL']),

      ('creole-louisianais', 'Créole louisianais',
       'Créoles afro-louisianais',
       'Créole à base française de Louisiane, marqué par les apports gbe et ouest-africains.',
       ARRAY['US','BJ']),

      ('palenquero', 'Palenquero',
       'San Basilio de Palenque (héritage kikongo)',
       'Créole espagnol de Colombie, premier village libre d''Amérique, au substrat kikongo affirmé.',
       ARRAY['CO','CD']),

      ('sranan-tongo', 'Sranan Tongo',
       'Afro-Surinamais (Akan, Gbe)',
       'Créole anglais du Suriname structuré par les langues akan et gbe.',
       ARRAY['SR','GH']),

      ('saramaccan', 'Saramaccan',
       'Marrons Saamaka (Gbe, Kikongo)',
       'Langue des Marrons Saamaka du Suriname, riche en racines gbe et kikongo.',
       ARRAY['SR','BJ']),

      ('lucumi-yoruba', 'Lucumí (Yorùbá afro-cubain)',
       'Yorùbá de la Santería (Cuba)',
       'Parler rituel yorùbá conservé à Cuba dans la pratique de la Santería (Regla de Ocha).',
       ARRAY['CU','NG']),

      ('yoruba-nago-bresil', 'Yorùbá nagô',
       'Candomblé Nagô (Brésil)',
       'Yorùbá liturgique transmis au Brésil par le Candomblé Nagô.',
       ARRAY['BR','NG']),

      ('creole-trinidadien', 'Créole trinidadien',
       'Diaspora afro-trinidadienne',
       'Créole de Trinité-et-Tobago nourri par les langues akan et yorùbá de la diaspora.',
       ARRAY['TT','GH'])
    ) AS t(slug, langue_cible, libre, description, codes)
  LOOP
    INSERT INTO afrolang.salle
        (titre, slug, description, langue_cible, groupe_ethnique_libre, cree_par)
    VALUES
        (r.langue_cible, r.slug, r.description, r.langue_cible, r.libre, v_createur)
    ON CONFLICT (slug) DO NOTHING;

    SELECT id INTO v_salle FROM afrolang.salle WHERE slug = r.slug;

    INSERT INTO afrolang.salle_pays_origine (salle_id, pays_id)
    SELECT v_salle, p.id
      FROM shared.pays p
     WHERE p.code_iso2 = ANY(r.codes)
    ON CONFLICT DO NOTHING;
  END LOOP;
END $$;
