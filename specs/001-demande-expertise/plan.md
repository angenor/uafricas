# Implementation Plan: Demande pour devenir expert avec validation admin

**Branch**: `001-demande-expertise` | **Date**: 2026-05-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-demande-expertise/spec.md`

## Summary

Remplacer le lien « Apporter mon expertise » (qui pointe aujourd'hui vers la liste publique `/experts`) par un parcours de candidature : un formulaire dédié où un membre connecté complète son profil de base (photo, fonction, pays) et renseigne son expertise (domaine, biographie, expérience, situations, portfolio), puis soumet sa demande. La demande est créée au statut `en_attente` et reste invisible publiquement. Un administrateur valide ou refuse la demande depuis le back-office ; le candidat est notifié **par email** ; après validation l'expert apparaît sur `/experts`.

**Approche technique** : réutilisation maximale de l'existant. La table `iam.expertise` (statut `en_attente`/`valide`/`refuse`, `valide_par`, `date_validation`) et l'endpoint `POST /api/experts/candidature` existent déjà, de même que le filtrage `statut='valide'` sur `/experts` et la mise à jour de profil (`PUT /api/auth/profil`, `POST /api/auth/profil/photo`). Les manques à combler : (1) migration SQL mineure (index unique partiel + colonne `commentaire_admin` + extension `ModifierProfilRequest` avec `pays_residence_id`), (2) endpoints admin de modération (handler `admin/expertise.rs`), (3) endpoint `GET /api/experts/moi` + ajustement de `creer_candidature` pour la re-soumission après refus, (4) emails de décision, (5) le formulaire frontend public, les pages admin, et le changement de lien.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) ; TypeScript / Nuxt 4 / Vue 3 SSR (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), lettre (SMTP, déjà présent), uuid, chrono, serde (backend) ; Pinia, $fetch, FontAwesome, daisyUI v5 (back-office uniquement) (frontend)
**Storage**: PostgreSQL 16 : schema `iam` existant (table `iam.expertise` étendue, aucun nouveau schema)
**Testing**: Aucun framework configuré (Principe « pas de CI/CD »), validation manuelle via quickstart
**Target Platform**: Serveur Linux (backend port 8080) + SSR Nuxt (port 3000)
**Project Type**: web (monorepo frontend + backend)
**Performance Goals**: Standard web ; liste admin paginée (≤ 100/page), formulaire soumis en < 3 min (SC-002)
**Constraints**: Français obligatoire ; site public en Tailwind v4 pur (pas de daisyUI) ; audit obligatoire sur mutations ; notification par email uniquement
**Scale/Scope**: Volume modéré (candidatures expert) ; ~1 nouveau handler admin, ~2 pages admin, 1 page publique, 1 composable admin, migration SQL mineure

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Conformité | Note |
|----------|-----------|------|
| I. Français d'Abord | ✅ | Tout le code/UI/colonnes en français |
| II. Monorepo Cohérent | ✅ | Changements cross-stack (SQL → backend → frontend) livrés ensemble ; types TS ↔ structs Rust ↔ SQL cohérents |
| III. SQL Source de Vérité | ✅ | Migration `04b_iam_expertise.sql` d'abord (index partiel + `commentaire_admin`), puis propagation backend/frontend |
| IV. Sécurité par Défaut | ✅ | JWT existant ; endpoints admin via extracteur `AdminUtilisateur` + `verifier_permission!` ; requêtes paramétrées sqlx ; validation des entrées ; pas de secrets en dur |
| V. Simplicité (YAGNI) | ✅ | Réutilisation de la table, des endpoints profil et du pattern biblio-humaine ; pas de nouvelle abstraction ; pas de table notification (email uniquement) |
| VI. Tailwind v4 (daisyUI back-office) | ✅ | `pages/devenir-expert.vue` (public) = Tailwind v4 pur ; pages `admin/experts/*` = daisyUI |
| VII. Audit & Traçabilité | ✅ | `audit::log_action` sur validation et refus |

**Résultat** : PASS : aucune violation. Section Complexity Tracking non requise.

## Project Structure

### Documentation (this feature)

```text
specs/001-demande-expertise/
├── plan.md              # Ce fichier
├── spec.md              # Spécification (entrée)
├── research.md          # Phase 0 : décisions techniques
├── data-model.md        # Phase 1 : modèle de données
├── quickstart.md        # Phase 1 : scénarios de validation manuelle
├── contracts/
│   └── api.md           # Phase 1 : contrats d'API
├── checklists/
│   └── requirements.md  # Checklist qualité (déjà créé par /speckit.specify)
└── tasks.md             # Phase 2 (généré par /speckit.tasks, PAS ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   ├── 04b_iam_expertise.sql        # MODIFIÉ : index unique partiel + colonne commentaire_admin
│   └── 15_seed.sql                  # MODIFIÉ : permissions expertise.voir / expertise.valider
├── src/
│   ├── models/
│   │   ├── utilisateur.rs           # MODIFIÉ : ModifierProfilRequest + pays_residence_id
│   │   ├── expert.rs                # MODIFIÉ : DTO statut/commentaire pour réponse "ma candidature"
│   │   └── admin/
│   │       ├── mod.rs               # MODIFIÉ : déclaration sous-module expertise
│   │       └── expertise.rs         # NOUVEAU : DTO admin (liste, détail, body traiter)
│   ├── handlers/
│   │   ├── auth.rs                  # MODIFIÉ : modifier_profil gère pays_residence_id
│   │   ├── experts.rs               # MODIFIÉ : creer_candidature (re-soumission) + ma_candidature
│   │   └── admin/
│   │       ├── mod.rs               # MODIFIÉ : déclaration sous-module expertise
│   │       └── expertise.rs         # NOUVEAU : lister/obtenir/valider/rejeter demandes
│   ├── email.rs                     # MODIFIÉ : email décision (approuvée / refusée)
│   └── routes.rs                    # MODIFIÉ : GET /experts/moi + 4 routes admin
└── ...

uafricas_frontend/
├── app/
│   ├── components/layout/
│   │   └── BoutonLateralGauche.vue  # MODIFIÉ : lien "Apporter mon expertise" → /devenir-expert
│   ├── composables/
│   │   ├── useExperts.ts            # MODIFIÉ : obtenirMaCandidature + soumission complète
│   │   └── useAdminExperts.ts       # NOUVEAU : CRUD modération admin
│   ├── pages/
│   │   ├── devenir-expert.vue       # NOUVEAU : formulaire public (Tailwind v4 pur)
│   │   ├── mon-compte/profil.vue    # MODIFIÉ : onglet "Expertise" (suivi statut + re-soumission)
│   │   └── admin/experts/
│   │       ├── index.vue            # NOUVEAU : liste filtrable des demandes (daisyUI)
│   │       └── [id].vue             # NOUVEAU : détail + actions valider/rejeter (daisyUI)
└── ...
```

**Structure Decision** : Application web (monorepo). Les changements respectent l'organisation feature-based existante (un handler/model admin par domaine, un composable par domaine, pages publiques en Tailwind v4 pur et pages admin en daisyUI). Aucun nouveau schema PostgreSQL : extension du schema `iam` existant.

## Complexity Tracking

> Aucune violation de la Constitution, section non applicable.
