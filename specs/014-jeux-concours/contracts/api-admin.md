# Contrat : API d'administration (jeux variés et concours)

Préfixe : `/api/admin/jeu`. Toutes les routes exigent la permission `jeu.gerer` (`verifier_permission!`, créée par la 013). Toute mutation écrit une ligne d'audit (`audit::log_action`, non bloquant) : action, table, id, état avant et après.

---

## 1. Épreuves (extension des routes de la 013)

### `POST /epreuves`, `PUT /epreuves/{id}` — étendus

Corps : celui de la 013, plus `type_reponse` et les champs propres au type, **dans l'ordre attendu** (le serveur les range au hasard avant de stocker, research D1) :

| Type | Champs |
|---|---|
| `choix` | `propositions`, `bonne_reponse` (inchangé) |
| `carte` | `reponse_pays_id` |
| `ordre` | `elements: [{ texte, valeur? }]` dans l'ordre attendu (3 à 6) |
| `paires` | `paires: [{ gauche, droite }]` (3 à 5) |

Refus en 400, avec le champ fautif nommé : élément en double, `valeur` vide sur une partie seulement des éléments, pays hors des 55, nombre d'éléments hors bornes, `media_type` sans `media_url`.

`GET /epreuves/{id}` rend l'épreuve **dans l'ordre attendu** (éléments, paires), pour que l'administrateur la relise comme il l'a saisie.

### `POST /medias` — multipart

Champ `fichier`, champ `type` (`image` | `audio`).

- `image` : `normaliser_photo` (JPEG/PNG/WebP, redimensionnée, EXIF supprimées).
- `audio` : signature binaire MP3, OGG, M4A ou WAV ; 1 Mo au plus (research D4).
- Réponse : `{ "media_type": "audio", "media_url": "/uploads/jeu/audios/<uuid>.mp3" }`.
- 400 avec un message qui dit quoi corriger.

### `POST /epreuves/derivation` — inchangé

Les nouvelles formes (research D5) apparaissent dans `GET /epreuves/formes` avec leur `type_reponse`, et se dérivent par module comme les autres.

### `PUT /regles` — étendu

Champs ajoutés : `majoration_ordre_paires_s`, `prime_concours_participation`, `prime_concours_podium` (3 valeurs décroissantes). Mêmes règles de bornes et d'audit (`REGLES_MODIFIEES`).

---

## 2. Concours

### `GET /concours`

Tous les concours, avec leur phase, leur nombre de participations par état, et leur nombre de voix (total et comptées). Paramètres : `phase`, `page`. **La lecture résout les concours** (research D6).

### `POST /concours`

Corps : `format`, `titre`, `theme`, `reglement`, `rattachement?`, `image_url?`, `appel_debut`, `vote_debut`, `vote_fin`, `participations_max?`, `minimum_participations?`, `votes_max?`, `presentations_min?`, `jury?`, `jury_finalistes?`, `jury_delai_jours?`, `prime_participation?`, `prime_podium?`.

400 si les dates ne sont pas ordonnées, si le vote dure moins de 24 h (FR-022), ou si `appel_debut` est dans le passé de plus d'une heure. Audit `CONCOURS_CREE`.

### `PUT /concours/{id}`

- Phase `a_venir` : tout est modifiable.
- Après : seuls `theme`, `reglement`, `image_url` et les dates **encore à venir** le sont (FR-024). Le reste → 409 qui nomme le champ.
- Audit `CONCOURS_MODIFIE`, avant et après.

### `DELETE /concours/{id}`

Phase `a_venir` seulement, sinon 409. Audit `CONCOURS_SUPPRIME`.

### `POST /concours/{id}/annuler`

Corps : `{ "motif": "…" }`. À toute phase sauf `resultats`. Verse les primes de participation déjà dues (participations publiées), notifie les participants. Audit `CONCOURS_ANNULE`.

---

## 3. Modération des participations

### `GET /concours/participations`

File de modération, tous concours confondus. Paramètres : `etat` (défaut `en_attente`), `concours`, `page`. Chaque ligne : la participation, son auteur, son concours et sa phase, le nombre de signalements.

### `POST /concours/participations/{pid}/accepter`

`en_attente` → `publiee`. 409 depuis un autre état. Notification `jeu.participation_acceptee`. Audit `PARTICIPATION_ACCEPTEE`.

### `POST /concours/participations/{pid}/rejeter`

Corps : `{ "motif": "…" }` (obligatoire, 400 sinon). `en_attente` ou `publiee` → `rejetee`. Notification avec le motif. Audit `PARTICIPATION_REJETEE`.

### `POST /concours/participations/moderation-groupee`

Corps : `{ "ids": [...], "decision": "accepter" | "rejeter", "motif"?: "…" }`. Même logique ligne à ligne ; renvoie le bilan (`acceptees`, `rejetees`, `ignorees`). Une ligne d'audit par participation. Cible : SC-014.

### `POST /concours/participations/{pid}/retablir`

`suspendue` → `publiee`, compteur de signalements remis à zéro (les lignes de signalement sont gardées). Audit `PARTICIPATION_RETABLIE`.

### `GET /concours/participations/{pid}/signalements`

Les signalements d'une participation : membre, motif, date.

---

## 4. Suivi du vote et anti-fraude (research D9)

### `GET /concours/{id}/suivi`

Visible pendant **et** après le vote, par l'administrateur seulement :

```json
{ "votes": { "total": 4210, "comptes": 3987, "ecartes": { "trop_rapide": 120, "compte_recent": 88, "non_verifie": 15, "ecartee_admin": 0 } },
  "votants": 312,
  "presentations": { "min": 31, "max": 44, "moyenne": 37.2 },
  "classement_provisoire": [ { "participation_id": "…", "taux": 71.4, "duels": 63 } ],
  "comptes_signales": [ { "utilisateur_id": "…", "nom": "…", "motifs": ["rythme", "preference"], "votes": 240 } ] }
```

### `POST /concours/{id}/votants/{uid}/ecarter`

Toutes les voix de ce votant dans ce concours : `comptee = false`, `motif_ecart = 'ecartee_admin'`. 409 une fois les résultats établis. Audit `VOIX_ECARTEES`.

### `POST /concours/{id}/votants/{uid}/retablir`

L'inverse, pour les seules voix `ecartee_admin`. 409 après les résultats. Audit `VOIX_RETABLIES`.

---

## 5. Jury

### `GET /concours/{id}/finalistes`

Phase `deliberation` : les `jury_finalistes` premiers du classement communautaire (sans les participations sous le seuil), avec auteur et taux.

### `POST /concours/{id}/deliberation`

Corps : `{ "podium": [pid1, pid2, pid3] }` : 1 à 3 participations distinctes, toutes parmi les finalistes. 409 hors phase `deliberation`. Fixe `podium_jury`, `delibere_par`, `delibere_at`. La lecture suivante établit les résultats. Audit `JURY_DELIBERATION`.
