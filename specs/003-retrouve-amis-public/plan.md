# Implementation Plan: Avis de Recherche Publics par Défaut

**Branch**: `003-retrouve-amis-public` | **Date**: 2026-03-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/003-retrouve-amis-public/spec.md`

## Summary

Refondre le formulaire de création d'avis de recherche (10 questions structurées en 6 étapes avec upload photo) et rendre tous les avis publics par défaut sur `/retrouve-amis`, accessibles sans connexion. Le backend passe en multipart pour supporter l'upload, le schéma SQL reçoit 14 nouvelles colonnes et 2 enums, et la page d'accueil Retrouve Amis affiche directement les avis au lieu d'un simple CTA.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 (frontend)
**Primary Dependencies**: Actix-Web 4, actix-multipart, sqlx, Pinia, Tailwind CSS v4
**Storage**: PostgreSQL 16, schema `retrouve_amis` (8 tables existantes)
**Testing**: Pas de CI/CD configuré : vérification manuelle
**Target Platform**: Web (SSR Nuxt 4), serveur Linux (VPS)
**Project Type**: Application web full-stack (monorepo)
**Performance Goals**: Pages publiques chargées en < 2s, formulaire soumis en < 3s
**Constraints**: Upload photo max 5 Mo, 10 avis actifs max par utilisateur
**Scale/Scope**: ~6 pages frontend modifiées, ~4 fichiers backend modifiés, 1 fichier SQL modifié

## Constitution Check

*GATE: Vérification pré-Phase 0 et post-Phase 1*

| Principe | Statut | Notes |
|----------|--------|-------|
| I. Français d'Abord | ✅ Conforme | Variables, colonnes SQL, UI en français |
| II. Monorepo Cohérent | ✅ Conforme | Modifications cross-stack dans la même branche |
| III. SQL Source de Vérité | ✅ Conforme | Schéma SQL modifié en premier, propagé vers Rust puis TS |
| IV. Sécurité par Défaut | ✅ Conforme | Coordonnées jamais exposées publiquement, upload sanitisé |
| V. Simplicité (YAGNI) | ✅ Conforme | Colonnes plates (pas de JSONB), pas d'abstraction supplémentaire |
| VI. Tailwind v4 (daisyUI admin) | ✅ Conforme | Pages publiques en Tailwind pur, pas de daisyUI |
| VII. Audit & Traçabilité | ✅ Conforme | Mutations existantes déjà auditées, nouvelles suivent le pattern |

**Résultat** : Aucune violation. Pas de complexité injustifiée.

## Project Structure

### Documentation (this feature)

```text
specs/003-retrouve-amis-public/
├── plan.md              # Ce fichier
├── research.md          # Recherche et décisions techniques
├── data-model.md        # Modifications du schéma SQL
├── quickstart.md        # Guide de démarrage rapide
├── contracts/
│   └── api-publique.md  # Contrat API modifié
├── checklists/
│   └── requirements.md  # Checklist qualité spec
└── tasks.md             # Tâches (créé par /speckit.tasks)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 16_retrouve_amis.sql          # +2 enums, +14 colonnes, modif default
├── src/
│   ├── models/
│   │   └── retrouve_amis.rs          # +champs structs, multipart request
│   ├── handlers/
│   │   ├── retrouve_amis.rs          # creer_avis/modifier_avis → multipart, publication auto
│   │   └── retrouve_amis_public.rs   # Nouveaux champs dans réponses publiques
│   └── routes.rs                     # Suppression route publier_avis
└── uploads/
    └── retrouve-amis/                # Nouveau dossier pour photos (créé au runtime)

uafricas_frontend/
├── app/
│   ├── composables/
│   │   └── useRetrouvAmis.ts         # Interfaces TS mises à jour, multipart upload
│   ├── components/retrouve-amis/
│   │   ├── AvisRechercheForm.vue     # Réécriture complète (6 étapes)
│   │   └── CarteAvisPublic.vue       # Affichage des nouveaux champs
│   └── pages/retrouve-amis/
│       ├── index.vue                 # Listing public + dashboard connecté
│       ├── nouveau.vue               # Adaptation au nouveau formulaire
│       └── rechercher.vue            # Ajout filtre type_relation
```

**Structure Decision**: Application web monorepo existante. Pas de nouvelle structure : modifications ciblées des fichiers existants dans `uafricas_backend/` et `uafricas_frontend/`.

## Complexity Tracking

Aucune violation de la constitution, section non applicable.
