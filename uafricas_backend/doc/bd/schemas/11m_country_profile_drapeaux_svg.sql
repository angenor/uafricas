-- ═══════════════════════════════════════════════════════════════════════
-- 11m — Drapeaux servis en SVG plutôt qu'en PNG matriciel
-- ═══════════════════════════════════════════════════════════════════════
--
-- Les fiches pointaient vers `flagcdn.com/w160/xx.png` (ou `w320`) : une image
-- matricielle de largeur fixe. Affichée à 48 px de haut dans le rail elle est
-- surdimensionnée, et sur un écran à forte densité elle est floue. Le même
-- hôte sert le SVG à `flagcdn.com/xx.svg` : net à toute taille, et plus léger.
--
-- Les seeds `20_` et `30_` sont corrigés en amont ; cette migration rattrape
-- les bases DÉJÀ peuplées. Sans elle, seule une base repartie de zéro aurait
-- les SVG.
--
-- Le motif est ancré (`^…$`) et ne touche QUE les URL flagcdn de cette forme :
-- un drapeau téléversé sous ./uploads/ ou saisi à la main est laissé intact.
--
-- `image_couverture_url` n'est PAS touché : `30_seed_pays_africains.sql` y met
-- lui aussi un drapeau, faute de photo, mais c'est un autre champ et un autre
-- sujet — le remplacer par une vraie photo de territoire est un travail
-- éditorial, pas une migration.
-- ═══════════════════════════════════════════════════════════════════════

UPDATE country_profile.fiche_pays
   SET image_drapeau_url = regexp_replace(
           image_drapeau_url,
           '^https://flagcdn\.com/w[0-9]+/([a-z]{2})\.png$',
           'https://flagcdn.com/\1.svg'
       )
 WHERE image_drapeau_url ~ '^https://flagcdn\.com/w[0-9]+/[a-z]{2}\.png$';
