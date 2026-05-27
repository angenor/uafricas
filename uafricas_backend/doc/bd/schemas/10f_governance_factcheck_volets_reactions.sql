-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — governance : FactCheck — Volets (préjugé/réalité) + réactions
-- ════════════════════════════════════════════════════════════════════════════
-- Idempotent : exécutable sur une base déjà initialisée comme en init fraîche.
--
-- 1. Chaque factcheck porte deux volets 1:1 — le préjugé et la réalité — chacun
--    avec son propre titre/description et son compteur de cœurs indépendant.
-- 2. La réaction globale au post devient un jeu d'emojis : cœur, pouce (j'aime),
--    rire (ça me fait rire), je n'aime pas — une seule réaction par utilisateur.
-- ════════════════════════════════════════════════════════════════════════════


-- ── Volets préjugé / réalité + compteurs de réaction globale ────────────────

ALTER TABLE governance.factcheck
    ADD COLUMN IF NOT EXISTS prejuge_titre        VARCHAR(350),
    ADD COLUMN IF NOT EXISTS prejuge_description  TEXT,
    ADD COLUMN IF NOT EXISTS prejuge_nombre_likes INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS realite_titre        VARCHAR(350),
    ADD COLUMN IF NOT EXISTS realite_description  TEXT,
    ADD COLUMN IF NOT EXISTS realite_nombre_likes INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS nombre_coeur         INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS nombre_pouce         INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS nombre_rire          INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS nombre_jaime_pas     INT NOT NULL DEFAULT 0;


-- ── Réactions multi-cibles (general / prejuge / realite) ────────────────────

ALTER TABLE governance.factcheck_reaction
    ADD COLUMN IF NOT EXISTS cible VARCHAR(10) NOT NULL DEFAULT 'general';

-- Migration des réactions héritées : like → coeur, dislike → jaime_pas (cible générale)
UPDATE governance.factcheck_reaction SET type_reaction = 'coeur'     WHERE type_reaction = 'like';
UPDATE governance.factcheck_reaction SET type_reaction = 'jaime_pas' WHERE type_reaction = 'dislike';

-- Report unique des compteurs hérités vers les nouveaux compteurs d'emojis
UPDATE governance.factcheck SET nombre_coeur     = nombre_likes    WHERE nombre_coeur = 0     AND nombre_likes    > 0;
UPDATE governance.factcheck SET nombre_jaime_pas = nombre_dislikes WHERE nombre_jaime_pas = 0 AND nombre_dislikes > 0;

-- Contrainte de type élargie selon la cible
ALTER TABLE governance.factcheck_reaction DROP CONSTRAINT IF EXISTS factcheck_reaction_type_reaction_check;
ALTER TABLE governance.factcheck_reaction DROP CONSTRAINT IF EXISTS factcheck_reaction_type_chk;
ALTER TABLE governance.factcheck_reaction
    ADD CONSTRAINT factcheck_reaction_type_chk CHECK (
        (cible = 'general'             AND type_reaction IN ('coeur', 'pouce', 'rire', 'jaime_pas'))
     OR (cible IN ('prejuge', 'realite') AND type_reaction = 'coeur')
    );

-- Unicité : une réaction par utilisateur, par cible, par factcheck
ALTER TABLE governance.factcheck_reaction DROP CONSTRAINT IF EXISTS factcheck_reaction_factcheck_id_utilisateur_id_key;
ALTER TABLE governance.factcheck_reaction DROP CONSTRAINT IF EXISTS factcheck_reaction_unique_cible;
ALTER TABLE governance.factcheck_reaction
    ADD CONSTRAINT factcheck_reaction_unique_cible UNIQUE (factcheck_id, utilisateur_id, cible);

CREATE INDEX IF NOT EXISTS idx_factcheck_react_cible
    ON governance.factcheck_reaction(factcheck_id, cible);
