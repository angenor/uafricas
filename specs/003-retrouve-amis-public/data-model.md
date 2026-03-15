# Data Model: 003-retrouve-amis-public

**Date**: 2026-03-15 | **Branch**: `003-retrouve-amis-public`

## Nouveaux Enums

### `genre_personne`
```sql
CREATE TYPE retrouve_amis.genre_personne AS ENUM ('homme', 'femme');
```

### `type_relation_recherche`
```sql
CREATE TYPE retrouve_amis.type_relation_recherche AS ENUM (
    'amis_enfance',
    'amis_ecole',
    'collegue',
    'connaissance',
    'frere_soeur',
    'parent'
);
```

## Modifications de `retrouve_amis.avis_recherche`

### Colonnes ajoutées

| Colonne | Type | Nullable | Default | Contrainte | Description |
|---------|------|----------|---------|------------|-------------|
| `est_anonyme` | BOOLEAN | NOT NULL | FALSE | — | L'auteur souhaite rester anonyme |
| `genre_recherche` | `genre_personne` | YES | NULL | — | Genre de la personne recherchée |
| `type_relation` | `type_relation_recherche` | YES | NULL | — | Type de relation avec la personne |
| `comment_connu` | VARCHAR(500) | YES | NULL | — | Comment la personne connaît l'auteur |
| `localite_rencontre` | VARCHAR(200) | YES | NULL | — | Localité du lieu de rencontre |
| `ecole_rencontre` | VARCHAR(250) | YES | NULL | — | École du lieu de rencontre |
| `ville_rencontre` | VARCHAR(200) | YES | NULL | — | Ville du lieu de rencontre |
| `jamais_rencontre` | BOOLEAN | NOT NULL | FALSE | — | Jamais rencontré en personne |
| `photo_url` | VARCHAR(500) | YES | NULL | — | Chemin vers la photo uploadée |
| `description_physique` | TEXT | YES | NULL | — | Description physique libre |
| `partage_coordonnees` | BOOLEAN | NOT NULL | FALSE | — | Souhaite partager ses coordonnées |
| `coordonnees_email` | VARCHAR(250) | YES | NULL | — | Email de contact (non public) |
| `coordonnees_telephone` | VARCHAR(50) | YES | NULL | — | Téléphone de contact (non public) |
| `coordonnees_whatsapp` | VARCHAR(50) | YES | NULL | — | WhatsApp de contact (non public) |

### Colonnes modifiées

| Colonne | Avant | Après | Raison |
|---------|-------|-------|--------|
| `est_public` | `DEFAULT FALSE` | `DEFAULT TRUE` | Tout public par défaut |

### Contraintes ajoutées

```sql
-- Au moins un lieu de rencontre OU jamais_rencontre
ALTER TABLE retrouve_amis.avis_recherche
ADD CONSTRAINT chk_lieu_ou_jamais CHECK (
    localite_rencontre IS NOT NULL
    OR ecole_rencontre IS NOT NULL
    OR ville_rencontre IS NOT NULL
    OR jamais_rencontre = TRUE
    OR type_relation IS NOT NULL
);

-- Si partage_coordonnees = true, au moins une coordonnée requise
ALTER TABLE retrouve_amis.avis_recherche
ADD CONSTRAINT chk_coordonnees_requises CHECK (
    partage_coordonnees = FALSE
    OR coordonnees_email IS NOT NULL
    OR coordonnees_telephone IS NOT NULL
    OR coordonnees_whatsapp IS NOT NULL
);
```

### Index ajoutés

```sql
-- Index pour le filtrage par type de relation
CREATE INDEX idx_avis_type_relation
ON retrouve_amis.avis_recherche (type_relation)
WHERE deleted_at IS NULL AND etat = 'actif';

-- Index trigram sur les lieux de rencontre
CREATE INDEX idx_avis_localite_trgm
ON retrouve_amis.avis_recherche USING gin (localite_rencontre gin_trgm_ops)
WHERE deleted_at IS NULL;

CREATE INDEX idx_avis_ecole_rencontre_trgm
ON retrouve_amis.avis_recherche USING gin (ecole_rencontre gin_trgm_ops)
WHERE deleted_at IS NULL;

CREATE INDEX idx_avis_ville_rencontre_trgm
ON retrouve_amis.avis_recherche USING gin (ville_rencontre gin_trgm_ops)
WHERE deleted_at IS NULL;
```

### Mise à jour du search_vector

Le calcul du `search_vector` est étendu pour inclure les nouveaux champs :

```sql
-- Ajout dans le trigger ou la requête de construction du TSVECTOR
search_vector = setweight(to_tsvector('french', COALESCE(nom_recherche, '')), 'A')
    || setweight(to_tsvector('french', COALESCE(prenom_recherche, '')), 'A')
    || setweight(to_tsvector('french', COALESCE(surnom, '')), 'B')
    || setweight(to_tsvector('french', COALESCE(ecole, '')), 'B')
    || setweight(to_tsvector('french', COALESCE(ecole_rencontre, '')), 'B')
    || setweight(to_tsvector('french', COALESCE(ville, '')), 'C')
    || setweight(to_tsvector('french', COALESCE(ville_rencontre, '')), 'C')
    || setweight(to_tsvector('french', COALESCE(localite_rencontre, '')), 'C')
    || setweight(to_tsvector('french', COALESCE(comment_connu, '')), 'D')
    || setweight(to_tsvector('french', COALESCE(description, '')), 'D')
    || setweight(to_tsvector('french', COALESCE(description_physique, '')), 'D');
```

## Entités inchangées

Les tables suivantes ne sont **pas modifiées** :
- `retrouve_amis.correspondance` — réutilisée telle quelle
- `retrouve_amis.parcours_trouvable` — réutilisée telle quelle
- `retrouve_amis.blacklist` — réutilisée telle quelle
- `retrouve_amis.signalement` — réutilisée telle quelle
- `retrouve_amis.notification_retrouve` — réutilisée telle quelle
- `retrouve_amis.reponse_publique` — réutilisée telle quelle
- `retrouve_amis.demande_retrait` — réutilisée telle quelle
- `iam.utilisateur` (colonne `est_trouvable`) — inchangée

## Diagramme des relations (entités impactées)

```
iam.utilisateur (est_trouvable)
    │
    ├──< avis_recherche (auteur_id) ── MODIFIÉE : +14 colonnes
    │       │
    │       ├──< correspondance (avis_id)
    │       ├──< signalement (avis_id)
    │       ├──< reponse_publique (avis_id)
    │       └──< demande_retrait (avis_id)
    │
    └──< parcours_trouvable (utilisateur_id)
```

## Migration

La migration doit :
1. Créer les 2 nouveaux enums
2. Ajouter les 14 nouvelles colonnes (toutes nullable ou avec default → pas de downtime)
3. Modifier le default de `est_public` à TRUE
4. Mettre à jour les avis existants : `SET est_public = TRUE` pour tous les avis actifs
5. Ajouter les contraintes CHECK
6. Ajouter les index
7. Mettre à jour le calcul du search_vector
