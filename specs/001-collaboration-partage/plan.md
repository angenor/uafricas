# Implementation Plan: Collaboration et Partage de l'Arbre

**Branch**: `001-collaboration-partage` | **Date**: 2026-03-16 | **Spec**: [spec.md](./spec.md)

## Summary

Système d'invitations par email avec 2 niveaux de permission (lecture_seule / edition). Nouvelles tables SQL (invitations, collaborateurs) + colonnes confidentialité (visible_matching, arbre_prive). 12 endpoints API. Modification de `arbre-complet` pour supporter les arbres partagés. Historique via audit_log existant filtré. Pages frontend : gestion collaborateurs, navigation multi-arbres, bandeau lecture seule.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) + TypeScript / Nuxt 4 / Vue 3 (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx, lettre (SMTP existant)
**Storage**: PostgreSQL 16 — 2 nouvelles tables + 3 colonnes
**Testing**: Vérification manuelle
**Target Platform**: Web (SSR Nuxt 4)
**Project Type**: Web application (monorepo)
**Performance Goals**: Historique <2s, invitation <2min
**Constraints**: Limite 20 collaborateurs/arbre, invitations 30j expiration
**Scale/Scope**: 12 endpoints, ~15 fichiers backend, ~8 fichiers frontend

## Constitution Check

| Principe | Statut | Justification |
|----------|--------|---------------|
| I. Français d'Abord | PASS | Code et UI en français |
| II. Monorepo Cohérent | PASS | Backend + frontend |
| III. SQL Source de Vérité | PASS | SQL d'abord |
| IV. Sécurité par Défaut | PASS | Vérification propriétaire/collaborateur, JWT |
| V. Simplicité (YAGNI) | PASS | 2 niveaux simples |
| VI. Tailwind CSS v4 | PASS | Tailwind v4 pur |
| VII. Audit & Traçabilité | PASS | Historique via audit_log |

## Project Structure

```text
uafricas_backend/
├── doc/bd/schemas/25_collaboration.sql  # NOUVEAU
├── src/
│   ├── models/collaboration.rs          # NOUVEAU
│   ├── handlers/collaboration.rs        # NOUVEAU (12 handlers)
│   ├── handlers/arbre_genealogique.rs   # MODIFIER (vérif accès)
│   ├── services/matching.rs             # MODIFIER (filtre confidentialité)
│   └── routes.rs                        # MODIFIER (+12 routes)

uafricas_frontend/
├── app/
│   ├── pages/arbre-genealogique/
│   │   ├── index.vue                    # MODIFIER (multi-arbres)
│   │   ├── gestion.vue                  # NOUVEAU
│   │   └── visualisation.vue            # MODIFIER (bandeau + permissions)
│   ├── composables/useCollaboration.ts  # NOUVEAU
│   ├── components/arbre-genealogique/
│   │   ├── CarteInvitation.vue          # NOUVEAU
│   │   └── BandeauLectureSeule.vue      # NOUVEAU
│   └── mocks/collaboration.ts           # NOUVEAU
```

## Complexity Tracking

Aucune violation.
