# Contrats REST — Endpoints administrateur de la plateforme

**Auth** : `admin_middleware` existant (rôle administrateur de la plateforme requis). Toute mutation est tracée par `audit::log_action`.

Wrapper `ApiResponse<T>` partout.

---

## A1. `GET /api/admin/afrolang/propositions`

Liste paginée et filtrable.

**Query params** :
- `statut?` (`en_attente | validee | rejetee | retiree`)
- `langue_code?` (string)
- `groupe_ethnique_id?` (uuid)
- `auteur_id?` (uuid)
- `date_debut?` / `date_fin?` (ISO timestamps)
- `page?` (défaut 1), `taille?` (défaut 20, max 100)
- `tri?` (`created_at_desc` (défaut), `created_at_asc`, `decide_at_desc`)

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

## A2. `GET /api/admin/afrolang/propositions/{id}`

Détail complet d'une proposition (auteur, groupe ethnique, pays d'origine, statut, décision).

**Réponse 200** : `{ success, data: PropositionResponse }`
**Réponse 404** : proposition introuvable.

---

## A3. `PATCH /api/admin/afrolang/propositions/{id}/valider`

Validation atomique : transaction sqlx (cf. research.md Décision 3) qui crée la salle et bascule la proposition en `validee`.

**Request body** :
```json
{
  "commentaire": "string|null"
}
```
*(commentaire facultatif côté validation, transmis à l'auteur si fourni)*

**Pré-conditions** :
- Proposition `statut='en_attente'`.
- Aucune `afrolang.salle` active sur le même `groupe_ethnique_id`.

**Effets** :
- Crée `afrolang.salle` (`titre`, `description`, `langue_cible`, `langue_code`, `groupe_ethnique_id`, `cree_par = auteur_id de la proposition`).
- Crée les lignes `afrolang.salle_pays_origine` correspondantes.
- Met à jour la proposition (`statut='validee'`, `decideur`, `decide_at`, `salle_id_creee`).
- Audit `VALIDATE proposition_salle` + `CREATE salle`.
- Notification auteur (in-app + e-mail best-effort).

**Réponses** :
- `200` : `{ success, data: { proposition: PropositionResponse, salle_id: uuid } }`
- `409` : déjà décidée OU salle existante sur ce groupe ethnique.

---

## A4. `PATCH /api/admin/afrolang/propositions/{id}/rejeter`

**Request body** :
```json
{
  "commentaire": "string (obligatoire, ≥ 10 caractères)"
}
```

**Pré-conditions** : `statut='en_attente'`.

**Effets** :
- `statut='rejetee'`, `decideur`, `decide_at`, `commentaire_decision`.
- Audit `REJECT proposition_salle`.
- Notification auteur avec commentaire.

**Réponses** :
- `200` : `{ success, data: PropositionResponse }`
- `400` : commentaire manquant/trop court.
- `409` : déjà décidée.

---

## A5. `POST /api/admin/afrolang/salles/{salle_id}/administrateurs`

Nomme un utilisateur administrateur d'une salle publique active.

**Request body** :
```json
{
  "utilisateur_id": "uuid"
}
```

**Pré-conditions** :
- Salle `actif=TRUE` et non supprimée.
- Utilisateur `etat='actif'` et non supprimé.
- Aucune nomination active existante pour ce couple.

**Effets** :
- INSERT `salle_administrateur` (`actif=TRUE`).
- Audit `CREATE salle_administrateur`.
- Notification utilisateur nommé.

**Réponses** :
- `201` : `{ success, data: SalleAdministrateurResponse }`
- `404` : salle ou utilisateur introuvable / inactif.
- `409` : nomination active déjà existante.

---

## A6. `DELETE /api/admin/afrolang/salles/{salle_id}/administrateurs/{utilisateur_id}`

Révoque la nomination active d'un administrateur de salle.

**Request body** :
```json
{
  "motif": "string|null"
}
```

**Effets** :
- `actif=FALSE`, `revoque_at=NOW()`, `revoque_par=admin courant`, `motif_revocation`.
- Audit `UPDATE salle_administrateur`.
- Notification utilisateur révoqué.

**Réponses** :
- `200` : `{ success, data: SalleAdministrateurResponse }`
- `404` : aucune nomination active à révoquer.

---

## A7. `GET /api/admin/afrolang/salles/{salle_id}/administrateurs`

Historique complet (actif + inactif) trié `nomme_at DESC`. Utilisé par le panneau admin de la salle.

**Réponse 200** : `{ success, data: SalleAdministrateurResponse[] }`

---

## DTO `SalleAdministrateurResponse`

```json
{
  "id": "uuid",
  "salle_id": "uuid",
  "utilisateur": { "id": "uuid", "nom": "...", "prenom": "...", "email": "...", "photo_url": "...|null" },
  "nomme_par": { "id": "uuid", "nom": "...", "prenom": "..." },
  "nomme_at": "ts",
  "actif": bool,
  "revoque_at": "ts|null",
  "revoque_par": { "id": "uuid", "nom": "...", "prenom": "..." } | null,
  "motif_revocation": "string|null",
  "suspendu_at": "ts|null",
  "motif_suspension": "string|null"
}
```

---

## Cascades automatiques (pas d'endpoint dédié)

Les handlers admin existants suivants sont **étendus** pour traiter la cascade (cf. research.md Décision 4) :

| Handler existant | Extension |
|------------------|-----------|
| `PATCH /api/admin/afrolang/salles/{id}` (champ `actif=false`) ou `DELETE` | Suspend toutes les `salle_administrateur` actives de cette salle (`motif_suspension='salle_archivee'`). |
| `PATCH /api/admin/utilisateurs/{id}` (changement `etat`→ inactif/suspendu/supprime) | Suspend toutes les `salle_administrateur` actives de cet utilisateur (`motif_suspension='compte_desactive'`). |

Chaque suspension génère une entrée `audit::log_action('UPDATE', 'afrolang', 'salle_administrateur', ...)`.
