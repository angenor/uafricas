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

**Domaines principaux** : Auth (JWT/refresh), Public (livres, centres-culturels, codimoi, evenements, gouvernance, marché, television, vidafrica), Admin (IAM, référentiels, programmes, marché, innovation, culture, gouvernance, médias, audit, profils pays, vidafrica, bibliothèques humaines, expertise, afrolang salles), Social (amitié + messagerie, schéma `social`), Engagement (points/badges paramétrables, schéma `engagement` : 8 routes membre + 28 routes admin).

**Services** : `src/services/` — `audit.rs` (`log_action` non-bloquant, ~100 mutations instrumentées), `messagerie_sse.rs`, `livekit_moderation.rs`, validations diverses.

### Auth
JWT HS256 access (15min) + refresh (7j, SHA-256 hashé dans `iam.refresh_token`). Bcrypt cost 12. Vérification email: inscription → `etat='en_attente'` → token SHA-256 (`iam.token_verification_email`, 24h) → SMTP STARTTLS 587 → clic = `etat='actif'` + `email_verifie=true` + auto-login. Modules: `jwt.rs`, `email.rs`.

### Deps backend
actix-web 4, actix-cors, actix-multipart, actix-files, sqlx (PostgreSQL), uuid, chrono, dotenvy, serde, sanitize-filename, bcrypt, jsonwebtoken, sha2, rand, livekit-api, lettre, futures-util, tokio, image.

### Upload
Stockage local sous `./uploads/` (`couvertures/`, `documents/`, `videos/`, `vignettes/`, `afrolang/ressources/`, `opportunite-afrique/photos/`, `marketplace/annonces/`), servi via actix-files sur `/uploads/`.

### Database
PostgreSQL 16 Docker. Schema SQL: `uafricas_backend/doc/bd/schema.sql` (orchestrateur via `\ir` dans `schemas/`). Init auto via `docker-init.sh`.

Schemas bounded-context : `shared`, `iam`, `marketplace`, `exchange`, `innovation`, `culture`, `afrolang`, `media_content`, `governance`, `country_profile`, `arbre_genealogique`, `social`, `engagement`.

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
- **Docker Prod** : `docker-compose.prod.yml` — 7 services (postgres, backend, frontend, nginx, livekit, **coturn**, adminer optionnel) + 2 volumes (pgdata, uploads_data). Nginx HTTPS Let's Encrypt, reverse proxy frontend:3000 + backend:8080, rate limit API 30r/s + auth 5r/s, gzip, HSTS. Domaine `www.africans-world.org`.
- **TURN (appels P2P PeerJS)** : service **coturn** (`network_mode: host`, port **3479** UDP/TCP — 3478 réservé à LiveKit, relais `49160-49200/udp`, credentials long-terme `TURN_USERNAME`/`TURN_PASSWORD`). **Indispensable en prod** : sans relais TURN, deux pairs derrière des NAT symétriques n'échangent pas leur flux vidéo (vidéo distante noire). Le frontend reçoit `NUXT_PUBLIC_ICE_SERVERS` (STUN public + TURN udp/tcp) **au runtime** (aucun rebuild Nuxt requis). Variables dans le `.env` prod (gitignored) : `TURN_PUBLIC_IP` (IP publique VPS, partagée par `--external-ip` coturn et l'URL ICE frontend), `TURN_USERNAME`, `TURN_PASSWORD`. Voir `.env.production.example`.
- **Déploiement** : `deploy.sh` → VPS `root@161.97.92.63:/opt/uafricas` via SSH+Docker. Commandes: `setup`, `deploy`, `update`, `rebuild`, `status`, `logs [svc]`, `restart [svc]`, `stop`, `ssl`, `backup`, `connect`. Migrations BD manuelles via SSH+psql.

## LSP & Diagnostics
- **rust-analyzer** + **Volar** (VS Code). Utiliser `getDiagnostics` après chaque modification de fichier.

## Parallel Sub-agents
Recherche frontend + backend simultanée, exploration multi-fichiers en parallèle, tests/vérifications en parallèle après modifications. Avant nouveau composant : vérifier si similaire existe dans `app/components/`.

## Auto-maintenance
Mettre à jour ce fichier lors de : ajout/suppression service Docker ou dépendance majeure, nouveau composable/store/module, nouvel endpoint API ou schema BDD, changement commandes dev, conventions, CI/CD.
**Recent Changes = index court (1 ligne/feature).** Ne pas y remettre de longs paragraphes : le détail est dans `git log`, `specs/` et le code. Une ligne cite au plus la migration SQL et les modules/composants clés.

## Test Users
- `test-admin@test.com` / `Test1234`
- `test-user@test.com` / `Test1234`

## Tech Stack par feature
Backend Rust Edition 2024 + Actix-Web 4 + sqlx (PostgreSQL) ; frontend Nuxt 4 / Vue 3 SSR + Pinia + Tailwind v4. Extensions notables : `pg_trgm` (matching arbres), LiveKit (afrolang temps réel + modération + événements streaming), SSE via `futures-util`/`tokio` (messagerie social), `image` crate (validation photos), `lettre` (SMTP), `peerjs` (visio P2P rendez-vous/appels), `@vue-flow/core` (arbre généalogique), `@excalidraw/excalidraw` (tableau blanc afrolang, MIT).

## Recent Changes (index)
Une ligne par feature ; détail via `git log` + `specs/` + code. Migrations SQL sous `uafricas_backend/doc/bd/schemas/`.
- **engagement — barème paramétrable, espace « Mon engagement », badges** (`007-engagement-points-badges`) : migrations `35c` (`categorie_points` + `regle_points.{categorie_id,seuil_declencheur}` + `mouvement_points.categorie_id` + `palier_popularite.type_objet` avec `idx_uq_palier_seuil_famille … NULLS NOT DISTINCT` + unicité `niveau.seuil_min`, 6 catégories et 4 règles seedées), `35d` (`badge`/`badge_obtenu`, `ck_badge_condition` rendant le paramétrage incohérent impossible **en SQL**, 10 badges, rétro-évaluation sans notification), `35e` (`partage_externe`, l'unicité `(membre, contenu, réseau)` rendant « réseaux distincts » structurel). **La catégorie est recopiée sur le mouvement à l'écriture** : re-catégoriser une règle ne déplace aucun point déjà gagné, alors que le libellé est relu dans la règle (renommer corrige tout l'historique affiché). `services::engagement` gagne `evaluer_badges` (5 conditions, enum fermé, **appelée après le COMMIT** et à la lecture de `/mes-badges` — aucune tâche de fond), `enregistrer_partage_externe`, la **substitution** des paliers par famille (jamais d'union : deux clés d'idempotence créditeraient deux fois) ; `recalculer_niveaux` (`handlers/admin/engagement.rs`) rebascule tous les comptes **dans la transaction** de chaque mutation de niveau et renvoie `comptes_recalcules`. `ACTIONS_INSTRUMENTEES` (const Rust, `GET /actions-disponibles`) est l'antidote à la règle orpheline : une règle hors catalogue s'affiche « non instrumentée ». 4 branchements médias post-commit (`admin/media_proposition` ×2 avec clé `animation:{id}` **partagée** avec le chemin co-détenteurs, `admin/radio_tele` ×7 sites — `chaine_tv` n'a pas de colonne `a_la_une`, seules 3 tables la portent + `a_la_une_globale` —, `media_social` avec décompte des likes **excluant l'auteur**, distinct du compteur affiché). Front : `/mon-compte/engagement` + `engagement/{ResumeEngagement,VentilationCategories,HistoriquePoints,MesBadges,BadgeSucces}` (Tailwind pur), `MesPointsPanel` réduit au résumé + lien, `/admin/engagement/{categories,niveaux,badges}`, `usePartageExterne` (traçage best-effort **après** l'ouverture de la fenêtre), Telegram + e-mail ajoutés aux 6 modales de partage (sans eux le seuil de 5 réseaux était inatteignable). ⚠️ Les plafonds sont **en points, pas en occurrences**.
- **télé & profil — accès à la gestion de ses supports** : aucune migration. `/medias/tele` charge `GET /api/medias/supports/moi` une fois au montage et passe `monRole` à `SectionChaine`, qui révèle « Gérer ma chaîne » (lien `?support=<id>`) aux seuls détenteurs ; `MesSupports` déplie le panneau visé par cette requête. `/mon-compte/profil` : menu déroulant de 10 onglets remplacé par un menu latéral groupé (3 sections, pilules défilables en mobile, conteneur `max-w-6xl`), + raccourci « Gérer mes supports médias » compté dans l'en-tête.
- **médias — contacts publics des supports** : migration `09p` (`contact_{email,telephone,whatsapp,site_web,adresse}` sur `chaine_tv` ET `station_radio`). Saisis à la création admin, à la proposition membre (`DonneesProposition`, publiés seulement à la validation) et modifiables en back-office ; affichés par `media/BlocContacts` sur `/medias/{chaines,stations}/[slug]`. `services::contacts_media` centralise trim + préfixe `https://` (une URL sans schéma serait un lien RELATIF) ; `useContactsSupport` construit les `mailto:`/`tel:`/`wa.me` (le « 00 » international est retiré, `wa.me` ne l'accepte pas). DTO `contacts` omis du JSON quand aucune coordonnée.
- **télé — barre de filtres en pied de vedette** (`001-refonte-tele-radio`) : migration `09o` (`chaine_tv.origine_publication` 'africans'|'territoire', pendant télé de `09j`, mais les deux familles cohabitent sur `/medias/tele` — c'est un filtre, pas deux pages). `GET /television/sections` gagne `origine`/`theme`/`en_direct` ; le thème étant porté par les **programmes**, une chaîne remonte via `EXISTS` sur ses contenus publiés. Front : `media/BarreFiltresTele` montée dans le slot `filtres` de `VedettePleinEcran` — elle remplace la flèche animée « Nos chaînes » ET le bandeau de statistiques (`useTelevision.obtenirStats`/`TvStat` supprimés, endpoint `/television/stats` conservé). Champ « Origine de publication » ajouté au back-office chaînes TV.
- **télé & radio — signalement & modération (lot 3, US7)** (`001-refonte-tele-radio`) : aucune migration neuve (`signalement_media` livrée par `09k`, `nombre_signalements` par `09j`). `media_social::signaler_media` — `INSERT … ON CONFLICT DO NOTHING` puis recompte distinct, bascule `etat = 'suspendu'` au **11ᵉ** signalement (`SEUIL_SIGNALEMENTS_SUSPENSION_MEDIA = 10`, comparateur `>`) : ces 4 tables n'ont pas de colonne booléenne `suspendu`, contrairement à `signalement_contribution`. Jamais de désuspension automatique — `admin/media_proposition::changer_etat_media` rétablit et **remet `nombre_signalements = 0`** (sans quoi le seuil resterait franchi), les lignes de signalement étant conservées pour l'historique. File admin `lister_signalements` (les 4 tables interrogées séparément puis fusionnées, colonne de titre hétérogène) + `detail_signalements`. Front : `media/{MediaSignalerModal,MediaSignalerBouton,ReglesContenuModal}`, `useAdminMediaSignalements`, `/admin/medias/signalements`, bouton branché sur les 4 pages de détail et les 2 sections. Nettoyage : `mocks/tele.ts` supprimé et `mocks/radios.ts` réduit à son contenu éditorial (interfaces `RadioStation`/`TvChannel`/`TvProgram` doublonnant les composables, jeux de démonstration sans consommateur).
- **télé & radio — programmation & engagement (lot 3, US5–US6)** (`001-refonte-tele-radio`) : migration `09n` (`creneau_programmation` : `TIME` + `jour_semaine` + `fuseau`, CHECK jour/récurrence et non-franchissement de minuit) ; `09m` complétée par 5 spécialités audiovisuelles. Backend `media_detention.rs` (`garde_detenteur` — **jamais `AdminUtilisateur`**, ces routes sont membres —, invitations, retrait soft, `contacter` dupliquant `contacter_auteur`), `media_programmation.rs` (**résolution paresseuse** du créneau courant par `(NOW() AT TIME ZONE fuseau)`, aucune tâche de fond ; verrou `FOR UPDATE` sur le **support parent** avant détection de chevauchement → 409 sans écriture). `diffusion_en_cours`/`creneau_suivant` greffés sur les endpoints `sections`. US6 : `appliquer_acceptation_engagement` partagée par la file admin et la décision des co-détenteurs (une demande d'animation acceptée crée le `support_detenteur`, `objet_id_cree` portant son id) ; filtre `specialite` + `GET /api/experts/specialites` (libellés réellement déclarés, `iam.expertise.specialites` étant du texte libre sans lien avec `specialite_bibliotheque`). Front : `useMediaDetention`/`useMediaProgrammation`, `media/{GrilleProgrammation,GestionCoDetenteurs,BandeauDiffusion,ProposerIdeeModal,DemanderAnimationModal,ContacterSupportModal}`, `/mon-compte/{mes-supports,invitations-medias}`, filtre spécialité sur `/experts`.
- **télé & radio — participation & modération (lot 2)** (`001-refonte-tele-radio`) : migrations `09k` (`media_reaction`/`media_commentaire`/`partage_media`/`signalement_media`, génériques par `(type_media, media_id)` sur les 4 tables médias), `09l` (`proposition_media` polymorphe + 4 CHECK rendant le workflow inviolable en SQL), `09m` (`support_detenteur`/`invitation_detenteur`, livrée en avance car la validation crée le propriétaire). Backend `media_social.rs` (réagir/commenter/partager + `compteurs_pour` : compteurs de toute une page en 2 requêtes), `media_proposition.rs` (soumission multipart, suivi, `PATCH …/metadonnees` publié vs `PUT …/media` remis en attente), `admin/media_proposition.rs` (file + validation atomique : objet créé → propriétaire → notification, en une transaction). **Faille fermée** : les 3 routes publiques de création inséraient `etat = 'publie'` sans contrôle de rôle → `'en_attente'`. `origine_publication` forcée à `'territoire'` côté serveur (non exprimable par le client). Front : 4 pages détail SSR+Open Graph (`medias/{chaines,stations,programmes-tele,programmes-radio}/[slug]`), `media/{MediaReactionsBar,MediaCommentaires,MediaPartagerModal,ProposerMediaModal}`, `useMediaSocial`/`useMediaProposition`/`useAdminMediaPropositions`, `/mon-compte/propositions-medias`, `/admin/medias/propositions`, 8ᵉ source du mur (`publications/MediaPartageCard`). Supprimé : `AddProgramModal.vue` (maquette morte).
- **télé & radio — refonte en vitrine éditorialisée (lot 1)** (`001-refonte-tele-radio`) : migration `09j` (`programme_tele.a_la_une_globale` + index unique global, `station_radio.origine_publication` 'africans'|'territoire', `theme_phare_*`, `role_partie_prenante_*`, état `en_attente`, `nombre_signalements`, 44 thèmes en `shared.categorie` contexte `media`, permissions `media.*`, FK manquantes sur `station_radio`). `/medias/tele` : vedette `100svh` + une section par chaîne au défilement (`GET /television/{vedette,sections}`) ; les deux pages Radio deviennent réellement distinctes par l'origine (`GET /stations-radio/sections?origine=`), les émissions radio obtenant enfin une exposition publique (`/programmes-radio`). Nouveaux `media/{VedettePleinEcran,SectionChaine,SectionStation,LecteurMedia,RangeeContenus,CarteContenu,BarreLecturePersistante,DecorRadio}`, `useLecteurMedia` (état global, barre montée dans `default.vue` hors `<slot/>` ⇒ l'écoute survit à la navigation), `useObservateurVisibilite`, `utils/media.ts`. Supprimés : `useAudioPlayer`, `AudioPlayer.vue`, `StationCard.vue`, contenu vedette codé en dur.
- **vidafrica — sous-titrage « au fil de la lecture »** : 2e stratégie (en plus du mode manuel) sur `/vidafrica/[slug]`. Nouveau `VidafricaCaptureSequentielle` pilote le lecteur unique (`VidafricaLecteur` : barre de lecture PERSONNALISÉE persistante — timeline/scrub/son/plein écran toujours visibles, contrôles natifs retirés ; expose `seek/lire/pause/positionMs/dureeMs` + emits `lecture`/`fin`) : lire → couper → saisir [curseur→coupe] → reprendre, segments contigus, timings issus de la lecture (aucun endpoint neuf, réutilise `creerSegment`). L'atelier est en **overlay sur le panneau droit** « Sous-titres enregistrés » (`lg:absolute inset-0`, empilé en mobile), vidéo pleine taille au centre. Bascule de mode interne à `VidafricaContribuerSousTitres`.
- **opportunité-afrique — sous-objets : réactions, partages sociaux & communautaires** : migration `11k` (`reaction_element`+`partage_element`, génériques par `(type_objet, objet_id)`). Backend `element_social.rs` (reagir/partager/lister) ; `obtenir_*` enrichis `nombre_likes/dislikes/ma_reaction`. Front `ReactionsBar`/`PartagerElementModal`, Open Graph sur les 4 pages détail, `PublicationsElementPartageCard` (source `element_partage` du mur). Pages détail dotées de heros immersifs (`DetailHero`).
- **opportunité-afrique — pages de détail dédiées** : `[id].vue`→`[id]/index.vue`, routes sœurs `[id]/{recettes,sites,secteurs,personnalites}/[itemId].vue` (SSR+OG). Handlers publics `obtenir_*` (`afripulse_public.rs`) + `useOpportuniteAfrique.obtenir*` ; sections naviguent (fini les modals de détail).
- **formations — page détail refondue (bannière + ancres) & objectif/certification** : migration `09i` (`mooc.objectif`, `presentation`, `a_evaluation`, `est_certifiante`). Page publique `[id].vue` : bannière couverture, sidebar gauche = menu d'ancres sticky (Objectif/Présentation/Programme/Intervenants) + carte d'inscription, intervenants en placeholders. Admin `create.vue`/`[id].vue` : 4 champs. Inscription ouverte aussi aux formations `en_cours`.
- **formations — chapitres → leçons + progression** : migration `09h`. Contenu leçon gaté aux inscrits (`moocs.rs` `GET /contenu`, completion) ; admin `formation_contenu.rs` (CRUD+réordonnancement) ; front `UniversiteInudaFormationCurriculum`/`AdminFormationCurriculum`.
- **radio & télé — fichiers/liens + split tables** : migrations `09f`/`09g`. `programme_radio_tele` scindée en `programme_radio`+`programme_tele` ; upload `POST /api/admin/medias/upload` ; `/admin/radio` + `/admin/television`, `useAdminRadio`/`useAdminTelevision`/`useAdminMediaUpload`.
- **événements — thématique + rediffusion** : migration `09e`. Colonne `type` (thématique) + `enregistrement_url` (YouTube embed si `statut='termine'`).
- **afrolang — signalement de salle depuis session** : migration `08i` (`signalement_salle`, >10 → `suspendu`). `session_signalement.rs`, `SignalerSessionModal`.
- **opportunité-afrique — signalement par contribution** : migration `11j` (`signalement_contribution`, >10 → `suspendu`, réactivation admin). `contribution_signalement.rs`, `ContributionSignalerBouton`.
- **télévision — chaînes + à la une en boucle** : migration `09d`. Un `a_la_une` par chaîne joué en boucle sur `/tele`.
- **facultés & écoles partenaires (INUDA) — back-office CRUD** : schéma `exchange`, perm `programme`. `admin/facultes.rs`, `useAdminFacultes`/`useAdminEcolesPartenaires`, `AdminFaculteForm`.
- **vidafrica — réactions + partage** : migration `27d` (`video_reaction`, `partage_video`). `VidafricaReactionsBar`/`VidafricaPartagerModal`, source `video_partage` sur `/publications`.
- **vidafrica — infos de proposition** : migration `27c` (territoires, décharge de droits, auteur réel).
- **biblio humaine — interactions profil** : migration `04g` (réactions/commentaires/recommandations sur `/profil/[id]`). `BibliothequeInteractions`.
- **événements — champs détail + type organisateur** : migration `09c` (adresse/lien/places/contact, `type_organisateur`). Onglet « Mes événements » + couverture modifiable.
- **profils — notation/partage/signalement** : `note_expertise` (`04d`), `partage_profil` (`32`), `signalement_profil`+suspension auto (`04e`).
- **arbre généalogique — assistant conversationnel** : panneau chatbot progressif. `useAssistantArbre`, `AssistantConversationnel`.
- **centre culturel — inscription aux programmations** : migrations `08f`/`08g` (`programmation_inscription` + infos). Bascule S'inscrire/Se désinscrire + admin liste inscrits.
- **centres culturels & programmations — image back-office** : migration `08e`. Réutilise `OpportuniteAfriqueImageUploadField`.
- **centres culturels — international/local** : migration `08d` (`type_centre`). `/centres` en 2 sections.
- **secteurs d'opportunités afripulse enrichis** : migration `11g` (localité, contacts, références, site web, image).
- **sites touristiques afripulse — 5 photos + lien web** : migration `11f`. Carte compacte + détail.
- **fenêtres flottantes déplaçables/redimensionnables** : `useFenetreFlottante` (messagerie + appel direct).
- **appel direct entre amis (visio P2P)** : registre mémoire `RegistreAppels`, `/api/appels`, `useAppels`, `AppelDirectSalle`/`AppelEntrantPrompt`, partage d'écran.
- **afrolang — multi-modérateurs + passation** : migration `08c`. Résolution paresseuse + DataPackets LiveKit `moderation.*`.
- **événements streaming direct** (`001-evenements-streaming`) : migration `09b`. Webinaire LiveKit (spectateurs `can_publish:false`), `evenement_streaming.rs`, `/evenements/[id]/direct`.
- **rendez-vous visio** (`001-rendez-vous-visio`) : migration `31` (`social.rendez_vous`). WebRTC/PeerJS P2P, orchestration prise de RDV backend, `useRendezVous`.
- **vidafrica — contributions membres** : migration `27b`. Proposer vidéo + piste sous-titres, modération par piste (`etat`). `vidafrica_contribution.rs`.
- **vidafrica — sous-titres collaboratifs** : migration `27e` (1 piste/auteur/langue, 1 seule publiée/langue).
- **factcheck — volets + réactions** : migration `10f` (préjugé/réalité + réactions emoji).
- **factcheck — signalement** : migration `10g` (>20 → `suspendu`).
- **marché membre** (`001-marche-achat-vente-troc-don`) : endpoints `/api/annonces` (publier/gérer/favoris/contacter via messagerie). Migration `30` (`conversation.annonce_id`).
- **social** (`001-demande-amitie`) : amitié + messagerie privée SSE. Schéma `social` (`29_social.sql`).
- **expertise** (`001-demande-expertise`) : candidature `/devenir-expert` + validation admin. Schéma `iam.expertise`.
- **annuaire-membres** : `/profil` (annuaire) + `/profil/[id]` (détail unifié membre/biblio/expertise).
- **bibliothèques humaines** (`001-admin-biblio-humaine`) : workflow admin de validation (`04b_iam_biblio_demande.sql`).
- **afrolang** : salles streaming public + privées (code bcrypt), propositions communautaires, modération de session, tableau blanc Excalidraw.
- **afripulse** (`001-afripulse-contributions`) : enrichissement collaboratif fiches pays `/opportunite-afrique` (`11c_country_profile_afripulse.sql`).
- **sites touristiques enrichis** (`001-sites-touristiques-enrichis`) : sous-type, fiche complète, badge « Vérifié », avis 1–5 (`11d`).
- **infos pratiques voyage** : 8 champs scalaires sur `fiche_pays` (`11e_country_profile_infos_voyage.sql`).
- **vidafrica** (`004-vidafrica-sous-titres`) : vidéos + sous-titres. Schéma `media_content`.
- **arbre généalogique** (`001-personnes-arbre` et suivantes) : schéma `arbre_genealogique`, visualisation @vue-flow, matching pg_trgm, collaboration/partage, notifications.

## Active Technologies
- Rust Edition 2024 (backend) · TypeScript 5 / Vue 3 SSR / Nuxt 4 (frontend) + Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde · Pinia, Tailwind CSS v4, FontAwesome (001-refonte-tele-radio)
- PostgreSQL 16, schéma `media_content` · uploads locaux `./uploads/medias/{videos,audios}/` servis par actix-files (001-refonte-tele-radio)
- **Aucune dépendance nouvelle** ; PostgreSQL 16 requis pour `NULLS NOT DISTINCT` (paliers par famille) ; notifications dans `arbre_genealogique.notifications`, table générique **de fait** de la plateforme — son `type` est un `VARCHAR` libre, contrairement à `social.notification` dont le `type` est un enum (007-engagement-points-badges)
