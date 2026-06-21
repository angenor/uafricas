-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : culture -- Type de centre culturel (international/local)
-- Feature : centres-culturels-international-local (2026-06-21)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Ajoute une distinction de type sur les centres culturels :
--   - 'international' : centre à rayonnement mondial
--   - 'local'        : centre ancré dans un territoire
-- La page publique /centres affiche désormais deux sections (internationaux
-- puis locaux). Colonne avec DEFAULT → rétrocompatibilité (les centres
-- existants deviennent 'local'). Idempotent.
--
-- Conformité : Principe III (SQL source de vérité).
-- ════════════════════════════════════════════════════════════════════════════

-- Enum (création conditionnelle car CREATE TYPE n'accepte pas IF NOT EXISTS)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_type t
        JOIN pg_namespace n ON n.oid = t.typnamespace
        WHERE t.typname = 'type_centre_culturel' AND n.nspname = 'culture'
    ) THEN
        CREATE TYPE culture.type_centre_culturel AS ENUM ('international', 'local');
    END IF;
END$$;

ALTER TABLE culture.centre_culturel
    ADD COLUMN IF NOT EXISTS type_centre culture.type_centre_culturel NOT NULL DEFAULT 'local';
