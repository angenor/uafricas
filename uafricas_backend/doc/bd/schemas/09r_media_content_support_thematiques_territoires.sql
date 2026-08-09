-- ============================================================================
-- 09r — media_content : thématiques multiples et couverture territoriale
-- ----------------------------------------------------------------------------
-- Un support média (chaîne TV ou station radio) ne portait qu'UNE catégorie
-- (`chaine_tv.categorie`, enum) et UN pays (`pays_id`). Une chaîne panafricaine
-- généraliste n'était donc trouvable que par un seul thème et un seul
-- territoire — précisément ce que le public cherche à croiser.
--
--   • support_thematique — 1..N thèmes du référentiel `shared.categorie`
--     (contexte 'media', 44 thèmes seedés par 09j §7) ;
--   • support_territoire — 0..N territoires, EXCLUSIF de
--     `couverture_continentale`, drapeau ajouté sur les deux tables de support.
--
-- Les deux tables sont polymorphes par (type_support, support_id), patron maison
-- employé identiquement par support_detenteur (09m), creneau_programmation (09n)
-- et les quatre tables d'interactions (09k) : une seule table sert la télé et la
-- radio, une seule requête sert les deux filtres.
--
-- ── Pourquoi un trigger et non un CHECK ────────────────────────────────────
-- L'exclusivité « territoires OU continentale » porte sur DEUX tables : aucun
-- CHECK ne peut l'exprimer. Le projet privilégie systématiquement les invariants
-- exprimés en SQL (cinq CHECK sur transaction_cadeau, quatre sur
-- proposition_media) ; le trigger est ici la seule expression possible, et il
-- reste peu coûteux — il ne se déclenche qu'à l'écriture d'un territoire.
--
-- ── Indépendance vis-à-vis de 09q ──────────────────────────────────────────
-- Cette migration est PUREMENT ADDITIVE : elle ne touche ni `programme_*`, ni
-- `emission_*`, ni `episode_*`. Elle s'applique avant comme après 09q.
--
-- Prérequis : 09 (chaine_tv, station_radio), 09j (référentiel `media` de
-- shared.categorie), 09m (ENUM type_support_media).
--
-- Migration IDEMPOTENTE : CREATE … IF NOT EXISTS, ON CONFLICT DO NOTHING,
-- CREATE OR REPLACE FUNCTION, DROP TRIGGER IF EXISTS puis CREATE.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. support_thematique — thématiques multiples (US3)
-- ════════════════════════════════════════════════════════════════════════════
-- `categorie_id` est une référence LOGIQUE [xref] vers shared.categorie, sans
-- FK — convention explicite du projet, déjà suivie par `theme_phare_id` (09j).
--
-- UNIQUE (type_support, support_id, categorie_id) : un thème n'est déclaré
-- qu'une fois par support, ce qui rend le « sans doublon » de FR-030 structurel
-- plutôt qu'affaire de vigilance applicative.

CREATE TABLE IF NOT EXISTS media_content.support_thematique (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_support media_content.type_support_media NOT NULL,
    support_id   UUID        NOT NULL,
    categorie_id UUID        NOT NULL,      -- [xref] shared.categorie (contexte='media')
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_support_thematique UNIQUE (type_support, support_id, categorie_id)
);

-- Filtre inverse : « quels supports portent ce thème ? »
CREATE INDEX IF NOT EXISTS idx_support_thematique_categorie
    ON media_content.support_thematique (categorie_id);
-- Fiche du support : « quels thèmes porte-t-il ? »
CREATE INDEX IF NOT EXISTS idx_support_thematique_support
    ON media_content.support_thematique (type_support, support_id);

COMMENT ON TABLE media_content.support_thematique IS
    'Thématiques déclarées d''un support média. Remplace chaine_tv.categorie et
     station_radio.genre, conservées en lecture le temps de la reprise.';


-- ── 1.1 Reprise : la catégorie unique devient la première thématique ────────
-- FR-057. Par correspondance de LIBELLÉ avec le référentiel `media` : l'enum
-- `categorie_chaine_tv` et les 44 thèmes ne se recouvrent que partiellement.
-- Les supports sans correspondance restent SANS THÉMATIQUE — la spec le tolère
-- en lecture, l'obligation ne valant qu'à la prochaine modification (FR-029).
-- Le rapport de reprise liste le travail éditorial restant.

INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
SELECT 'chaine_tv', ct.id, cat.id
  FROM media_content.chaine_tv ct
  JOIN shared.categorie cat
    ON cat.contexte = 'media'
   AND lower(cat.nom) = lower(ct.categorie::text)
 WHERE ct.deleted_at IS NULL
ON CONFLICT DO NOTHING;

-- Côté radio, `genres_liste` est déjà un tableau : chacun de ses éléments est
-- candidat, en plus du genre principal.
INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
SELECT 'station_radio', sr.id, cat.id
  FROM media_content.station_radio sr
  JOIN shared.categorie cat
    ON cat.contexte = 'media'
   AND lower(cat.nom) = lower(sr.genre)
 WHERE sr.deleted_at IS NULL AND sr.genre IS NOT NULL
ON CONFLICT DO NOTHING;

INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
SELECT 'station_radio', sr.id, cat.id
  FROM media_content.station_radio sr
  CROSS JOIN LATERAL unnest(COALESCE(sr.genres_liste, ARRAY[]::text[])) AS g(nom)
  JOIN shared.categorie cat
    ON cat.contexte = 'media'
   AND lower(cat.nom) = lower(g.nom)
 WHERE sr.deleted_at IS NULL
ON CONFLICT DO NOTHING;


-- ════════════════════════════════════════════════════════════════════════════
-- 2. support_territoire et couverture continentale (US4)
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE media_content.chaine_tv
    ADD COLUMN IF NOT EXISTS couverture_continentale BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS couverture_continentale BOOLEAN NOT NULL DEFAULT FALSE;

COMMENT ON COLUMN media_content.chaine_tv.couverture_continentale IS
    'Diffusion panafricaine. EXCLUSIF de support_territoire — le trigger
     verifier_couverture_exclusive refuse l''ajout d''un territoire individuel.';

CREATE TABLE IF NOT EXISTS media_content.support_territoire (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_support media_content.type_support_media NOT NULL,
    support_id   UUID        NOT NULL,
    pays_id      UUID        NOT NULL,      -- [xref] shared.pays
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_support_territoire UNIQUE (type_support, support_id, pays_id)
);

CREATE INDEX IF NOT EXISTS idx_support_territoire_pays
    ON media_content.support_territoire (pays_id);
CREATE INDEX IF NOT EXISTS idx_support_territoire_support
    ON media_content.support_territoire (type_support, support_id);

COMMENT ON TABLE media_content.support_territoire IS
    'Territoires couverts par un support média. Vide et le restant lorsque le
     support déclare une couverture continentale.';


-- ── 2.1 Trigger d'exclusivité (FR-034) ─────────────────────────────────────
-- Sans lui, l'exclusivité resterait une convention applicative — qu'un import ou
-- un correctif SQL manuel contournerait sans bruit.

CREATE OR REPLACE FUNCTION media_content.verifier_couverture_exclusive()
RETURNS TRIGGER AS $$
DECLARE
    continentale BOOLEAN;
BEGIN
    EXECUTE format(
        'SELECT couverture_continentale FROM media_content.%I WHERE id = $1',
        CASE NEW.type_support WHEN 'chaine_tv' THEN 'chaine_tv' ELSE 'station_radio' END)
    INTO continentale
    USING NEW.support_id;

    IF continentale THEN
        RAISE EXCEPTION
            'Couverture continentale déclarée : aucun territoire individuel ne peut être ajouté (FR-034)';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_support_territoire_exclusivite
    ON media_content.support_territoire;
CREATE TRIGGER trg_support_territoire_exclusivite
    BEFORE INSERT ON media_content.support_territoire
    FOR EACH ROW EXECUTE FUNCTION media_content.verifier_couverture_exclusive();


-- ── 2.2 Reprise : `pays_id` devient l'unique territoire ─────────────────────
-- FR-057. Aucun support n'étant continental à ce stade, le trigger laisse
-- passer.

INSERT INTO media_content.support_territoire (type_support, support_id, pays_id)
SELECT 'chaine_tv', ct.id, ct.pays_id
  FROM media_content.chaine_tv ct
 WHERE ct.pays_id IS NOT NULL AND ct.deleted_at IS NULL
ON CONFLICT DO NOTHING;

INSERT INTO media_content.support_territoire (type_support, support_id, pays_id)
SELECT 'station_radio', sr.id, sr.pays_id
  FROM media_content.station_radio sr
 WHERE sr.pays_id IS NOT NULL AND sr.deleted_at IS NULL
ON CONFLICT DO NOTHING;
