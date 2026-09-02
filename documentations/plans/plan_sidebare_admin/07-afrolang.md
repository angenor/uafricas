# 07 : AfroLang (Visioconférence linguistique)

> **Phase** : 3 : Fonctionnalités avancées
> **Section sidebar** : AfroLang
> **Icône** : faVideo
> **Statut global** : [x] Terminé

---

## Dépendances

### Fichiers SQL requis
- `schemas/08b_afrolang.sql` → `salle`, `salle_privee`, `session`, `session_participant`, `tableau_blanc`
- `schemas/04_iam.sql` → `utilisateur` (FK modérateur, créateur, participants)
- `schemas/13_contraintes_inter_schemas.sql` → FK afrolang ↔ iam
- **Pas d'enums SQL dédiés** : états gérés par colonnes texte (planifiee/en_cours/terminee/annulee)

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** : Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** : Utilisateurs (modérateurs, participants)

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** : Stats sessions (en cours, terminées, participants)

### Backend existant
- [x] `src/handlers/afrolang.rs` : 21 endpoints complets (salles, sessions, tokens, whiteboard), **Très avancé, adapter pour admin**
- [x] Endpoints admin de supervision : **CRÉÉS**

---

## Sous-rubriques

### 1. Salles publiques (`/admin/salles`)

#### Backend
- [x] `GET /api/admin/salles` : liste paginée + filtres (langue, état, recherche)
- [x] `GET /api/admin/salles/:id` : détail (avec sessions count, modérateur)
- [x] `POST /api/admin/salles` : création (nom, description, langue, modérateur désigné)
- [x] `PUT /api/admin/salles/:id` : modification (changer modérateur, description)
- [x] `DELETE /api/admin/salles/:id` : soft delete (actif = false)
- **Fichiers** : `src/handlers/admin/salles.rs`, `src/models/admin/salle.rs`

#### Frontend
- [x] `app/pages/admin/salles/index.vue`, liste + filtres
- [x] `app/pages/admin/salles/create.vue`, formulaire (nom, langue, modérateur via UUID)
- [x] `app/pages/admin/salles/[id].vue`, édition + onglet sessions
- [x] `app/composables/useAdminSalles.ts`

---

### 2. Salles privées (`/admin/salles-privees`)

#### Backend
- [x] `GET /api/admin/salles-privees`, liste paginée (supervision, lecture seule)
- [x] `GET /api/admin/salles-privees/:id`, détail (créateur, participants max, sessions)
- **Fichiers** : `src/handlers/admin/salles_privees.rs`, `src/models/admin/salle_privee.rs`

#### Frontend
- [x] `app/pages/admin/salles-privees/index.vue`, supervision lecture seule (DataTable)
- [x] `app/pages/admin/salles-privees/[id].vue`, détail lecture seule avec onglets Infos + Sessions
- [x] `app/composables/useAdminSallesPrivees.ts`

---

### 3. Sessions (`/admin/sessions`)

#### Backend
- [x] `GET /api/admin/sessions` : historique paginé + filtres (état, salle, date range, modérateur)
- [x] `GET /api/admin/sessions/:id` : détail (participants + tableau blanc snapshot)
- [x] `GET /api/admin/sessions/:id/participants`, liste participants (rôle, durée, timestamps)
- [x] `GET /api/admin/sessions/:id/tableau-blanc`, snapshot JSONB du tableau blanc
- **Fichiers** : `src/handlers/admin/sessions_afrolang.rs`, `src/models/admin/session_afrolang.rs`

#### Frontend
- [x] `app/pages/admin/sessions/index.vue`, historique + filtres (état, salle, date)
- [x] `app/pages/admin/sessions/[id].vue`, détail avec onglets :
  - [x] Onglet Infos : état, dates, modérateur, pic de participants
  - [x] Onglet Participants : liste (rôle, durée de participation)
  - [x] Onglet Tableau blanc : affichage JSON du snapshot JSONB (lecture seule)
- [x] `app/composables/useAdminSessions.ts`

---

## Critères de validation
- [x] CRUD complet salles publiques avec assignation modérateur
- [x] Supervision salles privées en lecture seule
- [x] Historique sessions avec filtres avancés
- [x] Détail session avec participants et snapshot tableau blanc
- [x] Affichage du tableau blanc JSONB en lecture seule (JSON pré-formaté)

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Salles publiques
- [x] **T7.1** : CRUD salles : créer "Salle Haoussa Test" (Haoussa), éditer → "Salle Haoussa Modifiee", list affiche 8 salles
- [x] **T7.2** : Filtres : recherche "Swahili" filtre correctement à 2 résultats

### Salles privées
- [x] **T7.3** : Supervision lecture seule : pas de bouton créer/éditer/supprimer, seulement icône œil (vue)
- [x] **T7.4** : Détail salle privée : affichage créateur, code accès, max participants, onglets Infos + Sessions

### Sessions
- [x] **T7.5** : Historique sessions : 3 sessions affichées, filtres état (dropdown Tous/Planifiee/En cours/Terminee/Annulee) + recherche
- [x] **T7.6** : Onglet Participants : 3 participants avec rôles (Moderateur/Participant), durées formatées
- [x] **T7.7** : Onglet Tableau blanc : affiche "Aucun tableau blanc pour cette session" (pas de données whiteboard)

### Sidebar
- [x] **T7.8** : Section AfroLang dans sidebar avec 3 sous-items : Salles publiques, Salles privées, Sessions

---

## Bugs corrigés pendant l'implémentation
1. **`deleted_at` inexistant** : Les tables afrolang n'ont pas de colonne `deleted_at`. Soft delete via `actif = false`. Corrigé dans les 6 fichiers backend.
2. **`#[serde(flatten)]` incompatible avec `serde_urlencoded`** : Les QueryParams utilisaient `#[serde(flatten)]` avec `PaginationParams`, causant une erreur de désérialisation (`invalid type: string "1", expected i64`). Corrigé en inlinant les champs de pagination directement dans les structs.

---

## Notes
- Le handler `afrolang.rs` existant est déjà très complet (21 endpoints). L'admin ajoute une vue de supervision globale plutôt que de dupliquer la logique.
- Le tableau blanc utilise du JSONB (strokes, shapes, text) → le rendu admin est en lecture seule (pas d'édition temps réel).
- L'intégration LiveKit (SFU) est gérée côté public. L'admin ne gère que les métadonnées et l'historique.
- **Permission** : resource `"afrolang"`, actions `"voir"`, `"modifier"`, `"supprimer"`.
- **Tables sans `deleted_at`** : Utilise `actif BOOLEAN` pour le soft delete sur `salle` et `salle_privee`. Pas de soft delete sur `session`.
