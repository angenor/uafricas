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
cargo build                    # Debug build
RUST_LOG=info cargo run        # Run with logging (http://127.0.0.1:8080)
cargo build --release          # Release build
```

Backend env vars: `HOST` (default: 127.0.0.1), `PORT` (default: 8080), `RUST_LOG` (info/debug/error).

### Database (Docker)

```bash
docker compose up -d              # Lancer PostgreSQL + pgAdmin
docker compose down               # Arrêter les services
docker compose down -v            # Arrêter + supprimer les données (reset complet)
docker compose logs postgres      # Voir les logs PostgreSQL
```

- **PostgreSQL** : `localhost:5432` (user: `uafricas`, db: `africans_db`)
- **pgAdmin** : `http://localhost:5050` (email: `admin@uafricas.dev`, mot de passe: `admin`)
- Config dans `.env` (gitignored). Le schéma SQL (`uafricas_backend/doc/bd/schema.sql`) s'initialise automatiquement au premier lancement.

### No linting, testing, or CI/CD configured yet.

## Architecture

### Frontend

**Nuxt 4 file-based routing** with pages in `app/pages/`. Dynamic routes use `[id].vue` syntax. Two layouts: `default.vue` (NavBar + Footer) and `auth.vue` (minimal, no chrome).

**Component organization** is feature-based under `app/components/` — each feature domain has its own directory (e.g., `marche/`, `experts/`, `evenements/`, `forums/`, `universite/`). Shared components live in `common/` and `layout/`.

**State management** uses a single Pinia store (`app/stores/user.ts`) for user authentication state with getters for fullName, displayName, isAdmin, isValidated.

**Composables** (`app/composables/`):
- `useAuth` — wraps mock login/logout/Google auth, exposes loading/error state and role checking
- `useAudioPlayer` — HTML5 audio controls for radio streaming (play/pause/volume/station switching)
- `useAOS` — initializes Animate On Scroll (1000ms duration, once, ease-out-cubic)

**Mock data layer** (`app/mocks/`, 22 files): Fichiers TypeScript de données fictives avec interfaces, tableaux et fonctions async simulant la latence réseau. Lors de l'intégration backend, remplacer les imports mock par des appels API.

> **⚠️ Source de vérité** : Le schéma SQL (`uafricas_backend/doc/bd/schema.sql` et `schemas/*.sql`) prime sur les données mock. Adapter les interfaces frontend au schéma SQL, pas l'inverse.

**Icons**: FontAwesome registered globally via `app/plugins/fontawesome.ts`. To add a new icon, import it and add it to the `library.add()` call.

### Backend

Minimal Actix-Web server with two endpoints: `GET /api/health` and `GET /api/`. Uses a generic `ApiResponse<T>` wrapper for JSON responses. Routes are configured in `configure_routes()`.

**Database** : PostgreSQL 16 via Docker (`docker-compose.yml` à la racine). Le schéma SQL complet est dans `uafricas_backend/doc/bd/` avec un fichier orchestrateur `schema.sql` qui inclut 15 fichiers via `\ir` (dans `schemas/`). Le script `docker-init.sh` lance l'init automatiquement au premier `docker compose up`.

10 schemas PostgreSQL (bounded contexts, microservice-ready) : `shared`, `iam`, `marketplace`, `exchange`, `innovation`, `culture`, `afrolang`, `media_content`, `governance`, `country_profile`. Conventions BDD : UUID v4 PKs, soft deletion (`deleted_at`), `TIMESTAMPTZ`, snake_case français, enums PostgreSQL, full-text search (`TSVECTOR` + GIN indexes).

## Conventions

- **Language**: Code comments, variable names, mock data, and UI text are in **French** (aligned with business domain)
- **Tailwind CSS v4 + daisyUI v5**: CSS-first config via `@theme` directive in `app/assets/css/main.css` (no `tailwind.config.ts`). Vite plugin `@tailwindcss/vite` in `nuxt.config.ts`. Custom colors `custom-chocolat` (#A54A1C), `custom-green` (#228B22), `custom-gray`. Fonts: Oswald (display/headings), Open Sans (body). Extended spacing utilities up to `164` (64rem). Custom background images for cultural sections. daisyUI loaded via `@plugin "daisyui"` in CSS.
- **Component naming**: PascalCase Vue components, feature-scoped directories. Hero/Card/Filters/Modal pattern per feature.
- **Mock pattern**: Each mock file exports TypeScript interfaces, a data array, and helper functions (getById, filter, create empty form). Async functions use `delay()` to simulate latency.

## Infrastructure

- **Docker** : `docker-compose.yml` à la racine avec 2 services (postgres, pgadmin) et 2 volumes (pgdata, pgadmin_data)
- **Variables d'env** : `.env` à la racine (gitignored), contient les credentials PostgreSQL et pgAdmin
- **Init BDD** : `uafricas_backend/doc/bd/docker-init.sh` exécute `schema.sql` au premier lancement du conteneur

## Parallel Sub-agents Strategy

Use multiple sub-agents in parallel for efficiency:
- Search frontend + backend simultaneously
- Explore multiple files/folders at the same time
- Run tests + verifications in parallel after modifications

## Auto-maintenance de ce fichier

**Ce fichier CLAUDE.md doit être mis à jour automatiquement** lors de changements importants :
- Ajout/suppression d'un service Docker, d'une dépendance majeure, ou d'un outil
- Nouveau module Nuxt, nouveau store Pinia, nouveau composable
- Nouvel endpoint API backend ou nouveau schema BDD
- Changement de commandes de dev (build, test, lint)
- Modification de conventions ou de patterns architecturaux
- Ajout de CI/CD, linting, ou testing

Après chaque modification significative du projet, vérifier si CLAUDE.md reflète toujours l'état actuel et le mettre à jour si nécessaire.
