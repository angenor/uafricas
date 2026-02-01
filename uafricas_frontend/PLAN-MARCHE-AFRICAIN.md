# Plan de Migration: `/marche-africain` (Vue 3 vers Nuxt 4)

## Résumé

Migration complète de la fonctionnalité Marché Africain depuis Vue 3 + Vite + Firebase vers Nuxt 4, **sans backend Firebase** (utilisation de données mock).

---

## 1. Fichiers Sources Identifiés

| Fichier Source | Description | Lignes |
|----------------|-------------|--------|
| `src/views/CentreCulturel/PromotionValeur/Marche/index.vue` | Page liste avec Hero, recherche, filtres latéraux, grille, pagination | ~924 |
| `src/views/CentreCulturel/PromotionValeur/Marche/show.vue` | Page détail d'une annonce | ~230 |
| `src/views/CentreCulturel/PromotionValeur/Marche/create.vue` | Modal de création (formulaire complet avec upload image) | ~1126 |

### Routes Source:
- `/marche-africain` → Liste des annonces
- `/marche-africain/:id` → Détail d'une annonce

---

## 2. Structure des Fichiers à Créer

```
app/
├── mocks/
│   └── marche-africain.ts              # Interfaces + données + helpers
├── components/
│   └── marche/
│       ├── MarcheHero.vue              # Hero section avec recherche
│       ├── MarcheFilters.vue           # Filtres latéraux (desktop)
│       ├── MarcheCategoryButtons.vue   # Boutons catégories
│       └── AnnonceCard.vue             # Carte d'une annonce
└── pages/
    └── marche-africain/
        ├── index.vue                   # Page liste
        └── [id].vue                    # Page détail
```

---

## 3. Ordre d'Implémentation

| Étape | Fichier | Priorité | Dépendances |
|-------|---------|----------|-------------|
| 1 | `app/mocks/marche-africain.ts` | Haute | Aucune |
| 2 | `app/components/marche/AnnonceCard.vue` | Haute | mocks |
| 3 | `app/components/marche/MarcheHero.vue` | Haute | Aucune |
| 4 | `app/components/marche/MarcheCategoryButtons.vue` | Moyenne | mocks |
| 5 | `app/components/marche/MarcheFilters.vue` | Moyenne | mocks |
| 6 | `app/pages/marche-africain/index.vue` | Haute | composants 2-5 |
| 7 | `app/pages/marche-africain/[id].vue` | Haute | mocks |
| 8 | Mise à jour `BreadcrumbNav.vue` | Basse | Aucune |

---

## 4. Détail de Chaque Fichier

### 4.1. `app/mocks/marche-africain.ts`

**Types à définir:**
```typescript
export type TypeEchange = 'Vente' | 'Troc' | 'Don'
export type Categorie = 'Agriculture' | 'Informatique' | 'Immobilier' | 'Voitures' | 'Electronique' | 'Formation'
export type Devise = 'XOF' | 'EUR' | 'NGN' | 'USD'

export interface Annonce {
  id: string
  titre: string
  description: string
  type_echange: TypeEchange
  categorie: Categorie
  prix: number
  devise: Devise
  pays: string
  ville: string
  tel: string
  photo_url: string
  minQty?: number
  user: { uid: string; email: string; nom: string; prenom: string }
  created_at: Date
  updated_at?: Date
}

export interface FiltresAnnonce {
  categorie: Categorie | 'Tout'
  typesEchange: TypeEchange[]
  prixMin: number | null
  prixMax: number | null
  recherche: string
  tri: 'recent' | 'price-asc' | 'price-desc'
}
```

**Données mock:** 12-15 annonces variées (Agriculture, Informatique, Immobilier, Voitures, Electronique, Formation)

**Fonctions utilitaires:**
- `getAllAnnonces()`, `getAnnonceById()`, `rechercherAnnonces()`, `formatPrix()`, `formatDate()`

---

### 4.2. `app/components/marche/AnnonceCard.vue`

- Image avec aspect-ratio 16:9
- Badge type d'échange (Vente/Troc/Don) avec couleurs distinctes
- Localisation (pays)
- Titre (2 lignes max)
- Prix formaté ou "Gratuit" pour les dons
- Hover: scale-105, shadow-xl
- Clic: navigation vers `/marche-africain/{id}`

---

### 4.3. `app/components/marche/MarcheHero.vue`

- Gradient: `from-emerald-600 via-green-600 to-teal-600`
- Titre: "Marché **Africain**"
- Barre de recherche avec dropdown catégories
- Bouton "Publier une annonce"

---

### 4.4. `app/components/marche/MarcheCategoryButtons.vue`

- Rangée de boutons pills horizontaux
- Catégories: Tout, Agriculture, Informatique, Immobilier, Voitures, Electronique, Formation
- Style actif/inactif distinct

---

### 4.5. `app/components/marche/MarcheFilters.vue`

- Card blanche sticky
- Checkboxes type d'échange avec compteurs
- Inputs fourchette de prix (min/max)
- Bouton "Réinitialiser les filtres"

---

### 4.6. `app/pages/marche-africain/index.vue`

- Hero avec recherche
- Breadcrumb
- Boutons catégories
- Layout: Filtres latéraux + Grille d'annonces
- Pagination (12 items/page)
- Tri (récent, prix croissant/décroissant)

---

### 4.7. `app/pages/marche-africain/[id].vue`

- Breadcrumb
- Info bar: pays, ville, date
- Prix en grand
- Titre
- Image principale
- Bouton "Je suis intéressé" (si authentifié) ou message de connexion
- Description
- Lien retour vers la liste

---

## 5. Points d'Attention

### Différences Nuxt vs Vue 3:

| Aspect | Vue 3 (Source) | Nuxt 4 (Cible) |
|--------|----------------|----------------|
| Routage | `<RouterLink>` | `<NuxtLink>` |
| Navigation | `router.push()` | `navigateTo()` |
| SEO | Non géré | `useHead()` |
| Route params | `props.id` | `route.params.id` |
| Firebase | Firestore | Mock data |
| Auth | Firebase Auth | `useUserStore()` |

### Simplifications (sans backend):

1. Données proviennent des mocks (pas de Firebase)
2. Images: URLs Unsplash/placeholder
3. Pagination côté client
4. Auth simulée avec `useUserStore()`
5. Création d'annonce: message "Fonctionnalité bientôt disponible"

---

## 6. Arborescence Finale

```
app/
├── components/
│   ├── common/
│   │   └── BreadcrumbNav.vue      # (modifié: +1 label)
│   └── marche/
│       ├── AnnonceCard.vue        # ~90 lignes
│       ├── MarcheHero.vue         # ~120 lignes
│       ├── MarcheCategoryButtons.vue  # ~50 lignes
│       └── MarcheFilters.vue      # ~100 lignes
├── mocks/
│   └── marche-africain.ts         # ~350 lignes
└── pages/
    └── marche-africain/
        ├── index.vue              # ~250 lignes
        └── [id].vue               # ~180 lignes
```

**Total estimé:** ~1150 lignes de code (vs ~2280 lignes source)

---

## 7. Critères de Validation

- [ ] Route `/marche-africain` affiche la liste des annonces
- [ ] Recherche textuelle filtre les résultats
- [ ] Filtres par catégorie fonctionnent
- [ ] Filtres latéraux (type, prix) fonctionnent
- [ ] Tri par prix et date fonctionne
- [ ] Pagination affiche 12 items par page
- [ ] Clic sur carte navigue vers `/marche-africain/{id}`
- [ ] Page détail affiche toutes les infos
- [ ] Bouton intérêt conditionnel selon auth
- [ ] SEO: titre dynamique sur page détail
- [ ] Responsive: mobile, tablet, desktop
- [ ] Animations AOS au scroll
