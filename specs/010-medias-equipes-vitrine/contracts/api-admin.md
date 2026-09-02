# Contrat : API back-office

**Feature**: 010-medias-equipes-vitrine
**Scope**: `/api/admin` (`routes.rs:33`)

Deux routes, calquées trait pour trait sur `media_support::admin_{obtenir,definir}_thematiques` (`routes.rs:133-136`) : même forme d'URL, même paire de permissions, même handler jumeau du chemin membre.

---

## Ordre de déclaration : contrainte dure

Le bloc s'insère dans le groupe médias du scope admin, **immédiatement après** les routes `thematiques`/`couverture` (`routes.rs:136`) et donc **avant** :

- `/medias/{type_media}/{id}/etat` (`:141`), arité identique (3 segments) ; le segment final littéral `equipe` les distingue, mais la déclaration reste ordonnée pour ne pas dépendre de ce détail ;
- `/medias/{id}` (`:147`) : capterait `/medias/equipe`.

```rust
.route("/medias/{type_porteur}/{porteur_id}/thematiques", …)   // existant :133-134
.route("/medias/{type_porteur}/{porteur_id}/couverture",  …)   // existant :135-136

// Équipes éditoriales (010) : même forme que les deux blocs ci-dessus.
.route("/medias/{type_porteur}/{porteur_id}/equipe", web::get().to(media_equipe::admin_obtenir_equipe))
.route("/medias/{type_porteur}/{porteur_id}/equipe", web::put().to(media_equipe::admin_definir_equipe))
```

Nuance par rapport aux voisines : `{type_porteur}` accepte **quatre** valeurs (`chaine_tv`, `station_radio`, `emission_tele`, `emission_radio`), alors que `{type_support}` des routes thématiques n'en accepte que deux. Le back-office édite l'équipe d'un programme depuis `/admin/medias/emissions/[id]`, pas seulement celle d'un support.

---

## 1. `GET /api/admin/medias/{type_porteur}/{porteur_id}/equipe`

- **Garde** : `AdminUtilisateur` + `verifier_permission!(admin, "media", "voir")`.
- **200** : identique à la route membre, `{ "success": true, "data": { "membres": [ … ] }, "error": null }`.
- **400** `type_porteur` invalide · **403** permission manquante · **404** porteur inexistant.

Contrairement à la route membre, elle sert aussi les porteurs **non publiés** (brouillon, en attente, suspendu) : le back-office prépare une fiche avant sa mise en ligne.

---

## 2. `PUT /api/admin/medias/{type_porteur}/{porteur_id}/equipe`

- **Garde** : `AdminUtilisateur` + `verifier_permission!(admin, "media", "modifier")`.
- **Requête, traitement et réponses** : rigoureusement ceux de `PUT /api/medias/{type_porteur}/{porteur_id}/equipe` : voir [api-membre.md §2](./api-membre.md). Les deux handlers partagent la même fonction de règles, `appliquer_equipe(tx, type_porteur, porteur_id, membres, auteur)`, exactement comme `appliquer_thematiques` est partagée par ses deux chemins (`media_support.rs:217`).
- **Seule différence** : l'autorité. Aucune garde de détention : l'administration n'a pas à détenir le support.
- **Audit** : même action `equipe_modifiee`, avec l'identité de l'administrateur.

---

## 3. Suggestions de fonction

Aucune route admin dédiée. Le back-office consomme `GET /api/medias/equipe/fonctions`, publique et sans paramètre. Dupliquer ce référentiel sous `/api/admin` créerait deux listes qui divergeraient (FR-015 exige un référentiel unique et global).

---

## 4. Routes admin existantes touchées, sans changement de signature

| Route | Changement |
|---|---|
| `POST /api/admin/medias/emissions` (`radio_tele.rs:1101`) | `cadence` accepte `mensuelle`, la validation passe par `valider_cadence`, étendu à 4 valeurs. Aucun changement de contrat. |
| `PUT /api/admin/medias/emissions/{id}` (`:1203`) | idem |
| `DELETE /api/admin/medias/emissions/{id}` (`:1325`) | Suppression douce de l'équipe du programme dans la même transaction (FR-019). |
| `DELETE /api/admin/chaines-tv/{id}` (`:936`) · `DELETE /api/admin/stations-radio/{id}` (`:524`) | Suppression douce de l'équipe du support. |

Les routes membres équivalentes (`PUT /api/medias/emissions/{id}`, `DELETE /api/medias/emissions/{id}`) reçoivent le même traitement : voir [api-membre.md](./api-membre.md).

> `porteur_id` n'a pas de FK (prix du polymorphisme, cf. [data-model.md §1](../data-model.md)). Oublier l'un de ces quatre points de suppression laisserait des équipes orphelines : invisibles à l'écran, mais bien présentes dans le référentiel de suggestions de fonctions.

---

## 5. Points d'appel back-office

| Page | Porteur | Composant |
|---|---|---|
| `/admin/television/[id]` | `chaine_tv` | `MediaGestionEquipe` |
| `/admin/radio/[id]` | `station_radio` | `MediaGestionEquipe` |
| `/admin/medias/emissions/[id]` | `emission_tele` \| `emission_radio` (selon `type_support`) | `MediaGestionEquipe` |

`MediaGestionEquipe` est **le même composant** que celui monté côté membre dans `/mon-compte/mes-supports`, précédent assumé de `GestionEpisodes.vue` (« un seul composant membre+admin, l'autorité seule diffère »). Il est écrit en **Tailwind v4 pur** : il est monté sur des pages publiques membres, où daisyUI est proscrit (Principe VI). Les trois composants membres voisins (`GestionEpisodes`, `MesSupports`, `GestionCoDetenteurs`) sont déjà dans ce cas, vérifié, zéro classe daisyUI.

Le composant reçoit une prop `base` (`'membre' | 'admin'`) qui choisit le préfixe d'URL, à l'image de `useMediaEmissions`/`useAdminMediaEmissions`.
