# Implementation Plan: Rendez-vous en visioconférence entre membres amis

**Branch**: `001-rendez-vous-visio` | **Date**: 2026-05-26 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-rendez-vous-visio/spec.md`

## Summary

Permettre à deux membres **amis** d'organiser et de tenir un entretien vidéo 1-à-1. Le backend (Rust/Actix) orchestre uniquement la prise de rendez-vous (proposer → répondre/contre-proposer → accepter → annuler), persiste l'état dans le schéma `social`, notifie en temps réel (SSE existant) et par cloche persistante. La visioconférence est **pair-à-pair (WebRTC via PeerJS)** : aucun média ne transite par le serveur. Les identifiants de pair sont **déterministes** (dérivés de l'identifiant du rendez-vous + de chaque participant), de sorte qu'aucune signalisation applicative n'est nécessaire ; STUN public Google, sans TURN dans ce lot (limite NAT symétrique documentée, repli messagerie). La feature réutilise intégralement le domaine social : amitié/blocage, `MembreLight`, messagerie privée flottante (qui héberge la vue de gestion), SSE et cloche.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 (Vue 3 SSR) (frontend)  
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde, `audit::log_action`, `RegistreSse` (SSE) (backend) ; Pinia, Tailwind CSS v4 (pur), FontAwesome, **peerjs (à ajouter via pnpm)** (frontend)  
**Storage**: PostgreSQL 16, schéma `social` (source de vérité — Principe III). Cloche persistante via `arbre_genealogique.notifications` (système cloche unifié existant). Aucun stockage de média (P2P).  
**Testing**: Aucun framework configuré (Principe — pas de CI/CD encore). Tests manuels via quickstart + scénarios d'acceptation.  
**Target Platform**: Navigateurs modernes (WebRTC : `getUserMedia`, RTCPeerConnection) ; backend Linux (port 8082 en dev).  
**Project Type**: Application web (monorepo frontend Nuxt + backend Actix).  
**Performance Goals**: Notification temps réel < 5 s (SC-002) ; établissement connexion vidéo < 10 s sur réseaux compatibles (SC-004) ; média 0 bande passante serveur (SC-005).  
**Constraints**: Tailwind v4 pur côté public (Principe VI) ; FR/accents dans l'UI, noms de fichiers sans accents ; JWT Bearer ; revérification amitié+blocage à chaque action (FR-034) ; aucun contenu sensible (sujet/description) dans l'audit (FR-033) ; hôte PeerJS + liste ICE configurables par variables d'environnement frontend.  
**Scale/Scope**: Échange 1-à-1 entre amis ; volumétrie faible (pas d'enjeu de montée en charge). Périmètre : 1 table + 1 enum SQL, 1 handler + 1 model backend, ~6 endpoints, 1 composable + 1 section panneau + 1 modal + 1 salle visio + 1 bouton profil frontend.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Conformité | Note |
|----------|-----------|------|
| I. Français d'abord | ✅ | Code/colonnes/UI en français (snake_case français : `rendez_vous`, `date_heure`, `duree_minutes`, `tour_id`). |
| II. Monorepo cohérent | ✅ | Types cohérents SQL ↔ struct Rust `FromRow` ↔ interface TS ; livrés ensemble. |
| III. SQL source de vérité | ✅ | Migration `schemas/31_social_rendez_vous.sql` d'abord, intégrée à l'orchestrateur, puis backend, puis frontend. |
| IV. Sécurité par défaut | ✅ (avec note) | JWT Bearer, sqlx paramétré, revérif amitié/blocage à chaque action, audit sans contenu sensible. **Note signalisation** : peer-id = hachage déterministe `(rendez_vous_id, participant_id)` ; le secret d'accès est l'UUID du rendez-vous, connu des seuls participants via l'API. STUN public, pas de TURN (documenté). Voir research.md §Sécurité. |
| V. Simplicité (YAGNI) | ✅ | 1 handler + 1 model par domaine, 1 composable, pas de Repository, pas de FK conversation (lien messagerie côté frontend via `demanderOuverture`). « expiré »/« terminé » dérivés par calcul (pas de statut ni tâche planifiée — clarifications). |
| VI. Tailwind v4 (daisyUI back-office only) | ✅ | UI membre (modal proposition, section panneau, salle visio, bouton profil) en Tailwind v4 pur. |
| VII. Audit & traçabilité | ✅ | `audit::log_action` sur chaque mutation (proposer/accepter/refuser/contre-proposer/annuler), sans sujet/description. |

**Résultat** : PASS. Aucune violation à justifier (Complexity Tracking vide).

## Project Structure

### Documentation (this feature)

```text
specs/001-rendez-vous-visio/
├── plan.md              # Ce fichier (/speckit.plan)
├── research.md          # Phase 0 — décisions techniques
├── data-model.md        # Phase 1 — entités & schéma
├── quickstart.md        # Phase 1 — mise en route & tests manuels
├── contracts/
│   └── rendez-vous.md   # Phase 1 — contrats d'API
├── checklists/
│   └── requirements.md  # /speckit.specify
└── tasks.md             # Phase 2 (/speckit.tasks — non créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                              # + ligne \ir schemas/31_social_rendez_vous.sql
│   └── schemas/
│       └── 31_social_rendez_vous.sql           # NOUVEAU — enum + table + index (idempotent)
└── src/
    ├── models/
    │   └── rendez_vous.rs                       # NOUVEAU — structs FromRow, DTO, COLONNES, evt_* SSE, peer-id
    ├── handlers/
    │   └── rendez_vous.rs                       # NOUVEAU — proposer/lister/detail/salle/accepter/refuser/contre/annuler
    ├── models/mod.rs                            # + pub mod rendez_vous;
    ├── handlers/mod.rs                          # + pub mod rendez_vous;
    └── routes.rs                                # + scope /rendez-vous

uafricas_frontend/
├── package.json                                # + peerjs (pnpm add peerjs)
├── nuxt.config.ts                              # + runtimeConfig.public.peerjs* / iceServers
├── app/
│   ├── composables/
│   │   └── useRendezVous.ts                     # NOUVEAU — $fetch + useState + gererEvenement
│   ├── components/
│   │   └── social/
│   │       ├── RendezVousProposerModal.vue      # NOUVEAU — formulaire (sujet/desc/date/heure/durée)
│   │       ├── RendezVousListe.vue              # NOUVEAU — vue gestion (4 filtres) + carte
│   │       ├── RendezVousCarte.vue              # NOUVEAU — carte RDV (MembreLight + actions)
│   │       └── RendezVousSalle.vue              # NOUVEAU — salle visio P2P (PeerJS)
│   ├── components/social/MessagerieFlottante.vue # MODIF — 3e onglet « Rendez-vous »
│   ├── plugins/messagerie.client.ts             # MODIF — dispatch evt rdv_* → useRendezVous + refresh cloche
│   └── pages/profil/[id].vue                    # MODIF — bouton « Proposer un rendez-vous » (si etat==='amis')
```

**Structure Decision**: Application web monorepo (Option 2). Le backend suit le pattern « 1 handler + 1 model par domaine » (calqué sur `amitie.rs` / `messagerie.rs`). Le frontend suit « 1 composable par domaine » + composants `social/` (PascalCase, préfixe auto `Social…`). La vue de gestion est hébergée dans le panneau de messagerie flottant existant (clarification Q3), pas une page dédiée.

## Complexity Tracking

> Aucune violation de la Constitution. Section non requise.
