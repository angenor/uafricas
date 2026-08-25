# Modèle de données : Demande pour devenir expert

**Feature**: 001-demande-expertise | **Date**: 2026-05-24 | **Schema**: `iam`

Aucune nouvelle table ni nouveau schema. Extension de la table existante `iam.expertise` et de la requête de mise à jour de profil. SQL = source de vérité (Principe III).

## Table `iam.expertise` (existante, modifiée)

### État actuel (rappel)

```sql
CREATE TABLE iam.expertise (
    id                          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    utilisateur_id              UUID NOT NULL UNIQUE       -- ← contrainte à modifier
                                REFERENCES iam.utilisateur(id) ON DELETE CASCADE,
    domaine                     iam.domaine_expertise NOT NULL,
    biographie                  TEXT NOT NULL,
    nb_annees_experience        INT NOT NULL CHECK (nb_annees_experience >= 0),
    rating                      NUMERIC(2,1) NOT NULL DEFAULT 0.0 CHECK (rating BETWEEN 0 AND 5),
    portfolio                   VARCHAR(500),
    situations_professionnelles iam.situation_professionnelle[] NOT NULL DEFAULT '{}',
    statut                      iam.statut_expertise NOT NULL DEFAULT 'en_attente',
    valide_par                  UUID REFERENCES iam.utilisateur(id) ON DELETE SET NULL,
    date_validation             TIMESTAMPTZ,
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at                  TIMESTAMPTZ,
    search_vector               TSVECTOR
);
```

### Modifications (migration `04b_iam_expertise.sql`)

1. **Supprimer** la contrainte `UNIQUE` totale sur `utilisateur_id` (la garder `NOT NULL`).
2. **Ajouter** un index unique partiel garantissant une seule demande active par membre :

```sql
CREATE UNIQUE INDEX idx_expertise_utilisateur_actif
    ON iam.expertise(utilisateur_id)
    WHERE deleted_at IS NULL;
```

3. **Ajouter** la colonne de commentaire de modération :

```sql
ALTER TABLE iam.expertise
    ADD COLUMN commentaire_admin TEXT;   -- NULL sauf en cas de refus
```

> Note : `valide_par` et `date_validation` sont réutilisés comme « admin ayant traité » et « date de décision » (validation **ou** refus). Aucune colonne supplémentaire (Principe V).

### Sémantique des colonnes (après migration)

| Colonne | Rôle dans cette feature |
|---------|-------------------------|
| `statut` | `en_attente` (initial) → `valide` ou `refuse` |
| `valide_par` | UUID de l'admin ayant pris la décision (validation ou refus) |
| `date_validation` | Date de la décision |
| `commentaire_admin` | Motif obligatoire en cas de refus (NULL sinon) |
| `deleted_at` | Soft-delete : positionné lors d'une re-soumission après refus (archivage) |

### Cycle de vie (états)

```text
            soumission                  validation (admin)
   (aucune) ───────────▶ en_attente ─────────────────────▶ valide ──▶ visible sur /experts
                              │
                              │ refus (admin, commentaire obligatoire)
                              ▼
                           refuse
                              │ re-soumission du membre
                              ▼
                    deleted_at = NOW() (archivée)  +  nouvelle ligne en_attente
```

Règles :
- **Invariant unicité** : au plus une ligne `deleted_at IS NULL` par `utilisateur_id` (index partiel).
- **FR-006** : blocage (409) si une demande active `en_attente` ou `valide` existe déjà.
- **FR-013** : seules les lignes `statut='valide' AND deleted_at IS NULL` dont `u.deleted_at IS NULL` apparaissent sur `/experts` (filtre déjà en place dans `lister_experts`).
- **FR-016** : la validation/refus ne s'applique que si `statut='en_attente'` (sinon « déjà traitée »).

### Règles de validation (entrées)

| Champ | Règle |
|-------|-------|
| `domaine` | Doit appartenir à `iam.domaine_expertise` (8 valeurs) ; mappé via `mapper_domaine_db` |
| `biographie` | Non vide ; borne max recommandée (ex. 5000 caractères) |
| `nb_annees_experience` | Entier ≥ 0 |
| `situations_professionnelles` | Chaque valeur ∈ `iam.situation_professionnelle` (5 valeurs) |
| `portfolio` | Optionnel ; URL ≤ 500 caractères |
| `commentaire_admin` (refus) | Obligatoire, non vide (ex. ≥ 10 caractères) |

## Table `iam.utilisateur` (existante), profil de base

Mise à jour via les endpoints profil existants. Extension requise : prise en charge de `pays_residence_id` dans `ModifierProfilRequest` / `modifier_profil`.

| Champ utilisé par la fiche expert | Source de mise à jour |
|-----------------------------------|------------------------|
| `photo_url` | `POST /api/auth/profil/photo` (existant) |
| `fonction` | `PUT /api/auth/profil` (existant) |
| `pays_residence_id` | `PUT /api/auth/profil` (**à étendre**) → FK `shared.pays(id)` |

## Permissions (seed `15_seed.sql`)

Ajout de deux permissions au catalogue `iam.permission` :

```sql
('Voir les demandes d''expertise',   'expertise.voir',    'expertise', 'voir'),
('Valider une demande d''expertise',  'expertise.valider', 'expertise', 'valider'),
```

Le rôle `super_admin` les couvre déjà via le wildcard `all.all`.

## DTO (mappage cross-stack)

### Backend : `models/admin/expertise.rs` (nouveau)

- `AdminDemandeExpertiseRow` (`FromRow`) : jointure `iam.expertise` + `iam.utilisateur` + `shared.pays` (nom, prénom, email, photo, domaine, biographie, expérience, situations, statut, commentaire_admin, valide_par_nom, date_validation, created_at).
- `AdminDemandeExpertiseListeResponse` : enveloppe paginée (items + total + page + par_page).
- `AdminDemandeExpertiseResponse` : DTO détail.
- `RejeterExpertiseBody { commentaire_admin: String }` (validation non vide).

### Backend : `models/expert.rs` (modifié)

- DTO « ma candidature » exposant au membre : `statut`, `commentaire_admin` (si refusé), `date_validation`, champs d'expertise, pour le suivi (US3).

### Frontend : types TS

- `useAdminExperts.ts` : `DemandeExpertiseAPI`, `DemandeExpertiseListe`, paramètres de filtre (`statut`, `recherche`, `page`, `par_page`).
- `useExperts.ts` : extension du type retour de `obtenirMaCandidature` (statut + commentaire).

Cohérence : enums `domaine`/`situation` ↔ `iam.domaine_expertise`/`iam.situation_professionnelle` ↔ labels frontend (déjà mappés).
