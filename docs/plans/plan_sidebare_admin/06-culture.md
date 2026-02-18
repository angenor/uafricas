# 06 — Culture (Centres culturels, Programmations, Codi-Moi)

> **Phase** : 2 — Modules métier
> **Section sidebar** : Culture
> **Icône** : faMasksTheater
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/08_culture.sql` → `centre_culturel`, `programmation_centre`, `membre_centre`, `codimoi`, `codimoi_tag`, `codimoi_commentaire`, `codimoi_reaction`
- `schemas/03_shared.sql` → `pays`, `tag` (FK)
- `schemas/04_iam.sql` → `utilisateur` (FK created_by, membres)
- `schemas/13_contraintes_inter_schemas.sql` → FK culture ↔ shared, iam
- **Enums** : `mode_evenement`, `type_codimoi`, `role_membre_centre`

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** — Utilisateurs (créateurs, membres centres)
- **`02-referentiels.md`** — Pays, Tags

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** — Stats centres culturels, posts Codi-Moi

### Backend existant
- [x] `src/handlers/centres_culturels.rs` — lecture publique centres — **À étendre pour admin**
- [x] `src/handlers/codimoi.rs` — CRUD public posts + commentaires — **À étendre pour admin**
- [ ] Endpoints admin programmations — **À CRÉER**

---

## Sous-rubriques

### 1. Centres culturels (`/admin/centres-culturels`)

#### Backend
- [ ] `GET /api/admin/centres-culturels` — liste paginée + filtres (pays, recherche)
- [ ] `GET /api/admin/centres-culturels/:id` — détail (avec membres + programmations count)
- [ ] `POST /api/admin/centres-culturels` — création
- [ ] `PUT /api/admin/centres-culturels/:id` — modification
- [ ] `DELETE /api/admin/centres-culturels/:id` — soft delete
- [ ] `GET /api/admin/centres-culturels/:id/membres` — liste membres
- [ ] `POST /api/admin/centres-culturels/:id/membres` — ajouter membre (utilisateur + rôle)
- [ ] `PUT /api/admin/centres-culturels/:id/membres/:membre_id` — changer rôle membre
- [ ] `DELETE /api/admin/centres-culturels/:id/membres/:membre_id` — retirer membre
- **Fichiers** : `src/handlers/admin/centres_culturels.rs`

#### Frontend
- [ ] `app/pages/admin/centres-culturels/index.vue` — liste + filtres
- [ ] `app/pages/admin/centres-culturels/create.vue` — formulaire (nom, description, adresse, pays, contact)
- [ ] `app/pages/admin/centres-culturels/[id].vue` — édition avec onglets :
  - [ ] Onglet Infos — données du centre
  - [ ] Onglet Membres — gestion équipe (président, VP, communication, membre)
  - [ ] Onglet Programmations — liste des événements du centre
- [ ] `app/composables/useAdminCentresCulturels.ts`

---

### 2. Programmations (`/admin/programmations`)

#### Backend
- [ ] `GET /api/admin/programmations` — liste paginée + filtres (centre, mode, dates)
- [ ] `GET /api/admin/programmations/:id` — détail
- [ ] `POST /api/admin/programmations` — création (lié à un centre)
- [ ] `PUT /api/admin/programmations/:id` — modification
- [ ] `DELETE /api/admin/programmations/:id` — soft delete
- **Fichiers** : `src/handlers/admin/programmations.rs`

#### Frontend
- [ ] `app/pages/admin/programmations/index.vue` — liste + filtre par centre culturel
- [ ] `app/pages/admin/programmations/create.vue` — formulaire (centre, titre, description, dates, mode: présentiel/en_ligne/hybride)
- [ ] `app/pages/admin/programmations/[id].vue` — édition
- [ ] `app/composables/useAdminProgrammations.ts`

---

### 3. Codi-Moi (`/admin/codimoi`)

#### Backend
- [ ] `GET /api/admin/codimoi` — liste paginée + filtres (type_codimoi, pays, ethnie, recherche full-text)
- [ ] `GET /api/admin/codimoi/:id` — détail (avec tags + commentaires + réactions count)
- [ ] `POST /api/admin/codimoi` — création
- [ ] `PUT /api/admin/codimoi/:id` — modification
- [ ] `DELETE /api/admin/codimoi/:id` — soft delete
- [ ] `POST /api/admin/codimoi/:id/tags` — ajouter tag
- [ ] `DELETE /api/admin/codimoi/:id/tags/:tag_id` — retirer tag
- [ ] `GET /api/admin/codimoi/:id/commentaires` — liste commentaires (arborescente)
- [ ] `DELETE /api/admin/codimoi/:id/commentaires/:commentaire_id` — supprimer commentaire (modération)
- [ ] `GET /api/admin/codimoi/:id/reactions` — stats réactions (like/dislike count)
- **Fichiers** : `src/handlers/admin/codimoi.rs`

#### Frontend
- [ ] `app/pages/admin/codimoi/index.vue` — liste + filtres (type, pays, ethnie)
- [ ] `app/pages/admin/codimoi/create.vue` — formulaire (type, contenu, source, pays, ethnie)
- [ ] `app/pages/admin/codimoi/[id].vue` — édition avec onglets :
  - [ ] Onglet Contenu — données principales
  - [ ] Onglet Tags — gestion tags (autocomplete depuis référentiel)
  - [ ] Onglet Commentaires — modération (suppression, vue arborescente)
  - [ ] Onglet Réactions — stats lecture seule (likes/dislikes, graphique)
- [ ] `app/composables/useAdminCodimoi.ts`

---

## Critères de validation
- [ ] CRUD complet centres culturels avec gestion membres
- [ ] CRUD complet programmations avec filtre par centre
- [ ] CRUD complet Codi-Moi avec tags, commentaires, réactions
- [ ] Modération commentaires (suppression)
- [ ] Vue arborescente commentaires (threaded)
- [ ] Stats réactions en lecture seule

---

## Notes
- Les handlers `centres_culturels.rs` et `codimoi.rs` existent pour l'usage public. L'admin ajoute la gestion d'équipe (membres) et la modération des commentaires.
- Les commentaires Codi-Moi sont hiérarchiques (parent_id) → la vue admin doit les afficher en arborescence.
- Les réactions sont en lecture seule côté admin (pas de modification, juste des stats).
