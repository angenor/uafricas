# Data Model: Retrouve Amis

**Feature**: 001-retrouve-amis | **Date**: 2026-02-27

---

## Schema : `retrouve_amis`

Nouveau bounded context (11ème schema). Dépendances cross-schema : `iam.utilisateur`, `shared.pays`.

---

## Enums

### `retrouve_amis.etat_avis`
```sql
CREATE TYPE retrouve_amis.etat_avis AS ENUM ('actif', 'cloture', 'suspendu');
```
- `actif` — Avis visible pour le moteur de recoupement
- `cloture` — Avis fermé par l'auteur (ami retrouvé ou abandon)
- `suspendu` — Avis suspendu par un admin suite à signalement

### `retrouve_amis.etat_correspondance`
```sql
CREATE TYPE retrouve_amis.etat_correspondance AS ENUM (
    'en_attente', 'acceptee_a', 'acceptee_b', 'mutuelle', 'declinee', 'archivee'
);
```
- `en_attente` — Correspondance détectée, aucune action des parties
- `acceptee_a` — L'auteur de l'avis source a accepté le contact
- `acceptee_b` — La cible a accepté le contact (l'auteur de l'avis n'a pas encore répondu)
- `mutuelle` — Les deux parties ont accepté → coordonnées partagées
- `declinee` — Une des parties a refusé → blacklist créée
- `archivee` — Aucune réponse après 30 jours → auto-archivage

### `retrouve_amis.type_cible`
```sql
CREATE TYPE retrouve_amis.type_cible AS ENUM ('avis', 'profil');
```
- `avis` — La correspondance a été trouvée entre deux avis de recherche
- `profil` — La correspondance a été trouvée entre un avis et un profil utilisateur "trouvable"

### `retrouve_amis.motif_signalement`
```sql
CREATE TYPE retrouve_amis.motif_signalement AS ENUM (
    'contenu_abusif', 'usurpation_identite', 'harcelement', 'autre'
);
```

### `retrouve_amis.etat_signalement`
```sql
CREATE TYPE retrouve_amis.etat_signalement AS ENUM ('en_attente', 'approuve', 'rejete');
```

### `retrouve_amis.type_parcours`
```sql
CREATE TYPE retrouve_amis.type_parcours AS ENUM ('ecole', 'ville_residence');
```
- `ecole` — École ou université fréquentée
- `ville_residence` — Ville de résidence passée

### `retrouve_amis.type_notification`
```sql
CREATE TYPE retrouve_amis.type_notification AS ENUM (
    'nouvelle_correspondance', 'acceptation_contact',
    'coordonnees_partagees', 'correspondance_archivee',
    'avis_suspendu'
);
```

---

## Tables

### 1. `retrouve_amis.avis_recherche`

Avis de recherche déposé par un utilisateur pour retrouver une personne.

| Colonne | Type | Contrainte | Description |
|---------|------|------------|-------------|
| `id` | UUID | PK DEFAULT uuid_generate_v4() | Identifiant unique |
| `auteur_id` | UUID | NOT NULL FK → iam.utilisateur | Utilisateur qui cherche |
| `nom_recherche` | VARCHAR(100) | NOT NULL | Nom de la personne recherchée |
| `prenom_recherche` | VARCHAR(100) | | Prénom (optionnel) |
| `surnom` | VARCHAR(100) | | Surnom ou diminutif |
| `ecole` | VARCHAR(250) | | École / université / lieu de rencontre |
| `ville` | VARCHAR(200) | | Ville connue |
| `pays_id` | UUID | FK → shared.pays | Pays connu |
| `periode_debut` | INT | CHECK (>= 1900 AND <= 2100) | Année de début approximative |
| `periode_fin` | INT | CHECK (>= 1900 AND <= 2100) | Année de fin approximative |
| `description` | TEXT | | Détails complémentaires (contexte, anecdote) |
| `etat` | etat_avis | NOT NULL DEFAULT 'actif' | Statut de l'avis |
| `search_vector` | TSVECTOR | | Index full-text search |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Date de création |
| `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Dernière modification |
| `deleted_at` | TIMESTAMPTZ | | Soft deletion |

**Indexes** :
```sql
CREATE INDEX idx_avis_recherche_auteur ON retrouve_amis.avis_recherche(auteur_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_avis_recherche_etat ON retrouve_amis.avis_recherche(etat) WHERE deleted_at IS NULL;
CREATE INDEX idx_avis_recherche_fts ON retrouve_amis.avis_recherche USING GIN(search_vector);
CREATE INDEX idx_avis_recherche_nom ON retrouve_amis.avis_recherche USING GIN(nom_recherche gin_trgm_ops) WHERE deleted_at IS NULL;
```

**Validations** :
- `nom_recherche` obligatoire + au moins un critère supplémentaire (vérifié côté handler)
- `periode_debut <= periode_fin` si les deux sont renseignés (CHECK constraint)
- Max 10 avis actifs par auteur (vérifié côté handler, pas en contrainte SQL)

**Transitions d'état** :
```
actif → cloture       (par l'auteur)
actif → suspendu      (par un admin, suite à signalement)
suspendu → actif      (par un admin, après modération)
```

---

### 2. `retrouve_amis.correspondance`

Résultat positif du recoupement entre un avis et une cible (autre avis ou profil trouvable).

| Colonne | Type | Contrainte | Description |
|---------|------|------------|-------------|
| `id` | UUID | PK DEFAULT uuid_generate_v4() | Identifiant unique |
| `avis_id` | UUID | NOT NULL FK → avis_recherche ON DELETE CASCADE | Avis source |
| `type_cible` | type_cible | NOT NULL | Type de la cible (avis ou profil) |
| `cible_avis_id` | UUID | FK → avis_recherche | Si cible = avis |
| `cible_utilisateur_id` | UUID | FK → iam.utilisateur | Si cible = profil trouvable |
| `score` | NUMERIC(5,2) | NOT NULL CHECK (>= 0 AND <= 100) | Score de correspondance |
| `details_score` | JSONB | NOT NULL DEFAULT '{}' | Détail par critère (nom, ville, période, école) |
| `etat` | etat_correspondance | NOT NULL DEFAULT 'en_attente' | Statut de la correspondance |
| `accepte_par_a_at` | TIMESTAMPTZ | | Date d'acceptation par l'auteur de l'avis |
| `accepte_par_b_at` | TIMESTAMPTZ | | Date d'acceptation par la cible |
| `coordonnees_a` | JSONB | | Coordonnées choisies par A (après mutuelle) |
| `coordonnees_b` | JSONB | | Coordonnées choisies par B (après mutuelle) |
| `expire_at` | TIMESTAMPTZ | | Date d'expiration (30j après création) |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Date de détection |
| `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Dernière modification |

**Indexes** :
```sql
CREATE INDEX idx_correspondance_avis ON retrouve_amis.correspondance(avis_id);
CREATE INDEX idx_correspondance_cible_avis ON retrouve_amis.correspondance(cible_avis_id) WHERE cible_avis_id IS NOT NULL;
CREATE INDEX idx_correspondance_cible_user ON retrouve_amis.correspondance(cible_utilisateur_id) WHERE cible_utilisateur_id IS NOT NULL;
CREATE INDEX idx_correspondance_etat ON retrouve_amis.correspondance(etat);
CREATE INDEX idx_correspondance_expire ON retrouve_amis.correspondance(expire_at) WHERE etat = 'en_attente';
```

**Contraintes** :
```sql
-- Au moins une cible doit être renseignée
CHECK (cible_avis_id IS NOT NULL OR cible_utilisateur_id IS NOT NULL)

-- Pas les deux cibles en même temps
CHECK (NOT (cible_avis_id IS NOT NULL AND cible_utilisateur_id IS NOT NULL))

-- Cohérence type/cible
CHECK (
  (type_cible = 'avis' AND cible_avis_id IS NOT NULL) OR
  (type_cible = 'profil' AND cible_utilisateur_id IS NOT NULL)
)
```

**Unicité** : Empêcher les doublons de correspondance :
```sql
CREATE UNIQUE INDEX idx_correspondance_unique_avis
  ON retrouve_amis.correspondance(avis_id, cible_avis_id)
  WHERE cible_avis_id IS NOT NULL AND etat NOT IN ('declinee', 'archivee');

CREATE UNIQUE INDEX idx_correspondance_unique_profil
  ON retrouve_amis.correspondance(avis_id, cible_utilisateur_id)
  WHERE cible_utilisateur_id IS NOT NULL AND etat NOT IN ('declinee', 'archivee');
```

**Transitions d'état** :
```
en_attente → acceptee_a     (auteur de l'avis accepte)
en_attente → acceptee_b     (cible accepte)
en_attente → declinee       (une partie refuse → blacklist)
en_attente → archivee       (30 jours sans réponse)
acceptee_a → mutuelle       (cible accepte aussi)
acceptee_a → declinee       (cible refuse → blacklist)
acceptee_a → archivee       (30 jours sans réponse de B)
acceptee_b → mutuelle       (auteur accepte aussi)
acceptee_b → declinee       (auteur refuse → blacklist)
acceptee_b → archivee       (30 jours sans réponse de A)
```

---

### 3. `retrouve_amis.parcours_trouvable`

Informations optionnelles ajoutées par un utilisateur "trouvable" pour améliorer le recoupement.

| Colonne | Type | Contrainte | Description |
|---------|------|------------|-------------|
| `id` | UUID | PK DEFAULT uuid_generate_v4() | Identifiant unique |
| `utilisateur_id` | UUID | NOT NULL FK → iam.utilisateur ON DELETE CASCADE | Propriétaire |
| `type_entree` | type_parcours | NOT NULL | Type (école ou ville) |
| `nom` | VARCHAR(250) | NOT NULL | Nom de l'école ou de la ville |
| `ville` | VARCHAR(200) | | Ville (pour les écoles) |
| `pays_id` | UUID | FK → shared.pays | Pays associé |
| `periode_debut` | INT | CHECK (>= 1900 AND <= 2100) | Année de début |
| `periode_fin` | INT | CHECK (>= 1900 AND <= 2100) | Année de fin |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Date de création |
| `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Dernière modification |

**Indexes** :
```sql
CREATE INDEX idx_parcours_utilisateur ON retrouve_amis.parcours_trouvable(utilisateur_id);
CREATE INDEX idx_parcours_nom ON retrouve_amis.parcours_trouvable USING GIN(nom gin_trgm_ops);
```

**Validations** :
- `periode_debut <= periode_fin` si les deux sont renseignés
- Un utilisateur peut avoir plusieurs entrées (1:N)

---

### 4. `retrouve_amis.blacklist`

Table symétrique empêchant toute future correspondance entre deux utilisateurs après un refus.

| Colonne | Type | Contrainte | Description |
|---------|------|------------|-------------|
| `utilisateur_a_id` | UUID | NOT NULL FK → iam.utilisateur | Premier utilisateur (le plus petit UUID) |
| `utilisateur_b_id` | UUID | NOT NULL FK → iam.utilisateur | Second utilisateur (le plus grand UUID) |
| `correspondance_id` | UUID | FK → correspondance | Correspondance à l'origine du blocage |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Date de blocage |

**Clé primaire** : `(utilisateur_a_id, utilisateur_b_id)`

**Contrainte de symétrie** :
```sql
CHECK (utilisateur_a_id < utilisateur_b_id)
```
Cela garantit qu'une paire n'est stockée qu'une fois. La vérification dans les requêtes utilise :
```sql
WHERE (LEAST(a, b), GREATEST(a, b)) IN (SELECT utilisateur_a_id, utilisateur_b_id FROM blacklist)
```

---

### 5. `retrouve_amis.signalement`

Signalement d'un avis de recherche par un utilisateur.

| Colonne | Type | Contrainte | Description |
|---------|------|------------|-------------|
| `id` | UUID | PK DEFAULT uuid_generate_v4() | Identifiant unique |
| `avis_id` | UUID | NOT NULL FK → avis_recherche ON DELETE CASCADE | Avis signalé |
| `signale_par` | UUID | NOT NULL FK → iam.utilisateur | Auteur du signalement |
| `motif` | motif_signalement | NOT NULL | Motif du signalement |
| `description` | TEXT | | Détail libre |
| `etat` | etat_signalement | NOT NULL DEFAULT 'en_attente' | Statut de modération |
| `modere_par` | UUID | FK → iam.utilisateur | Admin modérateur |
| `modere_at` | TIMESTAMPTZ | | Date de modération |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Date de signalement |

**Indexes** :
```sql
CREATE INDEX idx_signalement_avis ON retrouve_amis.signalement(avis_id);
CREATE INDEX idx_signalement_etat ON retrouve_amis.signalement(etat) WHERE etat = 'en_attente';
```

**Unicité** : Un utilisateur ne peut signaler le même avis qu'une fois :
```sql
CREATE UNIQUE INDEX idx_signalement_unique ON retrouve_amis.signalement(avis_id, signale_par);
```

---

### 6. `retrouve_amis.notification_retrouve`

Notifications internes liées à la fonctionnalité Retrouve Amis.

| Colonne | Type | Contrainte | Description |
|---------|------|------------|-------------|
| `id` | UUID | PK DEFAULT uuid_generate_v4() | Identifiant unique |
| `utilisateur_id` | UUID | NOT NULL FK → iam.utilisateur ON DELETE CASCADE | Destinataire |
| `correspondance_id` | UUID | FK → correspondance ON DELETE CASCADE | Correspondance liée (optionnel) |
| `type` | type_notification | NOT NULL | Type de notification |
| `lu` | BOOLEAN | NOT NULL DEFAULT FALSE | Lu par l'utilisateur |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | Date de création |

**Indexes** :
```sql
CREATE INDEX idx_notification_utilisateur ON retrouve_amis.notification_retrouve(utilisateur_id, lu) WHERE lu = FALSE;
CREATE INDEX idx_notification_created ON retrouve_amis.notification_retrouve(created_at);
```

---

## Modification cross-schema

### `iam.utilisateur` — Ajout de colonne

```sql
ALTER TABLE iam.utilisateur ADD COLUMN est_trouvable BOOLEAN NOT NULL DEFAULT FALSE;
```

Cette colonne indique si l'utilisateur consent à ce que ses informations de profil (nom, prénom, ville, pays) et son parcours (`retrouve_amis.parcours_trouvable`) soient utilisés par le moteur de recoupement.

---

## Fonction SQL de matching

### `retrouve_amis.calculer_correspondances(p_avis_id UUID)`

Fonction qui calcule les correspondances pour un avis donné.

**Pseudo-code** :
```sql
CREATE OR REPLACE FUNCTION retrouve_amis.calculer_correspondances(p_avis_id UUID)
RETURNS TABLE(cible_type type_cible, cible_id UUID, score NUMERIC, details JSONB) AS $$
DECLARE
    v_avis RECORD;
    v_auteur_id UUID;
BEGIN
    -- Charger l'avis
    SELECT * INTO v_avis FROM retrouve_amis.avis_recherche WHERE id = p_avis_id;
    v_auteur_id := v_avis.auteur_id;

    -- 1. Match contre les autres avis actifs
    RETURN QUERY
    SELECT 'avis'::type_cible, a2.id,
        -- Score nom (40 pts)
        GREATEST(
            similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(a2.nom_recherche))),
            COALESCE(similarity(unaccent(lower(v_avis.nom_recherche)), unaccent(lower(a2.prenom_recherche))), 0)
        ) * 40
        -- Score ville (15 pts)
        + CASE WHEN v_avis.ville IS NOT NULL AND a2.ville IS NOT NULL
            THEN similarity(unaccent(lower(v_avis.ville)), unaccent(lower(a2.ville))) * 15
            ELSE 0 END
        -- Score pays (10 pts)
        + CASE WHEN v_avis.pays_id IS NOT NULL AND a2.pays_id IS NOT NULL AND v_avis.pays_id = a2.pays_id
            THEN 10 ELSE 0 END
        -- Score école (20 pts)
        + CASE WHEN v_avis.ecole IS NOT NULL AND a2.ecole IS NOT NULL
            THEN similarity(unaccent(lower(v_avis.ecole)), unaccent(lower(a2.ecole))) * 20
            ELSE 0 END
        -- Score période (15 pts)
        + CASE WHEN v_avis.periode_debut IS NOT NULL AND a2.periode_debut IS NOT NULL
            THEN GREATEST(0,
                LEAST(COALESCE(v_avis.periode_fin, v_avis.periode_debut), COALESCE(a2.periode_fin, a2.periode_debut))
                - GREATEST(v_avis.periode_debut, a2.periode_debut)
            )::NUMERIC / GREATEST(1,
                GREATEST(COALESCE(v_avis.periode_fin, v_avis.periode_debut), COALESCE(a2.periode_fin, a2.periode_debut))
                - LEAST(v_avis.periode_debut, a2.periode_debut)
            ) * 15
            ELSE 0 END
        AS score_total,
        jsonb_build_object(/* détails */) AS details
    FROM retrouve_amis.avis_recherche a2
    WHERE a2.id != p_avis_id
      AND a2.auteur_id != v_auteur_id  -- Pas d'auto-correspondance
      AND a2.etat = 'actif'
      AND a2.deleted_at IS NULL
      -- Pas dans la blacklist
      AND NOT EXISTS (
          SELECT 1 FROM retrouve_amis.blacklist bl
          WHERE (bl.utilisateur_a_id, bl.utilisateur_b_id) = (LEAST(v_auteur_id, a2.auteur_id), GREATEST(v_auteur_id, a2.auteur_id))
      )
      -- Pas de correspondance active existante
      AND NOT EXISTS (
          SELECT 1 FROM retrouve_amis.correspondance c
          WHERE c.avis_id = p_avis_id AND c.cible_avis_id = a2.id
            AND c.etat NOT IN ('declinee', 'archivee')
      );

    -- 2. Match contre les profils trouvables (similaire, avec jointure sur parcours_trouvable)
    -- ...
END;
$$ LANGUAGE plpgsql;
```

**Note** : La fonction complète sera implémentée dans la phase de codage. Le pseudo-code ci-dessus illustre la logique de scoring.

---

## Diagramme de relations

```
iam.utilisateur (existant)
    │
    ├──── 1:N ────→ retrouve_amis.avis_recherche (auteur_id)
    ├──── 1:N ────→ retrouve_amis.parcours_trouvable (utilisateur_id)
    ├──── M:N ────→ retrouve_amis.blacklist (utilisateur_a_id, utilisateur_b_id)
    ├──── 1:N ────→ retrouve_amis.signalement (signale_par)
    └──── 1:N ────→ retrouve_amis.notification_retrouve (utilisateur_id)

shared.pays (existant)
    │
    ├──── 1:N ────→ retrouve_amis.avis_recherche (pays_id)
    └──── 1:N ────→ retrouve_amis.parcours_trouvable (pays_id)

retrouve_amis.avis_recherche
    │
    ├──── 1:N ────→ retrouve_amis.correspondance (avis_id)
    ├──── 1:N ────→ retrouve_amis.correspondance (cible_avis_id)
    └──── 1:N ────→ retrouve_amis.signalement (avis_id)

retrouve_amis.correspondance
    │
    ├──── 1:1 ────→ retrouve_amis.blacklist (correspondance_id)
    └──── 1:N ────→ retrouve_amis.notification_retrouve (correspondance_id)
```
