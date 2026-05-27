-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : Vidafrica — Contributions membres
-- ════════════════════════════════════════════════════════════════════════════
-- Objectif : permettre à tout utilisateur connecté de proposer une vidéo
-- (état 'brouillon' → validation admin) et de contribuer des pistes de
-- sous-titres sur une vidéo publiée. La modération se fait au niveau de la
-- PISTE : une piste proposée par un membre reste 'brouillon' (invisible au
-- public) jusqu'à publication par un admin.
--
-- Idempotente : peut être rejouée sans effet de bord (bases existantes).
-- ════════════════════════════════════════════════════════════════════════════

-- ── Colonne etat sur piste_sous_titre ────────────────────────────────────────
-- DEFAULT 'publie' : les pistes existantes (créées par les admins) restent
-- visibles. Les nouvelles pistes membres sont insérées explicitement en
-- 'brouillon'.

ALTER TABLE media_content.piste_sous_titre
    ADD COLUMN IF NOT EXISTS etat VARCHAR(50) NOT NULL DEFAULT 'publie';

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'chk_piste_sous_titre_etat'
    ) THEN
        ALTER TABLE media_content.piste_sous_titre
            ADD CONSTRAINT chk_piste_sous_titre_etat
            CHECK (etat IN ('brouillon', 'publie', 'masque'));
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_piste_sous_titre_etat
    ON media_content.piste_sous_titre (etat) WHERE deleted_at IS NULL;
