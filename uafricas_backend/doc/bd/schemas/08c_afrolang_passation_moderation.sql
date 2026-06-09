-- ════════════════════════════════════════════════════════════════════════
-- 08c — Afrolang : passation de modération (placeholder → modérateur attitré)
-- ════════════════════════════════════════════════════════════════════════
--
-- Contexte (refonte modération multi-modérateurs, 2026-06)
-- --------------------------------------------------------
-- Une session publique afrolang peut désormais avoir PLUSIEURS modérateurs
-- simultanés (plusieurs attitrés + admins coexistent — source de vérité =
-- afrolang.session_participant.role_session = 'moderateur', déjà existant).
--
-- Le premier utilisateur lambda qui démarre une session vide en devient le
-- modérateur « placeholder » (afrolang.session.moderateur_id pointe alors un
-- non-office). Quand un vrai modérateur attitré (désigné au back-office, table
-- afrolang.salle_moderateur) entre, on déclenche une DEMANDE DE PASSATION : le
-- placeholder est invité à céder la modération ; sans réponse au bout de 60 s,
-- la promotion est automatique. Les administrateurs de plateforme/salle, eux,
-- sont modérateurs immédiatement (« quoi qu'il arrive »).
--
-- Pourquoi deux colonnes plutôt qu'une table dédiée
-- -------------------------------------------------
-- L'échéance d'auto-promotion ne peut PAS être dérivée de
-- session_participant.rejoint_at : cette colonne est réécrite à NOW() à chaque
-- (re)connexion (ON CONFLICT DO UPDATE), donc une simple coupure réseau de
-- l'attitré repousserait indéfiniment la promotion. On matérialise donc un
-- horodatage STABLE, posé une seule fois à l'ouverture de la demande et remis
-- à NULL à la résolution. demande_passation_par mémorise le premier attitré
-- demandeur (affichage du prompt côté placeholder).
--
-- Idempotent : réexécutable sans effet de bord (ADD COLUMN IF NOT EXISTS).
-- ════════════════════════════════════════════════════════════════════════

ALTER TABLE afrolang.session
    ADD COLUMN IF NOT EXISTS demande_passation_at  TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS demande_passation_par UUID;  -- [xref] iam.utilisateur

COMMENT ON COLUMN afrolang.session.demande_passation_at IS
    'Instant stable d''ouverture d''une demande de passation de modération (échéance auto = +60 s). NULL hors demande en cours.';
COMMENT ON COLUMN afrolang.session.demande_passation_par IS
    'Premier modérateur attitré entrant ayant déclenché la demande de passation. NULL hors demande en cours.';

-- Accélère la résolution paresseuse (« y a-t-il une demande échue à promouvoir ? »).
CREATE INDEX IF NOT EXISTS idx_afrolang_session_passation
    ON afrolang.session(demande_passation_at)
    WHERE demande_passation_at IS NOT NULL;
