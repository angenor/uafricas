-- ════════════════════════════════════════════════════════════════════════════
-- AFRICANS-WORLD — Engagement : log des partages vers les réseaux externes
-- ════════════════════════════════════════════════════════════════════════════
-- Feature 007-engagement-points-badges — US5.
--
-- Les tables `partage_*` existantes (partage_media, partage_element, …) ne sont
-- que des REPOSTS INTERNES sur le mur : rien ne trace le partage vers un réseau
-- social. Sans cette table, la règle « N réseaux distincts » n'aurait aucune
-- donnée sur laquelle s'appuyer.
--
-- Le garde-fou central est STRUCTUREL et non déclaratif : l'unicité
-- (membre, contenu, réseau) rend « réseaux **distincts** » impossible à
-- contourner — répéter un réseau ne crée aucune ligne, donc n'avance pas le
-- compteur. Le partage externe étant la règle la moins vérifiable du barème,
-- c'est la contrainte qui protège, pas un contrôle applicatif.
--
-- Note d'honnêteté : ce log enregistre une **intention** de partage (le clic),
-- pas une publication effective. Aucune plateforme ne permet de vérifier l'autre.
--
-- Migration idempotente.
-- ════════════════════════════════════════════════════════════════════════════


-- ── Enum des réseaux traçables ───────────────────────────────────────────────
-- Un réseau futur s'ajoute par `ALTER TYPE … ADD VALUE`.
-- « Copier le lien » n'est PAS un réseau : invérifiable et trivialement
-- répétable, il ne compte pas et n'a donc pas de valeur ici.

DO $$ BEGIN
    CREATE TYPE engagement.reseau_social AS ENUM
        ('whatsapp', 'facebook', 'x', 'linkedin', 'telegram', 'email');
EXCEPTION WHEN duplicate_object THEN NULL;
END $$;


-- ── Log des partages externes ────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS engagement.partage_externe (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    -- Toujours issu du JWT, JAMAIS du corps de la requête : le client ne peut pas
    -- déclarer un partage au nom d'un autre membre.
    utilisateur_id UUID        NOT NULL REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    type_objet     VARCHAR(40) NOT NULL,   -- famille du contenu partagé
    objet_id       UUID        NOT NULL,
    reseau         engagement.reseau_social NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- C'est CETTE contrainte qui rend « distincts » structurel.
    UNIQUE (utilisateur_id, type_objet, objet_id, reseau)
);

-- Sert le `COUNT(DISTINCT reseau)` pour un couple (membre, contenu).
CREATE INDEX IF NOT EXISTS idx_partage_externe_contenu
    ON engagement.partage_externe(utilisateur_id, type_objet, objet_id);
