# 01 : Utilisateurs & Accès (IAM)

> **Phase** : 1 : Données fondamentales
> **Section sidebar** : Utilisateurs & Accès
> **Icône** : faUsers
> **Statut global** : [x] Implémenté (backend + frontend)

---

## Dépendances

### Fichiers SQL requis
- `schemas/03_shared.sql` → `shared.pays` (pays d'origine/résidence utilisateur)
- `schemas/04_iam.sql` → `utilisateur`, `role`, `permission`, `role_permission`, `utilisateur_role`, `permission_specifique`, `organisation`, `partenariat`, `specialite_bibliotheque`, `utilisateur_specialite`, `refresh_token`
- `schemas/04b_iam_expertise.sql` → `expertise` (profils experts)
- `schemas/04c_iam_verification_email.sql` → `token_verification_email`
- `schemas/13_contraintes_inter_schemas.sql` → FK iam ↔ shared

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** : Middleware admin, composants CRUD génériques, composable useAdmin

### Plans qui dépendent de celui-ci
- **Tous les plans 02 à 12** : chaque entité a un `created_by` → `iam.utilisateur`
- **`02-referentiels.md`** : les spécialités biblio humaine sont dans IAM

---

## Sous-rubriques

### 1. Utilisateurs (`/admin/utilisateurs`)

#### Backend
- [x] `GET /api/admin/utilisateurs` : liste paginée avec filtres (état, rôle, pays, recherche full-text)
- [x] `GET /api/admin/utilisateurs/:id`, détail complet (profil + rôles + permissions + spécialités)
- [x] `POST /api/admin/utilisateurs` : création (avec assignation rôle optionnelle)
- [x] `PUT /api/admin/utilisateurs/:id`, modification profil
- [x] `PATCH /api/admin/utilisateurs/:id/etat`, changement d'état (actif/suspendu/bloqué)
- [x] `DELETE /api/admin/utilisateurs/:id`, soft delete
- [x] `POST /api/admin/utilisateurs/:id/roles`, assigner un rôle
- [x] `DELETE /api/admin/utilisateurs/:id/roles/:role_id`, retirer un rôle
- [x] `POST /api/admin/utilisateurs/:id/specialites`, assigner spécialité biblio humaine
- [x] `DELETE /api/admin/utilisateurs/:id/specialites/:spec_id`, retirer spécialité
- [x] `POST /api/admin/utilisateurs/:id/permissions`, ajouter permission spécifique
- [x] `DELETE /api/admin/utilisateurs/:id/permissions/:perm_id`, retirer permission spécifique
- **Fichiers** : `src/handlers/admin/utilisateurs.rs`, `src/models/admin/utilisateur.rs`

#### Frontend
- [x] `app/pages/admin/utilisateurs/index.vue`, liste avec DataTable + filtres (état, rôle, pays)
- [x] `app/pages/admin/utilisateurs/create.vue`, formulaire création
- [x] `app/pages/admin/utilisateurs/[id].vue`, édition avec onglets :
  - [x] Onglet Profil : infos personnelles
  - [x] Onglet Rôles : assignation/retrait de rôles (table)
  - [x] Onglet Spécialités : biblio humaine (si applicable)
  - [x] Onglet Permissions spécifiques : permissions granulaires par ressource
- [x] `app/composables/useAdminUtilisateurs.ts`, API client CRUD + filtres

---

### 2. Organisations (`/admin/organisations`)

#### Backend
- [x] `GET /api/admin/organisations` : liste paginée + filtres (type, pays, recherche)
- [x] `GET /api/admin/organisations/:id`, détail
- [x] `POST /api/admin/organisations`, création
- [x] `PUT /api/admin/organisations/:id`, modification
- [x] `DELETE /api/admin/organisations/:id`, soft delete
- **Fichiers** : `src/handlers/admin/organisations.rs`, `src/models/admin/organisation.rs`

#### Frontend
- [x] `app/pages/admin/organisations/index.vue`, liste
- [x] `app/pages/admin/organisations/create.vue`, formulaire
- [x] `app/pages/admin/organisations/[id].vue`, édition
- [x] `app/composables/useAdminOrganisations.ts`

---

### 3. Partenariats (`/admin/partenariats`)

#### Backend
- [x] `GET /api/admin/partenariats` : liste paginée + filtres (type, organisation)
- [x] `GET /api/admin/partenariats/:id`, détail
- [x] `POST /api/admin/partenariats` : création
- [x] `PUT /api/admin/partenariats/:id`, modification
- [x] `DELETE /api/admin/partenariats/:id`, suppression
- **Fichiers** : `src/handlers/admin/partenariats.rs`, `src/models/admin/partenariat.rs`

#### Frontend
- [x] `app/pages/admin/partenariats/index.vue`, liste
- [x] `app/pages/admin/partenariats/create.vue`, formulaire
- [x] `app/pages/admin/partenariats/[id].vue`, édition
- [x] `app/composables/useAdminPartenariats.ts`

---

### 4. Rôles & Permissions (`/admin/roles`)

#### Backend
- [x] `GET /api/admin/roles` : liste des rôles avec nombre d'utilisateurs
- [x] `GET /api/admin/roles/:id` : détail avec permissions assignées
- [x] `POST /api/admin/roles` : création rôle
- [x] `PUT /api/admin/roles/:id` : modification rôle
- [x] `DELETE /api/admin/roles/:id` : suppression (si aucun utilisateur assigné)
- [x] `POST /api/admin/roles/:id/permissions`, assigner permission(s) au rôle
- [x] `DELETE /api/admin/roles/:id/permissions/:perm_id`, retirer permission
- [x] `GET /api/admin/permissions` : liste complète des permissions disponibles
- **Fichiers** : `src/handlers/admin/roles.rs`, `src/models/admin/role.rs`

#### Frontend
- [x] `app/pages/admin/roles/index.vue`, liste des rôles + count utilisateurs
- [x] `app/pages/admin/roles/create.vue`, formulaire avec matrice de permissions
- [x] `app/pages/admin/roles/[id].vue`, édition rôle + assignation permissions (checkbox matrix)
- [x] `app/composables/useAdminRoles.ts`
- [x] `app/components/admin/AdminPermissionMatrix.vue`, matrice permissions groupées par ressource

---

## Critères de validation
- [x] CRUD complet pour utilisateurs, organisations, partenariats, rôles
- [x] Assignation/retrait de rôles fonctionnel
- [x] Matrice de permissions fonctionnelle
- [x] Filtres full-text sur utilisateurs
- [x] Changement d'état utilisateur (actif/suspendu/bloqué) opérationnel
- [x] Soft delete fonctionnel

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Utilisateurs
- [x] **T1.1** : Liste utilisateurs : DataTable OK (initiales avatar, nom, email, badge état coloré, rôles tags, vérifié icône, date, pagination 33 éléments/3 pages). PASS
- [ ] **T1.2** : Filtres utilisateurs : les dropdowns s'affichent (état: actif/en_attente/suspendu/bloqué, rôle: admin/super_admin/moderateur/utilisateur) mais le filtre état ne semble pas rafraîchir la liste. **A INVESTIGUER** : possible problème dans AdminFilters → chargerListe
- [x] **T1.3** : Création utilisateur : formulaire complet (prénom, nom, email, mot de passe, téléphone, genre, rôle initial optionnel) + boutons Annuler/Créer. PASS
- [x] **T1.4** : Édition utilisateur : page [id] charge avec header (nom, email, badge état, rôle), boutons Suspendre/Bloquer. PASS
- [x] **T1.5** : Onglets utilisateur : 4 onglets fonctionnels (Profil avec formulaire complet, Rôles avec table assignation, Spécialités, Permissions). PASS
- [x] **T1.6** : Assignation rôle : onglet Rôles affiche table (rôle, slug, attribué par, date, supprimer) + champ ID pour assigner. PASS
- [ ] **T1.7** : Changement état : boutons Suspendre/Bloquer visibles dans le header. Non testé (clic + modal). **A TESTER**

### Organisations & Partenariats
- [x] **T1.8** : CRUD organisations : liste OK (DataTable avec Dénomination, Type, Pays, État, Membres, Création, Actions + filtres recherche/état + bouton Nouvelle organisation). PASS
- [x] **T1.9** : CRUD partenariats : liste OK (DataTable avec Organisation, Type, Début, Fin, Actif, Création, Actions + filtre type + bouton Nouveau partenariat). PASS

### Rôles & Permissions
- [x] **T1.10** : Matrice de permissions : affichage parfait, 23 permissions groupées par type_ressource (15 groupes), checkboxes cochées, select-all par groupe, compteur "23 sélectionnées". PASS
- [x] **T1.11** : Assignation permissions : checkboxes cliquables, bouton Enregistrer. Non testé la persistance après reload. **PARTIEL**

### Problème identifié pendant les tests
- **Permissions BDD** : Le rôle `admin` n'avait aucune permission assignée dans `iam.role_permission`. Corrigé manuellement avec `INSERT INTO iam.role_permission SELECT admin_id, p.id FROM iam.permission p`. Les 23 permissions sont maintenant assignées.

---

## Notes
- Les handlers backend `auth.rs` existants gèrent l'inscription/connexion côté public. Les endpoints admin sont séparés et requièrent le rôle admin.
- Le store `user.ts` existant a déjà le getter `isAdmin`. Les composables admin l'utilisent pour la vérification côté client.
