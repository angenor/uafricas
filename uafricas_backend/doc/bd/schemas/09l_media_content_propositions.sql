-- ============================================================================
-- 09l — media_content : propositions de médias et modération
-- ----------------------------------------------------------------------------
-- Ouvre la contribution à tout membre connecté tout en garantissant qu'aucun
-- contenu n'atteint le public sans validation administrative
-- (feature 001-refonte-tele-radio, US4 — et US6 pour deux de ses types).
--
-- Une SEULE table polymorphe plutôt que quatre tables dédiées : le workflow est
-- identique pour une chaîne, une station, un programme télé ou radio, et le
-- polymorphisme donne UNE file de modération et UN écran de suivi. Le patron
-- est déjà en production (country_profile.contribution_fiche, 11c:86-113) ;
-- le workflow lui-même reprend afrolang.proposition_salle (08b:359-403).
--
-- Le contenu proposé vit dans `donnees JSONB` jusqu'à la validation, qui crée
-- alors seulement l'objet métier réel. Rien de non validé n'existe dans les
-- tables publiques (FR-031).
--
-- Quatre CHECK rendent le workflow inviolable EN SQL, et pas seulement dans le
-- handler : une décision sans décideur, un rejet sans motif, une validation
-- sans objet créé ou une demande d'animation sans cible sont rejetées par la
-- base elle-même.
--
-- Aucune décharge de droits n'est stockée — décision explicite du commanditaire
-- (H-012). La colonne decharge_droits de vidafrica (27c:18) n'est donc PAS
-- reprise : l'examen de licéité incombe à l'administrateur au moment de la
-- validation (FR-033), à qui l'écran présente la source et l'auteur déclaré.
--
-- Migration idempotente : DO $$ sur pg_type, CREATE TABLE / INDEX IF NOT
-- EXISTS, DROP puis ADD CONSTRAINT.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. Types énumérés
-- ════════════════════════════════════════════════════════════════════════════
-- Les six types de proposition. Les quatre premiers créent un objet métier ;
-- 'animation_programme' crée une ligne de co-détention (FR-045) et
-- 'idee_contenu' ne crée rien du tout (FR-044).

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'type_objet_propose'
                   AND typnamespace = 'media_content'::regnamespace) THEN
        CREATE TYPE media_content.type_objet_propose AS ENUM (
            'chaine_tv',
            'station_radio',
            'programme_tele',
            'programme_radio',
            'animation_programme',   -- FR-045 : validation ⇒ ajoute un co-détenteur
            'idee_contenu'           -- FR-044 : suggestion, ne crée aucun objet
        );
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'statut_proposition_media'
                   AND typnamespace = 'media_content'::regnamespace) THEN
        CREATE TYPE media_content.statut_proposition_media AS ENUM (
            'en_attente',
            'validee',
            'rejetee',
            'retiree'
        );
    END IF;
END $$;


-- ════════════════════════════════════════════════════════════════════════════
-- 2. Table proposition_media
-- ════════════════════════════════════════════════════════════════════════════
-- target_id : NULL = création d'un objet neuf ; renseigné = modification d'un
-- objet existant, demande d'animation, ou idée déposée sur un support.
--
-- auteur_id et decideur sont des références LOGIQUES vers iam.utilisateur,
-- sans FK — convention [xref] du projet, cohérente avec proposition_salle.

CREATE TABLE IF NOT EXISTS media_content.proposition_media (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    auteur_id            UUID        NOT NULL,           -- [xref] iam.utilisateur
    type_objet           media_content.type_objet_propose       NOT NULL,
    target_id            UUID,                            -- objet visé (modification, animation, idée)
    donnees              JSONB       NOT NULL DEFAULT '{}'::jsonb,
    pieces_jointes       JSONB       NOT NULL DEFAULT '[]'::jsonb,
    justification        TEXT        NOT NULL,
    statut               media_content.statut_proposition_media NOT NULL DEFAULT 'en_attente',
    decideur             UUID,                            -- [xref] iam.utilisateur
    decide_at            TIMESTAMPTZ,
    commentaire_decision TEXT,
    objet_id_cree        UUID,                            -- objet réel né de la validation
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- ── Cohérence de la décision ────────────────────────────────────────────────
-- Une proposition en attente n'a ni décideur ni date ; une proposition tranchée
-- par un administrateur porte obligatoirement les deux ; un retrait est le fait
-- de l'auteur, donc sans décideur.

ALTER TABLE media_content.proposition_media
    DROP CONSTRAINT IF EXISTS ck_prop_media_decision_coherente;
ALTER TABLE media_content.proposition_media
    ADD CONSTRAINT ck_prop_media_decision_coherente CHECK (
        (statut = 'en_attente' AND decideur IS NULL AND decide_at IS NULL) OR
        (statut IN ('validee', 'rejetee') AND decideur IS NOT NULL AND decide_at IS NOT NULL) OR
        (statut = 'retiree' AND decideur IS NULL));

-- ── Un rejet est toujours motivé (FR-033) ───────────────────────────────────
-- L'auteur doit pouvoir comprendre le refus depuis son écran de suivi. La
-- garde applicative impose en plus une longueur minimale de 10 caractères.

ALTER TABLE media_content.proposition_media
    DROP CONSTRAINT IF EXISTS ck_prop_media_rejet_commente;
ALTER TABLE media_content.proposition_media
    ADD CONSTRAINT ck_prop_media_rejet_commente CHECK (
        statut <> 'rejetee'
        OR (commentaire_decision IS NOT NULL AND btrim(commentaire_decision) <> ''));

-- ── Une validation produit un objet, sauf pour une idée ─────────────────────
-- 'idee_contenu' est explicitement exempté : une idée retenue ne crée rien,
-- elle est simplement marquée comme telle (FR-044).

ALTER TABLE media_content.proposition_media
    DROP CONSTRAINT IF EXISTS ck_prop_media_validation_a_objet;
ALTER TABLE media_content.proposition_media
    ADD CONSTRAINT ck_prop_media_validation_a_objet CHECK (
        statut <> 'validee'
        OR type_objet = 'idee_contenu'
        OR objet_id_cree IS NOT NULL);

-- ── Une demande d'animation ou une idée vise un support existant ────────────

ALTER TABLE media_content.proposition_media
    DROP CONSTRAINT IF EXISTS ck_prop_media_cible_requise;
ALTER TABLE media_content.proposition_media
    ADD CONSTRAINT ck_prop_media_cible_requise CHECK (
        type_objet NOT IN ('animation_programme', 'idee_contenu')
        OR target_id IS NOT NULL);

-- ── Une justification vide n'en est pas une ─────────────────────────────────

ALTER TABLE media_content.proposition_media
    DROP CONSTRAINT IF EXISTS ck_prop_media_justification;
ALTER TABLE media_content.proposition_media
    ADD CONSTRAINT ck_prop_media_justification CHECK (btrim(justification) <> '');


-- ── Index ───────────────────────────────────────────────────────────────────
-- File de modération : les propositions en attente, les plus anciennes d'abord
-- côté handler, l'index servant le filtre par statut.

CREATE INDEX IF NOT EXISTS idx_prop_media_file
    ON media_content.proposition_media (statut, created_at DESC);

-- Écran « mes soumissions » (FR-034).
CREATE INDEX IF NOT EXISTS idx_prop_media_auteur
    ON media_content.proposition_media (auteur_id, created_at DESC);

-- Filtre combiné type + statut de la file admin.
CREATE INDEX IF NOT EXISTS idx_prop_media_type_statut
    ON media_content.proposition_media (type_objet, statut);

-- Propositions visant un support donné — exposées à ses co-détenteurs (FR-047).
CREATE INDEX IF NOT EXISTS idx_prop_media_target
    ON media_content.proposition_media (target_id)
    WHERE target_id IS NOT NULL;
