# Phase 1 : Modèle de données

**Feature**: 009-medias-programmes-episodes · **Migration**: `uafricas_backend/doc/bd/schemas/09q_media_content_emissions_episodes.sql`

Conventions du projet respectées : UUID v4 en clé primaire, `deleted_at` pour la suppression douce,
`TIMESTAMPTZ`, snake_case français, CHECK nommés explicitement, migration idempotente
(`CREATE … IF NOT EXISTS`, `DROP CONSTRAINT IF EXISTS` puis `ADD`, blocs `DO $$` sur `pg_constraint` et
`pg_type`).

**Vocabulaire** : `emission_*` en base et dans le code = « **Programme** » dans l'interface ;
`episode_*` = « **Épisode** ». Voir research.md R1.

---

## 1. Vue d'ensemble

```
chaine_tv ──┬── support_thematique  (1..N thèmes)        [R5]
            ├── support_territoire  (0..N territoires)   [R6]
            ├── couverture_continentale : BOOLEAN        [R6]
            ├── support_detenteur      (existant, 09m)
            ├── creneau_programmation  (existant, 09n → cible une émission)
            └── emission_tele ──── episode_tele
                                     · ordre           [R10]
                                     · etat : en_attente | publie | rejete | suspendu …
                                     · a_la_une, a_la_une_globale   [R9]
                                     · id et slug REPRIS de programme_tele  [R2]

station_radio ── (symétrique) ── emission_radio ── episode_radio
```

Les quatre tables d'interactions (`media_reaction`, `media_commentaire`, `partage_media`,
`signalement_media`) voient leur discriminant passer de 4 à 6 valeurs et acceptent désormais l'émission
comme l'épisode.

---

## 2. Nouvelles tables

### 2.1 `media_content.emission_tele` : conteneur télé

| Colonne | Type | Contraintes |
|---------|------|-------------|
| `id` | UUID | PK, `gen_random_uuid()` |
| `chaine_id` | UUID | NOT NULL, FK → `chaine_tv(id)` ON DELETE CASCADE |
| `titre` | VARCHAR(350) | NOT NULL |
| `slug` | VARCHAR(400) | UNIQUE |
| `description` | TEXT | NOT NULL DEFAULT `''` |
| `image_couverture_url` | VARCHAR(500) | |
| `info_animateur` | TEXT | |
| `info_producteur` | TEXT | |
| `langue` | VARCHAR(80) | NOT NULL DEFAULT `'Français'` |
| `theme_phare_id` | UUID | `[xref]` `shared.categorie` (contexte `media`) |
| `theme_phare_autre` | VARCHAR(200) | CHECK : NULL ou non vide après `btrim` |
| `cadence` | VARCHAR(20) | NOT NULL DEFAULT `'ponctuelle'`, CHECK ∈ (`quotidienne`, `hebdomadaire`, `ponctuelle`), FR-013 |
| `etat` | VARCHAR(50) | NOT NULL DEFAULT `'brouillon'`, CHECK ∈ (`brouillon`, `en_attente`, `publie`, `suspendu`, `supprime`) |
| `nombre_signalements` | INT | NOT NULL DEFAULT 0 |
| `cree_par` | UUID | NOT NULL, FK → `iam.utilisateur(id)` ON DELETE RESTRICT |
| `created_at` / `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() |
| `deleted_at` | TIMESTAMPTZ | |

Index : `(chaine_id) WHERE deleted_at IS NULL`, `(etat) WHERE deleted_at IS NULL`,
`(theme_phare_id) WHERE deleted_at IS NULL`.

`emission_radio` est identique, `chaine_id` devenant `station_id` → `station_radio(id)`, plus
`categorie_radio media_content.categorie_radio` reprise de `programme_radio`.

### 2.2 `media_content.episode_tele` : unité diffusable

| Colonne | Type | Contraintes |
|---------|------|-------------|
| `id` | UUID | PK : **repris de `programme_tele.id`** pour les lignes migrées (R2) |
| `emission_id` | UUID | NOT NULL, FK → `emission_tele(id)` ON DELETE RESTRICT, FR-010 |
| `titre` | VARCHAR(350) | NOT NULL |
| `slug` | VARCHAR(400) | UNIQUE : **repris de `programme_tele.slug`** (R2, FR-056) |
| `description` | TEXT | NOT NULL DEFAULT `''` |
| `image_couverture_url` | VARCHAR(500) | |
| `video_url` | VARCHAR(500) | fichier `/uploads/…` ou lien |
| `numero_episode` | INT | facultatif, FR-005 |
| `ordre` | INT | NOT NULL DEFAULT 0 : support de la rotation (R10) |
| `duree_minutes` | INT | CHECK NULL ou > 0 |
| `a_la_une` | BOOLEAN | NOT NULL DEFAULT FALSE, une par chaîne (R9) |
| `a_la_une_globale` | BOOLEAN | NOT NULL DEFAULT FALSE, une pour tout l'espace Télé (R9) |
| `etat` | VARCHAR(50) | NOT NULL DEFAULT `'en_attente'`, CHECK ∈ (`brouillon`, `en_attente`, `publie`, `rejete`, `suspendu`, `supprime`), FR-040 |
| `motif_rejet` | TEXT | CHECK : requis et non vide si `etat = 'rejete'`, FR-041 |
| `valide_par` | UUID | `[xref]` `iam.utilisateur`, décideur |
| `valide_at` | TIMESTAMPTZ | |
| `nombre_signalements` | INT | NOT NULL DEFAULT 0 |
| `cree_par` | UUID | NOT NULL, FK → `iam.utilisateur(id)` ON DELETE RESTRICT |
| `created_at` / `updated_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() |
| `deleted_at` | TIMESTAMPTZ | |

Contraintes propres :

```sql
-- Un épisode publié porte forcément son média (FR-008)
CONSTRAINT ck_episode_tele_media_publie
    CHECK (etat <> 'publie' OR video_url IS NOT NULL)

-- Un rejet est toujours motivé (FR-041), même exigence que ck_prop_media_rejet_commente (09l)
CONSTRAINT ck_episode_tele_rejet_motive
    CHECK (etat <> 'rejete' OR (motif_rejet IS NOT NULL AND btrim(motif_rejet) <> ''))

-- Une décision porte son décideur et sa date
CONSTRAINT ck_episode_tele_decision_coherente
    CHECK (etat NOT IN ('publie','rejete') OR (valide_par IS NOT NULL AND valide_at IS NOT NULL))
```

Index :

```sql
-- Requête chaude : les épisodes publiés d'une émission, dans l'ordre de rotation
CREATE INDEX idx_episode_tele_emission_ordre
    ON media_content.episode_tele (emission_id, ordre, created_at)
    WHERE etat = 'publie' AND deleted_at IS NULL;

-- File de modération, triée par ancienneté
CREATE INDEX idx_episode_tele_en_attente
    ON media_content.episode_tele (created_at)
    WHERE etat = 'en_attente' AND deleted_at IS NULL;

-- Une seule mise en avant par chaîne, une seule vedette globale (R9, transposé de 09g:82 et 09j §3)
CREATE UNIQUE INDEX uq_episode_tele_a_la_une_par_emission
    ON media_content.episode_tele (emission_id)
    WHERE a_la_une = TRUE AND deleted_at IS NULL;
CREATE UNIQUE INDEX uq_episode_tele_a_la_une_globale
    ON media_content.episode_tele ((TRUE))
    WHERE a_la_une_globale = TRUE AND deleted_at IS NULL;
```

`episode_radio` est identique, `video_url` devenant `audio_url` et sans `a_la_une_globale` (la vedette
plein écran n'existe que sur l'espace Télé).

> ⚠️ `ON DELETE RESTRICT` sur `emission_id` réalise FR-010 **en SQL** : une émission ne peut pas
> disparaître sous ses épisodes. La suppression douce reste possible côté émission.

### 2.3 `media_content.support_thematique`, thématiques multiples (R5)

| Colonne | Type | Contraintes |
|---------|------|-------------|
| `id` | UUID | PK |
| `type_support` | `media_content.type_support_media` | NOT NULL (enum posé par 09m) |
| `support_id` | UUID | NOT NULL : polymorphe, sans FK |
| `categorie_id` | UUID | NOT NULL : `[xref]` `shared.categorie` (contexte `media`) |
| `created_at` | TIMESTAMPTZ | NOT NULL DEFAULT NOW() |

`UNIQUE (type_support, support_id, categorie_id)`, un thème n'est déclaré qu'une fois par support,
ce qui rend le « sans doublon » de FR-030 structurel.
Index : `(categorie_id)` pour le filtre inverse, `(type_support, support_id)` pour la fiche.

### 2.4 `media_content.support_territoire`, couverture (R6)

Mêmes colonnes, `categorie_id` remplacé par `pays_id UUID NOT NULL` (`[xref]` `shared.pays`).
`UNIQUE (type_support, support_id, pays_id)`.

Trigger d'exclusivité :

```sql
CREATE OR REPLACE FUNCTION media_content.verifier_couverture_exclusive()
RETURNS TRIGGER AS $$
DECLARE continentale BOOLEAN;
BEGIN
    EXECUTE format(
        'SELECT couverture_continentale FROM media_content.%I WHERE id = $1',
        CASE NEW.type_support WHEN 'chaine_tv' THEN 'chaine_tv' ELSE 'station_radio' END)
    INTO continentale USING NEW.support_id;

    IF continentale THEN
        RAISE EXCEPTION
            'Couverture continentale déclarée : aucun territoire individuel ne peut être ajouté (FR-034)';
    END IF;
    RETURN NEW;
END $$ LANGUAGE plpgsql;
```

Le passage à `couverture_continentale = TRUE` supprime les lignes de territoire **dans la même
transaction**, côté handler.

---

## 3. Tables modifiées

### 3.1 `chaine_tv` et `station_radio`

```sql
ALTER TABLE media_content.chaine_tv
    ADD COLUMN IF NOT EXISTS couverture_continentale BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE media_content.station_radio
    ADD COLUMN IF NOT EXISTS couverture_continentale BOOLEAN NOT NULL DEFAULT FALSE;
```

`chaine_tv.categorie`, `chaine_tv.pays_id`, `station_radio.genre` / `genres_liste` / `pays_id` sont
**conservées** : la migration les lit pour amorcer les tables de liaison (FR-057), puis le code cesse de
les écrire. Elles ne sont pas supprimées dans cette migration, leur retrait relèvera d'un nettoyage
ultérieur, une fois le portage frontend confirmé.

### 3.2 `creneau_programmation` (09n)

```sql
ALTER TABLE media_content.creneau_programmation
    ADD COLUMN IF NOT EXISTS emission_id UUID,          -- remplace contenu_id
    ADD COLUMN IF NOT EXISTS date_effet  DATE NOT NULL DEFAULT CURRENT_DATE;   -- R4
```

`contenu_id` est repris vers `emission_id` (voir §4, étape 5) puis supprimé. `emission_id` reste sans FK
la cible est polymorphe selon `type_support`, comme l'était `contenu_id`.

Index à recréer : `idx_creneau_emission` sur `(emission_id) WHERE actif AND deleted_at IS NULL`,
en remplacement de `idx_creneau_contenu`.

### 3.3 Les quatre tables d'interactions (09k)

Les CHECK `ck_*_type_media` passent à six valeurs :

```sql
CHECK (type_media IN ('chaine_tv', 'station_radio',
                      'emission_tele', 'emission_radio',
                      'episode_tele', 'episode_radio'))
```

### 3.4 `proposition_media` (09l)

```sql
ALTER TYPE media_content.type_objet_propose ADD VALUE IF NOT EXISTS 'emission_tele';
ALTER TYPE media_content.type_objet_propose ADD VALUE IF NOT EXISTS 'emission_radio';
ALTER TYPE media_content.type_objet_propose ADD VALUE IF NOT EXISTS 'episode_tele';
ALTER TYPE media_content.type_objet_propose ADD VALUE IF NOT EXISTS 'episode_radio';
```

Les anciennes valeurs `programme_tele` / `programme_radio` restent dans l'enum (PostgreSQL ne sait pas
retirer une valeur) mais ne sont plus produites ; les propositions historiques les conservent.

> ⚠️ `ALTER TYPE … ADD VALUE` ne peut pas s'exécuter dans le même bloc transactionnel qu'une utilisation
> de la nouvelle valeur. Ces quatre instructions doivent précéder tout `INSERT` les employant, en
> pratique, elles sont en tête de migration et aucun seed ne les utilise.

---

## 4. Reprise de données : ordre des opérations

La migration est **une seule fenêtre**, sans cohabitation des deux modèles (hypothèse de la spec).

1. **Créer** `emission_tele`, `emission_radio`, `episode_tele`, `episode_radio`,
   `support_thematique`, `support_territoire`, le trigger d'exclusivité, les colonnes ajoutées.

2. **Une émission par contenu existant**, FR-055. Identifiant neuf, métadonnées éditoriales reprises,
   cadence `'ponctuelle'` (aucune périodicité n'était déclarée), état repris tel quel :

   ```sql
   INSERT INTO media_content.emission_tele
       (id, chaine_id, titre, slug, description, image_couverture_url,
        info_animateur, info_producteur, langue, theme_phare_id, theme_phare_autre,
        cadence, etat, nombre_signalements, cree_par, created_at, updated_at, deleted_at)
   SELECT gen_random_uuid(), p.chaine_id, p.nom_emission, p.slug || '-programme',
          p.description, p.image_couverture_url, p.info_animateur, p.info_producteur,
          p.langue, p.theme_phare_id, p.theme_phare_autre,
          'ponctuelle', p.etat, p.nombre_signalements, p.cree_par,
          p.created_at, p.updated_at, p.deleted_at
     FROM media_content.programme_tele p;
   ```

   Le suffixe `-programme` sur le slug évite la collision avec le slug de l'épisode, qui est **conservé
   à l'identique** (R2). Les contenus dont `chaine_id IS NULL` reçoivent une émission rattachée à une
   chaîne « Sans chaîne » créée pour l'occasion, ou sont écartés, décision de reprise à trancher au
   moment de l'exécution selon les données réellement présentes.

3. **Un épisode par contenu existant, identifiant et slug conservés**, FR-051, FR-056. `ordre = 0`
   (un seul épisode par émission à ce stade), état repris, `valide_par`/`valide_at` renseignés pour les
   contenus déjà publiés (ils sont réputés validés, hypothèse de la spec) :

   ```sql
   INSERT INTO media_content.episode_tele
       (id, emission_id, titre, slug, description, image_couverture_url, video_url,
        ordre, a_la_une, a_la_une_globale, etat, valide_par, valide_at,
        nombre_signalements, cree_par, created_at, updated_at, deleted_at)
   SELECT p.id, e.id, p.nom_emission, p.slug, p.description, p.image_couverture_url,
          p.video_url, 0, p.a_la_une, p.a_la_une_globale, p.etat,
          CASE WHEN p.etat = 'publie' THEN p.cree_par END,
          CASE WHEN p.etat = 'publie' THEN p.updated_at END,
          p.nombre_signalements, p.cree_par, p.created_at, p.updated_at, p.deleted_at
     FROM media_content.programme_tele p
     JOIN media_content.emission_tele e ON e.slug = p.slug || '-programme';
   ```

4. **Discriminant des interactions** : FR-051, en un `UPDATE` par table, sans toucher `media_id` :

   ```sql
   UPDATE media_content.media_reaction    SET type_media = 'episode_tele'
    WHERE type_media = 'programme_tele';
   -- idem media_commentaire, partage_media, signalement_media ; idem 'programme_radio'
   ```

5. **Créneaux** : FR-058. Chaque créneau désignait un contenu ; il désigne désormais l'émission née de
   ce contenu, `date_effet` valant la date de reprise :

   ```sql
   UPDATE media_content.creneau_programmation c
      SET emission_id = ep.emission_id
     FROM media_content.episode_tele ep
    WHERE c.type_support = 'chaine_tv' AND c.contenu_id = ep.id;
   -- idem episode_radio pour type_support = 'station_radio'
   ```

6. **Thématiques** : FR-057. La catégorie unique de la chaîne devient sa première thématique, par
   correspondance de libellé avec le référentiel `media` :

   ```sql
   INSERT INTO media_content.support_thematique (type_support, support_id, categorie_id)
   SELECT 'chaine_tv', ct.id, cat.id
     FROM media_content.chaine_tv ct
     JOIN shared.categorie cat
       ON cat.contexte = 'media'
      AND lower(cat.nom) = lower(ct.categorie::text)
    WHERE ct.deleted_at IS NULL
   ON CONFLICT DO NOTHING;
   ```

   L'enum `categorie_chaine_tv` et les 44 thèmes `media` ne se recouvrent que partiellement : les
   chaînes sans correspondance restent **sans thématique**, ce que la spec tolère en lecture (edge case
   « chaîne sans thématique héritée »), l'obligation ne valant qu'à la prochaine modification (FR-029).
   Un rapport de couverture est produit à l'exécution : voir `quickstart.md`.

7. **Territoires** : FR-057 : `pays_id` non nul devient l'unique ligne de `support_territoire`.

8. **Supprimer** `media_content.programme_tele` et `media_content.programme_radio` (`DROP TABLE …
   CASCADE`), puis `creneau_programmation.contenu_id`. C'est cette suppression qui fait échouer
   bruyamment toute requête non portée (R1).

---

## 5. Transitions d'état d'un épisode

```
        création par un co-détenteur
                    │
                    ▼
              [en_attente] ──── admin rejette (motif requis) ──▶ [rejete]
                    │                                               │
        admin valide│                                               │ auteur corrige
                    ▼                                               │ et resoumet
               [publie] ◀───────────────────────────────────────────┘
                    │
      signalements > 10 (recompte distinct)  ou  décision admin
                    ▼
              [suspendu] ──── admin rétablit (remet nombre_signalements = 0) ──▶ [publie]
```

- Seul `publie` entre dans la rotation et dans les compteurs publics (FR-018).
- `brouillon` reste réservé aux créations administratives non soumises.
- La désuspension n'est **jamais** automatique, règle déjà en vigueur pour les médias.

---

## 6. Correspondance exigences ↔ modèle

| Exigence | Réalisation |
|----------|-------------|
| FR-002 (appartenance stricte) | `emission_*.chaine_id`/`station_id` NOT NULL ; `episode_*.emission_id` NOT NULL |
| FR-003 (émission sans épisode) | Aucune contrainte n'exige d'épisode |
| FR-005 / FR-006 / FR-007 (ordre) | `ordre INT` + tri `(ordre, created_at, id)` + endpoint de réordonnancement |
| FR-008 (média requis à la publication) | `ck_episode_*_media_publie` |
| FR-010 (suppression refusée) | FK `ON DELETE RESTRICT` |
| FR-018 (attente hors rotation) | `WHERE etat = 'publie'` dans la `JOIN LATERAL` |
| FR-021 (émission sans épisode → non annoncée) | `JOIN LATERAL` intérieure : aucune ligne produite |
| FR-029 / FR-035 (thème et couverture requis) | Garde applicative à la publication (l'existant sans thème doit rester lisible) |
| FR-034 (exclusivité de couverture) | Trigger `verifier_couverture_exclusive` |
| FR-041 (rejet motivé) | `ck_episode_*_rejet_motive` |
| FR-048 (compteurs non agrégés) | Compteurs par `(type_media, media_id)`, aucune vue d'agrégation |
| FR-050 (suspension par niveau) | `nombre_signalements` porté par chaque table, recompte filtré sur la cible |
| FR-052 (mise en avant unique) | Index uniques partiels sur `episode_tele` |
