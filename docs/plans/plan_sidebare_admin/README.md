# Plan d'implémentation — Sidebar Admin UAfricas

> Généré le 18/02/2026 — basé sur `uafricas_frontend/doc/plans/admin/sidebar.yaml`

---

## Vue d'ensemble

13 fichiers de plan couvrant les **12 sections du sidebar admin** + la fondation technique.
Chaque rubrique est implémentée **backend d'abord, frontend ensuite**.

---

## Ordre d'implémentation

```
Phase 0 ─ Fondation
  └── 00-fondation-admin.md        ← Middleware, composants CRUD, composable de base

Phase 1 ─ Données fondamentales (parallélisable entre 01 et 02)
  ├── 01-utilisateurs-acces.md     ← IAM : Utilisateurs, Rôles, Permissions, Organisations
  └── 02-referentiels.md           ← Shared : Pays, Domaines, Catégories, Tags, Médias

Phase 2 ─ Modules métier (parallélisable entre 03, 04, 05, 06)
  ├── 03-marche-africain.md        ← Marketplace : Annonces, Favoris
  ├── 04-programmes-echange.md     ← Exchange : Programmes, Candidatures
  ├── 05-innovation.md             ← Innovation : Innovations, Projets, Africantives
  └── 06-culture.md                ← Culture : Centres culturels, Programmations, Codi-Moi

Phase 3 ─ Fonctionnalités avancées (parallélisable entre 07, 08, 09)
  ├── 07-afrolang.md               ← AfroLang : Salles, Sessions, Tableau blanc
  ├── 08-medias-contenus.md        ← Médias : Radio/TV, Événements, MOOC, Bibliothèque
  └── 09-gouvernance.md            ← Gouvernance : FactCheck, Bad Habits, Idées forces

Phase 4 ─ Modules complexes
  ├── 10-profils-pays.md           ← Country Profile : Fiches pays + 8 sous-entités
  └── 11-audit-logs.md             ← Audit : Journal d'actions + diff JSON

Phase 5 ─ Finalisation
  └── 12-dashboard.md              ← Dashboard : KPIs, graphiques, activité récente
```

---

## Graphe de dépendances

```
                        ┌─────────────────┐
                        │  00-fondation   │
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

## Correspondance Sidebar ↔ Plans ↔ SQL

| Section sidebar         | Plan                  | Schema SQL               | Tables principales                                    |
|-------------------------|-----------------------|--------------------------|-------------------------------------------------------|
| Dashboard               | `12-dashboard.md`     | Tous                     | Agrégation stats                                      |
| Utilisateurs & Accès    | `01-utilisateurs.md`  | `iam`                    | utilisateur, role, permission, organisation, partenariat |
| Référentiels            | `02-referentiels.md`  | `shared` + `iam`         | pays, domaine_secteur, categorie, tag, media, specialite |
| Marché Africain         | `03-marche.md`        | `marketplace`            | annonce, annonce_pays, annonce_media, annonce_favori   |
| Programmes d'échange    | `04-echange.md`       | `exchange`               | programme, candidature                                 |
| Innovation              | `05-innovation.md`    | `innovation`             | innovation, projet, africantive + médias/docs          |
| Culture                 | `06-culture.md`       | `culture`                | centre_culturel, programmation, codimoi + tags/commentaires |
| AfroLang                | `07-afrolang.md`      | `afrolang`               | salle, salle_privee, session, participants, tableau_blanc |
| Médias & Contenus       | `08-medias.md`        | `media_content`          | station_radio, chaine_tv, evenement, mooc, livre       |
| Gouvernance             | `09-gouvernance.md`   | `governance`             | factcheck, bad_habit, idea_force + médias/commentaires |
| Profils pays            | `10-pays.md`          | `country_profile`        | fiche_pays + 8 sous-entités + contributions            |
| Audit & Logs            | `11-audit.md`         | `shared` (audit_log)     | audit_log                                             |

---

## Suivi de progression

Chaque fichier de plan contient des checkboxes `[ ]` pour chaque tâche.
Pour marquer une tâche comme terminée, remplacer `[ ]` par `[x]`.

### Récapitulatif rapide

| #  | Plan                     | Phase | Statut         |
|----|--------------------------|-------|----------------|
| 00 | Fondation Admin          | 0     | [x] Terminé (19/02/2026) |
| 01 | Utilisateurs & Accès     | 1     | [ ] Non démarré |
| 02 | Référentiels             | 1     | [ ] Non démarré |
| 03 | Marché Africain          | 2     | [ ] Non démarré |
| 04 | Programmes d'échange     | 2     | [ ] Non démarré |
| 05 | Innovation               | 2     | [ ] Non démarré |
| 06 | Culture                  | 2     | [ ] Non démarré |
| 07 | AfroLang                 | 3     | [ ] Non démarré |
| 08 | Médias & Contenus        | 3     | [ ] Non démarré |
| 09 | Gouvernance              | 3     | [ ] Non démarré |
| 10 | Profils pays             | 4     | [ ] Non démarré |
| 11 | Audit & Logs             | 4     | [ ] Non démarré |
| 12 | Dashboard                | 5     | [ ] Non démarré |

---

## Tests manuels (`agent-browser --headed`)

Chaque plan contient une section **"Tests manuels"** avec des vérifications visuelles à effectuer dans le navigateur.

**Commande** : `agent-browser --headed`

**Quand tester** : Après l'implémentation du frontend de chaque rubrique.

**Convention** :
- Les tests sont numérotés `T{plan}.{numéro}` (ex: `T3.5` = test 5 du plan 03)
- Chaque test a une checkbox `[ ]` à cocher quand validé
- Les tests couvrent : rendu visuel, interactions UI, workflows, navigation, uploads

**Récapitulatif tests par plan** :

| Plan | Nb tests | Focus principal |
|------|----------|-----------------|
| 00   | 10       | Middleware, composants génériques, layout |
| 01   | 11       | CRUD utilisateurs, rôles, matrice permissions |
| 02   | 10       | Tree view catégories, upload drag & drop, galerie médias |
| 03   | 8        | Formulaire multi-étapes, modération, drag & drop médias |
| 04   | 8        | Workflow candidatures, téléchargement CV |
| 05   | 8        | Workflow approbation projets, upload documents |
| 06   | 10       | Gestion membres, commentaires arborescents, autocomplete tags |
| 07   | 7        | Rendu tableau blanc JSONB, supervision lecture seule |
| 08   | 13       | Onglets Radio/TV, inscriptions, barres progression |
| 09   | 10       | Badges verdict/gravité colorés, upload preuves |
| 10   | 12       | 9 onglets, CRUD inline, diff visuel contributions |
| 11   | 9        | Diff JSON before/after, enregistrement automatique |
| 12   | 11       | Graphiques Chart.js, timeline, actions rapides |
| **Total** | **127** | |

---

## Comment utiliser ces plans

1. **Commencer par `00-fondation-admin.md`** — c'est le prérequis absolu
2. **Puis `01` et `02` en parallèle** — données fondamentales
3. **Pour chaque plan**, suivre l'ordre : Backend d'abord → Frontend ensuite
4. **Cocher les tâches** au fur et à mesure dans chaque fichier
5. **Après chaque frontend**, lancer les tests manuels : `agent-browser --headed`
6. **Mettre à jour le statut** dans le tableau ci-dessus
7. **Consulter les dépendances** de chaque plan avant de le commencer
