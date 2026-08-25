# Quickstart : Validation manuelle

**Feature** : 001-admin-salles-publiques

Pas d'infra de test automatisée à ce stade (cf. CLAUDE.md). Ce document décrit les **scénarios manuels** à dérouler pour valider chaque user story.

---

## Pré-requis

```bash
docker compose up -d
cd uafricas_backend && RUST_LOG=info cargo run   # port 8080
cd uafricas_frontend && pnpm dev                 # port 3000
```

Comptes de test (CLAUDE.md) :
- Admin plateforme : `admin@test.com` / `Test1234`
- Utilisateur lambda : `user2@test.com` / `Test1234`

Adminer : http://localhost:8088 (BD `africans_db`, schéma `afrolang`).

---

## Scénario 1 : US1 : proposer une salle (utilisateur lambda)

1. Connexion avec `user2@test.com`.
2. Ouvrir `/afrolang/proposer`.
3. Renseigner :
   - Titre : « Salle Wolof : Sénégal »
   - Description : libre
   - Justification : libre
   - Langue cible : « Wolof », code : `wo`
   - Groupe ethnique : sélection dans la liste (référentiel existant)
   - Pays d'origine : Sénégal, Gambie
4. Soumettre → attendu : confirmation + bandeau « en attente de validation ».
5. Vérifier dans Adminer :
   ```sql
   SELECT id, statut, auteur_id, langue_cible FROM afrolang.proposition_salle ORDER BY created_at DESC LIMIT 1;
   ```
6. Resoumettre la même proposition → attendu : erreur 409 « proposition en attente déjà existante ».
7. Se déconnecter, tenter d'accéder à `/afrolang/proposer` → redirection vers `/login`.

✅ FR-001, FR-002, FR-003, FR-007 validés.

---

## Scénario 2 : US2 : valider/rejeter (admin plateforme)

1. Connexion `admin@test.com`.
2. Ouvrir `/admin/afrolang/propositions`.
3. La proposition du scénario 1 doit apparaître avec statut **en attente**. Tester les filtres (statut, groupe ethnique).
4. Ouvrir le détail → bouton **Valider** + bouton **Rejeter** visibles.
5. **Test rejet** : créer une seconde proposition côté `user2`, puis côté admin :
   - Saisir commentaire vide → bouton désactivé.
   - Saisir commentaire ≥ 10 car. → soumettre → statut passe à `rejetee`.
6. **Test validation** : sur la 1re proposition, cliquer **Valider** :
   ```sql
   SELECT statut, salle_id_creee, decideur, decide_at FROM afrolang.proposition_salle WHERE id = '...';
   SELECT id, titre, groupe_ethnique_id, actif FROM afrolang.salle WHERE id = '...';   -- créée
   SELECT pays_id FROM afrolang.salle_pays_origine WHERE salle_id = '...';            -- 2 lignes
   ```
7. Se reconnecter en tant que `user2`, ouvrir l'espace perso `/afrolang/proposer` (onglet « Mes propositions ») → statuts visibles + commentaire de rejet visible.
8. Vérifier audit : `SELECT action, table_concernee, entity_id FROM audit.log ORDER BY created_at DESC LIMIT 10;`, au moins `VALIDATE proposition_salle`, `CREATE salle`, `REJECT proposition_salle`.

✅ FR-008 → FR-012, SC-003 (les autres utilisateurs ne voient pas la proposition en attente).

---

## Scénario 3 : US3 : nommer un admin de salle

1. Connexion `admin@test.com`.
2. Ouvrir la fiche salle créée au scénario 2 dans `/admin/salles/{id}`.
3. Onglet **Administrateurs** → liste vide.
4. Saisir l'ID ou rechercher `user2@test.com`, cliquer **Nommer**.
   ```sql
   SELECT * FROM afrolang.salle_administrateur WHERE salle_id = '...';
   ```
5. Tenter de re-nommer le même utilisateur → erreur 409.
6. Se déconnecter, ouvrir la fiche **publique** de la salle (`/afrolang/salle/{id}`) → bandeau « Administrateurs de la salle » avec le nom de `user2`.
7. Reconnexion admin, cliquer **Révoquer**, saisir motif → ligne `actif=FALSE`, fiche publique se vide.
8. Vérifier audit : 2 entrées `salle_administrateur` (CREATE + UPDATE).

✅ FR-013 → FR-018, FR-020.

---

## Scénario 4 : Cascades (FR-021, FR-022, SC-008)

1. Re-nommer `user2` administrateur de la salle (cf. scénario 3).
2. **Cas A : salle archivée** : depuis l'admin, désactiver/archiver la salle.
   ```sql
   SELECT actif, suspendu_at, motif_suspension FROM afrolang.salle_administrateur WHERE utilisateur_id = '...';
   -- Attendu : actif=FALSE, motif='salle_archivee'.
   ```
3. Réactiver la salle → la nomination **reste suspendue** (FR-021 : pas de réactivation auto). Re-nommer manuellement.
4. **Cas B : compte désactivé** : depuis l'admin IAM, passer `user2` à `etat='suspendu'`.
   ```sql
   SELECT actif, suspendu_at, motif_suspension FROM afrolang.salle_administrateur WHERE utilisateur_id = '...';
   -- Attendu : actif=FALSE, motif='compte_desactive'.
   ```
5. Vérifier que les deux suspensions ont des entrées d'audit (`UPDATE salle_administrateur`) avec `motif_suspension` dans le `after` JSONB.

✅ FR-021, FR-022.

---

## Scénario 5 : Anti-spam (Décision 6 research.md)

1. Avec un compte tiers, créer 5 propositions, faire rejeter chacune par l'admin.
2. Tenter une 6e soumission → attendu **429** avec message d'attente jusqu'au `decide_at + 7j` du plus ancien rejet.

✅ Edge case « tentatives répétées de soumissions de mauvaise qualité ».

---

## Critères de succès vérifiables

| SC | Méthode |
|----|---------|
| SC-002 | Chronométrer le scénario 1 sur un utilisateur novice → < 5 min. |
| SC-003 | Avec un 3e compte, tenter `GET /api/afrolang/propositions/moi` → ne contient aucune proposition d'un autre auteur. |
| SC-004 | Mesurer le délai entre clic « Valider » et apparition de la notification chez `user2` → < 60 s. |
| SC-005 | `SELECT * FROM audit.log WHERE table_concernee IN ('salle_administrateur','proposition_salle')` → toutes les actions tracées. |
| SC-006 | Œil sur la fiche publique : badges « Admin de la salle » distincts du badge « Admin plateforme » du créateur initial. |
| SC-007 | `SELECT auteur_id, groupe_ethnique_id, COUNT(*) FROM afrolang.proposition_salle WHERE statut='en_attente' GROUP BY 1,2 HAVING COUNT(*) > 1;` → 0 ligne. |
| SC-008 | Chronométrer entre archivage salle et `actif=FALSE` sur les administrateurs → < 60 s. |
