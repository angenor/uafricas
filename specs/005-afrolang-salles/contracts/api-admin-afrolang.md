# Contract — Afrolang Admin API

**Feature**: 005-afrolang-salles
**Scope**: Endpoints administratifs montés sous `/api/admin/afrolang/**`. Auth JWT + rôle administrateur obligatoire. Toutes les mutations sont instrumentées avec `audit::log_action` (Principe VII).

Les endpoints existants (`/api/admin/afrolang/sessions/**`, `/api/admin/afrolang/salles-privees/{id}`) sont conservés. Cette section liste les **nouveaux endpoints** et **modifications** nécessaires à la feature.

---

## Gestion des propositions de salles publiques

### `GET /api/admin/afrolang/propositions`

File des propositions à traiter.

- **Query** : `etat?` (par défaut `en_attente`), `q?`, `pays_id?`, `page?`, `limit?`.
- **Response** : `data: { items: PropositionSalleAdminResponse[], total, page, limit }`.

### `GET /api/admin/afrolang/propositions/{id}`

Détail d'une proposition (avec proposant enrichi, conflits éventuels signalés).

### `POST /api/admin/afrolang/propositions/{id}/approuver`

Approuve la proposition :

- **Body** : `{ groupe_ethnique_id, titre?, image_couverture_url?, langue_code?, alphabet?, dictionnaire_url? }` (champs pour créer la salle associée ; `groupe_ethnique_id` obligatoire pour rattacher au référentiel unique).
- **Serveur** : crée `afrolang.salle` avec les métadonnées, met à jour la proposition (`etat='approuvee'`, `salle_id_creee`, `decide_par`, `decide_at`), insère une notification `afrolang.proposition_validee` au proposant, appelle `audit::log_action`.
- **Response** : `{ proposition: PropositionSalleAdminResponse, salle: SalleDetailResponse }`.

### `POST /api/admin/afrolang/propositions/{id}/refuser`

- **Body** : `{ motif_refus }` (NOT NULL, min 5 caractères).
- **Serveur** : `UPDATE etat='refusee'`, `motif_refus`, notification `afrolang.proposition_refusee`, audit.
- **Response** : `PropositionSalleAdminResponse`.

---

## Modérateurs Afrolang attitrés

### `GET /api/admin/afrolang/salles/{salle_id}/moderateurs`

Liste des modérateurs actifs d'une salle publique (`salle_moderateur.actif=TRUE`).

- **Response** : `ModerateurAttitreResponse[]`.

### `POST /api/admin/afrolang/salles/{salle_id}/moderateurs`

Désigne un modérateur Afrolang attitré (FR-008).

- **Body** : `{ utilisateur_id, disponibilite? }`.
- **Validation** : `utilisateur_id` existe et est actif ; pas déjà en `actif=TRUE` (sinon `409`).
- **Serveur** : insertion ou réactivation (si ligne existante avec `actif=FALSE`, `UPDATE actif=TRUE, retire_at=NULL`), audit.
- **Response** : `ModerateurAttitreResponse`.

### `DELETE /api/admin/afrolang/salles/{salle_id}/moderateurs/{utilisateur_id}`

Retire un modérateur (soft : `actif=FALSE`, `retire_at=NOW()`).

- **Response** : `ModerateurAttitreResponse`.

---

## Modération des ressources

### `GET /api/admin/afrolang/ressources/en-attente`

Liste des liens externes en `etat='en_attente_validation'` à modérer.

- **Query** : `salle_id?`, `page?`, `limit?`.
- **Response** : `RessourceSalleAdminResponse[]`.

### `POST /api/admin/afrolang/ressources/{id}/publier`

Approuve un lien externe : `etat='publiee'`, `valide_par`, `valide_at`. Audit.

### `POST /api/admin/afrolang/ressources/{id}/refuser`

- **Body** : `{ motif_refus }`.
- **Serveur** : `etat='refusee'`, notifie l'auteur, audit.

> Note : un modérateur Afrolang attitré d'une salle peut aussi publier / refuser un lien externe **de cette salle uniquement**, via les mêmes endpoints ; le contrôle d'accès serveur vérifie `salle_moderateur.actif=TRUE AND salle_id=<salle cible>` en plus du rôle admin global.

---

## Gestion des salles privées (admin)

### `GET /api/admin/afrolang/salles-privees` *(modifié)*

Ajout de filtres `motif?`, `visibilite?`, `archivee?` (true/false/tous).

### `POST /api/admin/afrolang/salles-privees/{id}/archiver`

Archivage manuel par l'admin (complément au déclenchement automatique lié à la suppression du créateur — FR-034).

- **Serveur** : `UPDATE archivee_at=NOW()` (si non déjà archivée), notifie les abonnés, audit.
- **Response** : `SallePriveeDetailResponse`.

### `GET /api/admin/afrolang/salles-privees/{id}/adhesions`

Audit en lecture complète des lignes d'adhésion d'une salle privée (demandes, invitations, abonnés, refus historiques).

---

## Audit de l'archivage automatique

### `POST /api/admin/afrolang/salles-privees/archiver-batch-utilisateur`

Handler interne (callable par le back-office admin suite à une désactivation utilisateur) qui archive toutes les salles privées actives dont le créateur est désactivé / supprimé (FR-034).

- **Body** : `{ utilisateur_id }`.
- **Serveur** : `UPDATE salle_privee SET archivee_at=NOW() WHERE cree_par=$1 AND archivee_at IS NULL AND deleted_at IS NULL` ; notifications participants ; audit par salle.
- **Response** : `{ archivees_count, items: SallePriveeDetailResponse[] }`.

> Note d'implémentation : peut aussi être déclenché en arrière-plan par le handler admin IAM `desactiver_utilisateur` (append à la fin du handler existant, via `tokio::spawn` pour rester non-bloquant — cohérent avec le pattern `audit::log_action`).

---

## Formats DTO admin (extraits)

```ts
interface PropositionSalleAdminResponse extends PropositionSalleResponse {
  proposant_nom_complet: string;
  proposant_email: string;
  salle_existante_id?: string;    // si un doublon a été détecté
  proposition_doublon_id?: string;
}

interface ModerateurAttitreResponse {
  id: string;
  salle_id: string;
  utilisateur_id: string;
  utilisateur_nom_complet: string;
  utilisateur_email: string;
  designe_par: string;
  designe_at: string;
  disponibilite: string | null;
  actif: boolean;
  retire_at: string | null;
}

interface RessourceSalleAdminResponse extends RessourceSalleResponse {
  salle_titre: string;
  ajoute_par_nom_complet: string;
}
```

---

## Audit (principe VII)

Actions instrumentées avec `audit::log_action` (non-bloquant, capture `before`/`after`) :

| Action | Table | Before | After |
|--------|-------|--------|-------|
| `proposition.approuver` | `proposition_salle` | ligne `en_attente` | ligne `approuvee` + `salle_id_creee` |
| `proposition.refuser` | `proposition_salle` | ligne `en_attente` | ligne `refusee` + `motif_refus` |
| `moderateur_attitre.designer` | `salle_moderateur` | N/A ou `actif=FALSE` | `actif=TRUE` |
| `moderateur_attitre.retirer` | `salle_moderateur` | `actif=TRUE` | `actif=FALSE` + `retire_at` |
| `ressource_lien.publier` | `ressource_salle` | `en_attente_validation` | `publiee` + `valide_par`/`valide_at` |
| `ressource_lien.refuser` | `ressource_salle` | `en_attente_validation` | `refusee` + `motif_refus` |
| `salle_privee.archiver_manuel` | `salle_privee` | `archivee_at=NULL` | `archivee_at=NOW()` |
| `salle_privee.archiver_batch_utilisateur` | `salle_privee` (n lignes) | idem | idem |

---

## Permissions

- `role=admin` : tous les endpoints ci-dessus.
- `role=moderateur_afrolang` (rôle IAM existant ou à créer au besoin) : `POST /ressources/{id}/publier` et `POST /ressources/{id}/refuser` **uniquement pour les salles où ils sont attitrés** (`salle_moderateur.actif=TRUE`).
- Autres rôles : 403.

> Si le rôle `moderateur_afrolang` n'existe pas encore dans `iam.role`, il sera créé lors de l'implémentation (migration SQL `INSERT INTO iam.role ...`) et les permissions associées listées dans `iam.permission`.
