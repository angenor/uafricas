-- ============================================================================
-- 37_jeu.sql — Activités interactives, ludiques et participatives
-- ============================================================================
-- Feature : specs/013-activites-ludiques (data-model.md, research.md)
--
-- Dépendances : 03_shared (pays), 04_iam (utilisateur, permission, rôle),
--   08_culture (codimoi), 10_governance (factcheck), 11_country_profile
--   (fiche_pays, site_touristique, recette_culinaire, personnalite_connue),
--   15_seed (rôles), 35c et 35d (catégories, règles et badges d'engagement).
--   D'où l'inclusion en PHASE 5 de schema.sql, après le bloc 35*.
--
-- Nouveau schéma `jeu`, déclaré ici comme `social` (29) et `engagement` (35) :
-- dix tables dont aucune n'appartient à un domaine existant.
--
-- Trois choix qui s'écartent des habitudes du dépôt, tous voulus :
--   • États et cadres en VARCHAR + CHECK, pas en enums (même raison qu'en 36 :
--     un enum se modifie mal, et sqlx les lit en ::text de toute façon).
--   • Pas de `deleted_at` : ces tables sont des journaux ou des états. Une
--     épreuve se RETIRE (etat = 'retiree'), elle ne se supprime pas, sinon les
--     réponses et les gains qui la citent perdraient leur objet.
--   • `updated_at` tenu par le code : 14_triggers.sql ne boucle que sur onze
--     schémas nommés, comme pour 29, 35* et 36.
--
-- Le jeu NE CRÉDITE AUCUN POINT D'ENGAGEMENT. Il a son propre journal
-- (`jeu.gain`). Réputation et distinctions passent par six règles à 0 point
-- ajoutées au barème existant (fin de fichier).
--
-- Migration idempotente et purement additive : un seul temps au déploiement.
-- ============================================================================

CREATE SCHEMA IF NOT EXISTS jeu;


-- ════════════════════════════════════════════════════════════════════════════
-- 1. MODULE — un module de la plateforme ouvert au jeu
-- ════════════════════════════════════════════════════════════════════════════
-- Clé naturelle `code` : elle est citée par les routes et par le catalogue des
-- formes de dérivation côté Rust.

CREATE TABLE IF NOT EXISTS jeu.module (
    code        VARCHAR(30)  PRIMARY KEY
                             CONSTRAINT ck_module_code CHECK (code ~ '^[a-z0-9_]+$'),
    libelle     VARCHAR(80)  NOT NULL,
    route       VARCHAR(200) NOT NULL,          -- page du module, pour le retour
    icone       VARCHAR(40),                    -- nom FontAwesome déjà enregistré
    ouvert      BOOLEAN      NOT NULL DEFAULT TRUE,
    ordre       SMALLINT     NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);


-- ════════════════════════════════════════════════════════════════════════════
-- 2. ÉPREUVE — une question jouable
-- ════════════════════════════════════════════════════════════════════════════
-- « Exactement une bonne réponse » est vrai PAR CONSTRUCTION : il n'y a qu'une
-- colonne pour la dire. Une table fille avec un booléen `correcte` laisserait
-- possible « aucune bonne réponse ».

CREATE TABLE IF NOT EXISTS jeu.epreuve (
    id                UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    module_code       VARCHAR(30)  NOT NULL REFERENCES jeu.module(code),
    enonce            TEXT         NOT NULL,
    media_type        VARCHAR(10),
    media_url         VARCHAR(500),
    propositions      TEXT[]       NOT NULL,
    bonne_reponse     SMALLINT     NOT NULL,     -- rang dans `propositions`, à partir de 1
    explication       TEXT,
    difficulte        SMALLINT     NOT NULL DEFAULT 1,
    theme             VARCHAR(80),
    pays_id           UUID         REFERENCES shared.pays(id) ON DELETE SET NULL,
    origine           VARCHAR(10)  NOT NULL,
    -- Référence polymorphe vers un contenu publié : pas de FK, l'existence et la
    -- publication se lisent dans `jeu.v_source` à chaque tirage.
    type_source       VARCHAR(30),
    source_id         UUID,
    forme             VARCHAR(40),               -- code de la forme de question
    source_empreinte  VARCHAR(32),               -- empreinte de la source à la dérivation
    etat              VARCHAR(12)  NOT NULL DEFAULT 'candidate',
    motif_rejet       TEXT,
    nombre_servie     INT          NOT NULL DEFAULT 0,
    nombre_bonnes     INT          NOT NULL DEFAULT 0,
    cree_par          UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    valide_par        UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    valide_at         TIMESTAMPTZ,
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_epreuve_enonce        CHECK (btrim(enonce) <> ''),
    CONSTRAINT ck_epreuve_media_type    CHECK (media_type IS NULL OR media_type IN ('image', 'audio')),
    CONSTRAINT ck_epreuve_media         CHECK ((media_type IS NULL) = (media_url IS NULL)),
    CONSTRAINT ck_epreuve_propositions  CHECK (cardinality(propositions) BETWEEN 2 AND 6),
    CONSTRAINT ck_epreuve_bonne_reponse CHECK (bonne_reponse BETWEEN 1 AND cardinality(propositions)),
    CONSTRAINT ck_epreuve_difficulte    CHECK (difficulte BETWEEN 1 AND 3),
    CONSTRAINT ck_epreuve_origine       CHECK (origine IN ('saisie', 'derivee')),
    CONSTRAINT ck_epreuve_etat          CHECK (etat IN ('candidate', 'jouable', 'a_revoir', 'rejetee', 'retiree')),
    CONSTRAINT ck_epreuve_source        CHECK ((type_source IS NULL) = (source_id IS NULL)),
    CONSTRAINT ck_epreuve_derivee       CHECK (
        origine <> 'derivee'
        OR (type_source IS NOT NULL AND forme IS NOT NULL AND source_empreinte IS NOT NULL)),
    -- La publication d'une épreuve incomplète est refusée EN SQL ; l'API nomme
    -- le manque avant d'en arriver là.
    CONSTRAINT ck_epreuve_jouable       CHECK (etat <> 'jouable' OR btrim(COALESCE(explication, '')) <> ''),
    CONSTRAINT ck_epreuve_rejet         CHECK (etat <> 'rejetee' OR btrim(COALESCE(motif_rejet, '')) <> '')
);

-- Couvre aussi les REJETÉES : une forme rejetée sur un contenu n'est jamais
-- reproposée par une dérivation ultérieure.
CREATE UNIQUE INDEX IF NOT EXISTS uq_epreuve_derivee
    ON jeu.epreuve (type_source, source_id, forme) WHERE origine = 'derivee';
CREATE INDEX IF NOT EXISTS idx_epreuve_module_etat ON jeu.epreuve (module_code, etat);
CREATE INDEX IF NOT EXISTS idx_epreuve_pays        ON jeu.epreuve (pays_id) WHERE etat = 'jouable';
CREATE INDEX IF NOT EXISTS idx_epreuve_source      ON jeu.epreuve (type_source, source_id);


-- ════════════════════════════════════════════════════════════════════════════
-- 3. SAISON — une période du Championship
-- ════════════════════════════════════════════════════════════════════════════
-- L'état (à venir, en cours, close) se DÉDUIT des dates : rien ne « passe »
-- d'un état à l'autre, donc rien n'a besoin de tourner à l'heure dite.
-- `cloturee_at` ne sert qu'à n'attribuer les distinctions de podium qu'une fois.

CREATE TABLE IF NOT EXISTS jeu.saison (
    id           UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom          VARCHAR(120) NOT NULL,
    debut_at     TIMESTAMPTZ  NOT NULL,
    fin_at       TIMESTAMPTZ  NOT NULL,
    cloturee_at  TIMESTAMPTZ,
    cree_par     UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_saison_nom    CHECK (btrim(nom) <> ''),
    CONSTRAINT ck_saison_dates  CHECK (fin_at > debut_at),
    -- Deux saisons ne se chevauchent pas, EN SQL. Les types d'intervalle ont
    -- leur classe d'opérateurs GiST native : aucune extension à ajouter.
    CONSTRAINT ex_saison_chevauchement
        EXCLUDE USING gist (tstzrange(debut_at, fin_at, '[)') WITH &&)
);


-- ════════════════════════════════════════════════════════════════════════════
-- 4. DÉFI — une série commune à tous, pour un jour ou une semaine
-- ════════════════════════════════════════════════════════════════════════════
-- L'unicité (periodicite, periode_debut) fait deux choses : un seul défi par
-- période, et « le premier lecteur le crée, tous lisent la même ligne »
-- (INSERT … ON CONFLICT DO NOTHING puis SELECT).

CREATE TABLE IF NOT EXISTS jeu.defi (
    id             UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    periodicite    VARCHAR(8)   NOT NULL,
    periode_debut  DATE         NOT NULL,        -- jour UTC, ou lundi UTC de la semaine
    module_code    VARCHAR(30)  REFERENCES jeu.module(code),
    titre          VARCHAR(120),
    epreuve_ids    UUID[]       NOT NULL,        -- série figée, dans l'ordre
    origine        VARCHAR(12)  NOT NULL DEFAULT 'automatique',
    cree_par       UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_defi_periodicite CHECK (periodicite IN ('jour', 'semaine')),
    CONSTRAINT ck_defi_origine     CHECK (origine IN ('programme', 'automatique')),
    CONSTRAINT ck_defi_serie       CHECK (cardinality(epreuve_ids) >= 1),
    CONSTRAINT ck_defi_semaine     CHECK (periodicite <> 'semaine' OR EXTRACT(ISODOW FROM periode_debut) = 1),
    CONSTRAINT uq_defi_periode     UNIQUE (periodicite, periode_debut)
);


-- ════════════════════════════════════════════════════════════════════════════
-- 5. DUEL — l'affrontement de deux membres sur une série commune
-- ════════════════════════════════════════════════════════════════════════════
-- Un seul cycle d'états pour les deux modes. Les colonnes `rang_courant`,
-- `manche_debut_at` et `presence_*` ne servent qu'au mode direct.

CREATE TABLE IF NOT EXISTS jeu.duel (
    id                      UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    proposant_id            UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    adversaire_id           UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    module_code             VARCHAR(30)  NOT NULL REFERENCES jeu.module(code),
    mode                    VARCHAR(8)   NOT NULL,
    etat                    VARCHAR(10)  NOT NULL DEFAULT 'propose',
    compte                  BOOLEAN      NOT NULL DEFAULT TRUE,   -- FALSE = amical, sans gain
    epreuve_ids             UUID[],                               -- figée à l'acceptation
    propose_at              TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    accepte_at              TIMESTAMPTZ,
    echeance_at             TIMESTAMPTZ  NOT NULL,                -- échéance de l'étape en cours
    issue                   VARCHAR(12),
    vainqueur_id            UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    termine_at              TIMESTAMPTZ,
    rang_courant            SMALLINT     NOT NULL DEFAULT 0,
    manche_debut_at         TIMESTAMPTZ,
    presence_proposant_at   TIMESTAMPTZ,
    presence_adversaire_at  TIMESTAMPTZ,

    CONSTRAINT ck_duel_joueurs CHECK (proposant_id <> adversaire_id),
    CONSTRAINT ck_duel_mode    CHECK (mode IN ('differe', 'direct')),
    CONSTRAINT ck_duel_etat    CHECK (etat IN ('propose', 'accepte', 'en_cours', 'termine', 'refuse', 'annule', 'expire')),
    CONSTRAINT ck_duel_issue   CHECK (issue IS NULL OR issue IN ('victoire', 'nul', 'forfait', 'sans_issue'))
);

CREATE INDEX IF NOT EXISTS idx_duel_proposant  ON jeu.duel (proposant_id, etat);
CREATE INDEX IF NOT EXISTS idx_duel_adversaire ON jeu.duel (adversaire_id, etat);


-- ════════════════════════════════════════════════════════════════════════════
-- 6. PARTIE — une série d'épreuves jouée par un membre
-- ════════════════════════════════════════════════════════════════════════════
-- La participation à un défi EST une partie (cadre = 'defi') ; chaque joueur
-- d'un duel a la sienne (cadre = 'duel'). Deux index uniques partiels en font
-- des règles : une participation par défi, une partie par joueur et par duel.

CREATE TABLE IF NOT EXISTS jeu.partie (
    id              UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    module_code     VARCHAR(30)  REFERENCES jeu.module(code),   -- NULL : défi multi-modules
    cadre           VARCHAR(12)  NOT NULL,
    defi_id         UUID         REFERENCES jeu.defi(id) ON DELETE CASCADE,
    duel_id         UUID         REFERENCES jeu.duel(id) ON DELETE CASCADE,
    theme           VARCHAR(80),
    pays_id         UUID         REFERENCES shared.pays(id) ON DELETE SET NULL,
    epreuve_ids     UUID[]       NOT NULL,       -- série figée, dans l'ordre
    rang_courant    SMALLINT     NOT NULL DEFAULT 0,   -- nombre d'épreuves déjà présentées
    presentee_at    TIMESTAMPTZ,                 -- début de l'épreuve en cours
    etat            VARCHAR(10)  NOT NULL DEFAULT 'en_cours',
    bonnes          SMALLINT     NOT NULL DEFAULT 0,
    score_gagne     INT          NOT NULL DEFAULT 0,
    temps_total_ms  INT          NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    terminee_at     TIMESTAMPTZ,

    CONSTRAINT ck_partie_cadre  CHECK (cadre IN ('libre', 'entrainement', 'defi', 'duel')),
    CONSTRAINT ck_partie_etat   CHECK (etat IN ('en_cours', 'terminee', 'close')),
    CONSTRAINT ck_partie_defi   CHECK ((cadre = 'defi') = (defi_id IS NOT NULL)),
    CONSTRAINT ck_partie_duel   CHECK ((cadre = 'duel') = (duel_id IS NOT NULL)),
    CONSTRAINT ck_partie_serie  CHECK (cardinality(epreuve_ids) >= 1)
);

CREATE UNIQUE INDEX IF NOT EXISTS uq_partie_defi
    ON jeu.partie (utilisateur_id, defi_id) WHERE defi_id IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uq_partie_duel
    ON jeu.partie (duel_id, utilisateur_id) WHERE duel_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_partie_utilisateur
    ON jeu.partie (utilisateur_id, created_at DESC);


-- ════════════════════════════════════════════════════════════════════════════
-- 7. RÉPONSE — ce qu'un membre a répondu à une épreuve d'une partie
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS jeu.reponse (
    id                   UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    partie_id            UUID         NOT NULL REFERENCES jeu.partie(id) ON DELETE CASCADE,
    utilisateur_id       UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    epreuve_id           UUID         NOT NULL REFERENCES jeu.epreuve(id),
    cadre                VARCHAR(12)  NOT NULL,   -- recopié de la partie
    rang                 SMALLINT     NOT NULL,   -- position dans la série, à partir de 1
    proposition_choisie  SMALLINT,                -- NULL = sans réponse
    issue                VARCHAR(12)  NOT NULL,
    temps_ms             INT          NOT NULL DEFAULT 0,
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_reponse_issue CHECK (issue IN ('bonne', 'mauvaise', 'sans_reponse', 'injouable')),
    -- Une seule réponse par épreuve d'une partie, quelles que soient les
    -- répétitions d'envoi : c'est le verrou de l'idempotence côté réponse.
    CONSTRAINT uq_reponse_rang  UNIQUE (partie_id, rang)
);

-- En partie libre, une épreuve ne se joue qu'une fois : ceinture en plus du
-- tirage, qui écarte déjà les épreuves vues.
CREATE UNIQUE INDEX IF NOT EXISTS uq_reponse_libre
    ON jeu.reponse (utilisateur_id, epreuve_id) WHERE cadre = 'libre';
-- « Jamais répondu », tous cadres confondus : l'anti-jointure du tirage.
CREATE INDEX IF NOT EXISTS idx_reponse_utilisateur_epreuve
    ON jeu.reponse (utilisateur_id, epreuve_id);


-- ════════════════════════════════════════════════════════════════════════════
-- 8. GAIN — le journal du score de jeu (append-only)
-- ════════════════════════════════════════════════════════════════════════════
-- Fait foi. `saison_id` et `pays_id` sont FIGÉS à l'écriture : un changement de
-- profil ultérieur ne déplace aucun gain.
--
-- Clés d'idempotence :
--   reponse:{reponse_id}          bonne réponse comptée (partie libre, défi)
--   defi:{defi_id}:{uid}          prime d'achèvement d'un défi
--   duel:{duel_id}:{uid}          prime de duel

CREATE TABLE IF NOT EXISTS jeu.gain (
    id                UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id    UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    montant           INT          NOT NULL,
    origine           VARCHAR(10)  NOT NULL,
    reference_id      UUID         NOT NULL,     -- réponse, défi ou duel
    module_code       VARCHAR(30),
    saison_id         UUID         REFERENCES jeu.saison(id) ON DELETE SET NULL,
    pays_id           UUID         REFERENCES shared.pays(id) ON DELETE SET NULL,
    cle_idempotence   TEXT         NOT NULL UNIQUE,
    annule_at         TIMESTAMPTZ,
    annule_par        UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    motif_annulation  TEXT,
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_gain_montant CHECK (montant > 0),
    CONSTRAINT ck_gain_origine CHECK (origine IN ('partie', 'defi', 'duel'))
);

CREATE INDEX IF NOT EXISTS idx_gain_utilisateur ON jeu.gain (utilisateur_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_gain_saison_pays ON jeu.gain (saison_id, pays_id);


-- ════════════════════════════════════════════════════════════════════════════
-- 9. SCORE_SAISON — l'agrégat lu par les classements
-- ════════════════════════════════════════════════════════════════════════════
-- Tenu dans la MÊME transaction que l'insertion du gain. Le pays fait partie de
-- la clé : un membre qui change de pays en cours de saison ouvre une seconde
-- ligne, l'ancienne ne bouge pas. NULLS NOT DISTINCT (PostgreSQL 16) pour que
-- « sans pays » soit une seule ligne et non une par gain.

CREATE TABLE IF NOT EXISTS jeu.score_saison (
    saison_id       UUID         NOT NULL REFERENCES jeu.saison(id) ON DELETE CASCADE,
    utilisateur_id  UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    pays_id         UUID         REFERENCES shared.pays(id) ON DELETE SET NULL,
    score           INT          NOT NULL DEFAULT 0,
    atteint_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),   -- départage des égalités

    CONSTRAINT ck_score_saison_positif CHECK (score >= 0),
    CONSTRAINT uq_score_saison UNIQUE NULLS NOT DISTINCT (saison_id, utilisateur_id, pays_id)
);

CREATE INDEX IF NOT EXISTS idx_score_saison_classement
    ON jeu.score_saison (saison_id, pays_id, score DESC, atteint_at);


-- ════════════════════════════════════════════════════════════════════════════
-- 10. JOUEUR — le total et la série de jours d'un membre
-- ════════════════════════════════════════════════════════════════════════════
-- Créé paresseusement, comme `engagement.compte`.

CREATE TABLE IF NOT EXISTS jeu.joueur (
    utilisateur_id      UUID         PRIMARY KEY REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    score_total         INT          NOT NULL DEFAULT 0,
    serie_jours         SMALLINT     NOT NULL DEFAULT 0,
    serie_dernier_jour  DATE,                    -- dernier jour UTC d'un défi du jour terminé
    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_joueur_score CHECK (score_total >= 0)
);


-- ════════════════════════════════════════════════════════════════════════════
-- 11. SIGNALEMENT D'ÉPREUVE
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS jeu.signalement_epreuve (
    id              UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    epreuve_id      UUID         NOT NULL REFERENCES jeu.epreuve(id) ON DELETE CASCADE,
    utilisateur_id  UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    motif           VARCHAR(20)  NOT NULL,
    commentaire     TEXT,
    etat            VARCHAR(12)  NOT NULL DEFAULT 'en_attente',
    decision_par    UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    decision_at     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_signalement_epreuve_motif
        CHECK (motif IN ('reponse_erronee', 'enonce_ambigu', 'contenu_deplace', 'autre')),
    CONSTRAINT ck_signalement_epreuve_etat
        CHECK (etat IN ('en_attente', 'confirme', 'classe')),
    CONSTRAINT uq_signalement_epreuve UNIQUE (epreuve_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_signalement_epreuve_etat
    ON jeu.signalement_epreuve (etat, created_at);


-- ════════════════════════════════════════════════════════════════════════════
-- 12. RÈGLES DU JEU — singleton
-- ════════════════════════════════════════════════════════════════════════════
-- Même patron que `engagement.parametre_monetisation` : la clé primaire booléenne
-- contrainte à TRUE rend une seconde ligne impossible, sans code de garde.
-- Les montants de RÉPUTATION ne sont pas ici : ils sont dans le barème
-- d'engagement (fin de fichier).

CREATE TABLE IF NOT EXISTS jeu.regles (
    id                             BOOLEAN   PRIMARY KEY DEFAULT TRUE CHECK (id),
    taille_partie                  SMALLINT  NOT NULL DEFAULT 10  CHECK (taille_partie       BETWEEN 3 AND 30),
    taille_defi_jour               SMALLINT  NOT NULL DEFAULT 5   CHECK (taille_defi_jour    BETWEEN 3 AND 30),
    taille_defi_semaine            SMALLINT  NOT NULL DEFAULT 15  CHECK (taille_defi_semaine BETWEEN 3 AND 30),
    taille_duel                    SMALLINT  NOT NULL DEFAULT 7   CHECK (taille_duel         BETWEEN 3 AND 30),
    temps_epreuve_s                SMALLINT  NOT NULL DEFAULT 30  CHECK (temps_epreuve_s     BETWEEN 5 AND 120),
    score_facile                   SMALLINT  NOT NULL DEFAULT 1   CHECK (score_facile    > 0),
    score_moyen                    SMALLINT  NOT NULL DEFAULT 2   CHECK (score_moyen     > 0),
    score_difficile                SMALLINT  NOT NULL DEFAULT 3   CHECK (score_difficile > 0),
    prime_defi_jour                SMALLINT  NOT NULL DEFAULT 5   CHECK (prime_defi_jour     >= 0),
    prime_defi_semaine             SMALLINT  NOT NULL DEFAULT 15  CHECK (prime_defi_semaine  >= 0),
    prime_duel_victoire            SMALLINT  NOT NULL DEFAULT 5   CHECK (prime_duel_victoire >= 0),
    prime_duel_nul                 SMALLINT  NOT NULL DEFAULT 2   CHECK (prime_duel_nul      >= 0),
    delai_duel_h                   SMALLINT  NOT NULL DEFAULT 48  CHECK (delai_duel_h     BETWEEN 1 AND 168),
    delai_direct_min               SMALLINT  NOT NULL DEFAULT 5   CHECK (delai_direct_min BETWEEN 1 AND 60),
    grace_direct_s                 SMALLINT  NOT NULL DEFAULT 60  CHECK (grace_direct_s   BETWEEN 10 AND 300),
    pause_revelation_s             SMALLINT  NOT NULL DEFAULT 5   CHECK (pause_revelation_s BETWEEN 2 AND 15),
    duels_comptes_par_paire_jour   SMALLINT  NOT NULL DEFAULT 3   CHECK (duels_comptes_par_paire_jour  >= 0),
    duels_comptes_par_membre_jour  SMALLINT  NOT NULL DEFAULT 10  CHECK (duels_comptes_par_membre_jour >= 0),
    joueurs_par_pays               SMALLINT  NOT NULL DEFAULT 10  CHECK (joueurs_par_pays BETWEEN 1 AND 100),
    updated_at                     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO jeu.regles (id) VALUES (TRUE) ON CONFLICT (id) DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
-- 13. VUE DES SOURCES — la source d'une épreuve tient-elle encore ?
-- ════════════════════════════════════════════════════════════════════════════
-- Lue à CHAQUE tirage : une épreuve n'est servie que si sa source est `visible`
-- et, pour une épreuve dérivée, si `empreinte` est encore celle enregistrée à
-- la dérivation. Aucune tâche de fond n'a donc à propager une dépublication.
--
-- L'empreinte ne porte QUE les champs dont on tire des questions. Ni
-- `updated_at` ni les compteurs (`nombre_likes`, `nombre_signalements`) n'y
-- entrent : les tables sources ont un trigger `updated_at`, et un simple
-- « j'aime » sur une fiche renverrait sinon toutes ses épreuves en revue.
--
-- Un objet Afripulse `suspendu` reste affiché sur les pages publiques (aucun
-- handler de liste ne filtre dessus) mais il est exclu du jeu : on ne tire pas
-- de question d'un contenu signalé.
--
-- Ajouter un module au jeu = ajouter une branche ici.

-- Changer ce qu'une empreinte couvre (nouvelle forme de question sur un champ
-- jusque-là ignoré) changerait l'empreinte de TOUTES les sources du type, et
-- rendrait leurs épreuves non servables d'un coup. On relève donc les
-- empreintes avant de remplacer la vue, et on réaligne après les épreuves dont
-- la source n'avait PAS bougé. Sans objet sur une base neuve ; sans effet au
-- rejeu (ancienne et nouvelle empreintes sont alors égales).
-- (Table temporaire sans ON COMMIT DROP : hors transaction explicite, chaque
-- instruction est validée seule et la table disparaîtrait aussitôt.)
DROP TABLE IF EXISTS pg_temp._empreinte_avant;
CREATE TEMP TABLE _empreinte_avant (type_source varchar(30), source_id uuid, empreinte varchar(32));
DO $$
BEGIN
    IF to_regclass('jeu.v_source') IS NOT NULL THEN
        INSERT INTO _empreinte_avant SELECT type_source, source_id, empreinte FROM jeu.v_source;
    END IF;
END $$;

CREATE OR REPLACE VIEW jeu.v_source AS
    SELECT 'fiche_pays'::varchar(30)  AS type_source,
           fp.id                      AS source_id,
           (fp.bloquee = FALSE)       AS visible,
           md5(concat_ws('|', COALESCE(p.nom, ''), COALESCE(p.capitale, ''),
                              COALESCE(fp.monnaie, ''), COALESCE(fp.image_drapeau_url, ''),
                              COALESCE(fp.image_devise_url, ''), COALESCE(p.indicatif_tel, ''),
                              COALESCE(fp.population::text, ''), COALESCE(fp.superficie_km2::text, ''),
                              COALESCE(fp.langue_officielle, '')))::varchar(32) AS empreinte,
           fp.pays_id                 AS pays_id
      FROM country_profile.fiche_pays fp
      JOIN shared.pays p ON p.id = fp.pays_id
    UNION ALL
    SELECT 'site_touristique', st.id,
           (st.deleted_at IS NULL AND st.suspendu = FALSE),
           md5(concat_ws('|', COALESCE(st.nom, ''), st.fiche_pays_id::text))::varchar(32),
           fp.pays_id
      FROM country_profile.site_touristique st
      JOIN country_profile.fiche_pays fp ON fp.id = st.fiche_pays_id
    UNION ALL
    SELECT 'recette_culinaire', rc.id,
           (rc.deleted_at IS NULL AND rc.suspendu = FALSE),
           md5(concat_ws('|', COALESCE(rc.titre, ''), rc.fiche_pays_id::text))::varchar(32),
           fp.pays_id
      FROM country_profile.recette_culinaire rc
      JOIN country_profile.fiche_pays fp ON fp.id = rc.fiche_pays_id
    UNION ALL
    SELECT 'personnalite_connue', pc.id,
           (pc.deleted_at IS NULL AND pc.suspendu = FALSE),
           md5(concat_ws('|', COALESCE(pc.nom_complet, ''), pc.domaine::text, pc.fiche_pays_id::text))::varchar(32),
           fp.pays_id
      FROM country_profile.personnalite_connue pc
      JOIN country_profile.fiche_pays fp ON fp.id = pc.fiche_pays_id
    UNION ALL
    -- Le groupe ethnique n'a ni suppression douce ni suspension : il suit sa fiche.
    SELECT 'groupe_ethnique', ge.id,
           (fp.bloquee = FALSE),
           md5(concat_ws('|', COALESCE(ge.nom, ''), ge.fiche_pays_id::text))::varchar(32),
           fp.pays_id
      FROM country_profile.groupe_ethnique ge
      JOIN country_profile.fiche_pays fp ON fp.id = ge.fiche_pays_id
    UNION ALL
    SELECT 'codimoi', c.id,
           (c.etat = 'publie' AND c.deleted_at IS NULL),
           md5(concat_ws('|', c.type::text, COALESCE(c.contenu, ''),
                              COALESCE(c.pays_id::text, ''), COALESCE(c.nom_auteur_originel, ''),
                              COALESCE(c.explication, '')))::varchar(32),
           c.pays_id
      FROM culture.codimoi c
    UNION ALL
    SELECT 'factcheck', f.id,
           (f.etat = 'publie' AND f.deleted_at IS NULL),
           md5(concat_ws('|', COALESCE(f.prejuge_titre, ''), COALESCE(f.verdict, ''),
                              COALESCE(f.realite_description, '')))::varchar(32),
           f.pays_id
      FROM governance.factcheck f;

UPDATE jeu.epreuve e
   SET source_empreinte = n.empreinte
  FROM _empreinte_avant a
  JOIN jeu.v_source n USING (type_source, source_id)
 WHERE e.origine = 'derivee'
   AND e.type_source = a.type_source AND e.source_id = a.source_id
   AND e.source_empreinte = a.empreinte
   AND e.source_empreinte <> n.empreinte;

DROP TABLE _empreinte_avant;

-- Mise en forme des grandeurs dans les explications des questions de
-- comparaison (« 30,3 millions d'habitants », « 587 041 km² »).
CREATE OR REPLACE FUNCTION jeu.fmt_habitants(n bigint) RETURNS text
LANGUAGE sql IMMUTABLE AS $$
    SELECT CASE
        WHEN n >= 2000000 THEN replace(round(n / 1000000.0, 1)::text, '.', ',') || ' millions d''habitants'
        WHEN n >= 1000000 THEN replace(round(n / 1000000.0, 1)::text, '.', ',') || ' million d''habitants'
        ELSE replace(to_char(n, 'FM999G999'), ',', ' ') || ' habitants'
    END
$$;

CREATE OR REPLACE FUNCTION jeu.fmt_km2(n numeric) RETURNS text
LANGUAGE sql IMMUTABLE AS $$
    SELECT replace(to_char(round(n), 'FM999G999G999'), ',', ' ') || ' km²'
$$;


-- ════════════════════════════════════════════════════════════════════════════
-- 14. DONNÉES DE RÉFÉRENCE
-- ════════════════════════════════════════════════════════════════════════════

-- 14.1 Les quatre modules pilotes. Afrolang n'a aucune forme de dérivation :
--      la plateforme n'a pas de référentiel de langues, ses épreuves sont saisies.
INSERT INTO jeu.module (code, libelle, route, icone, ordre) VALUES
    ('afrolang',  'Afrolang',  '/afrolang',                         'language',       1),
    ('codimoi',   'Codimoi',   '/codi-moi',                         'book-open',      2),
    ('afripulse', 'Afripulse', '/opportunite-afrique',              'earth-africa',   3),
    ('factcheck', 'FactCheck', '/universite/gouvernance/factcheck', 'scale-balanced', 4)
ON CONFLICT (code) DO NOTHING;

-- 14.2 Catégorie d'engagement « jeux » : rend filtrables, dans l'historique du
--      membre, les lignes à 0 point qui portent la réputation du jeu.
INSERT INTO engagement.categorie_points (code, libelle, description, ordre, couleur, icone) VALUES
    ('jeux', 'Activités',
     'Vos accomplissements dans les activités : défis terminés, duels gagnés, places d''honneur au Championship. Ils rapportent de la réputation et des distinctions, jamais de points.',
     8, 'violet', 'gamepad')
ON CONFLICT (code) DO NOTHING;

-- 14.3 Six règles À ZÉRO POINT. Le moteur d'engagement insère bien un mouvement
--      à 0 point et y journalise `reputation_delta` : le solde et le statut du
--      membre ne bougent pas, sa réputation si. Les deux règles à réputation
--      nulle n'existent que pour être COMPTÉES par un badge.
--      ⚠️ Désactiver l'une d'elles fige aussi le badge qui la compte.
INSERT INTO engagement.regle_points
    (type_action, libelle, points, reputation_delta, plafond_journalier,
     plafond_mensuel, seuil_declencheur, categorie_id, actif)
SELECT v.type_action, v.libelle, 0, v.reputation_delta, NULL, NULL, NULL,
       (SELECT id FROM engagement.categorie_points WHERE code = 'jeux'),
       TRUE
  FROM (VALUES
    ('jeu_premiere_partie',      'Première partie terminée',              0),
    ('jeu_defi_termine',         'Défi terminé',                          1),
    ('jeu_serie_7_jours',        'Sept jours de défi consécutifs',         0),
    ('jeu_duel_gagne',           'Duel gagné',                            1),
    ('jeu_podium_saison',        'Place d''honneur au Championship',      10),
    ('jeu_signalement_confirme', 'Signalement d''épreuve confirmé',       2)
  ) AS v(type_action, libelle, reputation_delta)
ON CONFLICT (type_action) DO NOTHING;

-- 14.4 Cinq distinctions, sur la condition existante `actions_comptees` : le
--      moteur compte les lignes du journal par `type_action`.
INSERT INTO engagement.badge
    (code, libelle, description, couleur, icone, ordre, manuel,
     type_condition, parametre_action, seuil)
SELECT v.code, v.libelle, v.description, v.couleur, v.icone, v.ordre, FALSE,
       'actions_comptees'::engagement.type_condition_badge, v.parametre_action, v.seuil
  FROM (VALUES
    ('jeu_premier_pas', 'Premier pas de jeu', 'Avoir terminé sa première partie.',
     'green',    'gamepad',       20, 'jeu_premiere_partie',      1),
    ('jeu_assidu',      'Assidu',             'Avoir terminé le défi du jour sept jours de suite.',
     'amber',    'fire',          21, 'jeu_serie_7_jours',        1),
    ('jeu_duelliste',   'Duelliste',          'Avoir gagné dix duels.',
     'rose',     'hand-fist',     22, 'jeu_duel_gagne',           10),
    ('jeu_champion',    'Champion',           'Avoir fini à une place d''honneur d''une saison du Championship.',
     'chocolat', 'trophy',        23, 'jeu_podium_saison',        1),
    ('jeu_vigie',       'Vigie',              'Trois signalements d''épreuve confirmés par l''équipe.',
     'sky',      'shield-halved', 24, 'jeu_signalement_confirme', 3)
  ) AS v(code, libelle, description, couleur, icone, ordre, parametre_action, seuil)
ON CONFLICT (code) DO NOTHING;

-- 14.5 Habilitation du back-office. 04h donne au rôle `admin` les permissions
--      existant AU MOMENT où il s'exécute : une permission créée après lui n'y
--      va pas seule, d'où le rattachement explicite aux deux rôles.
INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    ('Gérer les activités ludiques', 'jeu.gerer', 'jeu', 'gerer')
ON CONFLICT (slug) DO NOTHING;

INSERT INTO iam.role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM iam.role r, iam.permission p
 WHERE r.slug IN ('super_admin', 'admin') AND p.slug = 'jeu.gerer'
ON CONFLICT (role_id, permission_id) DO NOTHING;
