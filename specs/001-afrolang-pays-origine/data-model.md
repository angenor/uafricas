# Data Model — Pays d'origine des salles publiques Afrolang

**Phase 1** — Modèle de données dérivé de la spec et des décisions de `research.md`.

## Entités

### `afrolang.salle_pays_origine` *(nouvelle)*

Table de jointure pure modélisant la relation **plusieurs-à-plusieurs** entre une salle publique Afrolang et les pays d'origine de la langue qui y est enseignée.

| Colonne     | Type           | Contraintes                                                              | Notes                                            |
|-------------|----------------|--------------------------------------------------------------------------|--------------------------------------------------|
| `salle_id`  | `UUID`         | `NOT NULL`, FK → `afrolang.salle(id)` `ON DELETE CASCADE`                | Clé composite                                    |
| `pays_id`   | `UUID`         | `NOT NULL`, FK → `shared.pays(id)` `ON DELETE CASCADE`                   | Clé composite — FR-010 (cleanup auto)            |
| `created_at`| `TIMESTAMPTZ`  | `NOT NULL DEFAULT NOW()`                                                 | Audit léger                                      |
| **PK**      | composite      | `PRIMARY KEY (salle_id, pays_id)`                                        | Unicité gratuite — FR-002                        |

**Index complémentaire** :

```sql
CREATE INDEX idx_afrolang_salle_pays_origine_pays
    ON afrolang.salle_pays_origine (pays_id);
```

(L'index inverse est utile pour le filtre public `?pays_id=` qui pivote sur le pays.)

**Pas de `deleted_at`** : les associations sont binaires (présentes ou absentes). Le retrait est un `DELETE` physique, l'audit garde la trace.

### Entités existantes touchées

#### `afrolang.salle` *(inchangée)*

Aucune modification de colonne. La nouvelle relation est purement externe (table de jointure), ce qui préserve les requêtes existantes et la rétro-compatibilité (FR-009).

#### `shared.pays` *(inchangée)*

Référencée uniquement. Colonnes utilisées en lecture publique : `id`, `nom`, `code_iso2`. Le filtre `actif = TRUE` est appliqué côté requête (Q3) — aucun changement de structure nécessaire.

## DDL complet à insérer dans `08b_afrolang.sql`

À ajouter **après** la définition de `afrolang.salle_pays_session_participant` (ou tout autre bloc final de la section « salles ») :

```sql
-- ── Pays d'origine d'une salle publique (feature 001-afrolang-pays-origine) ──
--
-- Relation N-N entre une salle publique et les pays où la langue cible est
-- parlée à l'origine. Indépendante du pays implicite via groupe_ethnique →
-- fiche_pays. Enrichie manuellement par les admins (Q1 — aucun pré-remplissage).

CREATE TABLE afrolang.salle_pays_origine (
    salle_id    UUID         NOT NULL REFERENCES afrolang.salle(id)   ON DELETE CASCADE,
    pays_id     UUID         NOT NULL REFERENCES shared.pays(id)      ON DELETE CASCADE,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    PRIMARY KEY (salle_id, pays_id)
);

CREATE INDEX idx_afrolang_salle_pays_origine_pays
    ON afrolang.salle_pays_origine (pays_id);

COMMENT ON TABLE  afrolang.salle_pays_origine IS
    'Pays d''origine d''une salle publique Afrolang (feature 001-afrolang-pays-origine).';
COMMENT ON COLUMN afrolang.salle_pays_origine.pays_id IS
    'FK vers shared.pays. Filtré sur actif=TRUE côté API publique (Q3).';
```

## DTO Rust (`src/models/afrolang.rs`)

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaysOrigineLight {
    pub id: Uuid,
    pub nom: String,
    pub code_iso2: Option<String>,
}

// SalleResponse — ajout du champ :
pub struct SalleResponse {
    // ... champs existants ...
    pub pays_origine: Vec<PaysOrigineLight>,
}

// SalleDetailResponse — idem (hérite via aplatissement existant) :
pub struct SalleDetailResponse {
    // ... champs existants ...
    pub pays_origine: Vec<PaysOrigineLight>,
}
```

`SalleRow` (struct sqlx `FromRow`) reçoit la colonne agrégée comme `sqlx::types::Json<Vec<PaysOrigineLight>>` puis aplatit dans le `From<SalleRow> for SalleResponse`.

## Interface TypeScript (`app/composables/useAfrolang.ts`)

```ts
export interface PaysOrigineLight {
  id: string
  nom: string
  code_iso2: string | null
}

// SalleAPI — ajout du champ :
export interface SalleAPI {
  // ... champs existants ...
  pays_origine: PaysOrigineLight[]   // jamais null, [] par défaut
}

// SalleFiltres — ajout :
export interface SalleFiltres {
  // ... champs existants ...
  pays_id?: string
}
```

## Validation et règles métier

| Règle           | Implémentation                                                                 |
|-----------------|--------------------------------------------------------------------------------|
| FR-001 (0..N)   | Cardinalité naturelle de la table de jointure                                  |
| FR-002 (unicité)| `PRIMARY KEY (salle_id, pays_id)` + `INSERT ... ON CONFLICT DO NOTHING`        |
| FR-003 (ordre)  | `ORDER BY p.nom` dans la sous-requête `json_agg`                               |
| FR-006 (filtre) | `EXISTS (... WHERE pays_id = $X AND p.actif = TRUE)`                           |
| FR-008 (audit)  | `audit::log_action` à chaque appel ajouter/retirer                             |
| FR-009 (legacy) | Aucun seed → tables existantes → liste vide automatique                        |
| FR-010 (cleanup)| `FK ON DELETE CASCADE` côté `shared.pays`                                      |
| FR-011 (perms)  | `verifier_permission!(admin, "afrolang", "modifier")` sur les 2 endpoints     |
| Q3 (archivés)   | `JOIN shared.pays p ON p.id = spo.pays_id WHERE p.actif = TRUE` (public seul)  |

## États et transitions

Aucun état métier — la table est purement relationnelle. Cycle de vie :

```text
(rien) ──INSERT──▶ (couple existe) ──DELETE──▶ (rien)
```

Le `created_at` reste à titre informatif/audit, jamais modifié.

## Volume estimé

- ~30 salles × moyenne 3 pays = ~90 lignes en année 1.
- Croissance linéaire avec le nombre de salles publiques ajoutées par les admins.
- Aucun risque de scaling — la table restera ≤ 1 000 lignes à l'horizon visible.
