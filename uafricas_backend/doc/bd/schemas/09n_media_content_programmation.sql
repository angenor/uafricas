-- ============================================================================
-- 09n — media_content : grille de programmation récurrente (US5)
-- ----------------------------------------------------------------------------
-- Les co-détenteurs d'une chaîne ou d'une station établissent une grille dont
-- les contenus se diffusent d'eux-mêmes, SANS tâche de fond : le créneau courant
-- est résolu à la lecture, par calcul SQL sur (NOW() AT TIME ZONE fuseau).
-- C'est le patron maison de la résolution paresseuse (rendez_vous.rs:184,190 ;
-- afrolang.rs:422).
--
-- Écart assumé à « TIMESTAMPTZ partout » (schema.sql:32) : une récurrence n'est
-- pas un instant. Le couple TIME + jour_semaine + fuseau est le seul moyen de
-- l'exprimer sans matérialiser des lignes à l'infini — précisément ce que la
-- résolution paresseuse évite.
--
-- Migration idempotente : CREATE TABLE / INDEX IF NOT EXISTS, DROP puis ADD
-- CONSTRAINT.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. Table creneau_programmation
-- ════════════════════════════════════════════════════════════════════════════
-- type_support réutilise l'ENUM posé par 09m : seuls les SUPPORTS portent une
-- grille, un contenu n'en a pas.

CREATE TABLE IF NOT EXISTS media_content.creneau_programmation (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_support  media_content.type_support_media NOT NULL,
    support_id    UUID        NOT NULL,
    -- programme_tele.id ou programme_radio.id, selon type_support. Pas de FK :
    -- la cible est polymorphe, comme pour les interactions de 09k.
    contenu_id    UUID        NOT NULL,
    recurrence    VARCHAR(20) NOT NULL,
    -- 0 = dimanche … 6 = samedi (convention EXTRACT(DOW)), NULL si quotidien.
    jour_semaine  SMALLINT,
    -- Heure LOCALE du fuseau ci-dessous, jamais UTC.
    heure_debut   TIME        NOT NULL,
    duree_minutes INT         NOT NULL,
    -- Référentiel horaire explicite : une grille panafricaine ne se lit pas
    -- sans savoir de quelle heure on parle (FR-042).
    fuseau        VARCHAR(60) NOT NULL DEFAULT 'Africa/Abidjan',
    -- Co-détenteur auteur du créneau — exigé par la traçabilité (FR-055).
    cree_par      UUID        NOT NULL,   -- [xref] iam.utilisateur
    actif         BOOLEAN     NOT NULL DEFAULT TRUE,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ
);

ALTER TABLE media_content.creneau_programmation
    DROP CONSTRAINT IF EXISTS ck_creneau_recurrence;
ALTER TABLE media_content.creneau_programmation
    ADD CONSTRAINT ck_creneau_recurrence
        CHECK (recurrence IN ('quotidien', 'hebdomadaire'));

-- Un créneau quotidien ne désigne aucun jour ; un créneau hebdomadaire en
-- désigne exactement un. Sans ce CHECK, un « hebdomadaire sans jour » ne serait
-- jamais diffusé et resterait invisible pour son auteur.
ALTER TABLE media_content.creneau_programmation
    DROP CONSTRAINT IF EXISTS ck_creneau_jour_coherent;
ALTER TABLE media_content.creneau_programmation
    ADD CONSTRAINT ck_creneau_jour_coherent
        CHECK (
            (recurrence = 'quotidien'    AND jour_semaine IS NULL) OR
            (recurrence = 'hebdomadaire' AND jour_semaine BETWEEN 0 AND 6)
        );

ALTER TABLE media_content.creneau_programmation
    DROP CONSTRAINT IF EXISTS ck_creneau_duree;
ALTER TABLE media_content.creneau_programmation
    ADD CONSTRAINT ck_creneau_duree
        CHECK (duree_minutes BETWEEN 1 AND 1440);

-- Un créneau ne franchit pas minuit : le scinder en deux si nécessaire. Cette
-- contrainte est ce qui rend la comparaison de chevauchement écrivable en une
-- seule intervalle [heure_debut, heure_debut + duree) sans cas particulier.
ALTER TABLE media_content.creneau_programmation
    DROP CONSTRAINT IF EXISTS ck_creneau_pas_minuit;
ALTER TABLE media_content.creneau_programmation
    ADD CONSTRAINT ck_creneau_pas_minuit
        CHECK (heure_debut + make_interval(mins => duree_minutes) <= TIME '24:00');


-- ════════════════════════════════════════════════════════════════════════════
-- 2. Index
-- ════════════════════════════════════════════════════════════════════════════

-- Requête chaude : « quel contenu passe en ce moment sur ce support ? », jouée
-- à chaque affichage de section.
CREATE INDEX IF NOT EXISTS idx_creneau_support_actif
    ON media_content.creneau_programmation (type_support, support_id, heure_debut)
    WHERE actif = TRUE AND deleted_at IS NULL;

-- « Ce contenu est-il programmé quelque part ? » — utile au retrait d'un
-- contenu et à la détection des créneaux devenus invalides (FR-043).
CREATE INDEX IF NOT EXISTS idx_creneau_contenu
    ON media_content.creneau_programmation (contenu_id)
    WHERE actif = TRUE AND deleted_at IS NULL;


COMMENT ON TABLE media_content.creneau_programmation IS
    'Grille récurrente d''un support média. Le créneau courant est résolu à la
     lecture par (NOW() AT TIME ZONE fuseau) — aucune tâche de fond.';
COMMENT ON COLUMN media_content.creneau_programmation.jour_semaine IS
    '0 = dimanche … 6 = samedi, convention EXTRACT(DOW). NULL si quotidien.';
COMMENT ON COLUMN media_content.creneau_programmation.heure_debut IS
    'Heure LOCALE du fuseau porté par la ligne, jamais UTC.';
