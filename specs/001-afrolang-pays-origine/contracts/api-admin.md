# Contract : API admin

Deux nouveaux endpoints, calqués 1-pour-1 sur `marketplace.annonce_pays`.

**Permission requise** : `verifier_permission!(admin, "afrolang", "modifier")` (existante).
**Audit** : `audit::log_action` avec `module="afrolang"`, `table="salle_pays_origine"`, `entity_id=salle_id`, `action="CREATE"` ou `"DELETE"`.

---

## `POST /api/admin/afrolang/salles/{id}/pays`

Associe un pays au tableau « pays d'origine » d'une salle publique.

### Path parameters

| Param | Type | Notes                          |
|-------|------|--------------------------------|
| `id`  | UUID | Identifiant de la salle publique |

### Body

```json
{ "pays_id": "uuid-du-pays" }
```

### Pré-conditions

1. La salle existe (`afrolang.salle WHERE id=$1 AND deleted_at IS NULL`).
2. Le pays existe et est **actif** (`shared.pays WHERE id=$1 AND actif=TRUE`).

### Codes de retour

| Code | Cas                                                                     |
|------|-------------------------------------------------------------------------|
| 201  | Créé (ou déjà existant : `ON CONFLICT DO NOTHING`)                      |
| 401  | Non authentifié                                                         |
| 403  | Permission `afrolang:modifier` absente                                  |
| 404  | Salle ou pays inconnu / pays archivé                                    |

### Réponse 201

```json
{
  "success": true,
  "data": { "salle_id": "...", "pays_id": "..." },
  "error": null
}
```

### Effet sur l'audit

Une ligne dans `audit_log` :
- `action = "CREATE"`
- `module = "afrolang"`
- `table = "salle_pays_origine"`
- `entity_id = <salle_id>`
- `before = null`, `after = null` (cohérent avec `annonce_pays`)
- `ip` et `user_agent` extraits via `audit::extraire_ip` / `extraire_user_agent`

### Idempotence

Garantie par `INSERT ... ON CONFLICT DO NOTHING` sur la PK composite. Un second appel avec le même couple renvoie également 201 sans dupliquer (et **ne** ré-écrit pas une ligne d'audit en double, cf. note d'implémentation : si `rows_affected = 0`, log audit identique pour préserver la simplicité, comme `annonce_pays`).

---

## `DELETE /api/admin/afrolang/salles/{id}/pays/{pays_id}`

Retire un pays du tableau « pays d'origine » d'une salle publique.

### Path parameters

| Param      | Type |
|------------|------|
| `id`       | UUID : salle publique |
| `pays_id`  | UUID : pays à retirer |

### Codes de retour

| Code | Cas                                                       |
|------|-----------------------------------------------------------|
| 200  | Retiré                                                    |
| 401  | Non authentifié                                           |
| 403  | Permission absente                                        |
| 404  | Couple `(salle, pays)` inexistant                         |

### Réponse 200

```json
{ "success": true, "data": null, "error": null }
```

### Effet sur l'audit

Identique à POST mais `action = "DELETE"`.

---

## Lecture admin de la liste actuelle

Aucun nouvel endpoint dédié. La liste des pays d'origine d'une salle est exposée via le champ `pays_origine` ajouté à `SalleDetailResponse` côté `GET /api/admin/afrolang/salles/{id}` (handler existant `admin::salles::obtenir_salle`). Côté admin, **les pays archivés sont également renvoyés** mais marqués `code_iso2`/`nom` standard avec un flag dérivable côté front (cf. `pays.actif` ⇒ chip grisée) : le filtre `actif = TRUE` n'est appliqué qu'à l'API publique.

> Conséquence d'implémentation : le `json_agg` dans `obtenir_salle` (admin) **n'inclut pas** la condition `WHERE p.actif = TRUE`, différence intentionnelle avec la version publique pour permettre à l'admin de nettoyer les associations vers pays archivés (Q3).
