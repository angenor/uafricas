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
| **Public** | livres, centres-culturels, codimoi, evenements, gouvernance, annonces/marché, television, vidafrica | CRUD + filtres |
| **Admin Vidafrica** | ~18 | vidéos (CRUD+multipart+état), pistes sous-titres (CRUD), segments (CRUD+réordonnement), timings mot (batch enregistrement/suppression) |
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
| **Admin Bibliothèques Humaines** | 4 | liste paginée (filtres statut/recherche), détail, `PATCH /{id}/valider` (transaction : demande→valide + bibliotheque_humain=TRUE + spécialités + notification + audit), `PATCH /{id}/rejeter` (commentaire + notification + audit) |

**Fichiers admin** : handlers `src/handlers/admin/` et models `src/models/admin/` — sous-modules: utilisateurs, organisations, partenariats, roles, pays, domaines, categories, tags, medias, specialites, programmes, candidatures, annonces, annonces_favoris, innovations, projets_admin, africantives_admin, centres_culturels, programmations, codimoi_admin, gouvernance, radio_tele, evenements, mooc, livres, audit, profils_pays, vidafrica, bibliotheques_humaines.

**Services** : `src/services/` — audit.rs (`log_action` non-bloquant, `extraire_ip`, `extraire_user_agent`). ~100 mutations instrumentées auto.

### Auth
JWT HS256 access (15min) + refresh (7j, SHA-256 hashé dans `iam.refresh_token`). Bcrypt cost 12. Vérification email: inscription → `etat='en_attente'` → token SHA-256 dans `iam.token_verification_email` (24h) → SMTP (lettre, STARTTLS 587) → clic = `etat='actif'` + `email_verifie=true` + auto-login. Modules: `jwt.rs`, `email.rs`.

### Deps backend
actix-web 4, actix-cors, actix-multipart, actix-files, sqlx (PostgreSQL), uuid, chrono, dotenvy, serde, sanitize-filename, bcrypt, jsonwebtoken, sha2, rand, livekit-api, lettre.

### Upload
Stockage local `./uploads/couvertures/`, `./uploads/documents/`, `./uploads/videos/` et `./uploads/vignettes/`, servis via actix-files sur `/uploads/`.

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
  - Email : admin@test.com
  - Mot de passe : Test1234

  - Email : user2@test.com
  - Mot de passe : Test1234

## Active Technologies
- Rust (Edition 2024) + TypeScript (Nuxt 4 / Vue 3) + Actix-Web 4, sqlx (PostgreSQL), Nuxt 4, Pinia, Tailwind CSS v4 (001-retrouve-amis)
- PostgreSQL 16 — nouveau schema `retrouve_amis` ajouté aux 10 existants (001-retrouve-amis)
- Rust (Edition 2024) + TypeScript (Nuxt 4 / Vue 3 SSR) + Actix-Web 4, sqlx (PostgreSQL), Nuxt 4, Pinia, Tailwind CSS v4 (002-partage-avis-recherche)
- PostgreSQL 16, schema `retrouve_amis` existant (6 tables + 1 fonction PL/pgSQL) (002-partage-avis-recherche)
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 (frontend) + Actix-Web 4, actix-multipart, sqlx, Pinia, Tailwind CSS v4 (003-retrouve-amis-public)
- PostgreSQL 16, schema `retrouve_amis` (8 tables existantes) (003-retrouve-amis-public)
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 (frontend) + Actix-Web 4, sqlx (PostgreSQL async), uuid, chrono, serde — frontend : Nuxt 4, Pinia, $fetch (001-personnes-arbre)
- PostgreSQL 16 — nouveau schema `arbre_genealogique` (11e schema bounded-context) (001-personnes-arbre)
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 (frontend) + Actix-Web 4, sqlx (backend) ; @vue-flow/core, @vue-flow/controls, @vue-flow/minimap, relatives-tree (frontend) (001-visualisation-arbre)
- PostgreSQL 16 — schema `arbre_genealogique` existant (aucune migration) (001-visualisation-arbre)
- TypeScript / Nuxt 4 / Vue 3 (frontend uniquement) + @vue-flow/core (existant), composants Feature 2 (existants) (001-edition-arbre)
- Aucune modification — utilise les endpoints existants (001-edition-arbre)
- Rust Edition 2024 (backend) + TypeScript / Nuxt 4 / Vue 3 (frontend) + Actix-Web 4, sqlx, pg_trgm (PostgreSQL extension), tokio::spawn — frontend: @vue-flow/core (existant) (001-matching-arbres)
- PostgreSQL 16 — schema `arbre_genealogique` étendu (2 nouvelles tables + 2 colonnes + 3 indexes) (001-matching-arbres)
- Rust Edition 2024 (backend, 1 endpoint) + TypeScript / Nuxt 4 / Vue 3 (frontend, principal) + pg_trgm existant (Feature 4), @vue-flow/core existan (001-recherche-exploration)
- Aucune modification SQL — réutilise colonnes normalisées et indexes de Feature 4 (001-recherche-exploration)
- Rust Edition 2024 (backend) + TypeScript / Nuxt 4 / Vue 3 (frontend) + Actix-Web 4, sqlx, lettre (SMTP existant) (001-collaboration-partage)
- PostgreSQL 16 — 2 nouvelles tables + 3 colonnes (001-collaboration-partage)
- Rust Edition 2024 (backend) + TypeScript / Nuxt 4 / Vue 3 (frontend) + pg_trgm existant, audit existan (001-notifications-suggestions)
- PostgreSQL 16 — 2 nouvelles tables (notifications, doublons_ignores) (001-notifications-suggestions)
- TypeScript (Nuxt 4 / Vue 3 SSR) + GSAP 3.14.2 (existant), Tailwind CSS v4 (existant) (001-ajout-personne-ludique)
- N/A — aucune modification backend/BDD (001-ajout-personne-ludique)
- TypeScript (Nuxt 4 / Vue 3 SSR) + GSAP 3.14.2 (deja installe), Vue 3 Composition API, Tailwind CSS v4 (001-nouveau-avis-ludique)
- N/A (aucune modification BDD) (001-nouveau-avis-ludique)
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 (frontend) + Actix-Web 4, actix-multipart, sqlx (backend) ; Vue 3 Composition API, Pinia (frontend) (004-vidafrica-sous-titres)
- PostgreSQL 16, schema `media_content` (4 nouvelles tables) + stockage local `./uploads/videos/` et `./uploads/vignettes/` (004-vidafrica-sous-titres)
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 SSR (frontend) + Actix-Web 4, actix-multipart, sqlx (PostgreSQL), uuid, chrono, serde, sanitize-filename, livekit-api (backend) ; Pinia, $fetch, FontAwesome, GSAP, AOS (frontend) ; tableau blanc & chat temps réel via canal data LiveKit déjà configuré (005-afrolang-salles)
- PostgreSQL 16 — schema `afrolang` étendu (3 nouvelles tables + ajout de colonnes sur 2 tables existantes) ; FK vers `country_profile.groupe_ethnique` existant ; stockage local `./uploads/afrolang/ressources/` pour fichiers ressources (005-afrolang-salles)
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 SSR (frontend) + Actix-Web 4, actix-multipart, sqlx (PostgreSQL), `image` crate 0.25 (validation JPEG/PNG + dimensions 2048×2048) ; Pinia, $fetch, FontAwesome, AOS (frontend) (001-afripulse-contributions)
- PostgreSQL 16 — schema `country_profile` étendu (4 nouvelles tables : `personnalite_connue`, `savoir_pratique`, `recommandation_visiteur`, `photo_visiteur` ; 5 nouveaux enums ; ALTER sur `site_touristique` + `contribution_fiche` ; valeur `obsolete` ajoutée à `etat_contribution`) ; stockage `./uploads/opportunite-afrique/photos/` (001-afripulse-contributions)
- Rust 2024 Edition (backend), TypeScript / Nuxt 4 (frontend) + Actix-Web 4, sqlx (PostgreSQL), Pinia, $fetch (001-admin-biblio-humaine)
- PostgreSQL 16 — schema `iam` (2 nouvelles tables + 1 enum) (001-admin-biblio-humaine)
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 SSR (frontend) + Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde (backend) ; Pinia, $fetch, FontAwesome (frontend) — aucune nouvelle dépendance (001-afrolang-pays-origine)
- PostgreSQL 16 — schema `afrolang` étendu (1 nouvelle table de jointure `salle_pays_origine`) ; FK vers `shared.pays` (001-afrolang-pays-origine)

## Recent Changes
- 001-afrolang-pays-origine : nouvelle relation N-N `afrolang.salle ↔ shared.pays` (pays d'origine de la langue cible, indépendante du pays implicite via groupe ethnique). BDD : nouvelle table `afrolang.salle_pays_origine` (PK composite, 2 FK CASCADE, index sur `pays_id`) ajoutée à `08b_afrolang.sql`. Backend : `lister_salles` et `obtenir_salle` enrichis d'un `json_agg` filtré sur `shared.pays.actif=TRUE` (Q3 — pays archivés masqués côté public) ; `SalleFiltres.pays_id` accepté en query (filtre `EXISTS`) ; struct `PaysOrigineLight` ajoutée à `models/afrolang.rs` ; `SalleResponse` / `SalleDetailResponse` / `AdminSalleDetailResponse` étendus avec `pays_origine: Vec<PaysOrigineLight>` ; lecture admin sans filtre `actif=TRUE` (chips grisées côté UI). 2 nouveaux endpoints admin : `POST /api/admin/afrolang/salles/{id}/pays` (insertion idempotente `ON CONFLICT DO NOTHING`, vérif salle non supprimée + pays actif) et `DELETE /api/admin/afrolang/salles/{id}/pays/{pays_id}` ; `audit::log_action("CREATE"|"DELETE", "afrolang", "salle_pays_origine", entity_id=salle_id)` sur chaque mutation. Frontend : composable `useAfrolang` (interface `PaysOrigineLight`, `pays_origine[]` sur `SalleAPI`, `pays_id` sur `SalleFiltres`) ; `useAdminAfrolangSalles.ajouterPaysOrigine` / `retirerPaysOrigine` ; `SalleCard.vue` Tailwind v4 pur (bandeau « Pays d'origine » : 1-3 chips drapeau emoji + nom, ≥4 drapeaux seuls + tooltip), helper `drapeauEmoji(codeIso2)` ; `SalleFilters.vue` + `SalleFiltersMobile.vue` : nouveau select « Pays d'origine » (mono-valué) alimenté par les pays dérivés des salles affichées ; `pages/afrolang/index.vue` : `filtres.pays_id` ajouté à `buildApiFiltres`/`resetFilters` + `watch` ; nouvel onglet « Pays d'origine » (daisyUI) sur `pages/admin/salles/[id].vue` (chips actuelles avec X retrait, pays archivés grisés, sélecteur d'ajout depuis `useAdminPays`).
- 006-afrolang-excalidraw : migration du moteur du tableau blanc Afrolang de `tldraw@4.3.2` (désactivé en production faute de licence commerciale) vers `@excalidraw/excalidraw@^0.18` (MIT). Périmètre chirurgical : `whiteboard/package.json` (retrait tldraw, ajout Excalidraw), `whiteboard/src/App.tsx` réécrit (composant `<Excalidraw langCode="fr-FR">`, bridge `postMessage` avec types `excalidraw-ready|excalidraw-operation|excalidraw-snapshot|excalidraw-image-rejected` vers parent et `apply-operation|load-snapshot|get-snapshot|clear` depuis parent, flag `remote` anti-écho, débouncing 80 ms, filtre `filterAppState`, lecture défensive `estSnapshotExcalidrawValide` rejetant les snapshots tldraw legacy, validation image JPEG/PNG ≤ 2 Mo avec retrait automatique et notification au parent). `uafricas_frontend/app/components/afrolang/AfrolangWhiteboard.vue` refondu en Tailwind v4 pur (principe VI) : wire LiveKit `{ type:'whiteboard', payload }` unifié pour ops et `__clear`, snapshot périodique modérateur 30 s en pause hors connexion, `watch` LiveKit `state` pour resync automatique à la reconnexion (FR-016), toast d'erreur image localisé, bouton « Effacer tout » conditionné modérateur. `uafricas_frontend/public/whiteboard/` régénéré. Intouchés (conformément à FR-012/FR-013) : `AfrolangRoom.vue`, pages `session/[id].vue` + `session/privee/[id].vue`, `useAfrolang.ts`, tout le backend Rust, schéma SQL `afrolang.tableau_blanc_session.donnees` (JSONB opaque — nouveau marqueur `type:'excalidraw', version:1`).
- 001-admin-biblio-humaine (US1→US4 + US5 notifications livrés) : workflow admin complet de validation des demandes Bibliothèque Humaine. BDD : nouveau DDL `schemas/04b_iam_biblio_demande.sql` — enum `iam.statut_demande_biblio`, tables `iam.demande_biblio_humaine` (id, utilisateur_id, statut, fonction, biographie, pays_origine_id, commentaire_admin, traite_par, traite_le, deleted_at) + `iam.demande_biblio_specialite` (join) + `iam.notification_biblio_humaine` (type approuvee|rejetee, lu bool), 3 index (unicité active UNIQUE partiel). Backend : `inscrire_biblio` refondu → INSERT demande (statut en_attente) + 409 si demande active déjà existante ; `lister_biblios` filtre sur `demande_biblio_humaine.statut='valide'` (plus `bibliotheque_humain=TRUE`) ; `ma_demande` GET `/moi/demande` JWT ; handler `admin/bibliotheques_humaines.rs` (lister_demandes filtrable, obtenir_demande, valider_demande en transaction atomique, rejeter_demande) ; 4 nouvelles routes admin `/api/admin/bibliotheques-humaines`. Frontend : `useAdminBibliosHumaines.ts` (état réactif + listerDemandes/obtenirDemande/validerDemande/rejeterDemande) ; pages admin `pages/admin/bibliotheques-humaines/index.vue` + `[id].vue` daisyUI ; `useBibliothequeHumaine.obtenirMaDemande()` async API réelle (remplace mock) ; onglet "Bibliothèque" ajouté à `profil.vue` (badge statut coloré, commentaire admin, lien re-soumettre si rejeté).
- 001-afripulse-contributions (US1→US5 + Phase 8 livrés) : enrichissement collaboratif des fiches pays `/opportunite-afrique`. BDD : nouveau DDL `schemas/11c_country_profile_afripulse.sql` — 4 nouvelles tables (`personnalite_connue`, `savoir_pratique`, `recommandation_visiteur`, `photo_visiteur`), 5 nouveaux enums (`categorie_site_touristique`, `type_objet_contribution`, `section_afripulse`, `categorie_savoir`, `domaine_personnalite`), ALTER sur `site_touristique` (+ `categorie` + `deleted_at`) et `contribution_fiche` (+ `type_objet_contribution`, `section_afripulse`, `target_id`, `nouvelle_valeur_jsonb`, `ancienne_valeur_jsonb`, `pieces_jointes` JSONB), valeur `obsolete` ajoutée à `etat_contribution`, 3 indexes partiels rate-limit. Backend : constantes `afripulse_pays_autorises` (54 codes ISO africains partagés front/back), services `image_validation` (JPEG/PNG ≤ 2 Mo, ≤ 2048×2048) + `rate_limit_afripulse` (20 textes/j, 10 photos/j, 5 en attente par pays) ; handler `afripulse_public.rs` (lecture des 4 sections enrichies + recommandations + galerie photos + création de fiche pays + `GET /api/fiches-pays/moi/contributions`) ; handler multipart `soumettre_contribution_multipart` sur `POST /api/fiches-pays/{id}/contributions/multipart` (photos + légendes) ; `admin/profils_pays.rs` étendu avec transaction atomique `moderer_contribution` (approbation → application JSONB sur la table cible + marquage `obsolete` des contributions concurrentes + notification auteur) et `retirer_contribution_approuvee` (soft-delete post-approbation). audit::log_action sur toutes nouvelles mutations. Frontend (Tailwind v4 pur côté public) : composable `useOpportuniteAfrique` étendu (types Afripulse, `soumettreContributionEnrichie`, `creerFichePays`, `listerRecommandations`, `listerGaleriePhotos`, `soumettreContributionMultipart`) ; 4 composants de lecture (`SitesTouristiquesSection`, `SecteursOpportunitesSection`, `PersonnalitesSection`, `SavoirAvantVoyagerSection`) + `NouvelleFichePaysModal` ; refonte `ContributionModal` ; constantes `nomsPaysAfrique` + `afripulsePaysAutorises` ; nouvelle page `pages/mon-compte/contributions.vue` (suivi perso paginé filtrable) + lien NavBar « Mes contributions ». Spec : `specs/001-afripulse-contributions/` (spec.md, plan.md, research.md, data-model.md, contracts/, quickstart.md, tasks.md).
- 001-afrolang-salles-refonte (US1→US4 + endpoints additionnels livrés): refonte complète des salles Afrolang → streaming public en 1 clic + salles privées par code secret bcrypt. BDD table rase : `afrolang.salle_privee` refondue (suppression `motif`, `declaration_adulte_at`, `visibilite`, `code_acces` plaintext ; ajout `code_acces_hash CHAR(60)`), suppression tables `salle_privee_adhesion` et `proposition_salle` + 5 enums legacy, ajout table `tentative_code_acces` (rate limit). Backend : 6 nouveaux endpoints publics (`POST /salles-privees`, `GET /salles/{id}/salles-privees`, `POST /salles-privees/{id}/verifier-code`, `POST /salles-privees/{id}/sessions/demarrer-ou-rejoindre`, `PATCH /salles-privees/{id}/code-acces`, `POST /salles-privees/{id}/archiver`) + helpers `hasher_code_acces` / `verifier_code_acces_plain` / `valider_format_code_acces` dans `handlers/afrolang.rs` + service `services/afrolang_rate_limit.rs` (5 échecs / 60 s) + JWT accès jeton 4 h dans `jwt.rs` ; suppression handlers/models `admin/propositions_afrolang` et nettoyage `admin/salle_privee` (plus de visibilité/adhésion/invitation) ; audit::log_action sur toutes nouvelles mutations (jamais de plaintext dans before/after). Frontend : suppression 9 composants legacy (AnnuaireGroupesEthniques, ProposerSalleModal, PropositionCard, SalleModerationPanel, SallePriveeVisibilitePanel, DemandeAdhesionCard, InvitationBanner, SalleSessionsLive, admin/ValidationPropositionsList) + 3 pages legacy (proposer, salle-privee/[id], admin/propositions), refonte `SallePriveeCreateModal` / `SallePriveeJoinModal` / `SallePriveeCard`, widget Canal privé (création + ouverture code secret), nouvelle page `pages/afrolang/session/privee/[id].vue`, middleware `afrolang-redirect-legacy.global.ts` (redirige `/afrolang/salle-privee/*` → `/afrolang`), composables `useAfrolang` / `useAdminAfrolangSalles` nettoyés.
- 005-afrolang-salles (US1→US6 livrés): schema `afrolang` étendu (propositions, salle_moderateur, salle_privee_adhesion, ressource_salle, message_session + 7 enums) + FK RESTRICT salle_privee→salle. Backend: 25+ handlers (propositions, modérateurs attitrés, transfert modération session, visibilité/limite/adhésions/invitations atomiques, ressources fichier+lien modéré, messagerie écrite, liens en attente admin, archivage batch/manuel, désactivation cascade salle publique) + `audit::log_action` sur chaque mutation, notifications `afrolang.*`. Frontend: composables `useAfrolang` et `useAdminAfrolangSalles` complets, composants public Tailwind v4 (AnnuaireGroupesEthniques, ProposerSalleModal, PropositionCard, SalleModerationPanel, SallePriveeVisibilitePanel, DemandeAdhesionCard, InvitationBanner, SalleChat, SalleRessources), admin daisyUI (ValidationPropositionsList, ModerateursAttitresPanel, LiensExternesValidation) + 3 pages admin sous `/admin/afrolang/`.
- 001-personnes-arbre: Added schema `arbre_genealogique` (4 tables: personnes, arbres, rattachements, liens_familiaux). Backend: 8 handlers CRUD + liens + photo upload, cycle detection CTE, cascade soft delete. Frontend: composable `useArbreGenealogique`, mock `arbre-genealogique.ts`, composants `PersonneForm.vue` / `PersonneCard.vue` / `LienFamilialForm.vue`, pages `arbre-genealogique/index.vue` + `[id].vue`. Architecture fondation matching inter-arbres documentée (Décision 8 research.md).
- 001-retrouve-amis: Added Rust (Edition 2024) + TypeScript (Nuxt 4 / Vue 3) + Actix-Web 4, sqlx (PostgreSQL), Nuxt 4, Pinia, Tailwind CSS v4


## Conventions

- **Français avec accents** (é, è, ê, à, ç, ù) obligatoires dans le code et les contenus
- **Nommage de fichiers/dossiers : PAS d'accents ni de caractères spéciaux** (problèmes d'encodage SSH/Docker en production). Utiliser uniquement `[a-z0-9_-]`.
