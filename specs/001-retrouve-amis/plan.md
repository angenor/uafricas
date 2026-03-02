# Implementation Plan: Retrouve Amis

**Branch**: `001-retrouve-amis` | **Date**: 2026-02-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-retrouve-amis/spec.md`

## Summary

**Retrouve Amis** permet aux utilisateurs de la plateforme UAfricas de retrouver des amis perdus de vue grâce à un système de recoupement intelligent. Un utilisateur dépose un avis de recherche décrivant la personne cherchée (nom, école, ville, période), et le système croise ces informations avec les autres avis actifs et les profils d'utilisateurs consentants ("trouvables"). Quand le score de correspondance atteint 60%, les deux parties sont notifiées via un résumé anonymisé. Un mécanisme de consentement mutuel (double opt-in) protège la vie privée avant tout partage de coordonnées.

**Approche technique** : Nouveau schema PostgreSQL `retrouve_amis` avec algorithme de matching basé sur `pg_trgm` (similarité trigrammes) + `unaccent` (noms africains avec accents) + calcul de chevauchement de période. Notifications par base de données (polling). Pas de WebSocket pour le MVP (YAGNI).

## Technical Context

**Language/Version**: Rust (Edition 2024) + TypeScript (Nuxt 4 / Vue 3)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), Nuxt 4, Pinia, Tailwind CSS v4
**Storage**: PostgreSQL 16 — nouveau schema `retrouve_amis` ajouté aux 10 existants
**Testing**: Aucun configuré (conformément aux contraintes techniques du projet)
**Target Platform**: Web (SSR Nuxt 4) — Linux server (production VPS)
**Project Type**: Web application full-stack (monorepo frontend + backend)
**Performance Goals**: Recoupement < 5 minutes après création/modification d'un avis (SC-002)
**Constraints**: Consentement mutuel obligatoire, max 10 avis actifs par utilisateur, blacklist automatique après refus
**Scale/Scope**: Volume initial faible (centaines d'utilisateurs), croissance organique

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Détail |
|----------|--------|--------|
| **I. Français d'Abord** | ✅ PASS | Code, variables, tables SQL, UI, messages — tout en français |
| **II. Monorepo Cohérent** | ✅ PASS | Modifications cross-stack (SQL → backend Rust → frontend Nuxt) dans la même PR |
| **III. SQL Source de Vérité** | ✅ PASS | Schema SQL créé en premier (`16_retrouve_amis.sql`), structs Rust et interfaces TS propagées ensuite |
| **IV. Sécurité par Défaut** | ✅ PASS | JWT requis pour toutes les mutations, requêtes paramétrées (sqlx), audit sur toutes les mutations, consentement mutuel, blacklist anti-harcèlement |
| **V. Simplicité (YAGNI)** | ✅ PASS | Matching synchrone (pas de job queue), notifications DB polling (pas WebSocket), pas de chiffrement côté applicatif (HTTPS suffit) |
| **VI. Tailwind v4 (daisyUI back-office)** | ✅ PASS | Pages publiques en Tailwind pur, pages admin avec daisyUI |
| **VII. Audit & Traçabilité** | ✅ PASS | `audit::log_action` sur tous les handlers de mutation (création, modification, clôture, modération) |

**Schema Decision** : Nouveau schema `retrouve_amis` (justification : domaine métier distinct ne relevant d'aucun schema existant — ce n'est ni de l'IAM, ni du marketplace, ni de la culture. Bounded context autonome avec son propre cycle de vie).

**Modification cross-schema** : Ajout de `est_trouvable BOOLEAN DEFAULT FALSE` sur `iam.utilisateur` (champ profil, pas de nouveau schema IAM).

### Post-Phase 1 Re-check

| Principe | Statut | Commentaire |
|----------|--------|-------------|
| **V. Simplicité** | ✅ PASS | Score calculé par une fonction SQL pure (pas de service externe). Matching déclenché dans le handler HTTP (synchrone). Notifications simples en base (polling via composable). |
| **III. SQL Source de Vérité** | ✅ PASS | Toutes les entités documentées dans data-model.md correspondent au SQL. Interfaces TS et structs Rust dérivées fidèlement. |
| **IV. Sécurité** | ✅ PASS | Auto-correspondance empêchée (CHECK constraint). Blacklist symétrique (CHECK a < b). Limite 10 avis actifs vérifiée côté handler. |

## Project Structure

### Documentation (this feature)

```text
specs/001-retrouve-amis/
├── plan.md              # This file
├── research.md          # Phase 0 — décisions techniques documentées
├── data-model.md        # Phase 1 — modèle de données complet
├── quickstart.md        # Phase 1 — guide de démarrage
├── contracts/
│   ├── api-publique.md  # Phase 1 — endpoints publics
│   └── api-admin.md     # Phase 1 — endpoints admin
└── tasks.md             # Phase 2 — généré par /speckit.tasks
```

### Source Code (repository root)

```text
# Backend
uafricas_backend/
├── doc/bd/schemas/
│   ├── 16_retrouve_amis.sql              # NEW — schema complet
│   └── 13_contraintes_inter_schemas.sql  # MODIFY — FK cross-schema
├── src/
│   ├── models/
│   │   ├── mod.rs                        # MODIFY — pub mod retrouve_amis
│   │   ├── retrouve_amis.rs              # NEW — structs publiques
│   │   └── admin/
│   │       ├── mod.rs                    # MODIFY — pub mod retrouve_amis
│   │       └── retrouve_amis.rs          # NEW — structs admin
│   ├── handlers/
│   │   ├── mod.rs                        # MODIFY — pub mod retrouve_amis
│   │   ├── retrouve_amis.rs              # NEW — handlers publics
│   │   └── admin/
│   │       ├── mod.rs                    # MODIFY — pub mod retrouve_amis
│   │       └── retrouve_amis.rs          # NEW — handlers admin
│   └── routes.rs                         # MODIFY — ajouter scopes

# Frontend
uafricas_frontend/
├── app/
│   ├── pages/
│   │   ├── retrouve-amis/
│   │   │   ├── index.vue                 # NEW — page d'accueil fonctionnalité
│   │   │   ├── nouveau.vue               # NEW — formulaire création avis
│   │   │   ├── mes-recherches.vue        # NEW — liste mes avis
│   │   │   ├── correspondances.vue       # NEW — liste mes correspondances
│   │   │   └── correspondances/
│   │   │       └── [id].vue              # NEW — détail correspondance
│   │   └── admin/
│   │       └── retrouve-amis/
│   │           ├── index.vue             # NEW — admin liste avis
│   │           ├── [id].vue              # NEW — admin détail avis
│   │           └── signalements.vue      # NEW — admin modération
│   ├── components/
│   │   └── retrouve-amis/
│   │       ├── RetrouvAmisHero.vue       # NEW — hero section
│   │       ├── AvisRechercheCard.vue     # NEW — carte d'avis
│   │       ├── AvisRechercheForm.vue     # NEW — formulaire multi-étapes
│   │       ├── CorrespondanceCard.vue    # NEW — carte de correspondance
│   │       ├── CorrespondanceDetail.vue  # NEW — détail + actions
│   │       ├── ScoreBadge.vue            # NEW — badge score %
│   │       ├── TableauDeBord.vue         # NEW — dashboard résumé
│   │       └── ProfilTrouvableForm.vue   # NEW — formulaire parcours
│   ├── composables/
│   │   ├── useRetrouvAmis.ts             # NEW — composable public
│   │   └── useAdminRetrouvAmis.ts        # NEW — composable admin
│   └── pages/profil.vue                  # MODIFY — section trouvable
```

**Structure Decision** : Architecture web application (monorepo frontend/backend) conforme au pattern existant. Chaque nouveau domaine suit la convention : 1 composable public + 1 composable admin + pages + composants feature-based.

## Complexity Tracking

| Violation potentielle | Justification | Alternative rejetée |
|----------------------|---------------|---------------------|
| Nouveau schema `retrouve_amis` (11ème) | Bounded context distinct : cycle de vie indépendant, pas de couplage fort avec les schemas existants | Rattacher à `iam` → surcharge un schema déjà riche (utilisateurs, rôles, permissions, organisations, expertise) ; mélange de responsabilités |
| Ajout colonne `est_trouvable` sur `iam.utilisateur` | Attribut de profil utilisateur, pas une entité séparée. Évite une table 1:1 inutile | Table `retrouve_amis.preference_trouvabilite` 1:1 → violation YAGNI, complexité JOIN inutile |
| Fonction SQL de scoring | Encapsule la logique de matching en un seul lieu testable, évite la duplication handler/cron | Scoring côté Rust → nécessiterait de charger tous les profils en mémoire, N+1 queries |
