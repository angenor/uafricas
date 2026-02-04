-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : shared — Journal d'audit (transversal)
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE shared.audit_log (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID,                            -- [xref] iam.utilisateur (nullable = système)
    action          VARCHAR(60)  NOT NULL,            -- 'CREATE', 'UPDATE', 'DELETE', 'LOGIN'…
    schema_name     VARCHAR(60)  NOT NULL,
    table_name      VARCHAR(120) NOT NULL,
    record_id       UUID,
    ancien_etat     JSONB,                           -- snapshot avant modification
    nouvel_etat     JSONB,                           -- snapshot après modification
    ip_address      INET,
    user_agent      TEXT,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_utilisateur ON shared.audit_log(utilisateur_id);
CREATE INDEX idx_audit_table       ON shared.audit_log(schema_name, table_name);
CREATE INDEX idx_audit_record      ON shared.audit_log(record_id);
CREATE INDEX idx_audit_date        ON shared.audit_log(created_at);
