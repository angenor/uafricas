# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

UAfricas is a pan-African platform for sustainable development. Monorepo with two independent projects:

- **`uafricas_frontend/`** — Nuxt 4 (Vue 3 SSR) + TypeScript + Tailwind CSS v4 + daisyUI v5 + Pinia
- **`uafricas_backend/`** — Rust + Actix-Web 4 (minimal scaffold, in early development)

## Development Commands

### Frontend (`uafricas_frontend/`)

```bash
pnpm install          # Install dependencies
pnpm dev              # Dev server on http://localhost:3000
pnpm build            # Production build
pnpm generate         # Static site generation
pnpm preview          # Preview production build
```

### Backend (`uafricas_backend/`)

```bash
cargo build                                        # Debug build
RUST_LOG=info cargo run                            # Run with logging (http://127.0.0.1:8080)
cargo build --release                              # Release build
kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run  # Redémarrer proprement (kill ancien + relance)
```

> **Important** : `cargo run` recompile automatiquement si le code a changé. Pas besoin de `cargo build` avant. Pour redémarrer après modification, toujours **tuer l'ancien processus** avant de relancer, sinon le port 8080 reste occupé par l'ancienne version.

Backend env vars: `DATABASE_URL` (required), `JWT_SECRET` (required), `HOST` (default: 127.0.0.1), `PORT` (default: 8080), `UPLOAD_DIR` (default: ./uploads), `FRONTEND_URL` (default: http://localhost:3000), `JWT_EXPIRATION_MINUTES` (default: 15), `REFRESH_EXPIRATION_DAYS` (default: 7), `RUST_LOG` (info/debug/error), `SMTP_HOST` (required), `SMTP_PORT` (default: 587), `SMTP_USERNAME` (required), `SMTP_PASSWORD` (required), `SMTP_FROM_EMAIL` (default: SMTP_USERNAME), `SMTP_FROM_NAME` (default: UAfricas), `EMAIL_VERIFICATION_EXPIRATION_HOURS` (default: 24).

### Database (Docker)

```bash
docker compose up -d              # Lancer PostgreSQL + pgAdmin
docker compose down               # Arrêter les services
docker compose down -v            # Arrêter + supprimer les données (reset complet)
docker compose logs postgres      # Voir les logs PostgreSQL
```

- **PostgreSQL** : `localhost:5432` (user: `uafricas`, db: `africans_db`)
- **Adminer** : `http://localhost:8088` (système: PostgreSQL, serveur: `postgres`, utilisateur: `uafricas`, base: `africans_db`)
- Config dans `.env` (gitignored, contient les credentials PostgreSQL). Le schéma SQL (`uafricas_backend/doc/bd/schema.sql`) s'initialise automatiquement au premier lancement.

### No linting, testing, or CI/CD configured yet.

## Architecture

### Frontend

**Nuxt 4 file-based routing** with pages in `app/pages/`. Dynamic routes use `[id].vue` syntax. Two layouts: `default.vue` (NavBar + Footer) and `auth.vue` (minimal, no chrome).

**Component organization** is feature-based under `app/components/` — each feature domain has its own directory (e.g., `marche/`, `experts/`, `evenements/`, `forums/`, `universite/`). Shared components live in `common/` and `layout/`.

**State management** uses a single Pinia store (`app/stores/user.ts`) for user authentication state (Utilisateur interface with id, nom, prenom, email, roles, etat). Stores accessToken (in-memory) and refreshToken (localStorage). Getters: fullName, displayName, isAdmin, isValidated.

**Composables** (`app/composables/`):
- `useAfricantives` — API client pour les initiatives africaines (listerAfricantives avec filtres/pagination/tri, obtenirAfricantive, creerAfricantive multipart, listerDomaines, listerPays). Constantes DOMAINES_AFRICANTIVES/PAYS_AFRICAINS
- `useAuth` — API client for authentication (register, verifierEmail, renvoyerVerification, login, logout, refreshAccessToken, initAuth, hasRole). Connects to backend `/api/auth/*` endpoints via $fetch. Exposes loading/error state and user getters. Register retourne l'email (pas de tokens, compte en_attente). verifierEmail active le compte et connecte l'utilisateur.
- `useAudioPlayer` — HTML5 audio controls for radio streaming (play/pause/volume/station switching)
- `useAOS` — initializes Animate On Scroll (1000ms duration, once, ease-out-cubic)
- `useBibliotheque` — API client pour la bibliothèque numérique (CRUD livres via $fetch, upload multipart, mapping accès DB↔frontend)
- `useBibliothequeHumaine` — API client pour les bibliothèques humaines (listerBiblios avec filtres/pagination, obtenirBiblio, inscrireBiblioHumaine, listerSpecialites). Mapping BiblioHumaineAPI frontend
- `useCentresCulturels` — API client pour les centres culturels (listerCentres, obtenirCentre, obtenirProgrammation). Inclut utilitaires de formatage dates/heures en français et mapping mode DB↔frontend
- `useCodiMoi` — API client pour Codi-Moi (listerPosts avec filtres/pagination, obtenirPost, creerPost). Mapping CodiMoiPostAPI↔CodiMoiPost frontend
- `useEvenements` — API client pour les événements (listerEvenements avec filtres/pagination/année, obtenirEvenement, creerEvenement multipart, inscrireEvenement). Mapping format DB↔frontend, calcul statut temporel, constantes TYPES_EVENEMENT/ANNEES/PAYS_AFRICAINS
- `useExperts` — API client pour les experts (listerExperts avec filtres/pagination/tri, obtenirExpert, creerCandidature). Constantes CATEGORIES_EXPERTISE/PROFILS_PROFESSIONNELS/PAYS_EXPERTS, interfaces ExpertAPI/ExpertiseInfoAPI
- `useGouvernance` — API client pour la gouvernance citoyenne (getStats, getContributions). Requête UNION ALL sur factcheck + bad_habit + idea_force avec mapping vers ContributionCitoyenne
- `useMarcheAfricain` — API client pour le marché africain (listerAnnonces avec filtres/pagination/tri, obtenirAnnonce). Inclut constantes (CATEGORIES, TYPES_ECHANGE, DEVISES), utilitaires de formatage (prix, dates) et mapping type_operation DB↔frontend
- `useStationsRadio` — API client pour les stations radio (listerStations avec filtres/pagination, obtenirStation, listerPays, listerGenres, creerStation). Mapping StationRadioAPI↔RadioStation frontend
- `useOpportuniteAfrique` — API client pour les fiches pays (listerFiches avec filtres/pagination/region, obtenirFiche par UUID/code ISO, listerRegions). Formatage dates en français, interfaces FichePaysAPI/FichePaysDetailAPI
- `useProjets` — API client pour les projets de développement (listerProjets avec filtres/pagination/tri, obtenirProjet, creerProjet multipart, obtenirStatistiques). Constantes PAYS_PROJETS/BUDGETS/DUREES/OPTIONS_TRI/STATUTS_LABELS, utilitaires formatCurrency/formatDate/getStatutInfo/getInitiales, mapping etat DB→statut frontend
- `useTelevision` — API client pour la télévision (listerChaines avec filtres/pagination, obtenirChaine, listerProgrammesVedettes, obtenirProgrammeVedette, listerPays, listerCategories, obtenirStats, creerChaine, creerProgrammeVedette). Mapping ChaineTvAPI↔TvChannel et ProgrammeTeleAPI↔TvProgram frontend
- `useAfrolang` — API client pour les salles Afrolang (listerSalles avec filtres/pagination/langue, obtenirSalle, listerSallesPrivees, obtenirSallePrivee, creerSallePrivee, obtenirSession, creerSession, demarrerSession, terminerSession, rejoindreSession, quitterSession, genererTokenSession, obtenirTableauBlanc, sauvegarderTableauBlanc, effacerTableauBlanc, obtenirStats, listerLangues). Interfaces SalleAPI/SallePriveeAPI/SessionAPI/ParticipantAPI/TableauBlancData/TokenResponse, utilitaires getEtatInfo/formatDuree/formatDate/getInitiales
- `usePartenariat` — API client pour les demandes de partenariat (soumettreDemande multipart avec logo + document légal). Constantes TYPES_PARTENARIAT/TYPES_ORGANISATION/PAYS_PARTENARIAT. Interfaces PartenariatAPI/DemandePartenariatBody

**Mock data layer** (`app/mocks/`, 22 files): Fichiers TypeScript de données fictives avec interfaces, tableaux et fonctions async simulant la latence réseau. Lors de l'intégration backend, remplacer les imports mock par des appels API.

> **⚠️ Source de vérité** : Le schéma SQL (`uafricas_backend/doc/bd/schema.sql` et `schemas/*.sql`) prime sur les données mock. Adapter les interfaces frontend au schéma SQL, pas l'inverse.

**Icons**: FontAwesome registered globally via `app/plugins/fontawesome.ts`. To add a new icon, import it and add it to the `library.add()` call.

### Backend

Actix-Web 4 server with modular architecture (`config.rs`, `errors.rs`, `models/`, `handlers/`, `routes.rs`). Uses a generic `ApiResponse<T>` wrapper for JSON responses. Routes configured in `routes::configurer_routes()`.

**Endpoints API implémentés** :
- `GET /api/health` — Health check
- `POST /api/auth/inscription` — Inscription (nom, prenom, email, mot_de_passe, confirmation) → InscriptionResponse (message + email). Compte créé en état `en_attente`, email de vérification envoyé via SMTP
- `POST /api/auth/connexion` — Connexion (email, mot_de_passe) → AuthResponse
- `POST /api/auth/deconnexion` — Déconnexion (révoque refresh token)
- `GET /api/auth/moi` — Profil utilisateur connecté (Bearer token requis)
- `POST /api/auth/rafraichir` — Rafraîchir les tokens (rotation refresh token)
- `POST /api/auth/verifier-email` — Vérifier l'email avec token (active le compte, retourne AuthResponse avec tokens)
- `POST /api/auth/renvoyer-verification` — Renvoyer l'email de vérification (réponse générique anti-énumération)
- `GET /api/livres?recherche=&type_document=&page=&par_page=` — Liste paginée des livres
- `GET /api/livres/{id}` — Détail d'un livre (incrémente vues)
- `POST /api/livres` — Création multipart (image couverture + PDF + métadonnées)
- `DELETE /api/livres/{id}` — Suppression douce
- `GET /api/bibliotheques-humaines?recherche=&specialite=&pays=&page=&par_page=` — Liste paginée des bibliothèques humaines (filtres spécialité/pays/recherche)
- `GET /api/bibliotheques-humaines/{id}` — Détail d'une bibliothèque humaine
- `POST /api/bibliotheques-humaines/inscription` — Inscription comme bibliothèque humaine (JWT requis, liste de spécialités)
- `GET /api/bibliotheques-humaines/specialites` — Liste des spécialités disponibles
- `GET /api/centres-culturels?recherche=` — Liste des centres culturels actifs (recherche optionnelle par nom/ville)
- `GET /api/centres-culturels/{id}` — Détail d'un centre avec ses programmations
- `GET /api/centres-culturels/{centre_id}/programmations/{id}` — Détail d'une programmation avec info centre
- `GET /api/codimoi?type=&recherche=&pays=&page=&par_page=` — Liste paginée des posts Codi-Moi (filtres par type, recherche, pays)
- `GET /api/codimoi/{id}` — Détail d'un post Codi-Moi avec hashtags et auteur
- `POST /api/codimoi` — Création d'un post (type, contenu, explication, pays, hashtags, couleur_fond)
- `GET /api/gouvernance/stats` — Statistiques gouvernance (count factcheck, badhabits, ideaforces, total_likes)
- `GET /api/gouvernance/contributions?type=&page=&par_page=` — Liste paginée des contributions (UNION factcheck + bad_habit + idea_force avec auteur et pays)
- `GET /api/annonces?recherche=&type_operation=&categorie=&prix_min=&prix_max=&tri=&page=&par_page=` — Liste paginée des annonces (filtres multiples, tri, recherche textuelle, JOINs catégorie/auteur/média/pays)
- `GET /api/annonces/{id}` — Détail d'une annonce (incrémente vues, médias, pays multiples, info auteur)
- `GET /api/evenements?recherche=&format=&pays=&annee=&page=&par_page=` — Liste paginée des événements (filtres format/pays/année/recherche, statut calculé depuis etat+dates)
- `GET /api/evenements/{id}` — Détail d'un événement (organisateur, nombre inscrits, est_inscrit pour user connecté)
- `POST /api/evenements` — Création multipart (image couverture + métadonnées, résolution pays_id, slug auto)
- `POST /api/evenements/{id}/inscription` — Inscription à un événement (JWT requis, ON CONFLICT upsert)
- `GET /api/projets?recherche=&pays=&budget_max=&duree=&tri=&page=&par_page=` — Liste paginée des projets publics (filtres pays/budget/durée/recherche/tri, JOIN pays/utilisateur, sous-requête image couverture)
- `GET /api/projets/{id}` — Détail d'un projet (documents associés, objectifs parsés, JOIN pays/utilisateur)
- `POST /api/projets` — Création multipart (image couverture + métadonnées, résolution pays_id, slug auto, JWT requis, etat='soumis')
- `GET /api/projets/statistiques` — Statistiques des projets (total, validés, en cours, terminés via COUNT FILTER)
- `GET /api/experts?recherche=&domaine=&pays=&situation=&tri=&page=&par_page=` — Liste paginée des experts validés (filtres multiples, tri, recherche textuelle, JOIN utilisateur/pays)
- `GET /api/experts/{id}` — Détail d'un expert (par utilisateur_id, JOIN utilisateur/pays)
- `POST /api/experts/candidature` — Candidature expert (JWT requis, domaine, biographie, expérience, situations)
- `GET /api/fiches-pays?recherche=&region=&page=&par_page=` — Liste paginée des fiches pays (filtres recherche/region, JOIN shared.pays)
- `GET /api/fiches-pays/regions` — Liste des régions disponibles (calculées depuis les codes ISO)
- `GET /api/fiches-pays/{id}` — Détail d'une fiche pays (par UUID, code ISO ou nom, avec langues et ethnies)
- `GET /api/stations-radio?recherche=&type_station=&pays=&genre=&page=&par_page=` — Liste paginée des stations radio (filtres type/pays/genre/recherche)
- `GET /api/stations-radio/{id}` — Détail d'une station radio
- `GET /api/stations-radio/pays` — Liste des pays ayant des stations
- `GET /api/stations-radio/genres` — Liste des genres musicaux disponibles
- `POST /api/stations-radio` — Création d'une station (JWT requis, résolution pays_id par nom)
- `GET /api/television/chaines?recherche=&categorie=&pays=&page=&par_page=` — Liste paginée des chaînes TV (filtres catégorie/pays/recherche)
- `GET /api/television/chaines/{id}` — Détail d'une chaîne TV
- `POST /api/television/chaines` — Création d'une chaîne TV (JWT requis, résolution pays_id par nom)
- `GET /api/television/programmes-vedettes?recherche=&pays=&page=&par_page=` — Liste paginée des programmes TV à la une (table programme_radio_tele WHERE type='tele')
- `GET /api/television/programmes-vedettes/{id}` — Détail d'un programme TV vedette
- `POST /api/television/programmes-vedettes` — Création d'un programme TV (JWT requis)
- `GET /api/television/pays` — Liste des pays avec contenu TV (UNION chaînes + programmes)
- `GET /api/television/categories` — Liste des catégories de chaînes TV disponibles
- `GET /api/television/stats` — Statistiques TV (nombre chaînes, pays, programmes, en direct)
- `GET /api/africantives?recherche=&domaine=&pays=&tri=&page=&par_page=` — Liste paginée des initiatives africaines (filtres domaine/pays/recherche/tri)
- `GET /api/africantives/{id}` — Détail d'une initiative africaine
- `POST /api/africantives` — Création multipart (image couverture + métadonnées, résolution domaine_id/pays_id, slug auto, JWT requis)
- `GET /api/africantives/domaines` — Liste des domaines disponibles (avec africantives publiées)
- `GET /api/africantives/pays` — Liste des pays avec des africantives publiées
- `GET /api/afrolang/salles?recherche=&langue=&page=&par_page=` — Liste paginée des salles publiques actives (filtres langue/recherche, sous-requêtes COUNT salles privées/sessions en cours)
- `GET /api/afrolang/salles/{id}` — Détail d'une salle avec modérateur et salles privées associées
- `POST /api/afrolang/salles` — Création multipart salle publique (JWT + Admin, image couverture + métadonnées, slug auto)
- `PUT /api/afrolang/salles/{id}` — Modification salle publique (JWT + Admin, titre/description/langue/modérateur)
- `DELETE /api/afrolang/salles/{id}` — Soft delete salle publique (JWT + Admin, actif=false)
- `GET /api/afrolang/salles/{salle_id}/privees?recherche=&page=&par_page=` — Salles privées d'une salle publique (JOIN créateur/salle, sous-requête session en cours)
- `GET /api/afrolang/salles-privees/{id}` — Détail salle privée avec sessions associées (est_protegee, pas de code_acces en clair)
- `POST /api/afrolang/salles/{salle_id}/privees` — Création salle privée (JWT requis, titre/description/code_acces/max_participants)
- `PUT /api/afrolang/salles-privees/{id}` — Modification salle privée (JWT créateur uniquement)
- `DELETE /api/afrolang/salles-privees/{id}` — Soft delete salle privée (JWT créateur uniquement)
- `GET /api/afrolang/salles-privees/{sp_id}/sessions?etat=&page=&par_page=` — Sessions d'une salle privée (filtre par état)
- `GET /api/afrolang/sessions/{id}` — Détail session avec participants et modérateur (JOIN utilisateur)
- `POST /api/afrolang/salles-privees/{sp_id}/sessions` — Planifier session (JWT modérateur, date/max_participants/tableau_blanc)
- `PUT /api/afrolang/sessions/{id}/demarrer` — Démarrer session (JWT modérateur, vérifie état planifiée, ajoute participant modérateur)
- `PUT /api/afrolang/sessions/{id}/terminer` — Terminer session (JWT modérateur, calcul durée, ferme participants actifs)
- `POST /api/afrolang/sessions/{id}/rejoindre` — Rejoindre session (JWT, vérifie code_acces + max_participants, ON CONFLICT reconnexion)
- `POST /api/afrolang/sessions/{id}/quitter` — Quitter session (JWT, calcul durée_secondes participant)
- `POST /api/afrolang/sessions/{id}/token` — Générer token LiveKit (JWT requis, vérifie code_acces + max_participants, enregistre participant, retourne token/room_name/livekit_url/is_moderator)
- `GET /api/afrolang/sessions/{id}/tableau-blanc` — Obtenir le snapshot du tableau blanc (donnees JSONB + version)
- `PUT /api/afrolang/sessions/{id}/tableau-blanc` — Sauvegarder le snapshot (JWT modérateur, UPSERT avec incrémentation version)
- `DELETE /api/afrolang/sessions/{id}/tableau-blanc` — Effacer le tableau blanc (JWT modérateur, reset donnees à {})
- `GET /api/afrolang/stats` — Statistiques globales Afrolang (salles, privées, sessions en cours/terminées, participants uniques)
- `GET /api/afrolang/langues` — Liste des langues disponibles (DISTINCT depuis salles actives)

**Authentification** : JWT (HS256) access token (15 min) + refresh token (7 jours, SHA-256 hashé en BDD dans `iam.refresh_token`). Mot de passe hashé avec bcrypt (cost 12). Module `jwt.rs` pour génération/validation tokens. **Vérification email** : À l'inscription, compte créé en `etat='en_attente'`. Token de vérification SHA-256 hashé en BDD (`iam.token_verification_email`, expire 24h). Email envoyé via SMTP (lettre, STARTTLS port 587). Clic sur le lien → compte activé (`etat='actif'`, `email_verifie=true`) + auto-login. Module `email.rs` pour envoi SMTP asynchrone.

**Dépendances backend** : actix-web 4, actix-cors, actix-multipart, actix-files, sqlx (PostgreSQL), uuid, chrono, dotenvy, serde, sanitize-filename, bcrypt, jsonwebtoken, sha2, rand, livekit-api, lettre (SMTP email).

**Upload fichiers** : Stockage local dans `./uploads/couvertures/` et `./uploads/documents/`, servis statiquement via actix-files sur `/uploads/`.

**Configuration** : Variables d'environnement dans `.env` : `DATABASE_URL`, `UPLOAD_DIR`, `FRONTEND_URL`, `HOST`, `PORT`, `RUST_LOG`, `JWT_SECRET`, `JWT_EXPIRATION_MINUTES`, `REFRESH_EXPIRATION_DAYS`, `LIVEKIT_URL` (default: ws://localhost:7880), `LIVEKIT_API_KEY` (default: devkey), `LIVEKIT_API_SECRET` (default: secret), `SMTP_HOST` (required), `SMTP_PORT` (default: 587), `SMTP_USERNAME` (required), `SMTP_PASSWORD` (required), `SMTP_FROM_EMAIL`, `SMTP_FROM_NAME` (default: UAfricas), `EMAIL_VERIFICATION_EXPIRATION_HOURS` (default: 24).

**Database** : PostgreSQL 16 via Docker (`docker-compose.yml` à la racine). Le schéma SQL complet est dans `uafricas_backend/doc/bd/` avec un fichier orchestrateur `schema.sql` qui inclut 15 fichiers via `\ir` (dans `schemas/`). Le script `docker-init.sh` lance l'init automatiquement au premier `docker compose up`.

10 schemas PostgreSQL (bounded contexts, microservice-ready) : `shared`, `iam`, `marketplace`, `exchange`, `innovation`, `culture`, `afrolang`, `media_content`, `governance`, `country_profile`. Conventions BDD : UUID v4 PKs, soft deletion (`deleted_at`), `TIMESTAMPTZ`, snake_case français, enums PostgreSQL, full-text search (`TSVECTOR` + GIN indexes).

## Conventions

- **Language**: Code comments, variable names, mock data, and UI text are in **French** (aligned with business domain)
- **Tailwind CSS v4 + daisyUI v5**: CSS-first config via `@theme` directive in `app/assets/css/main.css` (no `tailwind.config.ts`). Vite plugin `@tailwindcss/vite` in `nuxt.config.ts`. Custom colors `custom-chocolat` (#A54A1C), `custom-green` (#228B22), `custom-gray`. Fonts: Oswald (display/headings), Open Sans (body). Extended spacing utilities up to `164` (64rem). Custom background images for cultural sections. daisyUI loaded via `@plugin "daisyui"` in CSS.
- **Component naming**: PascalCase Vue components, feature-scoped directories. Hero/Card/Filters/Modal pattern per feature.
- **Mock pattern**: Each mock file exports TypeScript interfaces, a data array, and helper functions (getById, filter, create empty form). Async functions use `delay()` to simulate latency.

## Infrastructure

- **Docker** : `docker-compose.yml` à la racine avec 3 services (postgres, adminer, livekit) et 1 volume (pgdata). LiveKit SFU sur ports 7880 (WebSocket), 7881 (HTTP API), 7882 (WebRTC TCP), 50000-50100/udp (WebRTC UDP). Config dans `livekit.yaml` à la racine.
- **Variables d'env** : `.env` à la racine (gitignored), contient les credentials PostgreSQL et pgAdmin
- **Init BDD** : `uafricas_backend/doc/bd/docker-init.sh` exécute `schema.sql` au premier lancement du conteneur

## LSP & Diagnostics

**LSP installés** :
- **rust-analyzer** (via `rustup component`) — Diagnostics Rust en temps réel (types, imports, lifetime, erreurs de compilation)
- **Volar** (extension VS Code) — Diagnostics Vue 3 / TypeScript / Tailwind CSS

**Utilisation par Claude Code** : Après chaque création ou modification de fichier, utiliser `getDiagnostics` pour vérifier les erreurs avant de passer à l'étape suivante. Cela évite le cycle écrire → compiler → corriger → recompiler et détecte les problèmes de typage, imports manquants et incompatibilités d'interfaces immédiatement.

## Parallel Sub-agents Strategy

Use multiple sub-agents in parallel for efficiency:
- Search frontend + backend simultaneously
- Explore multiple files/folders at the same time
- Run tests + verifications in parallel after modifications
- **Avant de créer un nouveau composant** : Toujours lancer un sous-agent pour vérifier si un composant similaire existe déjà dans `app/components/` (rechercher par nom et par fonctionnalité). Évite les redondances et favorise la réutilisation.

## Auto-maintenance de ce fichier

**Ce fichier CLAUDE.md doit être mis à jour automatiquement** lors de changements importants :
- Ajout/suppression d'un service Docker, d'une dépendance majeure, ou d'un outil
- Nouveau module Nuxt, nouveau store Pinia, nouveau composable
- Nouvel endpoint API backend ou nouveau schema BDD
- Changement de commandes de dev (build, test, lint)
- Modification de conventions ou de patterns architecturaux
- Ajout de CI/CD, linting, ou testing

Après chaque modification significative du projet, vérifier si CLAUDE.md reflète toujours l'état actuel et le mettre à jour si nécessaire.