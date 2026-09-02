# 05 : Innovation (Innovations, Projets, Africantives)

> **Phase** : 2 : Modules métier
> **Section sidebar** : Innovation
> **Icône** : faLightbulb
> **Statut global** : [x] Implémenté

---

## Dépendances

### Fichiers SQL requis
- `schemas/07_innovation.sql` → `innovation`, `innovation_media`, `projet`, `projet_document`, `africantive`
- `schemas/03_shared.sql` → `domaine_secteur`, `pays` (FK)
- `schemas/04_iam.sql` → `utilisateur`, `organisation` (FK)
- `schemas/13_contraintes_inter_schemas.sql` → FK innovation ↔ shared, iam
- **Enums** : `etat_contenu` (brouillon/publié/suspendu/supprimé), `etat_projet` (soumis/en_revue/approuvé/en_cours/terminé/suspendu/rejeté)

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** : Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** : Utilisateurs + Organisations (porteur de projet)
- **`02-referentiels.md`** : Pays, Domaines & Secteurs, Médiathèque (upload médias)

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** : Stats innovations, projets approuvés

### Backend existant
- [x] `src/handlers/projets.rs` : CRUD public projets, **À étendre pour admin**
- [x] `src/handlers/africantives.rs` : CRUD public africantives, **À étendre pour admin**
- [x] Endpoints admin innovations : **CRÉÉ**

---

## Sous-rubriques

### 1. Innovations (`/admin/innovations`)

#### Backend
- [x] `GET /api/admin/innovations` : liste paginée + filtres (état, domaine, pays, organisation, recherche full-text)
- [x] `GET /api/admin/innovations/:id`, détail complet (avec médias)
- [x] `POST /api/admin/innovations` : création
- [x] `PUT /api/admin/innovations/:id`, modification
- [x] `PATCH /api/admin/innovations/:id/etat`, modération état
- [x] `DELETE /api/admin/innovations/:id`, soft delete
- [x] `POST /api/admin/innovations/:id/medias`, upload média
- [x] `DELETE /api/admin/innovations/:id/medias/:media_id`, retirer média
- **Fichiers** : `src/handlers/admin/innovations.rs`, `src/models/admin/innovation.rs`

#### Frontend
- [x] `app/pages/admin/innovations/index.vue`, liste + filtres
- [x] `app/pages/admin/innovations/create.vue`, formulaire
- [x] `app/pages/admin/innovations/[id].vue`, édition avec onglets :
  - [x] Onglet Infos : données principales + état
  - [x] Onglet Médias : galerie (ajout/retrait médias)
- [x] `app/composables/useAdminInnovations.ts`

---

### 2. Projets (`/admin/projets`)

#### Backend
- [x] `GET /api/admin/projets` : liste paginée + filtres (état_projet, organisation, pays, recherche full-text)
- [x] `GET /api/admin/projets/:id` : détail complet (avec documents)
- [x] `POST /api/admin/projets` : création
- [x] `PUT /api/admin/projets/:id` : modification
- [x] `PATCH /api/admin/projets/:id/etat`, approbation : changer état (approuvé/rejeté/suspendu)
- [x] `DELETE /api/admin/projets/:id`, soft delete
- [x] `POST /api/admin/projets/:id/documents`, upload document supplémentaire
- [x] `DELETE /api/admin/projets/:id/documents/:doc_id`, retirer document
- **Fichiers** : `src/handlers/admin/projets_admin.rs`, `src/models/admin/projet.rs`

#### Frontend
- [x] `app/pages/admin/projets/index.vue`, liste + filtres (état, organisation)
- [x] `app/pages/admin/projets/create.vue`, formulaire :
  - Infos de base (titre, description, objectifs)
  - Budget, durée, calendrier de mise en oeuvre
  - Risques identifiés
  - Organisation porteuse, pays
- [x] `app/pages/admin/projets/[id].vue`, édition avec onglets :
  - [x] Onglet Infos : données principales + workflow approbation
  - [x] Onglet Documents : upload/téléchargement documents supplémentaires
- [x] `app/composables/useAdminProjets.ts`

---

### 3. Africantives (`/admin/africantives`)

#### Backend
- [x] `GET /api/admin/africantives` : liste paginée + filtres (domaine, pays, état)
- [x] `GET /api/admin/africantives/:id`, détail
- [x] `POST /api/admin/africantives` : création
- [x] `PUT /api/admin/africantives/:id`, modification
- [x] `PATCH /api/admin/africantives/:id/etat`, modération
- [x] `DELETE /api/admin/africantives/:id`, soft delete
- **Fichiers** : `src/handlers/admin/africantives_admin.rs`, `src/models/admin/africantive.rs`

#### Frontend
- [x] `app/pages/admin/africantives/index.vue`, liste + filtres
- [x] `app/pages/admin/africantives/create.vue`, formulaire
- [x] `app/pages/admin/africantives/[id].vue`, édition
- [x] `app/composables/useAdminAfricantives.ts`

---

## Critères de validation
- [x] CRUD complet pour innovations, projets, africantives
- [x] Workflow projet : soumis → en_revue → approuvé/rejeté
- [x] Upload médias innovations + documents projets
- [x] Filtres full-text sur innovations et projets
- [ ] Lien vers organisation porteuse fonctionnel

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Innovations
- [ ] **T5.1** : Liste innovations : vérifier DataTable + filtres (état, domaine, pays, organisation)
- [ ] **T5.2** : Onglet Médias : upload images/vidéos, vérifier previews dans la galerie
- [ ] **T5.3** : Modération : changer l'état (publier/suspendre) → vérifier mise à jour badge

### Projets
- [ ] **T5.4** : Formulaire projet : remplir tous les champs (budget, durée, calendrier, risques), soumettre
- [ ] **T5.5** : Workflow approbation : vérifier les boutons approuver/rejeter/suspendre → modal → changement état
- [ ] **T5.6** : Onglet Documents : upload document PDF/Word → vérifier dans la liste → télécharger → supprimer
- [ ] **T5.7** : Lien organisation : cliquer sur l'organisation porteuse → vérifier navigation vers `/admin/organisations/[id]`

### Africantives
- [ ] **T5.8** : CRUD complet africantives : créer, lister, éditer, supprimer

---

## Notes
- Les handlers publics `projets.rs` et `africantives.rs` existent déjà. Les endpoints admin ajoutent la modération et les opérations avancées.
- `etat_contenu` (innovations, africantives) et `etat_projet` (projets) sont des enums différents avec des workflows distincts.
- Permission type utilisé : `"innovation"` pour les 3 sous-rubriques (innovations, projets, africantives).
