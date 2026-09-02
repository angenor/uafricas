-- ═══════════════════════════════════════════════════════════════════════
-- 11n — Rattacher `fiche_pays.gerer` au rôle Administrateur
-- ═══════════════════════════════════════════════════════════════════════
--
-- La permission `fiche_pays.gerer` existait dans `iam.permission` mais
-- n'était accordée à AUCUN rôle. Combinée à un second défaut — le code
-- gardait ses routes sur une ressource `profil_pays` qui n'a jamais existé
-- en base —, le back-office Afripulse n'était joignable que par le Super
-- Administrateur, via son joker `*.*`.
--
-- Conséquence concrète : une contribution communautaire (une image ajoutée à
-- une recette, par exemple) restait `en_attente` indéfiniment, personne
-- d'autre ne pouvant la valider. Le contributeur voyait « soumise avec
-- succès » et son image n'apparaissait jamais.
--
-- Idempotent : `ON CONFLICT DO NOTHING` sur la clé (role_id, permission_id).
-- ═══════════════════════════════════════════════════════════════════════

INSERT INTO iam.role_permission (role_id, permission_id)
SELECT r.id, p.id
  FROM iam.role r
  CROSS JOIN iam.permission p
 WHERE r.nom = 'Administrateur'
   AND p.type_ressource = 'fiche_pays'
   AND p.action = 'gerer'
ON CONFLICT DO NOTHING;
