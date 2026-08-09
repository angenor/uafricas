-- ============================================================================
-- 09q — media_content : programmes conteneurs (émissions) et épisodes
-- ----------------------------------------------------------------------------
-- Le modèle actuel confond l'émission et le fichier : `programme_tele` porte une
-- `video_url`, `programme_radio` une `audio_url`. Cette migration introduit le
-- niveau manquant :
--
--   • emission_tele / emission_radio — le CONTENEUR (« Programme » à l'écran),
--     rattaché à une chaîne ou à une station, porteur de la cadence ;
--   • episode_tele  / episode_radio  — l'UNITÉ DIFFUSABLE, porteuse du média,
--     de son ordre dans le programme et de son état de modération.
--
-- ── Pourquoi des noms neufs plutôt qu'un RENAME ─────────────────────────────
-- Le commanditaire appelle « programme » le GROUPE — l'inverse exact du sens que
-- porte aujourd'hui `programme_tele`. Le projet interroge PostgreSQL par
-- `sqlx::query_as` (requêtes RUNTIME, pas de `query!` vérifiée à la
-- compilation) : réutiliser l'identifiant `programme_tele` pour le conteneur
-- ferait compiler ET s'exécuter sans erreur toute requête non portée, contre une
-- table homonyme de sens opposé, avec des résultats silencieusement faux. Les
-- deux tables `programme_*` sont donc SUPPRIMÉES : chaque référence oubliée
-- échoue bruyamment (`relation … does not exist`) au premier appel.
--
-- ── Identifiants et slugs conservés ─────────────────────────────────────────
-- `episode_*.id` et `episode_*.slug` reprennent À L'IDENTIQUE ceux de la ligne
-- `programme_*` d'origine. Les quatre tables d'interactions (09k) sont
-- polymorphes par (type_media, media_id) et ne portent AUCUNE FK sur `media_id` :
-- la reprise se réduit donc à un UPDATE du discriminant, sans jamais toucher
-- `media_id`. Les adresses publiques `/medias/programmes-{tele,radio}/<slug>`
-- restent valides et deviennent les pages d'ÉPISODE — aucune table de
-- redirection n'est nécessaire.
--
-- ── Décision de reprise : contenus sans support de rattachement ─────────────
-- Relevé du 2026-08-08 sur la base cible : 0 `programme_tele` sans `chaine_id`,
-- 0 `programme_radio` sans `station_id`. Aucun traitement particulier n'est donc
-- requis. La migration ne fabrique PAS de chaîne « Sans chaîne » de repli :
-- elle REFUSE de s'exécuter (RAISE EXCEPTION, §3) si un tel contenu apparaît.
-- Inventer un support fantôme polluerait le référentiel public et ferait passer
-- une anomalie de données pour un succès ; l'échec explicite laisse l'opérateur
-- rattacher ou écarter le contenu en connaissance de cause.
--
-- ── Ordre imposé des opérations ─────────────────────────────────────────────
-- Les CHECK `type_media` des quatre tables d'interactions sont RELÂCHÉS en §2,
-- puis reposés à six valeurs en §4 — et non élargis d'un coup : au moment du
-- relâchement les lignes portent encore `programme_tele`, valeur absente de la
-- liste cible. Poser la contrainte finale avant l'UPDATE du discriminant (§3)
-- échouerait.
--
-- Prérequis : 09g (programme_tele / programme_radio), 09j (etat 'en_attente',
-- nombre_signalements, theme_phare, a_la_une_globale), 09k (interactions),
-- 09l (type_objet_propose), 09m (type_support_media), 09n (creneau_programmation).
--
-- Migration IDEMPOTENTE : CREATE … IF NOT EXISTS, DROP CONSTRAINT IF EXISTS puis
-- ADD, blocs DO gardés sur `to_regclass`. La rejouer ne produit aucune erreur —
-- la reprise (§3) se saute d'elle-même une fois les tables sources disparues.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 0. Valeurs d'enum — EN TÊTE DE FICHIER
-- ════════════════════════════════════════════════════════════════════════════
-- `ALTER TYPE … ADD VALUE` ne peut pas cohabiter, dans un même bloc
-- transactionnel, avec une instruction employant la valeur ajoutée. Ces quatre
-- instructions précèdent donc tout le reste ; aucun INSERT de cette migration ne
-- les utilise.
--
-- Les anciennes valeurs 'programme_tele' / 'programme_radio' RESTENT dans l'enum
-- (PostgreSQL ne sait pas retirer une valeur) mais ne sont plus produites : les
-- propositions historiques les conservent.

ALTER TYPE media_content.type_objet_propose ADD VALUE IF NOT EXISTS 'emission_tele';
ALTER TYPE media_content.type_objet_propose ADD VALUE IF NOT EXISTS 'emission_radio';
ALTER TYPE media_content.type_objet_propose ADD VALUE IF NOT EXISTS 'episode_tele';
ALTER TYPE media_content.type_objet_propose ADD VALUE IF NOT EXISTS 'episode_radio';


-- ════════════════════════════════════════════════════════════════════════════
-- 1. Tables conteneurs et unités diffusables
-- ════════════════════════════════════════════════════════════════════════════

-- ── 1.1 emission_tele — conteneur télé ──────────────────────────────────────
-- `chaine_id` est NOT NULL : un programme appartient à une chaîne et à une seule
-- (FR-002). `cadence` porte la périodicité déclarée, socle des alertes de FR-024
-- et de la rotation de FR-016.

CREATE TABLE IF NOT EXISTS media_content.emission_tele (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chaine_id            UUID         NOT NULL
                         REFERENCES media_content.chaine_tv(id) ON DELETE CASCADE,
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL DEFAULT '',
    image_couverture_url VARCHAR(500),
    info_animateur       TEXT,
    info_producteur      TEXT,
    langue               VARCHAR(80)  NOT NULL DEFAULT 'Français',
    theme_phare_id       UUID,                       -- [xref] shared.categorie (contexte='media')
    theme_phare_autre    VARCHAR(200),
    cadence              VARCHAR(20)  NOT NULL DEFAULT 'ponctuelle',
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'brouillon',
    nombre_signalements  INT          NOT NULL DEFAULT 0,
    cree_par             UUID         NOT NULL
                         REFERENCES iam.utilisateur(id) ON DELETE RESTRICT,
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

-- ── 1.2 emission_radio — conteneur radio ────────────────────────────────────
-- Identique, `chaine_id` devenant `station_id`, plus `categorie_radio` reprise
-- de `programme_radio`.

CREATE TABLE IF NOT EXISTS media_content.emission_radio (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    station_id           UUID         NOT NULL
                         REFERENCES media_content.station_radio(id) ON DELETE CASCADE,
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL DEFAULT '',
    image_couverture_url VARCHAR(500),
    info_animateur       TEXT,
    info_producteur      TEXT,
    langue               VARCHAR(80)  NOT NULL DEFAULT 'Français',
    categorie_radio      media_content.categorie_radio,
    theme_phare_id       UUID,                       -- [xref] shared.categorie (contexte='media')
    theme_phare_autre    VARCHAR(200),
    cadence              VARCHAR(20)  NOT NULL DEFAULT 'ponctuelle',
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'brouillon',
    nombre_signalements  INT          NOT NULL DEFAULT 0,
    cree_par             UUID         NOT NULL
                         REFERENCES iam.utilisateur(id) ON DELETE RESTRICT,
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

-- ── 1.3 episode_tele — unité diffusable télé ────────────────────────────────
-- `emission_id` en ON DELETE RESTRICT : c'est FR-010 exprimé EN SQL — une
-- émission ne peut pas disparaître sous ses épisodes. La suppression douce de
-- l'émission (`deleted_at`) reste possible.
--
-- `etat` par défaut 'en_attente' (et non 'brouillon') : tout épisode versé par un
-- co-détenteur naît en file de modération (FR-040). L'administration force
-- explicitement 'publie' à la création — elle EST l'autorité de validation.

CREATE TABLE IF NOT EXISTS media_content.episode_tele (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    emission_id          UUID         NOT NULL
                         REFERENCES media_content.emission_tele(id) ON DELETE RESTRICT,
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL DEFAULT '',
    image_couverture_url VARCHAR(500),
    video_url            VARCHAR(500),               -- fichier (/uploads/…) ou lien
    numero_episode       INT,
    ordre                INT          NOT NULL DEFAULT 0,
    duree_minutes        INT,
    a_la_une             BOOLEAN      NOT NULL DEFAULT FALSE,
    a_la_une_globale     BOOLEAN      NOT NULL DEFAULT FALSE,
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'en_attente',
    motif_rejet          TEXT,
    valide_par           UUID,                       -- [xref] iam.utilisateur
    valide_at            TIMESTAMPTZ,
    nombre_signalements  INT          NOT NULL DEFAULT 0,
    cree_par             UUID         NOT NULL
                         REFERENCES iam.utilisateur(id) ON DELETE RESTRICT,
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

-- ── 1.4 episode_radio — unité diffusable radio ──────────────────────────────
-- `video_url` devient `audio_url` ; pas de `a_la_une_globale`, la vedette plein
-- écran n'existant que sur l'espace Télé.

CREATE TABLE IF NOT EXISTS media_content.episode_radio (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    emission_id          UUID         NOT NULL
                         REFERENCES media_content.emission_radio(id) ON DELETE RESTRICT,
    titre                VARCHAR(350) NOT NULL,
    slug                 VARCHAR(400) UNIQUE,
    description          TEXT         NOT NULL DEFAULT '',
    image_couverture_url VARCHAR(500),
    audio_url            VARCHAR(500),               -- fichier (/uploads/…) ou lien
    numero_episode       INT,
    ordre                INT          NOT NULL DEFAULT 0,
    duree_minutes        INT,
    a_la_une             BOOLEAN      NOT NULL DEFAULT FALSE,
    etat                 VARCHAR(50)  NOT NULL DEFAULT 'en_attente',
    motif_rejet          TEXT,
    valide_par           UUID,                       -- [xref] iam.utilisateur
    valide_at            TIMESTAMPTZ,
    nombre_signalements  INT          NOT NULL DEFAULT 0,
    cree_par             UUID         NOT NULL
                         REFERENCES iam.utilisateur(id) ON DELETE RESTRICT,
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);


-- ── 1.5 Contraintes des émissions ───────────────────────────────────────────

ALTER TABLE media_content.emission_tele  DROP CONSTRAINT IF EXISTS ck_emission_tele_cadence;
ALTER TABLE media_content.emission_tele
    ADD CONSTRAINT ck_emission_tele_cadence
        CHECK (cadence IN ('quotidienne', 'hebdomadaire', 'ponctuelle'));

ALTER TABLE media_content.emission_radio DROP CONSTRAINT IF EXISTS ck_emission_radio_cadence;
ALTER TABLE media_content.emission_radio
    ADD CONSTRAINT ck_emission_radio_cadence
        CHECK (cadence IN ('quotidienne', 'hebdomadaire', 'ponctuelle'));

ALTER TABLE media_content.emission_tele  DROP CONSTRAINT IF EXISTS ck_emission_tele_etat;
ALTER TABLE media_content.emission_tele
    ADD CONSTRAINT ck_emission_tele_etat
        CHECK (etat IN ('brouillon', 'en_attente', 'publie', 'suspendu', 'supprime'));

ALTER TABLE media_content.emission_radio DROP CONSTRAINT IF EXISTS ck_emission_radio_etat;
ALTER TABLE media_content.emission_radio
    ADD CONSTRAINT ck_emission_radio_etat
        CHECK (etat IN ('brouillon', 'en_attente', 'publie', 'suspendu', 'supprime'));

ALTER TABLE media_content.emission_tele  DROP CONSTRAINT IF EXISTS ck_emission_tele_theme_autre;
ALTER TABLE media_content.emission_tele
    ADD CONSTRAINT ck_emission_tele_theme_autre
        CHECK (theme_phare_autre IS NULL OR btrim(theme_phare_autre) <> '');

ALTER TABLE media_content.emission_radio DROP CONSTRAINT IF EXISTS ck_emission_radio_theme_autre;
ALTER TABLE media_content.emission_radio
    ADD CONSTRAINT ck_emission_radio_theme_autre
        CHECK (theme_phare_autre IS NULL OR btrim(theme_phare_autre) <> '');


-- ── 1.6 Contraintes des épisodes ────────────────────────────────────────────
-- Trois invariants exprimés EN SQL, à l'image des cinq CHECK de
-- `transaction_cadeau` (35g) et des quatre de `proposition_media` (09l) :
-- un épisode publié porte son média, un rejet est toujours motivé, une décision
-- porte son décideur et sa date.

ALTER TABLE media_content.episode_tele  DROP CONSTRAINT IF EXISTS ck_episode_tele_etat;
ALTER TABLE media_content.episode_tele
    ADD CONSTRAINT ck_episode_tele_etat
        CHECK (etat IN ('brouillon', 'en_attente', 'publie', 'rejete', 'suspendu', 'supprime'));

ALTER TABLE media_content.episode_radio DROP CONSTRAINT IF EXISTS ck_episode_radio_etat;
ALTER TABLE media_content.episode_radio
    ADD CONSTRAINT ck_episode_radio_etat
        CHECK (etat IN ('brouillon', 'en_attente', 'publie', 'rejete', 'suspendu', 'supprime'));

-- FR-008 — un épisode publié porte forcément son média.
ALTER TABLE media_content.episode_tele  DROP CONSTRAINT IF EXISTS ck_episode_tele_media_publie;
ALTER TABLE media_content.episode_tele
    ADD CONSTRAINT ck_episode_tele_media_publie
        CHECK (etat <> 'publie' OR video_url IS NOT NULL);

ALTER TABLE media_content.episode_radio DROP CONSTRAINT IF EXISTS ck_episode_radio_media_publie;
ALTER TABLE media_content.episode_radio
    ADD CONSTRAINT ck_episode_radio_media_publie
        CHECK (etat <> 'publie' OR audio_url IS NOT NULL);

-- FR-041 — un rejet est toujours motivé, même exigence que
-- ck_prop_media_rejet_commente (09l). Sans ce CHECK, l'auteur d'un épisode
-- refusé n'aurait aucun moyen de savoir quoi corriger.
ALTER TABLE media_content.episode_tele  DROP CONSTRAINT IF EXISTS ck_episode_tele_rejet_motive;
ALTER TABLE media_content.episode_tele
    ADD CONSTRAINT ck_episode_tele_rejet_motive
        CHECK (etat <> 'rejete' OR (motif_rejet IS NOT NULL AND btrim(motif_rejet) <> ''));

ALTER TABLE media_content.episode_radio DROP CONSTRAINT IF EXISTS ck_episode_radio_rejet_motive;
ALTER TABLE media_content.episode_radio
    ADD CONSTRAINT ck_episode_radio_rejet_motive
        CHECK (etat <> 'rejete' OR (motif_rejet IS NOT NULL AND btrim(motif_rejet) <> ''));

-- Une décision (publication ou rejet) porte son décideur et sa date.
ALTER TABLE media_content.episode_tele  DROP CONSTRAINT IF EXISTS ck_episode_tele_decision_coherente;
ALTER TABLE media_content.episode_tele
    ADD CONSTRAINT ck_episode_tele_decision_coherente
        CHECK (etat NOT IN ('publie', 'rejete')
               OR (valide_par IS NOT NULL AND valide_at IS NOT NULL));

ALTER TABLE media_content.episode_radio DROP CONSTRAINT IF EXISTS ck_episode_radio_decision_coherente;
ALTER TABLE media_content.episode_radio
    ADD CONSTRAINT ck_episode_radio_decision_coherente
        CHECK (etat NOT IN ('publie', 'rejete')
               OR (valide_par IS NOT NULL AND valide_at IS NOT NULL));

ALTER TABLE media_content.episode_tele  DROP CONSTRAINT IF EXISTS ck_episode_tele_duree;
ALTER TABLE media_content.episode_tele
    ADD CONSTRAINT ck_episode_tele_duree CHECK (duree_minutes IS NULL OR duree_minutes > 0);

ALTER TABLE media_content.episode_radio DROP CONSTRAINT IF EXISTS ck_episode_radio_duree;
ALTER TABLE media_content.episode_radio
    ADD CONSTRAINT ck_episode_radio_duree CHECK (duree_minutes IS NULL OR duree_minutes > 0);


-- ── 1.7 Index ───────────────────────────────────────────────────────────────

CREATE INDEX IF NOT EXISTS idx_emission_tele_chaine
    ON media_content.emission_tele (chaine_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_emission_tele_etat
    ON media_content.emission_tele (etat) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_emission_tele_theme_phare
    ON media_content.emission_tele (theme_phare_id) WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_emission_radio_station
    ON media_content.emission_radio (station_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_emission_radio_etat
    ON media_content.emission_radio (etat) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_emission_radio_theme_phare
    ON media_content.emission_radio (theme_phare_id) WHERE deleted_at IS NULL;

-- Requête chaude : les épisodes PUBLIÉS d'une émission, dans l'ordre de
-- rotation. C'est l'index que suit la JOIN LATERAL de résolution de diffusion.
CREATE INDEX IF NOT EXISTS idx_episode_tele_emission_ordre
    ON media_content.episode_tele (emission_id, ordre, created_at)
    WHERE etat = 'publie' AND deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_episode_radio_emission_ordre
    ON media_content.episode_radio (emission_id, ordre, created_at)
    WHERE etat = 'publie' AND deleted_at IS NULL;

-- File de modération, triée par ancienneté (FR-043).
CREATE INDEX IF NOT EXISTS idx_episode_tele_en_attente
    ON media_content.episode_tele (created_at)
    WHERE etat = 'en_attente' AND deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_episode_radio_en_attente
    ON media_content.episode_radio (created_at)
    WHERE etat = 'en_attente' AND deleted_at IS NULL;

-- FR-052 — une seule mise en avant par émission, une seule vedette pour tout
-- l'espace Télé. L'index sur l'expression constante ((TRUE)) est la transposition
-- exacte de uq_programme_tele_a_la_une_globale (09j §3). Le périmètre « une par
-- support » est tenu par le handler, qui bascule l'ancienne et désigne la
-- nouvelle DANS UNE MÊME TRANSACTION (research.md R9).
CREATE UNIQUE INDEX IF NOT EXISTS uq_episode_tele_a_la_une_par_emission
    ON media_content.episode_tele (emission_id)
    WHERE a_la_une = TRUE AND deleted_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_episode_radio_a_la_une_par_emission
    ON media_content.episode_radio (emission_id)
    WHERE a_la_une = TRUE AND deleted_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_episode_tele_a_la_une_globale
    ON media_content.episode_tele ((TRUE))
    WHERE a_la_une_globale = TRUE AND deleted_at IS NULL;


COMMENT ON TABLE media_content.emission_tele IS
    'Programme conteneur d''une chaîne TV. « Programme » à l''écran, « emission » en base
     — le mot « programme » désignait déjà l''unité diffusable avant 09q.';
COMMENT ON TABLE media_content.episode_tele IS
    'Unité diffusable d''un programme télé. id et slug REPRIS de programme_tele : les
     interactions (09k) et les adresses publiques suivent sans réécriture.';
COMMENT ON COLUMN media_content.episode_tele.ordre IS
    'Rang dans le programme. Tri effectif (ordre, created_at, id) — pas d''unicité :
     le tri secondaire rend l''ordre total et stable même en cas d''ex æquo.';
COMMENT ON COLUMN media_content.emission_tele.cadence IS
    'Périodicité déclarée. Alimente les alertes de FR-024 ; la rotation, elle, se
     déduit de la récurrence du créneau, pas de cette colonne.';


-- ════════════════════════════════════════════════════════════════════════════
-- 2. Modifications structurelles préalables à la reprise
-- ════════════════════════════════════════════════════════════════════════════

-- ── 2.1 creneau_programmation : le créneau cible désormais une ÉMISSION ─────
-- `date_effet` est l'ORIGINE DU COMPTAGE des occurrences (research.md R4). Elle
-- est un DATE et non un TIMESTAMPTZ — écart assumé au principe « TIMESTAMPTZ
-- partout », de même nature que celui déjà documenté par 09n : le rang
-- d'occurrence doit se calculer dans le MÊME référentiel local que `heure_debut`
-- et `jour_semaine`. Un TIMESTAMPTZ réintroduirait une conversion de fuseau au
-- moment de la soustraction de dates et décalerait le rang d'un cran selon
-- l'instant de lecture — exactement ce que FR-017 interdit.
--
-- `emission_id` reste sans FK : la cible est polymorphe selon `type_support`,
-- comme l'était `contenu_id`.

ALTER TABLE media_content.creneau_programmation
    ADD COLUMN IF NOT EXISTS emission_id UUID;
ALTER TABLE media_content.creneau_programmation
    ADD COLUMN IF NOT EXISTS date_effet  DATE NOT NULL DEFAULT CURRENT_DATE;

COMMENT ON COLUMN media_content.creneau_programmation.date_effet IS
    'Origine du comptage des occurrences, interprétée dans le fuseau du créneau.
     La déplacer redéfinit la rotation — c''est le seul levier de FR-016.';

-- ── 2.2 Interactions : relâchement temporaire des CHECK de discriminant ─────
-- Reposés à six valeurs en §4, une fois le discriminant repris. Les poser
-- maintenant échouerait : les lignes portent encore 'programme_tele'.

ALTER TABLE media_content.media_reaction     DROP CONSTRAINT IF EXISTS ck_media_reaction_type_media;
ALTER TABLE media_content.media_commentaire  DROP CONSTRAINT IF EXISTS ck_media_commentaire_type_media;
ALTER TABLE media_content.partage_media      DROP CONSTRAINT IF EXISTS ck_partage_media_type_media;
ALTER TABLE media_content.signalement_media  DROP CONSTRAINT IF EXISTS ck_signalement_media_type_media;


-- ════════════════════════════════════════════════════════════════════════════
-- 3. Reprise de données
-- ════════════════════════════════════════════════════════════════════════════
-- Bloc gardé : si `programme_tele` n'existe plus, la reprise a déjà eu lieu et
-- se saute silencieusement. C'est ce qui rend la migration rejouable.
--
-- Les statements sont exécutés par EXECUTE : sur une base déjà migrée, les tables
-- sources n'existent plus et un statement littéral serait rejeté à l'analyse.

DO $mig$
DECLARE
    v_orphelins  BIGINT;
    v_creneaux   BIGINT;
BEGIN
    IF to_regclass('media_content.programme_tele') IS NULL THEN
        RAISE NOTICE '09q — reprise déjà effectuée (programme_tele absente), étape sautée.';
        RETURN;
    END IF;

    -- ── 3.0 Garde : aucun contenu sans support de rattachement ──────────────
    -- Voir la décision de reprise en tête de fichier : on refuse plutôt que
    -- d'inventer une chaîne fantôme.
    EXECUTE $sql$
        SELECT (SELECT count(*) FROM media_content.programme_tele  WHERE chaine_id  IS NULL)
             + (SELECT count(*) FROM media_content.programme_radio WHERE station_id IS NULL)
    $sql$ INTO v_orphelins;

    IF v_orphelins > 0 THEN
        RAISE EXCEPTION
            '09q — % contenu(s) sans chaîne ni station. Les rattacher ou les supprimer avant de rejouer : la migration ne fabrique pas de support de repli.',
            v_orphelins;
    END IF;

    -- ── 3.1 Correspondance contenu → émission ───────────────────────────────
    -- Table temporaire plutôt que jointure par slug : `programme_*.slug` est
    -- NULLABLE, et une jointure sur NULL perdrait silencieusement la ligne.
    CREATE TEMP TABLE _reprise_09q_tele ON COMMIT DROP AS
        SELECT id AS contenu_id, gen_random_uuid() AS emission_id
          FROM media_content.programme_tele;
    CREATE TEMP TABLE _reprise_09q_radio ON COMMIT DROP AS
        SELECT id AS contenu_id, gen_random_uuid() AS emission_id
          FROM media_content.programme_radio;

    -- ── 3.2 Une émission par contenu existant (FR-055) ──────────────────────
    -- Identifiant neuf, métadonnées éditoriales reprises, cadence 'ponctuelle'
    -- (aucune périodicité n'était déclarée), état repris tel quel. Le suffixe
    -- « -programme » évite la collision avec le slug de l'épisode, CONSERVÉ à
    -- l'identique. Le repli sur l'identifiant couvre les contenus sans slug.
    EXECUTE $sql$
        INSERT INTO media_content.emission_tele
            (id, chaine_id, titre, slug, description, image_couverture_url,
             info_animateur, info_producteur, langue, theme_phare_id, theme_phare_autre,
             cadence, etat, nombre_signalements, cree_par, created_at, updated_at, deleted_at)
        SELECT m.emission_id, p.chaine_id, p.nom_emission,
               COALESCE(p.slug, p.id::text) || '-programme',
               p.description, p.image_couverture_url, p.info_animateur, p.info_producteur,
               p.langue, p.theme_phare_id, p.theme_phare_autre,
               'ponctuelle', p.etat, p.nombre_signalements, p.cree_par,
               p.created_at, p.updated_at, p.deleted_at
          FROM media_content.programme_tele p
          JOIN _reprise_09q_tele m ON m.contenu_id = p.id
    $sql$;

    EXECUTE $sql$
        INSERT INTO media_content.emission_radio
            (id, station_id, titre, slug, description, image_couverture_url,
             info_animateur, info_producteur, langue, categorie_radio,
             theme_phare_id, theme_phare_autre,
             cadence, etat, nombre_signalements, cree_par, created_at, updated_at, deleted_at)
        SELECT m.emission_id, p.station_id, p.nom_emission,
               COALESCE(p.slug, p.id::text) || '-programme',
               p.description, p.image_couverture_url, p.info_animateur, p.info_producteur,
               p.langue, p.categorie_radio, p.theme_phare_id, p.theme_phare_autre,
               'ponctuelle', p.etat, p.nombre_signalements, p.cree_par,
               p.created_at, p.updated_at, p.deleted_at
          FROM media_content.programme_radio p
          JOIN _reprise_09q_radio m ON m.contenu_id = p.id
    $sql$;

    -- ── 3.3 Un épisode par contenu, IDENTIFIANT ET SLUG CONSERVÉS ───────────
    -- (FR-051, FR-056). `ordre = 0` : un seul épisode par émission à ce stade.
    -- Les contenus déjà publiés sont réputés validés — hypothèse de la spec —
    -- d'où `valide_par` / `valide_at` renseignés, sans quoi
    -- ck_episode_*_decision_coherente refuserait la ligne.
    EXECUTE $sql$
        INSERT INTO media_content.episode_tele
            (id, emission_id, titre, slug, description, image_couverture_url, video_url,
             ordre, a_la_une, a_la_une_globale, etat, valide_par, valide_at,
             nombre_signalements, cree_par, created_at, updated_at, deleted_at)
        SELECT p.id, m.emission_id, p.nom_emission, p.slug, p.description,
               p.image_couverture_url, p.video_url,
               0, p.a_la_une, p.a_la_une_globale, p.etat,
               CASE WHEN p.etat = 'publie' THEN p.cree_par   END,
               CASE WHEN p.etat = 'publie' THEN p.updated_at END,
               p.nombre_signalements, p.cree_par, p.created_at, p.updated_at, p.deleted_at
          FROM media_content.programme_tele p
          JOIN _reprise_09q_tele m ON m.contenu_id = p.id
    $sql$;

    EXECUTE $sql$
        INSERT INTO media_content.episode_radio
            (id, emission_id, titre, slug, description, image_couverture_url, audio_url,
             ordre, a_la_une, etat, valide_par, valide_at,
             nombre_signalements, cree_par, created_at, updated_at, deleted_at)
        SELECT p.id, m.emission_id, p.nom_emission, p.slug, p.description,
               p.image_couverture_url, p.audio_url,
               0, p.a_la_une, p.etat,
               CASE WHEN p.etat = 'publie' THEN p.cree_par   END,
               CASE WHEN p.etat = 'publie' THEN p.updated_at END,
               p.nombre_signalements, p.cree_par, p.created_at, p.updated_at, p.deleted_at
          FROM media_content.programme_radio p
          JOIN _reprise_09q_radio m ON m.contenu_id = p.id
    $sql$;

    -- ── 3.4 Discriminant des interactions (FR-051) ──────────────────────────
    -- `media_id` n'est JAMAIS touché : l'identifiant de l'épisode est celui du
    -- contenu. Le filtre EXISTS garantit qu'aucune ligne ne bascule vers une
    -- cible qui n'aurait pas été reprise.
    EXECUTE $sql$
        UPDATE media_content.media_reaction r SET type_media = 'episode_tele'
         WHERE r.type_media = 'programme_tele'
           AND EXISTS (SELECT 1 FROM media_content.episode_tele e WHERE e.id = r.media_id)
    $sql$;
    EXECUTE $sql$
        UPDATE media_content.media_reaction r SET type_media = 'episode_radio'
         WHERE r.type_media = 'programme_radio'
           AND EXISTS (SELECT 1 FROM media_content.episode_radio e WHERE e.id = r.media_id)
    $sql$;
    EXECUTE $sql$
        UPDATE media_content.media_commentaire r SET type_media = 'episode_tele'
         WHERE r.type_media = 'programme_tele'
           AND EXISTS (SELECT 1 FROM media_content.episode_tele e WHERE e.id = r.media_id)
    $sql$;
    EXECUTE $sql$
        UPDATE media_content.media_commentaire r SET type_media = 'episode_radio'
         WHERE r.type_media = 'programme_radio'
           AND EXISTS (SELECT 1 FROM media_content.episode_radio e WHERE e.id = r.media_id)
    $sql$;
    EXECUTE $sql$
        UPDATE media_content.partage_media r SET type_media = 'episode_tele'
         WHERE r.type_media = 'programme_tele'
           AND EXISTS (SELECT 1 FROM media_content.episode_tele e WHERE e.id = r.media_id)
    $sql$;
    EXECUTE $sql$
        UPDATE media_content.partage_media r SET type_media = 'episode_radio'
         WHERE r.type_media = 'programme_radio'
           AND EXISTS (SELECT 1 FROM media_content.episode_radio e WHERE e.id = r.media_id)
    $sql$;
    EXECUTE $sql$
        UPDATE media_content.signalement_media r SET type_media = 'episode_tele'
         WHERE r.type_media = 'programme_tele'
           AND EXISTS (SELECT 1 FROM media_content.episode_tele e WHERE e.id = r.media_id)
    $sql$;
    EXECUTE $sql$
        UPDATE media_content.signalement_media r SET type_media = 'episode_radio'
         WHERE r.type_media = 'programme_radio'
           AND EXISTS (SELECT 1 FROM media_content.episode_radio e WHERE e.id = r.media_id)
    $sql$;

    -- Garde : plus aucune interaction ne doit désigner un contenu disparu.
    -- Le CHECK à six valeurs de §4 refuserait de toute façon ces lignes ; échouer
    -- ici donne un message lisible plutôt qu'une violation de contrainte.
    EXECUTE $sql$
        SELECT (SELECT count(*) FROM media_content.media_reaction    WHERE type_media IN ('programme_tele','programme_radio'))
             + (SELECT count(*) FROM media_content.media_commentaire WHERE type_media IN ('programme_tele','programme_radio'))
             + (SELECT count(*) FROM media_content.partage_media     WHERE type_media IN ('programme_tele','programme_radio'))
             + (SELECT count(*) FROM media_content.signalement_media WHERE type_media IN ('programme_tele','programme_radio'))
    $sql$ INTO v_orphelins;

    IF v_orphelins > 0 THEN
        RAISE EXCEPTION
            '09q — % interaction(s) désignent un contenu absent des tables reprises. Reprise incomplète, migration interrompue.',
            v_orphelins;
    END IF;

    -- ── 3.5 Créneaux : rattachement à l'émission née du contenu (FR-058) ────
    -- `date_effet` vaut la date de reprise, valeur par défaut de la colonne.
    EXECUTE $sql$
        UPDATE media_content.creneau_programmation c
           SET emission_id = ep.emission_id
          FROM media_content.episode_tele ep
         WHERE c.type_support = 'chaine_tv' AND c.contenu_id = ep.id
    $sql$;
    EXECUTE $sql$
        UPDATE media_content.creneau_programmation c
           SET emission_id = ep.emission_id
          FROM media_content.episode_radio ep
         WHERE c.type_support = 'station_radio' AND c.contenu_id = ep.id
    $sql$;

    EXECUTE $sql$
        SELECT count(*) FROM media_content.creneau_programmation
         WHERE emission_id IS NULL AND deleted_at IS NULL
    $sql$ INTO v_creneaux;

    IF v_creneaux > 0 THEN
        RAISE EXCEPTION
            '09q — % créneau(x) sans émission cible (contenu_id orphelin). Les corriger ou les supprimer avant de rejouer.',
            v_creneaux;
    END IF;

    RAISE NOTICE '09q — reprise effectuée.';
END
$mig$;


-- ════════════════════════════════════════════════════════════════════════════
-- 4. Verrouillage du nouveau modèle
-- ════════════════════════════════════════════════════════════════════════════

-- ── 4.1 Discriminant des interactions à six valeurs (research.md R8) ────────
-- Les anciennes valeurs sont volontairement ABSENTES : un client non porté
-- échoue visiblement plutôt que d'écrire sur une cible fantôme. Le VARCHAR(20)
-- suffit — 'emission_radio' fait 14 caractères.

ALTER TABLE media_content.media_reaction
    ADD CONSTRAINT ck_media_reaction_type_media
        CHECK (type_media IN ('chaine_tv', 'station_radio',
                              'emission_tele', 'emission_radio',
                              'episode_tele', 'episode_radio'));

ALTER TABLE media_content.media_commentaire
    ADD CONSTRAINT ck_media_commentaire_type_media
        CHECK (type_media IN ('chaine_tv', 'station_radio',
                              'emission_tele', 'emission_radio',
                              'episode_tele', 'episode_radio'));

ALTER TABLE media_content.partage_media
    ADD CONSTRAINT ck_partage_media_type_media
        CHECK (type_media IN ('chaine_tv', 'station_radio',
                              'emission_tele', 'emission_radio',
                              'episode_tele', 'episode_radio'));

ALTER TABLE media_content.signalement_media
    ADD CONSTRAINT ck_signalement_media_type_media
        CHECK (type_media IN ('chaine_tv', 'station_radio',
                              'emission_tele', 'emission_radio',
                              'episode_tele', 'episode_radio'));

-- ── 4.2 Suppression des tables de l'ancien modèle ───────────────────────────
-- C'est cette suppression qui fait échouer BRUYAMMENT toute requête non portée,
-- au lieu de la laisser lire une table homonyme de sens opposé (research.md R1).

DROP TABLE IF EXISTS media_content.programme_tele  CASCADE;
DROP TABLE IF EXISTS media_content.programme_radio CASCADE;

-- ── 4.3 creneau_programmation : `contenu_id` n'a plus de cible ──────────────

ALTER TABLE media_content.creneau_programmation DROP COLUMN IF EXISTS contenu_id;

DROP INDEX IF EXISTS media_content.idx_creneau_contenu;
CREATE INDEX IF NOT EXISTS idx_creneau_emission
    ON media_content.creneau_programmation (emission_id)
    WHERE actif = TRUE AND deleted_at IS NULL;

-- `emission_id` devient NOT NULL une fois la reprise vérifiée : un créneau sans
-- émission ne désigne rien et resterait invisible de son auteur.
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns
                WHERE table_schema = 'media_content'
                  AND table_name   = 'creneau_programmation'
                  AND column_name  = 'emission_id'
                  AND is_nullable  = 'YES') THEN
        ALTER TABLE media_content.creneau_programmation
            ALTER COLUMN emission_id SET NOT NULL;
    END IF;
END $$;
