# Implementation Plan: Recadrage de l'engagement, 3 sources de points, 4 statuts, cadeaux virtuels

**Branch**: `008-recadrage-engagement-cadeaux` | **Date**: 2026-08-08 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/008-recadrage-engagement-cadeaux/spec.md`

## Summary

Recadrer le moteur d'engagement livré (jamais mis en service) sur trois sources canoniques, **j'aime reçus**, **partages reçus**, **cadeaux virtuels reçus**, et quatre statuts (Membre Africans / Premium / Gold / Platinum), puis livrer le module de **cadeaux payants** avec paiement simulé et répartition 90 / 10.

L'approche tient en trois mouvements :

1. **Une migration de recadrage** (`35f`) : désactivation des 8 règles écartées et des paliers de popularité, création des 3 règles canoniques et de la catégorie « Cadeaux », refonte de la grille des niveaux. Aucun code applicatif n'est supprimé, le barème étant en base, le recadrage est une opération de **données**, pas de code.
2. **Un remplacement de branchement** : les 4 appels existants à `evaluer_popularite` et les 6 handlers de partage basculent sur deux nouvelles fonctions du service (`crediter_jaime`, `crediter_partage`), et 5 familles de j'aime jusqu'ici non instrumentées sont branchées.
3. **Un module neuf `cadeaux`** (migration `35g` + `models/handlers` backend + écrans front) qui réutilise intégralement `services::engagement::appliquer` pour le crédit : le cadeau n'invente aucun mécanisme de points, il fournit un `montant_override` et une clé d'idempotence.

Le point de conception structurant est **la substituabilité du paiement** (FR-020, SC-012) : un module `services::paiement` isole l'unique fonction que CinetPay remplacera, tandis que la transaction, la répartition et le crédit sont déjà définitifs.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) · TypeScript 5 / Vue 3 SSR / Nuxt 4 (frontend)

**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde, log · Pinia, Tailwind CSS v4, daisyUI v5 (admin seulement), FontAwesome. **Aucune dépendance nouvelle** : le paiement est simulé en interne, aucun SDK n'est intégré à cette itération.

**Storage**: PostgreSQL 16, schéma `engagement` (existant). Deux migrations : `35f_engagement_recadrage.sql`, `35g_engagement_cadeaux.sql`. Aucun upload de fichier (les cadeaux sont représentés par une icône FontAwesome + une couleur, comme les badges et niveaux existants).

**Testing**: Aucun harnais automatisé configuré sur le projet. La validation passe par `quickstart.md` (scénarios manuels reproductibles) et le contrôle des invariants SQL directement en base.

**Target Platform**: Serveur Linux (Docker, VPS `www.africans-world.org`), navigateurs modernes desktop et mobile.

**Project Type**: Application web monorepo (backend Actix-Web + frontend Nuxt SSR).

**Performance Goals**: Crédit de points visible en moins de 5 s après l'action (SC-004), atteint par écriture synchrone dans la transaction du mouvement, sans tâche de fond. Le décompte des cadeaux d'un contenu doit tenir en une requête par page, comme `media_social::compteurs_pour`.

**Constraints**:
- Attribution **non bloquante** (FR-034) : le service d'engagement journalise les erreurs sans les propager, contrat déjà tenu par `services::engagement`.
- **Idempotence structurelle** : tout crédit passe par une `cle_idempotence` `UNIQUE`, aucun verrou applicatif, aucune vérification en lecture-puis-écriture.
- **Aucune valeur monétaire ni bénéficiaire venant du client** : prix, points, taux et bénéficiaire sont résolus côté serveur.
- **Substituabilité du paiement** : un seul point de code change à l'arrivée de CinetPay.
- Pas de feature flag (Principe V) : ce qui doit pouvoir être coupé l'est par le **barème paramétrable** ou le champ `actif` du catalogue.

**Scale/Scope**: 2 migrations SQL · 3 nouveaux modules backend (+ 1 service de paiement) · ~14 routes nouvelles · 14 branchements de crédit (7 handlers de j'aime couvrant **13 valeurs de `type_objet`**, 6 handlers de partage interne, 1 traçage externe mutualisé) · ~8 composants front et 2 écrans d'administration.

## Constitution Check

*GATE: vérifié avant Phase 0, re-vérifié après Phase 1.*

| Principe | Évaluation | Vérification |
|----------|-----------|--------------|
| **I. Français d'abord** | ✅ Conforme | Tables `cadeau`, `transaction_cadeau`, `cagnotte`, `parametre_monetisation` ; colonnes `prix`, `mode`, `part_beneficiaire`, `part_plateforme`, `etat` ; composants `OffrirCadeauModal`, `MaCagnotte`. Un seul anglicisme toléré : aucun. |
| **II. Monorepo cohérent** | ✅ Conforme | Chaque entité livrée en une passe SQL → struct `FromRow` → DTO → interface TS. Les DTO de transaction exposent les montants en unité entière, jamais en flottant, pour que TS et Rust s'accordent. |
| **III. SQL source de vérité** | ✅ Conforme | Les migrations `35f`/`35g` précèdent tout code. Conventions respectées : UUID v4, TIMESTAMPTZ, snake_case français, enums PostgreSQL (`mode_cadeau`, `etat_paiement`), `deleted_at` **non retenu** sur les transactions (voir dérogation ci-dessous). |
| **IV. Sécurité par défaut** | ⚠️ Conforme avec risque assumé et documenté | Bénéficiaire, prix, points et taux résolus serveur ; offreur issu du JWT ; auto-cadeau refusé ; back-office sous `engagement.gerer`. **Le paiement simulé est par nature un moyen gratuit d'obtenir des points** : le risque est accepté par décision produit et neutralisé par la purge de fin de phase (FR-020b) et l'absence d'avantage algorithmique (FR-038). |
| **V. Simplicité (YAGNI)** | ✅ Conforme | Pas de trait `PrestatairePaiement` ni de couche d'abstraction : `services::paiement` expose deux fonctions concrètes. Pas de feature flag : le catalogue et le barème portent déjà `actif`. Les 8 règles écartées ne sont pas retirées du code : elles sont désactivées en base. |
| **VI. Tailwind v4 / daisyUI admin** | ✅ Conforme | Écrans membre (`/mon-compte/engagement`, modales sur pages publiques) en Tailwind v4 pur ; `/admin/engagement/{cadeaux,transactions}` en daisyUI, comme les 5 écrans d'engagement déjà livrés. |
| **VII. Audit & traçabilité** | ✅ Conforme | `audit::log_action` sur : CRUD catalogue, modification du taux, purge de fin de phase. Les transactions de cadeaux ont leur **propre** journal métier (`transaction_cadeau`), l'audit ne doublonne pas la comptabilité. |

**Dérogation documentée : pas de `deleted_at` sur `transaction_cadeau`** : une écriture comptable ne se supprime pas, même en douceur. L'état de la transaction (`etat`) porte tout le cycle de vie ; une transaction annulée reste lisible. Le principe III impose la soft deletion aux entités métier, pas au journal financier, même raisonnement que `mouvement_points`, déjà immuable et sans `deleted_at`.

**Verdict** : aucune violation bloquante. Aucune entrée de Complexity Tracking nécessaire.

## Project Structure

### Documentation (this feature)

```text
specs/008-recadrage-engagement-cadeaux/
├── plan.md              # Ce fichier
├── research.md          # Phase 0 : décisions techniques et alternatives écartées
├── data-model.md        # Phase 1 : entités, colonnes, invariants
├── quickstart.md        # Phase 1 : scénarios de validation exécutables
├── contracts/           # Phase 1 : contrats d'API
│   ├── api-cadeaux-membre.md
│   ├── api-cadeaux-admin.md
│   └── api-engagement-recadre.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 : produit par /speckit-tasks, PAS par ce plan
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   ├── 35f_engagement_recadrage.sql        # NOUVEAU, règles, catégories, niveaux, paliers
│   └── 35g_engagement_cadeaux.sql          # NOUVEAU, catalogue, transactions, cagnotte, paramètres
├── src/
│   ├── models/
│   │   └── engagement_cadeau.rs            # NOUVEAU, FromRow + DTO + payloads
│   ├── handlers/
│   │   ├── engagement_cadeau.rs            # NOUVEAU, catalogue public, envoi, confirmation, cagnotte
│   │   ├── engagement.rs                   # MODIFIÉ, mes-cadeaux, retrait du bonus 5 réseaux
│   │   ├── codimoi.rs                      # MODIFIÉ, crediter_jaime
│   │   ├── gouvernance.rs                  # MODIFIÉ, crediter_jaime + partage de contribution
│   │   ├── bibliotheques_humaines.rs       # MODIFIÉ, crediter_jaime
│   │   ├── media_social.rs                 # MODIFIÉ, crediter_jaime (bénéficiaire = propriétaire) + partage
│   │   ├── vidafrica_contribution.rs       # MODIFIÉ, crediter_jaime + partage (NOUVEAU branchement)
│   │   ├── element_social.rs               # MODIFIÉ, crediter_jaime + partage (NOUVEAU branchement)
│   │   ├── fiche_pays_social.rs            # MODIFIÉ, crediter_jaime + partage (NOUVEAU branchement)
│   │   ├── profil_social.rs                # MODIFIÉ, partage de profil (NOUVEAU branchement)
│   │   └── admin/
│   │       └── engagement_cadeau.rs        # NOUVEAU, CRUD catalogue, journal, paramètres, purge
│   ├── services/
│   │   ├── engagement.rs                   # MODIFIÉ, crediter_jaime / crediter_partage / crediter_cadeau
│   │   │                                   #            resoudre_beneficiaire ; evaluer_popularite retirée
│   │   └── paiement.rs                     # NOUVEAU, simulateur, unique point de bascule CinetPay
│   └── routes.rs                           # MODIFIÉ, ~14 routes
└── ...

uafricas_frontend/app/
├── composables/
│   ├── useCadeaux.ts                       # NOUVEAU, catalogue, envoi, confirmation, cadeaux d'un contenu
│   ├── useAdminCadeaux.ts                  # NOUVEAU, catalogue admin, journal, paramètres, purge
│   ├── useEngagement.ts                    # MODIFIÉ, cagnotte, cadeaux reçus
│   └── usePartageExterne.ts                # MODIFIÉ, retour « bonus 5 réseaux » supprimé
├── components/engagement/
│   ├── OffrirCadeauModal.vue               # NOUVEAU, catalogue + choix du mode + paiement simulé
│   ├── OffrirCadeauBouton.vue              # NOUVEAU, point d'entrée réutilisable (7 familles + profil)
│   ├── CadeauxRecus.vue                    # NOUVEAU, cadeaux affichés sur un contenu / profil
│   ├── MaCagnotte.vue                      # NOUVEAU, cumul + mention « versement indisponible »
│   ├── BandeauPaiementSimule.vue           # NOUVEAU, avertissement phase de test
│   ├── BadgeStatut.vue                     # MODIFIÉ, 4 statuts
│   └── {ResumeEngagement,VentilationCategories,HistoriquePoints}.vue  # MODIFIÉS, libellés & cagnotte
├── pages/
│   ├── mon-compte/engagement.vue           # MODIFIÉ, sections Cadeaux et Cagnotte
│   └── admin/engagement/
│       ├── cadeaux.vue                     # NOUVEAU, CRUD catalogue + taux de commission
│       └── transactions.vue                # NOUVEAU, journal filtrable + totaux + purge
└── components/{media,opportunite-afrique,profil,vidafrica,universite/gouvernance}/*PartagerModal.vue
                                            # MODIFIÉS : message de récompense recadré
```

**Structure Decision**: structure monorepo existante, sans nouveau répertoire de premier niveau. Le domaine « cadeaux » est rattaché au schéma PostgreSQL `engagement` déjà en place plutôt qu'à un schéma neuf : la transaction n'a de sens que par les points qu'elle produit, et un schéma `paiement` séparé imposerait des jointures inter-schémas pour chaque lecture de cagnotte. Côté backend, le découpage suit la convention du projet, un fichier `models/` et un fichier `handlers/` par domaine, l'administration sous `handlers/admin/`.

## Phase 0 : Research

Voir [research.md](./research.md). Onze décisions techniques y sont tranchées, dont les quatre structurantes :

- **R1** : le recadrage du barème est une **migration de données**, pas une suppression de code.
- **R3** : `evaluer_popularite` est **remplacée** par `crediter_jaime` ; la clé d'idempotence porte le membre qui aime, ce qui rend le « retrait puis remise » naturellement inoffensif.
- **R5** : le crédit de partage utilise **une clé unique commune** aux canaux internes et externes, ce qui réalise FR-013 sans aucun comptage.
- **R7** : `services::paiement` expose deux fonctions concrètes ; le basculement CinetPay touche **un seul fichier**.

Une contradiction interne de la spécification y est également résolue (**R2**, réactions du fact-check) et remontée à l'utilisateur.

## Phase 1 : Design & Contracts

- [data-model.md](./data-model.md) : 4 entités nouvelles, 4 entités modifiées, invariants SQL et transitions d'état.
- [contracts/api-cadeaux-membre.md](./contracts/api-cadeaux-membre.md), 7 routes membre/public.
- [contracts/api-cadeaux-admin.md](./contracts/api-cadeaux-admin.md), 7 routes d'administration.
- [contracts/api-engagement-recadre.md](./contracts/api-engagement-recadre.md), impacts sur les routes d'engagement existantes.
- [quickstart.md](./quickstart.md) : 9 scénarios de validation couvrant les 13 critères de succès.

### Re-vérification de la Constitution après conception

| Principe | Après conception | Note |
|----------|------------------|------|
| I. Français | ✅ | Contrats et modèle de données intégralement en français. |
| II. Monorepo | ✅ | Chaque champ des contrats a son pendant SQL et TS ; les montants circulent en entier. |
| III. SQL source de vérité | ✅ | `data-model.md` porte les CHECK et index uniques qui rendent les invariants **structurels** (répartition exacte, auto-cadeau impossible, un seul propriétaire crédité). |
| IV. Sécurité | ✅ (risque phase de test documenté) | Aucun champ monétaire ni bénéficiaire n'est accepté du client dans les contrats ; la confirmation de paiement simulé exige la propriété de la transaction. |
| V. Simplicité | ✅ | 3 fonctions publiques ajoutées au service d'engagement, 2 au service de paiement. Aucun pattern introduit. |
| VI. Tailwind/daisyUI | ✅ | Séparation public / admin respectée dans l'arborescence ci-dessus. |
| VII. Audit | ✅ | Mutations admin instrumentées ; la purge de fin de phase produit une entrée d'audit avec le décompte des lignes touchées. |

**Verdict post-conception** : conforme, Complexity Tracking non requis.

## Complexity Tracking

> Aucune violation constitutionnelle à justifier. Section conservée vide intentionnellement.
