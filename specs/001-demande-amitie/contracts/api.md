# API Contracts : Demande d'amitié & messagerie

**Feature**: `001-demande-amitie` | **Base**: `/api`

Toutes les routes (sauf le flux SSE) suivent l'enveloppe projet `ApiResponse<T>` : `{ "succes": bool, "donnees": T | null, "message": string | null, "meta": {...} | null }`. Toutes requièrent un JWT valide (en-tête `Authorization: Bearer <jwt>`), sauf indication contraire. L'utilisateur courant est extrait du token ; aucun endpoint n'accepte un `utilisateur_id` arbitraire comme acteur.

Codes d'erreur usuels : `401` non authentifié, `403` interdit (blocage), `404` introuvable, `409` conflit (doublon / déjà traité), `422` validation, `429` rate-limit.

---

## Amitié & relations

### `GET /api/amities/etat/{utilisateur_id}`
État de la relation entre l'utilisateur courant et le membre ciblé (FR-016), alimente le bouton sur `/profil` et `/profil/{id}`.
- **200** `{ etat: "aucune" | "demande_envoyee" | "demande_recue" | "amis" | "bloque_par_moi" | "indisponible", demande_id?: UUID }`
- `indisponible` si membre non actif / supprimé (FR-015), `bloque_par_moi` si l'utilisateur courant a bloqué la cible.

### `POST /api/amities/etats`
États relationnels **en lot** pour l'annuaire `/profil`, évite le N+1 (FR-016).
- **Body** `{ utilisateur_ids: UUID[] }` (≤ 50)
- **200** `{ "<utilisateur_id>": "aucune" | "demande_envoyee" | "demande_recue" | "amis" | "bloque_par_moi" | "indisponible", ... }` (une seule requête).

### `POST /api/amities/demandes`
Envoyer une demande (FR-001, US1).
- **Body** `{ destinataire_id: UUID }`
- **201** demande créée (`{ demande_id, statut: "en_attente" }`).
- **200** `{ statut: "amis" }` si demande croisée → auto-acceptation (FR-009, R3).
- **403** si l'un a bloqué l'autre (FR-013). **409** si doublon/amitié existante (FR-003). **422** auto-demande (FR-002) ou destinataire indisponible (FR-015). **429** rate-limit (FR-014).
- Audit : `CREATE` `social` `demande_amitie`. Notification `demande_recue` au destinataire (FR-005).

### `GET /api/amities/demandes/recues`
Demandes en attente reçues (US2/US4). Pagination. → liste `{ demande_id, demandeur: MembreLight, created_at }`.

### `GET /api/amities/demandes/envoyees`
Demandes en attente envoyées (US4). Pagination. → liste `{ demande_id, destinataire: MembreLight, created_at }`.

### `POST /api/amities/demandes/{id}/accepter`
Accepter (FR-007, US2). Transaction : statut→`acceptee`, création `amitie`, notification `demande_acceptee` à l'émetteur.
- **200** `{ statut: "amis" }`. **409** si déjà traitée. **403** si non destinataire.
- Audit : `UPDATE` `demande_amitie`.

### `POST /api/amities/demandes/{id}/refuser`
Refuser (FR-008, US2). Statut→`refusee`. **Pas** de notification à l'émetteur.
- **200**. **409** si déjà traitée. **403** si non destinataire. Audit : `UPDATE`.

### `DELETE /api/amities/demandes/{id}`
Annuler une demande émise tant qu'en attente (FR-010, US4). Statut→`annulee`.
- **200**. **403** si non émetteur. **409** si déjà traitée. Audit : `UPDATE`.

### `GET /api/amities`
Liste des amis de l'utilisateur courant (FR-011, FR-026, privée). Pagination, recherche optionnelle.
- **200** liste `{ utilisateur: MembreLight, ami_depuis: timestamp }`.

### `DELETE /api/amities/{utilisateur_id}`
Retirer un ami (FR-012). Supprime `amitie` (les deux côtés). La conversation est conservée mais verrouillée (FR-025).
- **200**. **404** si pas amis. Audit : `DELETE` `amitie`.

### `POST /api/blocages`
Bloquer un membre (FR-013). Transaction : crée `blocage`, supprime amitié + demandes actives, verrouille la conversation.
- **Body** `{ utilisateur_id: UUID }`
- **200**. **422** auto-blocage. **409** déjà bloqué. Audit : `CREATE` `social` `blocage`.

### `DELETE /api/blocages/{utilisateur_id}`
Débloquer (FR-013).
- **200**. **404** si non bloqué. Audit : `DELETE` `blocage`.

### `GET /api/blocages`
Liste des membres bloqués par l'utilisateur courant. → liste `{ utilisateur: MembreLight, depuis: timestamp }`.

---

## Messagerie (US3)

### `GET /api/messagerie/conversations`
Liste des conversations de l'utilisateur (FR-020), triées par `dernier_message_at` desc.
- **200** liste `{ conversation_id, ami: MembreLight, dernier_message?: { extrait, created_at }, non_lus: number, verrouillee: bool }`.
- `verrouillee=true` si l'amitié n'existe plus ou blocage (FR-025) → lecture seule.

### `GET /api/messagerie/conversations/{ami_id}/messages`
Historique paginé avec un ami (FR-023). Crée la conversation à la volée si amis et inexistante.
- **Query** `avant?: timestamp, limite?: number (<=50)`
- **200** liste `{ id, expediteur_id, contenu | null, supprime: bool, created_at, lu_at }` (contenu `null` + `supprime:true` si soft-deleted, FR-028).
- **403** si non amis / bloqué (FR-022).

### `POST /api/messagerie/conversations/{ami_id}/messages`
Envoyer un message (FR-021). Persiste puis pousse via SSE au destinataire (et aux autres connexions de l'expéditeur).
- **Body** `{ contenu: string }` (1..2000, FR-027)
- **201** `{ message: {...} }`. **403** non amis / bloqué (FR-022). **422** contenu vide ou > 2000.
- **Non audité** (Décision 9).

### `POST /api/messagerie/conversations/{ami_id}/lu`
Marquer comme lus tous les messages reçus de cet ami (FR-024). Met `lu_at=now()`. Pousse un évènement de mise à jour des non-lus.
- **200** `{ non_lus: 0 }`.

### `DELETE /api/messagerie/messages/{id}`
Supprimer un de ses propres messages (FR-028). Soft delete + push SSE de mise à jour.
- **200**. **403** si non expéditeur. **Non audité**.

### `GET /api/messagerie/non-lus`
Compteur global de messages non lus, pour le badge du bouton flottant (FR-024).
- **200** `{ total: number }`.

### `GET /api/messagerie/flux?token=<jwt>`, **SSE**
Flux temps réel serveur→client (Décision 2 & 3). **Pas** d'enveloppe `ApiResponse` ; `Content-Type: text/event-stream`.
- Auth via query param `token` (EventSource ne supporte pas les en-têtes).
- Évènements émis (chacun `data: <json>`):
  - `{ type: "message", conversation_id, message: {...} }`, nouveau message reçu.
  - `{ type: "message_supprime", conversation_id, message_id }`.
  - `{ type: "non_lus", conversation_id, non_lus }`, mise à jour de compteur.
  - `{ type: "demande_recue", demande_id, demandeur: MembreLight }` (FR-005).
  - `{ type: "demande_acceptee", utilisateur: MembreLight }` (FR-007).
  - `: keep-alive` (commentaire) périodique pour maintenir la connexion.
- **401** si token invalide. Fermeture propre à la déconnexion ; reconnexion client avec token rafraîchi.

---

## Notifications relationnelles

### `GET /api/amities/notifications`
Notifications sociales de l'utilisateur (FR-017). Pagination.
- **200** liste `{ id, type, demande_id?, acteur: MembreLight, lu, created_at }`.

### `PATCH /api/amities/notifications/{id}/lu`
Marquer une notification lue (FR-017). **200**.

### `PATCH /api/amities/notifications/tout-lu`
Tout marquer lu. **200**.

---

## Type partagé `MembreLight`
Champs publics uniquement (cohérent avec `models/membre.rs`, jamais d'email/téléphone) :
`{ id, nom, prenom, slug, photoUrl?, fonction?, pays? }`.
