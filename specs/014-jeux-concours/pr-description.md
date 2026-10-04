# Jeux variés et concours communautaires

Spec, plan, recette : `specs/014-jeux-concours/`. Prolonge la feature 013 (activités ludiques) **sans modifier ses règles** de score, de défi, de duel ni de Championship.

## Pourquoi

Après la recette de la 013, le client trouvait les jeux trop uniformes : on ne faisait que répondre à des questions à choix multiple. Cette PR ajoute deux familles :

- **A — d'autres façons de répondre**, sur le moteur existant ;
- **B — des concours où la communauté juge**, premier format : la bataille de photos.

## Famille A : carte, ordre, paires, photo et son

- **Trois nouveaux types de réponse** : désigner un pays sur la carte (ou dans une liste, au clavier), remettre des éléments dans l'ordre, associer des paires. Ils se jouent partout : partie, défis, duel différé, duel direct.
- **La solution ne quitte jamais le serveur avant la réponse** : l'ordre et la colonne de droite des paires sont stockés mélangés, et le joueur ne manipule que des numéros qui ne révèlent rien.
- **Pas de demi-point** : un ordre ou des paires sont justes si et seulement si tout est juste.
- **Plus de temps** pour l'ordre et les paires (15 s de plus, réglables).
- **Photo et son** déposés par l'administrateur. Les métadonnées des photos (dont le GPS) sont retirées. Les sons sont reconnus à leur contenu, pas à leur extension, et limités à 30 s et 1 Mo.
- **13 nouvelles formes de dérivation** à partir du contenu publié : classer des pays par population ou superficie, associer pays et capitales ou monnaies, carte par capitale, site ou peuple, photo d'un plat ou d'un site, proverbes ↔ pays, citations ↔ auteurs, idées reçues ↔ réalité. Toutes passent par la revue avant d'être jouables.

## Famille B : concours de photos

- **Programmation et modération** : thème, règlement, dates, module de rattachement. Les règles sont figées une fois l'appel ouvert. Chaque photo est relue avant publication ; un refus exige un motif, que le membre reçoit.
- **Dépôt** : une photo et une légende, remplaçables tant qu'elles ne sont pas publiées. La galerie reste anonyme et mélangée jusqu'aux résultats.
- **Vote à l'aveugle** : deux photos sans auteur, on choisit la meilleure. Jamais sa propre photo, jamais deux fois la même paire, et les photos les moins vues passent en premier.
- **Voix suspectes enregistrées mais écartées, sans le dire au votant** : vote trop rapide, compte créé après l'ouverture du vote, adresse non vérifiée.
- **Résultats établis automatiquement à la clôture** :
  - classement selon la part de duels gagnés ;
  - une photo trop peu présentée est classée hors podium ;
  - les ex aequo partagent le rang et la prime.
- **Récompenses** : score de jeu (compte pour le Championship), réputation et distinction « Lauréat ». **Aucun point d'engagement.** Chaque récompense est versée une seule fois.
- **Jury en option** : il fixe le podium parmi les finalistes. S'il ne délibère pas dans le délai, le classement de la communauté s'applique.
- **Protection** :
  - un suivi du vote signale les comptes anormaux (rythme, volume, préférence systématique) ;
  - l'administrateur écarte ou rétablit leurs voix ;
  - au-delà de 10 signalements, une photo est suspendue.

## Base de données

Une migration, **`uafricas_backend/doc/bd/schemas/38_jeu_concours.sql`**, additive et rejouable. Elle **suppose `37_jeu.sql`**.

- Colonnes de type de réponse sur `jeu.epreuve` et `jeu.reponse`, avec des contraintes qui rendent une épreuve incohérente impossible en base.
- Tables `jeu.concours`, `participation`, `confrontation`, `resultat_concours`, `signalement_participation`.
- Trois règles d'engagement **à 0 point** et deux distinctions.

## Ouverture en production

1. **Vérifier que `37_jeu.sql` est appliquée** (la 013 doit être en production) :
   ```bash
   ./deploy.sh psql "SELECT to_regclass('jeu.epreuve')"
   ```
2. **Migration avant le code** (l'ancien code ignore ce qu'elle ajoute) :
   ```bash
   git push origin main
   ./deploy.sh migrate uafricas_backend/doc/bd/schemas/38_jeu_concours.sql
   ./deploy.sh update
   ```
3. **Dériver puis revoir les nouvelles formes** dans `/admin/activites/revue` (la solution s'affiche dans la liste), et saisir des épreuves carte, ordre et paires pour Afrolang.
4. **Premier concours** dans `/admin/activites/concours` : prévoir un appel d'au moins une semaine. **Adapter le seuil de « duels minimum pour le podium »** (10 par défaut) à la participation attendue : en dessous, aucune photo n'accède au podium.

## Hypothèses à faire confirmer par le client

Retenues sur recommandation, consignées dans la spec (section *Clarifications*), et renversables.

| | Hypothèse | Ce qu'elle implique |
|---|---|---|
| **H1** | Vote à l'aveugle par paires, jury en option | Les J'aime ne désignent pas le gagnant. |
| **H2** | Récompenses en score de jeu et réputation | Aucun point d'engagement. |
| **H3** | Modération avant publication | Signalement après publication. |
| **H4** | 60 s par vidéo | Pour les défis vidéo à venir. |

Conséquence de H1 et H2 : les photos de concours ne portent pas de J'aime, puisque chaque J'aime reçu crédite des points d'engagement.

## Vérifications faites en local

- `cargo build` et `pnpm build` passent ; aucun avertissement Rust dans les fichiers du jeu.
- Migration `38` rejouée deux fois, même nombre d'épreuves servables avant et après.
- **Journal des gains** :
  - aucune clé d'idempotence en double ;
  - scores égaux au journal ;
  - 1 000 relectures des résultats, un seul versement.
- **Navigateur** : pages publiques à 375, 768 et 1280 px sans débordement ; aucune classe daisyUI hors du back-office.
- **Duel direct** à deux navigateurs sur carte, ordre et paires.
- **Vote** :
  - jamais sa propre photo, jamais une paire revue ;
  - présentations équilibrées (18 à 19 par photo après 48 votes) ;
  - aucun auteur ni décompte dans les réponses du serveur.
- **Non-régression de la 013** : partie à choix multiple, défis, classements.

## Défauts corrigés au passage

- **Envoi de fichiers dans le back-office** : `adminFetch` imposait le type JSON à tout envoi de fichier, qui échouait en 400. Les composables existants le contournaient.
- **Après un rechargement**, le jeton de connexion n'était restauré qu'après l'ouverture de la page d'un concours : un membre connecté se voyait invité à se connecter.
- **Podium** : avec deux 2ᵉ ex aequo, le gagnant était poussé hors du centre.

## Limites connues

- Le panneau d'activités n'est monté que sur les quatre modules jouables : un concours rattaché à Afroculture apparaît sur l'espace Activités, pas encore sur la page d'Afroculture.
- Une image source externe (`https://…`) n'est pas copiée lors de la dérivation : la copier demanderait un client HTTP.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
