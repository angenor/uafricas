-- ============================================================================
-- 012 — Vitrine télé mise en cohérence avec les DEUX filtres de la barre
--
-- La barre de `/medias/tele` porte depuis 09u deux référentiels disjoints :
--   • « Africans Thématique »        → les 22 genres de grille (09s),
--     déclarés par toutes les chaînes, quelle que soit leur origine ;
--   • « Africans Télé International » → les 44 lignes éditoriales (09u),
--     réservées aux chaînes de la plateforme (`origine_publication='africans'`),
--     la pastille activant cette origine dès qu'une ligne est cochée.
--
-- Le jeu 011 ne connaissait que le premier : aucune chaîne ne déclarait la
-- moindre ligne éditoriale, et le panneau des 44 s'ouvrait donc entièrement
-- à « (0) » — un filtre qui ne peut rien rendre. Ce jeu ferme l'écart.
--
-- ── Ce qu'il fait ───────────────────────────────────────────────────────────
--   1. PURGE les 9 chaînes de test laissées par les essais manuels
--      (« test chaine 2 », « Chaîne de test 007 »…). Aucun seed ne les crée :
--      elles n'ont ni couverture, ni description, ni thématique, et 011 se
--      contentait de les masquer en `brouillon`. Masquées, elles restaient
--      hors de portée des deux filtres tout en encombrant le back-office.
--   2. AJOUTE 3 chaînes de la plateforme (Gouvernance, Jeunesse, Terroirs).
--      Trois chaînes `africans` ne pouvaient pas porter 44 lignes éditoriales
--      sans que chacune en déclare quinze — un pied de charte, pas une ligne.
--      Six chaînes les répartissent avec des recoupements crédibles (1 à 2
--      chaînes par ligne), ce qui est aussi ce qui rend le filtre lisible :
--      un décompte qui varie se lit, un décompte constant ne dit rien.
--   3. DÉCLARE les 44 lignes sur les 6 chaînes `africans` : **aucune ligne
--      n'est orpheline**, chacune remonte au moins une chaîne. C'est la
--      condition pour que le panneau ne propose jamais un filtre vide.
--   4. RÉPARE deux incohérences héritées : « Panorama Continental » était
--      publiée sans couverture territoriale NI couverture continentale (donc
--      introuvable par le filtre Territoire), et deux de ses programmes
--      n'avaient pas de thème phare.
--
-- ── Ordre d'application ─────────────────────────────────────────────────────
-- APRÈS `011_demo_tele_vitrine.sql` : celui-ci réécrit intégralement les
-- thématiques des 12 chaînes qu'il gère (DELETE puis INSERT), et effacerait
-- donc les lignes éditoriales posées ici sur `africans-tele-international`.
-- Rejouer 011 impose de rejouer 012. Les deux sont idempotents.
--
-- Application :
--   docker exec -i uafricas_postgres psql -U uafricas -d africans_db < 012_...sql
-- ============================================================================

\set ON_ERROR_STOP on

BEGIN;

-- ────────────────────────────────────────────────────────────────────────────
-- Définitions déclaratives, tenues hors de la mécanique d'insertion pour que
-- le contenu éditorial reste lisible et modifiable sans y toucher.
-- ────────────────────────────────────────────────────────────────────────────

CREATE TEMP TABLE tmp_chaine012 (
    ordre        INT,
    nom          TEXT,
    slug         TEXT,
    description  TEXT,
    categorie    TEXT,
    langue       TEXT,
    en_direct    BOOLEAN,
    genres       TEXT[],   -- 22 genres de grille (09s) → « Africans Thématique »
    email        TEXT,
    site_web     TEXT
) ON COMMIT DROP;

-- Les trois sont des chaînes de la plateforme : `origine_publication` vaut
-- 'africans' et la couverture est continentale — le trigger d'exclusivité
-- refuse alors tout territoire nommé, on n'en pose donc aucun.
INSERT INTO tmp_chaine012 VALUES
(1, 'Africans Gouvernance', 'africans-gouvernance',
 'La chaîne de la vie publique africaine : ce que les institutions décident, ce qu''elles appliquent, et ce qu''il en reste sur le terrain. Sessions de l''Union africaine, budgets nationaux, réformes de la justice et scrutins — Africans Gouvernance suit les textes de leur adoption à leurs effets, et confronte les responsables à leurs propres engagements.',
 'info', 'Français', TRUE,
 ARRAY['Journal télévisé','Débats et analyses','Émissions citoyennes','Grandes interviews'],
 'gouvernance@africans-world.org', 'https://www.africans-world.org'),

(2, 'Africans Jeunesse', 'africans-jeunesse',
 'L''antenne des moins de trente ans, qui sont la majorité du continent. Écoles publiques et savoirs de la rue, concours scientifiques, sport amateur, orientation et premiers emplois : Africans Jeunesse filme ce que la jeunesse africaine apprend, invente et réclame — en la laissant parler elle-même.',
 'education', 'Français', FALSE,
 ARRAY['Éducation','Émissions jeunesse','Sport','Émissions interactives'],
 'jeunesse@africans-world.org', 'https://www.africans-world.org'),

(3, 'Africans Terroirs', 'africans-terroirs',
 'Les savoirs qui ne s''écrivent pas : gestes d''atelier, recettes de famille, semences gardées, itinéraires connus des seuls habitants. Africans Terroirs parcourt le continent pour filmer ce que les terroirs africains savent faire, et celles et ceux qui le transmettent.',
 'culture', 'Français', FALSE,
 ARRAY['Culture','Vie pratique','Tourisme','Agriculture'],
 'terroirs@africans-world.org', 'https://www.africans-world.org');


CREATE TEMP TABLE tmp_emission012 (
    chaine_slug  TEXT,
    ordre        INT,
    titre        TEXT,
    slug         TEXT,
    description  TEXT,
    cadence      TEXT,     -- quotidienne | hebdomadaire | mensuelle | ponctuelle
    theme        TEXT,     -- genre de grille (09s) : le thème phare reste un GENRE
    animateur    TEXT,
    producteur   TEXT,
    nb_episodes  INT
) ON COMMIT DROP;

INSERT INTO tmp_emission012 VALUES
-- ── Africans Gouvernance ────────────────────────────────────────────────────
('africans-gouvernance', 1, 'Le Bulletin de l''Union', 'le-bulletin-de-l-union',
 'Vingt minutes chaque soir sur les décisions prises à Addis-Abeba et dans les capitales : textes adoptés, calendriers annoncés, et ce que les États en font une fois les caméras parties.',
 'quotidienne', 'Journal télévisé', 'Ngozi Okonkwo', 'Rédaction Africans Gouvernance', 4),
('africans-gouvernance', 2, 'La Chambre des Débats', 'la-chambre-des-debats',
 'Deux thèses opposées, une heure, aucun arbitre complaisant. Chaque semaine, un désaccord public sur une décision qui engage le continent, argumenté par ceux qui la portent et ceux qui la contestent.',
 'hebdomadaire', 'Débats et analyses', 'Alassane Bâ', 'Studio Gouvernance', 3),
('africans-gouvernance', 3, 'Le Droit et la Cité', 'le-droit-et-la-cite',
 'Le magazine du droit vécu : nationalité, foncier, état civil, accès à la justice. Des juristes africains y expliquent des textes que personne ne lit et dont tout le monde dépend.',
 'hebdomadaire', 'Émissions citoyennes', 'Rania Haddad', 'Africans Gouvernance Productions', 3),
('africans-gouvernance', 4, 'Comptes à Rendre', 'comptes-a-rendre',
 'Un responsable public, une promesse datée, et les chiffres qui disent si elle a été tenue. L''entretien mensuel de la chaîne, préparé trois semaines durant.',
 'mensuelle', 'Grandes interviews', 'Thabo Nkosi', 'Africans Gouvernance Productions', 3),

-- ── Africans Jeunesse ───────────────────────────────────────────────────────
('africans-jeunesse', 1, 'Les Carrés de l''Instruction', 'les-carres-de-l-instruction',
 'Une classe, un enseignant, une difficulté concrète : effectifs, manuels, langue d''enseignement. Tourné dans les écoles publiques, du primaire au lycée, sans plateau ni décor.',
 'hebdomadaire', 'Éducation', 'Awa Diarra', 'Unité Éducation Africans', 4),
('africans-jeunesse', 2, 'Génie en Herbe', 'genie-en-herbe-africans',
 'Le concours de connaissances de la chaîne : deux lycées s''affrontent sur l''histoire, les sciences et les langues du continent, devant leurs camarades.',
 'hebdomadaire', 'Émissions jeunesse', 'Kofi Mensah', 'Africans Jeunesse Productions', 3),
('africans-jeunesse', 3, 'Terrain Junior', 'terrain-junior',
 'Le sport amateur africain, des centres de formation aux terrains de quartier : ceux qui percent, ceux qui arrêtent, et ce que coûte réellement une carrière.',
 'hebdomadaire', 'Sport', 'Sipho Molefe', 'Africans Jeunesse Productions', 3),
('africans-jeunesse', 4, 'La Thèse et le Village', 'la-these-et-le-village',
 'Un travail universitaire soutenu à l''étranger, ramené là où il s''applique. Les téléspectateurs interrogent le chercheur en direct, et une communauté locale juge de l''usage.',
 'mensuelle', 'Émissions interactives', 'Fatoumata Cissé', 'Unité Éducation Africans', 3),

-- ── Africans Terroirs ───────────────────────────────────────────────────────
('africans-terroirs', 1, 'La Main et la Matière', 'la-main-et-la-matiere',
 'Un artisan, un matériau, un geste appris de quelqu''un d''autre : bronze, indigo, raphia, cuir. Filmé en atelier, du premier outil à la pièce finie.',
 'hebdomadaire', 'Culture', 'Bintou Konaté', 'Africans Terroirs Productions', 4),
('africans-terroirs', 2, 'Cuisine de Chez Nous', 'cuisine-de-chez-nous-africans',
 'Une recette par jour, filmée chez l''habitant, des oasis du Nord aux côtes de l''Atlantique. Les producteurs de l''ingrédient principal sont toujours à l''écran.',
 'quotidienne', 'Vie pratique', 'Hawa Baldé', 'Africans Terroirs Productions', 3),
('africans-terroirs', 3, 'Sentiers du Continent', 'sentiers-du-continent',
 'Des itinéraires tenus par celles et ceux qui y vivent : parcs, deltas, massifs et villages d''étape, avec ce qu''il faut savoir avant de partir et ce que le passage laisse sur place.',
 'hebdomadaire', 'Tourisme', 'Tendai Moyo', 'Studio Terroirs', 3),
('africans-terroirs', 4, 'Greniers d''Afrique', 'greniers-d-afrique',
 'Semences paysannes, greniers collectifs, marchés de gros : le magazine des filières vivrières, suivi d''une parcelle jusqu''à l''assiette.',
 'hebdomadaire', 'Agriculture', 'Seydou Ouédraogo', 'Studio Terroirs', 3);


CREATE TEMP TABLE tmp_equipe012 (
    chaine_slug TEXT, ordre INT, prenom TEXT, nom TEXT,
    fonction TEXT, territoire TEXT, contact TEXT
) ON COMMIT DROP;

INSERT INTO tmp_equipe012 VALUES
('africans-gouvernance', 1, 'Ngozi',      'Okonkwo',  'Direction de l''antenne', 'Panafricain', 'n.okonkwo@africans-world.org'),
('africans-gouvernance', 2, 'Alassane',   'Bâ',       'Rédaction en chef',       'Panafricain', 'a.ba@africans-world.org'),
('africans-gouvernance', 3, 'Rania',      'Haddad',   'Pôle juridique',          'Maghreb',     NULL),
('africans-jeunesse',    1, 'Awa',        'Diarra',   'Direction de l''antenne', 'Panafricain', 'a.diarra@africans-world.org'),
('africans-jeunesse',    2, 'Kofi',       'Mensah',   'Production exécutive',    'Afrique de l''Ouest', NULL),
('africans-jeunesse',    3, 'Sipho',      'Molefe',   'Rédaction sport',         'Afrique australe',    NULL),
('africans-terroirs',    1, 'Bintou',     'Konaté',   'Direction de l''antenne', 'Panafricain', 'b.konate@africans-world.org'),
('africans-terroirs',    2, 'Tendai',     'Moyo',     'Réalisation',             'Afrique australe',    NULL),
('africans-terroirs',    3, 'Seydou',     'Ouédraogo','Rédaction agricole',      'Sahel',               NULL);


-- ────────────────────────────────────────────────────────────────────────────
-- Répartition des 44 lignes éditoriales sur les 6 chaînes de la plateforme.
--
-- Le contrôle final échoue si une ligne du référentiel 09u reste sans chaîne :
-- c'est la seule garantie que le panneau ne propose aucun filtre stérile.
-- ────────────────────────────────────────────────────────────────────────────

CREATE TEMP TABLE tmp_ligne012 (chaine_slug TEXT, ligne TEXT) ON COMMIT DROP;

INSERT INTO tmp_ligne012 VALUES
-- Africans Télé International — l'antenne généraliste : information, monde, diaspora.
('africans-tele-international', 'Retour des cerveaux'),
('africans-tele-international', 'Journal de l''Afrique'),
('africans-tele-international', 'Haro sur les hommes de l''Afrique'),
('africans-tele-international', 'L''intellectuel africain et développement'),
('africans-tele-international', 'Politique africaine'),
('africans-tele-international', 'La voix du terrain en Afrique'),
('africans-tele-international', 'Débats africains'),
('africans-tele-international', 'Infrastructures d''Afrique'),
('africans-tele-international', 'Mondialisation et coopération africaine'),
('africans-tele-international', 'Commerce africain et unité africaine'),
('africans-tele-international', 'Le monde de demain et mondialisation'),
('africans-tele-international', 'Immigration et l''avenir de l''Afrique'),
('africans-tele-international', 'Afrique Société'),

-- Africans Histoire — mémoire, récits, création.
('africans-histoire', 'Histoire de l''Afrique'),
('africans-histoire', 'Valeurs africaines et développement'),
('africans-histoire', 'Mystères africains'),
('africans-histoire', 'Traditions d''Afrique'),
('africans-histoire', 'Cinéma africain'),
('africans-histoire', 'Séries d''Afrique'),
('africans-histoire', 'Documentaires africains'),
('africans-histoire', 'Complexes d''Afrique'),

-- Africans Innovation — sciences, techniques, environnement, santé.
('africans-innovation', 'Afrique et technologies'),
('africans-innovation', 'Numérique et développement africain'),
('africans-innovation', 'Innovations simples chez nous'),
('africans-innovation', 'Futurs génies d''Afrique'),
('africans-innovation', 'De la thèse à l''action locale'),
('africans-innovation', 'Santé et développement'),
('africans-innovation', 'Environnement d''Afrique'),
('africans-innovation', 'Développement durable'),

-- Africans Gouvernance — institutions, droit, redevabilité.
('africans-gouvernance', 'Journal de l''Afrique'),
('africans-gouvernance', 'Politique africaine'),
('africans-gouvernance', 'Débats africains'),
('africans-gouvernance', 'Droit africain'),
('africans-gouvernance', 'Gouvernance d''Afrique aux défis'),
('africans-gouvernance', 'L''Afrique que nous voulons'),
('africans-gouvernance', 'Messages aux gouvernants'),
('africans-gouvernance', 'Rendez-vous des hauts et des bas'),
('africans-gouvernance', 'Commerce africain et unité africaine'),

-- Africans Jeunesse — écoles, savoirs, sport, relève.
('africans-jeunesse', 'Regards de la jeunesse africaine'),
('africans-jeunesse', 'Futurs génies d''Afrique'),
('africans-jeunesse', 'Éducation — Les carrés de l''instruction en Afrique'),
('africans-jeunesse', 'Éducation — Les carrés de l''école de la vie'),
('africans-jeunesse', 'Éducation — Les carrés de l''éducation à l''africaine'),
('africans-jeunesse', 'De la thèse à l''action locale'),
('africans-jeunesse', 'Sports d''Afrique'),
('africans-jeunesse', 'L''intellectuel africain et développement'),

-- Africans Terroirs — savoir-faire, terroirs, solidarités.
('africans-terroirs', 'Savoirs faire d''Afrique'),
('africans-terroirs', 'Cuisine de chez nous'),
('africans-terroirs', 'Safari d''Afrique'),
('africans-terroirs', 'Femmes d''Afrique'),
('africans-terroirs', 'Afrique Solidarité'),
('africans-terroirs', 'Traditions d''Afrique'),
('africans-terroirs', 'Valeurs africaines et développement'),
('africans-terroirs', 'La voix du terrain en Afrique');


-- ────────────────────────────────────────────────────────────────────────────
-- Mécanique.
-- ────────────────────────────────────────────────────────────────────────────

DO $seed012$
DECLARE
    v_auteur   UUID;
    v_chaine   UUID;
    v_emission UUID;
    v_theme    UUID;
    v_ids      UUID[];
    v_em_ids   UUID[];
    v_ep_ids   UUID[];
    c          RECORD;
    e          RECORD;
    m          RECORD;
    i          INT;
    v_compteur INT := 0;
    v_video    TEXT;
    v_nb_prog  INT;
    v_orphelines INT;
    -- Identifiants YouTube repris des jeux 009 et 011, vérifiés un à un par
    -- oEmbed : réutiliser des vidéos éprouvées évite les lecteurs noirs.
    v_videos   TEXT[] := ARRAY[
        'JeVaVtr_DCE','o_JuUo3XqG4','tuCIq9NPvQ4','ecdabz94_Co','Cm5yOJc_NLo',
        'NbZaaAdf5Aw','9z52xavACQY','OvTMkEYu6l8','W_hQj5mkvaI','RV6lvELxBuo',
        'Ofn31if1Fac','KzjEhgcBvSE','r7AaktS648I','u9uf-cd63Po','jx_FiRs39s8'];
    -- Grille couvrant les 24 h sans trou ni chevauchement : la somme des durées
    -- vaut 1440 minutes et aucun créneau ne franchit minuit (ck_creneau_pas_minuit).
    v_heures   TIME[] := ARRAY['00:00','06:00','08:00','09:30','11:00','12:00','13:00',
                               '15:00','16:30','18:00','19:00','20:00','22:00']::TIME[];
    v_durees   INT[]  := ARRAY[360, 120, 90, 90, 60, 60, 120, 90, 90, 60, 60, 120, 120];
BEGIN
    SELECT id INTO v_auteur FROM iam.utilisateur WHERE email = 'angenor99@gmail.com';
    IF v_auteur IS NULL THEN
        SELECT id INTO v_auteur FROM iam.utilisateur WHERE email = 'test-admin@test.com';
    END IF;
    IF v_auteur IS NULL THEN
        RAISE EXCEPTION 'Compte auteur introuvable — seed interrompu';
    END IF;

    -- ========================================================================
    -- 0. Purge des chaînes de test
    -- ------------------------------------------------------------------------
    -- Résidus d'essais manuels : aucun seed ne les crée, elles n'ont ni
    -- couverture ni thématique, et 011 se bornait à les masquer. Les tables de
    -- liaison et d'interaction étant POLYMORPHES (pas de clé étrangère sur
    -- `media_id`/`support_id`), rien ne s'efface en cascade : chaque niveau est
    -- nettoyé explicitement. Seul `emission_tele.chaine_id` cascade, et
    -- `episode_tele.emission_id` est en RESTRICT — d'où l'ordre épisodes,
    -- puis programmes, puis chaînes.
    -- ========================================================================
    SELECT array_agg(id) INTO v_ids
      FROM media_content.chaine_tv
     WHERE slug IN ('africa24-test','chaine-vide','sahel-culture','test-chaine-1',
                    'test-chaine-2','chaine-test-007','chaine-proposee-007',
                    'africans-doc-test','africans-innovation-test');

    IF v_ids IS NOT NULL THEN
        SELECT array_agg(id) INTO v_em_ids
          FROM media_content.emission_tele WHERE chaine_id = ANY(v_ids);

        IF v_em_ids IS NOT NULL THEN
            SELECT array_agg(id) INTO v_ep_ids
              FROM media_content.episode_tele WHERE emission_id = ANY(v_em_ids);

            IF v_ep_ids IS NOT NULL THEN
                DELETE FROM media_content.media_reaction    WHERE type_media = 'episode_tele' AND media_id = ANY(v_ep_ids);
                DELETE FROM media_content.media_commentaire WHERE type_media = 'episode_tele' AND media_id = ANY(v_ep_ids);
                DELETE FROM media_content.partage_media     WHERE type_media = 'episode_tele' AND media_id = ANY(v_ep_ids);
                DELETE FROM media_content.signalement_media WHERE type_media = 'episode_tele' AND media_id = ANY(v_ep_ids);
                DELETE FROM media_content.episode_tele      WHERE id = ANY(v_ep_ids);
            END IF;

            DELETE FROM media_content.media_reaction    WHERE type_media = 'emission_tele' AND media_id = ANY(v_em_ids);
            DELETE FROM media_content.media_commentaire WHERE type_media = 'emission_tele' AND media_id = ANY(v_em_ids);
            DELETE FROM media_content.partage_media     WHERE type_media = 'emission_tele' AND media_id = ANY(v_em_ids);
            DELETE FROM media_content.signalement_media WHERE type_media = 'emission_tele' AND media_id = ANY(v_em_ids);
            DELETE FROM media_content.membre_equipe     WHERE type_porteur = 'emission_tele' AND porteur_id = ANY(v_em_ids);
        END IF;

        DELETE FROM media_content.creneau_programmation WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.membre_equipe         WHERE type_porteur = 'chaine_tv'  AND porteur_id = ANY(v_ids);
        DELETE FROM media_content.support_thematique    WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.support_territoire    WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.invitation_detenteur  WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.support_detenteur     WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.media_reaction        WHERE type_media = 'chaine_tv' AND media_id = ANY(v_ids);
        DELETE FROM media_content.media_commentaire     WHERE type_media = 'chaine_tv' AND media_id = ANY(v_ids);
        DELETE FROM media_content.partage_media         WHERE type_media = 'chaine_tv' AND media_id = ANY(v_ids);
        DELETE FROM media_content.signalement_media     WHERE type_media = 'chaine_tv' AND media_id = ANY(v_ids);

        -- Une proposition validée pointe l'objet créé sans clé étrangère : la
        -- laisser derrière une chaîne supprimée mènerait le membre vers un 404
        -- depuis « Mes propositions ».
        DELETE FROM media_content.proposition_media WHERE objet_id_cree = ANY(v_ids);

        DELETE FROM media_content.emission_tele WHERE chaine_id = ANY(v_ids);
        DELETE FROM media_content.chaine_tv     WHERE id = ANY(v_ids);

        RAISE NOTICE '012 — % chaînes de test purgées.', array_length(v_ids, 1);
    END IF;

    -- ========================================================================
    -- 1. Les trois nouvelles chaînes de la plateforme
    -- ========================================================================
    FOR c IN SELECT * FROM tmp_chaine012 ORDER BY ordre LOOP
        INSERT INTO media_content.chaine_tv
            (nom, slug, description, image_couverture_url, categorie, pays_id, langue,
             est_en_direct, etat, origine_publication, couverture_continentale,
             stream_url, contact_email, contact_site_web, cree_par)
        VALUES
            (c.nom, c.slug, c.description,
             'https://picsum.photos/seed/uafricas-' || c.slug || '/600/600',
             c.categorie::media_content.categorie_chaine_tv, NULL, c.langue,
             c.en_direct, 'publie', 'africans', TRUE,
             CASE WHEN c.en_direct THEN 'https://www.youtube.com/watch?v=' || v_videos[1 + (c.ordre % 15)] END,
             c.email, c.site_web, v_auteur)
        ON CONFLICT (slug) DO UPDATE SET
            nom = EXCLUDED.nom,
            description = EXCLUDED.description,
            image_couverture_url = EXCLUDED.image_couverture_url,
            categorie = EXCLUDED.categorie,
            langue = EXCLUDED.langue,
            est_en_direct = EXCLUDED.est_en_direct,
            etat = 'publie',
            origine_publication = 'africans',
            couverture_continentale = TRUE,
            stream_url = EXCLUDED.stream_url,
            contact_email = EXCLUDED.contact_email,
            contact_site_web = EXCLUDED.contact_site_web
        RETURNING id INTO v_chaine;

        -- Genres de grille (« Africans Thématique ») — remplacement intégral.
        -- `parent_id IS NULL` borne la résolution au référentiel générique :
        -- les 44 lignes vivent dans le même contexte 'media' et un nom mal
        -- orthographié irait sinon chercher au hasard dans les deux listes.
        DELETE FROM media_content.support_thematique
              WHERE type_support = 'chaine_tv' AND support_id = v_chaine;
        INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
        SELECT 'chaine_tv', v_chaine, cat.id
          FROM shared.categorie cat
         WHERE cat.contexte = 'media' AND cat.actif AND cat.parent_id IS NULL
           AND cat.nom = ANY(c.genres)
        ON CONFLICT DO NOTHING;

        -- Couverture continentale : le trigger `verifier_couverture_exclusive`
        -- refuse tout territoire nommé, on n'en pose donc aucun.
        DELETE FROM media_content.support_territoire
              WHERE type_support = 'chaine_tv' AND support_id = v_chaine;

        DELETE FROM media_content.membre_equipe
              WHERE type_porteur = 'chaine_tv' AND porteur_id = v_chaine;
        FOR m IN SELECT * FROM tmp_equipe012 WHERE chaine_slug = c.slug ORDER BY ordre LOOP
            INSERT INTO media_content.membre_equipe
                (type_porteur, porteur_id, nom, prenom, fonction, territoire, contact, ordre, cree_par)
            VALUES ('chaine_tv', v_chaine, m.nom, m.prenom, m.fonction, m.territoire,
                    m.contact, m.ordre, v_auteur);
        END LOOP;
    END LOOP;

    -- ========================================================================
    -- 2. Programmes et épisodes
    -- ========================================================================
    FOR e IN SELECT * FROM tmp_emission012 ORDER BY chaine_slug, ordre LOOP
        SELECT id INTO v_chaine FROM media_content.chaine_tv WHERE slug = e.chaine_slug;

        -- Le thème phare d'un programme est un GENRE, jamais une ligne
        -- éditoriale : la ligne qualifie l'antenne, le genre qualifie la case.
        SELECT id INTO v_theme FROM shared.categorie
         WHERE contexte = 'media' AND actif AND parent_id IS NULL AND nom = e.theme
         LIMIT 1;

        INSERT INTO media_content.emission_tele
            (chaine_id, titre, slug, description, image_couverture_url,
             info_animateur, info_producteur, langue, theme_phare_id, cadence, etat, cree_par)
        VALUES
            (v_chaine, e.titre, e.slug, e.description,
             'https://picsum.photos/seed/uafricas-' || e.slug || '/960/540',
             e.animateur, e.producteur,
             (SELECT langue FROM media_content.chaine_tv WHERE id = v_chaine),
             v_theme, e.cadence, 'publie', v_auteur)
        ON CONFLICT (slug) DO UPDATE SET
            chaine_id = EXCLUDED.chaine_id,
            titre = EXCLUDED.titre,
            description = EXCLUDED.description,
            image_couverture_url = EXCLUDED.image_couverture_url,
            info_animateur = EXCLUDED.info_animateur,
            info_producteur = EXCLUDED.info_producteur,
            theme_phare_id = EXCLUDED.theme_phare_id,
            cadence = EXCLUDED.cadence,
            etat = 'publie'
        RETURNING id INTO v_emission;

        FOR i IN 1..e.nb_episodes LOOP
            v_compteur := v_compteur + 1;
            v_video := v_videos[1 + (v_compteur % 15)];

            INSERT INTO media_content.episode_tele
                (emission_id, titre, slug, description, image_couverture_url, video_url,
                 numero_episode, ordre, duree_minutes, a_la_une, etat,
                 valide_par, valide_at, cree_par, created_at)
            VALUES
                (v_emission,
                 e.titre || ' — n° ' || LPAD(i::TEXT, 2, '0'),
                 e.slug || '-ep-' || LPAD(i::TEXT, 2, '0'),
                 'Épisode ' || i || ' de « ' || e.titre || ' ». ' || e.description,
                 'https://i.ytimg.com/vi/' || v_video || '/maxresdefault.jpg',
                 'https://www.youtube.com/watch?v=' || v_video,
                 i, i, 18 + ((v_compteur * 7) % 62),
                 i = 1, 'publie', v_auteur, NOW(), v_auteur,
                 NOW() - ((e.nb_episodes - i) * 7 || ' days')::INTERVAL)
            ON CONFLICT (slug) DO UPDATE SET
                emission_id = EXCLUDED.emission_id,
                titre = EXCLUDED.titre,
                description = EXCLUDED.description,
                image_couverture_url = EXCLUDED.image_couverture_url,
                video_url = EXCLUDED.video_url,
                duree_minutes = EXCLUDED.duree_minutes,
                etat = 'publie',
                valide_par = EXCLUDED.valide_par,
                valide_at = EXCLUDED.valide_at;
        END LOOP;
    END LOOP;

    -- ========================================================================
    -- 3. Grilles — 24 h couvertes, pour que « En ce moment » ne soit jamais vide
    -- ------------------------------------------------------------------------
    -- `date_effet` reculée d'un mois : la rotation des épisodes a déjà tourné,
    -- le bandeau n'annonce donc pas systématiquement le premier numéro.
    -- ========================================================================
    FOR c IN SELECT * FROM tmp_chaine012 ORDER BY ordre LOOP
        SELECT id INTO v_chaine FROM media_content.chaine_tv WHERE slug = c.slug;

        DELETE FROM media_content.creneau_programmation
              WHERE type_support = 'chaine_tv' AND support_id = v_chaine;

        SELECT COUNT(*) INTO v_nb_prog FROM tmp_emission012 WHERE chaine_slug = c.slug;
        CONTINUE WHEN v_nb_prog = 0;

        FOR i IN 1..array_length(v_heures, 1) LOOP
            INSERT INTO media_content.creneau_programmation
                (type_support, support_id, emission_id, recurrence, jour_semaine,
                 heure_debut, duree_minutes, fuseau, date_effet, cree_par, actif)
            SELECT 'chaine_tv', v_chaine, em.id, 'quotidien', NULL,
                   v_heures[i], v_durees[i], 'Africa/Abidjan',
                   CURRENT_DATE - 30, v_auteur, TRUE
              FROM media_content.emission_tele em
              JOIN tmp_emission012 te ON te.slug = em.slug
             WHERE te.chaine_slug = c.slug
               AND te.ordre = 1 + ((i - 1) % v_nb_prog);
        END LOOP;
    END LOOP;

    -- ========================================================================
    -- 4. Lignes éditoriales (09u) sur les six chaînes de la plateforme
    -- ------------------------------------------------------------------------
    -- Ajout et non remplacement : les genres de grille posés plus haut (et par
    -- 011 pour les chaînes qu'il gère) vivent dans la même table et doivent
    -- survivre. La résolution est bornée au GROUPE des lignes, pour la raison
    -- symétrique de celle invoquée pour les genres.
    -- ========================================================================
    DELETE FROM media_content.support_thematique st
     USING shared.categorie cat, media_content.chaine_tv ct
     WHERE st.categorie_id = cat.id
       AND st.type_support = 'chaine_tv'
       AND st.support_id = ct.id
       AND cat.parent_id = (SELECT id FROM shared.categorie
                             WHERE slug = 'media-groupe-africans-tele-international');

    INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
    SELECT 'chaine_tv', ct.id, cat.id
      FROM tmp_ligne012 l
      JOIN media_content.chaine_tv ct ON ct.slug = l.chaine_slug AND ct.deleted_at IS NULL
      JOIN shared.categorie cat
        ON cat.nom = l.ligne
       AND cat.parent_id = (SELECT id FROM shared.categorie
                             WHERE slug = 'media-groupe-africans-tele-international')
    ON CONFLICT DO NOTHING;

    -- ========================================================================
    -- 5. Réparations héritées
    -- ------------------------------------------------------------------------
    -- « Panorama Continental » était publiée sans couverture continentale ET
    -- sans aucun territoire : introuvable par le filtre Territoire, alors même
    -- qu'elle porte la couverture du continent dans son nom.
    -- ========================================================================
    SELECT id INTO v_chaine FROM media_content.chaine_tv WHERE slug = 'panorama-continental-010';
    IF v_chaine IS NOT NULL THEN
        DELETE FROM media_content.support_territoire
              WHERE type_support = 'chaine_tv' AND support_id = v_chaine;
        UPDATE media_content.chaine_tv
           SET couverture_continentale = TRUE, updated_at = NOW()
         WHERE id = v_chaine;

        -- Deux de ses programmes n'avaient pas de thème phare : ils
        -- n'apparaissaient donc sous aucun genre.
        UPDATE media_content.emission_tele em
           SET theme_phare_id = (SELECT id FROM shared.categorie
                                  WHERE contexte = 'media' AND actif AND parent_id IS NULL
                                    AND nom = 'Investigations'),
               updated_at = NOW()
         WHERE em.chaine_id = v_chaine AND em.theme_phare_id IS NULL AND em.deleted_at IS NULL;
    END IF;

    -- ========================================================================
    -- 6. Contrôle — une ligne éditoriale orpheline vaut un filtre stérile
    -- ========================================================================
    SELECT count(*) INTO v_orphelines
      FROM shared.categorie cat
     WHERE cat.parent_id = (SELECT id FROM shared.categorie
                             WHERE slug = 'media-groupe-africans-tele-international')
       AND NOT EXISTS (
             SELECT 1 FROM media_content.support_thematique st
              JOIN media_content.chaine_tv ct ON ct.id = st.support_id
             WHERE st.categorie_id = cat.id
               AND st.type_support = 'chaine_tv'
               AND ct.etat = 'publie' AND ct.deleted_at IS NULL);

    IF v_orphelines > 0 THEN
        RAISE EXCEPTION '012 — % ligne(s) éditoriale(s) sans chaîne publiée : le panneau proposerait un filtre vide.', v_orphelines;
    END IF;
END
$seed012$;

DO $bilan$
DECLARE v_ch INT; v_af INT; v_li INT; v_ge INT;
BEGIN
    SELECT count(*) INTO v_ch FROM media_content.chaine_tv WHERE etat='publie' AND deleted_at IS NULL;
    SELECT count(*) INTO v_af FROM media_content.chaine_tv WHERE etat='publie' AND deleted_at IS NULL AND origine_publication='africans';
    SELECT count(DISTINCT cat.id) INTO v_li
      FROM media_content.support_thematique st JOIN shared.categorie cat ON cat.id=st.categorie_id
     WHERE st.type_support='chaine_tv' AND cat.parent_id IS NOT NULL;
    SELECT count(DISTINCT cat.id) INTO v_ge
      FROM media_content.support_thematique st JOIN shared.categorie cat ON cat.id=st.categorie_id
     WHERE st.type_support='chaine_tv' AND cat.parent_id IS NULL;
    RAISE NOTICE '012 — % chaînes publiées (dont % africans) · % / 44 lignes éditoriales servies · % / 22 genres servis.',
                 v_ch, v_af, v_li, v_ge;
END
$bilan$;

COMMIT;
