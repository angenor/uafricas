-- ============================================================================
-- 04h : catalogue de permissions du back-office, et attribution aux rôles
-- ============================================================================
--
-- CONSTAT
--
-- Le back-office était vide pour tout compte qui n'est pas Super
-- Administrateur, et cela tenait à DEUX défauts superposés.
--
--  1. Le code exige 50 permissions ; le catalogue n'en contenait que 11 des
--     siennes. Les 39 autres (`marketplace.voir`, `culture.voir`,
--     `mooc.voir`, `referentiel.*`, `gouvernance.*`…) n'existaient nulle
--     part : aucun rôle ne POUVAIT les détenir, puisqu'on ne peut pas
--     rattacher une permission qui n'a pas de ligne.
--
--  2. Le rôle « Administrateur » ne détenait AUCUNE permission. Le seed
--     `15_seed.sql` n'en attribue qu'au rôle `super_admin`, dont le joker
--     `*.*` masquait le problème pendant tout le développement.
--
-- Les 50 permissions insérées ici sont DÉRIVÉES des appels
-- `verifier_permission!` du code, pas d'une liste écrite à la main : c'est ce
-- que le serveur exige réellement, route par route.
--
-- Les 17 permissions historiques jamais demandées par aucune route
-- (`annonce.valider`, `livre.gerer`, `radio_tele.gerer`…) sont CONSERVÉES :
-- elles décrivent des intentions, et les supprimer effacerait une décision
-- sans en prendre une meilleure. Elles restent simplement sans effet.
-- ============================================================================

-- ── 1. Le catalogue ────────────────────────────────────────────────────────
-- `ON CONFLICT (slug) DO NOTHING` : onze de ces permissions existent déjà, la
-- migration doit pouvoir être rejouée sans faire d'erreur.
INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    ('Modérer Afrolang', 'afrolang.moderer', 'afrolang', 'moderer'),
    ('Modifier Afrolang', 'afrolang.modifier', 'afrolang', 'modifier'),
    ('Supprimer Afrolang', 'afrolang.supprimer', 'afrolang', 'supprimer'),
    ('Consulter Afrolang', 'afrolang.voir', 'afrolang', 'voir'),
    ('Consulter le journal d''audit', 'audit.voir', 'audit', 'voir'),
    ('Modifier les bibliothèques humaines', 'bibliotheque_humaine.modifier', 'bibliotheque_humaine', 'modifier'),
    ('Consulter les bibliothèques humaines', 'bibliotheque_humaine.voir', 'bibliotheque_humaine', 'voir'),
    ('Modifier la culture', 'culture.modifier', 'culture', 'modifier'),
    ('Supprimer la culture', 'culture.supprimer', 'culture', 'supprimer'),
    ('Consulter la culture', 'culture.voir', 'culture', 'voir'),
    ('Gérer l''engagement', 'engagement.gerer', 'engagement', 'gerer'),
    ('Modifier les événements', 'evenement.modifier', 'evenement', 'modifier'),
    ('Supprimer les événements', 'evenement.supprimer', 'evenement', 'supprimer'),
    ('Consulter les événements', 'evenement.voir', 'evenement', 'voir'),
    ('Valider les demandes d''expertise', 'expertise.valider', 'expertise', 'valider'),
    ('Consulter les demandes d''expertise', 'expertise.voir', 'expertise', 'voir'),
    ('Gérer les fiches pays', 'fiche_pays.gerer', 'fiche_pays', 'gerer'),
    ('Modifier la gouvernance', 'gouvernance.modifier', 'gouvernance', 'modifier'),
    ('Supprimer la gouvernance', 'gouvernance.supprimer', 'gouvernance', 'supprimer'),
    ('Consulter la gouvernance', 'gouvernance.voir', 'gouvernance', 'voir'),
    ('Modifier l''innovation', 'innovation.modifier', 'innovation', 'modifier'),
    ('Supprimer l''innovation', 'innovation.supprimer', 'innovation', 'supprimer'),
    ('Consulter l''innovation', 'innovation.voir', 'innovation', 'voir'),
    ('Modifier les livres', 'livre.modifier', 'livre', 'modifier'),
    ('Supprimer les livres', 'livre.supprimer', 'livre', 'supprimer'),
    ('Consulter les livres', 'livre.voir', 'livre', 'voir'),
    ('Modifier le marché', 'marketplace.modifier', 'marketplace', 'modifier'),
    ('Supprimer le marché', 'marketplace.supprimer', 'marketplace', 'supprimer'),
    ('Consulter le marché', 'marketplace.voir', 'marketplace', 'voir'),
    ('Modifier les supports médias', 'media.modifier', 'media', 'modifier'),
    ('Supprimer les supports médias', 'media.supprimer', 'media', 'supprimer'),
    ('Consulter les supports médias', 'media.voir', 'media', 'voir'),
    ('Modifier les médias', 'media_content.modifier', 'media_content', 'modifier'),
    ('Supprimer les médias', 'media_content.supprimer', 'media_content', 'supprimer'),
    ('Consulter les médias', 'media_content.voir', 'media_content', 'voir'),
    ('Modifier les formations', 'mooc.modifier', 'mooc', 'modifier'),
    ('Supprimer les formations', 'mooc.supprimer', 'mooc', 'supprimer'),
    ('Consulter les formations', 'mooc.voir', 'mooc', 'voir'),
    ('Modifier les programmes', 'programme.modifier', 'programme', 'modifier'),
    ('Supprimer les programmes', 'programme.supprimer', 'programme', 'supprimer'),
    ('Consulter les programmes', 'programme.voir', 'programme', 'voir'),
    ('Modifier les référentiels', 'referentiel.modifier', 'referentiel', 'modifier'),
    ('Supprimer les référentiels', 'referentiel.supprimer', 'referentiel', 'supprimer'),
    ('Consulter les référentiels', 'referentiel.voir', 'referentiel', 'voir'),
    ('Modifier Africonnect', 'retrouve_amis.modifier', 'retrouve_amis', 'modifier'),
    ('Consulter Africonnect', 'retrouve_amis.voir', 'retrouve_amis', 'voir'),
    ('Bloquer les utilisateurs', 'utilisateur.bloquer', 'utilisateur', 'bloquer'),
    ('Modifier les utilisateurs', 'utilisateur.modifier', 'utilisateur', 'modifier'),
    ('Supprimer les utilisateurs', 'utilisateur.supprimer', 'utilisateur', 'supprimer'),
    ('Consulter les utilisateurs', 'utilisateur.voir', 'utilisateur', 'voir')
ON CONFLICT (slug) DO NOTHING;


-- ── 2. Le rôle Administrateur reçoit tout le catalogue concret ─────────────
-- Tout SAUF le joker `all.all`, qui reste la marque du Super Administrateur.
-- C'est là toute la différence entre les deux rôles : le joker couvre par
-- avance les permissions à venir, l'énumération non. Un administrateur devra
-- donc être explicitement habilité à ce qui sera ajouté demain.
INSERT INTO iam.role_permission (role_id, permission_id)
SELECT r.id, p.id
FROM iam.role r, iam.permission p
WHERE r.slug = 'admin' AND p.slug <> 'all.all'
ON CONFLICT DO NOTHING;


-- ── 3. Le rôle Modérateur reçoit la lecture et la modération ──────────────
-- Ni suppression ni modification de fond : un modérateur arbitre ce que la
-- communauté produit, il n'administre pas le référentiel. `audit.voir` en est
-- exclu, le journal d'audit tracant les actions des administrateurs eux-mêmes.
INSERT INTO iam.role_permission (role_id, permission_id)
SELECT r.id, p.id
FROM iam.role r, iam.permission p
WHERE r.slug = 'moderateur'
  AND p.action IN ('voir', 'moderer', 'valider', 'suspendre')
  AND p.type_ressource NOT IN ('*', 'audit', 'utilisateur')
ON CONFLICT DO NOTHING;
