# Quickstart : Vidafrica

**Branch**: `004-vidafrica-sous-titres`

## Prérequis

- Docker compose lancé (`docker compose up -d` pour PostgreSQL)
- Backend Rust compilable (`cargo build`)
- Frontend Nuxt 4 fonctionnel (`pnpm dev`)

## Ordre d'implémentation recommandé

### Étape 1 : Schema SQL
1. Créer le fichier `uafricas_backend/doc/bd/schemas/27_vidafrica.sql`
2. Ajouter l'enum `langue_sous_titre` et les 4 tables (`video`, `piste_sous_titre`, `segment_sous_titre`, `timing_mot`)
3. Ajouter `\ir schemas/27_vidafrica.sql` dans `schema.sql`
4. Recréer la base : `docker compose down -v && docker compose up -d`

### Étape 2 : Backend : Models
1. Créer `src/models/admin/vidafrica.rs`, structs FromRow, DTOs, COLONNES
2. Créer `src/models/vidafrica.rs` : structs publiques
3. Déclarer les modules dans `src/models/admin/mod.rs` et `src/models/mod.rs`

### Étape 3 : Backend : Handlers Admin
1. Créer `src/handlers/admin/vidafrica.rs`, CRUD vidéos + pistes + segments + timings mot
2. Déclarer dans `src/handlers/admin/mod.rs`
3. Enregistrer les routes dans `src/routes.rs`

### Étape 4 : Backend : Handlers Public
1. Créer `src/handlers/vidafrica.rs` : lister vidéos, détail, sous-titres par langue
2. Déclarer dans `src/handlers/mod.rs`
3. Enregistrer les routes publiques dans `src/routes.rs`

### Étape 5 : Frontend : Composables + Mock
1. Créer `app/mocks/vidafrica.ts` : interfaces + données mock
2. Créer `app/composables/useAdminVidafrica.ts`, CRUD admin
3. Créer `app/composables/useVidafrica.ts`, lecture publique

### Étape 6 : Frontend : Admin Pages
1. Créer `app/pages/admin/vidafrica/index.vue`, liste des vidéos
2. Créer `app/pages/admin/vidafrica/create.vue`, formulaire création
3. Créer `app/pages/admin/vidafrica/[id].vue`, édition + gestion sous-titres + tap-to-mark

### Étape 7 : Frontend : Composant Lecteur Karaoké
1. Créer `app/components/vidafrica/VidafricaLecteur.vue`, lecteur vidéo + overlay karaoké
2. Implémenter la synchronisation `requestAnimationFrame` + `currentTime`
3. Implémenter le sélecteur de langue

### Étape 8 : Frontend : Page Publique
1. Créer `app/pages/vidafrica/index.vue`, catalogue avec filtres + recherche
2. Créer `app/pages/vidafrica/[slug].vue`, page de lecture vidéo

## Vérification rapide

```bash
# Backend
kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run

# Frontend
pnpm dev

# Test admin : créer une vidéo
# 1. Se connecter sur http://localhost:3000/login (admin@test.com / Test1234)
# 2. Aller sur /admin/vidafrica
# 3. Créer une vidéo avec un fichier MP4
# 4. Ajouter une piste de sous-titres en français
# 5. Saisir des segments + utiliser tap-to-mark
# 6. Publier la vidéo
# 7. Vérifier sur /vidafrica que la vidéo apparaît avec l'effet karaoké
```
