# 01 — Utilisateurs & Accès (IAM)

> **Phase** : 1 — Données fondamentales
> **Section sidebar** : Utilisateurs & Accès
> **Icône** : faUsers
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/03_shared.sql` → `shared.pays` (pays d'origine/résidence utilisateur)
- `schemas/04_iam.sql` → `utilisateur`, `role`, `permission`, `role_permission`, `utilisateur_role`, `permission_specifique`, `organisation`, `partenariat`, `specialite_bibliotheque`, `utilisateur_specialite`, `refresh_token`
- `schemas/04b_iam_expertise.sql` → `expertise` (profils experts)
- `schemas/04c_iam_verification_email.sql` → `token_verification_email`
- `schemas/13_contraintes_inter_schemas.sql` → FK iam ↔ shared

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Middleware admin, composants CRUD génériques, composable useAdmin

### Plans qui dépendent de celui-ci
- **Tous les plans 02 à 12** — chaque entité a un `created_by` → `iam.utilisateur`
- **`02-referentiels.md`** — les spécialités biblio humaine sont dans IAM

---

## Sous-rubriques

### 1. Utilisateurs (`/admin/utilisateurs`)

#### Backend
- [ ] `GET /api/admin/utilisateurs` — liste paginée avec filtres (état, rôle, pays, recherche full-text)
- [ ] `GET /api/admin/utilisateurs/:id` — détail complet (profil + rôles + permissions + spécialités)
- [ ] `POST /api/admin/utilisateurs` — création (avec assignation rôle optionnelle)
- [ ] `PUT /api/admin/utilisateurs/:id` — modification profil
- [ ] `PATCH /api/admin/utilisateurs/:id/etat` — changement d'état (actif/suspendu/bloqué)
- [ ] `DELETE /api/admin/utilisateurs/:id` — soft delete
- [ ] `POST /api/admin/utilisateurs/:id/roles` — assigner un rôle
- [ ] `DELETE /api/admin/utilisateurs/:id/roles/:role_id` — retirer un rôle
- [ ] `POST /api/admin/utilisateurs/:id/specialites` — assigner spécialité biblio humaine
- [ ] `DELETE /api/admin/utilisateurs/:id/specialites/:spec_id` — retirer spécialité
- [ ] `POST /api/admin/utilisateurs/:id/permissions` — ajouter permission spécifique
- [ ] `DELETE /api/admin/utilisateurs/:id/permissions/:perm_id` — retirer permission spécifique
- **Fichiers** : `src/handlers/admin/utilisateurs.rs`, `src/models/admin/utilisateurs.rs`

#### Frontend
- [ ] `app/pages/admin/utilisateurs/index.vue` — liste avec DataTable + filtres (état, rôle, pays)
- [ ] `app/pages/admin/utilisateurs/create.vue` — formulaire création
- [ ] `app/pages/admin/utilisateurs/[id].vue` — édition avec onglets :
  - [ ] Onglet Profil — infos personnelles
  - [ ] Onglet Rôles — assignation/retrait de rôles (table)
  - [ ] Onglet Spécialités — biblio humaine (si applicable)
  - [ ] Onglet Permissions spécifiques — permissions granulaires par ressource
- [ ] `app/composables/useAdminUtilisateurs.ts` — API client CRUD + filtres

---

### 2. Organisations (`/admin/organisations`)

#### Backend
- [ ] `GET /api/admin/organisations` — liste paginée + filtres (type, pays, recherche)
- [ ] `GET /api/admin/organisations/:id` — détail
- [ ] `POST /api/admin/organisations` — création
- [ ] `PUT /api/admin/organisations/:id` — modification
- [ ] `DELETE /api/admin/organisations/:id` — soft delete
- **Fichiers** : `src/handlers/admin/organisations.rs`

#### Frontend
- [ ] `app/pages/admin/organisations/index.vue` — liste
- [ ] `app/pages/admin/organisations/create.vue` — formulaire
- [ ] `app/pages/admin/organisations/[id].vue` — édition
- [ ] `app/composables/useAdminOrganisations.ts`

---

### 3. Partenariats (`/admin/partenariats`)

#### Backend
- [ ] `GET /api/admin/partenariats` — liste paginée + filtres (type, organisation)
- [ ] `GET /api/admin/partenariats/:id` — détail
- [ ] `POST /api/admin/partenariats` — création
- [ ] `PUT /api/admin/partenariats/:id` — modification
- [ ] `DELETE /api/admin/partenariats/:id` — soft delete
- **Fichiers** : `src/handlers/admin/partenariats.rs`

#### Frontend
- [ ] `app/pages/admin/partenariats/index.vue` — liste
- [ ] `app/pages/admin/partenariats/create.vue` — formulaire
- [ ] `app/pages/admin/partenariats/[id].vue` — édition
- [ ] `app/composables/useAdminPartenariats.ts`

---

### 4. Rôles & Permissions (`/admin/roles`)

#### Backend
- [ ] `GET /api/admin/roles` — liste des rôles avec nombre d'utilisateurs
- [ ] `GET /api/admin/roles/:id` — détail avec permissions assignées
- [ ] `POST /api/admin/roles` — création rôle
- [ ] `PUT /api/admin/roles/:id` — modification rôle
- [ ] `DELETE /api/admin/roles/:id` — suppression (si aucun utilisateur assigné)
- [ ] `POST /api/admin/roles/:id/permissions` — assigner permission(s) au rôle
- [ ] `DELETE /api/admin/roles/:id/permissions/:perm_id` — retirer permission
- [ ] `GET /api/admin/permissions` — liste complète des permissions disponibles
- **Fichiers** : `src/handlers/admin/roles.rs`

#### Frontend
- [ ] `app/pages/admin/roles/index.vue` — liste des rôles + count utilisateurs
- [ ] `app/pages/admin/roles/create.vue` — formulaire avec matrice de permissions
- [ ] `app/pages/admin/roles/[id].vue` — édition rôle + assignation permissions (checkbox matrix)
- [ ] `app/composables/useAdminRoles.ts`

---

## Critères de validation
- [ ] CRUD complet pour utilisateurs, organisations, partenariats, rôles
- [ ] Assignation/retrait de rôles fonctionnel
- [ ] Matrice de permissions fonctionnelle
- [ ] Filtres full-text sur utilisateurs
- [ ] Changement d'état utilisateur (actif/suspendu/bloqué) opérationnel
- [ ] Soft delete fonctionnel

---

## Notes
- Les handlers backend `auth.rs` existants gèrent l'inscription/connexion côté public. Les endpoints admin sont séparés et requièrent le rôle admin.
- Le store `user.ts` existant a déjà le getter `isAdmin`. Les composables admin l'utilisent pour la vérification côté client.
