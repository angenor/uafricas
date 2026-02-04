-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Fonctions utilitaires
-- ════════════════════════════════════════════════════════════════════════════

-- Mise à jour automatique de updated_at sur chaque UPDATE
CREATE OR REPLACE FUNCTION shared.trigger_set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
