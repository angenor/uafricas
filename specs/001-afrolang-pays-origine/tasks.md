---
description: "Tâches d'implémentation — Pays d'origine des salles publiques Afrolang"
---

# Tasks: Pays d'origine des salles publiques Afrolang

**Input**: Design documents from `/specs/001-afrolang-pays-origine/`
**Prerequisites**: plan.md, spec.md (US1, US2, US3), research.md, data-model.md, contracts/api-public.md, contracts/api-admin.md, quickstart.md

**Tests** : aucune génération de tests automatisés (le projet n'a pas encore de framework de tests configuré, cf. CLAUDE.md « No linting, testing, or CI/CD configured yet »). Validation = quickstart manuel.

**Organisation** : par user story pour livraison MVP indépendante. **MVP recommandé = US1 + US3** (P1 conjoints) ; US2 (P2) ajoutable indépendamment ensuite.

## Format: `[ID] [P?] [Story] Description`

- **[P]** : tâches parallélisables (fichiers indépendants, pas de dépendance bloquante)
- **[Story]** : US1 / US2 / US3 — absent pour Setup / Foundational / Polish

---

## Phase 1: Setup

**Purpose**: Aucune initialisation projet — monorepo existant. Cette feature ne crée pas de nouveau module.

- [X] T001 Vérifier que la branche `001-afrolang-pays-origine` est active et à jour (`git status` propre, `git pull origin main` si nécessaire) — pas de fichier à créer.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Modifications transverses qui DOIVENT être appliquées avant toute story (DDL + types Rust/TS partagés).

- [X] T002 Ajouter le DDL `CREATE TABLE afrolang.salle_pays_origine` (PK composite, 2 FK CASCADE, index `idx_afrolang_salle_pays_origine_pays`, COMMENT) à la fin de la section salles dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql` — DDL exact dans `data-model.md` § « DDL complet ».
- [X] T003 Recréer la base locale pour appliquer la migration : `docker compose down -v && docker compose up -d` puis vérifier `\d afrolang.salle_pays_origine` dans psql (cf. quickstart § 1).
- [X] T004 [P] Ajouter la struct `PaysOrigineLight { id, nom, code_iso2 }` (Serialize + Deserialize + Clone) dans `uafricas_backend/src/models/afrolang.rs`, et ajouter le champ `pub pays_origine: Vec<PaysOrigineLight>` aux structs `SalleResponse` et `SalleDetailResponse`.
- [X] T005 [P] Ajouter dans `uafricas_backend/src/models/afrolang.rs` la déserialisation côté `SalleRow` : champ `pays_origine_json: sqlx::types::Json<Vec<PaysOrigineLight>>` (ou `serde_json::Value`) + mapping dans `From<SalleRow> for SalleResponse` qui aplatit en `Vec<PaysOrigineLight>`.
- [X] T006 [P] Ajouter dans `uafricas_backend/src/handlers/afrolang.rs` (struct `SalleFiltres`) le champ `pub pays_id: Option<Uuid>`.
- [X] T007 [P] Ajouter dans `uafricas_frontend/app/composables/useAfrolang.ts` l'interface `PaysOrigineLight { id: string; nom: string; code_iso2: string | null }`, puis ajouter `pays_origine: PaysOrigineLight[]` à `SalleAPI` et `SalleDetailAPI`, et `pays_id?: string` à `SalleFiltres`.

**Checkpoint** : la BD a la table, les types Rust/TS reflètent le nouveau champ, mais aucune logique métier n'est encore branchée. Le backend doit toujours compiler (`cargo check`) — `pays_origine` retournera `[]` par défaut tant que T009 n'est pas fait.

---

## Phase 3: User Story 1 — Voir les pays d'origine sur l'annuaire (P1)

**Goal** : Sur `/afrolang`, chaque carte de salle publique affiche ses pays d'origine selon la règle 1-3 vs 4+.

**Independent test** : appliquer T002–T007 + cette phase, puis insérer manuellement 1 ligne dans `afrolang.salle_pays_origine` (`INSERT INTO afrolang.salle_pays_origine (salle_id, pays_id) VALUES (...)`), recharger `/afrolang` → la carte concernée affiche le pays. Couvre quickstart §§ 3-5.

- [X] T008 [US1] Étendre la sous-requête dans `lister_salles` (`uafricas_backend/src/handlers/afrolang.rs`) : ajouter au SELECT enrichi le bloc `COALESCE((SELECT json_agg(json_build_object('id', p.id, 'nom', p.nom, 'code_iso2', p.code_iso2) ORDER BY p.nom) FROM afrolang.salle_pays_origine spo JOIN shared.pays p ON p.id = spo.pays_id WHERE spo.salle_id = s.id AND p.actif = TRUE), '[]'::json) AS pays_origine`. Mettre à jour `SALLE_COLONNES` ou la SELECT-list inline en cohérence avec la déserialisation T005.
- [X] T009 [US1] Étendre `obtenir_salle` (handler public, `uafricas_backend/src/handlers/afrolang.rs`) à l'identique : ajouter le même `json_agg` (filtré sur `p.actif = TRUE`) au SELECT pour que `SalleDetailResponse.pays_origine` soit alimenté.
- [X] T010 [P] [US1] Modifier `uafricas_frontend/app/components/afrolang/SalleCard.vue` : ajouter un bandeau « Pays d'origine » sous le titre/description. Calculer en `computed` : `paysAffiches` (alias de `salle.pays_origine ?? []`), `modeCompact` (`paysAffiches.length >= 4`), `tooltipPays` (`paysAffiches.map(p => p.nom).join(', ')`). Rendu : si vide → masquer le bloc ; si 1-3 → chips drapeau emoji + nom court ; si ≥4 → rangée de drapeaux seuls avec attribut `:title="tooltipPays"` et `aria-label`. Tailwind v4 pur (Principe VI), pas de daisyUI.
- [X] T011 [P] [US1] Ajouter dans `SalleCard.vue` la fonction utilitaire locale `drapeauEmoji(codeIso2: string | null): string` qui calcule l'emoji régional via `String.fromCodePoint(0x1F1E6 + codeIso2.charCodeAt(0) - 65, 0x1F1E6 + codeIso2.charCodeAt(1) - 65)` ; renvoyer `''` si `codeIso2` invalide ou null (repli gracieux : nom seul).
- [X] T012 [US1] Tester manuellement quickstart §§ 3, 4 et 5 (cas 0 / 1-3 / ≥4 pays) en insérant temporairement des lignes via psql.

**Checkpoint** : US1 livrable — un visiteur voit les pays sur les cartes. Pas encore d'admin UI ni de filtre.

---

## Phase 4: User Story 3 — Gestion admin (ajout/retrait + audit) (P1)

**Goal** : Une admin peut associer / retirer des pays d'origine via le back-office Afrolang ; chaque action est auditée.

**Independent test** : utiliser un client HTTP (curl/Postman) avec un JWT admin pour POST/DELETE puis vérifier la BD et `audit_log`. Quickstart §§ 2, 8, 9, 10.

- [X] T013 [US3] Ajouter dans `uafricas_backend/src/handlers/admin/salles.rs` la struct `AjouterPaysOrigineRequest { pays_id: Uuid }` (Deserialize) et le handler `pub async fn ajouter_pays_origine_salle(...)` calqué sur `ajouter_pays_annonce` : `verifier_permission!(admin, "afrolang", "modifier")`, vérifier existence salle (`SELECT EXISTS ... WHERE deleted_at IS NULL`), vérifier existence pays actif (`WHERE actif = true`), `INSERT INTO afrolang.salle_pays_origine ... ON CONFLICT DO NOTHING`, `audit::log_action("CREATE", "afrolang", "salle_pays_origine", Some(salle_id), ...)`, retour 201 avec `{ salle_id, pays_id }`.
- [X] T014 [US3] Ajouter dans le même fichier le handler `pub async fn retirer_pays_origine_salle(...)` : `verifier_permission!`, `DELETE FROM afrolang.salle_pays_origine WHERE salle_id = $1 AND pays_id = $2`, 404 si `rows_affected = 0`, `audit::log_action("DELETE", ...)`, retour 200 `data: null`.
- [X] T015 [US3] Enregistrer les 2 routes dans `uafricas_backend/src/routes.rs` (scope `/api/admin`, après les routes salles existantes vers la ligne 188) : `.route("/afrolang/salles/{id}/pays", web::post().to(admin::salles::ajouter_pays_origine_salle))` et `.route("/afrolang/salles/{id}/pays/{pays_id}", web::delete().to(admin::salles::retirer_pays_origine_salle))`.
- [X] T016 [US3] Étendre `obtenir_salle` côté admin (`uafricas_backend/src/handlers/admin/salles.rs`) : ajouter le `json_agg` similaire à T009 mais **sans** le filtre `p.actif = TRUE` (cf. contracts/api-admin.md § « Lecture admin ») afin que l'admin voie aussi les pays archivés. Renvoyer dans le détail salle un champ supplémentaire `pays_origine` (réutiliser `PaysOrigineLight` ; ajouter au DTO Response admin si distinct).
- [X] T017 [P] [US3] Ajouter dans `uafricas_frontend/app/composables/useAdminAfrolangSalles.ts` deux fonctions : `ajouterPaysOrigine(salleId: string, paysId: string): Promise<boolean>` (POST, retourne true sur 201) et `retirerPaysOrigine(salleId: string, paysId: string): Promise<boolean>` (DELETE). Réutiliser `adminFetch` existant. Mettre à jour `SalleDetailAPI` côté admin pour inclure `pays_origine: PaysOrigineLight[]`.
- [X] T018 [P] [US3] Ajouter dans la page admin de détail salle (`uafricas_frontend/app/pages/admin/afrolang/salles/[id].vue`) un panneau « Pays d'origine » : liste des chips actuelles (avec bouton X pour retirer), select daisyUI alimenté par `useAdminPays.listerPays()` (composable existant) pour ajouter, indicateur visuel grisé si `pays.actif === false` (mention « archivé »). daisyUI v5 autorisé (back-office, Principe VI).
- [X] T019 [US3] Tester quickstart §§ 2, 7 (mention archivée admin), 8, 9, 10 (permission refusée).

**Checkpoint** : US3 livrable — l'admin enrichit les salles, audit OK, cleanup CASCADE OK. Combiné à US1, l'utilisateur final voit les pays renseignés par l'admin.

---

## Phase 5: User Story 2 — Filtre public par pays (P2)

**Goal** : Un visiteur filtre `/afrolang` par un pays d'origine choisi parmi ceux disponibles.

**Independent test** : avec quelques salles enrichies (issu d'US3), sélectionner un pays dans le filtre → liste réduite, compteur cohérent ; reset → liste complète. Quickstart § 6.

- [X] T020 [US2] Compléter `lister_salles` (`uafricas_backend/src/handlers/afrolang.rs`) : si `params.pays_id` est `Some`, ajouter à `conditions` : `EXISTS (SELECT 1 FROM afrolang.salle_pays_origine spo JOIN shared.pays p ON p.id = spo.pays_id WHERE spo.salle_id = s.id AND spo.pays_id = ${idx} AND p.actif = TRUE)`, push `uuid_binds` + `param_types.push("uuid")` + `bind_index += 1`. Appliquer la même condition au `count_query`.
- [X] T021 [P] [US2] Étendre `useAfrolang.ts` (`uafricas_frontend/app/composables/useAfrolang.ts`) : dans la construction de `URLSearchParams` de `listerSalles`, ajouter `if (filtres.pays_id) params.set('pays_id', filtres.pays_id)`.
- [X] T022 [P] [US2] Ajouter un select « Pays d'origine » dans `uafricas_frontend/app/components/afrolang/SalleFilters.vue` (desktop) : alimenté par un composable de récupération des pays (réutiliser `useAdminPays.listerPays()` filtré sur `actif=true`, ou créer un appel léger vers `GET /api/pays` existant). Liaison `v-model="filtres.pays_id"`. Inclure une option « Tous les pays » (`value=""`). Tailwind v4 pur.
- [X] T023 [P] [US2] Répliquer le même filtre dans `uafricas_frontend/app/components/afrolang/SalleFiltersMobile.vue`.
- [X] T024 [US2] Dans `uafricas_frontend/app/pages/afrolang/index.vue`, étendre `buildApiFiltres` pour propager `filtres.value.pays_id` ; étendre `resetFilters` pour remettre `pays_id: ''` ; ajouter un `watch(() => filtres.value.pays_id, ...)` qui repasse à `currentPage = 1` puis `chargerSalles()` (modèle identique à `langue`).
- [X] T025 [US2] Tester quickstart § 6 : sélection, reset, combinaison avec recherche texte ; vérifier qu'un `pays_id` archivé ou inconnu renvoie 0 résultat (200, liste vide).

**Checkpoint** : US2 livrable — toutes les user stories sont en production.

---

## Phase 6: Polish & Cross-Cutting

- [X] T026 [P] Mettre à jour `CLAUDE.md` (section « Recent Changes ») avec un résumé 1 ligne de la feature 001-afrolang-pays-origine (table N-N + endpoints admin + filtre public).
- [X] T027 [P] Ajouter une entrée correspondante dans `CLAUDE.md` § « Active Technologies » uniquement si nécessaire (la mise à jour automatique a déjà ajouté l'entrée — vérifier qu'il n'y a pas de doublon, sinon nettoyer).
- [X] T028 Mesurer le temps de réponse de `GET /api/afrolang/salles` avant/après (curl + `time`) pour valider SC-004 (≤ 110 % du baseline). Documenter le résultat dans `quickstart.md` § 10.
- [X] T029 Vérifier qu'aucun pays archivé n'apparaît dans la réponse publique (cas Q3) en désactivant temporairement un pays utilisé puis en interrogeant l'API.
- [X] T030 Commit final : `feat(afrolang): pays d'origine N-N pour les salles publiques + filtre public + admin (#001-afrolang-pays-origine)`.

---

## Dependencies

```
T001 (Setup)
  └─▶ T002 (DDL) ─▶ T003 (re-init BD)
        └─▶ T004, T005, T006, T007 [P]   (types Rust + TS)
              ├─▶ Phase 3 (US1) : T008 → T009 → T010 [P] T011 [P] → T012
              ├─▶ Phase 4 (US3) : T013 → T014 → T015 → T016 → T017 [P] T018 [P] → T019
              └─▶ Phase 5 (US2) : T020 → T021 [P] T022 [P] T023 [P] → T024 → T025
                    (US2 nécessite T008 — mêmes fichiers/handler que US1)
                    └─▶ Phase 6 : T026 [P] T027 [P] → T028 → T029 → T030
```

**Story-level dependencies** :
- US1 dépend de Foundational uniquement.
- US3 dépend de Foundational uniquement (peut être fait en parallèle d'US1).
- US2 dépend de Foundational + T008 (la fonction `lister_salles` modifiée par US1).
- US3 ↔ US1 : indépendantes côté code, mais US1 + US3 forment le **MVP cohérent** (afficher des pays nécessite d'en associer).

---

## Parallel Execution Opportunities

**Foundational (T004 → T007)** : 4 fichiers indépendants (3 backend, 1 frontend) modifiables en parallèle.

**US1 — frontend (T010 + T011)** : tout est dans `SalleCard.vue` mais sur deux blocs distincts (template/computed vs fonction utilitaire) — séquentiel recommandé.

**US3 — frontend (T017 + T018)** : composable et page admin dans deux fichiers distincts → parallélisables.

**US2 — frontend (T021 + T022 + T023)** : composable + 2 composants distincts → parallélisables.

**Polish (T026 + T027)** : retouches CLAUDE.md → parallélisables.

---

## Implementation Strategy

### MVP (livraison 1)
**Phases 1 → 4** (Setup + Foundational + US1 + US3) = T001 à T019.
- Valeur livrée : les visiteurs voient les pays d'origine renseignés par les admins.
- 19 tâches, ~1 journée de dev pour 1 développeur.

### Itération 2 (livraison 2)
**Phase 5** (US2) = T020 à T025.
- Valeur ajoutée : filtre public par pays.
- 6 tâches, ~½ journée.

### Polish
**Phase 6** = T026 à T030, à enchaîner avant commit final.

---

## Validation du format

Chaque tâche respecte : `- [ ] T### [P?] [US?] Description avec chemin de fichier`.
Total : **30 tâches** (1 setup + 6 foundational + 5 US1 + 7 US3 + 6 US2 + 5 polish).
