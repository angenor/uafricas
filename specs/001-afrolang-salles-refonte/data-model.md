# Data Model : Refonte salles Afrolang

**Branch** : `001-afrolang-salles-refonte`
**Date** : 2026-04-15
**Schema cible** : `afrolang` (PostgreSQL 16)

Ce document décrit l'état **cible** du schéma `afrolang` après la migration. Les diff par rapport au schéma actuel (`08b_afrolang.sql`) sont annotés.

---

## Vue d'ensemble

| Table | État | Diff |
|---|---|---|
| `afrolang.salle` | Conservée | Inchangée |
| `afrolang.salle_privee` | **Modifiée** | Colonnes legacy retirées + `code_acces_hash` ajouté |
| `afrolang.session` | Conservée | Inchangée (XOR salle/salle_privee toujours valide) |
| `afrolang.session_participant` | Conservée | Inchangée |
| `afrolang.tableau_blanc` | Conservée | Inchangée |
| `afrolang.ressource_salle` | Conservée | Inchangée |
| `afrolang.message_session` | Conservée | Inchangée |
| `afrolang.salle_moderateur` | Conservée | Concerne uniquement les salles publiques |
| `afrolang.proposition_salle` | **Supprimée** | Création publique = admin uniquement |
| `afrolang.salle_privee_adhesion` | **Supprimée** | Adhésion remplacée par code secret |
| `afrolang.tentative_code_acces` | **Nouvelle** | Rate limit (R4) |

**Types ENUM supprimés** : `motif_salle_privee`, `visibilite_salle_privee`, `type_adhesion`, `etat_adhesion`, `etat_proposition`.

**Types ENUM conservés** : `etat_session`, `type_ressource`, `etat_ressource`.

---

## Entité 1 : `afrolang.salle` (publique), INCHANGÉE

Salle publique thématique (groupe ethnique × langue), créée exclusivement par un administrateur.

| Colonne | Type | Contraintes | Notes |
|---|---|---|---|
| `id` | UUID | PK, default `uuid_generate_v4()` | |
| `groupe_ethnique_id` | UUID | FK `country_profile.groupe_ethnique` | Contexte thématique |
| `titre` | VARCHAR(350) | NOT NULL | |
| `description` | TEXT | | |
| `langue` | VARCHAR(120) | NOT NULL | |
| `actif` | BOOLEAN | NOT NULL DEFAULT TRUE | |
| `cree_par` | UUID | NOT NULL `[xref] iam.utilisateur` | Admin créateur |
| `created_at` / `updated_at` / `deleted_at` | TIMESTAMPTZ | | Soft delete |

**Aucune modification.** Les RLS / restrictions de création sont gérées côté handler admin (déjà en place).

---

## Entité 2 : `afrolang.salle_privee` (durable, code secret), MODIFIÉE

Cercle privé créé par n'importe quel utilisateur connecté, rattaché à exactement une salle publique. Objet **durable** (Q1) : alterne dormant ↔ session live en cours.

### Schéma cible

| Colonne | Type | Contraintes | Diff |
|---|---|---|---|
| `id` | UUID | PK, default `uuid_generate_v4()` |, |
| `salle_id` | UUID | NOT NULL FK `afrolang.salle(id) ON DELETE RESTRICT` |, |
| `titre` | VARCHAR(350) | NOT NULL, len ≥ 5 |, |
| `description` | TEXT | NULL OK, len ≤ 1000 |, |
| `code_acces_hash` | CHAR(60) | NOT NULL | **NOUVEAU** (bcrypt cost 10, R3) |
| `image_couverture_url` | VARCHAR(500) | |, (conservée, optionnelle) |
| `max_participants` | INT | DEFAULT 50 |, |
| `archivee_at` | TIMESTAMPTZ | | Cascade cf. FR-017 |
| `actif` | BOOLEAN | NOT NULL DEFAULT TRUE |, |
| `cree_par` | UUID | NOT NULL `[xref] iam.utilisateur` | Auteur |
| `created_at` / `updated_at` / `deleted_at` | TIMESTAMPTZ | | Soft delete |

### Colonnes supprimées (DROP COLUMN)

- `motif` (`afrolang.motif_salle_privee` ENUM), concept abandonné
- `declaration_adulte_at` (TIMESTAMPTZ), obligation 18+ retirée
- `visibilite` (`afrolang.visibilite_salle_privee` ENUM), toutes les salles privées sont listées dans le widget, l'accès est contrôlé par code secret
- `code_acces` (VARCHAR(100)) : remplacé par `code_acces_hash`

### Index conservés / mis à jour

- `idx_afrolang_privee_salle` sur `(salle_id)`, conservé.
- `idx_afrolang_privee_unique_par_salle` UNIQUE sur `(salle_id, cree_par) WHERE archivee_at IS NULL AND deleted_at IS NULL`, **conservé** (FR-010, SC-005).
- `idx_afrolang_privee_visibilite` : **supprimé** (colonne `visibilite` retirée).

### Règles métier (validées en handler)

- **VR-1** : `code_acces_hash` issu de bcrypt(plaintext) où `plaintext` matche `^[A-Za-z0-9!@#$%&*?-]{4,16}$`.
- **VR-2** : à la création, vérifier qu'il n'existe pas déjà une ligne `(salle_id, cree_par) WHERE archivee_at IS NULL AND deleted_at IS NULL` (en complément de la contrainte UNIQUE).
- **VR-3** : seul `cree_par` peut modifier `titre`, `description`, `code_acces_hash`, archiver.
- **VR-4** : modification du `code_acces_hash` nécessite un nouveau plaintext respectant VR-1.

### Transitions d'état

```text
                                 ┌─ session live en cours ──┐
créée (dormante) ─────────────►  │  (1 ligne afrolang.session│  ──► dormante
                                 │   etat='en_cours')        │
                                 └───────────────────────────┘
                                            │
                                            ▼
                                    archivée (archivee_at = now)
                                       │
                                       ▼
                                  supprimée (deleted_at = now)
```

Démarrage d'une session = INSERT dans `afrolang.session` avec `salle_privee_id` renseigné, `etat='en_cours'`, `moderateur_id = salle_privee.cree_par`, `cree_par = utilisateur courant` (toujours = auteur).

Fin de session = UPDATE `afrolang.session` SET `etat='terminee'`, `termine_at=now()`, `duree_secondes=...`. La salle privée redevient dormante mais reste persistante.

---

## Entité 3 : `afrolang.session` : INCHANGÉE

Conservée telle quelle. Pour rappel, contrainte XOR :

```sql
CONSTRAINT ck_session_contexte CHECK (
    (salle_id IS NOT NULL AND salle_privee_id IS NULL) OR
    (salle_id IS NULL     AND salle_privee_id IS NOT NULL)
)
```

**Sessions de salle publique** (refonte) : `cree_par` = n'importe quel utilisateur connecté ayant cliqué « Démarrer » (cf. FR-005b). `moderateur_id` = `cree_par` initialement (pas de transfert sur la refonte ; règle existante de transfert manuel applicable si la table `salle_moderateur` est conservée).

**Sessions de salle privée** : `cree_par` = `moderateur_id` = `salle_privee.cree_par` (auteur uniquement).

**Règle handler** : au plus une session avec `etat='en_cours'` par `salle_id` ou `salle_privee_id`. Vérification au démarrage (`SELECT … FOR UPDATE`).

---

## Entité 4 : `afrolang.tentative_code_acces`, NOUVELLE

Trace les tentatives de saisie du code secret pour appliquer le rate limit (R4).

```sql
CREATE TABLE afrolang.tentative_code_acces (
    id              UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id UUID         NOT NULL REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    utilisateur_id  UUID         NOT NULL,                          -- [xref] iam.utilisateur
    tente_at        TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    succes          BOOLEAN      NOT NULL DEFAULT FALSE,
    ip              INET,                                            -- traçabilité audit
    user_agent      TEXT
);

CREATE INDEX idx_afrolang_tentative_lookup
    ON afrolang.tentative_code_acces(salle_privee_id, utilisateur_id, tente_at DESC);

CREATE INDEX idx_afrolang_tentative_purge
    ON afrolang.tentative_code_acces(tente_at);
```

**Règles** :

- INSERT à chaque tentative (succès ou échec).
- Avant vérification du hash : `SELECT count(*) FROM afrolang.tentative_code_acces WHERE salle_privee_id=$1 AND utilisateur_id=$2 AND succes=FALSE AND tente_at > NOW() - INTERVAL '1 minute'`. Si ≥ 5 ET dernière tentative < 5 min → refuser sans vérifier le hash.
- Pas de purge automatique : volume attendu faible. Une purge cron > 30 j sera ajoutée si nécessaire.

---

## Entités SUPPRIMÉES

### `afrolang.salle_privee_adhesion`

`DROP TABLE afrolang.salle_privee_adhesion CASCADE;`

Mécanisme adhésion / invitation / demande remplacé par code secret unique.

### `afrolang.proposition_salle`

`DROP TABLE afrolang.proposition_salle CASCADE;`

Création de salles publiques par utilisateurs abandonnée, admin uniquement (FR-005).

### Types ENUM associés

```sql
DROP TYPE IF EXISTS afrolang.motif_salle_privee CASCADE;
DROP TYPE IF EXISTS afrolang.visibilite_salle_privee CASCADE;
DROP TYPE IF EXISTS afrolang.type_adhesion CASCADE;
DROP TYPE IF EXISTS afrolang.etat_adhesion CASCADE;
DROP TYPE IF EXISTS afrolang.etat_proposition CASCADE;
```

---

## Migration SQL (extrait)

À appliquer in-place dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (le fichier sera reconstruit au prochain `docker-init.sh` car le produit n'est pas en production, Q2).

```sql
-- Suppressions tables/types legacy
DROP TABLE IF EXISTS afrolang.salle_privee_adhesion CASCADE;
DROP TABLE IF EXISTS afrolang.proposition_salle CASCADE;
DROP TYPE  IF EXISTS afrolang.type_adhesion CASCADE;
DROP TYPE  IF EXISTS afrolang.etat_adhesion CASCADE;
DROP TYPE  IF EXISTS afrolang.etat_proposition CASCADE;
DROP TYPE  IF EXISTS afrolang.motif_salle_privee CASCADE;
DROP TYPE  IF EXISTS afrolang.visibilite_salle_privee CASCADE;

-- Refactor salle_privee
ALTER TABLE afrolang.salle_privee
    DROP COLUMN IF EXISTS motif,
    DROP COLUMN IF EXISTS declaration_adulte_at,
    DROP COLUMN IF EXISTS visibilite,
    DROP COLUMN IF EXISTS code_acces;

ALTER TABLE afrolang.salle_privee
    ADD COLUMN code_acces_hash CHAR(60) NOT NULL DEFAULT '$2b$10$placeholderforinit.................................';

DROP INDEX IF EXISTS afrolang.idx_afrolang_privee_visibilite;

-- Nouvelle table rate limit
CREATE TABLE afrolang.tentative_code_acces ( … );
CREATE INDEX idx_afrolang_tentative_lookup ON … ;
CREATE INDEX idx_afrolang_tentative_purge  ON … ;
```

> Note : le `DEFAULT` placeholder sur `code_acces_hash` n'est ajouté que pour permettre `ALTER TABLE` sur la base actuelle ; en pratique, comme on fait table rase via reset Docker, on peut écrire `NOT NULL` sans DEFAULT. La forme finale dans `08b_afrolang.sql` après refactor sera l'écriture directe (sans ALTER), cf. quickstart.

---

## Cohérence cross-stack

| Couche | Artefact |
|---|---|
| **SQL** | `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (réécriture in-place) |
| **Rust models** | `uafricas_backend/src/models/admin/salle_privee.rs` (struct `SallePrivee`, retrait des champs `motif`, `declaration_adulte_at`, `visibilite` ; ajout `code_acces_hash`) |
| **Rust DTOs** | `uafricas_backend/src/models/afrolang.rs`, DTO `SallePriveeAPI` ne contient JAMAIS `code_acces_hash` (juste `id`, `titre`, `auteur_id`, `auteur_nom`, `salle_id`, `created_at`, `archivee_at`, `session_en_cours: bool`) |
| **TS types** | `uafricas_frontend/app/composables/useAfrolang.ts`, `SallePriveeAPI` aligné, `SallePriveeCreatePayload { titre, description?, code_acces }`, `SallePriveeJoinPayload { code_acces }` |

Aucun mock concerné (la feature s'appuie sur la BDD réelle).
