-- ============================================================================
-- 11j — country_profile : signalement des CONTRIBUTIONS individuelles (afripulse)
-- ----------------------------------------------------------------------------
-- Remplace le signalement « fiche pays entière » par un signalement ciblé sur
-- chaque objet contribué (recette, site touristique, personnalité, secteur,
-- savoir pratique). Au-delà de 10 signalements distincts, l'objet est
-- automatiquement SUSPENDU (reste visible avec un bandeau, mais figé) jusqu'à
-- réactivation par un admin.
--
-- Une seule table générique clé (type_objet, objet_id, signale_par), calquée
-- sur governance.factcheck_signalement. Chaque table d'objet reçoit les deux
-- colonnes dénormalisées nombre_signalements + suspendu.
--
-- Migration idempotente.
-- ============================================================================

-- 1) Compteur + état de suspension sur chaque objet signalable ----------------
ALTER TABLE country_profile.site_touristique
    ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS suspendu BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE country_profile.secteur_developpement
    ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS suspendu BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE country_profile.recette_culinaire
    ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS suspendu BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE country_profile.personnalite_connue
    ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS suspendu BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE country_profile.savoir_pratique
    ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS suspendu BOOLEAN NOT NULL DEFAULT FALSE;

-- 2) Table générique des signalements de contributions -----------------------
-- type_objet réutilise l'enum afripulse existant ; seuls les 5 types ci-dessus
-- sont effectivement signalables (contrôle applicatif côté backend).
CREATE TABLE IF NOT EXISTS country_profile.signalement_contribution (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    type_objet  country_profile.type_objet_contribution NOT NULL,
    objet_id    UUID        NOT NULL,
    signale_par UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    motif       VARCHAR(50),                     -- catégorie facultative
    description TEXT CHECK (description IS NULL OR char_length(description) <= 1000),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (type_objet, objet_id, signale_par)   -- un signalement par membre et par objet
);

CREATE INDEX IF NOT EXISTS idx_signalement_contribution_objet
    ON country_profile.signalement_contribution (type_objet, objet_id);
