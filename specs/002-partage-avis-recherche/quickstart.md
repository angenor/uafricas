# Quickstart: Partage Public des Avis de Recherche

**Branch**: `002-partage-avis-recherche` | **Date**: 2026-03-02

## Prérequis

```bash
# 1. Démarrer PostgreSQL + services Docker
docker compose up -d

# 2. Vérifier la branche
git checkout 002-partage-avis-recherche
```

## Ordre d'implémentation recommandé

### Étape 1 : Schema SQL (source de vérité)

Modifier `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql` :

1. Ajouter les enums : `type_reponse_publique`, `etat_demande_retrait`, `source_signalement`
2. ALTER TABLE `avis_recherche` : ajouter `est_public`, `slug`, `date_publication_publique`, `compteur_partages`
3. ALTER TABLE `signalement` : ajouter `source`
4. ALTER TYPE `type_notification` : ajouter `reponse_publique`, `demande_retrait`
5. CREATE TABLE `reponse_publique`
6. CREATE TABLE `demande_retrait`
7. Créer les index

```bash
# Appliquer le schema (recréation complète en dev)
docker compose down -v && docker compose up -d
# Attendre ~5 secondes que PostgreSQL initialise
```

### Étape 2 : Backend : Modèles Rust

Fichiers à modifier/créer :
- `src/models/retrouve_amis.rs` : Ajouter structs + DTOs pour les nouvelles entités
- `src/models/admin/retrouve_amis.rs`, Ajouter structs admin pour demandes de retrait

### Étape 3 : Backend : Handlers publics (sans auth)

Créer `src/handlers/retrouve_amis_public.rs` :
- `detail_avis_public` : GET `/api/retrouve-amis/public/{slug}`
- `rechercher_avis_publics` : GET `/api/retrouve-amis/public/rechercher`
- `incrementer_partage` : POST `/api/retrouve-amis/public/{slug}/partage`

### Étape 4 : Backend : Handlers authentifiés

Modifier `src/handlers/retrouve_amis.rs` :
- `publier_avis` : PATCH `/api/retrouve-amis/avis/{id}/publier`
- `repondre_avis_public` : POST `/api/retrouve-amis/public/{slug}/repondre`
- `signaler_avis_public` : POST `/api/retrouve-amis/public/{slug}/signaler`
- `demander_retrait` : POST `/api/retrouve-amis/public/{slug}/demande-retrait`

Modifier `src/handlers/admin/retrouve_amis.rs` :
- `lister_demandes_retrait` : GET `/api/admin/retrouve-amis/demandes-retrait`
- `statuer_demande_retrait` : PATCH `/api/admin/retrouve-amis/demandes-retrait/{id}/statuer`

### Étape 5 : Backend : Routes

Modifier `src/routes.rs` :
- Ajouter les routes publiques (hors scope JWT)
- Ajouter les routes authentifiées (dans le scope JWT)
- Ajouter les routes admin (dans le scope admin)

### Étape 6 : Frontend : Page publique SSR

Créer `app/pages/retrouve-amis/public/[slug].vue` :
- SSR avec `useHead()` / `useSeoMeta()` pour Open Graph + Twitter Card
- Affichage conditionnel selon `etat` (actif, clôturé, suspendu, dépublié)
- Composants : `PagePublique.vue`, `BoutonsPartage.vue`, `FormulaireReponse.vue`, `DemandeRetrait.vue`

### Étape 7 : Frontend : Page de listing/recherche

Créer `app/pages/retrouve-amis/rechercher.vue` :
- Listing paginé des avis publics actifs
- Filtres : pays, ville, école
- Recherche full-text
- Composant : `CarteAvisPublic.vue`

### Étape 8 : Frontend : Toggle publication

Modifier `app/pages/retrouve-amis/mes-recherches.vue` :
- Ajouter interrupteur "Rendre public" par avis
- Afficher le lien public + compteur de partages

### Étape 9 : Frontend : Composable

Modifier `app/composables/useRetrouvAmis.ts` :
- Ajouter les fonctions pour les nouveaux endpoints
- Ajouter les types/interfaces pour les nouvelles entités

## Vérification rapide

```bash
# Backend : compiler et démarrer
cd uafricas_backend
kill $(lsof -i :8080 -t) 2>/dev/null
RUST_LOG=info cargo run

# Frontend : démarrer
cd uafricas_frontend
pnpm dev

# Test manuel :
# 1. Se connecter comme admin@test.com / Test1234
# 2. Créer un avis de recherche
# 3. Activer la visibilité publique
# 4. Ouvrir l'URL publique en navigation privée (non connecté)
# 5. Vérifier l'aperçu Open Graph : partager le lien sur WhatsApp
```

## Comptes de test

| Rôle | Email | Mot de passe |
|------|-------|-------------|
| Admin | admin@test.com | Test1234 |
| Standard | user@test.com | Test1234 |

## Points d'attention

- **Constitution VI** : Les pages publiques (`[slug].vue`, `rechercher.vue`) doivent utiliser Tailwind CSS v4 pur, PAS de classes daisyUI
- **Constitution VII** : Toute mutation (publier, répondre, signaler, retrait, modération) doit être auditée via `audit::log_action`
- **SEO** : Les balises `useHead()` / `useSeoMeta()` doivent être dans le `setup()` du composant (pas dans un `onMounted`) pour le SSR
- **Robots** : Ajouter `noindex, nofollow` pour les pages non-actives (suspendu, clôturé, dépublié)
