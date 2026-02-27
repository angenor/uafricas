-- ============================================================
-- RETROUVE AMIS — Schema PostgreSQL
-- Fichier: 16_retrouve_amis.sql
-- Schema: retrouve_amis (nouveau bounded-context)
-- ============================================================

-- Le schema sera ajouté dans doc/bd/schemas/16_retrouve_amis.sql
-- et référencé dans doc/bd/schema.sql via \ir schemas/16_retrouve_amis.sql

CREATE SCHEMA IF NOT EXISTS retrouve_amis;

-- ============================================================
-- ENUMS
-- ============================================================

-- État d'un avis de recherche
CREATE TYPE retrouve_amis.etat_avis AS ENUM (
    'brouillon',        -- En cours de rédaction
    'actif',            -- Publié et en recherche active
    'en_pause',         -- Temporairement désactivé par l'utilisateur
    'resolu',           -- Personne retrouvée
    'expire',           -- Expiré après 12 mois sans activité
    'modere'            -- Masqué par un modérateur
);

-- État d'une correspondance
CREATE TYPE retrouve_amis.etat_correspondance AS ENUM (
    'potentielle',      -- Détectée par l'algorithme, en attente de review
    'confirmee_a',      -- Confirmée par le chercheur (partie A)
    'confirmee_b',      -- Confirmée par la personne trouvée (partie B)
    'validee',          -- Double opt-in : les deux ont confirmé
    'rejetee',          -- L'une des parties a rejeté
    'expiree'           -- Pas de réponse dans le délai
);

-- Type de critère de recherche
CREATE TYPE retrouve_amis.type_critere AS ENUM (
    'nom',              -- Nom de famille
    'prenom',           -- Prénom
    'surnom',           -- Surnom / pseudonyme
    'ecole',            -- Établissement scolaire
    'universite',       -- Université
    'entreprise',       -- Lieu de travail
    'quartier',         -- Quartier
    'ville',            -- Ville
    'pays',             -- Pays
    'annee_debut',      -- Année de début de connaissance
    'annee_fin',        -- Année de fin de connaissance
    'tranche_age',      -- Tranche d'âge approximative
    'description',      -- Description physique ou autre
    'anecdote',         -- Anecdote partagée (aide à la confirmation)
    'autre'             -- Autre information
);

-- Type de contact partagé
CREATE TYPE retrouve_amis.type_contact AS ENUM (
    'telephone',        -- Numéro de téléphone personnel
    'email',            -- Adresse email
    'telephone_proche', -- Numéro d'un proche/intermédiaire
    'autre'             -- Autre moyen de contact
);

-- Motif de signalement
CREATE TYPE retrouve_amis.motif_signalement AS ENUM (
    'usurpation',       -- Usurpation d'identité
    'harcelement',      -- Harcèlement
    'faux_avis',        -- Avis manifestement faux
    'contenu_inapproprie', -- Contenu inapproprié
    'arnaque',          -- Tentative d'arnaque
    'autre'             -- Autre motif
);

-- ============================================================
-- TABLE : preference_trouvabilite
-- Préférences de l'utilisateur pour être retrouvable
-- ============================================================
CREATE TABLE retrouve_amis.preference_trouvabilite (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id UUID NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,

    -- L'utilisateur accepte-t-il d'être trouvé ?
    est_trouvable BOOLEAN NOT NULL DEFAULT false,

    -- Informations optionnelles pour faciliter le matching
    -- (renseignées volontairement par l'utilisateur)
    anciens_noms TEXT[],               -- Noms/prénoms précédents
    anciennes_villes TEXT[],           -- Villes où il/elle a vécu
    anciennes_ecoles TEXT[],           -- Écoles fréquentées
    anciennes_entreprises TEXT[],      -- Entreprises
    periode_debut INT,                 -- Année la plus ancienne (ex: 1995)
    periode_fin INT,                   -- Année la plus récente (ex: 2010)

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_preference_utilisateur UNIQUE (utilisateur_id)
);

-- ============================================================
-- TABLE : avis_recherche
-- Avis de recherche déposé par un utilisateur
-- ============================================================
CREATE TABLE retrouve_amis.avis_recherche (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Auteur de l'avis (toujours un utilisateur connecté)
    auteur_id UUID NOT NULL REFERENCES iam.utilisateur(id),

    -- Titre court affiché (ex: "Je cherche mon ami d'enfance de Douala")
    titre VARCHAR(300) NOT NULL,

    -- Description libre (contexte, souvenirs partagés...)
    description TEXT,

    -- Relation avec la personne cherchée
    relation VARCHAR(100),  -- "ami d'enfance", "camarade de classe", "voisin", "collègue"

    -- Anonymat : le chercheur veut-il rester anonyme ?
    est_anonyme BOOLEAN NOT NULL DEFAULT true,

    -- État de l'avis
    etat retrouve_amis.etat_avis NOT NULL DEFAULT 'actif',

    -- Compteur de vues (pour statistiques)
    nombre_vues INT NOT NULL DEFAULT 0,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,

    -- Expiration automatique (12 mois après dernière activité)
    derniere_activite TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expire_le TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '12 months')
);

CREATE INDEX idx_avis_auteur ON retrouve_amis.avis_recherche(auteur_id)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_avis_etat ON retrouve_amis.avis_recherche(etat)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_avis_expire ON retrouve_amis.avis_recherche(expire_le)
    WHERE deleted_at IS NULL AND etat = 'actif';

-- ============================================================
-- TABLE : critere_recherche
-- Critères descriptifs associés à un avis de recherche
-- Chaque avis a N critères typés (nom, ville, école, etc.)
-- ============================================================
CREATE TABLE retrouve_amis.critere_recherche (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    avis_id UUID NOT NULL REFERENCES retrouve_amis.avis_recherche(id) ON DELETE CASCADE,

    -- Type du critère
    type_critere retrouve_amis.type_critere NOT NULL,

    -- Valeur brute saisie par l'utilisateur
    valeur TEXT NOT NULL,

    -- Valeur normalisée (minuscule, sans accents, sans espaces superflus)
    -- Calculée automatiquement pour faciliter le matching
    valeur_normalisee TEXT NOT NULL,

    -- Poids/importance du critère (1 = faible, 5 = très important)
    -- Permet à l'utilisateur de prioriser certains critères
    poids INT NOT NULL DEFAULT 3 CHECK (poids BETWEEN 1 AND 5),

    -- Référence vers shared.pays si type_critere = 'pays'
    pays_id UUID REFERENCES shared.pays(id),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_critere_avis ON retrouve_amis.critere_recherche(avis_id);
CREATE INDEX idx_critere_type_valeur ON retrouve_amis.critere_recherche(type_critere, valeur_normalisee);

-- Index full-text pour recherche fuzzy
ALTER TABLE retrouve_amis.critere_recherche
    ADD COLUMN tsv TSVECTOR
    GENERATED ALWAYS AS (to_tsvector('french', valeur)) STORED;
CREATE INDEX idx_critere_fts ON retrouve_amis.critere_recherche USING GIN (tsv);

-- ============================================================
-- TABLE : correspondance
-- Correspondances détectées entre deux avis ou entre un avis et un profil
-- ============================================================
CREATE TABLE retrouve_amis.correspondance (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Avis de recherche du chercheur (partie A)
    avis_a_id UUID NOT NULL REFERENCES retrouve_amis.avis_recherche(id),

    -- Soit un autre avis (recherche croisée), soit NULL si match avec un profil
    avis_b_id UUID REFERENCES retrouve_amis.avis_recherche(id),

    -- Soit un utilisateur inscrit (match avec profil trouvable), soit NULL
    utilisateur_b_id UUID REFERENCES iam.utilisateur(id),

    -- Score de correspondance (0-100)
    score INT NOT NULL CHECK (score BETWEEN 0 AND 100),

    -- Détail du scoring (quels critères ont matché et avec quel poids)
    detail_score JSONB NOT NULL DEFAULT '{}',
    -- Exemple: {
    --   "criteres": [
    --     {"type": "ville", "valeur_a": "Douala", "valeur_b": "Douala", "score": 100},
    --     {"type": "ecole", "valeur_a": "Lycée Joss", "valeur_b": "Lycee Joss", "score": 95},
    --     {"type": "nom", "valeur_a": "Kamga", "valeur_b": "Kamga", "score": 100}
    --   ],
    --   "score_global": 85
    -- }

    -- État du processus de confirmation
    etat retrouve_amis.etat_correspondance NOT NULL DEFAULT 'potentielle',

    -- Dates de confirmation par chaque partie
    confirme_a_le TIMESTAMPTZ,       -- Date de confirmation par A
    confirme_b_le TIMESTAMPTZ,       -- Date de confirmation par B
    rejete_par UUID REFERENCES iam.utilisateur(id),  -- Qui a rejeté
    rejete_le TIMESTAMPTZ,

    -- Dates
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expire_le TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '30 days'),

    -- Un avis A ne peut matcher qu'une fois avec un avis B donné
    CONSTRAINT uq_correspondance_avis UNIQUE (avis_a_id, avis_b_id),
    -- Un avis A ne peut matcher qu'une fois avec un utilisateur B donné
    CONSTRAINT uq_correspondance_profil UNIQUE (avis_a_id, utilisateur_b_id),
    -- Au moins un des deux doit être renseigné
    CONSTRAINT chk_correspondance_cible CHECK (
        avis_b_id IS NOT NULL OR utilisateur_b_id IS NOT NULL
    )
);

CREATE INDEX idx_correspondance_avis_a ON retrouve_amis.correspondance(avis_a_id);
CREATE INDEX idx_correspondance_avis_b ON retrouve_amis.correspondance(avis_b_id);
CREATE INDEX idx_correspondance_utilisateur_b ON retrouve_amis.correspondance(utilisateur_b_id);
CREATE INDEX idx_correspondance_etat ON retrouve_amis.correspondance(etat);
CREATE INDEX idx_correspondance_score ON retrouve_amis.correspondance(score DESC);

-- ============================================================
-- TABLE : message_correspondance
-- Messages échangés dans le cadre d'une correspondance validée
-- ============================================================
CREATE TABLE retrouve_amis.message_correspondance (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    correspondance_id UUID NOT NULL REFERENCES retrouve_amis.correspondance(id) ON DELETE CASCADE,

    -- Auteur du message
    auteur_id UUID NOT NULL REFERENCES iam.utilisateur(id),

    -- Contenu du message (texte simple, pas de HTML)
    contenu TEXT NOT NULL CHECK (char_length(contenu) <= 2000),

    -- Si le message contient un partage de contact
    type_contact retrouve_amis.type_contact,
    -- Valeur du contact (chiffrée côté application avant stockage)
    valeur_contact_chiffree TEXT,
    -- Nom du contact (si c'est un proche)
    nom_contact VARCHAR(200),

    -- Lu par le destinataire ?
    lu BOOLEAN NOT NULL DEFAULT false,
    lu_le TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_message_correspondance ON retrouve_amis.message_correspondance(correspondance_id, created_at)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_message_auteur ON retrouve_amis.message_correspondance(auteur_id)
    WHERE deleted_at IS NULL;
CREATE INDEX idx_message_non_lu ON retrouve_amis.message_correspondance(correspondance_id)
    WHERE lu = false AND deleted_at IS NULL;

-- ============================================================
-- TABLE : signalement
-- Signalements d'abus sur un avis ou une correspondance
-- ============================================================
CREATE TABLE retrouve_amis.signalement (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Qui signale
    signaleur_id UUID NOT NULL REFERENCES iam.utilisateur(id),

    -- Ce qui est signalé (un avis OU une correspondance)
    avis_id UUID REFERENCES retrouve_amis.avis_recherche(id),
    correspondance_id UUID REFERENCES retrouve_amis.correspondance(id),

    -- Motif
    motif retrouve_amis.motif_signalement NOT NULL,
    description TEXT,

    -- Traitement par l'admin
    traite BOOLEAN NOT NULL DEFAULT false,
    traite_par UUID REFERENCES iam.utilisateur(id),
    traite_le TIMESTAMPTZ,
    decision VARCHAR(500),  -- Explication de la décision

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Au moins un des deux doit être renseigné
    CONSTRAINT chk_signalement_cible CHECK (
        avis_id IS NOT NULL OR correspondance_id IS NOT NULL
    )
);

CREATE INDEX idx_signalement_avis ON retrouve_amis.signalement(avis_id);
CREATE INDEX idx_signalement_non_traite ON retrouve_amis.signalement(traite)
    WHERE traite = false;

-- ============================================================
-- TABLE : journal_matching
-- Historique des exécutions du matching (pour debug et stats)
-- ============================================================
CREATE TABLE retrouve_amis.journal_matching (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Avis pour lequel le matching a été exécuté
    avis_id UUID NOT NULL REFERENCES retrouve_amis.avis_recherche(id),

    -- Résultats
    nombre_candidats INT NOT NULL DEFAULT 0,
    nombre_correspondances_creees INT NOT NULL DEFAULT 0,
    duree_ms INT,  -- Durée d'exécution en ms

    -- Détail (pour debugging)
    detail JSONB,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_journal_avis ON retrouve_amis.journal_matching(avis_id);

-- ============================================================
-- FONCTIONS
-- ============================================================

-- Fonction de normalisation de texte (minuscule, sans accents, trim)
CREATE OR REPLACE FUNCTION retrouve_amis.normaliser_texte(texte TEXT)
RETURNS TEXT AS $$
BEGIN
    RETURN lower(
        trim(
            translate(
                texte,
                'àâäéèêëïîôùûüçÀÂÄÉÈÊËÏÎÔÙÛÜÇ',
                'aaaeeeeiioouucAAAEEEEIIOOUUC'
            )
        )
    );
END;
$$ LANGUAGE plpgsql IMMUTABLE;

-- Trigger pour normaliser automatiquement les critères
CREATE OR REPLACE FUNCTION retrouve_amis.trigger_normaliser_critere()
RETURNS TRIGGER AS $$
BEGIN
    NEW.valeur_normalisee := retrouve_amis.normaliser_texte(NEW.valeur);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_normaliser_critere
    BEFORE INSERT OR UPDATE OF valeur ON retrouve_amis.critere_recherche
    FOR EACH ROW
    EXECUTE FUNCTION retrouve_amis.trigger_normaliser_critere();

-- Trigger pour mettre à jour updated_at
CREATE TRIGGER trg_avis_updated_at
    BEFORE UPDATE ON retrouve_amis.avis_recherche
    FOR EACH ROW
    EXECUTE FUNCTION shared.maj_updated_at();  -- Réutilise la fonction existante

CREATE TRIGGER trg_correspondance_updated_at
    BEFORE UPDATE ON retrouve_amis.correspondance
    FOR EACH ROW
    EXECUTE FUNCTION shared.maj_updated_at();

CREATE TRIGGER trg_preference_updated_at
    BEFORE UPDATE ON retrouve_amis.preference_trouvabilite
    FOR EACH ROW
    EXECUTE FUNCTION shared.maj_updated_at();

-- ============================================================
-- VUES
-- ============================================================

-- Vue des avis actifs avec nombre de critères et correspondances
CREATE OR REPLACE VIEW retrouve_amis.v_avis_actifs AS
SELECT
    a.id,
    a.auteur_id,
    a.titre,
    a.relation,
    a.est_anonyme,
    a.etat,
    a.nombre_vues,
    a.created_at,
    a.derniere_activite,
    COUNT(DISTINCT c.id) AS nombre_criteres,
    COUNT(DISTINCT co.id) AS nombre_correspondances,
    COUNT(DISTINCT co.id) FILTER (WHERE co.etat = 'validee') AS correspondances_validees
FROM retrouve_amis.avis_recherche a
LEFT JOIN retrouve_amis.critere_recherche c ON c.avis_id = a.id
LEFT JOIN retrouve_amis.correspondance co ON co.avis_a_id = a.id
WHERE a.deleted_at IS NULL
  AND a.etat = 'actif'
GROUP BY a.id;

-- ============================================================
-- COMMENTAIRES
-- ============================================================
COMMENT ON SCHEMA retrouve_amis IS 'Fonctionnalité de recherche d''amis perdus de vue par recoupement d''informations';
COMMENT ON TABLE retrouve_amis.avis_recherche IS 'Avis de recherche déposés par les utilisateurs';
COMMENT ON TABLE retrouve_amis.critere_recherche IS 'Critères typés associés à un avis (nom, ville, école, etc.)';
COMMENT ON TABLE retrouve_amis.correspondance IS 'Correspondances détectées entre avis ou entre avis et profils';
COMMENT ON TABLE retrouve_amis.message_correspondance IS 'Messages échangés entre correspondants après validation';
COMMENT ON TABLE retrouve_amis.signalement IS 'Signalements d''abus';
COMMENT ON TABLE retrouve_amis.preference_trouvabilite IS 'Préférences de trouvabilité des utilisateurs inscrits';
COMMENT ON TABLE retrouve_amis.journal_matching IS 'Journal technique des exécutions de l''algorithme de matching';
