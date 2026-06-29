-- ════════════════════════════════════════════════════════════════════════════
-- SEED : Programmes de formation — chapitres & leçons
-- ════════════════════════════════════════════════════════════════════════════
-- Remplit le programme (media_content.formation_chapitre + formation_lecon) de
-- trois formations déjà seedées (cf. 19_seed_moocs.sql) avec un contenu riche :
-- chapitres ordonnés, leçons avec texte pédagogique, vidéo, durée et documents.
--
-- NOTE sur les médias (données de démonstration, éditables au back-office) :
--   • video_url : fichiers MP4 d'exemple du bucket public Google (joués via la
--     balise <video>) + un exemple de lien YouTube (lecteur embarqué). À remplacer
--     par les vraies vidéos de cours via /admin/mooc/[id] → onglet « Programme ».
--   • document_url : PDF d'exemple public.
--
-- Idempotent : chaque formation n'est peuplée que si elle n'a encore aucun chapitre.
-- Les textes français utilisent le dollar-quoting ($c$…$c$) pour éviter d'avoir à
-- doubler les apostrophes.
-- ════════════════════════════════════════════════════════════════════════════

DO $seed$
DECLARE
    v_mooc_id UUID;
    v_ch1 UUID;
    v_ch2 UUID;
    v_ch3 UUID;
    v_ch4 UUID;
    -- Vidéos d'exemple (MP4 publics, lisibles directement)
    v_vid1 TEXT := 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4';
    v_vid2 TEXT := 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4';
    v_vid3 TEXT := 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4';
    v_vid4 TEXT := 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerEscapes.mp4';
    v_vid5 TEXT := 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerJoyrides.mp4';
    v_vid6 TEXT := 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/Sintel.mp4';
    v_yt   TEXT := 'https://www.youtube.com/watch?v=aqz-KE-bpKQ';  -- ex. de lien YouTube (lecteur embarqué)
    v_pdf  TEXT := 'https://www.africau.edu/images/default/sample.pdf';
BEGIN

-- ════════════════════════════════════════════════════════════════════════════
-- FORMATION 1 — Développement web moderne
-- ════════════════════════════════════════════════════════════════════════════
SELECT id INTO v_mooc_id FROM media_content.mooc WHERE slug = 'developpement-web-moderne';

IF v_mooc_id IS NOT NULL
   AND NOT EXISTS (SELECT 1 FROM media_content.formation_chapitre WHERE mooc_id = v_mooc_id AND deleted_at IS NULL)
THEN
    -- ── Chapitre 1 : Fondations du Web ────────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Fondations du Web (HTML & CSS)$t$,
            $t$Comprendre comment fonctionne le Web et poser des bases solides en HTML sémantique et en CSS.$t$, 0)
    RETURNING id INTO v_ch1;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch1, $t$Comment fonctionne le Web$t$,
     $c$Le Web repose sur un modèle client/serveur : votre navigateur (le client) envoie une requête HTTP à un serveur, qui renvoie une réponse (HTML, CSS, JavaScript, images…).

Points clés de cette leçon :
- Le protocole HTTP/HTTPS et le cycle requête → réponse.
- Le rôle des URL, des noms de domaine et du DNS.
- Ce que fait réellement le navigateur : téléchargement, analyse (parsing), rendu.
- La différence entre contenu statique et contenu dynamique.

À la fin, vous saurez décrire précisément ce qui se passe entre le moment où vous tapez une adresse et l'affichage de la page.$c$,
     v_vid1, NULL, 12, 0),

    (v_ch1, $t$Structurer une page avec HTML5 sémantique$t$,
     $c$Le HTML décrit la STRUCTURE et le SENS du contenu, pas son apparence. Écrire un HTML sémantique améliore l'accessibilité et le référencement.

Au programme :
- Les balises structurantes : header, nav, main, section, article, aside, footer.
- Titres (h1–h6), paragraphes, listes, liens et images.
- Les attributs essentiels (alt, href, lang) et leur importance pour l'accessibilité.
- Les formulaires : input, label, select, textarea, et la validation native.

Exercice : reconstruire la structure sémantique d'une page d'actualités.$c$,
     v_vid2, NULL, 18, 1),

    (v_ch1, $t$Mise en forme avec CSS et le modèle de boîte$t$,
     $c$Le CSS contrôle l'apparence. Tout élément est une « boîte » : contenu, padding, bordure, marge.

Vous apprendrez :
- Sélecteurs, spécificité et cascade (d'où vient le « C » de CSS).
- Le modèle de boîte et box-sizing: border-box.
- Couleurs, typographie, unités (px, rem, %, vh/vw).
- Positionnement : flux normal, position relative/absolute, z-index.$c$,
     v_vid3, NULL, 22, 2),

    (v_ch1, $t$Responsive design : Flexbox & Grid$t$,
     $c$Un site moderne doit s'adapter à toutes les tailles d'écran. Flexbox (1 dimension) et Grid (2 dimensions) sont les outils de mise en page d'aujourd'hui.

Contenu :
- Flexbox : axes, justify-content, align-items, gap.
- CSS Grid : colonnes, lignes, zones nommées.
- Les media queries et l'approche « mobile-first ».
- Bonnes pratiques d'images et de typographie fluides.

Le document joint récapitule les propriétés Flexbox/Grid sous forme d'aide-mémoire.$c$,
     v_vid4, v_pdf, 25, 3);

    -- ── Chapitre 2 : JavaScript moderne ──────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$JavaScript moderne (ES6+)$t$,
            $t$Rendre les pages interactives : le langage du Web, ses fondamentaux et l'asynchrone.$t$, 1)
    RETURNING id INTO v_ch2;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch2, $t$Variables, types et fonctions$t$,
     $c$JavaScript est le langage de programmation du navigateur. Cette leçon couvre les bases incontournables :
- let, const et la portée (scope) ; pourquoi éviter var.
- Les types primitifs et les objets/tableaux.
- Les fonctions classiques et les fonctions fléchées (=>).
- Conditions, boucles et opérateurs modernes (spread, destructuring, optional chaining).$c$,
     v_vid5, NULL, 15, 0),

    (v_ch2, $t$Le DOM et les événements$t$,
     $c$Le DOM (Document Object Model) est la représentation en mémoire de la page, que JavaScript peut lire et modifier.

Vous verrez :
- Sélectionner des éléments (querySelector) et modifier le contenu/les styles.
- Créer et supprimer des éléments dynamiquement.
- Écouter les événements (clic, saisie, soumission de formulaire) avec addEventListener.
- La délégation d'événements et la gestion de l'état de l'interface.$c$,
     v_vid6, NULL, 20, 1),

    (v_ch2, $t$Asynchrone : promesses, async/await et fetch$t$,
     $c$Les opérations réseau prennent du temps : JavaScript les gère de façon ASYNCHRONE pour ne pas bloquer l'interface.

Au menu :
- Le modèle d'exécution (event loop) expliqué simplement.
- Les promesses (Promise) : then/catch.
- La syntaxe async/await, plus lisible.
- L'API fetch pour appeler une API REST et traiter du JSON.
- La gestion des erreurs réseau.$c$,
     v_vid1, NULL, 24, 2),

    (v_ch2, $t$Modules ES et outils de build$t$,
     $c$Organiser son code en modules réutilisables et le préparer pour la production.
- import / export : découper le code en fichiers.
- npm et la gestion des dépendances.
- Les bundlers modernes (Vite) : pourquoi et comment.
- Notions de transpilation et de compatibilité navigateur.$c$,
     v_vid2, NULL, 16, 3);

    -- ── Chapitre 3 : Construire une application ──────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Construire une application avec un framework$t$,
            $t$Passer du DOM manuel à une architecture par composants réactifs (Vue, React…).$t$, 2)
    RETURNING id INTO v_ch3;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch3, $t$Penser en composants$t$,
     $c$Les frameworks modernes structurent l'interface en composants : des briques autonomes et réutilisables (un bouton, une carte, un formulaire).

Concepts :
- Qu'est-ce qu'un composant et pourquoi cela change tout.
- Template, logique et style co-localisés.
- Arborescence de composants et composition.$c$,
     v_vid3, NULL, 18, 0),

    (v_ch3, $t$État, props et réactivité$t$,
     $c$L'interface est une fonction de l'état : quand l'état change, la vue se met à jour automatiquement.
- L'état local (state) et la réactivité.
- Les props : passer des données d'un parent à un enfant.
- Les événements : communiquer de l'enfant vers le parent.
- Le rendu de listes et l'affichage conditionnel.$c$,
     v_vid4, NULL, 22, 1),

    (v_ch3, $t$Routage et appels API$t$,
     $c$Construire une application à plusieurs pages qui consomme des données distantes.
- Le routage côté client (SPA) : associer des URL à des vues.
- Récupérer des données depuis une API au chargement.
- Gérer les états de chargement et d'erreur.
- Bonnes pratiques d'organisation du code.$c$,
     v_yt, NULL, 20, 2);

    -- ── Chapitre 4 : Mise en production ──────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Mise en production$t$,
            $t$Performance, qualité et déploiement d'une application web.$t$, 3)
    RETURNING id INTO v_ch4;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch4, $t$Performance et bonnes pratiques$t$,
     $c$Un site rapide est un site utilisé. Optimiser le poids, le rendu et l'expérience.
- Optimisation des images et chargement différé (lazy loading).
- Minification, mise en cache et compression.
- Mesurer avec les outils du navigateur (Lighthouse).
- Accessibilité (a11y) et SEO de base.$c$,
     v_vid5, NULL, 14, 0),

    (v_ch4, $t$Déploiement et intégration continue$t$,
     $c$Mettre son application en ligne et automatiser les livraisons.
- Build de production et variables d'environnement.
- Hébergement statique et serveurs d'application.
- Notions de CI/CD : tester et déployer automatiquement.
- Suivi des erreurs en production.$c$,
     v_vid6, v_pdf, 17, 1);
END IF;

-- ════════════════════════════════════════════════════════════════════════════
-- FORMATION 2 — Introduction à l'économie africaine
-- ════════════════════════════════════════════════════════════════════════════
SELECT id INTO v_mooc_id FROM media_content.mooc WHERE slug = 'introduction-economie-africaine';

IF v_mooc_id IS NOT NULL
   AND NOT EXISTS (SELECT 1 FROM media_content.formation_chapitre WHERE mooc_id = v_mooc_id AND deleted_at IS NULL)
THEN
    -- ── Chapitre 1 ────────────────────────────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Panorama des économies africaines$t$,
            $t$Une lecture d'ensemble de la diversité et des structures économiques du continent.$t$, 0)
    RETURNING id INTO v_ch1;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch1, $t$Diversité des économies du continent$t$,
     $c$L'Afrique n'est pas un bloc homogène : 54 pays aux trajectoires très différentes, des économies de rente pétrolière aux pôles de services, en passant par les économies agricoles.

Cette leçon pose le décor :
- Les grandes régions économiques et leurs spécialisations.
- Économies tirées par les ressources vs économies diversifiées.
- Le rôle structurant de la démographie et de l'urbanisation.$c$,
     v_vid1, NULL, 16, 0),

    (v_ch1, $t$Les indicateurs macroéconomiques essentiels$t$,
     $c$Pour comprendre une économie, il faut savoir lire ses indicateurs.
- PIB, PIB par habitant et croissance : ce qu'ils disent et ne disent pas.
- Inflation, taux de change et politique monétaire.
- L'Indice de développement humain (IDH) : au-delà du seul revenu.
- Balance commerciale et dépendance aux exportations de matières premières.$c$,
     v_vid2, v_pdf, 19, 1),

    (v_ch1, $t$Le poids du secteur informel$t$,
     $c$Dans de nombreux pays africains, l'essentiel de l'emploi se trouve dans le secteur informel.
- Définition et mesure de l'informel.
- Pourquoi il est à la fois une force (résilience, emploi) et un défi (fiscalité, protection sociale).
- Pistes de formalisation progressive.$c$,
     v_vid3, NULL, 17, 2);

    -- ── Chapitre 2 ────────────────────────────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Commerce et intégration régionale$t$,
            $t$Échanges intra-africains, intégration continentale et chaînes de valeur.$t$, 1)
    RETURNING id INTO v_ch2;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch2, $t$La ZLECAf : un marché continental$t$,
     $c$La Zone de libre-échange continentale africaine (ZLECAf) vise à créer le plus grand marché unique du monde en nombre de pays.

Au programme :
- Objectifs : réduire les barrières tarifaires, stimuler le commerce intra-africain.
- Opportunités pour les PME et l'industrialisation.
- Les défis : infrastructures, normes, logistique, volonté politique.$c$,
     v_vid4, NULL, 21, 0),

    (v_ch2, $t$Les communautés économiques régionales$t$,
     $c$Avant la ZLECAf, l'intégration s'est construite par blocs régionaux.
- CEDEAO (Afrique de l'Ouest), EAC (Afrique de l'Est), SADC (Afrique australe), etc.
- Unions douanières, libre circulation et monnaies communes.
- Réussites et limites de l'intégration régionale.$c$,
     v_vid5, NULL, 18, 1),

    (v_ch2, $t$Matières premières et chaînes de valeur$t$,
     $c$Exporter des matières brutes capte peu de valeur ; transformer localement en capte davantage.
- L'exemple du cacao, du café, du coton ou des minerais.
- Le concept de « remontée dans la chaîne de valeur ».
- Politiques de transformation locale et de contenu local.$c$,
     v_vid6, v_pdf, 20, 2);

    -- ── Chapitre 3 ────────────────────────────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Financement et perspectives$t$,
            $t$Financer le développement et préparer la transformation structurelle.$t$, 2)
    RETURNING id INTO v_ch3;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch3, $t$Inclusion financière et mobile money$t$,
     $c$L'Afrique est pionnière du paiement mobile, qui a fait bondir l'inclusion financière.
- Le rôle du mobile money (ex. M-Pesa) dans l'accès aux services financiers.
- Banques, microfinance et fintech.
- Impact sur l'épargne, le crédit et l'entrepreneuriat.$c$,
     v_vid1, NULL, 18, 0),

    (v_ch3, $t$Investissement, dette et aide$t$,
     $c$Comment financer routes, écoles, énergie et entreprises ?
- Investissements directs étrangers et partenariats public-privé.
- La question de la dette publique : soutenabilité et risques.
- Aide publique au développement : utilité et limites.$c$,
     v_vid2, NULL, 19, 1),

    (v_ch3, $t$Démographie, jeunesse et transformation$t$,
     $c$La jeunesse africaine est le principal atout du continent — à condition d'en faire un dividende démographique.
- Croissance démographique et emploi des jeunes.
- Capital humain : éducation, santé, compétences.
- Industrialisation et transformation structurelle comme moteurs d'emploi.$c$,
     v_vid3, v_pdf, 22, 2);
END IF;

-- ════════════════════════════════════════════════════════════════════════════
-- FORMATION 3 — Agriculture durable et agroécologie
-- ════════════════════════════════════════════════════════════════════════════
SELECT id INTO v_mooc_id FROM media_content.mooc WHERE slug = 'agriculture-durable-agroecologie';

IF v_mooc_id IS NOT NULL
   AND NOT EXISTS (SELECT 1 FROM media_content.formation_chapitre WHERE mooc_id = v_mooc_id AND deleted_at IS NULL)
THEN
    -- ── Chapitre 1 ────────────────────────────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Principes de l'agroécologie$t$,
            $t$Comprendre les fondements écologiques d'une agriculture durable.$t$, 0)
    RETURNING id INTO v_ch1;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch1, $t$Qu'est-ce que l'agroécologie ?$t$,
     $c$L'agroécologie applique les principes de l'écologie à la conception de systèmes agricoles durables et résilients.

Cette leçon couvre :
- La différence entre agriculture conventionnelle et agroécologie.
- Les trois dimensions : science, pratiques et mouvement social.
- Pourquoi l'agroécologie est particulièrement pertinente en Afrique.$c$,
     v_vid4, NULL, 15, 0),

    (v_ch1, $t$Santé et fertilité des sols$t$,
     $c$Le sol vivant est le capital de l'agriculteur.
- La vie du sol : micro-organismes, vers de terre, matière organique.
- Le cycle des nutriments (azote, phosphore, potassium).
- Pratiques qui dégradent vs régénèrent les sols.
- Mesurer et améliorer la fertilité.$c$,
     v_vid5, v_pdf, 20, 1),

    (v_ch1, $t$Biodiversité et associations de cultures$t$,
     $c$La diversité est une assurance contre les risques et les ravageurs.
- Polyculture, rotations et associations de cultures.
- Les cultures qui se rendent service mutuellement (ex. maïs–haricot–courge).
- Haies, bandes fleuries et auxiliaires de culture.$c$,
     v_vid6, NULL, 18, 2);

    -- ── Chapitre 2 ────────────────────────────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Pratiques de terrain$t$,
            $t$Les techniques concrètes pour produire sainement et durablement.$t$, 1)
    RETURNING id INTO v_ch2;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch2, $t$Compostage et fertilisation organique$t$,
     $c$Transformer les déchets organiques en fertilisant gratuit et de qualité.
- Les principes du compostage (carbone/azote, aération, humidité).
- Compost, fumier, paillage et engrais verts.
- Fabriquer et utiliser un bon compost, étape par étape.$c$,
     v_vid1, NULL, 19, 0),

    (v_ch2, $t$Gestion de l'eau et irrigation$t$,
     $c$L'eau est souvent le facteur limitant. La gérer avec sobriété est décisif.
- Techniques de conservation de l'eau dans le sol.
- Irrigation au goutte-à-goutte et micro-irrigation.
- Récupération des eaux de pluie et aménagements anti-érosion.$c$,
     v_vid2, NULL, 21, 1),

    (v_ch2, $t$Lutte biologique et gestion des ravageurs$t$,
     $c$Protéger les cultures sans dépendre des pesticides de synthèse.
- Comprendre l'écosystème : ravageurs et leurs ennemis naturels.
- Préparations naturelles et pièges.
- Gestion intégrée : prévention d'abord, intervention en dernier recours.$c$,
     v_vid3, v_pdf, 18, 2);

    -- ── Chapitre 3 ────────────────────────────────────────────────────────────
    INSERT INTO media_content.formation_chapitre (mooc_id, titre, description, ordre)
    VALUES (v_mooc_id, $t$Vers des systèmes résilients$t$,
            $t$Construire des exploitations adaptées au climat et viables économiquement.$t$, 2)
    RETURNING id INTO v_ch3;

    INSERT INTO media_content.formation_lecon (chapitre_id, titre, contenu, video_url, document_url, duree_minutes, ordre) VALUES
    (v_ch3, $t$Agroforesterie et systèmes intégrés$t$,
     $c$Associer arbres, cultures et élevage pour des systèmes plus productifs et résilients.
- Les bénéfices des arbres : ombre, fertilité, fourrage, bois.
- Intégration culture–élevage.
- Exemples de systèmes agroforestiers en Afrique.$c$,
     v_vid4, NULL, 20, 0),

    (v_ch3, $t$Adaptation au changement climatique$t$,
     $c$Le climat change : variétés résistantes, calendriers adaptés, diversification.
- Comprendre les risques climatiques locaux.
- Variétés tolérantes à la sécheresse et semences paysannes.
- Diversifier pour réduire le risque.$c$,
     v_vid5, NULL, 17, 1),

    (v_ch3, $t$Filières, marchés et certification$t$,
     $c$Produire durablement, c'est aussi bien vendre.
- Circuits courts et marchés de proximité.
- Coopératives et mise en marché collective.
- Labels et certifications (bio, équitable) : intérêt et exigences.$c$,
     v_vid6, v_pdf, 19, 2);
END IF;

END $seed$;
