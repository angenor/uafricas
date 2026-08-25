# Research: Recherche et Exploration de l'Arbre

**Date**: 2026-03-16
**Feature Branch**: `001-recherche-exploration`

## Décision 1 : Recherche locale côté client

**Décision** : Filtrage côté client sur le graphe en mémoire (données `arbre-complet` déjà chargées). Pas de nouvel endpoint.

**Raisonnement** :
- Les données complètes de l'arbre sont déjà en mémoire (endpoint `arbre-complet`). Pour 200 personnes, un filtre JavaScript est instantané (< 1ms).
- Recherche multi-critères : nom, prénoms, lieu, date, filtre `.filter()` avec normalisation de casse et diacritiques.
- Debounce 300ms pour éviter les calculs inutiles à chaque frappe.

## Décision 2 : Recherche publique via endpoint dédié

**Décision** : Nouvel endpoint `GET /api/arbre/recherche-publique?q=...` utilisant pg_trgm sur les colonnes normalisées (Feature 4).

**Raisonnement** :
- La recherche publique compare contre toutes les personnes de tous les arbres, impossible côté client.
- Réutilise les indexes GIN trigram et les colonnes `nom_normalise`/`prenoms_normalise` de Feature 4.
- Résultats anonymisés (pas d'identité du propriétaire, juste "Membre #XXXX").
- Limite à 20 résultats pour la performance.

## Décision 3 : Algorithme de chemin de parenté (LCA)

**Décision** : BFS bidirectionnel côté client pour trouver le plus court chemin entre deux personnes, puis calcul de la terminologie familiale.

**Raisonnement** :
- Le graphe est déjà en mémoire (NoeudArbre avec parents/enfants/conjoints).
- BFS bidirectionnel : part simultanément des deux personnes, se rejoint au LCA (Lowest Common Ancestor).
- Terminologie calculée par le nombre de générations entre chaque personne et le LCA : père/mère (1 gen), grand-père (2 gen), cousin (LCA = grand-parent commun, même génération), oncle (LCA = grand-parent, 1 génération d'écart).
- Pour 200 nœuds, le BFS est quasi-instantané (< 10ms).

**Table de terminologie** :
| Gens montantes A | Gens montantes B | Relation A→B |
|------------------|------------------|--------------|
| 0 | 1 | enfant |
| 0 | 2 | petit-enfant |
| 1 | 0 | parent |
| 2 | 0 | grand-parent |
| 1 | 1 | frère/sœur |
| 2 | 1 | oncle/tante |
| 1 | 2 | neveu/nièce |
| 2 | 2 | cousin(e) |
| 3 | 3 | cousin(e) au 2ème degré |
| n | n | cousin(e) au (n-1)ème degré |

## Décision 4 : Filtres côté client

**Décision** : Tous les filtres (géographique, générationnel, branche) sont des transformations côté client du graphe existant.

**Raisonnement** :
- Les données sont en mémoire. Filtrer = masquer des nœuds dans vue-flow.
- Filtre géographique : `noeud.naissance_lieu?.toLowerCase().includes(terme)`.
- Filtre générationnel : `Math.abs(noeud.generation - centre.generation) <= N`.
- Filtre branche : DFS depuis un parent spécifique, ne garder que les nœuds accessibles.

## Décision 5 : Interface de recherche unifiée

**Décision** : Un seul champ de recherche dans la barre d'outils avec toggle "Mon arbre / Tous les arbres" (clarification spec).

**Raisonnement** :
- En mode "Mon arbre" : recherche côté client instantanée.
- En mode "Tous les arbres" : appel API avec debounce 500ms.
- Le toggle est visuellement clair (deux boutons segmentés).
- Les résultats apparaissent dans un dropdown sous le champ.
