-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Engagement : cadeaux virtuels, transactions & cagnottes
-- ════════════════════════════════════════════════════════════════════════════
-- Feature 008-recadrage-engagement-cadeaux — Phase 1 (socle SQL).
--
-- 1. Enums `mode_cadeau` et `etat_paiement`.
-- 2. `engagement.cadeau` — catalogue paramétrable (5 cadeaux seedés).
-- 3. `engagement.parametre_monetisation` — SINGLETON structurel.
-- 4. `engagement.transaction_cadeau` — journal comptable immuable.
-- 5. `engagement.cagnotte` — solde de soutien du bénéficiaire.
--
-- Parti pris central : les invariants métier sont portés par des CHECK, pas par
-- du code applicatif. La répartition exacte, l'auto-cadeau, la cohérence de la
-- cible et le mode « points » deviennent IMPOSSIBLES À VIOLER en SQL — y compris
-- par une écriture manuelle en base ou une requête mal écrite.
--
-- Dérogation documentée : pas de `deleted_at` sur `transaction_cadeau`. Une
-- écriture comptable ne se supprime pas, même en douceur ; `etat` porte tout le
-- cycle de vie et une transaction annulée reste lisible. Même raisonnement que
-- `mouvement_points`, déjà immuable.
--
-- Migration idempotente.
-- ════════════════════════════════════════════════════════════════════════════


-- ── 1. Types énumérés ───────────────────────────────────────────────────────
-- `purge` n'est pas un état de paiement à proprement parler : c'est la marque
-- laissée par la purge de fin de phase de test (research R11), qui conserve la
-- ligne pour l'historique tout en signalant que ses effets ont été annulés.

DO $$ BEGIN
    CREATE TYPE engagement.mode_cadeau AS ENUM ('soutien_financier', 'points');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE engagement.etat_paiement AS ENUM
        ('en_attente', 'abouti', 'echoue', 'expire', 'purge');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;


-- ── 2. Catalogue des cadeaux ────────────────────────────────────────────────
-- Aucun visuel à téléverser : un cadeau est une icône FontAwesome + une couleur,
-- exactement comme les badges et les niveaux déjà livrés.

CREATE TABLE IF NOT EXISTS engagement.cadeau (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code        VARCHAR(40)  NOT NULL UNIQUE,      -- clé stable, immuable après création
    libelle     VARCHAR(80)  NOT NULL,
    description TEXT,
    icone       VARCHAR(40),                       -- nom FontAwesome
    couleur     VARCHAR(20),                       -- jeton de couleur front
    prix        INTEGER      NOT NULL CHECK (prix > 0),    -- unité entière de la devise (FCFA)
    points      INTEGER      NOT NULL CHECK (points > 0),  -- points crédités au bénéficiaire
    ordre       SMALLINT     NOT NULL DEFAULT 0,
    actif       BOOLEAN      NOT NULL DEFAULT TRUE, -- désactivation, jamais suppression
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cadeau_actif_ordre
    ON engagement.cadeau(ordre, points DESC) WHERE actif = TRUE;


-- ── 3. Paramètres de monétisation (singleton) ───────────────────────────────
-- `id BOOLEAN PRIMARY KEY CHECK (id)` rend la SECONDE LIGNE IMPOSSIBLE en SQL :
-- il ne peut exister qu'un seul paramétrage, sans aucun code de garde applicatif.

CREATE TABLE IF NOT EXISTS engagement.parametre_monetisation (
    id                  BOOLEAN     PRIMARY KEY DEFAULT TRUE CHECK (id),
    taux_commission     SMALLINT    NOT NULL DEFAULT 10
                                    CHECK (taux_commission BETWEEN 0 AND 100),
    devise              VARCHAR(3)  NOT NULL DEFAULT 'XOF',
    -- Passe à TRUE le jour du branchement CinetPay. Bascule l'affichage du
    -- bandeau « phase de test » côté membre et CONDITIONNE l'accès à la purge :
    -- purger avant le basculement rouvrirait aussitôt la porte au minage.
    paiement_reel_actif BOOLEAN     NOT NULL DEFAULT FALSE,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


-- ── 4. Journal comptable des transactions ───────────────────────────────────

CREATE TABLE IF NOT EXISTS engagement.transaction_cadeau (
    id                 UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- Toujours issu du JWT, jamais du corps de la requête.
    offreur_id         UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    -- Résolu SERVEUR par `resoudre_beneficiaire` (research R4), jamais accepté du client.
    beneficiaire_id    UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    -- `ON DELETE RESTRICT` : supprimer un cadeau déjà offert devient structurellement
    -- impossible. L'erreur 409 du back-office n'est pas un contrôle applicatif.
    cadeau_id          UUID        NOT NULL REFERENCES engagement.cadeau(id) ON DELETE RESTRICT,
    -- Famille du contenu, ou 'profil' pour un cadeau offert depuis un profil.
    type_objet         VARCHAR(40) NOT NULL,
    objet_id           UUID        NOT NULL,
    mode               engagement.mode_cadeau NOT NULL,
    -- Prix, points et taux FIGÉS : une modification ultérieure du catalogue ou du
    -- taux ne réécrit jamais l'histoire comptable.
    montant            INTEGER     NOT NULL CHECK (montant > 0),
    points             INTEGER     NOT NULL CHECK (points > 0),
    taux_commission    SMALLINT    NOT NULL,
    part_beneficiaire  INTEGER     NOT NULL CHECK (part_beneficiaire >= 0),
    part_plateforme    INTEGER     NOT NULL CHECK (part_plateforme >= 0),
    etat               engagement.etat_paiement NOT NULL DEFAULT 'en_attente',
    -- Porté par la TRANSACTION, pas déduit de la configuration : c'est lui qui rend
    -- la purge de fin de phase exacte, même si des transactions réelles et simulées
    -- cohabitent le jour du basculement (research R7).
    simule             BOOLEAN     NOT NULL DEFAULT TRUE,
    reference_paiement TEXT        NOT NULL UNIQUE,
    message            VARCHAR(280),
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finalise_at        TIMESTAMPTZ,

    -- Répartition exacte par construction : calculer les deux parts indépendamment
    -- perdrait un franc d'arrondi (10 % de 1 001 → 100 + 900 ≠ 1 001).
    CONSTRAINT ck_transaction_repartition
        CHECK (part_beneficiaire + part_plateforme = montant),
    -- Auto-cadeau impossible : pas une garde applicative, une contrainte.
    CONSTRAINT ck_transaction_pas_auto_cadeau
        CHECK (offreur_id <> beneficiaire_id),
    -- Cadeau « au profil » : la cible EST le bénéficiaire. Interdit de pointer
    -- vers quelqu'un d'autre que lui.
    CONSTRAINT ck_transaction_cible_profil
        CHECK (type_objet <> 'profil' OR objet_id = beneficiaire_id),
    -- Mode « points » = 100 % plateforme, aucune cagnotte.
    CONSTRAINT ck_transaction_mode_points
        CHECK (mode <> 'points' OR part_beneficiaire = 0),
    -- Pas de date de finalisation sans état final.
    CONSTRAINT ck_transaction_finalisation
        CHECK (etat <> 'en_attente' OR finalise_at IS NULL)
);

CREATE INDEX IF NOT EXISTS idx_transaction_cadeau_beneficiaire
    ON engagement.transaction_cadeau(beneficiaire_id, etat);
CREATE INDEX IF NOT EXISTS idx_transaction_cadeau_offreur
    ON engagement.transaction_cadeau(offreur_id);
-- Affichage des cadeaux d'un contenu : une seule requête par page.
CREATE INDEX IF NOT EXISTS idx_transaction_cadeau_cible
    ON engagement.transaction_cadeau(type_objet, objet_id) WHERE etat = 'abouti';
-- Journal d'administration.
CREATE INDEX IF NOT EXISTS idx_transaction_cadeau_date
    ON engagement.transaction_cadeau(created_at DESC);
-- Purge de fin de phase de test.
CREATE INDEX IF NOT EXISTS idx_transaction_cadeau_purge
    ON engagement.transaction_cadeau(simule) WHERE etat = 'abouti';


-- ── 5. Cagnotte du bénéficiaire ─────────────────────────────────────────────
-- `montant_verse` reste à 0 pendant toute cette itération (aucun payout). La
-- colonne existe dès maintenant pour que l'arrivée du versement n'exige pas de
-- migration de la table la plus sensible.
--
-- Invariant vérifiable en recette :
--   montant_cumule = SUM(part_beneficiaire) des transactions abouties du membre

CREATE TABLE IF NOT EXISTS engagement.cagnotte (
    utilisateur_id  UUID        PRIMARY KEY REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    montant_cumule  INTEGER     NOT NULL DEFAULT 0 CHECK (montant_cumule >= 0),
    montant_verse   INTEGER     NOT NULL DEFAULT 0 CHECK (montant_verse >= 0),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT ck_cagnotte_verse_borne CHECK (montant_verse <= montant_cumule)
);


-- ════════════════════════════════════════════════════════════════════════════
-- SEED — catalogue initial et paramètres
-- ════════════════════════════════════════════════════════════════════════════
-- Le rapport prix/points est constant (100 FCFA le point) à la mise en service.
-- Rien ne l'impose : l'administration peut le rompre pour valoriser un cadeau
-- symbolique.

INSERT INTO engagement.cadeau (code, libelle, description, icone, couleur, prix, points, ordre) VALUES
    ('drapeau_ua', 'Drapeau de l''Union Africaine',
     'Le symbole panafricain, pour saluer une contribution marquante.', 'flag',      'green',  2000, 20, 1),
    ('badge',      'Badge',
     'Une distinction pour un contenu de qualité.',                     'certificate','amber', 1000, 10, 2),
    ('chapeau',    'Chapeau',
     'Un coup de chapeau amical.',                                      'hat-cowboy','chocolat', 500,  5, 3),
    ('fleur',      'Fleur',
     'Un petit geste qui fait plaisir.',                                'seedling',  'rose',    300,  3, 4),
    ('epingle',    'Épingle de costume',
     'Le clin d''œil discret.',                                         'thumbtack', 'slate',   100,  1, 5)
ON CONFLICT (code) DO NOTHING;

INSERT INTO engagement.parametre_monetisation (id, taux_commission, devise, paiement_reel_actif)
VALUES (TRUE, 10, 'XOF', FALSE)
ON CONFLICT (id) DO NOTHING;
