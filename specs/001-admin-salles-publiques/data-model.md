# Phase 1 : Data Model

**Feature** : 001-admin-salles-publiques
**Schéma cible** : `afrolang` (existant, étendu)
**Fichier SQL à modifier** : `uafricas_backend/doc/bd/schemas/08b_afrolang.sql`

---

## 1. Nouvel enum `afrolang.statut_proposition_salle`

```sql
CREATE TYPE afrolang.statut_proposition_salle AS ENUM (
    'en_attente',
    'validee',
    'rejetee',
    'retiree'
);
```

Transitions autorisées (appliquées côté backend) :

| De | Vers | Conditions |
|----|------|-----------|
| `en_attente` | `validee` | admin plateforme + transaction crée la salle |
| `en_attente` | `rejetee` | admin plateforme + commentaire obligatoire |
| `en_attente` | `retiree` | auteur lui-même |
| `validee` | : | terminal |
| `rejetee` | : | terminal |
| `retiree` | : | terminal |

---

## 2. Table `afrolang.proposition_salle`

```sql
CREATE TABLE afrolang.proposition_salle (
    id                    UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Données soumises par l'utilisateur
    auteur_id             UUID         NOT NULL,                 -- [xref] iam.utilisateur
    titre                 VARCHAR(350) NOT NULL,
    description           TEXT         NOT NULL,
    justification         TEXT         NOT NULL,
    langue_cible          VARCHAR(100) NOT NULL,
    langue_code           VARCHAR(40),
    groupe_ethnique_id    UUID         NOT NULL,                 -- [xref] country_profile.groupe_ethnique
    pays_origine_ids      UUID[]       NOT NULL DEFAULT '{}',    -- ≥ 1 (CHECK)

    -- Workflow
    statut                afrolang.statut_proposition_salle NOT NULL DEFAULT 'en_attente',
    decideur              UUID,                                  -- [xref] iam.utilisateur (admin plateforme)
    decide_at             TIMESTAMPTZ,
    commentaire_decision  TEXT,                                  -- obligatoire si statut=rejetee
    salle_id_creee        UUID         REFERENCES afrolang.salle(id) ON DELETE SET NULL,

    created_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_proposition_pays_non_vide CHECK (cardinality(pays_origine_ids) >= 1),
    CONSTRAINT ck_proposition_decision_coherente CHECK (
        (statut = 'en_attente' AND decideur IS NULL AND decide_at IS NULL) OR
        (statut IN ('validee', 'rejetee') AND decideur IS NOT NULL AND decide_at IS NOT NULL) OR
        (statut = 'retiree' AND decideur IS NULL)
    ),
    CONSTRAINT ck_proposition_rejet_commente CHECK (
        statut <> 'rejetee' OR commentaire_decision IS NOT NULL
    ),
    CONSTRAINT ck_proposition_validation_a_salle CHECK (
        statut <> 'validee' OR salle_id_creee IS NOT NULL
    )
);

-- Empêche un même auteur d'avoir plusieurs propositions en attente sur le même groupe ethnique
CREATE UNIQUE INDEX idx_proposition_salle_unique_attente
    ON afrolang.proposition_salle(auteur_id, groupe_ethnique_id)
    WHERE statut = 'en_attente';

CREATE INDEX idx_proposition_salle_statut
    ON afrolang.proposition_salle(statut, created_at DESC);

CREATE INDEX idx_proposition_salle_auteur
    ON afrolang.proposition_salle(auteur_id, created_at DESC);
```

**Règles de gestion** (appliquées dans `handlers/afrolang.rs`) :
- Soumission : refuser 409 si une `afrolang.salle` active existe déjà pour ce `groupe_ethnique_id` (cohérence avec `idx_afrolang_salle_groupe_unique`).
- Soumission : rate-limit anti-spam (Décision 6 research.md), refuser 429 si ≥ 5 rejets dans les 7 derniers jours pour cet auteur.
- Validation : transaction atomique (Décision 3 research.md).
- Retrait : autorisé seulement si `statut='en_attente'` et appelé par l'auteur.

---

## 3. Table `afrolang.salle_administrateur`

```sql
-- Rôle « administrateur de cette salle publique » : DISTINCT de salle_moderateur.
-- Les capacités effectives sont reportées (FR-019). Cette table est le réceptacle
-- d'autorisation : toute future capacité doit s'appuyer sur la fonction helper
-- est_administrateur_salle(salle_id, user_id) côté Rust.

CREATE TABLE afrolang.salle_administrateur (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id            UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    utilisateur_id      UUID         NOT NULL,                 -- [xref] iam.utilisateur
    nomme_par           UUID         NOT NULL,                 -- [xref] iam.utilisateur (admin plateforme)
    nomme_at            TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    actif               BOOLEAN      NOT NULL DEFAULT TRUE,
    revoque_at          TIMESTAMPTZ,
    revoque_par         UUID,                                   -- [xref] iam.utilisateur
    motif_revocation    TEXT,
    suspendu_at         TIMESTAMPTZ,                            -- cascade FR-021/FR-022
    motif_suspension    TEXT,                                   -- 'salle_archivee' | 'compte_desactive'

    created_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_admin_revocation_coherente CHECK (
        (actif = TRUE  AND revoque_at IS NULL AND revoque_par IS NULL AND motif_revocation IS NULL) OR
        (actif = FALSE)
    )
);

-- Une seule nomination active par (salle, utilisateur)
CREATE UNIQUE INDEX idx_salle_admin_unique_actif
    ON afrolang.salle_administrateur(salle_id, utilisateur_id)
    WHERE actif = TRUE;

CREATE INDEX idx_salle_admin_par_salle
    ON afrolang.salle_administrateur(salle_id) WHERE actif = TRUE;

CREATE INDEX idx_salle_admin_par_user
    ON afrolang.salle_administrateur(utilisateur_id) WHERE actif = TRUE;
```

**Règles** :
- Nomination : 409 si une ligne `actif=TRUE` existe déjà pour le couple.
- Révocation : `actif=FALSE`, `revoque_at=NOW()`, `revoque_par`, `motif_revocation` (libre).
- Suspension automatique (FR-021/FR-022) : `actif=FALSE`, `suspendu_at=NOW()`, `motif_suspension`. **Pas** de réactivation automatique : un admin doit re-nommer l'utilisateur si nécessaire.
- Distinction stricte avec `salle_moderateur` (FR-018) : aucun croisement sémantique. Un même utilisateur peut être simultanément modérateur attitré ET administrateur de la même salle (futur découplage des pouvoirs).

---

## 4. Extension du DTO `SalleResponse` (lecture publique)

`GET /api/afrolang/salles` et `GET /api/afrolang/salles/{id}` renvoient désormais en plus :

```rust
pub struct AdministrateurLight {
    pub utilisateur_id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub photo_url: Option<String>,
    pub nomme_at: DateTime<Utc>,
}

// SalleResponse étendu :
pub administrateurs: Vec<AdministrateurLight>,  // peuplé via json_agg actif=TRUE
```

---

## 5. Audit (Principe VII)

Mutations à instrumenter avec `audit::log_action(action, "afrolang", table, entity_id)` :

| Handler | Action | Table | Entity |
|---------|--------|-------|--------|
| `soumettre_proposition` | `CREATE` | `proposition_salle` | `proposition_id` |
| `retirer_proposition` (auteur) | `UPDATE` | `proposition_salle` | `proposition_id` |
| `valider_proposition` (admin) | `VALIDATE` | `proposition_salle` | `proposition_id` |
| `valider_proposition` (admin) | `CREATE` | `salle` | `salle_id` (créée par la transaction) |
| `rejeter_proposition` (admin) | `REJECT` | `proposition_salle` | `proposition_id` |
| `nommer_administrateur_salle` (admin) | `CREATE` | `salle_administrateur` | `lien_id` |
| `revoquer_administrateur_salle` (admin) | `UPDATE` | `salle_administrateur` | `lien_id` |
| Cascade salle archivée | `UPDATE` | `salle_administrateur` | `lien_id` (chaque ligne suspendue) |
| Cascade compte désactivé | `UPDATE` | `salle_administrateur` | `lien_id` (chaque ligne suspendue) |

Les avant/après JSONB sont automatiquement capturés par le pattern existant.

---

## 6. Diagramme relationnel (résumé)

```
iam.utilisateur ──┬──< afrolang.proposition_salle (auteur_id, decideur)
                  └──< afrolang.salle_administrateur (utilisateur_id, nomme_par, revoque_par)

afrolang.salle ───┬──< afrolang.salle_administrateur (salle_id) ON DELETE CASCADE
                  └──< afrolang.proposition_salle (salle_id_creee) ON DELETE SET NULL

country_profile.groupe_ethnique ──< afrolang.proposition_salle (groupe_ethnique_id)
shared.pays                      ──< afrolang.proposition_salle (pays_origine_ids[])
```

FK cross-schema sur `iam.utilisateur` et `country_profile.*` à ajouter dans `13_contraintes_inter_schemas.sql`.
