-- ════════════════════════════════════════════════════════════════════════════
-- 27e — Vidafrica : pistes de sous-titres collaboratives
-- ────────────────────────────────────────────────────────────────────────────
-- Plusieurs membres peuvent proposer une piste dans la MÊME langue (une par auteur),
-- mais une seule piste PUBLIÉE par langue (l'admin arbitre). L'auteur peut éditer
-- sa piste en direct, même après publication.
-- Migration idempotente.
-- ════════════════════════════════════════════════════════════════════════════

-- Ancienne contrainte : une seule piste par (vidéo, langue) tous auteurs/états confondus.
DROP INDEX IF EXISTS media_content.idx_piste_sous_titre_unique_langue;

-- Unicité par auteur : un membre ne peut avoir qu'une piste par (vidéo, langue),
-- mais d'autres membres peuvent proposer la même langue de leur côté.
CREATE UNIQUE INDEX IF NOT EXISTS idx_piste_sous_titre_unique_langue_auteur
    ON media_content.piste_sous_titre (video_id, langue, cree_par)
    WHERE deleted_at IS NULL;

-- Au plus une piste PUBLIÉE par (vidéo, langue) : l'affichage public reste non ambigu.
CREATE UNIQUE INDEX IF NOT EXISTS idx_piste_sous_titre_unique_publie
    ON media_content.piste_sous_titre (video_id, langue)
    WHERE etat = 'publie' AND deleted_at IS NULL;
