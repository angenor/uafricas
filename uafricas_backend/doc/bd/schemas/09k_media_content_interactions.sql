-- ============================================================================
-- 09k — media_content : interactions communautaires sur les médias
-- ----------------------------------------------------------------------------
-- Ouvre la participation sur les contenus télé et radio
-- (feature 001-refonte-tele-radio, US3 et US7) :
--
--   • media_reaction     — un « j'aime » ou « je n'aime pas » par membre et par
--                          contenu, changeable et retirable (FR-023) ;
--   • media_commentaire  — liste plate, suppression réservée à l'auteur
--                          (FR-024) ;
--   • partage_media      — alimente le mur /publications comme 8ᵉ source
--                          (FR-025) ;
--   • signalement_media  — idempotent par membre ; au-delà du seuil, le contenu
--                          bascule en etat = 'suspendu' (FR-049, FR-050).
--
-- Les quatre tables sont GÉNÉRIQUES, discriminées par (type_media, media_id).
-- Le discriminant est un VARCHAR + CHECK plutôt qu'un ENUM : il reste
-- extensible sans ALTER TYPE, à l'image de governance.partage_contribution.
-- Aucune FK ne porte sur media_id — la cible relève de quatre tables
-- distinctes ; l'intégrité est assurée à la lecture (JOIN) et à l'écriture
-- (vérification d'existence côté handler).
--
--   type_media ∈ ('chaine_tv', 'station_radio', 'programme_tele',
--                 'programme_radio')
--
-- Prérequis : 09j (colonne nombre_signalements, état 'en_attente').
--
-- Migration idempotente : CREATE TABLE / INDEX IF NOT EXISTS, DROP puis ADD
-- CONSTRAINT.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. Réactions like / dislike (FR-023)
-- ════════════════════════════════════════════════════════════════════════════
-- UNIQUE (type_media, media_id, utilisateur_id) : une seule réaction retenue
-- par membre et par contenu. Le changement d'avis se fait par UPDATE, le
-- retrait par DELETE — jamais par accumulation de lignes.

CREATE TABLE IF NOT EXISTS media_content.media_reaction (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_media     VARCHAR(20) NOT NULL,
    media_id       UUID        NOT NULL,
    utilisateur_id UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    type_reaction  VARCHAR(10) NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_media_reaction_membre UNIQUE (type_media, media_id, utilisateur_id)
);

ALTER TABLE media_content.media_reaction
    DROP CONSTRAINT IF EXISTS ck_media_reaction_type_media;
ALTER TABLE media_content.media_reaction
    ADD CONSTRAINT ck_media_reaction_type_media
        CHECK (type_media IN ('chaine_tv', 'station_radio',
                              'programme_tele', 'programme_radio'));

ALTER TABLE media_content.media_reaction
    DROP CONSTRAINT IF EXISTS ck_media_reaction_type_reaction;
ALTER TABLE media_content.media_reaction
    ADD CONSTRAINT ck_media_reaction_type_reaction
        CHECK (type_reaction IN ('like', 'dislike'));

-- Comptage des réactions d'un contenu — requête servie à chaque carte.
CREATE INDEX IF NOT EXISTS idx_media_reaction_cible
    ON media_content.media_reaction (type_media, media_id);

-- « Quelle est MA réaction ? » — résolu à l'affichage pour un membre connecté.
CREATE INDEX IF NOT EXISTS idx_media_reaction_utilisateur
    ON media_content.media_reaction (utilisateur_id);


-- ════════════════════════════════════════════════════════════════════════════
-- 2. Commentaires (FR-024)
-- ════════════════════════════════════════════════════════════════════════════
-- Liste PLATE, sans fil de réponses : FR-024 n'en demande pas, et aucun
-- précédent du projet n'en propose (modèle iam.biblio_commentaire, 04g:41-55).
-- La suppression est un soft delete réservé à l'auteur — aucune modération de
-- commentaire n'est prévue, la spec n'en exigeant pas.

CREATE TABLE IF NOT EXISTS media_content.media_commentaire (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_media VARCHAR(20) NOT NULL,
    media_id   UUID        NOT NULL,
    auteur_id  UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    contenu    TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

ALTER TABLE media_content.media_commentaire
    DROP CONSTRAINT IF EXISTS ck_media_commentaire_type_media;
ALTER TABLE media_content.media_commentaire
    ADD CONSTRAINT ck_media_commentaire_type_media
        CHECK (type_media IN ('chaine_tv', 'station_radio',
                              'programme_tele', 'programme_radio'));

ALTER TABLE media_content.media_commentaire
    DROP CONSTRAINT IF EXISTS ck_media_commentaire_longueur;
ALTER TABLE media_content.media_commentaire
    ADD CONSTRAINT ck_media_commentaire_longueur
        CHECK (char_length(btrim(contenu)) BETWEEN 1 AND 2000);

CREATE INDEX IF NOT EXISTS idx_media_commentaire_cible
    ON media_content.media_commentaire (type_media, media_id, created_at DESC)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_media_commentaire_auteur
    ON media_content.media_commentaire (auteur_id)
    WHERE deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- 3. Partages vers le mur communautaire (FR-025)
-- ════════════════════════════════════════════════════════════════════════════
-- Un même membre peut partager plusieurs fois le même contenu : chaque partage
-- est une publication datée du mur, pas un état. Aucune contrainte d'unicité,
-- contrairement aux réactions et aux signalements.

CREATE TABLE IF NOT EXISTS media_content.partage_media (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_media     VARCHAR(20) NOT NULL,
    media_id       UUID        NOT NULL,
    utilisateur_id UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    legende        TEXT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ
);

ALTER TABLE media_content.partage_media
    DROP CONSTRAINT IF EXISTS ck_partage_media_type_media;
ALTER TABLE media_content.partage_media
    ADD CONSTRAINT ck_partage_media_type_media
        CHECK (type_media IN ('chaine_tv', 'station_radio',
                              'programme_tele', 'programme_radio'));

ALTER TABLE media_content.partage_media
    DROP CONSTRAINT IF EXISTS ck_partage_media_legende;
ALTER TABLE media_content.partage_media
    ADD CONSTRAINT ck_partage_media_legende
        CHECK (legende IS NULL OR char_length(legende) <= 500);

-- Le mur trie strictement par date décroissante, toutes sources confondues.
CREATE INDEX IF NOT EXISTS idx_partage_media_mur
    ON media_content.partage_media (created_at DESC)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_partage_media_cible
    ON media_content.partage_media (type_media, media_id)
    WHERE deleted_at IS NULL;


-- ════════════════════════════════════════════════════════════════════════════
-- 4. Signalements (FR-049, FR-050)
-- ════════════════════════════════════════════════════════════════════════════
-- UNIQUE (type_media, media_id, signale_par) : un membre ne peut pas gonfler
-- le compteur en signalant plusieurs fois. L'insertion se fait en
-- ON CONFLICT DO NOTHING, puis le handler recompte les signalements DISTINCTS
-- et bascule etat = 'suspendu' au-delà du seuil (11ᵉ signalement).
--
-- Les lignes sont CONSERVÉES au rétablissement administratif : seul le
-- compteur dénormalisé nombre_signalements est remis à zéro (FR-051), afin de
-- garder l'historique sans resuspendre le contenu au signalement suivant.

CREATE TABLE IF NOT EXISTS media_content.signalement_media (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_media  VARCHAR(20) NOT NULL,
    media_id    UUID        NOT NULL,
    signale_par UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    motif       VARCHAR(50),
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_signalement_media_membre UNIQUE (type_media, media_id, signale_par)
);

ALTER TABLE media_content.signalement_media
    DROP CONSTRAINT IF EXISTS ck_signalement_media_type_media;
ALTER TABLE media_content.signalement_media
    ADD CONSTRAINT ck_signalement_media_type_media
        CHECK (type_media IN ('chaine_tv', 'station_radio',
                              'programme_tele', 'programme_radio'));

ALTER TABLE media_content.signalement_media
    DROP CONSTRAINT IF EXISTS ck_signalement_media_description;
ALTER TABLE media_content.signalement_media
    ADD CONSTRAINT ck_signalement_media_description
        CHECK (description IS NULL OR char_length(description) <= 1000);

CREATE INDEX IF NOT EXISTS idx_signalement_media_cible
    ON media_content.signalement_media (type_media, media_id);
