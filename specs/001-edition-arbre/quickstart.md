# Quickstart : Édition Interactive de l'Arbre Généalogique

## Prérequis

- Feature 1 (CRUD personnes) et Feature 2 (visualisation) déployées et fonctionnelles
- Docker (PostgreSQL) : `docker compose up -d`
- Backend Rust : `RUST_LOG=info cargo run` (port 8080)
- Frontend Nuxt : `pnpm dev` (port 3000)
- Compte test : `admin@test.com` / `Test1234`
- Arbre avec au moins 3 personnes et 2 liens (créés via Feature 1)

## Aucune dépendance à installer

Pas de nouvelle dépendance npm ni Cargo. Cette feature réutilise les composants et API existants.

## Fichiers à créer/modifier

### Frontend uniquement (Nuxt 4 / Vue 3)

| Fichier | Action | Description |
|---------|--------|-------------|
| `app/components/arbre-genealogique/PanneauPersonne.vue` | Modifier | Ajouter boutons d'action + mode formulaire (fiche ↔ ajout ↔ modifier) |
| `app/components/arbre-genealogique/NoeudPersonne.vue` | Modifier | Ajouter badge indicateur d'incomplétude |
| `app/components/arbre-genealogique/BarreOutils.vue` | Modifier | Ajouter compteur branches incomplètes |
| `app/pages/arbre-genealogique/visualisation.vue` | Modifier | Gérer les flux ajout/modifier/supprimer + rechargement arbre |
| `app/composables/useLayoutArbre.ts` | Modifier | Ajouter fonction `calculerIncompletude()` |

## Architecture des modifications

```
visualisation.vue (page — orchestrateur des mutations)
├── BarreOutils.vue (+ compteur incomplétude)
├── ArbreGraphe.vue (inchangé)
│   └── NoeudPersonne.vue (+ badge incomplétude)
└── PanneauPersonne.vue (+ boutons actions + formulaire intégré)
    ├── Mode 'fiche' : mini-fiche existante + boutons d'action
    ├── Mode 'ajout' : PersonneForm.vue + sélecteur type de lien
    └── Mode 'modifier' : PersonneForm.vue pré-rempli
```

## Flux de vérification

1. Se connecter avec `admin@test.com`
2. Naviguer vers `/arbre-genealogique/visualisation`
3. Cliquer sur un nœud → vérifier que la mini-fiche affiche les boutons d'action
4. Cliquer « Ajouter un enfant » → vérifier que le formulaire remplace la mini-fiche
5. Remplir et valider → vérifier que le nouvel enfant apparaît dans l'arbre
6. Cliquer sur un nœud sans parents → vérifier le badge d'incomplétude
7. Cliquer « Modifier » sur un nœud → vérifier le formulaire pré-rempli
8. Cliquer « Supprimer » → vérifier la confirmation avec le nombre de liens
9. Vérifier le compteur de branches incomplètes dans la barre d'outils
