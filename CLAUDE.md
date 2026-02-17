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

**Endpoints API** : Voir `uafricas_backend/src/routes.rs` pour la liste complète des routes. Les handlers sont dans `uafricas_backend/src/handlers/` (un fichier par domaine : auth, livres, evenements, projets, experts, afrolang, etc.). Pattern commun : CRUD avec filtres/pagination, upload multipart, JWT pour les mutations.

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