# Contrat : API publique et membre du jeu

Préfixe : `/api/jeu`. Enveloppe habituelle : `{ success, data, error }`. Erreurs par `ApiErreur` (400 validation, 401, 403, 404, 409 conflit).

**Auth** : les routes marquées *membre* passent par `garde_joueur` : JWT valide **et** compte `actif` non supprimé, vérifié en base (research D11). Un compte en attente ou suspendu reçoit 403. Les routes *publiques* ne lisent aucun jeton.

**Ordre de déclaration** (piège connu du dépôt) : dans `routes.rs`, tout segment littéral précède le motif paramétré de même profondeur. Concrètement : `/defis/courants` avant `/defis/{id}/…`, `/classements/membres` et `/classements/pays` avant tout motif, `/duels/{id}/direct/repondre` ne pose pas de problème (profondeur propre).

Types : voir [data-model.md §15](../data-model.md).

---

## 1. Lecture publique

### `GET /modules` — public

Modules ouverts, avec leur disponibilité.

```json
[{ "code": "afripulse", "libelle": "Afripulse", "route": "/opportunite-afrique", "icone": "earth-africa",
   "epreuves_jouables": 212, "disponible": true }]
```

`disponible` = `epreuves_jouables >= taille_partie` (FR-005). Un module fermé n'est pas listé.

### `GET /saisons` — public

```json
{ "courante": { "id": "…", "nom": "Saison 1", "debut_at": "…", "fin_at": "…", "etat": "en_cours" },
  "archives": [ { "id": "…", "nom": "…", "debut_at": "…", "fin_at": "…", "etat": "close" } ] }
```

`courante` est `null` hors saison. Cette lecture déclenche la clôture paresseuse d'une saison échue (research D6).

### `GET /classements/membres` — public

Paramètres : `saison` (uuid, défaut : courante), `pays` (uuid, facultatif), `page`, `taille` (défaut 50, max 100).

```json
{ "elements": [ { "rang": 1, "utilisateur_id": "…", "nom": "Kouassi", "prenom": "Awa", "slug": "…",
                  "photo_url": null, "pays": "Côte d'Ivoire", "pays_iso2": "ci", "niveau_code": "premium", "score": 482 } ],
  "total": 1280, "page": 1, "taille": 50,
  "moi": { "rang": 214, "score": 96, "voisins": [ /* 2 au-dessus, 2 au-dessous */ ] } }
```

- `moi` n'est présent que si la requête porte un jeton valide **et** que le membre a du score dans ce classement (FR-059). Le jeton est facultatif ici : c'est la seule route publique qui le lit.
- Aucun autre champ du membre n'est exposé (FR-060) : ni e-mail, ni réputation, ni solde de points.
- Comptes suspendus ou supprimés exclus (FR-061).

### `GET /classements/pays` — public

Paramètre : `saison`.

```json
[ { "rang": 1, "pays_id": "…", "nom": "Sénégal", "iso2": "sn", "score": 1840, "joueurs": 37 } ]
```

Les 55 pays d'Afrique y figurent **tous**, y compris à score 0 (rang `null`, FR-067). C'est la source de la carte.

### `GET /pays/{pays_id}` — public

Fiche d'un pays pour la carte (FR-065).

```json
{ "pays_id": "…", "nom": "Sénégal", "iso2": "sn", "rang": 1, "score": 1840, "joueurs": 37,
  "meilleurs": [ /* 5 lignes de classement */ ],
  "modules": [ { "code": "afripulse", "libelle": "Afripulse", "epreuves_jouables": 14, "disponible": true } ] }
```

`modules[].disponible` dit si une partie restreinte à ce pays est possible (FR-066).

---

## 2. Mon jeu — membre

### `GET /moi`

```json
{ "score_total": 312, "saison": { "id": "…", "nom": "Saison 1", "score": 96, "rang": 214 },
  "pays_rattachement": { "id": "…", "nom": "Côte d'Ivoire" },
  "serie_jours": 4,
  "par_module": [ { "code": "afripulse", "score": 180 } ],
  "par_origine": { "partie": 240, "defi": 52, "duel": 20 } }
```

`pays_rattachement` est `null` si le profil n'a aucun pays : l'écran invite alors à le compléter (FR-055).

### `GET /moi/parties`

Paramètres : `page`, `taille`. Historique des parties terminées ou closes (FR-021) : cadre, module, date, `bonnes`, nombre d'épreuves, `score_gagne`.

---

## 3. Parties — membre

### `POST /parties`

**Une seule partie libre ouverte à la fois** : lancer une partie clôt la partie libre ou d'entraînement encore en cours du membre, et compte sans réponse l'épreuve qui y était affichée. Sans cela, on pourrait ouvrir une partie, lire la question, l'abandonner, et la retrouver « neuve » dans la suivante.

```json
{ "module": "afripulse", "pays_id": null, "theme": null, "entrainement": false }
```

| Cas | Réponse |
|---|---|
| Assez d'épreuves neuves | 201, partie `cadre = "libre"` |
| Pas assez, `entrainement: false` | **409** `"Vous avez joué toutes les épreuves : seule une partie d'entraînement est possible"` |
| Pas assez, `entrainement: true` | 201, partie `cadre = "entrainement"` (série tirée parmi toutes les servables) |
| Module fermé ou indisponible | 409 |

Le 409 est ce qui garantit que l'entraînement est **annoncé avant de commencer** (FR-016) : le client ne peut pas y tomber sans l'avoir demandé.

```json
{ "id": "…", "cadre": "libre", "module": "afripulse", "nombre_epreuves": 10, "rang_courant": 0,
  "etat": "en_cours", "temps_epreuve_s": 30 }
```

### `GET /parties/{id}`

État de la partie du membre (404 si elle n'est pas la sienne). Terminée ou close, la réponse porte le **bilan** (FR-020) :

```json
{ "id": "…", "etat": "terminee", "cadre": "libre", "nombre_epreuves": 10, "bonnes": 7,
  "score_gagne": 13, "score_total": 325, "rang_saison": 198,
  "reponses": [ { "rang": 1, "epreuve_id": "…", "issue": "bonne" } ],
  "distinctions": [ ] }
```

Une partie `en_cours` de plus de 24 h est passée à `close` par cette lecture.

### `POST /parties/{id}/suivante`

Présente l'épreuve suivante.

- Si l'épreuve courante n'a pas de réponse, elle est enregistrée `sans_reponse` (abandon, rechargement, temps écoulé), et sa correction est jointe en `precedente`.
- Écrit `presentee_at = NOW()`.
- S'il n'y a plus d'épreuve : clôt la partie, écrit les primes, renvoie `{ "terminee": true }`.

```json
{ "terminee": false, "rang": 3, "sur": 10, "expire_a": "2026-10-01T12:00:35Z", "maintenant": "2026-10-01T12:00:05Z",
  "epreuve": { "id": "…", "enonce": "Quelle est la capitale du Sénégal ?", "media_type": null, "media_url": null,
               "difficulte": 1, "propositions": [ { "cle": 3, "texte": "Bamako" }, { "cle": 1, "texte": "Dakar" } ] },
  "precedente": null }
```

`maintenant` permet au client de caler son compte à rebours sur l'horloge du serveur. `propositions` est mélangé à chaque appel (FR-030). **Aucune clé de ce payload ne dit la bonne réponse.**

### `POST /parties/{id}/repondre`

```json
{ "rang": 3, "cle": 1 }
```

`cle` peut valoir `null` : le client signale ainsi que le temps s'est écoulé. L'épreuve est enregistrée `sans_reponse` et sa correction renvoyée **sans présenter la suivante**, pour que le membre lise la correction avant que l'horloge ne reparte. Envoyer `null` en avance ne rapporte rien : c'est renoncer à l'épreuve.

| Cas | Réponse |
|---|---|
| Dans le délai (+ 2 s de tolérance) | 200, correction |
| Hors délai | 200, correction avec `issue: "sans_reponse"` : la réponse est refusée, l'épreuve est close (FR-028) |
| `rang` ≠ épreuve en cours | 409 |
| Réponse déjà enregistrée pour ce rang | 200, **la correction déjà enregistrée**, sans rien écrire (FR-025) |

```json
{ "issue": "bonne", "bonne_cle": 1, "explication": "Dakar est la capitale depuis 1960…",
  "lien": "/opportunite-afrique/…", "score_gagne": 1, "signalable": true }
```

`score_gagne` vaut 0 en entraînement et en duel.

### `POST /parties/{id}/injouable`

`{ "rang": 3 }`. Réservé aux épreuves à média. Enregistre l'issue `injouable` : épreuve consommée, aucun score, non comptée comme mauvaise (research D8). 409 si l'épreuve n'a pas de média.

---

## 4. Défis

### `GET /defis/courants` — public, jeton facultatif

Crée paresseusement le défi du jour et celui de la semaine s'ils n'existent pas. Sans jeton, `ma_partie` vaut `null` et `serie_jours` vaut 0 : le hub des activités se consulte sans connexion.

```json
{ "jour":    { "id": "…", "periode_debut": "2026-10-01", "fin_at": "2026-10-02T00:00:00Z", "nombre_epreuves": 5,
               "titre": null, "ma_partie": null },
  "semaine": { "id": "…", "periode_debut": "2026-09-28", "fin_at": "2026-10-05T00:00:00Z", "nombre_epreuves": 15,
               "titre": "Semaine des capitales", "ma_partie": { "id": "…", "etat": "terminee", "bonnes": 12 } },
  "serie_jours": 4 }
```

`jour` ou `semaine` vaut `null` si le vivier ne permet pas de composer le défi.

### `POST /defis/{id}/jouer`

Membre. Ouvre la participation, ou **renvoie celle qui existe** : `{ partie_id, etat, deja_commencee }`. Il n'y a jamais de seconde partie sur un même défi (FR-034, index unique), mais le membre doit pouvoir reprendre celle qu'il a commencée ou en revoir le résultat, d'où un 200 et non un 409. 409 seulement pour **commencer** un défi dont la période est passée (FR-038) ; finir un défi commencé à temps reste permis. La partie se joue ensuite par les routes de la section 3.

### `GET /defis/{id}/resultats`

Public, jeton facultatif (`moi` n'est présent qu'avec un jeton). Nombre de participants, podium des 20 premiers (`bonnes DESC, temps_total_ms ASC`), rang du membre. Seules les parties terminées de comptes actifs sont classées. Accessible pour un défi passé.

---

## 5. Duels — membre

### `GET /duels`

Mes duels, répartis en `a_repondre`, `a_jouer`, `en_attente`, `termines` (paginés). Chaque duel non terminé est **résolu avant d'être listé** (research D6). Porte aussi `quotas: { comptes_aujourdhui, plafond }`.

### `POST /duels`

```json
{ "adversaire_id": "…", "module": "codimoi", "mode": "differe" }
```

- 403 si les deux membres ne sont pas amis, ou si l'un a bloqué l'autre (FR-039).
- 409 si le module n'a pas assez d'épreuves servables pour un duel, ou si un duel non terminé existe déjà entre les deux sur ce module.
- Réponse 201 : le duel, avec `sera_compte: false` si l'un des plafonds du jour est déjà atteint (FR-048).
- Effets : notification à l'adversaire, événement `duel_propose`.

### `GET /duels/{id}`

Le duel, résolu. Chaque joueur y voit **sa** partie. Le résultat de l'autre (`bonnes`, `temps_total_ms`) n'est sérialisé que si le duel est terminé, ou si le demandeur a lui-même fini de jouer **et** l'autre aussi (FR-042).

### `POST /duels/{id}/accepter` · `/refuser` · `/annuler`

- `accepter`, `refuser` : l'adversaire seul, duel `propose`.
- `annuler` : le proposant seul, duel `propose`.
- `accepter` fige la série (tirée en priorité parmi les épreuves qu'**aucun** des deux n'a jouées), fixe `compte`, et calcule l'échéance.
- Toute action sur un duel hors de l'état attendu : 409.

### `POST /duels/{id}/jouer` — différé

Crée (ou renvoie) la partie `cadre = "duel"` du membre. Elle se joue par les routes de la section 3. 409 si le duel n'est ni `accepte` ni `en_cours`, ou si l'échéance est passée.

### `POST /duels/{id}/convertir`

Le proposant d'un duel direct `expire` ouvre un nouveau duel différé avec le même adversaire et le même module (FR-046). Renvoie le nouveau duel.

### `GET /duels/{id}/direct` — direct

L'état autoritaire de la manche. **Chaque appel vaut battement de présence** et fait avancer le duel (research D1, D6). Le client l'appelle à chaque signal `duel_*` et toutes les 3 secondes.

```json
{ "etat": "en_cours", "maintenant": "2026-10-01T12:00:05.120Z",
  "phase": "question",
  "rang": 3, "sur": 7, "manche_debut_at": "2026-10-01T12:00:00Z", "manche_fin_at": "2026-10-01T12:00:30Z",
  "epreuve": { "id": "…", "enonce": "…", "propositions": [ { "cle": 2, "texte": "…" } ] },
  "ma_reponse": null, "adversaire_a_repondu": true,
  "correction": null,
  "scores": { "moi": 2, "adversaire": 1 },
  "adversaire_present": true }
```

| `phase` | Sens | Ce qui est servi |
|---|---|---|
| `attente` | l'un des deux n'est pas encore présent | rien d'autre |
| `question` | manche ouverte | `epreuve`, sans correction |
| `revelation` | manche close, pause avant la suivante | `correction` (bonne clé, explication), les deux réponses |
| `termine` | duel fini | issue, vainqueur, gain |

`adversaire_a_repondu` est un booléen : ce que l'adversaire a répondu n'est jamais servi pendant la phase `question`.

### `POST /duels/{id}/direct/repondre` — direct

`{ "rang": 3, "cle": 2 }`. Mêmes règles de délai et d'idempotence que `/parties/{id}/repondre`. **Ne renvoie pas la correction** : elle n'arrive qu'en phase `revelation`, aux deux en même temps, pour qu'un joueur ne puisse pas la transmettre à l'autre pendant la manche.

---

## 6. Signalement d'épreuve — membre

### `POST /epreuves/{id}/signaler`

```json
{ "motif": "reponse_erronee", "commentaire": "La capitale a changé en …" }
```

- 403 si le membre n'a aucune réponse sur cette épreuve.
- 409 s'il l'a déjà signalée (FR-082).
- Une épreuve déjà retirée : 201, le signalement est classé d'office.

---

## Récapitulatif

| Famille | Routes |
|---|---|
| Lecture publique | 5 |
| Mon jeu | 2 |
| Parties | 5 |
| Défis | 3 |
| Duels | 10 |
| Signalement | 1 |
| **Total** | **26** |
