-- ════════════════════════════════════════════════════════════════════════════
-- SCHEMA : social — Rendez-vous en visioconférence entre membres amis
-- Feature 001-rendez-vous-visio — prise de rendez-vous (proposer → répondre /
--   contre-proposer → accepter → annuler). La visioconférence est P2P (WebRTC) :
--   aucun média ni signalisation applicative ne transite par le serveur.
-- ────────────────────────────────────────────────────────────────────────────
-- Dépendances cross-schema : iam.utilisateur (déjà résolue par 29_social.sql).
--
-- Conventions : PK UUID (gen_random_uuid), TIMESTAMPTZ, soft delete deleted_at,
--   snake_case français, enums PostgreSQL.
--
-- Statuts persistés = ('propose', 'accepte', 'refuse', 'annule'). « expiré » et
--   « terminé/passé » sont DÉRIVÉS par calcul (statut + date_heure/durée + NOW())
--   côté applicatif, sans valeur d'énumération ni tâche planifiée (research §2).
--
-- Idempotent : exécutable à chaud en migration manuelle (SSH+psql) ou rejoué par
--   docker-init.sh. Enum via DO/EXCEPTION, table/index via IF NOT EXISTS.
-- ════════════════════════════════════════════════════════════════════════════


-- ── Types (Enums) ────────────────────────────────────────────────────────────

DO $$
BEGIN
    CREATE TYPE social.statut_rendez_vous AS ENUM (
        'propose', 'accepte', 'refuse', 'annule'
    );
EXCEPTION
    WHEN duplicate_object THEN NULL;
END
$$;


-- ── rendez_vous ──────────────────────────────────────────────────────────────
-- Entretien vidéo 1-à-1 entre deux membres amis. `tour_id` = partie qui doit
-- répondre (création : destinataire ; contre-proposition : bascule vers l'autre).

CREATE TABLE IF NOT EXISTS social.rendez_vous (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    initiateur_id    UUID                        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    destinataire_id  UUID                        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    sujet            VARCHAR(150)                NOT NULL,
    description      TEXT,
    date_heure       TIMESTAMPTZ                 NOT NULL,
    duree_minutes    SMALLINT                    NOT NULL,
    statut           social.statut_rendez_vous  NOT NULL DEFAULT 'propose',
    tour_id          UUID                        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    created_at       TIMESTAMPTZ                 NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ                 NOT NULL DEFAULT NOW(),
    deleted_at       TIMESTAMPTZ,

    -- FR-009 : on ne se propose pas un rendez-vous à soi-même
    CONSTRAINT ck_rdv_pas_soi CHECK (initiateur_id <> destinataire_id),
    -- FR-007/010 : créneau prédéfini
    CONSTRAINT ck_rdv_duree CHECK (duree_minutes IN (15, 30, 45, 60)),
    -- FR-006/010 : titre court obligatoire
    CONSTRAINT ck_rdv_sujet_longueur CHECK (char_length(sujet) BETWEEN 1 AND 150)
);

-- Listes « mes rendez-vous » par participant
CREATE INDEX IF NOT EXISTS idx_rdv_initiateur
    ON social.rendez_vous (initiateur_id)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_rdv_destinataire
    ON social.rendez_vous (destinataire_id)
    WHERE deleted_at IS NULL;

-- Filtre « en attente de ma réponse »
CREATE INDEX IF NOT EXISTS idx_rdv_tour
    ON social.rendez_vous (tour_id)
    WHERE statut = 'propose' AND deleted_at IS NULL;

-- Tri / fenêtres « à venir » / « passés »
CREATE INDEX IF NOT EXISTS idx_rdv_date
    ON social.rendez_vous (date_heure);
