# Contract : Afrolang Public API

**Feature**: 005-afrolang-salles
**Scope**: Endpoints publics (authentification JWT du membre). Tous sont montés sous `/api/afrolang/**`.

Les endpoints existants (visio, sessions LiveKit, tableau blanc existant) sont conservés tels quels. Cette section liste uniquement les **nouveaux endpoints** et **modifications** introduits par la feature.

Toutes les réponses suivent l'enveloppe standard UAfricas :

```json
{
  "success": true,
  "data": { /* ... */ },
  "error": null
}
```

En erreur : `success=false`, `data=null`, `error="message FR"`.

---

## Annuaire & salles publiques

### `GET /api/afrolang/groupes-ethniques`

Liste des groupes ethniques africains disponibles pour organiser l'annuaire des salles.

- **Auth** : optionnelle (lecture publique).
- **Query** : `q?` (recherche), `pays_id?`, `page?`, `limit?`.
- **Response** : `data: { items: GroupeEthniqueResume[], total, page, limit }` avec `GroupeEthniqueResume = { id, nom, pays_id, pays_nom, salle_id | null, salle_slug | null, salle_active }`.

### `GET /api/afrolang/salles` *(modifié)*

Ajout des filtres `groupe_ethnique_id?` et `langue_code?`. Chaque `SalleListeResponse` inclut désormais `groupe_ethnique`, `nombre_moderateurs_attitres`, `ressources_count`.

### `POST /api/afrolang/salles/propositions`

Soumission d'une proposition de salle publique pour un groupe ethnique absent.

- **Auth** : JWT requis (membre authentifié).
- **Body** : `{ nom_groupe_ethnique, pays_id?, groupe_ethnique_id?, langue_cible, description? }`.
- **Serveur** : détecte doublons (salle existante ou proposition `en_attente` avec nom normalisé) → `409 Conflict` + `{ error, data: { salle_id?, proposition_id? } }`. Sinon insertion `etat='en_attente'`, notification proposant.
- **Response** : `PropositionSalleResponse`.

### `GET /api/afrolang/salles/propositions/mine`

Liste les propositions du membre courant avec leur état.

- **Response** : `PropositionSalleResponse[]`.

---

## Modération de session

### `PUT /api/afrolang/sessions/{id}/moderation/transferer`

Transfert manuel du rôle de modérateur de session à un autre participant (FR-010).

- **Auth** : JWT requis, l'appelant doit être le modérateur de session actif.
- **Body** : `{ destinataire_id }` (participant cible présent dans la session).
- **Serveur** : vérifie que le destinataire est `session_participant.quitte_at IS NULL` ; `UPDATE session.moderateur_id`; notification aux deux parties.
- **Response** : `SessionDetailResponse` mis à jour.

### Logique implicite `POST /api/afrolang/sessions/{id}/rejoindre` *(modifié)*

Étend la logique existante :
- Si aucun modérateur actif dans la session : le nouvel arrivant devient modérateur de session (FR-009).
- Si un modérateur Afrolang attitré (via `salle_moderateur`) rejoint une session où le modérateur actuel n'est pas lui-même un Afrolang attitré : reprise automatique, ancien moderateur notifié (FR-011).

### Logique implicite `POST /api/afrolang/sessions/{id}/quitter` *(modifié)*

- Si l'appelant est le modérateur actif et qu'il reste des participants : réattribuer au plus ancien participant actif ; notifier (FR-012).

---

## Salles privées

### `GET /api/afrolang/salles/{salle_id}/privees` *(modifié)*

Filtre automatique : retourne uniquement les salles privées `visibilite='visible'` + celles où le membre courant est déjà `abonne` ou invité en `en_attente`. Masque les `fermee` des autres utilisateurs.

### `POST /api/afrolang/salles/{salle_id}/privees` *(modifié)*

Création d'une salle privée.

- **Auth** : JWT requis.
- **Body** :
  ```json
  {
    "titre": "string",
    "description": "string",
    "motif": "apprentissage_enfants | reseautage_adulte | echanges_groupe",
    "declaration_adulte": true,
    "visibilite": "fermee | visible",
    "max_participants": 50,
    "code_acces": "string?"
  }
  ```
- **Validation serveur** :
  - `declaration_adulte === true` sinon `400 Bad Request` (FR-016).
  - `motif` obligatoire ∈ énumération (FR-014).
  - Unicité métier : rejet via l'index unique partiel si le membre possède déjà une salle active dans cette salle publique (FR-035) → `409 Conflict`.
- **Response** : `SallePriveeDetailResponse`.

### `PATCH /api/afrolang/salles-privees/{id}/visibilite`

Bascule `fermee` ↔ `visible` (FR-019).

- **Auth** : JWT requis, créateur uniquement.
- **Body** : `{ visibilite: "fermee" | "visible" }`.
- **Response** : `SallePriveeDetailResponse`.

### Gestion des adhésions

| Méthode | Endpoint | Rôle appelant | Action |
|--------:|----------|---------------|--------|
| POST | `/api/afrolang/salles-privees/{id}/demandes` | Membre | Demande d'adhésion à une salle `visible` (FR-021). Si `max_participants` atteint → insertion `etat='groupe_complet'` automatique (FR-024). |
| POST | `/api/afrolang/salles-privees/{id}/invitations` | Créateur | Invite un membre (`{ utilisateur_id }`), fonctionne en `fermee` et `visible` (FR-020 + FR-025). |
| GET | `/api/afrolang/salles-privees/{id}/adhesions` | Créateur | Liste des lignes (demandes, invitations, abonnés). |
| PATCH | `/api/afrolang/adhesions/{id}/decision` | Créateur (pour `demande`) ou membre (pour `invitation`) | Body `{ decision: "acceptee" \| "refusee" }`. Sur `acceptee` : transition atomique + UPDATE type=`abonne` sous `SELECT ... FOR UPDATE` (SC-006). |
| DELETE | `/api/afrolang/adhesions/{id}` | Créateur | Retire un abonné (soft-delete). |

---

## Ressources de salle

### `GET /api/afrolang/salles/{salle_id}/ressources`

Liste des ressources publiées (+ celles ajoutées par l'appelant en `en_attente_validation` pour qu'il voie son propre suivi).

- **Response** : `RessourceSalleResponse[]`.

### `POST /api/afrolang/salles/{salle_id}/ressources/fichier`

Upload d'un fichier interne (multipart).

- **Auth** : JWT requis ; rôle : modérateur Afrolang attitré OU admin (FR-028). Les autres membres ne peuvent pas ajouter.
- **Content-Type** : `multipart/form-data` avec champs `titre`, `description?`, `fichier`.
- **Validation** : extension whitelist (`pdf`, `png`, `jpg`, `mp3`, `mp4`, `webm`, `ogg`, `wav`), taille max 50 Mo, `sanitize-filename`.
- **Serveur** : stockage `./uploads/afrolang/ressources/{uuid}-{sanitized}.ext` ; insertion `type='fichier'`, `etat='publiee'`.
- **Response** : `RessourceSalleResponse`.

### `POST /api/afrolang/salles/{salle_id}/ressources/lien`

Soumission d'un lien externe.

- **Auth** : JWT requis (tout membre peut proposer, conformément à la souplesse demandée par FR-028 ; modération préalable).
- **Body** : `{ titre, description?, lien_url }`.
- **Validation serveur** : URL valide, protocole `http` ou `https`, longueur ≤ 1000.
- **Serveur** : insertion `type='lien_externe'`, `etat='en_attente_validation'`.
- **Response** : `RessourceSalleResponse`.

### `DELETE /api/afrolang/ressources/{id}`

Soft-delete d'une ressource.

- **Auth** : JWT ; auteur OU modérateur Afrolang attitré de la salle OU admin.

---

## Messagerie instantanée écrite

### `GET /api/afrolang/sessions/{id}/messages`

Récupère l'historique des messages d'une session (utilisé au join pour reprendre le contexte).

- **Query** : `since?` (timestamp pour n'obtenir que les nouveaux), `limit?` (défaut 200, max 1000).
- **Response** : `MessageSessionResponse[]` ordonné par `created_at ASC`.

### `POST /api/afrolang/sessions/{id}/messages`

Publie un message écrit. Le frontend diffuse en parallèle via LiveKit data channel pour la latence temps réel.

- **Auth** : JWT + participant actif de la session (présent dans `session_participant` avec `quitte_at IS NULL`).
- **Body** : `{ contenu }` (1 ≤ longueur ≤ 4000 après trim).
- **Response** : `MessageSessionResponse`.

---

## Formats DTO (extraits clefs)

```ts
// Propositions
type EtatProposition = 'en_attente' | 'approuvee' | 'refusee';
interface PropositionSalleResponse {
  id: string;
  nom_groupe_ethnique: string;
  pays_id: string | null;
  groupe_ethnique_id: string | null;
  langue_cible: string | null;
  description: string | null;
  etat: EtatProposition;
  motif_refus: string | null;
  salle_id_creee: string | null;
  propose_par: string;
  decide_par: string | null;
  decide_at: string | null;
  created_at: string;
  updated_at: string;
}

// Adhésion
type TypeAdhesion = 'demande' | 'invitation' | 'abonne';
type EtatAdhesion = 'en_attente' | 'acceptee' | 'refusee' | 'groupe_complet';

interface AdhesionResponse {
  id: string;
  salle_privee_id: string;
  utilisateur_id: string;
  type: TypeAdhesion;
  etat: EtatAdhesion;
  initiateur_id: string;
  decideur_id: string | null;
  decided_at: string | null;
  created_at: string;
}

// Ressource
type TypeRessource = 'fichier' | 'lien_externe';
type EtatRessource = 'publiee' | 'en_attente_validation' | 'refusee';

interface RessourceSalleResponse {
  id: string;
  salle_id: string;
  titre: string;
  description: string | null;
  type: TypeRessource;
  fichier_url: string | null;
  lien_url: string | null;
  etat: EtatRessource;
  motif_refus: string | null;
  ajoute_par: string;
  valide_par: string | null;
  valide_at: string | null;
  created_at: string;
}

// Message
interface MessageSessionResponse {
  id: string;
  session_id: string;
  auteur_id: string;
  contenu: string;
  created_at: string;
}
```

---

## Codes HTTP standards

| Code | Sens |
|------|------|
| 200 | Succès lecture / transition |
| 201 | Ressource créée |
| 400 | Validation échouée (ex. `declaration_adulte=false`, motif invalide) |
| 401 | JWT manquant / expiré |
| 403 | Accès refusé (ex. modifier une visibilité sans être créateur) |
| 404 | Ressource introuvable |
| 409 | Doublon / contrainte unique (ex. 2e salle privée active, proposition de salle déjà en attente) |
| 413 | Upload trop volumineux |
| 422 | Format d'URL externe invalide |
| 500 | Erreur serveur inattendue |
