-- ============================================================================
-- 09u — Lignes éditoriales d'Africans Télé International (US : pastille dédiée)
--
-- Les 44 lignes éditoriales semées par 09j (« Retour des cerveaux », « Haro
-- sur les hommes de l'Afrique »...) avaient été désactivées par 09s au profit
-- des 22 genres de grille (Journal télévisé, Débats et analyses...) : ce que
-- TOUS les supports déclarent pour se signaler dans le catalogue.
--
-- Les 44 redeviennent sélectionnables, mais pour un usage DIFFÉRENT et
-- SÉPARÉ : la pastille de filtre « Africans Télé International » (barre de
-- la page Télé) porte désormais son propre panneau, peuplé par ces 44 lignes
-- et non par les 22 genres de 09s. Un `parent_id` commun les regroupe sous un
-- nœud technique dédié, jamais lui-même sélectionnable :
--   • les sélecteurs génériques (référentiel d'édition des thématiques d'un
--     support, panneau « Africans Thématique ») filtrent `parent_id IS NULL`
--     et ne les voient donc pas — 09s reste pleinement en vigueur pour eux ;
--   • seul le nouvel appel `?groupe=media-groupe-africans-tele-international`
--     (`GET .../thematiques`) les remonte, filtrant sur ce `parent_id`.
-- Aucune ligne n'est dupliquée : ce sont les 44 lignes de 09j, réactivées.
--
-- Chaque ligne gagne aussi une `description` : le nom court reste le libellé
-- de la pastille, la description s'affiche en infobulle native au survol.
--
-- Idempotent (INSERT ON CONFLICT DO NOTHING, UPDATE inconditionnel).
-- ============================================================================

BEGIN;

-- Nœud de regroupement technique : jamais sélectionnable (actif = FALSE), il
-- ne sert qu'à porter un `parent_id` distinctif pour les 44 lignes ci-dessous.
INSERT INTO shared.categorie (nom, slug, contexte, actif, description) VALUES
    ('Africans Télé International (groupe)',
     'media-groupe-africans-tele-international',
     'media', FALSE,
     'Regroupement technique des lignes éditoriales de la pastille de filtre « Africans Télé International ». Non sélectionnable.')
ON CONFLICT (slug) DO NOTHING;

-- Réactivation + rattachement au groupe + description (infobulle).
UPDATE shared.categorie SET
    parent_id = (SELECT id FROM shared.categorie WHERE slug = 'media-groupe-africans-tele-international'),
    actif = TRUE,
    updated_at = NOW(),
    description = CASE slug
        WHEN 'media-journal-de-l-afrique' THEN
            'Avancées de décisions de l''Union africaine ; actions renforçant l''unité africaine ; richesses et diversités de l''Afrique, etc.'
        WHEN 'media-haro-sur-les-hommes-de-l-afrique' THEN
            'Zoom sur les hommes qui prônent l''unité africaine.'
        WHEN 'media-l-intellectuel-africain-et-developpement' THEN
            'Décryptage d''un sujet, focus sur l''intérêt pour l''Afrique.'
        WHEN 'media-afrique-et-technologies' THEN
            'Mise en avant du développement, du déploiement et de la vulgarisation de la technologie africaine.'
        WHEN 'media-savoirs-faire-d-afrique' THEN
            'Vulgarisation des savoirs locaux, des richesses locales et des terroirs.'
        WHEN 'media-cuisine-de-chez-nous' THEN
            'Promotion des mets africains des quatre coins d''Afrique.'
        WHEN 'media-politique-africaine' THEN
            'Mise en avant des décisions importantes faites ou nécessaires pour l''unité africaine, et mise en scène des hommes politiques qui s''illustrent dans ce sens.'
        WHEN 'media-de-la-these-a-l-action-locale' THEN
            'Valorisation des thèses et mémoires africains pertinents faits dans les pays occidentaux et pouvant être vulgarisés dans les pays africains.'
        WHEN 'media-la-voix-du-terrain-en-afrique' THEN
            'Présentation des vécus des populations locales.'
        WHEN 'media-debats-africains' THEN
            'Débats sur les questions fâcheuses : nationalité, intégration, etc.'
        WHEN 'media-mysteres-africains' THEN
            'Mise en avant des spécificités de l''Afrique, des religions et des tabous (religions d''Afrique, traditions, etc.).'
        WHEN 'media-droit-africain' THEN
            'Mise en avant du droit africain et des défis pour la mondialisation.'
        WHEN 'media-environnement-d-afrique' THEN
            'Enjeux environnementaux et Afrique.'
        WHEN 'media-rendez-vous-des-hauts-et-des-bas' THEN
            'Débats entre intellectuels et moins instruits.'
        ELSE description
    END
WHERE contexte = 'media'
  AND slug IN (
    'media-retour-des-cerveaux', 'media-histoire-de-l-afrique', 'media-valeurs-africaines-et-developpement',
    'media-journal-de-l-afrique', 'media-haro-sur-les-hommes-de-l-afrique', 'media-l-intellectuel-africain-et-developpement',
    'media-afrique-et-technologies', 'media-savoirs-faire-d-afrique', 'media-cuisine-de-chez-nous',
    'media-politique-africaine', 'media-de-la-these-a-l-action-locale', 'media-la-voix-du-terrain-en-afrique',
    'media-debats-africains', 'media-mysteres-africains', 'media-droit-africain',
    'media-environnement-d-afrique', 'media-regards-de-la-jeunesse-africaine', 'media-femmes-d-afrique',
    'media-gouvernance-d-afrique-aux-defis', 'media-infrastructures-d-afrique', 'media-sante-et-developpement',
    'media-numerique-et-developpement-africain', 'media-traditions-d-afrique', 'media-mondialisation-et-cooperation-africaine',
    'media-commerce-africain-et-unite-africaine', 'media-developpement-durable', 'media-le-monde-de-demain-et-mondialisation',
    'media-immigration-et-l-avenir-de-l-afrique', 'media-sports-d-afrique', 'media-rendez-vous-des-hauts-et-des-bas',
    'media-education-carres-instruction-afrique', 'media-education-carres-ecole-de-la-vie', 'media-education-carres-education-a-l-africaine',
    'media-l-afrique-que-nous-voulons', 'media-messages-aux-gouvernants', 'media-cinema-africain',
    'media-series-d-afrique', 'media-documentaires-africains', 'media-safari-d-afrique',
    'media-futurs-genies-d-afrique', 'media-innovations-simples-chez-nous', 'media-complexes-d-afrique',
    'media-afrique-societe', 'media-afrique-solidarite');

DO $$
DECLARE v_reactivees INT;
BEGIN
    SELECT count(*) INTO v_reactivees
      FROM shared.categorie
     WHERE contexte = 'media' AND actif
       AND parent_id = (SELECT id FROM shared.categorie WHERE slug = 'media-groupe-africans-tele-international');
    RAISE NOTICE '09u — lignes éditoriales Africans Télé International réactivées : %.', v_reactivees;
END $$;

COMMIT;
