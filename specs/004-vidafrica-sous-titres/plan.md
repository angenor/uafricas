# Implementation Plan: Vidafrica — Sous-titrage vidéo multilingue karaoke

**Branch**: `004-vidafrica-sous-titres` | **Date**: 2026-04-13 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/004-vidafrica-sous-titres/spec.md`

## Summary

Vidafrica est une fonctionnalité de sous-titrage vidéo multilingue avec surlignage karaoké mot par mot. Les vidéos sont uploadées localement par l'admin, qui saisit les sous-titres via un back-office avec un mode "tap-to-mark" pour capturer les timings mot par mot. Côté public, un lecteur vidéo HTML5 affiche les sous-titres avec un surlignage dynamique synchronisé via `requestAnimationFrame`. Les données sont stockées dans 4 tables PostgreSQL du schema `media_content`.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 (frontend)  
**Primary Dependencies**: Actix-Web 4, actix-multipart, sqlx (backend) ; Vue 3 Composition API, Pinia (frontend)  
**Storage**: PostgreSQL 16, schema `media_content` (4 nouvelles tables) + stockage local `./uploads/videos/` et `./uploads/vignettes/`  
**Testing**: Aucun framework configuré (tests manuels)  
**Target Platform**: Web (SSR Nuxt 4), serveur Linux VPS  
**Project Type**: Web application monorepo (frontend + backend)  
**Performance Goals**: Surlignage karaoké fluide (~60fps via requestAnimationFrame), changement de langue < 1s  
**Constraints**: Upload local uniquement (pas de CDN/cloud), fichiers vidéo max 500 Mo, formats MP4/WebM  
**Scale/Scope**: ~100 vidéos publiées, jusqu'à 5 langues par vidéo, ~10-50 segments par piste

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Commentaire |
|----------|--------|-------------|
| I. Français d'Abord | PASS | Tous les noms de colonnes, variables, composants en français |
| II. Monorepo Cohérent | PASS | Modifications dans `uafricas_frontend/` et `uafricas_backend/` uniquement |
| III. SQL Source de Vérité | PASS | Schema SQL créé d'abord, structs Rust et interfaces TS alignées ensuite |
| IV. Sécurité par Défaut | PASS | JWT admin existant, validation upload (taille, format), requêtes paramétrées sqlx |
| V. Simplicité (YAGNI) | PASS | Pas de librairie vidéo externe, pas de streaming adaptatif, pattern existant réutilisé |
| VI. Tailwind CSS v4 | PASS | Site public = Tailwind v4 pur, back-office = daisyUI v5 autorisé |
| VII. Audit & Traçabilité | PASS | `audit::log_action` sur toutes les mutations admin (CRUD vidéos, pistes, segments) |

**Constitution Check post-design** : PASS — aucune violation. Le design utilise exclusivement les patterns existants du projet.

## Project Structure

### Documentation (this feature)

```text
specs/004-vidafrica-sous-titres/
├── spec.md              # Spécification fonctionnelle
├── plan.md              # Ce fichier
├── research.md          # Décisions techniques (8 décisions)
├── data-model.md        # Modèle de données (4 tables + 1 enum)
├── quickstart.md        # Guide de démarrage rapide
├── contracts/
│   ├── api-admin-vidafrica.md   # Contrats API admin (~18 endpoints)
│   └── api-public-vidafrica.md  # Contrats API public (4 endpoints)
├── checklists/
│   └── requirements.md  # Checklist qualité spec
└── tasks.md             # (Phase 2 — /speckit.tasks)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 27_vidafrica.sql                    # Schema SQL (enum + 4 tables)
├── src/
│   ├── models/
│   │   ├── vidafrica.rs                    # Structs publiques
│   │   └── admin/
│   │       └── vidafrica.rs                # Structs admin (liste, détail, DTOs, COLONNES)
│   └── handlers/
│       ├── vidafrica.rs                    # Handlers publics (4 endpoints)
│       └── admin/
│           └── vidafrica.rs                # Handlers admin (~18 endpoints)

uafricas_frontend/
├── app/
│   ├── mocks/
│   │   └── vidafrica.ts                    # Interfaces + données mock
│   ├── composables/
│   │   ├── useAdminVidafrica.ts            # Composable admin CRUD
│   │   └── useVidafrica.ts                 # Composable public
│   ├── components/
│   │   └── vidafrica/
│   │       ├── VidafricaLecteur.vue        # Lecteur vidéo + overlay karaoké
│   │       ├── VidafricaCarteVideo.vue     # Carte vidéo catalogue
│   │       ├── VidafricaSelecteurLangue.vue # Sélecteur de langue sous-titres
│   │       └── VidafricaTapToMark.vue      # Interface tap-to-mark admin
│   └── pages/
│       ├── vidafrica/
│       │   ├── index.vue                   # Catalogue public
│       │   └── [slug].vue                  # Page lecture vidéo
│       └── admin/
│           └── vidafrica/
│               ├── index.vue               # Liste admin
│               ├── create.vue              # Formulaire création
│               └── [id].vue                # Édition + sous-titres + tap-to-mark
```

**Structure Decision**: Suit la structure existante du monorepo — un fichier handler/model par domaine côté backend, un composable par domaine côté frontend, composants organisés par feature dans `app/components/vidafrica/`.
