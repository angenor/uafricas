# Tasks: Vidafrica : Sous-titrage vidéo multilingue karaoke

**Input**: Design documents from `/specs/004-vidafrica-sous-titres/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Non demandés dans la spécification, pas de tâches de tests générées.

**Organization**: Tâches groupées par user story. US3 (gestion vidéos) est traitée avant US1 (sous-titres) car les vidéos doivent exister avant de pouvoir y ajouter des sous-titres.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Peut s'exécuter en parallèle (fichiers différents, pas de dépendances)
- **[Story]**: User story associée (US1–US5)
- Chemins exacts inclus dans les descriptions

---

## Phase 1: Setup (Infrastructure partagée)

**Purpose**: Initialisation du schema SQL et de la structure des modules

- [x] T001 Créer le fichier schema SQL `uafricas_backend/doc/bd/schemas/27_vidafrica.sql` avec l'enum `langue_sous_titre` et les 4 tables (`video`, `piste_sous_titre`, `segment_sous_titre`, `timing_mot`) selon `data-model.md`
- [x] T002 Ajouter `\ir schemas/27_vidafrica.sql` dans `uafricas_backend/doc/bd/schema.sql` (après le dernier `\ir`)
- [x] T003 Recréer la base de données : `docker compose down -v && docker compose up -d`

---

## Phase 2: Foundational (Prérequis bloquants)

**Purpose**: Models Rust, déclarations de modules et enregistrement des routes, DOIT être complet avant toute implémentation de user story

- [x] T004 [P] Créer les structs admin dans `uafricas_backend/src/models/admin/vidafrica.rs` : `AdminVideoListeResponse`, `AdminVideoDetailRow`/`DetailResponse`, `CreerVideoRequest`, `ModifierVideoRequest`, `ChangerEtatVideoRequest`, `AdminVideoQueryParams`, `AdminPisteSousTitreResponse`, `AdminSegmentSousTitreResponse`, `TimingMotResponse`, `CreerPisteRequest`, `CreerSegmentRequest`, `ModifierSegmentRequest`, `EnregistrerTimingsMotRequest` + constantes `ADMIN_VIDEO_LISTE_COLONNES`, `ADMIN_VIDEO_DETAIL_COLONNES`, `VIDEO_TRI_COLONNES`
- [x] T005 [P] Créer les structs publiques dans `uafricas_backend/src/models/vidafrica.rs` : `VideoPubliqueListeResponse`, `VideoPubliqueDetailResponse`, `SousTitresResponse`, `SegmentPubliqueResponse`, `MotTimingResponse`, `LangueDisponibleResponse`, `VideoPubliqueQueryParams`
- [x] T006 Déclarer le module `pub mod vidafrica;` dans `uafricas_backend/src/models/admin/mod.rs` et `pub mod vidafrica;` dans `uafricas_backend/src/models/mod.rs`
- [x] T007 [P] Créer le fichier handler admin vide (squelette avec signatures) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`
- [x] T008 [P] Créer le fichier handler public vide (squelette avec signatures) dans `uafricas_backend/src/handlers/vidafrica.rs`
- [x] T009 Déclarer `pub mod vidafrica;` dans `uafricas_backend/src/handlers/admin/mod.rs` et `pub mod vidafrica;` dans `uafricas_backend/src/handlers/mod.rs`
- [x] T010 Enregistrer toutes les routes admin et publiques dans `uafricas_backend/src/routes.rs` selon les contrats API (`contracts/api-admin-vidafrica.md` et `contracts/api-public-vidafrica.md`)
- [x] T011 Vérifier la compilation backend : `cargo build`, corriger les erreurs éventuelles

**Checkpoint**: Structure Rust compilable avec tous les modules déclarés et routes enregistrées (handlers vides retournant 501)

---

## Phase 3: User Story 3 : L'admin gère les vidéos Vidafrica (Priority: P2) 🎯 MVP Foundation

**Goal**: CRUD complet des vidéos avec upload multipart (fichier vidéo + vignette), changement d'état, soft delete et audit

**Independent Test**: Créer, modifier, publier et supprimer une vidéo via les endpoints admin. Vérifier que le fichier vidéo est stocké dans `./uploads/videos/` et la vignette dans `./uploads/vignettes/`.

### Implementation

- [x] T012 [US3] Implémenter le handler `creer_video` (multipart : titre, description, fichier_video, vignette) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`, validation format (MP4/WebM), limite 500 Mo vidéo / 5 Mo vignette, stockage `./uploads/videos/` et `./uploads/vignettes/`, génération slug, extraction durée si possible, audit log
- [x] T013 [US3] Implémenter le handler `lister_videos` (paginé, filtres recherche + état, tri dynamique) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`, jointure COUNT pistes
- [x] T014 [US3] Implémenter le handler `obtenir_video` (détail avec pistes associées) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`
- [x] T015 [US3] Implémenter le handler `modifier_video` (multipart : titre, description, vignette, pas de remplacement vidéo) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`, audit log
- [x] T016 [P] [US3] Implémenter les handlers `changer_etat_video` et `supprimer_video` (soft delete) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`, audit log
- [x] T017 [US3] Créer le fichier mock `uafricas_frontend/app/mocks/vidafrica.ts`, interfaces TypeScript (`Video`, `PisteSousTitre`, `SegmentSousTitre`, `TimingMot`, `LangueSousTitre`) + données mock (3 vidéos, 2 pistes, 5 segments, timings mot) + helpers (`getVideoParId`, `getVideoParSlug`, `filtrerVideos`)
- [x] T018 [US3] Créer le composable admin `uafricas_frontend/app/composables/useAdminVidafrica.ts`, utilise `useAdmin()`, expose : `videos`, `filtres`, `pagination`, `sort`, `chargerListe`, `chargerDetail`, `creer` (multipart FormData), `modifier`, `changerEtat`, `supprimer`
- [x] T019 [US3] Créer la page admin liste `uafricas_frontend/app/pages/admin/vidafrica/index.vue`, `AdminPageHeader`, `AdminFilters` (recherche + état), `AdminDataTable` (colonnes: vignette, titre, durée, nb pistes, état, date), `AdminDeleteConfirm`, bouton "Nouvelle vidéo"
- [x] T020 [US3] Créer la page admin création `uafricas_frontend/app/pages/admin/vidafrica/create.vue`, formulaire multipart avec champs titre, description (textarea), fichier vidéo (input file accept=".mp4,.webm"), vignette (input file accept=".jpg,.png,.webp"), validation côté client, redirection vers `[id]` après création
- [x] T021 [US3] Créer la page admin édition `uafricas_frontend/app/pages/admin/vidafrica/[id].vue` : onglet "Informations" avec formulaire modification + changement d'état + aperçu vidéo + suppression. Préparer un onglet vide "Sous-titres" pour US1

**Checkpoint**: L'admin peut créer/modifier/publier/supprimer des vidéos. Les fichiers sont uploadés et servis correctement. La base de données est peuplée.

---

## Phase 4: User Story 1 : L'admin saisit des sous-titres pour une vidéo (Priority: P1) 🎯 MVP

**Goal**: Gestion complète des pistes de sous-titres, segments et timings mot par mot via l'interface "tap-to-mark"

**Independent Test**: Sélectionner une vidéo existante, créer une piste en français, ajouter 3 segments avec texte et timestamps, utiliser tap-to-mark pour enregistrer les timings mot, vérifier la persistance en BDD.

### Implementation : Backend

- [x] T022 [P] [US1] Implémenter les handlers pistes (`lister_pistes`, `creer_piste`, `supprimer_piste`) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`, validation unicité langue, audit log
- [x] T023 [P] [US1] Implémenter les handlers segments (`lister_segments`, `creer_segment`, `modifier_segment`, `supprimer_segment`, `reordonner_segments`) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`, validation timestamps (début < fin, pas de chevauchement), auto-incrémentation position, CASCADE DELETE timings mot, audit log
- [x] T024 [US1] Implémenter les handlers timings mot (`enregistrer_timings_mot`, `supprimer_timings_mot`) dans `uafricas_backend/src/handlers/admin/vidafrica.rs`, endpoint batch qui remplace les timings existants, validation positions et timestamps, mise à jour `est_complete` sur la piste, audit log

### Implementation : Frontend

- [x] T025 [US1] Enrichir le composable `uafricas_frontend/app/composables/useAdminVidafrica.ts`, ajouter : `chargerPistes`, `creerPiste`, `supprimerPiste`, `chargerSegments`, `creerSegment`, `modifierSegment`, `supprimerSegment`, `reordonnerSegments`, `enregistrerTimingsMot`, `supprimerTimingsMot`
- [x] T026 [US1] Compléter l'onglet "Sous-titres" de la page `uafricas_frontend/app/pages/admin/vidafrica/[id].vue`, liste des pistes (langue + badge complet/incomplet + nb segments), bouton "Ajouter une piste" (sélecteur langue), suppression piste, clic sur piste → affichage des segments
- [x] T027 [US1] Implémenter la gestion des segments dans `uafricas_frontend/app/pages/admin/vidafrica/[id].vue`, liste ordonnée des segments (position, texte tronqué, début/fin formatés), formulaire ajout/édition segment (texte, début_ms, fin_ms avec validation), suppression, réordonnement drag-and-drop ou boutons haut/bas
- [x] T028 [US1] Créer le composant `uafricas_frontend/app/components/vidafrica/VidafricaTapToMark.vue`, reçoit props `videoUrl` (string) et `mots` (string[]), émet `@timings-enregistres` (TimingMot[]) ; affiche la vidéo + les mots à marquer séquentiellement ; écoute touche Espace ou clic bouton pour capturer `currentTime` à chaque mot ; bouton "Recommencer" pour refaire ; affichage visuel du mot courant surligné ; état "terminé" quand tous les mots sont marqués
- [x] T029 [US1] Intégrer `VidafricaTapToMark` dans la page `[id].vue`, bouton "Marquer les timings" sur chaque segment ouvre le mode tap-to-mark, les timings résultants sont envoyés au backend via `enregistrerTimingsMot`, affichage des timings existants avec possibilité de "Remarquer"

**Checkpoint**: L'admin peut créer des pistes multilingues, saisir des segments, utiliser tap-to-mark pour les timings mot par mot. Les données sont persistées et récupérables.

---

## Phase 5: User Story 2 : Le visiteur regarde une vidéo avec sous-titres karaoké (Priority: P1) 🎯 MVP

**Goal**: Lecteur vidéo public avec sous-titres synchronisés et surlignage mot par mot (karaoké), changement de langue en temps réel

**Independent Test**: Accéder à une vidéo publiée avec sous-titres, lancer la lecture, vérifier le surlignage karaoké synchronisé, changer de langue pendant la lecture, tester pause/reprise et seek.

### Implementation : Backend

- [x] T030 [P] [US2] Implémenter le handler `obtenir_video_publique` (par slug, état publie uniquement) dans `uafricas_backend/src/handlers/vidafrica.rs`, retourne détail vidéo + langues disponibles
- [x] T031 [P] [US2] Implémenter le handler `obtenir_sous_titres` (par video_id + langue) dans `uafricas_backend/src/handlers/vidafrica.rs`, retourne tous les segments avec timings mot, ordonnés par position
- [x] T032 [P] [US2] Implémenter le handler `lister_langues_disponibles` dans `uafricas_backend/src/handlers/vidafrica.rs`, retourne les langues ayant au moins une vidéo publiée avec piste

### Implementation : Frontend

- [x] T033 [US2] Créer le composable public `uafricas_frontend/app/composables/useVidafrica.ts`, utilise `$fetch` avec `apiBase`, expose : `chargerVideo(slug)`, `chargerSousTitres(videoId, langue)`, `chargerLanguesDisponibles()`, interfaces DTO (`VideoAfricaAPI`, `SousTitresAPI`) + interfaces frontend (`VideoAfrica`, `SousTitres`, `SegmentKaraoke`, `MotKaraoke`) + mappers
- [x] T034 [US2] Créer le composant `uafricas_frontend/app/components/vidafrica/VidafricaSelecteurLangue.vue`, props `langues` (string[]) et `langueActive` (string), émet `@changer-langue`, affiche les langues comme boutons/chips avec la langue active surlignée (Tailwind v4 pur, pas de daisyUI)
- [x] T035 [US2] Créer le composant `uafricas_frontend/app/components/vidafrica/VidafricaLecteur.vue`, props `videoUrl` (string), `segments` (SegmentKaraoke[]) ; lecteur `<video>` natif HTML5 avec contrôles ; overlay de sous-titres positionné en bas ; logique de synchronisation : `requestAnimationFrame` qui lit `video.currentTime`, trouve le segment courant par recherche binaire sur `debut_ms/fin_ms`, trouve le mot courant dans le segment, applique une classe CSS de surlignage au mot actif ; gère pause/reprise (arrêt/relance du RAF), seek (recalcul immédiat du segment/mot courant) ; transition fluide entre segments
- [x] T036 [US2] Créer la page `uafricas_frontend/app/pages/vidafrica/[slug].vue`, charge la vidéo par slug, charge les sous-titres dans la première langue disponible, affiche `VidafricaLecteur` + `VidafricaSelecteurLangue`, gère le changement de langue (recharge les sous-titres sans interrompre la lecture via `watch` sur `langueActive`), affiche titre + description sous le lecteur, layout `default` (Tailwind v4 pur)

**Checkpoint**: Un visiteur peut regarder une vidéo avec sous-titres karaoké, changer de langue, faire pause/seek. L'effet karaoké est fluide et synchronisé.

---

## Phase 6: User Story 4 : L'admin prévisualise les sous-titres en temps réel (Priority: P2)

**Goal**: Prévisualisation de l'effet karaoké dans le back-office pendant la saisie des sous-titres

**Independent Test**: Depuis la page d'édition d'une vidéo avec des sous-titres et timings, cliquer "Prévisualiser", vérifier que l'effet karaoké s'affiche correctement avec les timings saisis.

### Implementation

- [x] T037 [US4] Réutiliser le composant `VidafricaLecteur.vue` dans la page admin `uafricas_frontend/app/pages/admin/vidafrica/[id].vue`, ajouter un bouton "Prévisualiser" dans l'onglet Sous-titres, qui ouvre un modal/section avec le lecteur alimenté par les segments/timings de la piste sélectionnée, permet de basculer entre les langues si plusieurs pistes existent
- [x] T038 [US4] Ajouter un mode "prévisualisation rapide" au composant `VidafricaTapToMark.vue`, après l'enregistrement des timings, afficher un bouton "Rejouer avec karaoké" qui relance la vidéo au début du segment avec l'overlay karaoké pour vérification immédiate

**Checkpoint**: L'admin peut prévisualiser l'effet karaoké directement dans le back-office pour ajuster les timings.

---

## Phase 7: User Story 5 : Le visiteur navigue dans le catalogue Vidafrica (Priority: P3)

**Goal**: Page catalogue public avec liste paginée, recherche full-text et filtrage par langue

**Independent Test**: Publier 5+ vidéos avec différentes langues, vérifier la pagination, la recherche par titre, et le filtrage par langue.

### Implementation : Backend

- [x] T039 [US5] Implémenter le handler `lister_videos_publiques` (paginé, filtres recherche full-text + langue) dans `uafricas_backend/src/handlers/vidafrica.rs`, requête avec `search_vector @@ plainto_tsquery`, jointure pistes pour filtrage langue, retourne `langues_disponibles` par vidéo

### Implementation : Frontend

- [x] T040 [US5] Créer le composant `uafricas_frontend/app/components/vidafrica/VidafricaCarteVideo.vue`, props `video` (VideoAfrica), affiche vignette (ou placeholder), titre, durée formatée, badges langues disponibles, lien vers `/vidafrica/{slug}` (Tailwind v4 pur)
- [x] T041 [US5] Créer la page `uafricas_frontend/app/pages/vidafrica/index.vue`, Hero section avec titre "Vidafrica" + description, barre de recherche, filtre par langue (utilise `chargerLanguesDisponibles`), grille de `VidafricaCarteVideo`, pagination, état vide "Aucune vidéo trouvée", layout `default` (Tailwind v4 pur)
- [x] T042 [US5] Enrichir le composable `useVidafrica.ts`, ajouter `listerVideos(params)` avec pagination + filtres recherche/langue

**Checkpoint**: Le catalogue public affiche les vidéos avec recherche et filtres fonctionnels.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Améliorations transversales

- [x] T043 [P] Ajouter le lien "Vidafrica" dans la navigation principale (NavBar) dans `uafricas_frontend/app/components/layout/NavBar.vue`, lien vers `/vidafrica`
- [x] T044 [P] Ajouter le lien "Vidafrica" dans la navigation admin (sidebar) pour accéder à `/admin/vidafrica`
- [x] T045 Vérifier les permissions admin sur tous les endpoints, s'assurer que `verifier_permission!` est appelé avec les bonnes actions (consulter, creer, modifier, supprimer)
- [x] T046 Vérifier que le soft delete fonctionne correctement : les vidéos et pistes supprimées ne remontent pas dans les listes, ni admin ni publiques
- [x] T047 Valider le parcours complet via `quickstart.md` : création vidéo → ajout piste → saisie segments → tap-to-mark → publication → lecture publique avec karaoké → changement langue
- [x] T048 Mettre à jour `CLAUDE.md` : ajouter les endpoints Vidafrica dans la section API Routes, les nouveaux composables et composants dans la section Architecture Frontend

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup SQL)**: Aucune dépendance, démarrage immédiat
- **Phase 2 (Foundational)**: Dépend de Phase 1, BLOQUE toutes les user stories
- **Phase 3 (US3 : Gestion vidéos)**: Dépend de Phase 2, BLOQUE US1 (il faut des vidéos pour y ajouter des sous-titres)
- **Phase 4 (US1 : Saisie sous-titres)**: Dépend de Phase 3
- **Phase 5 (US2 : Lecture karaoké)**: Dépend de Phase 4 (il faut des sous-titres à afficher), peut démarrer le backend (T030-T032) en parallèle de Phase 4
- **Phase 6 (US4 : Prévisualisation)**: Dépend de Phase 4 (US1) + Phase 5 (composant VidafricaLecteur)
- **Phase 7 (US5 : Catalogue)**: Dépend de Phase 3 (vidéos publiées), peut démarrer en parallèle de Phase 4/5
- **Phase 8 (Polish)**: Dépend de toutes les phases précédentes

### User Story Dependencies

```
Phase 1 (SQL) → Phase 2 (Models/Routes)
                    ↓
               Phase 3 (US3: CRUD Vidéos)
                    ↓
               Phase 4 (US1: Sous-titres + Tap-to-mark)
                  ↓              ↘
   Phase 5 (US2: Karaoké)    Phase 7 (US5: Catalogue) [parallélisable]
                  ↓
            Phase 6 (US4: Prévisualisation)
                  ↓
            Phase 8 (Polish)
```

### Within Each User Story

- Models avant handlers
- Handlers backend avant composables frontend
- Composables avant pages/composants
- Core avant intégration

### Parallel Opportunities

- T004 et T005 (models admin et public) en parallèle
- T007 et T008 (handlers squelettes admin et public) en parallèle
- T022 et T023 (handlers pistes et segments) en parallèle
- T030, T031 et T032 (handlers publics) en parallèle
- T040 et T042 (carte vidéo et enrichissement composable) en parallèle
- T043 et T044 (navigation) en parallèle

---

## Parallel Example: Phase 5 (US2)

```bash
# Backend handlers publics : tous en parallèle :
Task T030: "Implémenter obtenir_video_publique dans handlers/vidafrica.rs"
Task T031: "Implémenter obtenir_sous_titres dans handlers/vidafrica.rs"
Task T032: "Implémenter lister_langues_disponibles dans handlers/vidafrica.rs"

# Frontend composants : séquentiels (dépendances) :
Task T033: "Créer useVidafrica.ts" (d'abord)
Task T034: "Créer VidafricaSelecteurLangue.vue" (puis)
Task T035: "Créer VidafricaLecteur.vue" (puis)
Task T036: "Créer page [slug].vue" (enfin)
```

---

## Implementation Strategy

### MVP First (US3 + US1 + US2)

1. Compléter Phase 1 : Schema SQL
2. Compléter Phase 2 : Foundation Rust
3. Compléter Phase 3 : CRUD vidéos admin (US3)
4. Compléter Phase 4 : Sous-titres + tap-to-mark (US1)
5. Compléter Phase 5 : Lecture karaoké publique (US2)
6. **STOP et VALIDER** : Tester le parcours complet admin→public
7. Déployer/démo si prêt

### Incremental Delivery

1. Setup + Foundation → Structure prête
2. US3 (CRUD vidéos) → L'admin peut uploader des vidéos
3. US1 (sous-titres) → L'admin peut sous-titrer avec karaoké
4. US2 (lecture) → Les visiteurs voient le karaoké → **MVP complet !**
5. US4 (prévisualisation) → Qualité de saisie améliorée
6. US5 (catalogue) → Navigation enrichie
7. Polish → Intégration navigation + vérifications finales

---

## Notes

- [P] = fichiers différents, pas de dépendances
- [Story] = rattachement user story pour traçabilité
- Pas de tests automatisés (non configurés dans le projet)
- Committer après chaque tâche ou groupe logique
- Tailwind v4 pur sur les pages publiques, daisyUI v5 autorisé en admin
- Français obligatoire pour tout (variables, colonnes SQL, messages UI)
