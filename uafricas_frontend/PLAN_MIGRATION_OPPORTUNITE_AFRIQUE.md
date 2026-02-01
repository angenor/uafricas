# Plan de Migration: `/opportunite-afrique` (Vue 3 → Nuxt 4)

## Résumé

Migration de la page Opportunités en Afrique depuis Vue 3 + Vite vers Nuxt 4, **sans backend Firebase** (utilisation de données mock).

---

## 1. Fichiers à Créer

| Fichier | Description |
|---------|-------------|
| `app/mocks/opportunite-afrique.ts` | Interfaces TypeScript + données mock + helpers |
| `app/pages/opportunite-afrique/index.vue` | Page liste avec grille, recherche et filtres |
| `app/pages/opportunite-afrique/[id].vue` | Page détail d'un pays |

## 2. Fichiers à Modifier

| Fichier | Modification |
|---------|--------------|
| `app/components/common/BreadcrumbNav.vue` | Ajouter label route `opportunite-afrique` |

---

## 3. Étapes d'Implémentation

### Étape 1: Interfaces TypeScript et Données Mock

**Fichier:** `app/mocks/opportunite-afrique.ts`

```typescript
export interface FichePays {
  id: string
  nom: string
  code: string                    // ISO 3166
  imageCouverture?: string
  slogan?: string
  superficie?: string
  population?: string
  capitale?: string
  monnaie?: string
  drapeauURL?: string
  emblemeURL?: string
  devise?: string
  langues: string[]
  ethnies: string[]
  region: Region
  derniereValidation: Date
  contributeursPrincipaux: string[]
  nombreContributions: number
}

export type Region =
  | 'Afrique Centrale'
  | 'Afrique de l\'Ouest'
  | 'Afrique de l\'Est'
  | 'Afrique du Nord'
  | 'Afrique Australe'
```

**Fonctions helper:**
- `getAllPays(): FichePays[]`
- `getPaysById(code: string): FichePays | undefined`
- `getPaysByRegion(region: Region): FichePays[]`
- `searchPays(term: string): FichePays[]`
- `getRegionsUniques(): Region[]`

**Pays mock à inclure (10):**
1. Sénégal - Afrique de l'Ouest
2. Côte d'Ivoire - Afrique de l'Ouest
3. Cameroun - Afrique Centrale
4. RDC - Afrique Centrale
5. Kenya - Afrique de l'Est
6. Éthiopie - Afrique de l'Est
7. Égypte - Afrique du Nord
8. Maroc - Afrique du Nord
9. Afrique du Sud - Afrique Australe
10. Nigeria - Afrique de l'Ouest

---

### Étape 2: Page Liste `/opportunite-afrique`

**Fichier:** `app/pages/opportunite-afrique/index.vue`

**Fonctionnalités:**
- Hero section avec gradient vert/jaune
- Breadcrumb navigation
- Barre de recherche avec debounce (500ms)
- Filtre par région (dropdown)
- Compteur "X pays trouvés"
- Grille responsive: 1-2-3-4 colonnes
- Cards: image, badge région, nom, slogan, capitale, population
- États: loading (skeletons), error, empty
- Bouton "Voir plus" (pagination)

---

### Étape 3: Page Détail `/opportunite-afrique/[id]`

**Fichier:** `app/pages/opportunite-afrique/[id].vue`

**Fonctionnalités:**
- Hero avec image de couverture + overlay
- Breadcrumb à 3 niveaux
- Layout 2/3 + 1/3:
  - **Colonne principale:**
    - Card: Informations générales (capitale, population, superficie, monnaie)
    - Card: Culture et langues (tags)
    - Card: Symboles nationaux (devise, drapeau)
  - **Sidebar:**
    - Card: Statistiques (région, contributions, dernière validation)
    - Card: Actions (proposer modification, retour)

---

### Étape 4: Mise à jour BreadcrumbNav

Ajouter dans `routeLabels`:
```typescript
'opportunite-afrique': 'Opportunités en Afrique'
```

---

## 4. Arborescence Finale

```
app/
├── pages/
│   └── opportunite-afrique/
│       ├── index.vue          # Liste paginée
│       └── [id].vue           # Détail par code pays
├── components/
│   └── common/
│       └── BreadcrumbNav.vue  # (mis à jour)
└── mocks/
    └── opportunite-afrique.ts # Interfaces + données + helpers
```

---

## 5. Points d'Attention

1. **Pas de Firebase:** Données 100% mock, pas d'appel backend
2. **Pagination simplifiée:** Tout charger côté client puis filtrer
3. **Images:** URLs Unsplash ou placeholder
4. **Responsive:** Tester grille 1→4 colonnes
5. **SEO:** `useHead()` avec titre dynamique sur page détail
6. **Patterns:** Suivre les patterns existants (facultes, formations)

---

## 6. Ordre d'Exécution

1. ✅ Créer `app/mocks/opportunite-afrique.ts` (interfaces + données)
2. ✅ Créer `app/pages/opportunite-afrique/index.vue` (page liste)
3. ✅ Créer `app/pages/opportunite-afrique/[id].vue` (page détail)
4. ✅ Mettre à jour `BreadcrumbNav.vue`
5. ✅ Tester les routes et la navigation
