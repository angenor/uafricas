# Contrats HTTP : Modération administrative (admin)

**Préfixe** : `/api/admin/afrolang`
**Auth** : JWT bearer **+ rôle administrateur de la plateforme** sur tous les endpoints (helper `est_admin_plateforme` existant).

---

## POST `/api/admin/afrolang/sessions/{session_id}/fermer-admin`

Ferme une session livestream en cours pour cause d'abus et désactive la salle hôte.

### Body

```json
{ "motif": "Propos haineux récurrents signalés par 3 utilisateurs ; capture jointe au dossier." }
```

`motif` : 10..1000 chars, OBLIGATOIRE.

### Préconditions

- Session existante. Sinon **404** `code='session_introuvable'`.
- Salle hôte non déjà désactivée (`desactivee_admin_at IS NULL`). Sinon **409** `code='salle_deja_desactivee'` (idempotence FR-021).

### Effets (transaction unique)

1. Récupère `salle_id` de la session (`salle_id` ou `salle_privee.salle_id` selon le contexte XOR).
2. `UPDATE afrolang.session SET etat='terminee', termine_at=NOW(), updated_at=NOW() WHERE id=$1` (si en_cours).
3. `UPDATE afrolang.salle SET desactivee_admin_at=NOW(), desactivee_par=$me, motif_desactivation=$motif, updated_at=NOW() WHERE id=$salle_id`.
4. `INSERT INTO afrolang.evenement_moderation_salle (salle_id, session_concernee_id, type_action, admin_id, motif) VALUES (..., 'fermeture_admin', $me, $motif)`.
5. Hors transaction (asynchrone, non bloquant) :
   - `services::livekit_moderation::fermer_session_admin(room_name)` → broadcast data packet + `delete_room`.
   - Notifications type `afrolang.session.fermee_admin` (sans motif) aux `session_participant.utilisateur_id WHERE quitte_at IS NULL`.
   - Notifications type `afrolang.salle.desactivee_admin` (avec motif) aux administrateurs de salle publique (`salle_administrateur WHERE salle_id=$salle_id AND actif=TRUE`) ou au créateur de salle privée (`salle_privee.cree_par`).
6. `audit::log_action("UPDATE", "afrolang", "salle", entity_id=$salle_id, after={desactivee_admin_at, motif_desactivation})`.
7. `audit::log_action("CREATE", "afrolang", "evenement_moderation_salle", entity_id=$evt_id, after={type_action:'fermeture_admin', motif})`.

### Réponse 200

```json
{
  "success": true,
  "data": {
    "salle_id": "<uuid>",
    "session_id": "<uuid>",
    "fermeture": {
      "admin_id": "<uuid>",
      "motif": "...",
      "created_at": "2026-05-24T10:11:22Z"
    },
    "participants_notifies_count": 12
  },
  "error": null
}
```

---

## POST `/api/admin/afrolang/salles/{salle_id}/reactiver`

Réactive une salle précédemment désactivée par administration.

### Body (optionnel)

```json
{ "commentaire": "Sanctions individuelles appliquées, salle réautorisée." }
```

`commentaire` : 0..1000 chars.

### Préconditions

- Salle existe et `desactivee_admin_at IS NOT NULL`. Sinon **404** `code='salle_introuvable'` ou **409** `code='salle_non_desactivee'`.

### Effets (transaction)

1. `UPDATE afrolang.salle SET desactivee_admin_at=NULL, desactivee_par=NULL, motif_desactivation=NULL, reactivee_at=NOW(), reactivee_par=$me, commentaire_reactivation=$comm, updated_at=NOW() WHERE id=$1`.
2. `INSERT INTO afrolang.evenement_moderation_salle (salle_id, session_concernee_id=NULL, type_action='reactivation_admin', admin_id=$me, motif=$comm)`.
3. Hors transaction : notifier les admins de salle / créateur (type `afrolang.salle.reactivee_admin`).
4. Deux entrées d'audit (UPDATE salle + CREATE evenement).

### Réponse 200

```json
{
  "success": true,
  "data": {
    "salle_id": "<uuid>",
    "reactivee_at": "2026-05-24T11:00:00Z",
    "reactivee_par": "<uuid>",
    "commentaire": "..."
  },
  "error": null
}
```

---

## GET `/api/admin/afrolang/salles/{salle_id}/historique-moderation`

Historique chronologique des évènements de modération administrative pour une salle.

### Query parameters

| Paramètre | Type | Défaut |
|---|---|---|
| `page` | int | 1 |
| `limit` | int | 50 (max 100) |

### Réponse 200

```json
{
  "success": true,
  "data": [
    {
      "id": "<uuid>",
      "type_action": "fermeture_admin",
      "admin": { "id": "<uuid>", "nom": "Diop", "prenom": "Awa" },
      "session_concernee_id": "<uuid>",
      "motif": "Propos haineux récurrents...",
      "created_at": "2026-05-24T10:11:22Z"
    },
    {
      "id": "<uuid>",
      "type_action": "reactivation_admin",
      "admin": { "id": "<uuid>", "nom": "Sow", "prenom": "Modou" },
      "session_concernee_id": null,
      "motif": "Sanctions appliquées",
      "created_at": "2026-05-24T11:00:00Z"
    }
  ],
  "meta": { "total": 2, "page": 1, "limit": 50 }
}
```

Tri : `created_at DESC` (plus récent en premier).

---

## DELETE `/api/admin/afrolang/ressources-contribuees/{id}`

Retrait administratif d'une ressource jugée inappropriée. Réutilise l'endpoint public `DELETE /api/afrolang/ressources-contribuees/{id}` qui accepte déjà admin → on **ne crée pas** d'endpoint admin séparé (Principe V).

> **Note d'implémentation** : pas de nouvel endpoint, simplement étendre l'autorisation du `DELETE` public pour accepter `est_admin_plateforme(me)`. Audit identique avec acteur=admin.

---

## Lecture publique enrichie (impact)

Les endpoints publics existants `GET /api/afrolang/salles` et `GET /api/afrolang/salles/{id}` (livrés en `001-afrolang-salles-refonte` + `001-afrolang-pays-origine`) DOIVENT exposer dans leur DTO de réponse :

```json
{
  "...": "...",
  "desactivee_admin": {
    "desactivee_at": "2026-05-24T10:11:22Z",
    "motif": null   // exposé NULL en public ; rempli pour admin uniquement
  }
}
```

Champ `desactivee_admin = null` quand la salle est active. Le motif n'est inclus que si l'appelant est administrateur de la plateforme (cf. FR-020).

L'annuaire public continue d'inclure les salles désactivées (visibles avec badge), mais leur fiche n'autorise plus la jointure / création de session.
