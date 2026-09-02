-- ═══════════════════════════════════════════════════════════════════════
-- 11l — Symboles nationaux enrichis (Afripulse)
-- ═══════════════════════════════════════════════════════════════════════
--
-- La maquette Figma de la fiche territoire (« afripulse-2 ») décrit six
-- symboles nationaux, chacun avec une NOTICE explicative :
--   drapeau, armoiries, hymne, fleur, animal et oiseau nationaux.
--
-- La table n'en portait que quatre supports d'images ou de titres
-- (`image_drapeau_url`, `image_embleme_url`, `image_devise_url`,
-- `hymne_national`) et AUCUNE notice. La fiche affichait donc un drapeau
-- sans dire ce qu'il signifie, et taisait purement fleur, animal et oiseau.
--
-- Toutes les colonnes sont NULLABLES : les 54 fiches existantes restent
-- valides, et la fiche n'affiche un symbole que s'il est renseigné — jamais
-- un libellé vide, jamais une valeur inventée.
--
-- Les trois nouveaux symboles sont du TEXTE, sans URL d'image : la maquette
-- ne leur en montre pas. Une colonne d'image non utilisée serait une dette.
-- ═══════════════════════════════════════════════════════════════════════

ALTER TABLE country_profile.fiche_pays
    ADD COLUMN IF NOT EXISTS drapeau_description  TEXT,
    ADD COLUMN IF NOT EXISTS embleme_description  TEXT,
    ADD COLUMN IF NOT EXISTS hymne_description    TEXT,
    ADD COLUMN IF NOT EXISTS fleur_nationale      VARCHAR(250),
    ADD COLUMN IF NOT EXISTS fleur_description    TEXT,
    ADD COLUMN IF NOT EXISTS animal_national      VARCHAR(250),
    ADD COLUMN IF NOT EXISTS animal_description   TEXT,
    ADD COLUMN IF NOT EXISTS oiseau_national      VARCHAR(250),
    ADD COLUMN IF NOT EXISTS oiseau_description   TEXT;

COMMENT ON COLUMN country_profile.fiche_pays.drapeau_description IS
    'Notice du drapeau (date d''adoption, symbolique des couleurs).';
COMMENT ON COLUMN country_profile.fiche_pays.embleme_description IS
    'Notice des armoiries (date d''adoption, éléments représentés).';
COMMENT ON COLUMN country_profile.fiche_pays.hymne_description IS
    'Notice de l''hymne national ; `hymne_national` en porte le titre.';
COMMENT ON COLUMN country_profile.fiche_pays.fleur_nationale IS
    'Nom de la fleur nationale (nom commun, éventuellement nom savant).';
COMMENT ON COLUMN country_profile.fiche_pays.animal_national IS
    'Nom de l''animal national.';
COMMENT ON COLUMN country_profile.fiche_pays.oiseau_national IS
    'Nom de l''oiseau national.';
