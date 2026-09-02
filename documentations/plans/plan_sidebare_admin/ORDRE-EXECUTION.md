# Ordre d'exécution recommandé : Sidebar Admin UAfricas

## Phases d'exécution

```
Phase 0 ─ Fondation                                          [x] TERMINÉ
  └── 00 Infrastructure (middleware, composants CRUD, types)

Phase 1 ─ Données fondamentales                              [x] TERMINÉ
  ├── 01 Utilisateurs & Accès (IAM)      ✅ TERMINÉ
  └── 02 Référentiels (shared)           ✅ TERMINÉ

Phase 2 ─ Modules métier (parallélisable)                    [x] TERMINÉ
  ├── 03 Marché Africain       ✅ TERMINÉ
  ├── 04 Programmes d'échange  ✅ TERMINÉ
  ├── 05 Innovation            ✅ TERMINÉ
  └── 06 Culture               ✅ TERMINÉ

Phase 3 ─ Fonctionnalités avancées (parallélisable)          [x] TERMINÉ
  ├── 07 AfroLang              ✅ TERMINÉ
  ├── 08 Médias & Contenus     ✅ TERMINÉ
  └── 09 Gouvernance           ✅ TERMINÉ

Phase 4 ─ Modules complexes                                  [x] TERMINÉ
  ├── 10 Profils pays          ✅ TERMINÉ (40 endpoints, 9 onglets, contributions)
  └── 11 Audit & Logs          ✅ TERMINÉ

Phase 5 ─ Finalisation                                       [ ] NON DÉMARRÉ
  └── 12 Dashboard             ← agrège les données de tous les modules
```

---

## Règles d'exécution

1. **Backend d'abord, frontend ensuite**, Pour chaque plan, implémenter les endpoints API avant les pages Vue
2. **Respecter les dépendances** : Ne pas démarrer un plan tant que ses prérequis ne sont pas terminés
3. **Paralléliser quand possible** : Les plans d'une même phase (sauf contraintes) peuvent être faits en parallèle
4. **Tester après chaque frontend** : Lancer `agent-browser --headed` pour les tests manuels de chaque plan
5. **Cocher la progression** : Mettre à jour les `[ ]` → `[x]` dans chaque fichier de plan au fur et à mesure

---

## Chemin critique (séquentiel strict)

Le chemin le plus long qui ne peut pas être parallélisé :

```
00 → 01 → 02 → (03|04|05|06) → (07|08|09) → 11 → 12
                                               ↑
                                          10 ──┘ (parallèle avec 07-09)
```

**Ordre séquentiel minimum** (si un seul développeur) :

```
00 → 01 → 02 → 03 → 04 → 05 → 06 → 07 → 08 → 09 → 10 → 11 → 12
```

---

## Plans par volume (du plus lourd au plus léger)

| Plan | Backend | Frontend | Total | Complexité |
|------|:-------:|:--------:|:-----:|:----------:|
| 08 : Médias & Contenus | 30 | 22 | 52 | Haute (4 sous-modules) |
| 02 : Référentiels | 24 | 18 | 42 | Moyenne (données partagées) |
| 10 : Profils pays | 31 | 5 | 36 | Haute (8 sous-entités, 9 onglets) |
| 06 : Culture | 20 | 14 | 34 | Moyenne |
| 09 : Gouvernance | 19 | 13 | 32 | Moyenne |
| 05 : Innovation | 18 | 13 | 31 | Moyenne |
| 01 : Utilisateurs & Accès | 15 | 13 | 28 | Moyenne (RBAC) |
| 03 : Marché Africain | 11 | 8 | 19 | Faible |
| 04 : Programmes d'échange | 10 | 8 | 18 | Faible |
| 00 : Fondation | 6 | 12 | 18 | Faible |
| 07 : AfroLang | 7 | 9 | 16 | Faible |
| 11 : Audit & Logs | 8 | 5 | 13 | Faible (diff JSON) |
| 12 : Dashboard | 3 | 6 | 9 | Faible (agrégation) |

---

## Graphe de dépendances

```
                        ┌─────────────────┐
                        │  00-fondation   │ ✅ Terminé
                        └────────┬────────┘
                                 │
                    ┌────────────┼────────────┐
                    ▼                         ▼
          ┌─────────────────┐      ┌─────────────────┐
          │ 01-utilisateurs │      │ 02-referentiels │
          └────────┬────────┘      └────────┬────────┘
                   │                        │
      ┌────────────┼────────────────────────┤
      │            │            │           │
      ▼            ▼            ▼           ▼
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│03-marché │ │04-échange│ │05-innov. │ │06-culture│
└────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘
     │             │            │            │
     └──────┬──────┴────────────┴────────────┘
            │
      ┌─────┼──────────────────┐
      ▼     ▼                  ▼
┌──────────┐ ┌──────────┐ ┌──────────┐
│07-afro.  │ │08-médias │ │09-gouv.  │
└────┬─────┘ └────┬─────┘ └────┬─────┘
     │             │            │
     └──────┬──────┴────────────┘
            │
      ┌─────┼──────┐
      ▼            ▼
┌──────────┐ ┌──────────┐
│10-pays   │ │11-audit  │
└────┬─────┘ └────┬─────┘
     │             │
     └──────┬──────┘
            ▼
     ┌──────────────┐
     │ 12-dashboard │
     └──────────────┘
```

---

## Prochaine action

> **Commencer par le plan `01-utilisateurs-acces.md`**, Backend : endpoints CRUD utilisateurs, rôles, permissions, organisations. Frontend : pages admin correspondantes.
