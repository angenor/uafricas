# Quickstart : Validation manuelle

**Feature**: 001-ressources-fermeture-session
**Pré-requis**: Docker compose lancé (`docker compose up -d`), backend (`RUST_LOG=info cargo run`), frontend (`pnpm dev`), schéma SQL ré-appliqué après édition de `08b_afrolang.sql` (drop volume `docker compose down -v` + remontée si dev local).

Comptes test :
- `test-admin@test.com` / `Test1234` (admin plateforme)
- `test-user@test.com` / `Test1234` (utilisateur lambda)

---

## Scénario A : Ajout de ressources contribuées (US1)

1. **Préparation** : avec `test-admin`, créer (ou réutiliser) une salle publique Afrolang (ex : groupe ethnique « Bambara »). Démarrer une session via la page `/afrolang/salle/<id>`.
2. Se déconnecter, se connecter avec `test-user`. Rejoindre la même session.
3. Cliquer sur l'onglet **« Ressources contribuées »**. Vérifier la coexistence avec la section « Ressources officielles » (modérée, existante).
4. Cliquer **+ Ajouter une ressource**.
5. **Onglet Document** : choisir un PDF < 20 Mo, saisir titre « Bases bambara » + description. Soumettre. ✅ Doit apparaître immédiatement dans la liste, avec auteur = test-user, et le bouton de téléchargement fonctionnel.
6. **Onglet Vidéo** : coller `https://www.youtube.com/watch?v=dQw4w9WgXcQ`. Titre « Leçon 1 ». Soumettre. ✅ Apparait avec vignette + embed YouTube cliquable.
7. **Onglet Vidéo erreur** : coller `https://vimeo.com/1234`. Soumettre. ❌ Erreur attendue : « Seules les URLs YouTube sont acceptées ».
8. **Onglet Accompagnateur** : sélectionner test-admin. Motif « Locuteur natif, formateur depuis 8 ans ». Soumettre. ✅ Toast « Recommandation envoyée, en attente d'acceptation ». La carte n'est **pas** visible publiquement (vérifier avec un troisième compte si dispo).
9. Se reconnecter avec **test-admin**. Aller dans `/mon-compte/recommandations-accompagnateur`. Vérifier la présence de la recommandation en état `en_attente`.
10. Cliquer **Accepter**. ✅ Statut passe à `acceptee` ; retourner sur la session de la salle (test-user ou anonyme), la carte d'accompagnateur est désormais visible publiquement avec lien vers le profil.
11. Toujours en test-admin : retirer le consentement. ✅ La carte disparaît instantanément.

## Scénario B : Persistance au niveau salle (US1 acceptance #6)

1. Avec test-user, ajouter 2 ressources (1 document, 1 lien web) dans la session A.
2. Mettre fin à la session A (test-admin termine la session).
3. Avec test-admin, démarrer une **nouvelle session** dans la même salle.
4. Rejoindre avec test-user. Vérifier que les 2 ressources de la session A apparaissent toujours dans l'onglet « Ressources contribuées ». ✅

## Scénario C : Salle privée, visibilité restreinte (FR-001 option C)

1. Avec test-admin, créer une salle privée (code d'accès `BAMBARA2026`) sous la salle publique « Bambara ».
2. Avec test-user, accéder à la fiche de la salle privée et saisir le code. ✅ Accès accordé. Une ligne `acces_salle_privee` doit exister (vérifier via Adminer).
3. Démarrer une session privée, ajouter une ressource document.
4. Quitter la session, attendre l'expiration du JWT (4 h) ou simuler en supprimant le cookie `access_token` côté navigateur.
5. Reconnecter test-user. Accéder directement à `/afrolang/session/privee/<id>` → **lecture des ressources autorisée** (acces_salle_privee persistant). ✅
6. Avec un **troisième utilisateur** (ou en navigation privée) n'ayant jamais validé le code, tenter d'accéder à la liste des ressources via l'API directe. ✅ Doit renvoyer **403 `salle_privee_acces_requis`**.
7. Avec test-admin (créateur de la salle privée), changer le code via `PATCH /salles-privees/{id}/code-acces`. ✅ La ligne `acces_salle_privee` de test-user passe à `revoque_at IS NOT NULL`. Test-user retentant l'accès → 403.

## Scénario D : Fermeture admin et désactivation salle (US2)

1. Avec test-user, démarrer une session dans une salle publique. Rejoindre avec un second compte (créer test-user2 si nécessaire).
2. Avec test-admin, aller dans `/admin/afrolang/sessions`. Repérer la session en cours.
3. Cliquer **Fermer pour abus**. Saisir un motif (≥ 10 chars). Confirmer.
4. ✅ Côté test-user / test-user2 : la session se coupe immédiatement, un **toast persistant** « Session fermée par l'administration » s'affiche.
5. ✅ Côté UI : la fiche de la salle (`/afrolang/salle/<id>`) affiche le badge **« Désactivée par administration »**, le bouton « Démarrer la session » est désactivé.
6. Tenter de re-rejoindre la salle → message d'erreur explicite.
7. Avec test-user, tenter d'ajouter une ressource contribuée → **409 `salle_desactivee_admin`** (FR-010).
8. Avec test-user (non admin), tenter `POST /admin/afrolang/salles/<id>/reactiver` → **403**.
9. Avec test-admin, ouvrir l'onglet **Historique de modération** de la salle. ✅ La fermeture apparaît avec auteur, motif, horodatage.
10. Cliquer **Réactiver la salle**. Saisir un commentaire facultatif. ✅ La salle redevient utilisable ; un évènement `reactivation_admin` apparaît dans l'historique.

## Scénario E : Rate-limit ressources (FR-011)

1. Avec test-user, ajouter 10 ressources dans la même salle en < 24 h (peu importe le type).
2. À la 11ᵉ tentative, l'API doit renvoyer **429 `rate_limit_ressources`**. ✅

## Scénario F : Audit (Principe VII)

1. Via Adminer, ouvrir la table `audit.log_action` (ou nom équivalent).
2. Vérifier qu'à chaque action ci-dessus correspond une ligne :
   - Ajout ressource → `CREATE ressource_contribuee`
   - Suppression ressource → `DELETE ressource_contribuee`
   - Accepter/refuser/retirer accompagnateur → `UPDATE ressource_contribuee` avec `after` reflétant le nouveau statut
   - Fermeture admin → 2 lignes (`UPDATE salle` + `CREATE evenement_moderation_salle`)
   - Réactivation admin → 2 lignes (idem)
   - Validation code d'accès → `CREATE acces_salle_privee` (uniquement à la première validation)
   - Changement code d'accès → `UPDATE acces_salle_privee` (révocations cascade)

---

## Critères de sortie

- [ ] Tous les scénarios A→F passent à la première exécution.
- [ ] Aucune erreur 500 dans les logs `cargo run`.
- [ ] Aucune erreur console côté navigateur sur les pages modifiées (`/afrolang/session/[id]`, `/admin/afrolang/sessions`, `/admin/afrolang/salles/[id]`, `/mon-compte/recommandations-accompagnateur`).
- [ ] Le badge « Désactivée par administration » respecte Tailwind v4 (pas de classe daisyUI côté public).
- [ ] Les composants admin utilisent daisyUI v5 (vérifier `btn`, `modal`, `tabs`, `badge`).
- [ ] Performance : la liste de 50 ressources se charge < 500 ms en local.
