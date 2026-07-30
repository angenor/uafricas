# Phase 1 — Modèle de données

**Feature** : `007-engagement-points-badges` | **Date** : 2026-07-29
**Schéma** : `engagement` (existant — étendu, jamais recréé)

Trois migrations idempotentes, à ajouter à `uafricas_backend/doc/bd/schema.sql` **après** `35b_engagement_mise_en_avant.sql` :

```sql
\ir schemas/35c_engagement_categories_bareme.sql
\ir schemas/35d_engagement_badges.sql
\ir schemas/35e_engagement_partage_externe.sql
```

> Rappel de convention : noms de **fichiers** sans accent ni caractère spécial (`[a-z0-9_-]`), contenu SQL en français accentué. Toutes les migrations sont rejouables (`IF NOT EXISTS`, `ON CONFLICT DO NOTHING`) car appliquées à la main en production via SSH + psql.

---

## Vue d'ensemble

| Table | État | Rôle |
|---|---|---|
| `engagement.compte` | existante, **inchangée** | Solde, solde mensuel, réputation, niveau courant |
| `engagement.mouvement_points` | existante, **+1 colonne** | Journal append-only ; porte désormais la catégorie du mouvement |
| `engagement.regle_points` | existante, **+2 colonnes** | Barème ; gagne catégorie et seuil déclencheur |
| `engagement.palier_popularite` | existante, **+1 colonne** | Paliers de likes ; désormais restreignables à une famille |
| `engagement.niveau` | existante, **+1 contrainte** | Seuils de statut ; `seuil_min` rendu unique |
| `engagement.mise_en_avant` | existante, **inchangée** | Distinction éditoriale (+5), livrée par `35b` |
| `engagement.categorie_points` | **NEW** | Catégories de ventilation |
| `engagement.badge` | **NEW** | Définition d'un badge/succès |
| `engagement.badge_obtenu` | **NEW** | Badges détenus par un membre |
| `engagement.partage_externe` | **NEW** | Log des partages vers les réseaux sociaux |

---

## Migration `35c_engagement_categories_bareme.sql`

### 1. `engagement.categorie_points` (NEW)

| Colonne | Type | Contraintes | Sens |
|---|---|---|---|
| `id` | UUID | PK, `DEFAULT uuid_generate_v4()` | |
| `code` | VARCHAR(30) | NOT NULL, UNIQUE | Clé stable (`contributions`, `medias`…) |
| `libelle` | VARCHAR(80) | NOT NULL | Affiché au membre |
| `description` | TEXT | NULL | Texte pédagogique de l'état vide (FR-015) |
| `ordre` | SMALLINT | NOT NULL DEFAULT 0 | Ordre d'affichage |
| `couleur` | VARCHAR(20) | NULL | Jeton de couleur front |
| `icone` | VARCHAR(40) | NULL | Nom FontAwesome |
| `actif` | BOOLEAN | NOT NULL DEFAULT TRUE | Désactivation (pas de suppression si utilisée) |
| `created_at` / `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | |

**Règles de validation** :
- Suppression **refusée** si au moins une `regle_points` la référence (FR-004) → contrôle applicatif renvoyant 409, doublé de `ON DELETE RESTRICT` sur la FK.
- `code` immuable après création (le front l'utilise pour ses icônes ; le libellé, lui, est libre).

**Seed** (6 catégories) :

| `code` | `libelle` | `ordre` | `icone` |
|---|---|---|---|
| `contributions` | Contributions | 1 | `pen-nib` |
| `popularite` | Popularité | 2 | `heart` |
| `medias` | Médias (télé & radio) | 3 | `tv` |
| `factcheck` | Vérification de faits | 4 | `magnifying-glass-check` |
| `partages` | Partages | 5 | `share-nodes` |
| `ajustements` | Ajustements | 6 | `sliders` |

### 2. `engagement.regle_points` — +2 colonnes

| Colonne | Type | Sens |
|---|---|---|
| `categorie_id` | UUID NULL REFERENCES `categorie_points(id)` ON DELETE RESTRICT | Catégorie de ventilation |
| `seuil_declencheur` | INTEGER NULL | Nombre d'occurrences distinctes nécessaires avant crédit (R10). `NULL` = crédit immédiat |

**Rattachement des 6 règles seedées** par `UPDATE … WHERE type_action = …` :
`contribution_validee` et `contribution_mise_en_avant` → `contributions` · `factcheck_valide` et `factcheck_faux` → `factcheck` · `popularite_palier` → `popularite` · `ajustement_admin` → `ajustements`.

**Seed des 4 règles nouvelles** (`ON CONFLICT (type_action) DO NOTHING`) :

| `type_action` | `points` | `reputation_delta` | `plafond_journalier` | `seuil_declencheur` | Catégorie |
|---|---:|---:|---:|---:|---|
| `proposition_media_validee` | 5 | 1 | NULL | NULL | `medias` |
| `media_a_la_une` | 8 | 1 | NULL | NULL | `medias` |
| `animation_support_acceptee` | 15 | 2 | NULL | NULL | `medias` |
| `partage_externe_5reseaux` | 10 | 0 | **30** | **5** | `partages` |

> ⚠️ **Le plafond est exprimé en points, pas en occurrences** : `appliquer` compare `SUM(points)` du jour au `plafond_journalier`. « 3 bonus de partage par jour » se paramètre donc à **30**, pas à 3. À rappeler dans l'aide de l'écran back-office, sinon le paramétrage sera faux au premier ajustement.

### 3. `engagement.mouvement_points` — +1 colonne

| Colonne | Type | Sens |
|---|---|---|
| `categorie_id` | UUID NULL REFERENCES `categorie_points(id)` ON DELETE SET NULL | Catégorie **figée au moment du mouvement** (R1) |

- Index : `idx_mouvement_categorie (utilisateur_id, categorie_id)` — sert la ventilation et le filtre de l'historique.
- **Rattrapage unique** de l'existant :
  ```sql
  UPDATE engagement.mouvement_points m
     SET categorie_id = r.categorie_id
    FROM engagement.regle_points r
   WHERE r.type_action = m.type_action AND m.categorie_id IS NULL;
  ```
- Les lignes restées sans catégorie s'affichent sous « Autres » côté membre (aucune erreur, aucune ligne masquée).

### 4. `engagement.palier_popularite` — +1 colonne, unicité remplacée

| Colonne | Type | Sens |
|---|---|---|
| `type_objet` | VARCHAR(40) NULL | Famille de contenus visée. `NULL` = palier global |

```sql
ALTER TABLE engagement.palier_popularite DROP CONSTRAINT IF EXISTS palier_popularite_seuil_likes_key;
CREATE UNIQUE INDEX IF NOT EXISTS idx_uq_palier_seuil_famille
    ON engagement.palier_popularite (seuil_likes, type_objet) NULLS NOT DISTINCT;
```

`NULLS NOT DISTINCT` (PostgreSQL 15+) est indispensable : sans lui, deux paliers globaux de même seuil coexisteraient (deux `NULL` étant distincts par défaut), et le même seuil créditerait deux fois.

**Sémantique de résolution** (R4) : s'il existe au moins un palier actif pour la famille, ces paliers **remplacent** les globaux ; sinon les globaux s'appliquent.

### 5. `engagement.niveau` — +1 contrainte

```sql
CREATE UNIQUE INDEX IF NOT EXISTS idx_uq_niveau_seuil ON engagement.niveau (seuil_min);
```

Interdit deux niveaux au même seuil (edge case « niveaux mal ordonnés »). `ordre` est recalculé applicativement d'après `seuil_min` croissant à chaque mutation, de sorte que `ordre` et `seuil_min` ne puissent jamais se contredire. Aucune FK n'est ajoutée depuis `compte.niveau_code` (R5).

---

## Migration `35d_engagement_badges.sql`

### 1. Enums

```sql
CREATE TYPE engagement.type_condition_badge AS ENUM (
    'actions_comptees', 'points_categorie', 'solde_total', 'niveau_atteint', 'palier_popularite'
);
CREATE TYPE engagement.origine_badge AS ENUM ('automatique', 'manuel', 'retroactif');
```

*(encapsulés dans un `DO $$ … EXCEPTION WHEN duplicate_object THEN NULL; END $$;` pour rester rejouables)*

### 2. `engagement.badge` (NEW)

| Colonne | Type | Contraintes | Sens |
|---|---|---|---|
| `id` | UUID | PK | |
| `code` | VARCHAR(40) | NOT NULL UNIQUE | Clé stable |
| `libelle` | VARCHAR(80) | NOT NULL | Nom affiché |
| `description` | TEXT | NOT NULL | Condition en langage clair (FR-013) |
| `couleur` | VARCHAR(20) | NULL | Jeton de couleur |
| `icone` | VARCHAR(40) | NULL | Nom FontAwesome |
| `manuel` | BOOLEAN | NOT NULL DEFAULT FALSE | Badge éditorial : jamais évalué automatiquement |
| `type_condition` | `type_condition_badge` | NULL | Requis si `manuel = FALSE` |
| `parametre_action` | VARCHAR(50) | NULL | `type_action` visé (`actions_comptees`) |
| `parametre_categorie_id` | UUID | NULL REFERENCES `categorie_points(id)` ON DELETE RESTRICT | Catégorie visée (`points_categorie`) |
| `parametre_niveau_code` | VARCHAR(30) | NULL | Niveau visé (`niveau_atteint`) |
| `seuil` | INTEGER | NULL | Valeur à atteindre |
| `ordre` | SMALLINT | NOT NULL DEFAULT 0 | Ordre d'affichage |
| `actif` | BOOLEAN | NOT NULL DEFAULT TRUE | Retiré du catalogue « à débloquer », conservé chez les détenteurs |
| `created_at` / `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | |

**Contrainte de cohérence** (le paramétrage invalide doit être impossible **en SQL**, pas seulement dans l'UI) :

```sql
CONSTRAINT ck_badge_condition CHECK (
    (manuel = TRUE  AND type_condition IS NULL)
 OR (manuel = FALSE AND (
        (type_condition = 'actions_comptees'  AND parametre_action IS NOT NULL AND seuil > 0)
     OR (type_condition = 'points_categorie'  AND parametre_categorie_id IS NOT NULL AND seuil > 0)
     OR (type_condition = 'solde_total'       AND seuil > 0)
     OR (type_condition = 'niveau_atteint'    AND parametre_niveau_code IS NOT NULL)
     OR (type_condition = 'palier_popularite' AND seuil > 0)))
)
```

### 3. `engagement.badge_obtenu` (NEW)

| Colonne | Type | Contraintes |
|---|---|---|
| `id` | UUID | PK |
| `utilisateur_id` | UUID | NOT NULL REFERENCES `iam.utilisateur(id)` **ON DELETE CASCADE** |
| `badge_id` | UUID | NOT NULL REFERENCES `engagement.badge(id)` ON DELETE CASCADE |
| `origine` | `origine_badge` | NOT NULL DEFAULT `'automatique'` |
| `attribue_par` | UUID | NULL REFERENCES `iam.utilisateur(id)` ON DELETE SET NULL |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() |
| — | — | **UNIQUE (utilisateur_id, badge_id)** ← support de l'idempotence (FR-018, FR-034) |

Index : `idx_badge_obtenu_utilisateur (utilisateur_id, created_at DESC)`.
`ON DELETE CASCADE` sur l'utilisateur : un membre supprimé ne laisse pas de badge orphelin (edge case).

### 4. Seed des badges

| `code` | Condition | Paramètres |
|---|---|---|
| `premier_pas` | `actions_comptees` | `contribution_validee`, seuil 1 |
| `conteur` | `actions_comptees` | `contribution_validee`, seuil 10 |
| `pilier` | `actions_comptees` | `contribution_validee`, seuil 50 |
| `verificateur` | `actions_comptees` | `factcheck_valide`, seuil 5 |
| `voix_qui_porte` | `palier_popularite` | seuil 500 |
| `batisseur_medias` | `points_categorie` | catégorie `medias`, seuil 50 |
| `ambassadeur` | `actions_comptees` | `partage_externe_5reseaux`, seuil 3 |
| `statut_premium` | `niveau_atteint` | `premium` |
| `statut_platinum` | `niveau_atteint` | `platinum` |
| `distinction_editoriale` | *(manuel)* | — |

### 5. Rétro-évaluation unique (R9)

`INSERT INTO badge_obtenu (utilisateur_id, badge_id, origine) SELECT … 'retroactif' … ON CONFLICT DO NOTHING`, une requête par badge automatique seedé, évaluée sur l'état courant de `compte` / `mouvement_points`. **Aucune notification** n'est émise pour ce lot.

---

## Migration `35e_engagement_partage_externe.sql`

```sql
CREATE TYPE engagement.reseau_social AS ENUM
    ('whatsapp', 'facebook', 'x', 'linkedin', 'telegram', 'email');
```

### `engagement.partage_externe` (NEW)

| Colonne | Type | Contraintes | Sens |
|---|---|---|---|
| `id` | UUID | PK | |
| `utilisateur_id` | UUID | NOT NULL REFERENCES `iam.utilisateur(id)` ON DELETE CASCADE | Toujours issu du JWT, jamais du corps de requête |
| `type_objet` | VARCHAR(40) | NOT NULL | Famille du contenu partagé |
| `objet_id` | UUID | NOT NULL | Identifiant du contenu |
| `reseau` | `engagement.reseau_social` | NOT NULL | Réseau visé |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() | |
| — | — | **UNIQUE (utilisateur_id, type_objet, objet_id, reseau)** | Rend « réseaux **distincts** » structurel |

Index : `idx_partage_externe_contenu (utilisateur_id, type_objet, objet_id)` — sert le `COUNT(DISTINCT reseau)`.

**Séquence de traçage** (endpoint membre, best-effort) :

```
INSERT … ON CONFLICT DO NOTHING                       -- répéter un réseau ne crée rien
SELECT COUNT(DISTINCT reseau) …                       -- pour ce (membre, contenu)
si COUNT >= regle.seuil_declencheur  →  attribuer("partage_externe_5reseaux",
                                          clé "partage5:{type_objet}:{objet_id}:{utilisateur_id}")
```

Le plafond journalier de la règle (30 points) écrête au-delà de 3 bonus par jour ; l'écrêtage est journalisé (`plafond_atteint = TRUE`) et affiché au membre.

---

## Entités du domaine (correspondance spec ↔ SQL)

| Entité de la spec | Support SQL |
|---|---|
| Compte d'engagement | `engagement.compte` |
| Mouvement de points | `engagement.mouvement_points` (+ `categorie_id`) |
| Règle de points | `engagement.regle_points` (+ `categorie_id`, `seuil_declencheur`) |
| Catégorie de points | `engagement.categorie_points` |
| Palier de popularité | `engagement.palier_popularite` (+ `type_objet`) |
| Niveau | `engagement.niveau` (+ unicité de `seuil_min`) |
| Badge | `engagement.badge` |
| Badge obtenu | `engagement.badge_obtenu` |
| Partage externe | `engagement.partage_externe` |

## Transitions d'état

**Règle de points** : `créée (actif=TRUE)` ⇄ `désactivée (actif=FALSE)` — jamais supprimée si des mouvements la référencent (409).
**Badge** : `défini (actif=TRUE)` ⇄ `retiré du catalogue (actif=FALSE)`. Les `badge_obtenu` associés **survivent** dans les deux sens (FR-020).
**Badge obtenu** : `absent` → `obtenu` (automatique / rétroactif / manuel). Retour à `absent` **uniquement** par retrait manuel administrateur, tracé dans l'audit.
**Niveau d'un membre** : dérivé, recalculé à chaque mouvement (`niveau_pour_solde`) et à chaque mutation du référentiel des niveaux (`recalculer_niveaux`, R5). Monte et descend ; le badge de niveau suit, les badges de succès non.
**Mouvement de points** : **immuable**. Aucun `UPDATE`, aucun `DELETE`. Une correction se fait par un mouvement inverse (`ajustement_admin`).

## Invariants à ne pas casser

1. `cle_idempotence` reste `UNIQUE` et **porte tout ce qui distingue une attribution** (type d'action, objet, palier, membre pour le partage). Toute nouvelle règle doit définir sa clé avant d'être branchée.
2. `solde_points >= 0` (CHECK existant) — le plancher est appliqué dans `appliquer`, pas par l'appelant.
3. Aucune attribution dans une transaction métier : `attribuer` s'appelle **après** le `COMMIT`.
4. `mouvement_points` n'est jamais modifié ; la ventilation et les badges se recalculent toujours à partir de lui.
5. **Un seul champ du barème est figé sur le mouvement : `categorie_id`.** Le libellé de l'action, lui, est toujours relu dans `regle_points` à l'affichage — renommer une règle renomme l'action dans tout l'historique, re-catégoriser une règle ne déplace aucun point déjà gagné. Ne pas dénormaliser le libellé : la ventilation exige la stabilité, l'affichage exige la correction rétroactive.
6. Un membre ne peut jamais gagner de points sur une action qu'il a lui-même **décidée** (auto-validation, auto-mise à la une) ni sur sa propre réaction (auto-like). Le **partage externe est l'exception voulue** : son bénéficiaire est le partageur, même s'il partage son propre contenu (FR-030).
