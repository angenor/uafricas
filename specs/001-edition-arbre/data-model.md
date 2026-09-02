# Data Model: Édition Interactive de l'Arbre Généalogique

**Feature Branch**: `001-edition-arbre`
**Date**: 2026-03-16

## Entités existantes (aucune modification)

Cette feature est purement frontend. Aucune modification du schéma SQL, des modèles Rust ni des endpoints API. Elle réutilise intégralement les API de Feature 1 et les composants de Feature 2.

### APIs utilisées (Feature 1)

| Endpoint | Méthode | Usage dans cette feature |
|----------|---------|--------------------------|
| `/api/arbre/personnes` | POST | Créer une nouvelle personne depuis le formulaire |
| `/api/arbre/personnes/{id}` | PUT | Modifier une personne depuis le formulaire |
| `/api/arbre/personnes/{id}` | DELETE | Supprimer une personne après confirmation |
| `/api/arbre/liens` | POST | Créer le lien familial après ajout de la personne |
| `/api/arbre/arbre-complet` | GET | Recharger l'arbre complet après chaque mutation |

### Composants existants modifiés (Feature 2)

| Composant | Modification |
|-----------|-------------|
| `PanneauPersonne.vue` | Ajout des boutons d'action + état formulaire (fiche ↔ formulaire) |
| `NoeudPersonne.vue` | Ajout de l'indicateur d'incomplétude (badge) |
| `BarreOutils.vue` | Ajout du compteur de branches incomplètes |

## Structures de données frontend (côté client)

### État du panneau contextuel

```
ModePanneau = 'fiche' | 'ajout' | 'modifier'

ContexteAjout {
  personneSourceId: string      // rattachement_id de la personne cliquée
  typeAction: 'parent' | 'enfant' | 'conjoint'
  typeLienSuggere: TypeLien     // père/mère/parent/conjoint (pré-calculé)
}
```

### Calcul d'incomplétude

```
Pour chaque NoeudArbre dans le graphe :
  nbParents = noeud.parents.length
  estIncomplet = nbParents < 2
  parentManquant =
    si nbParents === 0 → "Parents manquants"
    si nbParents === 1 →
      si parent existant est de type 'pere' → "Mère manquante"
      si parent existant est de type 'mere' → "Père manquant"
      sinon → "Parent manquant"
    si nbParents >= 2 → null (complet)
```

### Flux de données : Ajout contextuel

```
1. Utilisateur clique nœud P → PanneauPersonne s'ouvre (mode: 'fiche')
2. Utilisateur clique "Ajouter un enfant" → PanneauPersonne passe en mode: 'ajout'
   ContexteAjout = { personneSourceId: P.id, typeAction: 'enfant', typeLienSuggere: 'pere'|'mere' }
3. Utilisateur remplit le formulaire et valide
4. Frontend appelle POST /api/arbre/personnes → obtient nouvelle personne + rattachement_id
5. Frontend appelle POST /api/arbre/liens → crée le lien (source=P, cible=nouveau, type=père|mère)
6. Frontend appelle GET /api/arbre/arbre-complet → recharge toutes les données
7. useLayoutArbre.calculerLayout() recalcule positions → arbre mis à jour
8. PanneauPersonne revient en mode: 'fiche' sur la nouvelle personne
```

### Flux de données : Suppression

```
1. Utilisateur clique "Supprimer" dans PanneauPersonne
2. Dialogue de confirmation avec comptage des liens (calculé côté client via graphe)
3. Si confirmé → DELETE /api/arbre/personnes/{id}
4. Recharger arbre-complet → recalculer layout
5. Si la personne supprimée était le centre → recentrer sur le nœud le plus connecté
6. PanneauPersonne se ferme
```
