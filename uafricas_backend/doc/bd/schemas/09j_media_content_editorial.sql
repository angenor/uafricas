-- ============================================================================
-- 09j — media_content : éditorial des pages Télé et Radio
-- ----------------------------------------------------------------------------
-- Socle de la refonte des pages `/medias/tele`, `/medias/radio/africans` et
-- `/medias/radio/nationales` (feature 001-refonte-tele-radio) :
--
--   • station_radio.origine_publication — départage réellement les deux pages
--     Radio, qui affichaient jusqu'ici le même contenu (FR-014) ;
--   • programme_tele.a_la_une_globale    — vedette unique de la page Télé,
--     distincte du « à la une » par chaîne déjà en place (FR-001) ;
--   • theme_phare / role_partie_prenante — référentiels éditoriaux exigés à la
--     soumission (FR-029, FR-030) ;
--   • etat = 'en_attente' + nombre_signalements — préparent la validation
--     administrative (FR-032) et le retrait sur signalement (FR-050).
--
-- Corrige au passage trois dettes bloquantes pour cette feature :
--   • station_radio n'avait AUCUNE clé étrangère ;
--   • aucune permission 'media' n'était seedée — seul super_admin (all.all)
--     pouvait modérer les médias ;
--   • notifications.type VARCHAR(30) était déjà trop court pour les types
--     existants, et le devient plus encore avec ceux de cette feature.
--
-- Migration idempotente : ADD COLUMN IF NOT EXISTS, DROP puis ADD CONSTRAINT,
-- CREATE INDEX IF NOT EXISTS, blocs DO $$ sur pg_constraint, ON CONFLICT.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. Origine de publication des stations de radio (FR-014)
-- ════════════════════════════════════════════════════════════════════════════
-- 'africans'   → /medias/radio/africans   (production propre de la plateforme,
--                                          décision éditoriale de ses créateurs)
-- 'territoire' → /medias/radio/nationales (station rattachée à un territoire)
--
-- Le défaut 'territoire' qualifie tout l'existant du côté Nationales. Le
-- basculement des stations relevant de Radio Africans se fait après livraison,
-- par UPDATE ciblé (cf. quickstart.md, reprise de données).
-- Invariant : NOT NULL + CHECK ⇒ une station relève d'exactement une page.

ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS origine_publication VARCHAR(20) NOT NULL DEFAULT 'territoire';

ALTER TABLE media_content.station_radio
    DROP CONSTRAINT IF EXISTS ck_station_radio_origine;
ALTER TABLE media_content.station_radio
    ADD CONSTRAINT ck_station_radio_origine
        CHECK (origine_publication IN ('africans', 'territoire'));

CREATE INDEX IF NOT EXISTS idx_station_radio_origine
    ON media_content.station_radio (origine_publication) WHERE deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- 2. Clés étrangères manquantes sur station_radio
-- ════════════════════════════════════════════════════════════════════════════
-- La table n'en portait aucune, contrairement à programme_radio (09g:87-119).

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_station_radio_pays') THEN
        ALTER TABLE media_content.station_radio
            ADD CONSTRAINT fk_station_radio_pays FOREIGN KEY (pays_id)
            REFERENCES shared.pays(id) ON DELETE SET NULL;
    END IF;
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'fk_station_radio_cree_par') THEN
        ALTER TABLE media_content.station_radio
            ADD CONSTRAINT fk_station_radio_cree_par FOREIGN KEY (cree_par)
            REFERENCES iam.utilisateur(id) ON DELETE RESTRICT;
    END IF;
END $$;


-- ════════════════════════════════════════════════════════════════════════════
-- 3. Vedette générale de la page Télé (FR-001, FR-007)
-- ════════════════════════════════════════════════════════════════════════════
-- Deux portées de mise en avant coexistent sans interférence :
--   • a_la_une_globale — 1 seule pour TOUTE la table  → vedette plein écran
--   • a_la_une         — 1 par chaîne (uq_…_par_chaine, 09g:82) → section
--
-- L'index sur l'expression constante ((TRUE)) impose l'unicité globale.
-- Conséquence côté Rust : la bascule de l'ancienne vedette à FALSE et la
-- désignation de la nouvelle DOIVENT tenir dans une même transaction, sinon la
-- seconde requête viole l'index en concurrence.

ALTER TABLE media_content.programme_tele
    ADD COLUMN IF NOT EXISTS a_la_une_globale BOOLEAN NOT NULL DEFAULT FALSE;

CREATE UNIQUE INDEX IF NOT EXISTS uq_programme_tele_a_la_une_globale
    ON media_content.programme_tele ((TRUE))
    WHERE a_la_une_globale = TRUE AND deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- 4. Thème phare des contenus (FR-030)
-- ════════════════════════════════════════════════════════════════════════════
-- theme_phare_id est une référence LOGIQUE vers shared.categorie, sans FK —
-- convention [xref] du projet, comme secteur_id (05c:14) et categorie_id.

ALTER TABLE media_content.programme_tele
    ADD COLUMN IF NOT EXISTS theme_phare_id    UUID,          -- [xref] shared.categorie (contexte='media')
    ADD COLUMN IF NOT EXISTS theme_phare_autre VARCHAR(200);

ALTER TABLE media_content.programme_radio
    ADD COLUMN IF NOT EXISTS theme_phare_id    UUID,          -- [xref] shared.categorie (contexte='media')
    ADD COLUMN IF NOT EXISTS theme_phare_autre VARCHAR(200);

-- Cohérence : on n'accepte pas un « Autre » vide. L'ABSENCE totale de thème
-- reste tolérée — les contenus créés avant cette migration n'en ont aucun, et
-- l'obligation d'en choisir un ne vaut qu'à la soumission (garde applicative,
-- FR-030). Un CHECK exigeant un thème sur toute ligne rejetterait l'existant.
ALTER TABLE media_content.programme_tele
    DROP CONSTRAINT IF EXISTS ck_programme_tele_theme_autre;
ALTER TABLE media_content.programme_tele
    ADD CONSTRAINT ck_programme_tele_theme_autre
        CHECK (theme_phare_autre IS NULL OR btrim(theme_phare_autre) <> '');

ALTER TABLE media_content.programme_radio
    DROP CONSTRAINT IF EXISTS ck_programme_radio_theme_autre;
ALTER TABLE media_content.programme_radio
    ADD CONSTRAINT ck_programme_radio_theme_autre
        CHECK (theme_phare_autre IS NULL OR btrim(theme_phare_autre) <> '');

CREATE INDEX IF NOT EXISTS idx_programme_tele_theme_phare
    ON media_content.programme_tele (theme_phare_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_programme_radio_theme_phare
    ON media_content.programme_radio (theme_phare_id) WHERE deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- 5. Rôle de partie prenante des supports (FR-029)
-- ════════════════════════════════════════════════════════════════════════════
-- Déclaré par le membre qui soumet une chaîne ou une station. NULL sur
-- l'existant, saisi par l'administrateur ou le contributeur ensuite.
-- « Autre » exige une précision — invariant exprimé en SQL, pas seulement
-- dans le handler.

ALTER TABLE media_content.chaine_tv
    ADD COLUMN IF NOT EXISTS role_partie_prenante       VARCHAR(40),
    ADD COLUMN IF NOT EXISTS role_partie_prenante_autre VARCHAR(200);

ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS role_partie_prenante       VARCHAR(40),
    ADD COLUMN IF NOT EXISTS role_partie_prenante_autre VARCHAR(200);

ALTER TABLE media_content.chaine_tv
    DROP CONSTRAINT IF EXISTS ck_chaine_tv_role_partie_prenante;
ALTER TABLE media_content.chaine_tv
    ADD CONSTRAINT ck_chaine_tv_role_partie_prenante
        CHECK (role_partie_prenante IS NULL OR role_partie_prenante IN (
            'chaine_tele', 'radio', 'journaliste', 'communicateur',
            'createur_contenu', 'influenceur', 'realisateur', 'producteur', 'autre'));

ALTER TABLE media_content.chaine_tv
    DROP CONSTRAINT IF EXISTS ck_chaine_tv_role_autre_precise;
ALTER TABLE media_content.chaine_tv
    ADD CONSTRAINT ck_chaine_tv_role_autre_precise
        CHECK (role_partie_prenante IS DISTINCT FROM 'autre'
               OR (role_partie_prenante_autre IS NOT NULL
                   AND btrim(role_partie_prenante_autre) <> ''));

ALTER TABLE media_content.station_radio
    DROP CONSTRAINT IF EXISTS ck_station_radio_role_partie_prenante;
ALTER TABLE media_content.station_radio
    ADD CONSTRAINT ck_station_radio_role_partie_prenante
        CHECK (role_partie_prenante IS NULL OR role_partie_prenante IN (
            'chaine_tele', 'radio', 'journaliste', 'communicateur',
            'createur_contenu', 'influenceur', 'realisateur', 'producteur', 'autre'));

ALTER TABLE media_content.station_radio
    DROP CONSTRAINT IF EXISTS ck_station_radio_role_autre_precise;
ALTER TABLE media_content.station_radio
    ADD CONSTRAINT ck_station_radio_role_autre_precise
        CHECK (role_partie_prenante IS DISTINCT FROM 'autre'
               OR (role_partie_prenante_autre IS NOT NULL
                   AND btrim(role_partie_prenante_autre) <> ''));


-- ════════════════════════════════════════════════════════════════════════════
-- 6. État « en attente » et compteur de signalements (FR-032, FR-050)
-- ════════════════════════════════════════════════════════════════════════════
-- 'en_attente' : contenu soumis, ou dont le média a été remplacé — non diffusé
-- tant qu'un administrateur ne l'a pas (re)validé. Les lectures publiques
-- filtrent sur etat = 'publie', donc rien de non validé n'est visible.
--
-- Les CHECK d'origine sont nommés par PostgreSQL d'après la table et la colonne
-- (station_radio_etat_check, …) : on les retire par leur nom généré avant de
-- reposer un CHECK nommé explicitement.

DO $$
DECLARE
    t TEXT;
BEGIN
    FOREACH t IN ARRAY ARRAY['chaine_tv', 'station_radio', 'programme_tele', 'programme_radio']
    LOOP
        EXECUTE format(
            'ALTER TABLE media_content.%I DROP CONSTRAINT IF EXISTS %I',
            t, t || '_etat_check');
        EXECUTE format(
            'ALTER TABLE media_content.%I DROP CONSTRAINT IF EXISTS %I',
            t, 'ck_' || t || '_etat');
        EXECUTE format(
            'ALTER TABLE media_content.%I ADD CONSTRAINT %I CHECK (etat IN
                (''brouillon'', ''en_attente'', ''publie'', ''suspendu'', ''supprime''))',
            t, 'ck_' || t || '_etat');
        EXECUTE format(
            'ALTER TABLE media_content.%I
                ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0',
            t);
    END LOOP;
END $$;


-- ════════════════════════════════════════════════════════════════════════════
-- 7. Référentiel des thèmes phares (FR-030)
-- ════════════════════════════════════════════════════════════════════════════
-- Rangés dans shared.categorie avec contexte = 'media'. Les slugs sont
-- préfixés « media- » : shared.categorie.slug est UNIQUE pour TOUS les
-- contextes, et plusieurs de ces libellés (Développement durable, Politique
-- africaine…) existent déjà ou existeront ailleurs.
-- `ordre` reprend l'ordre de la spécification.

INSERT INTO shared.categorie (nom, slug, contexte, ordre) VALUES
    ('Retour des cerveaux',                                  'media-retour-des-cerveaux',                       'media',  1),
    ('Histoire de l''Afrique',                               'media-histoire-de-l-afrique',                     'media',  2),
    ('Valeurs africaines et développement',                  'media-valeurs-africaines-et-developpement',       'media',  3),
    ('Journal de l''Afrique',                                'media-journal-de-l-afrique',                      'media',  4),
    ('Haro sur les hommes de l''Afrique',                    'media-haro-sur-les-hommes-de-l-afrique',          'media',  5),
    ('L''intellectuel africain et développement',            'media-l-intellectuel-africain-et-developpement',  'media',  6),
    ('Afrique et technologies',                              'media-afrique-et-technologies',                   'media',  7),
    ('Savoirs faire d''Afrique',                             'media-savoirs-faire-d-afrique',                   'media',  8),
    ('Cuisine de chez nous',                                 'media-cuisine-de-chez-nous',                      'media',  9),
    ('Politique africaine',                                  'media-politique-africaine',                       'media', 10),
    ('De la thèse à l''action locale',                       'media-de-la-these-a-l-action-locale',             'media', 11),
    ('La voix du terrain en Afrique',                        'media-la-voix-du-terrain-en-afrique',             'media', 12),
    ('Débats africains',                                     'media-debats-africains',                          'media', 13),
    ('Mystères africains',                                   'media-mysteres-africains',                        'media', 14),
    ('Droit africain',                                       'media-droit-africain',                            'media', 15),
    ('Environnement d''Afrique',                             'media-environnement-d-afrique',                   'media', 16),
    ('Regards de la jeunesse africaine',                     'media-regards-de-la-jeunesse-africaine',          'media', 17),
    ('Femmes d''Afrique',                                    'media-femmes-d-afrique',                          'media', 18),
    ('Gouvernance d''Afrique aux défis',                     'media-gouvernance-d-afrique-aux-defis',           'media', 19),
    ('Infrastructures d''Afrique',                           'media-infrastructures-d-afrique',                 'media', 20),
    ('Santé et développement',                               'media-sante-et-developpement',                    'media', 21),
    ('Numérique et développement africain',                  'media-numerique-et-developpement-africain',       'media', 22),
    ('Traditions d''Afrique',                                'media-traditions-d-afrique',                      'media', 23),
    ('Mondialisation et coopération africaine',              'media-mondialisation-et-cooperation-africaine',   'media', 24),
    ('Commerce africain et unité africaine',                 'media-commerce-africain-et-unite-africaine',      'media', 25),
    ('Développement durable',                                'media-developpement-durable',                     'media', 26),
    ('Le monde de demain et mondialisation',                 'media-le-monde-de-demain-et-mondialisation',      'media', 27),
    ('Immigration et l''avenir de l''Afrique',               'media-immigration-et-l-avenir-de-l-afrique',      'media', 28),
    ('Sports d''Afrique',                                    'media-sports-d-afrique',                          'media', 29),
    ('Rendez-vous des hauts et des bas',                     'media-rendez-vous-des-hauts-et-des-bas',          'media', 30),
    ('Éducation — Les carrés de l''instruction en Afrique',  'media-education-carres-instruction-afrique',      'media', 31),
    ('Éducation — Les carrés de l''école de la vie',         'media-education-carres-ecole-de-la-vie',          'media', 32),
    ('Éducation — Les carrés de l''éducation à l''africaine','media-education-carres-education-a-l-africaine',  'media', 33),
    ('L''Afrique que nous voulons',                          'media-l-afrique-que-nous-voulons',                'media', 34),
    ('Messages aux gouvernants',                             'media-messages-aux-gouvernants',                  'media', 35),
    ('Cinéma africain',                                      'media-cinema-africain',                           'media', 36),
    ('Séries d''Afrique',                                    'media-series-d-afrique',                          'media', 37),
    ('Documentaires africains',                              'media-documentaires-africains',                   'media', 38),
    ('Safari d''Afrique',                                    'media-safari-d-afrique',                          'media', 39),
    ('Futurs génies d''Afrique',                             'media-futurs-genies-d-afrique',                   'media', 40),
    ('Innovations simples chez nous',                        'media-innovations-simples-chez-nous',             'media', 41),
    ('Complexes d''Afrique',                                 'media-complexes-d-afrique',                       'media', 42),
    ('Afrique Société',                                      'media-afrique-societe',                           'media', 43),
    ('Afrique Solidarité',                                   'media-afrique-solidarite',                        'media', 44)
ON CONFLICT (slug) DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
-- 8. Permissions 'media' (dette bloquante)
-- ════════════════════════════════════════════════════════════════════════════
-- Le seed (15_seed.sql) ne déclare aucune permission sur la ressource 'media' :
-- sans elles, verifier_permission!(admin, "media", …) ne passe que pour
-- super_admin, porteur du wildcard all.all. La modération des médias serait
-- donc inaccessible aux administrateurs ordinaires.
-- Distinct de la ressource 'media_content' utilisée par vidafrica.

INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    ('Voir les médias radio/télé',      'media.voir',      'media', 'voir'),
    ('Modifier les médias radio/télé',  'media.modifier',  'media', 'modifier'),
    ('Supprimer les médias radio/télé', 'media.supprimer', 'media', 'supprimer')
ON CONFLICT (slug) DO NOTHING;

INSERT INTO iam.role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM iam.role r, iam.permission p
 WHERE r.slug = 'admin'
   AND p.slug IN ('media.voir', 'media.modifier', 'media.supprimer')
ON CONFLICT DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
-- 9. Élargissement de notifications.type (dette bloquante)
-- ════════════════════════════════════════════════════════════════════════════
-- VARCHAR(30) était déjà dépassé par des types existants ; les types de cette
-- feature (proposition_media_validee, media_codetenteur_ajoute…) débordent
-- franchement. Élargissement sans perte, rejouable sans effet.

ALTER TABLE arbre_genealogique.notifications
    ALTER COLUMN type TYPE VARCHAR(80);
