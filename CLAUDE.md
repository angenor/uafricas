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
Stockage local sous `./uploads/` (`couvertures/`, `documents/`, `videos/`, `vignettes/`, `afrolang/ressources/`, `opportunite-afrique/photos/`, `marketplace/annonces/`), servi via actix-files sur `/uploads/`.

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
- **événements streaming direct** (`001-evenements-streaming`) : un événement **« en ligne » / « hybride »** (`etat='publie'`) peut être diffusé **en direct sur la plateforme** selon un modèle **webinaire** — l'organisateur (= `cree_par`) diffuse caméra/micro/écran, les inscrits regardent et interagissent. Réutilise l'infra **LiveKit** (SFU + token JWT signé backend) déjà câblée pour afrolang ; **aucun média stocké** (pas d'enregistrement). Différence clé avec afrolang : le **token porte `can_publish:false` pour les spectateurs** (D2). Chat / réactions / lever-la-main circulent en **DataPackets LiveKit éphémères** ; seul `main_levee` est reflété en base (liste de demandes fiable pour l'organisateur). Schéma `media_content` : tables `evenement_session` (états `en_cours`/`terminee`, index unique partiel `WHERE etat='en_cours'` = une seule session active, `arret_securite_at` figé à l'ouverture) + `evenement_session_participant` (`role` ∈ `organisateur`/`intervenant`/`spectateur`, `main_levee`), migration idempotente `schemas/09b_media_content_evenements_streaming.sql`. `statut_direct` (`indisponible`/`en_attente`/`en_direct`/`termine`) **dérivé à la lecture**, **pas de cron** : arrêt de sécurité (= `fin+2h`) et cascade d'annulation (FR-016, via `changer_etat_evenement` admin) appliqués **paresseusement** (`cloturer_si_necessaire`). Endpoints sous `/api/evenements/{id}/direct` (JWT) : `GET` (état dérivé + `demandes_parole` si organisateur), `POST /rejoindre` (open-or-join, token scopé, capacité 409 D8, audit `OUVRIR` + notifs inscrits), `/quitter`, `/cloturer` (organisateur), `/lever-main` (spectateur, toggle), `/participants/{uid}/{promouvoir,retrograder,retirer}` (organisateur). Service `livekit_moderation` étendu : `update_participant_can_publish` + `retirer_participant`. Audit `media_content/evenement_session` (`OUVRIR`/`CLOTURER`/`PROMOUVOIR`/`RETROGRADER`/`RETIRER`, **sans** contenu chat/média). Notifs cloche `evenement_direct_demarre` + SSE `{type:"event_stream_demarre", evenement_id}` (flux messagerie existant, dispatch `event_stream_` dans `plugins/messagerie.client.ts`). Backend `models/evenement_streaming.rs` (calculs purs testés) + `handlers/evenement_streaming.rs`. Frontend : `useEvenements` étendu (état direct via `useState` signal SSE), composants Tailwind pur `evenements/EvenementDirectRoom`/`EvenementDirectControls`/`EvenementDirectChat`/`EvenementDirectReactions`/`EvenementDirectModerationPanel`, page plein écran `evenements/[id]/direct.vue`, boutons « Ouvrir/Rejoindre le direct » sur `evenements/[id].vue`. Repli `lien_en_ligne` si LiveKit KO (FR-023). Aucune config LiveKit côté frontend (URL/token fournis par le backend).
- **rendez-vous visio** (`001-rendez-vous-visio`) : deux membres **amis** organisent et tiennent un entretien vidéo 1-à-1. Le backend orchestre uniquement la prise de rendez-vous (proposer → accepter/refuser/contre-proposer → annuler) ; la visioconférence est **pair-à-pair (WebRTC via PeerJS, dép. frontend `peerjs`)** — aucun média ni signalisation applicative ne transite par le serveur (peer-id déterministe `uafr-{sha256(rdv_id‖participant_id)}`, anti-glare = plus petit UUID appelle, STUN public sans TURN, repli messagerie). Schéma `social` : enum `social.statut_rendez_vous` (`propose`/`accepte`/`refuse`/`annule`) + table `social.rendez_vous` (`tour_id` = partie devant répondre, bascule à chaque contre-proposition ; `duree_minutes ∈ {15,30,45,60}` ; soft delete), migration idempotente `schemas/31_social_rendez_vous.sql`. « expiré »/« terminé » **dérivés par calcul** (statut+date+durée), pas de cron. Endpoints `/api/rendez-vous` (JWT) : `POST` (proposer), `GET` (lister, filtres `attente_moi`/`attente_autre`/`a_venir`/`passes`), `GET /{id}`, `GET /{id}/salle` (config P2P, fenêtre `[date−5min, date+durée+15min]`), `POST /{id}/{accepter,refuser,contre-proposer,annuler}` (verrouillage optimiste 409). Revérif amitié/blocage à chaque action (FR-034), audit `social/rendez_vous` **sans** sujet/description (FR-033), notifications cloche unifiée (`creer_notification`, types `rdv_*`) + SSE (`{type:"rdv_*", rendez_vous_id}`) via le flux messagerie existant. Backend `models/rendez_vous.rs` + `handlers/rendez_vous.rs`. Frontend : composable `useRendezVous`, composants Tailwind pur `social/RendezVousProposerModal`/`RendezVousCarte`/`RendezVousContreModal`/`RendezVousListe`/`RendezVousSalle`, 3ᵉ onglet « Rendez-vous » (pastille) dans `MessagerieFlottante.vue`, bouton sur `profil/[id].vue`, dispatch `rdv_*` dans `plugins/messagerie.client.ts` (+ refresh cloche). Config runtime `nuxt.config.ts` : `peerjsHost/Port/Path/Secure`, `iceServers` (surchargeables `NUXT_PUBLIC_*`).
- **vidafrica contributions membres** : tout utilisateur connecté peut, depuis `vidafrica/[slug].vue`, **proposer une vidéo** (upload, créée en `etat='brouillon'`) et **contribuer une piste de sous-titres** (piste en `brouillon`, segments + tap-to-mark karaoké), accessible aussi via le bouton « Proposer une vidéo » de `vidafrica/index.vue`. Modération **au niveau de la piste** : nouvelle colonne `media_content.piste_sous_titre.etat` (`brouillon`|`publie`|`masque`, DEFAULT `publie`, migration idempotente `schemas/27b_vidafrica_contributions_membres.sql`). Les lectures publiques (`obtenir_sous_titres`, langues disponibles) ne montrent que les pistes `publie`. Handler membre `src/handlers/vidafrica_contribution.rs` (auth JWT in-handler comme le marché membre, contrôle de propriété + piste `brouillon` requis) exposé sous `/api/vidafrica/*` (POST `/videos`, `mes-pistes`, `/videos/{id}/pistes`, segments, timings-mot). Côté admin : `PATCH /api/admin/vidafrica/pistes/{id}/etat` (`changer_etat_piste`) pour valider/masquer ; la liste des pistes admin expose désormais `etat` + `cree_par_nom`. Frontend : composable `useVidafricaContribution`, composants Tailwind pur `VidafricaProposerVideoModal`/`VidafricaTapToMarkPublic`/`VidafricaContribuerSousTitres`, composable admin `useAdminVidafrica.changerEtatPiste`.
- **factcheck volets + réactions** : chaque factcheck porte deux volets 1:1 (préjugé/réalité) avec titre/description et compteur de cœurs indépendant (`prejuge_*`, `realite_*` sur `governance.factcheck`). Réaction globale au post = jeu d'emojis (cœur, pouce, rire, je n'aime pas) une par utilisateur. Table `governance.factcheck_reaction` étendue avec `cible` (`general`|`prejuge`|`realite`), `type_reaction` élargi, `UNIQUE(factcheck_id, utilisateur_id, cible)`. Migration `schemas/10f_governance_factcheck_volets_reactions.sql` (idempotente). Endpoint `POST /api/gouvernance/factcheck/{id}/reaction` (toggle, JWT requis) ; `lister_contributions` enrichi (volets + compteurs + `ma_reaction`/`a_like_*` via JWT optionnel — `getContributions` envoie le header auth) ; `creer_factcheck_public` + `FactCheckCreateModal` saisissent les volets. Page `factcheck.vue` branchée sur l'API (hors mocks), composable `useGouvernance.reagir(id, cible, type)`.
- **factcheck signalement (modération communautaire)** : bouton « Signaler » sur chaque carte ; un signalement par utilisateur (`UNIQUE`). Au-delà de **20 signalements distincts** (`SEUIL_SIGNALEMENTS_SUSPENSION`), le factcheck passe automatiquement en `etat='suspendu'` (retiré de la liste publique). Table `governance.factcheck_signalement` + colonne `nombre_signalements`, migration `schemas/10g_governance_factcheck_signalement.sql` (idempotente). Endpoint `POST /api/gouvernance/factcheck/{id}/signalement` (JWT requis, insert idempotent + recompte + suspension). `lister_contributions` expose `a_signale`. Composable `useGouvernance.signaler(id, motif?)`. Audit `SIGNALEMENT` instrumenté.
- **marché membre** (`001-marche-achat-vente-troc-don`) : couche d'endpoints membre `/api/annonces` (auth JWT) pour publier (multipart, photos `marketplace/annonces/`, `etat='publiee'` immédiat), gérer (« Mes annonces », modifier, conclure, supprimer soft), favoris, et contacter l'auteur via la **messagerie existante** (`social.conversation.annonce_id`, `30_social_conversation_annonce.sql`). Enum `etat_annonce` étend `'conclue'`. `envoyer_message` assoupli (amitié OU conversation existante, D2). Endpoint public `/api/annonces/categories`. Validation photos `image_validation::valider_photo_annonce` (JPEG/PNG/WebP, 3 Mo). Composable `useMarcheAfricain` étendu, composants `MarcheAnnonceForm.vue`/`MarcheFavoriBouton.vue`, pages `mes-annonces.vue`/`favoris.vue`.
- **social** (`001-demande-amitie`) : amitié entre membres + messagerie privée temps réel (SSE) via bouton flottant global. Schéma `social` (`schemas/29_social.sql`).
- **expertise** (`001-demande-expertise`) : candidature `/devenir-expert`, validation admin + email. Schéma `iam.expertise` étendu.
- **annuaire-membres** : `/profil` = annuaire public, `/profil/[id]` = page de détail unifiée (membre + biblio + expertise).
- **bibliothèques humaines** (`001-admin-biblio-humaine`) : workflow admin de validation. Schéma `iam` (`04b_iam_biblio_demande.sql`).
- **afrolang** : refonte salles (streaming public + privées par code bcrypt), pays d'origine, propositions communautaires + admins de salle, modération de session (permissions tableau blanc + spotlight), fermeture admin + historique modération, migration tableau blanc tldraw → Excalidraw.
- **afripulse** (`001-afripulse-contributions`) : enrichissement collaboratif fiches pays `/opportunite-afrique`. Schéma `country_profile` étendu (`11c_country_profile_afripulse.sql`).
- **sites touristiques enrichis** (`001-sites-touristiques-enrichis`) : sous-type (enum `sous_type_site`, 20 valeurs, validation famille↔sous-type en code), fiche complète (gestionnaire, localisation, GPS, info pertinente, contacts publics), constitution légale facultative, badge « Vérifié » admin (audité), avis visiteurs notés 1–5 (table `avis_site`, upsert écriture directe + modération admin). Schéma `country_profile` étendu (`11d_country_profile_sites_enrichis.sql`). Endpoints publics `/api/sites-touristiques/{id}/avis` + admin vérification/masquage. Composants `SiteTouristiqueCarte.vue`, `SiteAvisListe.vue`.
- **infos pratiques voyage** : bloc « À savoir avant de voyager » → section structurée (8 champs scalaires sur `country_profile.fiche_pays` : `voyage_langue_internationale`, `voyage_langue_locale`, `voyage_infos_visa`, `voyage_infos_sanitaires`, `voyage_meteo`, `voyage_prises_electriques`, `voyage_contacts_tourisme`, `voyage_recommandations_securite`). Schéma `11e_country_profile_infos_voyage.sql`. Contribuable par tous via le canal scalaire `fiche_pays` existant (`SECTIONS_VALIDES` + `appliquer_fiche_scalaire`, modération admin). UI : mode « champ ciblé » du `ContributionModal.vue`, bloc dans `SavoirAvantVoyagerSection.vue`.
- **vidafrica** (`004-vidafrica-sous-titres`) : vidéos + sous-titres. Schéma `media_content` étendu.
- **arbre généalogique** (`001-personnes-arbre` et suivantes) : schéma `arbre_genealogique`, visualisation @vue-flow, édition, matching inter-arbres (pg_trgm), recherche, collaboration/partage, notifications/suggestions.

## Active Technologies
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 (Vue 3 SSR) (frontend) + Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde, sanitize-filename, image, lettre (backend) ; Pinia, Tailwind CSS v4, FontAwesome (frontend) (001-sites-touristiques-enrichis)
- PostgreSQL 16, schéma `country_profile` (source de vérité — Principe III) (001-sites-touristiques-enrichis)
- Rust Edition 2024 (backend) ; TypeScript / Nuxt 4 (Vue 3 SSR) (frontend) + Actix-Web 4, sqlx (PostgreSQL), actix-multipart, `image` crate, service interne `image_validation`, JWT (`jwt.rs`), `audit::log_action`, SSE (`messagerie_sse::RegistreSse`) ; Pinia, Tailwind CSS v4 (pur), FontAwesome (frontend) (001-marche-achat-vente-troc-don)
- PostgreSQL 16, schémas `marketplace` (source de vérité — Principe III) et `social` (messagerie). Upload photos en local sous `./uploads/marketplace/annonces/` servi par actix-files (001-marche-achat-vente-troc-don)
- Rust Edition 2024 (backend), TypeScript / Nuxt 4 (Vue 3 SSR) (frontend) + Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde, `audit::log_action`, `RegistreSse` (SSE) (backend) ; Pinia, Tailwind CSS v4 (pur), FontAwesome, **peerjs** (WebRTC P2P) (frontend) (001-rendez-vous-visio)
- PostgreSQL 16, schéma `social` (source de vérité — Principe III). Cloche persistante via `arbre_genealogique.notifications` (système cloche unifié existant). Aucun stockage de média (P2P). (001-rendez-vous-visio)
- Rust Edition 2024 (backend) ; TypeScript / Nuxt 4 (Vue 3 SSR) (frontend) + Actix-Web 4, sqlx (PostgreSQL), `livekit-api` 0.4, `livekit-protocol` 0.7, `jsonwebtoken`, `uuid`, `chrono`, `serde` (backend) ; `livekit-client` ^2.17.1 (déjà installé), Pinia, Tailwind CSS v4 (pur), FontAwesome (frontend) (001-evenements-streaming)
- PostgreSQL 16, schéma `media_content` (source de vérité — Principe III). **Aucun stockage de média** (flux via SFU LiveKit). Cloche persistante via `arbre_genealogique.notifications` (système unifié existant). (001-evenements-streaming)
