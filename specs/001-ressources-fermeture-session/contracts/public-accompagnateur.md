# Contrats HTTP : Workflow Accompagnateur (public)

**Préfixe** : `/api/afrolang`
**Auth** : JWT bearer requis sur toutes les routes ci-dessous.

---

## GET `/api/afrolang/accompagnateur/recommandations-recues`

Liste paginée des recommandations dont je suis la personne recommandée (boîte de réception).

### Query parameters

| Paramètre | Type | Défaut |
|---|---|---|
| `statut` | string | (tous) : filtre `en_attente` \| `acceptee` \| `refusee` \| `retiree` |
| `page` | int | 1 |
| `limit` | int | 20 (max 50) |

### Autorisation

- JWT requis. Filtre implicite `membre_recommande_id = me AND type='accompagnateur' AND deleted_at IS NULL`.

### Réponse 200

```json
{
  "success": true,
  "data": [
    {
      "id": "<uuid>",
      "salle": { "id": "<uuid>", "nom": "Bambara", "groupe_ethnique": "Bambara" },
      "auteur": { "id": "<uuid>", "nom": "Diop", "prenom": "Awa", "avatar_url": null },
      "motif_recommandation": "Locuteur natif du bambara, formateur depuis 8 ans.",
      "statut_accompagnateur": "en_attente",
      "created_at": "2026-05-24T10:11:22Z",
      "reponse_at": null
    }
  ],
  "meta": { "total": 3, "page": 1, "limit": 20 }
}
```

---

## POST `/api/afrolang/ressources-contribuees/{id}/accepter`

### Autorisation

- JWT requis. `membre_recommande_id = me` ET `statut_accompagnateur = 'en_attente'`. Sinon **403** `code='action_non_autorisee'` ou **409** `code='statut_incompatible'`.

### Body : aucun

### Effets

- `UPDATE ressource_contribuee SET statut_accompagnateur='acceptee', reponse_at=NOW(), updated_at=NOW() WHERE id=$1`
- Notification `afrolang.accompagnateur.acceptee` à `auteur_id`.
- `audit::log_action("UPDATE", "afrolang", "ressource_contribuee", entity_id=$1, after={statut:'acceptee'})`.

### Réponse 200

DTO `RessourceContribueeResponse` mis à jour.

---

## POST `/api/afrolang/ressources-contribuees/{id}/refuser`

### Body (optionnel)

```json
{ "motif_refus": "Manque de temps cette année." }
```

`motif_refus` : 0..500 chars, non exposé publiquement.

### Autorisation

- JWT requis. `membre_recommande_id = me` ET `statut_accompagnateur = 'en_attente'`. Sinon **403** / **409**.

### Effets

- `UPDATE ressource_contribuee SET statut_accompagnateur='refusee', motif_refus=$2, reponse_at=NOW(), updated_at=NOW() WHERE id=$1`
- Notification `afrolang.accompagnateur.refusee` à `auteur_id` (sans le motif).
- Audit.

### Réponse 200

DTO mis à jour (motif_refus visible uniquement pour `auteur_id`, `membre_recommande_id` et admin).

---

## POST `/api/afrolang/ressources-contribuees/{id}/retirer-consentement`

### Autorisation

- JWT requis. `membre_recommande_id = me` ET `statut_accompagnateur = 'acceptee'`. Sinon **403** / **409**.

### Body : aucun

### Effets

- `UPDATE ressource_contribuee SET statut_accompagnateur='retiree', reponse_at=NOW(), updated_at=NOW() WHERE id=$1`
- Notification `afrolang.accompagnateur.retiree` à `auteur_id`.
- Audit.

### Réponse 200

DTO mis à jour. La recommandation disparaît instantanément de l'affichage public.
