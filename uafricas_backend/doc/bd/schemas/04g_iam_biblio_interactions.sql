-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : iam — Interactions sur les bibliothèques humaines
-- ════════════════════════════════════════════════════════════════════════════
-- Dépend de : 04_iam.sql (iam.utilisateur), 04b_iam_biblio_demande.sql
--
-- Une bibliothèque humaine = un utilisateur dont une demande est en statut
-- 'valide' (cf. obtenir_biblio, indexé par utilisateur_id). Tout membre connecté
-- peut, depuis la page profil :
--   • aimer / ne pas aimer (réaction unique, modifiable, basculable) ;
--   • laisser un commentaire (liste plate, livre d'or) ;
--   • recommander avec un témoignage facultatif (une recommandation par membre,
--     modifiable, retirable).
-- La cible est référencée par utilisateur_id (la biblio = l'utilisateur validé),
-- stable même après re-validation d'une nouvelle demande.
-- Aucun compteur dénormalisé : les totaux sont calculés à la lecture.
-- Migration idempotente.


-- ── Table biblio_reaction (like / dislike) ──────────────────────────────────

CREATE TABLE IF NOT EXISTS iam.biblio_reaction (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID        NOT NULL          -- biblio ciblée (= utilisateur validé)
                    REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    auteur_id       UUID        NOT NULL          -- membre qui réagit
                    REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    type_reaction   VARCHAR(10) NOT NULL CHECK (type_reaction IN ('like', 'dislike')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Une seule réaction par (auteur, biblio) : l'upsert / toggle s'appuie dessus
CREATE UNIQUE INDEX IF NOT EXISTS uniq_biblio_reaction
    ON iam.biblio_reaction (utilisateur_id, auteur_id);
CREATE INDEX IF NOT EXISTS idx_biblio_reaction_cible
    ON iam.biblio_reaction (utilisateur_id);


-- ── Table biblio_commentaire (liste plate) ──────────────────────────────────

CREATE TABLE IF NOT EXISTS iam.biblio_commentaire (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID        NOT NULL          -- biblio ciblée
                    REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    auteur_id       UUID        NOT NULL          -- membre qui commente
                    REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    contenu         TEXT        NOT NULL CHECK (char_length(contenu) BETWEEN 1 AND 2000),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_biblio_commentaire_cible
    ON iam.biblio_commentaire (utilisateur_id)
    WHERE deleted_at IS NULL;


-- ── Table biblio_recommandation (endossement avec témoignage) ───────────────

CREATE TABLE IF NOT EXISTS iam.biblio_recommandation (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id  UUID        NOT NULL          -- biblio ciblée
                    REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    auteur_id       UUID        NOT NULL          -- membre qui recommande
                    REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    message         TEXT        CHECK (message IS NULL OR char_length(message) <= 1000),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Une seule recommandation par (auteur, biblio) : upsert ON CONFLICT
CREATE UNIQUE INDEX IF NOT EXISTS uniq_biblio_recommandation
    ON iam.biblio_recommandation (utilisateur_id, auteur_id);
CREATE INDEX IF NOT EXISTS idx_biblio_recommandation_cible
    ON iam.biblio_recommandation (utilisateur_id);
