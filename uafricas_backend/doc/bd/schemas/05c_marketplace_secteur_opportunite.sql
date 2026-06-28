-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Migration : marketplace
--   • Nouveau type d'opération « opportunité d'investissement »
--   • Champ « secteur » sur les annonces (référentiel shared.domaine_secteur
--     OU libellé libre via secteur_autre quand l'auteur choisit « Autre »)
-- Idempotent — réexécutable sans effet de bord.
-- ════════════════════════════════════════════════════════════════════════════

-- 1) Nouvelle valeur d'enum (PostgreSQL 12+ : ADD VALUE IF NOT EXISTS).
--    N'est PAS utilisée dans une INSERT au sein de cette même migration.
ALTER TYPE marketplace.type_operation ADD VALUE IF NOT EXISTS 'opportunite_investissement';

-- 2) Colonnes secteur sur l'annonce (xref logique vers shared.domaine_secteur,
--    sans contrainte FK — même convention que categorie_id).
ALTER TABLE marketplace.annonce
    ADD COLUMN IF NOT EXISTS secteur_id    UUID,          -- [xref] shared.domaine_secteur
    ADD COLUMN IF NOT EXISTS secteur_autre VARCHAR(200);  -- libellé libre si « Autre »

CREATE INDEX IF NOT EXISTS idx_annonce_secteur
    ON marketplace.annonce(secteur_id) WHERE deleted_at IS NULL;

-- 3) Secteurs d'activité supplémentaires pour le menu déroulant du marché
--    (référentiel partagé). Idempotent via ON CONFLICT sur le slug unique.
--    Les 12 secteurs historiques restent définis dans 15_seed.sql.
INSERT INTO shared.domaine_secteur (nom, slug) VALUES
    ('Mines & Ressources naturelles', 'mines-ressources'),
    ('Tourisme & Hôtellerie',         'tourisme-hotellerie'),
    ('Immobilier & BTP',              'immobilier-btp'),
    ('Transport & Logistique',        'transport-logistique'),
    ('Industrie & Manufacture',       'industrie-manufacture'),
    ('Commerce & Distribution',       'commerce-distribution'),
    ('Finance & Assurance',           'finance-assurance'),
    ('Télécommunications',            'telecommunications'),
    ('Élevage & Pêche',               'elevage-peche'),
    ('Textile & Mode',                'textile-mode'),
    ('Médias & Communication',        'medias-communication'),
    ('Artisanat',                     'artisanat')
ON CONFLICT (slug) DO NOTHING;
