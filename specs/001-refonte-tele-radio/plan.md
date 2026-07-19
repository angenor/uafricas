# Implementation Plan: Refonte des pages Télé et Radio Africans

**Branch**: `001-refonte-tele-radio` | **Date**: 2026-07-19 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-refonte-tele-radio/spec.md`

## Summary

Faire passer les trois pages médias (`/medias/tele`, `/medias/radio/africans`, `/medias/radio/nationales`)
d'une grille de vignettes filtrable à une expérience éditorialisée : une vedette vidéo plein écran à
l'ouverture de la page Télé, puis une découverte chaîne par chaîne au défilement, en blocs empilés avec
rangée horizontale. Les deux pages Radio restent distinctes, désormais réellement différenciées par
l'**origine de publication** de leurs stations. S'y greffe une plateforme participative : soumission de
chaînes et de contenus par tout membre, validation administrative systématique, interactions
communautaires, co-détention, grille de programmation et signalement.

**Approche technique** : cinq migrations SQL incrémentales dans `media_content` (indices `09j` à `09n`), une
extension du backend Actix par domaine fonctionnel, et une refonte frontend en Tailwind v4 pur. Chaque brique
réutilise un patron déjà éprouvé du dépôt plutôt que d'en inventer un : `afrolang.proposition_salle` pour la
modération, `afrolang.salle_moderateur` pour la co-détention, la résolution paresseuse en SQL de
`rendez_vous.rs` pour la programmation, la persistance du layout Nuxt pour le lecteur audio. Détail et
justifications : [research.md](./research.md).

**Deux corrections indissociables du périmètre**, découvertes en Phase 0 : les routes publiques de création
insèrent `etat = 'publie'` en dur sans contrôle de rôle (faille), et le retrait du contenu vedette codé en
dur (FR-010) expose un bug latent de lecture des URL YouTube dans une balise `<video>`.

## Technical Context

**Language/Version**: Rust Edition 2024 (backend) · TypeScript 5 / Vue 3 SSR / Nuxt 4 (frontend)
**Primary Dependencies**: Actix-Web 4, sqlx (PostgreSQL), uuid, chrono, serde · Pinia, Tailwind CSS v4, FontAwesome
**Storage**: PostgreSQL 16, schéma `media_content` · uploads locaux `./uploads/medias/{videos,audios}/` servis par actix-files
**Testing**: aucun harnais configuré dans le projet — validation par parcours manuels décrits dans [quickstart.md](./quickstart.md)
**Target Platform**: navigateurs desktop et mobile · serveur Linux (Docker Compose, nginx)
**Project Type**: application web — monorepo frontend Nuxt + backend Actix
**Performance Goals**: vedette jouable < 3 s sur mobile (SC-001) · 50 sections défilables sans précharger les médias (SC-011) · créneau programmé servi à moins d'une minute (SC-010)
**Constraints**: Tailwind v4 pur sur les pages publiques, daisyUI proscrit (Principe VI) · français avec accents partout · audit obligatoire sur toute mutation (Principe VII) · URLs publiques existantes inchangées (FR-012) · migrations idempotentes, jouées manuellement en production
**Scale/Scope**: 3 pages publiques remaniées + 4 pages de détail nouvelles + back-office · 7 user stories, 56 exigences fonctionnelles · volumétrie cible ≈ 50 chaînes/stations

## Constitution Check

*GATE : évalué avant Phase 0, re-évalué après Phase 1.*

| Principe | Verdict | Justification |
|---|---|---|
| **I. Français d'Abord** | ✅ PASS | Tables, colonnes, handlers, composants et libellés en français accentué. Noms de fichiers en `[a-z0-9_-]` sans accent, conformément à la convention. |
| **II. Monorepo Cohérent** | ✅ PASS | Chaque lot livre SQL + Rust + TypeScript ensemble. Les contrats de [contracts/](./contracts/) fixent la correspondance types TS ↔ structs Rust ↔ DDL. |
| **III. SQL Source de Vérité** | ✅ PASS | Ordre imposé : migration `doc/bd/schemas/09*` → modèles Rust `FromRow` → DTO → interfaces TS. Les migrations sont déclarées dans `schema.sql`. |
| **IV. Sécurité par Défaut** | ✅ PASS — *renforce* | La feature **ferme** une faille existante (publication publique sans validation, R-constat n°1). Elle seede aussi les permissions `media` absentes, sans lesquelles seul `super_admin` peut modérer. Requêtes paramétrées sqlx ; discriminants en littéraux whitelistés côté Rust. |
| **V. Simplicité (YAGNI)** | ⚠️ PASS sous réserve | Aucune abstraction inventée : chaque brique copie un patron existant. Trois choix ajoutent néanmoins de la structure — voir [Complexity Tracking](#complexity-tracking). |
| **VI. Tailwind v4 (daisyUI back-office uniquement)** | ✅ PASS — *corrige* | Retire deux violations existantes dans le périmètre (`loading loading-spinner` sur les deux pages Radio) et migre les résidus `bg-gradient-to-*` (v3) vers `bg-linear-to-*` (v4). Aucune dépendance CSS tierce ajoutée (`vue3-carousel` écarté, R10). |
| **VII. Audit & Traçabilité** | ✅ PASS | `audit::log_action` sur chaque mutation nouvelle : décision de modération, ajout/retrait de co-détenteur, modification de grille, suspension par signalement (FR-055). |

**Contraintes techniques** : le domaine se rattache au schéma existant `media_content` — aucun schéma
nouveau. Upload local conservé. Icônes FontAwesome exclusivement. pnpm / Cargo.

**Re-évaluation post-Phase 1** : aucun gate ne bascule. La conception n'a introduit ni schéma, ni
dépendance, ni service supplémentaire. Le seul écart nouveau est l'introduction de `TIME` et
`SMALLINT jour_semaine` alors que le projet impose « TIMESTAMPTZ partout » (`schema.sql:32`) — écart
inhérent à la notion de récurrence, tracé ci-dessous.

## Project Structure

### Documentation (this feature)

```text
specs/001-refonte-tele-radio/
├── spec.md              # Spécification fonctionnelle (56 FR, 7 user stories)
├── plan.md              # Ce fichier
├── research.md          # Phase 0 — 18 décisions techniques argumentées
├── data-model.md        # Phase 1 — entités, DDL, transitions d'état
├── quickstart.md        # Phase 1 — mise en route et parcours de validation
├── contracts/           # Phase 1 — contrats d'API
│   ├── api-public.md
│   ├── api-membre.md
│   └── api-admin.md
├── checklists/
│   └── requirements.md  # Qualité de la spec — tous critères satisfaits
└── tasks.md             # Phase 2 — produit par /speckit.tasks, PAS par /speckit.plan
```

### Source Code (repository root)

```text
uafricas_backend/
├── doc/bd/
│   ├── schema.sql                                   # orchestrateur \ir — 5 lignes à ajouter
│   └── schemas/
│       ├── 09j_media_content_editorial.sql          # origine, vedette globale, thèmes, FK, permissions
│       ├── 09k_media_content_interactions.sql       # réactions, commentaires, partages, signalements
│       ├── 09l_media_content_propositions.sql       # workflow de soumission et modération
│       ├── 09m_media_content_codetention.sql        # co-détenteurs et invitations
│       └── 09n_media_content_programmation.sql      # grille récurrente
└── src/
    ├── handlers/
    │   ├── stations_radio.rs                        # MODIFIÉ — filtre origine, contenus publics, fin du 'publie' en dur
    │   ├── television.rs                            # MODIFIÉ — vedette globale, fin du 'publie' en dur
    │   ├── media_social.rs                          # NOUVEAU — réactions, commentaires, partages, signalements
    │   ├── media_proposition.rs                     # NOUVEAU — soumission membre et suivi
    │   ├── media_detention.rs                       # NOUVEAU — co-détenteurs, invitations, mise en relation
    │   ├── media_programmation.rs                   # NOUVEAU — grille et créneau courant
    │   ├── experts.rs                               # MODIFIÉ — filtre par spécialité
    │   └── admin/
    │       ├── radio_tele.rs                        # MODIFIÉ — vedette globale, origine, thème phare
    │       └── media_proposition.rs                 # NOUVEAU — file de modération
    ├── models/
    │   ├── station_radio.rs, television.rs          # MODIFIÉS
    │   ├── media_social.rs, media_proposition.rs    # NOUVEAUX
    │   ├── media_detention.rs, media_programmation.rs
    │   ├── notification.rs                          # MODIFIÉ — pub mod media
    │   └── admin/radio_tele.rs                      # MODIFIÉ
    └── routes.rs                                    # MODIFIÉ

uafricas_frontend/app/
├── layouts/default.vue                              # MODIFIÉ — montage de la barre de lecture persistante
├── pages/medias/
│   ├── tele.vue                                     # REMANIÉ — vedette plein écran + sections
│   ├── radio/africans.vue                           # REMANIÉ — sections, origine 'africans'
│   ├── radio/nationales.vue                         # REMANIÉ — sections, origine 'territoire'
│   ├── chaines/[slug].vue                           # NOUVEAU — détail SSR + Open Graph
│   ├── stations/[slug].vue                          # NOUVEAU
│   ├── programmes-tele/[slug].vue                   # NOUVEAU
│   └── programmes-radio/[slug].vue                  # NOUVEAU
├── components/media/
│   ├── VedettePleinEcran.vue                        # NOUVEAU — hero 100svh, routage média
│   ├── LecteurMedia.vue                             # NOUVEAU — iframe YouTube | <video>/<audio> natif
│   ├── SectionChaine.vue / SectionStation.vue       # NOUVEAUX — bloc empilé + rangée
│   ├── RangeeContenus.vue                           # NOUVEAU — rangée horizontale scroll-snap
│   ├── BarreLecturePersistante.vue                  # NOUVEAU — ancrée en bas, montée dans le layout
│   ├── MediaReactionsBar.vue / MediaCommentaires.vue
│   ├── MediaPartagerModal.vue / MediaSignalerModal.vue
│   ├── ProposerMediaModal.vue                       # NOUVEAU — remplace AddProgramModal (D-006)
│   ├── GrilleProgrammation.vue                      # NOUVEAU
│   ├── AudioPlayer.vue, StationCard.vue             # REMPLACÉS par les composants de section
│   └── AddProgramModal.vue                          # SUPPRIMÉ — maquette morte
├── composables/
│   ├── useTelevision.ts, useStationsRadio.ts        # MODIFIÉS — origine, vedette, contenus radio
│   ├── useLecteurMedia.ts                           # NOUVEAU — état audio global (useState), remplace useAudioPlayer
│   ├── useMediaSocial.ts, useMediaProposition.ts    # NOUVEAUX
│   ├── useMediaDetention.ts, useMediaProgrammation.ts
│   ├── useObservateurVisibilite.ts                  # NOUVEAU — IntersectionObserver
│   └── useAudioPlayer.ts                            # SUPPRIMÉ — code mort inadapté (R8)
└── pages/publications/index.vue                     # MODIFIÉ — nouvelle source de partage
```

**Structure Decision** : monorepo web existant, sans nouveau module ni nouveau schéma. Le domaine s'ajoute
au bounded context `media_content`. Le découpage backend suit la règle maison « un fichier handler et un
fichier modèle par domaine », les nouveaux domaines étant préfixés `media_` pour se distinguer de
`vidafrica_*` qui partage le même schéma SQL mais relève d'une autre ressource de permission
(`"media_content"` vs `"media"`, R15).

## Lots de livraison

Alignés sur les priorités de la spec. Chaque lot est livrable et démontrable seul.

| Lot | Stories | Migrations | Cœur du travail |
|---|---|---|---|
| **L1 — Consultation** | US1, US2 (P1) | `09j` | Vedette globale + sections + barre de lecture persistante + origine de publication + exposition publique des contenus radio. Retrait du provisoire codé en dur et routage média. **MVP.** |
| **L2 — Participation** | US3, US4 (P2) | `09k`, `09l` | Pages de détail SSR avec Open Graph (prérequis du partage), interactions communautaires, workflow de soumission et file de modération, fermeture de la faille de publication directe. |
| **L3 — Programmation & engagement** | US5, US6, US7 (P3) | `09m`, `09n` | Co-détention et invitations, grille récurrente en résolution paresseuse, propositions d'idées et demandes d'animation, recherche de réalisateurs, signalement avec seuil. |

**Dépendances entre lots** : L2 dépend de L1 pour les sections où s'ancrent les interactions. L3 dépend de
L2 pour la co-détention, issue de l'acceptation d'une demande. Aucune dépendance inverse.

## Complexity Tracking

> Écarts au Principe V assumés et justifiés.

| Écart | Pourquoi nécessaire | Alternative plus simple rejetée parce que |
|---|---|---|
| **Table `proposition_media` polymorphe** (`type_objet` + `target_id` + `donnees JSONB`) plutôt que 4 tables dédiées | US4 couvre chaîne, station, programme télé et programme radio ; FR-045 y ajoute la demande d'animation. Le polymorphisme donne **une** file de modération et **un** composable de suivi. | Quatre tables dédiées imposeraient 4 files d'attente admin, 4 écrans et 4 composables pour un workflow identique — et une cinquième à l'arrivée de FR-045. Le patron polymorphe est déjà en production (`contribution_fiche`, `11c:86-113`). |
| **Co-détention avec rôle + table d'invitation** (2 tables) plutôt qu'une colonne `proprietaire_id` | Arbitrage explicite du commanditaire (clarification du 2026-07-19) : plusieurs membres par chaîne. FR-045 exige que l'acceptation d'une demande ajoute un co-détenteur. | Une colonne unique ne porte pas la pluralité demandée. Sans table d'invitation, seul un administrateur pourrait ajouter un co-détenteur, ce qui contredit FR-045 (acceptation par la chaîne elle-même). |
| **Grille de programmation avec `TIME` + `jour_semaine`**, alors que le projet impose « TIMESTAMPTZ partout » | La récurrence quotidienne/hebdomadaire (FR-037) n'est pas représentable par un instant. FR-042 exige un référentiel horaire explicite, d'où la colonne `fuseau`. | Stocker des `TIMESTAMPTZ` matérialisés créneau par créneau imposerait de générer des lignes à l'infini et un travail de fond pour les entretenir — précisément ce que la résolution paresseuse (R7) évite. |
| **Quatre pages de détail non prévues par la spec** | Un aperçu social (FR-026) exige une URL propre au contenu ; aucune page de détail média n'existe (R12). | Partager l'URL de liste produirait un aperçu identique pour tous les contenus, vidant FR-026 de son sens. Une ancre `#` n'est pas transmise au serveur, donc inexploitable pour l'Open Graph. |
| **Nouveau composant de rangée horizontale** au lieu de `vue3-carousel` déjà présent | FR-022 demande une rangée catalogue défilante. | `vue3-carousel` est configuré en une-slide-à-la-fois avec autoplay et impose une feuille de style tierce sur des pages publiques tenues au Tailwind pur (Principe VI). |
| **`IntersectionObserver` introduit dans le projet** | SC-011 et FR-054 interdisent de précharger 50 sections de médias. | `useAOS` n'anime que l'apparition et ne diffère aucun chargement ; aucun autre mécanisme n'existe. |

**Dette préexistante corrigée au passage** (sans élargir le périmètre) : permissions `media` absentes du
seed, `notifications.type VARCHAR(30)` trop court, FK manquantes sur `station_radio`, deux violations
daisyUI, résidus de dégradés Tailwind v3. Chacune bloquerait une exigence de cette feature si elle était
laissée en l'état.
