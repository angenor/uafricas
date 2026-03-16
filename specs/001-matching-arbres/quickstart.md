# Quickstart : Matching et Découverte de Parents

## Prérequis

- Features 1-3 déployées (CRUD + visualisation + édition)
- Docker PostgreSQL avec extension pg_trgm activée
- Backend Rust + Frontend Nuxt en cours d'exécution
- **Deux comptes utilisateurs** avec des arbres contenant des personnes aux noms/dates/lieux similaires

## Extensions PostgreSQL requises

```sql
CREATE EXTENSION IF NOT EXISTS pg_trgm;
```

## Fichiers à créer/modifier

### Backend (Rust)

| Fichier | Action | Description |
|---------|--------|-------------|
| `doc/bd/schemas/24_matching.sql` | Créer | Tables suggestions_correspondance + demandes_contact + colonnes normalisées + indexes GIN |
| `src/models/matching.rs` | Créer | Structs: SuggestionCorrespondance, DemandeContact, DTOs response |
| `src/handlers/matching.rs` | Créer | 7 handlers: lister, confirmer, rejeter, branches, demande-contact, accepter, refuser |
| `src/services/matching.rs` | Créer | Algorithme: normaliser_nom(), matching_rapide(), matching_profond(), calculer_score() |
| `src/handlers/arbre_genealogique.rs` | Modifier | Ajouter normalisation + tokio::spawn matching dans creer_personne |
| `src/models/arbre_genealogique.rs` | Modifier | Ajouter champs nom_normalise/prenoms_normalise |
| `src/routes.rs` | Modifier | Ajouter scope /decouvertes avec les 7 routes |
| `src/handlers/mod.rs` | Modifier | pub mod matching |
| `src/models/mod.rs` | Modifier | pub mod matching |
| `src/services/mod.rs` | Modifier | pub mod matching |

### Frontend (Nuxt 4 / Vue 3)

| Fichier | Action | Description |
|---------|--------|-------------|
| `app/pages/arbre-genealogique/decouvertes.vue` | Créer | Page Découvertes (3 sections + actions) |
| `app/composables/useDecouvertes.ts` | Créer | API wrapper pour les 7 endpoints |
| `app/components/arbre-genealogique/CarteSuggestion.vue` | Créer | Carte suggestion (score, critères, boutons confirmer/rejeter) |
| `app/components/arbre-genealogique/SectionDecouvertes.vue` | Créer | Section paginée (en_attente, en_cours, confirmees) |
| `app/mocks/matching.ts` | Créer | Types TS + données mock |
| `app/pages/arbre-genealogique/visualisation.vue` | Modifier | Afficher branches découvertes (nœuds d'autres arbres) |
| `app/components/arbre-genealogique/NoeudPersonne.vue` | Modifier | Style visuel "branche découverte" (opacité, bordure pointillée) |

## Scénario de vérification

1. Créer **2 comptes** : userA@test.com et userB@test.com
2. Sur le compte A : ajouter "Diallo Ibrahim, né en 1850 à Ségou"
3. Sur le compte B : ajouter "Diallo Ibrahim, né en 1848 à Ségou"
4. Vérifier que le matching détecte la correspondance (score > 55%)
5. Sur le compte A : naviguer vers `/arbre-genealogique/decouvertes`
6. Vérifier la suggestion avec le score et les critères
7. Confirmer la suggestion (compte A)
8. Se connecter sur le compte B → confirmer aussi
9. Vérifier que les branches de l'arbre A sont visibles dans l'arbre de B et vice-versa
10. Tester la demande de contact → accepter → vérifier que les coordonnées sont visibles
