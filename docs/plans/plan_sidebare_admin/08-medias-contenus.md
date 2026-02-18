# 08 — Médias & Contenus (Radio/TV, Événements, MOOC, Bibliothèque)

> **Phase** : 3 — Fonctionnalités avancées
> **Section sidebar** : Médias & Contenus
> **Icône** : faTv
> **Statut global** : [ ] Non démarré

---

## Dépendances

### Fichiers SQL requis
- `schemas/09_media_content.sql` → `station_radio`, `programme_radio_tele`, `evenement`, `evenement_inscription`, `mooc`, `mooc_inscription`, `livre`, `livre_tag`, `chaine_tv`
- `schemas/03_shared.sql` → `pays`, `categorie`, `tag` (FK)
- `schemas/04_iam.sql` → `utilisateur` (FK created_by, inscriptions)
- `schemas/13_contraintes_inter_schemas.sql` → FK media_content ↔ shared, iam
- `migration_chaine_tv.sql` → migration standalone chaine_tv
- `migration_station_radio.sql` → migration standalone station_radio
- **Enums** : `type_programme_media`, `categorie_radio`, `format_evenement`, `acces_livre`, `type_station`, `categorie_chaine_tv`

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — Composants CRUD, middleware, useAdmin
- **`01-utilisateurs-acces.md`** — Utilisateurs (créateurs, inscrits)
- **`02-referentiels.md`** — Pays, Catégories, Tags, Médiathèque

### Plans qui dépendent de celui-ci
- **`12-dashboard.md`** — Stats événements, MOOC, bibliothèque

### Backend existant
- [x] `src/handlers/livres.rs` — CRUD livres public — **À étendre pour admin**
- [x] `src/handlers/evenements.rs` — CRUD événements public — **À étendre pour admin**
- [x] `src/handlers/moocs.rs` — CRUD MOOC public — **À étendre pour admin**
- [x] `src/handlers/stations_radio.rs` — stations radio public — **À étendre pour admin**
- [x] `src/handlers/television.rs` — chaînes TV public — **À étendre pour admin**
- [ ] Endpoints admin Radio & TV combinés — **À CRÉER**

---

## Sous-rubriques

### 1. Radio & TV (`/admin/radio-tele`)

#### Backend
- [ ] `GET /api/admin/stations-radio` — liste paginée + filtres (type_station, catégorie, pays)
- [ ] `GET /api/admin/stations-radio/:id` — détail
- [ ] `POST /api/admin/stations-radio` — création
- [ ] `PUT /api/admin/stations-radio/:id` — modification
- [ ] `DELETE /api/admin/stations-radio/:id` — soft delete
- [ ] `GET /api/admin/chaines-tv` — liste paginée + filtres (catégorie, pays)
- [ ] `GET /api/admin/chaines-tv/:id` — détail
- [ ] `POST /api/admin/chaines-tv` — création
- [ ] `PUT /api/admin/chaines-tv/:id` — modification
- [ ] `DELETE /api/admin/chaines-tv/:id` — soft delete
- [ ] `GET /api/admin/programmes-media` — liste paginée + filtres (type: radio/télé, catégorie)
- [ ] `POST /api/admin/programmes-media` — création
- [ ] `PUT /api/admin/programmes-media/:id` — modification
- [ ] `DELETE /api/admin/programmes-media/:id` — soft delete
- **Fichiers** : `src/handlers/admin/radio_tele.rs`

#### Frontend
- [ ] `app/pages/admin/radio-tele/index.vue` — vue combinée avec onglets (Stations Radio / Chaînes TV / Programmes)
- [ ] `app/pages/admin/radio-tele/create.vue` — formulaire avec sélecteur type (radio/TV/programme)
- [ ] `app/pages/admin/radio-tele/[id].vue` — édition
- [ ] `app/composables/useAdminRadioTele.ts`

---

### 2. Événements (`/admin/evenements`)

#### Backend
- [ ] `GET /api/admin/evenements` — liste paginée + filtres (format, statut, date range, pays, recherche full-text)
- [ ] `GET /api/admin/evenements/:id` — détail (avec inscriptions count)
- [ ] `POST /api/admin/evenements` — création
- [ ] `PUT /api/admin/evenements/:id` — modification
- [ ] `DELETE /api/admin/evenements/:id` — soft delete
- [ ] `GET /api/admin/evenements/:id/inscriptions` — liste inscrits + statut
- [ ] `PATCH /api/admin/evenements/:id/inscriptions/:insc_id/statut` — changer statut inscription (confirmé/annulé/présent/absent)
- [ ] `GET /api/admin/evenements/:id/inscriptions/stats` — stats (inscrits, confirmés, présents)
- **Fichiers** : `src/handlers/admin/evenements.rs`

#### Frontend
- [ ] `app/pages/admin/evenements/index.vue` — liste + filtres (format, statut, date)
- [ ] `app/pages/admin/evenements/create.vue` — formulaire (titre, description, format, dates, capacité, lieu)
- [ ] `app/pages/admin/evenements/[id].vue` — édition avec onglets :
  - [ ] Onglet Infos — données principales
  - [ ] Onglet Inscriptions — liste inscrits + changement statut + stats
- [ ] `app/composables/useAdminEvenements.ts`

---

### 3. MOOC (`/admin/mooc`)

#### Backend
- [ ] `GET /api/admin/mooc` — liste paginée + filtres (état, domaine, recherche)
- [ ] `GET /api/admin/mooc/:id` — détail (avec inscriptions count + progression moyenne)
- [ ] `POST /api/admin/mooc` — création
- [ ] `PUT /api/admin/mooc/:id` — modification
- [ ] `DELETE /api/admin/mooc/:id` — soft delete
- [ ] `GET /api/admin/mooc/:id/inscriptions` — liste inscrits + progression
- [ ] `GET /api/admin/mooc/:id/inscriptions/stats` — stats (inscrits, en_cours, complétés, abandonnés)
- **Fichiers** : `src/handlers/admin/mooc.rs`

#### Frontend
- [ ] `app/pages/admin/mooc/index.vue` — liste + filtres
- [ ] `app/pages/admin/mooc/create.vue` — formulaire (titre, description, format, prérequis, capacité)
- [ ] `app/pages/admin/mooc/[id].vue` — édition avec onglets :
  - [ ] Onglet Infos — données principales
  - [ ] Onglet Inscriptions — liste avec progression (barre %), stats
- [ ] `app/composables/useAdminMooc.ts`

---

### 4. Bibliothèque (`/admin/livres`)

#### Backend
- [ ] `GET /api/admin/livres` — liste paginée + filtres (type_document, catégorie, accès, recherche full-text)
- [ ] `GET /api/admin/livres/:id` — détail (avec tags)
- [ ] `POST /api/admin/livres` — création (avec upload fichier/couverture)
- [ ] `PUT /api/admin/livres/:id` — modification
- [ ] `DELETE /api/admin/livres/:id` — soft delete
- [ ] `POST /api/admin/livres/:id/tags` — ajouter tag
- [ ] `DELETE /api/admin/livres/:id/tags/:tag_id` — retirer tag
- **Fichiers** : `src/handlers/admin/livres.rs`

#### Frontend
- [ ] `app/pages/admin/livres/index.vue` — liste + filtres (type doc, catégorie, accès)
- [ ] `app/pages/admin/livres/create.vue` — formulaire (titre, auteur, ISBN, type, catégorie, accès, upload couverture + fichier)
- [ ] `app/pages/admin/livres/[id].vue` — édition avec onglets :
  - [ ] Onglet Infos — données principales + fichiers
  - [ ] Onglet Tags — gestion tags (autocomplete)
- [ ] `app/composables/useAdminLivres.ts`

---

## Critères de validation
- [ ] CRUD complet Radio & TV (stations, chaînes, programmes)
- [ ] CRUD événements avec gestion inscriptions + stats
- [ ] CRUD MOOC avec suivi progression inscrits
- [ ] CRUD bibliothèque avec tags + upload fichier
- [ ] Filtres full-text sur événements et livres
- [ ] Stats inscriptions (graphiques ou compteurs)

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Radio & TV
- [ ] **T8.1** — Vue combinée onglets : vérifier la navigation entre Stations Radio / Chaînes TV / Programmes
- [ ] **T8.2** — CRUD station radio : créer (URL stream, genre), éditer, supprimer
- [ ] **T8.3** — CRUD chaîne TV : créer (catégorie, stream), éditer, supprimer

### Événements
- [ ] **T8.4** — Liste événements : vérifier filtres (format présentiel/en_ligne/hybride, statut, date range)
- [ ] **T8.5** — Formulaire événement : vérifier tous les champs (dates, capacité, lieu, format)
- [ ] **T8.6** — Onglet Inscriptions : vérifier la liste des inscrits avec badges statut (inscrit/confirmé/annulé/présent/absent)
- [ ] **T8.7** — Changement statut inscription : sélectionner un inscrit → changer statut → vérifier mise à jour badge
- [ ] **T8.8** — Stats inscriptions : vérifier les compteurs (inscrits, confirmés, présents)

### MOOC
- [ ] **T8.9** — Onglet Inscriptions MOOC : vérifier l'affichage des barres de progression (%)
- [ ] **T8.10** — Stats MOOC : vérifier compteurs (inscrits, en_cours, complétés, abandonnés)

### Bibliothèque
- [ ] **T8.11** — Upload livre : upload couverture (preview image) + fichier document, vérifier
- [ ] **T8.12** — Onglet Tags : ajouter/retirer tags via autocomplete
- [ ] **T8.13** — Filtres livres : type document + catégorie + accès (lecture_seule/lecture_telechargement)

---

## Notes
- C'est la rubrique la plus volumineuse (4 sous-rubriques, 9 tables). Peut être découpée en sous-phases si nécessaire.
- Les handlers publics existent déjà pour livres, événements, moocs, stations radio, et TV. L'admin ajoute la gestion des inscriptions et les opérations avancées.
- Radio & TV combine 3 entités (stations, chaînes, programmes) dans une seule page avec onglets pour simplifier la navigation.
