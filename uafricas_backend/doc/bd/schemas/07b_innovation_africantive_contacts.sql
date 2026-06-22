-- ════════════════════════════════════════════════════════════════════
-- 07b_innovation_africantive_contacts.sql
-- Enrichissement des africantives : site web, réseau social, domaine
-- « Autre » (précision) et deux contacts de l'initiateur.
-- Migration idempotente (rétrocompat : colonnes nullables).
-- ════════════════════════════════════════════════════════════════════

-- Liens
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS site_web_url        VARCHAR(500);
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS lien_reseau_social  VARCHAR(500);

-- Précision du domaine quand « Autre » est choisi
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS domaine_autre       VARCHAR(200);

-- Contact 1 de l'initiateur
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS contact1_courriel   VARCHAR(255);
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS contact1_telephone  VARCHAR(50);
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS contact1_adresse    VARCHAR(350);

-- Contact 2 de l'initiateur
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS contact2_courriel   VARCHAR(255);
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS contact2_telephone  VARCHAR(50);
ALTER TABLE innovation.africantive
    ADD COLUMN IF NOT EXISTS contact2_adresse    VARCHAR(350);

-- Nouveau domaine « Union des africains » (résolu par nom/slug à la création)
INSERT INTO shared.domaine_secteur (nom, slug)
VALUES ('Union des africains', 'union-des-africains')
ON CONFLICT (slug) DO NOTHING;
