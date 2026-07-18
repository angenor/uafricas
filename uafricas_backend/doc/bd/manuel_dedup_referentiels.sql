-- ════════════════════════════════════════════════════════════════════════════
-- MAINTENANCE MANUELLE — Dédoublonnage des référentiels ethnies (audit #34)
-- ════════════════════════════════════════════════════════════════════════════
-- ⚠️  Hors dossier schemas/ : jamais exécuté automatiquement. Backup avant.
--
-- Problème : la même ethnie apparaît sous plusieurs graphies (Haoussa/Hausa,
-- Balanta/Balante) et des concepts distincts se mélangent (Amhara = peuple,
-- Amharique = langue). Deux tables sont concernées :
--   • country_profile.groupe_ethnique (par fiche pays, colonnes: fiche_pays_id, nom, langues)
--   • la liste de référence seedée dans 28_seed_groupes_ethniques.sql
-- Extension pg_trgm déjà installée (matching arbres) → utilisable ici.
-- ════════════════════════════════════════════════════════════════════════════


-- ── (1) DÉTECTION : doublons EXACTS (à la casse près) dans une même fiche ────
SELECT fiche_pays_id, LOWER(TRIM(nom)) AS cle, COUNT(*), ARRAY_AGG(nom)
  FROM country_profile.groupe_ethnique
 GROUP BY fiche_pays_id, LOWER(TRIM(nom))
HAVING COUNT(*) > 1
 ORDER BY COUNT(*) DESC;


-- ── (2) DÉTECTION : quasi-doublons (graphies proches) dans une même fiche ────
-- similarity() vient de pg_trgm ; seuil 0.45 attrape Haoussa/Hausa, Balanta/Balante.
SELECT a.fiche_pays_id, a.nom AS nom_a, b.nom AS nom_b,
       ROUND(similarity(a.nom, b.nom)::numeric, 2) AS score
  FROM country_profile.groupe_ethnique a
  JOIN country_profile.groupe_ethnique b
    ON a.fiche_pays_id = b.fiche_pays_id
   AND a.id < b.id
   AND LOWER(TRIM(a.nom)) <> LOWER(TRIM(b.nom))
   AND similarity(a.nom, b.nom) > 0.45
 ORDER BY score DESC;


-- ── (3) NORMALISATION : appliquer une graphie canonique ─────────────────────
-- Remplir la table de correspondance ci-dessous APRÈS validation produit, puis
-- décommenter l'UPDATE. Graphies recommandées (UI en français) :
--     'Hausa'      -> 'Haoussa'
--     'Balante'    -> 'Balanta'   (ou l'inverse, à trancher)
--     'Amharique'  -> À RETIRER des ethnies (c'est une LANGUE, pas un peuple)
--
-- WITH canon(variante, canonique) AS (VALUES
--     ('Hausa',   'Haoussa'),
--     ('Balante', 'Balanta')
-- )
-- UPDATE country_profile.groupe_ethnique g
--    SET nom = c.canonique
--   FROM canon c
--  WHERE LOWER(TRIM(g.nom)) = LOWER(c.variante);
--
-- Puis supprimer les doublons exacts nouvellement créés (garder le plus ancien) :
-- DELETE FROM country_profile.groupe_ethnique g
--  USING country_profile.groupe_ethnique g2
--  WHERE g.fiche_pays_id = g2.fiche_pays_id
--    AND LOWER(TRIM(g.nom)) = LOWER(TRIM(g2.nom))
--    AND g.ctid > g2.ctid;   -- garde une seule occurrence


-- ── (4) SOURCE DES DOUBLONS À CORRIGER DANS LES SEEDS ───────────────────────
-- 28_seed_groupes_ethniques.sql écrit 'Hausa' (NG/NE) tandis que
-- 20_seed_fiches_pays.sql écrit 'Haoussa' pour le Nigéria → aligner les deux
-- fichiers sur la graphie canonique retenue pour éviter la réapparition au reseed.
