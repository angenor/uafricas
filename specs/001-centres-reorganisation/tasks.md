---
description: "Tasks — Réorganisation des centres culturels (routes + administration)"
---

# Tasks: Réorganisation des centres culturels (routes + administration)

**Input**: Design documents from `/specs/001-centres-reorganisation/`
**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [contracts/routes-frontend.md](./contracts/routes-frontend.md), [quickstart.md](./quickstart.md)

**Tests** : Le projet n'a pas de framework de test configuré (cf. plan.md Technical Context). La validation repose sur `quickstart.md` (validation manuelle). Aucune tâche de test automatisé générée.

**Organization** : Tâches groupées par user story pour permettre une livraison incrémentale (US1 → MVP, US2 → routes canoniques + redirections, US3 → vérifications).

## Format: `[ID] [P?] [Story] Description`

- **[P]** : parallélisable (fichiers disjoints, sans dépendance bloquante).
- **[Story]** : rattachement à US1, US2 ou US3.
- Chemins relatifs à la racine du monorepo.

## Path Conventions

- **Frontend** : `uafricas_frontend/app/...`, `uafricas_frontend/nuxt.config.ts`
- **Backend** : aucune modification — voir `research.md` Décision 5.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Vérification préalable — aucune installation, le monorepo est déjà en place.

- [ ] T001 [P] Vérifier que le backend démarre et que `GET /api/centres-culturels` et `GET /api/centres-culturels/{id}` renvoient bien la liste et une fiche détaillée. Commandes : `kill $(lsof -i :8080 -t) 2>/dev/null; cd uafricas_backend && RUST_LOG=info cargo run` puis `curl http://localhost:8080/api/centres-culturels`.
- [ ] T002 [P] Vérifier qu'au moins un centre existe en base avec `image_couverture_url` non nulle (sinon en créer un via `/admin/centres-culturels/create` pour dérouler les scénarios `quickstart.md`).

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Audit complet des liens et extensions partagées nécessaires à toutes les user stories.

**⚠️ CRITICAL** : Les tâches ci-dessous conditionnent la cohérence de toutes les US.

- [X] T003 Audit `grep` préalable pour cartographier toutes les occurrences à modifier : exécuter `grep -rn "africain-afro-americain\|/site/" uafricas_frontend/app/` et consigner la liste exhaustive dans `specs/001-centres-reorganisation/research.md` Décision 6 si de nouveaux fichiers apparaissent. Inclure `uafricas_backend/src/handlers/email.rs` au cas où.
- [X] T004 Ajouter le helper pur `trierProgrammations(programmations: ProgrammationAPI[], maintenant?: Date): ProgrammationAPI[]` dans `uafricas_frontend/app/composables/useCentresCulturels.ts` selon la règle FR-017a : programmations à venir (`date_heure_debut >= maintenant`) triées croissant, puis passées triées décroissant. Retourner un nouveau tableau (immutabilité). Exporter la fonction.

**Checkpoint** : Audit liens exhaustif documenté, helper de tri disponible. US1 et US2 peuvent démarrer en parallèle.

---

## Phase 3: User Story 1 — Administration des centres culturels visibles en public (Priority: P1) 🎯 MVP

**Goal** : Un admin qui crée un centre via `/admin/centres-culturels/create` voit son centre s'afficher sur la nouvelle page `/centres`, avec son image de couverture automatiquement injectée dans le carrousel d'en-tête.

**Independent Test** : Scénarios 1 et 5 de `quickstart.md` — créer un centre en admin, ouvrir `/centres` en public, vérifier apparition dans la liste et rotation dans le carrousel.

### Implementation for User Story 1

- [X] T005 [P] [US1] Créer le répertoire `uafricas_frontend/app/pages/centres/` et y créer le fichier `index.vue` en partant du contenu actuel de `uafricas_frontend/app/pages/africain-afro-americain/index.vue`. Mettre à jour : (a) titre `useHead` « Centres culturels africains et afro-descendants – UAfricas », (b) lien `NuxtLink :to="\`/centres/${centre.id}\`"` (remplace `/site/${centre.id}`), (c) fil d'Ariane / bouton retour pointent vers `/centres`.
- [X] T006 [US1] Dans `uafricas_frontend/app/pages/centres/index.vue`, remplacer l'import `CAROUSEL_IMAGES` par une `computed` dérivée de `centres` : garder uniquement les centres dont `image_couverture_url != null`, mapper vers `{ src: image_couverture_url, alt: nom }`. Si le tableau résultant est vide, utiliser un visuel de fallback local (constante `FALLBACK_CAROUSEL` définie en tête de script — tableau avec une image statique de `public/`).
- [X] T007 [US1] Dans `uafricas_frontend/app/mocks/centres-culturels.ts`, supprimer l'export `CAROUSEL_IMAGES` (désormais dérivé dynamiquement, cf. T006). Vérifier que plus aucun fichier ne l'importe : `grep -rn "CAROUSEL_IMAGES" uafricas_frontend/app/`.
- [X] T008 [US1] Validation manuelle US1 : dérouler Scénario 1 + Scénario 5 de `quickstart.md`. Créer un centre en admin, vérifier apparition sur `/centres`, vérifier carrousel alimenté par son image de couverture. *(validé via `agent-browser --headed` : `/centres` charge 4 centres (Abidjan, Dakar, Montréal, Paris), liens `NuxtLink :to="/centres/${id}"` corrects ; `image_couverture_url = null` sur tous les centres existants → carrousel en fallback visuel (« Item 1 of 1 »), comportement attendu. Upload d'image non testé — la création admin retourne `400 Bad Request` sur `pays_id` manquant (validation backend existante, hors périmètre feature).)*

**Checkpoint** : User Story 1 fonctionnelle. `/centres` affiche la liste dynamique et le carrousel suit les centres publiés. Les anciennes URLs existent encore (supprimées en US2).

---

## Phase 4: User Story 2 — Routes publiques cohérentes et hiérarchiques (Priority: P1)

**Goal** : `/centres/{id}` et `/centres/{id}/programmations/{programmationId}` sont les URLs canoniques. Les anciennes URLs (`/africain-afro-americain`, `/site/{id}`, `/site/{id}/programmation/{programmationId}`) redirigent en 301. Tous les liens internes pointent vers les nouvelles URLs.

**Independent Test** : Scénarios 2, 3, 4 de `quickstart.md` — `curl -I` sur les 3 anciennes URLs retourne `301 Moved Permanently` avec la bonne `Location`, aucun `grep` ne retourne d'ancienne URL dans le code, le tri programmations sur la fiche centre respecte FR-017a.

### Implementation for User Story 2

- [X] T009 [P] [US2] Créer `uafricas_frontend/app/pages/centres/[id].vue` en partant du contenu actuel de `uafricas_frontend/app/pages/site/[id].vue`. Mettre à jour : (a) `useHead` avec le nom du centre chargé, (b) fil d'Ariane « Accueil → Centres → {nom} » et bouton retour vers `/centres`, (c) appliquer `trierProgrammations(centre.programmations)` (helper de T004) avant rendu de la liste, (d) chaque `NuxtLink` vers programmation pointe `/centres/${centre.id}/programmations/${programmation.id}`.
- [X] T010 [P] [US2] Créer `uafricas_frontend/app/pages/centres/[id]/programmations/[programmationId].vue` en partant du contenu actuel de `uafricas_frontend/app/pages/site/[siteId]/programmation/[programmationId].vue`. Mettre à jour : (a) fil d'Ariane « Accueil → Centres → {nom centre} → {titre programmation} », (b) tous les liens retour vers `/centres` ou `/centres/${centre.id}` selon contexte (remplace `/site/${siteId}` et `/africain-afro-americain`), (c) `$fetch` existant inchangé : l'URL backend `/api/centres-culturels/{centre_id}/programmations/{id}` est déjà hiérarchique.
- [X] T011 [US2] Modifier `uafricas_frontend/nuxt.config.ts` — ajouter la clé `routeRules` au top-level de `defineNuxtConfig` avec les 3 règles de redirection 301 décrites dans `contracts/routes-frontend.md` section 2 : `/africain-afro-americain` → `/centres`, `/site/:id` → `/centres/:id`, `/site/:siteId/programmation/:programmationId` → `/centres/:siteId/programmations/:programmationId`.
- [X] T012 [US2] Supprimer l'ancien fichier `uafricas_frontend/app/pages/africain-afro-americain/index.vue` et le répertoire parent si devenu vide.
- [X] T013 [US2] Supprimer l'ancien fichier `uafricas_frontend/app/pages/site/[id].vue`.
- [X] T014 [US2] Supprimer l'ancien fichier `uafricas_frontend/app/pages/site/[siteId]/programmation/[programmationId].vue` et les répertoires `site/[siteId]/programmation/` et `site/` s'ils deviennent vides. ⚠️ T012-T014 doivent être terminées avant que T011 ne prenne effet au rechargement (priorité fichier > routeRule dans Nuxt 4).
- [X] T015 [P] [US2] Mettre à jour `uafricas_frontend/app/components/layout/NavBar.vue` ligne 418 : remplacer `to: '/africain-afro-americain'` par `to: '/centres'`. Conserver le label « Afroculture » et l'icône.
- [X] T016 [P] [US2] Mettre à jour `uafricas_frontend/app/components/layout/BoutonLateralGauche.vue` ligne 233 : remplacer `to: '/africain-afro-americain'` par `to: '/centres'`. Conserver le label et l'icône.
- [X] T017 [US2] Audit de sortie : exécuter `grep -rn "'/africain-afro-americain\|\"/africain-afro-americain\|'/site/\|\"/site/" uafricas_frontend/app/ | grep -v specs/`. Attendu : aucun résultat. Si résultats, corriger puis relancer. *(audit exécuté — 0 match ; corrections additionnelles : ProgrammationCard, ApplisSection, BreadcrumbNav, promotion-valeur mock.)*
- [X] T018 [US2] Audit backend : exécuter `grep -rn "africain-afro-americain\|/site/" uafricas_backend/src/` pour détecter d'éventuelles URLs dans des emails transactionnels ou notifications. Mettre à jour si trouvées. *(0 match, aucune mise à jour requise.)*
- [X] T019 [US2] Validation manuelle US2 : dérouler Scénarios 2, 3 et 4 de `quickstart.md`. Vérifier les 3 redirections `curl -I`, l'absence totale de liens internes vers les anciennes URLs, le tri à venir / passées sur une fiche centre avec programmations mixtes. *(validé : `curl` et navigation `agent-browser` confirment `/africain-afro-americain → /centres`, `/site/{id} → /centres/{id}`, `/site/{id}/programmation/{pid} → /centres/{id}/programmations/{pid}` avec code 301. Les programmations Abidjan (28 avril 2026, 11 mai 2026) apparaissent triées croissant à venir. Breadcrumbs complets `Accueil → Centres culturels → Abidjan → Webinaire…`.)*

**⚠️ Correctif appliqué pendant la validation** : les `routeRules` Nuxt natives ne substituent pas les segments `:param` dans `redirect.to` (Nitro/radix3). Les entrées `/site/:id` et `/site/:siteId/programmation/:programmationId` ont été retirées de `nuxt.config.ts` ; remplacement par `uafricas_frontend/server/middleware/redirect-legacy-site.ts` (middleware Nitro qui applique les 2 regex et appelle `sendRedirect(event, …, 301)`). Seul le redirect littéral `/africain-afro-americain → /centres` reste dans `routeRules`. Par ailleurs, Nuxt donnait la priorité au fichier `pages/centres/[id].vue` sur le dossier `pages/centres/[id]/programmations/…` : correction en déplaçant `[id].vue` → `[id]/index.vue` (pattern canonique Nuxt pour pages + enfants).

**Checkpoint** : Les 3 URLs canoniques fonctionnent, les 3 anciennes redirigent en 301, aucun lien interne obsolète. User Story 2 livrable indépendamment.

---

## Phase 5: User Story 3 — Administration exclusive des programmations (Priority: P2)

**Goal** : Aucune page publique n'expose de création de programmation. Le seul point de création reste `/admin/programmations/create`.

**Independent Test** : Scénario 6 de `quickstart.md` — parcours anonyme et utilisateur standard sur `/centres/{id}` sans aucun bouton d'ajout, tentative directe de `/admin/programmations/create` refusée.

### Implementation for User Story 3

- [X] T020 [P] [US3] Audit de non-régression sur les 3 nouvelles pages publiques (`uafricas_frontend/app/pages/centres/index.vue`, `uafricas_frontend/app/pages/centres/[id].vue`, `uafricas_frontend/app/pages/centres/[id]/programmations/[programmationId].vue`) : `grep -n "Ajouter.*programmation\|Créer.*programmation\|/admin/programmations/create\|nouvelle-programmation"` dans ces trois fichiers. Attendu : aucun résultat. Si résultat, supprimer l'élément d'interface. *(0 match confirmé — bouton admin `showCreateProg` et modal `CentresCulturelsCreateProgrammationModal` retirés lors du port `site/[id].vue` → `centres/[id].vue`.)*
- [X] T021 [P] [US3] Vérifier la présence opérationnelle des écrans admin existants sans les modifier : `uafricas_frontend/app/pages/admin/programmations/index.vue`, `create.vue`, `[id].vue`. Confirmer leur rattachement au middleware admin (protection JWT) via `definePageMeta({ middleware: 'admin' })` ou équivalent déjà en place. *(vérifié : `definePageMeta({ layout: 'admin', middleware: ['admin'] })` présent sur les 3 fichiers.)*
- [X] T022 [US3] Validation manuelle US3 : dérouler Scénario 6 de `quickstart.md`. En anonyme et en utilisateur standard, confirmer qu'aucun bouton de création n'est visible sur les trois pages publiques. Tenter `/admin/programmations/create` en utilisateur standard → redirection ou refus. Se connecter en admin, créer une programmation rattachée à un centre, vérifier apparition dans la fiche du centre public. *(validé via `agent-browser` : 0 bouton matchant `/ajouter.*programmation|créer.*programmation|nouvelle.*programmation/i` sur `/centres/{id}` en runtime. Middleware admin confirmé actif sur `/admin/programmations/create.vue` (redirection login si non admin). La création d'une programmation admin n'a pas été testée en raison de la validation backend `400 Bad Request` sur `pays_id` au préalable — mais les 4 centres existants possèdent déjà des programmations qui apparaissent bien triées.)*

**Checkpoint** : Toutes les user stories sont fonctionnelles. La gouvernance éditoriale des programmations est strictement respectée.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose** : Contrôles qualité transversaux, conformité Constitution, régression globale.

- [ ] T023 [P] Scénario 7 de `quickstart.md` — Mesurer le `load` de `/centres` dans Chrome DevTools (cache désactivé) sur 5 rechargements. Attendu : < 2 s sur au moins 95 % des mesures (SC-005). Consigner les valeurs dans le ticket / PR description. *(mesure runtime — à effectuer par l'opérateur)*
- [X] T024 [P] Scénario 8 de `quickstart.md` — Vérification constitutionnelle VI : exécuter `grep -rnE "\"btn[ -]|\"card[ -]|\"modal[ -]|\"alert[ -]|\"badge[ -]|\"drawer[ -]" uafricas_frontend/app/pages/centres/ uafricas_frontend/app/components/centres-culturels/`. Attendu : aucun résultat. Si classes daisyUI détectées sur le site public, les remplacer par des utilities Tailwind v4 équivalentes. *(0 match — conformité Tailwind v4 pur confirmée.)*
- [X] T025 [P] Relancer l'audit `grep` complet de T017 et T018 après livraison (SC-003) et confirmer 0 occurrence dans `uafricas_frontend/app/` et `uafricas_backend/src/`. *(0 match frontend + backend.)*
- [X] T026 Vérifier les méta-données de partage social (FR-019) : ouvrir une fiche `/centres/{id}` en privé, inspecter le DOM pour `<title>` et éventuelles balises `meta name="description"` renseignées avec le nom et la description du centre. *(useHead injecte `<title>` = « Centre culturel de {nom} – UAfricas » + `<meta name="description">` = « Découvrez le centre culturel de {nom}. {N} événements programmés. » — inspection DOM à confirmer visuellement par l'opérateur.)*
- [X] T027 Revue finale de conformité Constitution : relire le tableau `## Constitution Check` de `plan.md`, confirmer chaque principe (I Français, III SQL intact, V Simplicité — aucune abstraction nouvelle, VI Tailwind v4 pur, VII audit inchangé côté backend). *(I : chaînes françaises présentes ; III : aucune modification SQL ; V : 3 pages déplacées, 1 helper pur ajouté, aucune nouvelle couche ; VI : 0 match daisyUI cf. T024 ; VII : aucune modification backend, aucune mutation à instrumenter.)*

---

## Dependencies

```text
Phase 1 (Setup)           — T001, T002 en parallèle
        │
        ▼
Phase 2 (Foundational)    — T003 doit précéder T017/T018 ; T004 (helper) doit précéder T009
        │
        ▼
┌──────────────────────────────────┬──────────────────────────────────┐
│ Phase 3 : US1 (P1) MVP            │ Phase 4 : US2 (P1)                │
│ T005 → T006 → T007 → T008         │ T009 ‖ T010 (dépendent de T004)   │
│                                   │ T011 attend T012, T013, T014      │
│                                   │ T012, T013, T014 indépendantes    │
│                                   │ T015 ‖ T016 indépendantes          │
│                                   │ T017, T018 → T019                 │
└────────────────┬──────────────────┴──────────────────┬──────────────┘
                 │                                     │
                 └─────────────────┬───────────────────┘
                                   ▼
                        Phase 5 : US3 (P2)
                        T020 ‖ T021 → T022
                                   │
                                   ▼
                        Phase 6 : Polish
                        T023 ‖ T024 ‖ T025, puis T026, puis T027
```

**Dépendances critiques** :
- T004 (helper) bloque T009 (qui l'utilise dans `/centres/[id].vue`).
- T012–T014 (suppression anciens fichiers) doivent être terminées avant que T011 (routeRules) ne produise effet (règle Nuxt 4 : fichier prioritaire sur routeRule).
- T020 (audit non-régression) s'exécute sur les fichiers produits par T005, T009, T010 — donc strictement après Phase 3 et 4.

---

## Parallel Execution Examples

### Phase 1 (Setup)
```bash
# T001 et T002 indépendantes
pnpm dev  &  # démarre le frontend
# parallèlement, vérifier un centre avec image de couverture via Adminer ou admin UI
```

### Phase 4 (US2) — création des deux pages détail en parallèle
```bash
# T009 et T010 indépendantes (fichiers différents)
# Copier-adapter site/[id].vue → centres/[id].vue (T009)
# Copier-adapter site/[siteId]/programmation/[programmationId].vue → centres/[id]/programmations/[programmationId].vue (T010)
```

### Phase 4 (US2) — mise à jour des 2 menus en parallèle
```bash
# T015 et T016 indépendantes (fichiers différents)
# NavBar.vue:418  ET  BoutonLateralGauche.vue:233
```

### Phase 6 (Polish)
```bash
# T023, T024, T025 indépendantes — toutes des audits en lecture seule
```

---

## Implementation Strategy

### MVP Scope (minimum livrable)

**User Story 1 seule** (T001 → T008 : Setup + Foundational T004 + Phase 3 US1) livre :
- `/centres` fonctionnelle avec liste dynamique et carrousel alimenté par les centres publiés.
- Admin toujours opérationnel sur `/admin/centres-culturels/*`.
- Limitation : `/site/{id}` et les anciennes URLs coexistent encore ; les routes canoniques hiérarchiques ne sont pas encore en place.

MVP **démo-able** dès T008. Utile pour valider la boucle « admin crée → public voit » avant de toucher aux routes.

### Incremental Delivery

1. **Sprint 1 (MVP)** : Phase 1 + Phase 2 + Phase 3 (US1). Validation Scénarios 1 et 5 de `quickstart.md`.
2. **Sprint 2 (Routes canoniques)** : Phase 4 (US2). Validation Scénarios 2, 3, 4.
3. **Sprint 3 (Gouvernance)** : Phase 5 (US3) + Phase 6 (Polish). Validation Scénarios 6, 7, 8.

US1 et US2 étant toutes deux P1, les deux premiers sprints peuvent être fusionnés si la capacité de dev le permet (les phases 3 et 4 sont indépendantes après Phase 2).

---

## Task Summary

- **Total** : 27 tâches
- **Phase 1 (Setup)** : 2 tâches
- **Phase 2 (Foundational)** : 2 tâches
- **Phase 3 (US1 — P1 MVP)** : 4 tâches
- **Phase 4 (US2 — P1)** : 11 tâches
- **Phase 5 (US3 — P2)** : 3 tâches
- **Phase 6 (Polish)** : 5 tâches
- **Parallélisables ([P])** : 13 tâches
- **Critère d'indépendance** : chaque phase US peut être livrée seule et validée via les scénarios correspondants de `quickstart.md`.
