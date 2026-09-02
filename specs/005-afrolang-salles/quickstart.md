# Quickstart : Validation manuelle de la feature 005-afrolang-salles

Pas de framework de tests automatisés (constitution UAfricas). Ce document sert de **check-list de validation manuelle** alignée sur les User Stories et critères de succès du spec.

## Pré-requis d'exécution

```bash
# 1. Docker infra (PostgreSQL + Adminer + LiveKit)
cd /Users/mac/Documents/projets/uafricas_projets/uafricas
docker compose up -d

# 2. Appliquer le schéma SQL mis à jour (tables + enums + colonnes)
#    L'ajout au fichier doc/bd/schemas/08b_afrolang.sql sera appliqué à l'init ;
#    pour un cluster déjà en cours, exécuter via Adminer le delta DDL du data-model.md.

# 3. Backend (port 8080)
cd uafricas_backend
kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run

# 4. Frontend (port 3000)
cd uafricas_frontend
pnpm dev
```

Comptes de test (cf. `CLAUDE.md`) :

- `admin@test.com` / `Test1234`
- `user2@test.com` / `Test1234`
- (créer au besoin un 3e compte `user3@test.com` pour tester invitations/adhésions)

---

## Scénario 1 : User Story 1 (P1) : accéder à la salle publique de son groupe

**Prérequis** : au moins une salle publique rattachée à un groupe ethnique existe (par exemple « Gurunsi »).

1. Se connecter avec `user2@test.com`.
2. Aller sur `/afrolang`.
3. ✅ L'annuaire des groupes ethniques s'affiche (piloté par `GET /afrolang/groupes-ethniques`).
4. Cliquer sur « Gurunsi ».
5. ✅ La salle publique s'ouvre en **<10 s** (SC-001). Les onglets **Visio, Chat, Ressources, Tableau blanc** sont disponibles.
6. Ouvrir « Ressources » → l'alphabet (et le dictionnaire s'il existe) sont visibles (SC-008).
7. Envoyer un message dans le Chat → l'horodatage et l'auteur apparaissent.
8. Ouvrir un 2e onglet incognito, se connecter `user3@test.com`, rejoindre la même salle → tracer sur le tableau blanc : ✅ le trait apparaît chez `user2` en <500 ms (SC-004).

---

## Scénario 2 : User Story 2 (P1) : proposer une salle absente

1. Connecté en `user2@test.com`, `/afrolang` → barre de recherche « Zulu » (absent).
2. Cliquer sur « Proposer cette salle » → modal `ProposerSalleModal.vue`.
3. Renseigner `nom_groupe_ethnique=Zulu`, `langue_cible=isiZulu`, description.
4. Soumettre → ✅ message « votre proposition sera examinée sous 7 jours ouvrés » (FR-004, SC-002) ; apparaît dans `/afrolang/proposer` avec état `en_attente`.
5. Se déconnecter, se reconnecter en `admin@test.com`.
6. Aller sur `/admin/afrolang/propositions` → la proposition apparaît.
7. Cliquer « Approuver », compléter `groupe_ethnique_id` (choix dans la liste du référentiel `country_profile.groupe_ethnique`).
8. ✅ La salle est créée, le proposant reçoit une notification (module `iam.notification`).
9. Reconnecté en `user2`, la salle « Zulu » est visible et rejoignable.

**Test négatif (doublon)** : soumettre une autre proposition avec le même nom → `409 Conflict` et pointeur vers la proposition existante (FR-007).

---

## Scénario 3 : User Story 3 (P1) : modération de session

### 3.1 Premier arrivé = modérateur de session

1. Admin désigne aucun modérateur attitré sur une salle (ex. « Fon »).
2. `user2` rejoint → ✅ `session_participant.role_session='moderateur'`, UI affiche « Vous modérez cette session ».

### 3.2 Transfert manuel

1. `user3` rejoint la même session.
2. `user2` clique « Transférer la modération » → choisit `user3`.
3. ✅ `user3` devient modérateur actif, `user2` perd l'état. Notification aux deux.

### 3.3 Reprise automatique par un modérateur Afrolang attitré

1. Admin désigne `user2` comme `salle_moderateur.actif=TRUE` sur la salle « Fon ».
2. `user3` (modérateur de session actuel) reste dans la session. `user2` rejoint.
3. ✅ Serveur détecte l'arrivée d'un attitré, `UPDATE session.moderateur_id = user2`, notifications aux deux en <5 s (SC-003).

### 3.4 Départ du modérateur actif

1. `user2` (modérateur actuel) quitte la session alors qu'il reste `user3` actif.
2. ✅ Réattribution automatique à `user3`.

---

## Scénario 4 : User Story 4 (P2) : création d'une salle privée

1. `user2` entre dans la salle publique « Gurunsi ».
2. ✅ Bouton permanent « Créer une salle privée » visible ; info-bulle de découverte à la 1ère visite uniquement (FR-013, confirmation Q3).
3. Clic → `SallePriveeCreateModal.vue` → sélectionner motif **« Apprentissage par mes enfants »** → ✅ notice d'alerte « un adulte doit être présent auprès des enfants » apparaît (FR-017).
4. Valider sans cocher la case « je déclare être majeur » → ✅ bloqué, message explicite (FR-016, SC-005).
5. Cocher la déclaration + soumettre → ✅ salle privée créée, `declaration_adulte_at` horodaté (FR-033).
6. Consulter la liste des salles privées rattachées à « Gurunsi » depuis `/afrolang/{salle_id}` → la salle privée y figure (FR-018).

**Test négatif d'unicité** (FR-035, SC-010) :

7. Sans archiver la première, `user2` clique à nouveau « Créer une salle privée » → motif `echanges_groupe`, déclaration cochée → ✅ `409 Conflict` : message « vous avez déjà une salle privée active dans cette salle publique, archivez-la avant d'en créer une nouvelle ».
8. ✅ `user2` peut néanmoins créer une salle privée dans **une autre** salle publique (ex. « Fon »).

---

## Scénario 5 : User Story 5 (P2) : visibilité et adhésions

### 5.1 Salle privée fermée + invitation directe

1. `user2` crée une salle privée `fermee` sur « Gurunsi » (motif `echanges_groupe`), limite 3 participants.
2. `user3` navigue sur « Gurunsi » → ✅ la salle privée **n'apparaît pas** (FR-019).
3. `user2` invite `user3` → notification reçue.
4. `user3` **refuse** l'invitation → ✅ `etat='refusee'`, `user2` notifié (FR-025).
5. `user2` invite `user3` à nouveau → `user3` **accepte** → ✅ `user3` devient `abonne`.

### 5.2 Salle privée visible + demande d'adhésion

1. `user2` crée une 2e salle privée rattachée à **« Fon »** (autre salle publique, motif `reseautage_adulte`), visibilité `visible`, limite 2.
2. `user3` sur « Fon » voit la salle privée → demande d'adhésion (FR-021).
3. `user2` la refuse → ✅ `user3` notifié.
4. `user3` redemande → `user2` accepte → `user3` devient `abonne`.

### 5.3 Refus automatique « groupe complet »

1. Créer un 4e compte `user4@test.com`.
2. `user4` tente une demande sur la même salle désormais pleine (max=2 atteint) → ✅ serveur insère `etat='groupe_complet'`, message explicite renvoyé (FR-024, SC-006).
3. Test de concurrence (optionnel) : deux onglets, deux demandes simultanées alors que 1 place reste → une seule acceptée, l'autre `groupe_complet` (verrou `FOR UPDATE` en transaction).

---

## Scénario 6 : User Story 6 (P2) : tableau blanc, ressources, chat

### 6.1 Tableau blanc temps réel

1. Deux onglets (2 utilisateurs) en session.
2. Utilisateur A trace un trait → ✅ visible chez B via canal data LiveKit en <500 ms (SC-004).
3. Utilisateur A (modérateur) clique « Effacer » → ✅ le canevas est remis à zéro chez tous (FR-027).
4. Fermer la session → ✅ le dernier snapshot persiste dans `afrolang.tableau_blanc`.

### 6.2 Ressources : fichier interne

1. `user2` (modérateur Afrolang attitré sur « Gurunsi ») ajoute une ressource fichier (upload PDF).
2. ✅ `etat='publiee'` immédiatement, visible par tous (FR-028).

### 6.3 Ressources : lien externe modéré

1. `user3` (simple membre) soumet un lien externe `https://dictionnaire.example/gurunsi` via `POST /ressources/lien`.
2. ✅ `etat='en_attente_validation'`, invisible aux autres membres mais visible pour `user3` dans son suivi.
3. Admin (ou `user2` en tant que modérateur Afrolang attitré) va sur `/admin/afrolang/liens-externes` → clique « Publier » → ✅ `etat='publiee'`, médiane de traitement <72 h (SC-009).

### 6.4 Chat

1. `user2` envoie « Bonjour » → `user3` reçoit en temps réel via data channel + la base contient la ligne (`message_session`).
2. `user3` se déconnecte, rejoint à nouveau → `GET /sessions/{id}/messages` renvoie l'historique (reprise).

---

## Scénario 7 : Edge Case : archivage automatique du créateur supprimé

1. Admin désactive `user2` (via `/admin/utilisateurs/{id}/etat` → `desactive`).
2. Hook / appel explicite `POST /admin/afrolang/salles-privees/archiver-batch-utilisateur { utilisateur_id=user2 }`.
3. ✅ Toutes les salles privées actives créées par `user2` passent `archivee_at=NOW()`, les participants reçoivent une notification (FR-034).
4. ✅ Les salles archivées ne peuvent plus démarrer de session (bouton « Démarrer » masqué).

---

## Checklist de conformité constitution

Avant toute PR :

- [ ] Le SQL a été modifié en premier (Principe III), puis Rust, puis TypeScript.
- [ ] Tous les nouveaux identifiants (colonnes, enums, structs, composants) sont en français (Principe I).
- [ ] Chaque mutation admin utilise `audit::log_action` (Principe VII).
- [ ] Les pages publiques `/afrolang/**` n'utilisent **aucune** classe daisyUI (Principe VI).
- [ ] Les pages admin `/admin/afrolang/**` peuvent utiliser daisyUI v5 (Principe VI).
- [ ] JWT requis sur tous les endpoints mutants (Principe IV).
- [ ] Uploads de fichiers passent par `sanitize-filename` + whitelist d'extensions (Principe IV).
- [ ] Pas d'abstraction prématurée (Principe V), réutilisation des patterns existants (`ApiResponse<T>`, `COLONNES`, `FromRow`).

---

## Observabilité manuelle

- Logs backend via `RUST_LOG=info cargo run`.
- Adminer : `http://localhost:8088` → `afrolang.proposition_salle`, `afrolang.salle_moderateur`, `afrolang.salle_privee_adhesion`, `afrolang.ressource_salle`, `afrolang.message_session`.
- LiveKit : vérifier les connexions via `docker logs uafricas-livekit-1`.

---

## Livrables attendus à la fin de la feature

1. Schéma SQL mis à jour dans `uafricas_backend/doc/bd/schemas/08b_afrolang.sql`.
2. Nouveaux structs + DTO dans `uafricas_backend/src/models/afrolang.rs` et `src/models/admin/session_afrolang.rs`.
3. Nouveaux handlers dans `uafricas_backend/src/handlers/afrolang.rs` et `src/handlers/admin/session_afrolang.rs`.
4. Nouvelles routes dans `uafricas_backend/src/routes.rs`.
5. Composable `useAfrolang.ts` étendu + nouveau `useAdminAfrolangSalles.ts`.
6. Nouveaux composants publics (liste ci-dessus dans `plan.md`) sans daisyUI.
7. Nouveaux composants admin avec daisyUI.
8. Mocks mis à jour dans `app/mocks/afrolang.ts`.
9. `CLAUDE.md` mis à jour (nouvelle entrée « Recent Changes »).
