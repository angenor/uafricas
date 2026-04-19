# Contract — Routes frontend (Nuxt 4)

**Feature**: `001-centres-reorganisation` | **Date**: 2026-04-19

Ce contrat fige les routes frontend introduites, supprimées et redirigées par la feature. Il est la référence utilisée par `tasks.md` pour ordonnancer les changements et par la validation finale (quickstart + SC-002/SC-003).

---

## 1. Nouvelles routes publiques (canoniques)

| Route | Fichier Nuxt | Backend consommé | Accès |
|---|---|---|---|
| `/centres` | `app/pages/centres/index.vue` | `GET /api/centres-culturels` | Public (anonyme OK) |
| `/centres/:id` | `app/pages/centres/[id].vue` | `GET /api/centres-culturels/{id}` | Public |
| `/centres/:id/programmations/:programmationId` | `app/pages/centres/[id]/programmations/[programmationId].vue` | `GET /api/centres-culturels/{centreId}/programmations/{id}` | Public |

### Contrat d'affichage `/centres` (FR-006, FR-005a)

- Liste de `CentreCulturelAPI` dans l'ordre renvoyé par le backend (sans tri supplémentaire frontend pour cette feature).
- En-tête : carrousel alimenté par `centres.filter(c => c.image_couverture_url).map(c => ({ src: c.image_couverture_url, alt: c.nom }))`. Si tableau vide, fallback visuel statique.
- Lien de carte : `NuxtLink :to="`/centres/${centre.id}`"`.
- Aucun bouton « Ajouter une programmation » ni « Ajouter un centre » sur cette page (FR-014).

### Contrat d'affichage `/centres/:id` (FR-007, FR-017, FR-017a)

- Bloc « programmations » trié via helper `trierProgrammations` (Décision 3 de research.md) :
  1. Programmations à venir (`date_heure_debut >= maintenant`) triées croissant.
  2. Programmations passées triées décroissant.
- Chaque programmation pointe vers `/centres/${centre.id}/programmations/${programmation.id}`.
- Fil d'Ariane : `Accueil → Centres → {centre.nom}`.
- Bouton « Retour » vers `/centres`.
- Aucun bouton d'ajout de programmation (FR-014).

### Contrat d'affichage `/centres/:id/programmations/:programmationId` (FR-008, FR-013)

- Fil d'Ariane : `Accueil → Centres → {centre.nom} → {programmation.titre}`.
- Bouton « Retour au centre » vers `/centres/${centre.id}`.
- Affichage en lecture seule (FR-014).

---

## 2. Redirections permanentes (HTTP 301)

Implémentées via `routeRules` dans `uafricas_frontend/nuxt.config.ts` (Décision 1 de research.md).

| Ancienne route | Nouvelle route | Code HTTP | FR |
|---|---|---|---|
| `/africain-afro-americain` | `/centres` | 301 | FR-009 |
| `/site/:id` | `/centres/:id` | 301 | FR-010 |
| `/site/:siteId/programmation/:programmationId` | `/centres/:siteId/programmations/:programmationId` | 301 | FR-011 |

### Forme attendue dans `nuxt.config.ts`

```ts
export default defineNuxtConfig({
  routeRules: {
    '/africain-afro-americain': {
      redirect: { to: '/centres', statusCode: 301 },
    },
    '/site/:id': {
      redirect: { to: '/centres/:id', statusCode: 301 },
    },
    '/site/:siteId/programmation/:programmationId': {
      redirect: { to: '/centres/:siteId/programmations/:programmationId', statusCode: 301 },
    },
  },
})
```

### Critères d'acceptation des redirections

- `GET /africain-afro-americain` répond `301` avec `Location: /centres`.
- `GET /site/<uuid>` répond `301` avec `Location: /centres/<uuid>` (paramètre préservé).
- `GET /site/<uuid>/programmation/<uuid>` répond `301` avec `Location: /centres/<uuid>/programmations/<uuid>` (deux paramètres préservés, segment `programmation` → `programmations`).
- Les réponses 301 sont servies avant rendu Nuxt (pas de hydration Vue inutile).
- La chaîne de redirection est **d'un seul saut** (aucun 301 → 301 en cascade).

---

## 3. Routes supprimées

| Fichier | Action |
|---|---|
| `app/pages/africain-afro-americain/index.vue` | SUPPRIMÉ (remplacé par `routeRule`) |
| `app/pages/site/[id].vue` | SUPPRIMÉ (remplacé par `routeRule`) |
| `app/pages/site/[siteId]/programmation/[programmationId].vue` | SUPPRIMÉ (remplacé par `routeRule`) |
| Dossier `app/pages/africain-afro-americain/` | SUPPRIMÉ si vide |
| Dossier `app/pages/site/` | SUPPRIMÉ si vide |

> ⚠️ Attention Nuxt 4 : un fichier de page prend priorité sur un `routeRule` au même chemin. Les fichiers doivent donc être supprimés **avant** ou **en même temps** que l'ajout des `routeRules` pour que la redirection soit active.

---

## 4. Liens internes à mettre à jour

| Fichier | Changement |
|---|---|
| `app/components/layout/NavBar.vue` | Entrée menu `to: '/africain-afro-americain'` → `to: '/centres'` |
| `app/components/layout/BoutonLateralGauche.vue` | Entrée menu `to: '/africain-afro-americain'` → `to: '/centres'` |
| Tout fichier contenant encore `to="/site/` | Remplacé par `to="/centres/` et `to="/centres/${id}/programmations/...`" selon contexte |

Un `grep -rn "'/africain-afro-americain\|'/site/" uafricas_frontend/app/` final à l'issue de la feature DOIT ne retourner aucun résultat hors fichiers de documentation (SC-003).

---

## 5. Routes admin — inchangées

Les routes suivantes restent strictement identiques (FR-015, Décision 5 de research.md) :

| Route admin | Fichier Nuxt | Backend |
|---|---|---|
| `/admin/centres-culturels` (liste) | `app/pages/admin/centres-culturels/index.vue` | `GET /admin/centres-culturels` |
| `/admin/centres-culturels/create` | `app/pages/admin/centres-culturels/create.vue` | `POST /admin/centres-culturels` |
| `/admin/centres-culturels/:id` | `app/pages/admin/centres-culturels/[id].vue` | `GET/PUT/DELETE /admin/centres-culturels/{id}` |
| `/admin/programmations` | `app/pages/admin/programmations/index.vue` | `GET /admin/programmations` |
| `/admin/programmations/create` | `app/pages/admin/programmations/create.vue` | `POST /admin/programmations` |
| `/admin/programmations/:id` | `app/pages/admin/programmations/[id].vue` | `GET/PUT/DELETE /admin/programmations/{id}` |

Aucune action requise sur ces routes par la présente feature. Audit visuel de conformité fonctionnelle à réaliser (présence des actions CRUD + instrumentation audit existante préservée) lors de la revue.

---

## 6. Contrat de parité fonctionnelle post-déploiement

Pour valider SC-002 et SC-003, les vérifications suivantes doivent être positives après livraison :

- [ ] Ouvrir `/africain-afro-americain` dans un navigateur → URL finale dans la barre = `/centres` (après 301).
- [ ] Ouvrir `/site/<uuid-d-un-centre-existant>` → URL finale = `/centres/<uuid>`.
- [ ] Ouvrir `/site/<uuid-centre>/programmation/<uuid-prog>` → URL finale = `/centres/<uuid-centre>/programmations/<uuid-prog>`.
- [ ] `curl -I /africain-afro-americain` retourne `301 Moved Permanently` avec en-tête `Location`.
- [ ] `grep` global confirme zéro lien interne vers `/africain-afro-americain` ou `/site/` (hors fichiers de spécification).
- [ ] Navigation complète : Accueil → menu « Afroculture » → `/centres` → clic centre → fiche → clic programmation → fiche programmation → bouton retour → fiche centre. Aucun 404, aucun saut d'URL incohérent.
- [ ] Aucun bouton « Ajouter une programmation » visible sur les trois pages publiques, en anonyme comme en utilisateur authentifié non-admin.
