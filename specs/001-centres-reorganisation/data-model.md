# Phase 1 : Data Model: Réorganisation des centres culturels

**Feature**: `001-centres-reorganisation` | **Date**: 2026-04-19

## Synthèse

**Aucune nouvelle entité. Aucune migration SQL.** Cette feature est un refactoring frontend : la source de vérité SQL (schéma `culture`) est réutilisée telle quelle. Ce document rappelle les entités consommées, leurs champs clés et leurs relations pour ancrer le contrat entre frontend et backend et confirmer le respect du principe III de la constitution (SQL source de vérité).

---

## Entités existantes consommées

### 1. `culture.centre_culturel`

Entité éditoriale d'un centre culturel africain ou afro-descendant.

| Champ | Type SQL | Contraintes | Rôle pour la feature |
|---|---|---|---|
| `id` | `UUID` | PK, default `uuid_generate_v4()` | Segment dynamique de la route `/centres/{id}` |
| `nom` | `VARCHAR(350)` | NOT NULL | Titre de la carte et de la fiche, balise `<title>` SEO |
| `slug` | `VARCHAR(400)` | nullable, UNIQUE | Non utilisé par la feature (URL canonique par UUID) |
| `description` | `TEXT` | nullable | Corps de la fiche |
| `image_couverture_url` | `VARCHAR(500)` | nullable | Image de carte, visuel du carrousel (FR-005a) |
| `pays_id` | `UUID` | FK → `shared.pays`, nullable | Pays du centre (FR-001 « pays »), FK vers le référentiel pays existant |
| `ville` | `VARCHAR(200)` | nullable | Localisation affichée |
| `adresse` | `TEXT` | nullable | Localisation affichée |
| `latitude`, `longitude` | `DECIMAL(10,7)` | nullable | Coordonnées géographiques (lien Google Maps côté frontend) |
| `actif` | `BOOLEAN` | NOT NULL, DEFAULT `TRUE` | **Flag de publication**, `TRUE` = centre publié (exposé sur `/centres`), `FALSE` = dépublié/archivé (exclu des endpoints publics) |
| `cree_par` | `UUID` | NOT NULL, FK `iam.utilisateur` | Auditabilité administrative |
| `created_at`, `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Audit, hors UI publique |

**État de publication** : tout `centre_culturel` avec `actif = TRUE` est « publié » et exposé sur `/centres`. Au moment du déploiement (contexte non-prod, Q4), les enregistrements existants sont déjà `actif = TRUE` (aucune migration requise). Aucun champ `published_at` ni `deleted_at` n'existe dans cette table, la bascule est binaire via `actif`.

**Cycle de vie** :
```
créé (admin, actif=TRUE)  →  affiché sur /centres  →  [modifié (admin)]  →  [dépublié (admin, actif=FALSE)]  →  invisible (restaurable en basculant actif=TRUE)
```

---

### 2. `culture.programmation_centre`

Événement ou activité culturelle rattachée à un centre.

| Champ | Type SQL | Contraintes | Rôle pour la feature |
|---|---|---|---|
| `id` | `UUID` | PK, default `uuid_generate_v4()` | Segment dynamique `/centres/{centreId}/programmations/{id}` |
| `centre_culturel_id` | `UUID` | FK → `centre_culturel.id`, NOT NULL, `ON DELETE CASCADE` | Contrainte d'appartenance : définit l'URL parent. La suppression d'un centre supprime en cascade ses programmations. |
| `titre` | `VARCHAR(350)` | NOT NULL | Libellé dans la liste et sur la fiche |
| `description` | `TEXT` | nullable | Corps de la fiche programmation |
| `lieu` | `VARCHAR(350)` | nullable | Affichage informationnel |
| `mode` | `culture.mode_evenement` (ENUM `presentiel` / `en_ligne` / `hybride`) | NOT NULL, DEFAULT `presentiel` | Affichage et pictogramme |
| `lien_en_ligne` | `VARCHAR(500)` | nullable | CTA (lien ouvert) pour les modes `en_ligne` / `hybride` |
| `date_heure_debut` | `TIMESTAMPTZ` | NOT NULL | **Clé de tri (FR-017a)**, à venir / passées |
| `date_heure_fin` | `TIMESTAMPTZ` | nullable | Affichage de plage horaire |
| `nombre_places` | `INT` | nullable | Affichage d'inscriptions |
| `cree_par` | `UUID` | NOT NULL, FK `iam.utilisateur` | Auditabilité administrative |
| `created_at`, `updated_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Audit |

> **Note** : la table ne possède **ni champ `statut`, ni flag `actif`, ni `deleted_at`**. La visibilité publique d'une programmation est conditionnée uniquement par la publication de son centre parent (`centre_culturel.actif = TRUE`). Une programmation « passée » reste affichée dans la fiche du centre selon le tri FR-017a ; aucune bascule de publication par programmation.

**Règle de tri (FR-017a, Décision 3 de research.md)** :

```
ordre = [
  programmations où date_heure_debut >= maintenant, tri ASC sur date_heure_debut,
  ...programmations où date_heure_debut < maintenant, tri DESC sur date_heure_debut,
]
```

---

### 3. `culture.membre_centre`

Membre rattaché à un centre (président, vice-président, responsable communication, membre). Consommée uniquement en lecture via la fiche détaillée du centre. Non modifiée par cette feature.

| Champ | Type SQL | Contraintes | Rôle pour la feature |
|---|---|---|---|
| `id` | `UUID` | PK, default `uuid_generate_v4()` | Identifiant du lien centre ↔ utilisateur |
| `centre_culturel_id` | `UUID` | FK → `centre_culturel.id`, NOT NULL, `ON DELETE CASCADE` | Appartenance au centre |
| `utilisateur_id` | `UUID` | NOT NULL, FK `iam.utilisateur` | Utilisateur titulaire du rôle (nom/prenom/email/telephone viennent de la jointure avec `iam.utilisateur`) |
| `role` | `culture.role_membre_centre` (ENUM `president` / `vice_president` / `resp_communication` / `membre`) | NOT NULL, DEFAULT `membre` | Affiché dans le bloc « membres » |
| `created_at` | `TIMESTAMPTZ` | NOT NULL, DEFAULT `NOW()` | Audit |
| | : | `UNIQUE (centre_culturel_id, utilisateur_id)` | Un utilisateur ne peut tenir qu'un rôle par centre |

> **Note DTO** : le DTO `MembreCentreAPI` côté frontend (`app/composables/useCentresCulturels.ts`) expose `nom`, `prenom`, `email`, `telephone`, `role`, `role_label`, ces champs sont issus de la **jointure** effectuée dans le handler `centres_culturels::obtenir_centre` avec `iam.utilisateur`, **pas** des colonnes de `culture.membre_centre` directement.

---

## DTO API consommés (rappel)

Les interfaces TypeScript ci-dessous existent déjà dans `app/composables/useCentresCulturels.ts` et reflètent fidèlement les structs Rust `FromRow` du backend (principe III) :

- `CentreCulturelAPI` : carte de liste (`/api/centres-culturels`)
- `CentreCulturelDetailAPI` : fiche + membres + programmations (`/api/centres-culturels/{id}`)
- `ProgrammationAPI` : ligne de programmation (rattachée)
- `ProgrammationDetailAPI` : fiche programmation + centre parent compact (`/api/centres-culturels/{centreId}/programmations/{id}`)

Aucun de ces DTO n'est modifié par la feature.

---

## Relations

```
centre_culturel (1) ──< programmation_centre (N)     [ON DELETE CASCADE]
     │
     └──< membre_centre (N)                          [ON DELETE CASCADE, UNIQUE(centre,user)]
```

Contraintes clés :
- `programmation_centre.centre_culturel_id` FK NOT NULL + `ON DELETE CASCADE` → un centre hard-deleted supprime ses programmations. Un centre dépublié (`actif = FALSE`) masque ses programmations côté public via le filtrage dans le handler.
- L'URL `/centres/{centreId}/programmations/{programmationId}` est valide **uniquement si** `programmation_centre.centre_culturel_id = centreId` (vérification côté backend, déjà en place dans le handler `centres_culturels::obtenir_programmation`).

---

## Invariants à vérifier à la livraison

1. Aucun nouveau champ, aucune nouvelle table, aucune ALTER TABLE dans `uafricas_backend/doc/bd/schemas/`.
2. Les interfaces TS (`CentreCulturelAPI`, `ProgrammationAPI`, etc.) restent strictement alignées sur les DTO Rust, aucune divergence introduite par la feature.
3. La logique de tri « à venir / passées » reste côté frontend (helper pur dans le composable), le SQL n'est pas sollicité pour cette règle d'affichage.
4. Le flag `centre_culturel.actif = FALSE` continue d'exclure les centres dépubliés des endpoints publics (et donc leurs programmations associées), aucun changement de filtrage. Les programmations n'ont aucun flag propre ; leur visibilité suit celle du centre parent.
