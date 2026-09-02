# Phase 1 : Contracts API : Modération de session Afrolang

**Date** : 2026-05-10
**Base path** : `/api/afrolang/sessions/{session_id}`
**Auth** : JWT obligatoire sur tous les endpoints (header `Authorization: Bearer ...`)
**Wrapper** : `ApiResponse<T>` standard du projet (`{ success: bool, data: T | null, error: string | null }`).

## Permissions tableau blanc

### GET `/api/afrolang/sessions/{session_id}/permissions-tableau-blanc`

Liste les permissions individuelles accordées dans la session courante. Accessible à tout participant authentifié de la session (lecture seule, pour que chaque client puisse savoir s'il est autorisé).

**Réponse 200**
```json
{
  "success": true,
  "data": {
    "session_id": "uuid",
    "moderateurs_office": [
      {"utilisateur_id": "uuid", "nom_complet": "Alice K.", "avatar_url": null, "niveau": "admin_salle"}
    ],
    "permissions_individuelles": [
      {
        "utilisateur_id": "uuid",
        "nom_complet": "Bob N.",
        "avatar_url": "https://...",
        "accorde_par": "uuid",
        "accorde_at": "2026-05-10T14:32:00Z"
      }
    ],
    "mon_niveau_moderateur": "createur_salle_privee"
  }
}
```

**Erreurs** : 401 (non auth), 403 (non participant de la session), 404 (session inexistante).

---

### POST `/api/afrolang/sessions/{session_id}/permissions-tableau-blanc`

Accorde la permission d'écriture à un participant.

**Autorisation** : tout `NiveauModerateur` ≠ `None` (FR-001/FR-001b).

**Body**
```json
{ "utilisateur_id": "uuid" }
```

**Effets serveur** (transaction Postgres + appel LiveKit) :
1. `INSERT INTO afrolang.session_permission_tableau_blanc (...) ON CONFLICT DO NOTHING`
2. `RoomServiceClient::update_participant(room=session_id, identity=utilisateur_id, permission.can_publish_data=true)`
3. `audit::log_action("CREATE", "afrolang", "session_permission_tableau_blanc", session_id, ...)`
4. `RoomServiceClient::send_data(...)` → DataPacket `moderation.permission_update` (voir plus bas)

**Réponse 201**
```json
{
  "success": true,
  "data": {
    "utilisateur_id": "uuid",
    "nom_complet": "Bob N.",
    "avatar_url": null,
    "accorde_par": "uuid",
    "accorde_at": "2026-05-10T14:32:00Z"
  }
}
```

**Erreurs** : 401, 403 (non modérateur de session), 404 (session ou utilisateur introuvable), 409 (l'utilisateur cible est déjà un modérateur de session, pas besoin de permission explicite, retour ApiError "L'utilisateur est déjà modérateur de session"), 422 (utilisateur_id manquant ou malformé).

---

### DELETE `/api/afrolang/sessions/{session_id}/permissions-tableau-blanc/{utilisateur_id}`

Retire une permission précédemment accordée.

**Autorisation** : tout `NiveauModerateur` ≠ `None`.

**Effets serveur** :
1. Vérification préalable : si la cible est elle-même modérateur de session → `409 "Cette permission ne peut pas être retirée à un modérateur"` (FR-013).
2. `DELETE FROM afrolang.session_permission_tableau_blanc WHERE session_id=$1 AND utilisateur_id=$2`
3. `RoomServiceClient::update_participant(... can_publish_data=false)`
4. `audit::log_action("DELETE", ...)`
5. DataPacket `moderation.permission_update`

**Réponse 204** (No Content)

**Erreurs** : 401, 403, 404 (permission inexistante), 409 (cible est modérateur).

---

## Spotlight (sessions publiques livestreamées uniquement)

### POST `/api/afrolang/sessions/{session_id}/spotlight`

Met en évidence un participant ; remplace automatiquement la mise en évidence en cours (FR-021).

**Autorisation** : `AdminPlateforme` ou `AdminSalle` uniquement (FR-001b). Session DOIT avoir `salle_id IS NOT NULL` (publique), sinon 422.

**Body**
```json
{ "utilisateur_id": "uuid" }
```

**Effets serveur** :
1. Vérification : la cible est dans `session_participant` avec `quitte_at IS NULL` (sinon 404 "participant absent de la session").
2. `UPDATE afrolang.session SET participant_mis_en_evidence_id=$1, mis_en_evidence_par=$auteur, mis_en_evidence_at=NOW() WHERE id=$session_id`
3. `audit::log_action("UPDATE", "afrolang", "session", session_id, before={spotlight_id=ancien}, after={spotlight_id=nouveau})`
4. DataPacket `moderation.spotlight` payload `SpotlightInfo`.

**Réponse 200**
```json
{
  "success": true,
  "data": {
    "utilisateur_id": "uuid",
    "nom_complet": "Carole M.",
    "avatar_url": null,
    "mis_en_evidence_par": "uuid",
    "mis_en_evidence_at": "2026-05-10T14:35:12Z"
  }
}
```

**Erreurs** : 401, 403 (pas admin plateforme/salle ; un modérateur attitré recevra explicitement 403), 404 (cible non présente), 422 (session privée → spotlight non disponible).

---

### DELETE `/api/afrolang/sessions/{session_id}/spotlight`

Désactive la mise en évidence active (FR-022).

**Autorisation** : `AdminPlateforme` ou `AdminSalle`.

**Effets serveur** :
1. `UPDATE afrolang.session SET participant_mis_en_evidence_id=NULL, mis_en_evidence_par=NULL, mis_en_evidence_at=NULL WHERE id=$session_id`
2. `audit::log_action("UPDATE", ..., after={spotlight_id=null})`
3. DataPacket `moderation.spotlight` payload `null`.

**Réponse 204**

**Erreurs** : 401, 403, 404 (session inexistante), 422 (session privée).

---

## DataPackets LiveKit (canal `RELIABLE`)

Le backend publie via `RoomServiceClient::send_data` ; les clients écoutent dans `useAfrolang` (`room.on('dataReceived', ...)`).

### `moderation.permission_update`

```json
{
  "type": "moderation",
  "subtype": "permission_update",
  "payload": {
    "session_id": "uuid",
    "utilisateur_id": "uuid",
    "action": "accordee" | "retiree",
    "accorde_par": "uuid",
    "accorde_at": "2026-05-10T14:32:00Z"
  }
}
```

**Réaction client** :
- Si `utilisateur_id` === local participant identity → bascule la barre d'outils du tableau blanc (active ↔ lecture seule, FR-018).
- Mise à jour du panneau modération (recharge ou patch local).

### `moderation.spotlight`

```json
{
  "type": "moderation",
  "subtype": "spotlight",
  "payload": null
}
```

ou

```json
{
  "type": "moderation",
  "subtype": "spotlight",
  "payload": {
    "utilisateur_id": "uuid",
    "nom_complet": "Carole M.",
    "avatar_url": null,
    "mis_en_evidence_par": "uuid",
    "mis_en_evidence_at": "2026-05-10T14:35:12Z"
  }
}
```

**Réaction client** : `AfrolangVideoGrid.vue` met en avant la tuile correspondante (centre, bordure `border-2 border-custom-chocolat`, libellé « En vedette »). Si `payload === null` → retour à la disposition par défaut.

---

## Endpoint complémentaire (état initial à la connexion, FR-024)

L'endpoint `GET /api/afrolang/sessions/{session_id}` existant DOIT être étendu pour inclure dans sa réponse :

```json
{
  "id": "uuid",
  "...": "...",
  "permissions_tableau_blanc_count": 3,
  "spotlight": null | SpotlightInfo
}
```

Cela permet à un client qui rejoint en cours de session de connaître immédiatement l'état spotlight sans appel supplémentaire (alternative : faire le `GET /permissions-tableau-blanc` séparément pour la liste détaillée).

---

## Tableau récapitulatif

| Méthode | Path | Rôles autorisés | Effet LiveKit |
|---|---|---|---|
| GET | `/api/afrolang/sessions/{id}/permissions-tableau-blanc` | tout participant |, |
| POST | `/api/afrolang/sessions/{id}/permissions-tableau-blanc` | tout modérateur de session | `update_participant(can_publish_data=true)` + `send_data` |
| DELETE | `/api/afrolang/sessions/{id}/permissions-tableau-blanc/{user_id}` | tout modérateur de session | `update_participant(can_publish_data=false)` + `send_data` |
| POST | `/api/afrolang/sessions/{id}/spotlight` | admin plateforme \| admin salle | `send_data` |
| DELETE | `/api/afrolang/sessions/{id}/spotlight` | admin plateforme \| admin salle | `send_data` |
