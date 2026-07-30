-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Engagement : badges et succès
-- ════════════════════════════════════════════════════════════════════════════
-- Feature 007-engagement-points-badges — US3.
--
-- 1. Enums `type_condition_badge` (5 valeurs) et `origine_badge` (3 valeurs).
-- 2. `engagement.badge` : définition d'un badge, avec un CHECK qui rend le
--    paramétrage incohérent IMPOSSIBLE EN SQL — pas seulement dans l'interface.
-- 3. `engagement.badge_obtenu` : `UNIQUE (utilisateur_id, badge_id)` — c'est
--    cette contrainte, et non un contrôle applicatif, qui interdit le doublon.
-- 4. Seed de 10 badges.
-- 5. Rétro-évaluation unique, SANS notification (R9) : notifier ce lot enverrait
--    des dizaines d'alertes à des membres qui n'ont rien fait à cet instant.
--
-- Migration idempotente. Les conditions de badge sont un enum fermé, pas un
-- langage : chaque type se traduit en UNE requête connue à l'avance (R6).
-- ════════════════════════════════════════════════════════════════════════════


-- ── 1. Enums ─────────────────────────────────────────────────────────────────

DO $$ BEGIN
    CREATE TYPE engagement.type_condition_badge AS ENUM (
        'actions_comptees',   -- N mouvements d'un `type_action` donné
        'points_categorie',   -- N points cumulés dans une catégorie
        'solde_total',        -- solde global >= N
        'niveau_atteint',     -- niveau courant >= ce niveau (comparaison sur `ordre`)
        'palier_popularite'   -- un palier >= N franchi au moins une fois
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE engagement.origine_badge AS ENUM ('automatique', 'manuel', 'retroactif');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;


-- ── 2. Définition d'un badge ─────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS engagement.badge (
    id                     UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code                   VARCHAR(40) NOT NULL UNIQUE,   -- clé stable, immuable
    libelle                VARCHAR(80) NOT NULL,
    description            TEXT        NOT NULL,          -- condition en langage clair (FR-013)
    couleur                VARCHAR(20),
    icone                  VARCHAR(40),
    -- Badge éditorial : jamais évalué automatiquement, attribué à la main.
    manuel                 BOOLEAN     NOT NULL DEFAULT FALSE,
    type_condition         engagement.type_condition_badge,
    parametre_action       VARCHAR(50),                   -- `actions_comptees`
    parametre_categorie_id UUID REFERENCES engagement.categorie_points(id) ON DELETE RESTRICT,
    parametre_niveau_code  VARCHAR(30),                   -- `niveau_atteint`
    seuil                  INTEGER,
    ordre                  SMALLINT    NOT NULL DEFAULT 0,
    -- Désactivé = retiré du catalogue « à débloquer », MAIS conservé chez ses
    -- détenteurs (FR-020).
    actif                  BOOLEAN     NOT NULL DEFAULT TRUE,
    created_at             TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at             TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Un badge automatique sans paramètre exploitable ne se déclencherait jamais et
-- resterait invisiblement stérile : le CHECK ferme cette porte au niveau SQL.
-- (`seuil > 0` est NULL quand `seuil` est NULL, donc la branche est fausse : le
-- seuil obligatoire est imposé sans clause supplémentaire.)
DO $$ BEGIN
    ALTER TABLE engagement.badge ADD CONSTRAINT ck_badge_condition CHECK (
        (manuel = TRUE  AND type_condition IS NULL)
     OR (manuel = FALSE AND (
            (type_condition = 'actions_comptees'  AND parametre_action IS NOT NULL AND seuil > 0)
         OR (type_condition = 'points_categorie'  AND parametre_categorie_id IS NOT NULL AND seuil > 0)
         OR (type_condition = 'solde_total'       AND seuil > 0)
         OR (type_condition = 'niveau_atteint'    AND parametre_niveau_code IS NOT NULL)
         OR (type_condition = 'palier_popularite' AND seuil > 0)))
    );
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

CREATE INDEX IF NOT EXISTS idx_badge_actif ON engagement.badge(actif, ordre);


-- ── 3. Badges détenus ────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS engagement.badge_obtenu (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- CASCADE : un membre supprimé ne laisse pas de badge orphelin.
    utilisateur_id UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    badge_id       UUID NOT NULL REFERENCES engagement.badge(id) ON DELETE CASCADE,
    origine        engagement.origine_badge NOT NULL DEFAULT 'automatique',
    attribue_par   UUID REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Support de l'idempotence (FR-018, FR-034) : réévaluer est inoffensif.
    UNIQUE (utilisateur_id, badge_id)
);

CREATE INDEX IF NOT EXISTS idx_badge_obtenu_utilisateur
    ON engagement.badge_obtenu(utilisateur_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_badge_obtenu_badge
    ON engagement.badge_obtenu(badge_id);


-- ════════════════════════════════════════════════════════════════════════════
-- SEED — 10 badges
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO engagement.badge
    (code, libelle, description, couleur, icone, ordre, manuel,
     type_condition, parametre_action, parametre_categorie_id, parametre_niveau_code, seuil)
SELECT v.code, v.libelle, v.description, v.couleur, v.icone, v.ordre, v.manuel,
       v.type_condition::engagement.type_condition_badge,
       v.parametre_action,
       (SELECT id FROM engagement.categorie_points WHERE code = v.code_categorie),
       v.parametre_niveau_code,
       v.seuil
  FROM (VALUES
    ('premier_pas', 'Premier pas', 'Votre première contribution validée.',
     'green', 'seedling', 1, FALSE, 'actions_comptees', 'contribution_validee', NULL, NULL, 1),
    ('conteur', 'Conteur', '10 contributions validées.',
     'green', 'feather', 2, FALSE, 'actions_comptees', 'contribution_validee', NULL, NULL, 10),
    ('pilier', 'Pilier', '50 contributions validées.',
     'chocolat', 'landmark', 3, FALSE, 'actions_comptees', 'contribution_validee', NULL, NULL, 50),
    ('verificateur', 'Vérificateur', '5 vérifications de faits jugées correctes.',
     'sky', 'magnifying-glass-chart', 4, FALSE, 'actions_comptees', 'factcheck_valide', NULL, NULL, 5),
    ('voix_qui_porte', 'Voix qui porte', 'Une publication ayant franchi le palier de 500 « j''aime ».',
     'rose', 'bullhorn', 5, FALSE, 'palier_popularite', NULL, NULL, NULL, 500),
    ('batisseur_medias', 'Bâtisseur de médias', '50 points gagnés dans la catégorie Médias.',
     'amber', 'tower-broadcast', 6, FALSE, 'points_categorie', NULL, 'medias', NULL, 50),
    ('ambassadeur', 'Ambassadeur', 'Avoir partagé 3 contenus vers plusieurs réseaux sociaux.',
     'violet', 'share-nodes', 7, FALSE, 'actions_comptees', 'partage_externe_5reseaux', NULL, NULL, 3),
    ('statut_premium', 'Membre Premium', 'Avoir atteint le statut Membre Premium.',
     'amber', 'star', 8, FALSE, 'niveau_atteint', NULL, NULL, 'premium', NULL),
    ('statut_platinum', 'Influenceur Platinum', 'Avoir atteint le statut Influenceur Platinum.',
     'slate', 'crown', 9, FALSE, 'niveau_atteint', NULL, NULL, 'platinum', NULL),
    ('distinction_editoriale', 'Distinction éditoriale',
     'Distinction remise par l''équipe éditoriale.',
     'chocolat', 'award', 10, TRUE, NULL, NULL, NULL, NULL, NULL)
  ) AS v(code, libelle, description, couleur, icone, ordre, manuel,
         type_condition, parametre_action, code_categorie, parametre_niveau_code, seuil)
ON CONFLICT (code) DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
-- RÉTRO-ÉVALUATION UNIQUE (R9) — aucune notification pour ce lot
-- ════════════════════════════════════════════════════════════════════════════
-- La spec accepte la rétro-évaluation des badges (et refuse celle des points).
-- `origine = 'retroactif'` distingue ces lignes de 'automatique' et 'manuel'.
-- `ON CONFLICT DO NOTHING` rend la migration rejouable sans effet de bord.

-- ── actions_comptees ──
INSERT INTO engagement.badge_obtenu (utilisateur_id, badge_id, origine)
SELECT m.utilisateur_id, b.id, 'retroactif'::engagement.origine_badge
  FROM engagement.badge b
  JOIN engagement.mouvement_points m ON m.type_action = b.parametre_action
 WHERE b.manuel = FALSE AND b.actif = TRUE AND b.type_condition = 'actions_comptees'
 GROUP BY m.utilisateur_id, b.id, b.seuil
HAVING COUNT(*) >= b.seuil
ON CONFLICT (utilisateur_id, badge_id) DO NOTHING;

-- ── points_categorie ──
INSERT INTO engagement.badge_obtenu (utilisateur_id, badge_id, origine)
SELECT m.utilisateur_id, b.id, 'retroactif'::engagement.origine_badge
  FROM engagement.badge b
  JOIN engagement.mouvement_points m ON m.categorie_id = b.parametre_categorie_id
 WHERE b.manuel = FALSE AND b.actif = TRUE AND b.type_condition = 'points_categorie'
 GROUP BY m.utilisateur_id, b.id, b.seuil
HAVING COALESCE(SUM(m.points), 0) >= b.seuil
ON CONFLICT (utilisateur_id, badge_id) DO NOTHING;

-- ── solde_total ──
INSERT INTO engagement.badge_obtenu (utilisateur_id, badge_id, origine)
SELECT c.utilisateur_id, b.id, 'retroactif'::engagement.origine_badge
  FROM engagement.badge b
  JOIN engagement.compte c ON c.solde_points >= b.seuil
 WHERE b.manuel = FALSE AND b.actif = TRUE AND b.type_condition = 'solde_total'
ON CONFLICT (utilisateur_id, badge_id) DO NOTHING;

-- ── niveau_atteint (comparaison sur `ordre`, jamais sur le code) ──
INSERT INTO engagement.badge_obtenu (utilisateur_id, badge_id, origine)
SELECT c.utilisateur_id, b.id, 'retroactif'::engagement.origine_badge
  FROM engagement.badge b
  JOIN engagement.niveau cible ON cible.code = b.parametre_niveau_code
  JOIN engagement.compte c ON TRUE
  JOIN engagement.niveau courant ON courant.code = c.niveau_code
 WHERE b.manuel = FALSE AND b.actif = TRUE AND b.type_condition = 'niveau_atteint'
   AND courant.ordre >= cible.ordre
ON CONFLICT (utilisateur_id, badge_id) DO NOTHING;

-- ── palier_popularite ──
-- Le seuil franchi n'est pas une colonne du journal : il est porté par la clé
-- d'idempotence, dont le format `popularite:{type_objet}:{objet_id}:{seuil}` est
-- fixé par `services::engagement::evaluer_popularite`. Le garde `~ '^\d+$'`
-- protège d'une clé malformée (qui ferait échouer le cast et donc la migration).
INSERT INTO engagement.badge_obtenu (utilisateur_id, badge_id, origine)
SELECT DISTINCT m.utilisateur_id, b.id, 'retroactif'::engagement.origine_badge
  FROM engagement.badge b
  JOIN engagement.mouvement_points m ON m.type_action = 'popularite_palier'
 WHERE b.manuel = FALSE AND b.actif = TRUE AND b.type_condition = 'palier_popularite'
   AND split_part(m.cle_idempotence, ':', 4) ~ '^\d+$'
   AND split_part(m.cle_idempotence, ':', 4)::integer >= b.seuil
ON CONFLICT (utilisateur_id, badge_id) DO NOTHING;
