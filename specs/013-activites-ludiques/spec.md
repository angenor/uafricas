# Feature Specification : Activités interactives, ludiques et participatives

**Feature Branch** : `013-activites-ludiques`

**Created** : 2026-10-01

**Status** : Draft

**Input** : document client « AFRICANS WORLD - Activités interactives, ludiques et participatives ». Activités transversales : Championship panafricain, défis quotidiens et hebdomadaires, duels, carte interactive de l'Afrique, système de réputation. Activités par module : vingt modules, d'Afrolang (quiz linguistiques, jeux audio) à Africans Radio (quiz audio, radio participative).

> Cette spécification est une **réécriture**. Une première version, validée, n'avait jamais été enregistrée dans le dépôt et a été perdue avec la machine qui la portait. Les trois décisions prises alors par le client sont reprises telles quelles (section *Clarifications*) ; le reste a été reconstruit à partir du document client et de l'état réel de la plateforme.

## Contexte

Le document client énumère une soixantaine d'activités réparties sur vingt modules. Les lire une à une cache l'essentiel : **presque toutes sont la même chose**. Un quiz linguistique, un quiz sur l'Agenda 2063, un quiz après émission, un quiz scientifique, la détection de fausses nouvelles : c'est chaque fois une question, des propositions, une bonne réponse et une explication. Ce qui change d'un module à l'autre, c'est la matière, pas le mécanisme. Et les cinq activités transversales (Championship, défis, duels, carte, réputation) ne sont que des façons différentes de faire jouer ces mêmes questions et d'en restituer le résultat.

La plateforme n'a aujourd'hui **rien** de tout cela : ni question, ni partie, ni score, ni classement. Elle a en revanche une partie de la matière. Les proverbes de Codimoi, les idées reçues de FactCheck, les fiches pays, sites, recettes et personnalités d'Afripulse sont publiés, modérés et rattachés à des pays. Afrolang fait exception : la plateforme n'a aucun référentiel de langues, ses épreuves seront donc saisies.

Cette feature livre donc un **socle ludique unique**, que tout module peut utiliser sans qu'on le modifie, et l'ouvre sur **quatre modules pilotes** : Afrolang, Codimoi, Afripulse et FactCheck, les trois derniers parce que leur contenu déjà publié permet de produire des épreuves sans attendre, Afrolang parce que les quiz linguistiques ouvrent la liste du client. Les seize autres modules s'y brancheront par vagues (section *Couverture du document client*).

Un point de vocabulaire, tenu dans tout le document : une **épreuve** est une question jouable ; une **partie** est une série d'épreuves jouée par un membre ; le **score de jeu** est ce que le jeu rapporte, et il n'a rien à voir avec les points d'engagement de la plateforme.

## User Scenarios & Testing *(mandatory)*

### User Story 1 : Jouer une partie dans un module (Priority : P1)

Un membre ouvre Codimoi et y trouve un espace d'activités. Il lance une partie : dix épreuves s'enchaînent, une à la fois, chacune en temps limité. Après chaque réponse, il voit s'il avait raison, lit l'explication et peut ouvrir le proverbe dont l'épreuve est tirée. À la fin, un bilan lui donne ses bonnes réponses et le score de jeu gagné.

**Why this priority** : c'est le geste élémentaire dont tout le reste dépend. Un défi, un duel, un classement ne sont que des parties jouées dans un cadre particulier. Livrée seule, cette histoire donne déjà un jeu complet sur quatre modules.

**Independent Test** : avec un compte de membre et un module doté d'au moins dix épreuves jouables, lancer une partie, répondre à toutes les épreuves, vérifier le bilan et le score acquis.

**Acceptance Scenarios** :

1. **Given** un membre connecté et un module pilote doté d'assez d'épreuves, **When** il lance une partie, **Then** il reçoit une série d'épreuves qu'il n'a jamais jouées, présentées une à une.
2. **Given** une épreuve affichée, **When** le membre choisit une proposition dans le temps imparti, **Then** il voit la bonne réponse, l'explication et, si l'épreuve en référence un, le lien vers le contenu d'origine.
3. **Given** une épreuve affichée, **When** le temps imparti s'écoule sans réponse, **Then** l'épreuve est comptée sans réponse, la bonne réponse est montrée et la partie passe à la suivante.
4. **Given** la dernière épreuve jouée, **When** la partie se termine, **Then** un bilan indique le nombre de bonnes réponses, le score de jeu gagné et le total du membre.
5. **Given** un visiteur non connecté, **When** il ouvre l'espace d'activités d'un module, **Then** il en voit la présentation et les classements, et l'entrée de jeu le mène à la connexion.
6. **Given** un membre qui a déjà joué toutes les épreuves d'un module, **When** il relance une partie, **Then** il est prévenu **avant de commencer** qu'il s'agit d'un entraînement qui ne rapporte aucun score.

---

### User Story 2 : Constituer et tenir le vivier d'épreuves (Priority : P1)

Un administrateur ouvre le back-office des activités. Il saisit une épreuve pour Afrolang : un énoncé accompagné d'un extrait sonore, quatre propositions, la bonne, une explication, un niveau de difficulté. Il la publie ; elle devient jouable. Plus tard, il constate qu'une épreuve est ratée par presque tout le monde, la relit, la corrige ou la retire.

**Why this priority** : sans épreuve, il n'y a rien à jouer. La saisie par un administrateur est le chemin le plus court vers une première partie, et c'est le seul qui convienne aux modules dont le contenu ne se prête pas à la dérivation. P1 avec l'histoire 1 : les deux forment le minimum viable.

**Independent Test** : saisir dix épreuves dans un module, les publier, vérifier qu'une partie devient possible ; en retirer une, vérifier qu'elle n'est plus servie et que les scores déjà gagnés dessus sont intacts.

**Acceptance Scenarios** :

1. **Given** un administrateur habilité, **When** il saisit une épreuve complète et la publie, **Then** elle peut être servie dans une partie du module.
2. **Given** une épreuve incomplète (pas de bonne réponse, une seule proposition, pas d'explication), **When** l'administrateur tente de la publier, **Then** la publication est refusée et le manque est nommé.
3. **Given** une épreuve jouable, **When** l'administrateur la retire, **Then** elle n'est plus servie à personne, et les scores acquis sur elle restent acquis.
4. **Given** un module dont le nombre d'épreuves jouables est inférieur à la taille d'une partie, **When** un membre ouvre son espace d'activités, **Then** aucune partie n'est proposée et le module s'annonce comme bientôt disponible.
5. **Given** un compte sans l'habilitation dédiée, **When** il tente d'accéder au back-office des activités, **Then** l'accès lui est refusé.

---

### User Story 3 : Relever le défi du jour et le défi de la semaine (Priority : P2)

Chaque jour, un membre trouve un défi : une courte série d'épreuves, la même pour tous. Il n'a qu'un essai. S'il revient le lendemain, sa série de jours consécutifs s'allonge. Chaque semaine, un défi plus long, sur un thème, court du lundi au dimanche.

**Why this priority** : la partie libre donne une raison de jouer, le défi donne une raison de **revenir**. C'est le premier levier de fidélité, et il ne demande rien d'autre que le socle de l'histoire 1.

**Independent Test** : jouer le défi du jour avec deux comptes, vérifier que les épreuves sont identiques ; tenter de le rejouer, vérifier le refus ; changer de jour, vérifier qu'un nouveau défi est proposé et que la série de jours a progressé.

**Acceptance Scenarios** :

1. **Given** deux membres le même jour, **When** chacun ouvre le défi du jour, **Then** ils reçoivent exactement les mêmes épreuves, dans le même ordre.
2. **Given** un membre qui a commencé le défi du jour, **When** il tente de le recommencer, **Then** le système refuse et lui montre son résultat.
3. **Given** un membre qui a terminé le défi hier et le termine aujourd'hui, **When** il consulte son espace, **Then** sa série de jours consécutifs vaut un de plus qu'hier.
4. **Given** un membre qui a manqué un jour, **When** il joue le défi suivant, **Then** sa série repart de un.
5. **Given** un jour pour lequel aucun administrateur n'a rien préparé, **When** le premier membre ouvre le défi, **Then** un défi lui est proposé, et tout membre qui l'ouvre ensuite reçoit le même.
6. **Given** un défi dont la période est passée, **When** un membre le consulte, **Then** il ne peut plus le jouer mais voit son résultat et son rang parmi les participants.

---

### User Story 4 : Défier un ami en duel différé (Priority : P2)

Un membre propose un duel à un ami sur Afripulse. L'ami accepte. Chacun joue la même série quand il le veut, dans les 48 heures. Quand le second a joué, le résultat tombe pour les deux. Si l'un des deux ne joue pas, l'autre gagne par forfait.

**Why this priority** : le duel transforme un jeu solitaire en jeu social et fait venir des membres qui ne seraient pas venus seuls. Le mode différé est celui qui convient à une communauté dispersée sur plusieurs fuseaux et à des connexions irrégulières : personne n'a besoin d'être là en même temps.

**Independent Test** : avec deux comptes amis, proposer, accepter, jouer des deux côtés, vérifier le vainqueur et le gain ; recommencer en ne jouant que d'un côté, attendre l'échéance, vérifier le forfait.

**Acceptance Scenarios** :

1. **Given** deux membres liés par une amitié en vigueur, **When** l'un propose un duel différé sur un module, **Then** l'autre en est notifié et peut l'accepter ou le refuser.
2. **Given** un duel accepté, **When** chacun joue, **Then** tous deux affrontent les mêmes épreuves dans le même ordre.
3. **Given** un duel où un seul joueur a joué, **When** il consulte le duel, **Then** il ne voit pas son score comparé et l'autre ne voit rien de son résultat.
4. **Given** un duel où les deux ont joué, **When** l'un ou l'autre le consulte, **Then** il voit les deux résultats, le vainqueur et le gain.
5. **Given** un duel accepté où un seul a joué, **When** les 48 heures sont écoulées, **Then** celui qui a joué est déclaré vainqueur par forfait.
6. **Given** une proposition restée sans réponse, **When** les 48 heures sont écoulées, **Then** le duel est expiré, sans vainqueur ni gain.
7. **Given** deux membres qui ne sont pas amis, **When** l'un tente de défier l'autre, **Then** la proposition est refusée.

---

### User Story 5 : Suivre le Championship panafricain (Priority : P2)

Le Championship se joue par saisons. Tout score de jeu gagné pendant une saison y compte. Un membre voit son rang au classement de tous les membres, son rang dans le classement de son pays, et le rang de son pays parmi les autres. À la fin de la saison, les classements sont figés, les meilleurs sont distingués, une nouvelle saison repart de zéro.

**Why this priority** : c'est ce qui donne un sens collectif au score. Un Ivoirien de Paris joue pour la Côte d'Ivoire : le Championship fait du jeu une affaire de pays, ce qui est l'esprit même de la plateforme. Il suppose que du score existe déjà, d'où P2.

**Independent Test** : avec une saison ouverte et plusieurs comptes de pays différents ayant joué, vérifier les trois classements ; changer le pays d'un compte, rejouer, vérifier que les gains antérieurs n'ont pas bougé.

**Acceptance Scenarios** :

1. **Given** une saison en cours, **When** un membre gagne du score de jeu, **Then** son score de saison augmente d'autant et son rang se met à jour.
2. **Given** un membre dont le pays d'origine est renseigné, **When** il gagne du score, **Then** ce score est porté au compte de son pays d'origine.
3. **Given** un membre sans pays d'origine mais avec un pays de résidence, **When** il gagne du score, **Then** ce score est porté au compte de son pays de résidence.
4. **Given** un membre sans aucun pays renseigné, **When** il gagne du score, **Then** il figure au classement de tous les membres et dans aucun classement de pays, et il est invité à compléter son profil.
5. **Given** un membre qui change de pays en cours de saison, **When** il consulte les classements, **Then** le score gagné avant le changement reste au compte de l'ancien pays, et seul le score gagné ensuite va au nouveau.
6. **Given** une saison dont la date de fin est passée, **When** quiconque consulte le Championship, **Then** le classement de cette saison est figé et consultable dans les archives, sans qu'un administrateur ait eu à intervenir.
7. **Given** un membre classé hors des premières places, **When** il consulte un classement, **Then** il voit son propre rang et ses voisins immédiats.

---

### User Story 6 : Explorer la carte interactive de l'Afrique (Priority : P3)

Un membre ouvre la carte. Chaque pays y est teinté selon sa place au Championship. Il touche le Sénégal : il voit le rang du pays, ses meilleurs joueurs, et peut lancer une partie sur le Sénégal.

**Why this priority** : la carte n'ajoute pas de règle de jeu, elle rend le Championship lisible d'un coup d'œil et sert de porte d'entrée par pays. Elle suppose les classements de l'histoire 5.

**Independent Test** : avec une saison en cours et des scores dans plusieurs pays, ouvrir la carte, vérifier que la teinte suit le classement, sélectionner un pays, lancer une partie sur ce pays.

**Acceptance Scenarios** :

1. **Given** une saison en cours, **When** un membre ouvre la carte, **Then** chacun des pays d'Afrique y apparaît, teinté selon son rang.
2. **Given** la carte affichée, **When** le membre sélectionne un pays, **Then** il voit le rang du pays, son score et ses meilleurs joueurs.
3. **Given** un pays doté d'assez d'épreuves qui le concernent, **When** le membre lance une partie depuis la carte, **Then** toutes les épreuves de la partie portent sur ce pays.
4. **Given** un pays pour lequel personne n'a encore de score, **When** le membre le sélectionne, **Then** le pays s'annonce comme sans joueur et invite à être le premier, il n'est ni masqué ni affiché en erreur.
5. **Given** un écran étroit ou un lecteur d'écran, **When** le membre ouvre la carte, **Then** la même information est disponible sous forme de liste.

---

### User Story 7 : Affronter un ami en duel direct (Priority : P3)

Deux amis sont en ligne. L'un propose un duel direct, l'autre accepte aussitôt. Ils voient la même épreuve au même instant, répondent chacun de son côté, découvrent ensemble la bonne réponse, puis passent à la suivante. Le vainqueur est connu à la dernière épreuve.

**Why this priority** : c'est la forme la plus spectaculaire du duel, mais aussi le seul point de toute la feature qui exige que deux écrans avancent ensemble. Séparée de l'histoire 4, elle se livre après sans rien changer au cycle de vie du duel, et un imprévu sur la synchronisation ne bloque pas le duel différé.

**Independent Test** : avec deux comptes amis connectés en même temps, proposer et accepter un duel direct, jouer jusqu'au bout ; recommencer en coupant la connexion d'un joueur en pleine partie.

**Acceptance Scenarios** :

1. **Given** deux amis connectés, **When** l'un propose un duel direct et l'autre accepte, **Then** la partie commence pour les deux au même moment.
2. **Given** un duel direct en cours, **When** une épreuve est présentée, **Then** elle l'est aux deux joueurs en même temps, avec le même temps imparti.
3. **Given** une épreuve en cours, **When** les deux joueurs ont répondu ou que le temps est écoulé, **Then** la bonne réponse est révélée aux deux et l'épreuve suivante commence.
4. **Given** un joueur dont la connexion tombe, **When** il revient avant la fin du délai de grâce, **Then** il reprend à l'épreuve en cours, et celles qu'il a manquées sont comptées sans réponse.
5. **Given** un joueur absent au-delà du délai de grâce, **When** ce délai expire, **Then** son adversaire est déclaré vainqueur par forfait.
6. **Given** une proposition de duel direct restée sans réponse quelques minutes, **When** elle expire, **Then** celui qui l'a faite peut la transformer en duel différé.

---

### User Story 8 : Tirer des épreuves du contenu déjà publié (Priority : P2)

Un administrateur demande au système de proposer des épreuves à partir des proverbes publiés de Codimoi. Le système lui présente des épreuves candidates. Il les passe en revue : il en accepte certaines telles quelles, en corrige d'autres avant de les accepter, en rejette avec un motif. Seules les acceptées deviennent jouables.

**Why this priority** : la saisie à la main suffit pour démarrer, pas pour tenir vingt modules. La dérivation donne le volume, la revue garantit la qualité. Elle vient juste après le minimum viable parce que sans elle les viviers s'épuisent vite et les parties tournent à l'entraînement.

**Independent Test** : demander la dérivation sur un module pilote, vérifier que des candidates apparaissent et qu'aucune n'est jouable ; en accepter une, vérifier qu'elle l'est ; relancer la dérivation, vérifier qu'aucun doublon n'apparaît.

**Acceptance Scenarios** :

1. **Given** un module pilote doté de contenu publié, **When** l'administrateur demande la dérivation, **Then** des épreuves candidates sont créées, chacune rattachée au contenu dont elle est tirée.
2. **Given** une épreuve candidate, **When** un membre lance une partie, **Then** elle ne lui est jamais servie.
3. **Given** une candidate, **When** l'administrateur l'accepte, éventuellement après correction, **Then** elle devient jouable.
4. **Given** une candidate, **When** l'administrateur la rejette avec un motif, **Then** elle ne sera plus jamais proposée pour ce contenu et cette forme de question.
5. **Given** une dérivation déjà faite, **When** l'administrateur la redemande, **Then** seules des candidates portant sur du contenu ou des formes de question nouveaux apparaissent.
6. **Given** une épreuve dérivée jouable, **When** le contenu dont elle est tirée est dépublié, suspendu ou supprimé, **Then** l'épreuve cesse aussitôt d'être servie.
7. **Given** une épreuve dérivée jouable, **When** le contenu dont elle est tirée est modifié, **Then** l'épreuve est retirée du jeu et revient en revue.

---

### User Story 9 : Gagner en réputation et en distinctions (Priority : P3)

Un membre qui tient ses défis, gagne ses duels et finit sur un podium voit sa réputation monter et reçoit des distinctions visibles sur son profil. Un membre qui signale une épreuve erronée, et que l'administrateur confirme, y gagne aussi.

**Why this priority** : c'est la récompense durable du jeu, celle qui survit aux saisons. Elle ne crée aucune mécanique de jeu nouvelle et s'appuie sur tout ce qui précède.

**Independent Test** : terminer un défi, gagner un duel, faire confirmer un signalement d'épreuve ; vérifier que la réputation a monté à chaque fois, que les distinctions prévues sont attribuées une seule fois, et que ni les points d'engagement ni le statut du membre n'ont bougé.

**Acceptance Scenarios** :

1. **Given** un membre qui termine un défi ou gagne un duel compté, **When** le résultat est acquis, **Then** sa réputation augmente du montant prévu pour cet accomplissement.
2. **Given** un membre qui franchit un jalon (première partie, série de sept jours, dixième duel gagné), **When** le jalon est atteint, **Then** la distinction correspondante lui est attribuée, une seule fois, et il en est notifié.
3. **Given** un accomplissement ludique quelconque, **When** il est acquis, **Then** ni les points d'engagement du membre ni son statut ne changent.
4. **Given** un membre dont le signalement d'épreuve est confirmé, **When** l'administrateur statue, **Then** sa réputation augmente et il en est notifié.

---

### Edge Cases

- **Vivier épuisé en cours de saison** : un membre assidu a tout joué dans un module. Il n'est pas bloqué : la partie devient un entraînement, annoncé avant de commencer, et les défis et duels restent ouverts.
- **Abandon en cours de partie** : l'épreuve affichée au moment de l'abandon est comptée sans réponse, sinon quitter serait un moyen de voir une question sans la jouer. Les épreuves non encore affichées restent neuves pour le membre.
- **Épreuve retirée pendant qu'un membre y répond** : sa réponse est acceptée et comptée ; l'épreuve n'est simplement plus servie ensuite.
- **Épreuve dont la bonne réponse était fausse** : l'administrateur la corrige ou la retire. Les scores déjà distribués ne sont pas repris : reprendre des gains pour une erreur qui n'est pas celle du joueur serait vécu comme une injustice.
- **Contenu source dépublié alors que l'épreuve figure dans un duel accepté** : la série du duel est figée à l'acceptation, les deux joueurs la jouent telle quelle, sinon ils n'affronteraient plus les mêmes épreuves.
- **Changement de pays en cours de saison** : le membre peut apparaître dans deux classements de pays, chacun pour le score gagné sous ce pays. C'est la conséquence voulue du pays figé à l'acquisition.
- **Égalité de score au classement** : le premier à avoir atteint ce score est classé devant.
- **Compte suspendu ou supprimé** : il disparaît de tous les classements aussitôt, et y revient avec son score s'il est rétabli. Ses duels en cours sont annulés sans gain.
- **Amitié rompue ou blocage pendant un duel** : le duel est annulé sans vainqueur ni gain.
- **Les deux joueurs absents d'un duel direct** : le duel se termine sans vainqueur ni gain.
- **Deux amis qui enchaînent les duels pour gonfler leur score** : au-delà d'un nombre de duels comptés par jour entre les deux mêmes membres, les duels restent possibles mais deviennent amicaux, sans gain, et c'est annoncé avant de commencer.
- **Passage de minuit pendant le défi du jour** : un défi commencé avant la fin du jour se termine normalement et compte pour le jour où il a été commencé.
- **Aucune saison en cours** : on joue quand même, le score de jeu s'accumule au total du membre, mais ne compte pour aucune saison.
- **Extrait sonore qui ne se charge pas** : le temps imparti ne commence qu'une fois l'épreuve réellement présentée ; une épreuve impossible à présenter n'est pas comptée contre le membre.
- **Signalement d'une épreuve déjà retirée** : il est accepté et classé comme déjà traité, pour ne pas laisser le membre sans réponse.

## Requirements *(mandatory)*

### Functional Requirements

**Accès et modules**

- **FR-001** : Jouer DOIT exiger d'être connecté. Un visiteur DOIT pouvoir consulter la présentation des activités et les classements, et toute entrée de jeu DOIT le mener à la connexion.
- **FR-002** : Un compte en attente de validation ou suspendu NE DOIT PAS pouvoir jouer.
- **FR-003** : Chaque module ouvert au jeu DOIT présenter un espace d'activités donnant accès à la partie libre, au défi en cours, aux duels et au classement. À la livraison, quatre modules sont ouverts : Afrolang, Codimoi, Afripulse et FactCheck.
- **FR-004** : L'ouverture d'un module supplémentaire au jeu DOIT se faire depuis l'administration, sans modifier les règles communes à tous les modules.
- **FR-005** : Un module dont le nombre d'épreuves jouables est inférieur à la taille d'une partie NE DOIT PAS proposer de partie ; il s'annonce comme bientôt disponible.

**Épreuves**

- **FR-006** : Une épreuve DOIT porter un énoncé, de deux à six propositions dont **exactement une** bonne, une explication, un niveau de difficulté parmi trois, et le module auquel elle appartient. Elle PEUT porter un thème et un pays.
- **FR-007** : Une épreuve PEUT référencer un contenu déjà publié sur la plateforme (proverbe, idée reçue vérifiée, fiche pays, site, recette, personnalité, langue). L'explication DOIT alors offrir un lien vers ce contenu.
- **FR-008** : Une épreuve DOIT naître de l'une de deux origines : saisie par un administrateur, ou dérivée d'un contenu publié. Une épreuve dérivée DOIT naître à l'état *candidate* et NE DOIT JAMAIS être servie à un membre dans cet état.
- **FR-009** : Seul un administrateur DOIT pouvoir rendre une épreuve jouable. Une épreuve saisie par un administrateur PEUT être rendue jouable sans passer par la revue.
- **FR-010** : L'énoncé d'une épreuve PEUT être accompagné d'une image ou d'un extrait sonore. Un extrait sonore DOIT pouvoir être réécouté tant que le temps imparti n'est pas écoulé.
- **FR-011** : Une épreuve dont le contenu référencé n'est plus publié DOIT cesser d'être servie aussitôt, sans attendre aucun traitement.
- **FR-012** : Une épreuve dérivée dont le contenu source a été modifié après son acceptation DOIT être retirée du jeu et revenir en revue.
- **FR-013** : Une épreuve retirée NE DOIT PLUS être servie ; les réponses déjà données et les scores déjà acquis sur elle DOIVENT être conservés.

**Parties**

- **FR-014** : Un membre DOIT pouvoir lancer une partie dans un module, éventuellement restreinte à un thème ou à un pays. Une partie est une série d'un nombre fixe d'épreuves.
- **FR-015** : La série DOIT être composée en priorité d'épreuves auxquelles le membre n'a jamais répondu.
- **FR-016** : Quand les épreuves neuves ne suffisent plus à composer une série, la partie DOIT être proposée comme **entraînement**, ne rapportant aucun score, et le membre DOIT en être prévenu avant de commencer.
- **FR-017** : Les épreuves DOIVENT être présentées une à la fois, chacune en temps limité. Une épreuve dont le temps s'écoule sans réponse est comptée sans réponse.
- **FR-018** : Après chaque réponse ou expiration, le système DOIT montrer la bonne réponse, l'explication et le lien vers le contenu d'origine s'il existe.
- **FR-019** : Une partie interrompue DOIT pouvoir être reprise pendant 24 heures à la première épreuve non encore présentée. L'épreuve affichée au moment de l'interruption est comptée sans réponse. Passé ce délai, la partie est close en l'état.
- **FR-020** : En fin de partie, le système DOIT présenter un bilan : bonnes réponses, score de jeu gagné, total du membre et effet sur son rang.
- **FR-021** : Un membre DOIT pouvoir consulter l'historique de ses parties et de leurs résultats.

**Score de jeu**

- **FR-022** : Une bonne réponse comptée DOIT rapporter un score de jeu fonction du niveau de difficulté de l'épreuve. Une mauvaise réponse ou une absence de réponse rapporte zéro ; le score de jeu NE DOIT JAMAIS diminuer du fait d'une réponse.
- **FR-023** : Le score de jeu DOIT être distinct des points d'engagement. Aucune activité ludique NE DOIT créditer de points d'engagement ni modifier le statut d'un membre.
- **FR-024** : Chaque gain DOIT être enregistré avec son origine (partie, défi, duel), son instant et le pays de rattachement du membre à cet instant.
- **FR-025** : Un même événement NE DOIT JAMAIS créditer deux fois, quelles que soient les répétitions d'envoi ou les reprises.
- **FR-026** : Un membre DOIT pouvoir consulter son score total, son score de la saison en cours et leur répartition par module et par origine.

**Intégrité du jeu**

- **FR-027** : La bonne réponse d'une épreuve NE DOIT JAMAIS être communiquée au membre avant que sa réponse soit enregistrée ou que le temps soit écoulé.
- **FR-028** : Le temps imparti DOIT être mesuré par le système et non par l'appareil du membre ; une réponse arrivée hors délai est refusée.
- **FR-029** : En partie libre, seule la **première** réponse d'un membre à une épreuve DOIT compter pour le score ; aucune épreuve ne peut être rejouée pour améliorer un score acquis.
- **FR-030** : L'ordre des propositions d'une épreuve DOIT varier d'une présentation à l'autre.
- **FR-031** : Un administrateur DOIT pouvoir annuler les gains d'un membre dont la triche est établie, en indiquant un motif ; le membre en est notifié.

**Défis**

- **FR-032** : Le système DOIT proposer chaque jour un défi du jour : une série d'épreuves **identique pour tous les membres**, ouverte pendant un jour calendaire défini selon une heure de référence unique pour toute la plateforme.
- **FR-033** : Le système DOIT proposer chaque semaine un défi de la semaine : une série plus longue, identique pour tous, ouverte du lundi au dimanche.
- **FR-034** : Un membre NE DOIT disposer que d'une participation par défi ; un défi commencé est consommé.
- **FR-035** : Un défi DOIT exister sans que personne ait eu à le préparer : un administrateur peut le programmer à l'avance, à défaut il est composé automatiquement, et il DOIT être le même pour tous quel que soit le premier membre à l'ouvrir.
- **FR-036** : Terminer un défi DOIT rapporter le score des bonnes réponses augmenté d'une prime d'achèvement. La série d'un défi étant imposée, ses bonnes réponses comptent même si le membre a déjà rencontré l'épreuve en partie libre ; il en va de même en duel.
- **FR-037** : Le système DOIT tenir, pour chaque membre, sa série de jours consécutifs de défi terminé ; elle repart de un après un jour manqué.
- **FR-038** : Un défi dont la période est passée NE DOIT PLUS être jouable ; ses résultats et le rang du membre parmi les participants DOIVENT rester consultables.

**Duels**

- **FR-039** : Un membre DOIT pouvoir proposer un duel à un membre avec lequel il a une amitié en vigueur, en choisissant le module et le mode : différé ou direct.
- **FR-040** : Un duel DOIT opposer exactement deux membres et suivre un cycle unique, commun aux deux modes : *proposé* → *accepté* → *en cours* → *terminé*, avec trois sorties : *refusé*, *annulé*, *expiré*.
- **FR-041** : Les deux joueurs d'un duel DOIVENT affronter les mêmes épreuves, dans le même ordre ; la série est figée à l'acceptation et ne change plus.
- **FR-042** : En mode différé, une proposition restée sans réponse pendant 48 heures DOIT expirer. Une fois le duel accepté, chaque joueur dispose de 48 heures pour jouer, et aucun NE DOIT voir le résultat de l'autre avant d'avoir joué lui-même.
- **FR-043** : En mode différé, si un seul joueur a joué à l'échéance, il DOIT être déclaré vainqueur par forfait ; si aucun n'a joué, le duel expire sans vainqueur ni gain.
- **FR-044** : En mode direct, le duel NE DOIT commencer que lorsque les deux joueurs sont présents. Chaque épreuve DOIT être présentée aux deux en même temps ; la suivante commence quand les deux ont répondu ou que le temps est écoulé.
- **FR-045** : En mode direct, une épreuve manquée pour cause de déconnexion est comptée sans réponse ; le joueur DOIT pouvoir reprendre le duel en cours. Une absence prolongée au-delà d'un délai de grâce vaut forfait ; si les deux sont absents, le duel se termine sans vainqueur ni gain.
- **FR-046** : Une proposition de duel direct restée sans réponse au-delà d'un court délai DOIT expirer, et son auteur DOIT pouvoir la transformer en duel différé.
- **FR-047** : Le vainqueur DOIT être le joueur ayant le plus de bonnes réponses ; en cas d'égalité, celui dont le temps de réponse cumulé est le plus court ; à égalité parfaite, le duel est nul.
- **FR-048** : Une victoire DOIT rapporter une prime de victoire, un nul une prime moindre partagée. Au-delà d'un nombre de duels comptés par jour, par membre et entre deux mêmes membres, les duels restent jouables mais deviennent amicaux, sans gain, et les joueurs DOIVENT en être prévenus avant de commencer.
- **FR-049** : La rupture de l'amitié, un blocage ou la suspension de l'un des comptes DOIT annuler tout duel non terminé entre les deux membres, sans vainqueur ni gain.
- **FR-050** : Les joueurs DOIVENT être notifiés de la proposition, de son acceptation ou de son refus, du fait que c'est à leur tour de jouer, et du résultat.

**Championship et classements**

- **FR-051** : Le Championship DOIT être organisé en saisons portant un nom, une date de début et une date de fin. Une seule saison peut être en cours à un instant donné ; des périodes sans saison sont possibles.
- **FR-052** : Le score de saison d'un membre DOIT être la somme des gains de score de jeu acquis entre le début et la fin de la saison.
- **FR-053** : Un gain DOIT être rattaché au **pays d'origine** du membre et, si celui-ci n'est pas renseigné, à son **pays de résidence**.
- **FR-054** : Le pays de rattachement DOIT être **figé au moment où le gain est acquis**. Un changement de profil ultérieur ne s'applique qu'aux gains à venir.
- **FR-055** : Un membre sans aucun pays renseigné DOIT figurer au classement de tous les membres et dans aucun classement de pays ; il DOIT être invité à compléter son profil.
- **FR-056** : Le système DOIT présenter trois classements par saison : celui de tous les membres, celui des membres d'un pays donné, et celui des pays entre eux.
- **FR-057** : Le score d'un pays DOIT être calculé à partir de ses meilleurs joueurs, en nombre plafonné, de sorte qu'un pays peu peuplé puisse rivaliser avec un pays très peuplé.
- **FR-058** : À égalité de score, le membre ou le pays qui a atteint ce score le premier DOIT être classé devant.
- **FR-059** : Un membre DOIT voir son propre rang et ses voisins immédiats, même lorsqu'il est hors des premières places affichées.
- **FR-060** : Les classements DOIVENT être consultables sans être connecté et NE DOIVENT montrer d'un membre que son nom d'affichage, son pays de rattachement, son statut et son score.
- **FR-061** : Un compte suspendu ou supprimé DOIT disparaître de tous les classements aussitôt, et y reparaître avec son score s'il est rétabli.
- **FR-062** : À la date de fin d'une saison, son classement DOIT être figé sans intervention de quiconque, rester consultable en archive, et la saison suivante DOIT partir de zéro. Le score total des membres est conservé.
- **FR-063** : Les premiers du classement final, tous membres confondus et dans chaque pays, DOIVENT recevoir une distinction de saison.

**Carte interactive**

- **FR-064** : Le système DOIT présenter une carte de l'Afrique où chacun des 55 pays est teinté selon son rang dans la saison en cours.
- **FR-065** : La sélection d'un pays DOIT montrer son rang, son score, ses meilleurs joueurs et les activités qui le concernent.
- **FR-066** : Depuis un pays doté d'assez d'épreuves qui le concernent, le membre DOIT pouvoir lancer une partie portant uniquement sur ce pays.
- **FR-067** : Un pays sans aucun score DOIT rester visible et sélectionnable, et s'annoncer comme sans joueur.
- **FR-068** : L'information portée par la carte DOIT être disponible aussi sous forme de liste, pour les écrans étroits et les lecteurs d'écran.

**Réputation et distinctions**

- **FR-069** : Certains accomplissements ludiques DOIVENT augmenter la réputation du membre : défi terminé, duel compté gagné, place d'honneur en fin de saison, signalement d'épreuve confirmé. Les montants DOIVENT être réglables par un administrateur.
- **FR-070** : La réputation DOIT rester distincte du score de jeu et des points d'engagement, et NE DOIT PAS entrer dans le calcul du statut.
- **FR-071** : Le franchissement de jalons ludiques DOIT attribuer une distinction visible sur le profil, une seule fois par jalon et par membre, accompagnée d'une notification.
- **FR-072** : Une triche établie DOIT pouvoir entraîner un retrait de réputation, décidé par un administrateur avec un motif.

**Administration**

- **FR-073** : L'administration des activités DOIT être réservée aux comptes disposant d'une habilitation dédiée.
- **FR-074** : Un administrateur DOIT pouvoir saisir, modifier, publier et retirer une épreuve. La publication d'une épreuve incomplète DOIT être refusée en nommant ce qui manque.
- **FR-075** : Un administrateur DOIT pouvoir demander la dérivation d'épreuves candidates pour un module. Une même forme de question sur un même contenu NE DOIT JAMAIS être proposée deux fois, y compris après un rejet.
- **FR-076** : Un administrateur DOIT disposer d'un écran de revue des candidates lui permettant d'accepter telle quelle, de corriger puis d'accepter, ou de rejeter avec un motif, une à une ou par lot.
- **FR-077** : Les règles chiffrées du jeu DOIVENT être réglables sans nouvelle livraison : score par niveau de difficulté, temps par épreuve, taille des séries, primes, délais et plafonds des duels. Un changement ne s'applique qu'aux gains à venir.
- **FR-078** : Un administrateur DOIT pouvoir créer, modifier et clore une saison. Deux saisons NE DOIVENT PAS se chevaucher, et la date de début d'une saison commencée NE DOIT PLUS pouvoir être modifiée.
- **FR-079** : Un administrateur DOIT pouvoir programmer à l'avance le contenu d'un défi du jour ou de la semaine.
- **FR-080** : Un administrateur DOIT voir, pour chaque épreuve, combien de fois elle a été servie et son taux de bonnes réponses, et repérer celles dont le taux est anormalement bas ou anormalement haut.
- **FR-081** : Un administrateur DOIT pouvoir ouvrir ou fermer les activités d'un module.

**Signalement et traçabilité**

- **FR-082** : Un membre DOIT pouvoir signaler une épreuve à laquelle il a répondu (réponse erronée, énoncé ambigu, contenu déplacé), une seule fois par épreuve, avec un motif.
- **FR-083** : Les signalements DOIVENT arriver dans une file d'administration où l'administrateur peut corriger l'épreuve, la retirer ou classer le signalement ; l'auteur du signalement est notifié de la décision.
- **FR-084** : La correction ou le retrait d'une épreuve NE DOIT PAS modifier les gains déjà acquis sur elle.
- **FR-085** : Toute création, modification, publication, retrait, décision de revue, décision de signalement, changement de règle, opération sur une saison et annulation de gains DOIT être journalisée comme les autres mutations de la plateforme.

### Key Entities

- **Module ludique** : un module de la plateforme ouvert au jeu. Porte son état (ouvert ou fermé) et les formes de question qu'il sait dériver de son contenu. Quatre à la livraison.
- **Épreuve** : une question jouable. Porte son module, son énoncé et son média éventuel, ses propositions dont une bonne, son explication, son niveau de difficulté, un thème et un pays facultatifs, son origine (saisie ou dérivée), le contenu publié qu'elle référence le cas échéant. Traverse les états *candidate*, *jouable*, *à revoir*, *rejetée*, *retirée*.
- **Partie** : une série d'épreuves jouée par un membre dans un module. Porte son cadre (libre, entraînement, défi, duel), sa série figée, son avancement, son résultat.
- **Réponse** : ce qu'un membre a répondu à une épreuve dans une partie : la proposition choisie ou l'absence de réponse, le temps mis, et si elle compte pour le score.
- **Gain de score de jeu** : une ligne du journal du score. Porte le membre, le montant, l'origine (partie, défi, duel), l'instant, la saison s'il y en a une, et le **pays de rattachement figé à cet instant**. Les classements se déduisent de ce journal ; ils ne sont pas une entité.
- **Défi** : une série commune à tous, ouverte pendant une période (un jour ou une semaine). Porte sa période, sa série, son origine (programmée ou composée automatiquement).
- **Participation à un défi** : le fait qu'un membre a joué un défi donné, unique par couple membre/défi, avec son résultat. La série de jours consécutifs s'en déduit.
- **Duel** : l'affrontement de deux membres sur une série commune. Porte ses deux joueurs, son module, son mode (différé ou direct), sa série figée, son état dans le cycle, ses échéances, son issue (vainqueur, nul, forfait, sans issue) et s'il est compté ou amical.
- **Saison** : une période du Championship. Porte son nom, ses dates, et si elle est à venir, en cours ou close.
- **Signalement d'épreuve** : le fait qu'un membre a signalé une épreuve, avec son motif et la décision prise. Unique par couple membre/épreuve.
- **Règles du jeu** : l'ensemble des valeurs réglables par l'administration (scores, temps, tailles, primes, délais, plafonds, montants de réputation).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001** : Un membre connecté lance une partie en trois gestes au plus depuis la page d'un module pilote, et la termine en moins de cinq minutes.
- **SC-002** : À la livraison, chacun des quatre modules pilotes dispose d'au moins 100 épreuves jouables.
- **SC-003** : Sur un jeu de contrôle, aucune épreuve à l'état candidate, à revoir, rejetée ou retirée n'est jamais servie à un membre : 100 % d'absence.
- **SC-004** : Sur un jeu de contrôle, aucune bonne réponse n'est observable par le membre avant qu'il ait répondu ou que le temps soit écoulé.
- **SC-005** : Rejouer, renvoyer ou reprendre une même épreuve ne produit jamais un second gain : sur 1 000 répétitions de contrôle, un seul gain est enregistré.
- **SC-006** : Après la livraison, les points d'engagement et le statut d'un membre sont strictement identiques avant et après n'importe quelle activité ludique.
- **SC-007** : Deux membres ouvrant le défi du jour à n'importe quelle heure de la même journée reçoivent la même série dans 100 % des cas.
- **SC-008** : Un administrateur traite une épreuve candidate (accepter, corriger ou rejeter) en moins de 30 secondes en moyenne.
- **SC-009** : Un duel différé va de la proposition au résultat sans qu'à aucun moment les deux joueurs aient eu à être connectés ensemble.
- **SC-010** : Dans un duel direct, les deux joueurs voient chaque épreuve apparaître avec moins d'une seconde d'écart, et une coupure de connexion de moins de 30 secondes ne fait pas perdre le duel.
- **SC-011** : Un classement, quel qu'il soit, s'affiche en moins de deux secondes avec 10 000 membres classés.
- **SC-012** : Un changement de pays en cours de saison ne déplace aucun gain déjà acquis : la somme des scores de chaque pays avant le changement est inchangée après.
- **SC-013** : Une saison se clôt et la suivante s'ouvre à leurs dates sans aucune intervention humaine le jour même.
- **SC-014** : Un cinquième module peut être ouvert au jeu en n'apportant que ses épreuves et ses formes de question, sans changer une seule règle commune.
- **SC-015** : Trois mois après l'ouverture, au moins 30 % des membres ayant joué une partie ont joué au moins un défi, et au moins 20 % jouent au moins une fois par semaine.

## Couverture du document client

Le document client liste vingt modules. Cette feature livre le socle et quatre pilotes ; ce tableau dit ce que devient chaque ligne, pour qu'aucune ne soit perdue de vue.

| # | Module | Activités demandées | Sort dans cette feature |
|---|--------|---------------------|-------------------------|
| I | Transversal | Championship, défis quotidiens et hebdomadaires, duels, carte interactive, réputation | **Livré** (histoires 3 à 7 et 9) |
| 1 | Afrolang | Quiz linguistiques, jeux audio | **Pilote** |
| 1 | Afrolang | Défis de traduction, cartographie linguistique | Vague ultérieure : réponse libre et carte des langues |
| 2 | Codimoi | Proverbes, citations, défis culturels, duel de proverbes | **Pilote** (le duel de proverbes est un duel sur le module Codimoi) |
| 2 | Codimoi | Storytelling collaboratif | Vague ultérieure : écriture à plusieurs, pas une épreuve |
| 3 | Afripulse | Découverte géographique, tourisme, gastronomie, personnalités | **Pilote** |
| 10 | FactCheck Africa | Détection de fausses nouvelles | **Pilote** |
| 10 | FactCheck Africa | Vérification collaborative | Existe déjà hors jeu (contributions et modération) |
| 4 | Afroculture | Alliances ethniques, musiques, instruments, costumes, gastronomie | Vague ultérieure : le socle suffit, la matière reste à saisir |
| 5 | Rootstree | Diaspora, généalogie, migrations, grandes dates | Vague ultérieure : le socle suffit, la matière reste à saisir |
| 7 | Diapertise | Quiz métiers, Agenda 2063, défis experts | Vague ultérieure : le socle suffit, la matière reste à saisir |
| 8 | Sabbafrica | ODD, Agenda 2063 | Vague ultérieure : le socle suffit, la matière reste à saisir |
| 8 | Sabbafrica | Simulations de missions | Feature distincte : scénario à embranchements |
| 9 | Afromarket | ZLECAf, commerce intra-africain, défis entrepreneuriaux | Vague ultérieure : le socle suffit, la matière reste à saisir |
| 14 | HumanTech | Mémoire africaine, transmission des savoirs | Vague ultérieure : le socle suffit |
| 15 | NuMeTech | Recherche documentaire, quiz scientifiques | Vague ultérieure : le socle suffit ; le module n'existe pas sous ce nom, à rapprocher de la bibliothèque numérique |
| 16 | Muniversa | Olympiades des connaissances | Vague ultérieure : le socle suffit (une olympiade est une saison thématique) |
| 16 | Muniversa | Certifications gamifiées | Feature distincte : évaluation des formations |
| 18 | VidAfrica | Sous-titrage, traduction | Existe déjà hors jeu ; décryptage culturel en vague ultérieure |
| 19 | Africans Télé | Quiz après émission | Vague ultérieure : le socle suffit, l'épreuve référence un épisode |
| 20 | Africans Radio | Quiz audio | Vague ultérieure : le socle suffit |
| 6 | Africonnect | Jeux de connexions, réseautage | Feature distincte : ce n'est pas une épreuve |
| 11 | IdeaForces | Laboratoire d'idées, hackathons virtuels | Feature distincte |
| 12 | BadGoodHabit | Votes citoyens, classements | Feature distincte : vote, pas épreuve |
| 13 | AfricaLive | Quiz live, interactions événementielles | Feature distincte : jeu à plusieurs en direct pendant un événement |
| 17 | Africantives | Concours d'initiatives | Feature distincte : jury et candidatures |
| 19-20 | Télé, Radio | Débats interactifs, radio participative | Feature distincte |

## Clarifications

### Session d'origine (spec perdue), reportée le 2026-10-01

- **Q1 : Sous quel pays un membre est-il classé au Championship ?** → **Pays d'origine, repli sur la résidence, figé au moment où le score est acquis.** Un Ivoirien de Paris joue pour la Côte d'Ivoire ; le repli évite d'écarter ceux qui n'ont renseigné que leur résidence. Encodé dans FR-053 à FR-055, dans le scénario 5 de l'histoire 5 et dans le cas limite du changement de pays.
- **Q2 : D'où viennent les épreuves ?** → **Mixte : dérivation depuis le contenu publié, revue par un administrateur, puis épreuve jouable.** La saisie directe par un administrateur reste possible. Encodé dans FR-008, FR-009, FR-011, FR-012, FR-075, FR-076 et dans l'histoire 8.
- **Q3 : Le duel se joue-t-il en différé ou en direct ?** → **Les deux.** Le duel différé (48 heures) est l'histoire 4, en P2 ; le duel direct est l'histoire 7, en P3, détachée pour être livrable après sans changer le cycle d'états. Encodé dans FR-039 à FR-050. Les tournois à plus de deux joueurs sont hors périmètre.

## Assumptions

Ces choix ont été faits faute de précision dans le document client, sur la base des usages du genre et des habitudes de la plateforme. Ils sont à confirmer ou à corriger.

- **Le jeu ne rapporte pas de points d'engagement.** La plateforme a recadré son système d'engagement sur trois sources seulement (J'aime reçus, partages reçus, cadeaux reçus), qui déterminent les quatre statuts. Y ajouter le jeu rouvrirait cette décision et permettrait d'acheter un statut à force de quiz. Le jeu produit donc son propre score, de la réputation et des distinctions. **C'est l'hypothèse la plus structurante de la spec, et le client peut la renverser.**
- **Deux formes d'épreuve** dans cette version : choix d'une proposition parmi plusieurs, et vrai ou faux (qui n'en est qu'un cas à deux propositions). Les réponses libres, les traductions et les associations demandent un jugement humain ou une tolérance d'orthographe, et sont renvoyées à plus tard.
- **Valeurs de départ**, toutes réglables : dix épreuves par partie, cinq pour le défi du jour, quinze pour le défi de la semaine, sept par duel ; trente secondes par épreuve ; un, deux ou trois de score selon la difficulté ; trois duels comptés par jour entre deux mêmes membres.
- **Heure de référence unique** pour le jour et la semaine des défis : le temps universel. Une journée par fuseau donnerait à certains membres la série avant les autres.
- **Le duel se propose à un ami.** L'adversaire tiré au hasard demande un appariement et ouvre la porte aux sollicitations non désirées ; il est renvoyé à plus tard.
- **Le score d'un pays est la somme de ses dix meilleurs joueurs.** Une somme de tous les joueurs donnerait la victoire au pays le plus peuplé, une moyenne au pays d'un seul joueur brillant.
- **Pas de reprise des gains** lorsqu'une épreuve est corrigée ou retirée, comme ailleurs sur la plateforme.
- **Pas d'option pour se masquer des classements** dans cette version : le classement ne montre que ce que le profil public montre déjà.
- **La carte réutilise la carte de l'Afrique déjà familière des membres** dans Afripulse et Africonnect, plutôt qu'une troisième représentation.
- **Tout ce qui est périodique se constate à la consultation** (défi du jour, échéance d'un duel, fin de saison), sans traitement déclenché à heure fixe : c'est le mécanisme déjà retenu partout ailleurs sur la plateforme.
- **Le duel direct s'appuiera sur l'un des deux moyens d'échange en temps réel déjà en service**, sans en introduire un troisième. Lequel est une décision du plan.
- **La dérivation se fait à la demande d'un administrateur**, non au fil de l'eau à chaque publication de contenu.
- **Périmètre exclu de cette version** : tournois et jeux à plus de deux joueurs, adversaire tiré au hasard, quiz en direct pendant un événement, votes et concours avec jury, hackathons, écriture collaborative, simulations à embranchements, certifications, récompenses matérielles ou financières, et l'ouverture des seize modules non pilotes.

## Dépendances

- Le compte membre et son profil, en particulier le pays d'origine et le pays de résidence, tous deux facultatifs.
- Le contenu publié et modéré des quatre modules pilotes, et son état de publication à l'instant de la lecture.
- Le lien d'amitié et le blocage entre membres, dont dépend le droit de proposer un duel.
- Le mécanisme de distinctions et le compteur de réputation du système d'engagement, déjà en place ; la réputation n'est aujourd'hui alimentée par aucune règle active.
- Les notifications de la plateforme.
- Le référentiel des 55 pays d'Afrique et la carte déjà utilisée par Afripulse et Africonnect.
- Les habilitations du back-office et le journal d'audit.
