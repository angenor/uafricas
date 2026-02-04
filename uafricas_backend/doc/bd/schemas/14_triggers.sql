-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Triggers : mise à jour automatique de updated_at
-- ════════════════════════════════════════════════════════════════════════════

DO $$
DECLARE
    t RECORD;
BEGIN
    FOR t IN
        SELECT schemaname, tablename
        FROM pg_tables
        WHERE schemaname IN (
            'shared','iam','marketplace','exchange',
            'innovation','culture','media_content',
            'governance','country_profile'
        )
        AND tablename NOT IN ('audit_log')
    LOOP
        -- Vérifie que la table a bien une colonne updated_at
        IF EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_schema = t.schemaname
              AND table_name   = t.tablename
              AND column_name  = 'updated_at'
        ) THEN
            EXECUTE format(
                'CREATE TRIGGER trg_%I_updated_at
                 BEFORE UPDATE ON %I.%I
                 FOR EACH ROW
                 EXECUTE FUNCTION shared.trigger_set_updated_at()',
                t.tablename, t.schemaname, t.tablename
            );
        END IF;
    END LOOP;
END;
$$;
