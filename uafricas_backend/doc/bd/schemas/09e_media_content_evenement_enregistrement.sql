-- ─────────────────────────────────────────────────────────────────────────
-- 09e — media_content.evenement : enregistrement video (rediffusion)
-- ─────────────────────────────────────────────────────────────────────────
-- Une fois l'evenement TERMINE, l'organisateur (ou un admin) peut renseigner
-- le lien d'une rediffusion video (YouTube) qui sera affichee en lecteur
-- embarque sur la page detail de l'evenement.
-- Colonne nullable (retrocompatibilite). Migration idempotente.

ALTER TABLE media_content.evenement
    ADD COLUMN IF NOT EXISTS enregistrement_url TEXT;
