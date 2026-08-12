-- ============================================================================
-- 09t — media_content : équipes éditoriales et périodicité à quatre valeurs
-- ----------------------------------------------------------------------------
-- Deux chantiers, une migration.
--
--   • membre_equipe — la personne qui fait une chaîne, une station ou un
--     programme n'était nulle part. Les deux seules traces étaient
--     `emission_*.info_animateur` et `info_producteur`, deux TEXT scalaires
--     sans multiplicité, sans rang et sans champs séparés. La table les
--     remplace comme SOURCE D'AFFICHAGE ; les colonnes restent en base, aucune
--     saisie n'est perdue.
--
--   • cadence — la périodicité passe de 3 à 4 valeurs. Seule `mensuelle`
--     s'ajoute : les trois clés existantes sont CONSERVÉES TELLES QUELLES, et
--     cette migration ne comporte AUCUN `UPDATE` de données (FR-043). Seuls les
--     libellés d'affichage changent, côté frontend.
--
-- ── Pourquoi une table polymorphe et non deux ───────────────────────────────
-- `type_porteur` porte QUATRE valeurs : deux supports (chaine_tv,
-- station_radio) et deux programmes (emission_tele, emission_radio). Les
-- colonnes et toutes les règles sont identiques aux deux niveaux ; deux tables
-- dupliqueraient requêtes, DTO, handlers, validations et audit. Le patron est
-- déjà employé trois fois dans ce schéma : support_thematique et
-- support_territoire par (type_support, support_id) (09r), et les quatre tables
-- d'interactions par (type_media, media_id) (09k).
--
-- ── Pourquoi un CHECK et non un ENUM ────────────────────────────────────────
-- Aucun enum existant ne décrit ce jeu précis. `type_support_media` (09m) n'a
-- que 2 valeurs — les supports seuls, par construction. Le CHECK des
-- interactions (09k, porté à 6 par 09q) inclut les épisodes, qui n'ont pas
-- d'équipe. Créer un cinquième enum pour 4 valeurs coûterait, à chaque
-- extension, un `ALTER TYPE … ADD VALUE` qui ne peut cohabiter avec son usage
-- dans la même transaction — la contrainte qui a imposé le préambule de 09q. Un
-- CHECK s'étend par un DROP/ADD CONSTRAINT ordinaire, comme le fait d'ailleurs
-- la seconde moitié de ce fichier.
--
-- ── Le prix du polymorphisme ────────────────────────────────────────────────
-- `porteur_id` n'a PAS de clé étrangère : il désigne quatre tables. La
-- suppression d'un porteur ne peut donc pas cascader — le nettoyage est
-- EXPLICITE dans les handlers, en quatre endroits (FR-019). L'oublier ne casse
-- rien de visible : les équipes orphelines restent simplement dans le
-- référentiel de suggestions de fonctions.
--
-- Prérequis : 09 (chaine_tv, station_radio), 09q (emission_tele,
-- emission_radio), 01/02 (iam.utilisateur).
--
-- Migration IDEMPOTENTE : CREATE … IF NOT EXISTS, DROP CONSTRAINT IF EXISTS
-- puis ADD.
-- ============================================================================


-- ════════════════════════════════════════════════════════════════════════════
-- 1. membre_equipe — l'équipe éditoriale d'un support ou d'un programme
-- ════════════════════════════════════════════════════════════════════════════

CREATE TABLE IF NOT EXISTS media_content.membre_equipe (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_porteur   VARCHAR(20)  NOT NULL,
    porteur_id     UUID         NOT NULL,      -- [xref] chaine_tv | station_radio | emission_tele | emission_radio
    nom            VARCHAR(150) NOT NULL,
    prenom         VARCHAR(150),
    fonction       VARCHAR(120) NOT NULL,
    territoire     VARCHAR(150),
    contact        VARCHAR(250),
    utilisateur_id UUID         REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    ordre          INT          NOT NULL DEFAULT 0,
    cree_par       UUID         NOT NULL REFERENCES iam.utilisateur(id) ON DELETE RESTRICT,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at     TIMESTAMPTZ
);


-- ── 1.1 Contraintes ────────────────────────────────────────────────────────
-- Aucune unicité sur (porteur_id, nom, prenom) : le cas limite « homonymes dans
-- une même équipe » l'exige explicitement — deux personnes de même nom se
-- distinguent par leur fonction. Aucune unicité sur (porteur_id, utilisateur_id)
-- non plus : rien n'interdit qu'un compte figure deux fois sous deux fonctions.

ALTER TABLE media_content.membre_equipe
    DROP CONSTRAINT IF EXISTS ck_membre_equipe_type_porteur;
ALTER TABLE media_content.membre_equipe
    ADD CONSTRAINT ck_membre_equipe_type_porteur
        CHECK (type_porteur IN ('chaine_tv', 'station_radio', 'emission_tele', 'emission_radio'));

-- FR-012 : nom et fonction obligatoires. « Obligatoire » veut dire NON VIDE et
-- non « non NULL » — une chaîne d'espaces passerait le NOT NULL sans broncher et
-- produirait une fiche muette à l'écran.
ALTER TABLE media_content.membre_equipe
    DROP CONSTRAINT IF EXISTS ck_membre_equipe_nom_non_vide;
ALTER TABLE media_content.membre_equipe
    ADD CONSTRAINT ck_membre_equipe_nom_non_vide      CHECK (btrim(nom) <> '');

ALTER TABLE media_content.membre_equipe
    DROP CONSTRAINT IF EXISTS ck_membre_equipe_fonction_non_vide;
ALTER TABLE media_content.membre_equipe
    ADD CONSTRAINT ck_membre_equipe_fonction_non_vide CHECK (btrim(fonction) <> '');


-- ── 1.2 Index ──────────────────────────────────────────────────────────────

-- Lecture groupée (equipes_par_porteurs) : le seul accès chaud. L'ordre est
-- dans l'index, la requête de lecture n'a donc rien à trier.
CREATE INDEX IF NOT EXISTS idx_membre_equipe_porteur
    ON media_content.membre_equipe (type_porteur, porteur_id, ordre)
    WHERE deleted_at IS NULL;

-- Route de suggestions des fonctions (FR-015), globale à toute la plateforme.
CREATE INDEX IF NOT EXISTS idx_membre_equipe_fonction
    ON media_content.membre_equipe (fonction)
    WHERE deleted_at IS NULL;


-- ── 1.3 Commentaires ───────────────────────────────────────────────────────

COMMENT ON TABLE media_content.membre_equipe IS
    'Équipe éditoriale d''un support média (chaîne, station) ou d''un programme
     (émission télé, émission radio). Remplace info_animateur / info_producteur
     comme source d''affichage public ; ces colonnes restent en base.';

COMMENT ON COLUMN media_content.membre_equipe.type_porteur IS
    'Support (chaine_tv, station_radio) ou programme (emission_tele,
     emission_radio). Volontairement PAS l''enum type_support_media (09m), qui ne
     couvre que les supports, ni le CHECK des interactions (09k), qui inclut les
     épisodes.';

COMMENT ON COLUMN media_content.membre_equipe.porteur_id IS
    'Référence LOGIQUE vers l''une des quatre tables désignées par type_porteur.
     Aucune FK possible : le nettoyage à la suppression du porteur est explicite
     dans les handlers (FR-019).';

COMMENT ON COLUMN media_content.membre_equipe.utilisateur_id IS
    'Rattachement FACULTATIF à un compte (FR-013). ON DELETE SET NULL : la fiche
     survit à la fermeture du compte, sans lien mort. Ne confère AUCUN droit —
     les droits vivent dans support_detenteur (09m).';

COMMENT ON COLUMN media_content.membre_equipe.contact IS
    'Coordonnée professionnelle SAISIE par le gestionnaire. N''est JAMAIS dérivée
     de iam.utilisateur.email, même quand utilisateur_id est renseigné.';

COMMENT ON COLUMN media_content.membre_equipe.fonction IS
    'Texte libre (FR-015). Normalisée à l''écriture (btrim + espaces internes
     réduits) ; les suggestions sont dédupliquées à la lecture, casse ignorée.';

COMMENT ON COLUMN media_content.membre_equipe.ordre IS
    'Rang d''affichage public (FR-016), réécrit à chaque PUT depuis l''index reçu.';


-- ════════════════════════════════════════════════════════════════════════════
-- 2. Périodicité — le CHECK de cadence passe à quatre valeurs
-- ════════════════════════════════════════════════════════════════════════════
-- AUCUN `UPDATE` de données (FR-043) : les trois clés existantes sont
-- conservées, `mensuelle` est la seule valeur neuve, et DEFAULT 'ponctuelle'
-- reste en place (FR-042). Les libellés affichés changent côté frontend :
--   ponctuelle → « Non périodique » · quotidienne → « Journalier »
--   hebdomadaire → « Hebdomadaire » · mensuelle → « Mensuel »
--
-- Conséquence côté code, à ne pas manquer : `mes_alertes_cadence` calculait
-- « 24 h si quotidienne, sinon 24×7 ». Tel quel, un programme mensuel serait
-- signalé en retard dès le 8ᵉ jour. La période devient une fonction de la
-- cadence (models/media_emission.rs::periode_heures_cadence).

ALTER TABLE media_content.emission_tele  DROP CONSTRAINT IF EXISTS ck_emission_tele_cadence;
ALTER TABLE media_content.emission_tele
    ADD CONSTRAINT ck_emission_tele_cadence
        CHECK (cadence IN ('quotidienne', 'hebdomadaire', 'mensuelle', 'ponctuelle'));

ALTER TABLE media_content.emission_radio DROP CONSTRAINT IF EXISTS ck_emission_radio_cadence;
ALTER TABLE media_content.emission_radio
    ADD CONSTRAINT ck_emission_radio_cadence
        CHECK (cadence IN ('quotidienne', 'hebdomadaire', 'mensuelle', 'ponctuelle'));

COMMENT ON COLUMN media_content.emission_tele.cadence IS
    'Périodicité déclarée : quotidienne | hebdomadaire | mensuelle | ponctuelle.
     Socle des alertes de cadence. ''ponctuelle'' (« Non périodique ») est le
     défaut et ne déclenche aucune alerte.';

COMMENT ON COLUMN media_content.emission_radio.cadence IS
    'Périodicité déclarée : quotidienne | hebdomadaire | mensuelle | ponctuelle.
     Socle des alertes de cadence. ''ponctuelle'' (« Non périodique ») est le
     défaut et ne déclenche aucune alerte.';
