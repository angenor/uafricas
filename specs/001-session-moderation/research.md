# Phase 0 : Research : Modération de session Afrolang

**Date** : 2026-05-10
**Spec** : [spec.md](./spec.md)

Quatre choix d'architecture ont été investigués. Aucun `NEEDS CLARIFICATION` n'est resté à la sortie de la phase de spécification ; cette section documente les arbitrages techniques pris en amont du data model.

---

## R1 : Enforcement serveur des permissions d'écriture sur le tableau blanc

**Décision** : utiliser le mécanisme natif LiveKit `RoomService::update_participant` (méthode `mutate_participant` du SDK `livekit-api`) pour basculer la `ParticipantPermission::can_publish_data` à chaque mutation. Le SFU LiveKit rejette lui-même tout `DataPacket` émis par un participant non autorisé. Le backend applique la mutation côté LiveKit immédiatement après le commit Postgres.

**Rationale** :
- Satisfait FR-015 (refus côté serveur) sans relais applicatif intermédiaire, l'enforcement est délégué au SFU, qui voit tout le trafic.
- Tirer parti d'une primitive existante de LiveKit (déjà serveur d'autorité du livestream) est plus simple qu'écrire un proxy WebSocket pour les opérations whiteboard.
- Cohérent avec le principe V (Simplicité, YAGNI) : pas de nouveau service ni de nouvelle dépendance.
- La latence d'application est sub-seconde (< 200 ms en pratique LiveKit), bien sous le budget SC-002 de 2 s.

**Alternatives évaluées** :
1. **Filtrage purement client** (chaque client reçoit la liste des autorisés et drop les packets venant d'un sender non autorisé), rejeté car un client malveillant peut publier librement (FR-015 non garanti).
2. **Relais applicatif** (les opérations whiteboard passent par un WebSocket backend qui valide et republie), rejeté car double chemin de transport, latence supplémentaire, et duplication de l'effort déjà accompli par LiveKit.
3. **Token LiveKit ré-émis** (générer un nouveau token sans `can_publish_data` et forcer une reconnexion), rejeté car interrompt la session vidéo/audio à chaque mutation de permission.

---

## R2 : Persistance de l'état spotlight

**Décision** : ajouter 3 colonnes nullables directement sur `afrolang.session` : `participant_mis_en_evidence_id UUID NULL`, `mis_en_evidence_par UUID NULL`, `mis_en_evidence_at TIMESTAMPTZ NULL`.

**Rationale** :
- Par FR-021 il y a **au plus une seule** mise en évidence active à la fois → relation 0..1 → colonnes nullables suffisent (table dédiée serait sur-ingénierie).
- L'historique des spotlights n'est pas une exigence (FR-030 utilise déjà `audit::log_action` pour la traçabilité hors-bande).
- Pas besoin d'index dédié : lecture par `session_id` (PK) déjà couverte.
- À la clôture de session (`etat='terminee'` + `termine_at`), un `UPDATE` met les trois colonnes à NULL (équivalent FR-017 pour le spotlight).

**Alternatives évaluées** :
1. **Table `afrolang.session_spotlight`** (PK = session_id, FK CASCADE), rejeté car la cardinalité 0..1 et l'absence de besoin historique ne justifient pas une table.
2. **Métadonnée de room LiveKit** (`set_room_metadata`), rejeté car (a) état non interrogeable depuis SQL pour audit/reporting, (b) duplication de source de vérité, (c) viole principe III (SQL source de vérité).

---

## R3 : Propagation temps réel des mutations vers les clients

**Décision** : après commit Postgres, le backend appelle `RoomServiceClient::send_data` (LiveKit) pour publier un `DataPacket` JSON `{ type: 'moderation', subtype: 'permission_update' | 'spotlight', payload: {...} }` à tous les participants de la room. Les clients Vue écoutent l'évènement `dataReceived` existant dans `useAfrolang` et mettent à jour leur état Pinia + UI.

**Rationale** :
- Le canal data LiveKit est déjà ouvert et utilisé (whiteboard, chat), aucune infra supplémentaire.
- Diffusion 1-à-N native, livraison fiable (`RELIABLE`), latence sub-seconde.
- Pas de risque de désynchronisation : à la connexion (`participantConnected`), le client fait un `GET` sur l'état courant des permissions/spotlight pour rattraper l'état (FR-024, pas de désynchronisation pour nouveaux entrants).

**Alternatives évaluées** :
1. **WebSocket applicatif dédié** côté backend Actix, rejeté pour duplication d'infrastructure (LiveKit déjà connecté).
2. **Polling SSE/HTTP toutes les 2 s** : rejeté pour latence et bruit réseau (50 participants × 30 req/min = 1500 req/min inutiles).
3. **NOTIFY/LISTEN PostgreSQL** : rejeté car incompatible avec le frontend (nécessiterait un pont WebSocket de toute façon).

---

## R4 : Source de la liste « participants éligibles au spotlight »

**Décision** : utiliser `afrolang.session_participant` (jointure avec `iam.utilisateur` pour le nom/avatar) filtré par `quitte_at IS NULL` comme source de vérité pour la liste affichée au modérateur. Inclut **tous les participants connectés**, indépendamment de l'état caméra/micro (réponse Q4 = B).

**Rationale** :
- Table déjà mise à jour à chaque `rejoint_at` / `quitte_at` par les handlers de session existants.
- Plus fiable que la liste des `Participant` du SDK LiveKit côté frontend (qui dépend de la fenêtre de souscription du client moderator).
- Cohérent avec FR-025 (spotlight retiré si la personne quitte) : un `UPDATE quitte_at` déclenche déjà un trigger applicatif (à étendre) qui annule le spotlight si la cible quitte.

**Alternatives évaluées** :
1. **Liste LiveKit live (`Room::participants`)**, rejeté car dépend du contexte côté client et peut diverger entre modérateurs en cas de désync momentanée.
2. **Filtrer par caméra/micro actif** : rejeté par décision utilisateur Q4 (= B, tous éligibles).

---

## R5 : Détection automatique du départ d'une personne mise en évidence

**Décision** : étendre le handler `POST /api/afrolang/sessions/{id}/quitter` (ou équivalent existant qui met à jour `session_participant.quitte_at`) pour effectuer en transaction :
1. `UPDATE session_participant SET quitte_at = NOW() WHERE ...`
2. `UPDATE afrolang.session SET participant_mis_en_evidence_id = NULL, mis_en_evidence_par = NULL, mis_en_evidence_at = NULL WHERE id = $session_id AND participant_mis_en_evidence_id = $utilisateur_qui_quitte`
3. Si la deuxième requête affecte 1 ligne → publier un `DataPacket` `moderation.spotlight` payload null pour notifier les clients (FR-025).

**Rationale** :
- Atomique au niveau Postgres (1 transaction), pas de race.
- Logique localisée dans le seul handler qui supprime un participant (clean + lisible).
- Couvre aussi les déconnexions involontaires si le serveur LiveKit notifie le backend via webhook `participant_left` (déjà câblé dans `services/livekit_*` existant, à vérifier en implémentation).

**Alternative évaluée** : trigger PostgreSQL → rejeté car cache la logique métier dans la BDD (principe V, lisibilité).

---

## R6 : Identification du rôle « modérateur de session » à l'exécution

**Décision** : créer un helper `est_moderateur_session(pool, session_id, utilisateur_id) -> Result<NiveauModerateur>` côté Rust qui retourne un enum `NiveauModerateur { AdminPlateforme, AdminSalle, ModerateurAttitre, CreateurSallePrivee, None }`. Logique :
1. Si l'utilisateur a un rôle global `administrateur` → `AdminPlateforme`.
2. Si la session est rattachée à une `salle_id` :
   - Présent dans `salle_administrateur` actif → `AdminSalle`.
   - Présent dans `salle_moderateur` actif → `ModerateurAttitre`.
3. Si la session est rattachée à une `salle_privee_id` :
   - `salle_privee.cree_par = utilisateur_id` → `CreateurSallePrivee`.
4. Sinon → `None`.

Le helper est appelé en début de chaque handler de modération, et le contrôle d'accès dépend du niveau :
- Mutations permissions tableau blanc : tous niveaux ≠ `None`.
- Mutations spotlight : `AdminPlateforme` | `AdminSalle` | `CreateurSallePrivee` uniquement (pas `ModerateurAttitre`) : voir FR-001b.

**Rationale** :
- Encode la hiérarchie de FR-001/FR-001b en un seul point.
- Retourne le niveau (et non un bool) pour permettre la différenciation spotlight vs permissions.
- Recalculé à chaque appel (FR-003) : aucun cache, pas de risque de privilège résiduel après révocation.

**Alternatives évaluées** :
1. **Middleware Actix** qui pose le niveau dans `Extensions`, rejeté car nécessite de connaître `session_id` (path) avant de pouvoir interroger les tables ; complexification du middleware.
2. **Vue SQL `session_moderateur_v`** : rejeté car la jointure utilisateur×session est de toute façon faite côté Rust pour l'audit (auteur), donc pas de gain.

---

## Synthèse

Aucune dépendance nouvelle. Aucun risque architectural identifié. Toutes les contraintes de la constitution sont respectées. Prêt pour la phase 1 (data model + contracts + quickstart).
