# 07 — AfroLang (Visioconférence linguistique)

> **Phase** : 3 — Fonctionnalités avancées
> **Section sidebar** : AfroLang
> **Icône** : faVideo
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/08b_afrolang.sql` → `salle`, `salle_privee`, `session`, `session_participant`, `tableau_blanc`
- `schemas/04_iam.sql` → `utilisateur` (FK modérateur, créateur, participants)
- `schemas/13_contraintes_inter_schemas.sql` → FK afrolang ↔ iam
- **Pas d'enums SQL dédiés** — états gérés par colonnes texte (planifiee/en_cours/terminee/annulee)

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** — Utilisateurs (modérateurs, participants)

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** — Stats sessions (en cours, terminées, participants)

### Backend existant
- [x] `src/handlers/afrolang.rs` — 21 endpoints complets (salles, sessions, tokens, whiteboard) — **Très avancé, adapter pour admin**
- [ ] Endpoints admin de supervision — **À CRÉER si besoin**

---

## Sous-rubriques

### 1. Salles publiques (`/admin/salles`)

#### Backend
- [ ] `GET /api/admin/salles` — liste paginée + filtres (langue, état, recherche)
- [ ] `GET /api/admin/salles/:id` — détail (avec sessions count, modérateur)
- [ ] `POST /api/admin/salles` — création (nom, description, langue, modérateur désigné)
- [ ] `PUT /api/admin/salles/:id` — modification (changer modérateur, description)
- [ ] `DELETE /api/admin/salles/:id` — soft delete
- **Fichiers** : `src/handlers/admin/salles.rs`

#### Frontend
- [ ] `app/pages/admin/salles/index.vue` — liste + filtres
- [ ] `app/pages/admin/salles/create.vue` — formulaire (nom, langue, modérateur via sélecteur utilisateur)
- [ ] `app/pages/admin/salles/[id].vue` — édition + historique sessions
- [ ] `app/composables/useAdminSalles.ts`

---

### 2. Salles privées (`/admin/salles-privees`)

#### Backend
- [ ] `GET /api/admin/salles-privees` — liste paginée (supervision, lecture seule)
- [ ] `GET /api/admin/salles-privees/:id` — détail (créateur, participants max, sessions)
- **Fichiers** : `src/handlers/admin/salles_privees.rs`

#### Frontend
- [ ] `app/pages/admin/salles-privees/index.vue` — supervision lecture seule (DataTable)
- [ ] `app/pages/admin/salles-privees/[id].vue` — détail lecture seule
- [ ] `app/composables/useAdminSallesPrivees.ts`

---

### 3. Sessions (`/admin/sessions`)

#### Backend
- [ ] `GET /api/admin/sessions` — historique paginé + filtres (état, salle, date range, modérateur)
- [ ] `GET /api/admin/sessions/:id` — détail (participants + tableau blanc snapshot)
- [ ] `GET /api/admin/sessions/:id/participants` — liste participants (rôle, durée, timestamps)
- [ ] `GET /api/admin/sessions/:id/tableau-blanc` — snapshot JSONB du tableau blanc
- **Fichiers** : `src/handlers/admin/sessions.rs`

#### Frontend
- [ ] `app/pages/admin/sessions/index.vue` — historique + filtres (état, salle, date)
- [ ] `app/pages/admin/sessions/[id].vue` — détail avec onglets :
  - [ ] Onglet Infos — état, dates, modérateur, pic de participants
  - [ ] Onglet Participants — liste (rôle, durée de participation)
  - [ ] Onglet Tableau blanc — rendu visuel du snapshot JSONB (lecture seule)
- [ ] `app/composables/useAdminSessions.ts`

---

## Critères de validation
- [ ] CRUD complet salles publiques avec assignation modérateur
- [ ] Supervision salles privées en lecture seule
- [ ] Historique sessions avec filtres avancés
- [ ] Détail session avec participants et snapshot tableau blanc
- [ ] Rendu visuel du tableau blanc JSONB (canvas ou SVG)

---

## Notes
- Le handler `afrolang.rs` existant est déjà très complet (21 endpoints). L'admin ajoute une vue de supervision globale plutôt que de dupliquer la logique.
- Le tableau blanc utilise du JSONB (strokes, shapes, text) → le rendu admin est en lecture seule (pas d'édition temps réel).
- L'intégration LiveKit (SFU) est gérée côté public. L'admin ne gère que les métadonnées et l'historique.
