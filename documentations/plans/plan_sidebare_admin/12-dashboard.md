# 12 — Dashboard (Vue d'ensemble & KPIs)

> **Phase** : 5 — Finalisation
> **Section sidebar** : Dashboard
> **Icône** : faChartLine
> **Statut global** : [x] Backend + Frontend terminés

---

## Dépendances

### Fichiers SQL requis
- **Tous les schemas** — Le dashboard agrège des statistiques de l'ensemble des modules :
  - `schemas/04_iam.sql` → count utilisateurs, organisations, rôles
  - `schemas/05_marketplace.sql` → count annonces par état, favoris
  - `schemas/06_exchange.sql` → count programmes, candidatures
  - `schemas/07_innovation.sql` → count innovations, projets, africantives
  - `schemas/08_culture.sql` → count centres culturels, posts codimoi
  - `schemas/08b_afrolang.sql` → count sessions, participants
  - `schemas/09_media_content.sql` → count événements, moocs, livres, stations radio, chaînes TV
  - `schemas/10_governance.sql` → count factchecks, bad habits, idées forces
  - `schemas/11_country_profile.sql` → count fiches pays, contributions en attente
  - `schemas/12_audit.sql` → activité récente

### Plans précédents (prérequis)
- **`00-fondation-admin.md`** — AdminStatsCard, middleware, useAdmin
- **Tous les plans 01 à 11** — Le dashboard consomme les données de chaque module. Il peut être développé progressivement au fur et à mesure que les modules sont implémentés, mais la version complète nécessite que tous les modules soient en place.

### Plans qui dépendent de celui-ci
- Aucun — c'est le dernier plan

### Backend existant
- [x] `app/pages/admin/index.vue` — placeholder avec cartes KPI vides — **À remplacer**

---

## Backend

### B12.1 — Endpoint de statistiques globales
- [x] `GET /api/admin/dashboard/stats` — retourne un objet agrégé :
  ```
  {
    utilisateurs: { total, actifs, en_attente, suspendus },
    organisations: { total },
    annonces: { total, publiees, en_attente, expirees },
    programmes: { total, actifs, candidatures_en_attente },
    innovations: { total, publiees },
    projets: { total, approuves, en_revue, soumis },
    africantives: { total },
    centres_culturels: { total },
    codimoi: { total, par_type },
    sessions_afrolang: { total, en_cours },
    evenements: { total, a_venir, inscrits_total },
    moocs: { total, inscrits_total, en_cours },
    livres: { total },
    radio_tv: { stations_radio, chaines_tv },
    factchecks: { total, par_verdict },
    bad_habits: { total, par_gravite },
    idea_forces: { total },
    fiches_pays: { total, contributions_en_attente },
    audit: { actions_aujourd_hui, actions_cette_semaine }
  }
  ```
- **Fichiers** : `src/handlers/admin/dashboard.rs`

### B12.2 — Endpoint d'activité récente
- [x] `GET /api/admin/dashboard/activite-recente` — dernières 20 actions d'audit (timeline)
- **Fichiers** : `src/handlers/admin/dashboard.rs`

### B12.3 — Endpoint de tendances
- [x] `GET /api/admin/dashboard/tendances?periode=7j|30j|90j` — données pour graphiques :
  - Inscriptions utilisateurs par jour
  - Annonces publiées par jour
  - Événements par mois
  - Contributions fiches pays par semaine
- **Fichiers** : `src/handlers/admin/dashboard.rs`

---

## Frontend

### Page principale
- [x] `app/pages/admin/index.vue` — refonte complète :

  **Section 1 : KPIs principaux** (grille de AdminStatsCard)
  - [x] Utilisateurs actifs
  - [x] Annonces publiées
  - [x] Événements à venir
  - [x] Projets en revue
  - [x] Candidatures en attente
  - [x] Contributions pays en attente
  - [x] Sessions AfroLang en cours
  - [x] MOOC en cours

  **Section 2 : Graphiques de tendances**
  - [x] Courbe inscriptions utilisateurs (7j/30j/90j)
  - [x] Barres annonces par état
  - [x] Camembert factchecks par verdict
  - [x] Barres mauvaises pratiques par gravité

  **Section 3 : Activité récente** (timeline)
  - [x] Fil chronologique des dernières actions (audit_log simplifié)
  - [x] Icônes par type d'action, liens vers les entités

  **Section 4 : Alertes & Actions rapides**
  - [x] Candidatures en attente de revue (lien direct)
  - [x] Contributions pays en attente de modération (lien direct)
  - [x] Annonces en attente de publication (lien direct)

### Composables
- [x] `app/composables/useAdminDashboard.ts` — API client dashboard (stats, activité, tendances)

### Composants spécifiques
- [x] `app/components/admin/AdminChart.vue` — graphiques CSS/SVG natifs (barres horizontales + donut SVG, sans dépendance externe)
- [x] `app/components/admin/AdminActivityTimeline.vue` — timeline d'activité récente
- [x] `app/components/admin/AdminQuickActions.vue` — section alertes/actions rapides

---

## Critères de validation
- [x] KPIs affichent des données réelles de chaque module
- [x] Graphiques de tendances fonctionnels avec sélection de période
- [x] Timeline d'activité récente avec liens vers les entités
- [x] Actions rapides mènent aux pages de modération correspondantes
- [x] Le dashboard se charge rapidement (requêtes SQL optimisées avec indexes)

---

## Tests manuels (`agent-browser --headed`)

> Les tests suivants nécessitent une vérification visuelle dans le navigateur.
> Commande : `agent-browser --headed`

### KPIs
- [x] **T12.1** — Cartes KPI : 8 KPIs avec valeurs numériques réelles (32 utilisateurs actifs, 1 annonce publiée, 8 événements, etc.)
- [x] **T12.2** — Icônes et couleurs : chaque KPI a une icône FA et une couleur distincte (primary, success, info, warning, accent, secondary, error, neutral)

### Graphiques
- [x] **T12.3** — Barres inscriptions : rendu CSS natif, sélecteur 7j/30j/90j fonctionnel, mise à jour dynamique confirmée
- [x] **T12.4** — Barres annonces par état : couleurs success/warning/error, valeurs affichées dans les barres
- [x] **T12.5** — Donut factchecks : légende SVG avec 5 verdicts et compteurs, icône placeholder quand total=0
- [x] **T12.6** — Barres mauvaises pratiques : couleurs success/warning/error par gravité (faible/élevée/critique)

### Timeline & Actions rapides
- [x] **T12.7** — Timeline activité récente : icônes colorées par type (+ vert CREATE, stylo bleu UPDATE, poubelle rouge DELETE), badges schema, dates relatives
- [x] **T12.8** — Actions rapides : href `/admin/candidatures` vérifié ✓ (badge "2")
- [x] **T12.9** — Actions rapides : contributions en attente = 0, alerte non affichée (comportement conditionnel correct)
- [x] **T12.10** — Actions rapides : annonces en attente = 0, alerte non affichée (comportement conditionnel correct)

### Performance
- [x] **T12.11** — Temps de chargement : ~2.1 secondes (< 3s), requête CTE unique avec 24 CTEs

---

## Notes
- Le dashboard est la dernière pièce car il agrège les données de tous les modules. Il peut cependant être développé de façon incrémentale : ajouter les KPIs de chaque module au fur et à mesure de leur implémentation.
- La page `admin/index.vue` existe déjà en placeholder. Elle sera complètement réécrite.
- Les graphiques nécessitent une librairie frontend (Chart.js recommandé, léger et compatible Vue 3).
- Les requêtes SQL du dashboard doivent être optimisées (COUNT avec conditions, pas de SELECT * sur toutes les tables). Considérer des vues matérialisées si les performances sont insuffisantes.
