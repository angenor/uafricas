# Contrat — API publique (lecture, sans authentification)

Toutes les réponses sont enveloppées dans `ApiResponse<T>` (`{ succes, message, data }`), convention du
projet. Toute lecture publique filtre `etat = 'publie' AND deleted_at IS NULL`, sur l'émission **comme**
sur l'épisode (FR-011).

Rappel de vocabulaire : `emission` en API = « Programme » à l'écran ; `episode` = « Épisode ».

---

## 1. Espace Télé

### `GET /api/television/sections`

Inchangé dans sa forme, **enrichi dans son contenu** : chaque chaîne porte désormais ses émissions, et
chaque émission un extrait de ses épisodes.

Paramètres existants conservés : `origine`, `theme`, `en_direct`.
Paramètres ajoutés : `territoire` (UUID de pays — remonte aussi les chaînes continentales, FR-036),
`thematique` (UUID de catégorie, répétable).

```jsonc
{
  "chaines": [{
    "id": "…", "nom": "…", "slug": "…", "image_couverture_url": "…",
    "origine_publication": "territoire",
    "couverture_continentale": false,
    "territoires":  [{ "id": "…", "nom": "Côte d'Ivoire" }],
    "thematiques":  [{ "id": "…", "nom": "Débats africains" }],
    "diffusion_en_cours": {
      "creneau_id": "…", "heure_debut": "18:00", "duree_minutes": 60, "fuseau": "Africa/Abidjan",
      "emission": { "id": "…", "titre": "Débats africains", "slug": "…" },
      "episode":  { "id": "…", "titre": "Épisode 12", "slug": "…", "video_url": "…" },
      "est_rediffusion": false
    },
    "creneau_suivant": { "…": "même forme" },
    "emissions": [{
      "id": "…", "titre": "…", "slug": "…", "image_couverture_url": "…",
      "cadence": "hebdomadaire",
      "theme_phare": { "id": "…", "nom": "…" },
      "nombre_episodes": 12,
      "dernier_episode_at": "2026-08-02T18:00:00Z",
      "episodes_apercu": [ { "id": "…", "titre": "…", "slug": "…", "image_couverture_url": "…" } ]
    }]
  }]
}
```

- `nombre_episodes` et `dernier_episode_at` ne comptent que les épisodes **publiés** (FR-012).
- `episodes_apercu` est borné à 12 épisodes, les plus récents d'abord. Au-delà, la page d'émission.
- Une émission sans épisode publié **n'apparaît pas** (FR-011, US1 §6).
- `est_rediffusion` vaut `true` quand la rotation a déjà bouclé (FR-020).

### `GET /api/television/vedette`

Renvoie l'**épisode** portant `a_la_une_globale`, accompagné de son émission et de sa chaîne (FR-052).

### `GET /api/television/emissions/slug/{slug}`

Détail d'une émission. `404` si l'émission n'est pas publiée ou n'a aucun épisode publié.

```jsonc
{
  "id": "…", "titre": "…", "slug": "…", "description": "…",
  "cadence": "hebdomadaire", "image_couverture_url": "…",
  "info_animateur": "…", "info_producteur": "…", "langue": "Français",
  "theme_phare": { "id": "…", "nom": "…" },
  "chaine": { "id": "…", "nom": "…", "slug": "…" },
  "nombre_episodes": 12,
  "creneaux": [{ "recurrence": "hebdomadaire", "jour_semaine": 6,
                 "heure_debut": "18:00", "duree_minutes": 60, "fuseau": "Africa/Abidjan" }],
  "compteurs": { "likes": 34, "dislikes": 2, "commentaires": 8, "partages": 5 }
}
```

`compteurs` sont ceux de **l'émission seule** — jamais la somme de ceux de ses épisodes (FR-048).

### `GET /api/television/emissions/{id}/episodes`

Liste paginée des épisodes publiés, `?page=&taille=` (défaut 24), triés `(ordre, created_at, id)`.
C'est ce qui tient la promesse de navigabilité à 500 épisodes (SC-009).

### `GET /api/television/episodes/slug/{slug}`

**Remplace `GET /api/television/programmes/slug/{slug}`.** Les slugs étant conservés (R2), les adresses
publiques existantes continuent de résoudre (FR-056).

```jsonc
{
  "id": "…", "titre": "…", "slug": "…", "description": "…",
  "video_url": "…", "image_couverture_url": "…",
  "numero_episode": 12, "duree_minutes": 58,
  "publie_at": "2026-08-02T18:00:00Z",
  "emission": { "id": "…", "titre": "…", "slug": "…" },
  "chaine":   { "id": "…", "nom": "…", "slug": "…" },
  "episodes_voisins": [ { "id": "…", "titre": "…", "slug": "…" } ],
  "compteurs": { "likes": 120, "dislikes": 3, "commentaires": 41, "partages": 17 }
}
```

`episodes_voisins` sert US1 §4 (« propose les autres épisodes de la même émission »).

### Référentiels de filtre

- `GET /api/television/thematiques` — thèmes `media` **réellement déclarés** par au moins une chaîne
  publiée, avec leur décompte. Même principe que `GET /api/experts/specialites`.
- `GET /api/television/territoires` — territoires réellement couverts, plus un marqueur
  `{ "continentales": 4 }` indiquant le nombre de chaînes panafricaines.

> `GET /api/television/categories` et `GET /api/television/pays` sont **conservés** le temps du portage
> frontend, puis retirés : ils exposent la catégorie et le pays uniques, remplacés par les deux routes
> ci-dessus.

---

## 2. Espace Radio

Strictement symétrique, sur le préfixe `/api/stations-radio` :

| Route | Rôle |
|-------|------|
| `GET /api/stations-radio/sections` | + `territoire`, `thematique`, émissions et diffusion |
| `GET /api/stations-radio/emissions/slug/{slug}` | Détail d'une émission radio |
| `GET /api/stations-radio/emissions/{id}/episodes` | Épisodes paginés |
| `GET /api/stations-radio/episodes/slug/{slug}` | Détail d'un épisode (remplace `/api/programmes-radio/slug/{slug}`) |
| `GET /api/stations-radio/thematiques`, `…/territoires` | Référentiels de filtre |

Différence unique : `audio_url` au lieu de `video_url`, et pas de vedette plein écran.

---

## 3. Grille et diffusion (commun aux deux)

### `GET /api/medias/{type_support}/{support_id}/grille`

`type_support` ∈ `chaine_tv` | `station_radio`. La grille désigne désormais des **émissions**
(FR-014).

```jsonc
{
  "creneaux": [{
    "id": "…", "recurrence": "hebdomadaire", "jour_semaine": 6,
    "heure_debut": "18:00", "duree_minutes": 60,
    "fuseau": "Africa/Abidjan", "date_effet": "2026-08-08",
    "emission": { "id": "…", "titre": "…", "slug": "…", "cadence": "hebdomadaire",
                  "nombre_episodes": 12 }
  }]
}
```

Un créneau dont l'émission n'a aucun épisode publié **n'est pas renvoyé** au public (FR-021) ; il reste
visible du détenteur via l'API membre, assorti d'une alerte.

### `GET /api/medias/{type_support}/{support_id}/diffusion`

Deux requêtes, comme aujourd'hui — la rotation s'ajoute en `JOIN LATERAL` sans requête supplémentaire
(research.md R3).

```jsonc
{
  "diffusion_en_cours": {
    "creneau_id": "…", "heure_debut": "18:00", "duree_minutes": 60, "fuseau": "Africa/Abidjan",
    "emission": { "id": "…", "titre": "…", "slug": "…" },
    "episode":  { "id": "…", "titre": "…", "slug": "…", "video_url": "…",
                  "numero_episode": 12 },
    "rang_occurrence": 37,
    "est_rediffusion": true
  },
  "creneau_suivant": { "…": "même forme" }
}
```

**Invariants de rotation** (FR-016, FR-017) :

- `rang_occurrence` est le nombre d'occurrences écoulées depuis `date_effet`, calculé dans `fuseau`.
- L'épisode est celui de rang `rang_occurrence mod nombre_episodes_publies`.
- Deux appels dans la même plage renvoient **le même `episode.id`** (SC-006).
- `est_rediffusion = rang_occurrence >= nombre_episodes_publies`.
- Si l'émission n'a aucun épisode publié, `diffusion_en_cours` vaut `null`.

---

## 4. Interactions

`GET|POST /api/medias/{type_media}/{media_id}/reactions`, `…/commentaires`, `…/partages`,
`…/signalements` — routes existantes, `type_media` passant de 4 à 6 valeurs :

```
chaine_tv | station_radio | emission_tele | emission_radio | episode_tele | episode_radio
```

Une valeur hors de cette liste renvoie `400`. Les anciennes valeurs `programme_tele` /
`programme_radio` sont **rejetées** après migration — c'est volontaire : un client non porté échoue
visiblement plutôt que d'écrire sur une cible fantôme.

Les compteurs restent servis par cible ; `compteurs_pour` conserve sa forme (deux requêtes pour toute
une page).
