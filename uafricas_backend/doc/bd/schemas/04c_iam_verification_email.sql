-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD -- Schema : iam -- Tokens de verification email
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE iam.token_verification_email (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    token_hash      VARCHAR(255) NOT NULL UNIQUE,
    expire_at       TIMESTAMPTZ  NOT NULL,
    utilise         BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_token_verif_email_user ON iam.token_verification_email(utilisateur_id)
    WHERE utilise = FALSE;
