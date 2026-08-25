# Implementation Plan: Notifications et Suggestions Intelligentes

**Branch**: `001-notifications-suggestions` | **Date**: 2026-03-16 | **Spec**: [spec.md](./spec.md)

## Summary

Système de notifications in-app (table + polling au chargement de page, pas de WebSocket). Notifications créées dans les handlers existants (matching, collaboration). Suggestions proactives calculées côté client. Détection de doublons intra-arbre via pg_trgm (seuil 70%). Fusion de doublons transactionnelle. Cloche avec badge dans la navbar globale.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) + TypeScript / Nuxt 4 / Vue 3 (frontend)
**Primary Dependencies**: pg_trgm existant, audit existant
**Storage**: PostgreSQL 16 - 2 nouvelles tables (notifications, doublons_ignores)
**Testing**: Vérification manuelle
**Target Platform**: Web (SSR Nuxt 4)
**Project Type**: Web application (monorepo)
**Performance Goals**: Compteur notifs <100ms, doublons <2s, fusion <1s
**Constraints**: Polling (pas WebSocket), max 10 suggestions affichées
**Scale/Scope**: 7 endpoints backend, ~6 fichiers frontend, 2 nouvelles tables

## Constitution Check

| Principe | Statut | Justification |
|----------|--------|---------------|
| I. Français d'Abord | PASS | Messages notifications en français |
| II. Monorepo Cohérent | PASS | Backend + frontend |
| III. SQL Source de Vérité | PASS | SQL d'abord |
| IV. Sécurité par Défaut | PASS | JWT sur tous les endpoints, notifications personnelles |
| V. Simplicité (YAGNI) | PASS | Polling simple, suggestions côté client, pas de WebSocket |
| VI. Tailwind CSS v4 | PASS | Composants en Tailwind v4 pur |
| VII. Audit & Traçabilité | PASS | Fusion auditée |

## Project Structure

```text
uafricas_backend/
├── doc/bd/schemas/26_notifications.sql  # NOUVEAU
├── src/
│   ├── models/notification.rs           # NOUVEAU
│   ├── handlers/notification.rs         # NOUVEAU (7 handlers)
│   ├── handlers/matching.rs             # MODIFIER (INSERT notifs)
│   ├── handlers/collaboration.rs        # MODIFIER (INSERT notifs)
│   └── routes.rs                        # MODIFIER (+7 routes)

uafricas_frontend/
├── app/
│   ├── composables/useNotifications.ts  # NOUVEAU
│   ├── composables/useSuggestions.ts    # NOUVEAU
│   ├── components/layout/ClocheNotifications.vue  # NOUVEAU
│   ├── components/arbre-genealogique/PanneauDoublons.vue  # NOUVEAU
│   ├── pages/notifications.vue          # NOUVEAU
│   └── layouts/default.vue              # MODIFIER (cloche)
```

## Complexity Tracking

Aucune violation.
