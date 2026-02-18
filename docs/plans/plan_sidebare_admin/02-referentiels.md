# 02 — Référentiels (Données de référence partagées)

> **Phase** : 1 — Données fondamentales
> **Section sidebar** : Référentiels
> **Icône** : faDatabase
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/03_shared.sql` → `pays`, `domaine_secteur`, `categorie`, `tag`, `media`
- `schemas/04_iam.sql` → `specialite_bibliotheque`, `utilisateur_specialite`
- `schemas/13_contraintes_inter_schemas.sql` → FK shared ↔ iam

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD génériques, middleware admin
- **`01-utilisateurs-acces.md`** — IAM nécessaire pour `created_by` et spécialités biblio

### Plans qui dépendent de celui-ci
- **`03-marche-africain.md`** → utilise `categorie`, `pays`, `media`, `tag`
- **`04-programmes-echange.md`** → utilise `pays`, `domaine_secteur`
- **`05-innovation.md`** → utilise `domaine_secteur`, `pays`
- **`06-culture.md`** → utilise `pays`, `tag`
- **`08-medias-contenus.md`** → utilise `pays`, `categorie`, `tag`
- **`09-gouvernance.md`** → utilise `pays`
- **`10-profils-pays.md`** → utilise `pays` (1-to-1)
- **`12-dashboard.md`** → stats sur toutes les entités de référence

---

## Sous-rubriques

### 1. Pays (`/admin/pays`)

#### Backend
- [ ] `GET /api/admin/pays` — liste paginée + filtres (continent/région, recherche)
- [ ] `GET /api/admin/pays/:id` — détail
- [ ] `POST /api/admin/pays` — création (code ISO, nom, capitale, coordonnées, drapeau)
- [ ] `PUT /api/admin/pays/:id` — modification
- [ ] `DELETE /api/admin/pays/:id` — soft delete (vérifier qu'aucune FK bloquante)
- **Fichiers** : `src/handlers/admin/pays.rs`, `src/models/admin/pays.rs`

#### Frontend
- [ ] `app/pages/admin/pays/index.vue` — liste avec drapeaux + recherche
- [ ] `app/pages/admin/pays/create.vue` — formulaire avec carte/coordonnées
- [ ] `app/pages/admin/pays/[id].vue` — édition
- [ ] `app/composables/useAdminPays.ts`

---

### 2. Domaines & Secteurs (`/admin/domaines`)

#### Backend
- [ ] `GET /api/admin/domaines` — liste paginée
- [ ] `GET /api/admin/domaines/:id` — détail
- [ ] `POST /api/admin/domaines` — création
- [ ] `PUT /api/admin/domaines/:id` — modification
- [ ] `DELETE /api/admin/domaines/:id` — soft delete
- **Fichiers** : `src/handlers/admin/domaines.rs`

#### Frontend
- [ ] `app/pages/admin/domaines/index.vue` — liste
- [ ] `app/pages/admin/domaines/create.vue` — formulaire
- [ ] `app/pages/admin/domaines/[id].vue` — édition
- [ ] `app/composables/useAdminDomaines.ts`

---

### 3. Catégories (`/admin/categories`)

#### Backend
- [ ] `GET /api/admin/categories` — liste arborescente (parent/enfant) + filtre par contexte
- [ ] `GET /api/admin/categories/:id` — détail avec enfants
- [ ] `POST /api/admin/categories` — création (avec parent_id optionnel)
- [ ] `PUT /api/admin/categories/:id` — modification (déplacer dans l'arbre)
- [ ] `DELETE /api/admin/categories/:id` — soft delete (vérifier enfants)
- **Fichiers** : `src/handlers/admin/categories.rs`

#### Frontend
- [ ] `app/pages/admin/categories/index.vue` — arborescence hiérarchique (tree view) + filtres par contexte
- [ ] `app/pages/admin/categories/create.vue` — formulaire avec sélecteur parent
- [ ] `app/pages/admin/categories/[id].vue` — édition + vue des enfants
- [ ] `app/composables/useAdminCategories.ts`
- [ ] `app/components/admin/AdminTreeView.vue` — composant arborescence réutilisable (si besoin)

---

### 4. Tags (`/admin/tags`)

#### Backend
- [ ] `GET /api/admin/tags` — liste paginée + recherche
- [ ] `GET /api/admin/tags/:id` — détail avec count d'utilisation
- [ ] `POST /api/admin/tags` — création
- [ ] `PUT /api/admin/tags/:id` — modification
- [ ] `DELETE /api/admin/tags/:id` — soft delete
- **Fichiers** : `src/handlers/admin/tags.rs`

#### Frontend
- [ ] `app/pages/admin/tags/index.vue` — liste avec badge count utilisation
- [ ] `app/pages/admin/tags/create.vue` — formulaire
- [ ] `app/pages/admin/tags/[id].vue` — édition
- [ ] `app/composables/useAdminTags.ts`

---

### 5. Médiathèque (`/admin/medias`)

#### Backend
- [ ] `GET /api/admin/medias` — liste paginée + filtres (type MIME, date, utilisateur)
- [ ] `GET /api/admin/medias/:id` — détail (métadonnées, preview)
- [ ] `POST /api/admin/medias` — upload (multipart, validation type/taille)
- [ ] `DELETE /api/admin/medias/:id` — suppression (fichier + entrée BDD)
- **Fichiers** : `src/handlers/admin/medias.rs`

#### Frontend
- [ ] `app/pages/admin/medias/index.vue` — galerie (grille/liste) + filtres par type MIME
- [ ] `app/pages/admin/medias/create.vue` — upload avec drag & drop, preview, barre de progression
- [ ] `app/composables/useAdminMedias.ts`
- [ ] `app/components/admin/AdminMediaUpload.vue` — composant upload réutilisable

---

### 6. Spécialités Biblio Humaine (`/admin/specialites`)

#### Backend
- [ ] `GET /api/admin/specialites` — liste paginée
- [ ] `GET /api/admin/specialites/:id` — détail + count utilisateurs associés
- [ ] `POST /api/admin/specialites` — création
- [ ] `PUT /api/admin/specialites/:id` — modification
- [ ] `DELETE /api/admin/specialites/:id` — soft delete
- **Fichiers** : `src/handlers/admin/specialites.rs`

#### Frontend
- [ ] `app/pages/admin/specialites/index.vue` — liste
- [ ] `app/pages/admin/specialites/create.vue` — formulaire
- [ ] `app/pages/admin/specialites/[id].vue` — édition
- [ ] `app/composables/useAdminSpecialites.ts`

---

## Critères de validation
- [ ] CRUD complet pour les 6 sous-rubriques
- [ ] Arborescence catégories fonctionnelle (parent/enfant)
- [ ] Upload média avec validation type MIME
- [ ] Galerie médias avec filtres
- [ ] Tous les référentiels sont utilisables comme sélecteurs dans les formulaires des autres plans

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Pays
- [ ] **T2.1** — Liste pays : vérifier l'affichage des drapeaux dans la DataTable, recherche par nom
- [ ] **T2.2** — Formulaire pays : vérifier les champs (code ISO, coordonnées, capitale), soumission

### Catégories
- [ ] **T2.3** — Arborescence catégories : vérifier le rendu tree view (expand/collapse niveaux parent/enfant)
- [ ] **T2.4** — Création catégorie : vérifier le sélecteur de parent (dropdown hiérarchique)
- [ ] **T2.5** — Filtre par contexte : changer le contexte (annonce, livre, radio...) et vérifier le filtrage de l'arbre

### Médiathèque
- [ ] **T2.6** — Galerie médias : vérifier le rendu en grille et en liste, les previews d'images
- [ ] **T2.7** — Upload drag & drop : glisser un fichier sur la zone d'upload, vérifier preview + barre de progression
- [ ] **T2.8** — Filtres type MIME : filtrer par image/vidéo/document, vérifier le résultat
- [ ] **T2.9** — Suppression média : cliquer supprimer → modal confirmation → vérifier disparition de la galerie

### CRUD standard (Domaines, Tags, Spécialités)
- [ ] **T2.10** — Parcours CRUD complet pour chaque sous-rubrique : créer, lister, éditer, supprimer

---

## Notes
- Les référentiels sont les données de base utilisées par toutes les rubriques métier. Ils doivent être implémentés tôt.
- La table `categorie` est multi-contexte (annonce, livre, radio, etc.) → le filtre par contexte est essentiel.
- Le composant `AdminMediaUpload` sera réutilisé dans les plans 03, 05, 08, 09, 10.
