# Quickstart — validation de bout en bout

**Feature** : `007-engagement-points-badges` | **Date** : 2026-07-29

Le projet n'a **aucun harnais de test** (contrainte constitutionnelle assumée) : la validation est manuelle et scénarisée. Chaque scénario ci-dessous cite le critère de succès qu'il prouve. Modèle de données : [data-model.md](./data-model.md) · Contrats : [contracts/](./contracts/).

---

## Prérequis

```bash
# 1. Infrastructure
docker compose up -d                      # postgres + adminer + livekit

# 2. Migrations (dans l'ordre, idempotentes)
docker compose exec -T postgres psql -U uafricas -d africans_db \
  -f /dev/stdin < uafricas_backend/doc/bd/schemas/35c_engagement_categories_bareme.sql
docker compose exec -T postgres psql -U uafricas -d africans_db \
  -f /dev/stdin < uafricas_backend/doc/bd/schemas/35d_engagement_badges.sql
docker compose exec -T postgres psql -U uafricas -d africans_db \
  -f /dev/stdin < uafricas_backend/doc/bd/schemas/35e_engagement_partage_externe.sql

# 3. Backend (toujours tuer l'ancien processus : le port 8082 reste occupé sinon)
kill $(lsof -i :8082 -t) 2>/dev/null; cd uafricas_backend && RUST_LOG=info cargo run

# 4. Frontend
cd uafricas_frontend && pnpm dev           # http://localhost:3000
```

**Comptes de test** : `test-admin@test.com` / `Test1234` (administrateur) · `test-user@test.com` / `Test1234` (membre).

**Contrôle d'installation** — les 3 migrations sont bien passées :

```sql
-- attendu : 6 catégories, 10 règles (6 + 4 nouvelles), 10 badges, les 3 nouvelles tables
SELECT (SELECT COUNT(*) FROM engagement.categorie_points)                   AS categories,
       (SELECT COUNT(*) FROM engagement.regle_points)                       AS regles,
       (SELECT COUNT(*) FROM engagement.badge)                              AS badges,
       (SELECT COUNT(*) FROM engagement.regle_points WHERE categorie_id IS NULL) AS regles_sans_categorie;
```

`regles_sans_categorie` doit valoir **0** : une règle sans catégorie enverrait ses mouvements dans « Autres ».

---

## S1 — Espace membre : soldes, catégories, progression, historique (US1 · SC-003, SC-005, SC-008)

1. Se connecter en `test-user`, ouvrir `/mon-compte/profil` → onglet **« Mes points »** → cliquer **« Voir tout mon engagement »**. *(2 clics depuis le profil → SC-003)*
2. Vérifier l'en-tête : solde total, solde du mois, réputation, badge de niveau, **« N points avant le niveau X »**.
3. Vérifier la ventilation : la somme des catégories affichées = `total_gagne` renvoyé par l'API ; « points gagnés » et « solde courant » sont **libellés distinctement** (SC-005).
4. Filtrer l'historique sur une catégorie, puis sur le mois en cours : la pagination se réinitialise, l'ordre reste du plus récent au plus ancien.
5. Réconciliation SQL :

```sql
SELECT c.solde_points,
       (SELECT SUM(points) FROM engagement.mouvement_points WHERE utilisateur_id = c.utilisateur_id) AS total_journal
  FROM engagement.compte c
  JOIN iam.utilisateur u ON u.id = c.utilisateur_id
 WHERE u.email = 'test-user@test.com';
```

`total_journal >= solde_points` est **normal** (plancher 0 après malus). Tout autre écart est un bug.

6. **Niveau maximal** : porter le solde au-delà du dernier seuil (`POST /admin/engagement/ajustement`) → l'espace affiche « niveau maximal atteint », pas une barre de progression vide.
7. **État vide** : créer un membre neuf, ouvrir l'espace → liste pédagogique des actions récompensées, aucun compteur incohérent, aucune erreur console.
8. **Écrêtage visible** (SC-008) : voir S5, étape 5.

## S2 — Paramétrage intégral du barème (US2 · SC-001, SC-002, SC-009)

1. En `test-admin`, ouvrir `/admin/engagement/regles`. Chronométrer : **créer** une règle (action prise dans « actions disponibles », montant, plafond, catégorie), enregistrer, déclencher l'action correspondante, constater le crédit → **< 5 min, sans redéploiement** (SC-001).
2. **Désactiver** cette règle → déclencher à nouveau l'action : aucun point crédité, **et l'action métier réussit quand même**. Les mouvements passés restent lisibles.
3. **Doublons refusés** : recréer une règle avec un `type_action` existant → message explicite, pas d'erreur SQL brute. Idem pour deux niveaux au même `seuil_min`.
4. **Règle orpheline** : créer une règle avec un `type_action` inventé → l'écran affiche « non instrumentée ». C'est le garde-fou de R3 : vérifier qu'il est visible **sans survol**.
5. **Catégories** (`/admin/engagement/categories`) : créer, renommer (le nouveau libellé apparaît côté membre), tenter de supprimer une catégorie rattachée à une règle → **409** avec message clair.
6. **Niveaux** (`/admin/engagement/niveaux`) : insérer un niveau à 500 points → la réponse indique le nombre de comptes recalculés ; vérifier en SQL qu'un membre à 620 points porte bien le nouveau code sans qu'aucun mouvement n'ait eu lieu :

```sql
SELECT solde_points, niveau_code FROM engagement.compte ORDER BY solde_points DESC LIMIT 5;
```

Puis retirer ce niveau → les mêmes comptes retombent au niveau inférieur, **sans perdre de points**. Tenter de retirer le niveau plancher (`seuil_min = 0`) → refus.
7. **Paliers par famille** : créer un palier à 2 000 likes pour `programme_tele` → vérifier que les paliers globaux ne s'appliquent plus à cette famille (règle de substitution) et restent actifs ailleurs.
8. **Permissions** (SC-009) : se connecter avec un compte sans `engagement.gerer` → toutes les routes du module renvoient **403**, y compris les `GET`. Vérifier ensuite dans `/admin/audit` que **chaque** modification du barème est tracée avec son auteur.
9. **Aucun texte figé** (SC-002) : renommer une règle, une catégorie, un niveau et un badge, puis recharger l'espace membre → les 4 nouveaux libellés y apparaissent.

## S3 — Badges et succès (US3 · SC-006, SC-010)

1. Créer un badge `actions_comptees` sur `contribution_validee`, seuil **2**.
2. Faire valider 2 contributions de `test-user` → le badge apparaît dans « Mes badges » avec sa date, **une seule fois**, et une notification `engagement.badge_debloque` arrive dans la cloche (SC-010).
3. Recharger l'espace plusieurs fois (chaque lecture réévalue les conditions) → **aucun doublon**, **aucune notification supplémentaire** (SC-006) :

```sql
SELECT badge_id, COUNT(*) FROM engagement.badge_obtenu
 GROUP BY badge_id HAVING COUNT(*) > 1;   -- doit renvoyer 0 ligne
```

4. **Progression** : un badge à seuil 50 affiche « 12 / 50 » ; un badge non chiffrable n'affiche aucune barre plutôt qu'une barre à zéro.
5. **Désactiver** le badge → il quitte le catalogue « à débloquer » mais **reste** chez ceux qui l'ont obtenu. **Modifier** sa condition → même conclusion (FR-020).
6. **Badge manuel** : attribuer `distinction_editoriale` depuis `/admin/engagement/badges` → visible chez le membre, notifié, tracé dans l'audit avec le motif. Le retirer → disparaît, tracé, **sans notification**.
7. **Baisse de niveau** : appliquer un malus faisant repasser le membre sous son seuil → le **badge de niveau** suit à la baisse, les **badges de succès restent acquis**.
8. **Profil public** : ouvrir `/profil/{id}` en visiteur non connecté → badge de niveau et badges obtenus visibles, **aucun** solde, **aucune** ligne de journal (FR-014).

## S4 — Actions médias récompensées (US4 · SC-006, SC-007, SC-011)

Pour chacune des 4 actions, vérifier le **crédit unique**, la **catégorie « Médias »** et l'**absence d'auto-attribution** :

| Action | Parcours | Contrôle spécifique |
|---|---|---|
| Proposition validée | membre propose une chaîne → admin valide dans `/admin/medias/propositions` | rejouer la validation ne recrédite pas ; si l'admin est l'auteur → **0 point** |
| Mise à la une | admin met un programme à la une, la **retire**, la **repose** | crédité **une seule fois** pour ce contenu |
| Animation acceptée | demande d'animation acceptée **par un co-détenteur** puis, sur une autre demande, **par le back-office** | même montant dans les deux chemins, une seule fois chacun |
| Popularité médias | faire aimer un programme jusqu'à un palier | palier crédité une fois ; le **like de l'auteur ne compte pas** ; le compteur affiché reste le total réel |

Contrôle d'idempotence global (SC-006) :

```sql
SELECT cle_idempotence, COUNT(*) FROM engagement.mouvement_points
 GROUP BY cle_idempotence HAVING COUNT(*) > 1;   -- doit renvoyer 0 ligne
```

**Non-régression (SC-007, SC-011)** : arrêter PostgreSQL une fraction de seconde pendant une validation de proposition (ou désactiver toutes les règles) → l'action métier **réussit**, seule l'attribution est loguée en erreur (`RUST_LOG=info`). Puis re-vérifier qu'à barème inchangé, les actions déjà récompensées avant la feature (contribution Codimoi validée, factcheck, mise en avant) créditent exactement les mêmes montants qu'avant.

## S5 — Partage externe (US5 · SC-008)

1. Ouvrir une modale de partage : vérifier que **6 réseaux** sont proposés (WhatsApp, Facebook, X, LinkedIn, **Telegram**, **E-mail**) — sans Telegram ni E-mail, le seuil de 5 est inatteignable (R10).
2. Partager un même contenu vers 5 réseaux distincts → bonus crédité **une fois** ; le 6ᵉ réseau ne crédite rien de plus.
3. Répéter le même réseau 5 fois → **aucun** bonus :

```sql
SELECT reseau, COUNT(*) FROM engagement.partage_externe
 WHERE utilisateur_id = '<uuid>' GROUP BY reseau;   -- 1 ligne max par réseau
```

4. **Robustesse** : couper le backend, cliquer un partage → la fenêtre du réseau s'ouvre normalement, aucune erreur visible pour l'utilisateur.
5. **Écrêtage** (SC-008) : enchaîner 4 contenus complets à 5 réseaux le même jour (plafond 30 points = 3 bonus) → le 4ᵉ n'est pas crédité et l'historique affiche « plafond atteint, aucun point crédité ».

## S6 — Réactivité perçue (SC-004)

Garder l'espace « Mon engagement » ouvert dans un onglet, déclencher une action récompensée dans un autre, recharger : le mouvement est visible **en moins de 5 secondes** (l'attribution est synchrone post-commit, il n'y a aucune file d'attente).

---

## Checklist de sortie

- [ ] S1 → S6 verts sur les deux comptes de test
- [ ] Les 2 requêtes de contrôle de doublons (`cle_idempotence`, `badge_obtenu`) renvoient 0 ligne
- [ ] `regles_sans_categorie = 0`
- [ ] `getDiagnostics` propre (rust-analyzer + Volar) sur tous les fichiers touchés
- [ ] `/admin/audit` contient une entrée par mutation de barème effectuée pendant la campagne
- [ ] Aucune classe daisyUI dans `pages/mon-compte/engagement.vue` ni dans `components/engagement/` (Principe VI)
- [ ] Aucun libellé de barème écrit en dur dans le front (tout vient de l'API)
