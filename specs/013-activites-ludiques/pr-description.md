# Activités ludiques : socle de jeu et 4 modules pilotes

Spec, plan et recette : `specs/013-activites-ludiques/`.

## Ce que la PR apporte

- **Épreuves** saisies à la main ou dérivées du contenu publié (Codimoi, Afripulse, FactCheck), revues par un administrateur avant d'être jouables ; Afrolang en saisie seule.
- **Parties** de 10 épreuves, horloge tenue par le serveur : la bonne réponse ne part jamais avant la réponse, une épreuve comptée ne se rejoue pas.
- **Défis** du jour et de la semaine, **Championship** par saisons (classement global, par pays, carte de l'Afrique colorée par rang).
- **Duels entre amis**, différé (48 h, forfait) et direct (même épreuve au même instant, déconnexions tolérées). Le direct passe par le flux SSE de la messagerie, sans nouveau mécanisme temps réel.
- **Réputation et distinctions** : le jeu produit un score de jeu distinct, il **ne crédite aucun point d'engagement** (règles à 0 point qui ne portent que la réputation).
- Back-office `/admin/activites/*` : épreuves, revue, défis, saisons, joueurs (annulation de gains), règles du jeu, signalements.

Rattachement territorial : pays d'origine, repli sur la résidence si l'origine est vide, **figé au moment du gain**.

## Base de données

Une migration, `uafricas_backend/doc/bd/schemas/37_jeu.sql` : nouveau schéma `jeu`, plus des lignes ajoutées dans `engagement` (catégorie « jeux », règles à 0 point, distinctions) et `iam` (permission). Elle est **purement additive** et rejouable : un seul temps, rien à supprimer après le déploiement.

Elle lit des tables d'engagement livrées par `35c` à `35g` (`categorie_points`, `badge`, `parametre_monetisation`). Les vérifier avant :

```bash
./deploy.sh psql "SELECT to_regclass('engagement.categorie_points'), to_regclass('engagement.badge'), to_regclass('engagement.parametre_monetisation')"
```

Une valeur vide : appliquer d'abord les migrations `35*` manquantes.

## Ouverture en production

1. **Migration avant le code** (l'ancien code ignore le schéma, le nouveau en a besoin) :
   ```bash
   git push origin main
   ./deploy.sh migrate uafricas_backend/doc/bd/schemas/37_jeu.sql
   ./deploy.sh update
   ```
2. **Mesurer le contenu disponible**, en lecture seule :
   ```bash
   ./deploy.sh psql "SELECT type_source, count(*) FILTER (WHERE visible) FROM jeu.v_source GROUP BY 1"
   ```
3. **Dériver puis revoir** : `/admin/activites/revue`, dérivation sur Codimoi, Afripulse et FactCheck, puis acceptation ou rejet des candidates. Rien de dérivé n'est jouable sans revue.
4. **Saisir les épreuves Afrolang** dans `/admin/activites/epreuves` (aucune dérivation possible : pas de référentiel de langues).
5. **Ouvrir un module** seulement quand il atteint le volume visé (100 épreuves jouables). En dessous de la taille d'une partie, il s'affiche de lui-même « bientôt disponible ».
6. **Créer la première saison** dans `/admin/activites/saisons`. Sans saison, on joue, mais aucun classement du Championship ne se remplit.

## Vérifications faites en local

- `cargo build` et `nuxt build` passent ; aucun avertissement Rust dans les fichiers du jeu.
- Migration rejouée deux fois sans erreur, données intactes.
- Journal des gains : aucune clé d'idempotence en double ; scores par saison et cumuls joueur égaux au journal.
- Une ligne d'audit par mutation d'administration (17 types d'action relevés).
- `/activites/**` à 375, 768 et 1280 px : aucune colonne à 0 px, aucun débordement ; aucune classe daisyUI hors `/admin/**`.
- Duel direct dans deux navigateurs : 1 ms d'écart d'apparition de la question, 0 ms pour la correction ; coupure réseau de 8 s rattrapée ; forfait après le délai de grâce.
- Classements à 10 000 membres : 10 à 40 ms.
- Non-régression : messages et appels entre amis toujours reçus en temps réel.

## Limites connues

- Avec 10 épreuves par partie, « jouer sur ce pays » n'est possible que pour les pays riches en contenu (une fiche pays donne 5 épreuves dérivées).
- La Gambie, enclavée et minuscule, est difficile à viser au clic sur la carte.
- Les 16 autres modules se brancheront sur le socle dans des vagues ultérieures.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
