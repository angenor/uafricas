# Contrat : API d'administration du jeu

Préfixe : `/api/admin/jeu`, déclaré à plat dans `web::scope("/admin")` comme `/admin/engagement/*`.

**Auth** : extracteur `AdminUtilisateur`, puis `verifier_permission!(admin, "jeu", "gerer")` en première ligne de chaque handler (FR-073). **Toute mutation** appelle `audit::log_action` sur le schéma `jeu`, avec état avant et après (FR-085, principe VII).

**Ordre de déclaration** : `/epreuves/revue`, `/epreuves/derivation` et `/epreuves/formes` avant `/epreuves/{id}` ; `/saisons/{id}/clore` ne pose pas de problème ; `/defis/{periodicite}/{date}` est le seul motif à deux segments sous `/defis`.

Pagination des listes : celle de `models/pagination.rs` (`page`, `par_page`, `tri_par`, `tri_dir` → `{ data, total, page, par_page, total_pages }`), pour que `useAdmin().listerPagine` et `AdminDataTable` s'y branchent sans adaptation.

---

## 1. Modules

| Route | Effet |
|---|---|
| `GET /modules` | tous les modules, avec décompte d'épreuves **par état** |
| `PATCH /modules/{code}` | `{ ouvert }` (FR-081). Audit `MODULE_OUVERT` / `MODULE_FERME` |

---

## 2. Épreuves

### `GET /epreuves`

Filtres : `module`, `etat`, `origine`, `difficulte`, `pays`, `recherche` (sur l'énoncé), `anomalie` (booléen).

Chaque ligne porte `nombre_servie`, `taux_reussite` et `source_etat` :

| `source_etat` | Sens |
|---|---|
| `aucune` | épreuve sans référence |
| `conforme` | source visible, empreinte inchangée |
| `modifiee` | source visible, empreinte différente |
| `indisponible` | source dépubliée, suspendue ou supprimée |

`anomalie = true` ne garde que les épreuves servies au moins 30 fois dont le taux de réussite est inférieur à 15 % ou supérieur à 95 % (FR-080). Les seuils sont des constantes Rust, pas des réglages : ce sont des aides à la relecture, pas des règles du jeu.

### `POST /epreuves`

Saisie par un administrateur. Naît `candidate`, `origine = 'saisie'`.

```json
{ "module": "afrolang", "enonce": "Quelle langue entendez-vous ?", "media_type": "audio",
  "media_url": "/uploads/medias/audios/….mp3", "propositions": ["Wolof", "Bambara", "Lingala", "Swahili"],
  "bonne_reponse": 1, "explication": "…", "difficulte": 2, "theme": "Reconnaissance", "pays_id": null,
  "type_source": null, "source_id": null }
```

Le média se dépose par les routes d'upload **existantes** : `POST /api/admin/medias/upload` pour l'audio (mp3, ogg, wav, m4a, aac, 80 Mo). Aucune route d'upload neuve pour le son ; pour l'image, on réutilise une route d'upload d'image du back-office, à identifier à l'implémentation.

Si `type_source` et `source_id` sont fournis, la source doit exister et être visible dans `jeu.v_source` (400 sinon).

### `GET /epreuves/{id}` · `PUT /epreuves/{id}`

`PUT` remplace le contenu. Sur une épreuve `jouable`, la modification est permise et **ne touche à aucun gain** (FR-084). Sur une épreuve dérivée, corriger puis accepter enregistre l'empreinte courante de la source.

### `POST /epreuves/{id}/publier`

`candidate` ou `a_revoir` → `jouable`. Validation **avant** l'écriture, avec un message qui nomme le manque (FR-074) :

| Manque | Message |
|---|---|
| explication vide | « L'explication est obligatoire » |
| moins de deux propositions non vides | « Il faut au moins deux propositions » |
| propositions en double | « Deux propositions sont identiques » |
| `media_type` sans `media_url` | « Le média annoncé est manquant » |
| source indisponible | « Le contenu référencé n'est plus publié » |

### `POST /epreuves/{id}/retirer`

`jouable` → `retiree`. Les réponses et les gains restent (FR-013).

### `GET /epreuves/formes`

Le catalogue des formes de dérivation (research D5), par module, avec pour chacune le nombre de sources éligibles et le nombre de candidates déjà produites. Un module sans forme (Afrolang) renvoie une liste vide : l'écran le dit, il ne propose pas de bouton qui ne ferait rien.

### `POST /epreuves/derivation`

```json
{ "module": "afripulse", "formes": ["capitale", "monnaie"] }
```

`formes` omis = toutes celles du module.

```json
{ "creees": 96, "deja_proposees": 14, "sans_distracteurs": 3, "par_forme": [ { "forme": "capitale", "creees": 52 } ] }
```

- `INSERT … ON CONFLICT (type_source, source_id, forme) WHERE origine = 'derivee' DO NOTHING` : rejouer la dérivation ne crée aucun doublon, rejetées comprises (FR-075).
- Une source pour laquelle on ne trouve pas trois distracteurs distincts est comptée dans `sans_distracteurs` et ne produit rien.
- Une seule ligne d'audit `DERIVATION` pour l'opération, avec le décompte.

### `POST /epreuves/revue`

La revue, une à une ou par lot (FR-076).

```json
{ "ids": ["…", "…"], "decision": "accepter" }
{ "ids": ["…"], "decision": "rejeter", "motif": "Distracteurs trop proches" }
```

- `accepter` : `candidate` ou `a_revoir` → `jouable`, avec les mêmes validations que `publier`. Les épreuves du lot qui échouent sont renvoyées avec leur raison ; les autres sont acceptées.
- `rejeter` : `motif` obligatoire.
- « Corriger puis accepter » = `PUT /epreuves/{id}` suivi de `revue` : pas de route à part.

```json
{ "acceptees": 18, "rejetees": 0, "refus": [ { "id": "…", "raison": "L'explication est obligatoire" } ] }
```

**Bascule paresseuse** : toute lecture du vivier par un administrateur (`GET /epreuves`, `GET /modules`) commence par passer en `a_revoir` les épreuves `jouable` dérivées dont l'empreinte ne correspond plus (FR-012). C'est le seul endroit où cet état s'écrit. Ré-accepter une épreuve `a_revoir` réenregistre l'empreinte mais **ne régénère pas** son texte : si la source a changé de valeur (une capitale corrigée), l'administrateur corrige l'épreuve avant de l'accepter.

---

## 3. Règles du jeu

| Route | Effet |
|---|---|
| `GET /regles` | le singleton |
| `PUT /regles` | remplacement intégral, bornes vérifiées en SQL (CHECK) et rappelées en 400 lisible. Audit `REGLES_MODIFIEES` avec avant et après |

Les montants de réputation se règlent dans `/admin/engagement/regles`, écran existant ; l'écran des règles du jeu y renvoie par un lien.

---

## 4. Saisons

| Route | Effet |
|---|---|
| `GET /saisons` | toutes, avec état dérivé et nombre de joueurs classés |
| `POST /saisons` | `{ nom, debut_at, fin_at }` |
| `PUT /saisons/{id}` | modification |
| `POST /saisons/{id}/clore` | ramène `fin_at` à maintenant |

- Chevauchement : la contrainte d'exclusion le refuse ; l'erreur PostgreSQL est traduite en 409 « Cette période chevauche la saison … ».
- `debut_at` d'une saison commencée : non modifiable (400). `fin_at` d'une saison close : non modifiable.
- Un `debut_at` dans le passé, à la création ou sur une saison à venir, est **ramené à maintenant** : les gains acquis avant ne portent pas cette saison, la faire commencer plus tôt afficherait une période pendant laquelle rien n'a compté.
- `clore` n'est permis que sur la saison en cours (409 sinon) ; il attribue aussitôt les distinctions de podium.
- Une saison ayant des gains ne se supprime pas ; il n'y a pas de route de suppression.

---

## 5. Défis

| Route | Effet |
|---|---|
| `GET /defis` | filtres `periodicite`, `depuis`, `jusqua` ; défis passés avec participation, défis à venir programmés |
| `PUT /defis/{periodicite}/{date}` | programme le défi de la période : `{ titre?, module?, epreuve_ids }` (FR-079) |
| `DELETE /defis/{periodicite}/{date}` | retire une programmation **à venir** : le défi redeviendra automatique |

- `date` doit être à venir (400 sinon) ; pour `semaine`, un lundi.
- `epreuve_ids` : exactement la taille réglée, toutes servables.

---

## 6. Signalements d'épreuve

| Route | Effet |
|---|---|
| `GET /signalements` | file, filtre `etat` ; regroupée par épreuve, avec le nombre de signaleurs |
| `POST /signalements/{id}/decision` | `{ decision: "confirmer" \| "classer", retirer_epreuve?: bool }` |

- `confirmer` : crédite `jeu_signalement_confirme` au signaleur, le notifie ; si `retirer_epreuve`, l'épreuve passe `retiree`. Confirmer un signalement confirme d'office les autres signalements en attente de la même épreuve.
- `classer` : notifie le signaleur, rien d'autre.
- Aucun gain n'est repris (FR-084).

---

## 7. Triche

### `POST /joueurs/{utilisateur_id}/annuler-gains`

```json
{ "motif": "Réponses en moins d'une seconde sur 40 épreuves", "depuis": "2026-09-01T00:00:00Z", "retrait_reputation": 5 }
```

Dans une transaction :

1. Marque `annule_at`, `annule_par`, `motif_annulation` sur les gains du membre depuis `depuis` (omis = tous).
2. **Recalcule** `score_saison` et `joueur.score_total` du membre depuis les gains restants. On ne soustrait pas : le journal fait foi.
3. Après le COMMIT : `engagement::ajuster(pool, uid, 0, -retrait_reputation)` si demandé (FR-072), notification au membre (FR-031), audit `GAINS_ANNULES`.

`motif` obligatoire. L'opération n'a pas d'inverse dans cette version.

---

## Récapitulatif

| Famille | Routes |
|---|---|
| Modules | 2 |
| Épreuves | 9 |
| Règles | 2 |
| Saisons | 4 |
| Défis | 3 |
| Signalements | 2 |
| Triche | 1 |
| **Total** | **23** |
