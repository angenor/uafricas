-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Engagement : catégories de points & barème paramétrable
-- ════════════════════════════════════════════════════════════════════════════
-- Feature 007-engagement-points-badges — Phase 2 (socle commun US1 + US2).
--
-- 1. `engagement.categorie_points` : catégories de ventilation des points.
-- 2. `regle_points` gagne `categorie_id` et `seuil_declencheur`.
-- 3. `mouvement_points` gagne `categorie_id`, FIGÉE à l'écriture (R1) : une
--    re-catégorisation d'une règle ne réécrit jamais l'historique déjà journalisé.
-- 4. `palier_popularite` gagne `type_objet` (NULL = palier global) ; l'unicité
--    passe de `(seuil_likes)` à `(seuil_likes, type_objet) NULLS NOT DISTINCT`.
-- 5. `niveau.seuil_min` devient unique (interdit deux niveaux au même seuil).
-- 6. Seeds : 6 catégories, rattachement des 6 règles existantes, 4 règles neuves.
-- 7. Rattrapage unique de l'historique (`mouvement_points.categorie_id`).
--
-- Migration idempotente (IF NOT EXISTS, ON CONFLICT DO NOTHING) — appliquée à la
-- main en production via SSH + psql.
-- ════════════════════════════════════════════════════════════════════════════


-- ── 1. Catégories de points ─────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS engagement.categorie_points (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code        VARCHAR(30)  NOT NULL UNIQUE,       -- clé stable, immuable après création
    libelle     VARCHAR(80)  NOT NULL,              -- affiché au membre
    description TEXT,                               -- texte pédagogique de l'état vide (FR-015)
    ordre       SMALLINT     NOT NULL DEFAULT 0,
    couleur     VARCHAR(20),                        -- jeton de couleur front
    icone       VARCHAR(40),                        -- nom FontAwesome
    actif       BOOLEAN      NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_categorie_points_ordre
    ON engagement.categorie_points(ordre);


-- ── 2. Règles de barème : catégorie + seuil déclencheur ─────────────────────

ALTER TABLE engagement.regle_points
    ADD COLUMN IF NOT EXISTS categorie_id UUID
        REFERENCES engagement.categorie_points(id) ON DELETE RESTRICT;

-- Nombre d'occurrences distinctes nécessaires avant crédit (R10).
-- NULL = crédit immédiat (cas de toutes les règles sauf « 5 réseaux distincts »).
ALTER TABLE engagement.regle_points
    ADD COLUMN IF NOT EXISTS seuil_declencheur INTEGER;


-- ── 3. Mouvements : catégorie figée au moment du mouvement (R1) ──────────────

ALTER TABLE engagement.mouvement_points
    ADD COLUMN IF NOT EXISTS categorie_id UUID
        REFERENCES engagement.categorie_points(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_mouvement_categorie
    ON engagement.mouvement_points(utilisateur_id, categorie_id);


-- ── 4. Paliers de popularité : restriction à une famille de contenus (R4) ────

ALTER TABLE engagement.palier_popularite
    ADD COLUMN IF NOT EXISTS type_objet VARCHAR(40);   -- NULL = palier global

-- L'unicité ne porte plus sur le seuil seul : un même seuil peut exister une
-- fois en global et une fois par famille.
ALTER TABLE engagement.palier_popularite
    DROP CONSTRAINT IF EXISTS palier_popularite_seuil_likes_key;

-- `NULLS NOT DISTINCT` (PostgreSQL 15+) est INDISPENSABLE : sans lui, deux
-- paliers globaux de même seuil coexisteraient (deux NULL étant distincts par
-- défaut) et le même seuil créditerait deux fois.
CREATE UNIQUE INDEX IF NOT EXISTS idx_uq_palier_seuil_famille
    ON engagement.palier_popularite (seuil_likes, type_objet) NULLS NOT DISTINCT;

CREATE INDEX IF NOT EXISTS idx_palier_famille
    ON engagement.palier_popularite(type_objet) WHERE actif = TRUE;


-- ── 5. Niveaux : un seul niveau par seuil ────────────────────────────────────
-- Interdit l'edge case « niveaux mal ordonnés ». `ordre` est recalculé
-- applicativement d'après `seuil_min` croissant à chaque mutation (R5), de sorte
-- que `ordre` et `seuil_min` ne puissent jamais se contredire.

CREATE UNIQUE INDEX IF NOT EXISTS idx_uq_niveau_seuil
    ON engagement.niveau (seuil_min);


-- ════════════════════════════════════════════════════════════════════════════
-- SEED — 6 catégories de ventilation
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO engagement.categorie_points (code, libelle, description, ordre, couleur, icone) VALUES
    ('contributions', 'Contributions',
     'Vos contributions validées par la modération : Codimoi, vidéos, idées-forces, mauvaises habitudes, fiches pays.',
     1, 'green', 'pen-nib'),
    ('popularite', 'Popularité',
     'Les points gagnés lorsque vos publications franchissent des paliers de « j''aime ».',
     2, 'rose', 'heart'),
    ('medias', 'Médias (télé & radio)',
     'Vos propositions de chaînes, stations et programmes validées, vos contenus mis à la une et vos animations acceptées.',
     3, 'amber', 'tv'),
    ('factcheck', 'Vérification de faits',
     'Vos vérifications de faits jugées correctes par la modération.',
     4, 'sky', 'magnifying-glass-chart'),
    ('partages', 'Partages',
     'Le partage d''un contenu vers plusieurs réseaux sociaux différents.',
     5, 'violet', 'share-nodes'),
    ('ajustements', 'Ajustements',
     'Les corrections manuelles effectuées par l''équipe d''administration.',
     6, 'gray', 'sliders')
ON CONFLICT (code) DO NOTHING;


-- ── Rattachement des 6 règles seedées par 35_engagement.sql ─────────────────
-- Idempotent : une re-catégorisation manuelle ultérieure ne sera pas écrasée,
-- car on ne rattache que les règles encore sans catégorie.

UPDATE engagement.regle_points r
   SET categorie_id = c.id, updated_at = NOW()
  FROM engagement.categorie_points c
 WHERE r.categorie_id IS NULL
   AND c.code = CASE r.type_action
        WHEN 'contribution_validee'       THEN 'contributions'
        WHEN 'contribution_mise_en_avant' THEN 'contributions'
        WHEN 'factcheck_valide'           THEN 'factcheck'
        WHEN 'factcheck_faux'             THEN 'factcheck'
        WHEN 'popularite_palier'          THEN 'popularite'
        WHEN 'ajustement_admin'           THEN 'ajustements'
        ELSE NULL
       END;


-- ════════════════════════════════════════════════════════════════════════════
-- SEED — 4 règles nouvelles (US4 : médias · US5 : partage externe)
-- ════════════════════════════════════════════════════════════════════════════
-- ⚠️ `plafond_journalier` est exprimé EN POINTS, pas en nombre d'actions :
--    le moteur compare `SUM(points)` du jour au plafond. « 3 bonus de 10 points
--    par jour » se paramètre donc à 30, pas à 3.

INSERT INTO engagement.regle_points
    (type_action, libelle, points, reputation_delta, plafond_journalier, plafond_mensuel, seuil_declencheur, categorie_id)
SELECT v.type_action, v.libelle, v.points, v.reputation_delta,
       v.plafond_journalier, v.plafond_mensuel, v.seuil_declencheur,
       (SELECT id FROM engagement.categorie_points WHERE code = v.code_categorie)
  FROM (VALUES
    ('proposition_media_validee',  'Proposition de média validée',                    5, 1, NULL::integer, NULL::integer, NULL::integer, 'medias'),
    ('media_a_la_une',             'Contenu média mis à la une',                       8, 1, NULL,          NULL,          NULL,          'medias'),
    ('animation_support_acceptee', 'Demande d''animation d''un support acceptée',     15, 2, NULL,          NULL,          NULL,          'medias'),
    ('partage_externe_5reseaux',   'Contenu partagé sur 5 réseaux sociaux distincts', 10, 0, 30,            NULL,          5,             'partages')
  ) AS v(type_action, libelle, points, reputation_delta, plafond_journalier, plafond_mensuel, seuil_declencheur, code_categorie)
ON CONFLICT (type_action) DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
-- RATTRAPAGE UNIQUE — catégorie des mouvements déjà journalisés
-- ════════════════════════════════════════════════════════════════════════════
-- Les mouvements produits avant cette migration n'ont pas de catégorie. On les
-- rattache d'après la règle qui les a émis. Les rares reliquats (règle
-- supprimée, action jamais déclarée) restent NULL et s'affichent sous « Autres »
-- côté membre — aucune erreur, aucune ligne masquée.

UPDATE engagement.mouvement_points m
   SET categorie_id = r.categorie_id
  FROM engagement.regle_points r
 WHERE r.type_action = m.type_action
   AND m.categorie_id IS NULL
   AND r.categorie_id IS NOT NULL;
