-- ════════════════════════════════════════════════════════════════════════════
-- MAINTENANCE MANUELLE — Purge des données de test / démo (audit #10 & #11)
-- ════════════════════════════════════════════════════════════════════════════
-- ⚠️  Ce fichier N'EST PAS orchestré par schema.sql (hors dossier schemas/).
--     Il ne doit JAMAIS être exécuté automatiquement.
--
-- USAGE :
--   1. Exécuter d'ABORD les blocs « DÉTECTION » (SELECT) et RELIRE les lignes.
--   2. Ajuster les motifs si nécessaire (certains comptes @uafricas.org peuvent
--      être légitimes — l'audit signale une COHABITATION démo/réel).
--   3. Décommenter les blocs « SUPPRESSION » (soft-delete) UNIQUEMENT pour les
--      lignes validées, idéalement dans une transaction (BEGIN … COMMIT/ROLLBACK).
--
-- Convention projet : soft-deletion via deleted_at (jamais de DELETE physique).
-- Faire un backup (deploy.sh backup) avant toute suppression en production.
-- ════════════════════════════════════════════════════════════════════════════


-- ────────────────────────────────────────────────────────────────────────────
-- (A) CONTENUS DE TEST EN PRODUCTION  (audit #10)
-- ────────────────────────────────────────────────────────────────────────────

-- A.1 — DÉTECTION : mauvaises pratiques / idées force intitulées « Test … »
SELECT 'bad_habit' AS source, id, titre, cree_par, created_at
  FROM governance.bad_habit
 WHERE deleted_at IS NULL AND titre ILIKE 'test%'
UNION ALL
SELECT 'idea_force', id, titre, cree_par, created_at
  FROM governance.idea_force
 WHERE deleted_at IS NULL AND titre ILIKE 'test%'
 ORDER BY created_at;

-- A.2 — DÉTECTION : publications Codimoi de test + hashtags malformés (##)
SELECT id, LEFT(contenu, 80) AS apercu, cree_par, created_at
  FROM culture.codimoi
 WHERE deleted_at IS NULL
   AND (contenu ILIKE 'test%' OR contenu ILIKE '%##%')
 ORDER BY created_at;

-- A.3 — DÉTECTION : salles Afrolang de test (ex. « test salle angenor »)
SELECT id, titre, cree_par, created_at
  FROM afrolang.salle
 WHERE deleted_at IS NULL AND titre ILIKE '%test%'
 ORDER BY created_at;

-- A.x — SUPPRESSION (soft-delete) — DÉCOMMENTER APRÈS RELECTURE :
-- UPDATE governance.bad_habit SET deleted_at = NOW()
--  WHERE deleted_at IS NULL AND titre ILIKE 'test%';
-- UPDATE governance.idea_force SET deleted_at = NOW()
--  WHERE deleted_at IS NULL AND titre ILIKE 'test%';
-- UPDATE culture.codimoi SET deleted_at = NOW()
--  WHERE deleted_at IS NULL AND contenu ILIKE 'test%';   -- ⚠️ revalider ligne par ligne
-- UPDATE afrolang.salle SET deleted_at = NOW()
--  WHERE deleted_at IS NULL AND titre ILIKE '%test%';


-- ────────────────────────────────────────────────────────────────────────────
-- (B) COMPTES DE DÉMONSTRATION MÊLÉS AUX COMPTES RÉELS  (audit #11)
-- ────────────────────────────────────────────────────────────────────────────
-- ⚠️ Certains @uafricas.org peuvent être des comptes internes LÉGITIMES.
--    NE PAS supprimer en masse : valider chaque ligne.

-- B.1 — DÉTECTION : comptes de démo probables (domaines de test)
SELECT id, prenom, nom, email, etat, created_at
  FROM iam.utilisateur
 WHERE deleted_at IS NULL
   AND (email LIKE '%@uafricas.org' OR email LIKE '%@test.com')
 ORDER BY email;

-- B.2 — DÉTECTION ciblée : les 5 comptes nommés dans le rapport d'audit
SELECT id, prenom, nom, email, etat
  FROM iam.utilisateur
 WHERE deleted_at IS NULL
   AND email IN (
       -- Remplacer par les adresses exactes après relecture de B.1
       -- 'moussa.ndiaye@uafricas.org',
       -- 'aminata.diallo@uafricas.org',
       -- 'kouassi.yao@uafricas.org',
       -- 'aissatou.bamba@uafricas.org',
       -- 'ibrahim.kone@uafricas.org'
       ''
   );

-- B.x — SUPPRESSION (soft-delete + état) — DÉCOMMENTER pour les IDs validés :
-- UPDATE iam.utilisateur
--    SET etat = 'supprime', deleted_at = NOW()
--  WHERE id IN ( /* … IDs issus de B.1/B.2, un par un … */ );
-- NB : les FK (ON DELETE CASCADE) ne se déclenchent PAS sur un soft-delete ;
--      le contenu rattaché reste en base mais l'auteur devient invisible
--      (obtenir_membre filtre déjà etat='actif').
