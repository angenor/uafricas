# 09 — Gouvernance (FactCheck, Mauvaises pratiques, Idées forces)

> **Phase** : 3 — Fonctionnalités avancées
> **Section sidebar** : Gouvernance
> **Icône** : faScaleBalanced
> **Statut global** : [x] Terminé ✅

---

## Dépendances

### Fichiers SQL requis
- `schemas/10_governance.sql` → `factcheck`, `factcheck_commentaire`, `factcheck_reaction`, `bad_habit`, `bad_habit_media`, `idea_force`, `idea_force_media`
- `schemas/03_shared.sql` → `pays` (FK)
- `schemas/04_iam.sql` → `utilisateur` (FK created_by)
- `schemas/13_contraintes_inter_schemas.sql` → FK governance ↔ shared, iam
- **Enums** : `niveau_gravite` (faible/élevée/critique)

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** — Utilisateurs (créateurs, commentateurs)
- **`02-referentiels.md`** — Pays, Médiathèque (upload preuves/médias)

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** — Stats gouvernance (factchecks, signalements)

### Backend existant
- [x] `src/handlers/gouvernance.rs` — stats + contributions publiques — **À étendre pour admin**
- [x] Endpoints admin FactCheck, Bad habits, Idées forces — **CRÉÉ** (`src/handlers/admin/gouvernance.rs` + `src/models/admin/gouvernance.rs`)

---

## Sous-rubriques

### 1. FactCheck (`/admin/factcheck`)

#### Backend
- [x] `GET /api/admin/factcheck` — liste paginée + filtres (verdict, pays, recherche full-text)
- [x] `GET /api/admin/factcheck/:id` — détail (avec commentaires + réactions count)
- [x] `POST /api/admin/factcheck` — création
- [x] `PUT /api/admin/factcheck/:id` — modification + verdict (vrai/faux/partiellement_vrai/trompeur/non_vérifié)
- [x] `DELETE /api/admin/factcheck/:id` — soft delete
- [x] `GET /api/admin/factcheck/:id/commentaires` — liste commentaires (type: soutien/contradiction)
- [x] `DELETE /api/admin/factcheck/:id/commentaires/:commentaire_id` — modération (suppression)
- [x] `GET /api/admin/factcheck/:id/reactions` — stats réactions
- **Fichiers** : `src/handlers/admin/gouvernance.rs` (section factcheck)

#### Frontend
- [x] `app/pages/admin/factcheck/index.vue` — liste + filtres (verdict, pays)
- [x] `app/pages/admin/factcheck/create.vue` — formulaire (contenu, source_originale, verdict, couleur_fond)
- [x] `app/pages/admin/factcheck/[id].vue` — édition avec onglets :
  - [x] Onglet Contenu — contenu + source_originale + verdict (sélecteur) + couleur_fond
  - [x] Onglet Commentaires — modération (soutien/contradiction, suppression arborescente)
  - [x] Onglet Réactions — stats lecture seule (likes/dislikes + barre ratio)
- [x] `app/composables/useAdminFactcheck.ts`

---

### 2. Mauvaises pratiques (`/admin/bad-habits`)

#### Backend
- [x] `GET /api/admin/bad-habits` — liste paginée + filtres (catégorie, gravité, pays, date)
- [x] `GET /api/admin/bad-habits/:id` — détail (avec médias preuves)
- [x] `POST /api/admin/bad-habits` — création
- [x] `PUT /api/admin/bad-habits/:id` — modification
- [x] `DELETE /api/admin/bad-habits/:id` — soft delete
- [x] `POST /api/admin/bad-habits/:id/medias` — upload preuve (photo/vidéo)
- [x] `DELETE /api/admin/bad-habits/:id/medias/:media_id` — retirer preuve
- [x] `GET /api/admin/bad-habits/:id/medias` — lister médias
- **Fichiers** : `src/handlers/admin/gouvernance.rs` (section bad_habits)

#### Frontend
- [x] `app/pages/admin/bad-habits/index.vue` — liste + filtres (catégorie, gravité, pays)
- [x] `app/pages/admin/bad-habits/create.vue` — formulaire (titre, description, catégorie, gravité, pays, géolocalisation optionnelle, anonyme)
- [x] `app/pages/admin/bad-habits/[id].vue` — édition avec onglets :
  - [x] Onglet Infos — données principales + gravité
  - [x] Onglet Preuves — galerie photos/vidéos (ajout URL + suppression)
- [x] `app/composables/useAdminBadHabits.ts`

---

### 3. Idées forces (`/admin/idea-forces`)

#### Backend
- [x] `GET /api/admin/idea-forces` — liste paginée + filtres (catégorie, pays, urgence)
- [x] `GET /api/admin/idea-forces/:id` — détail (avec médias)
- [x] `POST /api/admin/idea-forces` — création
- [x] `PUT /api/admin/idea-forces/:id` — modification
- [x] `DELETE /api/admin/idea-forces/:id` — soft delete
- [x] `POST /api/admin/idea-forces/:id/medias` — upload média
- [x] `DELETE /api/admin/idea-forces/:id/medias/:media_id` — retirer média
- [x] `GET /api/admin/idea-forces/:id/medias` — lister médias
- **Fichiers** : `src/handlers/admin/gouvernance.rs` (section idea_forces)

#### Frontend
- [x] `app/pages/admin/idea-forces/index.vue` — liste + filtres (catégorie, pays)
- [x] `app/pages/admin/idea-forces/create.vue` — formulaire (titre, description, catégorie, pays, urgence, plan_implementation, ressources, impact)
- [x] `app/pages/admin/idea-forces/[id].vue` — édition avec onglets :
  - [x] Onglet Infos — données principales
  - [x] Onglet Médias — galerie (ajout URL + suppression)
- [x] `app/composables/useAdminIdeaForces.ts`

---

## Critères de validation
- [x] CRUD complet FactCheck avec verdicts
- [x] Modération commentaires FactCheck (soutien/contradiction)
- [x] CRUD mauvaises pratiques avec upload preuves
- [x] CRUD idées forces avec médias
- [x] Filtres par gravité, verdict, catégorie fonctionnels
- [x] Stats réactions en lecture seule

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### FactCheck
- [x] **T9.1** — Liste factcheck : header, filtres (recherche, verdict 5 options, état), tableau 7 colonnes OK ✅
- [x] **T9.2** — Formulaire factcheck : contenu textarea, source URL, verdict select, couleur fond, état OK ✅
- [ ] **T9.3** — Onglet Commentaires : vérifier distinction visuelle soutien (vert) vs contradiction (rouge) — nécessite données
- [ ] **T9.4** — Modération commentaire : supprimer un commentaire → modal confirmation → vérifier disparition — nécessite données
- [ ] **T9.5** — Onglet Réactions : vérifier stats likes/dislikes en lecture seule — nécessite données

### Mauvaises pratiques
- [x] **T9.6** — Liste bad-habits : header, 4 filtres, tableau 7 colonnes (Titre, Catégorie, Gravité, État, Soutiens, Anonyme, Création) OK ✅
- [ ] **T9.7** — Onglet Preuves : upload photos/vidéos comme preuves, vérifier previews, supprimer — nécessite données
- [x] **T9.8** — Formulaire : tous les champs OK, toggle géolocalisation fait apparaître longitude/latitude, toggle anonyme présent ✅

### Idées forces
- [x] **T9.9** — Liste idées forces : header, 4 filtres, tableau 6 colonnes + formulaire création complet OK ✅
- [ ] **T9.10** — Onglet Médias : upload + galerie + suppression — nécessite données

---

## Notes
- Le handler `gouvernance.rs` existant gère les stats et contributions publiques. L'admin ajoute le CRUD complet + la modération.
- Les verdicts FactCheck (vrai/faux/partiellement_vrai/trompeur/non_vérifié) sont un élément central de l'interface — utiliser des badges colorés distinctifs.
- Les mauvaises pratiques peuvent être anonymes → l'admin voit l'auteur mais l'interface publique ne l'affiche pas.
- Les niveaux de gravité (faible/élevée/critique) doivent être visuellement distincts (vert/orange/rouge).
- **Architecture** : Tous les endpoints gouvernance sont regroupés dans un seul fichier handler (`gouvernance.rs`) et un seul fichier models (`gouvernance.rs`) au lieu de fichiers séparés par entité.
- **Médias** : Chargés séparément via des endpoints dédiés (GET/POST/DELETE), pas inclus dans le détail principal.
