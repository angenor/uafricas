# 06 — Culture (Centres culturels, Programmations, Codi-Moi)

> **Phase** : 2 — Modules métier
> **Section sidebar** : Culture
> **Icône** : faMasksTheater
> **Statut global** : [x] Terminé

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
- [x] Endpoints admin programmations — **CRÉÉ**

---

## Sous-rubriques

### 1. Centres culturels (`/admin/centres-culturels`)

#### Backend
- [x] `GET /api/admin/centres-culturels` — liste paginée + filtres (pays, recherche)
- [x] `GET /api/admin/centres-culturels/:id` — détail (avec membres + programmations count)
- [x] `POST /api/admin/centres-culturels` — création
- [x] `PUT /api/admin/centres-culturels/:id` — modification
- [x] `DELETE /api/admin/centres-culturels/:id` — soft delete
- [x] `GET /api/admin/centres-culturels/:id/membres` — liste membres
- [x] `POST /api/admin/centres-culturels/:id/membres` — ajouter membre (utilisateur + rôle)
- [x] `PUT /api/admin/centres-culturels/:id/membres/:membre_id` — changer rôle membre
- [x] `DELETE /api/admin/centres-culturels/:id/membres/:membre_id` — retirer membre
- **Fichiers** : `src/handlers/admin/centres_culturels.rs`

#### Frontend
- [x] `app/pages/admin/centres-culturels/index.vue` — liste + filtres
- [x] `app/pages/admin/centres-culturels/create.vue` — formulaire (nom, description, adresse, pays, contact)
- [x] `app/pages/admin/centres-culturels/[id].vue` — édition avec onglets :
  - [x] Onglet Infos — données du centre
  - [x] Onglet Membres — gestion équipe (président, VP, communication, membre)
  - [x] Onglet Programmations — liste des événements du centre
- [x] `app/composables/useAdminCentresCulturels.ts`

---

### 2. Programmations (`/admin/programmations`)

#### Backend
- [x] `GET /api/admin/programmations` — liste paginée + filtres (centre, mode, dates)
- [x] `GET /api/admin/programmations/:id` — détail
- [x] `POST /api/admin/programmations` — création (lié à un centre)
- [x] `PUT /api/admin/programmations/:id` — modification
- [x] `DELETE /api/admin/programmations/:id` — soft delete
- **Fichiers** : `src/handlers/admin/programmations.rs`

#### Frontend
- [x] `app/pages/admin/programmations/index.vue` — liste + filtre par centre culturel
- [x] `app/pages/admin/programmations/create.vue` — formulaire (centre, titre, description, dates, mode: présentiel/en_ligne/hybride)
- [x] `app/pages/admin/programmations/[id].vue` — édition
- [x] `app/composables/useAdminProgrammations.ts`

---

### 3. Codi-Moi (`/admin/codimoi`)

#### Backend
- [x] `GET /api/admin/codimoi` — liste paginée + filtres (type_codimoi, pays, ethnie, recherche full-text)
- [x] `GET /api/admin/codimoi/:id` — détail (avec tags + commentaires + réactions count)
- [x] `POST /api/admin/codimoi` — création
- [x] `PUT /api/admin/codimoi/:id` — modification
- [x] `DELETE /api/admin/codimoi/:id` — soft delete
- [x] `POST /api/admin/codimoi/:id/tags` — ajouter tag
- [x] `DELETE /api/admin/codimoi/:id/tags/:tag_id` — retirer tag
- [x] `GET /api/admin/codimoi/:id/commentaires` — liste commentaires (arborescente)
- [x] `DELETE /api/admin/codimoi/:id/commentaires/:commentaire_id` — supprimer commentaire (modération)
- [x] `GET /api/admin/codimoi/:id/reactions` — stats réactions (like/dislike count)
- **Fichiers** : `src/handlers/admin/codimoi.rs`

#### Frontend
- [x] `app/pages/admin/codimoi/index.vue` — liste + filtres (type, pays, ethnie)
- [x] `app/pages/admin/codimoi/create.vue` — formulaire (type, contenu, source, pays, ethnie)
- [x] `app/pages/admin/codimoi/[id].vue` — édition avec onglets :
  - [x] Onglet Contenu — données principales
  - [x] Onglet Tags — gestion tags (autocomplete depuis référentiel)
  - [x] Onglet Commentaires — modération (suppression, vue arborescente)
  - [x] Onglet Réactions — stats lecture seule (likes/dislikes, graphique)
- [x] `app/composables/useAdminCodimoi.ts`

---

## Critères de validation
- [x] CRUD complet centres culturels avec gestion membres
- [x] CRUD complet programmations avec filtre par centre
- [x] CRUD complet Codi-Moi avec tags, commentaires, réactions
- [x] Modération commentaires (suppression)
- [x] Vue arborescente commentaires (threaded)
- [x] Stats réactions en lecture seule

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Centres culturels
- [ ] **T6.1** — Liste centres culturels : vérifier DataTable + filtres pays
- [ ] **T6.2** — Onglet Membres : ajouter un membre (sélecteur utilisateur + rôle), changer rôle, retirer → vérifier la table
- [ ] **T6.3** — Onglet Programmations : vérifier la liste des programmations du centre (lien vers édition)

### Programmations
- [ ] **T6.4** — Formulaire programmation : vérifier le sélecteur de centre culturel, champs dates, mode (présentiel/en_ligne/hybride)
- [ ] **T6.5** — Filtre par centre : changer de centre → vérifier que la liste filtre correctement

### Codi-Moi
- [ ] **T6.6** — Liste Codi-Moi : vérifier filtres (type: proverbe/citation/ressource/bonne_pratique, pays, ethnie)
- [ ] **T6.7** — Onglet Tags : ajouter un tag via autocomplete (tape les premières lettres → suggestions), retirer un tag
- [ ] **T6.8** — Onglet Commentaires : vérifier le rendu arborescent (commentaires parents → réponses indentées)
- [ ] **T6.9** — Modération commentaire : cliquer supprimer sur un commentaire → modal confirmation → vérifier disparition
- [ ] **T6.10** — Onglet Réactions : vérifier l'affichage stats (like/dislike counts, éventuellement graphique)

---

## Notes
- Les handlers `centres_culturels.rs` et `codimoi.rs` existent pour l'usage public. L'admin ajoute la gestion d'équipe (membres) et la modération des commentaires.
- Les commentaires Codi-Moi sont hiérarchiques (parent_id) → la vue admin doit les afficher en arborescence.
- Les réactions sont en lecture seule côté admin (pas de modification, juste des stats).
