# Quickstart : recette des activités ludiques

**Feature** : `013-activites-ludiques`

Le dépôt n'a ni tests automatisés ni CI. La validation est manuelle, scénario par scénario. sqlx étant vérifié à l'exécution, **toute requête écrite doit avoir été exécutée au moins une fois** : chaque scénario ci-dessous existe aussi pour cela.

Références : [data-model.md](./data-model.md), [contracts/api-membre.md](./contracts/api-membre.md), [contracts/api-admin.md](./contracts/api-admin.md), [contracts/temps-reel.md](./contracts/temps-reel.md).

## Mise en place

```bash
docker compose up -d                                             # PostgreSQL, Adminer, LiveKit

# Base déjà initialisée : appliquer la migration seule
docker exec -i uafricas_postgres psql -U uafricas -d africans_db -v ON_ERROR_STOP=1 \
  < uafricas_backend/doc/bd/schemas/37_jeu.sql

# Seeds locaux, dans cet ordre (99 crée les comptes, 98 le contenu Afripulse, 95 le jeu)
for f in 99_seed_local_tests 98_seed_local_afripulse 95_seed_local_jeu; do
  docker exec -i uafricas_postgres psql -U uafricas -d africans_db -v ON_ERROR_STOP=1 \
    < uafricas_backend/doc/bd/seeds-locaux/$f.sql
done

cd uafricas_backend && kill $(lsof -i :8080 -t) 2>/dev/null; RUST_LOG=info cargo run
cd uafricas_frontend && pnpm dev
```

Le port du backend est celui du `.env` (`PORT`) ; le frontend vise `http://localhost:8080` par défaut (`nuxt.config.ts`).

**Comptes** : `test-admin@test.com` et `martialdjezou@gmail.com`, mot de passe `Test1234`. Sur une base neuve ils n'existent pas : le seed `99` ne les crée pas (leur mot de passe doit être haché par le backend). Les créer **avant** les seeds, backend démarré, par `POST /api/auth/inscription` (commande dans l'en-tête de `99_seed_local_tests.sql`), puis jouer les trois seeds. Le seed `95` les rend **amis**, leur donne deux pays d'origine différents et ouvre une saison en cours. Deux navigateurs (ou une fenêtre privée) sont nécessaires pour les duels.

**Contrôle d'idempotence** : rejouer `37_jeu.sql` puis `95_seed_local_jeu.sql` une seconde fois ne doit produire ni erreur ni doublon.

---

## Scénario 1 : constituer le vivier (US2, US8)

1. Connecté en administrateur, ouvrir `/admin/activites/epreuves`. La liste est vide ou ne contient que le seed.
2. `/admin/activites/revue` → « Proposer des épreuves » sur **Afripulse**, toutes les formes.
   - **Attendu** : environ 250 candidates, un récapitulatif par forme. Aucune n'est jouable.
3. Relancer la même dérivation.
   - **Attendu** : `creees: 0`, tout est compté en `deja_proposees`.
4. Accepter 30 candidates par lot, en corriger une avant de l'accepter, en rejeter une avec un motif.
   - **Attendu** : rejet sans motif refusé. Après une troisième dérivation, la rejetée ne revient pas.
5. Saisir une épreuve Afrolang avec un extrait sonore ; tenter de la publier sans explication.
   - **Attendu** : refus, « L'explication est obligatoire ». Publication acceptée une fois complétée.
6. Avec le compte membre simple, appeler `GET /api/admin/jeu/epreuves`.
   - **Attendu** : 403.

**Vérification en base** :

```sql
SELECT etat, origine, count(*) FROM jeu.epreuve GROUP BY 1, 2 ORDER BY 1, 2;
SELECT count(*) FROM shared.audit_log WHERE schema_name = 'jeu';   -- une ligne par mutation ci-dessus
```

## Scénario 2 : jouer une partie (US1)

1. Non connecté, ouvrir `/activites`.
   - **Attendu** : présentation et classement visibles ; « Jouer » mène à la connexion.
2. Connecté, ouvrir `/opportunite-afrique`.
   - **Attendu** : un panneau « Activités » dans le rail, qui mène à `/activites/afripulse`.
3. Lancer une partie. Dans l'onglet réseau, inspecter la réponse de `…/suivante`.
   - **Attendu** : aucune clé ne désigne la bonne réponse ; l'ordre des propositions change si on recharge.
4. Répondre juste, répondre faux, laisser un temps s'écouler.
   - **Attendu** : correction, explication et lien vers la fiche après chaque épreuve ; l'épreuve au temps écoulé est comptée sans réponse.
5. En pleine épreuve, recharger la page.
   - **Attendu** : l'épreuve affichée est perdue (sans réponse), la partie reprend à la suivante.
6. Finir la partie.
   - **Attendu** : bilan avec bonnes réponses, score gagné, total et rang.
7. Rejouer `POST …/repondre` de la dernière épreuve (curl, même corps).
   - **Attendu** : même correction, **aucun** second gain.

```sql
SELECT origine, count(*), sum(montant) FROM jeu.gain WHERE utilisateur_id = :uid GROUP BY 1;
SELECT solde_points, niveau_code FROM engagement.compte WHERE utilisateur_id = :uid;   -- inchangés (SC-006)
```

8. Jouer jusqu'à épuiser les épreuves neuves du module.
   - **Attendu** : la partie suivante est refusée en 409, l'écran propose un **entraînement** et le dit avant de commencer ; l'entraînement ne rapporte rien.
9. Retirer une épreuve au back-office.
   - **Attendu** : elle n'est plus servie ; le score du membre n'a pas bougé.
10. Suspendre le compte membre en base (`UPDATE iam.utilisateur SET etat = 'suspendu'`) sans le déconnecter, puis lancer une partie.
    - **Attendu** : 403 immédiat, sans attendre l'expiration du jeton. Rétablir le compte ensuite.

## Scénario 3 : la source d'une épreuve change (US8, FR-011, FR-012)

1. Repérer une épreuve jouable dérivée d'une fiche pays. Bloquer la fiche (`UPDATE country_profile.fiche_pays SET bloquee = TRUE WHERE id = …`).
   - **Attendu** : l'épreuve n'est plus servie tout de suite ; au back-office, `source_etat = indisponible`. Débloquer : elle revient.
2. Donner un J'aime à une fiche pays depuis le site.
   - **Attendu** : **rien ne change** pour ses épreuves (l'empreinte ignore les compteurs).
3. Modifier la capitale dans `shared.pays` pour ce pays.
   - **Attendu** : l'épreuve n'est plus servie ; elle apparaît dans la revue avec `a_revoir` à l'ouverture de la liste.

## Scénario 4 : défis (US3)

1. Avec les deux comptes, ouvrir `/activites` le même jour.
   - **Attendu** : le même défi du jour, mêmes épreuves, même ordre.

```sql
SELECT periodicite, periode_debut, origine, cardinality(epreuve_ids) FROM jeu.defi ORDER BY 2 DESC;
```

2. Commencer le défi, le quitter, tenter de le recommencer.
   - **Attendu** : refus ; l'écran propose de reprendre la partie en cours ou montre le résultat.
3. Terminer le défi.
   - **Attendu** : score des bonnes réponses + prime ; série de jours à 1 ; dans `/mon-compte/engagement`, une ligne « réputation +1 » à 0 point.
4. Simuler le lendemain : `UPDATE jeu.defi SET periode_debut = periode_debut - 1 WHERE periodicite = 'jour'; UPDATE jeu.joueur SET serie_dernier_jour = serie_dernier_jour - 1;` puis rouvrir `/activites`.
   - **Attendu** : un nouveau défi du jour ; l'ancien n'est plus jouable mais ses résultats restent consultables ; après l'avoir terminé, la série vaut 2.
5. Programmer au back-office le défi d'une date à venir ; tenter de programmer celui d'aujourd'hui.
   - **Attendu** : le premier accepté, le second refusé.

## Scénario 5 : duel différé (US4)

1. Compte A propose à compte B un duel différé sur Afripulse.
   - **Attendu** : B reçoit une notification ; aucune fenêtre ne surgit.
2. B accepte. A joue.
   - **Attendu** : A ne voit pas de comparaison ; B ne voit rien du résultat de A. B reçoit « à vous ».
3. B joue.
   - **Attendu** : mêmes épreuves, même ordre que A ; résultat, vainqueur et gain visibles des deux ; réputation +1 au vainqueur.
4. Nouveau duel, B accepte, **seul A joue**. Avancer l'échéance : `UPDATE jeu.duel SET echeance_at = NOW() - interval '1 minute' WHERE id = …`. Ouvrir la liste des duels.
   - **Attendu** : A vainqueur par forfait.
5. Nouvelle proposition, sans réponse, échéance avancée de même.
   - **Attendu** : duel expiré, aucun gain.
6. Enchaîner quatre duels comptés entre A et B le même jour.
   - **Attendu** : le quatrième est annoncé **amical avant de commencer** et ne rapporte rien.
7. Retirer l'amitié pendant un duel accepté.
   - **Attendu** : duel annulé, sans gain. Proposer un duel à un non-ami : refusé.

## Scénario 6 : Championship et carte (US5, US6)

1. `/activites/championship` : trois classements.
   - **Attendu** : A et B classés chacun sous son pays ; le classement des pays liste les 55.
2. Changer le pays d'origine de A dans son profil, rejouer une partie.

```sql
SELECT pays_id, score FROM jeu.score_saison WHERE utilisateur_id = :uidA;   -- deux lignes, l'ancienne inchangée
```

   - **Attendu** : A figure dans deux classements de pays ; une seule fois, avec la somme, au classement de tous.
3. Vider les deux pays d'un compte, jouer.
   - **Attendu** : présent au classement de tous, absent de tout classement de pays ; l'écran l'invite à compléter son profil.
4. Bascule « Carte ».
   - **Attendu** : 55 pays teintés ; un pays sans score est cliquable et s'annonce sans joueur ; « Jouer sur ce pays » lance une partie dont toutes les épreuves portent sur lui.
5. Réduire la fenêtre sous 1024 px.
   - **Attendu** : la liste est la vue par défaut, la carte reste accessible par la bascule.
6. Clore la saison : `UPDATE jeu.saison SET fin_at = NOW() - interval '1 minute'`. Recharger la page.
   - **Attendu** : classement figé dans les archives, sans action d'administrateur ; `cloturee_at` renseigné ; badge « Champion » aux premiers ; un gain écrit ensuite n'a pas de `saison_id`.
7. Créer au back-office une saison qui chevauche une autre.
   - **Attendu** : 409 lisible.
8. Vérifier sur `/retrouve-amis` que la carte d'Africonnect se comporte comme avant (le composant a été déplacé et généralisé).

## Scénario 7 : duel direct (US7)

Deux navigateurs, les deux comptes connectés.

1. A propose un duel **direct**.
   - **Attendu** : une fenêtre d'invitation surgit chez B, où qu'il soit sur le site.
2. B accepte.
   - **Attendu** : la première épreuve apparaît chez les deux avec moins d'une seconde d'écart ; le compte à rebours est le même.
3. A répond le premier.
   - **Attendu** : A voit « en attente de l'adversaire », **sans correction** ; B voit que A a répondu, pas ce qu'il a répondu. La correction arrive aux deux quand B répond.
4. Couper le réseau de B 15 secondes pendant une manche, le rétablir.
   - **Attendu** : B reprend à la manche en cours ; celle manquée est sans réponse ; le duel continue.
5. Fermer l'onglet de B plus longtemps que le délai de grâce.
   - **Attendu** : A vainqueur par forfait.
6. Couper le flux SSE de B (bloquer `/api/messagerie/flux` dans les outils du navigateur) et jouer un duel.
   - **Attendu** : le duel se déroule quand même, avec au plus 3 secondes de retard côté B.
7. A propose un duel direct, B ne répond pas 5 minutes (ou avancer l'échéance).
   - **Attendu** : expiré ; A peut le convertir en duel différé.

## Scénario 8 : réputation, distinctions, signalement, triche (US9)

1. Après les scénarios précédents, ouvrir `/mon-compte/engagement`.
   - **Attendu** : badges « Premier pas » et, pour le vainqueur de saison, « Champion » ; lignes de réputation à 0 point ; solde de points et statut inchangés.
2. `/admin/engagement/regles` : les six règles `jeu_*` y figurent, **instrumentées**, montants modifiables.
3. Signaler une épreuve après y avoir répondu ; tenter de la signaler deux fois ; tenter de signaler une épreuve jamais jouée.
   - **Attendu** : accepté, puis 409, puis 403.
4. Au back-office, confirmer le signalement en retirant l'épreuve.
   - **Attendu** : le signaleur est notifié et gagne de la réputation ; aucun score n'est repris à personne.
5. Annuler les gains d'un membre avec un motif.

```sql
SELECT (SELECT COALESCE(sum(montant), 0) FROM jeu.gain WHERE utilisateur_id = :uid AND annule_at IS NULL) AS journal,
       (SELECT score_total FROM jeu.joueur WHERE utilisateur_id = :uid) AS agregat;   -- égaux
```

   - **Attendu** : le membre sort du classement ou y recule, il est notifié, l'audit porte `GAINS_ANNULES`.

## Critères de sortie

| Critère | Mesure |
|---|---|
| Aucune fuite de bonne réponse | scénario 2.3 et 7.3, sur les payloads réels |
| Idempotence des gains | scénario 2.7 ; `SELECT cle_idempotence, count(*) FROM jeu.gain GROUP BY 1 HAVING count(*) > 1` renvoie 0 ligne |
| Agrégats fidèles au journal | pour tout membre, `sum(gain non annulé) = joueur.score_total` et `= sum(score_saison)` par saison |
| Engagement intact | `solde_points` et `niveau_code` identiques avant et après chaque scénario |
| Audit | chaque mutation d'administration a sa ligne dans `shared.audit_log` |
| Migration rejouable | `37_jeu.sql` appliquée deux fois sans erreur |
| Classement à l'échelle (SC-011) | avec 10 000 lignes générées dans `score_saison` (`generate_series`), les trois classements répondent en moins de 2 s |
| Pas de régression | carte d'Africonnect, appels entre amis et messagerie fonctionnent comme avant |

## Avant l'ouverture en production

Ce n'est pas du développement, mais l'ouverture en dépend (research D14).

1. Appliquer la migration **avant** de déployer le code : `./deploy.sh migrate uafricas_backend/doc/bd/schemas/37_jeu.sql`. Elle est purement additive (l'ancien code l'ignore, le nouveau en a besoin), donc un seul temps, sans suppression à rejouer après.
2. Mesurer le contenu réellement disponible, en lecture seule :

```bash
./deploy.sh psql "SELECT type_source, count(*) FILTER (WHERE visible) FROM jeu.v_source GROUP BY 1"
```

3. Lancer la dérivation sur les trois modules qui en ont une, puis faire la revue.
4. Saisir les épreuves Afrolang.
5. N'ouvrir un module (`jeu.module.ouvert`) que lorsqu'il atteint le volume visé (SC-002 : 100 épreuves jouables) ; en dessous de la taille d'une partie, il s'annonce de lui-même « bientôt disponible ».
6. Créer la première saison.
