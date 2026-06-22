-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : programmes télé rattachés à une chaîne + à la une
-- ════════════════════════════════════════════════════════════════════════════
-- Une « télé » = une chaîne TV (media_content.chaine_tv). Chaque vidéo/programme
-- télé (media_content.programme_radio_tele, type='tele') peut être rattaché à une
-- chaîne via `chaine_id`. Au sein d'une chaîne, un seul programme peut être marqué
-- « à la une » (`a_la_une`) : c'est lui qui joue en boucle sur l'écran principal
-- de la page /tele. Migration idempotente.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE media_content.programme_radio_tele
    ADD COLUMN IF NOT EXISTS chaine_id UUID;          -- [xref] media_content.chaine_tv

ALTER TABLE media_content.programme_radio_tele
    ADD COLUMN IF NOT EXISTS a_la_une BOOLEAN NOT NULL DEFAULT FALSE;

-- Index de rattachement programme → chaîne
CREATE INDEX IF NOT EXISTS idx_radio_tele_chaine
    ON media_content.programme_radio_tele(chaine_id) WHERE deleted_at IS NULL;

-- Un seul programme « à la une » par chaîne (parmi les programmes télé non supprimés)
CREATE UNIQUE INDEX IF NOT EXISTS uq_programme_a_la_une_par_chaine
    ON media_content.programme_radio_tele(chaine_id)
    WHERE a_la_une = TRUE AND deleted_at IS NULL AND type = 'tele';
