# Contract — API publique salles privées Afrolang

**Branch** : `001-afrolang-salles-refonte`
**Date** : 2026-04-15
**Base path** : `/api/afrolang`
**Auth** : JWT obligatoire sur tous les endpoints (utilisateur connecté).

Tous les payloads / réponses sont en JSON et s'inscrivent dans le wrapper `ApiResponse<T>` standard du projet :

```json
{ "success": true, "data": { ... }, "message": null }
{ "success": false, "data": null, "message": "Code incorrect" }
```

---

## Endpoint 1 — Créer une salle privée

`POST /api/afrolang/salles-privees`

**Auth** : utilisateur connecté.

**Request body** :

```json
{
  "salle_id": "uuid-de-la-salle-publique",
  "titre": "Mon cercle Wolof du soir",
  "description": "Conversations légères en wolof, niveau intermédiaire.",
  "code_acces": "wolof2026"
}
```

**Validations** :

- `salle_id` : UUID valide, salle publique existante et `actif=true`.
- `titre` : string, 5 à 350 caractères.
- `description` : string, 0 à 1000 caractères, optional.
- `code_acces` : string, regex `^[A-Za-z0-9!@#$%&*?-]{4,16}$`.

**Règles métier** :

- Refus si le couple `(salle_id, utilisateur courant)` a déjà une salle privée non archivée et non supprimée (FR-010).
- Hash du `code_acces` via bcrypt cost 10 avant INSERT.
- Audit : `audit::log_action("creer_salle_privee", utilisateur_id, "afrolang.salle_privee", id, ip, ua, before=NULL, after={titre, salle_id})`.

**Response 201 Created** :

```json
{
  "success": true,
  "data": {
    "id": "uuid-de-la-salle-privee-creee",
    "salle_id": "uuid-salle-publique",
    "titre": "Mon cercle Wolof du soir",
    "description": "Conversations légères en wolof, niveau intermédiaire.",
    "auteur_id": "uuid-utilisateur",
    "auteur_nom": "Aïssatou Diop",
    "session_en_cours": false,
    "created_at": "2026-04-15T14:30:00Z"
  },
  "message": null
}
```

**Erreurs** :

| Code | Cause |
|---|---|
| 400 | Validation payload (titre court, code mal formé, salle inexistante) |
| 401 | JWT manquant/invalide |
| 409 | Une salle privée existe déjà pour (utilisateur, salle publique) — message « Vous avez déjà une salle privée pour cette salle publique », `data.salle_privee_existante_id` rendu pour bouton « Ouvrir ma salle privée » |
| 422 | Salle publique inactive |

---

## Endpoint 2 — Lister les salles privées d'une salle publique

`GET /api/afrolang/salles/{salle_id}/salles-privees`

**Auth** : utilisateur connecté.

**Réponse 200 OK** :

```json
{
  "success": true,
  "data": [
    {
      "id": "uuid-1",
      "titre": "Mon cercle Wolof du soir",
      "auteur_id": "uuid-aissatou",
      "auteur_nom": "Aïssatou Diop",
      "session_en_cours": true,
      "est_auteur": false,
      "created_at": "2026-04-15T14:30:00Z"
    },
    {
      "id": "uuid-2",
      "titre": "Atelier conjugaison",
      "auteur_id": "uuid-courant",
      "auteur_nom": "Vous",
      "session_en_cours": false,
      "est_auteur": true,
      "created_at": "2026-04-10T09:15:00Z"
    }
  ],
  "message": null
}
```

**Notes** :

- `est_auteur` est calculé côté backend par comparaison `cree_par == utilisateur_courant`. Permet au frontend de proposer « Ouvrir ma salle privée » sans demander de code (FR-014).
- `code_acces_hash` n'est JAMAIS exposé.
- `description` est exposée pour permettre l'affichage en infobulle si souhaité (optionnel).
- Liste filtrée sur `archivee_at IS NULL AND deleted_at IS NULL`.

---

## Endpoint 3 — Vérifier le code secret et obtenir l'accès

`POST /api/afrolang/salles-privees/{id}/verifier-code`

**Auth** : utilisateur connecté.

**Request body** :

```json
{ "code_acces": "wolof2026" }
```

**Logique handler** :

1. Charger `salle_privee` ; si non trouvée ou archivée → 404 (sans détails).
2. Si `cree_par == utilisateur courant` → 200 immédiat avec token d'accès (FR-014 : auteur entre sans code).
3. Compter les tentatives échouées du couple `(salle_privee_id, utilisateur_id)` sur les 60 dernières secondes.
4. Si count ≥ 5 ET dernière tentative < 5 min → 429 Too Many Requests (sans vérifier le hash).
5. Vérifier `bcrypt::verify(code_acces, code_acces_hash)`.
6. INSERT dans `afrolang.tentative_code_acces` (succès ou échec, IP, UA).
7. Si échec → 403 Forbidden, message générique « Code incorrect ».
8. Si succès → 200 OK avec `acces_jeton` (cf. ci-dessous).

**Response 200 OK** (succès ou auteur) :

```json
{
  "success": true,
  "data": {
    "salle_privee_id": "uuid",
    "acces_jeton": "jwt-court-livre-au-frontend",
    "expires_at": "2026-04-15T18:30:00Z"
  },
  "message": null
}
```

`acces_jeton` est un JWT court (durée = durée de la session applicative ou max 4 h, claim `salle_privee_id`) signé par le backend, à présenter à l'endpoint 4 pour démarrer/rejoindre. Permet de mémoriser l'accès pendant la session sans re-saisir le code (cf. A2 spec).

**Erreurs** :

| Code | Cause |
|---|---|
| 401 | JWT manquant |
| 403 | Code incorrect |
| 404 | Salle privée inexistante / archivée (message générique pour ne pas fuiter) |
| 429 | Rate limit (5 tentatives / min, verrouillage 5 min) |

**Audit** : `audit::log_action("verifier_code_salle_privee_echec", …)` uniquement sur échec (les succès ne génèrent pas d'audit pour ne pas saturer ; succès loggés implicitement par la session démarrée endpoint 4).

---

## Endpoint 4 — Démarrer / rejoindre la session live d'une salle privée

`POST /api/afrolang/salles-privees/{id}/sessions/demarrer-ou-rejoindre`

**Auth** : utilisateur connecté + header `X-Afrolang-Acces-Jeton: <jeton-issu-endpoint-3>`.

**Logique** :

1. Valider `acces_jeton` (signature + claim `salle_privee_id == {id}` + non expiré).
2. Charger la salle privée ; si archivée → 410 Gone.
3. Chercher `afrolang.session` avec `salle_privee_id={id} AND etat='en_cours'`.
   - Si trouvée → réutiliser, INSERT `afrolang.session_participant`.
   - Si absente → INSERT nouvelle `afrolang.session` (`etat='en_cours'`, `moderateur_id=salle_privee.cree_par`, `cree_par=utilisateur courant` — qui peut différer de l'auteur si celui-ci n'est pas en ligne).
4. Émettre un token LiveKit (déjà géré par crate `livekit-api`).

**Response 200 OK** :

```json
{
  "success": true,
  "data": {
    "session_id": "uuid",
    "livekit_url": "wss://livekit.uafricas.local",
    "livekit_token": "jwt-livekit",
    "moderateur_id": "uuid-auteur"
  }
}
```

**Erreurs** :

| Code | Cause |
|---|---|
| 401 | JWT principal manquant ou `acces_jeton` invalide/expiré |
| 410 | Salle archivée |
| 503 | LiveKit indisponible |

**Audit** : `audit::log_action("rejoindre_session_salle_privee", …)`.

---

## Endpoint 5 — Modifier le code secret

`PATCH /api/afrolang/salles-privees/{id}/code-acces`

**Auth** : utilisateur connecté + utilisateur = `cree_par`.

**Request body** :

```json
{ "nouveau_code_acces": "nouveauCode!" }
```

**Validation** : regex identique endpoint 1.

**Response 204 No Content**.

**Erreurs** :

| Code | Cause |
|---|---|
| 400 | Format invalide |
| 401 | JWT manquant |
| 403 | Utilisateur non auteur |
| 404 | Salle inexistante |

**Audit** : `audit::log_action("modifier_code_salle_privee", …)`. Le hash before/after est tracé en `before/after JSONB` (les hashes uniquement, jamais les plaintexts).

---

## Endpoint 6 — Archiver sa salle privée

`POST /api/afrolang/salles-privees/{id}/archiver`

**Auth** : utilisateur connecté + utilisateur = `cree_par`.

**Logique** :

- UPDATE `afrolang.salle_privee` SET `archivee_at = NOW()`.
- Si une session live est en cours → la terminer (`etat='terminee'`).
- Libère le verrou unicité `(salle_id, cree_par)` → l'utilisateur peut recréer une nouvelle salle privée si désiré.

**Response 204 No Content**.

**Erreurs** :

| Code | Cause |
|---|---|
| 401 | JWT manquant |
| 403 | Non auteur |
| 404 | Salle inexistante |

**Audit** : `audit::log_action("archiver_salle_privee", …)`.

---

## Endpoints SUPPRIMÉS (legacy à retirer)

| Méthode | Path | Raison |
|---|---|---|
| GET / POST | `/api/afrolang/salles-privees/{id}/adhesions` | Adhésion abandonnée |
| POST | `/api/afrolang/salles-privees/{id}/adhesions/{aid}/decider` | Adhésion abandonnée |
| POST | `/api/afrolang/salles-privees/{id}/inviter` | Invitation abandonnée |
| GET / POST / DELETE | `/api/afrolang/salles-privees/{id}/moderateurs` | Modération salle privée abandonnée |
| PATCH | `/api/afrolang/salles-privees/{id}/visibilite` | Visibilité abandonnée |
| GET / POST | `/api/afrolang/propositions-salle` | Création publique = admin uniquement |

---

## Cohérence avec les exigences spec

| FR / SC | Endpoint(s) couvrant |
|---|---|
| FR-008 | 1 |
| FR-009 / FR-010 | 1 (validations, conflit 409) |
| FR-011 | 1 (création) + 5 (modification) |
| FR-012 | 2 |
| FR-013 | 3 (vérification code obligatoire) |
| FR-014 | 3 (court-circuit auteur) |
| FR-015 | 3 (404 générique, 403 message « Code incorrect ») |
| FR-016 | 1 (FK salle_id) |
| FR-017 | (côté admin : archivage en cascade quand `salle.actif=false`) |
| FR-018 | 4 (démarrage indépendant de session salle publique) |
| SC-005 | 1 (UNIQUE + 409) |
| SC-006 | 3 + 4 (≤ 2 s) |
| Rate limit (A3 / R4) | 3 (429 Too Many Requests) |
