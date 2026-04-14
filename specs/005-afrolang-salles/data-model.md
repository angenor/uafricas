# Phase 1 — Data Model

**Feature**: Afrolang — Ajustements salles publiques et privées
**Schema cible**: `afrolang` (étendu) + FK vers `country_profile.groupe_ethnique`
**Principe constitution III**: le SQL est la source de vérité ; les structs Rust et interfaces TS reflètent fidèlement ce modèle.

---

## Vue d'ensemble des changements

### Tables existantes modifiées

| Table | Modification |
|-------|--------------|
| `afrolang.salle` | + `groupe_ethnique_id UUID NOT NULL` FK, - `moderateur_id` supprimée (remplacée par `salle_moderateur`), + `langue_code VARCHAR(40)`, + `alphabet TEXT`, + `dictionnaire_url VARCHAR(500)`, index unique partiel sur `groupe_ethnique_id WHERE actif=TRUE AND deleted_at IS NULL` |
| `afrolang.salle_privee` | + `motif motif_salle_privee NOT NULL`, + `declaration_adulte_at TIMESTAMPTZ NOT NULL`, + `visibilite visibilite_salle_privee NOT NULL DEFAULT 'fermee'`, + `archivee_at TIMESTAMPTZ NULL`, + `deleted_at TIMESTAMPTZ NULL`, + index unique partiel `(salle_id, cree_par) WHERE archivee_at IS NULL AND deleted_at IS NULL` |

### Nouvelles tables

1. `afrolang.proposition_salle` — propositions de salles publiques à valider.
2. `afrolang.salle_moderateur` — affectation many-to-many modérateurs Afrolang attitrés ↔ salle publique.
3. `afrolang.salle_privee_adhesion` — demandes d'adhésion, invitations et abonnés d'une salle privée.
4. `afrolang.ressource_salle` — ressources pédagogiques (fichiers internes + liens externes modérés).
5. `afrolang.message_session` — messages de la messagerie instantanée écrite.

### Nouveaux types (enums)

- `afrolang.etat_proposition` ∈ {`en_attente`, `approuvee`, `refusee`}
- `afrolang.motif_salle_privee` ∈ {`apprentissage_enfants`, `reseautage_adulte`, `echanges_groupe`}
- `afrolang.visibilite_salle_privee` ∈ {`fermee`, `visible`}
- `afrolang.type_adhesion` ∈ {`demande`, `invitation`, `abonne`}
- `afrolang.etat_adhesion` ∈ {`en_attente`, `acceptee`, `refusee`, `groupe_complet`}
- `afrolang.type_ressource` ∈ {`fichier`, `lien_externe`}
- `afrolang.etat_ressource` ∈ {`publiee`, `en_attente_validation`, `refusee`}

### Ajout global

- Dossier d'upload `./uploads/afrolang/ressources/` servi via `actix-files` sur `/uploads/afrolang/ressources/...`.

---

## DDL complet (référence SQL)

```sql
-- ── Enums ajoutés ─────────────────────────────────────────────────────────
CREATE TYPE afrolang.etat_proposition AS ENUM (
    'en_attente',
    'approuvee',
    'refusee'
);

CREATE TYPE afrolang.motif_salle_privee AS ENUM (
    'apprentissage_enfants',
    'reseautage_adulte',
    'echanges_groupe'
);

CREATE TYPE afrolang.visibilite_salle_privee AS ENUM (
    'fermee',
    'visible'
);

CREATE TYPE afrolang.type_adhesion AS ENUM (
    'demande',
    'invitation',
    'abonne'
);

CREATE TYPE afrolang.etat_adhesion AS ENUM (
    'en_attente',
    'acceptee',
    'refusee',
    'groupe_complet'
);

CREATE TYPE afrolang.type_ressource AS ENUM (
    'fichier',
    'lien_externe'
);

CREATE TYPE afrolang.etat_ressource AS ENUM (
    'publiee',
    'en_attente_validation',
    'refusee'
);


-- ── afrolang.salle : modifications ───────────────────────────────────────
ALTER TABLE afrolang.salle
    ADD COLUMN groupe_ethnique_id UUID NOT NULL REFERENCES country_profile.groupe_ethnique(id) ON DELETE RESTRICT,
    ADD COLUMN langue_code VARCHAR(40),
    ADD COLUMN alphabet TEXT,
    ADD COLUMN dictionnaire_url VARCHAR(500),
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE afrolang.salle DROP COLUMN moderateur_id;  -- remplacé par salle_moderateur

CREATE UNIQUE INDEX idx_afrolang_salle_groupe_unique
    ON afrolang.salle(groupe_ethnique_id)
    WHERE actif = TRUE AND deleted_at IS NULL;

CREATE INDEX idx_afrolang_salle_groupe ON afrolang.salle(groupe_ethnique_id);


-- ── afrolang.salle_privee : modifications ────────────────────────────────
ALTER TABLE afrolang.salle_privee
    ADD COLUMN motif afrolang.motif_salle_privee NOT NULL,
    ADD COLUMN declaration_adulte_at TIMESTAMPTZ NOT NULL,
    ADD COLUMN visibilite afrolang.visibilite_salle_privee NOT NULL DEFAULT 'fermee',
    ADD COLUMN archivee_at TIMESTAMPTZ,
    ADD COLUMN deleted_at TIMESTAMPTZ;

CREATE UNIQUE INDEX idx_afrolang_privee_unique_par_salle
    ON afrolang.salle_privee(salle_id, cree_par)
    WHERE archivee_at IS NULL AND deleted_at IS NULL;

CREATE INDEX idx_afrolang_privee_visibilite
    ON afrolang.salle_privee(salle_id, visibilite)
    WHERE archivee_at IS NULL AND deleted_at IS NULL;


-- ── afrolang.proposition_salle : nouvelle table ──────────────────────────
CREATE TABLE afrolang.proposition_salle (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nom_groupe_ethnique     VARCHAR(250) NOT NULL,
    pays_id                 UUID,                          -- [xref] shared.pays (facultatif)
    groupe_ethnique_id      UUID,                          -- [xref] country_profile.groupe_ethnique, si le proposant a pu pointer un existant
    langue_cible            VARCHAR(100),
    description             TEXT,
    etat                    afrolang.etat_proposition NOT NULL DEFAULT 'en_attente',
    motif_refus             TEXT,
    salle_id_creee          UUID REFERENCES afrolang.salle(id) ON DELETE SET NULL,  -- rempli à l'approbation
    propose_par             UUID NOT NULL,                 -- [xref] iam.utilisateur
    decide_par              UUID,                          -- [xref] iam.utilisateur (admin)
    decide_at               TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_afrolang_proposition_etat
    ON afrolang.proposition_salle(etat) WHERE deleted_at IS NULL;

CREATE INDEX idx_afrolang_proposition_auteur
    ON afrolang.proposition_salle(propose_par) WHERE deleted_at IS NULL;


-- ── afrolang.salle_moderateur : affectation attitrée ─────────────────────
CREATE TABLE afrolang.salle_moderateur (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id          UUID NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    utilisateur_id    UUID NOT NULL,                       -- [xref] iam.utilisateur
    designe_par       UUID NOT NULL,                       -- [xref] iam.utilisateur (admin)
    designe_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    disponibilite     TEXT,                                -- libre-texte (horaires, fuseau)
    actif             BOOLEAN NOT NULL DEFAULT TRUE,       -- retrait = actif=FALSE (conserve historique)
    retire_at         TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (salle_id, utilisateur_id)
);

CREATE INDEX idx_afrolang_moderateur_salle
    ON afrolang.salle_moderateur(salle_id) WHERE actif = TRUE;

CREATE INDEX idx_afrolang_moderateur_user
    ON afrolang.salle_moderateur(utilisateur_id) WHERE actif = TRUE;


-- ── afrolang.salle_privee_adhesion : demandes / invitations / abonnés ─────
CREATE TABLE afrolang.salle_privee_adhesion (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id   UUID NOT NULL REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    utilisateur_id    UUID NOT NULL,                       -- [xref] iam.utilisateur (destinataire/demandeur)
    type              afrolang.type_adhesion NOT NULL,
    etat              afrolang.etat_adhesion NOT NULL DEFAULT 'en_attente',
    initiateur_id     UUID NOT NULL,                       -- qui a déclenché (le demandeur pour type=demande, le créateur pour type=invitation)
    decideur_id       UUID,                                -- qui a tranché (créateur pour demandes, invité pour invitations)
    decided_at        TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at        TIMESTAMPTZ,
    UNIQUE (salle_privee_id, utilisateur_id)
);

CREATE INDEX idx_afrolang_adhesion_salle
    ON afrolang.salle_privee_adhesion(salle_privee_id) WHERE deleted_at IS NULL;

CREATE INDEX idx_afrolang_adhesion_user
    ON afrolang.salle_privee_adhesion(utilisateur_id) WHERE deleted_at IS NULL;

CREATE INDEX idx_afrolang_adhesion_attente
    ON afrolang.salle_privee_adhesion(salle_privee_id, etat)
    WHERE etat = 'en_attente' AND deleted_at IS NULL;


-- ── afrolang.ressource_salle : rubrique Ressources ────────────────────────
CREATE TABLE afrolang.ressource_salle (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id          UUID NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    titre             VARCHAR(350) NOT NULL,
    description       TEXT,
    type              afrolang.type_ressource NOT NULL,
    fichier_url       VARCHAR(500),                        -- requis si type=fichier
    lien_url          VARCHAR(1000),                       -- requis si type=lien_externe
    etat              afrolang.etat_ressource NOT NULL DEFAULT 'publiee',
    motif_refus       TEXT,
    ajoute_par        UUID NOT NULL,                       -- [xref] iam.utilisateur
    valide_par        UUID,                                -- [xref] iam.utilisateur (modérateur Afrolang ou admin)
    valide_at         TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at        TIMESTAMPTZ,
    CONSTRAINT ck_ressource_url_coherence CHECK (
        (type = 'fichier'      AND fichier_url IS NOT NULL AND lien_url IS NULL) OR
        (type = 'lien_externe' AND lien_url    IS NOT NULL AND fichier_url IS NULL)
    ),
    CONSTRAINT ck_ressource_etat_initial CHECK (
        (type = 'fichier'      AND etat IN ('publiee', 'refusee')) OR
        (type = 'lien_externe')  -- peut passer par tous les états
    )
);

CREATE INDEX idx_afrolang_ressource_salle
    ON afrolang.ressource_salle(salle_id) WHERE deleted_at IS NULL;

CREATE INDEX idx_afrolang_ressource_attente
    ON afrolang.ressource_salle(etat)
    WHERE etat = 'en_attente_validation' AND deleted_at IS NULL;


-- ── afrolang.message_session : messagerie instantanée écrite ─────────────
CREATE TABLE afrolang.message_session (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id      UUID NOT NULL REFERENCES afrolang.session(id) ON DELETE CASCADE,
    auteur_id       UUID NOT NULL,                         -- [xref] iam.utilisateur
    contenu         TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_afrolang_message_session
    ON afrolang.message_session(session_id, created_at)
    WHERE deleted_at IS NULL;
```

---

## Entités détaillées

### 1. `afrolang.salle` (modifiée)

| Champ | Type | Règles |
|-------|------|--------|
| `id` | UUID PK | auto |
| `titre` | VARCHAR(350) | NOT NULL |
| `slug` | VARCHAR(400) | UNIQUE |
| `description` | TEXT | |
| `image_couverture_url` | VARCHAR(500) | |
| `langue_cible` | VARCHAR(100) | (existant) |
| **`langue_code`** | **VARCHAR(40)** | **nouveau** — ISO ou code métier de la langue |
| **`alphabet`** | **TEXT** | **nouveau** — alphabet affiché dans Ressources |
| **`dictionnaire_url`** | **VARCHAR(500)** | **nouveau** — lien direct vers le dictionnaire intégré (le lien externe validé passe par `ressource_salle`) |
| **`groupe_ethnique_id`** | **UUID NOT NULL FK** | **nouveau** — référentiel unique (Décision 1) |
| ~~`moderateur_id`~~ | ~~UUID~~ | **supprimé** — remplacé par `salle_moderateur` |
| `actif` | BOOLEAN | DEFAULT TRUE |
| `cree_par` | UUID NOT NULL | admin créateur |
| `created_at` / `updated_at` | TIMESTAMPTZ | |
| **`deleted_at`** | **TIMESTAMPTZ** | **nouveau** — soft deletion |

**Règles métier** :
- Au plus une salle active par groupe ethnique (index unique partiel).
- La création est réservée aux admins (directe) ou issue d'une proposition validée.

### 2. `afrolang.salle_privee` (modifiée)

| Champ | Type | Règles |
|-------|------|--------|
| (champs existants conservés) | | |
| **`motif`** | **enum `motif_salle_privee`** | **NOT NULL** |
| **`declaration_adulte_at`** | **TIMESTAMPTZ** | **NOT NULL** — capture de la case cochée (FR-033) |
| **`visibilite`** | **enum `visibilite_salle_privee`** | **DEFAULT `fermee`** |
| **`archivee_at`** | **TIMESTAMPTZ NULL** | archivage automatique si créateur supprimé (FR-034) ou en cascade lors de la désactivation de la salle publique de rattachement (Edge Case) |
| **`deleted_at`** | **TIMESTAMPTZ NULL** | soft deletion explicite |

**Modification de la FK `salle_privee.salle_id`** : la contrainte existante `ON DELETE CASCADE` est remplacée par `ON DELETE RESTRICT` pour empêcher la perte silencieuse de salles privées (Edge Case « salle publique désactivée »). L'archivage en cascade est piloté par un handler dédié `archiver_salle_publique_avec_cascade` (cf. contracts/api-admin-afrolang.md).

```sql
ALTER TABLE afrolang.salle_privee
    DROP CONSTRAINT salle_privee_salle_id_fkey,
    ADD CONSTRAINT salle_privee_salle_id_fkey
        FOREIGN KEY (salle_id) REFERENCES afrolang.salle(id) ON DELETE RESTRICT;
```

**Contrainte métier (index unique partiel)** :
```sql
UNIQUE (salle_id, cree_par) WHERE archivee_at IS NULL AND deleted_at IS NULL
```
→ garantit « 1 salle privée active par membre par salle publique » (FR-035, SC-010).

**Transitions de `archivee_at`** : NULL → timestamp lors de `UPDATE ... SET archivee_at = NOW()` (irréversible, sauf intervention admin).

### 3. `afrolang.proposition_salle` (nouvelle)

| Champ | Type |
|-------|------|
| `id` | UUID PK |
| `nom_groupe_ethnique` | VARCHAR(250) NOT NULL |
| `pays_id` | UUID FK nullable |
| `groupe_ethnique_id` | UUID FK nullable (si pointage déjà existant) |
| `langue_cible` | VARCHAR(100) |
| `description` | TEXT |
| `etat` | enum NOT NULL DEFAULT `en_attente` |
| `motif_refus` | TEXT (si `etat=refusee`) |
| `salle_id_creee` | UUID FK nullable (rempli si `etat=approuvee`) |
| `propose_par` | UUID NOT NULL |
| `decide_par` | UUID nullable |
| `decide_at` | TIMESTAMPTZ nullable |
| `created_at` / `updated_at` / `deleted_at` | TIMESTAMPTZ |

**Transitions** : `en_attente` → `approuvee` (crée une `salle` et remplit `salle_id_creee`) | `refusee` (avec `motif_refus`). Déclenche notification au proposant (FR-031).

**Détection doublon** : avant insertion, vérifier que :
```sql
NOT EXISTS (
  SELECT 1 FROM afrolang.salle s
  JOIN country_profile.groupe_ethnique ge ON ge.id = s.groupe_ethnique_id
  WHERE lower(unaccent(ge.nom)) = lower(unaccent($1)) AND s.deleted_at IS NULL
)
AND NOT EXISTS (
  SELECT 1 FROM afrolang.proposition_salle p
  WHERE lower(unaccent(p.nom_groupe_ethnique)) = lower(unaccent($1))
    AND p.etat = 'en_attente' AND p.deleted_at IS NULL
)
```
(nécessite l'extension `unaccent` ou une fonction équivalente).

### 4. `afrolang.salle_moderateur` (nouvelle)

Table d'affectation N-N entre `salle` et `utilisateur`. Retirer = `actif=FALSE` + `retire_at` (conserve l'historique pour audit).

| Champ | Type |
|-------|------|
| `id` | UUID PK |
| `salle_id` | UUID FK → salle |
| `utilisateur_id` | UUID FK |
| `designe_par` | UUID FK (admin) |
| `designe_at` | TIMESTAMPTZ |
| `disponibilite` | TEXT |
| `actif` | BOOLEAN DEFAULT TRUE |
| `retire_at` | TIMESTAMPTZ nullable |
| `created_at` / `updated_at` | TIMESTAMPTZ |
| UNIQUE `(salle_id, utilisateur_id)` | |

### 5. `afrolang.salle_privee_adhesion` (nouvelle)

Modèle unifié pour demandes, invitations et abonnés confirmés.

| Champ | Type |
|-------|------|
| `id` | UUID PK |
| `salle_privee_id` | UUID FK |
| `utilisateur_id` | UUID FK |
| `type` | enum `type_adhesion` |
| `etat` | enum `etat_adhesion` |
| `initiateur_id` | UUID FK |
| `decideur_id` | UUID FK nullable |
| `decided_at` | TIMESTAMPTZ nullable |
| `created_at` / `updated_at` / `deleted_at` | TIMESTAMPTZ |
| UNIQUE `(salle_privee_id, utilisateur_id)` | |

**Règles de transition** :

```
┌─ type=demande  (initiateur = utilisateur, decideur = créateur de la salle) ──┐
│   en_attente → acceptee   (UPDATE type='abonne', etat='acceptee')             │
│   en_attente → refusee                                                        │
│   en_attente → groupe_complet (auto si max_participants atteint)              │
└──────────────────────────────────────────────────────────────────────────────┘

┌─ type=invitation (initiateur = créateur, decideur = utilisateur invité) ─────┐
│   en_attente → acceptee   (UPDATE type='abonne', etat='acceptee')             │
│   en_attente → refusee                                                        │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Règle d'atomicité de la limite de participants** (évite conditions de course — SC-006) :
```sql
BEGIN;
SELECT max_participants FROM salle_privee WHERE id = $1 FOR UPDATE;
-- comparer avec COUNT(*) des abonne actifs
-- si < max : accepter ; sinon : etat='groupe_complet'
COMMIT;
```

### 6. `afrolang.ressource_salle` (nouvelle)

| Champ | Type |
|-------|------|
| `id` | UUID PK |
| `salle_id` | UUID FK |
| `titre` | VARCHAR(350) NOT NULL |
| `description` | TEXT |
| `type` | enum `type_ressource` |
| `fichier_url` | VARCHAR(500) nullable |
| `lien_url` | VARCHAR(1000) nullable |
| `etat` | enum `etat_ressource` DEFAULT `publiee` |
| `motif_refus` | TEXT |
| `ajoute_par` | UUID FK |
| `valide_par` | UUID FK nullable (modérateur Afrolang attitré ou admin) |
| `valide_at` | TIMESTAMPTZ nullable |
| `created_at` / `updated_at` / `deleted_at` | TIMESTAMPTZ |

**CHECK** :
- `type=fichier` ⇒ `fichier_url NOT NULL` et `lien_url NULL` et `etat ∈ {publiee, refusee}` (fichier publié directement ; possibilité de refus admin a posteriori).
- `type=lien_externe` ⇒ `lien_url NOT NULL` et `fichier_url NULL` ; `etat` initial = `en_attente_validation` (règle applicative).

**Transitions** (lien externe) : `en_attente_validation` → `publiee` (admin ou modérateur Afrolang attitré remplit `valide_par` / `valide_at`) | `refusee` (avec `motif_refus`).

### 7. `afrolang.message_session` (nouvelle)

| Champ | Type |
|-------|------|
| `id` | UUID PK |
| `session_id` | UUID FK |
| `auteur_id` | UUID FK |
| `contenu` | TEXT NOT NULL |
| `created_at` / `deleted_at` | TIMESTAMPTZ |

**Validation applicative** : longueur max 4000 caractères côté serveur ; trim ; refus si vide après trim.

---

## Relations consolidées

```
country_profile.groupe_ethnique ──1─┐
                                    │
                                    ▼
iam.utilisateur ──*──salle_moderateur──*── afrolang.salle ──*── salle_privee
                                           │               │
                                           │               ├── salle_privee_adhesion ──*── iam.utilisateur
                                           │               │
                                           ▼               ▼
                                      ressource_salle    session (existante)
                                                           │
                                                           ├── session_participant (existante)
                                                           ├── tableau_blanc (existante)
                                                           └── message_session (nouvelle)

afrolang.proposition_salle ──(après approbation)──▶ afrolang.salle
```

---

## État machine agrégé

### Proposition de salle publique
```
en_attente ─► approuvee ─► (salle créée, salle_id_creee rempli)
           ╲─► refusee   (motif_refus rempli)
```

### Salle privée (lifecycle)
```
actif (archivee_at=NULL, deleted_at=NULL)
  │
  ├─► archivée  (archivee_at=NOW())   — créateur désactivé ou supprimé (FR-034)
  └─► supprimée (deleted_at=NOW())    — soft delete admin/créateur
```

### Adhésion à une salle privée
```
en_attente ─► acceptee          (type devient abonne)
           ├─► refusee
           └─► groupe_complet   (refus automatique pour demande quand max atteint)
```

### Ressource (lien externe)
```
en_attente_validation ─► publiee
                      ╲─► refusee
```

---

## Mapping vers les structs Rust (résumé)

Les structs existants `SalleRow`, `SallePriveeRow`, `SessionRow`, etc. dans `uafricas_backend/src/models/afrolang.rs` seront étendus en respectant les conventions UAfricas :

- Nouveaux champs ajoutés en snake_case français.
- Nouveaux structs : `PropositionSalleRow`, `SalleModerateurRow`, `SallePriveeAdhesionRow`, `RessourceSalleRow`, `MessageSessionRow`.
- Pour chaque enum PostgreSQL : un enum Rust `#[derive(sqlx::Type, serde::Serialize, serde::Deserialize)]` avec `#[sqlx(type_name = "afrolang.xxx", rename_all = "snake_case")]`.
- DTO Request / Response séparés (pattern UAfricas) pour chaque endpoint, avec validation des champs via `serde` + vérifs applicatives serveur (longueurs, cohérence motif/déclaration adulte, URL valide, etc.).

## Mapping vers les interfaces TypeScript (frontend)

Dans `uafricas_frontend/app/mocks/afrolang.ts` :

- Mise à jour de l'interface `Salle` : ajout de `groupeEthniqueId`, `langueCode`, `alphabet`, `dictionnaireUrl`, retrait de `moderateurId` (remplacé par `moderateursAttitres: ModerateurAttitre[]`).
- Mise à jour de l'interface `SallePrivee` : ajout de `motif`, `declarationAdulteAt`, `visibilite`, `archiveeAt`.
- Nouvelles interfaces : `PropositionSalle`, `ModerateurAttitre`, `AdhesionSallePrivee`, `RessourceSalle`, `MessageSession`.
- Enums TS literals correspondants (`'apprentissage_enfants' | 'reseautage_adulte' | 'echanges_groupe'`, etc.) pour rester 1:1 avec l'enum SQL.

---

## Statut Phase 1 (data model)

**Complet** — schéma SQL spécifié, contraintes métier exprimées par index partiels + CHECK + transactions, mappings Rust/TS documentés.
