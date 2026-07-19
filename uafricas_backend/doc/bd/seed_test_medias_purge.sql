-- ============================================================================
--  Purge du jeu de données de TEST — refonte Télé & Radio
-- ============================================================================
--
--  Retire tout ce qu'a créé `seed_test_medias.sql`, et rien d'autre.
--
--  Marqueur : tous les identifiants du seed commencent par `dddd0000-`. La
--  purge s'appuie exclusivement dessus, ce qui la rend sûre même sur une base
--  contenant par ailleurs de vraies données.
--
--  Les interactions produites PENDANT les tests (réactions, commentaires,
--  partages, signalements) portent des identifiants aléatoires : elles sont
--  retirées par leur rattachement au contenu de test, jamais par leur id.
--
--  Usage :
--    psql -h localhost -U uafricas -d africans_db -f doc/bd/seed_test_medias_purge.sql
-- ============================================================================

\set ON_ERROR_STOP on

BEGIN;

-- Les identifiants des supports et contenus de test, rassemblés une fois.
CREATE TEMP TABLE cibles_test ON COMMIT DROP AS
SELECT 'chaine_tv' AS type_media, id FROM media_content.chaine_tv
 WHERE id::text LIKE 'dddd0000-%'
UNION ALL
SELECT 'station_radio', id FROM media_content.station_radio
 WHERE id::text LIKE 'dddd0000-%'
UNION ALL
SELECT 'programme_tele', id FROM media_content.programme_tele
 WHERE id::text LIKE 'dddd0000-%'
UNION ALL
SELECT 'programme_radio', id FROM media_content.programme_radio
 WHERE id::text LIKE 'dddd0000-%';

-- 1. Interactions accumulées pendant les tests, sur les contenus de test.
DELETE FROM media_content.media_reaction    m USING cibles_test c
 WHERE m.type_media = c.type_media AND m.media_id = c.id;
DELETE FROM media_content.media_commentaire m USING cibles_test c
 WHERE m.type_media = c.type_media AND m.media_id = c.id;
DELETE FROM media_content.partage_media     m USING cibles_test c
 WHERE m.type_media = c.type_media AND m.media_id = c.id;
DELETE FROM media_content.signalement_media m USING cibles_test c
 WHERE m.type_media = c.type_media AND m.media_id = c.id;

-- 2. Programmation, co-détention, invitations.
DELETE FROM media_content.creneau_programmation
 WHERE id::text LIKE 'dddd0000-%'
    OR support_id::text LIKE 'dddd0000-%'
    OR contenu_id::text LIKE 'dddd0000-%';

DELETE FROM media_content.support_detenteur
 WHERE support_id::text LIKE 'dddd0000-%';

DELETE FROM media_content.invitation_detenteur
 WHERE support_id::text LIKE 'dddd0000-%';

-- 3. Propositions de test, y compris celles validées pendant les essais
--    (leur objet créé est retiré à l'étape suivante s'il porte le marqueur).
DELETE FROM media_content.proposition_media
 WHERE id::text LIKE 'dddd0000-%'
    OR target_id::text LIKE 'dddd0000-%'
    OR objet_id_cree::text LIKE 'dddd0000-%';

-- 4. Contenus, puis supports : l'ordre respecte les clés étrangères.
DELETE FROM media_content.programme_tele  WHERE id::text LIKE 'dddd0000-%';
DELETE FROM media_content.programme_radio WHERE id::text LIKE 'dddd0000-%';
DELETE FROM media_content.chaine_tv       WHERE id::text LIKE 'dddd0000-%';
DELETE FROM media_content.station_radio   WHERE id::text LIKE 'dddd0000-%';

-- 5. Traces d'audit produites par les tests sur les objets de test.
DELETE FROM shared.audit_log
 WHERE record_id::text LIKE 'dddd0000-%';

COMMIT;

\echo ''
\echo '=== Jeu de données de test purgé ==='
SELECT 'lignes marquées restantes : ' || (
    (SELECT count(*) FROM media_content.chaine_tv       WHERE id::text LIKE 'dddd0000-%')
  + (SELECT count(*) FROM media_content.station_radio   WHERE id::text LIKE 'dddd0000-%')
  + (SELECT count(*) FROM media_content.programme_tele  WHERE id::text LIKE 'dddd0000-%')
  + (SELECT count(*) FROM media_content.programme_radio WHERE id::text LIKE 'dddd0000-%')
  + (SELECT count(*) FROM media_content.proposition_media WHERE id::text LIKE 'dddd0000-%')
);

\echo ''
\echo 'ATTENTION : la vedette générale de la page Télé a été retirée avec le'
\echo 'jeu de test. Désignez-en une autre depuis /admin/television avant de'
\echo 'juger la page, sinon elle basculera sur son repli automatique.'
