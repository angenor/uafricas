-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Extensions PostgreSQL requises
-- ════════════════════════════════════════════════════════════════════════════

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";   -- uuid_generate_v4()
CREATE EXTENSION IF NOT EXISTS "citext";      -- emails insensibles à la casse
CREATE EXTENSION IF NOT EXISTS "pg_trgm";     -- recherche par similarité / trigrammes
CREATE EXTENSION IF NOT EXISTS "unaccent";    -- recherche sans accents
