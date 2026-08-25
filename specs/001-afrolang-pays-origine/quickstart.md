# Quickstart : Pays d'origine des salles publiques Afrolang

Parcours de validation manuelle de bout en bout après implémentation. Couvre les 3 user stories et toutes les clarifications.

## 0. Pré-requis

- PostgreSQL démarré (`docker compose up -d`).
- Migration appliquée : la table `afrolang.salle_pays_origine` existe (`\d afrolang.salle_pays_origine` dans `psql`).
- Backend démarré : `kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run` depuis `uafricas_backend/`.
- Frontend démarré : `pnpm dev` depuis `uafricas_frontend/` (port 3000).
- Comptes de test : `admin@test.com` / `Test1234` (admin), `user2@test.com` / `Test1234` (visiteur).

## 1. Vérifier la migration SQL

```bash
docker compose exec postgres psql -U uafricas -d africans_db \
  -c "\d afrolang.salle_pays_origine"
```

Attendu : table avec `salle_id UUID NOT NULL`, `pays_id UUID NOT NULL`, `created_at TIMESTAMPTZ`, PK `(salle_id, pays_id)`, 2 FK CASCADE, 1 index sur `pays_id`.

## 2. US3 : Associer des pays côté admin

1. Se connecter en admin sur `http://localhost:3000/login`.
2. Naviguer vers `Admin → Afrolang → Salles → [choisir « Wolof »]`.
3. Dans le panneau « Pays d'origine », sélectionner successivement Sénégal, Gambie, Mauritanie, valider chaque ajout.
4. **Attendu UI** : 3 chips apparaissent immédiatement, l'API a renvoyé 201 pour chaque appel.
5. **Attendu BD** : `SELECT pays_id FROM afrolang.salle_pays_origine WHERE salle_id = '<wolof>';` → 3 lignes.
6. **Attendu audit** : `SELECT action, table_name FROM audit_log WHERE entity_id = '<wolof>' ORDER BY created_at DESC LIMIT 5;` → 3 entrées `CREATE / salle_pays_origine`.
7. Tenter de réajouter Sénégal → l'UI ne crée pas de doublon, l'API renvoie 201, la table reste à 3 lignes (`ON CONFLICT DO NOTHING`).

## 3. US1 : Voir les pays côté public (1 à 3 pays)

1. Se déconnecter (visiteur anonyme).
2. Aller sur `http://localhost:3000/afrolang`.
3. **Attendu** sur la carte « Wolof » : bandeau « Pays d'origine » avec 3 chips drapeau + nom (`🇬🇲 Gambie`, `🇲🇷 Mauritanie`, `🇸🇳 Sénégal`), ordre alphabétique.
4. Vérifier la requête réseau `GET /api/afrolang/salles` : la salle « Wolof » contient bien `pays_origine: [...]` triée alpha.

## 4. US1 : Liste vide

1. Sur `/afrolang`, vérifier qu'une salle non éditée (ex. salle de démo créée sans pays) n'affiche **aucun bandeau** « Pays d'origine » et que la mise en page n'est pas cassée (FR-004 cas vide).

## 5. US1 : Liste longue (≥ 4 pays)

1. Côté admin, ouvrir la salle « Swahili » et associer Tanzanie, Kenya, Ouganda, Rwanda, RDC, Burundi (6 pays).
2. Côté public `/afrolang`, sur la carte « Swahili » : **Attendu** rangée de 6 drapeaux **sans nom**, et survol = tooltip « Burundi, Kenya, Ouganda, Rwanda, RDC, Tanzanie ».

## 6. US2 : Filtre par pays

1. Sur `/afrolang`, ouvrir le panneau « Filtres ».
2. Sélectionner « Sénégal » dans le filtre « Pays d'origine ».
3. **Attendu** : seules les salles ayant le Sénégal parmi `pays_origine` (donc « Wolof ») apparaissent. Le compteur « N salle(s) trouvée(s) » correspond.
4. URL réseau : `GET /api/afrolang/salles?pays_id=<sn>&...` renvoie 1 résultat.
5. Cliquer « Réinitialiser les filtres » → toutes les salles réapparaissent.
6. Combiner recherche `wolof` + filtre Sénégal → résultats cumulatifs (ET).

## 7. Q3 : Pays archivé masqué côté public

1. Côté admin : `Admin → Pays → [Mauritanie]` → désactiver (`actif = false`).
2. Côté public sur `/afrolang`, recharger : la carte « Wolof » n'affiche **plus** la Mauritanie (Sénégal + Gambie restent). Le compteur ne change pas (la salle reste visible).
3. `GET /api/afrolang/salles?pays_id=<mr>` → 0 résultat (filtre côté public exclut les pays archivés).
4. Côté admin sur la fiche salle « Wolof » : la Mauritanie est **toujours visible** dans la liste, marquée comme « archivé » (chip grisée), l'admin peut la retirer.
5. Réactiver la Mauritanie pour la suite des tests.

## 8. US3 : Retrait + audit

1. Côté admin sur la salle « Wolof », retirer la Mauritanie.
2. **Attendu** : 200 sur le `DELETE`, chip disparue, audit log enrichi d'une entrée `DELETE / salle_pays_origine`.
3. Public : Mauritanie disparue de la carte.

## 9. FR-010 : Cleanup automatique sur suppression d'un pays

1. Créer un pays factice « Testlandia » côté admin.
2. L'associer à la salle « Wolof » (1 ligne dans `salle_pays_origine`).
3. Supprimer définitivement le pays côté admin (`DELETE FROM shared.pays WHERE nom='Testlandia'` ou via UI si dispo).
4. **Attendu** : `SELECT * FROM afrolang.salle_pays_origine WHERE salle_id = '<wolof>';` → la ligne « Testlandia » a disparu (CASCADE).
5. SC-006 vérifié : aucune association orpheline.

## 10. Permissions (FR-011)

1. Se connecter avec un compte sans permission `afrolang:modifier`.
2. Tenter `POST /api/admin/afrolang/salles/.../pays` via outil HTTP.
3. **Attendu** : 403.

## Critères de validation

| ID      | Status                                                           |
|---------|------------------------------------------------------------------|
| US1     | ✅ Bandeau visible, mode 1-3 et 4+ corrects                       |
| US2     | ✅ Filtre mono-pays fonctionnel, reset OK, combinable             |
| US3     | ✅ Add/Remove + audit + idempotence                               |
| Q1      | ✅ Aucune salle existante pré-remplie                              |
| Q2      | ✅ `?pays_id=` mono-valué accepté                                  |
| Q3      | ✅ Pays archivé masqué public, visible admin                       |
| Q4      | ✅ Bascule à 4 pays, tooltip présent                              |
| FR-010  | ✅ CASCADE vérifié                                                |
| SC-004  | Mesurer manuellement : `GET /api/afrolang/salles` < 110 % du baseline |

## Rollback

- Migration purement additive : `DROP TABLE afrolang.salle_pays_origine;` suffit.
- Aucun `ALTER` sur table existante, aucune donnée à restaurer.
