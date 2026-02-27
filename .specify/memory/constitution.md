<!--
=== Sync Impact Report ===
Version change: 1.0.0 → 1.1.0 (MINOR)
Modified principles:
  - VI. "Tailwind CSS v4 + daisyUI v5" → "Tailwind CSS v4 (daisyUI v5 back-office uniquement)"
    daisyUI v5 restreint au back-office admin ; site public = Tailwind CSS v4 pur
Added sections: N/A
Removed sections: N/A
Templates requiring updates:
  - .specify/templates/plan-template.md ✅ compatible
  - .specify/templates/spec-template.md ✅ compatible
  - .specify/templates/tasks-template.md ✅ compatible
Follow-up TODOs: None
===========================
-->

# UAfricas Constitution

## Core Principles

### I. Français d'Abord

Tout le code, les commentaires, les variables, les noms de colonnes SQL,
les messages UI et la documentation DOIVENT être rédigés en français.

- Noms de variables, fonctions et composants en français (snake_case Rust,
  camelCase TypeScript, PascalCase composants Vue)
- Messages d'erreur et labels UI en français
- Commits et documentation technique en français
- Seules exceptions autorisées : mots-clés du langage, noms de
  bibliothèques tierces, termes techniques sans équivalent français
  consacré (JWT, CRUD, API, UUID)

**Justification** : Cohérence linguistique pour une plateforme panafricaine
francophone ; réduit l'ambiguïté entre code et domaine métier.

### II. Monorepo Cohérent

Le projet DOIT rester un monorepo unique contenant `uafricas_frontend/`
(Nuxt 4) et `uafricas_backend/` (Rust/Actix-Web 4).

- Aucune extraction en sous-dépôt sans justification documentée et
  approuvée
- Les contrats d'API entre frontend et backend DOIVENT être cohérents
  (types TS ↔ structs Rust ↔ schéma SQL)
- Les modifications cross-stack (ex. ajout d'un champ) DOIVENT être
  livrées dans le même commit ou la même PR

**Justification** : Garantit la cohérence des types et contrats entre
les couches ; simplifie le déploiement et le review.

### III. SQL Source de Vérité

Le schéma PostgreSQL (`uafricas_backend/doc/bd/schema.sql` et fichiers
`schemas/`) constitue la source de vérité unique pour le modèle de données.

- Toute interface TypeScript, struct Rust `FromRow` ou DTO DOIT refléter
  fidèlement le schéma SQL
- Les modifications de données DOIVENT commencer par le schéma SQL, puis
  se propager vers backend puis frontend
- Les mocks frontend (`app/mocks/`) DOIVENT respecter les types et
  contraintes du schéma SQL
- Conventions SQL obligatoires : UUID v4 PKs, soft deletion (`deleted_at`),
  TIMESTAMPTZ, snake_case français, enums PostgreSQL

**Justification** : Évite les dérives de modèle entre les couches ;
le SQL est le contrat le plus proche de la persistance réelle.

### IV. Sécurité par Défaut

Toute fonctionnalité DOIT intégrer la sécurité dès la conception, pas en
ajout ultérieur.

- Authentification JWT HS256 (access 15min + refresh 7j hashé SHA-256)
  via `jwt.rs` — pas de modification du mécanisme sans revue
- Bcrypt cost 12 pour les mots de passe — pas de réduction du coût
- Validation des entrées utilisateur côté backend (sanitize, types stricts)
- Pas de secrets en dur dans le code ; variables d'environnement
  obligatoires via `.env` (gitignored)
- Prévention OWASP Top 10 : injection SQL (requêtes paramétrées sqlx),
  XSS (échappement Vue natif), CSRF (tokens), upload sécurisé
  (sanitize-filename)
- CORS configuré explicitement via actix-cors

**Justification** : Plateforme multi-utilisateurs avec données
personnelles ; le coût de correction post-incident est prohibitif.

### V. Simplicité (YAGNI)

Le code DOIT être le plus simple possible pour satisfaire le besoin actuel.

- Pas d'abstraction prématurée : 3 lignes dupliquées valent mieux qu'une
  abstraction non justifiée
- Pas de feature flags ni de rétro-compatibilité artificielle — modifier
  directement le code
- Pas de sur-ingénierie : pas de patterns (Repository, Factory, etc.)
  sauf si la complexité le justifie concrètement
- Les composants Vue DOIVENT suivre le pattern existant
  (Hero/Card/Filters/Modal) sans inventer de nouvelles abstractions
- Un composable par domaine, pas de couche d'indirection supplémentaire

**Justification** : Vélocité de développement sur un projet en phase
de construction active ; la simplicité réduit la surface de bugs.

### VI. Tailwind CSS v4 (daisyUI v5 back-office uniquement)

L'UI DOIT utiliser Tailwind CSS v4 (CSS-first) sur l'ensemble du projet.
daisyUI v5 DOIT être utilisé **exclusivement dans le back-office**
(pages et composants admin). Le site public (vitrine, pages accessibles
sans authentification admin) NE DOIT PAS utiliser les classes daisyUI.

- **Site public** : Tailwind CSS v4 pur — composants custom construits
  avec les utility classes Tailwind, sans dépendance aux classes
  sémantiques daisyUI (`btn`, `card`, `modal`, etc.)
- **Back-office admin** : daisyUI v5 autorisé et recommandé pour
  accélérer le développement des interfaces d'administration
- Configuration via `@theme` dans `app/assets/css/main.css` uniquement —
  pas de `tailwind.config.ts`
- Plugin Vite `@tailwindcss/vite` ; daisyUI via `@plugin "daisyui"`
- Couleurs du thème : `custom-chocolat` (#A54A1C), `custom-green`
  (#228B22), `custom-gray`
- Polices : Oswald (titres), Open Sans (body)
- Si des résidus Tailwind v3 sont détectés dans un fichier en cours de
  modification, ils DOIVENT être migrés vers la syntaxe v4
- Pas de CSS custom en dehors de `main.css` sauf cas exceptionnel justifié

**Justification** : Le site public nécessite une identité visuelle
unique et sur mesure, non contrainte par les composants pré-stylés de
daisyUI. Le back-office, à usage interne, bénéficie de la productivité
de daisyUI sans impact sur l'image de marque.

### VII. Audit & Traçabilité

Toute mutation de données DOIT être auditée automatiquement.

- Utiliser `audit::log_action` (non-bloquant) pour chaque handler de
  mutation dans `src/handlers/`
- L'audit DOIT capturer : action, utilisateur, table, IP, user-agent,
  état avant/après (JSONB)
- Les endpoints admin d'audit (liste paginée + détail) DOIVENT rester
  fonctionnels et à jour
- ~100 mutations sont déjà instrumentées — toute nouvelle mutation DOIT
  suivre le même pattern

**Justification** : Plateforme à vocation institutionnelle nécessitant
transparence et traçabilité des actions administratives.

## Contraintes Techniques

- **Stack imposée** : Nuxt 4 (Vue 3 SSR) + TypeScript (frontend),
  Rust + Actix-Web 4 (backend), PostgreSQL 16 (BDD)
- **Gestionnaires de paquets** : pnpm (frontend), Cargo (backend) —
  pas de npm, yarn ou autre
- **10 schemas PostgreSQL bounded-context** : `shared`, `iam`,
  `marketplace`, `exchange`, `innovation`, `culture`, `afrolang`,
  `media_content`, `governance`, `country_profile` — tout nouveau
  domaine DOIT être rattaché à un schema existant ou en créer un
  nouveau avec justification
- **Upload** : stockage local `./uploads/` servi via actix-files —
  pas de service cloud sans migration approuvée
- **Pas de linting, testing ni CI/CD configuré** — à mettre en place
  progressivement sans bloquer le développement actuel
- **Icons** : FontAwesome exclusivement, via le plugin
  `app/plugins/fontawesome.ts`

## Workflow de Développement

- **Processus de modification** :
  1. Lire et comprendre le code existant avant toute modification
  2. Vérifier si un composant/composable similaire existe déjà
  3. Modifier le schéma SQL d'abord si le modèle change (Principe III)
  4. Propager les changements backend → frontend
  5. Vérifier la cohérence des types cross-stack
- **Backend** : toujours tuer le processus existant sur le port 8080
  avant de relancer (`kill $(lsof -i :8080 -t) 2>/dev/null`)
- **Docker** : `docker compose up -d` pour PostgreSQL + Adminer + LiveKit
- **Sous-agents parallèles** : utiliser des sous-agents en parallèle
  pour la recherche frontend/backend simultanée, l'exploration multi-
  fichiers et les vérifications post-modification
- **Commits** : messages en français, descriptifs, une modification
  logique par commit

## Governance

Cette constitution est le document de référence suprême pour toutes les
décisions techniques du projet UAfricas. En cas de conflit entre cette
constitution et toute autre pratique ou documentation, la constitution
prévaut.

**Processus d'amendement** :
1. Proposer la modification avec justification écrite
2. Documenter l'impact sur les artefacts existants
3. Mettre à jour la constitution et propager les changements
4. Incrémenter la version selon le versionnement sémantique :
   - MAJEUR : suppression ou redéfinition incompatible de principes
   - MINEUR : ajout de principe ou expansion significative
   - PATCH : clarifications, corrections de formulation

**Conformité** :
- Toute PR/review DOIT vérifier la conformité avec ces principes
- Toute complexité ajoutée DOIT être justifiée par rapport au
  Principe V (Simplicité)
- Le fichier `CLAUDE.md` à la racine sert de guide d'exécution
  opérationnel et DOIT rester synchronisé avec cette constitution

**Version**: 1.1.0 | **Ratified**: 2026-02-27 | **Last Amended**: 2026-02-27
