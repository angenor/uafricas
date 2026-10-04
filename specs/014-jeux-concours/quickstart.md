# Quickstart : recette des jeux variés et des concours

**Feature** : `014-jeux-concours`. Pas de harnais de tests dans le dépôt : la vérification se fait par les scénarios ci-dessous, déroulés par l'API et dans un navigateur.

## Prérequis

- Environnement local de la 013 en marche : PostgreSQL, backend sur 8080, frontend sur 3000.
- Migrations `37_jeu.sql` puis `38_jeu_concours.sql` appliquées ; seeds `99`, `98`, `95` de la 013 appliqués.
- Comptes locaux : `test-admin@test.com` (admin) et `martialdjezou@gmail.com` (membre), mot de passe `Test1234`. Pour le vote, au moins **trois** comptes membres vérifiés, créés **avant** l'ouverture du vote.
- Après l'ajout de composables, **redémarrer** le serveur Nuxt (piège rencontré pendant la 013).

```bash
docker exec -i uafricas_postgres psql -U uafricas -d africans_db -v ON_ERROR_STOP=1 < uafricas_backend/doc/bd/schemas/38_jeu_concours.sql
# rejouer une seconde fois : aucune erreur, aucun doublon
```

## Scénario 1 — Répondre autrement (US1)

1. Admin : saisir une épreuve de chaque type (`carte`, `ordre` à 4 éléments avec valeurs, `paires` à 4 paires), les publier.
2. Membre : lancer des parties dans le module jusqu'à rencontrer chacun des types (le tirage varié les fait alterner).
3. Pour chaque type, répondre **juste** une fois et **faux** une fois. Attendu : issue correcte ; la correction montre la solution complète (pays mis en évidence, ordre attendu avec ses valeurs, paires attendues) ; score versé comme pour un choix multiple.
4. Laisser expirer une épreuve « ordre » : `sans_reponse`, et le délai observé tient compte de la majoration.
5. Carte **au clavier seul** : tabuler jusqu'à la liste, choisir le pays, valider (SC-004).
6. **Fuite** : inspecter la réponse réseau de `suivante` pour un « ordre » et un « paires ». Les clés ne suivent pas l'ordre de la solution, et aucun champ ne la porte (SC-002). Répéter sur 20 épreuves : l'ordre des clés servies n'est jamais systématiquement celui de la solution.
7. **Réponses invalides** : un `ordre` qui omet une clé ou en répète une, un `pays` hors d'Afrique, une forme qui ne correspond pas au type → 400, rien d'écrit.
8. Duel direct sur un module dont le vivier ne contient que des épreuves `ordre` ou `paires` (en local, retirer temporairement les autres) : les deux écrans reçoivent la même épreuve, le même délai majoré, et la manche se résout.

## Scénario 2 — Photo et son (US2)

1. Admin : déposer une photo (JPEG avec EXIF GPS) et un extrait audio MP3 de 20 s via le formulaire d'épreuve. Vérifier que le fichier stocké n'a plus d'EXIF (`exiftool` ou `identify -verbose`).
2. Déposer un MP3 de plus de 30 s : refusé par l'écran ; un fichier de plus d'1 Mo : refusé par le serveur ; un `.mp3` qui est en fait un PNG renommé : refusé (signature binaire).
3. Dériver `photo_recette` et `photo_site` : les candidates portent une copie de l'image dans `uploads/jeu/images/`. Modifier ensuite l'image de la recette source : l'épreuve garde la sienne.
4. Jouer une épreuve à son : le temps ne démarre qu'à la présentation (marge média de la 013).

## Scénario 3 — Vivier des nouveaux types (US3)

1. Dériver Afripulse : `ordre_population`, `ordre_superficie`, `paires_capitales`, `paires_monnaies`, `carte_capitale`, `carte_site`, `carte_peuple` produisent des candidates.
2. Contrôles SQL d'ambiguïté (aucun résultat attendu) :
   - une épreuve `paires_monnaies` réunissant deux pays de même monnaie ;
   - une `ordre_*` dont deux valeurs consécutives sont à moins de 15 % ;
   - une `carte_peuple` dont le peuple est déclaré par deux pays.
3. Accepter des candidates en revue ; vérifier qu'aucune candidate n'est jamais servie (013, SC-003).
4. Saisie : refus d'un « ordre » à 2 éléments, d'une paire en double, d'un pays hors d'Afrique, chacun avec un message qui nomme le champ.
5. SC-005 : chaque module pilote compte au moins 30 épreuves jouables d'un type autre que `choix`. Afrolang n'y arrivera que par la saisie, ce qui est attendu.

## Scénario 4 — Programmer un concours et y participer (US4, US6)

1. Admin : créer « Mon plat du dimanche » avec un appel ouvert maintenant, un vote dans 10 minutes (en local, on avance les dates en SQL plutôt que d'attendre), clôture 24 h après. Refus attendu : vote de moins de 24 h, dates désordonnées.
2. Membre A : déposer une photo. Attendu : `en_attente`, visible de lui seul. Deuxième dépôt : 409 (`participations_max = 1`). Remplacer la photo : possible.
3. Admin : rejeter la photo de A avec un motif. A reçoit la notification et redépose. Accepter. Notification « acceptée ».
4. Membres B, C, D : déposer, faire accepter (au moins 4 publiées).
5. Concours n° 2 avec un seul participant publié : à l'heure du vote, la première lecture l'**annule**, le participant est notifié et reçoit la prime de participation (une seule fois, même après dix relectures).
6. Le concours apparaît sur `/activites` et sur la page de son module de rattachement.
7. Journal d'audit : une ligne par création, modification, modération.

## Scénario 5 — Voter à l'aveugle (US5)

1. Passer le concours en phase de vote (dates en SQL).
2. Membre A (participant) vote 15 fois. Attendu : jamais sa propre photo, jamais deux fois la même paire, aucun nom d'auteur, aucun décompte nulle part (page, réseau, galerie).
3. Recharger la page au milieu d'une paire : **la même paire** revient.
4. Avec 5 participations, un votant non participant voit au plus 10 paires, puis « vous avez tout vu ».
5. Vote moins d'une seconde après la présentation (script) : accepté, mais `comptee = false`, `motif_ecart = 'trop_rapide'` en base.
6. Compte créé après l'ouverture du vote : ses voix sont enregistrées avec `compte_recent`.
7. Après 40 votes de plusieurs comptes, l'écart entre la participation la plus présentée et la moins présentée est d'au plus 2 (équilibrage, research D7).
8. SC-007 : 20 confrontations en moins de 2 minutes, sur téléphone (375 px).

## Scénario 6 — Résultats et récompenses (US7)

1. Clôturer le vote (date en SQL), ouvrir le concours. Attendu : résultats établis à cette première lecture, auteurs révélés, rang et taux de victoire affichés, podium.
2. `jeu.resultat_concours` est rempli une fois ; relire 1 000 fois (script) ne change rien et ne verse rien de plus (SC-012).
3. Gains : une prime de participation par participation publiée, une prime de podium pour les 3 premiers, origine `concours`, comptés dans la saison et au pays de rattachement du moment.
4. **Points d'engagement et statut inchangés** pour tous les participants et votants (SC-013) : comparer `engagement.compte` avant et après.
5. Réputation (règles à 0 point) et distinction `jeu_laureat` pour le premier.
6. Ex aequo : forcer deux taux égaux (SQL sur des données de test) sans jury → même rang, même prime.
7. Le concours terminé reste consultable comme galerie, et partageable.

## Scénario 7 — Jury (US8)

1. Concours avec `jury = true` : à la clôture, phase `deliberation`, aucun résultat public.
2. Admin : `GET /finalistes` (10 premiers), puis fixer un podium différent du classement communautaire. Lecture suivante : résultats publiés, podium du jury en tête.
3. Concours avec jury sans délibération : une fois `jury_delai_jours` passé, le classement communautaire s'applique.

## Scénario 8 — Protection (US9)

1. Un compte vote 60 fois en une minute (script) : il apparaît dans `comptes_signales` du suivi.
2. Admin : écarter ses voix → le classement provisoire du suivi change ; audit `VOIX_ECARTEES`. Rétablir → retour à l'état précédent.
3. Onze comptes signalent une photo : elle passe `suspendue`, sort du vote et de la galerie ; les confrontations qui l'incluaient ne comptent plus au dépouillement.
4. Admin : rétablir → publiée, compteur à zéro.
5. Suspendre le compte d'un participant avant la clôture : sa participation et ses voix sont absentes des résultats.

## Critères de sortie

- Migration `38_jeu_concours.sql` rejouée deux fois sans erreur ; les épreuves existantes sont toutes de type `choix` et toujours servables (même décompte avant et après).
- Aucune clé d'idempotence en double dans `jeu.gain` ; agrégats de score égaux au journal (contrôle de la 013).
- Aucune classe daisyUI hors `/admin/**` ; pages `/activites/concours/**` sans débordement à 375, 768 et 1280 px.
- `cargo build` et `pnpm build` passent, sans avertissement dans les fichiers touchés.
- Non-régression de la 013 : partie, défi, duel différé, duel direct et Championship sur des épreuves `choix`.

## Avant l'ouverture en production

1. `./deploy.sh migrate uafricas_backend/doc/bd/schemas/38_jeu_concours.sql` **avant** le déploiement du code. La migration est additive : l'ancien code ignore les colonnes et tables nouvelles. Elle suppose `37_jeu.sql` appliquée.
2. Dériver les nouvelles formes, puis faire la revue.
3. Créer le premier concours avec un appel d'au moins une semaine, et annoncer le règlement, dont les règles d'admissibilité des voix.
