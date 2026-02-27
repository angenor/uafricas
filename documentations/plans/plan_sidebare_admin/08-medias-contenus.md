# 08 — Médias & Contenus (Radio/TV, Événements, MOOC, Bibliothèque)

> **Phase** : 3 — Fonctionnalités avancées
> **Section sidebar** : Médias & Contenus
> **Icône** : faTv
> **Statut global** : [x] Terminé

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
- [x] Endpoints admin Radio & TV combinés — **CRÉÉ**

---

## Sous-rubriques

### 1. Radio & TV (`/admin/radio-tele`)

#### Backend
- [x] `GET /api/admin/stations-radio` — liste paginée + filtres (type_station, catégorie, pays)
- [x] `GET /api/admin/stations-radio/:id` — détail
- [x] `POST /api/admin/stations-radio` — création
- [x] `PUT /api/admin/stations-radio/:id` — modification
- [x] `DELETE /api/admin/stations-radio/:id` — soft delete
- [x] `GET /api/admin/chaines-tv` — liste paginée + filtres (catégorie, pays)
- [x] `GET /api/admin/chaines-tv/:id` — détail
- [x] `POST /api/admin/chaines-tv` — création
- [x] `PUT /api/admin/chaines-tv/:id` — modification
- [x] `DELETE /api/admin/chaines-tv/:id` — soft delete
- [x] `GET /api/admin/programmes-media` — liste paginée + filtres (type: radio/télé, catégorie)
- [x] `POST /api/admin/programmes-media` — création
- [x] `PUT /api/admin/programmes-media/:id` — modification
- [x] `DELETE /api/admin/programmes-media/:id` — soft delete
- **Fichiers** : `src/handlers/admin/radio_tele.rs`, `src/models/admin/radio_tele.rs`

#### Frontend
- [x] `app/pages/admin/radio-tele/index.vue` — vue combinée avec onglets (Stations Radio / Chaînes TV / Programmes)
- [x] `app/pages/admin/radio-tele/create.vue` — formulaire avec sélecteur type (radio/TV/programme)
- [x] `app/pages/admin/radio-tele/[id].vue` — édition
- [x] `app/composables/useAdminRadioTele.ts`

---

### 2. Événements (`/admin/evenements`)

#### Backend
- [x] `GET /api/admin/evenements` — liste paginée + filtres (format, statut, date range, pays, recherche full-text)
- [x] `GET /api/admin/evenements/:id` — détail (avec inscriptions count)
- [x] `POST /api/admin/evenements` — création
- [x] `PUT /api/admin/evenements/:id` — modification
- [x] `DELETE /api/admin/evenements/:id` — soft delete
- [x] `GET /api/admin/evenements/:id/inscriptions` — liste inscrits + statut
- [x] `PATCH /api/admin/evenements/:id/inscriptions/:insc_id/statut` — changer statut inscription (confirmé/annulé/présent/absent)
- [x] `GET /api/admin/evenements/:id/inscriptions/stats` — stats (inscrits, confirmés, présents)
- **Fichiers** : `src/handlers/admin/evenements.rs`, `src/models/admin/evenement.rs`

#### Frontend
- [x] `app/pages/admin/evenements/index.vue` — liste + filtres (format, statut, date)
- [x] `app/pages/admin/evenements/create.vue` — formulaire (titre, description, format, dates, capacité, lieu)
- [x] `app/pages/admin/evenements/[id].vue` — édition avec onglets :
  - [x] Onglet Infos — données principales
  - [x] Onglet Inscriptions — liste inscrits + changement statut + stats
- [x] `app/composables/useAdminEvenements.ts`

---

### 3. MOOC (`/admin/mooc`)

#### Backend
- [x] `GET /api/admin/mooc` — liste paginée + filtres (état, domaine, recherche)
- [x] `GET /api/admin/mooc/:id` — détail (avec inscriptions count + progression moyenne)
- [x] `POST /api/admin/mooc` — création
- [x] `PUT /api/admin/mooc/:id` — modification
- [x] `DELETE /api/admin/mooc/:id` — soft delete
- [x] `GET /api/admin/mooc/:id/inscriptions` — liste inscrits + progression
- [x] `GET /api/admin/mooc/:id/inscriptions/stats` — stats (inscrits, en_cours, complétés, abandonnés)
- **Fichiers** : `src/handlers/admin/mooc.rs`, `src/models/admin/mooc.rs`

#### Frontend
- [x] `app/pages/admin/mooc/index.vue` — liste + filtres
- [x] `app/pages/admin/mooc/create.vue` — formulaire (titre, description, format, prérequis, capacité)
- [x] `app/pages/admin/mooc/[id].vue` — édition avec onglets :
  - [x] Onglet Infos — données principales
  - [x] Onglet Inscriptions — liste avec progression (barre %), stats
- [x] `app/composables/useAdminMooc.ts`

---

### 4. Bibliothèque (`/admin/livres`)

#### Backend
- [x] `GET /api/admin/livres` — liste paginée + filtres (type_document, catégorie, accès, recherche full-text)
- [x] `GET /api/admin/livres/:id` — détail (avec tags)
- [x] `POST /api/admin/livres` — création (avec upload fichier/couverture)
- [x] `PUT /api/admin/livres/:id` — modification
- [x] `DELETE /api/admin/livres/:id` — soft delete
- [x] `POST /api/admin/livres/:id/tags` — ajouter tag
- [x] `DELETE /api/admin/livres/:id/tags/:tag_id` — retirer tag
- **Fichiers** : `src/handlers/admin/livres.rs`, `src/models/admin/livre.rs`

#### Frontend
- [x] `app/pages/admin/livres/index.vue` — liste + filtres (type doc, catégorie, accès)
- [x] `app/pages/admin/livres/create.vue` — formulaire (titre, auteur, ISBN, type, catégorie, accès, upload couverture + fichier)
- [x] `app/pages/admin/livres/[id].vue` — édition avec onglets :
  - [x] Onglet Infos — données principales + fichiers
  - [x] Onglet Tags — gestion tags (autocomplete)
- [x] `app/composables/useAdminLivres.ts`

---

## Critères de validation
- [x] CRUD complet Radio & TV (stations, chaînes, programmes)
- [x] CRUD événements avec gestion inscriptions + stats
- [x] CRUD MOOC avec suivi progression inscrits
- [x] CRUD bibliothèque avec tags + upload fichier
- [x] Filtres full-text sur événements et livres
- [x] Stats inscriptions (graphiques ou compteurs)

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### Radio & TV
- [x] **T8.1** — Vue combinée onglets : vérifier la navigation entre Stations Radio / Chaînes TV / Programmes
- [x] **T8.2** — CRUD station radio : créer (URL stream, genre), éditer, supprimer
- [x] **T8.3** — CRUD chaîne TV : créer (catégorie, stream), éditer, supprimer

### Événements
- [x] **T8.4** — Liste événements : vérifier filtres (format présentiel/en_ligne/hybride, statut, date range)
- [x] **T8.5** — Formulaire événement : vérifier tous les champs (dates, capacité, lieu, format)
- [ ] **T8.6** — Onglet Inscriptions : vérifier la liste des inscrits avec badges statut (inscrit/confirmé/annulé/présent/absent)
- [ ] **T8.7** — Changement statut inscription : sélectionner un inscrit → changer statut → vérifier mise à jour badge
- [ ] **T8.8** — Stats inscriptions : vérifier les compteurs (inscrits, confirmés, présents)

### MOOC
- [x] **T8.9** — Onglet Inscriptions MOOC : vérifier l'affichage des barres de progression (%)
- [x] **T8.10** — Stats MOOC : vérifier compteurs (inscrits, en_cours, complétés, abandonnés)

### Bibliothèque
- [x] **T8.11** — Upload livre : upload couverture (preview image) + fichier document, vérifier
- [x] **T8.12** — Onglet Tags : ajouter/retirer tags via autocomplete
- [x] **T8.13** — Filtres livres : type document + catégorie + accès (lecture_seule/lecture_telechargement)

---

## Notes
- C'est la rubrique la plus volumineuse (4 sous-rubriques, 9 tables). Peut être découpée en sous-phases si nécessaire.
- Les handlers publics existent déjà pour livres, événements, moocs, stations radio, et TV. L'admin ajoute la gestion des inscriptions et les opérations avancées.
- Radio & TV combine 3 entités (stations, chaînes, programmes) dans une seule page avec onglets pour simplifier la navigation.
