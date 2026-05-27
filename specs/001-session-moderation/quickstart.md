# Phase 1 — Quickstart : Modération de session Afrolang

**Date** : 2026-05-10
**Prérequis** : Docker (postgres + livekit) lancé, backend Actix sur port 8080, frontend Nuxt sur port 3000, schéma SQL `08b_afrolang.sql` réappliqué (ou migration manuelle pour les 3 colonnes + 1 table).

## Préparation des utilisateurs de test

- **`admin@test.com / Test1234`** — admin plateforme (rôle global).
- **`user2@test.com / Test1234`** — utilisateur lambda (sera créateur de salle privée).
- Créer 3 utilisateurs supplémentaires via inscription (`alice@test.com`, `bob@test.com`, `carole@test.com`) pour servir de participants.

## Application de la migration BD

```bash
docker compose exec -T postgres psql -U uafricas -d africans_db < uafricas_backend/doc/bd/schemas/08b_afrolang.sql
# (ré-exécution idempotente non garantie pour les ALTER ; en cas d'échec, appliquer manuellement
#  la nouvelle table et les 3 colonnes via psql interactif)
```

## Scénario 1 — Permissions tableau blanc en salle privée (FR-010, FR-011, US3)

1. Se connecter en tant que `user2@test.com`.
2. Créer une salle privée Afrolang ; noter le code d'accès.
3. Démarrer une session dans cette salle. Ouvrir l'interface : le panneau « Permissions tableau blanc » DOIT être visible (US3 — le créateur est reconnu comme modérateur).
4. Dans un autre navigateur (ou onglet privé), se connecter en tant que `alice@test.com`, entrer dans la salle privée avec le code d'accès et rejoindre la session.
5. Côté `user2` : ouvrir le panneau, vérifier qu'Alice apparaît dans la liste « Participants », et que sa permission est désactivée par défaut.
6. Côté Alice : confirmer que la barre d'outils du tableau blanc est désactivée avec un libellé « lecture seule ».
7. Côté `user2` : activer la permission d'Alice → vérifier que **dans un délai < 2 s** la barre d'outils d'Alice s'active (SC-002).
8. Côté Alice : dessiner un trait → le trait apparaît chez `user2`.
9. Côté `user2` : retirer la permission → Alice repasse en lecture seule, le trait existant **reste affiché** chez tous (FR-016).

**Résultat attendu** : 100 % des actions propagent en < 2 s ; aucune perte de contenu existant.

## Scénario 2 — Refus serveur des opérations non autorisées (FR-015)

1. Reprendre l'état du scénario 1 (Alice non autorisée).
2. Côté Alice, ouvrir les DevTools → tenter de publier manuellement un DataPacket whiteboard via `room.localParticipant.publishData(...)`.
3. **Résultat attendu** : LiveKit rejette le packet (visible dans la console : `Permission denied: cannot publish data`). Aucune trace du packet n'apparaît chez `user2`.

## Scénario 3 — Permission préservée à la reconnexion (Edge case « reconnexion »)

1. Reprendre l'état du scénario 1 avec Alice autorisée.
2. Côté Alice : couper la connexion réseau pendant 10 s, puis la rétablir.
3. **Résultat attendu** : à la reconnexion, Alice retrouve la barre d'outils active sans intervention de `user2`.

## Scénario 4 — Mise en évidence en session publique (US2, FR-020, FR-021)

1. Se connecter en tant que `admin@test.com` (admin plateforme).
2. Démarrer une session sur une salle publique existante. Bob et Carole rejoignent la session avec caméra active.
3. Côté admin : ouvrir le panneau modération, cliquer « Mettre en évidence Bob ».
4. **Résultat attendu** : chez tous les participants, la tuile vidéo de Bob est agrandie au centre, avec une bordure `custom-chocolat` et le libellé « En vedette », dans un délai < 2 s.
5. Côté admin : cliquer « Mettre en évidence Carole ».
6. **Résultat attendu** : transfert immédiat — Bob revient à la normale, Carole est mise en avant.
7. Côté admin : cliquer « Désactiver la mise en évidence ».
8. **Résultat attendu** : disposition mosaïque normale rétablie chez tous.

## Scénario 5 — Mise en évidence non disponible en privé (FR-027)

1. Reprendre la salle privée du scénario 1 (créée par `user2`).
2. Démarrer une session ; `user2` ouvre le panneau modération.
3. **Résultat attendu** : aucun bouton ou option « Mettre en évidence » n'est rendu dans l'interface. L'endpoint REST `POST /spotlight` renvoie `422` si appelé directement.

## Scénario 6 — Modérateur attitré sans pouvoir spotlight (FR-001b)

1. Se connecter en tant que `admin@test.com`. Dans `/admin/afrolang/salles/{id}`, nommer Bob comme **modérateur attitré** d'une salle publique.
2. Bob se connecte et rejoint une session sur cette salle.
3. Bob ouvre le panneau modération : **doit voir** le panneau permissions tableau blanc, **ne doit pas voir** la section spotlight.
4. Bob teste l'endpoint `POST /spotlight` directement (DevTools) → **403 Forbidden** attendu.
5. Bob accorde une permission tableau blanc à Carole → ça marche.

## Scénario 7 — Cascade au départ de la personne mise en évidence (FR-025)

1. Reprendre le scénario 4 avec Bob mis en évidence.
2. Bob ferme l'onglet (déconnexion).
3. **Résultat attendu** : la mise en évidence est automatiquement levée chez tous les participants en < 2 s (DataPacket `spotlight: null`). La disposition normale est rétablie.

## Validation finale

- Vérifier via Adminer (`http://localhost:8088`) que la table `afrolang.session_permission_tableau_blanc` est vide après clôture de la session (CASCADE) et que `afrolang.session.participant_mis_en_evidence_id` est repassé à `NULL`.
- Vérifier dans `/admin/audit` que chaque action de modération a généré une ligne d'audit (action `CREATE` / `DELETE` / `UPDATE`, ressource `session_permission_tableau_blanc` ou `session`, before/after JSONB cohérent).

## Critères de succès (mapping)

| Scénario | FR couverts | SC couverts |
|---|---|---|
| 1 | FR-010, FR-011, FR-014, FR-018, US3 | SC-001, SC-002, SC-005 |
| 2 | FR-015 | SC-001 |
| 3 | Edge case reconnexion | — |
| 4 | FR-020, FR-021, FR-023, FR-026 | SC-002 |
| 5 | FR-027 | — |
| 6 | FR-001b | — |
| 7 | FR-025 | — |
