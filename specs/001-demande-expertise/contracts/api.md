# Contrats d'API : Demande pour devenir expert

**Feature**: 001-demande-expertise | **Date**: 2026-05-24

Enveloppe standard du projet : `{ "success": bool, "data": T | null, "error": string | null }`.
Auth : `Authorization: Bearer <access_token>`. Endpoints admin : extracteur `AdminUtilisateur` + `verifier_permission!`.

---

## Endpoints publics / membre (`/api/experts`, `/api/auth`)

### POST /api/experts/candidature  *(existant — modifié)*

Soumettre (ou re-soumettre après refus) une candidature d'expertise. JWT requis.

**Body**
```json
{
  "domaine": "Informatique",
  "biographie": "…",
  "nb_annees_experience": 8,
  "portfolio": "https://… (optionnel)",
  "situations_professionnelles": ["consultance", "volontariat_expertise"]
}
```

**Comportement (modifié)** :
- 409 si une demande active `en_attente` **ou** `valide` existe (`deleted_at IS NULL`).
- Si une demande active `refuse` existe : la soft-deleter puis insérer une nouvelle ligne `en_attente` (même transaction).
- Sinon : insérer une nouvelle ligne `en_attente`.

**Réponses**
- `201 Created` → `data`: la candidature créée (statut `en_attente`).
- `401` non authentifié ; `404` utilisateur introuvable ; `409` demande active déjà existante ; `422` validation.

---

### GET /api/experts/moi  *(nouveau)*

Récupérer la candidature **active** du membre connecté (pour le suivi US3). JWT requis.

**Réponses**
- `200 OK` → `data`:
```json
{
  "id": "uuid",
  "domaine": "Informatique",
  "biographie": "…",
  "nb_annees_experience": 8,
  "portfolio": "…",
  "situations_professionnelles": ["consultance"],
  "statut": "en_attente | valide | refuse",
  "commentaire_admin": "… (si refuse, sinon null)",
  "date_validation": "2026-05-24T… | null",
  "created_at": "2026-05-20T…"
}
```
- `200 OK` avec `data: null` si aucune candidature active.
- `401` non authentifié.

---

### PUT /api/auth/profil  *(existant — étendu)*

Étendu pour accepter `pays_residence_id` (en plus de `fonction`, `ville`, etc.).

**Body (champs pertinents)**
```json
{ "fonction": "Ingénieure logiciel", "pays_residence_id": "uuid-pays" }
```
- Validation : `pays_residence_id` doit référencer un `shared.pays` existant.
- `200 OK` → profil mis à jour ; `422` si pays invalide.

### POST /api/auth/profil/photo  *(existant — inchangé)*

Upload de la photo de profil (multipart). Utilisé par le formulaire.

---

## Endpoints admin (`/api/admin/experts`)

> Permission : `expertise.voir` (lecture), `expertise.valider` (décisions). Audit sur les mutations.

### GET /api/admin/experts

Liste paginée et filtrable des demandes d'expertise.

**Query params** : `statut` (`en_attente|valide|refuse`), `recherche` (nom/prénom/biographie/domaine), `page` (défaut 1), `par_page` (défaut 20, max 100).

**Réponse `200 OK`**
```json
{
  "items": [
    {
      "id": "uuid",
      "utilisateur_id": "uuid",
      "nom": "Diop", "prenom": "Awa", "email": "awa@…", "photo_url": "/uploads/…",
      "domaine": "Informatique",
      "nb_annees_experience": 8,
      "pays_nom": "Sénégal",
      "statut": "en_attente",
      "commentaire_admin": null,
      "valide_par_nom": null,
      "date_validation": null,
      "created_at": "2026-05-20T…"
    }
  ],
  "total": 42, "page": 1, "par_page": 20
}
```
- `401/403` si non admin / permission manquante.

### GET /api/admin/experts/{id}

Détail complet d'une demande (candidat + expertise proposée + décision éventuelle).

- `200 OK` → DTO détail (mêmes champs + `biographie`, `portfolio`, `situations_professionnelles`).
- `404` introuvable.

### PATCH /api/admin/experts/{id}/valider

Valider une demande `en_attente`.

**Comportement** (transaction) : `statut='valide'`, `valide_par=admin.id`, `date_validation=NOW()` ; puis hors transaction : email d'approbation au candidat (async) + `audit::log_action("VALIDATE", "iam", "expertise", id)`.

**Réponses**
- `200 OK` → demande mise à jour ; l'expert devient visible sur `/experts`.
- `404` introuvable ; `409` si déjà traitée (`statut != 'en_attente'`).

### PATCH /api/admin/experts/{id}/rejeter

Refuser une demande `en_attente` avec commentaire obligatoire.

**Body**
```json
{ "commentaire_admin": "Dossier incomplet : merci de préciser votre expérience." }
```
- Validation : `commentaire_admin` non vide (≥ 10 caractères recommandé).

**Comportement** (transaction) : `statut='refuse'`, `valide_par=admin.id`, `date_validation=NOW()`, `commentaire_admin=…` ; puis hors transaction : email de refus (avec commentaire, async) + `audit::log_action("REJECT", "iam", "expertise", id)`.

**Réponses**
- `200 OK` → demande mise à jour.
- `404` introuvable ; `409` si déjà traitée ; `422` commentaire manquant.

---

## Emails (canal de notification — `email.rs`)

| Déclencheur | Fonction | Contenu |
|-------------|----------|---------|
| Validation | `envoyer_email_expertise_validee` (async) | Félicitations + lien vers sa fiche `/experts/{id}` |
| Refus | `envoyer_email_expertise_refusee` (async) | Motif (`commentaire_admin`) + invitation à re-soumettre |

Envoi « fire-and-forget » (ne bloque pas la réponse HTTP), pattern identique à `envoyer_verification_async`.

---

## Tableau de couverture des exigences

| Exigence | Endpoint(s) |
|----------|-------------|
| FR-001/FR-002 (lien → formulaire, auth) | Frontend `/devenir-expert` + redirection login |
| FR-003 / FR-003a (champs expertise + profil) | `POST /api/experts/candidature` + `PUT /api/auth/profil` + `POST /api/auth/profil/photo` |
| FR-004 (validation) | Validation backend sur tous les endpoints |
| FR-005/FR-006 (statut initial, anti-doublon) | `POST /api/experts/candidature` |
| FR-007 (confirmation) | Réponse `201` + UI |
| FR-008/FR-009 (liste, détail admin) | `GET /api/admin/experts`, `GET /api/admin/experts/{id}` |
| FR-010 (valider) | `PATCH /api/admin/experts/{id}/valider` |
| FR-011 (refuser + commentaire) | `PATCH /api/admin/experts/{id}/rejeter` |
| FR-012 (notification email) | Emails validée/refusée |
| FR-013 (visibilité publique) | `lister_experts` (filtre existant) |
| FR-014 (suivi candidat) | `GET /api/experts/moi` |
| FR-015 (re-soumission) | `POST /api/experts/candidature` (logique soft-delete) |
| FR-016 (idempotence décision) | `409` si `statut != 'en_attente'` |
| FR-017 (audit) | `audit::log_action` sur valider/rejeter |
