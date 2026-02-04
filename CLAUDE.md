# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

UAfricas is a pan-African platform for sustainable development. Monorepo with two independent projects:

- **`uafricas_frontend/`** — Nuxt 4 (Vue 3 SSR) + TypeScript + Tailwind CSS + Pinia
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

**Mock data layer** (`app/mocks/`, 22 files): The frontend is **not connected to any backend**. All data comes from TypeScript mock files that export interfaces, data arrays, and async utility functions with simulated network delays. When integrating with the real backend, replace mock imports with API calls — the interfaces serve as the data contract.

**Icons**: FontAwesome registered globally via `app/plugins/fontawesome.ts`. To add a new icon, import it and add it to the `library.add()` call.

### Backend

Minimal Actix-Web server with two endpoints: `GET /api/health` and `GET /api/`. Uses a generic `ApiResponse<T>` wrapper for JSON responses. Routes are configured in `configure_routes()`.

**Database design** is documented in `doc/bd/schema.sql` — a comprehensive PostgreSQL schema with 10 bounded contexts (schemas): `shared`, `iam`, `marketplace`, `exchange`, `innovation`, `culture`, `afrolang`, `media_content`, `governance`, `country_profile`. Designed monolith-first with microservice split in mind. Uses UUID v4 PKs, soft deletion (`deleted_at`), and `TIMESTAMPTZ` throughout.

## Conventions

- **Language**: Code comments, variable names, mock data, and UI text are in **French** (aligned with business domain)
- **Tailwind custom theme**: Custom colors `custom-chocolat` (#A54A1C), `custom-green` (#228B22), `custom-gray`. Fonts: Oswald (display/headings), Open Sans (body). Extended spacing utilities up to `164` (64rem). Many custom background images for cultural sections.
- **Component naming**: PascalCase Vue components, feature-scoped directories. Hero/Card/Filters/Modal pattern per feature.
- **Mock pattern**: Each mock file exports TypeScript interfaces, a data array, and helper functions (getById, filter, create empty form). Async functions use `delay()` to simulate latency.

## Parallel Sub-agents Strategy

Use multiple sub-agents in parallel for efficiency:
- Search frontend + backend simultaneously
- Explore multiple files/folders at the same time
- Run tests + verifications in parallel after modifications
