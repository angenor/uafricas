# Contrat — API publique (lecture)

**Feature**: 010-medias-equipes-vitrine

**Aucune route publique n'est créée.** L'équipe voyage dans les payloads existants (décision D7). Ce document décrit ce qui **change** dans les réponses déjà servies.

---

## 1. `GET /api/television/sections` — et son pendant `GET /api/stations-radio/sections`

### Ce qui s'ajoute

`sections[].chaine.equipe` (resp. `sections[].station.equipe`) — tableau ordonné, **omis quand vide** :

```json
{
  "sections": [{
    "chaine": {
      "id": "…", "nom": "Africa TV", "slug": "africa-tv",
      "description": "…", "image_couverture_url": "…",
      "equipe": [
        { "id": "…", "nom": "Diallo", "prenom": "Aminata",
          "fonction": "Directrice", "territoire": "Sénégal",
          "contact": "direction@africatv.example", "utilisateur_id": "…", "ordre": 0 },
        { "id": "…", "nom": "Kouassi", "fonction": "Producteur", "ordre": 1 }
      ],
      "thematiques": [ … ], "couverture": { … }, "interactions": { … }
    },
    "emissions": [
      { "id": "…", "titre": "Le Grand Débat", "slug": "le-grand-debat",
        "description": "…", "image_couverture_url": "…",
        "cadence": "hebdomadaire", "nombre_episodes": 12 }
    ],
    "total_emissions": 3,
    "diffusion_en_cours": { … },
    "creneau_suivant": { … }
  }],
  "total": 14, "page": 1, "par_page": 6, "total_pages": 3
}
```

### Ce qui disparaît

- **`emissions[].episodes_apercu`** — les sections ne rendent plus d'épisode (FR-002). Le champ était déjà `skip_serializing_if = "Vec::is_empty"` ; il devient systématiquement absent parce que `greffer_apercus_et_compteurs` n'est plus appelé.
- **`emissions[].interactions`** — même raison : plus de barre de réaction sur un contenu qui n'est plus affiché. Les `interactions` de la **chaîne** sont conservées.

### Ce qui change de comportement — attention au recettage

| Avant | Après | Exigence |
|---|---|---|
| Une chaîne n'apparaît que si elle a au moins un épisode publié (`EXISTS` du WHERE, `television.rs:376-381`) | Toute chaîne publiée apparaît | Cas limite « chaîne sans programme » |
| Côté radio, filtrage a posteriori `sections.retain(…)` (`stations_radio.rs:468`), qui désaccordait `total` du nombre de sections servies | Filtre supprimé ; `total` redevient exact | idem + cohérence de la pagination |
| Un programme sans épisode publié est écarté (`JOIN LATERAL … ON agg.nombre_episodes > 0`) | Il est listé, avec `nombre_episodes: 0` | FR-005 |
| `contenus_par_section` vaut **12** par défaut et plafonne à **30** | Il vaut **30** par défaut et plafonne à **60** | FR-008 |

**Détection de troncature, sans champ neuf** : `total_emissions` est déjà servi par section. Le client compare `emissions.length` à `total_emissions` ; s'ils diffèrent, il annonce le total et renvoie vers la page de la chaîne (FR-008). Le plafond était jusqu'ici sans conséquence — il bornait un aperçu ; il borne désormais le **contenu principal** de la section.

> **Effet visible attendu** : des chaînes et stations jusqu'ici invisibles vont apparaître sur les vitrines, et les compteurs affichés vont augmenter. C'est le comportement demandé, mais c'est un changement de **contenu servi**, pas seulement de présentation.

### Ce qui ne change pas

`diffusion_en_cours` et `creneau_suivant` restent servis : le bandeau de programmation est conservé en vitrine (Q3 → A). `direct_disponible` reste sur les sections radio.

---

## 2. `GET /api/television/chaines/slug/{slug}` — et `GET /api/stations-radio/slug/{slug}`

Réponse `{ chaine, emissions, total_emissions }` (resp. `station`).

**Ajouts** :
- `chaine.equipe` — l'équipe du support, complète et ordonnée.
- `emissions[].equipe` — **l'équipe propre à chaque programme** (FR-025), indépendante de celle du support.
- `emissions[].episodes_apercu` reste servi : c'est la page qui liste les vidéos (FR-027).

Aucune troncature serveur : le repli au-delà d'un seuil (FR-021, FR-024) est une décision d'affichage, prise côté client.

---

## 3. `GET /api/television/emissions/slug/{slug}` — et `GET /api/stations-radio/emissions/slug/{slug}`

**Ajout** : `equipe` — l'équipe du programme (FR-030, FR-032). Jamais celle du support en repli : un programme sans équipe déclarée renvoie un champ absent, et la page n'affiche pas de bloc.

**Champs conservés mais dépubliés** : `info_animateur` et `info_producteur` restent servis par l'API (les formulaires d'édition les lisent encore, libellés « hérité »), mais **cessent d'être affichés au visiteur** (FR-034). Ne pas les retirer du DTO : ce sont les seules traces des saisies antérieures, tant que les gestionnaires ne les ont pas reportées dans l'équipe.

**Changement de comportement** :

| Avant | Après |
|---|---|
| `404` si l'émission est publiée mais n'a **aucun épisode publié** (`media_emission.rs:278`) | `200`, avec `nombre_episodes: 0` et `episodes_apercu` absent |

Le `404` ne subsiste que si l'émission elle-même n'est pas publiée. Exigence : FR-033 — « un programme sans vidéo publiée DOIT rester consultable ».

---

## 4. Forme du membre d'équipe (partagée par tous les payloads)

```ts
interface MembreEquipeAPI {
  id: string
  nom: string           // toujours présent
  prenom?: string       // omis si non saisi
  fonction: string      // toujours présent
  territoire?: string   // omis si non saisi
  contact?: string      // omis si non saisi
  utilisateur_id?: string  // omis si non rattaché OU si le compte est supprimé
  ordre: number         // rang d'affichage, croissant
}
```

Trois garanties de contrat :

1. **Les champs vides sont omis, jamais servis à `""` ou `null`.** FR-007 (« aucun libellé vide ») devient une propriété du contrat, pas seulement du gabarit.
2. **`utilisateur_id` absent ⇒ nom en texte simple.** Il n'est renseigné que si le compte existe *et* n'est pas supprimé (`LEFT JOIN … AND u.deleted_at IS NULL`). Le frontend n'a jamais à vérifier la validité du lien.
3. **`contact` est une donnée saisie**, jamais l'e-mail du compte rattaché. Aucun rattachement n'expose une adresse de compte.

---

## 5. Compatibilité

Toutes les adresses publiques existantes restent valides et conservent leur forme (SC-009). Les seuls retraits de champs (`episodes_apercu`, `interactions` sur `emissions[]` dans `/sections`) portent sur des champs déjà optionnels, et sur le seul endpoint dont le rendu client est réécrit dans la même livraison.
