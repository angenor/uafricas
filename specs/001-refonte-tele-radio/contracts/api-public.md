# Contrat : API publique (sans authentification)

**Feature** : `001-refonte-tele-radio` | Enveloppe standard du projet : `ApiResponse<T>`
(`{ success, data, error }`). Filtre implicite de toute lecture publique :
`etat = 'publie' AND deleted_at IS NULL`.

---

## 1. Page Télé (US1)

### `GET /api/television/vedette`

Programme mis en avant pour toute la page, avec repli déterministe (FR-001, FR-007).

**Réponse** `ProgrammeTeleResponse` enrichi :

```jsonc
{
  "id": "uuid", "slug": "journal-afrique-12-07",
  "nom_emission": "…", "description": "…",
  "image_couverture_url": "/uploads/…", "video_url": "https://youtu.be/… | /uploads/…",
  "source_media": "externe",            // "hebergee" | "externe", pilote le choix du lecteur (FR-056)
  "chaine_id": "uuid", "chaine_nom": "Africa24",
  "est_repli": false,                    // true si servi par défaut faute de vedette désignée
  "nombre_likes": 12, "nombre_dislikes": 1, "nombre_commentaires": 4
}
```

`data: null` si aucun programme publié n'existe, la page affiche alors son message d'état vide, jamais un
lecteur en erreur.

### `GET /api/television/sections`

Une section par chaîne, prête à l'affichage : évite les 100 éléments chargés d'un bloc aujourd'hui et sert
FR-054 / SC-011.

| Paramètre | Défaut | Notes |
|---|---|---|
| `page`, `par_page` | 1, 6 | pagination des **sections**, chargées au défilement |
| `contenus_par_section` | 12 | taille de la rangée horizontale |
| `recherche`, `pays`, `categorie` |, | filtres conservés |

```jsonc
{
  "sections": [{
    "chaine": { "id", "nom", "slug", "description", "cover", "pays", "categorie", "est_en_direct" },
    "mis_en_evidence": { /* ProgrammeTele, a_la_une de la chaîne, sinon le plus récent */ },
    "contenus": [ /* … contenus_par_section éléments, hors mis_en_evidence */ ],
    "total_contenus": 37,
    "diffusion_en_cours": { "contenu_id": "uuid", "fin_prevue": "…" } | null,   // US5
    "creneau_suivant":    { "contenu_id": "uuid", "debut_prevu": "…" } | null
  }],
  "total": 12, "page": 1, "par_page": 6, "total_pages": 2
}
```

**Invariant (FR-008)** : aucune section n'est retournée pour une chaîne sans contenu publié.
**Ordre stable (FR-004)** : `ORDER BY chaine.nom ASC, chaine.id ASC`, déterministe entre deux visites.

### `GET /api/television/chaines/{slug}` · `GET /api/television/programmes/{slug}`

Détail par slug, pour les pages SSR et les aperçus sociaux (R12). Renvoie l'objet complet plus
`nombre_likes`, `nombre_dislikes`, `ma_reaction` (null hors session), `nombre_commentaires`, `nombre_partages`.

---

## 2. Pages Radio (US2)

### `GET /api/stations-radio/sections`

**Paramètre déterminant** : `origine` ∈ `africans` | `territoire`, porté par la page, **jamais** par un
filtre utilisateur (FR-014). `/medias/radio/africans` envoie `africans`, `/medias/radio/nationales` envoie
`territoire`.

Les autres paramètres (`type_station`, `pays`, `genre`, `recherche`) restent des filtres utilisateur et
s'appliquent **en plus** de l'origine.

```jsonc
{
  "sections": [{
    "station": { "id", "nom", "slug", "description", "cover", "pays", "ville",
                 "type_station", "genres_liste", "stream_url" },
    "direct_disponible": true,                 // stream_url non nul, FR-016
    "mis_en_evidence": { /* ProgrammeRadio */ },
    "contenus": [ … ],
    "total_contenus": 8,
    "diffusion_en_cours": null, "creneau_suivant": null
  }],
  "total": 5, "page": 1, "par_page": 6, "total_pages": 1
}
```

### `GET /api/programmes-radio` · `GET /api/programmes-radio/{slug}`

**Comble D-002** : les émissions radio n'ont aujourd'hui aucun endpoint public, alors que leur équivalent
télévision en a trois. Mêmes filtres et même forme que `programmes-vedettes`.

---

## 3. Interactions en lecture (US3)

| Endpoint | Réponse |
|---|---|
| `GET /api/medias/{type_media}/{media_id}/commentaires?page=&par_page=` | liste paginée `{ id, auteur: {id, nom, prenom, photo}, contenu, created_at }`, `deleted_at IS NULL`, `ORDER BY created_at DESC` |
| `GET /api/medias/partages?page=&par_page=` | 8ᵉ source du mur `/publications`, `{ id, legende, created_at, utilisateur, media: {type_media, id, slug, titre, image, support_nom} }` |

`type_media` ∈ `chaine_tv` | `station_radio` | `programme_tele` | `programme_radio`. Valeur hors liste →
**400** ; le handler ne compose jamais de SQL à partir de l'entrée brute (whitelist de littéraux, R3).

Les compteurs et `ma_reaction` sont **portés par les DTO de détail**, pas par un endpoint séparé : cela
évite un aller-retour par carte affichée. `ma_reaction` vaut `null` sans jeton, la lecture restant publique
(FR-027).

---

## 4. Programmation et référentiels

| Endpoint | Rôle |
|---|---|
| `GET /api/medias/{type_support}/{support_id}/grille` | grille complète + `en_cours` + `suivant`, résolus paresseusement en SQL (R7). Horaires renvoyés avec leur `fuseau` explicite (FR-042) |
| `GET /api/medias/themes-phares` | référentiel des 43 thèmes (`shared.categorie`, `contexte='media'`, `actif = TRUE`, `ORDER BY ordre, nom`) |
| `GET /api/medias/roles-partie-prenante` | 9 valeurs + `autre` |

---

## 5. Codes d'erreur

| Code | Cas |
|---|---|
| 400 | `type_media` / `type_support` / `origine` hors whitelist ; pagination invalide |
| 404 | slug inconnu, ou objet dont `etat <> 'publie'`, un contenu retiré est indiscernable d'un contenu inexistant (FR-028) |
| 429 | quota de lecture dépassé (rate limit nginx existant, 30 r/s) |

**Contrat de suspension** : un contenu passé en `etat = 'suspendu'` par franchissement du seuil de
signalements (FR-050) disparaît de tous les endpoints publics à la requête suivante, y compris des
sections et de la vedette, qui basculent sur leur repli.
