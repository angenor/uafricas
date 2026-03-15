# Quickstart: 003-retrouve-amis-public

**Branch**: `003-retrouve-amis-public`

## Prérequis

```bash
# S'assurer que Docker tourne (PostgreSQL + Adminer)
docker compose up -d

# S'assurer d'être sur la bonne branche
git checkout 003-retrouve-amis-public
```

## Ordre d'implémentation

### 1. Schéma SQL

Modifier `uafricas_backend/doc/bd/schemas/16_retrouve_amis.sql` :
- Ajouter les 2 nouveaux enums (`genre_personne`, `type_relation_recherche`)
- Ajouter les 14 nouvelles colonnes à `avis_recherche`
- Modifier `est_public DEFAULT TRUE`
- Ajouter les contraintes CHECK
- Ajouter les index
- Mettre à jour le calcul du `search_vector`

Appliquer le schéma :
```bash
# Recréer la BDD complète (dev uniquement)
docker compose down -v && docker compose up -d
```

### 2. Backend Rust

Fichiers à modifier :
- `src/models/retrouve_amis.rs` : ajouter les champs aux structs `AvisRecherche`, `CreerAvisRecherche`, réponses
- `src/handlers/retrouve_amis.rs` : passer `creer_avis` et `modifier_avis` en multipart, publication auto, upload photo
- `src/handlers/retrouve_amis_public.rs` : inclure les nouveaux champs dans les réponses publiques
- `src/routes.rs` : supprimer la route `publier_avis`

```bash
# Compiler et vérifier
cd uafricas_backend
cargo check

# Lancer
kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run
```

### 3. Frontend Nuxt

Fichiers à modifier :
- `app/composables/useRetrouvAmis.ts` : mettre à jour les interfaces TypeScript
- `app/components/retrouve-amis/AvisRechercheForm.vue` : réécrire avec 6 étapes
- `app/pages/retrouve-amis/index.vue` : afficher les avis publics
- `app/pages/retrouve-amis/rechercher.vue` : ajouter filtre type_relation
- `app/components/retrouve-amis/CarteAvisPublic.vue` : afficher les nouveaux champs

```bash
# Lancer le dev
cd uafricas_frontend
pnpm dev
```

## Vérification

1. Ouvrir `http://localhost:3000/retrouve-amis` en navigation privée → voir les avis
2. Se connecter → créer un avis avec le nouveau formulaire (6 étapes + photo)
3. Vérifier que l'avis apparaît immédiatement sur la page publique
4. Vérifier que les coordonnées ne sont PAS visibles sur la page publique
5. Tester les filtres (type de relation, recherche textuelle)

## Test utilisateurs

- **Admin** : `admin@test.com` / `Test1234`
- **Standard** : `user@test.com` / `Test1234`
