# Implementation Plan: Médias — programmes conteneurs, épisodes, thématiques multiples et couverture panafricaine

**Branch**: `009-medias-programmes-episodes` | **Date**: 2026-08-08 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/009-medias-programmes-episodes/spec.md`

## Summary

Le modèle actuel confond l'émission et le fichier : `media_content.programme_tele` porte une
`video_url`, `programme_radio` une `audio_url`. Cette feature introduit le niveau manquant. Deux tables
conteneurs (`emission_tele`, `emission_radio`) sont créées ; les tables `programme_*` existantes
deviennent `episode_tele` / `episode_radio` **en conservant leurs identifiants et leurs slugs**, ce qui
laisse intactes les quatre tables d'interactions polymorphes et les adresses publiques déjà indexées.

La grille de programmation cesse de pointer un fichier pour pointer une **émission**, et l'épisode
diffusé se déduit par **rotation** : le créneau porte une `date_effet`, le rang de l'occurrence courante
se calcule en SQL depuis cette origine, et l'épisode retenu est celui de ce rang modulo le nombre
d'épisodes publiés. Aucune tâche de fond n'est ajoutée — c'est le prolongement exact de la résolution
paresseuse déjà en place dans `media_programmation.rs`.

Trois évolutions latérales accompagnent le recadrage : les supports gagnent des **thématiques multiples**
et une **couverture territoriale** (liste de territoires ou continentale) via deux tables de liaison
polymorphes ; tout épisode versé par un co-détenteur naît en `etat = 'en_attente'` et n'entre dans la
rotation qu'une fois **validé par un administrateur** ; les interactions acceptent désormais l'émission
comme l'épisode, portées par un discriminant élargi de 4 à 6 valeurs.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) · TypeScript 5 / Vue 3 SSR / Nuxt 4 (frontend)

**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL, requêtes runtime `query_as`), uuid, chrono,
serde · Pinia, Tailwind CSS v4, daisyUI v5 (back-office uniquement), FontAwesome. **Aucune dépendance
nouvelle.**

**Storage**: PostgreSQL 16, schéma `media_content`. Uploads locaux `./uploads/medias/{videos,audios}/`
servis par actix-files — inchangés.

**Testing**: Aucun harnais configuré sur le projet. La validation passe par `quickstart.md` (scénarios
manuels reproductibles) et par `cargo check` / diagnostics Volar.

**Target Platform**: Serveur Linux (Docker Compose prod), navigateurs desktop et mobile.

**Project Type**: Application web monorepo — backend Rust + frontend Nuxt.

**Performance Goals**: Résolution de la diffusion en cours en 2 requêtes par support (inchangé) ;
sections d'une page Télé/Radio servies sans requête N+1 ; filtre thématique/territoire indexé.

**Constraints**: Pas de tâche de fond (la rotation se calcule à la lecture) ; migration en une seule
fenêtre, sans cohabitation des deux modèles ; conservation des identifiants, slugs et interactions
existants ; PostgreSQL 16 requis (déjà exigé depuis la migration `35c`).

**Scale/Scope**: 18 fichiers backend et 24 fichiers frontend touchent `programme_tele` /
`programme_radio` (137 occurrences côté Rust). Une migration SQL (`09q`). Cible de charge : un support
de 50 émissions et 500 épisodes navigable sans dégradation.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principe | Statut | Justification |
|----------|--------|---------------|
| **I. Français d'abord** | ✅ | Tables, colonnes, structs, composables et libellés en français. Le mot « programme » du commanditaire désigne le **conteneur** ; le code le nomme `emission_*` (voir R1) et l'UI publique affiche « Programme ». Ce décalage libellé↔code suit un précédent établi du projet : l'UI dit « territoire » là où la BD dit `pays`. |
| **II. Monorepo cohérent** | ✅ | Migration SQL, structs Rust, DTO et types TS livrés dans la même PR. Aucune extraction. |
| **III. SQL source de vérité** | ✅ | La migration `09q` précède tout code. Conventions respectées : UUID v4, `deleted_at`, TIMESTAMPTZ, snake_case français, CHECK explicites. **Écart assumé** : `creneau_programmation.date_effet` est un `DATE` et non un `TIMESTAMPTZ` — voir Complexity Tracking. |
| **IV. Sécurité par défaut** | ✅ | Les routes de gestion passent par `garde_detenteur` (jamais `AdminUtilisateur`), les routes admin par `verifier_permission!(admin, "media", …)`. FR-040 **ferme un trou** : aujourd'hui un co-détenteur publie un contenu sans revue ; désormais tout épisode naît `en_attente`. Requêtes paramétrées sqlx, `sanitize-filename` sur les uploads — inchangés. |
| **V. Simplicité (YAGNI)** | ⚠️ | Aucun trait, aucune abstraction nouvelle, aucun feature flag, aucune tâche de fond. Les patrons existants sont réutilisés tels quels (polymorphisme `(type_support, support_id)` de 09k/09m/09n ; `ordre` + endpoint `reordonner` de `formation_contenu.rs`). **Deux points de complexité assumés** sur décision du commanditaire — voir Complexity Tracking. |
| **VI. Tailwind v4 / daisyUI back-office** | ✅ | Pages publiques `/medias/**` en Tailwind v4 pur ; `/admin/medias/**` en daisyUI. Aucun CSS hors `main.css`. |
| **VII. Audit & traçabilité** | ✅ | `audit::log_action` sur chaque mutation : émission, épisode, ordre, thématiques, couverture, créneau, décision de modération (FR-045). |

**Verdict initial** : PASS, avec deux entrées en Complexity Tracking.

**Re-vérification post-Phase 1** : PASS. La conception n'a introduit ni abstraction, ni dépendance, ni
tâche de fond supplémentaire. Le nombre d'objets SQL nouveaux (4 tables métier + 2 tables de liaison) est
la conséquence directe du modèle demandé, pas d'une généralisation spéculative.

## Project Structure

### Documentation (this feature)

```text
specs/009-medias-programmes-episodes/
├── plan.md              # Ce fichier
├── spec.md              # Spécification approuvée (58 FR, 12 SC)
├── research.md          # Phase 0 — 10 décisions techniques
├── data-model.md        # Phase 1 — schéma cible et migration 09q
├── quickstart.md        # Phase 1 — scénarios de validation
├── contracts/
│   ├── api-public.md    # Lecture publique (télé, radio, grille, diffusion)
│   ├── api-membre.md    # Gestion par les co-détenteurs
│   └── api-admin.md     # Back-office et file de modération
├── checklists/
│   └── requirements.md  # Checklist qualité de la spec (16/16)
└── tasks.md             # Phase 2 — produit par /speckit-tasks
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/schemas/
│   └── 09q_media_content_emissions_episodes.sql   # NOUVEAU — migration unique
├── src/
│   ├── models/
│   │   ├── media_emission.rs          # NOUVEAU — Emission{Tele,Radio}, DTO, COLONNES
│   │   ├── media_episode.rs           # NOUVEAU — Episode{Tele,Radio}, DTO, COLONNES
│   │   ├── media_support.rs           # NOUVEAU — thématiques et couverture territoriale
│   │   ├── media_programmation.rs     # MODIFIÉ — date_effet, rotation, est_rediffusion
│   │   ├── media_detention.rs         # MODIFIÉ — table_contenu_pour_support → émissions
│   │   ├── media_social.rs            # MODIFIÉ — discriminant 4 → 6 valeurs
│   │   ├── media_proposition.rs       # MODIFIÉ — type_objet emission_*/episode_*
│   │   ├── television.rs              # MODIFIÉ — sections porteuses d'émissions
│   │   ├── programme_radio.rs         # MODIFIÉ — devient le pendant radio
│   │   └── station_radio.rs           # MODIFIÉ — thématiques, couverture
│   ├── handlers/
│   │   ├── media_emission.rs          # NOUVEAU — CRUD émissions (détenteurs)
│   │   ├── media_episode.rs           # NOUVEAU — CRUD épisodes, ordre, déplacement
│   │   ├── media_programmation.rs     # MODIFIÉ — rotation, grille sur émission
│   │   ├── television.rs              # MODIFIÉ — vedette, sections, slugs
│   │   ├── stations_radio.rs          # MODIFIÉ — idem côté radio
│   │   ├── media_social.rs            # MODIFIÉ — cibles émission et épisode
│   │   ├── media_proposition.rs       # MODIFIÉ — proposer une émission / un épisode
│   │   └── admin/
│   │       ├── media_moderation_episode.rs  # NOUVEAU — file, valider, rejeter
│   │       ├── radio_tele.rs          # MODIFIÉ — CRUD admin émissions/épisodes
│   │       └── media_proposition.rs   # MODIFIÉ — validation crée émission ou épisode
│   ├── services/
│   │   └── engagement.rs              # MODIFIÉ — resoudre_beneficiaire : 4 nouveaux type_objet
│   └── routes.rs                      # MODIFIÉ — ~24 routes ajoutées/renommées

uafricas_frontend/app/
├── composables/
│   ├── useMediaEmissions.ts           # NOUVEAU — émissions et épisodes (public + détenteur)
│   ├── useAdminMediaEmissions.ts      # NOUVEAU — back-office
│   ├── useAdminMediaModeration.ts     # NOUVEAU — file de validation des épisodes
│   ├── useTelevision.ts               # MODIFIÉ
│   ├── useStationsRadio.ts            # MODIFIÉ
│   ├── useMediaProgrammation.ts       # MODIFIÉ — rotation, rediffusion
│   ├── useMediaSocial.ts              # MODIFIÉ — 6 cibles
│   └── useMediaProposition.ts         # MODIFIÉ
├── components/media/
│   ├── CarteEmission.vue              # NOUVEAU — bloc émission d'une section
│   ├── ListeEpisodes.vue              # NOUVEAU — épisodes paginés d'une émission
│   ├── GestionEpisodes.vue            # NOUVEAU — ajout, ordre, état (détenteur)
│   ├── SelecteurThematiques.vue       # NOUVEAU — 1..N thèmes
│   ├── SelecteurCouverture.vue        # NOUVEAU — territoires ou « toute l'Afrique »
│   ├── SectionChaine.vue              # MODIFIÉ — émissions au lieu de vidéos
│   ├── SectionStation.vue             # MODIFIÉ
│   ├── RangeeContenus.vue             # MODIFIÉ
│   ├── CarteContenu.vue               # MODIFIÉ
│   ├── GrilleProgrammation.vue        # MODIFIÉ — cible émission
│   ├── BandeauDiffusion.vue           # MODIFIÉ — émission + épisode + rediffusion
│   └── ProposerMediaModal.vue         # MODIFIÉ
├── pages/medias/
│   ├── emissions-tele/[slug].vue      # NOUVEAU — page émission télé
│   ├── emissions-radio/[slug].vue     # NOUVEAU — page émission radio
│   ├── programmes-tele/[slug].vue     # MODIFIÉ — devient la page ÉPISODE (slug conservé)
│   ├── programmes-radio/[slug].vue    # MODIFIÉ — idem
│   ├── chaines/[slug].vue             # MODIFIÉ
│   ├── stations/[slug].vue            # MODIFIÉ
│   └── tele.vue                       # MODIFIÉ
└── pages/admin/medias/
    ├── emissions/                     # NOUVEAU — CRUD émissions + épisodes
    └── moderation-episodes.vue        # NOUVEAU — file de validation
```

**Structure Decision**: Structure web monorepo existante — `uafricas_backend/` (Rust/Actix) et
`uafricas_frontend/` (Nuxt 4). Aucun nouveau répertoire de premier niveau. Les nouveaux modules backend
suivent la règle « un fichier handler/model par domaine » ; les nouveaux composants rejoignent
`app/components/media/` déjà constitué. Les pages `programmes-{tele,radio}/[slug].vue` sont **conservées
à leur emplacement** et changent de sens (page d'épisode) : c'est ce qui garantit FR-056 sans table de
redirection.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|--------------------------------------|
| **Interactions sur deux niveaux** (émission ET épisode) — double la surface de modération, les compteurs et les cibles de signalement (FR-047 à FR-050) | Décision explicite du commanditaire (clarification Q3, 2026-08-08) : une série se suit et se commente en tant que telle, indépendamment de chaque épisode | Interactions sur l'épisode seul : plus simple, un seul fil, une seule file de signalement — mais prive l'émission de toute vie communautaire propre, ce que le commanditaire a écarté |
| **`creneau_programmation.date_effet` en `DATE` et non `TIMESTAMPTZ`** — écart au principe III | L'origine du comptage des occurrences doit être exprimée dans le **même référentiel local** que `heure_debut` et `jour_semaine`. Un `TIMESTAMPTZ` décalerait le rang d'occurrence d'un cran selon le fuseau du lecteur, rendant la rotation non déterministe (FR-017) | `TIMESTAMPTZ` : conforme à la lettre du principe, mais casse la garantie de déterminisme. L'écart prolonge celui déjà assumé et documenté par la migration `09n` (`TIME` + `jour_semaine` + `fuseau`) |
| **Modération systématique des épisodes** — introduit un état `rejete`, un motif, une file admin et un risque de goulot d'étranglement | Décision explicite du commanditaire (clarification Q2). Elle **ferme aussi une faille** : aujourd'hui un co-détenteur publie sans revue | Publication immédiate par le co-détenteur : plus fluide et sans file — mais écartée par le commanditaire, et laisse le contrôle éditorial au seul signalement a posteriori |
