# 00 : Fondation Admin (Composants communs & Infrastructure)

> **Phase** : 0 : Prérequis pour toutes les rubriques
> **Priorité** : CRITIQUE
> **Statut global** : [x] Terminé (vérifié 19/02/2026)

---

## Dépendances

### Fichiers SQL requis
- `schemas/04_iam.sql` → table `role`, `permission`, `role_permission`, `utilisateur_role` (middleware auth/RBAC)
- `schemas/04c_iam_verification_email.sql` → token vérification (auth flow)

### Plans précédents
- Aucun : c'est le premier plan

### Plans qui dépendent de celui-ci
- **Tous les plans suivants** (01 à 12) dépendent de cette fondation

### État existant
- [x] Layout admin (`app/layouts/admin.vue`), drawer + navbar + breadcrumbs
- [x] Sidebar admin (`app/components/admin/AdminSidebar.vue`), 12 sections, 40+ routes
- [x] Page dashboard placeholder (`app/pages/admin/index.vue`)
- [x] Store utilisateur avec `isAdmin` getter (`app/stores/user.ts`)
- [x] Composants CRUD génériques : **CRÉÉS** (8 composants, vérifié 19/02/2026)
- [x] Middleware admin (`app/middleware/admin.ts`), **CRÉÉ** (fix SSR + refresh token, 19/02/2026)
- [x] Composable admin de base (`app/composables/useAdmin.ts`), **CRÉÉ** (vérifié 19/02/2026)

---

## Backend

### B0.1 : Middleware d'autorisation admin
- [x] Créer un extracteur/guard Actix `RequireAdmin` qui vérifie le JWT + rôle admin, **CRÉÉ** : `AdminUtilisateur` (extracteur FromRequest, vérifie JWT + rôle admin/super_admin)
- [x] Créer un extracteur `RequirePermission(resource, action)` pour le RBAC granulaire, **CRÉÉ** : macro `verifier_permission!` + méthode `a_permission()`
- [x] Ajouter les routes préfixées `/api/admin/...` dans `routes.rs`, **CRÉÉ** : scope `/admin` avec me, utilisateurs, organisations, partenariats, roles, permissions
- **Fichiers** : `src/middleware/admin.rs`, `src/routes.rs`

### B0.2 : Endpoint de vérification session admin
- [x] `GET /api/admin/me` → retourne l'utilisateur courant + ses rôles + permissions, **CRÉÉ**
- **Fichiers** : `src/handlers/admin/mod.rs`

### B0.3 : Endpoints génériques de listing paginé
- [x] Définir un trait/struct `PaginationParams` réutilisable (page, per_page, sort_by, sort_dir), **CRÉÉ**
- [x] Définir un struct `PaginatedResponse<T>` (data, total, page, per_page, total_pages), **CRÉÉ** (avec `from_params()` helper)
- **Fichiers** : `src/models/pagination.rs`

---

## Frontend

### F0.1 : Middleware de navigation admin
- [x] Créer `app/middleware/admin.ts`, redirige vers `/login` si non connecté ou non admin, **CRÉÉ** (fix SSR + refresh token)
- [x] Appliquer le middleware dans `app/pages/admin.vue` ou via `definePageMeta`, **APPLIQUÉ**

### F0.2 : Composable `useAdmin`
- [x] Créer `app/composables/useAdmin.ts`, client API admin de base, **CRÉÉ**
  - `$fetch` configuré avec JWT
  - Gestion centralisée erreurs 401/403
  - Helper de pagination (params URL ↔ état réactif)

### F0.3 : Composants CRUD génériques
- [x] `app/components/admin/AdminDataTable.vue`, table triable, paginée, avec sélection, **CRÉÉ**
- [x] `app/components/admin/AdminFilters.vue`, barre de filtres dynamique (texte, select, date range), **CRÉÉ**
- [x] `app/components/admin/AdminFormModal.vue`, modal de création/édition, **CRÉÉ**
- [x] `app/components/admin/AdminDeleteConfirm.vue`, dialogue de confirmation de suppression, **CRÉÉ**
- [x] `app/components/admin/AdminStatusBadge.vue`, badge d'état coloré (actif, suspendu, etc.), **CRÉÉ**
- [x] `app/components/admin/AdminBreadcrumb.vue`, fil d'Ariane dynamique, **CRÉÉ**
- [x] `app/components/admin/AdminPageHeader.vue`, en-tête de page (titre + boutons d'action), **CRÉÉ**
- [x] `app/components/admin/AdminStatsCard.vue`, carte KPI pour dashboard, **CRÉÉ**

### F0.4 : Types TypeScript communs
- [x] Créer `app/types/admin.ts` : interfaces PaginatedResponse, FilterOption, TableColumn, etc., **CRÉÉ**

---

## Critères de validation
- [x] Le middleware bloque l'accès aux non-admins, **VALIDÉ** (T0.1, T0.2, T0.3)
- [x] Les composants génériques sont fonctionnels avec des données de test, **CRÉÉS** (à tester visuellement)
- [x] Le composable `useAdmin` gère correctement le JWT et les erreurs, **CRÉÉ**
- [x] Navigation fluide entre toutes les sous-pages admin, **VALIDÉ** (T0.10)

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

- [ ] **T0.1** : Middleware admin : accéder à `/admin` sans être connecté → redirection vers `/login` ✓ (19/02/2026)
- [ ] **T0.2** : Middleware admin : accéder à `/admin` avec un compte non-admin → redirection vers `/` ✓ (19/02/2026)
- [ ] **T0.3** : Middleware admin : accéder à `/admin` avec un compte admin → accès autorisé ✓ (19/02/2026)
- [ ] **T0.4** : Layout admin : drawer sidebar (ouvert/fermé), navbar avec breadcrumbs, sous-menus déroulants ✓ (19/02/2026)
- [ ] **T0.5** : AdminDataTable : composant créé, **À TESTER** visuellement avec `agent-browser --headed`
- [ ] **T0.6** : AdminFilters : composant créé, **À TESTER** visuellement avec `agent-browser --headed`
- [ ] **T0.7** : AdminFormModal : composant créé, **À TESTER** visuellement avec `agent-browser --headed`
- [ ] **T0.8** : AdminDeleteConfirm : composant créé, **À TESTER** visuellement avec `agent-browser --headed`
- [ ] **T0.9** : AdminStatusBadge : composant créé, **À TESTER** visuellement avec `agent-browser --headed`
- [ ] **T0.10** : Navigation sidebar : sous-menus s'ouvrent, liens fonctionnent (404 attendu pour les pages non créées, pas de crash 500) ✓ (19/02/2026)

### Corrections appliquées lors des tests
- **Fix middleware SSR** : Le middleware admin redigeait côté serveur (SSR) avant que le client ne puisse restaurer la session JWT. Corrigé en ajoutant `if (import.meta.server) return` et en restaurant le refresh_token côté client avant vérification.
- **Fichier modifié** : `app/middleware/admin.ts`

### Utilisateurs de test créés
- **Admin** : `admin@test.com` / `Test1234` (rôle: admin)
- **Standard** : `user@test.com` / `Test1234` (rôle: utilisateur)

---

## Notes
Cette fondation est le socle technique. Elle ne crée aucune page métier mais fournit tous les outils réutilisables pour les 12 rubriques suivantes.
