# Implementation Plan: Réorganisation des centres culturels (routes + administration)

**Branch**: `001-centres-reorganisation` | **Date**: 2026-04-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-centres-reorganisation/spec.md`

## Summary

Refonte **principalement frontend** : renommage de la page publique `/africain-afro-americain` en `/centres`, de la fiche centre `/site/{id}` en `/centres/{id}`, et de la fiche programmation `/site/{siteId}/programmation/{programmationId}` en `/centres/{centreId}/programmations/{programmationId}` (motif canonique figé en clarification Q1). Redirections permanentes des anciennes URLs vers les nouvelles. Carrousel d'en-tête alimenté automatiquement par les images de couverture des centres publiés (Q2). Tri des programmations sur la fiche centre : à venir d'abord (date croissante), puis passées (date décroissante) (Q3). Aucune modification du schéma SQL ni des endpoints backend : le backend expose déjà `/admin/centres-culturels/*`, `/admin/programmations/*`, `/api/centres-culturels/{id}` et `/api/centres-culturels/{centreId}/programmations/{id}`. Le back-office admin existant (`/admin/centres-culturels/*`, `/admin/programmations/*`) est déjà opérationnel en CRUD : vérification de complétude uniquement.

## Technical Context

**Language/Version** : TypeScript (Nuxt 4 / Vue 3 SSR) — aucun changement backend ; Rust Edition 2024 côté backend inchangé
**Primary Dependencies** : Nuxt 4 (file-based routing, `routeRules`), Pinia, Tailwind CSS v4 pur (pas de daisyUI sur pages publiques) ; composables `useCentresCulturels` et `useAdminCentresCulturels` / `useAdminProgrammations` déjà existants
**Storage** : PostgreSQL 16 — schema `culture` — **aucune migration requise** (tables `centre_culturel`, `programmation_centre`, `membre_centre` déjà en place ; la publication d'un centre est gérée par le flag `centre_culturel.actif BOOLEAN`, pas par un soft-delete `deleted_at`)
**Testing** : Aucun framework de test configuré sur le projet — validation manuelle via scénarios de la section « quickstart »
**Target Platform** : Navigateur moderne (SSR Nuxt 4) ; serveur Node côté Nuxt, Actix-Web côté Rust
**Project Type** : Web application (frontend Nuxt + backend Rust séparés dans le même monorepo)
**Performance Goals** : SC-005 — page `/centres` affiche sa liste en < 2 s sur connexion standard (95ᵉ percentile)
**Constraints** :
- Redirections permanentes (HTTP 301) depuis `/africain-afro-americain`, `/site/{id}`, `/site/{siteId}/programmation/{programmationId}` via `routeRules` dans `nuxt.config.ts` (pattern `redirect: { to: '...', statusCode: 301 }`)
- Contexte non-production (Q4) : tous les centres et programmations existants considérés publiés au déploiement, aucune migration de données requise
- Tailwind CSS v4 pur sur les trois pages publiques (`/centres`, `/centres/{id}`, `/centres/{centreId}/programmations/{programmationId}`) — interdiction des classes daisyUI (principe constitutionnel VI)
**Scale/Scope** :
- 3 pages publiques déplacées + 3 règles de redirection `routeRules`
- 1 composable étendu (`useCentresCulturels` — ajout helper de tri « à venir / passées »)
- ~8–10 fichiers mettant à jour des liens internes (NavBar, BoutonLateralGauche, boutons « retour », éventuelles cartes partagées)
- Suppression de l'import `CAROUSEL_IMAGES` depuis mocks, remplacement par dérivation dynamique depuis les centres chargés
- Vérification de l'absence de boutons de création de programmation sur pages publiques (FR-014 / FR-016)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Justification |
|---|---|---|
| I. Français d'Abord | ✅ | Toutes les nouvelles chaînes UI, noms de variables, commentaires et messages seront en français. Les noms de fichiers restent ASCII (`centres/[id].vue`) par convention projet. |
| II. Monorepo Cohérent | ✅ | Aucune modification backend ni extraction. Les trois nouvelles routes frontend consomment les endpoints publics existants du backend sans changement de contrat. |
| III. SQL Source de Vérité | ✅ | Aucune mutation de schéma. Les interfaces TS (`CentreCulturelAPI`, `ProgrammationAPI`) du composable `useCentresCulturels` restent alignées sur les structs Rust et le schéma `culture` (tables réelles : `centre_culturel`, `programmation_centre`, `membre_centre`). |
| IV. Sécurité par Défaut | ✅ | Les pages publiques `/centres` et `/centres/{id}` restent accessibles anonymement (FR-018). Les endpoints admin CRUD existants sont déjà protégés JWT. Pas de nouvelle surface d'attaque. |
| V. Simplicité (YAGNI) | ✅ | Feature strictement refactoring : déplacement de fichiers Nuxt, mise à jour de liens, 3 redirections 301 via `routeRules`. Pas d'abstraction nouvelle, pas de nouvelle couche. Carrousel simplifié (suppression du mock `CAROUSEL_IMAGES`). |
| VI. Tailwind v4 / daisyUI back-office uniquement | ✅ | Les trois pages publiques utilisent Tailwind v4 pur. Vérification à la livraison qu'aucune classe daisyUI (`btn`, `card`, `modal`, etc.) n'est introduite sur `/centres/*`. Le back-office admin existant utilise déjà daisyUI. |
| VII. Audit & Traçabilité | ✅ | Les mutations CRUD admin existantes (`admin::centres_culturels::*`, `admin::programmations::*`) sont déjà instrumentées `audit::log_action` dans le backend. Aucune nouvelle mutation à instrumenter côté frontend. |

**Gate PASS** — aucune violation, aucune justification complémentaire requise. Section « Complexity Tracking » restera vide.

## Project Structure

### Documentation (this feature)

```text
specs/001-centres-reorganisation/
├── plan.md              # Ce fichier
├── spec.md              # Spécification + Clarifications (existant)
├── research.md          # Phase 0 — décisions techniques
├── data-model.md        # Phase 1 — rappel des entités existantes (aucune nouvelle)
├── quickstart.md        # Phase 1 — scénarios de validation manuelle
├── contracts/
│   └── routes-frontend.md   # Contrat des routes Nuxt (nouvelles + redirections)
├── checklists/
│   └── requirements.md  # Checklist qualité spec (existant)
└── tasks.md             # Généré par /speckit.tasks
```

### Source Code (repository root)

```text
uafricas_frontend/
├── app/
│   ├── pages/
│   │   ├── centres/
│   │   │   ├── index.vue                                          # NOUVEAU — remplace africain-afro-americain/index.vue
│   │   │   ├── [id].vue                                           # NOUVEAU — remplace site/[id].vue
│   │   │   └── [id]/
│   │   │       └── programmations/
│   │   │           └── [programmationId].vue                      # NOUVEAU — remplace site/[siteId]/programmation/[programmationId].vue
│   │   ├── africain-afro-americain/                               # SUPPRIMÉ après mise en place de routeRules redirect 301
│   │   └── site/                                                  # SUPPRIMÉ après mise en place de routeRules redirect 301
│   ├── components/
│   │   ├── centres-culturels/                                     # existant (déjà utilisé par l'ancienne page)
│   │   └── layout/
│   │       ├── NavBar.vue                                         # MODIFIÉ — lien /africain-afro-americain → /centres
│   │       └── BoutonLateralGauche.vue                            # MODIFIÉ — même lien
│   ├── composables/
│   │   └── useCentresCulturels.ts                                 # ÉTENDU — ajout helper de tri programmations à venir / passées
│   └── mocks/
│       └── centres-culturels.ts                                   # MODIFIÉ — suppression export CAROUSEL_IMAGES (remplacé par flux dynamique)
└── nuxt.config.ts                                                 # MODIFIÉ — ajout routeRules pour redirections 301

uafricas_backend/
└── (aucun changement)
```

**Structure Decision** : Web application — monorepo existant. La feature est frontend-only (Nuxt `app/pages/*`). Les trois anciens répertoires de pages (`africain-afro-americain/`, `site/`) sont supprimés après mise en place des redirections via `routeRules` dans `nuxt.config.ts`. Aucun nouveau composant n'est créé — les composants sous `app/components/centres-culturels/` sont réutilisés tels quels.

## Complexity Tracking

*Non applicable — Constitution Check PASS sans violation.*
