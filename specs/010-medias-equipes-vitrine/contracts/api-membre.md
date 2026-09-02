# Contrat : API membre (détenteurs de support)

**Feature**: 010-medias-equipes-vitrine
**Scope**: `/api/medias` (`routes.rs:1109-1170`)

Trois routes. Toutes gardées par `garde_detenteur(pool, moi, type_support, support_id, "co_detenteur")`, **jamais par `AdminUtilisateur`** : ce sont des routes membres. L'erreur inverse a été commise et corrigée en 009.

---

## Ordre de déclaration : contrainte dure

```rust
web::scope("/medias")
    // … routes littérales existantes (/partages, /themes, /propositions, …)

    // Équipes éditoriales (010). Segment littéral : DOIT précéder
    // « /{type_support}/{support_id}/… », qui tenterait sinon de parser
    // « equipe » comme un UUID et renverrait 404.
    .route("/equipe/fonctions", web::get().to(media_equipe::lister_fonctions))

    // … /emissions/{id}, /episodes/{id}, … (routes.rs:1141-1149)

    // Motifs à deux paramètres : le segment final « equipe » les distingue
    // sans ambiguïté de /detenteurs, /thematiques, /couverture, /grille…
    .route("/{type_porteur}/{porteur_id}/equipe", web::get().to(media_equipe::obtenir_equipe))
    .route("/{type_porteur}/{porteur_id}/equipe", web::put().to(media_equipe::definir_equipe))
```

`{type_porteur}` accepte **quatre** valeurs (`chaine_tv`, `station_radio`, `emission_tele`, `emission_radio`), là où les motifs voisins n'en acceptent que deux. Une valeur hors liste renvoie `400`, jamais `404` : le message doit dire ce qui était attendu.

---

## 1. `GET /api/medias/{type_porteur}/{porteur_id}/equipe`

Lecture de travail (le public lit l'équipe dans les payloads de support/programme, cf. `api-public.md`). Sert à repeupler le formulaire d'édition.

- **Authentification** : aucune. Cohérent avec `obtenir_thematiques` (`media_support.rs:376`), qui est également public : la donnée est publiée de toute façon.
- **200** :

```json
{ "success": true, "data": { "membres": [ { "id": "…", "nom": "Diallo", "prenom": "Aminata",
  "fonction": "Directrice", "territoire": "Sénégal", "contact": "…",
  "utilisateur_id": "…", "ordre": 0 } ] }, "error": null }
```

- **400** : `type_porteur` hors des 4 valeurs.
- **404** : porteur inexistant ou supprimé.

---

## 2. `PUT /api/medias/{type_porteur}/{porteur_id}/equipe`

Remplacement intégral et ordonné (décision D6).

### Requête

```json
{
  "membres": [
    { "nom": "Diallo", "prenom": "Aminata", "fonction": "Directrice",
      "territoire": "Sénégal", "contact": "direction@africatv.example",
      "utilisateur_id": "8f1c…" },
    { "nom": "Kouassi", "prenom": null, "fonction": "Producteur",
      "territoire": null, "contact": null, "utilisateur_id": null }
  ]
}
```

- L'**ordre du tableau fait foi** : `ordre` est l'index. Réordonner, c'est renvoyer la liste dans le nouvel ordre (FR-016).
- `"membres": []` **supprime toute l'équipe**. C'est valide, et c'est le seul moyen de vider un bloc.
- `utilisateur_id` est facultatif (FR-013). Une équipe entièrement composée de non-inscrits est acceptée sans réserve.

### Traitement

1. Résolution du support : si `type_porteur` est une émission, `contexte_emission(pool, porteur_id)` donne `(type_support, support_id)` ; sinon le porteur **est** le support.
2. `garde_detenteur(…, "co_detenteur")`.
3. Validation (`EquipeRequest::valider`) : voir [data-model.md §3](../data-model.md).
4. Transaction : instantané avant → `DELETE` des membres du porteur → `INSERT` de la liste, `fonction` normalisée (`btrim` + espaces réduits) → `COMMIT`.
5. `audit::log_action` après commit : action `equipe_modifiee`, table `media_content.membre_equipe`, avant/après en JSONB (Principe VII, FR-018).

### Réponses

| Code | Cas |
|---|---|
| `200` | `{ "success": true, "data": { "membres": [ … ] }, "error": null }`, la liste relue, avec ses `id` neufs et ses `ordre` |
| `400` | `type_porteur` invalide · nom vide · fonction vide · plus de 60 membres |
| `401` | Pas de jeton |
| `403` | Le demandeur ne détient pas le support (`garde_detenteur`) |
| `404` | Porteur inexistant ou supprimé |

> **Rotation des identifiants** : chaque `PUT` réattribue les `id`. Aucune table ne référence un membre d'équipe : c'est ce qui autorise le remplacement intégral. Ne pas introduire de référence entrante sans revoir cette décision.

---

## 3. `GET /api/medias/equipe/fonctions`

Suggestions pour le champ « fonction » (FR-015).

- **Authentification** : aucune, aucun paramètre, patron `GET /api/experts/specialites` (`handlers/experts.rs:616`).
- **200** : `{ "success": true, "data": ["Concepteur", "Directeur", "Producteur", "Réalisatrice"], "error": null }`, un `Vec<String>` nu, trié alphabétiquement.
- **Déduplication** : une seule entrée par clé insensible à la casse et aux espaces ; l'orthographe restituée est **la plus employée**. « Directeur », « directeur » et « directeur  » donnent une ligne, pas trois. Requête en [research.md D3](../research.md).
- Portée volontairement **globale** (toutes chaînes, stations et programmes confondus) : une fonction déclarée sur une chaîne doit être proposée sur un programme, sinon le référentiel ne se constituerait jamais.

---

## Récapitulatif des points d'appel frontend

| Écran | Porteur | Route |
|---|---|---|
| `/mon-compte/mes-supports` → panneau du support → « Équipe éditoriale » | `chaine_tv` \| `station_radio` | `GET`/`PUT …/{type}/{id}/equipe` |
| `/mon-compte/mes-supports` → panneau → Programmes → fiche d'un programme | `emission_tele` \| `emission_radio` | idem |
| Champ « fonction » des deux écrans |, | `GET /api/medias/equipe/fonctions` |

Le composable `useMediaEquipe.ts` expose `obtenirEquipe`, `definirEquipe`, `listerFonctions`.
