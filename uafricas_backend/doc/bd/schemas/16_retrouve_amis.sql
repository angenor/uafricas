-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : retrouve_amis — Retrouver des amis perdus de vue
-- ════════════════════════════════════════════════════════════════════════════
--
-- Bounded context : système de recoupement intelligent pour retrouver des amis.
-- Algorithme de matching basé sur pg_trgm (similarité trigrammes) + unaccent
-- (noms africains avec accents) + chevauchement de période.
--
-- Dépendances cross-schema : iam.utilisateur, shared.pays
-- ════════════════════════════════════════════════════════════════════════════


-- ── Schema ─────────────────────────────────────────────────────────────────

CREATE SCHEMA IF NOT EXISTS retrouve_amis;


-- ── Types (Enums) ──────────────────────────────────────────────────────────

CREATE TYPE retrouve_amis.etat_avis AS ENUM ('actif', 'cloture', 'suspendu');

CREATE TYPE retrouve_amis.etat_correspondance AS ENUM (
    'en_attente', 'acceptee_a', 'acceptee_b', 'mutuelle', 'declinee', 'archivee'
);

CREATE TYPE retrouve_amis.type_cible AS ENUM ('avis', 'profil');

CREATE TYPE retrouve_amis.motif_signalement AS ENUM (
    'contenu_abusif', 'usurpation_identite', 'harcelement', 'autre'
);

CREATE TYPE retrouve_amis.etat_signalement AS ENUM ('en_attente', 'approuve', 'rejete');

CREATE TYPE retrouve_amis.type_parcours AS ENUM ('ecole', 'ville_residence');

CREATE TYPE retrouve_amis.type_notification AS ENUM (
    'nouvelle_correspondance', 'acceptation_contact',
    'coordonnees_partagees', 'correspondance_archivee',
    'avis_suspendu'
);


-- ── Table : avis_recherche ─────────────────────────────────────────────────

CREATE TABLE retrouve_amis.avis_recherche (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    auteur_id           UUID                            NOT NULL,   -- [xref] iam.utilisateur
    nom_recherche       VARCHAR(100)                    NOT NULL,
    prenom_recherche    VARCHAR(100),
    surnom              VARCHAR(100),
    ecole               VARCHAR(250),
    ville               VARCHAR(200),
    pays_id             UUID,                                       -- [xref] shared.pays
    periode_debut       INT                             CHECK (periode_debut >= 1900 AND periode_debut <= 2100),
    periode_fin         INT                             CHECK (periode_fin >= 1900 AND periode_fin <= 2100),
    description         TEXT,
    etat                retrouve_amis.etat_avis         NOT NULL DEFAULT 'actif',
    search_vector       TSVECTOR,
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,

    CONSTRAINT chk_avis_periode CHECK (
        periode_debut IS NULL OR periode_fin IS NULL OR periode_debut <= periode_fin
    )
);

-- Index B-tree avec filtres partiels
CREATE INDEX idx_avis_recherche_auteur ON retrouve_amis.avis_recherche(auteur_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_avis_recherche_etat ON retrouve_amis.avis_recherche(etat) WHERE deleted_at IS NULL;

-- Index GIN full-text search
CREATE INDEX idx_avis_recherche_fts ON retrouve_amis.avis_recherche USING GIN(search_vector);

-- Index GIN trigramme pour recherche par similarité sur le nom
CREATE INDEX idx_avis_recherche_nom ON retrouve_amis.avis_recherche USING GIN(nom_recherche gin_trgm_ops) WHERE deleted_at IS NULL;

-- Trigger tsvector_update_trigger pour populer search_vector
CREATE TRIGGER trg_avis_recherche_search_vector
    BEFORE INSERT OR UPDATE ON retrouve_amis.avis_recherche
    FOR EACH ROW EXECUTE FUNCTION
    tsvector_update_trigger(search_vector, 'pg_catalog.french', nom_recherche, prenom_recherche, surnom, ecole, ville, description);


-- ── Table : correspondance ─────────────────────────────────────────────────

CREATE TABLE retrouve_amis.correspondance (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    avis_id                 UUID                                NOT NULL REFERENCES retrouve_amis.avis_recherche(id) ON DELETE CASCADE,
    type_cible              retrouve_amis.type_cible            NOT NULL,
    cible_avis_id           UUID                                REFERENCES retrouve_amis.avis_recherche(id) ON DELETE CASCADE,
    cible_utilisateur_id    UUID,                                                       -- [xref] iam.utilisateur
    score                   NUMERIC(5,2)                        NOT NULL CHECK (score >= 0 AND score <= 100),
    details_score           JSONB                               NOT NULL DEFAULT '{}',
    etat                    retrouve_amis.etat_correspondance   NOT NULL DEFAULT 'en_attente',
    accepte_par_a_at        TIMESTAMPTZ,
    accepte_par_b_at        TIMESTAMPTZ,
    coordonnees_a           JSONB,
    coordonnees_b           JSONB,
    expire_at               TIMESTAMPTZ,
    created_at              TIMESTAMPTZ                         NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ                         NOT NULL DEFAULT NOW(),

    -- Au moins une cible doit être renseignée
    CONSTRAINT chk_correspondance_cible_non_null
        CHECK (cible_avis_id IS NOT NULL OR cible_utilisateur_id IS NOT NULL),

    -- Pas les deux cibles en même temps
    CONSTRAINT chk_correspondance_cible_exclusive
        CHECK (NOT (cible_avis_id IS NOT NULL AND cible_utilisateur_id IS NOT NULL)),

    -- Cohérence type/cible
    CONSTRAINT chk_correspondance_type_cible
        CHECK (
            (type_cible = 'avis' AND cible_avis_id IS NOT NULL) OR
            (type_cible = 'profil' AND cible_utilisateur_id IS NOT NULL)
        )
);

-- Index B-tree
CREATE INDEX idx_correspondance_avis ON retrouve_amis.correspondance(avis_id);
CREATE INDEX idx_correspondance_cible_avis ON retrouve_amis.correspondance(cible_avis_id) WHERE cible_avis_id IS NOT NULL;
CREATE INDEX idx_correspondance_cible_user ON retrouve_amis.correspondance(cible_utilisateur_id) WHERE cible_utilisateur_id IS NOT NULL;
CREATE INDEX idx_correspondance_etat ON retrouve_amis.correspondance(etat);
CREATE INDEX idx_correspondance_expire ON retrouve_amis.correspondance(expire_at) WHERE etat = 'en_attente';

-- Unicité : empêcher les doublons de correspondance active
CREATE UNIQUE INDEX idx_correspondance_unique_avis
    ON retrouve_amis.correspondance(avis_id, cible_avis_id)
    WHERE cible_avis_id IS NOT NULL AND etat NOT IN ('declinee', 'archivee');

CREATE UNIQUE INDEX idx_correspondance_unique_profil
    ON retrouve_amis.correspondance(avis_id, cible_utilisateur_id)
    WHERE cible_utilisateur_id IS NOT NULL AND etat NOT IN ('declinee', 'archivee');


-- ── Table : parcours_trouvable ─────────────────────────────────────────────

CREATE TABLE retrouve_amis.parcours_trouvable (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id      UUID                            NOT NULL,   -- [xref] iam.utilisateur
    type_entree         retrouve_amis.type_parcours     NOT NULL,
    nom                 VARCHAR(250)                    NOT NULL,
    ville               VARCHAR(200),
    pays_id             UUID,                                       -- [xref] shared.pays
    periode_debut       INT                             CHECK (periode_debut >= 1900 AND periode_debut <= 2100),
    periode_fin         INT                             CHECK (periode_fin >= 1900 AND periode_fin <= 2100),
    created_at          TIMESTAMPTZ                     NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ                     NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_parcours_periode CHECK (
        periode_debut IS NULL OR periode_fin IS NULL OR periode_debut <= periode_fin
    )
);

-- Index B-tree
CREATE INDEX idx_parcours_utilisateur ON retrouve_amis.parcours_trouvable(utilisateur_id);

-- Index GIN trigramme pour matching
CREATE INDEX idx_parcours_nom ON retrouve_amis.parcours_trouvable USING GIN(nom gin_trgm_ops);


-- ── Table : blacklist ──────────────────────────────────────────────────────

CREATE TABLE retrouve_amis.blacklist (
    utilisateur_a_id    UUID            NOT NULL,   -- [xref] iam.utilisateur (le plus petit UUID)
    utilisateur_b_id    UUID            NOT NULL,   -- [xref] iam.utilisateur (le plus grand UUID)
    correspondance_id   UUID            REFERENCES retrouve_amis.correspondance(id) ON DELETE SET NULL,
    created_at          TIMESTAMPTZ     NOT NULL DEFAULT NOW(),

    PRIMARY KEY (utilisateur_a_id, utilisateur_b_id),

    -- Garantit que la paire est stockée dans un ordre canonique
    CONSTRAINT chk_blacklist_ordre CHECK (utilisateur_a_id < utilisateur_b_id)
);


-- ── Table : signalement ────────────────────────────────────────────────────

CREATE TABLE retrouve_amis.signalement (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    avis_id         UUID                                NOT NULL REFERENCES retrouve_amis.avis_recherche(id) ON DELETE CASCADE,
    signale_par     UUID                                NOT NULL,   -- [xref] iam.utilisateur
    motif           retrouve_amis.motif_signalement     NOT NULL,
    description     TEXT,
    etat            retrouve_amis.etat_signalement      NOT NULL DEFAULT 'en_attente',
    modere_par      UUID,                                           -- [xref] iam.utilisateur
    modere_at       TIMESTAMPTZ,
    created_at      TIMESTAMPTZ                         NOT NULL DEFAULT NOW()
);

-- Index B-tree
CREATE INDEX idx_signalement_avis ON retrouve_amis.signalement(avis_id);
CREATE INDEX idx_signalement_etat ON retrouve_amis.signalement(etat) WHERE etat = 'en_attente';

-- Unicité : un utilisateur ne peut signaler le même avis qu'une fois
CREATE UNIQUE INDEX idx_signalement_unique ON retrouve_amis.signalement(avis_id, signale_par);


-- ── Table : notification_retrouve ──────────────────────────────────────────

CREATE TABLE retrouve_amis.notification_retrouve (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id      UUID                                NOT NULL,   -- [xref] iam.utilisateur
    correspondance_id   UUID                                REFERENCES retrouve_amis.correspondance(id) ON DELETE CASCADE,
    type                retrouve_amis.type_notification     NOT NULL,
    lu                  BOOLEAN                             NOT NULL DEFAULT FALSE,
    created_at          TIMESTAMPTZ                         NOT NULL DEFAULT NOW()
);

-- Index B-tree pour les requêtes par utilisateur (notifications non lues)
CREATE INDEX idx_notification_utilisateur ON retrouve_amis.notification_retrouve(utilisateur_id, lu) WHERE lu = FALSE;
CREATE INDEX idx_notification_created ON retrouve_amis.notification_retrouve(created_at);


-- ── Colonne est_trouvable sur iam.utilisateur ──────────────────────────────

ALTER TABLE iam.utilisateur ADD COLUMN est_trouvable BOOLEAN NOT NULL DEFAULT FALSE;


-- ════════════════════════════════════════════════════════════════════════════
-- Fonction de matching : calculer_correspondances
-- ════════════════════════════════════════════════════════════════════════════
--
-- Calcule les correspondances pour un avis donné.
-- Deux branches : 1) avis↔avis, 2) avis↔profil trouvable.
-- Score multi-critères (100 pts max) :
--   - Nom/prénom 40 pts (pg_trgm + unaccent)
--   - École 20 pts (similarité trigramme)
--   - Ville 15 pts (similarité trigramme)
--   - Période 15 pts (chevauchement temporel)
--   - Pays 10 pts (correspondance exacte)
-- ════════════════════════════════════════════════════════════════════════════

CREATE OR REPLACE FUNCTION retrouve_amis.calculer_correspondances(p_avis_id UUID)
RETURNS TABLE(
    cible_type      retrouve_amis.type_cible,
    cible_id        UUID,
    score_total     NUMERIC(5,2),
    details         JSONB
) AS $$
DECLARE
    v_avis RECORD;
    v_auteur_id UUID;
BEGIN
    -- Charger l'avis source
    SELECT * INTO v_avis FROM retrouve_amis.avis_recherche WHERE id = p_avis_id AND deleted_at IS NULL;
    IF NOT FOUND THEN
        RETURN;
    END IF;
    v_auteur_id := v_avis.auteur_id;

    -- ── Branche 1 : avis ↔ avis ───────────────────────────────────────────
    RETURN QUERY
    SELECT
        'avis'::retrouve_amis.type_cible AS cible_type,
        a2.id AS cible_id,
        (
            -- Score nom (40 pts) : max entre similarités directes et croisées
            GREATEST(
                similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(a2.nom_recherche))),
                COALESCE(similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(a2.prenom_recherche))), 0),
                COALESCE(similarity(unaccent(lower(COALESCE(v_avis.prenom_recherche, ''))), unaccent(lower(a2.nom_recherche))), 0),
                CASE
                    WHEN v_avis.prenom_recherche IS NOT NULL AND a2.prenom_recherche IS NOT NULL THEN
                        similarity(
                            unaccent(lower(v_avis.nom_recherche || ' ' || v_avis.prenom_recherche)),
                            unaccent(lower(a2.nom_recherche || ' ' || a2.prenom_recherche))
                        )
                    ELSE 0
                END
            ) * 40
            -- Score école (20 pts)
            + CASE
                WHEN v_avis.ecole IS NOT NULL AND a2.ecole IS NOT NULL THEN
                    similarity(unaccent(lower(v_avis.ecole)), unaccent(lower(a2.ecole))) * 20
                ELSE 0
              END
            -- Score ville (15 pts)
            + CASE
                WHEN v_avis.ville IS NOT NULL AND a2.ville IS NOT NULL THEN
                    similarity(unaccent(lower(v_avis.ville)), unaccent(lower(a2.ville))) * 15
                ELSE 0
              END
            -- Score période (15 pts) : ratio de chevauchement
            + CASE
                WHEN v_avis.periode_debut IS NOT NULL AND a2.periode_debut IS NOT NULL THEN
                    GREATEST(0,
                        LEAST(
                            COALESCE(v_avis.periode_fin, v_avis.periode_debut),
                            COALESCE(a2.periode_fin, a2.periode_debut)
                        )
                        - GREATEST(v_avis.periode_debut, a2.periode_debut)
                    )::NUMERIC / GREATEST(1,
                        GREATEST(
                            COALESCE(v_avis.periode_fin, v_avis.periode_debut),
                            COALESCE(a2.periode_fin, a2.periode_debut)
                        )
                        - LEAST(v_avis.periode_debut, a2.periode_debut)
                    ) * 15
                ELSE 0
              END
            -- Score pays (10 pts) : correspondance exacte
            + CASE
                WHEN v_avis.pays_id IS NOT NULL AND a2.pays_id IS NOT NULL AND v_avis.pays_id = a2.pays_id THEN 10
                ELSE 0
              END
        )::NUMERIC(5,2) AS score_total,
        jsonb_build_object(
            'nom', GREATEST(
                similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(a2.nom_recherche))),
                COALESCE(similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(a2.prenom_recherche))), 0),
                COALESCE(similarity(unaccent(lower(COALESCE(v_avis.prenom_recherche, ''))), unaccent(lower(a2.nom_recherche))), 0)
            ) * 40,
            'ecole', CASE
                WHEN v_avis.ecole IS NOT NULL AND a2.ecole IS NOT NULL THEN
                    similarity(unaccent(lower(v_avis.ecole)), unaccent(lower(a2.ecole))) * 20
                ELSE 0
            END,
            'ville', CASE
                WHEN v_avis.ville IS NOT NULL AND a2.ville IS NOT NULL THEN
                    similarity(unaccent(lower(v_avis.ville)), unaccent(lower(a2.ville))) * 15
                ELSE 0
            END,
            'periode', CASE
                WHEN v_avis.periode_debut IS NOT NULL AND a2.periode_debut IS NOT NULL THEN
                    GREATEST(0,
                        LEAST(
                            COALESCE(v_avis.periode_fin, v_avis.periode_debut),
                            COALESCE(a2.periode_fin, a2.periode_debut)
                        )
                        - GREATEST(v_avis.periode_debut, a2.periode_debut)
                    )::NUMERIC / GREATEST(1,
                        GREATEST(
                            COALESCE(v_avis.periode_fin, v_avis.periode_debut),
                            COALESCE(a2.periode_fin, a2.periode_debut)
                        )
                        - LEAST(v_avis.periode_debut, a2.periode_debut)
                    ) * 15
                ELSE 0
            END,
            'pays', CASE
                WHEN v_avis.pays_id IS NOT NULL AND a2.pays_id IS NOT NULL AND v_avis.pays_id = a2.pays_id THEN 10
                ELSE 0
            END
        ) AS details
    FROM retrouve_amis.avis_recherche a2
    WHERE a2.id != p_avis_id
      AND a2.auteur_id != v_auteur_id
      AND a2.etat = 'actif'
      AND a2.deleted_at IS NULL
      -- Pas dans la blacklist
      AND NOT EXISTS (
          SELECT 1 FROM retrouve_amis.blacklist bl
          WHERE bl.utilisateur_a_id = LEAST(v_auteur_id, a2.auteur_id)
            AND bl.utilisateur_b_id = GREATEST(v_auteur_id, a2.auteur_id)
      )
      -- Pas de correspondance active existante entre cet avis source et cet avis cible
      AND NOT EXISTS (
          SELECT 1 FROM retrouve_amis.correspondance c
          WHERE c.avis_id = p_avis_id AND c.cible_avis_id = a2.id
            AND c.etat NOT IN ('declinee', 'archivee')
      );

    -- ── Branche 2 : avis ↔ profil trouvable ───────────────────────────────
    RETURN QUERY
    SELECT
        'profil'::retrouve_amis.type_cible AS cible_type,
        u.id AS cible_id,
        best.score_total,
        best.details
    FROM iam.utilisateur u
    WHERE u.est_trouvable = TRUE
      AND u.id != v_auteur_id
      AND u.deleted_at IS NULL
      AND u.etat = 'actif'
      -- Pas dans la blacklist
      AND NOT EXISTS (
          SELECT 1 FROM retrouve_amis.blacklist bl
          WHERE bl.utilisateur_a_id = LEAST(v_auteur_id, u.id)
            AND bl.utilisateur_b_id = GREATEST(v_auteur_id, u.id)
      )
      -- Pas de correspondance active existante
      AND NOT EXISTS (
          SELECT 1 FROM retrouve_amis.correspondance c
          WHERE c.avis_id = p_avis_id AND c.cible_utilisateur_id = u.id
            AND c.etat NOT IN ('declinee', 'archivee')
      )
    CROSS JOIN LATERAL (
        SELECT
            GREATEST(
                -- Score nom direct contre utilisateur (40 pts)
                GREATEST(
                    similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(u.nom))),
                    similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(u.prenom))),
                    COALESCE(
                        similarity(unaccent(lower(COALESCE(v_avis.prenom_recherche, ''))), unaccent(lower(u.nom))),
                        0
                    )
                ) * 40
                -- Meilleur score école parmi les parcours (20 pts)
                + COALESCE((
                    SELECT MAX(similarity(unaccent(lower(v_avis.ecole)), unaccent(lower(p.nom)))) * 20
                    FROM retrouve_amis.parcours_trouvable p
                    WHERE p.utilisateur_id = u.id AND p.type_entree = 'ecole'
                      AND v_avis.ecole IS NOT NULL
                ), 0)
                -- Meilleur score ville parmi les parcours (15 pts)
                + COALESCE((
                    SELECT MAX(GREATEST(
                        similarity(unaccent(lower(v_avis.ville)), unaccent(lower(p.nom))),
                        COALESCE(similarity(unaccent(lower(v_avis.ville)), unaccent(lower(COALESCE(p.ville, '')))), 0)
                    )) * 15
                    FROM retrouve_amis.parcours_trouvable p
                    WHERE p.utilisateur_id = u.id
                      AND p.type_entree = 'ville_residence'
                      AND v_avis.ville IS NOT NULL
                ), 0)
                -- Meilleur score période parmi les parcours (15 pts)
                + COALESCE((
                    SELECT MAX(
                        GREATEST(0,
                            LEAST(
                                COALESCE(v_avis.periode_fin, v_avis.periode_debut),
                                COALESCE(p.periode_fin, p.periode_debut)
                            )
                            - GREATEST(v_avis.periode_debut, p.periode_debut)
                        )::NUMERIC / GREATEST(1,
                            GREATEST(
                                COALESCE(v_avis.periode_fin, v_avis.periode_debut),
                                COALESCE(p.periode_fin, p.periode_debut)
                            )
                            - LEAST(v_avis.periode_debut, p.periode_debut)
                        ) * 15
                    )
                    FROM retrouve_amis.parcours_trouvable p
                    WHERE p.utilisateur_id = u.id
                      AND v_avis.periode_debut IS NOT NULL
                      AND p.periode_debut IS NOT NULL
                ), 0)
                -- Score pays (10 pts)
                + CASE
                    WHEN v_avis.pays_id IS NOT NULL AND EXISTS (
                        SELECT 1 FROM retrouve_amis.parcours_trouvable p
                        WHERE p.utilisateur_id = u.id AND p.pays_id = v_avis.pays_id
                    ) THEN 10
                    ELSE 0
                  END
            )::NUMERIC(5,2) AS score_total,
            jsonb_build_object(
                'nom', GREATEST(
                    similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(u.nom))),
                    similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(u.prenom)))
                ) * 40,
                'ecole', COALESCE((
                    SELECT MAX(similarity(unaccent(lower(v_avis.ecole)), unaccent(lower(p.nom)))) * 20
                    FROM retrouve_amis.parcours_trouvable p
                    WHERE p.utilisateur_id = u.id AND p.type_entree = 'ecole' AND v_avis.ecole IS NOT NULL
                ), 0),
                'ville', COALESCE((
                    SELECT MAX(similarity(unaccent(lower(v_avis.ville)), unaccent(lower(p.nom)))) * 15
                    FROM retrouve_amis.parcours_trouvable p
                    WHERE p.utilisateur_id = u.id AND p.type_entree = 'ville_residence' AND v_avis.ville IS NOT NULL
                ), 0),
                'periode', COALESCE((
                    SELECT MAX(
                        GREATEST(0,
                            LEAST(
                                COALESCE(v_avis.periode_fin, v_avis.periode_debut),
                                COALESCE(p.periode_fin, p.periode_debut)
                            )
                            - GREATEST(v_avis.periode_debut, p.periode_debut)
                        )::NUMERIC / GREATEST(1,
                            GREATEST(
                                COALESCE(v_avis.periode_fin, v_avis.periode_debut),
                                COALESCE(p.periode_fin, p.periode_debut)
                            )
                            - LEAST(v_avis.periode_debut, p.periode_debut)
                        ) * 15
                    )
                    FROM retrouve_amis.parcours_trouvable p
                    WHERE p.utilisateur_id = u.id AND v_avis.periode_debut IS NOT NULL AND p.periode_debut IS NOT NULL
                ), 0),
                'pays', CASE
                    WHEN v_avis.pays_id IS NOT NULL AND EXISTS (
                        SELECT 1 FROM retrouve_amis.parcours_trouvable p
                        WHERE p.utilisateur_id = u.id AND p.pays_id = v_avis.pays_id
                    ) THEN 10
                    ELSE 0
                END
            ) AS details
    ) best
    WHERE best.score_total > 0;
END;
$$ LANGUAGE plpgsql;
