# Implementation Plan: Afripulse — Enrichissement collaboratif des fiches pays

**Branch**: `001-afripulse-contributions` | **Date**: 2026-04-18 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-afripulse-contributions/spec.md`

## Summary

Afripulse étend la page publique `/opportunite-afrique/[id]` avec six sections enrichies à modération (sites touristiques emblématiques + privés, secteurs d'opportunités, personnalités connues, savoir avant de voyager, recommandations visiteurs avec note 1–5, galerie photos légendées) et ouvre trois flux contributifs : (a) création d'une nouvelle fiche pays depuis `/opportunite-afrique`, (b) ajout / édition / suppression d'éléments de section sur une fiche existante, (c) dépôt de photos et recommandations. Toute contribution traverse le pipeline de modération admin déjà en production (`country_profile.contribution_fiche` + enums `etat_contribution` / `type_contribution`) avant publication. Rate-limit anti-spam (20 textes/j, 10 photos/j, 5 en attente par pays), bornes strictes photos (2 Mo, 2048×2048, JPEG/PNG, 5/soumission), périmètre figé aux 54 codes ISO africains déjà énumérés côté frontend, recommandations unique par (utilisateur, pays) avec remplacement par édition. Les contributeurs validés sont listés publiquement par pays.

**Approche technique** : extension minimale et réutilisation maximale du pipeline existant — aucune nouvelle abstraction, réutilisation de `contribution_fiche`, extension par ajout de 4 nouvelles tables de données (personnalités, savoirs, recommandations, photos) et extension de `site_touristique` avec une catégorie emblématique/privé, ajout de 2 enums de section et de type-objet dans le schéma `country_profile`, un service backend unique pour rate-limit et validation photos, composants Vue publics en Tailwind v4 pur, page admin en daisyUI (réutilisation de `/admin/profils-pays/contributions.vue` étendue).

## Technical Context

**Language/Version** : Rust Edition 2024 (backend) + TypeScript 5.x / Nuxt 4 / Vue 3 SSR (frontend)
**Primary Dependencies** :
- Backend : Actix-Web 4, actix-multipart, actix-files, sqlx 0.8 (PostgreSQL), uuid, chrono, serde, sanitize-filename, jsonwebtoken (existant), `image` crate 0.25 (NOUVEAU — validation dimensions PNG/JPEG en mémoire)
- Frontend : Nuxt 4, Pinia, `$fetch`, FontAwesome (existant), GSAP/AOS (existant)
**Storage** : PostgreSQL 16 — schema `country_profile` étendu (site_touristique + 4 nouvelles tables + 2 enums). Stockage local `./uploads/opportunite-afrique/photos/` et `./uploads/opportunite-afrique/sections/` servis via actix-files
**Testing** : aucune suite automatisée configurée (principe constitutionnel — pas de CI/CD actuel). Validation manuelle via `quickstart.md` + seeds SQL. Tests critiques identifiés (rate-limit, périmètre ISO, unicité recommandation) à convertir en scénarios cURL reproductibles dans `quickstart.md`
**Target Platform** : Linux serveur (VPS prod derrière nginx + Docker) + navigateurs modernes (Chrome/Firefox/Safari/Edge récents)
**Project Type** : web — monorepo avec `uafricas_frontend/` (Nuxt 4) et `uafricas_backend/` (Rust/Actix)
**Performance Goals** :
- Affichage de la fiche pays enrichie : p95 < 2 s (latence API agrégée + rendu SSR)
- Soumission contribution texte seule : p95 < 400 ms côté API
- Soumission contribution avec photo : p95 < 3 s pour 5 photos (validation + stockage local)
- Modération côté admin : p95 < 500 ms par décision (approuver/refuser)
**Constraints** :
- Photo : 2 Mo max par fichier, 2048×2048 max, JPEG+PNG exclusivement, 5 max/soumission (validation client + serveur)
- Rate-limit : 20 contributions textuelles / 24 h glissantes / utilisateur ; 10 contributions photo / 24 h / utilisateur ; 5 contributions en statut `en_attente` simultanées sur un même pays / utilisateur
- Commentaire recommandation : 50 à 2000 caractères, note entière 1–5
- Périmètre pays : 54 codes ISO africains figés (source unique partagée frontend/backend)
**Scale/Scope** :
- 54 fiches pays maximum
- Volumétrie estimée cible : 10 000 contributions/an (≈ 200 / pays / an), dont 40 % textuelles et 60 % photos
- Stockage photos : en moyenne 5 Mo / utilisateur actif / pays → ≈ 5 Go à 1 an sur 1 000 contributeurs actifs
- 5 User Stories (2×P1, 2×P2, 1×P3), 28 FR + 1 sous-FR (FR-014a), 8 Success Criteria

## Constitution Check

*GATE: Doit passer avant Phase 0. À re-vérifier après Phase 1.*

| Principe constitutionnel | Statut | Justification |
|---|---|---|
| **I. Français d'Abord** | PASS | Tous les identifiants SQL (`personnalite_connue`, `savoir_pratique`, `recommandation_visiteur`, `photo_visiteur`), variables TypeScript, composants Vue, messages UI et commentaires sont rédigés en français. Seules exceptions : termes techniques consacrés (UUID, JWT, JPEG, PNG, ISO, CRUD) et mots-clés de langage. |
| **II. Monorepo Cohérent** | PASS | Aucune extraction : modifications internes à `uafricas_backend/` et `uafricas_frontend/`. Les contrats (types TS ↔ structs Rust `FromRow` ↔ schéma SQL) sont livrés dans la même PR. |
| **III. SQL Source de Vérité** | PASS | DDL livré d'abord dans `uafricas_backend/doc/bd/schemas/11c_country_profile_afripulse.sql` (nouveau fichier), puis propagé dans les structs Rust (`models/admin/profils_pays.rs` + `models/contribution_fiche.rs`), puis dans les types TS de `useOpportuniteAfrique.ts`. Conventions respectées : UUID v4, `deleted_at`, TIMESTAMPTZ, snake_case français, enums PostgreSQL. |
| **IV. Sécurité par Défaut** | PASS | Auth JWT existant (obligatoire pour toute contribution, `jwt::extract_user_id`). Uploads sécurisés : validation magic bytes + dimensions via crate `image` (rejet avant `sanitize-filename` + écriture disque). Rate-limit côté serveur comme défense principale (pas uniquement côté client). Requêtes paramétrées sqlx. Audit `audit::log_action` sur chaque mutation. CORS inchangé. Pas de secrets en dur. |
| **V. Simplicité (YAGNI)** | PASS | Réutilisation intégrale du pipeline `contribution_fiche` existant. Pas de nouveau pattern : extension par 4 tables supplémentaires et 2 enums. Pas de Repository/Factory. Pas de feature flag. Un nouveau handler par nouvelle entité, aligné sur le pattern des `admin/profils_pays.rs`. Un composable existant `useOpportuniteAfrique` étendu, pas dédoublé. |
| **VI. Tailwind v4 (daisyUI back-office)** | PASS | Les nouveaux composants publics (`SitesTouristiquesSection.vue`, `PersonnalitesSection.vue`, etc.) et les modals publics (`NouvelleFichePaysModal`, `UploadPhotosModal`) utilisent exclusivement des utility classes Tailwind v4. La page admin `/admin/profils-pays/contributions.vue` (déjà en daisyUI) est étendue avec composants daisyUI pour la validation enrichie (diff viewer, lightbox photos). |
| **VII. Audit & Traçabilité** | PASS | Chaque nouvelle mutation (soumission, approbation, refus, retrait après approbation, application d'une contribution à la table cible) appelle `audit::log_action(pool, user, "action", "country_profile", "<table>", id, ancien_etat, nouvel_etat, ip, ua)`. Les décisions de modération sont déjà tracées par le handler existant ; on le conserve. |

**Résultat** : tous les principes passent. Aucune section « Complexity Tracking » nécessaire.

## Project Structure

### Documentation (this feature)

```text
specs/001-afripulse-contributions/
├── plan.md              # Ce fichier (output de /speckit.plan)
├── spec.md              # Spec fonctionnelle (output de /speckit.specify + /speckit.clarify)
├── research.md          # Phase 0 — décisions techniques
├── data-model.md        # Phase 1 — DDL complet + mapping TS/Rust
├── quickstart.md        # Phase 1 — scénario de test bout-en-bout (cURL + UI)
├── contracts/           # Phase 1 — contrats HTTP par endpoint
│   ├── public.openapi.yaml
│   └── admin.openapi.yaml
├── checklists/
│   └── requirements.md  # Généré par /speckit.specify
└── tasks.md             # Généré par /speckit.tasks
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                                   # + include 11c_country_profile_afripulse.sql
│   └── schemas/
│       ├── 11_country_profile.sql                   # EXISTANT — extension : site_touristique.categorie
│       ├── 11b_country_profile_contributions.sql    # EXISTANT — extension : enum section + type_objet
│       └── 11c_country_profile_afripulse.sql        # NOUVEAU — personnalite_connue, savoir_pratique,
│                                                    #            recommandation_visiteur, photo_visiteur
├── src/
│   ├── handlers/
│   │   ├── fiches_pays.rs                           # EXISTANT — extension : lecture nouvelles sections
│   │   ├── contributions_fiche.rs                   # EXISTANT — extension : multipart photos + rate-limit
│   │   ├── afripulse_public.rs                      # NOUVEAU — POST /api/fiches-pays (création modérée),
│   │   │                                            #            GET /api/fiches-pays/moi/contributions
│   │   └── admin/
│   │       └── profils_pays.rs                      # EXISTANT — extension : diff enrichi, retrait post-approbation
│   ├── models/
│   │   ├── contribution_fiche.rs                    # EXISTANT — extension : champs action + target_id + pieces_jointes
│   │   ├── afripulse.rs                             # NOUVEAU — structs FromRow des 4 nouvelles entités
│   │   └── admin/profils_pays.rs                    # EXISTANT — extension : DTOs nouveaux enums
│   ├── services/
│   │   ├── audit.rs                                 # EXISTANT — inchangé
│   │   ├── rate_limit_afripulse.rs                  # NOUVEAU — enforcement 20/10/5
│   │   └── image_validation.rs                      # NOUVEAU — magic bytes + dimensions via `image`
│   ├── routes.rs                                    # EXISTANT — ajout des nouvelles routes
│   └── Cargo.toml                                   # + dépendance `image = "0.25"`
└── uploads/
    └── opportunite-afrique/
        ├── photos/                                  # NOUVEAU — galerie visiteurs (UUID + .jpg/.png)
        └── sections/                                # NOUVEAU — images sites / personnalités contributions

uafricas_frontend/
├── app/
│   ├── composables/
│   │   ├── useOpportuniteAfrique.ts                 # EXISTANT — extension : nouvelles sections + upload photos + création fiche + mes contributions
│   │   └── useAdminContributions.ts                 # EXISTANT — extension : diff enrichi + retrait post-approbation
│   ├── components/
│   │   ├── common/
│   │   │   └── AfripulseCountryCodesIso.ts          # NOUVEAU — constante partagée 54 codes ISO africains
│   │   └── opportunite-afrique/
│   │       ├── ContributionModal.vue                # EXISTANT — remaniement : 6 sections, action ajout/édition/suppression, upload photos
│   │       ├── ContributeursSection.vue             # EXISTANT — inchangé
│   │       ├── NouvelleFichePaysModal.vue           # NOUVEAU — création de fiche pays (US3)
│   │       ├── SitesTouristiquesSection.vue         # NOUVEAU — emblématiques + privés
│   │       ├── SecteursOpportunitesSection.vue      # NOUVEAU
│   │       ├── PersonnalitesSection.vue             # NOUVEAU
│   │       ├── SavoirAvantVoyagerSection.vue        # NOUVEAU
│   │       ├── RecommandationsSection.vue           # NOUVEAU — note 1–5 + commentaire 50–2000 car.
│   │       └── GaleriePhotosSection.vue             # NOUVEAU — lightbox + légende + auteur
│   └── pages/
│       ├── opportunite-afrique/
│       │   ├── index.vue                            # EXISTANT — ajout bouton « Proposer nouvelle fiche pays »
│       │   └── [id].vue                             # EXISTANT — intégration des 6 nouvelles sections
│       ├── mon-compte/
│       │   └── contributions.vue                    # NOUVEAU — FR-026 : suivi contributions utilisateur
│       └── admin/
│           └── profils-pays/
│               └── contributions.vue                # EXISTANT — extension : filtre section, diff enrichi, galerie photos en attente, retrait post-approbation
```

**Structure Decision** : Option 2 — Web application (frontend + backend séparés), imposée par la constitution (§II Monorepo). Extension minimale de l'existant : 1 nouveau fichier SQL (11c), 2 nouveaux handlers Rust (`afripulse_public.rs`, 2 services), 1 nouveau module de modèles (`afripulse.rs`), 6 nouveaux composants Vue publics, 1 nouvelle page utilisateur, 1 nouvelle modal de création de fiche pays. Réutilisation intégrale du pipeline `contribution_fiche` + handlers admin existants.

## Complexity Tracking

*Section vide — aucun principe constitutionnel n'est violé. Aucune justification requise.*

---

## Phase 0 Output

Voir [research.md](./research.md) pour le détail des décisions techniques suivantes :

- D1. Réutilisation de `country_profile.contribution_fiche` vs création d'un pipeline parallèle (décision : réutilisation, justifiée par §V Simplicité et §VII Audit déjà câblé).
- D2. Extension des enums `type_contribution` et ajout d'un enum `section_contribution` (décision : ajouter valeurs + nouvel enum `type_objet_contribution` distinguant `fiche_pays` / `element_section` / `photo_visiteur` / `recommandation_visiteur`).
- D3. Stratégie d'application d'une contribution approuvée aux tables cibles (décision : le handler `moderer_contribution` exécute une transaction SQL qui insère/met à jour/supprime la ligne cible selon `type_contribution` + `section` + `target_id`, et marque les contributions concurrentes en `obsolete`).
- D4. Validation côté serveur des photos (décision : crate `image` 0.25 — lecture magic bytes + décodage en mémoire pour obtenir dimensions ; rejet si > 2 Mo, > 2048×2048, format non JPEG/PNG ; refus AVANT stockage disque).
- D5. Rate-limit : calcul en temps réel via 3 requêtes COUNT paramétrées sur `contribution_fiche` (pas de table compteur) filtrant par `cree_par`, `type_objet`, `created_at > NOW() - INTERVAL '24h'` + filtre `etat = 'en_attente'` pour la règle « 5 en attente par pays ».
- D6. Source unique des 54 codes ISO africains : fichier TS partagé `app/composables/useAfripulsePaysAutorises.ts` + constante Rust `PAYS_ISO_AUTORISES: &[&str]` dans `src/handlers/afripulse_public.rs`. Synchronisation par review manuelle de la PR (deux listes alignées, test unitaire de comparaison en dev).
- D7. Multipart upload strategy (décision : 1 endpoint POST `/api/fiches-pays/{id}/contributions` accepte soit JSON text-only, soit multipart avec champs `section`, `type_contribution`, `action`, `target_id`, `nouvelle_valeur`, `justification` + 1 à 5 fichiers `photos[]` avec une légende associée par champ `legendes[]`). Un seul endpoint → une seule modération.

## Phase 1 Outputs

- [data-model.md](./data-model.md) — DDL complet des 4 nouvelles tables, extensions de `site_touristique` et `contribution_fiche`, enums ajoutés, relations, triggers d'unicité recommandation, mapping Rust `FromRow` et TS.
- [contracts/public.openapi.yaml](./contracts/public.openapi.yaml) — contrat HTTP des endpoints publics (POST création fiche modérée, POST contribution multipart, GET /moi/contributions, lectures sections enrichies).
- [contracts/admin.openapi.yaml](./contracts/admin.openapi.yaml) — contrat HTTP des endpoints admin (modération enrichie, retrait post-approbation, diff structuré).
- [quickstart.md](./quickstart.md) — scénarios cURL + UI reproductibles couvrant les 5 User Stories et les règles critiques (rate-limit, périmètre ISO, unicité recommandation).

## Post-Design Constitution Re-Check

Après rédaction de `research.md`, `data-model.md` et `contracts/`, re-vérification :

| Principe | Statut | Note |
|---|---|---|
| I. Français | PASS | Tous identifiants, DTOs et routes en français. |
| II. Monorepo | PASS | Cross-stack cohérent (SQL → Rust → TS). |
| III. SQL SoT | PASS | DDL complet dans `11c_country_profile_afripulse.sql` avant propagation. |
| IV. Sécurité | PASS | Auth obligatoire, validation photos côté serveur, rate-limit SQL, `audit::log_action` sur toutes mutations. |
| V. Simplicité | PASS | Aucune abstraction nouvelle : extension du pipeline existant. |
| VI. Tailwind v4 / daisyUI | PASS | Composants publics Tailwind pur, admin daisyUI. |
| VII. Audit | PASS | Toutes nouvelles mutations instrumentées. |

**Verdict** : le design passe les 7 principes. Le plan est prêt pour la phase `/speckit.tasks`.
