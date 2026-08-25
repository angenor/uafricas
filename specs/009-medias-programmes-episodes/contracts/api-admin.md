# Contrat : API back-office (administrateurs)

**Authentification** : JWT admin (`AdminUtilisateur`). **Autorisation** :
`verifier_permission!(admin, "media", "voir" | "modifier" | "supprimer")`, permissions seedées par
09j §8. **Audit** : `audit::log_action` sur chaque mutation, avec état avant/après (FR-045, principe
VII).

Préfixe : `/api/admin/medias`.

---

## 1. File de modération des épisodes : le cœur de FR-040 à FR-043

### `GET /api/admin/medias/episodes`

```
?etat=en_attente          (défaut ; aussi rejete, publie, suspendu)
&type=tele|radio
&support_id=…
&tri=echeance|anciennete  (défaut : echeance)
&page=&taille=
```

```jsonc
{
  "episodes": [{
    "id": "…", "titre": "…", "type": "tele",
    "media_url": "/uploads/medias/videos/…",
    "duree_minutes": 58,
    "emission": { "id": "…", "titre": "Débats africains", "cadence": "hebdomadaire" },
    "support":  { "type": "chaine_tv", "id": "…", "nom": "Africa 24" },
    "auteur":   { "id": "…", "nom_complet": "…", "role_detention": "co_detenteur" },
    "soumis_at": "2026-08-06T09:12:00Z",
    "anciennete_heures": 51,
    "prochaine_echeance": "2026-08-09T18:00:00Z",   // null si l'émission n'est pas programmée
    "heures_avant_echeance": 27
  }],
  "pagination": { "page": 1, "taille": 25, "total": 43 }
}
```

Le tri `echeance` remonte d'abord les épisodes attendus à l'antenne (`prochaine_echeance` la plus
proche), puis les épisodes sans échéance par ancienneté. C'est ce qui empêche qu'un épisode dû samedi
soit traité au même rang qu'un contenu sans date (FR-043) et ce qui rend SC-007 atteignable.

`prochaine_echeance` se calcule à la lecture depuis les créneaux de l'émission, aucune tâche de fond.

### `PATCH /api/admin/medias/episodes/{id}/valider`

`200` → l'épisode passe `publie`, `valide_par` et `valide_at` sont renseignés, et il entre dans la
rotation à l'occurrence suivante. Notification à l'auteur (FR-041).

`409` si l'épisode n'est pas `en_attente`, ou s'il est dépourvu de média (le CHECK
`ck_episode_*_media_publie` refuse de toute façon).

### `PATCH /api/admin/medias/episodes/{id}/rejeter`

```jsonc
{ "motif": "Qualité audio insuffisante sur les 4 premières minutes." }
```

`400` si le motif fait moins de 10 caractères, même garde applicative que le rejet d'une proposition
(09l). Le CHECK `ck_episode_*_rejet_motive` interdit un rejet vide en base. Notification à l'auteur
avec le motif (FR-041, SC-008).

### `PATCH /api/admin/medias/{type_media}/{id}/etat`

Route existante, `type_media` passant à 6 valeurs. Rappel du comportement conservé : rétablir un
contenu suspendu **remet `nombre_signalements = 0`**, sans quoi le seuil resterait franchi ; les lignes
de signalement sont conservées pour l'historique.

---

## 2. CRUD émissions et épisodes

Remplacent `/api/admin/programmes-tele` et `/api/admin/programmes-radio`, supprimées.

| Route | Rôle |
|-------|------|
| `GET    /api/admin/medias/emissions` | Liste paginée. Filtres : `type`, `support_id`, `etat`, `cadence`, `recherche` (FR-046) |
| `POST   /api/admin/medias/emissions` | Création administrative |
| `GET    /api/admin/medias/emissions/{id}` | Détail + décompte d'épisodes par état |
| `PUT    /api/admin/medias/emissions/{id}` | Modification |
| `PATCH  /api/admin/medias/emissions/{id}/etat` | Changement d'état |
| `DELETE /api/admin/medias/emissions/{id}` | `409` si épisodes publiés (FR-010) |
| `GET    /api/admin/medias/emissions/{id}/episodes` | Épisodes, filtrables par état et date de soumission |
| `POST   /api/admin/medias/emissions/{id}/episodes` | Création administrative, naît `publie` (l'admin est l'autorité de validation) |
| `PUT    /api/admin/medias/episodes/{id}` | Modification |
| `DELETE /api/admin/medias/episodes/{id}` | Suppression douce |
| `PUT    /api/admin/medias/emissions/{id}/episodes/reordonner` | Réordonnancement atomique |
| `PATCH  /api/admin/medias/episodes/{id}/vedette-globale` | Remplace `/programmes-tele/{id}/vedette-globale`, bascule et désignation dans **une seule transaction** (R9) |
| `POST   /api/admin/medias/upload` | Inchangée |

**Asymétrie assumée** : un épisode créé par un administrateur naît `publie`, un épisode créé par un
co-détenteur naît `en_attente`. C'est la conséquence directe de FR-040, l'administrateur *est* le
validateur, le faire passer par sa propre file n'aurait pas de sens.

---

## 3. Thématiques et couverture territoriale

| Route | Rôle |
|-------|------|
| `GET /api/admin/medias/{type_support}/{support_id}/thematiques` | Thèmes déclarés |
| `PUT /api/admin/medias/{type_support}/{support_id}/thematiques` | Remplacement intégral (FR-029) |
| `GET /api/admin/medias/{type_support}/{support_id}/couverture` | Couverture déclarée |
| `PUT /api/admin/medias/{type_support}/{support_id}/couverture` | Territoires ou continentale, exclusifs (FR-034, FR-035) |

Mêmes règles de validation que côté membre (`contracts/api-membre.md` §4). Les formulaires
back-office de chaîne TV et de station radio intègrent les deux sélecteurs et **refusent
l'enregistrement d'un support publié sans thématique ni couverture**.

---

## 4. Propositions de médias

`GET /api/admin/medias/propositions` et les décisions associées restent en place. Deux évolutions :

- `type_objet` accepte `emission_tele`, `emission_radio`, `episode_tele`, `episode_radio` ; les
  anciennes valeurs `programme_tele` / `programme_radio` subsistent dans l'enum pour l'historique mais
  ne sont plus produites (data-model.md §3.4).
- La validation d'une proposition d'**épisode** crée l'épisode directement en `publie`, la décision
  administrative *est* la validation, l'épisode ne repasse pas par la file de modération. La validation
  d'une proposition d'**émission** crée l'émission puis, comme aujourd'hui, la ligne de propriété de son
  auteur dans la **même transaction**.

Le circuit `animation_programme` (validation ⇒ `support_detenteur`) et `idee_contenu` (ne crée rien)
sont inchangés.

---

## 5. Purge de reprise

### `POST /api/admin/medias/rapport-reprise`

Route de vérification **à usage unique**, exécutée après la migration `09q` et retirée ensuite. Elle ne
mute rien et renvoie les compteurs qui prouvent SC-001 :

```jsonc
{
  "episodes_tele": 412, "emissions_tele": 412,
  "episodes_radio": 187, "emissions_radio": 187,
  "interactions_reportees": { "reactions": 1204, "commentaires": 318,
                              "partages": 96, "signalements": 12 },
  "creneaux_rattaches": 34, "creneaux_orphelins": 0,
  "chaines_sans_thematique": 7,
  "chaines_sans_couverture": 2,
  "episodes_sans_emission": 0,
  "slugs_en_collision": 0
}
```

`episodes_sans_emission`, `creneaux_orphelins` et `slugs_en_collision` **doivent valoir 0** : toute
valeur non nulle signale une reprise incomplète. `chaines_sans_thematique` et `chaines_sans_couverture`
sont attendus non nuls (l'enum `categorie_chaine_tv` ne recouvre pas les 44 thèmes `media`) et listent
le travail éditorial manuel restant.
