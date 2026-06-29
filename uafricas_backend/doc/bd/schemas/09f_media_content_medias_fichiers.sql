-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : médias par fichiers/liens (radio & télé)
-- ════════════════════════════════════════════════════════════════════════════
-- Refonte du concept radio/télé : on abandonne le « live streaming » obligatoire
-- au profit de FICHIERS (uploadés dans le back-office) ou de LIENS vidéo/audio.
--
--  • station_radio : le flux live (`stream_url`) devient OPTIONNEL ; une station
--    porte désormais un `audio_url` (fichier/lien audio joué dans le lecteur) et
--    peut être marquée « à la une » (`a_la_une`) pour être mise en avant sur sa
--    page publique (Radios Africans / Nationales).
--  • chaine_tv : le flux live (`stream_url`) devient OPTIONNEL (le cœur = les
--    programmes télé avec leur vidéo, cf. migration 09d).
--
-- Migration idempotente.
-- ════════════════════════════════════════════════════════════════════════════

-- ── Station radio ─────────────────────────────────────────────────────────
ALTER TABLE media_content.station_radio
    ALTER COLUMN stream_url DROP NOT NULL;

ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS audio_url VARCHAR(500);   -- fichier (/uploads/...) ou lien externe

ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS a_la_une BOOLEAN NOT NULL DEFAULT FALSE;

-- Index pour remonter rapidement les stations à la une (parmi les non supprimées)
CREATE INDEX IF NOT EXISTS idx_station_a_la_une
    ON media_content.station_radio(a_la_une) WHERE deleted_at IS NULL;

-- ── Chaîne TV ─────────────────────────────────────────────────────────────
ALTER TABLE media_content.chaine_tv
    ALTER COLUMN stream_url DROP NOT NULL;
