# 09 — Gouvernance (FactCheck, Mauvaises pratiques, Idées forces)

> **Phase** : 3 — Fonctionnalités avancées
> **Section sidebar** : Gouvernance
> **Icône** : faScaleBalanced
> **Statut global** : [ ] Non démarré

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
- [ ] Endpoints admin FactCheck, Bad habits, Idées forces — **À CRÉER**

---

## Sous-rubriques

### 1. FactCheck (`/admin/factcheck`)

#### Backend
- [ ] `GET /api/admin/factcheck` — liste paginée + filtres (verdict, pays, recherche full-text)
- [ ] `GET /api/admin/factcheck/:id` — détail (avec commentaires + réactions count)
- [ ] `POST /api/admin/factcheck` — création
- [ ] `PUT /api/admin/factcheck/:id` — modification + verdict (vrai/faux/partiellement_vrai/trompeur/non_vérifié)
- [ ] `DELETE /api/admin/factcheck/:id` — soft delete
- [ ] `GET /api/admin/factcheck/:id/commentaires` — liste commentaires (type: soutien/contradiction)
- [ ] `DELETE /api/admin/factcheck/:id/commentaires/:commentaire_id` — modération (suppression)
- [ ] `GET /api/admin/factcheck/:id/reactions` — stats réactions
- **Fichiers** : `src/handlers/admin/factcheck.rs`

#### Frontend
- [ ] `app/pages/admin/factcheck/index.vue` — liste + filtres (verdict, pays)
- [ ] `app/pages/admin/factcheck/create.vue` — formulaire (affirmation, source, analyse, verdict)
- [ ] `app/pages/admin/factcheck/[id].vue` — édition avec onglets :
  - [ ] Onglet Contenu — affirmation + analyse + verdict (sélecteur)
  - [ ] Onglet Commentaires — modération (soutien/contradiction, suppression)
  - [ ] Onglet Réactions — stats lecture seule (likes/dislikes)
- [ ] `app/composables/useAdminFactcheck.ts`

---

### 2. Mauvaises pratiques (`/admin/bad-habits`)

#### Backend
- [ ] `GET /api/admin/bad-habits` — liste paginée + filtres (catégorie, gravité, pays, date)
- [ ] `GET /api/admin/bad-habits/:id` — détail (avec médias preuves)
- [ ] `POST /api/admin/bad-habits` — création
- [ ] `PUT /api/admin/bad-habits/:id` — modification
- [ ] `DELETE /api/admin/bad-habits/:id` — soft delete
- [ ] `POST /api/admin/bad-habits/:id/medias` — upload preuve (photo/vidéo)
- [ ] `DELETE /api/admin/bad-habits/:id/medias/:media_id` — retirer preuve
- **Fichiers** : `src/handlers/admin/bad_habits.rs`

#### Frontend
- [ ] `app/pages/admin/bad-habits/index.vue` — liste + filtres (catégorie, gravité, pays)
- [ ] `app/pages/admin/bad-habits/create.vue` — formulaire (titre, description, catégorie, gravité, pays, géolocalisation optionnelle, anonyme?)
- [ ] `app/pages/admin/bad-habits/[id].vue` — édition avec onglets :
  - [ ] Onglet Infos — données principales + gravité
  - [ ] Onglet Preuves — galerie photos/vidéos (upload + suppression)
- [ ] `app/composables/useAdminBadHabits.ts`

---

### 3. Idées forces (`/admin/idea-forces`)

#### Backend
- [ ] `GET /api/admin/idea-forces` — liste paginée + filtres (catégorie, pays, urgence)
- [ ] `GET /api/admin/idea-forces/:id` — détail (avec médias)
- [ ] `POST /api/admin/idea-forces` — création
- [ ] `PUT /api/admin/idea-forces/:id` — modification
- [ ] `DELETE /api/admin/idea-forces/:id` — soft delete
- [ ] `POST /api/admin/idea-forces/:id/medias` — upload média
- [ ] `DELETE /api/admin/idea-forces/:id/medias/:media_id` — retirer média
- **Fichiers** : `src/handlers/admin/idea_forces.rs`

#### Frontend
- [ ] `app/pages/admin/idea-forces/index.vue` — liste + filtres (catégorie, pays)
- [ ] `app/pages/admin/idea-forces/create.vue` — formulaire (titre, description, catégorie, pays, niveau urgence)
- [ ] `app/pages/admin/idea-forces/[id].vue` — édition avec onglets :
  - [ ] Onglet Infos — données principales
  - [ ] Onglet Médias — galerie (réutilise AdminMediaUpload)
- [ ] `app/composables/useAdminIdeaForces.ts`

---

## Critères de validation
- [ ] CRUD complet FactCheck avec verdicts
- [ ] Modération commentaires FactCheck (soutien/contradiction)
- [ ] CRUD mauvaises pratiques avec upload preuves
- [ ] CRUD idées forces avec médias
- [ ] Filtres par gravité, verdict, catégorie fonctionnels
- [ ] Stats réactions en lecture seule

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### FactCheck
- [ ] **T9.1** — Liste factcheck : vérifier les badges verdict colorés (vrai=vert, faux=rouge, partiellement_vrai=orange, trompeur=jaune, non_vérifié=gris)
- [ ] **T9.2** — Formulaire factcheck : vérifier le sélecteur de verdict, champs source et analyse
- [ ] **T9.3** — Onglet Commentaires : vérifier distinction visuelle soutien (vert) vs contradiction (rouge)
- [ ] **T9.4** — Modération commentaire : supprimer un commentaire → modal confirmation → vérifier disparition
- [ ] **T9.5** — Onglet Réactions : vérifier stats likes/dislikes en lecture seule

### Mauvaises pratiques
- [ ] **T9.6** — Badges gravité : vérifier couleurs distinctives (faible=vert, élevée=orange, critique=rouge)
- [ ] **T9.7** — Onglet Preuves : upload photos/vidéos comme preuves, vérifier previews, supprimer
- [ ] **T9.8** — Formulaire : vérifier champ géolocalisation optionnel + toggle anonyme

### Idées forces
- [ ] **T9.9** — CRUD complet idées forces : créer, lister, éditer, supprimer
- [ ] **T9.10** — Onglet Médias : upload + galerie + suppression

---

## Notes
- Le handler `gouvernance.rs` existant gère les stats et contributions publiques. L'admin ajoute le CRUD complet + la modération.
- Les verdicts FactCheck (vrai/faux/partiellement_vrai/trompeur/non_vérifié) sont un élément central de l'interface — utiliser des badges colorés distinctifs.
- Les mauvaises pratiques peuvent être anonymes → l'admin voit l'auteur mais l'interface publique ne l'affiche pas.
- Les niveaux de gravité (faible/élevée/critique) doivent être visuellement distincts (vert/orange/rouge).
