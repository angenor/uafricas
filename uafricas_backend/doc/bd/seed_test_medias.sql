-- ============================================================================
--  Jeu de données de TEST — refonte Télé & Radio (001-refonte-tele-radio)
-- ============================================================================
--
--  Objet : rendre démontrables les parcours de
--          specs/001-refonte-tele-radio/quickstart.md sur les trois lots.
--
--  ⚠  DÉVELOPPEMENT LOCAL UNIQUEMENT — ne jamais jouer en production.
--
--  Toutes les lignes créées portent un identifiant préfixé `dddd` : le script
--  de nettoyage `seed_test_medias_purge.sql` s'appuie sur ce marqueur pour
--  tout retirer sans toucher aux données existantes.
--
--  Idempotent : rejouable sans erreur ni doublon (ON CONFLICT DO NOTHING sur
--  des UUID fixes). Les créneaux, dont l'heure dépend de l'instant présent,
--  sont recalculés à chaque passage.
--
--  Usage :
--    psql -h localhost -U uafricas -d africans_db -f doc/bd/seed_test_medias.sql
--
--  Comptes utilisés (cf. CLAUDE.md § Test Users) :
--    test-admin@test.com / Test1234   — administrateur, propriétaire des supports
--    test-user@test.com  / Test1234   — membre, auteur des propositions
-- ============================================================================

\set ON_ERROR_STOP on

BEGIN;

-- ── Références résolues une fois pour toutes ────────────────────────────────
--
-- Passer par une table temporaire plutôt que par des sous-requêtes répétées :
-- si un compte de test manque, l'échec est immédiat et explicite.

CREATE TEMP TABLE ref_seed ON COMMIT DROP AS
SELECT
    (SELECT id FROM iam.utilisateur WHERE email = 'test-admin@test.com') AS admin_id,
    (SELECT id FROM iam.utilisateur WHERE email = 'test-user@test.com')  AS membre_id,
    (SELECT id FROM shared.pays ORDER BY nom LIMIT 1)                    AS pays_id,
    (SELECT id FROM shared.categorie WHERE contexte = 'media' ORDER BY nom LIMIT 1) AS theme_id,
    -- Un membre tiers, auteur des demandes d'engagement : sans lui le détenteur
    -- se retrouverait à arbitrer ses propres demandes.
    (SELECT id FROM iam.utilisateur
      WHERE deleted_at IS NULL
        AND email NOT IN ('test-admin@test.com', 'test-user@test.com')
      ORDER BY created_at LIMIT 1) AS tiers_id;

DO $$
DECLARE r RECORD;
BEGIN
    SELECT * INTO r FROM ref_seed;
    IF r.admin_id IS NULL OR r.membre_id IS NULL THEN
        RAISE EXCEPTION
            'Comptes de test absents. Créez test-admin@test.com et test-user@test.com avant de jouer ce seed.';
    END IF;
    IF r.theme_id IS NULL THEN
        RAISE EXCEPTION
            'Aucun thème phare en base : la migration 09j n''a pas été jouée.';
    END IF;
END $$;


-- ════════════════════════════════════════════════════════════════════════════
--  1. STATIONS RADIO — deux par page, pour que le défilement en sections ait
--     un sens (US2). L'origine décide de la page : jamais les deux.
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO media_content.station_radio
    (id, nom, slug, description, stream_url, image_couverture_url, genre,
     genres_liste, pays_id, ville, type_station, etat, cree_par,
     origine_publication, role_partie_prenante, a_la_une)
SELECT v.id, v.nom, v.slug, v.description, v.stream_url, v.image, v.genre,
       v.genres, r.pays_id, v.ville, v.type::media_content.type_station,
       'publie', r.admin_id, v.origine, 'journaliste', v.a_la_une
FROM ref_seed r,
(VALUES
    ('dddd0000-0000-0000-0000-000000000101'::uuid,
     'Africans Panafrique', 'africans-panafrique-test',
     'La voix panafricaine d''Africans : débats, cultures et innovations du continent.',
     'https://stream.zeno.fm/0r0xa792kwzuv', NULL,
     'Talk Show', ARRAY['Talk Show','Débats','Culture'], 'Abidjan',
     'internationale', 'africans', TRUE),

    ('dddd0000-0000-0000-0000-000000000102'::uuid,
     'Africans Musique', 'africans-musique-test',
     'Le meilleur des musiques africaines, du mbalax à l''amapiano.',
     'https://stream.zeno.fm/0r0xa792kwzuv', NULL,
     'Afrobeats', ARRAY['Afrobeats','Amapiano','Coupé-Décalé'], 'Dakar',
     'internationale', 'africans', FALSE),

    ('dddd0000-0000-0000-0000-000000000103'::uuid,
     'Radio Kilimandjaro', 'radio-kilimandjaro-test',
     'Station nationale tanzanienne : information de proximité et Bongo Flava.',
     'https://stream.zeno.fm/0r0xa792kwzuv', NULL,
     'Bongo Flava', ARRAY['Bongo Flava','Actualité'], 'Dar es Salaam',
     'nationale', 'territoire', TRUE),

    ('dddd0000-0000-0000-0000-000000000104'::uuid,
     'Radio Sahel Info', 'radio-sahel-info-test',
     'L''actualité du Sahel, en français et en haoussa.',
     'https://stream.zeno.fm/0r0xa792kwzuv', NULL,
     'Actualité', ARRAY['Actualité','Débats'], 'Niamey',
     'nationale', 'territoire', FALSE)
) AS v(id, nom, slug, description, stream_url, image, genre, genres, ville, type, origine, a_la_une)
ON CONFLICT (id) DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
--  2. ÉMISSIONS RADIO — au moins deux par station : une mise en évidence
--     (a_la_une) et d'autres pour alimenter la rangée horizontale.
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO media_content.programme_radio
    (id, nom_emission, slug, description, image_couverture_url, audio_url,
     info_animateur, info_producteur, pays_id, langue, categorie_radio,
     station_id, a_la_une, etat, cree_par, theme_phare_id)
SELECT v.id, v.nom, v.slug, v.description, v.image, v.audio,
       v.animateur, 'Studio Africans', r.pays_id, 'Français',
       v.categorie::media_content.categorie_radio,
       v.station, v.a_la_une, 'publie', r.admin_id, r.theme_id
FROM ref_seed r,
(VALUES
    ('dddd0000-0000-0000-0000-000000000201'::uuid, 'Le Grand Débat Panafricain',
     'grand-debat-panafricain-test',
     'Chaque semaine, trois voix du continent confrontent leurs analyses.',
     NULL,
     'https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3',
     'Aminata Diallo', 'radio_africans_international',
     'dddd0000-0000-0000-0000-000000000101'::uuid, TRUE),

    ('dddd0000-0000-0000-0000-000000000202'::uuid, 'Chroniques du Continent',
     'chroniques-du-continent-test',
     'Le récit des initiatives qui transforment l''Afrique, territoire par territoire.',
     NULL,
     'https://www.soundhelix.com/examples/mp3/SoundHelix-Song-2.mp3',
     'Kwame Mensah', 'radio_africans_international',
     'dddd0000-0000-0000-0000-000000000101'::uuid, FALSE),

    ('dddd0000-0000-0000-0000-000000000203'::uuid, 'Afrobeats Session',
     'afrobeats-session-test',
     'Deux heures de découvertes musicales, des classiques aux nouvelles scènes.',
     NULL,
     'https://www.soundhelix.com/examples/mp3/SoundHelix-Song-3.mp3',
     'DJ Fatou', 'radio_africans_international',
     'dddd0000-0000-0000-0000-000000000102'::uuid, TRUE),

    ('dddd0000-0000-0000-0000-000000000204'::uuid, 'Matinale de Dar es Salaam',
     'matinale-dar-es-salaam-test',
     'L''information locale et régionale, chaque matin.',
     NULL,
     'https://www.soundhelix.com/examples/mp3/SoundHelix-Song-4.mp3',
     'Juma Kileo', 'radio_nationale_national',
     'dddd0000-0000-0000-0000-000000000103'::uuid, TRUE),

    ('dddd0000-0000-0000-0000-000000000205'::uuid, 'Voix du Sahel',
     'voix-du-sahel-test',
     'Reportages et témoignages des territoires sahéliens.',
     NULL,
     'https://www.soundhelix.com/examples/mp3/SoundHelix-Song-5.mp3',
     'Ibrahim Sanogo', 'radio_nationale_national',
     'dddd0000-0000-0000-0000-000000000104'::uuid, TRUE)
) AS v(id, nom, slug, description, image, audio, animateur, categorie, station, a_la_une)
ON CONFLICT (id) DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
--  3. CHAÎNES TÉLÉ — deux chaînes supplémentaires, pour que la page Télé
--     présente plusieurs sections au défilement (US1).
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO media_content.chaine_tv
    (id, nom, slug, description, stream_url, image_couverture_url, categorie,
     pays_id, langue, est_en_direct, etat, cree_par, role_partie_prenante)
SELECT v.id, v.nom, v.slug, v.description, v.stream, v.image,
       v.categorie::media_content.categorie_chaine_tv,
       r.pays_id, 'Français', v.direct, 'publie',
       -- « Africans Doc » est créée par le MEMBRE, comme le serait toute chaîne
       -- née d'une proposition validée : c'est le support qui sert à éprouver
       -- le parcours d'un détenteur non administrateur.
       CASE WHEN v.par_le_membre THEN r.membre_id ELSE r.admin_id END,
       'producteur'
FROM ref_seed r,
(VALUES
    ('dddd0000-0000-0000-0000-000000000301'::uuid, 'Africans Doc',
     'africans-doc-test',
     'Documentaires et grands reportages sur les transformations du continent.',
     'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
     NULL, 'culture', TRUE, TRUE),

    ('dddd0000-0000-0000-0000-000000000302'::uuid, 'Africans Innovation',
     'africans-innovation-test',
     'L''Afrique qui invente : start-up, recherche et technologies du continent.',
     'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
     NULL, 'education', FALSE, FALSE)
) AS v(id, nom, slug, description, stream, image, categorie, direct, par_le_membre)
ON CONFLICT (id) DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
--  4. ÉMISSIONS TÉLÉ — dont UNE vedette générale de la page (a_la_une_globale).
--
--  L'index unique partiel `uq_programme_tele_a_la_une_globale` n'autorise
--  qu'une seule vedette : on retire la précédente AVANT d'insérer, sans quoi
--  le second passage du script échouerait.
-- ════════════════════════════════════════════════════════════════════════════

UPDATE media_content.programme_tele
   SET a_la_une_globale = FALSE
 WHERE a_la_une_globale
   AND deleted_at IS NULL
   AND id <> 'dddd0000-0000-0000-0000-000000000401'::uuid;

INSERT INTO media_content.programme_tele
    (id, nom_emission, slug, description, image_couverture_url, video_url,
     info_animateur, info_producteur, pays_id, langue, chaine_id,
     a_la_une, a_la_une_globale, etat, cree_par, theme_phare_id)
SELECT v.id, v.nom, v.slug, v.description, v.image, v.video,
       v.animateur, 'Africans Productions', r.pays_id, 'Français',
       v.chaine, v.a_la_une, v.globale, 'publie', r.admin_id, r.theme_id
FROM ref_seed r,
(VALUES
    -- La vedette plein écran de /medias/tele.
    ('dddd0000-0000-0000-0000-000000000401'::uuid, 'Afrique, terre d''avenir',
     'afrique-terre-avenir-test',
     'Grand format documentaire : ceux qui bâtissent l''Afrique de demain, du Caire au Cap.',
     NULL,
     'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
     'Nadia Benali', 'dddd0000-0000-0000-0000-000000000301'::uuid, TRUE, TRUE),

    -- Fichier TÉLÉVERSÉ, et non lien tiers : c'est la seconde branche de
    -- `LecteurMedia`, que le reste du jeu n'exerce pas (FR-056).
    -- Prérequis : `uafricas_backend/uploads/medias/videos/demo.mp4` doit
    -- exister — le dossier `uploads/` est gitignoré, donc absent d'un clone
    -- neuf. Sans le fichier, le lecteur s'affiche mais reste muet.
    ('dddd0000-0000-0000-0000-000000000402'::uuid, 'Mémoires du Continent',
     'memoires-du-continent-test',
     'Les grandes pages de l''histoire africaine, racontées par ses historiens.',
     NULL,
     '/uploads/medias/videos/demo.mp4',
     'Cheikh Ndiaye', 'dddd0000-0000-0000-0000-000000000301'::uuid, FALSE, FALSE),

    ('dddd0000-0000-0000-0000-000000000403'::uuid, 'Start-up Afrique',
     'start-up-afrique-test',
     'Portraits d''entrepreneurs qui inventent des solutions africaines.',
     NULL,
     'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
     'Grace Okonkwo', 'dddd0000-0000-0000-0000-000000000302'::uuid, TRUE, FALSE),

    ('dddd0000-0000-0000-0000-000000000404'::uuid, 'Labo Africain',
     'labo-africain-test',
     'La recherche africaine à l''œuvre : santé, énergie, agriculture.',
     NULL,
     'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
     'Yao Kouassi', 'dddd0000-0000-0000-0000-000000000302'::uuid, FALSE, FALSE)
) AS v(id, nom, slug, description, image, video, animateur, chaine, a_la_une, globale)
ON CONFLICT (id) DO NOTHING;

-- Rétablit la vedette si le script est rejoué après un changement manuel.
UPDATE media_content.programme_tele
   SET a_la_une_globale = TRUE
 WHERE id = 'dddd0000-0000-0000-0000-000000000401'::uuid;


-- ════════════════════════════════════════════════════════════════════════════
--  5. CO-DÉTENTION (US5)
--
--  « Africans Doc » appartient au MEMBRE : c'est le support qui éprouve le
--  parcours d'un détenteur ordinaire — grille, équipe, et décision sur les
--  idées et demandes d'animation reçues (US6). L'administrateur n'y est que
--  co-détenteur, pour vérifier qu'un non-propriétaire ne peut pas tout faire.
--  `uq_support_un_proprietaire` n'admet qu'un propriétaire par support.
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO media_content.support_detenteur
    (type_support, support_id, utilisateur_id, role, designe_par, actif)
SELECT v.type::media_content.type_support_media, v.support, v.qui,
       v.role::media_content.role_detenteur, r.admin_id, TRUE
FROM ref_seed r,
(VALUES
    ('chaine_tv',    'dddd0000-0000-0000-0000-000000000301'::uuid, 'membre', 'proprietaire'),
    ('chaine_tv',    'dddd0000-0000-0000-0000-000000000301'::uuid, 'admin',  'co_detenteur'),
    ('chaine_tv',    'dddd0000-0000-0000-0000-000000000302'::uuid, 'admin',  'proprietaire'),
    ('station_radio','dddd0000-0000-0000-0000-000000000101'::uuid, 'admin',  'proprietaire'),
    ('station_radio','dddd0000-0000-0000-0000-000000000101'::uuid, 'membre', 'programmateur'),
    ('station_radio','dddd0000-0000-0000-0000-000000000103'::uuid, 'admin',  'proprietaire')
) AS s(type, support, cible, role)
CROSS JOIN LATERAL (
    SELECT s.type, s.support, s.role,
           CASE s.cible WHEN 'admin' THEN r.admin_id ELSE r.membre_id END AS qui
) v
ON CONFLICT DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
--  6. GRILLE DE PROGRAMMATION (US5)
--
--  Deux créneaux calculés à l'instant présent, dans le fuseau du créneau :
--    • l'un DÉJÀ COMMENCÉ  → alimente « En ce moment »
--    • l'autre dans ~3 min → alimente « À suivre »
--
--  La résolution étant paresseuse, il suffit de recharger la page à l'échéance
--  pour voir la bascule : aucune tâche de fond n'est à attendre.
--
--  Le CHECK de non-franchissement de minuit interdit `heure_debut + durée`
--  au-delà de 24:00 — d'où le repli sur 08:00 quand le script est joué en fin
--  de soirée.
-- ════════════════════════════════════════════════════════════════════════════

DELETE FROM media_content.creneau_programmation
 WHERE id IN ('dddd0000-0000-0000-0000-000000000501'::uuid,
              'dddd0000-0000-0000-0000-000000000502'::uuid,
              'dddd0000-0000-0000-0000-000000000503'::uuid,
              'dddd0000-0000-0000-0000-000000000504'::uuid);

INSERT INTO media_content.creneau_programmation
    (id, type_support, support_id, contenu_id, recurrence, jour_semaine,
     heure_debut, duree_minutes, fuseau, cree_par, actif)
SELECT c.id,
       c.type::media_content.type_support_media,
       c.support, c.contenu, 'quotidien', NULL,
       c.debut, c.duree, 'Africa/Abidjan', r.admin_id, TRUE
FROM ref_seed r,
LATERAL (
    SELECT (NOW() AT TIME ZONE 'Africa/Abidjan')::time AS maintenant
) t,
LATERAL (
    -- Repli matinal si l'heure courante ne laisse pas la place aux créneaux.
    SELECT CASE WHEN t.maintenant > TIME '22:00' THEN TIME '08:00'
                ELSE t.maintenant END AS base
) b,
(VALUES
    -- En cours : commencé il y a 2 min, s'achève dans 3 min. Court à dessein,
    -- pour que la bascule vers le créneau suivant soit observable pendant la
    -- session de test — il suffit de recharger la page à l'échéance.
    ('dddd0000-0000-0000-0000-000000000501'::uuid, 'chaine_tv',
     'dddd0000-0000-0000-0000-000000000301'::uuid,
     'dddd0000-0000-0000-0000-000000000402'::uuid, -2, 5),
    -- À suivre : enchaîne exactement à la fin du précédent. Contigu et non
    -- chevauchant — un chevauchement serait refusé par l'API (409, FR-040).
    ('dddd0000-0000-0000-0000-000000000502'::uuid, 'chaine_tv',
     'dddd0000-0000-0000-0000-000000000301'::uuid,
     'dddd0000-0000-0000-0000-000000000401'::uuid, 3, 45),
    -- Troisième créneau, plus tard dans la journée : « À suivre » reste
    -- renseigné une fois la bascule ci-dessus survenue, comme dans une vraie
    -- grille.
    ('dddd0000-0000-0000-0000-000000000504'::uuid, 'chaine_tv',
     'dddd0000-0000-0000-0000-000000000301'::uuid,
     'dddd0000-0000-0000-0000-000000000402'::uuid, 120, 30),
    -- Côté radio, un créneau en cours de longue durée.
    ('dddd0000-0000-0000-0000-000000000503'::uuid, 'station_radio',
     'dddd0000-0000-0000-0000-000000000101'::uuid,
     'dddd0000-0000-0000-0000-000000000202'::uuid, -5, 60)
) AS v(id, type, support, contenu, decalage_min, duree)
CROSS JOIN LATERAL (
    SELECT v.id, v.type, v.support, v.contenu, v.duree,
           (b.base + make_interval(mins => v.decalage_min))::time AS debut
) c
-- Garde-fou : n'insère que si le créneau tient dans la journée.
WHERE (c.debut + make_interval(mins => c.duree)) <= TIME '24:00'
  AND c.debut >= TIME '00:00';


-- ════════════════════════════════════════════════════════════════════════════
--  7. PROPOSITIONS EN ATTENTE (US4/US6) — alimentent la file
--     /admin/medias/propositions et le suivi /mon-compte/propositions-medias.
--
--  `statut = 'en_attente'` impose decideur et decide_at à NULL
--  (ck_prop_media_decision_coherente).
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO media_content.proposition_media
    (id, auteur_id, type_objet, target_id, donnees, justification, statut)
SELECT v.id,
       -- Les demandes VISANT un support émanent d'un tiers : le détenteur ne
       -- déciderait pas de ses propres demandes. Les propositions d'objets
       -- nouveaux restent au membre, pour son suivi dans /mon-compte.
       CASE WHEN v.cible IS NULL THEN r.membre_id ELSE COALESCE(r.tiers_id, r.membre_id) END,
       v.type::media_content.type_objet_propose,
       v.cible, v.donnees::jsonb, v.justification, 'en_attente'
FROM ref_seed r,
(VALUES
    -- Une chaîne proposée par un membre : rien ne doit paraître publiquement.
    ('dddd0000-0000-0000-0000-000000000601'::uuid, 'chaine_tv', NULL::uuid,
     '{"nom":"Télé Grands Lacs","description":"Chaîne généraliste de la région des Grands Lacs.","categorie":"generaliste","langue":"Français","role_partie_prenante":"promoteur"}',
     'Je dirige cette chaîne et souhaite la faire connaître sur Africans.'),

    -- Une émission radio proposée avec un thème phare « Autre » précisé.
    ('dddd0000-0000-0000-0000-000000000602'::uuid, 'programme_radio', NULL::uuid,
     '{"nom_emission":"Paroles de Diaspora","description":"Témoignages de la diaspora africaine.","langue":"Français","theme_phare_autre":"Diaspora et retour au pays"}',
     'Émission hebdomadaire produite depuis Bruxelles, diffusée sur trois radios.'),

    -- Une IDÉE déposée sur une chaîne : ne crée aucun objet à la validation.
    ('dddd0000-0000-0000-0000-000000000603'::uuid, 'idee_contenu',
     'dddd0000-0000-0000-0000-000000000301'::uuid,
     '{"titre":"Série sur les femmes scientifiques africaines","description":"Un portrait par épisode, dans dix territoires."}',
     'Ce sujet manque cruellement sur les antennes du continent.'),

    -- Une DEMANDE D''ANIMATION visant la même chaîne : l''acceptation ajoutera
    -- son auteur aux co-détenteurs du support (FR-045). Elle porte sur
    -- « Africans Doc » pour que son propriétaire ait les DEUX types de demande
    -- à arbitrer depuis un seul support.
    ('dddd0000-0000-0000-0000-000000000604'::uuid, 'animation_programme',
     'dddd0000-0000-0000-0000-000000000301'::uuid,
     '{"motivation":"Documentariste depuis 12 ans, je souhaite animer une case documentaire.","experience":"RTI puis Canal+ Afrique, 2014-2023"}',
     'Je propose d''animer bénévolement la case documentaire du dimanche.')
) AS v(id, type, cible, donnees, justification)
ON CONFLICT (id) DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
--  8. INVITATION DE CO-DÉTENTION EN ATTENTE — visible sur
--     /mon-compte/invitations-medias, côté membre.
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO media_content.invitation_detenteur
    (type_support, support_id, email_invite, utilisateur_invite_id, role,
     invite_par, statut)
SELECT 'chaine_tv'::media_content.type_support_media,
       'dddd0000-0000-0000-0000-000000000302'::uuid,
       'test-user@test.com',
       r.membre_id,
       'co_detenteur'::media_content.role_detenteur,
       r.admin_id, 'en_attente'
FROM ref_seed r
-- Pas de contrainte d'unicité sur cette table : le doublon se garde à la main,
-- sans quoi chaque rejeu du script empilerait une invitation de plus.
WHERE NOT EXISTS (
    SELECT 1 FROM media_content.invitation_detenteur i
     WHERE i.support_id = 'dddd0000-0000-0000-0000-000000000302'::uuid
       AND i.email_invite = 'test-user@test.com'
);


COMMIT;

-- ── Récapitulatif ───────────────────────────────────────────────────────────

\echo ''
\echo '=== Jeu de données de test installé ==='
SELECT 'stations radio  : ' || count(*) FILTER (WHERE origine_publication = 'africans')
       || ' africans / ' || count(*) FILTER (WHERE origine_publication = 'territoire') || ' territoire'
  FROM media_content.station_radio WHERE deleted_at IS NULL AND etat = 'publie'
UNION ALL
SELECT 'chaînes télé    : ' || count(*)
  FROM media_content.chaine_tv WHERE deleted_at IS NULL AND etat = 'publie'
UNION ALL
SELECT 'émissions télé  : ' || count(*) || ' (dont ' || count(*) FILTER (WHERE a_la_une_globale) || ' vedette générale)'
  FROM media_content.programme_tele WHERE deleted_at IS NULL AND etat = 'publie'
UNION ALL
SELECT 'émissions radio : ' || count(*)
  FROM media_content.programme_radio WHERE deleted_at IS NULL AND etat = 'publie'
UNION ALL
SELECT 'créneaux actifs : ' || count(*)
  FROM media_content.creneau_programmation WHERE actif AND deleted_at IS NULL
UNION ALL
SELECT 'propositions en attente : ' || count(*)
  FROM media_content.proposition_media WHERE statut = 'en_attente';
