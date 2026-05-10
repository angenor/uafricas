# Implementation Plan: Pays d'origine des salles publiques Afrolang

**Branch**: `001-afrolang-pays-origine` | **Date**: 2026-05-10 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-afrolang-pays-origine/spec.md`

## Summary

Ajouter une relation N-N **`afrolang.salle ↔ shared.pays`** (« pays d'origine ») indépendante du pays implicitement déduit via le groupe ethnique. Côté API publique : la liste `/api/afrolang/salles` enrichit chaque salle d'un tableau `pays_origine[]` (filtré sur `actif=true`) et accepte un paramètre `pays_id` mono-valué. Côté admin : 2 endpoints (ajouter, retirer) calqués strictement sur `marketplace.annonce_pays`. Côté UI : la carte salle affiche 1-3 pays avec drapeau + nom, ou drapeaux seuls + tooltip au-delà ; un nouveau filtre « Pays d'origine » alimente le panneau latéral. Migration unique : ajout d'une nouvelle table dans `08b_afrolang.sql` (les salles existantes restent vides — choix Q1).

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 SSR (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde (backend) ; Pinia, $fetch, FontAwesome (frontend) — aucune nouvelle dépendance
**Storage**: PostgreSQL 16 — schema `afrolang` étendu (1 nouvelle table de jointure `salle_pays_origine`) ; FK vers `shared.pays`
**Testing**: Aucun framework de test configuré (CLAUDE.md « No linting, testing, or CI/CD configured yet »). Validation via quickstart manuel + audit log.
**Target Platform**: Backend Linux/Docker (port 8080), Frontend Nuxt SSR (port 3000)
**Project Type**: Web (monorepo `uafricas_backend/` + `uafricas_frontend/`)
**Performance Goals**: Liste `/afrolang` ≤ 110 % du temps actuel (SC-004). Une seule sous-requête `LEFT JOIN LATERAL` ou agrégation `array_agg` ajoutée au SELECT principal — pas de N+1.
**Constraints**: Filtre public mono-pays (Q2). Pays archivés masqués côté public (Q3). Affichage adaptatif 1-3 vs 4+ pays (Q4). Aucune migration éditoriale automatique des salles existantes (Q1).
**Scale/Scope**: ~quelques dizaines de salles publiques au total ; ~54 pays africains au référentiel ; cardinalité moyenne 1-6 pays par salle. Charge faible, pas de pagination dédiée pour les pays d'une salle.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Conformité | Notes |
|---|---|---|
| **I. Français d'Abord** | ✅ | Table `salle_pays_origine`, colonne `pays_id`, handler `ajouter_pays_origine_salle` / `retirer_pays_origine_salle`, composable `useAfrolang.ajouterPaysOrigine`, libellé UI « Pays d'origine ». |
| **II. Monorepo Cohérent** | ✅ | Modification SQL + Rust + TS livrée dans la même PR. Types alignés : `PaysOrigineLight` (TS) ↔ `PaysOrigineLight` (Rust struct) ↔ colonnes `shared.pays`. |
| **III. SQL Source de Vérité** | ✅ | DDL ajouté dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` **en premier** ; les structs Rust et interfaces TS découlent du schéma. |
| **IV. Sécurité par Défaut** | ✅ | `verifier_permission!(admin, "afrolang", "modifier")` sur les 2 endpoints admin. Requêtes paramétrées sqlx. Pas de nouveau secret. CORS inchangé. Validation existence pays via `WHERE actif=true` côté admin (cohérent avec `ajouter_pays_annonce`). |
| **V. Simplicité (YAGNI)** | ✅ | Pas de service intermédiaire, pas de DTO custom : on copie 1-pour-1 le pattern `marketplace.annonce_pays` (déjà éprouvé en production). Pas de cache, pas de tri configurable, ordre alpha en SQL. |
| **VI. Tailwind CSS v4 (daisyUI back-office uniquement)** | ✅ | Carte publique `AfrolangSalleCard` + filtre public `AfrolangSalleFilters` : Tailwind v4 pur. Page admin (à créer) : daisyUI v5 autorisé. |
| **VII. Audit & Traçabilité** | ✅ | `audit::log_action` appelé sur ajout/retrait avec `module="afrolang"`, `table="salle_pays_origine"`, `entity_id=salle_id`. |

**Verdict** : Aucune violation, aucun écart à justifier. La feature s'inscrit strictement dans les patterns existants du monorepo.

## Project Structure

### Documentation (this feature)

```text
specs/001-afrolang-pays-origine/
├── plan.md              # Ce fichier
├── spec.md              # Spec validée + Clarifications
├── research.md          # Phase 0 (résolutions techniques)
├── data-model.md        # Phase 1 (DDL + entités)
├── quickstart.md        # Phase 1 (parcours de validation)
├── contracts/
│   ├── api-public.md    # GET /api/afrolang/salles enrichi + ?pays_id=
│   └── api-admin.md     # POST/DELETE /api/admin/afrolang/salles/{id}/pays
├── checklists/
│   └── requirements.md  # (existe)
└── tasks.md             # (à générer par /speckit.tasks)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 08b_afrolang.sql                  # ➕ CREATE TABLE afrolang.salle_pays_origine + index
├── src/
│   ├── handlers/
│   │   ├── afrolang.rs                   # ✏️ lister_salles : enrichir SELECT (array_agg pays) + filtre ?pays_id
│   │   └── admin/
│   │       └── salles.rs                 # ➕ ajouter_pays_origine_salle / retirer_pays_origine_salle
│   ├── models/
│   │   └── afrolang.rs                   # ➕ struct PaysOrigineLight (FromRow + Serialize)
│   │                                     # ✏️ SalleResponse : champ pays_origine: Vec<PaysOrigineLight>
│   └── routes.rs                         # ➕ 2 routes /api/admin/afrolang/salles/{id}/pays(/:pays_id)
│                                         # ✏️ /api/afrolang/salles déjà existant (filtre ajouté côté handler)

uafricas_frontend/
├── app/
│   ├── composables/
│   │   ├── useAfrolang.ts                # ➕ type PaysOrigineLight ; ✏️ SalleAPI (champ pays_origine[])
│   │   │                                 # ✏️ SalleFiltres (champ pays_id)
│   │   └── useAdminAfrolangSalles.ts     # ➕ ajouterPaysOrigine / retirerPaysOrigine
│   ├── components/afrolang/
│   │   ├── SalleCard.vue                 # ✏️ Bandeau « Pays d'origine » (1-3 vs 4+)
│   │   ├── SalleFilters.vue              # ➕ select « Pays d'origine » (mono)
│   │   └── SalleFiltersMobile.vue        # ➕ idem version mobile
│   └── pages/
│       ├── afrolang/index.vue            # ✏️ wire le filtre pays_id ; rien d'autre
│       └── admin/afrolang/salles/[id].vue # ➕ panneau « Pays d'origine » (sélecteur + chips) — daisyUI
```

**Structure Decision** : Web monorepo existant (Option 2 du template). Aucun nouveau module, aucun nouveau composable global ni nouveau schema PostgreSQL. La feature ajoute strictement 1 table + 2 endpoints admin + enrichissement de 1 endpoint public + 4 composants frontend modifiés / 1 panneau admin créé.

## Complexity Tracking

> Aucune violation de constitution à tracer. La feature reste sous le seuil de complexité (Principe V) en réutilisant intégralement le pattern `annonce_pays` (référence existante dans le monorepo).
