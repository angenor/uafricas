# 04 — Programmes d'échange (Exchange)

> **Phase** : 2 — Modules métier
> **Section sidebar** : Programmes d'échange
> **Icône** : faPlane
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/06_exchange.sql` → `programme`, `candidature`, `ecole_partenaire`, `faculte`
- `schemas/03_shared.sql` → `pays`, `domaine_secteur` (FK)
- `schemas/04_iam.sql` → `utilisateur` (FK created_by, validated_by, candidat)
- `schemas/13_contraintes_inter_schemas.sql` → FK exchange ↔ shared, iam
- **Enums** : `etat_programme`, `duree_programme`, `etat_candidature`

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** — Utilisateurs (créateurs, validateurs, candidats)
- **`02-referentiels.md`** — Pays (origine/destination), Domaines & Secteurs

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** — Stats programmes (actifs, candidatures en attente)

### Backend existant
- [x] `src/handlers/sabbatiques.rs` — programmes sabbatiques publics — **Potentiellement à refactorer pour admin**
- [ ] Endpoints admin CRUD programmes + candidatures — **À CRÉER**

---

## Sous-rubriques

### 1. Programmes (`/admin/programmes`)

#### Backend
- [ ] `GET /api/admin/programmes` — liste paginée + filtres (état, domaine, durée, pays, dates)
- [ ] `GET /api/admin/programmes/:id` — détail complet (avec candidatures count)
- [ ] `POST /api/admin/programmes` — création
- [ ] `PUT /api/admin/programmes/:id` — modification
- [ ] `PATCH /api/admin/programmes/:id/etat` — validation : changer état (publié/suspendu/annulé)
- [ ] `DELETE /api/admin/programmes/:id` — soft delete
- [ ] `GET /api/admin/programmes/:id/candidatures` — liste candidatures du programme
- **Fichiers** : `src/handlers/admin/programmes.rs`, `src/models/admin/programmes.rs`

#### Frontend
- [ ] `app/pages/admin/programmes/index.vue` — liste avec DataTable + filtres (état, domaine, durée)
- [ ] `app/pages/admin/programmes/create.vue` — formulaire :
  - Infos de base (titre, description, objectifs)
  - Domaine, pays origine/destination
  - Dates, durée, capacité
  - Couverture (billets, hébergement, subsistance)
  - Prérequis
- [ ] `app/pages/admin/programmes/[id].vue` — édition + onglet candidatures
- [ ] `app/composables/useAdminProgrammes.ts`

---

### 2. Candidatures (`/admin/candidatures`)

#### Backend
- [ ] `GET /api/admin/candidatures` — liste paginée + filtres (état, programme, candidat, date)
- [ ] `GET /api/admin/candidatures/:id` — détail (motivation, CV, profil candidat)
- [ ] `PATCH /api/admin/candidatures/:id/etat` — revue : accepter/refuser (avec commentaire)
- **Fichiers** : `src/handlers/admin/candidatures.rs`

#### Frontend
- [ ] `app/pages/admin/candidatures/index.vue` — liste avec DataTable + filtres (état, programme)
- [ ] `app/pages/admin/candidatures/[id].vue` — revue détaillée :
  - Profil candidat (lien vers utilisateur)
  - Lettre de motivation
  - CV (téléchargement)
  - Boutons accepter / refuser avec champ commentaire
- [ ] `app/composables/useAdminCandidatures.ts`

---

## Critères de validation
- [ ] CRUD complet programmes avec validation d'état
- [ ] Workflow candidature : soumise → en_revue → acceptée/refusée
- [ ] Filtres par état, domaine, durée fonctionnels
- [ ] Lien candidature → profil utilisateur fonctionnel
- [ ] Téléchargement CV depuis la page de revue

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Programmes
- [ ] **T4.1** — Liste programmes : vérifier DataTable + badges état (publié=vert, suspendu=orange, annulé=rouge)
- [ ] **T4.2** — Formulaire programme : remplir tous les champs (titre, dates, durée, couverture, prérequis), soumettre
- [ ] **T4.3** — Validation état : changer l'état d'un programme (publier/suspendre/annuler) → modal confirmation → vérifier badge

### Candidatures
- [ ] **T4.4** — Liste candidatures : vérifier filtres par état et par programme
- [ ] **T4.5** — Revue candidature : ouvrir une candidature → vérifier affichage profil candidat, lettre de motivation, lien CV
- [ ] **T4.6** — Téléchargement CV : cliquer sur le lien CV → vérifier que le fichier se télécharge
- [ ] **T4.7** — Workflow accepter/refuser : cliquer accepter → remplir commentaire → confirmer → vérifier changement état
- [ ] **T4.8** — Lien profil candidat : cliquer sur le nom du candidat → vérifier navigation vers `/admin/utilisateurs/[id]`

---

## Notes
- Le handler `sabbatiques.rs` existant couvre une partie des programmes publics. L'admin ajoute la gestion complète + le workflow de candidatures.
- Les écoles partenaires et facultés (`ecole_partenaire`, `faculte`) sont dans le même schema mais gérées dans une rubrique INUDA dédiée (hors sidebar admin actuel, possiblement ajoutée plus tard).
