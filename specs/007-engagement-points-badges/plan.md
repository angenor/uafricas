# Implementation Plan: Récompenses par points — barème 100 % paramétrable & espace « Mon engagement »

**Branch**: `007-engagement-points-badges` | **Date**: 2026-07-29 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/007-engagement-points-badges/spec.md`

## Summary

Le moteur d'engagement de la phase 1 est en production : `engagement.compte`, `mouvement_points` (append-only, `cle_idempotence UNIQUE`), `regle_points`, `palier_popularite`, `niveau`, `mise_en_avant`, le service non-bloquant `services/engagement.rs` (`attribuer` / `retirer` / `evaluer_popularite` / `ajuster`), 13 routes admin, 3 routes membre, 2 écrans back-office et l'encart `EngagementMesPointsPanel`.

Cette feature **complète ce socle sans le réécrire**, en trois mouvements :

1. **Rendre le barème réellement paramétrable** — aujourd'hui `modifier_regle` est la seule mutation possible sur les règles : pas de création, pas de catégorie, paliers globaux uniquement, niveaux non créables. On ajoute `engagement.categorie_points`, le CRUD des règles / catégories / niveaux, un **catalogue des actions instrumentées par le code** (`GET /actions-disponibles`) pour que l'administrateur ne crée jamais une règle orpheline à son insu, et un `seuil_declencheur` paramétrable (règle « 5 réseaux distincts »).
2. **Exposer côté membre** — page dédiée `/mon-compte/engagement` (Tailwind v4 pur, Principe VI) : résumé + progression, **ventilation par catégorie** (nouvelle colonne `mouvement_points.categorie_id`, figée au moment du mouvement), **badges obtenus / à débloquer** (nouvelles tables `badge` / `badge_obtenu`), historique paginé filtrable. L'onglet « Mes points » de `/mon-compte/profil` devient le résumé et la porte d'entrée.
3. **Élargir la couverture** — 4 branchements médias (proposition validée, mise à la une, animation acceptée, popularité télé/radio) + le **log de partage externe** qui manquait totalement (`engagement.partage_externe`), les tables `partage_*` existantes n'étant que des reposts internes.

Approche technique : **aucune tâche de fond, aucun nouveau service transverse**. Tout reste dans le pattern éprouvé — résolution paresseuse, attribution post-commit non-bloquante, idempotence structurelle par clé unique, notifications dans `arbre_genealogique.notifications` (table générique de fait de la plateforme), audit via `audit::log_action`.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) · TypeScript 5 / Vue 3 SSR / Nuxt 4 (frontend)

**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde — **aucune dépendance nouvelle** · Pinia, Tailwind CSS v4, daisyUI v5 (back-office uniquement), FontAwesome

**Storage**: PostgreSQL 16, schéma **`engagement`** (existant, étendu par 3 migrations : `35c`, `35d`, `35e`) ; lecture seule sur `media_content` (résolution des auteurs/détenteurs) et `iam.utilisateur`

**Testing**: aucun harnais de test configuré (contrainte constitutionnelle assumée) → validation manuelle scénarisée dans [quickstart.md](./quickstart.md), chaque scénario tracé à un `SC-xxx`

**Target Platform**: serveur Linux (Docker ; backend port 8082 en dev, 8080 en prod), navigateurs modernes desktop + mobile

**Project Type**: web — monorepo `uafricas_backend/` (Actix-Web) + `uafricas_frontend/` (Nuxt 4 SSR)

**Performance Goals**: espace « Mon engagement » complet en **≤ 4 requêtes SQL** (résumé, ventilation, badges, page de journal) ; ventilation calculée par agrégation sur le journal du membre (index `idx_mouvement_utilisateur` déjà présent) ; évaluation des badges en **1 requête par badge actif non encore obtenu** (≈ 10–20 badges attendus), déclenchée post-commit et jamais dans le chemin critique d'une action métier

**Constraints**: attribution **non-bloquante** (SC-007 : 0 % d'échec métier imputable) ; **idempotence structurelle** (jamais de vérification applicative concurrente) ; plancher de solde à 0 ; **pas de tâche planifiée** (le reset mensuel reste paresseux, l'évaluation des badges reste événementielle) ; migrations **idempotentes** (`IF NOT EXISTS`, `ON CONFLICT DO NOTHING`) car appliquées à la main en production

**Scale/Scope**: 3 migrations SQL · +4 tables (`categorie_points`, `badge`, `badge_obtenu`, `partage_externe`) · +4 colonnes sur tables existantes · **+20 endpoints** (15 admin, 5 membre) et 6 endpoints modifiés · 4 branchements médias + 1 branchement partage · 1 page membre + 5 composants · 3 pages back-office dont 1 refonte · ~15 000 membres attendus à terme

## Constitution Check

*GATE : vérifié avant Phase 0, re-vérifié après Phase 1.*

| Principe | Verdict | Application dans cette feature |
|---|---|---|
| **I. Français d'abord** | ✅ PASS | Tables/colonnes `categorie_points`, `badge_obtenu`, `partage_externe`, `seuil_declencheur`, `type_condition` ; fonctions `evaluer_badges`, `enregistrer_partage_externe`, `recalculer_niveaux` ; composants `EngagementVentilationCategories`, `EngagementMesBadges` ; libellés UI et messages d'erreur en français accentué. Noms de **fichiers** sans accent (`35c_engagement_categories_bareme.sql`). |
| **II. Monorepo cohérent** | ✅ PASS | Chaque ajout de champ est livré cross-stack dans le même lot : SQL → struct `FromRow`/DTO Rust → interface TS du composable. Les DTO sont énumérés dans [contracts/](./contracts/) pour que les trois couches ne dérivent pas. |
| **III. SQL source de vérité** | ✅ PASS | Les 3 migrations sont écrites **en premier** ; `data-model.md` en est le miroir. UUID v4, TIMESTAMPTZ, snake_case français, enums PostgreSQL (`engagement.type_condition_badge`, `engagement.reseau_social`). **Écart assumé** : pas de soft delete dans `engagement` (voir Complexity Tracking). |
| **IV. Sécurité par défaut** | ✅ PASS | Routes admin derrière `AdminUtilisateur` + `verifier_permission!(admin, "engagement", "gerer")` (permission `engagement.gerer` déjà seedée par `35_engagement.sql`) ; routes membre derrière le JWT via `extraire_utilisateur_id`, **jamais** `AdminUtilisateur`. Requêtes 100 % paramétrées ; les rares interpolations (nom de table média) viennent d'un `match` sur littéraux fixes, comme `table_pour_type`. Le journal d'un membre n'est lisible que par lui-même ou un administrateur (FR-014). Le traçage d'un partage externe ne fait jamais confiance au client sur l'identité (`utilisateur_id` pris du JWT). |
| **V. Simplicité (YAGNI)** | ⚠️ PASS avec garde-fous | Deux tentations écartées explicitement : (a) **moteur d'expressions** pour les conditions de badge → **enum fermé de 5 types** + 3 paramètres (R6) ; (b) **soldes persistés par catégorie** → **agrégation à la lecture** sur le journal (R2). Aucun pattern Repository/Factory ; on étend les 5 fichiers existants du domaine plutôt que d'en créer de nouveaux. |
| **VI. Tailwind v4 (daisyUI back-office seulement)** | ✅ PASS | `/mon-compte/engagement` et les composants `components/engagement/` : **Tailwind v4 pur**, palette `custom-chocolat` / `custom-green`, Oswald + Open Sans. `pages/admin/engagement/*` : daisyUI autorisé (comme `regles.vue` aujourd'hui). Résidus v3 rencontrés dans un fichier touché → migrés. |
| **VII. Audit & traçabilité** | ✅ PASS | `audit::log_action` sur **toutes** les mutations admin nouvelles : CRUD règle/catégorie/niveau/badge, attribution et retrait manuels de badge (l'ajustement de points l'est déjà). Le journal `mouvement_points` reste la piste métier, l'audit la piste administrative. |

**Verdict initial** : PASS — aucune violation à justifier, un écart de convention documenté.

**Re-vérification post-Phase 1** : PASS — le design de Phase 1 n'introduit ni dépendance, ni service transverse, ni tâche de fond, ni abstraction supplémentaire ; les 12 décisions de `research.md` vont toutes dans le sens du Principe V.

## Project Structure

### Documentation (this feature)

```text
specs/007-engagement-points-badges/
├── plan.md              # Ce fichier
├── research.md          # Phase 0 — 12 décisions techniques
├── data-model.md        # Phase 1 — 3 migrations, 4 tables neuves, 4 colonnes
├── quickstart.md        # Phase 1 — scénarios de validation manuelle
├── contracts/
│   ├── api-membre.md    # 8 routes membre (5 neuves, 1 modifiée, 2 inchangées)
│   └── api-admin.md     # 28 routes back-office (15 neuves, 5 modifiées, 8 inchangées)
├── checklists/
│   └── requirements.md  # Qualité de la spec (16/16)
└── tasks.md             # Phase 2 — produit par /speckit-tasks (PAS par /speckit-plan)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                                   # + 3 lignes \ir après 35b
│   └── schemas/
│       ├── 35c_engagement_categories_bareme.sql     # NEW — catégories, colonnes barème, seeds
│       ├── 35d_engagement_badges.sql                # NEW — badge, badge_obtenu, rétro-évaluation
│       └── 35e_engagement_partage_externe.sql       # NEW — enum réseau + log de partage
└── src/
    ├── services/engagement.rs                       # MOD — catégorie du mouvement, evaluer_badges,
    │                                                #       paliers par famille, partage externe,
    │                                                #       recalculer_niveaux
    ├── models/
    │   ├── engagement.rs                            # MOD — DTO ventilation, badges, partage
    │   ├── notification.rs                          # MOD — pub mod engagement { NIVEAU_ATTEINT, BADGE_DEBLOQUE }
    │   └── admin/engagement.rs                      # MOD — DTO CRUD règle/catégorie/niveau/badge
    ├── handlers/
    │   ├── engagement.rs                            # MOD — mes-categories, mes-badges, partage externe
    │   ├── media_social.rs                          # MOD — popularité médias (hook)
    │   ├── media_proposition.rs                     # MOD — animation acceptée (hook, post-commit)
    │   └── admin/
    │       ├── engagement.rs                        # MOD — CRUD complet + actions-disponibles + badges
    │       ├── media_proposition.rs                 # MOD — proposition validée (hook, post-commit)
    │       └── radio_tele.rs                        # MOD — mise à la une (hook, 4 tables)
    └── routes.rs                                    # MOD — +20 routes

uafricas_frontend/app/
├── pages/
│   ├── mon-compte/
│   │   ├── engagement.vue                           # NEW — espace membre (Tailwind pur)
│   │   └── profil.vue                               # MOD — onglet « Mes points » → résumé + lien
│   ├── profil/[id].vue                              # MOD — badges obtenus sur le profil public
│   └── admin/engagement/
│       ├── regles.vue                               # MOD — création/désactivation + catégorie + seuil
│       ├── categories.vue                           # NEW — CRUD catégories
│       ├── niveaux.vue                              # NEW — CRUD niveaux (extrait de regles.vue)
│       └── badges.vue                               # NEW — CRUD badges + attribution manuelle
├── components/
│   ├── engagement/
│   │   ├── MesPointsPanel.vue                       # MOD — résumé + lien vers l'espace
│   │   ├── BadgeStatut.vue                          # inchangé
│   │   ├── ResumeEngagement.vue                     # NEW — soldes, niveau, progression
│   │   ├── VentilationCategories.vue                # NEW
│   │   ├── MesBadges.vue                            # NEW — obtenus + à débloquer
│   │   ├── BadgeSucces.vue                          # NEW — vignette unitaire réutilisable
│   │   └── HistoriquePoints.vue                     # NEW — filtres + pagination
│   ├── admin/AdminSidebar.vue                       # MOD — 3 entrées sous « Engagement »
│   └── (6 modales de partage existantes)            # MOD — traçage du partage externe
└── composables/
    ├── useEngagement.ts                             # MOD — catégories, badges, journal filtré
    ├── useAdminEngagement.ts                        # MOD — CRUD règle/catégorie/niveau/badge
    └── usePartageExterne.ts                         # NEW — traçage best-effort d'un partage
```

Les 6 modales de partage à instrumenter (elles portent déjà les URL `sharer.php` / `wa.me` / `intent/tweet` / `linkedin`) :
`media/MediaPartagerModal.vue`, `opportunite-afrique/PartagerElementModal.vue`, `opportunite-afrique/PartagerFicheModal.vue`, `evenements/EvenementPartage.vue`, `universite/gouvernance/PartagePublication.vue`, `retrouve-amis/BoutonsPartage.vue`.

**Structure Decision** : structure web du monorepo existant, **aucun nouveau module de premier niveau**. Le domaine `engagement` possède déjà son schéma SQL, son service, ses handlers public/admin et ses modèles : les trois axes de la feature s'y greffent. Les 6 composants de partage sont modifiés en place, sans composant de partage supplémentaire.

## Complexity Tracking

> Aucune violation de la constitution. Les deux points ci-dessous sont des **écarts de convention documentés**.

| Écart | Pourquoi nécessaire | Alternative plus simple rejetée parce que |
|---|---|---|
| Pas de `deleted_at` dans `engagement.*` (conventions BDD du Principe III) | `mouvement_points` est un journal append-only : l'idempotence repose sur `cle_idempotence UNIQUE`, un soft delete rendrait le rejeu à nouveau attribuable. Les référentiels de barème (`regle_points`, `palier_popularite`, `badge`) se **désactivent** (`actif = FALSE`) pour que l'historique reste lisible (FR-002). Cohérent avec `35_engagement.sql` déjà livré. | Ajouter `deleted_at` partout obligerait chaque lecture du barème et chaque agrégation de journal à filtrer, sans bénéfice : rien ne doit jamais être supprimé ici. |
| `mouvement_points.categorie_id` dénormalisée (copie de `regle_points.categorie_id`) | La spec exige que la ventilation reflète la catégorie **au moment du mouvement** : une re-catégorisation ne doit pas réécrire le passé. | Joindre `regle_points` à la lecture est plus normalisé mais rendrait toute re-catégorisation **rétroactive** — comportement interdit par la spec. |

**Budget de complexité respecté** : 0 nouvelle dépendance, 0 nouveau service transverse, 0 tâche planifiée, 0 pattern d'indirection. Les conditions de badge sont un enum fermé de 5 valeurs, pas un langage.
