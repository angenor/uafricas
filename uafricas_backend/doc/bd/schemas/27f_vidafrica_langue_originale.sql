-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : Vidafrica — Langue initiale de la vidéo
-- ════════════════════════════════════════════════════════════════════════════
-- Objectif : permettre au contributeur de préciser, à la soumission, la langue
--   parlée/chantée dans la vidéo (« langue initiale »). Les propositions par
--   défaut côté frontend proviennent des langues Afrolang ; l'utilisateur peut
--   saisir librement une autre langue si elle n'y figure pas.
--
-- Texte libre (VARCHAR) plutôt que l'enum `langue_sous_titre` : ce dernier est
--   fermé (15 langues), alors que la langue d'origine doit rester ouverte.
-- Colonne nullable → rétrocompatible avec les vidéos existantes.
-- Idempotente : peut être rejouée sans effet de bord.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE media_content.video
    ADD COLUMN IF NOT EXISTS langue_originale VARCHAR(80);

COMMENT ON COLUMN media_content.video.langue_originale IS
    'Langue parlée/chantée dans la vidéo, saisie à la soumission. Texte libre (langues Afrolang + « Autre »).';
