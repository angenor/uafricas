-- ============================================================================
--  38 — JEUX VARIÉS ET CONCOURS COMMUNAUTAIRES (feature 014)
-- ============================================================================
--
--  Prolonge le schéma `jeu` de la 013 (37_jeu.sql, PRÉREQUIS) :
--
--  1. Famille A — d'autres façons de répondre : carte, ordre, paires. Le
--     principe de la CLÉ OPAQUE de la 013 s'étend : le joueur ne manipule que
--     des rangs dans des tableaux stockés au hasard, la solution ne sort pas.
--  2. Famille B — les concours : la communauté juge des créations (bataille
--     de photos d'abord), par confrontations de paires à l'aveugle.
--
--  Additive et rejouable : `IF NOT EXISTS`, `DROP CONSTRAINT IF EXISTS` avant
--  chaque contrainte, `ON CONFLICT DO NOTHING`. L'ancien code ignore tout ce
--  qui est ajouté ici : la migration se joue AVANT le déploiement.
--
--  Spécification : specs/014-jeux-concours/ (data-model.md).
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. ÉPREUVE — types de réponse (research D1)
-- ════════════════════════════════════════════════════════════════════════════

-- Vrai si le tableau contient exactement 1..n, chacun une fois : la forme de
-- toute solution d'« ordre » ou de « paires ».
CREATE OR REPLACE FUNCTION jeu.est_permutation(t smallint[]) RETURNS boolean
LANGUAGE sql IMMUTABLE AS $$
    SELECT t IS NOT NULL AND cardinality(t) > 0
       AND (SELECT array_agg(x ORDER BY x) FROM unnest(t) x)
         = (SELECT array_agg(g::smallint ORDER BY g) FROM generate_series(1, cardinality(t)) g)
$$;

ALTER TABLE jeu.epreuve ADD COLUMN IF NOT EXISTS type_reponse    VARCHAR(10) NOT NULL DEFAULT 'choix';
-- ordre  : rangs (dans `propositions`) des éléments, dans l'ordre attendu ;
-- paires : solution[i] = rang, dans `appariements`, du correspondant de propositions[i].
ALTER TABLE jeu.epreuve ADD COLUMN IF NOT EXISTS solution        SMALLINT[];
-- paires : colonne de droite, stockée dans un ordre ALÉATOIRE.
ALTER TABLE jeu.epreuve ADD COLUMN IF NOT EXISTS appariements    TEXT[];
-- ordre : justification de chaque élément (« 46,0 millions d'habitants »),
-- alignée sur `propositions`, montrée à la correction seulement.
ALTER TABLE jeu.epreuve ADD COLUMN IF NOT EXISTS valeurs         TEXT[];
-- carte : le pays attendu.
ALTER TABLE jeu.epreuve ADD COLUMN IF NOT EXISTS reponse_pays_id UUID REFERENCES shared.pays(id);

ALTER TABLE jeu.epreuve ALTER COLUMN bonne_reponse DROP NOT NULL;

-- Les deux contraintes de la 013 supposaient que toute épreuve est un choix.
ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_propositions;
ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_bonne_reponse;

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_type_reponse;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_type_reponse
    CHECK (type_reponse IN ('choix', 'carte', 'ordre', 'paires'));

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_choix;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_choix CHECK (
    type_reponse <> 'choix'
    OR (cardinality(propositions) BETWEEN 2 AND 6
        AND bonne_reponse BETWEEN 1 AND cardinality(propositions)));

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_carte;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_carte CHECK (
    type_reponse <> 'carte' OR cardinality(propositions) = 0);

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_ordre;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_ordre CHECK (
    type_reponse <> 'ordre'
    OR (cardinality(propositions) BETWEEN 3 AND 6
        AND cardinality(solution) = cardinality(propositions)
        AND jeu.est_permutation(solution)));

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_paires;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_paires CHECK (
    type_reponse <> 'paires'
    OR (cardinality(propositions) BETWEEN 3 AND 5
        AND cardinality(appariements) = cardinality(propositions)
        AND cardinality(solution) = cardinality(propositions)
        AND jeu.est_permutation(solution)));

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_bonne_si_choix;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_bonne_si_choix
    CHECK ((bonne_reponse IS NOT NULL) = (type_reponse = 'choix'));

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_pays_si_carte;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_pays_si_carte
    CHECK ((reponse_pays_id IS NOT NULL) = (type_reponse = 'carte'));

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_solution_si_ordre_paires;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_solution_si_ordre_paires
    CHECK ((solution IS NOT NULL) = (type_reponse IN ('ordre', 'paires')));

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_appariements_si_paires;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_appariements_si_paires
    CHECK ((appariements IS NOT NULL) = (type_reponse = 'paires'));

ALTER TABLE jeu.epreuve DROP CONSTRAINT IF EXISTS ck_epreuve_valeurs;
ALTER TABLE jeu.epreuve ADD  CONSTRAINT ck_epreuve_valeurs CHECK (
    valeurs IS NULL
    OR (type_reponse = 'ordre' AND cardinality(valeurs) = cardinality(propositions)));


-- ════════════════════════════════════════════════════════════════════════════
-- 2. RÉPONSE — la réponse jouée, quel que soit son type (research D2)
-- ════════════════════════════════════════════════════════════════════════════

-- ordre  : les clés dans l'ordre proposé par le joueur ;
-- paires : clé de droite choisie pour chaque clé de gauche, dans l'ordre des
--          clés de gauche.
ALTER TABLE jeu.reponse ADD COLUMN IF NOT EXISTS reponse_detail SMALLINT[];
ALTER TABLE jeu.reponse ADD COLUMN IF NOT EXISTS pays_choisi_id UUID REFERENCES shared.pays(id);


-- ════════════════════════════════════════════════════════════════════════════
-- 3. RÈGLES DU JEU (research D3, D10)
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE jeu.regles ADD COLUMN IF NOT EXISTS majoration_ordre_paires_s   SMALLINT   NOT NULL DEFAULT 15;
ALTER TABLE jeu.regles ADD COLUMN IF NOT EXISTS prime_concours_participation SMALLINT  NOT NULL DEFAULT 2;
ALTER TABLE jeu.regles ADD COLUMN IF NOT EXISTS prime_concours_podium       SMALLINT[] NOT NULL DEFAULT '{30,20,10}';

ALTER TABLE jeu.regles DROP CONSTRAINT IF EXISTS ck_regles_majoration;
ALTER TABLE jeu.regles ADD  CONSTRAINT ck_regles_majoration
    CHECK (majoration_ordre_paires_s BETWEEN 0 AND 60);
ALTER TABLE jeu.regles DROP CONSTRAINT IF EXISTS ck_regles_prime_participation;
ALTER TABLE jeu.regles ADD  CONSTRAINT ck_regles_prime_participation
    CHECK (prime_concours_participation >= 0);
ALTER TABLE jeu.regles DROP CONSTRAINT IF EXISTS ck_regles_prime_podium;
ALTER TABLE jeu.regles ADD  CONSTRAINT ck_regles_prime_podium CHECK (
    cardinality(prime_concours_podium) = 3
    AND prime_concours_podium[3] >= 0
    AND prime_concours_podium[1] >= prime_concours_podium[2]
    AND prime_concours_podium[2] >= prime_concours_podium[3]);


-- ════════════════════════════════════════════════════════════════════════════
-- 4. GAIN — une origine de plus
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE jeu.gain DROP CONSTRAINT IF EXISTS ck_gain_origine;
ALTER TABLE jeu.gain ADD  CONSTRAINT ck_gain_origine
    CHECK (origine IN ('partie', 'defi', 'duel', 'concours'));


-- ════════════════════════════════════════════════════════════════════════════
-- 5. CONCOURS (research D6, D13)
-- ════════════════════════════════════════════════════════════════════════════
-- La PHASE n'est jamais stockée : elle se calcule à la lecture d'après les
-- dates et l'état. Deux transitions seulement s'écrivent (annulation au seuil
-- du vote, établissement des résultats), sous verrou de la ligne.

CREATE TABLE IF NOT EXISTS jeu.concours (
    id                      UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    format                  VARCHAR(12)   NOT NULL DEFAULT 'photo',
    titre                   VARCHAR(150)  NOT NULL,
    theme                   TEXT          NOT NULL,
    reglement               TEXT          NOT NULL,
    -- Code d'un module de la PLATEFORME (afroculture, afripulse…), pour
    -- l'affichage sur sa page ; pas une règle de jeu, donc pas de FK.
    rattachement            VARCHAR(30),
    image_url               VARCHAR(500),
    appel_debut             TIMESTAMPTZ   NOT NULL,
    vote_debut              TIMESTAMPTZ   NOT NULL,
    vote_fin                TIMESTAMPTZ   NOT NULL,
    participations_max      SMALLINT      NOT NULL DEFAULT 1,
    minimum_participations  SMALLINT      NOT NULL DEFAULT 4,
    votes_max               INT,
    presentations_min       SMALLINT      NOT NULL DEFAULT 10,
    jury                    BOOLEAN       NOT NULL DEFAULT FALSE,
    jury_finalistes         SMALLINT      NOT NULL DEFAULT 10,
    jury_delai_jours        SMALLINT      NOT NULL DEFAULT 7,
    prime_participation     SMALLINT,
    prime_podium            SMALLINT[],
    etat                    VARCHAR(10)   NOT NULL DEFAULT 'actif',
    motif_annulation        TEXT,
    podium_jury             UUID[],
    delibere_par            UUID          REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    delibere_at             TIMESTAMPTZ,
    resultats_at            TIMESTAMPTZ,
    cree_par                UUID          REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    created_at              TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ   NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_concours_format       CHECK (format IN ('photo')),
    CONSTRAINT ck_concours_titre        CHECK (btrim(titre) <> ''),
    CONSTRAINT ck_concours_theme        CHECK (btrim(theme) <> ''),
    CONSTRAINT ck_concours_reglement    CHECK (btrim(reglement) <> ''),
    CONSTRAINT ck_concours_rattachement CHECK (rattachement IS NULL OR rattachement ~ '^[a-z_]{3,30}$'),
    CONSTRAINT ck_concours_dates        CHECK (appel_debut < vote_debut AND vote_fin >= vote_debut + INTERVAL '24 hours'),
    CONSTRAINT ck_concours_participations CHECK (participations_max BETWEEN 1 AND 10),
    CONSTRAINT ck_concours_minimum      CHECK (minimum_participations >= 2),
    CONSTRAINT ck_concours_votes_max    CHECK (votes_max IS NULL OR votes_max >= 10),
    CONSTRAINT ck_concours_presentations CHECK (presentations_min >= 1),
    CONSTRAINT ck_concours_finalistes   CHECK (jury_finalistes BETWEEN 3 AND 30),
    CONSTRAINT ck_concours_jury_delai   CHECK (jury_delai_jours BETWEEN 1 AND 30),
    CONSTRAINT ck_concours_prime_part   CHECK (prime_participation IS NULL OR prime_participation >= 0),
    CONSTRAINT ck_concours_prime_podium CHECK (
        prime_podium IS NULL
        OR (cardinality(prime_podium) = 3 AND prime_podium[3] >= 0
            AND prime_podium[1] >= prime_podium[2] AND prime_podium[2] >= prime_podium[3])),
    CONSTRAINT ck_concours_etat         CHECK (etat IN ('actif', 'annule', 'resultats')),
    CONSTRAINT ck_concours_annulation   CHECK (etat <> 'annule' OR btrim(COALESCE(motif_annulation, '')) <> ''),
    CONSTRAINT ck_concours_resultats    CHECK (etat <> 'resultats' OR resultats_at IS NOT NULL),
    CONSTRAINT ck_concours_podium_jury  CHECK (podium_jury IS NULL OR cardinality(podium_jury) BETWEEN 1 AND 3)
);

CREATE INDEX IF NOT EXISTS idx_concours_vote_fin     ON jeu.concours (vote_fin);
CREATE INDEX IF NOT EXISTS idx_concours_rattachement ON jeu.concours (rattachement, vote_fin);


-- ════════════════════════════════════════════════════════════════════════════
-- 6. PARTICIPATION (research D6, D12)
-- ════════════════════════════════════════════════════════════════════════════
-- Le vote et le dépouillement ne lisent JAMAIS la création elle-même : un
-- futur défi vidéo ou de traduction ajoutera un format, pas un cycle.

CREATE TABLE IF NOT EXISTS jeu.participation (
    id                    UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    concours_id           UUID          NOT NULL REFERENCES jeu.concours(id) ON DELETE CASCADE,
    auteur_id             UUID          NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    media_type            VARCHAR(10)   NOT NULL DEFAULT 'image',
    media_url             VARCHAR(500)  NOT NULL,
    legende               VARCHAR(200),
    etat                  VARCHAR(12)   NOT NULL DEFAULT 'en_attente',
    motif_rejet           TEXT,
    modere_par            UUID          REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    modere_at             TIMESTAMPTZ,
    -- Incrémenté à chaque PRÉSENTATION (pas au vote) : c'est ce que le tirage
    -- équilibre (research D7).
    nombre_presentations  INT           NOT NULL DEFAULT 0,
    nombre_signalements   INT           NOT NULL DEFAULT 0,
    created_at            TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ   NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_participation_media_type CHECK (media_type IN ('image')),
    CONSTRAINT ck_participation_media      CHECK (btrim(media_url) <> ''),
    CONSTRAINT ck_participation_etat       CHECK (etat IN ('en_attente', 'publiee', 'rejetee', 'retiree', 'suspendue')),
    CONSTRAINT ck_participation_rejet      CHECK (etat <> 'rejetee' OR btrim(COALESCE(motif_rejet, '')) <> '')
);

CREATE INDEX IF NOT EXISTS idx_participation_concours_etat ON jeu.participation (concours_id, etat);
CREATE INDEX IF NOT EXISTS idx_participation_presentations ON jeu.participation (concours_id, nombre_presentations)
    WHERE etat = 'publiee';
CREATE INDEX IF NOT EXISTS idx_participation_auteur        ON jeu.participation (auteur_id);


-- ════════════════════════════════════════════════════════════════════════════
-- 7. CONFRONTATION — une paire présentée à un votant (research D7, D9)
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS jeu.confrontation (
    id            UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    concours_id   UUID          NOT NULL REFERENCES jeu.concours(id) ON DELETE CASCADE,
    votant_id     UUID          NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    a_id          UUID          NOT NULL REFERENCES jeu.participation(id) ON DELETE CASCADE,
    b_id          UUID          NOT NULL REFERENCES jeu.participation(id) ON DELETE CASCADE,
    gauche_est_a  BOOLEAN       NOT NULL,
    presentee_at  TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    choix_id      UUID,
    vote_at       TIMESTAMPTZ,
    comptee       BOOLEAN,
    motif_ecart   VARCHAR(16),

    CONSTRAINT ck_confrontation_paire  CHECK (a_id < b_id),
    CONSTRAINT ck_confrontation_choix  CHECK (choix_id IS NULL OR choix_id IN (a_id, b_id)),
    CONSTRAINT ck_confrontation_vote   CHECK ((choix_id IS NULL) = (vote_at IS NULL)),
    CONSTRAINT ck_confrontation_compte CHECK (
        (choix_id IS NULL AND comptee IS NULL AND motif_ecart IS NULL)
        OR (choix_id IS NOT NULL AND comptee IS NOT NULL AND comptee = (motif_ecart IS NULL))),
    CONSTRAINT ck_confrontation_motif  CHECK (
        motif_ecart IS NULL OR motif_ecart IN ('trop_rapide', 'compte_recent', 'non_verifie', 'ecartee_admin')),
    -- Une paire n'est JAMAIS représentée à un même votant (FR-036).
    CONSTRAINT uq_confrontation_paire  UNIQUE (concours_id, votant_id, a_id, b_id)
);

-- Au plus UNE confrontation ouverte par votant : recharger la page rend la
-- même paire, on ne « passe » pas une paire qui déplaît (research D7).
CREATE UNIQUE INDEX IF NOT EXISTS uq_confrontation_ouverte
    ON jeu.confrontation (concours_id, votant_id) WHERE choix_id IS NULL;
CREATE INDEX IF NOT EXISTS idx_confrontation_depouillement
    ON jeu.confrontation (concours_id) WHERE choix_id IS NOT NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- 8. RÉSULTAT — figé à l'établissement, jamais mis à jour (research D8)
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS jeu.resultat_concours (
    concours_id       UUID          NOT NULL REFERENCES jeu.concours(id) ON DELETE CASCADE,
    participation_id  UUID          NOT NULL REFERENCES jeu.participation(id) ON DELETE CASCADE,
    rang              SMALLINT      NOT NULL,
    victoires         INT           NOT NULL DEFAULT 0,
    duels             INT           NOT NULL DEFAULT 0,
    taux              NUMERIC(5,4)  NOT NULL DEFAULT 0,
    sous_seuil        BOOLEAN       NOT NULL DEFAULT FALSE,
    place_jury        SMALLINT,

    PRIMARY KEY (concours_id, participation_id),
    CONSTRAINT ck_resultat_rang       CHECK (rang >= 1),
    CONSTRAINT ck_resultat_taux       CHECK (taux BETWEEN 0 AND 1),
    CONSTRAINT ck_resultat_place_jury CHECK (place_jury IS NULL OR place_jury BETWEEN 1 AND 3)
);


-- ════════════════════════════════════════════════════════════════════════════
-- 9. SIGNALEMENT DE PARTICIPATION (research D12)
-- ════════════════════════════════════════════════════════════════════════════
-- Au-delà de 10 signalements distincts : `suspendue` (même seuil que les médias).

CREATE TABLE IF NOT EXISTS jeu.signalement_participation (
    id                UUID          PRIMARY KEY DEFAULT uuid_generate_v4(),
    participation_id  UUID          NOT NULL REFERENCES jeu.participation(id) ON DELETE CASCADE,
    utilisateur_id    UUID          NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    motif             TEXT          NOT NULL,
    created_at        TIMESTAMPTZ   NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_signalement_participation_motif CHECK (btrim(motif) <> ''),
    CONSTRAINT uq_signalement_participation UNIQUE (participation_id, utilisateur_id)
);


-- ════════════════════════════════════════════════════════════════════════════
-- 10. ENGAGEMENT — réputation et distinctions des concours (research D10)
-- ════════════════════════════════════════════════════════════════════════════
-- Règles À ZÉRO POINT, comme celles de la 013 : la réputation bouge, ni le
-- solde de points ni le statut.

INSERT INTO engagement.regle_points
    (type_action, libelle, points, reputation_delta, plafond_journalier,
     plafond_mensuel, seuil_declencheur, categorie_id, actif)
SELECT v.type_action, v.libelle, 0, v.reputation_delta, NULL, NULL, NULL,
       (SELECT id FROM engagement.categorie_points WHERE code = 'jeux'),
       TRUE
  FROM (VALUES
    ('jeu_concours_participation', 'Participation publiée dans un concours', 1),
    ('jeu_concours_podium',        'Place sur le podium d''un concours',      5),
    ('jeu_concours_vote',          'Votant actif dans un concours',           1)
  ) AS v(type_action, libelle, reputation_delta)
ON CONFLICT (type_action) DO NOTHING;

INSERT INTO engagement.badge
    (code, libelle, description, couleur, icone, ordre, manuel,
     type_condition, parametre_action, seuil)
SELECT v.code, v.libelle, v.description, v.couleur, v.icone, v.ordre, FALSE,
       'actions_comptees'::engagement.type_condition_badge, v.parametre_action, v.seuil
  FROM (VALUES
    ('jeu_laureat',        'Lauréat',        'Être monté sur le podium d''un concours.',
     'amber', 'medal',          25, 'jeu_concours_podium', 1),
    ('jeu_jure_populaire', 'Juré populaire', 'Avoir voté dans cinq concours.',
     'sky',   'scale-balanced', 26, 'jeu_concours_vote',   5)
  ) AS v(code, libelle, description, couleur, icone, ordre, parametre_action, seuil)
ON CONFLICT (code) DO NOTHING;
