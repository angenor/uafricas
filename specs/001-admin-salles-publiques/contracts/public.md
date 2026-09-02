# Contrats REST : Endpoints publics (utilisateur authentifié)

**Auth** : JWT access (middleware existant `auth_middleware`). Refus 401 si non authentifié, 403 si compte non actif.

Toutes les réponses suivent le wrapper `ApiResponse<T>` existant : `{ success: bool, data: T | null, error: string | null }`.

---

## P1. `POST /api/afrolang/propositions`

Soumet une nouvelle proposition de salle publique.

**Request body** :
```json
{
  "titre": "string (≤ 350)",
  "description": "string",
  "justification": "string",
  "langue_cible": "string (≤ 100)",
  "langue_code": "string (≤ 40, optionnel)",
  "groupe_ethnique_id": "uuid",
  "pays_origine_ids": ["uuid", "..."]
}
```

**Validation backend** :
- Auteur authentifié + `etat='actif'`.
- Tous champs obligatoires non vides ; `pays_origine_ids` ≥ 1.
- `groupe_ethnique_id` existe dans `country_profile.groupe_ethnique`.
- Tous les `pays_origine_ids` existent dans `shared.pays` et sont actifs.

**Réponses** :
- `201` : `{ success: true, data: PropositionResponse }`
- `400` : champs manquants / invalides.
- `401` / `403` : auth.
- `409` : duplicat (auteur a déjà une proposition `en_attente` sur ce groupe ethnique) **ou** une `salle` active existe déjà pour ce groupe ethnique.
- `429` : ≥ 5 propositions rejetées dans les 7 derniers jours.

---

## P2. `GET /api/afrolang/propositions/moi`

Liste les propositions de l'utilisateur authentifié, plus récentes d'abord.

**Query params** : `statut?` (filtre), `page?` (défaut 1), `taille?` (défaut 20).

**Réponse 200** :
```json
{
  "success": true,
  "data": {
    "items": [PropositionResponse, ...],
    "total": int,
    "page": int,
    "taille": int
  }
}
```

---

## P3. `PATCH /api/afrolang/propositions/{id}/retirer`

Retrait par l'auteur d'une proposition encore en attente.

**Pré-conditions** :
- Authentifié.
- `auteur_id` = utilisateur courant.
- `statut='en_attente'`.

**Réponses** :
- `200` : `{ success: true, data: PropositionResponse (statut=retiree) }`
- `403` : ce n'est pas votre proposition.
- `409` : proposition déjà décidée (statut ≠ `en_attente`).

---

## P4. `GET /api/afrolang/salles` (existant, étendu)

DTO étendu : chaque `SalleResponse` contient maintenant `administrateurs: AdministrateurLight[]` (peuplé via `json_agg` filtré sur `salle_administrateur.actif=TRUE`).

```json
{
  "id": "uuid",
  "titre": "string",
  ...
  "pays_origine": [...],
  "administrateurs": [
    { "utilisateur_id": "uuid", "nom": "...", "prenom": "...", "photo_url": "...|null", "nomme_at": "ts" },
    ...
  ]
}
```

---

## DTO commun `PropositionResponse`

```json
{
  "id": "uuid",
  "auteur": { "id": "uuid", "nom": "string", "prenom": "string" },
  "titre": "string",
  "description": "string",
  "justification": "string",
  "langue_cible": "string",
  "langue_code": "string|null",
  "groupe_ethnique": { "id": "uuid", "nom": "string" },
  "pays_origine": [{ "id": "uuid", "nom": "string", "code_iso2": "string" }, ...],
  "statut": "en_attente|validee|rejetee|retiree",
  "decideur": { "id": "uuid", "nom": "string", "prenom": "string" } | null,
  "decide_at": "timestamp|null",
  "commentaire_decision": "string|null",
  "salle_id_creee": "uuid|null",
  "created_at": "timestamp",
  "updated_at": "timestamp"
}
```
