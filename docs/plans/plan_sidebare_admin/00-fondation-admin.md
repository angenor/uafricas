# 00 — Fondation Admin (Composants communs & Infrastructure)

> **Phase** : 0 — Prérequis pour toutes les rubriques
> **Priorité** : CRITIQUE
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/04_iam.sql` → table `role`, `permission`, `role_permission`, `utilisateur_role` (middleware auth/RBAC)
- `schemas/04c_iam_verification_email.sql` → token vérification (auth flow)

### Plans précédents
- Aucun — c'est le premier plan

### Plans qui dépendent de celui-ci
- **Tous les plans suivants** (01 à 12) dépendent de cette fondation

### État existant
- [x] Layout admin (`app/layouts/admin.vue`) — drawer + navbar + breadcrumbs
- [x] Sidebar admin (`app/components/admin/AdminSidebar.vue`) — 12 sections, 40+ routes
- [x] Page dashboard placeholder (`app/pages/admin/index.vue`)
- [x] Store utilisateur avec `isAdmin` getter (`app/stores/user.ts`)
- [ ] Composants CRUD génériques — **À CRÉER**
- [ ] Middleware admin — **À CRÉER**
- [ ] Composable admin de base — **À CRÉER**

---

## Backend

### B0.1 — Middleware d'autorisation admin
- [ ] Créer un extracteur/guard Actix `RequireAdmin` qui vérifie le JWT + rôle admin
- [ ] Créer un extracteur `RequirePermission(resource, action)` pour le RBAC granulaire
- [ ] Ajouter les routes préfixées `/api/admin/...` dans `routes.rs`
- **Fichiers** : `src/middleware/admin.rs`, `src/routes.rs`

### B0.2 — Endpoint de vérification session admin
- [ ] `GET /api/admin/me` → retourne l'utilisateur courant + ses rôles + permissions
- **Fichiers** : `src/handlers/admin/mod.rs`

### B0.3 — Endpoints génériques de listing paginé
- [ ] Définir un trait/struct `PaginationParams` réutilisable (page, per_page, sort_by, sort_dir)
- [ ] Définir un struct `PaginatedResponse<T>` (data, total, page, per_page, total_pages)
- **Fichiers** : `src/models/pagination.rs`

---

## Frontend

### F0.1 — Middleware de navigation admin
- [ ] Créer `app/middleware/admin.ts` — redirige vers `/login` si non connecté ou non admin
- [ ] Appliquer le middleware dans `app/pages/admin.vue` ou via `definePageMeta`

### F0.2 — Composable `useAdmin`
- [ ] Créer `app/composables/useAdmin.ts` — client API admin de base
  - `$fetch` configuré avec JWT
  - Gestion centralisée erreurs 401/403
  - Helper de pagination (params URL ↔ état réactif)

### F0.3 — Composants CRUD génériques
- [ ] `app/components/admin/AdminDataTable.vue` — table triable, paginée, avec sélection
- [ ] `app/components/admin/AdminFilters.vue` — barre de filtres dynamique (texte, select, date range)
- [ ] `app/components/admin/AdminFormModal.vue` — modal de création/édition
- [ ] `app/components/admin/AdminDeleteConfirm.vue` — dialogue de confirmation de suppression
- [ ] `app/components/admin/AdminStatusBadge.vue` — badge d'état coloré (actif, suspendu, etc.)
- [ ] `app/components/admin/AdminBreadcrumb.vue` — fil d'Ariane dynamique
- [ ] `app/components/admin/AdminPageHeader.vue` — en-tête de page (titre + boutons d'action)
- [ ] `app/components/admin/AdminStatsCard.vue` — carte KPI pour dashboard

### F0.4 — Types TypeScript communs
- [ ] Créer `app/types/admin.ts` — interfaces PaginatedResponse, FilterOption, TableColumn, etc.

---

## Critères de validation
- [ ] Le middleware bloque l'accès aux non-admins
- [ ] Les composants génériques sont fonctionnels avec des données de test
- [ ] Le composable `useAdmin` gère correctement le JWT et les erreurs
- [ ] Navigation fluide entre toutes les sous-pages admin

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

- [ ] **T0.1** — Middleware admin : accéder à `/admin` sans être connecté → vérifier redirection vers `/login`
- [ ] **T0.2** — Middleware admin : accéder à `/admin` avec un compte non-admin → vérifier redirection vers `/`
- [ ] **T0.3** — Middleware admin : accéder à `/admin` avec un compte admin → accès autorisé
- [ ] **T0.4** — Layout admin : vérifier le rendu du drawer sidebar (ouvert/fermé), navbar, breadcrumbs
- [ ] **T0.5** — AdminDataTable : vérifier le rendu avec données de test (tri colonnes, pagination, skeleton loading)
- [ ] **T0.6** — AdminFilters : vérifier le rendu des filtres (texte, select, date range), reset
- [ ] **T0.7** — AdminFormModal : vérifier ouverture/fermeture modale, soumission formulaire
- [ ] **T0.8** — AdminDeleteConfirm : vérifier dialogue de confirmation, annulation, validation
- [ ] **T0.9** — AdminStatusBadge : vérifier les couleurs par état (actif=vert, suspendu=orange, bloqué=rouge)
- [ ] **T0.10** — Navigation : cliquer sur chaque lien du sidebar → vérifier que la page se charge sans erreur

---

## Notes
Cette fondation est le socle technique. Elle ne crée aucune page métier mais fournit tous les outils réutilisables pour les 12 rubriques suivantes.
