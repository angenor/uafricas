# Phase 0 : Research: Réorganisation des centres culturels

**Feature**: `001-centres-reorganisation` | **Date**: 2026-04-19

Ce document consigne les décisions techniques prises avant l'écriture du data model et des contrats. Aucun marqueur `NEEDS CLARIFICATION` ne subsiste dans la spec (résolus via `/speckit.clarify`, voir section `## Clarifications` de [spec.md](./spec.md)).

---

## Décision 1 : Mécanisme de redirection permanente pour les anciennes URLs

**Decision** : Utiliser `routeRules` de Nuxt 4 dans `nuxt.config.ts`, avec `redirect: { to: '...', statusCode: 301 }`. Trois règles :
- `/africain-afro-americain` → `/centres` (redirection simple)
- `/site/:id` → `/centres/:id` (redirection avec paramètre dynamique)
- `/site/:siteId/programmation/:programmationId` → `/centres/:siteId/programmations/:programmationId` (redirection avec deux paramètres + changement de segment singulier → pluriel)

**Rationale** :
- `routeRules` est le mécanisme natif Nuxt 4 pour les règles statiques de niveau route, appliquées côté serveur SSR (visible aux crawlers SEO) et côté client.
- Le code HTTP 301 est supporté nativement et correspond exactement à la sémantique de « redirection permanente » exigée par FR-009/FR-010/FR-011 et SC-002.
- Le support des paramètres dynamiques via `:param` (ou syntaxe `**` pour catch-all) couvre les deux cas à paramètres.
- Pas besoin de middleware Nuxt (option envisagée), qui serait plus complexe et moins déclaratif pour un simple rewrite d'URL.

**Alternatives considered** :
- **Middleware Nuxt global** (`middleware/redirect-legacy.global.ts`) : plus verbeux, exécuté à chaque navigation, moins performant et contourne la couche Nitro, rejeté.
- **Redirections côté reverse proxy (Nginx)** : efficace mais éclate la configuration entre code et infra, contourne le SSR de Nuxt (moins traçable en dev local), rejeté pour ce refactoring principalement frontend.
- **Pages « stub » qui appellent `navigateTo(..., { redirectCode: 301 })`** : coûte une hydration Vue pour rien, rejeté.

---

## Décision 2 : Carrousel d'en-tête alimenté automatiquement depuis les centres publiés

**Decision** : Dans `/centres/index.vue`, dériver `carouselImages` directement depuis la liste de centres renvoyée par `$fetch<ApiResponse<CentreCulturelAPI[]>>('/api/centres-culturels')`. Filtrer les centres qui possèdent `image_couverture_url != null` et mapper vers le format attendu par `CentresCulturelsCentreCulturelCarousel`. Fallback : si aucun centre publié n'a d'image, afficher un visuel par défaut (constante locale ou image statique servie depuis `public/`).

**Rationale** :
- Clarification Q2 : agrégation automatique, pas de gestion séparée côté admin.
- FR-005a impose que le carrousel reflète les centres publiés, une dérivation `computed` depuis la même source de vérité garantit la cohérence (un seul `$fetch`, pas d'état parallèle).
- Supprime l'import `CAROUSEL_IMAGES` depuis `~/mocks/centres-culturels`, réduisant la dette technique mock.

**Alternatives considered** :
- Garder l'import mock : contredit FR-005a (le carrousel ne refléterait pas les centres admin), rejeté.
- Endpoint backend dédié `/api/centres-culturels/carousel` : complexité inutile (YAGNI, principe V), la liste complète suffit, rejeté.
- Flag `vedette` côté centre : écarté par clarification Q2 (option B non retenue).

---

## Décision 3 : Tri « à venir / passées » côté frontend

**Decision** : Implémenter le tri dans le composable `useCentresCulturels` via un helper pur `trierProgrammations(programmations: ProgrammationAPI[], maintenant: Date = new Date()): ProgrammationAPI[]`. Le helper retourne un nouveau tableau (immutabilité) avec :
1. Programmations dont `date_heure_debut >= maintenant`, triées par `date_heure_debut` croissant.
2. Suivies des programmations dont `date_heure_debut < maintenant`, triées par `date_heure_debut` décroissant.

Le tri est appliqué dans la page `/centres/[id].vue` sur le champ `centre.programmations` reçu du backend.

**Rationale** :
- Clarification Q3 figée.
- L'endpoint backend `/api/centres-culturels/{id}` renvoie déjà les programmations dans le DTO `CentreCulturelDetailResponse` ; pas besoin de modifier le backend.
- Un helper pur dans le composable est testable isolément, respecte l'immutabilité (spread / new array), et limite la logique dans le template Vue.
- Le paramètre `maintenant` injectable garantit une comparaison déterministe (et facilite d'éventuels tests).

**Alternatives considered** :
- Tri côté backend via un nouveau paramètre de query : viole le principe V (Simplicité), complexifie l'API pour une règle d'affichage, rejeté.
- Tri inline dans le template : mélange logique et présentation, moins réutilisable si la fiche est embarquée ailleurs, rejeté.

---

## Décision 4 : Confirmation de l'absence de création publique de programmation

**Decision** : Audit des fichiers actuels confirme qu'aucune page publique n'offre de création de programmation :
- `app/pages/africain-afro-americain/index.vue`, liste seule, pas de formulaire ni de bouton d'ajout.
- `app/pages/site/[id].vue` : fiche centre, pas de création.
- `app/pages/site/[siteId]/programmation/[programmationId].vue`, détail programmation en lecture.

Le seul point de création est `/admin/programmations/create.vue` (admin). FR-014 est donc déjà satisfait en l'état ; la migration des pages vers `/centres/*` doit préserver cette absence d'action de création côté public. Aucune suppression de bouton nécessaire, vérification à la revue uniquement.

**Rationale** : Éviter d'introduire à tort un bouton dans la nouvelle version. La revue de code doit confirmer ce point lors de la PR.

**Alternatives considered** : aucune : la contrainte est de ne rien ajouter, pas de corriger quelque chose d'existant.

---

## Décision 5 : Aucune migration SQL ni modification backend

**Decision** : Ne toucher ni au schéma `culture` ni au backend. Les tables `centre_culturel`, `programmation_centre`, `membre_centre` restent inchangées. Le flag de publication `centre_culturel.actif BOOLEAN` est l'unique mécanisme de visibilité publique ; aucun champ `deleted_at` ou `published_at` n'est introduit. Les endpoints suivants, déjà en place, sont réutilisés tels quels :

| Usage | Endpoint |
|---|---|
| Liste publique centres | `GET /api/centres-culturels` |
| Détail public centre + programmations | `GET /api/centres-culturels/{id}` |
| Détail public programmation | `GET /api/centres-culturels/{centre_id}/programmations/{id}` |
| Admin CRUD centres | `GET/POST/PUT/DELETE /admin/centres-culturels[/{id}]` |
| Admin membres d'un centre | `GET/POST/PUT/DELETE /admin/centres-culturels/{id}/membres[/{membre_id}]` |
| Admin CRUD programmations | `GET/POST/PUT/DELETE /admin/programmations[/{id}]` |

**Rationale** :
- Principe III (SQL source de vérité) et principe V (Simplicité) : aucune donnée n'est ajoutée ni modifiée par cette feature.
- Clarification Q4 : contexte non-production, aucun risque à considérer les centres/programmations existants comme publiés au déploiement. Aucune migration SQL ni script de backfill nécessaire.
- L'URL publique backend `/api/centres-culturels/{centre_id}/programmations/{id}` est déjà hiérarchique et cohérente avec la nouvelle URL frontend `/centres/{centreId}/programmations/{programmationId}`, parité naturelle.

**Alternatives considered** :
- Ajouter un flag `publie` ou une date `published_at` sur les tables : hors périmètre, plan-level, non demandé par la spec, rejeté pour cette feature.
- Endpoint admin dédié pour basculer publication : non requis par la spec, rejeté.

---

## Décision 6 : Liens internes à mettre à jour

**Decision** : Audit préalable ciblé via recherche dans les sources :

| Fichier | Occurrence à modifier |
|---|---|
| `app/components/layout/NavBar.vue:418` | `to: '/africain-afro-americain'` → `to: '/centres'` |
| `app/components/layout/BoutonLateralGauche.vue:233` | `to: '/africain-afro-americain'` → `to: '/centres'` |
| `app/pages/africain-afro-americain/index.vue:88` | ligne obsolète (fichier supprimé) ; la nouvelle `pages/centres/index.vue` doit pointer vers `/centres/{id}` (au lieu de `/site/{id}`) |
| `app/pages/site/[siteId]/programmation/[programmationId].vue:75,97,199,206` | obsolète ; la nouvelle page programmation doit pointer vers `/centres/{id}` pour le retour au centre parent |
| `app/pages/site/[id].vue:110` | obsolète ; la nouvelle fiche centre doit pointer vers `/centres` dans son fil d'Ariane |
| `app/components/centres-culturels/ProgrammationCard.vue:19` | `NuxtLink :to="/site/${siteId}/programmation/..."` → `/centres/${siteId}/programmations/...` |
| `app/components/home/ApplisSection.vue:118` | `to: '/africain-afro-americain'` → `to: '/centres'` |
| `app/components/common/BreadcrumbNav.vue:108-110` | clé `routeLabels` : ajouter `'centres': 'Centres culturels'`, `'programmations': 'Programmation'` ; les clés `'africain-afro-americain'` et `'site'` peuvent rester (legacy) mais deviennent inutilisées. |
| `app/mocks/promotion-valeur.ts:48` | `link: '/africain-afro-americain'` → `link: '/centres'` |

Audit T003 (2026-04-19) : `grep -rn "africain-afro-americain\|/site/" uafricas_backend/src/` retourne `No matches found`. Aucune URL legacy dans les emails transactionnels ou notifications backend.

**Rationale** : FR-012 exige 100 % des liens internes mis à jour (SC-003). L'audit préalable limite les oublis et sert de checklist à `tasks.md`.

---

## Synthèse des dépendances & hypothèses retenues

- **Aucune nouvelle dépendance** npm ou Cargo.
- **Hypothèse non-production active** : tout centre/programmation existant est considéré publié sans revalidation admin (Q4, consigné en `Assumptions` de la spec).
- **Contrainte daisyUI** : les nouvelles pages publiques sous `/centres/*` doivent n'utiliser que Tailwind CSS v4 pur (principe constitutionnel VI). Les composants existants sous `app/components/centres-culturels/` sont déjà conformes (utilisés par l'ancienne page publique).
- **Pas de framework de test** sur le projet, la validation repose sur le parcours manuel documenté dans `quickstart.md`.

Tous les `NEEDS CLARIFICATION` sont résolus. Prêt pour Phase 1.
