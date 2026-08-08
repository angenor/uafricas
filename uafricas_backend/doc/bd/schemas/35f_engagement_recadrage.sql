-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Engagement : recadrage du barème (3 sources, 4 statuts)
-- ════════════════════════════════════════════════════════════════════════════
-- Feature 008-recadrage-engagement-cadeaux — Phase 1 (socle SQL).
--
-- Le barème vivant EN BASE, ce recadrage est une opération de DONNÉES, pas de
-- code (research R1) : aucun appel n'est retiré des handlers. `appliquer`
-- commence par `charger_regle(… WHERE actif = TRUE)` et retourne sans rien
-- écrire quand la règle est inactive — une règle désactivée ne crédite donc
-- rien tout en restant réactivable depuis le back-office, sans redéploiement.
--
-- 1. Désactivation des 8 règles écartées + `popularite_palier`.
-- 2. Catégorie « Cadeaux ».
-- 3. Création des 3 règles canoniques : `jaime_recu`, `partage_recu`,
--    `cadeau_recu` (cette dernière PORTEUSE : montant 0, le vrai montant vient
--    du catalogue figé sur la transaction — research R9).
-- 4. Refonte de `engagement.niveau` en 4 statuts, dans un ordre IMPÉRATIF.
-- 5. Neutralisation de tous les `palier_popularite`.
-- 6. Rebascule de `engagement.compte.niveau_code`, dans la MÊME transaction.
--
-- Migration idempotente (ON CONFLICT, UPDATE idempotents) — appliquée à la main
-- en production via SSH + psql.
-- ════════════════════════════════════════════════════════════════════════════

BEGIN;


-- ── 1. Désactivation des 8 règles écartées + les paliers de popularité ──────
-- Les montants d'origine sont CONSERVÉS : réactiver une règle depuis
-- `/admin/engagement/regles` doit restituer exactement le barème d'avant.
-- `ajustement_admin` reste ACTIVE : c'est l'outil de correction de
-- l'administration, pas une source de points communautaire.

UPDATE engagement.regle_points
   SET actif = FALSE, updated_at = NOW()
 WHERE type_action IN ('contribution_validee', 'contribution_mise_en_avant',
                       'factcheck_valide', 'factcheck_faux',
                       'proposition_media_validee', 'media_a_la_une',
                       'animation_support_acceptee', 'partage_externe_5reseaux',
                       'popularite_palier')
   AND actif = TRUE;


-- ── 2. Catégorie « Cadeaux » ────────────────────────────────────────────────
-- Les catégories devenues sans règle active (`contributions`, `medias`,
-- `factcheck`) sont CONSERVÉES : leur suppression est interdite tant qu'une
-- règle y est rattachée, et elles redeviennent utiles dès qu'une règle écartée
-- est réactivée.

INSERT INTO engagement.categorie_points (code, libelle, description, ordre, couleur, icone)
VALUES ('cadeaux', 'Cadeaux',
        'Points reçus grâce aux cadeaux virtuels offerts par la communauté.',
        4, 'amber', 'gift')
ON CONFLICT (code) DO NOTHING;


-- ── 3. Les 3 règles canoniques ──────────────────────────────────────────────
-- ⚠️ `plafond_journalier` s'exprime EN POINTS, pas en occurrences : le moteur
--    compare `SUM(points)` du jour au plafond. Livrés à NULL (illimité), ils
--    restent réglables en back-office.
--
-- `cadeau_recu` porte `points = 0` volontairement : elle sert de PORTE
-- (activable/désactivable) et de porteuse de catégorie et de plafonds. Le
-- montant réel vient du catalogue, figé sur la transaction, et arrive par
-- `montant_override` (research R9). La désactiver journalise la transaction et
-- la répartition sans créditer un seul point.

INSERT INTO engagement.regle_points
    (type_action, libelle, points, reputation_delta, plafond_journalier,
     plafond_mensuel, seuil_declencheur, categorie_id, actif)
SELECT v.type_action, v.libelle, v.points, v.reputation_delta,
       v.plafond_journalier, v.plafond_mensuel, v.seuil_declencheur,
       (SELECT id FROM engagement.categorie_points WHERE code = v.code_categorie),
       TRUE
  FROM (VALUES
    ('jaime_recu',   'J''aime reçu sur un contenu',    1, 0, NULL::integer, NULL::integer, NULL::integer, 'popularite'),
    ('partage_recu', 'Contenu partagé par un membre',  1, 0, NULL,          NULL,          NULL,          'partages'),
    ('cadeau_recu',  'Cadeau virtuel reçu',            0, 0, NULL,          NULL,          NULL,          'cadeaux')
  ) AS v(type_action, libelle, points, reputation_delta, plafond_journalier,
         plafond_mensuel, seuil_declencheur, code_categorie)
ON CONFLICT (type_action) DO UPDATE
    SET actif = TRUE,
        libelle = EXCLUDED.libelle,
        categorie_id = COALESCE(engagement.regle_points.categorie_id, EXCLUDED.categorie_id),
        updated_at = NOW();


-- ── 4. Quatre statuts ───────────────────────────────────────────────────────
-- ORDRE D'EXÉCUTION IMPÉRATIF (research R6) : `platinum` doit libérer l'ordre 3
-- AVANT l'insertion de `gold`, sinon deux niveaux porteraient momentanément le
-- même ordre. Les CODES sont réutilisés et jamais supprimés : ce sont des clés
-- stables référencées par `compte.niveau_code` et `badge.parametre_niveau_code`.
--
-- `seuil_min` porte un index UNIQUE (`idx_uq_niveau_seuil`, migration 35c) :
-- l'ordre des seuils ci-dessous est choisi pour qu'aucune valeur intermédiaire
-- n'entre en collision (200 → 500, 1000 → 10000, puis 2000 inséré).
--
-- La borne haute d'un statut n'est JAMAIS stockée : elle se déduit du seuil
-- suivant, ce qui rend une grille incohérente impossible à exprimer.

UPDATE engagement.niveau
   SET libelle = 'Membre Africans', seuil_min = 0, ordre = 1,
       badge_couleur = 'gray', badge_icone = 'user'
 WHERE code = 'membre';

UPDATE engagement.niveau
   SET libelle = 'Premium', seuil_min = 500, ordre = 2,
       badge_couleur = 'amber', badge_icone = 'star'
 WHERE code = 'premium';

UPDATE engagement.niveau
   SET libelle = 'Platinum', seuil_min = 10000, ordre = 4,
       badge_couleur = 'slate', badge_icone = 'crown'
 WHERE code = 'platinum';

INSERT INTO engagement.niveau (code, libelle, seuil_min, ordre, badge_couleur, badge_icone)
VALUES ('gold', 'Gold', 2000, 3, 'yellow', 'medal')
ON CONFLICT (code) DO UPDATE
    SET libelle = EXCLUDED.libelle,
        seuil_min = EXCLUDED.seuil_min,
        ordre = EXCLUDED.ordre,
        badge_couleur = EXCLUDED.badge_couleur,
        badge_icone = EXCLUDED.badge_icone;


-- ── 5. Neutralisation des paliers de popularité ─────────────────────────────
-- La table et son écran d'administration sont CONSERVÉS : le mécanisme n'est
-- plus alimenté mais reste réactivable. `services::engagement::evaluer_popularite`
-- est en revanche supprimée du code — sa sémantique (crédit par palier de
-- contenu, une fois par contenu) est incompatible avec le crédit unitaire par
-- membre qui la remplace (research R3).

UPDATE engagement.palier_popularite SET actif = FALSE WHERE actif = TRUE;


-- ── 6. Rebascule des comptes sur la nouvelle grille ─────────────────────────
-- DANS LA MÊME TRANSACTION que les UPDATE de niveaux : entre les deux, aucun
-- compte ne doit pouvoir être lu avec un `niveau_code` pointant sur une grille
-- déjà changée. Un compte à 300 points passe ainsi de « Premium » (ancien seuil
-- 200) à « Membre Africans » (nouveau seuil 500) sans état intermédiaire.

UPDATE engagement.compte c
   SET niveau_code = COALESCE(
           (SELECT n.code FROM engagement.niveau n
             WHERE n.seuil_min <= c.solde_points
             ORDER BY n.seuil_min DESC LIMIT 1),
           'membre'),
       updated_at = NOW()
 WHERE c.niveau_code IS DISTINCT FROM COALESCE(
           (SELECT n.code FROM engagement.niveau n
             WHERE n.seuil_min <= c.solde_points
             ORDER BY n.seuil_min DESC LIMIT 1),
           'membre');


COMMIT;
