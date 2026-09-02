-- ════════════════════════════════════════════════════════════════════════════
-- 96 — Jeu de démonstration LOCAL : Diapertise, Humantech, Librafrica,
--      Africantives, Africamood
-- ════════════════════════════════════════════════════════════════════════════
--
-- Hors de `schemas/` : ce fichier n'est PAS inclus par `schema.sql` et ne part
-- donc jamais en production. Même règle que 97, 98 et 99.
--
-- Rejouable : chaque insertion se garde par un `WHERE NOT EXISTS` sur une clé
-- naturelle (titre, ou couple utilisateur/objet). Aucun UUID n'est écrit en
-- dur — les utilisateurs, pays, domaines et spécialités sont résolus par leur
-- e-mail, leur code ISO ou leur nom. Un identifiant recopié d'une base à
-- l'autre serait faux dès la première réinitialisation.
--
-- Application :
--   docker exec -i uafricas_postgres psql -U uafricas -d africans_db \
--     < doc/bd/seeds-locaux/96_seed_local_modules.sql
-- ════════════════════════════════════════════════════════════════════════════


-- ════════════════════════════════════════════════════════════════════════════
-- DIAPERTISE — expertises validées (iam.expertise)
-- ════════════════════════════════════════════════════════════════════════════
--
-- `statut = 'valide'` : `GET /api/experts` ne sert que les expertises
-- validées. En 'en_attente', l'annuaire resterait vide et le module semblerait
-- cassé.
-- `date_validation` est renseignée avec `valide_par` : une expertise validée
-- sans date ni validateur serait un état que le back-office ne produit jamais.

INSERT INTO iam.expertise (
    utilisateur_id, domaine, biographie, nb_annees_experience, rating,
    specialites, objectifs, realisations, situations_professionnelles,
    statut, valide_par, date_validation)
SELECT u.id, d.domaine::iam.domaine_expertise, d.biographie, d.annees, d.rating,
       d.specialites, d.objectifs::iam.objectif_expertise[],
       d.realisations, d.situations::iam.situation_professionnelle[],
       'valide'::iam.statut_expertise, a.id, NOW() - INTERVAL '20 days'
FROM (VALUES
    ('aminata.ndiaye@uafricas.org', 'sante',
     'Médecin de santé publique, j''ai coordonné des campagnes de vaccination et des programmes de santé communautaire dans quatre pays d''Afrique de l''Ouest. Je forme aujourd''hui des équipes soignantes en zone rurale.',
     14, 4.6,
     ARRAY['Santé communautaire', 'Épidémiologie', 'Formation soignante'],
     ARRAY['consultance', 'benevolat'],
     ARRAY['Campagne de vaccination multi-pays', 'Ouverture de trois centres de santé ruraux'],
     ARRAY['en_emploi', 'consultance']),
    ('ibrahima.fall@uafricas.org', 'agriculture',
     'Ingénieur agronome spécialisé en agriculture de conservation. J''accompagne des coopératives dans la transition vers des pratiques qui restaurent les sols plutôt que de les épuiser.',
     11, 4.8,
     ARRAY['Agriculture de conservation', 'Coopératives agricoles', 'Irrigation'],
     ARRAY['consultance', 'offre_services_court_terme'],
     ARRAY['Accompagnement de 40 coopératives', 'Manuel de terrain diffusé en trois langues'],
     ARRAY['consultance']),
    ('ousmane.sow@uafricas.org', 'informatique',
     'Architecte logiciel, je conçois des systèmes qui tiennent avec une connexion intermittente et un matériel modeste. C''est la contrainte réelle de la plupart des usages sur le continent.',
     9, 4.4,
     ARRAY['Architecture logicielle', 'Hors-ligne d''abord', 'Formation technique'],
     ARRAY['reseautage', 'consultance'],
     ARRAY['Plateforme de suivi scolaire déployée dans 60 écoles'],
     ARRAY['en_emploi']),
    ('fatou.traore@uafricas.org', 'education',
     'Formatrice en pédagogie active. Je travaille sur la place des langues maternelles dans les premiers apprentissages, un levier que les systèmes scolaires laissent largement de côté.',
     16, 4.9,
     ARRAY['Pédagogie active', 'Langues maternelles', 'Formation de formateurs'],
     ARRAY['volontariat', 'reseautage'],
     ARRAY['Programme bilingue expérimenté sur douze écoles', 'Guide du formateur en wolof et en bambara'],
     ARRAY['en_emploi'])
) AS d(email, domaine, biographie, annees, rating, specialites, objectifs, realisations, situations)
JOIN iam.utilisateur u ON u.email = d.email AND u.deleted_at IS NULL
CROSS JOIN LATERAL (SELECT id FROM iam.utilisateur WHERE email = 'test-admin@test.com' LIMIT 1) a
WHERE NOT EXISTS (
    SELECT 1 FROM iam.expertise e WHERE e.utilisateur_id = u.id AND e.deleted_at IS NULL);


-- ════════════════════════════════════════════════════════════════════════════
-- HUMANTECH — bibliothèques humaines validées (iam.demande_biblio_humaine)
-- ════════════════════════════════════════════════════════════════════════════
--
-- Une bibliothèque humaine est une PERSONNE que l'on vient écouter : la
-- demande validée EST la fiche publique, il n'y a pas de seconde table.
--
-- UNE fiche active par compte : `idx_demande_biblio_active_unique` l'impose.
-- Chaque entrée ci-dessous appartient donc à un utilisateur distinct.

INSERT INTO iam.demande_biblio_humaine (
    utilisateur_id, statut, fonction, biographie, pays_origine_id, traite_par, traite_le)
SELECT u.id, 'valide'::iam.statut_demande_biblio, d.fonction, d.biographie, p.id,
       a.id, NOW() - INTERVAL '15 days'
FROM (VALUES
    ('moussa.coulibaly@uafricas.org', 'CI', 'Griot et conteur',
     'Depuis quarante ans je transmets l''histoire des familles et des villages du centre de la Côte d''Ivoire. Ce que je garde en mémoire n''est écrit nulle part, et c''est précisément pour cela que je le raconte.'),
    ('mariam.toure@uafricas.org', 'ML', 'Tisserande et formatrice',
     'J''ai appris le tissage bogolan de ma grand-mère et je l''enseigne depuis vingt ans. Chaque motif porte un sens ; les reproduire sans le savoir, c''est perdre la moitié du geste.'),
    ('aissatou.bamba@uafricas.org', 'SN', 'Ancienne sage-femme de brousse',
     'Trente-deux ans de pratique dans des villages sans électricité. Je parle de ce que la médecine moderne gagne à écouter des savoirs qu''elle a longtemps regardés de haut.')
) AS d(email, iso, fonction, biographie)
JOIN iam.utilisateur u ON u.email = d.email AND u.deleted_at IS NULL
JOIN shared.pays p ON p.code_iso2 = d.iso
CROSS JOIN LATERAL (SELECT id FROM iam.utilisateur WHERE email = 'test-admin@test.com' LIMIT 1) a
WHERE NOT EXISTS (
    SELECT 1 FROM iam.demande_biblio_humaine b
    WHERE b.utilisateur_id = u.id AND b.fonction = d.fonction AND b.deleted_at IS NULL);

-- Rattachement aux spécialités du référentiel (21 lignes déjà semées).
INSERT INTO iam.demande_biblio_specialite (demande_id, specialite_id)
SELECT b.id, s.id
FROM iam.demande_biblio_humaine b
JOIN (VALUES
    ('Griot et conteur',              'Contes et proverbes'),
    ('Griot et conteur',              'Histoire'),
    ('Tisserande et formatrice',      'Bonnes pratiques africaines'),
    ('Ancienne sage-femme de brousse','Culture générale')
) AS m(fonction, specialite) ON m.fonction = b.fonction
JOIN iam.specialite_bibliotheque s ON s.nom = m.specialite
WHERE b.deleted_at IS NULL
  AND NOT EXISTS (
    SELECT 1 FROM iam.demande_biblio_specialite ds
    WHERE ds.demande_id = b.id AND ds.specialite_id = s.id);


-- ════════════════════════════════════════════════════════════════════════════
-- LIBRAFRICA — publications numériques (media_content.livre)
-- ════════════════════════════════════════════════════════════════════════════
--
-- `document_pdf_url` est NOT NULL : les chemins pointent sous `./uploads/`,
-- servi par actix-files. Les fichiers n'existent pas en local — la fiche se
-- consulte, le téléchargement échouera. C'est assumé pour un jeu de
-- démonstration : inventer un PDF serait plus trompeur qu'un lien mort.
--
-- `acceptation_diffusion` à TRUE : sans elle, le contributeur n'a pas cédé le
-- droit de diffusion et la fiche ne devrait pas être publiée.

INSERT INTO media_content.livre (
    titre, slug, description, document_pdf_url, type_document, acces,
    info_auteur, date_publication, acceptation_diffusion, langue,
    nombre_pages, etat, cree_par)
SELECT d.titre, d.slug, d.description, d.pdf, d.type_doc,
       d.acces::media_content.acces_livre, d.auteur, d.date_pub::date,
       TRUE, d.langue, d.pages, 'publie', u.id
FROM (VALUES
    ('Manuel pratique de l''agriculture de conservation', 'manuel-agriculture-conservation',
     'Un guide de terrain pour les coopératives : rotation, couverture des sols, semis direct. Écrit à partir de dix ans d''accompagnement de fermes familiales en Afrique de l''Ouest.',
     '/uploads/documents/manuel-agriculture-conservation.pdf', 'manuel', 'lecture_telechargement',
     'Ibrahima Fall, ingénieur agronome', '2026-02-10', 'fr', 148,
     'ibrahima.fall@uafricas.org'),
    ('Histoires du fleuve : contes recueillis en pays malinké', 'histoires-du-fleuve',
     'Vingt-huit contes transcrits auprès de conteurs de six villages, avec leur version en malinké en regard. La transcription conserve les formules d''ouverture et de clôture, souvent perdues à l''écrit.',
     '/uploads/documents/histoires-du-fleuve.pdf', 'recueil', 'lecture_seule',
     'Moussa Coulibaly, griot', '2025-11-22', 'fr', 212,
     'moussa.coulibaly@uafricas.org'),
    ('Enseigner en langue maternelle : douze écoles, trois ans', 'enseigner-langue-maternelle',
     'Le compte rendu d''une expérimentation bilingue menée sur trois années scolaires. Les résultats, les échecs, et ce que les enseignants en ont dit eux-mêmes.',
     '/uploads/documents/enseigner-langue-maternelle.pdf', 'rapport', 'lecture_telechargement',
     'Fatou Traoré, formatrice', '2026-05-03', 'fr', 96,
     'fatou.traore@uafricas.org'),
    ('Concevoir pour une connexion intermittente', 'concevoir-connexion-intermittente',
     'Les choix techniques qui font qu''une application reste utilisable quand le réseau tombe. Destiné aux développeurs qui conçoivent pour le continent et non pour un bureau câblé.',
     '/uploads/documents/concevoir-connexion-intermittente.pdf', 'guide', 'lecture_telechargement',
     'Ousmane Sow, architecte logiciel', '2026-06-18', 'fr', 74,
     'ousmane.sow@uafricas.org')
) AS d(titre, slug, description, pdf, type_doc, acces, auteur, date_pub, langue, pages, email)
JOIN iam.utilisateur u ON u.email = d.email AND u.deleted_at IS NULL
WHERE NOT EXISTS (
    SELECT 1 FROM media_content.livre l WHERE l.slug = d.slug AND l.deleted_at IS NULL);


-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANTIVES — initiatives publiées (innovation.africantive)
-- ════════════════════════════════════════════════════════════════════════════

INSERT INTO innovation.africantive (
    titre, slug, description, domaine_id, pays_id, ville, etat, cree_par,
    site_web_url, contact1_courriel)
SELECT d.titre, d.slug, d.description, ds.id, p.id, d.ville,
       'publie'::innovation.etat_contenu, u.id, d.site, d.courriel
FROM (VALUES
    ('Bibliothèques de quartier de Bouaké', 'bibliotheques-quartier-bouake',
     'Six bibliothèques ouvertes dans des quartiers sans librairie ni médiathèque. Le fonds est constitué par dons et par échanges entre les six sites, ce qui évite d''immobiliser un budget d''achat.',
     'Culture & Patrimoine', 'CI', 'Bouaké',
     'https://exemple.org/bibliotheques-bouake', 'contact@exemple.org',
     'moussa.coulibaly@uafricas.org'),
    ('Coopérative maraîchère de Ségou', 'cooperative-maraichere-segou',
     'Quatre-vingts productrices mutualisent l''irrigation goutte-à-goutte et la vente. La mise en commun du transport a fait plus pour leurs marges que n''importe quelle aide directe.',
     'Agriculture', 'ML', 'Ségou',
     NULL, 'cooperative@exemple.org',
     'ibrahima.fall@uafricas.org'),
    ('Atelier de réparation solidaire de Dakar', 'atelier-reparation-dakar',
     'Un atelier où l''on répare téléphones, ordinateurs et petit électroménager, et où l''on apprend à le faire. La formation est gratuite pour qui accepte d''y donner ensuite quelques heures.',
     'Artisanat', 'SN', 'Dakar',
     'https://exemple.org/atelier-dakar', NULL,
     'ousmane.sow@uafricas.org'),
    ('Radio scolaire de Kumasi', 'radio-scolaire-kumasi',
     'Une radio animée par des élèves de trois lycées, diffusée sur la bande FM locale. Les émissions sont préparées en classe : la radio est le support, pas la finalité.',
     'Éducation', 'GH', 'Kumasi',
     NULL, 'radio@exemple.org',
     'fatou.traore@uafricas.org')
) AS d(titre, slug, description, domaine, iso, ville, site, courriel, email)
JOIN iam.utilisateur u ON u.email = d.email AND u.deleted_at IS NULL
JOIN shared.pays p ON p.code_iso2 = d.iso
LEFT JOIN shared.domaine_secteur ds ON ds.nom = d.domaine
WHERE NOT EXISTS (
    SELECT 1 FROM innovation.africantive a WHERE a.slug = d.slug AND a.deleted_at IS NULL);


-- ════════════════════════════════════════════════════════════════════════════
-- AFRICAMOOD — radio et Vidafrica
-- ════════════════════════════════════════════════════════════════════════════
--
-- La TÉLÉ est déjà couverte par `seeds/009_demo_medias.sql` (3 chaînes,
-- 4 émissions, 13 épisodes, avec de vraies URL YouTube vérifiées). On ne la
-- redouble pas ici : deux jeux concurrents sur les mêmes tables divergeraient.
-- Ne manquaient que la radio et Vidafrica.
--
-- Les flux et fichiers pointés n'existent pas en local : les fiches se
-- consultent, la lecture échouera. Inventer une URL de flux qui semble
-- fonctionner serait plus trompeur qu'un lien manifestement absent.

INSERT INTO media_content.station_radio (
    nom, slug, description, image_couverture_url, genre, genres_liste,
    pays_id, ville, type_station, etat, origine_publication, cree_par,
    contact_email, contact_site_web)
SELECT d.nom, d.slug, d.description, NULL, d.genre, d.genres,
       p.id, d.ville, d.type_st::media_content.type_station, 'publie',
       d.origine, a.id, d.courriel, d.site
FROM (VALUES
    ('Africans Radio Panafricaine', 'africans-radio-panafricaine',
     'La station de la plateforme : débats panafricains, musiques du continent et chroniques de la diaspora, en français et en anglais.',
     'Généraliste', ARRAY['Généraliste', 'Débat', 'Musique'],
     'CI', 'Abidjan', 'internationale', 'africans',
     'radio@exemple.org', 'https://exemple.org/africans-radio'),
    ('Radio Sahel Voix', 'radio-sahel-voix',
     'Une radio de proximité qui émet en bambara, en peul et en français. L''information agricole y tient la première place, parce que c''est ce que les auditeurs demandent.',
     'Proximité', ARRAY['Proximité', 'Information', 'Agriculture'],
     'ML', 'Ségou', 'locale', 'territoire',
     'contact@exemple.org', NULL),
    ('Teranga FM', 'teranga-fm',
     'Musique sénégalaise et magazines culturels. La programmation fait une large place aux artistes qui n''ont pas encore de maison de disques.',
     'Musique', ARRAY['Musique', 'Culture'],
     'SN', 'Dakar', 'nationale', 'territoire',
     NULL, 'https://exemple.org/teranga-fm')
) AS d(nom, slug, description, genre, genres, iso, ville, type_st, origine, courriel, site)
JOIN shared.pays p ON p.code_iso2 = d.iso
CROSS JOIN LATERAL (
    SELECT u.id FROM iam.utilisateur u
    JOIN iam.utilisateur_role ur ON ur.utilisateur_id = u.id
    JOIN iam.role r ON r.id = ur.role_id
    WHERE r.nom IN ('Administrateur', 'Super Administrateur') AND u.deleted_at IS NULL
    ORDER BY u.created_at LIMIT 1) a
WHERE NOT EXISTS (
    SELECT 1 FROM media_content.station_radio s WHERE s.slug = d.slug AND s.deleted_at IS NULL);


-- Émissions rattachées à leur station, par slug.
INSERT INTO media_content.emission_radio (
    station_id, titre, slug, description, info_animateur, langue,
    categorie_radio, cadence, etat, cree_par)
SELECT s.id, d.titre, d.slug, d.description, d.animateur, d.langue,
       d.categorie::media_content.categorie_radio, d.cadence, 'publie', s.cree_par
FROM (VALUES
    ('africans-radio-panafricaine', 'Le Grand Débat panafricain', 'grand-debat-panafricain',
     'Chaque semaine, deux invités de deux pays différents confrontent leurs lectures d''une même question continentale.',
     'Animé par Aminata Ndiaye', 'fr', 'radio_africans_international', 'hebdomadaire'),
    ('africans-radio-panafricaine', 'Réveil Continental', 'reveil-continental',
     'La matinale : revue de presse des grands titres africains, météo agricole et agenda culturel.',
     'Animé par Ousmane Sow', 'fr', 'radio_africans_international', 'quotidienne'),
    ('radio-sahel-voix', 'Champs et Saisons', 'champs-et-saisons',
     'Conseils de campagne, prix des marchés et réponses aux questions des auditeurs, en bambara et en français.',
     'Animé par Ibrahima Fall', 'bm', 'radio_nationale_local', 'hebdomadaire'),
    ('teranga-fm', 'Scène Ouverte', 'scene-ouverte',
     'Une heure consacrée aux artistes sénégalais qui s''autoproduisent. Un titre, un entretien, pas de sélection par le catalogue.',
     'Animé par Fatou Traoré', 'fr', 'radio_nationale_national', 'hebdomadaire')
) AS d(station_slug, titre, slug, description, animateur, langue, categorie, cadence)
JOIN media_content.station_radio s ON s.slug = d.station_slug AND s.deleted_at IS NULL
WHERE NOT EXISTS (
    SELECT 1 FROM media_content.emission_radio e WHERE e.slug = d.slug AND e.deleted_at IS NULL);


-- Épisodes. `a_la_une` reste FAUX partout : un index unique borne les mises en
-- avant, et un seed n'a pas à décider de la vitrine.
INSERT INTO media_content.episode_radio (
    emission_id, titre, slug, description, audio_url, numero_episode, ordre,
    duree_minutes, etat, valide_par, valide_at, cree_par)
SELECT e.id, d.titre, d.slug, d.description,
       '/uploads/audios/' || d.slug || '.mp3',
       d.numero, d.numero, d.duree,
       'publie', e.cree_par, NOW() - INTERVAL '10 days', e.cree_par
FROM (VALUES
    ('grand-debat-panafricain', 'La monnaie unique, promesse ou mirage ?', 'debat-monnaie-unique',
     'Un économiste ghanéen et une universitaire ivoirienne débattent des conditions préalables à une monnaie commune.', 1, 52),
    ('grand-debat-panafricain', 'Faut-il enseigner en langue maternelle ?', 'debat-langue-maternelle',
     'Retour sur trois années d''expérimentation bilingue, avec les enseignants qui l''ont menée.', 2, 48),
    ('reveil-continental', 'Revue de presse du 12 mars', 'reveil-12-mars',
     'Les titres de six quotidiens du continent, et ce qu''ils ne disent pas.', 1, 25),
    ('champs-et-saisons', 'Préparer la saison sèche', 'champs-saison-seche',
     'Couverture des sols, stockage de l''eau, choix des variétés : ce qui se décide maintenant.', 1, 38),
    ('scene-ouverte', 'Autoproduction : le premier disque', 'scene-premier-disque',
     'Trois artistes racontent le financement, l''enregistrement et la diffusion de leur premier album.', 1, 61)
) AS d(emission_slug, titre, slug, description, numero, duree)
JOIN media_content.emission_radio e ON e.slug = d.emission_slug AND e.deleted_at IS NULL
WHERE NOT EXISTS (
    SELECT 1 FROM media_content.episode_radio ep WHERE ep.slug = d.slug AND ep.deleted_at IS NULL);


-- Vidafrica. `decharge_droits` à TRUE : sans elle, le contributeur n'a pas
-- déclaré détenir les droits, et la vidéo ne devrait pas être publiée.
INSERT INTO media_content.video (
    titre, slug, description, fichier_video_url, vignette_url, duree_secondes,
    format_video, etat, cree_par, territoires, decharge_droits, auteur_reel,
    langue_originale)
SELECT d.titre, d.slug, d.description, d.fichier, NULL, d.duree,
       'mp4', 'publie', u.id, d.territoires, TRUE, d.auteur, d.langue
FROM (VALUES
    ('Le bogolan, du coton au motif', 'bogolan-du-coton-au-motif',
     'Le parcours complet d''une pièce de bogolan, filmé sur trois semaines dans un atelier de Ségou : filature, teinture à la boue fermentée, tracé des motifs.',
     '/uploads/videos/bogolan-du-coton-au-motif.mp4', 812,
     ARRAY['Mali'], 'Mariam Touré', 'bm'),
    ('Semer sans labourer', 'semer-sans-labourer',
     'Une démonstration de semis direct sur couverture végétale, avec les rendements comparés de deux parcelles voisines sur trois campagnes.',
     '/uploads/videos/semer-sans-labourer.mp4', 645,
     ARRAY['Mali', 'Burkina Faso'], 'Ibrahima Fall', 'fr'),
    ('Une classe en deux langues', 'une-classe-en-deux-langues',
     'Une matinée dans une classe de CP bilingue wolof-français. On y voit ce que les rapports décrivent mal : le moment où un enfant comprend.',
     '/uploads/videos/une-classe-en-deux-langues.mp4', 1174,
     ARRAY['Sénégal'], 'Fatou Traoré', 'wo')
) AS d(titre, slug, description, fichier, duree, territoires, auteur, langue)
CROSS JOIN LATERAL (
    SELECT u2.id FROM iam.utilisateur u2
    JOIN iam.utilisateur_role ur ON ur.utilisateur_id = u2.id
    JOIN iam.role r ON r.id = ur.role_id
    WHERE r.nom IN ('Administrateur', 'Super Administrateur') AND u2.deleted_at IS NULL
    ORDER BY u2.created_at LIMIT 1) u
WHERE NOT EXISTS (
    SELECT 1 FROM media_content.video v WHERE v.slug = d.slug AND v.deleted_at IS NULL);
