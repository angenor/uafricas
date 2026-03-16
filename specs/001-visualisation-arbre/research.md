# Research: Visualisation et Navigation de l'Arbre Généalogique

**Date**: 2026-03-15
**Feature Branch**: `001-visualisation-arbre`

## Décision 1 : Bibliothèque de rendu graphique

**Décision** : vue-flow (`@vue-flow/core`) + relatives-tree (calcul de layout)

**Raisonnement** :
- **vue-flow** est natif Vue 3, utilise des composants Vue comme nœuds custom (notre `PersonneCard.vue` existant peut être adapté). Zoom/pan/touch intégrés via d3-zoom interne. Transitions animées via `setCenter()`. ~47 kB gzip.
- **relatives-tree** est une bibliothèque dédiée au calcul de positions pour arbres généalogiques : gestion des couples côte à côte, multi-parents, alignement générationnel. ~3 kB gzip. MIT.
- L'architecture est : `relatives-tree` calcule les positions → `vue-flow` rend les nœuds/arêtes avec interactions.

**Alternatives évaluées** :
- **D3.js (d3-hierarchy + d3-zoom)** : `d3-hierarchy` est conçu pour des arbres simples (un parent par nœud), pas des DAGs familiaux (2 parents, conjoints). Nécessiterait un algorithme de layout custom conséquent. API impérative en conflit avec la réactivité Vue.
- **SVG/Canvas custom** : Zéro dépendance mais réinventer le zoom/pan tactile (200-500 lignes non triviales) + l'algorithme de layout familial. Viole YAGNI.
- **Cytoscape.js** : 250 kB gzip, API impérative non-Vue, conçu pour graphes scientifiques. Surdimensionné.
- **GoJS** : Licence commerciale ($3 995+). Éliminé.

## Décision 2 : Endpoint API dédié pour l'arbre complet

**Décision** : Créer un nouvel endpoint `GET /api/arbre/arbre-complet` retournant toutes les personnes et tous les liens de l'arbre de l'utilisateur en un seul appel.

**Raisonnement** :
- L'endpoint `lister_personnes` existant est paginé et ne retourne pas les liens familiaux — inutilisable pour construire un graphe.
- L'endpoint `obtenir_personne/{id}` retourne les liens d'une seule personne (un niveau) — nécessiterait N appels pour N personnes (problème N+1).
- Un endpoint dédié fait un seul aller-retour réseau, simplifie le frontend et permet une construction de graphe directe côté client.
- La volumétrie typique (5-200 personnes) ne justifie pas de pagination pour cet endpoint.

**Alternatives évaluées** :
- **Utiliser les endpoints existants** : Multiples appels API, complexité client, latence cumulée. Rejeté.
- **Endpoint avec récursivité SQL** : Plus complexe côté backend pour un gain minimal — le chargement complet est suffisant pour la taille attendue des arbres.

## Décision 3 : Stratégie SSR

**Décision** : Rendu client uniquement (`<ClientOnly>`) avec skeleton de chargement.

**Raisonnement** :
- La visualisation d'arbre est intrinsèquement interactive (zoom, pan, clic). Aucune valeur SEO à rendre les nœuds côté serveur.
- vue-flow utilise d3-zoom qui nécessite `window`/`document`.
- Le chargement des données se fait dans le composable (SSR-compatible), seul le rendu graphique est client-only.
- Un skeleton placeholder assure une bonne expérience de chargement.

**Alternatives évaluées** :
- **SSR complet** : Impossible avec vue-flow (dépendance DOM). Hydration mismatch garantie.
- **SSR partiel (données + placeholder statique)** : Complexité supplémentaire sans bénéfice utilisateur. La page est derrière authentification (pas de SEO).

## Décision 4 : Affichage progressif par génération

**Décision** : Charger toutes les données en un appel, mais n'afficher que 3 générations autour de la personne centrée. Expansion à la demande.

**Raisonnement** :
- Le chargement réseau est unique (endpoint arbre-complet), mais le rendu est filtré côté client.
- Afficher 200 nœuds simultanément dégrade la performance et la lisibilité sur mobile.
- Le filtrage par génération se fait en traversant le graphe côté frontend à partir de la personne centrée.
- Les boutons « voir plus » aux extrémités permettent d'étendre progressivement.

**Alternatives évaluées** :
- **Tout afficher** : Problèmes de performance et lisibilité au-delà de 50 nœuds, surtout sur mobile.
- **Chargement serveur progressif** : Complexité backend (endpoint par profondeur) pour un gain minimal — les données complètes font < 50 kB pour 200 personnes.

## Décision 5 : Panneau contextuel (mini-fiche)

**Décision** : Un clic sur un nœud recentre la vue et ouvre un panneau latéral/flottant affichant les informations clés de la personne + bouton « Voir détail ».

**Raisonnement** :
- Sépare l'action de navigation dans l'arbre (clic = recentrer + info) de la navigation vers une autre page (bouton explicite).
- Le panneau affiche : photo, nom complet, dates, lieu, nombre de liens (parents/enfants/conjoints).
- Sur mobile, le panneau s'affiche en bas de l'écran (bottom sheet pattern).
- Sur desktop, le panneau s'affiche à droite de la visualisation.

## Dépendances npm à ajouter

| Package | Version | Taille gzip | Rôle |
|---------|---------|-------------|------|
| `@vue-flow/core` | ^1.48 | ~47 kB | Rendu graphe + zoom/pan/touch |
| `@vue-flow/controls` | ^1.1 | ~2 kB | Boutons zoom +/- et réinitialisation |
| `@vue-flow/minimap` | ^1.5 | ~3 kB | Mini-carte de navigation (optionnel) |
| `relatives-tree` | ^1.1 | ~3 kB | Calcul layout arbre généalogique |

**Total** : ~55 kB gzip — acceptable pour une page feature.
