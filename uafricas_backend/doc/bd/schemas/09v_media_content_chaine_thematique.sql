-- ============================================================================
-- 09v — media_content : chaîne thématique vs chaîne territoriale
-- ----------------------------------------------------------------------------
-- Un support média déclare depuis 09r une couverture : soit des territoires
-- (`support_territoire`, 1..N), soit toute l'Afrique
-- (`couverture_continentale`), les deux étant exclusifs.
--
-- Il manquait la SECONDE nature de support, portée par l'exigence métier :
--
--     « Une chaîne peut être thématique. Lorsqu'elle est thématique, elle n'est
--       pas liée à un territoire, elle concerne d'office tous les territoires
--       (une chaîne thématique concerne une seule thématique). »
--
-- D'où `est_thematique`, et deux invariants qui en découlent :
--
--   1. thématique ⇒ couverture continentale. Un simple CHECK suffit : les deux
--      colonnes vivent sur la même ligne. C'est ce CHECK qui rend le champ
--      « territoires » inutile à l'écran — il n'est pas masqué par politesse
--      d'interface, il est SANS OBJET.
--   2. thématique ⇒ AU PLUS une thématique de grille. Celle-là porte sur une
--      autre table (`support_thematique`) et sur la nature de la catégorie
--      (`shared.categorie.parent_id`) : ni CHECK ni index unique partiel ne
--      peuvent l'exprimer, d'où un trigger, comme pour l'exclusivité de 09r.
--
-- ── Portée de « au plus une » ──────────────────────────────────────────────
-- La règle ne vise QUE les thématiques de grille (`parent_id IS NULL`), jamais
-- les 44 lignes éditoriales d'Africans Télé International (09u), qu'une chaîne
-- déclare par paquets et qui vivent sous un `parent_id` dédié. Les confondre
-- rendrait insérable une seule ligne éditoriale par chaîne et casserait le
-- filtre livré par 09u.
--
-- « Au plus une » et non « exactement une » : une chaîne naît sans thématique,
-- l'INSERT de la chaîne précède forcément celui de sa thématique. Le « au
-- moins une » est une règle de PUBLICATION, tenue par l'API (400 explicite),
-- là où le trigger tient ce qu'aucun chemin d'écriture ne doit pouvoir forcer.
--
-- ── Pourquoi les deux tables de support ────────────────────────────────────
-- `support_thematique` et `support_territoire` sont polymorphes (type_support,
-- support_id) et servent indifféremment la télé et la radio. Poser la colonne
-- sur la seule `chaine_tv` obligerait le trigger — et chaque écriture — à
-- brancher sur le type de support pour savoir si la colonne existe. La station
-- radio la reçoit donc aussi, à FALSE : la contrainte ne coûte rien tant que
-- personne ne la lève, et la symétrie du patron est préservée.
--
-- Prérequis : 09r (couverture_continentale, support_thematique), 09u (parent_id
-- du groupe des lignes éditoriales).
--
-- Migration IDEMPOTENTE : ADD COLUMN IF NOT EXISTS, DO $$ … $$ pour les CHECK
-- (PostgreSQL n'a pas d'ADD CONSTRAINT IF NOT EXISTS), CREATE OR REPLACE
-- FUNCTION, DROP TRIGGER IF EXISTS puis CREATE.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. Nature du support
-- ════════════════════════════════════════════════════════════════════════════

ALTER TABLE media_content.chaine_tv
    ADD COLUMN IF NOT EXISTS est_thematique BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS est_thematique BOOLEAN NOT NULL DEFAULT FALSE;

COMMENT ON COLUMN media_content.chaine_tv.est_thematique IS
    'Chaîne thématique : une seule thématique de grille, et tous les territoires
     d''office. EXCLUT toute couverture territoriale — voir le CHECK
     ck_chaine_tv_thematique_continentale et le trigger
     trg_support_thematique_unicite.';
COMMENT ON COLUMN media_content.station_radio.est_thematique IS
    'Pendant radio de chaine_tv.est_thematique. Posée pour la symétrie du patron
     polymorphe des tables support_* ; aucun écran ne la lève à ce jour.';


-- ── 1.1 Invariant 1 : thématique ⇒ continentale ────────────────────────────
-- Les supports existants sont tous à FALSE : la contrainte est satisfaite dès
-- sa création, aucune reprise de données n'est nécessaire.

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
         WHERE conname = 'ck_chaine_tv_thematique_continentale'
    ) THEN
        ALTER TABLE media_content.chaine_tv
            ADD CONSTRAINT ck_chaine_tv_thematique_continentale
            CHECK (NOT est_thematique OR couverture_continentale);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint
         WHERE conname = 'ck_station_radio_thematique_continentale'
    ) THEN
        ALTER TABLE media_content.station_radio
            ADD CONSTRAINT ck_station_radio_thematique_continentale
            CHECK (NOT est_thematique OR couverture_continentale);
    END IF;
END $$;


-- ════════════════════════════════════════════════════════════════════════════
-- 2. Invariant 2 : une seule thématique de grille par support thématique
-- ════════════════════════════════════════════════════════════════════════════
-- Le trigger ne se déclenche qu'à l'écriture d'une thématique, et sort
-- immédiatement pour l'écrasante majorité des supports, qui ne sont pas
-- thématiques.

CREATE OR REPLACE FUNCTION media_content.verifier_thematique_unique()
RETURNS TRIGGER AS $$
DECLARE
    thematique BOOLEAN;
    de_grille  BOOLEAN;
    deja       INTEGER;
BEGIN
    EXECUTE format(
        'SELECT est_thematique FROM media_content.%I WHERE id = $1',
        CASE NEW.type_support WHEN 'chaine_tv' THEN 'chaine_tv' ELSE 'station_radio' END)
    INTO thematique
    USING NEW.support_id;

    IF thematique IS NOT TRUE THEN
        RETURN NEW;
    END IF;

    -- Une ligne éditoriale (parent_id renseigné) n'entre pas dans le compte :
    -- ce n'est pas de cette « thématique » que parle la règle.
    SELECT parent_id IS NULL INTO de_grille
      FROM shared.categorie WHERE id = NEW.categorie_id;

    IF de_grille IS NOT TRUE THEN
        RETURN NEW;
    END IF;

    SELECT COUNT(*) INTO deja
      FROM media_content.support_thematique st
      JOIN shared.categorie c ON c.id = st.categorie_id
     WHERE st.type_support = NEW.type_support
       AND st.support_id   = NEW.support_id
       AND st.categorie_id <> NEW.categorie_id
       AND c.parent_id IS NULL;

    IF deja > 0 THEN
        RAISE EXCEPTION
            'Support thématique : une seule thématique de grille est admise (déjà % déclarée(s))', deja;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_support_thematique_unicite
    ON media_content.support_thematique;
CREATE TRIGGER trg_support_thematique_unicite
    BEFORE INSERT ON media_content.support_thematique
    FOR EACH ROW EXECUTE FUNCTION media_content.verifier_thematique_unique();


-- ════════════════════════════════════════════════════════════════════════════
-- 3. Fin de la double source de vérité : `chaine_tv.pays_id`
-- ════════════════════════════════════════════════════════════════════════════
-- La chaîne portait ENCORE un pays unique, hérité d'avant 09r, pendant que sa
-- couverture réelle vivait dans `support_territoire`. Les deux divergeaient
-- nécessairement : le filtre « Territoire » de /medias/tele interrogeait la
-- colonne, l'écran d'édition écrivait la table. Une chaîne déclarant trois
-- territoires n'était trouvable que par un seul — l'exigence même à laquelle
-- cette migration répond.
--
-- La colonne DISPARAÎT donc, plutôt que d'être « tenue à jour » : deux sources
-- de vérité pour une même information finissent toujours par se contredire, et
-- celle-ci n'apportait rien que `support_territoire` ne dise mieux.
--
-- La station radio CONSERVE la sienne : elle émet depuis un lieu, qui n'est pas
-- sa zone de diffusion. Les deux informations y sont distinctes.

-- ── 3.1 Reprise, pour les chaînes créées après 09r ──────────────────────────
-- Les chaînes continentales sont écartées : le trigger d'exclusivité (09r)
-- refuserait le territoire, et il aurait raison.

INSERT INTO media_content.support_territoire (type_support, support_id, pays_id)
SELECT 'chaine_tv', ct.id, ct.pays_id
  FROM media_content.chaine_tv ct
 WHERE ct.pays_id IS NOT NULL
   AND ct.deleted_at IS NULL
   AND ct.couverture_continentale = FALSE
ON CONFLICT DO NOTHING;

-- ── 3.2 Suppression ────────────────────────────────────────────────────────

ALTER TABLE media_content.chaine_tv DROP COLUMN IF EXISTS pays_id;
