# Contrats d'API — Rendez-vous visioconférence

Toutes les routes sous `web::scope("/api")` → `web::scope("/rendez-vous")`. **JWT Bearer obligatoire** (extraction via le helper `utilisateur_courant(&req)` du domaine social). Enveloppe de réponse : `ApiResponse<T> { success, data, error }`. Sur chaque action mutante : revérification amitié active + absence de blocage (FR-034), audit `log_action` (sans sujet/description), push SSE + notification cloche le cas échéant.

Codes d'erreur : `400` Validation, `401` non authentifié, `403` AccesInterdit (non-ami/bloqué/non-participant), `404` introuvable, `409` Conflit (état/tour invalide, hors fenêtre, verrouillage optimiste).

---

## 1. Proposer un rendez-vous — `POST /api/rendez-vous`

Crée un rendez-vous `propose` (FR-006..FR-011). Notifie le destinataire (SSE `rdv_propose` + cloche).

**Requête**
```json
{
  "destinataire_id": "uuid",
  "sujet": "Mentorat carrière",
  "description": "Optionnel",
  "date_heure": "2026-06-01T14:00:00Z",
  "duree_minutes": 30
}
```

**Validations** : `destinataire_id <> moi` (FR-009) ; amis + non bloqués (FR-001/003) ; `sujet` 1..150 (FR-010) ; `duree_minutes ∈ {15,30,45,60}` (FR-007/010) ; `date_heure > NOW()` (FR-008).

**Réponse 201** : `ApiResponse<RendezVousResponse>` (statut `propose`, `tour_id = destinataire_id`, `mon_tour = false`).

---

## 2. Lister mes rendez-vous — `GET /api/rendez-vous?filtre=&page=`

Liste paginée des rendez-vous où l'utilisateur courant est participant (FR-019/020).

**Query** : `filtre ∈ {attente_moi, attente_autre, a_venir, passes}` (défaut : tous, triés par `date_heure`) ; `page` (défaut 1), `taille` (défaut 20).

**Réponse 200** : `ApiResponse<{ items: RendezVousResponse[], page, taille, total }>`.

---

## 3. Détail d'un rendez-vous — `GET /api/rendez-vous/{id}`

Détail pour un participant (sinon `403`/`404`).

**Réponse 200** : `ApiResponse<RendezVousResponse>`.

---

## 4. Accéder à la salle visio — `GET /api/rendez-vous/{id}/salle`

Renvoie la configuration P2P **seulement si** `statut='accepte'`, participant, amis + non bloqués, et `NOW() ∈ [date_heure − 5min, date_heure + duree + 15min]` (FR-024). Sinon `409`/`403`.

**Réponse 200**
```json
{
  "success": true,
  "data": {
    "rendez_vous_id": "uuid",
    "mon_peer_id": "uafr-3f9a…",
    "pair_peer_id": "uafr-b71c…",
    "suis_appelant": true,
    "autre": { "id": "uuid", "nom": "…", "prenom": "…", "slug": "…", "photoUrl": "…", "fonction": "…", "pays": "…" }
  }
}
```
`mon_peer_id`/`pair_peer_id` = `uafr-` + `hex(sha256(rendez_vous_id ‖ participant_id))` tronqué (déterministe, research §7). `suis_appelant = (moi_id < autre_id)`.

---

## 5. Accepter — `POST /api/rendez-vous/{id}/accepter`

Autorisé si `statut='propose' AND tour_id=moi` (verrouillage optimiste FR-035). → `statut='accepte'`, créneau figé (FR-013). Notifie l'autre (SSE `rdv_accepte` + cloche).

**Réponse 200** : `ApiResponse<RendezVousResponse>`. **409** si tour/statut invalide.

---

## 6. Refuser — `POST /api/rendez-vous/{id}/refuser`

Autorisé si `statut='propose' AND tour_id=moi`. → `statut='refuse'` (FR-014). Notifie l'autre (SSE `rdv_refuse` + cloche).

**Réponse 200** : `ApiResponse<RendezVousResponse>`.

---

## 7. Contre-proposer — `POST /api/rendez-vous/{id}/contre-proposer`

Autorisé si `statut='propose' AND tour_id=moi` (FR-015/016/017). Reste `propose`, met à jour date/heure/durée, **bascule** `tour_id` vers l'autre. Interdit si `accepte` (FR-018 → `409`).

**Requête**
```json
{ "date_heure": "2026-06-02T09:00:00Z", "duree_minutes": 45 }
```
**Validations** : `date_heure > NOW()`, `duree_minutes ∈ {15,30,45,60}`.

**Réponse 200** : `ApiResponse<RendezVousResponse>` (`mon_tour = false`). Notifie l'autre (SSE `rdv_contre_propose` + cloche).

---

## 8. Annuler — `POST /api/rendez-vous/{id}/annuler`

Autorisé à **l'un ou l'autre** participant si `statut ∈ {propose, accepte}` (FR-022). → `statut='annule'`. Notifie l'autre (SSE `rdv_annule` + cloche).

**Réponse 200** : `ApiResponse<RendezVousResponse>`. **409** si déjà terminal.

---

## DTO `RendezVousResponse`

```json
{
  "id": "uuid",
  "sujet": "string",
  "description": "string | null",
  "date_heure": "ISO-8601",
  "duree_minutes": 30,
  "statut": "propose | accepte | refuse | annule",
  "tour_id": "uuid",
  "mon_tour": true,
  "suis_initiateur": true,
  "etat_derive": "expire | termine | null",
  "peut_rejoindre": false,
  "autre": { "id": "uuid", "nom": "…", "prenom": "…", "slug": "…", "photoUrl": "…", "fonction": "…", "pays": "…" },
  "created_at": "ISO-8601",
  "updated_at": "ISO-8601"
}
```

## Événements SSE (canal existant `/api/messagerie/flux?token=`)

Forme générique poussée à l'autre participant (sans contenu sensible) :
```json
{ "type": "rdv_propose|rdv_accepte|rdv_refuse|rdv_contre_propose|rdv_annule", "rendez_vous_id": "uuid" }
```
Le frontend (plugin `messagerie.client.ts`) : transmet à `useRendezVous().gererEvenement` et rafraîchit la cloche (`useNotifications().compteurNonLues()`).
