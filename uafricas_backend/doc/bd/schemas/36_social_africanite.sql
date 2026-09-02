-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Schema : social — Africanité (publications éphémères)
-- ════════════════════════════════════════════════════════════════════════════
-- Dépend de : 29_social.sql (social.amitie), 04_iam.sql (iam.utilisateur)
-- Spec : specs/012-africanite-ephemere/spec.md
--
-- Première notion de contenu ÉPHÉMÈRE de la plateforme. Tout le reste — post
-- Codimoi, avis de recherche, contribution citoyenne, partage — reste
-- consultable indéfiniment. Une africanité est faite pour disparaître, et c'est
-- ce renversement qui commande le schéma.
--
-- TROIS ÉTATS, et non deux (décision Q2 de la spec) :
--   active   → `expire_at > NOW()`            : servie
--   échue    → `expire_at <= NOW()`           : invisible de tous, média encore là
--   détruite → `media_detruit_at IS NOT NULL` : fichier effacé du disque
--
-- L'expiration ne déclenche RIEN : elle se constate à la lecture (`expire_at >
-- NOW()` dans chaque requête). C'est le patron déjà retenu pour les créneaux de
-- programmation Afrolang et la rotation des épisodes médias — aucune tâche de
-- fond à surveiller, et une base restaurée ne « rattrape » pas des expirations.
--
-- Migration idempotente.


CREATE TABLE IF NOT EXISTS social.africanite (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    auteur_id        UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,

    -- Les trois formes de la décision Q3. Un CHECK plutôt qu'un enum : la spec
    -- envisage d'en ajouter, et un enum PostgreSQL se modifie moins aisément.
    forme            VARCHAR(10) NOT NULL,

    -- Formes `image` et `video` : chemin relatif servi par actix-files.
    media_url        TEXT,
    -- Forme `texte` : le texte EST le média, posé sur sa couleur.
    texte            TEXT,
    couleur_fond     VARCHAR(9),

    legende          TEXT,

    -- Échéance fixée à la création. NOT NULL : une africanité sans échéance
    -- serait une publication ordinaire déguisée.
    expire_at        TIMESTAMPTZ NOT NULL,
    -- Horodatage de la destruction du fichier. Distingue « échue » de
    -- « détruite » : sans cette colonne, on ne saurait pas quels médias restent
    -- à purger, et la rétention dériverait en silence.
    media_detruit_at TIMESTAMPTZ,

    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at       TIMESTAMPTZ
);

-- Cohérence forme / colonnes. Rendre l'incohérence impossible EN SQL plutôt que
-- de la contrôler dans le handler : une africanité `texte` sans texte, ou
-- `image` sans fichier, n'aurait rien à afficher.
ALTER TABLE social.africanite DROP CONSTRAINT IF EXISTS ck_africanite_forme;
ALTER TABLE social.africanite ADD CONSTRAINT ck_africanite_forme
    CHECK (forme IN ('image', 'video', 'texte'));

ALTER TABLE social.africanite DROP CONSTRAINT IF EXISTS ck_africanite_contenu;
ALTER TABLE social.africanite ADD CONSTRAINT ck_africanite_contenu CHECK (
    (forme IN ('image', 'video') AND media_url IS NOT NULL AND texte IS NULL)
 OR (forme = 'texte' AND media_url IS NULL AND texte IS NOT NULL AND btrim(texte) <> '')
);

-- Un `NOT NULL` laisserait passer une chaîne d'espaces ; le `btrim` ci-dessus
-- ferme ce cas. Ici on borne les longueurs pour que tout tienne en un écran.
ALTER TABLE social.africanite DROP CONSTRAINT IF EXISTS ck_africanite_longueurs;
ALTER TABLE social.africanite ADD CONSTRAINT ck_africanite_longueurs CHECK (
    (texte IS NULL OR char_length(texte) <= 280)
AND (legende IS NULL OR char_length(legende) <= 200)
);

-- Index de lecture : toutes les requêtes filtrent sur l'échéance et l'auteur.
CREATE INDEX IF NOT EXISTS idx_africanite_actives
    ON social.africanite (auteur_id, created_at)
    WHERE deleted_at IS NULL;

-- Sert la purge : retrouver les médias échus qu'il reste à détruire.
CREATE INDEX IF NOT EXISTS idx_africanite_a_purger
    ON social.africanite (expire_at)
    WHERE media_detruit_at IS NULL AND media_url IS NOT NULL;


-- ────────────────────────────────────────────────────────────────────────────
-- VUES
--
-- Sert DEUX usages distincts qu'il ne faut pas confondre : l'état de l'anneau
-- côté lecteur (« ai-je tout vu de cette personne ? ») et le décompte côté
-- auteur (« combien de personnes m'ont vue ? »).
--
-- Clé primaire composite : une vue est unique par couple lecteur/africanité.
-- Regarder deux fois ne compte qu'une fois — c'est la règle FR-011.
-- ────────────────────────────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS social.africanite_vue (
    africanite_id  UUID NOT NULL REFERENCES social.africanite(id) ON DELETE CASCADE,
    utilisateur_id UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    vue_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (africanite_id, utilisateur_id)
);

CREATE INDEX IF NOT EXISTS idx_africanite_vue_par_lecteur
    ON social.africanite_vue (utilisateur_id);
