-- ============================================================================
-- Purge du jeu de démonstration de la vitrine télé (011_demo_tele_vitrine.sql)
--
-- Retire les 12 chaînes ajoutées, leurs programmes, épisodes, grilles, équipes,
-- thématiques, territoires et interactions — et REPUBLIE les chaînes de test
-- que le seed avait basculées en `brouillon`.
--
-- L'enrichissement de « Panorama Continental » (titres et couvertures de ses 31
-- programmes) n'est pas défait : il ne fait qu'habiller des lignes qui
-- préexistaient au seed, et les défaire supposerait de reconstituer des
-- libellés que personne ne regrette.
-- ============================================================================

\set ON_ERROR_STOP on

BEGIN;

DO $purge$
DECLARE
    v_slugs TEXT[] := ARRAY[
        'africans-tele-international','sahel-info-tv','lagos-business-channel',
        'nil-sport','kilimandjaro-nature','kin-musique-tv','atlas-maghreb-tv',
        'cap-sud-divertissement','teranga-jeunesse','abidjan-talk-tv',
        'foi-esperance-tv','sawa-tv'];
    v_ids   UUID[];
BEGIN
    SELECT array_agg(id) INTO v_ids
      FROM media_content.chaine_tv WHERE slug = ANY(v_slugs);

    IF v_ids IS NULL THEN
        RAISE NOTICE 'Jeu de démonstration absent — rien à purger.';
    ELSE
        -- Les épisodes d'abord : `episode_tele.emission_id` est en ON DELETE
        -- RESTRICT, la suppression d'un programme échouerait sinon.
        DELETE FROM media_content.episode_tele ep
         USING media_content.emission_tele em
         WHERE ep.emission_id = em.id AND em.chaine_id = ANY(v_ids);

        DELETE FROM media_content.creneau_programmation
         WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.membre_equipe
         WHERE type_porteur = 'chaine_tv' AND porteur_id = ANY(v_ids);
        DELETE FROM media_content.support_thematique
         WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.support_territoire
         WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.support_detenteur
         WHERE type_support = 'chaine_tv' AND support_id = ANY(v_ids);
        DELETE FROM media_content.media_reaction
         WHERE type_media = 'chaine_tv' AND media_id = ANY(v_ids);
        DELETE FROM media_content.media_commentaire
         WHERE type_media = 'chaine_tv' AND media_id = ANY(v_ids);
        DELETE FROM media_content.partage_media
         WHERE type_media = 'chaine_tv' AND media_id = ANY(v_ids);

        DELETE FROM media_content.emission_tele WHERE chaine_id = ANY(v_ids);
        DELETE FROM media_content.chaine_tv WHERE id = ANY(v_ids);
    END IF;

    -- Remise en vitrine des chaînes de test masquées par le seed.
    UPDATE media_content.chaine_tv
       SET etat = 'publie'
     WHERE etat = 'brouillon'
       AND slug IN ('africa24-test','chaine-vide','sahel-culture','test-chaine-1',
                    'test-chaine-2','chaine-test-007','chaine-proposee-007',
                    'africans-doc-test','africans-innovation-test');
END
$purge$;

COMMIT;
