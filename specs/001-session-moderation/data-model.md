# Phase 1 — Data Model : Modération de session Afrolang

**Date** : 2026-05-10
**Schema cible** : `afrolang` (existant)
**Source canonique** : `uafricas_backend/doc/bd/schemas/08b_afrolang.sql`

## Modifications de schéma

### 1. Nouvelle table : `afrolang.session_permission_tableau_blanc`

État éphémère des permissions d'écriture sur le tableau blanc, accordées explicitement par un modérateur de session. Les modérateurs eux-mêmes (admin plateforme, admin salle, modérateur attitré, créateur salle privée) **ne sont pas** stockés dans cette table — leur droit d'écriture est calculé à la volée par `est_moderateur_session()` (R6).

```sql
CREATE TABLE afrolang.session_permission_tableau_blanc (
    session_id      UUID         NOT NULL REFERENCES afrolang.session(id) ON DELETE CASCADE,
    utilisateur_id  UUID         NOT NULL,                      -- [xref] iam.utilisateur
    accorde_par     UUID         NOT NULL,                      -- [xref] iam.utilisateur (modérateur)
    accorde_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    PRIMARY KEY (session_id, utilisateur_id)
);

CREATE INDEX idx_afrolang_perm_tb_session
    ON afrolang.session_permission_tableau_blanc(session_id);
CREATE INDEX idx_afrolang_perm_tb_user
    ON afrolang.session_permission_tableau_blanc(utilisateur_id);
```

**Caractéristiques** :
- PK composite `(session_id, utilisateur_id)` → garantit l'unicité (1 ligne par couple, FR-011/FR-012).
- `ON DELETE CASCADE` sur `session_id` → satisfait FR-017 sans cron : à la suppression de la session (cascade depuis `salle` ou `salle_privee` ou suppression explicite), les permissions disparaissent.
- Pas de `deleted_at` (soft-delete) : retirer une permission = `DELETE` direct (cohérent avec `salle_pays_origine` de la feature `001-afrolang-pays-origine`).
- Pas de FK vers `iam.utilisateur` (convention cross-schema du projet : `[xref]`).

**Cardinalités** :
- 1 session × N permissions individuelles.
- 1 utilisateur × N permissions (une par session active à laquelle il participe).

### 2. Extension de `afrolang.session`

Trois colonnes nullables pour porter l'état spotlight (R2). Aucune table dédiée car cardinalité 0..1 par session.

```sql
ALTER TABLE afrolang.session
    ADD COLUMN participant_mis_en_evidence_id UUID         NULL,  -- [xref] iam.utilisateur
    ADD COLUMN mis_en_evidence_par            UUID         NULL,  -- [xref] iam.utilisateur (admin)
    ADD COLUMN mis_en_evidence_at             TIMESTAMPTZ  NULL;

-- Cohérence : les trois colonnes sont toutes NULL, ou toutes NOT NULL
ALTER TABLE afrolang.session
    ADD CONSTRAINT ck_session_spotlight_coherent CHECK (
        (participant_mis_en_evidence_id IS NULL AND mis_en_evidence_par IS NULL AND mis_en_evidence_at IS NULL)
        OR
        (participant_mis_en_evidence_id IS NOT NULL AND mis_en_evidence_par IS NOT NULL AND mis_en_evidence_at IS NOT NULL)
    );
```

**Caractéristiques** :
- `CHECK` garantit que les trois colonnes sont mises à jour ensemble (cohérence FR-020/FR-022).
- Pas d'index dédié : accès par PK `session.id` toujours.
- Mise à NULL à la clôture de la session (handler de fermeture).

### 3. Aucune nouvelle valeur d'ENUM, aucun nouveau type

Le rôle « modérateur de session » est calculé applicatif (R6) — pas de matérialisation SQL.

## Entités modèle (Rust / TypeScript)

### Rust — `uafricas_backend/src/models/afrolang.rs` (extension)

```rust
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PermissionTableauBlanc {
    pub utilisateur_id: Uuid,
    pub accorde_par: Uuid,
    pub accorde_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionTableauBlancResponse {
    pub utilisateur_id: Uuid,
    pub nom_complet: String,            // jointure iam.utilisateur
    pub avatar_url: Option<String>,
    pub accorde_par: Uuid,
    pub accorde_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AccorderPermissionPayload {
    pub utilisateur_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotlightInfo {
    pub utilisateur_id: Uuid,
    pub nom_complet: String,
    pub avatar_url: Option<String>,
    pub mis_en_evidence_par: Uuid,
    pub mis_en_evidence_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct MettreEnEvidencePayload {
    pub utilisateur_id: Uuid,
}

#[derive(Debug, Serialize)]
pub enum NiveauModerateur {
    AdminPlateforme,
    AdminSalle,
    ModerateurAttitre,
    CreateurSallePrivee,
}

impl NiveauModerateur {
    pub fn peut_spotlight(&self) -> bool {
        matches!(self, Self::AdminPlateforme | Self::AdminSalle | Self::CreateurSallePrivee)
    }
}
```

### TypeScript — `uafricas_frontend/app/composables/useAfrolang.ts` (extension)

```ts
export interface PermissionTableauBlancAPI {
  utilisateur_id: string
  nom_complet: string
  avatar_url: string | null
  accorde_par: string
  accorde_at: string
}

export interface SpotlightInfoAPI {
  utilisateur_id: string
  nom_complet: string
  avatar_url: string | null
  mis_en_evidence_par: string
  mis_en_evidence_at: string
}

export type NiveauModerateur =
  | 'admin_plateforme'
  | 'admin_salle'
  | 'moderateur_attitre'
  | 'createur_salle_privee'
  | null
```

## Flux d'état (state transitions)

### Permission tableau blanc

```
ABSENT  --(POST permissions)-->  ACCORDEE
ACCORDEE  --(DELETE permissions)-->  ABSENT
ACCORDEE  --(session terminée → CASCADE)-->  ABSENT
```

### Spotlight de session (transitions sur `afrolang.session`)

```
                ┌─────────────────┐
                ▼                 │
  AUCUN ──(POST spotlight, payload=A)──► A_EN_VEDETTE
                                          │
                            ┌─────────────┤
                            │             │
   (POST spotlight, payload=B)    (DELETE spotlight)
                            │             │
                            ▼             ▼
                       B_EN_VEDETTE     AUCUN

  (cible quitte la session)─► AUCUN  (FR-025 — handler quitter session)
  (session terminée)         ─► AUCUN  (handler fermeture)
```

## Conformité aux principes de la constitution

| Principe | Application |
|---|---|
| III — SQL source de vérité | DDL écrit en premier ; structs Rust et types TS dérivent exactement de ce DDL |
| I — Français | Tous les noms : `session_permission_tableau_blanc`, `participant_mis_en_evidence_id`, `accorde_par`, `mis_en_evidence_at` |
| V — Simplicité | Aucune nouvelle table pour le spotlight (3 colonnes nullables) ; aucun trigger, aucun ENUM |
| VII — Audit | Toutes les mutations passent par les handlers Rust → `audit::log_action` systématique |
