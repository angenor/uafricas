# Contrat — API membre (co-détenteurs de supports)

**Authentification** : JWT membre. **Autorisation** : `garde_detenteur(pool, utilisateur_id,
type_support, support_id, role_minimum)` — jamais `AdminUtilisateur`, ce sont des routes membres
(règle posée par la feature 001, lot 3).

Rôles de détention (enum `media_content.role_detenteur`, 09m) :

| Rôle | Émissions | Épisodes | Grille | Fiche du support |
|------|-----------|----------|--------|------------------|
| `proprietaire` | ✅ | ✅ | ✅ | ✅ + inviter/révoquer |
| `co_detenteur` | ✅ | ✅ | ✅ | ✅ |
| `programmateur` | ❌ | ❌ | ✅ | ❌ |

---

## 1. Émissions

### `POST /api/medias/{type_support}/{support_id}/emissions`
Rôle minimum : `co_detenteur`.

```jsonc
{
  "titre": "Débats africains",
  "description": "…",
  "cadence": "hebdomadaire",              // quotidienne | hebdomadaire | ponctuelle
  "image_couverture_url": "/uploads/…",   // facultatif
  "info_animateur": "…", "info_producteur": "…",
  "langue": "Français",
  "theme_phare_id": "…", "theme_phare_autre": null
}
```

`201` → `{ "id": "…", "slug": "…" }`. L'émission naît `etat = 'brouillon'` **sans épisode** (FR-003) ;
elle passe `publie` dès qu'un épisode l'est. Le slug est dérivé du titre et unique.

### `PUT /api/medias/emissions/{id}` · `DELETE /api/medias/emissions/{id}`

La suppression est refusée en `409` tant que l'émission compte des épisodes publiés (FR-010), avec le
décompte dans le message. La clé étrangère `ON DELETE RESTRICT` fait le même refus en dernier recours.

### `GET /api/medias/{type_support}/{support_id}/emissions`

Vue détenteur : **toutes** les émissions, y compris `brouillon` et sans épisode, avec pour chacune le
décompte par état (`en_attente`, `publie`, `rejete`) — c'est le tableau de bord de FR-042.

---

## 2. Épisodes

### `POST /api/medias/emissions/{id}/episodes`
Rôle minimum : `co_detenteur`. Multipart (fichier) ou JSON (lien), comme
`POST /api/admin/medias/upload` aujourd'hui.

```jsonc
{
  "titre": "Épisode 12 — La dette africaine",
  "description": "…",
  "video_url": "/uploads/medias/videos/…",   // ou audio_url côté radio
  "numero_episode": 12,                       // facultatif
  "duree_minutes": 58,
  "image_couverture_url": "/uploads/…"
}
```

`201` → `{ "id": "…", "slug": "…", "etat": "en_attente", "ordre": 12 }`.

**Invariants** (FR-007, FR-040) :

- `etat` vaut **toujours** `en_attente` — le client ne peut pas en décider ; toute valeur transmise est
  ignorée.
- `ordre = COALESCE(MAX(ordre), -1) + 1` sur l'émission : l'épisode prend rang **à la fin**, sans
  déplacer les existants ni altérer l'occurrence en cours (FR-019).
- L'épisode n'entre ni dans la rotation, ni dans les compteurs publics, ni dans la liste publique de son
  émission tant qu'il n'est pas validé (FR-018).

### `PUT /api/medias/episodes/{id}`

Modifier un épisode `publie` **remet son état à `en_attente`** si le média change (`video_url` /
`audio_url`), comme le fait déjà `PUT …/media` sur les propositions. Une modification purement
éditoriale (titre, description, image) reste publiée.

Un épisode `rejete` que l'on modifie repasse `en_attente` et son `motif_rejet` est effacé — c'est le
parcours de correction-resoumission de FR-041.

### `DELETE /api/medias/episodes/{id}`

Suppression douce (`deleted_at`). Le cycle de rotation se recalcule à la lecture suivante (FR-019) ;
aucune action supplémentaire n'est requise.

### `PUT /api/medias/emissions/{id}/episodes/reordonner`

```jsonc
{ "ordres": [ { "episode_id": "…", "ordre": 0 }, { "episode_id": "…", "ordre": 1 } ] }
```

Réécriture **atomique** — tout réordonner ou rien (patron de
`admin/formation_contenu.rs:350`). `400` si la liste ne couvre pas exactement les épisodes de
l'émission. Le nouvel ordre s'applique à partir de l'occurrence suivante (FR-006).

### `PATCH /api/medias/episodes/{id}/emission`

```jsonc
{ "emission_id": "…" }
```

Déplace un épisode vers une autre émission **du même support** (`400` sinon). L'épisode conserve
intégralement ses interactions — rien à faire, elles sont indexées par `(type_media, media_id)` et ni
l'un ni l'autre ne change (FR-009). Il prend rang en fin de la nouvelle émission ; les deux cycles se
recalculent.

### `PATCH /api/medias/episodes/{id}/a-la-une`

Désigne l'épisode mis en avant pour son support. La bascule de l'ancien à `FALSE` et la désignation du
nouveau tiennent dans **une même transaction** — sinon l'index unique partiel est violé en concurrence
(règle héritée de 09j §3, R9).

---

## 3. Grille de programmation

### `POST /api/medias/{type_support}/{support_id}/creneaux`
Rôle minimum : `programmateur`.

```jsonc
{
  "emission_id": "…",              // remplace contenu_id
  "recurrence": "hebdomadaire",    // quotidien | hebdomadaire
  "jour_semaine": 6,               // requis si hebdomadaire, interdit si quotidien
  "heure_debut": "18:00",
  "duree_minutes": 60,
  "fuseau": "Africa/Abidjan",
  "date_effet": "2026-08-08"       // facultatif, défaut = aujourd'hui
}
```

Gardes conservées de 09n / `media_programmation.rs` :

- Verrou `FOR UPDATE` sur le **support parent** avant détection de chevauchement, puis `409` sans
  écriture si conflit, avec la plage en cause (FR-022).
- `400` si le créneau franchit minuit (FR-023) — le CHECK SQL refuse aussi.
- `400` si l'émission n'appartient pas au support visé.

**Nouveau** : `date_effet` est l'origine du comptage des occurrences. La changer **redéfinit la
rotation** — l'API renvoie dans la réponse `episode_actuel` pour que le détenteur voie immédiatement
l'effet de son choix.

### `PUT /api/medias/creneaux/{id}` · `DELETE /api/medias/creneaux/{id}`

Inchangés, `contenu_id` devenant `emission_id`.

### `GET /api/medias/{type_support}/{support_id}/grille?vue=detenteur`

Comme la grille publique, plus les créneaux **en défaut** — ceux dont l'émission n'a aucun épisode
publié — assortis de `alerte: "aucun_episode_publie"` (FR-021, FR-024).

---

## 4. Fiche du support : thématiques et couverture

### `PUT /api/medias/{type_support}/{support_id}/thematiques`

```jsonc
{ "categorie_ids": ["…", "…", "…"] }
```

Remplacement intégral. `400` si la liste est vide sur un support publié (FR-029), ou si un identifiant
ne relève pas du contexte `media`.

### `PUT /api/medias/{type_support}/{support_id}/couverture`

```jsonc
{ "couverture_continentale": false, "pays_ids": ["…", "…"] }
```

- `400` si `couverture_continentale = true` **et** `pays_ids` non vide (FR-034) ; le trigger SQL refuse
  aussi, mais le message d'API doit être lisible.
- `400` si `couverture_continentale = false` **et** `pays_ids` vide sur un support publié (FR-035).
- Le passage à `true` supprime les lignes de `support_territoire` **dans la même transaction**.

---

## 5. Alertes de cadence

### `GET /api/medias/mes-alertes-cadence`

Alimente la notification de FR-024. Calcul **à la lecture**, aucune tâche de fond : pour chaque émission
périodique d'un support détenu, comparer la date du dernier épisode publié à la cadence déclarée.

```jsonc
{
  "alertes": [{
    "emission": { "id": "…", "titre": "…", "slug": "…" },
    "support":  { "type": "chaine_tv", "id": "…", "nom": "…" },
    "cadence": "hebdomadaire",
    "dernier_episode_at": "2026-07-25T18:00:00Z",
    "prochaine_echeance": "2026-08-09",
    "niveau": "approche",            // approche | depassee | aucun_episode
    "episodes_en_attente": 1
  }]
}
```

- `approche` se déclenche **2 jours avant** l'échéance en cadence hebdomadaire, **6 heures avant** en
  quotidienne : la marge doit absorber le délai de validation administrative (FR-024).
- `episodes_en_attente` évite l'alerte trompeuse — le détenteur a fait sa part, la file n'a pas suivi.
- `aucun_episode` correspond à l'émission programmée mais vide (FR-021).
