-- ============================================================================
-- Jeu de démonstration de la VITRINE TÉLÉ (/medias/tele)
--
-- But : pouvoir juger la disposition et le rendu graphique de la page sur un
-- volume réaliste — vedette plein écran, barre de filtres nourrie, une section
-- par chaîne avec identité, équipe, bandeau de diffusion, grille de cartes,
-- compteurs d'interaction, pagination au défilement.
--
-- Écrit en SQL et non par l'API, comme 009 et 010 : les invariants que l'API
-- défend sont reproduits ici explicitement (états, unicité des mises en avant,
-- exclusivité de la couverture, cohérence de la décision de modération).
--
-- ── Images de simulation ────────────────────────────────────────────────────
-- Aucune image n'est téléversée : les couvertures de chaînes et de programmes
-- pointent sur `picsum.photos` (photographies libres, déterministes par graine
-- — la même chaîne garde la même image d'une exécution à l'autre), et les
-- couvertures d'épisodes sur les vignettes officielles YouTube des vidéos
-- réellement référencées. Les identifiants de vidéos sont repris du jeu 009,
-- où ils avaient été vérifiés un à un via l'endpoint oEmbed : aucun lecteur ne
-- reste noir.
--
-- ── Ce que le jeu couvre volontairement ─────────────────────────────────────
--   • 12 chaînes neuves + 4 conservées → 16 sections, soit 3 pages de 6 ;
--   • les deux origines (`africans` / `territoire`) et les deux couvertures
--     (continentale / territoires nommés) → la barre de filtres a de quoi
--     mordre ;
--   • une chaîne SANS aucun programme (Sawa TV) → FR-005 ;
--   • une chaîne AU-DELÀ du plafond de 30 programmes (Panorama Continental,
--     dont les 31 programmes reçoivent ici de vrais titres et de vraies
--     couvertures) → FR-008, « Voir les 31 programmes » ;
--   • une grille couvrant les 24 heures sur chaque chaîne → « En ce moment »
--     et « À suivre » sont peuplés quelle que soit l'heure de la démonstration ;
--   • `test-user@test.com` est propriétaire de deux chaînes → le bouton
--     « Gérer ma chaîne » apparaît quand il est connecté.
--
-- ── Effet de bord assumé ────────────────────────────────────────────────────
-- Les chaînes de test bruyantes (« test chaine 2 », « Chaine Vide »…) sont
-- basculées en `brouillon` : elles n'ont ni couverture ni description et
-- salissaient la vitrine. Rien n'est supprimé — `011_demo_tele_vitrine_purge.sql`
-- les republie.
--
-- Idempotent : rejouable sans doublon (ON CONFLICT sur les slugs, enfants
-- réécrits par remplacement intégral).
-- ============================================================================

\set ON_ERROR_STOP on

BEGIN;

-- ────────────────────────────────────────────────────────────────────────────
-- Définitions déclaratives. Les tenir dans des tables temporaires plutôt que
-- dans des tableaux plpgsql laisse le contenu éditorial lisible et modifiable
-- sans toucher à la mécanique d'insertion, qui suit.
-- ────────────────────────────────────────────────────────────────────────────

CREATE TEMP TABLE tmp_chaine (
    ordre         INT,
    nom           TEXT,
    slug          TEXT,
    description   TEXT,
    categorie     TEXT,
    pays          TEXT,          -- NULL pour une chaîne de la plateforme
    langue        TEXT,
    en_direct     BOOLEAN,
    origine       TEXT,          -- 'africans' | 'territoire'
    continentale  BOOLEAN,
    themes        TEXT[],        -- shared.categorie, contexte 'media'
    territoires   TEXT[],        -- ignoré si continentale
    email         TEXT,
    telephone     TEXT,
    whatsapp      TEXT,
    site_web      TEXT,
    adresse       TEXT
) ON COMMIT DROP;

INSERT INTO tmp_chaine VALUES
(1, 'Africans Télé International', 'africans-tele-international',
 'La chaîne généraliste de la plateforme, diffusée d''un bout à l''autre du continent et dans la diaspora. Information continue, grands entretiens, magazines économiques et documentaires de création : Africans Télé International raconte l''Afrique depuis l''Afrique, en français, sans intermédiaire. Une rédaction répartie entre Dakar, Nairobi et Johannesburg, et des correspondants dans vingt-deux territoires.',
 'generaliste', NULL, 'Français', TRUE, 'africans', TRUE,
 ARRAY['Journal télévisé','Débats et analyses','Grandes interviews','Diaspora'], NULL,
 'redaction@africans-world.org', '+221 33 800 12 12', '+221 77 800 12 12',
 'https://www.africans-world.org/tele', 'Immeuble Téranga, Route de Ngor, Dakar'),

(2, 'Sahel Info TV', 'sahel-info-tv',
 'L''information du Sahel, traitée depuis le Sahel. Sahel Info TV consacre l''essentiel de son antenne au terrain : sécurité alimentaire, mobilité pastorale, chantiers de la transition, vie des communes rurales. Une rédaction de vingt-huit journalistes basée à Ouagadougou, avec des bureaux à Bobo-Dioulasso et à Bamako.',
 'info', 'Burkina Faso', 'Français', TRUE, 'territoire', FALSE,
 ARRAY['Journal télévisé','Investigations','Débats et analyses','Agriculture'],
 ARRAY['Burkina Faso','Mali'],
 'contact@sahelinfo.tv', '+226 25 30 44 10', NULL, 'https://sahelinfo.tv',
 'Avenue Kwame Nkrumah, Ouagadougou'),

(3, 'Lagos Business Channel', 'lagos-business-channel',
 'Le rendez-vous quotidien de l''économie ouest-africaine : marchés, matières premières, fintech, capital-investissement. Lagos Business Channel décrypte les chiffres sans jargon et donne la parole à ceux qui construisent — fondateurs, régulateurs, investisseurs. Diffusion en anglais, sous-titrage français en préparation.',
 'generaliste', 'Nigeria', 'Anglais', FALSE, 'territoire', FALSE,
 ARRAY['Émissions économiques','Grandes interviews','Débats et analyses'],
 ARRAY['Nigeria','Ghana'],
 'desk@lagosbusiness.tv', '+234 1 270 88 00', '+234 803 270 88 00',
 'https://lagosbusiness.tv', '14 Karimu Kotun St, Victoria Island, Lagos'),

(4, 'Nil Sport', 'nil-sport',
 'Toute la passion du sport africain, du championnat de quartier aux grands rendez-vous continentaux. Nil Sport suit les clubs, les fédérations et les athlètes, et consacre chaque mois un long format aux légendes de la CAN. Antenne en arabe, avec commentaire français sur les rencontres internationales.',
 'sport', 'Égypte', 'Arabe', TRUE, 'territoire', FALSE,
 ARRAY['Sport','Grandes interviews'],
 ARRAY['Égypte','Tunisie','Maroc'],
 'sport@nilsport.tv', '+20 2 2735 66 00', NULL, 'https://nilsport.tv',
 'Corniche El Nil, Le Caire'),

(5, 'Kilimandjaro Nature', 'kilimandjaro-nature',
 'Une chaîne de documentaires animaliers et environnementaux produite en Afrique de l''Est. Kilimandjaro Nature filme les parcs, les corridors de migration et les communautés qui vivent à leur lisière, et accompagne les programmes de conservation sans les idéaliser. Chaque saison, une série est co-produite avec des étudiants en biologie de Dar es Salaam et de Nairobi.',
 'education', 'Tanzanie', 'Anglais', FALSE, 'territoire', FALSE,
 ARRAY['Environnement','Tourisme','Agriculture','Éducation'],
 ARRAY['Tanzanie','Kenya','Ouganda'],
 'hello@kilimanjaronature.tv', '+255 22 213 40 00', '+255 754 213 400',
 'https://kilimanjaronature.tv', 'Msasani Peninsula, Dar es Salaam'),

(6, 'Kin Musique TV', 'kin-musique-tv',
 'Kinshasa bat au rythme de la rumba, du ndombolo et, depuis peu, de l''afro-drill. Kin Musique TV filme les répétitions, les scènes de quartier et les grandes salles, et consacre un rendez-vous mensuel aux archives sonores du fleuve. Une chaîne née d''un collectif de vidéastes de Bandal.',
 'musique', 'République Démocratique du Congo', 'Français', TRUE, 'territoire', FALSE,
 ARRAY['Divertissement','Culture','Cinéma'],
 ARRAY['République Démocratique du Congo','République du Congo'],
 'studio@kinmusique.tv', '+243 81 500 77 20', '+243 81 500 77 20',
 'https://kinmusique.tv', 'Avenue Kasa-Vubu, Kinshasa'),

(7, 'Atlas Maghreb TV', 'atlas-maghreb-tv',
 'Culture, patrimoine et art de vivre du Maghreb, du Rif aux oasis du Sud. Atlas Maghreb TV met en avant les métiers d''artisanat, les cuisines régionales et la littérature contemporaine, avec un magazine de voyage mensuel tourné en caravane. Antenne bilingue arabe-français.',
 'generaliste', 'Maroc', 'Arabe', FALSE, 'territoire', FALSE,
 ARRAY['Culture','Vie pratique','Tourisme'],
 ARRAY['Maroc','Algérie','Tunisie'],
 'contact@atlasmaghreb.tv', '+212 522 48 10 10', NULL, 'https://atlasmaghreb.tv',
 'Boulevard d''Anfa, Casablanca'),

(8, 'Cap Sud Divertissement', 'cap-sud-divertissement',
 'Le divertissement d''Afrique australe : talk-show nocturne, cinéma indépendant, humour de township et compétition de danse. Cap Sud Divertissement produit l''essentiel de sa grille en interne, dans ses studios du Cap, et travaille avec des troupes de Soweto et de Gugulethu.',
 'divertissement', 'Afrique du Sud', 'Anglais', FALSE, 'territoire', FALSE,
 ARRAY['Divertissement','Cinéma','Talk-show'],
 ARRAY['Afrique du Sud'],
 'onair@capsud.tv', '+27 21 421 55 00', '+27 82 421 55 00', 'https://capsud.tv',
 'Long Street, Le Cap'),

(9, 'Téranga Jeunesse', 'teranga-jeunesse',
 'La première chaîne sénégalaise entièrement consacrée aux enfants et aux adolescents. Téranga Jeunesse alterne éveil, contes wolof sous-titrés, révisions du baccalauréat et concours de sciences, sans publicité et sans écran de veille. Conçue avec des enseignants du public.',
 'education', 'Sénégal', 'Français', FALSE, 'territoire', FALSE,
 ARRAY['Émissions jeunesse','Éducation','Culture'],
 ARRAY['Sénégal','Mali'],
 'bonjour@terangajeunesse.tv', '+221 33 869 40 40', NULL,
 'https://terangajeunesse.tv', 'Sacré-Cœur 3, Dakar'),

(10, 'Abidjan Talk TV', 'abidjan-talk-tv',
 'On parle, on écoute, on tranche. Abidjan Talk TV fait de la parole publique son format : débat de société en direct chaque soir, ligne ouverte aux téléspectateurs l''après-midi, et un magazine hebdomadaire consacré aux initiatives citoyennes des communes du district.',
 'divertissement', 'Côte d''Ivoire', 'Français', TRUE, 'territoire', FALSE,
 ARRAY['Talk-show','Émissions interactives','Émissions citoyennes','Débats et analyses'],
 ARRAY['Côte d''Ivoire','Ghana','Togo'],
 'studio@abidjantalk.tv', '+225 27 22 41 30 30', '+225 07 41 30 30 30',
 'https://abidjantalk.tv', 'Rue des Jardins, Cocody, Abidjan'),

(11, 'Foi & Espérance TV', 'foi-esperance-tv',
 'Une antenne œcuménique camerounaise : offices, enseignements, accompagnement des familles et magazine de santé communautaire. Foi & Espérance TV donne la parole aux aumôneries d''hôpitaux et aux associations de quartier, et diffuse ses cultes en français et en douala.',
 'religieux', 'Cameroun', 'Français', FALSE, 'territoire', FALSE,
 ARRAY['Religion et spiritualité','Santé','Vie pratique'],
 ARRAY['Cameroun','Tchad'],
 'accueil@foiesperance.tv', '+237 233 42 18 18', '+237 699 42 18 18',
 'https://foiesperance.tv', 'Rue Joss, Akwa, Douala'),

-- Chaîne volontairement SANS programme : la vitrine doit l'afficher quand même,
-- identité et équipe comprises (FR-005).
(12, 'Sawa TV', 'sawa-tv',
 'Chaîne béninoise en cours de lancement. L''équipe est constituée, les studios de Cotonou sont livrés, la grille sera annoncée à la rentrée. En attendant, Sawa TV publie ses intentions éditoriales et recrute ses premiers animateurs sur la plateforme.',
 'generaliste', 'Bénin', 'Français', FALSE, 'territoire', FALSE,
 ARRAY['Culture','Émissions citoyennes'],
 ARRAY['Bénin','Togo'],
 'contact@sawa.tv', '+229 21 30 55 55', NULL, 'https://sawa.tv',
 'Quartier Ganhi, Cotonou');


CREATE TEMP TABLE tmp_emission (
    chaine_slug   TEXT,
    ordre         INT,
    titre         TEXT,
    slug          TEXT,
    description   TEXT,
    cadence       TEXT,        -- quotidienne | hebdomadaire | mensuelle | ponctuelle
    theme         TEXT,        -- shared.categorie, contexte 'media'
    animateur     TEXT,
    producteur    TEXT,
    nb_episodes   INT
) ON COMMIT DROP;

INSERT INTO tmp_emission VALUES
-- ── Africans Télé International ─────────────────────────────────────────────
('africans-tele-international', 1, 'Le Journal Panafricain', 'le-journal-panafricain',
 'Vingt-six minutes pour faire le tour du continent, chaque soir à 20 h. Les correspondants ouvrent le journal ; les rédactions de Dakar, Nairobi et Johannesburg se relaient sur le plateau.',
 'quotidienne', 'Journal télévisé', 'Aminata Diallo et Kwame Mensah', 'Rédaction Africans Télé', 4),
('africans-tele-international', 2, 'Grand Angle', 'grand-angle-ati',
 'Un sujet, quatre invités, aucune coupure publicitaire. Grand Angle prend le temps d''un désaccord argumenté sur les grandes décisions qui engagent le continent.',
 'hebdomadaire', 'Débats et analyses', 'Nadia Benali', 'Studio Grand Angle', 3),
('africans-tele-international', 3, 'Face au Continent', 'face-au-continent',
 'L''entretien de la semaine, sans montage complaisant : chefs d''État, artistes, scientifiques, syndicalistes. Trente minutes, une seule caméra, aucune question interdite.',
 'hebdomadaire', 'Grandes interviews', 'Sékou Camara', 'Africans Télé Productions', 3),
('africans-tele-international', 4, 'Diaspora Connect', 'diaspora-connect',
 'Le magazine des Africains d''ailleurs : retours au pays, transferts, double nationalité, réussites et désillusions. Tourné en alternance à Paris, Montréal, Londres et Dubaï.',
 'hebdomadaire', 'Diaspora', 'Zainab Abubakar', 'Africans Télé Productions', 3),
('africans-tele-international', 5, 'Afrique 2050', 'afrique-2050',
 'La grande enquête mensuelle de la chaîne. Démographie, énergie, souveraineté alimentaire, villes : chaque numéro instruit une trajectoire à trente ans, chiffres et contradicteurs à l''appui.',
 'mensuelle', 'Investigations', 'Fatou Traoré', 'Unité documentaire Africans', 3),
('africans-tele-international', 6, 'Le Zoom Éco', 'le-zoom-eco',
 'Cinq minutes d''économie expliquée simplement, juste après le journal : une courbe, un chiffre, une conséquence concrète pour le foyer.',
 'quotidienne', 'Émissions économiques', 'Mamadou Sow', 'Rédaction Africans Télé', 3),

-- ── Sahel Info TV ───────────────────────────────────────────────────────────
('sahel-info-tv', 1, 'Le 20 Heures du Sahel', 'le-20-heures-du-sahel',
 'Le rendez-vous d''information de la chaîne : l''actualité du Burkina Faso et de ses voisins, avec un module quotidien consacré aux communes rurales.',
 'quotidienne', 'Journal télévisé', 'Salif Ouédraogo', 'Rédaction Sahel Info', 4),
('sahel-info-tv', 2, 'Enquête Sahel', 'enquete-sahel',
 'Un mois d''enquête pour un sujet. Filières d''or, marchés du bétail, contrats miniers : Enquête Sahel documente ce que les communiqués officiels résument en une ligne.',
 'mensuelle', 'Investigations', 'Hawa Barry', 'Cellule investigation', 3),
('sahel-info-tv', 3, 'Parole Citoyenne', 'parole-citoyenne',
 'Chaque samedi, une commune reçoit le plateau. Habitants, élus et services techniques répondent ensemble des chantiers annoncés l''année précédente.',
 'hebdomadaire', 'Émissions citoyennes', 'Boubacar Sangaré', 'Sahel Info Productions', 3),
('sahel-info-tv', 4, 'Météo des Champs', 'meteo-des-champs',
 'Prévisions agricoles, calendrier des semis et conseils phytosanitaires, en français et en mooré. Réalisé avec l''institut national de l''environnement et de la recherche agricole.',
 'quotidienne', 'Agriculture', 'Awa Konaté', 'Sahel Info Productions', 3),

-- ── Lagos Business Channel ──────────────────────────────────────────────────
('lagos-business-channel', 1, 'Naija Business Daily', 'naija-business-daily',
 'Ouverture et clôture des marchés, taux, carburant, inflation : l''essentiel économique nigérian en trente minutes, chaque jour ouvré.',
 'quotidienne', 'Émissions économiques', 'Ngozi Eze', 'LBC Newsroom', 4),
('lagos-business-channel', 2, 'Founders', 'founders-lbc',
 'Un fondateur, une trajectoire, un bilan comptable ouvert. Founders reçoit ceux qui ont levé, ceux qui ont échoué, et ceux qui ont racheté les seconds.',
 'hebdomadaire', 'Grandes interviews', 'Emeka Nwosu', 'LBC Studios', 3),
('lagos-business-channel', 3, 'Market Watch Africa', 'market-watch-africa',
 'Le suivi quotidien des places africaines — Lagos, Nairobi, Johannesburg, Casablanca — et des matières premières qui les font bouger.',
 'quotidienne', 'Émissions économiques', 'Chidi Okonkwo', 'LBC Newsroom', 3),
('lagos-business-channel', 4, 'The Deal Room', 'the-deal-room',
 'Le long format mensuel du capital-investissement africain : une opération décortiquée, de la lettre d''intention au closing, avec les deux parties sur le plateau.',
 'mensuelle', 'Émissions économiques', 'Ngozi Eze', 'LBC Studios', 3),

-- ── Nil Sport ───────────────────────────────────────────────────────────────
('nil-sport', 1, 'Le Grand Stade', 'le-grand-stade',
 'Résultats, analyses et coulisses des championnats nord-africains et continentaux, tous les soirs après les rencontres.',
 'quotidienne', 'Sport', 'Youssef Hassan', 'Nil Sport Productions', 4),
('nil-sport', 2, 'CAN Légendes', 'can-legendes',
 'Chaque mois, une édition de la Coupe d''Afrique des Nations racontée par ceux qui l''ont jouée. Archives restaurées et entretiens inédits.',
 'mensuelle', 'Sport', 'Rania Zerrouki', 'Unité archives Nil Sport', 3),
('nil-sport', 3, 'Athlétisme Continental', 'athletisme-continental',
 'Le magazine hebdomadaire de la piste et du fond : meetings, qualifications olympiques, centres de formation d''Addis-Abeba à Eldoret.',
 'hebdomadaire', 'Sport', 'Kaleb Tesfaye', 'Nil Sport Productions', 3),
('nil-sport', 4, 'Troisième Mi-temps', 'troisieme-mi-temps',
 'Le débat d''après-match, entre supporters, anciens joueurs et arbitres. Ton libre, plateau ouvert, prolongations fréquentes.',
 'hebdomadaire', 'Sport', 'Youssef Hassan', 'Nil Sport Productions', 3),

-- ── Kilimandjaro Nature ─────────────────────────────────────────────────────
('kilimandjaro-nature', 1, 'Terres Sauvages', 'terres-sauvages',
 'Série documentaire tournée dans les parcs d''Afrique de l''Est. Chaque épisode suit une espèce sur une saison complète, sans musique ajoutée ni mise en scène.',
 'hebdomadaire', 'Environnement', 'Tendai Chikwanda', 'Kilimandjaro Films', 4),
('kilimandjaro-nature', 2, 'Les Gardiens du Parc', 'les-gardiens-du-parc',
 'Le long format mensuel consacré aux rangers, aux vétérinaires de brousse et aux communautés riveraines qui vivent du parc autant qu''elles le protègent.',
 'mensuelle', 'Environnement', 'Lerato Molefe', 'Kilimandjaro Films', 3),
('kilimandjaro-nature', 3, 'Route des Épices', 'route-des-epices',
 'De Zanzibar à Mombasa, un magazine de voyage qui suit les cultures d''épices, les marchés et les cuisines côtières swahilies.',
 'hebdomadaire', 'Tourisme', 'Sipho Dlamini', 'Kilimandjaro Films', 3),
('kilimandjaro-nature', 4, 'Agriculture Demain', 'agriculture-demain',
 'Agroécologie, irrigation goutte-à-goutte, semences paysannes : ce qui marche déjà dans les exploitations de la vallée du Rift, expliqué par ceux qui l''appliquent.',
 'hebdomadaire', 'Agriculture', 'Tendai Chikwanda', 'Kilimandjaro Films', 3),

-- ── Kin Musique TV ──────────────────────────────────────────────────────────
('kin-musique-tv', 1, 'Rumba Éternelle', 'rumba-eternelle',
 'L''histoire vivante de la rumba congolaise, inscrite au patrimoine immatériel de l''humanité : orchestres, guitaristes, arrangeurs, et les salles où tout s''est joué.',
 'hebdomadaire', 'Culture', 'Djibril Sylla', 'Collectif Bandal', 4),
('kin-musique-tv', 2, 'Top Afro Charts', 'top-afro-charts',
 'Le classement quotidien des titres les plus écoutés sur le continent, commenté sans complaisance et illustré par les clips.',
 'quotidienne', 'Divertissement', 'Adjoa Owusu', 'Kin Musique Studios', 3),
('kin-musique-tv', 3, 'Studio Live Kin', 'studio-live-kin',
 'Un groupe, une prise, aucune retouche. Studio Live Kin enregistre chaque semaine une session acoustique dans les locaux de la chaîne.',
 'hebdomadaire', 'Divertissement', 'Moussa Bah', 'Kin Musique Studios', 3),
('kin-musique-tv', 4, 'Les Voix du Fleuve', 'les-voix-du-fleuve',
 'Le rendez-vous mensuel des archives sonores du bassin du Congo : bandes retrouvées, restaurations, et rencontres avec les familles des interprètes.',
 'mensuelle', 'Culture', 'Djibril Sylla', 'Unité archives Kin', 3),

-- ── Atlas Maghreb TV ────────────────────────────────────────────────────────
('atlas-maghreb-tv', 1, 'Medina', 'medina-atlas',
 'Le magazine du patrimoine urbain maghrébin : médinas, fondouks, métiers d''art menacés et chantiers de restauration.',
 'hebdomadaire', 'Culture', 'Salma El Amrani', 'Atlas Productions', 4),
('atlas-maghreb-tv', 2, 'Cuisine des Terroirs', 'cuisine-des-terroirs',
 'Une recette par jour, filmée chez l''habitant, du Rif aux oasis du Sud. Les producteurs de l''ingrédient principal sont toujours à l''écran.',
 'quotidienne', 'Vie pratique', 'Nourhan Cherif', 'Atlas Productions', 3),
('atlas-maghreb-tv', 3, 'Caravane', 'caravane-atlas',
 'Un mois de route pour un numéro : le magazine de voyage de la chaîne suit les anciennes pistes caravanières et les villages qu''elles traversent encore.',
 'mensuelle', 'Tourisme', 'Youssef Benjelloun', 'Atlas Productions', 3),
('atlas-maghreb-tv', 4, 'Divan Littéraire', 'divan-litteraire',
 'La littérature maghrébine contemporaine, en arabe et en français : rentrée éditoriale, traductions, poésie orale et jeunes maisons d''édition.',
 'hebdomadaire', 'Culture', 'Salma El Amrani', 'Atlas Productions', 3),

-- ── Cap Sud Divertissement ──────────────────────────────────────────────────
('cap-sud-divertissement', 1, 'Cap Sud Tonight', 'cap-sud-tonight',
 'Le talk-show nocturne de la chaîne : monologue d''ouverture, deux invités, un groupe en live. Enregistré chaque soir devant public au Cap.',
 'quotidienne', 'Talk-show', 'Thabo Nkosi', 'Cap Sud Studios', 4),
('cap-sud-divertissement', 2, 'Ciné Kasi', 'cine-kasi',
 'Le cinéma indépendant d''Afrique australe : premiers films, courts métrages de township, entretiens avec les réalisatrices et les monteurs.',
 'hebdomadaire', 'Cinéma', 'Lerato Molefe', 'Cap Sud Studios', 3),
('cap-sud-divertissement', 3, 'Comedy Township', 'comedy-township',
 'Le plateau d''humour de la chaîne, tourné en alternance à Soweto, Gugulethu et Khayelitsha, en anglais, xhosa et zoulou.',
 'hebdomadaire', 'Divertissement', 'Sipho Dlamini', 'Cap Sud Studios', 3),
('cap-sud-divertissement', 4, 'Danse Nation', 'danse-nation',
 'La compétition mensuelle de danse urbaine : crews amateurs, chorégraphes invités, et une finale filmée en extérieur.',
 'mensuelle', 'Divertissement', 'Adjoa Owusu', 'Cap Sud Studios', 3),

-- ── Téranga Jeunesse ────────────────────────────────────────────────────────
('teranga-jeunesse', 1, 'Kids Académie', 'kids-academie',
 'Éveil, langage et calcul pour les 4-7 ans, conçu avec des enseignantes du public. Vingt minutes par jour, sans publicité ni écran de veille.',
 'quotidienne', 'Émissions jeunesse', 'Aïcha Ndiaye', 'Téranga Productions', 4),
('teranga-jeunesse', 2, 'Contes de Teranga', 'contes-de-teranga',
 'Les grands récits wolof, sérère et peul racontés par des conteurs, sous-titrés en français. Une histoire par semaine, illustrée en animation.',
 'hebdomadaire', 'Émissions jeunesse', 'Oumar Diarra', 'Téranga Productions', 3),
('teranga-jeunesse', 3, 'Bac Blanc', 'bac-blanc-teranga',
 'Révisions filmées du baccalauréat : un sujet corrigé en direct chaque samedi, avec les professeurs et les copies des candidats.',
 'hebdomadaire', 'Éducation', 'Mariam Touré', 'Téranga Productions', 3),
('teranga-jeunesse', 4, 'Petits Génies', 'petits-genies',
 'Le concours mensuel de sciences des collèges sénégalais : expériences, démonstrations et finale nationale filmée à Dakar.',
 'mensuelle', 'Éducation', 'Ibrahim Keïta', 'Téranga Productions', 3),

-- ── Abidjan Talk TV ─────────────────────────────────────────────────────────
('abidjan-talk-tv', 1, 'Le Débat d''Abidjan', 'le-debat-d-abidjan',
 'Le débat de société de la chaîne, en direct chaque soir : logement, transport, école, emploi des jeunes. Public en plateau, contradiction obligatoire.',
 'quotidienne', 'Talk-show', 'Kouassi Yao', 'Abidjan Talk Studios', 4),
('abidjan-talk-tv', 2, 'Allô la Ville', 'allo-la-ville',
 'La ligne ouverte de l''après-midi : les téléspectateurs appellent, les services concernés répondent en direct. Aucun appel n''est filtré à l''antenne.',
 'quotidienne', 'Émissions interactives', 'Aissatou Bamba', 'Abidjan Talk Studios', 3),
('abidjan-talk-tv', 3, 'Woubi Show', 'woubi-show',
 'Le divertissement du samedi soir : sketchs, musique live et jeux avec le public, enregistré au Palais de la Culture de Treichville.',
 'hebdomadaire', 'Divertissement', 'Moussa Coulibaly', 'Abidjan Talk Studios', 3),
('abidjan-talk-tv', 4, 'Génération Engagée', 'generation-engagee',
 'Le magazine des initiatives citoyennes du district : associations de quartier, collectifs de ramassage, mutuelles de santé, tontines d''investissement.',
 'hebdomadaire', 'Émissions citoyennes', 'Mariam Touré', 'Abidjan Talk Studios', 3),

-- ── Foi & Espérance TV ──────────────────────────────────────────────────────
('foi-esperance-tv', 1, 'Le Temps de la Prière', 'le-temps-de-la-priere',
 'Le rendez-vous spirituel quotidien de la chaîne : lecture, méditation et intentions, en français et en douala.',
 'quotidienne', 'Religion et spiritualité', 'Pasteur Emmanuel Nkoa', 'Foi & Espérance Productions', 3),
('foi-esperance-tv', 2, 'Paroles de Vie', 'paroles-de-vie',
 'Un enseignement hebdomadaire ouvert aux différentes confessions présentes au Cameroun, suivi d''un temps de questions des fidèles.',
 'hebdomadaire', 'Religion et spiritualité', 'Sœur Marie-Claire Etoundi', 'Foi & Espérance Productions', 3),
-- ── Chaînes héritées du jeu 009 : de quoi remplir leur section ──────────────
-- Elles n'avaient qu'un seul programme chacune, ce qui donnait des sections
-- creuses au milieu d'une vitrine par ailleurs dense. Leurs programmes
-- d'origine sont conservés ; ceux-ci s'y ajoutent.
('africans-histoire', 2, 'Routes du Sel', 'routes-du-sel',
 'Les grandes routes commerciales sahariennes, de Taghaza à Tombouctou : caravanes, monnaies, comptoirs et les villes que le sel a fait naître puis abandonnées.',
 'hebdomadaire', 'Culture', 'Oumar Diarra', 'Unité documentaire Africans', 3),
('africans-histoire', 3, 'Figures Oubliées', 'figures-oubliees',
 'Portraits de celles et ceux que les manuels ont effacés : reines, savants, navigatrices, syndicalistes. Une biographie par semaine, sourcée et contredite.',
 'hebdomadaire', 'Culture', 'Aïcha Ndiaye', 'Unité documentaire Africans', 3),
('africans-histoire', 4, 'Archives du Continent', 'archives-du-continent',
 'Le rendez-vous mensuel des fonds d''archives africains : ce qu''ils contiennent, où ils dorment, et les batailles menées pour leur restitution.',
 'mensuelle', 'Investigations', 'Sékou Camara', 'Unité archives Africans', 3),

('africans-innovation', 2, 'Fintech Lab', 'fintech-lab',
 'Paiement mobile, monnaie numérique, assurance indicielle : le magazine des services financiers africains, expliqué par ceux qui les codent et ceux qui les régulent.',
 'hebdomadaire', 'Magazine Innovation', 'Ngozi Eze', 'Africans Innovation Studio', 3),
('africans-innovation', 3, 'Énergies Nouvelles', 'energies-nouvelles',
 'Mini-réseaux solaires, cuisson propre, stockage : ce qui électrifie réellement les zones non raccordées, chantier par chantier.',
 'hebdomadaire', 'Environnement', 'Kaleb Tesfaye', 'Africans Innovation Studio', 3),
('africans-innovation', 4, 'Agri-Tech', 'agri-tech',
 'Capteurs de sol, prévision météo locale, marchés numériques : la technologie agricole africaine vue depuis les parcelles, pas depuis les salons.',
 'mensuelle', 'Agriculture', 'Awa Konaté', 'Africans Innovation Studio', 3),

('terrain-afrique', 3, 'Villes en Mutation', 'villes-en-mutation',
 'Abidjan, Kinshasa, Addis-Abeba, Le Caire : comment les métropoles africaines absorbent leur croissance — logement, transport, eau, déchets.',
 'hebdomadaire', 'Débats et analyses', 'Kouassi Yao', 'Terrain Afrique Productions', 3),
('terrain-afrique', 4, 'Frontières', 'frontieres-terrain',
 'Le magazine des zones de passage : postes-frontières, commerce informel, familles séparées par un tracé colonial jamais rediscuté.',
 'hebdomadaire', 'Investigations', 'Hawa Barry', 'Terrain Afrique Productions', 3),
('terrain-afrique', 5, 'Métiers de l''Ombre', 'metiers-de-l-ombre',
 'Ceux sans qui rien ne fonctionne et que personne ne filme : dockers, éboueurs, aiguilleurs, veilleuses de nuit. Un métier par mois, une journée entière.',
 'mensuelle', 'Émissions citoyennes', 'Fatoumata Sylla', 'Terrain Afrique Productions', 3),

('foi-esperance-tv', 3, 'Santé et Bien-être', 'sante-et-bien-etre',
 'Le magazine de santé communautaire : prévention, aumôneries d''hôpitaux, accompagnement des aidants et des malades chroniques.',
 'hebdomadaire', 'Santé', 'Dr Yacine Bah', 'Foi & Espérance Productions', 3);


-- Équipes éditoriales : trois personnes par chaîne. La fonction est saisie,
-- jamais dérivée d'un rôle applicatif — c'est une information de générique.
CREATE TEMP TABLE tmp_equipe (
    porteur_slug TEXT,
    type_porteur TEXT,
    ordre        INT,
    prenom       TEXT,
    nom          TEXT,
    fonction     TEXT,
    territoire   TEXT,
    contact      TEXT
) ON COMMIT DROP;

INSERT INTO tmp_equipe
SELECT c.slug, 'chaine_tv', g.i,
       (ARRAY['Aminata','Kwame','Fatou','Thabo','Chidi','Nadia','Sékou','Zainab','Mamadou','Lerato',
              'Yacine','Ibrahim','Awa','Kofi','Rania','Tendai','Moussa','Ngozi','Hawa','Djibril',
              'Salma','Oumar','Aïcha','Emeka','Fatoumata','Boubacar','Nourhan','Kaleb','Mariam',
              'Sipho','Adjoa','Youssef','Marie-Claire','Seydou','Bintou','Alassane'])[1 + ((c.ordre * 3 + g.i) % 36)],
       (ARRAY['Diallo','Mensah','Traoré','Nkosi','Okonkwo','Benali','Camara','Abubakar','Sow','Molefe',
              'Cherif','Keïta','Barry','Asante','El Amrani','Chikwanda','Bah','Eze','Touré','Ndiaye',
              'Zerrouki','Konaté','Sylla','Nwosu','Sangaré','Coulibaly','Hassan','Tesfaye','Diarra',
              'Dlamini','Owusu','Benjelloun','Etoundi','Ouédraogo','Fofana','Nkoa'])[1 + ((c.ordre * 7 + g.i * 5) % 36)],
       (ARRAY['Direction de l''antenne','Rédaction en chef','Production exécutive','Réalisation'])[g.i],
       COALESCE(c.pays, 'Panafricain'),
       CASE WHEN g.i = 1 THEN c.email ELSE NULL END
  FROM tmp_chaine c
 CROSS JOIN generate_series(1, 3) AS g(i);


-- ────────────────────────────────────────────────────────────────────────────
-- Mécanique d'insertion.
-- ────────────────────────────────────────────────────────────────────────────

DO $seed$
DECLARE
    v_auteur      UUID;
    v_membre      UUID;
    v_chaine      UUID;
    v_emission    UUID;
    v_pays        UUID;
    v_theme       UUID;
    c             RECORD;
    e             RECORD;
    m             RECORD;
    v_img         TEXT;
    v_compteur    INT := 0;
    v_video       TEXT;
    -- Identifiants YouTube repris du jeu 009, chacun vérifié par oEmbed :
    -- réutiliser des vidéos éprouvées évite des lecteurs noirs et muets.
    v_videos      TEXT[] := ARRAY[
        'JeVaVtr_DCE','o_JuUo3XqG4','tuCIq9NPvQ4','ecdabz94_Co','Cm5yOJc_NLo',
        'NbZaaAdf5Aw','9z52xavACQY','OvTMkEYu6l8','W_hQj5mkvaI','RV6lvELxBuo',
        'Ofn31if1Fac','KzjEhgcBvSE','r7AaktS648I','u9uf-cd63Po','jx_FiRs39s8'];
    -- Grille couvrant les 24 heures sans trou ni chevauchement : la somme des
    -- durées vaut exactement 1440 minutes, et aucun créneau ne franchit minuit
    -- (ck_creneau_pas_minuit).
    v_heures      TIME[] := ARRAY['00:00','06:00','08:00','09:30','11:00','12:00','13:00',
                                  '15:00','16:30','18:00','19:00','20:00','22:00']::TIME[];
    v_durees      INT[]  := ARRAY[360, 120, 90, 90, 60, 60, 120, 90, 90, 60, 60, 120, 120];
    i             INT;
    v_nb_prog     INT;
BEGIN
    SELECT id INTO v_auteur FROM iam.utilisateur WHERE email = 'angenor99@gmail.com';
    IF v_auteur IS NULL THEN
        SELECT id INTO v_auteur FROM iam.utilisateur WHERE email = 'test-admin@test.com';
    END IF;
    IF v_auteur IS NULL THEN
        RAISE EXCEPTION 'Compte auteur introuvable — seed interrompu';
    END IF;
    SELECT id INTO v_membre FROM iam.utilisateur WHERE email = 'test-user@test.com';

    -- ========================================================================
    -- 0. Retirer de la vitrine les chaînes de test sans identité visuelle
    -- ------------------------------------------------------------------------
    -- Rien n'est supprimé : `brouillon` les sort de la liste publique, un
    -- UPDATE inverse les y remet (cf. le fichier de purge).
    -- ========================================================================
    UPDATE media_content.chaine_tv
       SET etat = 'brouillon'
     WHERE etat = 'publie'
       AND slug IN ('africa24-test','chaine-vide','sahel-culture','test-chaine-1',
                    'test-chaine-2','chaine-test-007','chaine-proposee-007',
                    'africans-doc-test','africans-innovation-test');

    -- ========================================================================
    -- 1. Chaînes
    -- ========================================================================
    FOR c IN SELECT * FROM tmp_chaine ORDER BY ordre LOOP
        -- `c.pays` ne sert plus qu'à vérifier que le territoire existe : depuis
        -- 09v la chaîne n'a plus de pays propre, sa couverture (plus bas) est
        -- l'unique déclaration territoriale.
        IF c.pays IS NOT NULL
           AND NOT EXISTS (SELECT 1 FROM shared.pays WHERE nom = c.pays) THEN
            RAISE EXCEPTION 'Territoire introuvable : %', c.pays;
        END IF;

        -- Graine stable : la même chaîne garde la même photo d'une exécution
        -- à l'autre, sinon la vitrine changerait de visage à chaque rejeu.
        v_img := 'https://picsum.photos/seed/uafricas-' || c.slug || '/600/600';

        INSERT INTO media_content.chaine_tv
            (nom, slug, description, image_couverture_url, categorie, langue,
             est_en_direct, etat, origine_publication, couverture_continentale,
             stream_url, contact_email, contact_telephone, contact_whatsapp,
             contact_site_web, contact_adresse, cree_par)
        VALUES
            (c.nom, c.slug, c.description, v_img, c.categorie::media_content.categorie_chaine_tv,
             c.langue, c.en_direct, 'publie', c.origine, c.continentale,
             CASE WHEN c.en_direct THEN 'https://www.youtube.com/watch?v=' || v_videos[1 + (c.ordre % 15)] END,
             c.email, c.telephone, c.whatsapp, c.site_web, c.adresse, v_auteur)
        ON CONFLICT (slug) DO UPDATE SET
            nom = EXCLUDED.nom,
            description = EXCLUDED.description,
            image_couverture_url = EXCLUDED.image_couverture_url,
            categorie = EXCLUDED.categorie,
            langue = EXCLUDED.langue,
            est_en_direct = EXCLUDED.est_en_direct,
            etat = 'publie',
            origine_publication = EXCLUDED.origine_publication,
            couverture_continentale = EXCLUDED.couverture_continentale,
            stream_url = EXCLUDED.stream_url,
            contact_email = EXCLUDED.contact_email,
            contact_telephone = EXCLUDED.contact_telephone,
            contact_whatsapp = EXCLUDED.contact_whatsapp,
            contact_site_web = EXCLUDED.contact_site_web,
            contact_adresse = EXCLUDED.contact_adresse
        RETURNING id INTO v_chaine;

        -- Thématiques déclarées (US3) — remplacement intégral.
        DELETE FROM media_content.support_thematique
              WHERE type_support = 'chaine_tv' AND support_id = v_chaine;
        INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
        SELECT 'chaine_tv', v_chaine, cat.id
          FROM shared.categorie cat
         WHERE cat.contexte = 'media' AND cat.nom = ANY(c.themes)
        ON CONFLICT DO NOTHING;

        -- Couverture territoriale (US4). Le trigger d'exclusivité refuse tout
        -- territoire nommé sur une chaîne continentale : on n'en propose donc
        -- pas, plutôt que de compter sur l'exception.
        DELETE FROM media_content.support_territoire
              WHERE type_support = 'chaine_tv' AND support_id = v_chaine;
        IF NOT c.continentale AND c.territoires IS NOT NULL THEN
            INSERT INTO media_content.support_territoire (type_support, support_id, pays_id)
            SELECT 'chaine_tv', v_chaine, p.id
              FROM shared.pays p
             WHERE p.nom = ANY(c.territoires)
            ON CONFLICT DO NOTHING;
        END IF;

        -- Équipe éditoriale (010) — remplacement intégral, `ordre` = rang.
        DELETE FROM media_content.membre_equipe
              WHERE type_porteur = 'chaine_tv' AND porteur_id = v_chaine;
        FOR m IN SELECT * FROM tmp_equipe
                  WHERE porteur_slug = c.slug AND type_porteur = 'chaine_tv'
                  ORDER BY ordre LOOP
            INSERT INTO media_content.membre_equipe
                (type_porteur, porteur_id, nom, prenom, fonction, territoire, contact,
                 ordre, cree_par)
            VALUES ('chaine_tv', v_chaine, m.nom, m.prenom, m.fonction, m.territoire,
                    m.contact, m.ordre, v_auteur);
        END LOOP;

        -- Deux chaînes détenues par le compte de test : sans elles, le bouton
        -- « Gérer ma chaîne » de la vitrine reste invérifiable.
        IF v_membre IS NOT NULL AND c.ordre IN (2, 9) THEN
            INSERT INTO media_content.support_detenteur
                (type_support, support_id, utilisateur_id, role, designe_par)
            VALUES ('chaine_tv', v_chaine, v_membre, 'proprietaire', v_auteur)
            ON CONFLICT (type_support, support_id, utilisateur_id)
            DO UPDATE SET role = 'proprietaire', actif = TRUE, retire_at = NULL;
        END IF;
    END LOOP;

    -- ========================================================================
    -- 2. Programmes (émissions) et épisodes
    -- ========================================================================
    FOR e IN SELECT * FROM tmp_emission ORDER BY chaine_slug, ordre LOOP
        SELECT id INTO v_chaine FROM media_content.chaine_tv WHERE slug = e.chaine_slug;
        SELECT id INTO v_theme FROM shared.categorie
         WHERE contexte = 'media' AND nom = e.theme AND actif LIMIT 1;

        INSERT INTO media_content.emission_tele
            (chaine_id, titre, slug, description, image_couverture_url,
             info_animateur, info_producteur, langue, theme_phare_id, cadence,
             etat, cree_par)
        VALUES
            (v_chaine, e.titre, e.slug, e.description,
             'https://picsum.photos/seed/uafricas-' || e.slug || '/960/540',
             e.animateur, e.producteur,
             (SELECT langue FROM media_content.chaine_tv WHERE id = v_chaine),
             v_theme, e.cadence, 'publie', v_auteur)
        ON CONFLICT (slug) DO UPDATE SET
            chaine_id = EXCLUDED.chaine_id,
            titre = EXCLUDED.titre,
            description = EXCLUDED.description,
            image_couverture_url = EXCLUDED.image_couverture_url,
            info_animateur = EXCLUDED.info_animateur,
            info_producteur = EXCLUDED.info_producteur,
            theme_phare_id = EXCLUDED.theme_phare_id,
            cadence = EXCLUDED.cadence,
            etat = 'publie'
        RETURNING id INTO v_emission;

        FOR i IN 1..e.nb_episodes LOOP
            v_compteur := v_compteur + 1;
            v_video := v_videos[1 + (v_compteur % 15)];

            INSERT INTO media_content.episode_tele
                (emission_id, titre, slug, description, image_couverture_url, video_url,
                 numero_episode, ordre, duree_minutes, a_la_une, etat,
                 valide_par, valide_at, cree_par, created_at)
            VALUES
                (v_emission,
                 e.titre || ' — n° ' || LPAD(i::TEXT, 2, '0'),
                 e.slug || '-ep-' || LPAD(i::TEXT, 2, '0'),
                 'Épisode ' || i || ' de « ' || e.titre || ' ». ' || e.description,
                 'https://i.ytimg.com/vi/' || v_video || '/maxresdefault.jpg',
                 'https://www.youtube.com/watch?v=' || v_video,
                 i, i, 18 + ((v_compteur * 7) % 62),
                 i = 1, 'publie', v_auteur, NOW(), v_auteur,
                 NOW() - ((e.nb_episodes - i) * 7 || ' days')::INTERVAL)
            ON CONFLICT (slug) DO UPDATE SET
                emission_id = EXCLUDED.emission_id,
                titre = EXCLUDED.titre,
                description = EXCLUDED.description,
                image_couverture_url = EXCLUDED.image_couverture_url,
                video_url = EXCLUDED.video_url,
                duree_minutes = EXCLUDED.duree_minutes,
                etat = 'publie',
                valide_par = EXCLUDED.valide_par,
                valide_at = EXCLUDED.valide_at;
        END LOOP;
    END LOOP;

    -- ========================================================================
    -- 3. Grilles de programmation — 24 h couvertes sur chaque chaîne
    -- ------------------------------------------------------------------------
    -- Le bandeau « En ce moment / À suivre » est résolu à la lecture, à partir
    -- de l'heure courante dans le fuseau du créneau. Sans grille pleine, la
    -- démonstration ne montrerait le bandeau qu'à certaines heures de la
    -- journée. `date_effet` est reculée d'un mois pour que la rotation des
    -- épisodes ait déjà tourné.
    -- ========================================================================
    FOR c IN SELECT * FROM tmp_chaine ORDER BY ordre LOOP
        SELECT id INTO v_chaine FROM media_content.chaine_tv WHERE slug = c.slug;

        DELETE FROM media_content.creneau_programmation
              WHERE type_support = 'chaine_tv' AND support_id = v_chaine;

        SELECT COUNT(*) INTO v_nb_prog FROM tmp_emission WHERE chaine_slug = c.slug;
        CONTINUE WHEN v_nb_prog = 0;   -- Sawa TV : aucune grille, c'est le cas creux

        FOR i IN 1..array_length(v_heures, 1) LOOP
            INSERT INTO media_content.creneau_programmation
                (type_support, support_id, emission_id, recurrence, jour_semaine,
                 heure_debut, duree_minutes, fuseau, date_effet, cree_par, actif)
            SELECT 'chaine_tv', v_chaine, em.id, 'quotidien', NULL,
                   v_heures[i], v_durees[i], 'Africa/Abidjan',
                   CURRENT_DATE - 30, v_auteur, TRUE
              FROM media_content.emission_tele em
              JOIN tmp_emission te ON te.slug = em.slug
             WHERE te.chaine_slug = c.slug
               AND te.ordre = 1 + ((i - 1) % v_nb_prog);
        END LOOP;
    END LOOP;

    -- ========================================================================
    -- 4. Vedette de la page
    -- ------------------------------------------------------------------------
    -- Un seul épisode peut porter `a_la_une_globale` (index unique partiel) :
    -- la mise à zéro préalable n'est pas une précaution, c'est la condition
    -- pour que l'INSERT passe.
    -- ========================================================================
    UPDATE media_content.episode_tele SET a_la_une_globale = FALSE
     WHERE a_la_une_globale = TRUE AND deleted_at IS NULL;

    UPDATE media_content.episode_tele SET
        titre = 'Afrique 2050 : le continent qui se réinvente',
        description = 'Un milliard six cents millions d''habitants, la moitié de moins de vingt-cinq ans, et des villes qui doubleront de taille en une génération. Cette grande enquête suit sur trois continents les décisions énergétiques, agricoles et urbaines qui engagent l''Afrique pour les trente prochaines années — avec les chiffres, les contradicteurs, et ceux qui construisent déjà la suite.',
        image_couverture_url = 'https://i.ytimg.com/vi/9z52xavACQY/maxresdefault.jpg',
        video_url = 'https://www.youtube.com/watch?v=9z52xavACQY',
        duree_minutes = 52,
        a_la_une_globale = TRUE
     WHERE slug = 'afrique-2050-ep-01';

    -- ========================================================================
    -- 5. Panorama Continental — la chaîne qui déborde le plafond de 30
    -- ------------------------------------------------------------------------
    -- Ses 31 programmes existent déjà (jeu 010) mais s'appellent « Panorama —
    -- numéro NN » et n'ont aucune couverture : le cas est juste, le rendu ne
    -- l'est pas. On lui donne une identité sans toucher au nombre, qui est
    -- précisément ce qu'elle démontre.
    -- ========================================================================
    SELECT id INTO v_chaine FROM media_content.chaine_tv WHERE slug = 'panorama-continental-010';
    IF v_chaine IS NOT NULL THEN
        UPDATE media_content.chaine_tv SET
            image_couverture_url = 'https://picsum.photos/seed/uafricas-panorama-continental/600/600',
            contact_email = COALESCE(contact_email, 'redaction@panorama-continental.tv'),
            contact_site_web = COALESCE(contact_site_web, 'https://panorama-continental.tv')
         WHERE id = v_chaine;

        WITH numerotes AS (
            SELECT em.id,
                   ROW_NUMBER() OVER (ORDER BY em.slug) AS n
              FROM media_content.emission_tele em
             WHERE em.chaine_id = v_chaine
               AND em.deleted_at IS NULL
               AND em.slug LIKE 'panorama-continental-010-prog-%'
        )
        UPDATE media_content.emission_tele em SET
            titre = (ARRAY['Chroniques','Regards','Carnets','Le Rendez-vous','Horizons',
                           'Passages','Terrains','Échos'])[1 + (n::INT % 8)]
                    || ' ' ||
                    (ARRAY['du Sahel','de l''Atlantique','des Grands Lacs','du Nil',
                           'de la Corne','du Golfe de Guinée','de l''Océan Indien',
                           'du Kalahari','des Hauts Plateaux','du Fleuve Niger',
                           'de la Diaspora'])[1 + (n::INT % 11)]
                    || ' · ' || LPAD(n::TEXT, 2, '0'),
            image_couverture_url = 'https://picsum.photos/seed/uafricas-panorama-' || n || '/960/540',
            -- Ses 33 programmes partageaient une seule et même phrase : sur une
            -- grille de trente cartes, la répétition saute aux yeux et fausse le
            -- jugement porté sur la mise en page.
            description = (ARRAY[
                'Un magazine hebdomadaire consacré aux transformations silencieuses du continent : celles qui ne font pas la une mais changent le quotidien.',
                'Reportages de terrain et entretiens longs, tournés au plus près des habitants, loin des capitales et des plateaux.',
                'Chaque numéro croise trois regards sur une même question — un chercheur, un praticien, un habitant — sans chercher à les réconcilier.',
                'Une immersion mensuelle dans une filière économique, de la matière première au consommateur final.',
                'Le récit d''un territoire par ses cartes, ses langues et ses routes, avec les archives qui en gardent la trace.',
                'Portraits croisés de deux générations sur un même métier : ce qui a disparu, ce qui résiste, ce qui vient.',
                'Un format court et documenté sur les politiques publiques annoncées, un an après leur lancement.',
                'Le magazine culturel de la chaîne : scènes, éditions, festivals et les publics qui les font vivre.'
            ])[1 + (n::INT % 8)],
            -- Aucun de ses programmes ne portait de thème phare : la chaîne
            -- restait alors invisible de TOUS les filtres thématiques, et sa
            -- section n'annonçait rien. Le thème se répartit sur le rang.
            theme_phare_id = (SELECT cat.id FROM shared.categorie cat
                               WHERE cat.contexte = 'media' AND cat.actif
                                 AND cat.nom = (ARRAY['Journal télévisé','Débats et analyses',
                                                      'Grandes interviews','Diaspora',
                                                      'Investigations'])[1 + (n::INT % 5)])
          FROM numerotes
         WHERE em.id = numerotes.id;
    END IF;

    -- ========================================================================
    -- 6. Interactions — pour que la barre de réactions ne soit pas à zéro
    -- ------------------------------------------------------------------------
    -- Les compteurs sont recalculés à la lecture depuis ces lignes ; il n'y a
    -- aucune colonne de dénormalisation à tenir en cohérence.
    -- ========================================================================
    DELETE FROM media_content.media_reaction r
     WHERE r.type_media = 'chaine_tv'
       AND r.media_id IN (SELECT ct.id FROM media_content.chaine_tv ct
                           JOIN tmp_chaine tc ON tc.slug = ct.slug);
    DELETE FROM media_content.partage_media p
     WHERE p.type_media = 'chaine_tv'
       AND p.media_id IN (SELECT ct.id FROM media_content.chaine_tv ct
                           JOIN tmp_chaine tc ON tc.slug = ct.slug);
    DELETE FROM media_content.media_commentaire mc
     WHERE mc.type_media = 'chaine_tv'
       AND mc.media_id IN (SELECT ct.id FROM media_content.chaine_tv ct
                            JOIN tmp_chaine tc ON tc.slug = ct.slug);

    FOR c IN SELECT * FROM tmp_chaine ORDER BY ordre LOOP
        SELECT id INTO v_chaine FROM media_content.chaine_tv WHERE slug = c.slug;

        -- Répartition déterministe : le rang de l'utilisateur et l'ordre de la
        -- chaîne suffisent à varier les compteurs sans tirage aléatoire, qui
        -- rendrait le jeu non reproductible.
        INSERT INTO media_content.media_reaction
            (type_media, media_id, utilisateur_id, type_reaction)
        SELECT 'chaine_tv', v_chaine, u.id,
               CASE WHEN (u.rang * 3 + c.ordre) % 11 = 0 THEN 'dislike' ELSE 'like' END
          FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY created_at) AS rang
                  FROM iam.utilisateur WHERE deleted_at IS NULL) u
         WHERE (u.rang * 5 + c.ordre * 3) % 7 < 4 + (c.ordre % 3)
        ON CONFLICT (type_media, media_id, utilisateur_id) DO NOTHING;

        INSERT INTO media_content.partage_media (type_media, media_id, utilisateur_id, legende)
        SELECT 'chaine_tv', v_chaine, u.id, 'À suivre : ' || c.nom
          FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY created_at) AS rang
                  FROM iam.utilisateur WHERE deleted_at IS NULL) u
         WHERE (u.rang + c.ordre * 2) % 6 = 0;

        INSERT INTO media_content.media_commentaire (type_media, media_id, auteur_id, contenu)
        SELECT 'chaine_tv', v_chaine, u.id,
               (ARRAY['Enfin une chaîne qui parle de chez nous sans caricature.',
                      'La grille du soir est très bien pensée, bravo à l''équipe.',
                      'Est-ce que les programmes sont disponibles en rediffusion ?',
                      'Excellente qualité d''image, on sent le travail de production.',
                      'J''aimerais voir plus de sujets sur les zones rurales.',
                      'Le générique est magnifique. Qui l''a composé ?'])[1 + ((u.rang::INT + c.ordre) % 6)]
          FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY created_at) AS rang
                  FROM iam.utilisateur WHERE deleted_at IS NULL) u
         WHERE (u.rang * 4 + c.ordre) % 9 < 2;
    END LOOP;

    -- ========================================================================
    -- 7. Chaînes héritées des jeux 009 et 010
    -- ------------------------------------------------------------------------
    -- Elles précèdent ce seed et n'ont ni équipe, ni grille, ni interactions :
    -- au milieu d'une vitrine dense, leurs sections se lisaient comme une
    -- panne. On ne les réécrit pas — on ne comble que ce qui manque, chaque
    -- ajout étant gardé par un test d'absence, ce qui garde le rejeu sans effet.
    --
    -- Leurs thématiques sont DÉDUITES des thèmes phares de leurs programmes
    -- publiés : une chaîne annonce ainsi ce qu'elle diffuse réellement, plutôt
    -- qu'une liste saisie à côté.
    -- ========================================================================
    FOR c IN SELECT ct.id, ct.nom, ct.slug,
                    -- Le pays unique n'existe plus (09v) : le territoire
                    -- affiché est le premier de la couverture déclarée, s'il y
                    -- en a une.
                    (SELECT p.nom
                       FROM media_content.support_territoire ste
                       JOIN shared.pays p ON p.id = ste.pays_id
                      WHERE ste.type_support = 'chaine_tv' AND ste.support_id = ct.id
                      ORDER BY p.nom LIMIT 1) AS pays,
                    ROW_NUMBER() OVER (ORDER BY ct.nom) AS ordre
               FROM media_content.chaine_tv ct
              WHERE ct.etat = 'publie' AND ct.deleted_at IS NULL
                AND ct.slug NOT IN (SELECT slug FROM tmp_chaine)
              ORDER BY ct.nom LOOP
        v_chaine := c.id;

        IF NOT EXISTS (SELECT 1 FROM media_content.support_thematique
                        WHERE type_support = 'chaine_tv' AND support_id = v_chaine) THEN
            INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
            SELECT DISTINCT 'chaine_tv'::media_content.type_support_media, v_chaine, em.theme_phare_id
              FROM media_content.emission_tele em
             WHERE em.chaine_id = v_chaine AND em.etat = 'publie'
               AND em.deleted_at IS NULL AND em.theme_phare_id IS NOT NULL
            ON CONFLICT DO NOTHING;
        END IF;

        IF NOT EXISTS (SELECT 1 FROM media_content.membre_equipe
                        WHERE type_porteur = 'chaine_tv' AND porteur_id = v_chaine
                          AND deleted_at IS NULL) THEN
            FOR i IN 1..3 LOOP
                INSERT INTO media_content.membre_equipe
                    (type_porteur, porteur_id, nom, prenom, fonction, territoire, ordre, cree_par)
                VALUES ('chaine_tv', v_chaine,
                    (ARRAY['Diallo','Mensah','Traoré','Nkosi','Okonkwo','Benali','Camara','Abubakar',
                           'Sow','Molefe','Cherif','Keïta','Barry','Asante','El Amrani','Chikwanda',
                           'Bah','Eze','Touré','Ndiaye'])[1 + ((c.ordre::INT * 11 + i * 3) % 20)],
                    (ARRAY['Aminata','Kwame','Fatou','Thabo','Chidi','Nadia','Sékou','Zainab',
                           'Mamadou','Lerato','Yacine','Ibrahim','Awa','Kofi','Rania','Tendai',
                           'Moussa','Ngozi','Hawa','Djibril'])[1 + ((c.ordre::INT * 5 + i * 7) % 20)],
                    (ARRAY['Direction de l''antenne','Rédaction en chef','Production exécutive'])[i],
                    COALESCE(c.pays, 'Panafricain'), i, v_auteur);
            END LOOP;
        END IF;

        -- Grille : seulement si la chaîne n'en a aucune ET qu'elle a de quoi
        -- programmer. Un créneau dont l'émission n'a pas d'épisode publié n'est
        -- pas servi au public — il ne ferait que trouer la grille (FR-021).
        SELECT COUNT(*) INTO v_nb_prog
          FROM media_content.emission_tele em
         WHERE em.chaine_id = v_chaine AND em.etat = 'publie' AND em.deleted_at IS NULL
           AND EXISTS (SELECT 1 FROM media_content.episode_tele ep
                        WHERE ep.emission_id = em.id AND ep.etat = 'publie'
                          AND ep.deleted_at IS NULL);

        IF v_nb_prog > 0 AND NOT EXISTS (
                SELECT 1 FROM media_content.creneau_programmation
                 WHERE type_support = 'chaine_tv' AND support_id = v_chaine
                   AND actif = TRUE AND deleted_at IS NULL) THEN
            FOR i IN 1..array_length(v_heures, 1) LOOP
                INSERT INTO media_content.creneau_programmation
                    (type_support, support_id, emission_id, recurrence, jour_semaine,
                     heure_debut, duree_minutes, fuseau, date_effet, cree_par, actif)
                SELECT 'chaine_tv', v_chaine, prog.id, 'quotidien', NULL,
                       v_heures[i], v_durees[i], 'Africa/Abidjan',
                       CURRENT_DATE - 30, v_auteur, TRUE
                  FROM (SELECT em.id, ROW_NUMBER() OVER (ORDER BY em.created_at, em.id) AS rang
                          FROM media_content.emission_tele em
                         WHERE em.chaine_id = v_chaine AND em.etat = 'publie'
                           AND em.deleted_at IS NULL
                           AND EXISTS (SELECT 1 FROM media_content.episode_tele ep
                                        WHERE ep.emission_id = em.id AND ep.etat = 'publie'
                                          AND ep.deleted_at IS NULL)) prog
                 WHERE prog.rang = 1 + ((i - 1) % v_nb_prog);
            END LOOP;
        END IF;

        IF NOT EXISTS (SELECT 1 FROM media_content.media_reaction
                        WHERE type_media = 'chaine_tv' AND media_id = v_chaine) THEN
            INSERT INTO media_content.media_reaction
                (type_media, media_id, utilisateur_id, type_reaction)
            SELECT 'chaine_tv', v_chaine, u.id,
                   CASE WHEN (u.rang * 3 + c.ordre) % 13 = 0 THEN 'dislike' ELSE 'like' END
              FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY created_at) AS rang
                      FROM iam.utilisateur WHERE deleted_at IS NULL) u
             WHERE (u.rang * 5 + c.ordre * 3) % 7 < 3 + (c.ordre % 3)
            ON CONFLICT (type_media, media_id, utilisateur_id) DO NOTHING;

            INSERT INTO media_content.partage_media (type_media, media_id, utilisateur_id, legende)
            SELECT 'chaine_tv', v_chaine, u.id, 'À suivre : ' || c.nom
              FROM (SELECT id, ROW_NUMBER() OVER (ORDER BY created_at) AS rang
                      FROM iam.utilisateur WHERE deleted_at IS NULL) u
             WHERE (u.rang + c.ordre * 2) % 7 = 0;
        END IF;
    END LOOP;

    RAISE NOTICE 'Vitrine télé : % chaînes publiées, % programmes, % épisodes.',
        (SELECT COUNT(*) FROM media_content.chaine_tv WHERE etat = 'publie' AND deleted_at IS NULL),
        (SELECT COUNT(*) FROM media_content.emission_tele WHERE etat = 'publie' AND deleted_at IS NULL),
        (SELECT COUNT(*) FROM media_content.episode_tele WHERE etat = 'publie' AND deleted_at IS NULL);
END
$seed$;

COMMIT;
