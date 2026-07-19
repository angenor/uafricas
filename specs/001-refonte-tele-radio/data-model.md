# Phase 1 — Modèle de données : Refonte des pages Télé et Radio Africans

**Feature** : `001-refonte-tele-radio` | **Date** : 2026-07-19
**Schéma cible** : `media_content` | **Migrations** : `09j` → `09n` sous `uafricas_backend/doc/bd/schemas/`

Conventions respectées (Principe III) : UUID v4 en PK, `TIMESTAMPTZ`, soft delete par `deleted_at`,
snake_case français, migrations **idempotentes** (`IF NOT EXISTS`, blocs `DO $$` pour les enums et les FK),
et déclaration de chaque fichier dans `doc/bd/schema.sql` par `\ir`, après la ligne de `09i`.

Les tables existantes concernées sont `media_content.chaine_tv`, `station_radio`, `programme_tele`,
`programme_radio` (DDL consolidé en [research.md](./research.md)).

---

## Vue d'ensemble

```
                    ┌──────────────────────┐         ┌──────────────────────┐
                    │  chaine_tv (exist.)  │         │ station_radio (exist)│
                    │  + role_partie_pren. │         │ + origine_publication│
                    └──────────┬───────────┘         │ + role_partie_pren.  │
                               │                     └──────────┬───────────┘
                    ┌──────────▼───────────┐         ┌──────────▼───────────┐
                    │ programme_tele (ex.) │         │ programme_radio (ex.)│
                    │ + a_la_une_globale   │         │ + theme_phare_id     │
                    │ + theme_phare_id     │         │ + nombre_signalements│
                    └──────────┬───────────┘         └──────────┬───────────┘
                               └───────────┬─────────────────────┘
                                           │  (type_media, media_id)
        ┌──────────────┬───────────────────┼───────────────────┬──────────────────┐
        ▼              ▼                   ▼                   ▼                  ▼
  media_reaction  media_commentaire   partage_media     signalement_media   creneau_programmation
                                                                                   ▲
   proposition_media ──valide──► crée l'objet ──► support_detenteur ◄── invitation_detenteur
```

---

## 1. Migration `09j` — Éditorial, origine, référentiels

### 1.1 Origine de publication des stations (FR-014)

```sql
ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS origine_publication VARCHAR(20) NOT NULL DEFAULT 'territoire';

ALTER TABLE media_content.station_radio DROP CONSTRAINT IF EXISTS ck_station_radio_origine;
ALTER TABLE media_content.station_radio
    ADD CONSTRAINT ck_station_radio_origine
        CHECK (origine_publication IN ('africans', 'territoire'));

CREATE INDEX IF NOT EXISTS idx_station_radio_origine
    ON media_content.station_radio (origine_publication) WHERE deleted_at IS NULL;
```

| Valeur | Page | Sens |
|---|---|---|
| `africans` | `/medias/radio/africans` | Production propre de la plateforme, décision éditoriale de ses créateurs |
| `territoire` | `/medias/radio/nationales` | Station rattachée à un territoire africain |

**Reprise de données** : le défaut `'territoire'` qualifie tout l'existant du côté Nationales. Aucun seed
radio n'existe (vérifié), donc la reprise porte uniquement sur ce que les administrateurs ont saisi
manuellement ; elle se fait par `UPDATE … SET origine_publication = 'africans' WHERE id IN (…)` après
livraison. **Invariant** : une station relève d'exactement une page — garanti par le `NOT NULL` + `CHECK`.

### 1.2 FK manquantes sur `station_radio`

La table n'a **aucune** contrainte de clé étrangère aujourd'hui, contrairement à `programme_radio`
(`09g:87-119`). Les poser via le bloc `DO $$ … pg_constraint` idempotent :

```sql
-- fk_station_radio_pays     : pays_id  → shared.pays(id)       ON DELETE SET NULL
-- fk_station_radio_cree_par : cree_par → iam.utilisateur(id)   ON DELETE RESTRICT
```

### 1.3 Vedette générale de la page Télé (FR-001, FR-007)

```sql
ALTER TABLE media_content.programme_tele
    ADD COLUMN IF NOT EXISTS a_la_une_globale BOOLEAN NOT NULL DEFAULT FALSE;

CREATE UNIQUE INDEX IF NOT EXISTS uq_programme_tele_a_la_une_globale
    ON media_content.programme_tele ((TRUE))
    WHERE a_la_une_globale = TRUE AND deleted_at IS NULL;
```

Deux portées de mise en avant coexistent, sans interférence :

| Portée | Colonne | Unicité | Exigence |
|---|---|---|---|
| Générale (page Télé) | `a_la_une_globale` | 1 pour toute la table | FR-001, FR-002 |
| Par chaîne | `a_la_une` (existant) | 1 par `chaine_id` — `uq_programme_tele_a_la_une_par_chaine` (`09g:82`) | FR-005 |
| Par station | `a_la_une` (existant) | 1 par `station_id` — `uq_programme_radio_a_la_une_par_station` (`09g:48`) | FR-013 |

**Repli déterministe** (FR-007, edge case « vedette indisponible ») : à défaut de programme portant
`a_la_une_globale = TRUE` **et** `etat = 'publie'`, servir
`ORDER BY created_at DESC LIMIT 1` sur les programmes publiés, toutes chaînes confondues.

**Concurrence** : la bascule de l'ancienne vedette à `FALSE` puis l'INSERT/UPDATE de la nouvelle **doivent
être dans une même transaction**. L'existant fait ces deux requêtes sur le pool sans transaction
(`admin/radio_tele.rs:1256-1265`) : avec un index unique, la seconde échouerait en concurrence.

### 1.4 Thème phare et rôle de partie prenante (FR-029, FR-030)

Référentiel des 43 thèmes dans `shared.categorie` avec `contexte = 'media'`, alimenté par
`INSERT … ON CONFLICT (slug) DO NOTHING` (patron `05c:25-38`).

```sql
ALTER TABLE media_content.programme_tele
    ADD COLUMN IF NOT EXISTS theme_phare_id    UUID,          -- [xref] shared.categorie (contexte='media')
    ADD COLUMN IF NOT EXISTS theme_phare_autre VARCHAR(200);
-- idem programme_radio

ALTER TABLE media_content.chaine_tv
    ADD COLUMN IF NOT EXISTS role_partie_prenante       VARCHAR(40),
    ADD COLUMN IF NOT EXISTS role_partie_prenante_autre VARCHAR(200);
-- idem station_radio, avec le même CHECK
```

`role_partie_prenante ∈ ('chaine_tele','radio','journaliste','communicateur','createur_contenu',
'influenceur','realisateur','producteur','autre')`.

**Règle de validation commune** (FR-029, FR-030, edge case « Contribution avec Autre ») — exprimée en SQL,
pas seulement applicative :

```sql
CHECK (role_partie_prenante <> 'autre' OR
       (role_partie_prenante_autre IS NOT NULL AND btrim(role_partie_prenante_autre) <> ''))
CHECK (theme_phare_id IS NOT NULL OR
       (theme_phare_autre IS NOT NULL AND btrim(theme_phare_autre) <> ''))
```

**Convention xref** : `theme_phare_id` reste une référence logique **sans contrainte FK**, comme
`secteur_id` (`05c:14`) et `categorie_id`.

### 1.5 État « en attente » et compteur de signalements

Les quatre tables portent `etat VARCHAR(50) CHECK IN ('brouillon','publie','suspendu','supprime')`.
FR-032 introduit un état d'attente de revalidation :

```sql
-- Élargir le CHECK des 4 tables : ajout de 'en_attente'
--   ('brouillon','en_attente','publie','suspendu','supprime')
ALTER TABLE media_content.programme_tele
    ADD COLUMN IF NOT EXISTS nombre_signalements INT NOT NULL DEFAULT 0;
-- idem programme_radio, chaine_tv, station_radio
```

> **Conséquence de conception à valider** : un contenu dont le média est remplacé passe en `'en_attente'` et
> **cesse d'être diffusé** jusqu'à revalidation. La spec (FR-032) exige la revalidation mais ne tranche pas
> le sort de la diffusion pendant l'attente. Le choix retenu est le plus sûr : rien de non validé n'est
> public (FR-031). L'alternative — conserver l'ancien média en ligne — imposerait un versionnement du média.

### 1.6 Corrections de dette bloquantes

```sql
-- Sans ceci, seul super_admin peut modérer : le seed ne déclare aucune permission 'media' (R15)
INSERT INTO iam.permission (nom, slug, type_ressource, action) VALUES
    ('Voir les médias radio/télé',      'media.voir',      'media', 'voir'),
    ('Modifier les médias radio/télé',  'media.modifier',  'media', 'modifier'),
    ('Supprimer les médias radio/télé', 'media.supprimer', 'media', 'supprimer')
ON CONFLICT (slug) DO NOTHING;
-- + liaison au rôle 'admin' via iam.role_permission

-- Types de notification actuels déjà > 30 caractères (R14)
ALTER TABLE arbre_genealogique.notifications ALTER COLUMN type TYPE VARCHAR(80);
```

---

## 2. Migration `09k` — Interactions communautaires (US3, US7)

Discriminant local, en `VARCHAR + CHECK` pour rester extensible sans `ALTER TYPE` (patron
`governance.partage_contribution:16-18`) :

```
type_media ∈ ('chaine_tv', 'station_radio', 'programme_tele', 'programme_radio')
```

### 2.1 `media_content.media_reaction` (FR-023)

| Colonne | Type | Notes |
|---|---|---|
| `id` | UUID PK | |
| `type_media` | VARCHAR(20) NOT NULL | CHECK sur les 4 valeurs |
| `media_id` | UUID NOT NULL | |
| `utilisateur_id` | UUID NOT NULL | → `iam.utilisateur` ON DELETE CASCADE |
| `type_reaction` | VARCHAR(10) NOT NULL | CHECK `IN ('like','dislike')` |
| `created_at` / `updated_at` | TIMESTAMPTZ | |

`UNIQUE (type_media, media_id, utilisateur_id)` — une seule réaction retenue par membre et par contenu ;
le changement se fait par `ON CONFLICT … DO UPDATE`, le retrait par `DELETE`.

### 2.2 `media_content.media_commentaire` (FR-024)

Modèle : `iam.biblio_commentaire` (`04g:41-55`), **liste plate** — FR-024 ne demande pas de fil de réponses.

`id`, `type_media`, `media_id`, `auteur_id` (→ `iam.utilisateur` CASCADE),
`contenu TEXT NOT NULL CHECK (char_length BETWEEN 1 AND 2000)`, `created_at`, `updated_at`, `deleted_at`.
Suppression = soft delete par l'auteur. Aucune modération de commentaire : aucun précédent n'existe dans le
projet et la spec n'en demande pas.

### 2.3 `media_content.partage_media` (FR-025)

`id`, `type_media`, `media_id`, `utilisateur_id`, `legende TEXT CHECK (char_length <= 500)`,
`created_at`, `deleted_at`. Alimente le mur `/publications` comme 8ᵉ source.

### 2.4 `media_content.signalement_media` (FR-049, FR-050)

`id`, `type_media`, `media_id`, `signale_par` (→ `iam.utilisateur` CASCADE),
`motif VARCHAR(50)`, `description TEXT CHECK (char_length <= 1000)`, `created_at`,
`UNIQUE (type_media, media_id, signale_par)` — idempotence : un membre ne peut pas gonfler le compteur.

**Algorithme** (calqué sur `contribution_signalement.rs:100-175`, adapté à `etat`) :

1. `INSERT … ON CONFLICT DO NOTHING`
2. `SELECT COUNT(*)` des signalements distincts
3. si `count > 10` → `UPDATE <table> SET nombre_signalements = $n, etat = 'suspendu'`
4. `audit::log_action` avec action `SIGNALEMENT` ou `SIGNALEMENT_SUSPENSION`

`SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA: i64 = 10`, comparateur `>` (suspension au 11ᵉ signalement distinct).
Jamais de désuspension automatique : le rétablissement est administratif et remet `nombre_signalements = 0`
(FR-051).

---

## 3. Migration `09l` — Propositions et modération (US4)

```sql
CREATE TYPE media_content.type_objet_propose AS ENUM (
    'chaine_tv', 'station_radio', 'programme_tele', 'programme_radio',
    'animation_programme',   -- FR-045 : validation ⇒ ajoute un co-détenteur
    'idee_contenu'           -- FR-044 : suggestion, ne crée aucun objet
);
CREATE TYPE media_content.statut_proposition_media AS ENUM (
    'en_attente', 'validee', 'rejetee', 'retiree'
);
```

### `media_content.proposition_media`

| Colonne | Type | Rôle |
|---|---|---|
| `id` | UUID PK | |
| `auteur_id` | UUID NOT NULL | [xref] `iam.utilisateur` |
| `type_objet` | `type_objet_propose` NOT NULL | discriminant |
| `target_id` | UUID NULL | NULL = création ; renseigné = modification ou demande d'animation |
| `donnees` | JSONB NOT NULL | payload complet de l'objet proposé |
| `pieces_jointes` | JSONB NOT NULL DEFAULT `'[]'` | URLs des médias téléversés |
| `justification` | TEXT NOT NULL | motif du contributeur |
| `statut` | `statut_proposition_media` NOT NULL DEFAULT `'en_attente'` | |
| `decideur` | UUID NULL | administrateur ayant tranché |
| `decide_at` | TIMESTAMPTZ NULL | |
| `commentaire_decision` | TEXT NULL | **obligatoire au rejet** |
| `objet_id_cree` | UUID NULL | objet réel créé à la validation |
| `created_at` / `updated_at` | TIMESTAMPTZ | |

**Quatre contraintes rendent le workflow inviolable en SQL** (modèle `afrolang.proposition_salle:359-403`) :

```sql
CONSTRAINT ck_prop_media_decision_coherente CHECK (
    (statut = 'en_attente' AND decideur IS NULL AND decide_at IS NULL) OR
    (statut IN ('validee','rejetee') AND decideur IS NOT NULL AND decide_at IS NOT NULL) OR
    (statut = 'retiree' AND decideur IS NULL))
CONSTRAINT ck_prop_media_rejet_commente CHECK (
    statut <> 'rejetee' OR commentaire_decision IS NOT NULL)
CONSTRAINT ck_prop_media_validation_a_objet CHECK (
    statut <> 'validee' OR type_objet = 'idee_contenu' OR objet_id_cree IS NOT NULL)
CONSTRAINT ck_prop_media_cible_requise CHECK (
    type_objet NOT IN ('animation_programme','idee_contenu') OR target_id IS NOT NULL)
```

**Ce que produit une validation, selon le type** :

| `type_objet` | `objet_id_cree` référence | Effet |
|---|---|---|
| `chaine_tv`, `station_radio` | la chaîne ou station créée | + 1 ligne `support_detenteur` en `proprietaire` |
| `programme_tele`, `programme_radio` | le contenu créé | rattaché au support de `target_id` |
| `animation_programme` | la ligne `support_detenteur` créée | ajoute le demandeur aux co-détenteurs (FR-045) |
| `idee_contenu` | *aucun* — exempté par le CHECK | l'idée est marquée retenue, rien n'est créé (FR-044) |

Index : `(statut, created_at DESC)` pour la file admin, `(auteur_id, created_at DESC)` pour « mes
soumissions », `(type_objet, statut)`.

**Aucune décharge de droits n'est stockée** — décision explicite du commanditaire (H-012). La colonne
`decharge_droits` de vidafrica (`27c:18`) n'est donc **pas** reprise. L'examen de licéité incombe à
l'administrateur au moment de la validation (FR-033).

### Transitions d'état

```
                    ┌──────────────┐
   soumission ─────►│  en_attente  │
                    └──┬────┬───┬──┘
      valider (admin)  │    │   │  retirer (auteur)
              ┌────────┘    │   └──────────┐
              ▼             ▼              ▼
        ┌──────────┐  ┌──────────┐  ┌──────────┐
        │ validee  │  │ rejetee  │  │ retiree  │
        └────┬─────┘  └──────────┘  └──────────┘
             │  crée l'objet + 1er co-détenteur
             ▼
   chaine_tv | station_radio | programme_* en etat='publie'
```

Tous les états hors `en_attente` sont **terminaux** : `moderer` refuse de re-trancher une proposition déjà
décidée. Le rejet exige un motif d'au moins 10 caractères (garde applicative, doublée du CHECK SQL).

**Effets atomiques de la validation**, en une seule transaction :
`SELECT … FOR UPDATE` → `INSERT` de l'objet métier `RETURNING id` → `INSERT` du premier co-détenteur avec
`role = 'proprietaire'` → `UPDATE proposition SET statut='validee', decideur, decide_at, objet_id_cree` →
`INSERT` de la notification (style transactionnel, `admin/profils_pays.rs:2545-2559`) → `commit` → audit.

---

## 4. Migration `09m` — Co-détention (FR-037, FR-045)

```sql
CREATE TYPE media_content.type_support_media AS ENUM ('chaine_tv', 'station_radio');
CREATE TYPE media_content.role_detenteur AS ENUM ('proprietaire', 'co_detenteur', 'programmateur');
```

### `media_content.support_detenteur`

Modèle : `afrolang.salle_moderateur` (`08b:224-242`), enrichi du rôle.

`id`, `type_support`, `support_id`, `utilisateur_id`, `role` (DEFAULT `'co_detenteur'`),
`designe_par`, `designe_at`, `actif BOOLEAN NOT NULL DEFAULT TRUE`, `retire_at`, `created_at`, `updated_at`.

- `UNIQUE (type_support, support_id, utilisateur_id)` — **sans filtre** : une seule ligne par paire, jamais
  de doublon historique. Le retrait est un soft-delete (`actif = FALSE`, `retire_at = NOW()`), l'ajout un
  upsert-réactivation à trois branches (`admin/moderateurs_afrolang.rs:59-190`).
- `CREATE UNIQUE INDEX uq_support_un_proprietaire ON …(type_support, support_id) WHERE role = 'proprietaire'
  AND actif = TRUE` — un seul propriétaire actif par support.
- Index partiels sur `(type_support, support_id) WHERE actif` et `(utilisateur_id) WHERE actif`.

| Rôle | Droits |
|---|---|
| `proprietaire` | tout, plus inviter et révoquer les autres |
| `co_detenteur` | éditer la fiche, gérer les contenus, programmer |
| `programmateur` | programmer uniquement |

**Edge case « dernier co-détenteur retiré »** : aucune contrainte n'impose au moins un détenteur actif. Le
support reste diffusé et sa grille modifiable par un administrateur — conforme à l'exigence.

### `media_content.invitation_detenteur`

Modèle : `arbre_genealogique.invitations` (`25:16-38`).

`id`, `type_support`, `support_id`, `email_invite`, `utilisateur_invite_id` NULL, `role`,
`statut VARCHAR(20) CHECK IN ('en_attente','acceptee','refusee','expiree')`, `invite_par`,
`created_at`, `expire_at DEFAULT (NOW() + INTERVAL '30 days')`, `traitee_le`.

**Garde d'autorisation** : ces endpoints s'adressent à des **membres**. Ne pas utiliser l'extracteur
`AdminUtilisateur`, qui rejette tout non-admin (`middleware/admin.rs:100-105`). Écrire
`garde_detenteur(pool, type_support, support_id, moi, roles_admis) -> Result<(), ApiErreur>` sur le modèle
de `garde_proprietaire` (`handlers/annonces.rs:111`), et l'appeler aussi depuis les handlers de contenu pour
que les co-détenteurs puissent publier sur leur chaîne.

---

## 5. Migration `09n` — Grille de programmation (US5)

### `media_content.creneau_programmation`

| Colonne | Type | Notes |
|---|---|---|
| `id` | UUID PK | |
| `type_support` | `type_support_media` NOT NULL | |
| `support_id` | UUID NOT NULL | chaîne ou station |
| `contenu_id` | UUID NOT NULL | `programme_tele.id` ou `programme_radio.id` |
| `recurrence` | VARCHAR(20) NOT NULL | CHECK `IN ('quotidien','hebdomadaire')` |
| `jour_semaine` | SMALLINT NULL | 0 = dimanche … 6 = samedi ; NULL si quotidien |
| `heure_debut` | TIME NOT NULL | heure locale du `fuseau` |
| `duree_minutes` | INT NOT NULL | CHECK `BETWEEN 1 AND 1440` |
| `fuseau` | VARCHAR(60) NOT NULL DEFAULT `'Africa/Abidjan'` | référentiel explicite (FR-042) |
| `cree_par` | UUID NOT NULL | co-détenteur auteur du créneau (FR-055) |
| `actif` | BOOLEAN NOT NULL DEFAULT TRUE | |
| `created_at` / `updated_at` / `deleted_at` | TIMESTAMPTZ | |

```sql
CONSTRAINT ck_creneau_jour_coherent CHECK (
    (recurrence = 'quotidien'    AND jour_semaine IS NULL) OR
    (recurrence = 'hebdomadaire' AND jour_semaine BETWEEN 0 AND 6))
-- Un créneau ne franchit pas minuit : le scinder en deux si nécessaire.
CONSTRAINT ck_creneau_pas_minuit CHECK (
    heure_debut + make_interval(mins => duree_minutes) <= TIME '24:00')
```

**Écart assumé à « TIMESTAMPTZ partout »** (`schema.sql:32`) : la récurrence n'est pas un instant. Le
couple `TIME` + `jour_semaine` + `fuseau` est le seul moyen de l'exprimer sans matérialiser des lignes à
l'infini.

### Résolution du créneau courant — paresseuse, sans worker (R7)

Le patron maison est le calcul à la lecture (`rendez_vous.rs:184,190`, `afrolang.rs:422`). Aucune tâche de
fond n'est introduite :

```sql
-- « quel contenu passe en ce moment sur ce support ? »
WITH maintenant AS (SELECT (NOW() AT TIME ZONE c.fuseau) AS local FROM … )
SELECT c.contenu_id
  FROM media_content.creneau_programmation c, maintenant m
 WHERE c.support_id = $1 AND c.actif AND c.deleted_at IS NULL
   AND (c.recurrence = 'quotidien' OR c.jour_semaine = EXTRACT(DOW FROM m.local))
   AND m.local::time >= c.heure_debut
   AND m.local::time <  c.heure_debut + make_interval(mins => c.duree_minutes)
 ORDER BY c.heure_debut DESC
 LIMIT 1;
```

Le contenu servi est filtré sur `etat = 'publie'` ; s'il ne l'est plus, la requête retombe sur le contenu
mis en évidence du support (FR-041, FR-043) et le créneau est marqué invalide pour ses co-détenteurs.

**Détection de chevauchement (FR-040)** : vérification **applicative en transaction**, précédée d'un
verrou sur la ligne du support parent (`SELECT id FROM media_content.chaine_tv WHERE id = $1 FOR UPDATE`).
Ce verrou sérialise toutes les modifications de grille d'un même support et couvre l'edge case
« co-détenteurs en concurrence », y compris les insertions concurrentes que `FOR UPDATE` sur les créneaux
existants ne verrouillerait pas.

> Une contrainte d'exclusion GiST serait plus élégante mais imposerait l'extension `btree_gist`, un type
> range sur `TIME` et le traitement du franchissement de minuit — pour une garantie qu'un verrou de ligne
> apporte déjà à cette volumétrie. Le projet n'utilise aucune contrainte d'exclusion aujourd'hui.

---

## 6. Correspondance entités de la spec → tables

| Entité (spec) | Support physique |
|---|---|
| Chaîne (télé) | `chaine_tv` + `role_partie_prenante` |
| Station (radio) | `station_radio` + `origine_publication` + `role_partie_prenante` |
| Programme (contenu) | `programme_tele` / `programme_radio` + `theme_phare_id` + `etat` |
| Mise en avant | `programme_tele.a_la_une_globale` (générale) · `a_la_une` (par support) |
| Rôle de partie prenante | `role_partie_prenante` + `_autre` (CHECK, 9 valeurs) |
| Thème phare | `shared.categorie` contexte `'media'` + `theme_phare_id` / `_autre` |
| Co-détention | `support_detenteur` + `invitation_detenteur` |
| Créneau de programmation | `creneau_programmation` |
| Interaction communautaire | `media_reaction`, `media_commentaire`, `partage_media` |
| Proposition d'engagement | `proposition_media` (`type_objet = 'animation_programme'` pour les demandes) |
| Signalement | `signalement_media` + `nombre_signalements` + `etat = 'suspendu'` |

## 7. Récapitulatif — ce qui est créé, modifié, non touché

**Créé** : 8 tables — `media_reaction`, `media_commentaire`, `partage_media`, `signalement_media`
(`09k`) ; `proposition_media` (`09l`) ; `support_detenteur`, `invitation_detenteur` (`09m`) ;
`creneau_programmation` (`09n`). Plus 4 types ENUM (`type_objet_propose`, `statut_proposition_media`,
`type_support_media`, `role_detenteur`) et 1 référentiel peuplé dans `shared.categorie`.

**Modifié** : 4 tables existantes (colonnes ajoutées, `CHECK etat` élargi), `iam.permission` (seed),
`arbre_genealogique.notifications` (élargissement de `type`).

**Non touché** : `categorie_radio` (reste utile en back-office), `type_station` (axe portée, conservé avec
son mapper bidirectionnel), toutes les tables `vidafrica` (`27*`), `country_profile.reaction_element` et
`partage_element` (aucun refactor du générique afripulse).
