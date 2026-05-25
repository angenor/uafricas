# CLAUDE.md

## Project Overview
UAfricas — plateforme panafricaine de développement durable. Monorepo :
- **`uafricas_frontend/`** — Nuxt 4 (Vue 3 SSR) + TypeScript + Tailwind CSS v4 + daisyUI v5 + Pinia
- **`uafricas_backend/`** — Rust (Edition 2024) + Actix-Web 4

## Dev Commands

| Commande | Description |
|----------|-------------|
| `pnpm dev` | Frontend dev (port 3000) |
| `pnpm build / generate / preview` | Build prod / SSG / preview |
| `RUST_LOG=info cargo run` | Backend (port 8082, recompile auto) |
| `kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run` | Restart backend propre |
| `docker compose up -d / down / down -v` | PostgreSQL + Adminer + LiveKit |

> Toujours tuer l'ancien processus avant relancer le backend (port 8082 occupé sinon).

**BDD** : PostgreSQL `localhost:5432` (user: `uafricas`, db: `africans_db`). Adminer: `http://localhost:8088`. Schema SQL auto-init via `docker-init.sh`.

**No linting, testing, or CI/CD configured yet.**

## Backend Env Vars
Required: `DATABASE_URL`, `JWT_SECRET`, `SMTP_HOST`, `SMTP_USERNAME`, `SMTP_PASSWORD`.
Defaults: `HOST` (127.0.0.1), `PORT` (8080), `UPLOAD_DIR` (./uploads), `FRONTEND_URL` (http://localhost:3000), `JWT_EXPIRATION_MINUTES` (15), `REFRESH_EXPIRATION_DAYS` (7), `SMTP_PORT` (587), `SMTP_FROM_EMAIL` (=SMTP_USERNAME), `SMTP_FROM_NAME` (UAfricas), `EMAIL_VERIFICATION_EXPIRATION_HOURS` (24), `LIVEKIT_URL` (ws://localhost:7880), `LIVEKIT_API_KEY` (devkey), `LIVEKIT_API_SECRET` (secret).

## Architecture Frontend

- **Routing** : Nuxt 4 file-based, `app/pages/`, `[id].vue` dynamique. Layouts: `default.vue` (NavBar+Footer), `auth.vue` (minimal).
- **Composants** : `app/components/` feature-based (marche/, experts/, evenements/, forums/, universite/, social/, afrolang/, common/, layout/). PascalCase, pattern Hero/Card/Filters/Modal.
- **Store** : Pinia unique `app/stores/user.ts` — Utilisateur (id, nom, prenom, email, roles, etat), accessToken (mémoire), refreshToken (localStorage). Getters: fullName, displayName, isAdmin, isValidated.
- **Icons** : FontAwesome global via `app/plugins/fontawesome.ts`, ajouter dans `library.add()`.
- **Mocks** : `app/mocks/` — interfaces TS + données + helpers async. **Source de vérité = schéma SQL**, adapter interfaces au SQL.

### Composables (`app/composables/`)
Pattern : API via $fetch, filtres/pagination, mapping DB↔frontend. Un composable par domaine (public et admin). Nommage: `useXxx` (public), `useAdminXxx` (admin, base `useAdmin` avec adminFetch/listerPagine/pagination/sort). Globaux : `useAuth`, `useAOS`.

## Architecture Backend

Actix-Web 4 modulaire : `config.rs`, `errors.rs`, `models/`, `handlers/`, `routes.rs`, `jwt.rs`. Pattern: `ApiResponse<T>` wrapper, `COLONNES` const, `FromRow` structs, DTO Response séparés. Routes complètes : voir `src/routes.rs`. Un fichier handler/model par domaine ; admin sous `src/handlers/admin/` et `src/models/admin/`.

**Domaines principaux** : Auth (JWT/refresh), Public (livres, centres-culturels, codimoi, evenements, gouvernance, marché, television, vidafrica), Admin (IAM, référentiels, programmes, marché, innovation, culture, gouvernance, médias, audit, profils pays, vidafrica, bibliothèques humaines, expertise, afrolang salles), Social (amitié + messagerie, schéma `social`).

**Services** : `src/services/` — `audit.rs` (`log_action` non-bloquant, ~100 mutations instrumentées), `messagerie_sse.rs`, `livekit_moderation.rs`, validations diverses.

### Auth
JWT HS256 access (15min) + refresh (7j, SHA-256 hashé dans `iam.refresh_token`). Bcrypt cost 12. Vérification email: inscription → `etat='en_attente'` → token SHA-256 (`iam.token_verification_email`, 24h) → SMTP STARTTLS 587 → clic = `etat='actif'` + `email_verifie=true` + auto-login. Modules: `jwt.rs`, `email.rs`.

### Deps backend
actix-web 4, actix-cors, actix-multipart, actix-files, sqlx (PostgreSQL), uuid, chrono, dotenvy, serde, sanitize-filename, bcrypt, jsonwebtoken, sha2, rand, livekit-api, lettre, futures-util, tokio, image.

### Upload
Stockage local sous `./uploads/` (`couvertures/`, `documents/`, `videos/`, `vignettes/`, `afrolang/ressources/`, `opportunite-afrique/photos/`), servi via actix-files sur `/uploads/`.

### Database
PostgreSQL 16 Docker. Schema SQL: `uafricas_backend/doc/bd/schema.sql` (orchestrateur via `\ir` dans `schemas/`). Init auto via `docker-init.sh`.

Schemas bounded-context : `shared`, `iam`, `marketplace`, `exchange`, `innovation`, `culture`, `afrolang`, `media_content`, `governance`, `country_profile`, `arbre_genealogique`, `social`.

Conventions BDD: UUID v4 PKs, soft deletion (`deleted_at`), TIMESTAMPTZ, snake_case français, enums PostgreSQL, full-text search (TSVECTOR + GIN).

## Conventions
- **Langue** : code, commentaires, variables, UI en **français** avec accents (é, è, ê, à, ç, ù) obligatoires.
- **Nommage fichiers/dossiers** : PAS d'accents ni de caractères spéciaux (encodage SSH/Docker en prod). Uniquement `[a-z0-9_-]`.
- **Pages publiques = Tailwind v4 pur** ; daisyUI réservé au back-office admin (Principe VI).
- **Tailwind v4 + daisyUI v5** : CSS-first via `@theme` dans `app/assets/css/main.css` (pas de `tailwind.config.ts`). Plugin Vite `@tailwindcss/vite`. daisyUI via `@plugin "daisyui"`. Couleurs: `custom-chocolat` (#A54A1C), `custom-green` (#228B22), `custom-gray`. Fonts: Oswald (titres), Open Sans (body). Spacing jusqu'à `164` (64rem).
- **Migration v3 → v4** : projet migré ; des résidus v3 peuvent subsister. Si détectés, migrer vers v4 (sous-agent dédié).
- **Mock pattern** : interfaces TS + tableau + helpers (getById, filter, formulaire vide) + async delay().

## Infrastructure

- **Docker Dev** : `docker-compose.yml` — postgres, adminer, livekit + volume pgdata. LiveKit: 7880 (WS), 7881 (HTTP), 7882 (TCP), 50000-50100/udp. Config `livekit.yaml`. `.env` gitignored.
- **Docker Prod** : `docker-compose.prod.yml` — 6 services (postgres, backend, frontend, nginx, livekit, adminer optionnel) + 2 volumes (pgdata, uploads_data). Nginx HTTPS Let's Encrypt, reverse proxy frontend:3000 + backend:8080, rate limit API 30r/s + auth 5r/s, gzip, HSTS. Domaine `www.africans-world.org`.
- **Déploiement** : `deploy.sh` → VPS `root@161.97.92.63:/opt/uafricas` via SSH+Docker. Commandes: `setup`, `deploy`, `update`, `rebuild`, `status`, `logs [svc]`, `restart [svc]`, `stop`, `ssl`, `backup`, `connect`. Migrations BD manuelles via SSH+psql.

## LSP & Diagnostics
- **rust-analyzer** + **Volar** (VS Code). Utiliser `getDiagnostics` après chaque modification de fichier.

## Parallel Sub-agents
Recherche frontend + backend simultanée, exploration multi-fichiers en parallèle, tests/vérifications en parallèle après modifications. Avant nouveau composant : vérifier si similaire existe dans `app/components/`.

## Auto-maintenance
Mettre à jour ce fichier lors de : ajout/suppression service Docker ou dépendance majeure, nouveau composable/store/module, nouvel endpoint API ou schema BDD, changement commandes dev, conventions, CI/CD.

## Test Users
- `test-admin@test.com` / `Test1234`
- `test-user@test.com` / `Test1234`

## Tech Stack par feature
Backend Rust Edition 2024 + Actix-Web 4 + sqlx (PostgreSQL) ; frontend Nuxt 4 / Vue 3 SSR + Pinia + Tailwind v4. Extensions notables : `pg_trgm` (matching arbres), LiveKit (afrolang temps réel + modération), SSE via `futures-util`/`tokio` (messagerie social), `image` crate (validation photos), `lettre` (SMTP), `@vue-flow/core` (visualisation arbre généalogique), `@excalidraw/excalidraw` (tableau blanc afrolang, MIT).

## Recent Changes (résumé)
Spécifications détaillées sous `specs/`. Historique des features livrées (consulter git log + specs pour le détail) :
- **social** (`001-demande-amitie`) : amitié entre membres + messagerie privée temps réel (SSE) via bouton flottant global. Schéma `social` (`schemas/29_social.sql`).
- **expertise** (`001-demande-expertise`) : candidature `/devenir-expert`, validation admin + email. Schéma `iam.expertise` étendu.
- **annuaire-membres** : `/profil` = annuaire public, `/profil/[id]` = page de détail unifiée (membre + biblio + expertise).
- **bibliothèques humaines** (`001-admin-biblio-humaine`) : workflow admin de validation. Schéma `iam` (`04b_iam_biblio_demande.sql`).
- **afrolang** : refonte salles (streaming public + privées par code bcrypt), pays d'origine, propositions communautaires + admins de salle, modération de session (permissions tableau blanc + spotlight), fermeture admin + historique modération, migration tableau blanc tldraw → Excalidraw.
- **afripulse** (`001-afripulse-contributions`) : enrichissement collaboratif fiches pays `/opportunite-afrique`. Schéma `country_profile` étendu (`11c_country_profile_afripulse.sql`).
- **vidafrica** (`004-vidafrica-sous-titres`) : vidéos + sous-titres. Schéma `media_content` étendu.
- **arbre généalogique** (`001-personnes-arbre` et suivantes) : schéma `arbre_genealogique`, visualisation @vue-flow, édition, matching inter-arbres (pg_trgm), recherche, collaboration/partage, notifications/suggestions.
