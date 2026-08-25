# Implementation Plan: Administrateurs de salle publique & propositions communautaires

**Branch**: `001-admin-salles-publiques` | **Date**: 2026-05-10 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-admin-salles-publiques/spec.md`

## Summary

Réintroduire et stabiliser deux mécanismes liés autour des salles publiques Afrolang (schéma `afrolang`) :

1. **Propositions communautaires** : tout utilisateur authentifié soumet une proposition de salle publique (langue, groupe ethnique, pays d'origine, justification) ; un administrateur de la plateforme valide (création atomique de `afrolang.salle`) ou rejette (commentaire obligatoire).
2. **Administrateurs de salle publique** : nouveau rôle scopé à une salle, distinct de l'« administrateur de la plateforme » et **distinct du modérateur attitré existant** (`afrolang.salle_moderateur` reste inchangée). Les capacités opérationnelles concrètes sont **explicitement reportées** (FR-019) : on livre uniquement la nomination/révocation, l'audit, la visibilité publique et le point d'autorisation centralisé pour brancher les pouvoirs ultérieurement.

Approche technique : nouvelle table `afrolang.proposition_salle` (workflow en_attente → validee/rejetee/retiree, transition atomique vers `afrolang.salle` à la validation) + nouvelle table `afrolang.salle_administrateur` (séparée de `salle_moderateur` pour préserver la sémantique distincte exigée par FR-018) + extension du middleware d'autorisation Actix pour exposer la prédicat `est_admin_salle(salle_id, user_id)` réutilisable. UI : 1 page publique de soumission + suivi perso, 1 page admin file d'attente + détail, 1 onglet « Administrateurs » sur la fiche salle admin, + affichage public léger des admins de salle.

## Technical Context

**Language/Version** : Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 SSR (frontend)
**Primary Dependencies** : Actix-Web 4, sqlx (PostgreSQL async), uuid, chrono, serde (backend) ; Pinia, $fetch, FontAwesome (frontend), **aucune nouvelle dépendance**
**Storage** : PostgreSQL 16 : schéma `afrolang` étendu (2 nouvelles tables + 1 enum) ; aucun nouveau bounded-context (Principe III)
**Testing** : Aucune infra de test configurée à ce jour (CLAUDE.md). Validation manuelle via `quickstart.md` et inspection Adminer/curl.
**Target Platform** : Linux server (backend Actix), web SSR Nuxt 4
**Project Type** : Web application (monorepo `uafricas_frontend/` + `uafricas_backend/`)
**Performance Goals** : SC-004 notification < 60 s post-décision, SC-008 suspension cascade < 60 s. Charge attendue : < 10 décisions modération / jour. Indexation pour file d'attente paginée < 100 ms.
**Constraints** :
- Réutiliser `audit::log_action` (Principe VII) pour toute mutation
- Ne jamais fusionner sémantiquement avec `salle_moderateur` (FR-018), table séparée obligatoire
- Backend : pas de modification du JWT, pas de nouveau secret, pas de service externe
- Frontend public : Tailwind v4 pur (Principe VI) ; admin : daisyUI v5 autorisé
**Scale/Scope** : ~6 endpoints REST (3 publics, 3 admin), 2 tables, 1 enum, 1 page publique + suivi perso, 2 pages admin, 1 widget public sur fiche salle, 1 composable public + 1 composable admin.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Conformité | Vérification |
|----------|-----------|--------------|
| I. Français d'abord | ✅ | Tables `proposition_salle`, `salle_administrateur` ; colonnes en snake_case français ; enum `statut_proposition_salle` (en_attente / validee / rejetee / retiree) ; UI fr-FR. |
| II. Monorepo cohérent | ✅ | Frontend + backend livrés dans la même PR. Types TS ↔ structs Rust ↔ schéma SQL alignés. |
| III. SQL source de vérité | ✅ | Modification SQL d'abord (`schemas/08b_afrolang.sql`), puis `models/`, puis `handlers/`, puis composables, puis pages. Tables placées dans schéma existant `afrolang`, pas de nouveau bounded-context. |
| IV. Sécurité par défaut | ✅ | JWT existant, requêtes paramétrées sqlx, validation côté backend. Endpoints publics protégés par `auth_middleware` (utilisateur connecté + état actif). Endpoints admin sous `admin_middleware` existant. Aucune élévation de privilège : la nomination admin de salle n'octroie aucun pouvoir effectif (FR-019). |
| V. Simplicité (YAGNI) | ✅ | Pas d'abstraction nouvelle : pattern `ApiResponse<T>` + `COLONNES` const + handlers plats déjà en place. Pas de couche service additionnelle. La table `salle_administrateur` est un simple lien N-N avec champs d'audit. Aucune capacité effective implémentée → aucun code spéculatif. |
| VI. Tailwind v4 (daisyUI back-office uniquement) | ✅ | Page publique de proposition (`pages/afrolang/proposer.vue`) et widget « Administrateurs » sur fiche salle publique : Tailwind v4 pur. Pages admin (`pages/admin/afrolang/propositions/...` et onglet sur `pages/admin/salles/[id]`) : daisyUI v5. |
| VII. Audit & traçabilité | ✅ | `audit::log_action` sur 6 mutations : `CREATE proposition_salle`, `UPDATE proposition_salle` (retrait, validation, rejet), `CREATE salle_administrateur`, `UPDATE salle_administrateur` (révocation, suspension auto). Capture utilisateur + IP + UA + before/after JSONB. |

**Résultat** : 0 violation. Pas de section *Complexity Tracking* à remplir.

## Project Structure

### Documentation (this feature)

```text
specs/001-admin-salles-publiques/
├── plan.md              # Ce fichier
├── research.md          # Phase 0 : décisions techniques
├── data-model.md        # Phase 1 : entités SQL + transitions
├── quickstart.md        # Phase 1 : scénarios manuels de validation
├── contracts/           # Phase 1 : contrats REST
│   ├── public.md
│   └── admin.md
├── checklists/
│   └── requirements.md  # Quality checklist (déjà créée par /speckit.specify)
└── tasks.md             # Phase 2 : généré par /speckit.tasks (NON créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 08b_afrolang.sql              # Étendu : enum + 2 tables + indexes
├── src/
│   ├── models/
│   │   └── afrolang.rs               # Étendu : structs Proposition*, SalleAdministrateur*
│   ├── handlers/
│   │   ├── afrolang.rs               # Étendu : 3 endpoints publics propositions
│   │   └── admin/
│   │       └── propositions_salle.rs # NOUVEAU : 4 endpoints admin (lister, valider, rejeter, détail)
│   │       # salle_administrateur géré directement dans admin/salle_publique.rs (déjà existant)
│   └── routes.rs                     # Étendu : 7 nouvelles routes

uafricas_frontend/
├── app/
│   ├── composables/
│   │   ├── useAfrolang.ts            # Étendu : proposerSalle, listerMesPropositions, retirerProposition
│   │   ├── useAdminAfrolangSalles.ts # Étendu : nommerAdministrateur, revoquerAdministrateur, listerAdministrateurs
│   │   └── useAdminPropositionsSalle.ts # NOUVEAU : lister/obtenir/valider/rejeter
│   ├── components/
│   │   ├── afrolang/
│   │   │   ├── PropositionSalleForm.vue       # NOUVEAU (Tailwind v4 pur)
│   │   │   ├── PropositionSalleCard.vue       # NOUVEAU (suivi perso)
│   │   │   └── SalleAdministrateursWidget.vue # NOUVEAU (badge sur fiche publique)
│   │   └── admin/afrolang/
│   │       ├── PropositionRow.vue             # NOUVEAU (daisyUI)
│   │       ├── PropositionDetail.vue          # NOUVEAU (daisyUI, valider/rejeter)
│   │       └── SalleAdministrateursPanel.vue  # NOUVEAU (daisyUI, onglet salle)
│   └── pages/
│       ├── afrolang/
│       │   └── proposer.vue                   # NOUVEAU (formulaire + listing perso)
│       └── admin/afrolang/
│           ├── propositions/
│           │   ├── index.vue                  # NOUVEAU (liste filtrable)
│           │   └── [id].vue                   # NOUVEAU (détail + décision)
│           └── salles/[id].vue                # ÉTENDU : nouvel onglet « Administrateurs »
```

**Structure Decision** : Web application monorepo conforme Principe II. Aucune nouvelle racine. Toutes les modifications restent dans les arborescences existantes `uafricas_backend/src/` et `uafricas_frontend/app/`. Le schéma SQL est étendu in-place dans `08b_afrolang.sql` (pas de nouveau fichier `xxx.sql`) puisqu'il s'agit d'extensions cohérentes du même bounded-context.

## Complexity Tracking

> Aucune violation à justifier : section omise volontairement.
