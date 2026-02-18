# 10 — Profils pays (Country Profile)

> **Phase** : 4 — Modules complexes
> **Section sidebar** : Profils pays
> **Icône** : faEarthAfrica
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/11_country_profile.sql` → `fiche_pays`, `region`, `groupe_ethnique`, `alliance_interethnique`, `conte_histoire`, `site_touristique`, `secteur_developpement`, `saison`, `lien_interethnique`
- `schemas/11b_country_profile_contributions.sql` → `contribution_fiche`
- `schemas/03_shared.sql` → `pays` (FK 1-to-1 unique)
- `schemas/04_iam.sql` → `utilisateur` (FK created_by, modérateur contributions)
- `schemas/13_contraintes_inter_schemas.sql` → FK country_profile ↔ shared, iam
- **Enums** : `etat_contribution` (en_attente/approuvée/rejetée), `type_contribution` (modification/ajout/suppression)

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** — Utilisateurs (contributeurs, modérateurs)
- **`02-referentiels.md`** — Pays (la fiche est liée 1-to-1 à shared.pays)

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** — Stats fiches pays, contributions en attente

### Backend existant
- [x] `src/handlers/fiches_pays.rs` — CRUD fiches pays + contributions publiques — **À étendre pour admin**
- [x] `src/handlers/contributions_fiche.rs` — contributions publiques — **À étendre pour admin**

---

## Structure spéciale

Ce module est le plus complexe : une fiche pays contient **8 sous-entités** gérées comme onglets dans la page d'édition `[id].vue`. Pas de pages dédiées pour les sous-entités.

---

## Backend

### Fiche pays (CRUD principal)
- [ ] `GET /api/admin/profils-pays` — liste paginée + filtres (continent, recherche)
- [ ] `GET /api/admin/profils-pays/:id` — détail complet (avec counts de chaque sous-entité)
- [ ] `POST /api/admin/profils-pays` — création (lié à un pays existant dans shared.pays)
- [ ] `PUT /api/admin/profils-pays/:id` — modification (biographie, contexte, histoire, drapeaux, hymne, langues, monnaie, fuseau)
- [ ] `DELETE /api/admin/profils-pays/:id` — soft delete
- **Fichiers** : `src/handlers/admin/profils_pays.rs`, `src/models/admin/profils_pays.rs`

### Régions (sous-entité)
- [ ] `GET /api/admin/profils-pays/:id/regions` — liste
- [ ] `POST /api/admin/profils-pays/:id/regions` — création
- [ ] `PUT /api/admin/profils-pays/:id/regions/:region_id` — modification
- [ ] `DELETE /api/admin/profils-pays/:id/regions/:region_id` — suppression

### Groupes ethniques (sous-entité)
- [ ] `GET /api/admin/profils-pays/:id/groupes-ethniques` — liste
- [ ] `POST /api/admin/profils-pays/:id/groupes-ethniques` — création (langues, population, objets culturels)
- [ ] `PUT /api/admin/profils-pays/:id/groupes-ethniques/:ge_id` — modification
- [ ] `DELETE /api/admin/profils-pays/:id/groupes-ethniques/:ge_id` — suppression

### Alliances interethniques (sous-entité)
- [ ] `GET /api/admin/profils-pays/:id/alliances` — liste
- [ ] `POST /api/admin/profils-pays/:id/alliances` — création
- [ ] `PUT /api/admin/profils-pays/:id/alliances/:alliance_id` — modification
- [ ] `DELETE /api/admin/profils-pays/:id/alliances/:alliance_id` — suppression

### Contes & Histoires (sous-entité)
- [ ] `GET /api/admin/profils-pays/:id/contes` — liste + filtre par type (conte/histoire_drôle/légende/mythe)
- [ ] `POST /api/admin/profils-pays/:id/contes` — création
- [ ] `PUT /api/admin/profils-pays/:id/contes/:conte_id` — modification
- [ ] `DELETE /api/admin/profils-pays/:id/contes/:conte_id` — suppression

### Sites touristiques (sous-entité)
- [ ] `GET /api/admin/profils-pays/:id/sites-touristiques` — liste
- [ ] `POST /api/admin/profils-pays/:id/sites-touristiques` — création (coordonnées, région)
- [ ] `PUT /api/admin/profils-pays/:id/sites-touristiques/:site_id` — modification
- [ ] `DELETE /api/admin/profils-pays/:id/sites-touristiques/:site_id` — suppression

### Secteurs de développement (sous-entité)
- [ ] `GET /api/admin/profils-pays/:id/secteurs` — liste
- [ ] `POST /api/admin/profils-pays/:id/secteurs` — création
- [ ] `PUT /api/admin/profils-pays/:id/secteurs/:secteur_id` — modification
- [ ] `DELETE /api/admin/profils-pays/:id/secteurs/:secteur_id` — suppression

### Saisons (sous-entité)
- [ ] `GET /api/admin/profils-pays/:id/saisons` — liste
- [ ] `POST /api/admin/profils-pays/:id/saisons` — création (plage de mois)
- [ ] `PUT /api/admin/profils-pays/:id/saisons/:saison_id` — modification
- [ ] `DELETE /api/admin/profils-pays/:id/saisons/:saison_id` — suppression

### Liens interethniques transfrontaliers (sous-entité)
- [ ] `GET /api/admin/profils-pays/:id/liens-interethniques` — liste
- [ ] `POST /api/admin/profils-pays/:id/liens-interethniques` — création (type: migration/parenté/commerce, pays lié)
- [ ] `PUT /api/admin/profils-pays/:id/liens-interethniques/:lien_id` — modification
- [ ] `DELETE /api/admin/profils-pays/:id/liens-interethniques/:lien_id` — suppression

### Contributions collaboratives (modération)
- [ ] `GET /api/admin/profils-pays/contributions` — liste globale + filtres (état, fiche, contributeur, date)
- [ ] `GET /api/admin/profils-pays/contributions/:id` — détail (section cible, type, ancien/nouveau, contributeur)
- [ ] `PATCH /api/admin/profils-pays/contributions/:id/etat` — approuver / rejeter (avec commentaire modérateur)

---

## Frontend

### Pages
- [ ] `app/pages/admin/profils-pays/index.vue` — liste des fiches pays (avec drapeau + count sous-entités)
- [ ] `app/pages/admin/profils-pays/create.vue` — formulaire de création (sélecteur pays + infos de base)
- [ ] `app/pages/admin/profils-pays/[id].vue` — édition avec **9 onglets** :
  - [ ] Onglet Général — biographie, contexte, histoire, drapeaux, emblèmes, hymne, langues, monnaie, fuseau horaire
  - [ ] Onglet Régions — CRUD inline (table éditable)
  - [ ] Onglet Groupes ethniques — CRUD inline (langues, population, objets culturels)
  - [ ] Onglet Alliances interethniques — CRUD inline
  - [ ] Onglet Contes & Histoires — CRUD inline + filtre par type
  - [ ] Onglet Sites touristiques — CRUD inline (avec coordonnées + carte?)
  - [ ] Onglet Secteurs de développement — CRUD inline
  - [ ] Onglet Saisons — CRUD inline
  - [ ] Onglet Liens interethniques — CRUD inline (type, pays lié)
- [ ] `app/composables/useAdminProfilsPays.ts`

### Contributions (page séparée ou modale)
- [ ] Vue contributions en attente (accessible depuis la page de la fiche ou globalement)
- [ ] Détail contribution : diff ancien/nouveau valeur, boutons approuver/rejeter
- [ ] `app/composables/useAdminContributions.ts`

---

## Critères de validation
- [ ] CRUD fiche pays principal fonctionnel
- [ ] CRUD inline pour les 8 sous-entités (dans les onglets)
- [ ] Workflow contributions : en_attente → approuvée/rejetée
- [ ] Diff visuel ancien/nouveau pour les contributions
- [ ] Lien 1-to-1 fiche ↔ pays respecté (pas de doublon)

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Navigation onglets
- [ ] **T10.1** — Page `[id].vue` : vérifier la navigation entre les 9 onglets, chaque onglet charge ses données
- [ ] **T10.2** — Vérifier que le changement d'onglet ne perd pas les données non sauvegardées (ou affiche un avertissement)

### CRUD inline sous-entités
- [ ] **T10.3** — Onglet Régions : ajouter une région (formulaire inline dans la table), modifier, supprimer
- [ ] **T10.4** — Onglet Groupes ethniques : ajouter (langues, population, objets culturels), vérifier rendu table
- [ ] **T10.5** — Onglet Alliances interethniques : CRUD inline
- [ ] **T10.6** — Onglet Contes : ajouter un conte + vérifier filtre par type (conte/histoire_drôle/légende/mythe)
- [ ] **T10.7** — Onglet Sites touristiques : ajouter un site avec coordonnées, vérifier rendu (carte si implémentée)
- [ ] **T10.8** — Onglet Saisons : ajouter une saison avec plage de mois
- [ ] **T10.9** — Onglet Liens interethniques : ajouter un lien (type + sélecteur pays lié)

### Contributions
- [ ] **T10.10** — Liste contributions : vérifier filtres (état, fiche, contributeur)
- [ ] **T10.11** — Diff visuel : ouvrir une contribution → vérifier l'affichage côte à côte ancien/nouveau (highlighting des changements)
- [ ] **T10.12** — Workflow : approuver une contribution → modal → vérifier changement état ; rejeter → pareil

---

## Notes
- Module le plus complexe en nombre de sous-entités. L'approche par onglets dans `[id].vue` évite la multiplication des pages.
- Les handlers `fiches_pays.rs` et `contributions_fiche.rs` existent. L'admin étend avec les sous-entités CRUD et la modération.
- Les contributions collaboratives utilisent un système de diff (old_value/new_value en JSONB) → la vue admin doit afficher un diff visuel clair.
- Les contes ont un enum de type (conte/histoire_drole/legende/mythe) pour le filtre.
