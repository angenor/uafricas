# Contrats HTTP : Ressources contribuées (public)

**Préfixe** : `/api/afrolang`
**Auth** : JWT bearer requis sur tous les `POST` et `DELETE`. Lecture ouverte selon FR-001 (publique pour salle publique ; restreinte pour salle privée : voir 401/403).

Toutes les réponses suivent l'enveloppe standard `ApiResponse<T> = { success: boolean, data: T | null, error: string | null, meta?: PaginationMeta }`.

---

## GET `/api/afrolang/salles/{salle_id}/ressources-contribuees`

Liste paginée des ressources contribuées rattachées à la salle.

### Query parameters

| Paramètre | Type | Défaut | Description |
|---|---|---|---|
| `page` | int | 1 | Numéro de page (1-indexé) |
| `limit` | int | 20 | Max 50 |
| `type` | string | (tous) | Filtre : `document` \| `video_youtube` \| `accompagnateur` \| `lien_web` |

### Autorisation

- Salle publique (`afrolang.salle` rattachée par groupe ethnique sans `salle_privee` parent) → accès libre, JWT optionnel.
- Salle privée (`afrolang.salle_privee`) → JWT requis ET ligne active dans `afrolang.acces_salle_privee` pour le couple `(salle_privee_id, utilisateur_id)`. Sinon **403 Forbidden** `code='salle_privee_acces_requis'`.

Filtrage par visibilité accompagnateur :
- Pour `type = 'accompagnateur'`, seules les lignes avec `statut_accompagnateur = 'acceptee'` sont retournées, **sauf** si l'utilisateur authentifié est `auteur_id` ou `membre_recommande_id` (alors `en_attente` et `refusee` sont également visibles).

### Réponse 200

```json
{
  "success": true,
  "data": [
    {
      "id": "...",
      "type": "document",
      "titre": "Grammaire bambara : bases",
      "description": "Polycopié 12 pages introduction conjugaison.",
      "auteur": { "id": "...", "nom": "Diop", "prenom": "Awa", "avatar_url": null },
      "session_origine_id": "...",
      "fichier_url": "/uploads/afrolang/ressources_contribuees/<uuid>/grammaire.pdf",
      "fichier_taille_octets": 524288,
      "video_id_youtube": null, "video_url": null, "lien_url": null,
      "accompagnateur": null,
      "created_at": "2026-05-24T10:11:22Z"
    }
  ],
  "error": null,
  "meta": { "total": 38, "page": 1, "limit": 20 }
}
```

### Erreurs

- **404** salle introuvable ou soft-deleted (`code='salle_introuvable'`).
- **403** salle privée sans accès actif (`code='salle_privee_acces_requis'`).
- **403** salle désactivée par administration **n'est pas** retournée 403 en lecture (on continue à exposer la liste read-only ; voir FR-010).

---

## POST `/api/afrolang/salles/{salle_id}/ressources-contribuees`

Ajoute une ressource. Variant `document` → multipart ; autres variants → JSON.

### Autorisation

- JWT requis, état utilisateur = `actif`.
- Salle non soft-deleted ET non désactivée admin (`desactivee_admin_at IS NULL`). Sinon **409 Conflict** `code='salle_desactivee_admin'`.
- Rate limit : `COUNT(*) WHERE auteur_id = me AND salle_id = $1 AND created_at > NOW() - INTERVAL '24h' AND deleted_at IS NULL`. Si ≥ 10 → **429** `code='rate_limit_ressources'`.
- Si salle privée → ligne active dans `acces_salle_privee` requise. Sinon **403** `code='salle_privee_acces_requis'`.

### Body : variant `document` (multipart/form-data)

| Champ | Type | Contraintes |
|---|---|---|
| `type` | text | `"document"` |
| `titre` | text | 1..120 chars |
| `description` | text | 0..500 chars |
| `session_origine_id` | text | UUID optionnel (session courante) |
| `fichier` | file | MIME ∈ {`application/pdf`, `application/msword`, `application/vnd.openxmlformats-officedocument.wordprocessingml.document`, `application/vnd.oasis.opendocument.text`} ; extension ∈ {`.pdf`, `.doc`, `.docx`, `.odt`} ; taille ≤ 20 Mo |

### Body : variant `video_youtube` (JSON)

```json
{
  "type": "video_youtube",
  "titre": "Conjugaison bambara : leçon 1",
  "description": null,
  "session_origine_id": "...",
  "video_url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
}
```

Le backend extrait l'ID 11 chars via regex et le stocke dans `video_id_youtube`. URL non-YouTube ou ID introuvable → **400** `code='url_youtube_invalide'`.

### Body : variant `lien_web` (JSON)

```json
{
  "type": "lien_web",
  "titre": "Dictionnaire Bambara en ligne",
  "description": "Référence open source",
  "session_origine_id": null,
  "lien_url": "https://exemple.org/bambara"
}
```

### Body : variant `accompagnateur` (JSON)

```json
{
  "type": "accompagnateur",
  "titre": "Recommandation accompagnateur bambara",
  "session_origine_id": "...",
  "membre_recommande_id": "<uuid>",
  "motif_recommandation": "Locuteur natif du bambara, formateur depuis 8 ans."
}
```

Contraintes : `membre_recommande_id` ≠ auteur ; état utilisateur recommandé = `actif` ; `motif_recommandation` ≥ 20 chars. Sinon **400** ou **422** selon le cas. Création en état `statut_accompagnateur='en_attente'`. Une notification `afrolang.accompagnateur.recommandation_recue` est envoyée au membre recommandé.

### Réponse 201

```json
{ "success": true, "data": { /* RessourceContribueeResponse */ }, "error": null }
```

---

## DELETE `/api/afrolang/ressources-contribuees/{id}`

Soft-delete une ressource.

### Autorisation

- JWT requis. Autorisé si `auteur_id = me` OU `est_admin_plateforme(me)`.
- Sinon **403** `code='retrait_non_autorise'`.

### Effets

- `UPDATE ressource_contribuee SET deleted_at = NOW(), supprime_par = me WHERE id = $1`
- `audit::log_action("DELETE", "afrolang", "ressource_contribuee", entity_id=$1)`

### Réponse 204

Pas de corps.
