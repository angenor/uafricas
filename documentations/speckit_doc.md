# Spec Kit : mémo d'utilisation

> Version installée : **spec-kit v0.14.4** (CLI `specify`).
> Depuis la v0.14, l'intégration Claude passe par des **skills** (`.claude/skills/speckit-*`) :
> les commandes s'écrivent avec un **tiret** (`/speckit-plan`) et non plus un point (`/speckit.plan`).

## Le cycle en un schéma

```
constitution → specify → [clarify] → plan → [checklist] → tasks → [analyze] → implement → [converge] → PR
                                                                                              ↑
                            ↑                                                    mon code fait-il vraiment
                    ma spec est-elle bien écrite ?                               ce que la spec demande ?

                                    [taskstoissues] : à tout moment après tasks,
                                          pour suivre le travail sur GitHub
```

Les étapes entre `[crochets]` sont optionnelles. Les autres forment le chemin normal.

## Les étapes obligatoires

| Commande | Rôle |
|---|---|
| `/speckit-constitution` | Définir les principes du projet (qualité, UX, conventions). Une fois, puis on n'y revient qu'en cas de changement de cap. |
| `/speckit-specify` | Décrire **quoi** construire et **pourquoi**, sans choix technique. Crée la branche + `specs/<nnn>-<nom>/spec.md`. |
| `/speckit-plan` | Le **comment** : stack, architecture, modèle de données → `plan.md`. |
| `/speckit-tasks` | Découper le plan en tâches ordonnées → `tasks.md`. |
| `/speckit-implement` | Exécuter les tâches et écrire le code. |

## Les étapes optionnelles

### `/speckit-clarify` : avant le plan

Pose jusqu'à 5 questions ciblées sur les zones floues de la spec, puis **réécrit les réponses dans `spec.md`**. À lancer quand la spec a été écrite vite.

### `/speckit-checklist` : le correcteur de votre spec

Relit `spec.md` et signale **ce que vous avez oublié d'écrire ou écrit de façon trop vague**. Il ne regarde jamais le code : il évalue la qualité de vos exigences.

> Votre spec est une commande passée à un artisan. Checklist est le collègue qui la relit avant envoi et demande : « tu as dit "livraison rapide"… rapide, c'est combien de jours ? »

Si la spec dit *« les membres gagnent des points quand ils publient, les plus actifs apparaissent en évidence »*, il produit `specs/.../checklists/gamification.md` :

```markdown
- [ ] CHK001 Combien de points par type d'action est-il précisé ? [Manque]
- [ ] CHK002 « Apparaître en évidence » est-il chiffré (taille, position) ? [Flou]
- [ ] CHK003 Si une publication est supprimée, les points sont-ils retirés ? [Cas limite]
- [ ] CHK004 « Les plus actifs » = le top 10 ? le top 1 % ? [Flou]
```

Ce sont des **questions à vous**, pas des tests. Jamais « vérifier que le compteur affiche 10 points ».

### `/speckit-analyze` : après les tâches

Rapport de cohérence croisée **spec ↔ plan ↔ tasks** : contradictions, exigences sans tâche correspondante. Lecture seule, ne modifie rien.

> Différence avec `converge` : `analyze` compare les **documents entre eux**, `converge` compare les documents **au code**.

### `/speckit-converge` : l'inspecteur des travaux finis

À lancer **après** `implement`. Compare la spec au code réellement écrit et **ajoute les oublis en bas de `tasks.md`** :

```markdown
## Phase 8: Convergence
- [ ] T112 Retirer les classes daisyUI de SectionChaine.vue per Constitution VI (contradicts)
- [ ] T113 Ajouter la désuspension manuelle d'une chaîne per FR-014 (missing)
- [ ] T114 Compléter les 44 thèmes : 38 seulement en base per FR-003 (partial)
```

Chaque ligne indique **d'où vient l'exigence** (`FR-014`) et **la nature du manque** :

| Type | En clair |
|---|---|
| `missing` | absent, rien n'a été fait |
| `partial` | commencé mais incomplet |
| `contradicts` | fait, mais à l'envers de ce qui était demandé |
| `unrequested` | présent dans le code alors que personne ne l'avait demandé |

Puis on relance `/speckit-implement` pour traiter ces nouvelles tâches.

**Garanties :** il n'écrit **jamais** de code, ne supprime **jamais** rien, n'ajoute que des lignes à la fin de `tasks.md`. Si tout est conforme, il ne touche pas au fichier et répond « ✅ Converged ».

### `/speckit-taskstoissues` : vers GitHub

Transforme chaque tâche de `tasks.md` en **issue GitHub** sur `github.com/angenor/uafricas` :

```
- [ ] T001 Créer la table joueur_points   →   issue « T001: Créer la table joueur_points »
```

Utile à plusieurs, ou pour suivre l'avancement dans l'onglet Issues. Il saute les issues déjà créées (relançable sans doublons) et refuse d'écrire ailleurs que sur le dépôt du remote.

**Prérequis :** le connecteur GitHub (MCP) doit être activé : ce n'est pas le cas par défaut.

## Commandes terminal

```bash
# Installation
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git

# Mise à jour du CLI (épingler la version évite les surprises)
uv tool install specify-cli --force --from git+https://github.com/github/spec-kit.git@v0.14.4

# Vérifier les outils et intégrations disponibles
specify check

# Initialiser un nouveau projet / le dossier courant
specify init <nom> --integration claude
specify init . --integration claude

# Mettre à jour les templates et skills d'un projet existant
# (--force écrase templates/scripts ; la constitution est préservée)
specify init --here --force --integration claude --script sh
```

> `--integration` remplace l'ancien `--ai`.
> Toujours sauvegarder `.specify/` avant un `--force` (voir `backups/speckit-0.12.16/`).

## Nouveautés v0.14 hors skills

| Commande | Rôle |
|---|---|
| `specify workflow run speckit` | Enchaîne `specify → plan → tasks → implement` avec des **portes de revue** : l'exécution s'arrête et attend votre `approve`/`reject`. Défini dans `.specify/workflows/speckit/workflow.yml`. |
| `specify extension` | Hooks déclarés dans `.specify/extensions.yml` (ex. `after_converge`), pour greffer des actions automatiques entre les étapes. |
| `specify preset` / `specify bundle` | Jeux de principes et de templates réutilisables. |

## Corriger un bug dans une feature déjà spécifiée

Approche recommandée (Discussion #152 du dépôt) :

1. Corriger le bug directement avec l'agent, dans le code de la feature.
2. Puis remettre la spec à jour :

```
/speckit-specify L'implémentation de <00X-nom-spec> était incorrecte et a été corrigée
comme ci-dessus. Mets à jour les fichiers nécessaires de la spec <00X>.
```
