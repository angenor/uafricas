-- ============================================================================
-- 09o — media_content : origine de publication des chaînes TV
-- ----------------------------------------------------------------------------
-- Pendant télé de `station_radio.origine_publication` (09j:41), introduit pour
-- la barre de filtres de `/medias/tele` :
--
--   'africans'   → « Africans Télé International » — chaînes produites par la
--                  plateforme elle-même, décision éditoriale de ses créateurs ;
--   'territoire' → chaîne rattachée à un territoire africain.
--
-- Contrairement à la radio, l'origine ne sépare PAS deux pages : les deux
-- familles cohabitent sur `/medias/tele`, où « Africans Télé International »
-- n'est qu'un filtre. Aucune chaîne « africans » n'existe encore — elles seront
-- créées depuis le back-office, d'où le défaut 'territoire' qui qualifie
-- correctement tout l'existant.
--
-- Migration idempotente : ADD COLUMN IF NOT EXISTS, DROP puis ADD CONSTRAINT,
-- CREATE INDEX IF NOT EXISTS.
-- ============================================================================

ALTER TABLE media_content.chaine_tv
    ADD COLUMN IF NOT EXISTS origine_publication VARCHAR(20) NOT NULL DEFAULT 'territoire';

ALTER TABLE media_content.chaine_tv
    DROP CONSTRAINT IF EXISTS ck_chaine_tv_origine;
ALTER TABLE media_content.chaine_tv
    ADD CONSTRAINT ck_chaine_tv_origine
        CHECK (origine_publication IN ('africans', 'territoire'));

-- Le filtre porte toujours sur des chaînes vivantes : l'index partiel suffit.
CREATE INDEX IF NOT EXISTS idx_chaine_tv_origine
    ON media_content.chaine_tv (origine_publication) WHERE deleted_at IS NULL;

-- Le filtre « chaînes thématiques » remonte les chaînes par le thème phare de
-- leurs programmes publiés : sans cet index, chaque chargement de sections
-- balaie programme_tele en entier.
CREATE INDEX IF NOT EXISTS idx_programme_tele_theme_phare
    ON media_content.programme_tele (theme_phare_id, chaine_id)
    WHERE theme_phare_id IS NOT NULL AND deleted_at IS NULL;

COMMENT ON COLUMN media_content.chaine_tv.origine_publication IS
    'africans = chaîne produite par la plateforme (filtre « Africans Télé International ») ; territoire = chaîne rattachée à un territoire.';
