# Implementation Plan: Refonte salles Afrolang, streaming direct & salles privées par code secret

**Branch**: `001-afrolang-salles-refonte` | **Date**: 2026-04-15 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-afrolang-salles-refonte/spec.md`

## Summary

Refonte de la feature Afrolang « salles » livrée précédemment (`005-afrolang-salles`), pour remettre le produit en conformité avec l'intention initiale.

**Objectifs produit** :

1. Retirer la section « Annuaire des groupes ethniques » de `/afrolang`.
2. Faire en sorte que le bouton « Démarrer / Rejoindre » de chaque carte de salle publique entre directement dans la session live (LiveKit), sans page intermédiaire.
3. Supprimer la page `/afrolang/salle-privee/[id].vue` et toutes les pages dédiées aux salles privées : la création et l'accès se font via modale et widget dropdown déjà présent sur la carte.
4. Permettre à n'importe quel utilisateur connecté de créer **une** salle privée par salle publique (durable), et de la démarrer/rejoindre indépendamment de l'état de la salle publique parente.
5. Remplacer le contrôle d'accès legacy (adhésion / invitation / modérateur attitré) par un **code secret** unique (hashé), saisi par tout utilisateur non-auteur pour entrer.

**Approche technique** :

- **BDD** : table rase des tables legacy spécifiques aux salles privées (`afrolang.salle_privee_adhesion`, `afrolang.proposition_salle`, `afrolang.salle_moderateur` propres aux salles privées). Migration SQL ajoute `code_acces_hash CHAR(60)` (bcrypt cost 10) en remplacement de `code_acces` clair, supprime les colonnes `motif`, `declaration_adulte_at`, `visibilite` désormais obsolètes côté `salle_privee`.
- **Backend Rust/Actix-Web 4** : nouveaux endpoints publics minces sur `salle_privee` (créer, rouvrir = démarrer session, vérifier code, lister par salle publique). Suppression des endpoints d'adhésion/invitation/modérateur attitré liés aux salles privées. Création de salle publique restreinte au rôle `admin` (déjà en place côté admin handlers, vérification et nettoyage des points d'entrée publics).
- **Frontend Nuxt 4 / Vue 3** : nettoyage page `/afrolang` (suppression `AnnuaireGroupesEthniques`), recâblage du bouton « Démarrer » vers `/afrolang/session/[salleId]`, suppression des routes/composants legacy (`/afrolang/salle-privee/[id].vue`, `/afrolang/proposer.vue`, `ProposerSalleModal`, `PropositionCard`, `SalleModerationPanel`, `SallePriveeVisibilitePanel`, `DemandeAdhesionCard`, `InvitationBanner`), adaptation des composants `SallePriveeCreateModal` et `SallePriveeJoinModal` au nouveau modèle code-secret.

## Technical Context

**Language/Version** : Rust Edition 2024 (backend) + TypeScript / Nuxt 4 / Vue 3 SSR (frontend)
**Primary Dependencies** : Actix-Web 4, sqlx (PostgreSQL), bcrypt (hash code secret), uuid, chrono, livekit-api (déjà présent) ; côté frontend Pinia, $fetch, FontAwesome, daisyUI **réservé aux pages admin** (constitution Principe VI), Tailwind CSS v4 pur sur le public
**Storage** : PostgreSQL 16, schema `afrolang` (modifié par migration `08b_afrolang_refonte.sql`)
**Testing** : aucun framework de test configuré dans le projet (cf. CLAUDE.md). Validation manuelle via parcours utilisateur + commandes `cargo build` / `pnpm dev`. À documenter en quickstart.
**Target Platform** : Linux server (backend), navigateurs modernes (frontend SSR Nuxt)
**Project Type** : Web application (monorepo backend + frontend + BDD), Constitution Principe II
**Performance Goals** : SC-001 ≤ 3 s pour entrée dans une session live publique ; SC-006 ≤ 2 s pour vérification code secret + entrée salle privée
**Constraints** : Tailwind v4 pur sur le public (pas de daisyUI, Principe VI). Audit `audit::log_action` non bloquant sur chaque mutation (Principe VII). Code secret jamais stocké en clair (Principe IV, Sécurité par défaut)
**Scale/Scope** : ~5 endpoints publics modifiés/ajoutés sur `salle_privee` ; 1 migration SQL ; ~10 composants frontend impactés (suppressions + adaptations) ; 1 page `/afrolang` refactorisée ; 0 nouveau composant frontend (réutilisation des modales existantes)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Vérification | Statut |
|---|---|---|
| **I : Français d'Abord** | Toutes les variables, colonnes SQL (`code_acces_hash`, `cree_par`, `archivee_at`, `salle_privee`, etc.), composants et messages UI sont en français. Migration SQL nommée `08b_afrolang_refonte.sql`. | ✅ Pass |
| **II : Monorepo Cohérent** | Modifications cross-stack (SQL + Rust + Vue) livrées dans la même branche `001-afrolang-salles-refonte`. Types `SallePriveeAPI` ↔ struct Rust ↔ schéma SQL maintenus cohérents. | ✅ Pass |
| **III : SQL Source de Vérité** | Le plan part du schéma SQL (migration 08b), puis dérive backend (handlers/models) puis frontend (composables/types). Les mocks ne sont pas concernés (feature en BDD réelle). | ✅ Pass |
| **IV : Sécurité par Défaut** | Code secret hashé bcrypt cost 10 (allègement vs cost 12 mots de passe : code court, vérif fréquente, payload faible entropie, détaillé dans research.md R3). Rate limit 5 tentatives/min/utilisateur/salle (R4). Vérification code via requête paramétrée sqlx. JWT existant inchangé. Audit appliqué. | ✅ Pass |
| **V : Simplicité (YAGNI)** | Aucune abstraction nouvelle. Pas de pattern Repository ajouté. Réutilisation des composables existants `useAfrolang`. Suppression nette de code legacy plutôt que feature flags. | ✅ Pass |
| **VI : Tailwind v4 (daisyUI back-office uniquement)** | Composants publics (`SallePriveeCreateModal`, `SallePriveeJoinModal`, page `/afrolang`) restent en Tailwind v4 pur. Pages admin Afrolang non touchées. | ✅ Pass |
| **VII : Audit & Traçabilité** | Toutes les nouvelles mutations (`creer_salle_privee_publique`, `modifier_code_secret`, `archiver_salle_privee_par_auteur`, `verifier_code_secret_echec`) appellent `audit::log_action`. | ✅ Pass |

**Résultat** : aucune violation. Pas d'entrée à ajouter dans Complexity Tracking.

## Project Structure

### Documentation (this feature)

```text
specs/001-afrolang-salles-refonte/
├── plan.md              # Ce fichier
├── spec.md              # Spec produit (déjà rédigée)
├── research.md          # Phase 0 : décisions techniques (R1→R6)
├── data-model.md        # Phase 1 : schéma SQL cible & entités
├── quickstart.md        # Phase 1 : instructions de validation manuelle
├── contracts/
│   └── salles-privees-public-api.md   # Phase 1, contrats endpoints publics
├── checklists/
│   └── requirements.md  # checklist qualité spec (déjà créée)
└── tasks.md             # Phase 2 : généré par /speckit.tasks (NON créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 08b_afrolang.sql                          # MODIFIE in-place (table rase + nouvelles colonnes)
│
├── src/
│   ├── handlers/
│   │   ├── afrolang.rs                           # MODIFIE, endpoints publics salle_privee :
│   │   │                                         #   creer, lister_par_salle_publique,
│   │   │                                         #   verifier_code_acces, modifier_code_acces,
│   │   │                                         #   archiver_par_auteur
│   │   └── admin/
│   │       ├── salles_privees.rs                 # SIMPLIFIE, visibilite/adhesion/invitation supprimés
│   │       ├── propositions_afrolang.rs          # SUPPRIME (création publique uniquement par admin)
│   │       └── moderateurs_afrolang.rs           # SUPPRIME si dédié salles privées ; sinon CONSERVE
│   │                                             #   pour salles publiques
│   ├── models/
│   │   ├── afrolang.rs                           # MODIFIE, DTOs alignés sur nouveau schéma
│   │   └── admin/
│   │       ├── salle_privee.rs                   # MODIFIE, colonnes legacy retirées
│   │       └── propositions_afrolang.rs          # SUPPRIME
│   ├── routes.rs                                 # MODIFIE, nouvelles routes publiques + retraits
│   └── services/
│       └── audit.rs                              # INCHANGE, utilisé tel quel
│
└── (autres fichiers backend inchangés)

uafricas_frontend/
├── app/
│   ├── pages/afrolang/
│   │   ├── index.vue                             # MODIFIE, retire AnnuaireGroupesEthniques,
│   │   │                                         #   recâble bouton "Démarrer" vers /session/[id]
│   │   ├── [id].vue                              # CONSERVE, fiche salle publique (ou MODIFIE
│   │   │                                         #   pour rediriger directement vers /session/[id])
│   │   ├── proposer.vue                          # SUPPRIME (création par admin uniquement)
│   │   ├── salle-privee/                         # SUPPRIME entièrement (FR-006)
│   │   │   └── [id].vue
│   │   └── session/                              # CONSERVE, page livestream LiveKit
│   │       └── [id].vue
│   ├── components/afrolang/
│   │   ├── AnnuaireGroupesEthniques.vue          # SUPPRIME (FR-001)
│   │   ├── ProposerSalleModal.vue                # SUPPRIME
│   │   ├── PropositionCard.vue                   # SUPPRIME
│   │   ├── SalleModerationPanel.vue              # SUPPRIME (legacy modérateurs salle privée)
│   │   ├── SallePriveeVisibilitePanel.vue        # SUPPRIME (visibilité abandonnée)
│   │   ├── DemandeAdhesionCard.vue               # SUPPRIME (adhésion abandonnée)
│   │   ├── InvitationBanner.vue                  # SUPPRIME (invitation abandonnée)
│   │   ├── SalleCard.vue                         # MODIFIE, bouton Démarrer/Rejoindre
│   │   ├── SallePriveeCard.vue                   # MODIFIE, affiche état dormante/live
│   │   ├── SallePriveeCreateModal.vue            # MODIFIE, champs : titre, code secret
│   │   └── SallePriveeJoinModal.vue              # MODIFIE, champ unique : code secret
│   ├── composables/
│   │   └── useAfrolang.ts                        # MODIFIE, supprime adhesion/invitation/proposition,
│   │                                             #   ajoute verifierCodeAcces,
│   │                                             #   modifierCodeAcces
│   └── stores/                                   # INCHANGE
│
└── (autres fichiers frontend inchangés)
```

**Structure Decision** : monorepo web app (Constitution Principe II). Modifications cantonnées à `uafricas_backend/{doc/bd/schemas, src/handlers, src/models, src/routes.rs}` et à `uafricas_frontend/app/{pages/afrolang, components/afrolang, composables/useAfrolang.ts}`. Aucun nouveau dossier ; suppressions nettes plutôt que feature flags (Principe V).

## Complexity Tracking

> *(Aucune violation Constitution Check, section laissée vide.)*
