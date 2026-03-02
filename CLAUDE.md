# CLAUDE.md

## Project Overview
UAfricas — plateforme panafricaine de développement durable. Monorepo :
- **`uafricas_frontend/`** — Nuxt 4 (Vue 3 SSR) + TypeScript + Tailwind CSS v4 + daisyUI v5 + Pinia
- **`uafricas_backend/`** — Rust + Actix-Web 4

## Dev Commands

| Commande | Description |
|----------|-------------|
| `pnpm dev` | Frontend dev (port 3000) |
| `pnpm build / generate / preview` | Build prod / SSG / preview |
| `RUST_LOG=info cargo run` | Backend (port 8080, recompile auto) |
| `kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run` | Restart backend propre |
| `docker compose up -d / down / down -v` | PostgreSQL + Adminer + LiveKit |

> Toujours tuer l'ancien processus avant relancer le backend (port 8080 occupé sinon).

**BDD** : PostgreSQL `localhost:5432` (user: `uafricas`, db: `africans_db`). Adminer: `http://localhost:8088`. Schema SQL auto-init via `docker-init.sh`.

**No linting, testing, or CI/CD configured yet.**

## Backend Env Vars
Required: `DATABASE_URL`, `JWT_SECRET`, `SMTP_HOST`, `SMTP_USERNAME`, `SMTP_PASSWORD`.
Defaults: `HOST` (127.0.0.1), `PORT` (8080), `UPLOAD_DIR` (./uploads), `FRONTEND_URL` (http://localhost:3000), `JWT_EXPIRATION_MINUTES` (15), `REFRESH_EXPIRATION_DAYS` (7), `SMTP_PORT` (587), `SMTP_FROM_EMAIL` (=SMTP_USERNAME), `SMTP_FROM_NAME` (UAfricas), `EMAIL_VERIFICATION_EXPIRATION_HOURS` (24), `LIVEKIT_URL` (ws://localhost:7880), `LIVEKIT_API_KEY` (devkey), `LIVEKIT_API_SECRET` (secret).

## Architecture Frontend

- **Routing** : Nuxt 4 file-based, `app/pages/`, `[id].vue` dynamique. Layouts: `default.vue` (NavBar+Footer), `auth.vue` (minimal).
- **Composants** : `app/components/` feature-based (marche/, experts/, evenements/, forums/, universite/, common/, layout/). PascalCase, pattern Hero/Card/Filters/Modal.
- **Store** : Pinia unique `app/stores/user.ts` — Utilisateur (id, nom, prenom, email, roles, etat), accessToken (mémoire), refreshToken (localStorage). Getters: fullName, displayName, isAdmin, isValidated.
- **Icons** : FontAwesome global via `app/plugins/fontawesome.ts`, ajouter dans `library.add()`.
- **Mocks** : `app/mocks/` (22 fichiers) — interfaces TS + données + helpers async avec delay(). **Source de vérité = schéma SQL**, adapter interfaces au SQL.

### Composables (`app/composables/`)
Pattern commun : API via $fetch, filtres/pagination, mapping DB↔frontend. Un composable par domaine (public et admin). Nommage: `useXxx` (public), `useAdminXxx` (admin, base: useAdmin avec adminFetch/listerPagine/pagination/sort).

**Globaux (utilisés sur toutes les pages)** : useAuth (authentification), useAOS (animations scroll).

## Architecture Backend

Actix-Web 4 modulaire : `config.rs`, `errors.rs`, `models/`, `handlers/`, `routes.rs`, `jwt.rs`. Pattern: `ApiResponse<T>` wrapper, `COLONNES` const, `FromRow` structs, DTO Response séparés.

### API Routes (voir `src/routes.rs`)
Handlers dans `src/handlers/` (un fichier/domaine). Pattern: CRUD + filtres/pagination, multipart upload, JWT mutations.

| Domaine | Routes | Endpoints principaux |
|---------|--------|---------------------|
| **Auth** | auth | inscription/connexion/JWT/refresh |
| **Public** | livres, centres-culturels, codimoi, evenements, gouvernance, annonces/marché, television | CRUD + filtres |
| **Admin IAM** | ~30 | utilisateurs (CRUD+état+rôles+spécialités+permissions), organisations, partenariats, roles (CRUD+permissions), permissions |
| **Admin Référentiels** | ~28 | pays (+continent), domaines, categories (+contexte/parent/enfants), tags, medias, specialites |
| **Admin Programmes** | ~10 | programmes (CRUD+état+candidatures), candidatures (liste/détail/statut) |
| **Admin Marché** | ~13 | annonces (CRUD+modération+pays+médias+réordonnement), annonces-favoris (liste+stats) |
| **Admin Innovation** | ~22 | innovations (CRUD+modération+médias), projets (workflow soumis→en_revue→approuvé/rejeté+documents), africantives (CRUD+modération) |
| **Admin Culture** | ~20 | centres-culturels (CRUD+membres president/vice/resp_comm/membre), programmations (+centre/mode), codimoi (CRUD+tags+commentaires arborescents+réactions) |
| **Admin Gouvernance** | ~26 | factcheck (CRUD+commentaires+réactions), bad-habits (CRUD+médias/preuves), idea-forces (CRUD+médias) |
| **Admin Médias** | ~48 | stations-radio, chaines-tv, programmes-media (CRUD), evenements/mooc (CRUD+état+inscriptions+stats), livres (CRUD+état+tags) |
| **Admin Audit** | 2 | liste paginée (filtres action/user/table/date/IP) + détail (before/after JSONB) |
| **Admin Profils Pays** | ~43 | fiches pays + 8 sous-entités (regions, groupes-ethniques, alliances, contes, sites-touristiques, secteurs, saisons, liens-interethniques) + modération contributions |

**Fichiers admin** : handlers `src/handlers/admin/` et models `src/models/admin/` — sous-modules: utilisateurs, organisations, partenariats, roles, pays, domaines, categories, tags, medias, specialites, programmes, candidatures, annonces, annonces_favoris, innovations, projets_admin, africantives_admin, centres_culturels, programmations, codimoi_admin, gouvernance, radio_tele, evenements, mooc, livres, audit, profils_pays.

**Services** : `src/services/` — audit.rs (`log_action` non-bloquant, `extraire_ip`, `extraire_user_agent`). ~100 mutations instrumentées auto.

### Auth
JWT HS256 access (15min) + refresh (7j, SHA-256 hashé dans `iam.refresh_token`). Bcrypt cost 12. Vérification email: inscription → `etat='en_attente'` → token SHA-256 dans `iam.token_verification_email` (24h) → SMTP (lettre, STARTTLS 587) → clic = `etat='actif'` + `email_verifie=true` + auto-login. Modules: `jwt.rs`, `email.rs`.

### Deps backend
actix-web 4, actix-cors, actix-multipart, actix-files, sqlx (PostgreSQL), uuid, chrono, dotenvy, serde, sanitize-filename, bcrypt, jsonwebtoken, sha2, rand, livekit-api, lettre.

### Upload
Stockage local `./uploads/couvertures/` et `./uploads/documents/`, servis via actix-files sur `/uploads/`.

### Database
PostgreSQL 16 Docker. Schema SQL: `uafricas_backend/doc/bd/schema.sql` (orchestrateur, 15 fichiers via `\ir` dans `schemas/`). Init auto via `docker-init.sh`.

10 schemas bounded-context: `shared`, `iam`, `marketplace`, `exchange`, `innovation`, `culture`, `afrolang`, `media_content`, `governance`, `country_profile`.

Conventions BDD: UUID v4 PKs, soft deletion (`deleted_at`), TIMESTAMPTZ, snake_case français, enums PostgreSQL, full-text search (TSVECTOR + GIN).

## Conventions
- **Langue** : code, commentaires, variables, UI en **français**
- **Tailwind v4 + daisyUI v5** : CSS-first via `@theme` dans `app/assets/css/main.css` (pas de tailwind.config.ts). Plugin Vite `@tailwindcss/vite`. daisyUI via `@plugin "daisyui"`. Couleurs: `custom-chocolat` (#A54A1C), `custom-green` (#228B22), `custom-gray`. Fonts: Oswald (titres), Open Sans (body). Spacing jusqu'à `164` (64rem).
- **Migration Tailwind v3 → v4** : Ce projet a été migré de Tailwind CSS v3 vers v4. Des résidus v3 non migrés peuvent subsister (ex: classes dépréciées, anciennes syntaxes `@apply`, directives `@screen`, `theme()` au lieu de variables CSS, etc.). Si des résidus v3 sont détectés lors du travail sur un fichier, les migrer vers la syntaxe v4 avec un sous-agent dédié.
- **Mock pattern** : interfaces TS + tableau + helpers (getById, filter, formulaire vide) + async delay().

## Infrastructure

### Docker Dev
`docker-compose.yml` : 3 services (postgres, adminer, livekit) + 1 volume (pgdata). LiveKit: 7880 (WS), 7881 (HTTP), 7882 (TCP), 50000-50100/udp. Config: `livekit.yaml`. `.env` gitignored.

### Docker Prod
`docker-compose.prod.yml` : 6 services (postgres 16-alpine, backend, frontend, nginx, livekit, adminer optionnel). 2 volumes (pgdata, uploads_data). Nginx: `nginx/nginx.conf`, HTTPS Let's Encrypt, reverse proxy → frontend:3000 + backend:8080, rate limit API 30r/s + auth 5r/s, gzip, HSTS. Domaine: `www.africans-world.org`.

### Déploiement
`deploy.sh` → VPS `root@161.97.92.63:/opt/uafricas` via SSH+Docker.
Commandes: `setup`, `deploy`, `update`, `rebuild`, `status`, `logs [svc]`, `restart [svc]`, `stop`, `ssl`, `backup`, `connect`.

## LSP & Diagnostics
- **rust-analyzer** + **Volar** (VS Code)
- Utiliser `getDiagnostics` après chaque modification de fichier pour détecter erreurs de typage/imports immédiatement.

## Parallel Sub-agents Strategy
- Recherche frontend + backend simultanée
- Explorer plusieurs fichiers/dossiers en parallèle
- Tests + vérifications en parallèle après modifications
- Avant nouveau composant: vérifier si similaire existe dans `app/components/`

## Auto-maintenance
Mettre à jour ce fichier lors de: ajout/suppression service Docker ou dépendance majeure, nouveau composable/store/module, nouvel endpoint API ou schema BDD, changement commandes dev, conventions, CI/CD.

## Test Users
- **Admin** : `admin@test.com` / `Test1234`
- **Standard** : `user@test.com` / `Test1234`

## Active Technologies
- Rust (Edition 2024) + TypeScript (Nuxt 4 / Vue 3) + Actix-Web 4, sqlx (PostgreSQL), Nuxt 4, Pinia, Tailwind CSS v4 (001-retrouve-amis)
- PostgreSQL 16 — nouveau schema `retrouve_amis` ajouté aux 10 existants (001-retrouve-amis)
- Rust (Edition 2024) + TypeScript (Nuxt 4 / Vue 3 SSR) + Actix-Web 4, sqlx (PostgreSQL), Nuxt 4, Pinia, Tailwind CSS v4 (002-partage-avis-recherche)
- PostgreSQL 16, schema `retrouve_amis` existant (6 tables + 1 fonction PL/pgSQL) (002-partage-avis-recherche)

## Recent Changes
- 001-retrouve-amis: Added Rust (Edition 2024) + TypeScript (Nuxt 4 / Vue 3) + Actix-Web 4, sqlx (PostgreSQL), Nuxt 4, Pinia, Tailwind CSS v4
