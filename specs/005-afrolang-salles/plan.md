# Implementation Plan: Afrolang — Ajustements salles publiques et privées

**Branch**: `005-afrolang-salles` | **Date**: 2026-04-14 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/005-afrolang-salles/spec.md`

## Summary

Étendre la fonctionnalité Afrolang existante (schema `afrolang` avec `salle`, `salle_privee`, `session`, `session_participant`, `tableau_blanc`) pour :

1. **Rattacher les salles publiques au référentiel ethnique unique** (`country_profile.groupe_ethnique`) et afficher l'annuaire des groupes comme porte d'entrée.
2. **Proposer la création d'une salle publique** pour un groupe ethnique absent, avec validation par l'administration (workflow en_attente → approuvée / refusée + notification).
3. **Distinguer deux niveaux de modération** : modérateurs Afrolang attitrés (many-to-many via table d'affectation) et modérateur de session (attribution dynamique au premier arrivé, transférable, repris par un modérateur Afrolang entrant).
4. **Enrichir la salle privée** avec motif (apprentissage_enfants, reseautage_adulte, echanges_groupe), déclaration d'âge adulte horodatée, mode de visibilité (fermee / visible), limite de participants, état actif/archivée, et contrainte d'unicité « 1 salle privée active par membre par salle publique ».
5. **Gérer les adhésions et invitations** : demandes d'adhésion (depuis salle visible), invitations directes (depuis salle fermée), acceptation/refus, refus automatique « groupe complet ».
6. **Rendre fonctionnel le tableau blanc collaboratif** (déjà persisté dans `tableau_blanc`, à compléter par la synchronisation temps réel).
7. **Ajouter une rubrique Ressources** acceptant fichiers internes (publication directe) et liens externes (modération préalable).
8. **Ajouter une messagerie instantanée écrite** par session, avec auteur et horodatage.

L'approche technique : nouvelles tables dans le schema `afrolang` et une FK vers `country_profile.groupe_ethnique`, extension des handlers/models existants, nouveaux endpoints publics et admin, composables et composants frontaux additionnels, transport temps réel via le WebSocket/SFU déjà en place (LiveKit pour media + canal data pour tableau blanc et messagerie).

## Technical Context

**Language/Version**: Rust Edition 2024 (backend), TypeScript / Nuxt 4 / Vue 3 SSR (frontend)
**Primary Dependencies**: Actix-Web 4, actix-multipart, sqlx (PostgreSQL), uuid, chrono, serde, sanitize-filename, livekit-api (backend) ; Pinia, $fetch, FontAwesome, GSAP, AOS (frontend) ; tableau blanc & chat temps réel via canal data LiveKit déjà configuré
**Storage**: PostgreSQL 16 — schema `afrolang` étendu (3 nouvelles tables + ajout de colonnes sur 2 tables existantes) ; FK vers `country_profile.groupe_ethnique` existant ; stockage local `./uploads/afrolang/ressources/` pour fichiers ressources
**Testing**: Aucun framework de tests configuré dans le projet (constitution « pas de linting, testing ni CI/CD configuré ») — validation par scénarios manuels documentés dans quickstart.md
**Target Platform**: Serveur Linux (backend Actix-Web port 8080) + navigateurs modernes via Nuxt 4 SSR (port 3000), déploiement Docker sur VPS
**Project Type**: Web application monorepo (frontend Nuxt 4 + backend Rust Actix-Web 4)
**Performance Goals**: Latence perçue comme instantanée pour tableau blanc (<500 ms pour 95 % des tracés en session ≤10 participants, SC-004) ; accès à une salle publique en <10 s post-clic (SC-001) ; délai de validation médiane <72 h pour liens externes (SC-009)
**Constraints**: Conformité Constitution UAfricas v1.1.0 — français strict, SQL source de vérité, JWT access 15 min + refresh 7 j, bcrypt cost 12, audit::log_action non-bloquant sur toute mutation, Tailwind v4 pur pour pages publiques (pas de daisyUI), daisyUI v5 autorisé sur back-office admin uniquement, soft deletion par `deleted_at`, UUID v4 PK, snake_case français
**Scale/Scope**: Ordre de grandeur plateforme panafricaine — plusieurs centaines de groupes ethniques, quelques milliers de membres inscrits, sessions simultanées dans la dizaine à faible centaine ; salle privée plafonnée à 50 participants (valeur existante) ; nouvelle contrainte « 1 salle privée active par membre par salle publique »

## Constitution Check

Évaluation des 7 principes de la Constitution UAfricas v1.1.0 pour cette feature :

| Principe | Applicable | Conformité | Notes |
|----------|------------|------------|-------|
| I. Français d'Abord | Oui | ✅ Conforme | Toutes les nouvelles tables, colonnes, enums, structs, composants, routes et messages seront rédigés en français (snake_case SQL, camelCase TS, PascalCase composants) |
| II. Monorepo Cohérent | Oui | ✅ Conforme | Extensions livrées de façon coordonnée backend (sqlx + handlers) + frontend (composables + composants) + SQL dans le même set de commits |
| III. SQL Source de Vérité | Oui | ✅ Conforme | Schéma SQL modifié en premier (nouvelles tables + colonnes dans `afrolang`), puis propagé aux structs Rust `FromRow`, aux interfaces TS de `useAfrolang` et aux mocks si nécessaires. FK explicite vers `country_profile.groupe_ethnique` (référentiel unique) |
| IV. Sécurité par Défaut | Oui | ✅ Conforme | JWT requis pour créer une proposition / une salle privée / une invitation ; validation serveur de la déclaration d'âge adulte ; sanitize des uploads de ressources (fichiers) ; validation de l'URL des liens externes avant publication ; CORS déjà configuré ; pas de secrets en dur |
| V. Simplicité (YAGNI) | Oui | ✅ Conforme | Réutilisation maximale des handlers/models afrolang existants ; pas d'abstraction nouvelle (pas de Repository, pas de Factory) ; extension du composable `useAfrolang` et ajout d'un `useAdminAfrolangSalles` pour la partie admin plutôt que refonte ; pas de feature flag |
| VI. Tailwind v4 / daisyUI v5 | Oui | ✅ Conforme | Pages publiques `/afrolang/...` = Tailwind v4 pur (déjà le cas dans les composants existants) ; pages admin `/admin/afrolang/...` = daisyUI v5 autorisé |
| VII. Audit & Traçabilité | Oui | ✅ Conforme | Chaque nouvelle mutation admin (validation/refus proposition, désignation modérateur Afrolang, validation lien externe, archivage salle privée) sera instrumentée avec `audit::log_action` non-bloquant |

**Gate**: PASS — aucune violation, aucune justification de complexité requise.

## Project Structure

### Documentation (this feature)

```text
specs/005-afrolang-salles/
├── plan.md              # Ce fichier (/speckit.plan)
├── spec.md              # Spécification fonctionnelle (/speckit.specify + /speckit.clarify)
├── research.md          # Phase 0 (décisions techniques, alternatives rejetées)
├── data-model.md        # Phase 1 (entités, tables, relations, transitions)
├── quickstart.md        # Phase 1 (scénarios de validation manuelle)
├── contracts/           # Phase 1 (contrats API : publics + admin)
│   ├── api-public-afrolang.md
│   └── api-admin-afrolang.md
├── checklists/
│   └── requirements.md  # Qualité du spec (existant)
└── tasks.md             # Phase 2 (/speckit.tasks — non créé ici)
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 08b_afrolang.sql                          # [MODIFIÉ] colonnes ajoutées + 4 nouvelles tables
├── src/
│   ├── models/
│   │   ├── afrolang.rs                           # [MODIFIÉ] nouveaux structs Request/Response
│   │   └── admin/
│   │       └── session_afrolang.rs               # [MODIFIÉ] + nouveaux DTO admin (propositions, modérateurs attitrés)
│   ├── handlers/
│   │   ├── afrolang.rs                           # [MODIFIÉ] nouveaux endpoints publics (propositions, adhésions, ressources, messages, modération de session)
│   │   └── admin/
│   │       └── session_afrolang.rs               # [MODIFIÉ] + nouveaux endpoints admin (validation propositions, modérateurs attitrés, validation liens externes)
│   ├── services/
│   │   └── audit.rs                              # existant, utilisé sur toutes les mutations
│   └── routes.rs                                 # [MODIFIÉ] enregistrement des nouvelles routes
└── uploads/
    └── afrolang/
        └── ressources/                           # [NOUVEAU] dossier local des fichiers Ressources

uafricas_frontend/
├── app/
│   ├── components/
│   │   ├── afrolang/
│   │   │   ├── AnnuaireGroupesEthniques.vue      # [NOUVEAU] parcours par groupe ethnique
│   │   │   ├── ProposerSalleModal.vue            # [NOUVEAU] formulaire de proposition
│   │   │   ├── PropositionCard.vue               # [NOUVEAU] suivi des propositions
│   │   │   ├── SalleModerationPanel.vue          # [NOUVEAU] transfert de modération de session
│   │   │   ├── SalleChat.vue                     # [NOUVEAU] messagerie instantanée écrite
│   │   │   ├── SalleRessources.vue               # [NOUVEAU] rubrique Ressources
│   │   │   ├── AfrolangWhiteboard.vue            # [MODIFIÉ] synchronisation temps réel fonctionnelle
│   │   │   ├── SallePriveeCreateModal.vue        # [MODIFIÉ] motifs + déclaration adulte + notice enfants
│   │   │   ├── SallePriveeVisibilitePanel.vue    # [NOUVEAU] bascule fermée/visible + invitations
│   │   │   ├── DemandeAdhesionCard.vue           # [NOUVEAU] demandes à traiter côté créateur
│   │   │   └── InvitationBanner.vue              # [NOUVEAU] invitation reçue à accepter/refuser
│   │   └── admin/afrolang/
│   │       ├── ValidationPropositionsList.vue    # [NOUVEAU] back-office file de propositions
│   │       ├── ModerateursAttitresPanel.vue      # [NOUVEAU] désignation modérateurs Afrolang
│   │       └── LiensExternesValidation.vue       # [NOUVEAU] modération liens externes
│   ├── composables/
│   │   ├── useAfrolang.ts                        # [MODIFIÉ] nouveaux appels (propositions, modération, adhésions, ressources, messages)
│   │   └── useAdminAfrolangSalles.ts             # [NOUVEAU] appels admin dédiés
│   ├── mocks/
│   │   └── afrolang.ts                           # [MODIFIÉ] nouvelles interfaces + données mock
│   └── pages/
│       ├── afrolang/
│       │   ├── index.vue                         # [MODIFIÉ] annuaire ethnique + bouton proposer
│       │   ├── [id].vue                          # [MODIFIÉ] chat + ressources + modération visible
│       │   ├── proposer.vue                      # [NOUVEAU] formulaire et suivi propositions
│       │   ├── salle-privee/
│       │   │   ├── creer.vue                     # [MODIFIÉ] motif + adulte + visibilité
│       │   │   └── [id].vue                      # [MODIFIÉ] gestion adhésions et invitations
│       │   └── session/                          # existant
│       └── admin/
│           └── afrolang/
│               ├── propositions.vue              # [NOUVEAU] file de validation
│               ├── moderateurs.vue               # [NOUVEAU] désignation modérateurs Afrolang
│               └── liens-externes.vue            # [NOUVEAU] validation des liens externes
```

**Structure Decision**: Monorepo web (Option 2) — `uafricas_backend/` Rust Actix-Web 4 + `uafricas_frontend/` Nuxt 4, structure déjà en place. Aucun nouveau projet, aucun nouveau schema PostgreSQL. Extension du schema existant `afrolang` et liaison à `country_profile.groupe_ethnique` (source unique validée en clarification Q1).

## Complexity Tracking

Aucune violation de la Constitution détectée → tableau laissé vide.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| _(aucune)_ | — | — |
