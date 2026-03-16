# Quickstart : Recherche et Exploration de l'Arbre

## Prérequis

- Features 1-4 déployées
- Arbre avec 10+ personnes (pour tester la recherche)
- Deux comptes avec des personnes similaires (pour tester la recherche publique)

## Fichiers à créer/modifier

### Backend (1 nouveau endpoint)

| Fichier | Action | Description |
|---------|--------|-------------|
| `src/handlers/matching.rs` | Modifier | Ajouter handler `recherche_publique` |
| `src/routes.rs` | Modifier | Ajouter route `/recherche-publique` |

### Frontend (principalement côté client)

| Fichier | Action | Description |
|---------|--------|-------------|
| `app/composables/useRechercheArbre.ts` | Créer | Recherche locale + publique + chemin parenté + filtres |
| `app/components/arbre-genealogique/ChampRecherche.vue` | Créer | Champ avec toggle + dropdown résultats |
| `app/components/arbre-genealogique/PanneauChemin.vue` | Créer | Affichage chemin de parenté |
| `app/components/arbre-genealogique/PanneauFiltres.vue` | Créer | Panneau de filtres combinables |
| `app/components/arbre-genealogique/BarreOutils.vue` | Modifier | Intégrer champ recherche + bouton filtres |
| `app/pages/arbre-genealogique/visualisation.vue` | Modifier | Intégrer recherche, chemin, filtres |

## Scénario de vérification

1. Se connecter, naviguer vers la visualisation
2. Taper "Diallo" dans le champ de recherche → résultats instantanés
3. Cliquer sur un résultat → vue centrée sur la personne
4. Basculer sur "Tous les arbres" → résultats de la base publique
5. Sélectionner deux personnes → demander le chemin de parenté
6. Vérifier la terminologie affichée (grand-père, cousin, etc.)
7. Appliquer un filtre géographique "Mali" → nœuds filtrés
8. Combiner avec filtre générationnel ±2 → double filtrage
9. Désactiver tous les filtres → arbre complet restauré
