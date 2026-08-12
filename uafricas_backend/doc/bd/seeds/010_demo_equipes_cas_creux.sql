-- ============================================================================
-- Jeu de démonstration de la feature 010 — les CAS CREUX
--
-- Cette feature se juge autant sur ce qu'elle affiche que sur ce qu'elle
-- n'affiche pas. Les objets ci-dessous sont ceux qui font échouer une
-- implémentation naïve ; sans eux, la moitié des exigences est invérifiable :
--
--   • une chaîne à description LONGUE (> 900 caractères)   → FR-003 / FR-021
--   • une chaîne portant PLUS DE 30 programmes             → FR-008, plafond
--   • un programme PUBLIÉ SANS AUCUN ÉPISODE publié        → FR-005, FR-033
--   • un programme à description longue (> 400 caractères) → FR-004
--   • un programme MENSUEL                                 → FR-040, alertes
--
-- Le cas « chaîne sans aucun programme » est déjà couvert par « Chaine Vide »
-- du jeu existant : rien à créer.
--
-- Écrit en SQL et non par l'API, comme 009_demo_medias.sql : les invariants que
-- l'API défend sont reproduits ici explicitement (état, cadence, slug unique).
-- Le programme mensuel N'EST PAS saisissable en back-office avant la Phase 7 —
-- le CHECK de 09t l'accepte, le sélecteur ne le proposera qu'ensuite.
--
-- Idempotent : garde en tête, ON CONFLICT sur les slugs.
-- ============================================================================

\set ON_ERROR_STOP on

BEGIN;

DO $seed$
DECLARE
    v_auteur       UUID;
    v_chaine       UUID;
    v_emission     UUID;
    v_description  TEXT;
    v_desc_prog    TEXT;
    i              INT;
BEGIN
    SELECT id INTO v_auteur FROM iam.utilisateur WHERE email = 'angenor99@gmail.com';
    IF v_auteur IS NULL THEN
        SELECT id INTO v_auteur FROM iam.utilisateur WHERE email = 'test-admin@test.com';
    END IF;
    IF v_auteur IS NULL THEN
        RAISE EXCEPTION 'Compte auteur introuvable — seed interrompu';
    END IF;

    IF EXISTS (SELECT 1 FROM media_content.chaine_tv
                WHERE slug = 'panorama-continental-010' AND deleted_at IS NULL) THEN
        RAISE NOTICE 'Cas creux 010 déjà en place — rien à faire.';
        RETURN;
    END IF;

    -- ── Description longue : ~1100 caractères, au-delà du seuil de troncature
    v_description := 'Panorama Continental est une chaîne généraliste panafricaine qui '
        || 'consacre l''essentiel de sa grille au récit du continent par ceux qui y vivent. '
        || 'Née d''un collectif de rédactions indépendantes réparties entre Dakar, Abidjan, '
        || 'Douala, Nairobi et Johannesburg, elle produit chaque semaine des magazines '
        || 'd''actualité, des documentaires de création, des débats en public et des '
        || 'programmes courts destinés aux jeunes publics. Sa ligne éditoriale tient en une '
        || 'phrase : montrer l''Afrique en train de se faire, sans exotisme ni misérabilisme, '
        || 'en donnant la parole aux praticiens plutôt qu''aux commentateurs. La chaîne '
        || 'diffuse en français, en anglais et en swahili, et sous-titre l''intégralité de ses '
        || 'documentaires dans les trois langues. Elle collabore avec une trentaine de '
        || 'producteurs indépendants dont elle assure la formation technique, et reverse une '
        || 'part de ses recettes de diffusion à un fonds de soutien à la création '
        || 'audiovisuelle continentale. Son équipe éditoriale réunit des journalistes, des '
        || 'réalisateurs et des documentaristes issus de quinze territoires.';

    -- ── Description de programme longue : ~520 caractères
    v_desc_prog := 'Un magazine hebdomadaire consacré aux transformations silencieuses du '
        || 'continent : celles qui ne font pas la une mais changent durablement la vie des '
        || 'gens. Chaque numéro part d''un lieu précis — un marché, un hôpital de district, '
        || 'une coopérative agricole, un atelier de réparation — et remonte le fil de ce qui '
        || 'y a changé en dix ans. Le magazine refuse le format du reportage-catastrophe et '
        || 'privilégie le temps long, avec des séquences tournées sur plusieurs mois et des '
        || 'retours réguliers auprès des mêmes personnes.';

    -- ════════════════════════════════════════════════════════════════════════
    -- 1. Chaîne à description longue, portant plus de 30 programmes (FR-008)
    -- ════════════════════════════════════════════════════════════════════════
    INSERT INTO media_content.chaine_tv
        (nom, slug, description, categorie, langue, est_en_direct, etat,
         origine_publication, cree_par)
    VALUES
        ('Panorama Continental', 'panorama-continental-010', v_description,
         'generaliste', 'Français', FALSE, 'publie', 'territoire', v_auteur)
    RETURNING id INTO v_chaine;

    -- 32 programmes publiés : au-delà du plafond de 30 par section, la vitrine
    -- doit ANNONCER le total et mener au reste, jamais tronquer en silence.
    FOR i IN 1..32 LOOP
        INSERT INTO media_content.emission_tele
            (chaine_id, titre, slug, description, cadence, etat, cree_par)
        VALUES
            (v_chaine,
             'Panorama — numéro ' || lpad(i::text, 2, '0'),
             'panorama-continental-010-prog-' || lpad(i::text, 2, '0'),
             CASE WHEN i = 1 THEN v_desc_prog
                  ELSE 'Rendez-vous régulier de la grille Panorama Continental.' END,
             CASE WHEN i = 1 THEN 'hebdomadaire' ELSE 'ponctuelle' END,
             'publie', v_auteur);
    END LOOP;

    -- ════════════════════════════════════════════════════════════════════════
    -- 2. Programme publié SANS AUCUN épisode publié (FR-005, FR-033)
    -- ════════════════════════════════════════════════════════════════════════
    -- Avant cette feature, un tel programme était invisible en vitrine ET
    -- renvoyait 404 sur sa page de détail. Les deux comportements tombent.
    INSERT INTO media_content.emission_tele
        (chaine_id, titre, slug, description, cadence, etat, cree_par)
    VALUES
        (v_chaine, 'Annoncé mais pas encore tourné',
         'annonce-pas-encore-tourne-010',
         'Ce programme est annoncé dans la grille : sa première diffusion n''a pas '
         || 'encore eu lieu. Il doit rester listé et sa page rester consultable.',
         'ponctuelle', 'publie', v_auteur);

    -- ════════════════════════════════════════════════════════════════════════
    -- 3. Programme MENSUEL (FR-040) — non saisissable avant la Phase 7
    -- ════════════════════════════════════════════════════════════════════════
    INSERT INTO media_content.emission_tele
        (chaine_id, titre, slug, description, cadence, etat, cree_par)
    VALUES
        (v_chaine, 'Le grand format du mois',
         'grand-format-du-mois-010',
         'Un documentaire de création par mois, produit avec un réalisateur du continent.',
         'mensuelle', 'publie', v_auteur)
    RETURNING id INTO v_emission;

    -- Pendant radio du programme mensuel, pour éprouver la parité (FR-060).
    IF EXISTS (SELECT 1 FROM media_content.station_radio
                WHERE slug = 'radio-africans-voix' AND deleted_at IS NULL) THEN
        INSERT INTO media_content.emission_radio
            (station_id, titre, slug, description, cadence, etat, cree_par)
        SELECT sr.id, 'La chronique mensuelle', 'chronique-mensuelle-010',
               'Une chronique longue, une fois par mois.', 'mensuelle', 'publie', v_auteur
          FROM media_content.station_radio sr
         WHERE sr.slug = 'radio-africans-voix' AND sr.deleted_at IS NULL
        ON CONFLICT (slug) DO NOTHING;

        INSERT INTO media_content.emission_radio
            (station_id, titre, slug, description, cadence, etat, cree_par)
        SELECT sr.id, 'Antenne libre (sans diffusion)', 'antenne-libre-sans-diffusion-010',
               'Programme annoncé, aucune diffusion enregistrée à ce jour.',
               'ponctuelle', 'publie', v_auteur
          FROM media_content.station_radio sr
         WHERE sr.slug = 'radio-africans-voix' AND sr.deleted_at IS NULL
        ON CONFLICT (slug) DO NOTHING;
    END IF;

    RAISE NOTICE 'Cas creux 010 créés — chaîne %, 34 programmes télé dont 1 mensuel et 1 sans épisode.', v_chaine;
END
$seed$;

COMMIT;
