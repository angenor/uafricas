# Quickstart : Vérification manuelle du système d'engagement (Phase 1)

Objectif : dérouler la boucle complète « action mesurable → points → statut → affichage » et prouver les invariants (idempotence, plafond, plancher, badge). Pas de harnais de test automatisé dans le projet ; vérification manuelle + `cargo check`.

## Prérequis

```bash
docker compose up -d                      # PostgreSQL + Adminer + LiveKit
# Appliquer la migration engagement (dev) :
#   via docker-init.sh au premier init, ou manuellement :
#   psql "$DATABASE_URL" -f uafricas_backend/doc/bd/schemas/NN_engagement.sql
kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run   # backend
pnpm dev                                   # frontend (port 3000)
```

Comptes de test : `test-admin@test.com` / `Test1234`, `test-user@test.com` / `Test1234`.

## Scénarios

### S1 : Contribution validée (US1, FR-011)
1. Avec `test-user`, créer une contribution Codimoi (récit/proverbe).
2. Avec `test-admin`, la **valider** en modération.
3. `GET /api/engagement/mon-compte` (en tant que `test-user`) → `solde_points` = +2 ; un mouvement `contribution_validee/codimoi` apparaît dans `mon-journal`.
4. **Idempotence** : re-déclencher la validation → solde inchangé, pas de second mouvement (SC-004).
5. **Anti-auto-attribution** : `test-admin` valide sa **propre** contribution → aucun point (FR-009).

### S2 : Mise en avant (FR-012)
1. `test-admin` marque une contribution validée de `test-user` comme **« mise en avant »**.
2. `test-user` reçoit +5 (`contribution_mise_en_avant`) en plus, tracé au journal.

### S3 : FactCheck correct puis faux (US4, FR-013/014, D7)
1. `test-user` soumet un FactCheck ; `test-admin` le juge **correct** → +3 points, +1 réputation.
2. Un autre FactCheck de `test-user` jugé **faux** → −2 points (jamais sous 0), −3 réputation.
3. Vérifier que la réputation peut baisser indépendamment du solde, et que le solde reste ≥ 0 même si le malus dépasse le solde (plancher, D7).

### S4 : Palier de popularité (US3, FR-015/016)
1. Faire « aimer » une publication de `test-user` par ≥ 100 comptes distincts (ou insérer des lignes de réaction en base via Adminer).
2. Au franchissement de 100 likes → `test-user` reçoit +10 une **seule fois** (`popularite:codimoi:{id}:100`).
3. Retirer puis remettre des likes autour de 100 → **aucun** nouveau gain (idempotence, FR-016).
4. Atteindre 500 → +30 (palier 500 seulement) ; l'auto-like de `test-user` n'est pas compté (FR-017).

### S5 : Niveau & badge (US2, FR-002/018/019/020)
1. Amener `test-user` à ≥ 200 points → `mon-compte` renvoie `niveau.code = "premium"` sans action manuelle.
2. Ouvrir `/mon-compte/profil` onglet **« Mes points »** → solde global/mensuel, réputation, badge Premium, historique visibles.
3. Ouvrir le **profil public** de `test-user` → badge affiché ; le journal détaillé n'est **pas** exposé (seul `GET /api/engagement/niveau/{id}` répond).

### S6 : Plafond journalier (FR-010, SC-007)
1. Admin : `PUT /api/admin/engagement/regles/{id}` sur `contribution_validee`, mettre `plafond_journalier = 4`.
2. Faire valider 3 contributions (+2 chacune) → au 3ᵉ, le gain est **écrêté** (mouvement `plafond_atteint = true`, `points` partiel) ; le solde ne dépasse pas +4 ce jour-là.

### S7 : Barème sans redéploiement (US5, FR-022, SC-005)
1. Admin : changer `contribution_validee.points` de 2 → 3 et enregistrer.
2. Valider une nouvelle contribution → +3 appliqué **sans** recompilation/redéploiement, en < 2 min. Le changement figure dans l'**audit** (`log_action`).

### S8 : Ajustement manuel & journal global (US5)
1. Admin : `POST /api/admin/engagement/ajustement` `{ points: -10, motif: "…" }` sur `test-user`.
2. `GET /api/admin/engagement/journal?utilisateur_id=…` → le mouvement `ajustement_admin` apparaît avec le motif ; entrée d'audit créée.

## Critères de sortie (mapping Success Criteria)

- S1–S4 verts ⇒ SC-001 (< 5 s), SC-002 (100 % tracé), SC-004 (0 doublon).
- S3/S6 verts ⇒ plancher + écrêtage (SC-007).
- S5 vert ⇒ SC-006 (niveau/badge auto).
- S7 vert ⇒ SC-005 (barème à chaud).
- Aucune action métier (validation, like, jugement) ne renvoie une erreur imputable au moteur ⇒ SC-003.
