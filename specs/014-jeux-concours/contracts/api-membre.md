# Contrat : API publique et membre (jeux variés et concours)

Préfixe : `/api/jeu`. Enveloppe `{ success, data, error }`. Erreurs par `ApiErreur` (400, 401, 403, 404, 409). Les routes *membre* passent par `garde_joueur` (013, research D11) ; les routes *publiques* acceptent un jeton facultatif (`joueur_optionnel`).

**Ordre de déclaration dans `routes.rs`** : les segments littéraux avant les motifs paramétrés de même profondeur. Ici : `/concours/mes-participations` avant `/concours/{id}`.

Types : [data-model.md](../data-model.md).

---

## 1. Épreuves : ce qui change dans les routes de la 013

Aucune route nouvelle. `POST /parties`, `POST /parties/{id}/suivante`, `POST /parties/{id}/repondre`, `GET /duels/{id}/direct` et `POST /duels/{id}/direct/repondre` gardent leurs chemins ; seules les formes de l'épreuve servie, de la réponse et de la correction s'étendent.

### Épreuve servie (`EpreuveServie`)

Champs ajoutés : `type_reponse`, `appariements` (paires uniquement).

```json
{ "id": "…", "type_reponse": "ordre", "enonce": "Classez ces pays du plus peuplé au moins peuplé.",
  "media_type": null, "media_url": null, "difficulte": 1,
  "propositions": [ { "cle": 3, "texte": "Sénégal" }, { "cle": 1, "texte": "Nigeria" },
                    { "cle": 4, "texte": "Gabon" },   { "cle": 2, "texte": "Kenya" } ],
  "appariements": null, "delai_ms": 45000 }
```

- `choix` : inchangé.
- `carte` : `propositions` vide ; le client affiche la carte et la liste des 55 pays (`NOMS_PAYS_FR`).
- `ordre` : `propositions` = les éléments, **mélangés**, avec leur clé.
- `paires` : `propositions` = colonne de gauche, `appariements` = colonne de droite (`[{ cle, texte }]`), **chacune mélangée indépendamment**.
- Une clé est le rang de l'élément dans le tableau stocké, lui-même rangé au hasard à la création : **elle ne révèle rien de la solution** (research D1).
- `delai_ms` tient compte de la majoration ordre/paires et de la marge média.

### Réponse (`POST /parties/{id}/repondre`, `POST /duels/{id}/direct/repondre`)

Corps : `{ rang, ...une seule des formes ci-dessous }`.

| Type | Forme | Règle |
|---|---|---|
| choix | `{ "cle": 2 }` | inchangé ; `cle: null` = temps écoulé |
| carte | `{ "pays": "td" }` | code ISO2 d'un pays d'Afrique, sinon 400 |
| ordre | `{ "ordre": [1, 2, 3, 4] }` | permutation exacte des clés servies, sinon 400 |
| paires | `{ "paires": [2, 3, 1, 4] }` | `paires[i]` = clé de droite associée à la i-ième clé de gauche **par ordre de clé croissant** ; permutation exacte, sinon 400 |
| (tous) | `{ "cle": null }` | temps écoulé : `sans_reponse` |

Une réponse d'une forme qui ne correspond pas au type de l'épreuve : 400. Rejouer le même envoi renvoie la même correction (idempotence de la 013).

### Correction (`Correction`, `CorrectionManche`)

Champs ajoutés :

```json
{ "rang": 3, "issue": "mauvaise", "type_reponse": "ordre",
  "bonne_cle": null,
  "solution": [1, 2, 3, 4],
  "valeurs": { "1": "230,8 millions d'habitants", "2": "56,4 millions", "3": "18,4 millions", "4": "2,4 millions" },
  "bon_pays": null,
  "jouee": { "ordre": [2, 1, 3, 4] },
  "explication": "…", "lien": "/opportunite-afrique/…", "score_gagne": 0, "signalable": true }
```

- `ordre` : `solution` = clés dans l'ordre attendu ; `valeurs` = justification par clé.
- `paires` : `solution[i]` = clé de droite attendue pour la i-ième clé de gauche (par ordre de clé croissant).
- `carte` : `bon_pays: { "iso": "td", "nom": "Tchad" }` ; `jouee: { "pays": "ne" }`.
- `choix` : `bonne_cle` comme dans la 013.

---

## 2. Concours : lecture publique

### `GET /concours` — public

Paramètres : `phase` (`appel`, `vote`, `resultats`, `en_cours` = appel ou vote, `termines`), `rattachement` (code de module), `page`, `taille` (défaut 12).

```json
{ "elements": [ { "id": "…", "format": "photo", "titre": "Mon plat du dimanche", "theme": "…",
                  "rattachement": "afroculture", "image_url": null, "phase": "vote",
                  "appel_debut": "…", "vote_debut": "…", "vote_fin": "…",
                  "participations_publiees": 38 } ],
  "total": 4 }
```

Lire la liste **résout** chaque concours listé (research D6) : annulation ou établissement des résultats si leur heure est venue.

### `GET /concours/{id}` — public (jeton facultatif)

```json
{ "id": "…", "format": "photo", "titre": "…", "theme": "…", "reglement": "…", "rattachement": "afroculture",
  "phase": "vote", "appel_debut": "…", "vote_debut": "…", "vote_fin": "…",
  "participations_max": 1, "jury": false,
  "participations_publiees": 38,
  "moi": { "participations": [ { "id": "…", "etat": "en_attente", "media_url": "…", "legende": "…", "motif_rejet": null } ],
           "peut_participer": false, "peut_voter": true, "votes_exprimes": 12, "votes_max": null } }
```

- `moi` n'est présent qu'avec un jeton valide.
- Pendant le vote, **aucun** décompte n'est exposé (FR-039) : ni victoires, ni présentations, ni rang.

### `GET /concours/{id}/participations` — public

Galerie. Paramètres : `page`, `taille` (défaut 24).

- Phases `appel` et `vote` : participations **publiées**, **sans auteur**, dans un ordre aléatoire à chaque lecture (un ordre fixe trahirait l'ordre de dépôt ou un classement).
- Phase `resultats` : participations classées, avec auteur révélé, `rang`, `taux` (en %), `duels`, `sous_seuil`, `place_jury`.
- Phase `annule` : participations publiées avec leur auteur, sans classement.

```json
{ "elements": [ { "id": "…", "media_url": "…", "legende": "…",
                  "auteur": { "id": "…", "nom": "…", "prenom": "…", "photo_url": null },
                  "rang": 1, "taux": 71.4, "duels": 63, "sous_seuil": false, "place_jury": null } ],
  "total": 38 }
```

---

## 3. Concours : membre

### `POST /concours/{id}/participations` — membre (multipart)

Champs : `photo` (fichier), `legende` (facultatif, 200 signes).

- 409 hors phase `appel` ; 409 si `participations_max` est atteint (participations `en_attente` et `publiee`).
- Photo contrôlée et normalisée (`normaliser_photo`) ; 400 avec message si format ou fichier illisible.
- 201 → la participation, `etat: "en_attente"`.

### `PUT /concours/{id}/participations/{pid}` — membre, auteur (multipart)

Remplace la photo et/ou la légende. 409 si la participation n'est pas `en_attente` ou `rejetee`, ou hors phase `appel`. Une participation `rejetee` remplacée repasse `en_attente`.

### `DELETE /concours/{id}/participations/{pid}` — membre, auteur

Retire (`retiree`), à toute phase sauf `resultats`. Idempotent.

### `GET /concours/mes-participations` — membre

Participations du membre, tous concours confondus, avec la phase de chaque concours et, une fois les résultats établis, le rang et la récompense. Source de « Mes activités » (FR-059).

### `POST /concours/{id}/confrontations` — membre

Rend la confrontation ouverte du votant, ou en tire une nouvelle (research D7).

```json
{ "id": "…", "gauche": { "id": "…", "media_url": "…", "legende": "…" },
              "droite": { "id": "…", "media_url": "…", "legende": "…" },
  "votes_exprimes": 12, "votes_max": null }
```

- 409 hors phase `vote`.
- Quand il n'existe plus de paire nouvelle pour ce votant, ou que son plafond est atteint (US5, scénario 4) : `200` avec `{ "termine": true, "raison": "toutes_vues" | "plafond", "votes_exprimes": 40 }` à la place de la confrontation. Pas de 204 : le client lit toujours un corps.
- Jamais d'auteur, jamais de décompte.

### `POST /concours/{id}/confrontations/{cid}/voter` — membre

Corps : `{ "choix": "gauche" | "droite" }`.

- 409 si la confrontation est déjà votée (renvoie la même réponse : idempotent), hors phase `vote`, ou si elle n'appartient pas au votant.
- Le vote est **toujours accepté** ; `comptee` et `motif_ecart` sont fixés côté serveur et **jamais renvoyés** (research D9).
- Réponse : la confrontation suivante (même forme que ci-dessus), pour enchaîner sans aller-retour supplémentaire.

### `POST /concours/{id}/participations/{pid}/signaler` — membre

Corps : `{ "motif": "…" }`. Une fois par membre ; un second envoi renvoie 200 sans effet. Interdit sur sa propre participation (400). Au-delà du seuil : suspension (research D12).
