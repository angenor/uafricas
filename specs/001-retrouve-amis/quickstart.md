# Quickstart: Retrouve Amis

**Feature**: 001-retrouve-amis | **Date**: 2026-02-27

Guide de démarrage pour l'implémentation de la fonctionnalité Retrouve Amis.

---

## Prérequis

- Docker Compose actif (`docker compose up -d`), PostgreSQL 16, Adminer, LiveKit
- Frontend : `pnpm install` dans `uafricas_frontend/`
- Backend : Rust toolchain (Edition 2024) + `.env` configuré

## Ordre d'implémentation recommandé

L'implémentation suit le principe **III. SQL Source de Vérité** : schema SQL → backend Rust → frontend Nuxt.

### Étape 1 : Schema SQL

1. Créer `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql`
   - Créer le schema `retrouve_amis`
   - Créer les 7 enums
   - Créer les 6 tables (voir `data-model.md`)
   - Ajouter les index (GIN trigramme, GIN FTS, B-tree avec filtres)
2. Modifier `uafricas_backend/doc/bd/schemas/13_contraintes_inter_schemas.sql`
   - Ajouter les FK cross-schema (→ iam.utilisateur, → shared.pays)
3. Ajouter `est_trouvable` sur `iam.utilisateur`
   - `ALTER TABLE iam.utilisateur ADD COLUMN est_trouvable BOOLEAN NOT NULL DEFAULT FALSE;`
4. Ajouter le schema dans le trigger auto-update (`14_triggers.sql`)
   - Ajouter `'retrouve_amis'` à la liste des schemas
5. Créer la fonction de matching `retrouve_amis.calculer_correspondances()`
6. Référencer le fichier dans `schema.sql` via `\ir schemas/16_retrouve_amis.sql`

**Validation** : Relancer `docker compose down -v && docker compose up -d` pour recréer la BDD, vérifier via Adminer (http://localhost:8088).

### Étape 2 : Backend Rust (Models)

1. Créer `src/models/retrouve_amis.rs`, Structs publiques
   - `AvisRecherche`, `AvisRechercheDetail` (avec correspondances)
   - `Correspondance`, `CorrespondanceDetail`
   - `NotificationRetrouve`
   - `ParcoursTrouvable`
   - `TableauDeBord` (struct résumé)
   - DTOs de création/modification
2. Créer `src/models/admin/retrouve_amis.rs`, Structs admin
   - `AdminAvisRecherche`, `AdminAvisRechercheDetail`
   - `AdminSignalement`, `AdminSignalementDetail`
   - `AdminStatistiques`
3. Déclarer les modules dans `src/models/mod.rs` et `src/models/admin/mod.rs`

### Étape 3 : Backend Rust (Handlers publics)

1. Créer `src/handlers/retrouve_amis.rs`
   - CRUD avis de recherche (créer, lister, détail, modifier, clôturer)
   - Lister/détail correspondances + accepter/refuser
   - Signaler un avis
   - Notifications (lister, marquer lu, tout lire)
   - Tableau de bord
2. Ajouter les handlers profil trouvable dans `src/handlers/auth.rs` ou créer un handler dédié
   - PATCH trouvable, CRUD parcours
3. Implémenter la logique de matching dans le handler de création d'avis
   - Appeler `retrouve_amis.calculer_correspondances()` après insertion
   - Insérer les correspondances avec score >= 60
   - Créer les notifications associées

### Étape 4 : Backend Rust (Handlers admin)

1. Créer `src/handlers/admin/retrouve_amis.rs`
   - Lister/détail avis (avec filtres admin)
   - Changer état avis (suspension/réactivation)
   - Lister/détail signalements
   - Modérer signalements
   - Statistiques
2. Instrumenter toutes les mutations avec `audit::log_action`

### Étape 5 : Backend Rust (Routes)

1. Modifier `src/routes.rs`
   - Ajouter scope `/api/retrouve-amis` avec les handlers publics
   - Ajouter scope `/api/admin/retrouve-amis` avec les handlers admin
   - Ajouter routes profil trouvable dans le scope profil existant

### Étape 6 : Frontend (Composables)

1. Créer `app/composables/useRetrouvAmis.ts`
   - Pattern : types + constantes + composable hook
   - Méthodes : CRUD avis, correspondances, notifications, tableau de bord, profil trouvable
2. Créer `app/composables/useAdminRetrouvAmis.ts`
   - Étend `useAdmin()` comme base
   - Méthodes : lister/détail avis admin, signalements, modération, statistiques

### Étape 7 : Frontend (Composants)

1. Créer `app/components/retrouve-amis/`
   - `RetrouvAmisHero.vue` : Section hero de la page d'accueil
   - `AvisRechercheCard.vue` : Carte résumée d'un avis
   - `AvisRechercheForm.vue` : Formulaire multi-étapes (5 étapes)
   - `CorrespondanceCard.vue` : Carte de correspondance anonymisée
   - `CorrespondanceDetail.vue` : Détail + boutons accepter/refuser
   - `ScoreBadge.vue` : Badge visuel du score (couleur selon %)
   - `TableauDeBord.vue` : Dashboard résumé avec compteurs
   - `ProfilTrouvableForm.vue` : Gestion du parcours trouvable

### Étape 8 : Frontend (Pages publiques)

1. `app/pages/retrouve-amis/index.vue`, Landing page (Tailwind pur)
2. `app/pages/retrouve-amis/nouveau.vue`, Formulaire création
3. `app/pages/retrouve-amis/mes-recherches.vue`, Liste des avis
4. `app/pages/retrouve-amis/correspondances.vue`, Liste correspondances
5. `app/pages/retrouve-amis/correspondances/[id].vue`, Détail correspondance
6. Modifier `app/pages/profil.vue` : Ajouter section trouvable + parcours

### Étape 9 : Frontend (Pages admin)

1. `app/pages/admin/retrouve-amis/index.vue`, Liste avis (daisyUI)
2. `app/pages/admin/retrouve-amis/[id].vue`, Détail avis
3. `app/pages/admin/retrouve-amis/signalements.vue`, Modération

### Étape 10 : Navigation et intégration

1. Ajouter le lien "Retrouve Amis" dans la NavBar (`app/components/layout/NavBar.vue`)
2. Ajouter la section dans le sidebar admin (`app/components/admin/AdminSidebar.vue`)
3. Vérifier la cohérence des types cross-stack

---

## Commandes de développement

```bash
# BDD : recréer après modification du schema
docker compose down -v && docker compose up -d

# Backend : lancer après modification des handlers
kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run

# Frontend : dev server
cd uafricas_frontend && pnpm dev

# Adminer : vérifier le schema
open http://localhost:8088
```

---

## Fichiers clés à consulter

| Fichier | Raison |
|---------|--------|
| `data-model.md` | Modèle de données complet (tables, enums, contraintes) |
| `contracts/api-publique.md` | Endpoints publics avec DTOs |
| `contracts/api-admin.md` | Endpoints admin avec DTOs |
| `research.md` | Décisions techniques et alternatives considérées |
| `src/handlers/codimoi.rs` | Exemple de handler public avec FTS |
| `src/handlers/admin/annonces.rs` | Exemple de handler admin complet |
| `app/composables/useCodiMoi.ts` | Exemple de composable public |
| `app/composables/useAdminAnnonces.ts` | Exemple de composable admin |
