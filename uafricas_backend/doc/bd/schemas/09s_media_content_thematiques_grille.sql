-- ============================================================================
-- 09s — Référentiel des thématiques médias : les 22 genres de grille
--
-- Les 44 libellés semés par 09j décrivaient des LIGNES ÉDITORIALES de la
-- plateforme (« Retour des cerveaux », « Haro sur les hommes de l'Afrique »).
-- Ce que les supports doivent déclarer, et ce sur quoi le public filtre, ce
-- sont des GENRES DE GRILLE : journal, débat, magazine, jeunesse, sport.
--
-- Les 44 sont **désactivées, pas supprimées**. Trois raisons :
--   • `emission_*.theme_phare_id` et `support_thematique.categorie_id` les
--     référencent ; les supprimer casserait ces lignes ou les effacerait en
--     silence selon la clé étrangère ;
--   • tous les sélecteurs (`referentiels_edition`, `/api/admin/categories`,
--     le formulaire de proposition) filtrent déjà sur `actif = TRUE` : les
--     désactiver suffit à les retirer de la saisie ;
--   • l'affichage d'une thématique déjà déclarée joint `shared.categorie`
--     SANS filtrer `actif`, donc rien ne disparaît d'une fiche existante.
-- La bascule est donc réversible par un seul UPDATE.
--
-- Idempotent : `ON CONFLICT (slug) DO UPDATE` sur l'insertion, et la
-- désactivation ne vise que le contexte média hors nouvelle liste.
-- ============================================================================

BEGIN;

-- Slugs préfixés « media- » : `shared.categorie.slug` est UNIQUE tous contextes
-- confondus, et « Sport », « Culture » ou « Éducation » existent ailleurs.
INSERT INTO shared.categorie (nom, slug, contexte, ordre, actif) VALUES
    ('Journal télévisé',          'media-journal-televise',          'media',  1, TRUE),
    ('Débats et analyses',        'media-debats-et-analyses',        'media',  2, TRUE),
    ('Émissions économiques',     'media-emissions-economiques',     'media',  3, TRUE),
    ('Magazine Innovation',       'media-magazine-innovation',       'media',  4, TRUE),
    ('Éducation',                 'media-education',                 'media',  5, TRUE),
    ('Culture',                   'media-culture',                   'media',  6, TRUE),
    ('Talk-show',                 'media-talk-show',                 'media',  7, TRUE),
    ('Émissions jeunesse',        'media-emissions-jeunesse',        'media',  8, TRUE),
    ('Sport',                     'media-sport',                     'media',  9, TRUE),
    ('Santé',                     'media-sante',                     'media', 10, TRUE),
    ('Agriculture',               'media-agriculture',               'media', 11, TRUE),
    ('Religion et spiritualité',  'media-religion-et-spiritualite',  'media', 12, TRUE),
    ('Tourisme',                  'media-tourisme',                  'media', 13, TRUE),
    ('Environnement',             'media-environnement',             'media', 14, TRUE),
    ('Divertissement',            'media-divertissement',            'media', 15, TRUE),
    ('Cinéma',                    'media-cinema',                    'media', 16, TRUE),
    ('Diaspora',                  'media-diaspora',                  'media', 17, TRUE),
    ('Émissions citoyennes',      'media-emissions-citoyennes',      'media', 18, TRUE),
    ('Grandes interviews',        'media-grandes-interviews',        'media', 19, TRUE),
    ('Émissions interactives',    'media-emissions-interactives',    'media', 20, TRUE),
    ('Investigations',            'media-investigations',            'media', 21, TRUE),
    ('Vie pratique',              'media-vie-pratique',              'media', 22, TRUE)
ON CONFLICT (slug) DO UPDATE
    SET nom   = EXCLUDED.nom,
        ordre = EXCLUDED.ordre,
        actif = TRUE,
        updated_at = NOW();

-- Retrait de la saisie des 44 anciennes lignes éditoriales.
UPDATE shared.categorie
   SET actif = FALSE, updated_at = NOW()
 WHERE contexte = 'media'
   AND actif = TRUE
   AND slug NOT IN (
        'media-journal-televise', 'media-debats-et-analyses', 'media-emissions-economiques',
        'media-magazine-innovation', 'media-education', 'media-culture', 'media-talk-show',
        'media-emissions-jeunesse', 'media-sport', 'media-sante', 'media-agriculture',
        'media-religion-et-spiritualite', 'media-tourisme', 'media-environnement',
        'media-divertissement', 'media-cinema', 'media-diaspora', 'media-emissions-citoyennes',
        'media-grandes-interviews', 'media-emissions-interactives', 'media-investigations',
        'media-vie-pratique');

-- ────────────────────────────────────────────────────────────────────────────
-- Report des déclarations existantes sur la nouvelle liste
--
-- Une déclaration pointant une thématique désactivée resterait affichée mais
-- deviendrait immodifiable telle quelle : le sélecteur ne proposant plus la
-- valeur, le premier enregistrement de la fiche la ferait disparaître sans
-- que personne ne l'ait décidé. On reporte donc explicitement, par équivalence
-- de sens, et on laisse tomber ce qui n'a pas d'équivalent.
-- ────────────────────────────────────────────────────────────────────────────

CREATE TEMP TABLE _report_09s (ancien TEXT, nouveau TEXT) ON COMMIT DROP;
INSERT INTO _report_09s VALUES
    ('media-journal-de-l-afrique',                     'media-journal-televise'),
    ('media-debats-africains',                         'media-debats-et-analyses'),
    ('media-cinema-africain',                          'media-cinema'),
    ('media-documentaires-africains',                  'media-investigations'),
    ('media-histoire-de-l-afrique',                    'media-culture'),
    ('media-traditions-d-afrique',                     'media-culture'),
    ('media-afrique-et-technologies',                  'media-magazine-innovation'),
    ('media-numerique-et-developpement-africain',      'media-magazine-innovation'),
    ('media-innovations-simples-chez-nous',            'media-magazine-innovation'),
    ('media-futurs-genies-d-afrique',                  'media-emissions-jeunesse'),
    ('media-sports-d-afrique',                         'media-sport'),
    ('media-sante-et-developpement',                   'media-sante'),
    ('media-environnement-d-afrique',                  'media-environnement'),
    ('media-developpement-durable',                    'media-environnement'),
    ('media-safari-d-afrique',                         'media-tourisme'),
    ('media-cuisine-de-chez-nous',                     'media-vie-pratique'),
    ('media-afrique-societe',                          'media-emissions-citoyennes'),
    ('media-la-voix-du-terrain-en-afrique',            'media-investigations'),
    ('media-immigration-et-l-avenir-de-l-afrique',     'media-diaspora'),
    ('media-retour-des-cerveaux',                      'media-diaspora'),
    ('media-commerce-africain-et-unite-africaine',     'media-emissions-economiques'),
    ('media-series-d-afrique',                         'media-divertissement');

-- `ON CONFLICT DO NOTHING` : deux anciennes thématiques peuvent converger vers
-- la même nouvelle sur un même support, et l'unicité (support, catégorie) le
-- refuserait. Le doublon est écarté, pas l'opération.
INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
SELECT st.type_support, st.support_id, neuf.id
  FROM media_content.support_thematique st
  JOIN shared.categorie ancien ON ancien.id = st.categorie_id
  JOIN _report_09s r           ON r.ancien = ancien.slug
  JOIN shared.categorie neuf   ON neuf.slug = r.nouveau
ON CONFLICT DO NOTHING;

DELETE FROM media_content.support_thematique st
 USING shared.categorie ancien
 WHERE ancien.id = st.categorie_id
   AND ancien.contexte = 'media'
   AND ancien.actif = FALSE;

-- Même report pour le thème phare des programmes, qui puise au même référentiel.
UPDATE media_content.emission_tele e
   SET theme_phare_id = neuf.id, updated_at = NOW()
  FROM shared.categorie ancien
  JOIN _report_09s r         ON r.ancien = ancien.slug
  JOIN shared.categorie neuf ON neuf.slug = r.nouveau
 WHERE ancien.id = e.theme_phare_id;

UPDATE media_content.emission_radio e
   SET theme_phare_id = neuf.id, updated_at = NOW()
  FROM shared.categorie ancien
  JOIN _report_09s r         ON r.ancien = ancien.slug
  JOIN shared.categorie neuf ON neuf.slug = r.nouveau
 WHERE ancien.id = e.theme_phare_id;

-- Un thème phare resté sur une valeur sans équivalent devient NULL : le laisser
-- afficherait une étiquette que plus personne ne peut resélectionner.
UPDATE media_content.emission_tele e
   SET theme_phare_id = NULL, updated_at = NOW()
  FROM shared.categorie c
 WHERE c.id = e.theme_phare_id AND c.contexte = 'media' AND c.actif = FALSE;

UPDATE media_content.emission_radio e
   SET theme_phare_id = NULL, updated_at = NOW()
  FROM shared.categorie c
 WHERE c.id = e.theme_phare_id AND c.contexte = 'media' AND c.actif = FALSE;

DO $$
DECLARE v_actives INT; v_desactivees INT;
BEGIN
    SELECT count(*) INTO v_actives      FROM shared.categorie WHERE contexte='media' AND actif;
    SELECT count(*) INTO v_desactivees  FROM shared.categorie WHERE contexte='media' AND NOT actif;
    RAISE NOTICE '09s — thématiques médias : % actives, % désactivées.', v_actives, v_desactivees;
END $$;

COMMIT;
