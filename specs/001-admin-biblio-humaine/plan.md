# Implementation Plan: Validation Admin des Bibliothèques Humaines

**Branch**: `001-admin-biblio-humaine` | **Date**: 2026-04-22 | **Spec**: [spec.md](spec.md)

## Summary

Ajouter un workflow de validation admin pour les demandes de Bibliothèque Humaine. Actuellement, l'inscription active directement le flag `bibliotheque_humain = TRUE`. Cette feature introduit un état intermédiaire `en_attente` via une nouvelle table `iam.demande_biblio_humaine`, une interface admin pour approuver/rejeter les demandes, et la visibilité du statut côté candidat.

## Technical Context

**Language/Version**: Rust 2024 Edition (backend), TypeScript / Nuxt 4 (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), Pinia, $fetch
**Storage**: PostgreSQL 16 — schema `iam` (2 nouvelles tables + 1 enum)
**Testing**: N/A (pas de CI configuré)
**Target Platform**: Linux server (prod) / macOS (dev)
**Project Type**: Web application (frontend SSR + backend API REST)
**Performance Goals**: Réponse API < 500ms p95 pour les listes admin
**Constraints**: Français obligatoire dans le code, daisyUI autorisé côté admin uniquement, audit::log_action sur toutes les mutations
**Scale/Scope**: ~100 demandes attendues à l'ouverture

## Constitution Check

| Principe | Statut | Notes |
|----------|--------|-------|
| I. Français d'Abord | ✅ | Variables, colonnes SQL, messages UI en français |
| II. Monorepo Cohérent | ✅ | Modifications frontend + backend dans la même PR |
| III. SQL Source de Vérité | ✅ | DDL d'abord, puis backend, puis frontend |
| IV. Sécurité par Défaut | ✅ | JWT requis sur tous les endpoints admin + candidat |
| V. Simplicité (YAGNI) | ✅ | Pas de workflow multi-approbateurs, pas d'email |
| VI. Tailwind v4 (daisyUI admin) | ✅ | daisyUI pour pages admin, Tailwind pur pour profil public |
| VII. Audit & Traçabilité | ✅ | `audit::log_action` sur valider + rejeter |

## Project Structure

### Documentation (cette feature)

```text
specs/001-admin-biblio-humaine/
├── plan.md              ✅ Ce fichier
├── research.md          ✅ Phase 0
├── data-model.md        ✅ Phase 1
├── quickstart.md        ✅ Phase 1
├── contracts/
│   └── api.md           ✅ Phase 1
└── tasks.md             ⏳ Phase 2 (/speckit.tasks)
```

### Source Code

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 04b_iam_biblio_demande.sql          CRÉER
├── src/
│   ├── models/
│   │   ├── bibliotheque_humaine.rs          MODIFIER (+DemandeCreeeResponse, MaDemandeResponse)
│   │   └── admin/
│   │       ├── mod.rs                       MODIFIER (+biblio_humaine)
│   │       └── biblio_humaine.rs            CRÉER
│   ├── handlers/
│   │   ├── bibliotheques_humaines.rs        MODIFIER (inscrire_biblio → crée demande)
│   │   └── admin/
│   │       ├── mod.rs                       MODIFIER (+bibliotheques_humaines)
│   │       └── bibliotheques_humaines.rs    CRÉER
│   └── routes.rs                            MODIFIER (+routes admin + moi/demande)

uafricas_frontend/
└── app/
    ├── composables/
    │   ├── useBibliothequeHumaine.ts        MODIFIER (+obtenirMaDemande)
    │   └── useAdminBibliosHumaines.ts       CRÉER
    └── pages/
        ├── profil.vue                       MODIFIER (+affichage statut demande)
        └── admin/
            └── bibliotheques-humaines/
                ├── index.vue                CRÉER
                └── [id].vue                 CRÉER
```

## Phase 0 — Recherche ✅

Voir [research.md](research.md).

**Décisions clés** :
- Nouvelle table `iam.demande_biblio_humaine` (statut : en_attente / valide / rejete)
- `inscrire_biblio` crée une demande au lieu de setter directement le flag
- La validation admin applique les changements de profil en transaction atomique
- Pattern admin existant (`useAdminCandidatures`) réutilisé comme modèle

## Phase 1 — Design & Contrats ✅

- [data-model.md](data-model.md) — DDL SQL, types Rust, types TypeScript
- [contracts/api.md](contracts/api.md) — 6 endpoints (2 modifiés, 1 nouveau public, 3 nouveaux admin)
- [quickstart.md](quickstart.md) — ordre d'implémentation + vérification rapide

## Phase 2 — Tâches (à générer via `/speckit.tasks`)

### Groupe A — SQL & Modèles (base)
- A1 : DDL `04b_iam_biblio_demande.sql` (enum + 2 tables + 3 index)
- A2 : Types Rust dans `bibliotheque_humaine.rs` (DemandeCreeeResponse, MaDemandeResponse)
- A3 : Modèle admin `src/models/admin/biblio_humaine.rs`
- A4 : Déclarations dans `mod.rs` (models + handlers)

### Groupe B — Backend handlers (dépend A)
- B1 : Modifier `inscrire_biblio` — créer demande + vérifier unicité active
- B2 : Ajouter `GET /moi/demande`
- B3 : Modifier `lister_biblios` — filtrer sur demande valide
- B4 : Handler admin lister + détail
- B5 : Handler admin `valider_demande` (transaction atomique + audit)
- B6 : Handler admin `rejeter_demande` (+ audit)
- B7 : Routes dans `routes.rs`

### Groupe C — Frontend composables (dépend B)
- C1 : `obtenirMaDemande()` dans `useBibliothequeHumaine.ts`
- C2 : Créer `useAdminBibliosHumaines.ts`

### Groupe D — Frontend pages admin (dépend C)
- D1 : `admin/bibliotheques-humaines/index.vue`
- D2 : `admin/bibliotheques-humaines/[id].vue`

### Groupe E — Frontend candidat (dépend C)
- E1 : Modifier `profil.vue` — encart statut demande

## Risques & Mitigations

| Risque | Mitigation |
|--------|------------|
| Race condition double soumission | Index UNIQUE sur `(utilisateur_id)` WHERE statut IN ('en_attente','valide') |
| Régression listing public | Vérifier que `lister_biblios` retourne uniquement les `valide` |
| Transaction validation partielle | `BEGIN/COMMIT` explicite avec rollback sqlx |
