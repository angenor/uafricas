-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : Vidafrica — Infos de proposition (territoires,
--                  décharge de droits, auteur réel)
-- ════════════════════════════════════════════════════════════════════════════
-- Objectif : enrichir une vidéo proposée par un membre avec :
--   - les territoires concernés (liste, ex. pays d'origine de la chanson),
--   - une décharge de droits (« Je ne suis pas l'auteur de cette chanson et ne
--     revendique aucun droit à ce sujet ») acceptée à la proposition,
--   - le nom de l'auteur réel (facultatif, à préciser le cas échéant).
--
-- Colonnes nullables / DEFAULT → rétrocompatibles avec les vidéos existantes.
-- Idempotente : peut être rejouée sans effet de bord.
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE media_content.video
    ADD COLUMN IF NOT EXISTS territoires       TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS decharge_droits   BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN IF NOT EXISTS auteur_reel       TEXT;
