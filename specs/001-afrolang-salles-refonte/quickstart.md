# Quickstart : Validation manuelle de la refonte

**Branch** : `001-afrolang-salles-refonte`
**Date** : 2026-04-15

Le projet n'a pas de framework de test configuré (cf. CLAUDE.md). Cette procédure décrit la validation manuelle bout-en-bout après implémentation.

---

## Pré-requis

- Docker en route : `docker compose up -d` (PostgreSQL + Adminer + LiveKit).
- Backend recompilé : `kill $(lsof -i :8080 -t) 2>/dev/null; cd uafricas_backend && RUST_LOG=info cargo run`.
- Frontend démarré : `cd uafricas_frontend && pnpm dev`.
- BDD réinitialisée pour table rase legacy (Q2) :

  ```bash
  docker compose down -v && docker compose up -d
  # Attendre ~10 s que docker-init.sh re-joue 08b_afrolang.sql refondu
  ```

- Comptes test (cf. CLAUDE.md) :
  - `admin@test.com` / `Test1234` (admin pour créer la salle publique)
  - `user2@test.com` / `Test1234` (utilisateur lambda)
  - Un troisième compte à créer pour valider les conflits d'unicité.

---

## Scénario 1 : Page `/afrolang` épurée (FR-001, FR-002, US1)

1. Se connecter en tant qu'`admin`. Aller dans l'admin Afrolang et créer une salle publique « Wolof, Soirée découverte » sur le groupe ethnique Wolof.
2. Se déconnecter, se reconnecter en tant qu'`user2`.
3. Aller sur `/afrolang`.
4. **Vérifier** : aucune section « Annuaire des groupes ethniques » n'est visible (audit DOM).
5. **Vérifier** : la salle publique « Wolof, Soirée découverte » apparaît dans « Toutes les salles publiques » avec un bouton « Démarrer ».
6. Cliquer sur « Démarrer ».
7. **Vérifier** : navigation directe vers `/afrolang/session/{salle_id}` (ou route équivalente) sans page intermédiaire ; LiveKit charge ; user2 entre comme participant.

---

## Scénario 2 : Création salle privée depuis le widget Canal privé (FR-008, FR-009, US4)

1. Toujours connecté en `user2`, retour à `/afrolang`.
2. Sur la carte « Wolof : Soirée découverte », ouvrir le dropdown « Canal privé ».
3. **Vérifier** : un bouton « Créer ma salle privée » est présent (l'utilisateur n'en a pas encore).
4. Cliquer ; le modale `SallePriveeCreateModal` s'ouvre.
5. Saisir : titre = « Cercle Wolof débutant », code secret = `wolof2026`, description = « Pratique douce ».
6. Soumettre.
7. **Vérifier** (réseau) : `POST /api/afrolang/salles-privees` → 201, payload retourne `id`, `auteur_nom: "Vous"`, `session_en_cours: false`.
8. **Vérifier** (UI) : la salle apparaît dans le dropdown ; le bouton de création devient « Ouvrir ma salle privée ».

---

## Scénario 3 : Conflit d'unicité (FR-010, SC-005)

1. Toujours en `user2`, tenter de créer une seconde salle privée pour la même salle publique (via le bouton ou un appel API direct).
2. **Vérifier** : `POST /api/afrolang/salles-privees` → 409, message « Vous avez déjà une salle privée pour cette salle publique », `data.salle_privee_existante_id` renseigné.
3. **Vérifier** (UI) : le frontend bascule vers « Ouvrir ma salle privée ».

---

## Scénario 4 : Accès par code secret (FR-013, US3)

1. Se déconnecter, se reconnecter en tant qu'`admin` (autre utilisateur que l'auteur).
2. Aller sur `/afrolang`, ouvrir le dropdown « Canal privé » de « Wolof, Soirée découverte ».
3. **Vérifier** : la salle privée « Cercle Wolof débutant » est listée avec auteur « user2 ».
4. Cliquer dessus → `SallePriveeJoinModal` s'ouvre, demandant le code.
5. Saisir un code incorrect, soumettre.
6. **Vérifier** : 403, message « Code incorrect », saisie reste ouverte.
7. Saisir le bon code `wolof2026`.
8. **Vérifier** : 200, navigation vers la session live de la salle privée. Si `user2` n'y est pas, `admin` démarre la session (rôle participant ; modérateur effectif = `user2` selon BDD).

---

## Scénario 5 : Auteur entre sans code (FR-014)

1. Se reconnecter en tant que `user2`.
2. Ouvrir le dropdown Canal privé sur la salle publique parente.
3. Cliquer « Ouvrir ma salle privée ».
4. **Vérifier** : aucune saisie de code n'est demandée ; entrée directe dans la session.

---

## Scénario 6 : Rate limit (R4)

1. Se reconnecter en `admin`.
2. Tenter 6 fois de suite la saisie d'un code incorrect sur la salle privée de `user2`.
3. **Vérifier** :
   - Tentatives 1 à 5 : 403 « Code incorrect ».
   - Tentative 6 : 429, message « Trop de tentatives, réessayez dans quelques minutes ».
4. Attendre ~5 minutes ; retenter avec le bon code.
5. **Vérifier** : accès rétabli, 200.

---

## Scénario 7 : Indépendance salle privée ↔ salle publique (FR-018)

1. Mettre fin à toute session live de la salle publique « Wolof, Soirée découverte ».
2. En tant que `user2`, ouvrir « Ma salle privée » via le dropdown.
3. **Vérifier** : la session privée démarre normalement, indépendamment de l'absence de session publique.

---

## Scénario 8 : Suppression page `/afrolang/salle-privee/[id].vue` (FR-006, SC-007)

1. Tenter d'accéder directement via URL : `http://localhost:3000/afrolang/salle-privee/{uuid-de-cercle-wolof}`.
2. **Vérifier** : redirection (vers `/afrolang/{salle_id}` ou `/afrolang`), pas d'erreur 500. Idéalement : 404 ou middleware Nuxt qui catch la route inexistante.
3. **Vérifier** (filesystem) : `uafricas_frontend/app/pages/afrolang/salle-privee/` n'existe plus.

---

## Scénario 9 : Endpoints legacy supprimés

`curl` chacun de ces endpoints (avec JWT valide) et vérifier 404 / 405 :

```bash
curl -i -X GET    http://localhost:8080/api/afrolang/salles-privees/<id>/adhesions
curl -i -X POST   http://localhost:8080/api/afrolang/salles-privees/<id>/inviter
curl -i -X PATCH  http://localhost:8080/api/afrolang/salles-privees/<id>/visibilite
curl -i -X GET    http://localhost:8080/api/afrolang/propositions-salle
```

Tous DOIVENT renvoyer 404 (route inexistante) ou 405.

---

## Scénario 10 : Audit (Principe VII)

1. Après les scénarios 2, 4, 6, 7, ouvrir Adminer → table `shared.audit_log` (ou équivalent).
2. **Vérifier** : présence d'événements `creer_salle_privee`, `verifier_code_salle_privee_echec`, `rejoindre_session_salle_privee`, avec IP, user_agent, before/after JSONB.

---

## Build sanity checks

```bash
# Backend compile
cd uafricas_backend && cargo check
# Frontend type check (rapide, sans test)
cd uafricas_frontend && pnpm typecheck   # si script existe ; sinon pnpm build
```

---

## Done quand

- [ ] Tous les scénarios 1 → 10 passent.
- [ ] `cargo check` et `pnpm build` sans erreur.
- [ ] Diff git : aucun fichier legacy oublié dans `app/components/afrolang/` ou `src/handlers/admin/`.
- [ ] Constitution Check post-implémentation re-validée (notamment Principe IV, code secret jamais en clair en BDD ni dans l'audit).
