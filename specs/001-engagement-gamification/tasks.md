---
description: "Task list : Système d'engagement / gamification AFRICANS, Phase 1"
---

# Tasks: Système d'engagement / gamification AFRICANS, Phase 1

**Input**: Design documents from `/specs/001-engagement-gamification/`
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/engagement-api.md](./contracts/engagement-api.md)

**Tests**: Aucun harnais de test dans le projet (constitution : pas de testing/CI). Pas de tâches de test. Vérification via [quickstart.md](./quickstart.md) + `cargo check` + diagnostics LSP (rust-analyzer / Volar).

**Organisation** : tâches groupées par user story pour livraison incrémentale indépendante.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers différents, aucune dépendance sur une tâche incomplète)
- **[Story]** : US1–US5 (référence spec.md)
- Chemins de fichiers exacts inclus. Langue et conventions : français (Principe I), SQL source de vérité (Principe III), audit sur mutations admin (Principe VII).

---

## Phase 1: Setup (infrastructure partagée)

**Purpose** : créer le schéma de données et les fichiers squelettes qui compilent.

- [ ] T001 [P] Créer la migration idempotente `uafricas_backend/doc/bd/schemas/NN_engagement.sql` (numéro = prochain libre, ≈`33`) : `CREATE SCHEMA IF NOT EXISTS engagement` + 5 tables `compte`, `mouvement_points`, `regle_points`, `palier_popularite`, `niveau` avec index, contraintes (`CHECK (solde_points>=0)`, `UNIQUE(cle_idempotence)`, `UNIQUE(seuil_likes)`, `UNIQUE(code)`) et **seed du barème** (règles, paliers 100/500/1000, niveaux membre/premium/platinum), cf. [data-model.md](./data-model.md).
- [ ] T002 Brancher la migration dans l'orchestrateur `uafricas_backend/doc/bd/schema.sql` via `\ir schemas/NN_engagement.sql` (après les autres schémas) et l'appliquer en dev (`psql "$DATABASE_URL" -f …` ou re-init Docker).
- [ ] T003 [P] Seeder la **permission admin `engagement`** dans le référentiel IAM (fichier de seed IAM / migration `04*`), sur le modèle des permissions existantes (`mooc`, `media`, `gouvernance`).
- [ ] T004 [P] Créer les fichiers squelettes compilables : `uafricas_backend/src/services/engagement.rs`, `src/models/engagement.rs`, `src/models/admin/engagement.rs`, `src/handlers/engagement.rs`, `src/handlers/admin/engagement.rs` (stubs) + déclarer les `mod` dans `services/mod.rs`, `models/mod.rs`, `models/admin/mod.rs`, `handlers/mod.rs`, `handlers/admin/mod.rs`.

---

## Phase 2: Foundational (prérequis bloquants) ⚠️

**Purpose** : le moteur de points partagé par TOUTES les user stories. Aucune US ne peut démarrer avant.

**⚠️ CRITICAL** : rien des Phases 3+ ne fonctionne sans ce noyau.

- [ ] T005 [P] Implémenter `recalculer_niveau(solde_points) -> String` (fonction pure, lit `engagement.niveau` : plus grand `seuil_min <= solde`) dans `uafricas_backend/src/services/engagement.rs`.
- [ ] T006 Implémenter le helper interne d'upsert du compte + **reset mensuel paresseux** (D5) et le **calcul d'écrêtage plafond** journalier/mensuel (D6) dans `src/services/engagement.rs` (dépend de T005).
- [ ] T007 Implémenter `attribuer(pool, utilisateur_id, type_action, type_objet, objet_id, cle_idempotence)` : charge la règle active, `INSERT mouvement ON CONFLICT (cle_idempotence) DO NOTHING`, met à jour `compte` (solde global/mensuel, réputation, `dernier_mouvement_at`), recalcule `niveau_code` ; **non-bloquant** (erreurs `log::error!`, jamais propagées, D1) dans `src/services/engagement.rs` (dépend de T006).
- [ ] T008 Implémenter `retirer(...)` (malus factcheck : `points` via `GREATEST(0, solde+delta)` plancher 0, `reputation_delta` sans plancher, D7) dans `src/services/engagement.rs` (dépend de T007).
- [ ] T009 Implémenter `evaluer_popularite(pool, type_objet, objet_id, auteur_id, likes_count)` : pour chaque `palier_popularite` actif `seuil_likes <= likes_count`, appelle `attribuer` avec la clé `popularite:{type_objet}:{objet_id}:{seuil}` et le montant du palier (D3) dans `src/services/engagement.rs` (dépend de T007).
- [ ] T010 [P] Créer les DTO publics `FromRow` + `Response` (Compte, MouvementPoints, Niveau, ProchainNiveau) dans `src/models/engagement.rs` (reflètent le schéma, Principe III).
- [ ] T011 [P] Créer les DTO admin (Regle/Palier/Niveau list+detail, `CreerPalier`/`ModifierRegle`/`ModifierNiveau`/`ModifierPalier` Request, `AjustementRequest`, ligne de journal enrichie du nom membre) dans `src/models/admin/engagement.rs`.
- [ ] T012 Enregistrer les scopes `/api/engagement` (public, JWT) et `/api/admin/engagement` (admin + permission `engagement`) dans `uafricas_backend/src/routes.rs`, câblés sur les handlers stubs (compilent) (dépend de T004).

**Checkpoint** : `cargo check` vert ; le moteur peut créditer/débiter et évaluer la popularité (testable en base via Adminer).

---

## Phase 3: User Story 1 : Points pour une contribution validée (Priority: P1) 🎯 MVP

**Goal** : crédit automatique, idempotent et anti-auto-attribution quand un modérateur valide/met en avant une contribution.

**Independent Test** : valider une contribution Codimoi en modération → le solde de l'auteur augmente (+2), une ligne apparaît dans `engagement.mouvement_points` ; re-valider ne recrédite pas (idempotence) ; auto-validation ne crédite pas.

- [ ] T013 [US1] Câbler `engagement::attribuer("contribution_validee", "codimoi", id, cle)` à la fin de la validation Codimoi réussie dans `uafricas_backend/src/handlers/admin/codimoi_admin.rs`, avec **garde anti-auto-attribution** (bénéficiaire = auteur ≠ modérateur, FR-009) ; audit existant inchangé.
- [ ] T014 [P] [US1] Câbler `attribuer("contribution_validee", "video", piste_id, cle)` dans `changer_etat_piste` (passage à `publie`) de `uafricas_backend/src/handlers/admin/vidafrica.rs` (garde auteur≠acteur).
- [ ] T015 [P] [US1] Câbler `attribuer("contribution_validee", "ideaforce"|"bad_habit", id, cle)` aux points de validation Ideaforces et BadGoodHabit dans `uafricas_backend/src/handlers/admin/gouvernance.rs`.
- [ ] T016 [P] [US1] Câbler `attribuer("contribution_mise_en_avant", …)` au(x) flag(s) « mise en avant / vedette » de contribution (Codimoi/Gouvernance selon le champ existant) dans le handler admin concerné.
- [ ] T017 [US1] Vérifier `cargo check` + dérouler S1/S2 de [quickstart.md](./quickstart.md) (crédit, idempotence, anti-auto-attribution, mise en avant).

**Checkpoint** : la boucle « validation → points → journal » fonctionne de bout en bout (MVP moteur).

---

## Phase 4: User Story 2 : Consulter mes points, mon statut, mes badges (Priority: P1)

**Goal** : rendre les points perceptibles, vue profil + badge public.

**Independent Test** : avec un membre ayant des mouvements en base, `GET /api/engagement/mon-compte` renvoie solde/mensuel/réputation/niveau ; l'onglet « Mes points » affiche l'historique ; le badge apparaît sur le profil public sans exposer le journal.

- [ ] T018 [US2] Implémenter les endpoints publics `GET /mon-compte`, `GET /mon-journal` (paginé, réutilise `listerPagine`), `GET /niveau/{utilisateur_id}` (léger, badge seul) dans `uafricas_backend/src/handlers/engagement.rs` (dépend de T010, T012).
- [ ] T019 [P] [US2] Créer le composable public `useEngagement` (`obtenirMonCompte`, `listerMonJournal`, `obtenirNiveau`, types `CompteEngagement`/`MouvementPoints`/`Niveau`) dans `uafricas_frontend/app/composables/useEngagement.ts`.
- [ ] T020 [P] [US2] Créer le composant Tailwind pur `BadgeStatut.vue` (badge niveau réutilisable, icône FontAwesome, prop `utilisateurId` ou `niveau`) dans `uafricas_frontend/app/components/engagement/BadgeStatut.vue`.
- [ ] T021 [US2] Créer le composant Tailwind pur `MesPointsPanel.vue` (solde global/mensuel, réputation, badge, barre vers le prochain niveau, historique paginé) dans `uafricas_frontend/app/components/engagement/MesPointsPanel.vue` (dépend de T019, T020).
- [ ] T022 [US2] Ajouter l'onglet **« Mes points »** dans le dropdown d'onglets de `uafricas_frontend/app/pages/mon-compte/profil.vue` montant `MesPointsPanel` (dépend de T021).
- [ ] T023 [P] [US2] Afficher `BadgeStatut` sur le profil public `uafricas_frontend/app/pages/profil/[id].vue` (et sous les contenus si l'auteur est exposé) via `GET /niveau/{id}` (dépend de T020).
- [ ] T024 [P] [US2] Ajouter les icônes de badge nécessaires (`star`, `crown`, …) au plugin `uafricas_frontend/app/plugins/fontawesome.ts` si absentes.

**Checkpoint** : US1 + US2 livrables ensemble = MVP complet visible par l'utilisateur.

---

## Phase 5: User Story 3 : Points de popularité (paliers de likes) (Priority: P2)

**Goal** : récompenser l'auteur au franchissement de paliers de « j'aime », une seule fois par palier.

**Independent Test** : faire franchir 100 likes à une publication → auteur +10 une fois ; osciller autour de 100 → aucun gain ; 500 → +30 seulement ; auto-like non compté.

- [ ] T025 [US3] Câbler `engagement::evaluer_popularite("codimoi", id, auteur, count)` après l'ajout d'un like dans le handler de réaction Codimoi (`uafricas_backend/src/handlers/codimoi.rs`), en excluant l'auto-like du `count` (FR-017).
- [ ] T026 [P] [US3] Idem pour les réactions FactCheck dans `uafricas_backend/src/handlers/gouvernance.rs` (`type_objet="factcheck"`).
- [ ] T027 [P] [US3] Idem pour les réactions bibliothèque humaine dans `uafricas_backend/src/handlers/bibliotheques_humaines.rs` (`type_objet="biblio_humaine"`).
- [ ] T028 [P] [US3] Idem pour les réactions VidAfrica dans `uafricas_backend/src/handlers/vidafrica_contribution.rs` (`type_objet="video"`).
- [ ] T029 [P] [US3] Idem pour les réactions de fiche pays dans `uafricas_backend/src/handlers/fiche_pays_social.rs` (`type_objet="fiche_pays"`).
- [ ] T030 [US3] Vérifier `cargo check` + dérouler S4 de [quickstart.md](./quickstart.md) (franchissement unique, oscillation sans doublon, palier supérieur).

**Checkpoint** : popularité opérationnelle sur tous les types de contenus « likables ».

---

## Phase 6: User Story 4 : Points/réputation via FactCheck (Priority: P2)

**Goal** : gain sur factcheck validé, malus points + réputation sur factcheck faux.

**Independent Test** : factcheck jugé correct → +3 pts / +1 réputation ; jugé faux → −2 pts (plancher 0) / −3 réputation.

- [ ] T031 [US4] Câbler `attribuer("factcheck_valide", "factcheck", id, cle)` quand un modérateur juge un factcheck correct/validé, et `retirer("factcheck_faux", …)` quand il le juge faux/abusif, dans `uafricas_backend/src/handlers/admin/gouvernance.rs` (mapping sur `verdict` + état de publication ; garde auteur≠modérateur).
- [ ] T032 [US4] Vérifier `cargo check` + dérouler S3 de [quickstart.md](./quickstart.md) (gain, malus, plancher solde, réputation indépendante).

**Checkpoint** : dimension négative + réputation en place.

---

## Phase 7: User Story 5 : Administration du barème & journal (Priority: P2)

**Goal** : configurer le barème sans redéploiement et auditer les points.

**Independent Test** : modifier `contribution_validee.points` en back-office → la validation suivante applique le nouveau montant (< 2 min, sans redéploiement) ; le journal global filtrable liste les mouvements ; un ajustement manuel est tracé.

- [ ] T033 [US5] Implémenter les endpoints admin (perm `engagement`) dans `uafricas_backend/src/handlers/admin/engagement.rs` : `GET/PUT regles`, `GET/POST/PUT/DELETE paliers`, `GET/PUT niveaux`, `GET journal` (filtres membre/type/période, paginé), `POST ajustement` (mouvement `ajustement_admin`), chaque mutation **auditée** via `audit::log_action` (schema `engagement`, Principe VII) (dépend de T011, T012).
- [ ] T034 [P] [US5] Créer le composable admin `useAdminEngagement` (base `useAdmin` : `listerReglesPagine`/`modifierRegle`, CRUD paliers, `modifierNiveau`, `listerJournal`, `ajuster`) dans `uafricas_frontend/app/composables/useAdminEngagement.ts`.
- [ ] T035 [P] [US5] Créer `ReglesBaremeTable.vue` (daisyUI : édition règles + paliers + niveaux) dans `uafricas_frontend/app/components/admin/engagement/ReglesBaremeTable.vue`.
- [ ] T036 [P] [US5] Créer `JournalPointsTable.vue` (daisyUI : journal filtrable + ajustement manuel) dans `uafricas_frontend/app/components/admin/engagement/JournalPointsTable.vue`.
- [ ] T037 [US5] Créer les pages admin `uafricas_frontend/app/pages/admin/engagement/regles.vue` et `journal.vue` (daisyUI) montant les composants ci-dessus (dépend de T034, T035, T036).
- [ ] T038 [P] [US5] Ajouter les entrées de sidebar admin « Engagement » (Barème + Journal, icône FontAwesome) dans le composant de navigation admin.
- [ ] T039 [US5] Vérifier `cargo check` + dérouler S6/S7/S8 de [quickstart.md](./quickstart.md) (plafond/écrêtage, barème à chaud, ajustement + audit).

**Checkpoint** : barème exploitable et auditable sans intervention technique.

---

## Phase 8: Polish & transverse

- [ ] T040 [P] Vérifier les diagnostics (`getDiagnostics`) sur tous les fichiers Rust et Vue modifiés ; corriger warnings/erreurs.
- [ ] T041 [P] Mettre à jour `CLAUDE.md` (section « Recent Changes ») avec un résumé de la feature engagement (schéma `engagement`, service non-bloquant, barème paramétrable, endpoints public/admin).
- [ ] T042 Dérouler l'intégralité de [quickstart.md](./quickstart.md) (S1→S8) comme validation finale des critères SC-001 à SC-007.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Ph.1)** : démarrage immédiat. T002 dépend de T001.
- **Foundational (Ph.2)** : dépend de Setup. **Bloque toutes les US.** Ordre interne : T005 → T006 → T007 → (T008, T009) ; T010/T011 en //; T012 après T004.
- **US1 (Ph.3)** & **US2 (Ph.4)** : P1 : après Foundational. US2 (lecture) est indépendante de US1 (peut se tester avec des mouvements semés) mais ensemble = MVP.
- **US3 (Ph.5)**, **US4 (Ph.6)**, **US5 (Ph.7)**, P2 : après Foundational, indépendantes entre elles.
- **Polish (Ph.8)** : après les US visées.

### User Story Dependencies

- US1 : Foundational (T007) uniquement.
- US2 : Foundational (T010, T012). Indépendante de US1.
- US3 : Foundational (T009). Indépendante.
- US4 : Foundational (T007, T008). Indépendante.
- US5 : Foundational (T011, T012). Indépendante.

### Parallel Opportunities

- Setup : T001, T003, T004 en // (T002 après T001).
- Foundational : T010 + T011 en //.
- US1 : T014, T015, T016 en // (fichiers différents) après le pattern posé par T013.
- US2 : T019, T020, T024 en // ; T023 en // de T022.
- US3 : T026, T027, T028, T029 en // après T025.
- US5 : T034, T035, T036, T038 en //.
- Une fois Foundational terminé, US1–US5 peuvent être menées en parallèle par plusieurs développeurs.

---

## Implementation Strategy

### MVP (US1 + US2)

1. Phase 1 Setup → Phase 2 Foundational (noyau moteur).
2. Phase 3 US1 (crédit sur validation) → **valider S1/S2**.
3. Phase 4 US2 (vue « Mes points » + badge) → **valider S5**.
4. Démo : un membre voit ses points monter et son badge changer. **STOP & VALIDATE.**

### Incrémental

- + US3 (popularité) → valider S4.
- + US4 (factcheck +/−) → valider S3.
- + US5 (admin barème + journal) → valider S6/S7/S8.
- Chaque story ajoute de la valeur sans casser les précédentes.

---

## Notes

- `[P]` = fichiers différents, pas de dépendance sur une tâche incomplète.
- Chaque câblage de call-site (US1/US3/US4) est **non-bloquant** : ne jamais faire échouer l'action métier (validation, like, jugement) si le moteur échoue (FR-007, SC-003).
- Idempotence garantie par `UNIQUE(cle_idempotence)`, pas de vérification applicative concurrente.
- Aucune table existante n'est modifiée : seuls des appels sont ajoutés.
- Commit par tâche ou groupe logique, message en français (Principe I).
