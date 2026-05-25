# Implementation Plan: Demande d'amitié entre membres

**Branch**: `001-demande-amitie` | **Date**: 2026-05-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-demande-amitie/spec.md`

## Summary

Permettre à tout membre connecté et actif d'envoyer une demande d'amitié à un autre membre depuis l'annuaire `/profil` et la fiche `/profil/{id}`, de l'accepter/refuser, de gérer ses relations (annuler, retirer, bloquer), et — une fois amis — de **discuter en temps réel** via un **bouton flottant de messagerie** présent sur toutes les pages.

Approche technique : nouveau **schéma PostgreSQL `social`** (demandes, amitiés, blocages, conversations, messages, notifications) ; backend Rust/Actix-Web exposant des endpoints REST + un **flux SSE** pour le temps réel (registre de connexions en mémoire, mono-instance, sans nouvelle dépendance) ; frontend Nuxt 4 en **Tailwind v4 pur** (composables `useAmis`/`useMessagerie`, plugin client SSE, composant flottant global, page `/mon-compte/amis`, boutons sur `/profil`). Voir [research.md](./research.md) pour les décisions.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) ; TypeScript / Nuxt 4 / Vue 3 SSR (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde, `futures-util` (SSE), `tokio` (déjà présents) — **aucune nouvelle dépendance** ; Pinia/useState, $fetch, `EventSource` (navigateur), FontAwesome (frontend)
**Storage**: PostgreSQL 16 — nouveau schéma `social` (`schemas/29_social.sql`), aucune modification de `iam.utilisateur`
**Testing**: aucun framework configuré (cf. constitution) — validation manuelle via [quickstart.md](./quickstart.md)
**Target Platform**: serveur Linux (Docker), navigateurs modernes (SSE/EventSource)
**Project Type**: web (monorepo frontend Nuxt + backend Rust)
**Performance Goals**: remise d'un message < 2 s (SC-008) ; bouton flottant accessible en 1 clic sur 100 % des pages (SC-007)
**Constraints**: temps réel **mono-instance** (registre SSE en mémoire) ; UI publique/membre = Tailwind v4 pur sans daisyUI (Principe VI) ; messages ≤ 2000 caractères
**Scale/Scope**: plateforme communautaire, mono-backend ; messagerie 1-1 (pas de groupes), texte uniquement

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Justification |
|----------|--------|---------------|
| I. Français d'abord | ✅ PASS | Tables/colonnes/structs/composables en français ; UI en français. |
| II. Monorepo cohérent | ✅ PASS | Livraison cross-stack (SQL → Rust → TS) cohérente ; types alignés via `contracts/api.md`. |
| III. SQL source de vérité | ✅ PASS | Schéma `29_social.sql` écrit en premier ; structs `FromRow` + interfaces TS en découlent. Conventions UUID/TIMESTAMPTZ/soft-delete/snake_case respectées. |
| IV. Sécurité par défaut | ✅ PASS | JWT sur tous les endpoints ; requêtes paramétrées sqlx ; validation (≤2000, non vide, états actifs) ; blocage ; rate-limit (FR-014) ; liste d'amis privée (FR-026) ; pas de secret en dur. SSE auth par token (Décision 3, exposition limitée documentée). |
| V. Simplicité (YAGNI) | ✅ PASS | SSE plutôt que WebSocket ; pas de table rate-limit dédiée ; composables (pas de store ad hoc) ; ordre canonique des paires. Nouveau schéma justifié (bounded-context). |
| VI. Tailwind v4 (daisyUI back-office only) | ✅ PASS | Aucune surface admin ; toute l'UI (public + espace membre) en Tailwind v4 pur, **sans daisyUI** (Décision 8). |
| VII. Audit & traçabilité | ✅ PASS | **Toutes** les mutations auditées via `log_action`. Pour les messages, l'audit ne capture que des **métadonnées** (id message/conversation/expéditeur, longueur) — jamais le contenu, pour préserver la confidentialité. |

**Verdict** : PASS. Aucune violation ; aucune déviation.

## Project Structure

### Documentation (this feature)

```text
specs/001-demande-amitie/
├── plan.md              # Ce fichier
├── research.md          # Phase 0 — décisions techniques
├── data-model.md        # Phase 1 — schéma social
├── quickstart.md        # Phase 1 — mise en route & validation
├── contracts/
│   └── api.md           # Phase 1 — contrats REST + SSE
├── checklists/
│   └── requirements.md  # Qualité de la spec
└── tasks.md             # Phase 2 (/speckit.tasks — non créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                       # + \ir schemas/29_social.sql
│   └── schemas/29_social.sql            # NOUVEAU : schéma social complet
├── src/
│   ├── models/
│   │   ├── amitie.rs                    # NOUVEAU : demande, amitié, blocage, notification
│   │   ├── messagerie.rs                # NOUVEAU : conversation, message, DTO SSE
│   │   └── mod.rs                       # + déclarations
│   ├── handlers/
│   │   ├── amitie.rs                    # NOUVEAU : demandes, amitiés, blocages, notifications
│   │   ├── messagerie.rs                # NOUVEAU : conversations, messages, flux SSE
│   │   └── mod.rs                       # + déclarations
│   ├── services/
│   │   ├── messagerie_sse.rs            # NOUVEAU : registre connexions + broadcaster
│   │   └── mod.rs                       # + déclaration
│   ├── routes.rs                        # + scope /api/amities et /api/messagerie
│   └── main.rs                          # + état partagé du registre SSE (web::Data)

uafricas_frontend/
├── app/
│   ├── composables/
│   │   ├── useAmis.ts                   # NOUVEAU : demandes, amitiés, blocages, état relation
│   │   └── useMessagerie.ts             # NOUVEAU : conversations, messages, état global non-lus
│   ├── plugins/
│   │   └── messagerie.client.ts         # NOUVEAU : ouverture/gestion du flux SSE après auth
│   ├── components/social/
│   │   ├── BoutonAmitie.vue             # NOUVEAU : bouton d'état sur /profil
│   │   ├── MessagerieFlottante.vue      # NOUVEAU : bouton flottant + fenêtre
│   │   ├── ListeAmis.vue                # NOUVEAU : liste d'amis dans la fenêtre
│   │   └── FenetreConversation.vue      # NOUVEAU : fil de messages + saisie
│   ├── layouts/
│   │   └── default.vue                  # + <SocialMessagerieFlottante> (client, si connecté)
│   ├── components/layout/
│   │   └── NavBar.vue                   # + lien « Mes amis » vers /mon-compte/amis
│   └── pages/
│       ├── profil/index.vue             # + BoutonAmitie sur les cartes
│       ├── profil/[id].vue              # + BoutonAmitie + état relation
│       └── mon-compte/amis.vue          # NOUVEAU : gestion (amis/demandes/bloqués)
```

**Structure Decision**: Application web monorepo. Le domaine social est isolé dans un schéma SQL `social`, des modules backend dédiés (`amitie`, `messagerie`, `messagerie_sse`) et un dossier frontend `components/social/` + deux composables, conformément aux conventions « un fichier/domaine » du projet. Le bouton flottant est monté une seule fois dans le layout `default.vue` (présent sur toutes les pages publiques/membre).

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Nouveau schéma `social` (vs rattachement à `iam`) | Bounded-context dédié (amitié + messagerie) au cycle de vie propre ; cohérent avec `retrouve_amis`, `arbre_genealogique` | Mettre dans `iam` mélangerait identité/accès et relations sociales ; `retrouve_amis` est du matching, sémantiquement distinct |

> Aucune déviation au Principe VII : les messages **sont** audités (métadonnées seules, sans contenu — Décision 9 de research.md).
