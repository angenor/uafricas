# Data Model: Visualisation de l'Arbre Généalogique

**Feature Branch**: `001-visualisation-arbre`
**Date**: 2026-03-15

## Entités existantes (Feature 1 : aucune modification SQL)

Cette feature est en lecture seule sur le modèle de données existant. Aucune migration SQL requise.

### Schema `arbre_genealogique` (existant)

```
┌──────────────┐       ┌──────────────┐       ┌──────────────────┐
│   personnes  │       │    arbres     │       │  rattachements   │
├──────────────┤       ├──────────────┤       ├──────────────────┤
│ id (PK)      │       │ id (PK)      │       │ id (PK)          │
│ nom          │◄──────│ utilisateur_id│       │ arbre_id (FK)    │──► arbres
│ prenoms      │       │ created_at   │       │ personne_id (FK) │──► personnes
│ genre        │       │ updated_at   │       │ ajoute_le        │
│ naissance_*  │       │ deleted_at   │       │ deleted_at       │
│ deces_*      │       └──────────────┘       └──────────────────┘
│ naissance_lieu│                                      │
│ deces_lieu   │                                      │
│ photo_url    │                              ┌──────────────────┐
│ cree_par     │                              │ liens_familiaux  │
│ created_at   │                              ├──────────────────┤
│ updated_at   │                              │ id (PK)          │
│ deleted_at   │                              │ arbre_id (FK)    │──► arbres
└──────────────┘                              │ rattachement_    │
                                              │   source_id (FK) │──► rattachements
                                              │ rattachement_    │
                                              │   cible_id (FK)  │──► rattachements
                                              │ type_lien        │ (pere/mere/parent/conjoint)
                                              │ created_at       │
                                              │ deleted_at       │
                                              └──────────────────┘
```

## Nouveaux types de réponse (backend)

### ArbreCompletResponse

Nouveau DTO pour l'endpoint `GET /api/arbre/arbre-complet` :

```
ArbreCompletResponse
├── arbre_id: UUID
├── personnes: Vec<PersonneNoeud>
│   ├── id: UUID (personne_id)
│   ├── rattachement_id: UUID
│   ├── nom: String
│   ├── prenoms: Option<String>
│   ├── genre: String
│   ├── naissance: Option<DatePartielle>
│   ├── deces: Option<DatePartielle>
│   ├── naissance_lieu: Option<String>
│   ├── deces_lieu: Option<String>
│   └── photo_url: Option<String>
└── liens: Vec<LienArbreResponse>
    ├── id: UUID (lien_familial id)
    ├── rattachement_source_id: UUID
    ├── rattachement_cible_id: UUID
    └── type_lien: String
```

**Notes** :
- `PersonneNoeud` inclut le `rattachement_id` car les liens référencent les rattachements, pas les personnes directement.
- `DatePartielle` (existant) : `{ annee?: i16, mois?: i16, jour?: i16 }`
- Pas de pagination : l'arbre complet est retourné en un seul appel.

## Structures de données frontend (côté client)

### Graphe en mémoire

```
NoeudArbre (construit côté client à partir de PersonneNoeud)
├── id: string (rattachement_id : identifiant unique dans l'arbre)
├── personne_id: string
├── nom: string
├── prenoms: string | null
├── genre: string
├── naissance: DatePartielle | null
├── deces: DatePartielle | null
├── naissance_lieu: string | null
├── deces_lieu: string | null
├── photo_url: string | null
├── parents: string[]     (rattachement_ids des parents)
├── enfants: string[]     (rattachement_ids des enfants)
├── conjoints: string[]   (rattachement_ids des conjoints)
└── generation: number    (calculé par traversée de graphe, 0 = personne centrée)
```

### Modes de filtrage

| Mode | Nœuds affichés | Logique |
|------|---------------|---------|
| Complet | 3 générations autour du centre | BFS depuis centre, profondeur ≤ 1 haut + 1 bas |
| Ascendant | Ancêtres uniquement | DFS remontant via `parents[]` depuis la personne |
| Descendant | Descendants uniquement | DFS descendant via `enfants[]` depuis la personne |

### Mapping vers vue-flow

```
NoeudArbre  →  vue-flow Node
├── id: rattachement_id
├── type: 'personne' (custom node component)
├── position: { x, y } (calculé par relatives-tree)
└── data: { ...NoeudArbre fields }

LienArbreResponse  →  vue-flow Edge
├── id: lien_id
├── source: rattachement_source_id
├── target: rattachement_cible_id
├── type: 'parentEnfant' | 'conjoint' (custom edge style)
└── animated: false
```
