-- ============================================================================
-- 11k — country_profile : réactions & partages communautaires des SOUS-OBJETS
--       afripulse (secteur, recette, site touristique, personnalité)
-- ----------------------------------------------------------------------------
-- Généralise à tous les sous-objets d'une fiche pays ce que 11h a fait pour la
-- fiche elle-même :
--   • aimer / ne pas aimer un sous-objet (like/dislike binaire, une réaction
--     par utilisateur, bascule on/off — modèle country_profile.reaction_fiche) ;
--   • partager un sous-objet dans le mur communautaire « /publications » avec
--     une légende facultative (nouvelle source agrégée du mur).
--
-- Le signalement de ces sous-objets existe déjà (11j — signalement_contribution).
--
-- Design GÉNÉRIQUE : clés (type_objet, objet_id) plutôt qu'une table par type.
-- `type_objet` réutilise l'enum existant country_profile.type_objet_contribution
-- (valeurs concernées : 'secteur_developpement', 'recette_culinaire',
-- 'site_touristique', 'personnalite_connue'). Pas de FK possible (objet_id
-- pointe vers 4 tables distinctes) : l'intégrité est assurée applicativement.
--
-- Migration idempotente (tables IF NOT EXISTS).
-- ============================================================================

-- 1) Réactions like / dislike (une par utilisateur et par sous-objet) ---------
CREATE TABLE IF NOT EXISTS country_profile.reaction_element (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type_objet     country_profile.type_objet_contribution NOT NULL,
    objet_id       UUID NOT NULL,
    utilisateur_id UUID NOT NULL,                       -- [xref] iam.utilisateur
    type_reaction  VARCHAR(10) NOT NULL CHECK (type_reaction IN ('like', 'dislike')),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (type_objet, objet_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_reaction_element_objet
    ON country_profile.reaction_element (type_objet, objet_id);

-- 2) Partages vers le mur communautaire (avec légende facultative) ------------
CREATE TABLE IF NOT EXISTS country_profile.partage_element (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type_objet     country_profile.type_objet_contribution NOT NULL,
    objet_id       UUID NOT NULL,
    utilisateur_id UUID NOT NULL,                       -- [xref] iam.utilisateur
    legende        TEXT CHECK (legende IS NULL OR char_length(legende) <= 500),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ                          -- soft delete
);

CREATE INDEX IF NOT EXISTS idx_partage_element_actifs
    ON country_profile.partage_element (created_at DESC)
    WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_partage_element_objet
    ON country_profile.partage_element (type_objet, objet_id)
    WHERE deleted_at IS NULL;
