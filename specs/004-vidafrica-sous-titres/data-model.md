# Data Model — Vidafrica (sous-titrage vidéo multilingue karaoke)

**Branch**: `004-vidafrica-sous-titres` | **Date**: 2026-04-13 | **Schema**: `media_content`

## Enum

### `langue_sous_titre`

```
francais | anglais | arabe | portugais | swahili | wolof | haoussa |
amharique | zoulou | lingala | bambara | yoruba | peul | espagnol | mandarin
```

## Entités

### 1. `media_content.video`

Vidéo uploadée localement par un admin.

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK, DEFAULT uuid_generate_v4() | Identifiant unique |
| `titre` | VARCHAR(300) | NOT NULL | Titre de la vidéo |
| `slug` | VARCHAR(350) | NOT NULL, UNIQUE (WHERE deleted_at IS NULL) | Slug URL-friendly |
| `description` | TEXT | | Description longue |
| `fichier_video_url` | VARCHAR(500) | NOT NULL | Chemin relatif du fichier vidéo (`/uploads/videos/...`) |
| `vignette_url` | VARCHAR(500) | | Chemin relatif de la vignette (`/uploads/vignettes/...`) |
| `duree_secondes` | INTEGER | | Durée en secondes |
| `taille_octets` | BIGINT | | Taille du fichier en octets |
| `format_video` | VARCHAR(20) | | Format (mp4, webm) |
| `etat` | VARCHAR(50) | NOT NULL, DEFAULT 'brouillon', CHECK IN ('brouillon','publie','suspendu','supprime') | État de publication |
| `cree_par` | UUID | NOT NULL, xref iam.utilisateur | Admin créateur |
| `search_vector` | TSVECTOR | | Recherche full-text (titre + description) |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Date de création |
| `updated_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Dernière modification |
| `deleted_at` | TIMESTAMPTZ | | Soft delete |

**Indexes** :
- `idx_video_slug` : UNIQUE sur `slug` WHERE `deleted_at IS NULL`
- `idx_video_etat` : sur `etat` WHERE `deleted_at IS NULL`
- `idx_video_search` : GIN sur `search_vector`
- `idx_video_cree_par` : sur `cree_par`

---

### 2. `media_content.piste_sous_titre`

Piste de sous-titres dans une langue donnée, associée à une vidéo.

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK, DEFAULT uuid_generate_v4() | Identifiant unique |
| `video_id` | UUID | NOT NULL, FK → media_content.video(id) | Vidéo parente |
| `langue` | langue_sous_titre | NOT NULL | Langue de la piste |
| `est_complete` | BOOLEAN | NOT NULL, DEFAULT false | Indique si tous les segments ont un timing mot par mot |
| `cree_par` | UUID | NOT NULL, xref iam.utilisateur | Admin créateur |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Date de création |
| `updated_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Dernière modification |
| `deleted_at` | TIMESTAMPTZ | | Soft delete |

**Contraintes** :
- UNIQUE (`video_id`, `langue`) WHERE `deleted_at IS NULL` — une seule piste par langue par vidéo

**Indexes** :
- `idx_piste_sous_titre_video` : sur `video_id` WHERE `deleted_at IS NULL`
- `idx_piste_sous_titre_unique_langue` : UNIQUE sur (`video_id`, `langue`) WHERE `deleted_at IS NULL`

---

### 3. `media_content.segment_sous_titre`

Segment individuel (cue) d'une piste de sous-titres.

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK, DEFAULT uuid_generate_v4() | Identifiant unique |
| `piste_id` | UUID | NOT NULL, FK → media_content.piste_sous_titre(id) | Piste parente |
| `position` | INTEGER | NOT NULL | Ordre du segment dans la piste (1, 2, 3...) |
| `texte` | TEXT | NOT NULL | Texte complet du segment |
| `debut_ms` | INTEGER | NOT NULL | Timestamp de début en millisecondes |
| `fin_ms` | INTEGER | NOT NULL | Timestamp de fin en millisecondes |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Date de création |
| `updated_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Dernière modification |

**Contraintes** :
- CHECK (`debut_ms` < `fin_ms`) — le début doit précéder la fin
- CHECK (`debut_ms` >= 0) — pas de timestamp négatif
- UNIQUE (`piste_id`, `position`) — pas de doublons de position

**Indexes** :
- `idx_segment_piste_position` : sur (`piste_id`, `position`)
- `idx_segment_piste_debut` : sur (`piste_id`, `debut_ms`)

---

### 4. `media_content.timing_mot`

Timing individuel d'un mot dans un segment, pour l'effet karaoké.

| Colonne | Type | Contraintes | Description |
|---------|------|-------------|-------------|
| `id` | UUID | PK, DEFAULT uuid_generate_v4() | Identifiant unique |
| `segment_id` | UUID | NOT NULL, FK → media_content.segment_sous_titre(id) | Segment parent |
| `position` | INTEGER | NOT NULL | Ordre du mot dans le segment (1, 2, 3...) |
| `mot` | VARCHAR(200) | NOT NULL | Le mot tel qu'affiché |
| `debut_ms` | INTEGER | NOT NULL | Timestamp de début en millisecondes |
| `fin_ms` | INTEGER | NOT NULL | Timestamp de fin en millisecondes |

**Contraintes** :
- CHECK (`debut_ms` < `fin_ms`)
- CHECK (`debut_ms` >= 0)
- UNIQUE (`segment_id`, `position`) — pas de doublons de position

**Indexes** :
- `idx_timing_mot_segment_position` : sur (`segment_id`, `position`)

---

## Relations

```
media_content.video (1) ──── (N) media_content.piste_sous_titre
                                    │
                                    (1) ──── (N) media_content.segment_sous_titre
                                                    │
                                                    (1) ──── (N) media_content.timing_mot
```

- Une **vidéo** possède 0 à N **pistes de sous-titres** (une par langue, max 1 par langue)
- Une **piste** possède 0 à N **segments** ordonnés par `position`
- Un **segment** possède 0 à N **timings mot** ordonnés par `position` (optionnel — si absent, affichage en bloc)

## Transitions d'état

### Vidéo

```
brouillon → publie → suspendu → publie (réactivation)
                   → supprime (soft delete)
brouillon → supprime
suspendu → supprime
```

- **brouillon** : Vidéo créée, pas encore visible publiquement. Sous-titres en cours de saisie.
- **publie** : Vidéo visible sur la page publique Vidafrica.
- **suspendu** : Vidéo temporairement retirée du public (masquée).
- **supprime** : Soft delete via `deleted_at`.

## Cascade de suppression

- Suppression d'une **vidéo** (soft delete) → les pistes restent intactes (possibilité de restauration)
- Suppression d'une **piste** (soft delete) → les segments et timings mot restent intacts
- Suppression physique d'un **segment** → CASCADE DELETE sur les timings mot associés
