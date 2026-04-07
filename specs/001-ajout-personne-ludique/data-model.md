# Data Model: Ajout de personne ludique

**Branch**: `001-ajout-personne-ludique` | **Date**: 2026-04-06

## Aucune modification du modèle de données

Cette feature est purement frontend (UI/UX). Aucune table, colonne ou schema PostgreSQL n'est modifié.

### Entités existantes réutilisées

**Personne** (schema `arbre_genealogique`) :
- `id` : UUID v4 (PK)
- `nom` : TEXT NOT NULL
- `prenoms` : TEXT
- `genre` : ENUM ('masculin', 'feminin', 'autre', 'non_precise')
- `naissance` : DATE PARTIELLE (annee, mois, jour)
- `naissance_lieu` : TEXT
- `est_decede` : BOOLEAN
- `photo_url` : TEXT
- `created_at` : TIMESTAMPTZ

### DTOs frontend existants réutilisés

```typescript
// Création — utilisé par le wizard ET le formulaire classique
interface CreerPersonneForm {
  nom: string              // obligatoire
  prenoms?: string
  genre?: Genre
  naissance?: DatePartielle
  naissance_lieu?: string
  est_decede?: boolean
}

// Le wizard produit exactement ce même DTO en sortie
```

### Entités frontend uniquement (état local du wizard)

**ÉtapeWizard** (pas persisté, état composant) :
- `ordre` : number (1-7, incluant récapitulatif)
- `champ` : string (clé du champ associé)
- `obligatoire` : boolean
- `texteAccroche` : string (dynamique selon contexte)
- `texteTransition` : string (dynamique selon réponses précédentes)

**ContexteAjout** (props du composant wizard) :
- `typeLien?` : 'pere' | 'mere' | 'parent' | 'conjoint' | 'enfant'
- `personneLiee?` : { id: string, nom: string, prenoms?: string }

Ces structures existent uniquement en mémoire durant le parcours et ne sont pas persistées.
