# Quickstart : Validation du recadrage de l'engagement

**Feature**: `008-recadrage-engagement-cadeaux`

Guide de validation manuelle reproductible. Le projet n'a pas de harnais de tests automatisés ; ces neuf scénarios couvrent les 13 critères de succès de la spécification. Chacun est indépendant, sauf indication contraire.

---

## Prérequis

```bash
# 1. Base de données
docker compose up -d                       # postgres + adminer + livekit

# 2. Migrations du recadrage (dans l'ordre)
docker compose exec -T postgres psql -U uafricas -d africans_db \
  -f /docker-entrypoint-initdb.d/schemas/35f_engagement_recadrage.sql
docker compose exec -T postgres psql -U uafricas -d africans_db \
  -f /docker-entrypoint-initdb.d/schemas/35g_engagement_cadeaux.sql

# 3. Backend (toujours tuer l'ancien processus d'abord)
cd uafricas_backend
kill $(lsof -i :8082 -t) 2>/dev/null; RUST_LOG=info cargo run

# 4. Frontend
cd uafricas_frontend && pnpm dev
```

**Comptes** : `test-admin@test.com` / `Test1234` (porteur de `engagement.gerer`) et `test-user@test.com` / `Test1234`. Un **troisième** compte membre est nécessaire pour les scénarios de cadeaux et de partage, le créer via `/inscription`.

**Console SQL** : Adminer sur `http://localhost:8088`, ou `docker compose exec postgres psql -U uafricas -d africans_db`.

---

## S1 : Le barème est recadré (US1 · SC-001, SC-002)

1. Se connecter en administrateur, ouvrir `/admin/engagement/regles`.
2. **Attendu** : exactement 3 règles actives, `jaime_recu`, `partage_recu`, `cadeau_recu`, plus `ajustement_admin` ; les 8 règles écartées et `popularite_palier` sont présentes et **inactives**, montants d'origine conservés.
3. Ouvrir `/admin/engagement/niveaux`. **Attendu** : Membre Africans (0), Premium (500), Gold (2 000), Platinum (10 000), dans cet ordre.
4. Ouvrir `/admin/engagement/paliers`. **Attendu** : tous inactifs.

Contrôle en base :

```sql
SELECT type_action, actif FROM engagement.regle_points ORDER BY actif DESC, type_action;
SELECT code, libelle, seuil_min, ordre FROM engagement.niveau ORDER BY ordre;
SELECT bool_and(NOT actif) AS tous_inactifs FROM engagement.palier_popularite;

-- FR-036 : aucune rétroactivité. Immédiatement après la migration, les j'aime et
-- partages déjà existants ne doivent avoir crédité personne.
SELECT COUNT(*) FROM engagement.mouvement_points
 WHERE type_action IN ('jaime_recu', 'partage_recu', 'cadeau_recu');
-- Attendu : 0
```

---

## S2 : Une action écartée ne crédite plus, sans casser le parcours (US1 · SC-007)

1. En administrateur, valider une proposition de média en attente (`/admin/medias/propositions`).
2. **Attendu** : la validation aboutit normalement, le média est créé, son auteur devient propriétaire.
3. Contrôle : aucun mouvement `proposition_media_validee` n'a été créé après cette validation.

```sql
SELECT COUNT(*) FROM engagement.mouvement_points
 WHERE type_action = 'proposition_media_validee' AND created_at > NOW() - INTERVAL '5 minutes';
-- Attendu : 0
```

4. Réactiver la règle depuis `/admin/engagement/regles`, valider une seconde proposition. **Attendu** : le mouvement apparaît cette fois, sans redéploiement (FR-002).
5. **Remettre la règle inactive** avant de poursuivre.

---

## S3 : Un j'aime rapporte exactement 1 point, une seule fois (US2 · SC-003)

1. Avec le compte A, publier un contenu Codi-moi.
2. Avec le compte B, aimer ce contenu. **Attendu** : A voit +1 point dans `/mon-compte/engagement`, catégorie « Popularité », libellé « J'aime reçu sur un contenu ».
3. Avec B, retirer le j'aime, puis le remettre, **trois fois**.
4. **Attendu** : A a toujours exactement **1 point**, et un seul mouvement.

```sql
SELECT COUNT(*), SUM(points) FROM engagement.mouvement_points
 WHERE type_action = 'jaime_recu' AND cle_idempotence LIKE 'jaime:codimoi:%';
-- Attendu : 1 mouvement, 1 point
```

5. Avec A, aimer son propre contenu. **Attendu** : aucun mouvement supplémentaire (FR-009).
6. Répéter sur une vidéo Vidafrica et sur une chaîne TV. Pour la chaîne, vérifier que le crédité est le **propriétaire** du support, pas nécessairement son créateur :

```sql
SELECT m.utilisateur_id, sd.utilisateur_id AS proprietaire
  FROM engagement.mouvement_points m
  JOIN media_content.support_detenteur sd
    ON sd.support_id = m.objet_id AND sd.role = 'proprietaire' AND sd.actif
 WHERE m.type_action = 'jaime_recu' AND m.type_objet = 'chaine_tv';
-- Attendu : les deux colonnes sont identiques
```

7. **FR-008c** : aimer un **site touristique** puis un **secteur de développement** d'une fiche pays. **Attendu** : la réaction est enregistrée et le compteur s'incrémente normalement, mais **aucun mouvement de points** n'est créé et aucune erreur n'apparaît côté serveur. Aimer ensuite une **personnalité connue** ou une **recette culinaire** : là, le `cree_par` de l'élément est bien crédité.

```sql
SELECT type_objet, COUNT(*) FROM engagement.mouvement_points
 WHERE type_action = 'jaime_recu' GROUP BY type_objet;
-- Attendu : ni 'site_touristique' ni 'secteur_developpement' ; jamais la valeur 'element'
```

---

## S4 : Un partageur ne crédite qu'une fois, tous canaux confondus (US4 · SC-011)

1. Avec le compte B, partager un contenu de A vers **WhatsApp** depuis la modale de partage.
2. **Attendu** : A gagne 1 point, catégorie « Partages ».
3. Toujours avec B, partager le **même** contenu vers Facebook, puis Telegram, puis sur le mur de la plateforme.
4. **Attendu** : A a toujours **1 seul** point de partage pour ce contenu, et les 4 gestes sont tracés.

```sql
SELECT COUNT(*) FROM engagement.mouvement_points
 WHERE type_action = 'partage_recu' AND objet_id = '<objet_id>';
-- Attendu : 1

SELECT reseau FROM engagement.partage_externe WHERE objet_id = '<objet_id>';
-- Attendu : whatsapp, facebook, telegram (la trace, elle, est complète)
```

5. Avec le compte C, partager le même contenu. **Attendu** : A gagne un 2ᵉ point (partageur distinct).
6. Avec A, partager son propre contenu. **Attendu** : aucun point (FR-014).

---

## S5 : Cadeau en soutien financier, paiement simulé abouti (US3 · SC-004, SC-009)

1. Avec B, ouvrir une vidéo Vidafrica publiée par A, cliquer « Offrir un cadeau ».
2. **Attendu** : le bandeau « paiement simulé, phase de test » est visible (FR-020a).
3. Choisir « Drapeau de l'Union Africaine », mode **Soutien financier**, confirmer, puis faire **aboutir** le paiement simulé.
4. **Attendu, en moins de 5 secondes** : A est crédité de **+20 points** (catégorie « Cadeaux »), reçoit une notification, le cadeau s'affiche sur la vidéo avec le nom de B et **aucun montant**.

```sql
SELECT montant, part_beneficiaire, part_plateforme, points, mode, etat, simule
  FROM engagement.transaction_cadeau ORDER BY created_at DESC LIMIT 1;
-- Attendu : 2000 | 1800 | 200 | 20 | soutien_financier | abouti | true

SELECT montant_cumule FROM engagement.cagnotte WHERE utilisateur_id = '<id de A>';
-- Attendu : 1800
```

5. Ouvrir `/mon-compte/engagement` avec A : la cagnotte affiche 1 800 FCFA et la mention explicite que le versement n'est pas disponible (FR-026). Vérifier l'accès en **au plus 2 clics** depuis le profil (SC-010).
6. **FR-031** : ouvrir `/profil/<id de A>` avec le compte B, non connecté puis connecté : le statut et les badges de A sont visibles, **ni son solde, ni sa cagnotte, ni son historique** ne le sont. Contrôler également la réponse réseau, pas seulement l'affichage.

---

## S6 : Cadeau en points et paiement en échec (US3 · SC-005)

1. Avec B, offrir une « Fleur » à A en mode **Cadeau en points**, paiement simulé **abouti**.
2. **Attendu** : A gagne +3 points, `part_beneficiaire = 0`, `part_plateforme = 300`, cagnotte **inchangée**.
3. Recommencer avec un « Badge », mais faire **échouer** le paiement simulé.
4. **Attendu** : aucun point, aucune cagnotte, aucun cadeau affiché ; la transaction est en `echoue` et B peut réessayer.

```sql
SELECT etat, COUNT(*) FROM engagement.transaction_cadeau GROUP BY etat;
```

---

## S7 : Rejeu et auto-cadeau (US3 · SC-006, SC-011)

1. Rejouer la confirmation d'une transaction déjà aboutie (recharger la page de retour, ou rappeler la route).
2. **Attendu** : réponse `200` identique, **0 point supplémentaire**, **0 cagnotte supplémentaire**.
3. Avec A, tenter d'offrir un cadeau sur son propre contenu, puis sur son propre profil.
4. **Attendu** : refus explicite dans les deux cas (`403`), aucune transaction créée.

```sql
SELECT COUNT(*) FROM engagement.transaction_cadeau WHERE offreur_id = beneficiaire_id;
-- Attendu : 0 : et la contrainte CHECK le rend structurellement impossible
```

---

## S8 : Statuts et administration du catalogue (US1, US5 · SC-002, SC-008)

1. En administrateur, appliquer un ajustement manuel de **+500 points** à un compte de test (`/admin/engagement/journal`).
2. **Attendu** : le compte bascule immédiatement sur « Premium ». Passer à 2 000 → « Gold » ; à 10 000 → « Platinum ». À 499 → « Membre Africans ».
3. Ouvrir `/admin/engagement/cadeaux`, créer un cadeau « Tam-tam » (50 points, 5 000 FCFA), l'activer.
4. **Attendu** : il apparaît immédiatement dans la modale côté membre, sans redémarrage.
5. Modifier le **taux de commission** à 20 %, offrir un nouveau cadeau, vérifier la répartition 80 / 20, et vérifier que les transactions **antérieures** conservent leur taux de 10 % (FR-024).
6. Tenter de supprimer un cadeau déjà offert. **Attendu** : refus explicite, désactivation proposée.
7. Ouvrir `/admin/engagement/transactions`, vérifier l'invariant des totaux :

```sql
SELECT SUM(montant) AS total,
       SUM(part_beneficiaire) AS cagnottes,
       SUM(part_plateforme)   AS recettes,
       SUM(part_beneficiaire) + SUM(part_plateforme) = SUM(montant) AS invariant_ok
  FROM engagement.transaction_cadeau WHERE etat = 'abouti';
-- Attendu : invariant_ok = true
```

---

## S9 : Purge de fin de phase de test (SC-013)

> Scénario destructif : à exécuter **en dernier**, sur un environnement de recette.

1. Relever l'état avant purge :

```sql
SELECT utilisateur_id, solde_points, niveau_code FROM engagement.compte ORDER BY solde_points DESC;
SELECT type_action, COUNT(*) FROM engagement.mouvement_points GROUP BY type_action;
```

2. Tenter la purge **avant** toute bascule → **attendu** : refus `409` (purger tant que le paiement reste simulé rouvrirait aussitôt la porte au minage).
3. En administrateur, passer `paiement_reel_actif` à `true` dans `/admin/engagement/cadeaux` (section paramètres). **Ce basculement simule la mise en service de CinetPay** : le bandeau « phase de test » disparaît alors côté membre, ce qui est le comportement attendu, pendant la recette, l'état est volontairement transitoire. **Repasser le drapeau à `false` après le scénario** tant que l'encaissement réel n'est pas branché, sans quoi les membres verraient un parcours factice présenté comme réel.
4. Déclencher `POST /purger-phase-test` avec la confirmation.
5. **Attendu** :
   - tous les mouvements `cadeau_recu` issus de transactions simulées ont disparu ;
   - **aucun** mouvement `jaime_recu` ni `partage_recu` n'a été supprimé ;
   - les soldes et statuts reflètent le journal restant ;
   - les cagnottes concernées sont revenues à 0 ;
   - les transactions sont en `etat = 'purge'`, **présentes**.

```sql
SELECT type_action, COUNT(*) FROM engagement.mouvement_points GROUP BY type_action;
-- Attendu : jaime_recu et partage_recu inchangés, cadeau_recu à 0

SELECT c.utilisateur_id,
       c.solde_points,
       COALESCE(SUM(m.points), 0) AS somme_journal,
       c.solde_points = GREATEST(COALESCE(SUM(m.points), 0), 0) AS coherent
  FROM engagement.compte c
  LEFT JOIN engagement.mouvement_points m ON m.utilisateur_id = c.utilisateur_id
 GROUP BY c.utilisateur_id, c.solde_points;
-- Attendu : coherent = true partout
```

6. Rejouer la purge. **Attendu** : décomptes à zéro, aucune erreur.

---

## Matrice de couverture

| Critère | Scénarios | Résultat (exécution du 2026-08-08) |
|---------|-----------|------------------------------------|
| SC-001 recadrage en moins de 5 min | S1, S2 | ✅ 4 règles actives (3 canoniques + `ajustement_admin`), 9 inactives avec montants conservés |
| SC-002 quatre statuts cohérents | S1, S8 | ✅ 0/500/2 000/10 000 ; bascules constatées 523→premium, 2 023→gold, 10 023→platinum |
| SC-003 N j'aime = N points, 0 doublon | S3 | ✅ 3 cycles pose/retrait → 1 mouvement, 1 point ; auto-like sans effet |
| SC-004 crédit du cadeau < 5 s | S5 | ✅ crédit synchrone après COMMIT, aucune tâche de fond |
| SC-005 échec = 0 point, 0 répartition | S6 | ✅ `etat=echoue`, `points_credites=0`, cagnotte inchangée |
| SC-006 rejeu = 0 doublon | S7 | ✅ réponse identique, 1 seul `mouvement_points` de clé `cadeau:{id}` |
| SC-007 aucune action métier en échec | S2, S3, S4 | ✅ réaction, partage et validation aboutissent malgré les règles désactivées |
| SC-008 100 % paramétrable | S1, S8 | ✅ 6ᵉ cadeau créé et visible sans redémarrage ; taux modifiable |
| SC-009 répartition exacte | S5, S8 | ✅ 2 000 → 1 800/200 ; `recettes + cagnottes = total` vérifié en base et dans les totaux du journal |
| SC-010 accès en 2 clics | S5 (étape 5) | ✅ profil → « Mon engagement » → cagnotte et cadeaux sur la même page |
| SC-011 aucune auto-attribution | S3, S4, S7 | ✅ auto-cadeau `403` ; 6 gestes de partage (3 internes + 3 externes) → **1 seul** point |
| SC-012 substituabilité du paiement | Revue de code | ✅ `services/paiement.rs` (76 lignes) ne référence ni catalogue, ni journal, ni répartition, ni points ; 5 sites d'appel, tous dans `handlers/engagement_cadeau.rs` |
| SC-013 purge exacte | S9 | ✅ 2 transactions purgées, 2 mouvements supprimés, 1 compte recalculé, 1 800 de cagnottes annulés ; `jaime_recu` et `partage_recu` **intacts** ; rejeu à zéro sans erreur |

### Écarts relevés et corrigés pendant la validation

1. **`source_titre` visait des colonnes inexistantes** (`culture.codimoi.titre`, `programme_tele.titre`, `personnalite_connue.nom`…). Les noms réels diffèrent par famille (`contenu`, `nom_emission`, `nom_complet`) et `fiche_pays` n'a pas de nom propre : il vient de `shared.pays`. L'échec était **silencieux** (`.ok().flatten()`), le journal admin affichant simplement des titres vides.
2. **`site_touristique` répondait `400` au lieu de `409`.** La famille était refusée comme « inconnue » alors qu'elle est connue mais sans auteur. Une constante `FAMILLES_SANS_AUTEUR` sépare désormais les deux cas : le client doit pouvoir dire « ce contenu n'a pas d'auteur » et non « votre requête est erronée ».
3. **`titre_cible` restait `null` sur `/mes-cadeaux`**, contrairement au contrat §5. `resoudre_titre` a été rendue partagée entre le handler membre et le handler d'administration.
