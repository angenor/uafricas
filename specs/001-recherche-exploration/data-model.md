# Data Model: Recherche et Exploration de l'Arbre

**Feature Branch**: `001-recherche-exploration`
**Date**: 2026-03-16

## Aucune modification SQL

Cette feature n'ajoute pas de nouvelles tables. Elle réutilise :
- Le graphe en mémoire (`arbre-complet`) pour recherche locale, filtres et chemin de parenté
- Les colonnes `nom_normalise`/`prenoms_normalise` et indexes GIN (Feature 4) pour recherche publique

## Nouveau endpoint backend

### GET /api/arbre/recherche-publique

Recherche dans tous les arbres (sauf celui de l'utilisateur).

**Query params** : `q` (terme de recherche, min 2 caractères)

**Réponse** : Liste de `PersonneResumePubliqueResponse` (max 20 résultats).

```
PersonneResumePubliqueResponse
├── nom: String
├── prenoms: Option<String>
├── naissance_annee: Option<i16>
├── naissance_lieu: Option<String>
├── genre: Option<String>
├── membre_id_anonymise: String ("Membre #XXXX")
├── score_similarite: f32 (score pg_trgm)
└── source: "autre_arbre"
```

## Structures côté client

### Résultat de recherche locale

```
ResultatRecherche
├── id: string (rattachement_id)
├── personne_id: string
├── nom: string
├── prenoms?: string
├── naissance_annee?: number
├── naissance_lieu?: string
├── source: 'mon_arbre'
```

### Chemin de parenté

```
CheminParente
├── source: NoeudArbre
├── cible: NoeudArbre
├── noeuds: NoeudArbre[] (chemin ordonné)
├── liens: string[] (IDs des liens sur le chemin)
├── description: string ("X est le grand-père de Y")
├── generations_montantes: number (vers le LCA)
├── generations_descendantes: number (depuis le LCA)
└── lca: NoeudArbre | null (ancêtre commun le plus proche)
```

### Filtre actif

```
FiltreArbre
├── type: 'geographique' | 'generationnel' | 'branche'
├── valeur: string | number | { parentId: string }
└── actif: boolean
```
