# Quickstart : Visualisation de l'Arbre Généalogique

## Prérequis

- Docker (PostgreSQL) : `docker compose up -d`
- Backend Rust en cours : `kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run` (depuis `uafricas_backend/`)
- Frontend Nuxt : `pnpm dev` (depuis `uafricas_frontend/`)
- Compte test : `admin@test.com` / `Test1234`

## Dépendances à installer

```bash
cd uafricas_frontend
pnpm add @vue-flow/core @vue-flow/controls @vue-flow/minimap relatives-tree
```

## Fichiers à créer/modifier

### Backend (Rust)

| Fichier | Action | Description |
|---------|--------|-------------|
| `src/models/arbre_genealogique.rs` | Modifier | Ajouter `ArbreCompletResponse`, `PersonneNoeud`, `LienArbreResponse` |
| `src/handlers/arbre_genealogique.rs` | Modifier | Ajouter handler `obtenir_arbre_complet` |
| `src/routes.rs` | Modifier | Ajouter route `.route("/arbre-complet", web::get().to(...))` |

### Frontend (Nuxt 4 / Vue 3)

| Fichier | Action | Description |
|---------|--------|-------------|
| `app/composables/useArbreGenealogique.ts` | Modifier | Ajouter `obtenirArbreComplet()` |
| `app/mocks/arbre-genealogique.ts` | Modifier | Ajouter types + mock pour arbre complet |
| `app/pages/arbre-genealogique/visualisation.vue` | Créer | Page de visualisation (route `/arbre-genealogique/visualisation`) |
| `app/components/arbre-genealogique/ArbreGraphe.vue` | Créer | Composant principal vue-flow avec zoom/pan |
| `app/components/arbre-genealogique/NoeudPersonne.vue` | Créer | Nœud custom vue-flow (photo, nom, dates) |
| `app/components/arbre-genealogique/PanneauPersonne.vue` | Créer | Panneau contextuel (mini-fiche + bouton détail) |
| `app/components/arbre-genealogique/BarreOutils.vue` | Créer | Barre de modes (Complet/Ascendant/Descendant) + réinitialiser |
| `app/composables/useLayoutArbre.ts` | Créer | Logique de conversion données API → positions vue-flow via relatives-tree |
| `app/pages/arbre-genealogique/index.vue` | Modifier | Ajouter bouton/lien vers `/arbre-genealogique/visualisation` |

## Architecture des composants

```
visualisation.vue (page)
├── BarreOutils.vue (modes + reset)
├── ArbreGraphe.vue (vue-flow wrapper)
│   ├── NoeudPersonne.vue (custom node × N)
│   └── [edges vue-flow natifs avec styles custom]
└── PanneauPersonne.vue (mini-fiche latérale/bottom sheet)
```

## Flux de données

```
1. Page charge → useArbreGenealogique.obtenirArbreComplet()
2. Données API → useLayoutArbre.calculerLayout(personnes, liens, personnecentree, mode)
3. relatives-tree calcule positions → conversion en nodes/edges vue-flow
4. vue-flow rend le graphe avec zoom/pan/touch
5. Clic nœud → recentrer vue + ouvrir PanneauPersonne
6. Changement mode → recalculer layout avec filtre (ascendant/descendant/complet)
```

## Vérification rapide

1. Se connecter avec `admin@test.com`
2. Ajouter 5+ personnes via `/arbre-genealogique` (Feature 1)
3. Créer des liens parent-enfant et conjoint
4. Naviguer vers `/arbre-genealogique/visualisation`
5. Vérifier : graphe affiché, clic recentre, modes fonctionnent, zoom tactile OK
