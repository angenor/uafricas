-- ============================================================================
-- 09m — media_content : co-détention des supports (chaînes et stations)
-- ----------------------------------------------------------------------------
-- Plusieurs membres peuvent détenir un même support — arbitrage explicite du
-- commanditaire (clarification du 2026-07-19, feature 001-refonte-tele-radio).
--
-- Cette migration est nominalement rattachée à US5 (programmation), mais elle
-- est livrée avec US4 : la validation d'une proposition de chaîne ou de station
-- crée, dans la MÊME transaction, la ligne de propriété de son auteur. Sans
-- support_detenteur, cette transaction n'a pas de cible et l'auteur validé ne
-- devient propriétaire de rien.
--
--   • support_detenteur    — qui détient quoi, à quel rôle (modèle
--                            afrolang.salle_moderateur, 08b:224-242) ;
--   • invitation_detenteur — comment un propriétaire en associe d'autres
--                            (modèle arbre_genealogique.invitations, 25:16-38).
--
-- Migration idempotente : DO $$ sur pg_type, CREATE TABLE / INDEX IF NOT
-- EXISTS, DROP puis ADD CONSTRAINT.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. Types énumérés
-- ════════════════════════════════════════════════════════════════════════════
-- Seuls les SUPPORTS se détiennent : un programme appartient à son support, il
-- n'a pas de détenteur propre. D'où deux valeurs, contre quatre pour le
-- type_media des interactions (09k).

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'type_support_media'
                   AND typnamespace = 'media_content'::regnamespace) THEN
        CREATE TYPE media_content.type_support_media AS ENUM (
            'chaine_tv',
            'station_radio'
        );
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'role_detenteur'
                   AND typnamespace = 'media_content'::regnamespace) THEN
        CREATE TYPE media_content.role_detenteur AS ENUM (
            'proprietaire',   -- tout, plus inviter et révoquer les autres
            'co_detenteur',   -- éditer la fiche, gérer les contenus, programmer
            'programmateur'   -- programmer uniquement
        );
    END IF;
END $$;


-- ════════════════════════════════════════════════════════════════════════════
-- 2. Table support_detenteur
-- ════════════════════════════════════════════════════════════════════════════
-- Le retrait est un SOFT delete (actif = FALSE, retire_at) et non une
-- suppression : l'historique de détention n'est jamais effacé. L'ajout est donc
-- un upsert-réactivation à trois branches, repris tel quel de
-- admin/moderateurs_afrolang.rs:59-190.

CREATE TABLE IF NOT EXISTS media_content.support_detenteur (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_support   media_content.type_support_media NOT NULL,
    support_id     UUID        NOT NULL,
    utilisateur_id UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    role           media_content.role_detenteur NOT NULL DEFAULT 'co_detenteur',
    designe_par    UUID,                            -- [xref] iam.utilisateur
    designe_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    actif          BOOLEAN     NOT NULL DEFAULT TRUE,
    retire_at      TIMESTAMPTZ,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- SANS filtre sur `actif` : une seule ligne par paire, jamais de doublon
    -- historique. C'est ce qui rend l'upsert-réactivation possible.
    CONSTRAINT uq_support_detenteur_membre UNIQUE (type_support, support_id, utilisateur_id)
);

-- Un seul propriétaire ACTIF par support. Un support peut en revanche n'en
-- avoir aucun : le retrait du dernier détenteur laisse le support diffusé et
-- administrable (edge case « dernier co-détenteur retiré »).
CREATE UNIQUE INDEX IF NOT EXISTS uq_support_un_proprietaire
    ON media_content.support_detenteur (type_support, support_id)
    WHERE role = 'proprietaire' AND actif = TRUE;

CREATE INDEX IF NOT EXISTS idx_support_detenteur_support
    ON media_content.support_detenteur (type_support, support_id)
    WHERE actif = TRUE;

-- « Quels supports est-ce que je détiens ? » — page /mon-compte/mes-supports.
CREATE INDEX IF NOT EXISTS idx_support_detenteur_utilisateur
    ON media_content.support_detenteur (utilisateur_id)
    WHERE actif = TRUE;


-- ════════════════════════════════════════════════════════════════════════════
-- 3. Table invitation_detenteur
-- ════════════════════════════════════════════════════════════════════════════
-- L'invitation porte un COURRIEL et non un identifiant : on invite aussi des
-- membres qu'on ne sait pas nommer dans l'annuaire. utilisateur_invite_id est
-- résolu quand le destinataire est reconnu.

CREATE TABLE IF NOT EXISTS media_content.invitation_detenteur (
    id                    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_support          media_content.type_support_media NOT NULL,
    support_id            UUID        NOT NULL,
    email_invite          VARCHAR(255) NOT NULL,
    utilisateur_invite_id UUID,                       -- [xref] iam.utilisateur
    role                  media_content.role_detenteur NOT NULL DEFAULT 'co_detenteur',
    statut                VARCHAR(20) NOT NULL DEFAULT 'en_attente',
    invite_par            UUID        NOT NULL,       -- [xref] iam.utilisateur
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expire_at             TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '30 days'),
    traitee_le            TIMESTAMPTZ
);

ALTER TABLE media_content.invitation_detenteur
    DROP CONSTRAINT IF EXISTS ck_invitation_detenteur_statut;
ALTER TABLE media_content.invitation_detenteur
    ADD CONSTRAINT ck_invitation_detenteur_statut
        CHECK (statut IN ('en_attente', 'acceptee', 'refusee', 'expiree'));

CREATE INDEX IF NOT EXISTS idx_invitation_detenteur_destinataire
    ON media_content.invitation_detenteur (utilisateur_invite_id, statut)
    WHERE utilisateur_invite_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_invitation_detenteur_email
    ON media_content.invitation_detenteur (LOWER(email_invite), statut);

CREATE INDEX IF NOT EXISTS idx_invitation_detenteur_support
    ON media_content.invitation_detenteur (type_support, support_id);


-- ════════════════════════════════════════════════════════════════════════════
-- 4. Spécialités audiovisuelles (US6, FR-046)
-- ════════════════════════════════════════════════════════════════════════════
-- Un porteur de projet doit pouvoir trouver un réalisateur ou un producteur
-- dans l'annuaire des experts. Ces cinq métiers manquaient au référentiel semé
-- par 15_seed.sql : sans eux, le filtre par spécialité ne rend rien.

INSERT INTO iam.specialite_bibliotheque (nom, slug) VALUES
    ('Réalisateur',     'realisateur'),
    ('Producteur',      'producteur'),
    ('Cadreur',         'cadreur'),
    ('Monteur',         'monteur'),
    ('Animateur radio', 'animateur-radio')
ON CONFLICT (slug) DO NOTHING;
