# Contrat REST — Direct d'événement

Toutes les routes sont sous le scope public existant `/api/evenements`, préfixe `/{id}/direct` (où `{id}` = `evenement_id`). Authentification : JWT `Authorization: Bearer <access>` (in-handler, pattern `extraire_utilisateur_id`). Enveloppe standard : `ApiResponse<T> = { success: bool, data: T | null, error: string | null }`.

Codes : `200/201` succès, `401` non connecté, `403` non éligible (non inscrit / non organisateur), `404` événement/session introuvable, `409` conflit (capacité atteinte, déjà ouvert, état incohérent), `422` validation (format non diffusable, hors fenêtre).

---

## `GET /api/evenements/{id}/direct`

État du direct pour l'appelant (lecture ; JWT optionnel mais recommandé pour `mon_role`). Applique l'arrêt de sécurité paresseux (D6).

**200** →
```json
{
  "success": true,
  "data": {
    "statut_direct": "en_attente",          // indisponible | en_attente | en_direct | termine
    "peut_ouvrir": true,                      // organisateur + fenêtre OK + pas de session active
    "peut_rejoindre": false,                  // session en_cours + éligible + capacité OK
    "est_organisateur": true,
    "est_inscrit": true,
    "session_id": null,                       // UUID si en_direct
    "nombre_participants": 0,
    "max_participants": 100,
    "fenetre_ouverture_at": "2026-06-01T13:45:00Z",
    "demandes_parole": []                     // [{utilisateur_id, nom, main_levee_at}] — uniquement si est_organisateur
  },
  "error": null
}
```

---

## `POST /api/evenements/{id}/direct/rejoindre`

Ouvre la session si l'appelant est l'organisateur et qu'aucune n'est active, sinon rejoint la session active. Renvoie le token LiveKit scopé selon le rôle (D2). Crée/réactive l'enregistrement participant (`quitte_at = NULL`).

- **Garde** : connecté (401) ; organisateur OU inscrit (403) ; format diffusable + fenêtre OK pour l'ouverture (422) ; capacité (409, D8).
- **Body** : aucun.

**201 / 200** →
```json
{
  "success": true,
  "data": {
    "session_id": "…uuid…",
    "room_name": "evenement-…uuid…",
    "livekit_url": "wss://…",
    "token": "eyJ…",
    "role": "spectateur"                      // organisateur | intervenant | spectateur
  },
  "error": null
}
```
Grants du token : organisateur/intervenant → `can_publish:true`; spectateur → `can_publish:false`; tous → `can_subscribe:true, can_publish_data:true`.

---

## `POST /api/evenements/{id}/direct/quitter`

Marque l'appelant sorti (`quitte_at = NOW()`, cumule `duree_secondes`). Idempotent.

**200** → `{ "success": true, "data": { "quitte": true }, "error": null }`

---

## `POST /api/evenements/{id}/direct/cloturer`

Clôture la session active (`etat='terminee'`, `termine_at`, `duree_secondes`). Diffuse le DataPacket `session_fermee` puis `delete_room` (best-effort, non bloquant). **Organisateur uniquement** (403 sinon).

**200** → `{ "success": true, "data": { "cloture": true }, "error": null }`

---

## `POST /api/evenements/{id}/direct/lever-main`

Bascule `main_levee` de l'appelant (spectateur). Diffuse un DataPacket `{type:'moderation', subtype:'main_levee', payload:{utilisateur_id, levee:bool}}`. **Spectateur uniquement** (422 si déjà intervenant/organisateur).

**Body** (optionnel) : `{ "levee": true }` (défaut : toggle).

**200** → `{ "success": true, "data": { "main_levee": true }, "error": null }`

---

## `POST /api/evenements/{id}/direct/participants/{utilisateur_id}/promouvoir`

Promeut un spectateur en intervenant : `role='intervenant'`, `main_levee=false`, `update_participant_can_publish(room, identity, true)` (D3), DataPacket `{subtype:'role_update', payload:{utilisateur_id, role:'intervenant'}}`. **Organisateur uniquement**.

**200** → `{ "success": true, "data": { "role": "intervenant" }, "error": null }`

---

## `POST /api/evenements/{id}/direct/participants/{utilisateur_id}/retrograder`

Rétrograde un intervenant en spectateur : `role='spectateur'`, `update_participant_can_publish(room, identity, false)`, DataPacket `{subtype:'role_update', payload:{utilisateur_id, role:'spectateur'}}`. **Organisateur uniquement**.

**200** → `{ "success": true, "data": { "role": "spectateur" }, "error": null }`

---

## `POST /api/evenements/{id}/direct/participants/{utilisateur_id}/retirer`

Retire (kick) un participant : `quitte_at=NOW()`, `retirer_participant(room, identity)` (D3), DataPacket `{subtype:'retire', payload:{utilisateur_id}}`. **Organisateur uniquement**. Ne s'applique pas à l'organisateur lui-même (422).

**200** → `{ "success": true, "data": { "retire": true }, "error": null }`

---

## Audit (Principe VII)

`audit::log_action(pool, Some(moi), action, "media_content", "evenement_session", Some(session_id), None, Some(nouvel_etat_json), ip, ua)` pour : `OUVRIR`, `CLOTURER`, `PROMOUVOIR`, `RETROGRADER`, `RETIRER`. `nouvel_etat_json` ne contient **aucun** contenu de chat ni média (rôle, utilisateur cible, état session seulement).
