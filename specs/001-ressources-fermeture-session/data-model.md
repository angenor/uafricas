# Phase 1 — Data Model

**Feature**: 001-ressources-fermeture-session
**Date**: 2026-05-24
**Schema cible** : `afrolang` (existant)
**Fichier SQL** : `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` (édité, pas de nouveau fichier — la feature reste cohérente avec le bounded-context Afrolang).

---

## 1. Nouveaux types (enums)

```sql
-- Type d'une ressource contribuée par la communauté.
CREATE TYPE afrolang.type_ressource_contribuee AS ENUM (
    'document',         -- PDF / DOC / DOCX / ODT téléversé
    'video_youtube',    -- URL YouTube + ID extrait
    'accompagnateur',   -- Recommandation d'un membre tiers
    'lien_web'          -- Lien web complémentaire
);

-- Cycle de vie d'une recommandation d'accompagnateur.
CREATE TYPE afrolang.statut_accompagnateur AS ENUM (
    'en_attente',   -- créé par l'auteur, en attente de la personne recommandée
    'acceptee',     -- visible publiquement
    'refusee',      -- fermé sans visibilité publique
    'retiree'       -- accepté puis retiré par la personne recommandée
);

-- Type d'évènement de modération administrative d'une salle.
CREATE TYPE afrolang.type_evenement_moderation AS ENUM (
    'fermeture_admin',
    'reactivation_admin'
);
```

## 2. Extension `afrolang.salle` — état de désactivation administrative

```sql
ALTER TABLE afrolang.salle
    ADD COLUMN desactivee_admin_at      TIMESTAMPTZ,
    ADD COLUMN desactivee_par           UUID,           -- [xref] iam.utilisateur
    ADD COLUMN motif_desactivation      TEXT,
    ADD COLUMN reactivee_at             TIMESTAMPTZ,    -- snapshot de la dernière réactivation
    ADD COLUMN reactivee_par            UUID,           -- [xref] iam.utilisateur
    ADD COLUMN commentaire_reactivation TEXT,
    ADD CONSTRAINT ck_salle_desactivation_coherente CHECK (
        (desactivee_admin_at IS NULL AND desactivee_par IS NULL AND motif_desactivation IS NULL)
        OR
        (desactivee_admin_at IS NOT NULL AND desactivee_par IS NOT NULL AND motif_desactivation IS NOT NULL)
    );

-- Index utilisé par l'annuaire public pour filtrer les salles actives
CREATE INDEX idx_afrolang_salle_active
    ON afrolang.salle(id)
    WHERE desactivee_admin_at IS NULL AND deleted_at IS NULL;
```

**Sémantique** : `desactivee_admin_at IS NOT NULL` → salle gelée. La fermeture admin met les 3 colonnes `desactivee_*` ; la réactivation met `reactivee_*` ET remet `desactivee_admin_at`, `desactivee_par`, `motif_desactivation` à NULL (pour permettre une refermeture ultérieure tracée distinctement). L'historique reste intégralement dans `evenement_moderation_salle`.

## 3. Nouvelle table `afrolang.ressource_contribuee`

```sql
CREATE TABLE afrolang.ressource_contribuee (
    id                       UUID PRIMARY KEY DEFAULT uuid_generate_v4(),

    -- Rattachement (FR-007)
    salle_id                 UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    session_origine_id       UUID         REFERENCES afrolang.session(id) ON DELETE SET NULL, -- info seulement
    auteur_id                UUID         NOT NULL,           -- [xref] iam.utilisateur

    -- Discriminant + champs communs
    type                     afrolang.type_ressource_contribuee NOT NULL,
    titre                    VARCHAR(120) NOT NULL,
    description              VARCHAR(500),

    -- Variant : document
    fichier_url              VARCHAR(500),                   -- requis si type='document'
    fichier_taille_octets    BIGINT,
    fichier_mime             VARCHAR(120),

    -- Variant : video_youtube
    video_url                VARCHAR(500),                   -- requis si type='video_youtube'
    video_id_youtube         VARCHAR(20),                    -- 11 chars, extrait à l'insert

    -- Variant : lien_web
    lien_url                 VARCHAR(1000),                  -- requis si type='lien_web'

    -- Variant : accompagnateur
    membre_recommande_id     UUID,                           -- [xref] iam.utilisateur (requis si type='accompagnateur')
    motif_recommandation     VARCHAR(2000),                  -- requis si type='accompagnateur', ≥ 20 chars
    statut_accompagnateur    afrolang.statut_accompagnateur, -- requis si type='accompagnateur', défaut 'en_attente'
    motif_refus              TEXT,                           -- facultatif si statut='refusee'
    reponse_at               TIMESTAMPTZ,                    -- horodatage acceptation/refus/retrait

    -- Cycle de vie
    created_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at               TIMESTAMPTZ,                    -- soft-delete (auteur ou admin)
    supprime_par             UUID,                           -- [xref] iam.utilisateur

    -- Cohérence variant ↔ champs
    CONSTRAINT ck_ressource_contribuee_type CHECK (
        (type = 'document'        AND fichier_url IS NOT NULL AND video_url IS NULL AND lien_url IS NULL AND membre_recommande_id IS NULL)
     OR (type = 'video_youtube'   AND video_url IS NOT NULL AND video_id_youtube IS NOT NULL AND fichier_url IS NULL AND lien_url IS NULL AND membre_recommande_id IS NULL)
     OR (type = 'lien_web'        AND lien_url IS NOT NULL AND fichier_url IS NULL AND video_url IS NULL AND membre_recommande_id IS NULL)
     OR (type = 'accompagnateur'  AND membre_recommande_id IS NOT NULL AND statut_accompagnateur IS NOT NULL
                                  AND motif_recommandation IS NOT NULL AND char_length(motif_recommandation) >= 20
                                  AND fichier_url IS NULL AND video_url IS NULL AND lien_url IS NULL)
    ),
    CONSTRAINT ck_ressource_accompagnateur_pas_soi CHECK (
        type <> 'accompagnateur' OR membre_recommande_id <> auteur_id
    )
);

-- Index lecture liste par salle (publique et admin)
CREATE INDEX idx_afrolang_ressource_contribuee_salle
    ON afrolang.ressource_contribuee(salle_id, created_at DESC)
    WHERE deleted_at IS NULL;

-- Index rate-limit (10 / utilisateur / salle / 24h glissantes)
CREATE INDEX idx_afrolang_ressource_contribuee_rate_limit
    ON afrolang.ressource_contribuee(auteur_id, salle_id, created_at)
    WHERE deleted_at IS NULL;

-- Index pour la liste "mes recommandations reçues" (boîte de réception accompagnateur)
CREATE INDEX idx_afrolang_ressource_recommandations_recues
    ON afrolang.ressource_contribuee(membre_recommande_id, statut_accompagnateur)
    WHERE type = 'accompagnateur' AND deleted_at IS NULL;

-- Index recherche par session d'origine (analytics)
CREATE INDEX idx_afrolang_ressource_contribuee_session_origine
    ON afrolang.ressource_contribuee(session_origine_id)
    WHERE session_origine_id IS NOT NULL;
```

**Transitions accompagnateur** (`statut_accompagnateur`) :
- `en_attente` → `acceptee` (action : personne recommandée accepte) → set `reponse_at = NOW()`.
- `en_attente` → `refusee` (action : personne recommandée refuse, motif facultatif).
- `acceptee` → `retiree` (action : personne recommandée retire son consentement).
- Tout état → soft-delete (`deleted_at IS NOT NULL`) par l'auteur ou un admin (FR-008).

**Visibilité publique** (filtres à appliquer dans la requête GET liste) :
- Pour `type IN ('document', 'video_youtube', 'lien_web')` : toujours visible si `deleted_at IS NULL`.
- Pour `type = 'accompagnateur'` : visible publiquement uniquement si `statut_accompagnateur = 'acceptee'`. Sinon visible uniquement à `auteur_id` et `membre_recommande_id`.

## 4. Nouvelle table `afrolang.acces_salle_privee`

```sql
-- Mémorise les utilisateurs ayant validé le code d'accès d'une salle privée.
-- Sert au contrôle d'accès en lecture des ressources contribuées de la salle
-- (FR-001 option C de la clarification).
CREATE TABLE afrolang.acces_salle_privee (
    id              UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_privee_id UUID         NOT NULL REFERENCES afrolang.salle_privee(id) ON DELETE CASCADE,
    utilisateur_id  UUID         NOT NULL,                       -- [xref] iam.utilisateur
    valide_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    revoque_at      TIMESTAMPTZ                                  -- mis à NOW() lors d'un changement de code_acces_hash
);

-- Au plus un accès actif par (salle, utilisateur)
CREATE UNIQUE INDEX idx_afrolang_acces_unique_actif
    ON afrolang.acces_salle_privee(salle_privee_id, utilisateur_id)
    WHERE revoque_at IS NULL;

-- Lookup rapide « cet utilisateur a-t-il accès à cette salle privée ? »
CREATE INDEX idx_afrolang_acces_lookup
    ON afrolang.acces_salle_privee(utilisateur_id, salle_privee_id)
    WHERE revoque_at IS NULL;
```

**Sémantique** :
- À chaque succès de `POST /salles-privees/{id}/verifier-code` : `INSERT ... ON CONFLICT (salle_privee_id, utilisateur_id) WHERE revoque_at IS NULL DO NOTHING`.
- À chaque `PATCH /salles-privees/{id}/code-acces` : `UPDATE acces_salle_privee SET revoque_at = NOW() WHERE salle_privee_id = $1 AND revoque_at IS NULL` dans la même transaction que la mise à jour du hash.

## 5. Nouvelle table `afrolang.evenement_moderation_salle`

```sql
-- Historique chronologique des fermetures et réactivations administratives.
-- Append-only ; aucun update, aucun delete (immuable, audit-grade).
CREATE TABLE afrolang.evenement_moderation_salle (
    id                   UUID         PRIMARY KEY DEFAULT uuid_generate_v4(),
    salle_id             UUID         NOT NULL REFERENCES afrolang.salle(id) ON DELETE CASCADE,
    session_concernee_id UUID         REFERENCES afrolang.session(id) ON DELETE SET NULL, -- session live au moment d'une fermeture
    type_action          afrolang.type_evenement_moderation NOT NULL,
    admin_id             UUID         NOT NULL,                       -- [xref] iam.utilisateur
    motif                TEXT,                                         -- 10..1000 chars pour 'fermeture_admin', facultatif pour 'reactivation_admin'
    created_at           TIMESTAMPTZ  NOT NULL DEFAULT NOW(),

    CONSTRAINT ck_moderation_motif_fermeture CHECK (
        (type_action = 'fermeture_admin'   AND motif IS NOT NULL AND char_length(motif) BETWEEN 10 AND 1000)
     OR (type_action = 'reactivation_admin' AND (motif IS NULL OR char_length(motif) <= 1000))
    )
);

CREATE INDEX idx_afrolang_moderation_salle_chrono
    ON afrolang.evenement_moderation_salle(salle_id, created_at DESC);
```

## 6. Représentation Rust (structs)

```rust
// src/models/ressource_contribuee.rs

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "afrolang.type_ressource_contribuee", rename_all = "snake_case")]
pub enum TypeRessourceContribuee {
    Document,
    VideoYoutube,
    Accompagnateur,
    LienWeb,
}

#[derive(sqlx::Type, Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "afrolang.statut_accompagnateur", rename_all = "snake_case")]
pub enum StatutAccompagnateur {
    EnAttente,
    Acceptee,
    Refusee,
    Retiree,
}

#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize)]
pub struct RessourceContribuee {
    pub id: uuid::Uuid,
    pub salle_id: uuid::Uuid,
    pub session_origine_id: Option<uuid::Uuid>,
    pub auteur_id: uuid::Uuid,
    pub r#type: TypeRessourceContribuee,
    pub titre: String,
    pub description: Option<String>,
    pub fichier_url: Option<String>,
    pub fichier_taille_octets: Option<i64>,
    pub fichier_mime: Option<String>,
    pub video_url: Option<String>,
    pub video_id_youtube: Option<String>,
    pub lien_url: Option<String>,
    pub membre_recommande_id: Option<uuid::Uuid>,
    pub motif_recommandation: Option<String>,
    pub statut_accompagnateur: Option<StatutAccompagnateur>,
    pub motif_refus: Option<String>,
    pub reponse_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub supprime_par: Option<uuid::Uuid>,
}

// DTO réponse publique (champs internes admin masqués)
#[derive(Debug, Clone, serde::Serialize)]
pub struct RessourceContribueeResponse {
    pub id: uuid::Uuid,
    pub r#type: TypeRessourceContribuee,
    pub titre: String,
    pub description: Option<String>,
    pub auteur: AuteurLight,                       // {id, nom, prenom, avatar_url}
    pub session_origine_id: Option<uuid::Uuid>,
    pub fichier_url: Option<String>,
    pub fichier_taille_octets: Option<i64>,
    pub video_id_youtube: Option<String>,
    pub video_url: Option<String>,
    pub lien_url: Option<String>,
    pub accompagnateur: Option<AccompagnateurPublicInfo>, // {membre, motif, statut} si type=accompagnateur
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

## 7. Représentation TypeScript (composables / pages)

```ts
// app/composables/useAfrolangRessources.ts (extrait)

export type TypeRessourceContribuee = 'document' | 'video_youtube' | 'accompagnateur' | 'lien_web';
export type StatutAccompagnateur = 'en_attente' | 'acceptee' | 'refusee' | 'retiree';

export interface AuteurLight {
  id: string;
  nom: string;
  prenom: string;
  avatar_url: string | null;
}

export interface AccompagnateurPublicInfo {
  membre: AuteurLight;
  motif: string;
  statut: StatutAccompagnateur;
}

export interface RessourceContribueeAPI {
  id: string;
  type: TypeRessourceContribuee;
  titre: string;
  description: string | null;
  auteur: AuteurLight;
  session_origine_id: string | null;
  fichier_url: string | null;
  fichier_taille_octets: number | null;
  video_id_youtube: string | null;
  video_url: string | null;
  lien_url: string | null;
  accompagnateur: AccompagnateurPublicInfo | null;
  created_at: string; // ISO 8601
}
```

## 8. Récapitulatif des modifications SQL

| Élément | Type | Fichier |
|---|---|---|
| `type_ressource_contribuee` | ENUM | `08b_afrolang.sql` |
| `statut_accompagnateur` | ENUM | `08b_afrolang.sql` |
| `type_evenement_moderation` | ENUM | `08b_afrolang.sql` |
| `salle` (+6 colonnes + 1 CHECK + 1 index) | ALTER | `08b_afrolang.sql` |
| `ressource_contribuee` | TABLE | `08b_afrolang.sql` |
| `acces_salle_privee` | TABLE | `08b_afrolang.sql` |
| `evenement_moderation_salle` | TABLE | `08b_afrolang.sql` |

**Total** : 3 enums + 3 nouvelles tables + 6 colonnes ajoutées à `salle` + 6 indexes (4 sur ressource_contribuee + 1 sur acces_salle_privee + 1 sur salle + 1 sur evenement_moderation_salle).
