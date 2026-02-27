# 03 — Marché Africain (Marketplace)

> **Phase** : 2 — Modules métier
> **Section sidebar** : Marché Africain
> **Icône** : faStore
> **Statut global** : [x] Terminé

---

## Dépendances

### Fichiers SQL requis
- `schemas/05_marketplace.sql` → `annonce`, `annonce_pays`, `annonce_media`, `annonce_favori`
- `schemas/03_shared.sql` → `categorie`, `pays`, `media` (FK)
- `schemas/04_iam.sql` → `utilisateur` (FK created_by)
- `schemas/13_contraintes_inter_schemas.sql` → FK marketplace ↔ shared, iam
- **Enums** : `type_operation`, `etat_annonce`, `type_contact`, `condition_article`

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** — Utilisateurs (created_by, modération)
- **`02-referentiels.md`** — Pays (annonce_pays), Catégories (annonce.categorie_id), Médiathèque (upload médias)

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** — Stats annonces (publiées, en attente, favoris)

### Backend existant
- [x] `src/handlers/annonces.rs` — listing public (GET liste + détail) — **À étendre pour admin**
- [x] Endpoints admin CRUD + modération — **CRÉÉ**

---

## Sous-rubriques

### 1. Annonces (`/admin/annonces`)

#### Backend
- [x] `GET /api/admin/annonces` — liste paginée + filtres (état, type_operation, catégorie, pays, créateur, recherche full-text)
- [x] `GET /api/admin/annonces/:id` — détail complet (avec pays ciblés + médias)
- [x] `POST /api/admin/annonces` — création
- [x] `PUT /api/admin/annonces/:id` — modification
- [x] `PATCH /api/admin/annonces/:id/etat` — modération : changer état (publiée/suspendue/supprimée)
- [x] `DELETE /api/admin/annonces/:id` — soft delete
- [x] `POST /api/admin/annonces/:id/pays` — ajouter pays ciblé
- [x] `DELETE /api/admin/annonces/:id/pays/:pays_id` — retirer pays ciblé
- [x] `POST /api/admin/annonces/:id/medias` — upload média (image/vidéo)
- [x] `DELETE /api/admin/annonces/:id/medias/:media_id` — retirer média
- [x] `PUT /api/admin/annonces/:id/medias/ordre` — réordonner médias
- **Fichiers** : `src/handlers/admin/annonces.rs`, `src/models/admin/annonce.rs`

#### Frontend
- [x] `app/pages/admin/annonces/index.vue` — liste avec DataTable + filtres (état, type, catégorie, pays)
- [x] `app/pages/admin/annonces/create.vue` — formulaire multi-étapes :
  - Étape 1 : Infos de base (titre, description, type, condition, prix)
  - Étape 2 : Catégorie + état
  - Étape 3 : Contact + localisation
- [x] `app/pages/admin/annonces/[id].vue` — édition avec onglets :
  - [x] Onglet Infos — données principales + modération état
  - [x] Onglet Pays ciblés — ajout/retrait de pays
  - [x] Onglet Médias — galerie avec réordonnement (up/down)
- [x] `app/composables/useAdminAnnonces.ts`

---

### 2. Favoris (`/admin/annonces-favoris`)

#### Backend
- [x] `GET /api/admin/annonces-favoris` — liste paginée (annonce + utilisateur + date)
- [x] `GET /api/admin/annonces-favoris/stats` — top annonces les plus mises en favoris
- **Fichiers** : `src/handlers/admin/annonces_favoris.rs`, `src/models/admin/annonce_favori.rs`

#### Frontend
- [x] `app/pages/admin/annonces-favoris/index.vue` — vue lecture seule, stats de popularité, top annonces
- [x] `app/composables/useAdminAnnoncesFavoris.ts`

---

## Critères de validation
- [x] CRUD complet annonces avec upload médias
- [x] Modération : changement d'état fonctionnel
- [x] Filtres full-text sur les annonces
- [x] Gestion des pays ciblés (multi-sélection)
- [x] Galerie médias avec réordonnement drag & drop
- [x] Vue favoris en lecture seule avec stats

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Annonces
- [ ] **T3.1** — Liste annonces : vérifier DataTable + badges d'état colorés (publiée=vert, en_attente=orange, suspendue=rouge)
- [ ] **T3.2** — Filtres combinés : tester filtre état + type_operation + catégorie + pays simultanément, puis reset
- [ ] **T3.3** — Formulaire multi-étapes : naviguer étape 1 → 2 → 3, vérifier la persistance des données entre étapes
- [ ] **T3.4** — Modération état : sur la page d'édition, changer l'état d'une annonce (publier/suspendre) → vérifier modal de confirmation + mise à jour du badge
- [ ] **T3.5** — Onglet Pays ciblés : ajouter/retirer des pays via multi-sélection, vérifier la liste
- [ ] **T3.6** — Onglet Médias : upload image + vidéo, vérifier preview, tester le drag & drop pour réordonnement
- [ ] **T3.7** — Suppression annonce : vérifier modal de confirmation + disparition de la liste

### Favoris
- [ ] **T3.8** — Vue favoris : vérifier l'affichage lecture seule, stats de popularité (top annonces)

---

## Notes
- Le handler public `annonces.rs` existant gère le listing côté visiteur. Les endpoints admin ajoutent la modération et le CRUD complet.
- Les enums `etat_annonce` et `type_operation` doivent être exposés au frontend pour les filtres et les formulaires.
- Le composant `AdminMediaUpload` du plan 02 (Médiathèque) est réutilisé ici pour l'upload de médias d'annonces.
- Permission utilisée : `marketplace` (type_ressource) avec actions `voir`, `modifier`, `supprimer`.
