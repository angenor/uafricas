# Feature Specification : Jeux variés et concours communautaires

**Feature Branch** : `014-jeux-concours`

**Created** : 2026-10-04

**Status** : Draft

**Input** : retour du client après la recette de la feature 013 (Activités ludiques) : les jeux sont trop uniformes, on ne fait que répondre à des questions à choix multiple. Il demande d'autres formes de jeu, dont des « battles » de photos et des défis vidéo.

## Contexte

La feature 013 a livré un socle ludique complet (parties, défis, duels, Championship, carte, réputation) sur **un seul type d'épreuve** : une question, quatre propositions, une bonne réponse connue de la plateforme. Quelle que soit la richesse du vivier, le geste du joueur reste le même : lire, choisir, recommencer.

Les demandes du client relèvent en réalité de **deux familles qu'il ne faut pas confondre** :

- **Famille A — d'autres façons de répondre.** Désigner un pays sur la carte, remettre des éléments dans l'ordre, associer des paires, reconnaître une photo ou un son. Il y a toujours une bonne réponse, connue de la plateforme : ces épreuves se jouent partout où se joue une épreuve aujourd'hui (partie, défi, duel, Championship), sans changer une seule règle de score.
- **Famille B — des concours où la communauté juge.** Une bataille de photos n'a pas de bonne réponse : des membres créent, d'autres membres départagent. C'est un **cycle nouveau** (participation, modération, vote, résultats), qui ne ressemble à rien de ce que fait le socle. Cette feature en livre le premier format, la **bataille de photos** ; les défis vidéo, défis de traduction, récits à plusieurs mains, concours d'idées et votes citoyens réutiliseront le même cycle dans des vagues ultérieures.

Un point de vocabulaire, en plus de celui de la 013 : un **concours** est un appel à créations sur un thème, borné dans le temps ; une **participation** est la création déposée par un membre ; une **confrontation** est la présentation de deux participations à un votant, qui en choisit une.

## User Scenarios & Testing *(mandatory)*

### User Story 1 : Répondre autrement que par un choix (Priority : P1)

Un membre lance une partie Afripulse. La première épreuve lui demande de désigner le Tchad sur la carte de l'Afrique : il clique sur le pays. La suivante lui demande de classer quatre pays du plus peuplé au moins peuplé : il les fait glisser dans l'ordre. La troisième lui demande d'associer quatre capitales à leur pays. Chaque fois, il valide, voit la bonne réponse, et lit l'explication.

**Why this priority** : c'est ce qui change le plus visiblement l'expérience de jeu, au moindre coût : tout le reste du socle (tirage, temps imparti, score, défis, duels, Championship) fonctionne déjà et n'a pas à être touché.

**Independent Test** : avec un vivier contenant au moins une épreuve de chaque nouveau type, lancer des parties jusqu'à rencontrer chacun d'eux, y répondre juste puis faux, et vérifier la correction et le score.

**Acceptance Scenarios** :

1. **Given** une épreuve « carte » affichée, **When** le membre désigne un pays et valide, **Then** il voit si c'était le bon, et le bon pays est mis en évidence sur la carte.
2. **Given** une épreuve « carte » affichée, **When** le membre n'utilise pas de souris (clavier, lecteur d'écran), **Then** il peut désigner le pays par une liste, sans désavantage de temps.
3. **Given** une épreuve « ordre » affichée, **When** le membre valide un ordre, **Then** l'épreuve est juste si et seulement si l'ordre est entièrement correct, et la correction montre l'ordre attendu avec la valeur qui le justifie (population, date…).
4. **Given** une épreuve « paires » affichée, **When** le membre valide ses associations, **Then** l'épreuve est juste si et seulement si toutes les paires sont correctes, et la correction montre les bonnes paires.
5. **Given** n'importe quel nouveau type d'épreuve, **When** le temps imparti s'écoule avant la validation, **Then** l'épreuve est comptée sans réponse, comme une question à choix multiple.
6. **Given** une épreuve de n'importe quel nouveau type dans un défi ou un duel, **When** elle est jouée, **Then** elle compte exactement comme une question à choix multiple : mêmes gains, même délai de réponse et même règle de non-rejeu.

---

### User Story 2 : Reconnaître une photo ou un son (Priority : P1)

Un membre voit apparaître la photo d'un plat, sans légende : de quel pays vient-il ? Plus loin, un extrait sonore de dix secondes : quelle langue entend-il ? L'épreuve ne démarre qu'une fois la photo ou le son réellement chargé.

**Why this priority** : le client cite expressément les jeux audio (Afrolang, Radio) et la découverte visuelle (Afripulse, Afroculture). Le socle accepte déjà une image ou un son en énoncé, mais seulement par une adresse saisie à la main : rien ne permet de déposer le fichier, et aucune épreuve à photo n'est tirée du contenu publié.

**Independent Test** : déposer une photo et un extrait sonore depuis le back-office, publier les épreuves, les jouer ; dériver des épreuves à photo depuis les sites touristiques et les recettes, les passer en revue, les jouer.

**Acceptance Scenarios** :

1. **Given** un administrateur qui saisit une épreuve, **When** il dépose une photo ou un extrait sonore, **Then** le fichier est contrôlé (format, poids, durée pour le son) et attaché à l'épreuve, sans qu'il ait à fournir d'adresse.
2. **Given** une épreuve à photo ou à son, **When** elle est présentée, **Then** le temps imparti ne commence qu'une fois le média chargé, avec la marge de chargement déjà prévue par le socle.
3. **Given** un site touristique ou une recette publiés avec une photo, **When** l'administrateur lance la dérivation, **Then** des épreuves candidates « De quel pays vient ce plat ? » ou « Où se trouve ce site ? » sont proposées, la photo servant d'énoncé.
4. **Given** une photo de contenu publié qui affiche son propre nom (légende incrustée, panneau), **When** l'administrateur la voit en revue, **Then** il peut la rejeter avec un motif, et elle n'est jamais reproposée.

---

### User Story 3 : Constituer le vivier des nouveaux types (Priority : P1)

Un administrateur saisit une épreuve « ordre » sur les dates d'indépendance, une épreuve « paires » entre mots swahili et leur traduction. Il lance aussi la dérivation : la plateforme lui propose des épreuves « classer ces pays par superficie », « associer ces capitales à leur pays », « désigner sur la carte le pays dont voici la devise ». Il les passe en revue comme les autres.

**Why this priority** : sans vivier, les nouveaux types ne se jouent pas. La dérivation est ce qui a donné du volume à la 013 ; la même logique doit nourrir les nouveaux types.

**Independent Test** : saisir une épreuve de chaque nouveau type, lancer la dérivation, accepter des candidates, vérifier qu'elles deviennent jouables et qu'aucune candidate ne l'est.

**Acceptance Scenarios** :

1. **Given** le formulaire de saisie, **When** l'administrateur choisit un type de réponse, **Then** le formulaire s'adapte (pays cible, éléments à ordonner avec le critère, paires) et refuse une épreuve incohérente (moins de 3 éléments, doublon, deux pays identiques…).
2. **Given** la dérivation, **When** elle propose une épreuve « paires » ou « ordre », **Then** aucune valeur ne peut convenir à deux éléments à la fois (deux pays partageant une monnaie ne figurent jamais dans la même épreuve de paires sur la monnaie).
3. **Given** la dérivation, **When** elle propose une épreuve « carte », **Then** la réponse désigne un seul pays possible (une devise ou un peuple partagés par plusieurs pays ne donnent pas d'épreuve « carte »).
4. **Given** une candidate de n'importe quel nouveau type, **When** elle n'est pas acceptée, **Then** elle n'est jamais servie.

---

### User Story 4 : Participer à une bataille de photos (Priority : P2)

L'administrateur a ouvert le concours « Mon plat du dimanche ». Un membre le découvre sur l'espace Activités, lit le thème et le règlement, et dépose la photo de son plat avec une courte légende. Il est prévenu que sa photo sera relue avant d'être publiée. Le lendemain, une notification lui apprend qu'elle est acceptée ; il la voit dans le concours, au milieu des autres.

**Why this priority** : c'est la demande la plus concrète du client et la première brique du moteur de concours. Elle vient après la famille A parce qu'elle exige un cycle entièrement nouveau.

**Independent Test** : ouvrir un concours, y déposer une photo avec un compte de membre, la faire accepter par un administrateur, vérifier qu'elle est publiée et que le membre est notifié.

**Acceptance Scenarios** :

1. **Given** un concours en appel à participation, **When** un membre connecté dépose une photo conforme et une légende, **Then** sa participation est enregistrée « en attente de modération » et il en est informé.
2. **Given** un membre qui a déjà une participation dans ce concours, **When** il en dépose une autre, **Then** c'est refusé, sauf si le concours autorise plusieurs participations par membre.
3. **Given** une participation en attente, **When** le membre la remplace ou la retire, **Then** c'est possible jusqu'à la modération ; après publication, il peut la retirer, mais plus la remplacer.
4. **Given** une photo non conforme (format, poids, dimensions), **When** le membre la dépose, **Then** elle est refusée avec un message qui dit quoi corriger.
5. **Given** un concours dont l'appel à participation est clos, **When** un membre tente de déposer, **Then** c'est refusé, et la date de clôture lui est rappelée.
6. **Given** un concours rattaché à un module (Afroculture, Afripulse…), **When** un membre ouvre ce module, **Then** il y trouve le concours en cours.

---

### User Story 5 : Voter à l'aveugle (Priority : P2)

Le vote est ouvert. Un membre entre dans le concours : deux photos s'affichent côte à côte, sans nom d'auteur, sans compteur. Il choisit celle qu'il préfère ; deux autres apparaissent aussitôt. Il enchaîne une vingtaine de confrontations en deux minutes, puis s'arrête quand il veut. Il ne voit jamais sa propre photo, et ne voit aucun résultat avant la clôture.

**Why this priority** : c'est ce qui rend le concours juste. Un vote aux J'aime récompense la popularité d'un compte plus que la qualité d'une photo, se truque, avantage la photo déposée la première, et chaque J'aime reçu crédite déjà des points d'engagement : il ferait du concours une machine à points.

**Independent Test** : avec un concours en vote et au moins quatre participations publiées, voter avec plusieurs comptes ; vérifier qu'aucun ne voit sa propre photo, qu'aucun décompte n'est visible, et que chaque photo est présentée un nombre comparable de fois.

**Acceptance Scenarios** :

1. **Given** un concours en phase de vote, **When** un membre connecté y entre, **Then** il voit deux participations publiées, sans auteur ni décompte, et en choisit une.
2. **Given** un votant qui a une participation dans le concours, **When** des confrontations lui sont présentées, **Then** sa propre participation n'y figure jamais.
3. **Given** une paire déjà départagée par un votant, **When** il continue à voter, **Then** cette paire ne lui est jamais représentée.
4. **Given** un votant qui a vu toutes les paires possibles, ou atteint la limite de votes du concours, **When** il continue, **Then** il est remercié et invité à revenir après la clôture.
5. **Given** la phase de vote en cours, **When** n'importe qui (votant, participant, visiteur) consulte le concours, **Then** aucun décompte, aucun classement provisoire n'est visible.
6. **Given** un visiteur non connecté, **When** il ouvre un concours en vote, **Then** il voit le thème et un aperçu des participations, et l'entrée du vote le mène à la connexion.

---

### User Story 6 : Programmer et modérer un concours (Priority : P2)

Un administrateur crée le concours « Tenue traditionnelle » : thème, règlement, module de rattachement, dates d'appel à participation et de vote, nombre de participations par membre, récompenses. Pendant l'appel, il relit les photos déposées : il accepte, ou il rejette avec un motif que le membre reçoit. Il suit le nombre de participations et de votes.

**Why this priority** : sans programmation ni modération, aucun concours n'existe. La modération avant publication est indispensable : ce sont des photos de membres, qui peuvent montrer des personnes, des mineurs, ou n'avoir rien à voir avec le thème.

**Independent Test** : créer un concours, déposer des photos avec des comptes de membres, en accepter et en rejeter, vérifier les notifications et l'état public du concours à chaque date.

**Acceptance Scenarios** :

1. **Given** le formulaire de création, **When** l'administrateur enregistre un concours, **Then** les dates sont cohérentes (l'appel précède le vote, le vote a une durée minimale) ; sinon c'est refusé avec un message clair.
2. **Given** une participation en attente, **When** l'administrateur l'accepte, **Then** elle est publiée et le membre est notifié ; **When** il la rejette, **Then** un motif est obligatoire, le membre le reçoit et peut redéposer tant que l'appel est ouvert.
3. **Given** un concours dont la date d'ouverture du vote est passée, **When** quiconque le consulte, **Then** il est en phase de vote sans qu'aucune action humaine ait été nécessaire ce jour-là.
4. **Given** un concours qui a moins de participations publiées que le minimum requis à l'ouverture du vote, **When** cette date passe, **Then** le concours est annulé, les participants sont prévenus et gardent la récompense de participation.
5. **Given** un concours pas encore ouvert, **When** l'administrateur le modifie ou le supprime, **Then** c'est possible ; une fois l'appel ouvert, seules la description et les dates encore à venir restent modifiables.
6. **Given** n'importe quelle décision de modération ou modification de concours, **When** elle est enregistrée, **Then** elle figure au journal d'audit.

---

### User Story 7 : Découvrir les résultats et être récompensé (Priority : P2)

Le vote se clôt. Le premier membre qui ouvre le concours voit les résultats : le podium, puis toutes les photos classées, avec leur auteur désormais révélé. Les lauréats reçoivent une notification, du score de jeu, de la réputation et une distinction ; chaque participant publié reçoit une récompense de participation. Le concours reste consultable comme galerie.

**Why this priority** : c'est la raison de participer. Sans récompense lisible, un concours ne revient pas.

**Independent Test** : clore un concours après des votes, vérifier le classement, les gains de chaque participant, les distinctions des lauréats, et qu'aucun gain n'est versé deux fois.

**Acceptance Scenarios** :

1. **Given** un concours dont la date de clôture est passée, **When** il est consulté pour la première fois, **Then** les résultats sont établis et figés, sans intervention humaine.
2. **Given** les résultats, **When** ils sont affichés, **Then** chaque participation porte son rang et sa proportion de confrontations gagnées, et les auteurs sont révélés.
3. **Given** les résultats établis, **When** les récompenses sont versées, **Then** chaque participant publié reçoit sa récompense de participation, et les trois premiers une récompense supérieure et une distinction, une seule fois chacun, même si les résultats sont relus mille fois.
4. **Given** les récompenses d'un concours, **When** elles sont versées, **Then** elles s'ajoutent au score de jeu et comptent pour le Championship comme toute victoire de jeu, et ne créditent aucun point d'engagement.
5. **Given** deux participations à égalité, **When** le classement est établi, **Then** le jury les départage s'il est prévu ; sinon elles sont classées ex aequo et reçoivent la même récompense.

---

### User Story 8 : Départager le podium par un jury (Priority : P3)

Pour un concours important, l'administrateur prévoit un jury. À la clôture du vote, les dix premières photos du vote communautaire lui sont soumises ; le jury établit le podium, qui s'impose au classement communautaire pour les trois premières places.

**Why this priority** : utile pour des concours à enjeu (concours d'idées des vagues suivantes, partenaires), mais pas nécessaire au premier concours.

**Independent Test** : créer un concours avec jury, le mener jusqu'à la clôture du vote, faire établir le podium par le jury, vérifier le classement final et les récompenses.

**Acceptance Scenarios** :

1. **Given** un concours avec jury dont le vote est clos, **When** il est consulté, **Then** il est « en délibération », et aucun résultat n'est publié avant la décision du jury.
2. **Given** la délibération, **When** le jury fixe le podium parmi les finalistes, **Then** les résultats sont publiés, le podium du jury en tête, le reste dans l'ordre du vote.
3. **Given** un jury qui ne délibère pas dans le délai prévu, **When** ce délai passe, **Then** le classement communautaire s'applique tel quel.

---

### User Story 9 : Protéger le vote et les photos (Priority : P3)

Un membre crée cinq comptes la veille d'un concours pour voter pour sa propre photo ; un autre clique au hasard le plus vite possible. Une photo publiée se révèle choquante et est signalée par plusieurs membres. La plateforme écarte les votes suspects sans bruit, suspend la photo signalée, et l'administrateur voit tout cela dans un tableau dédié.

**Why this priority** : les protections de base (un compte, une voix par paire, pas d'auto-vote) sont dans l'histoire 5. Celle-ci couvre la fraude organisée et le contenu inapproprié publié malgré la modération.

**Independent Test** : voter avec un compte créé après l'ouverture du vote, enchaîner des votes en moins d'une seconde, signaler une photo au-delà du seuil ; vérifier que ces votes ne comptent pas, que la photo sort du vote, et que l'administrateur voit les signaux.

**Acceptance Scenarios** :

1. **Given** un compte créé après l'ouverture du vote, ou dont l'adresse n'est pas vérifiée, **When** il vote, **Then** il peut voter, mais ses voix ne comptent pas, et cette règle est publiée dans le règlement.
2. **Given** un vote exprimé trop vite après la présentation de la paire, **When** il est enregistré, **Then** il ne compte pas.
3. **Given** un compte qui vote anormalement (rythme, volume, toujours pour la même photo), **When** l'administrateur consulte le concours, **Then** ce compte lui est signalé, et il peut en écarter les voix ; la décision est journalisée.
4. **Given** une participation publiée, **When** elle dépasse le seuil de signalements, **Then** elle est suspendue et retirée du vote aussitôt, et les confrontations qui l'incluaient ne comptent plus.
5. **Given** un compte suspendu, **When** les résultats sont établis, **Then** ni ses voix ni sa participation ne comptent.

### Edge Cases

- **Photo retirée par son auteur pendant le vote** : elle sort du vote, les confrontations qui l'incluaient ne comptent plus, et l'auteur perd la récompense de participation.
- **Trop peu de votes** : une participation présentée moins d'un nombre minimal de fois à la clôture est classée après celles qui l'ont atteint, sans rang au podium. C'est annoncé dans le règlement.
- **Participant qui vote** : il vote normalement sur les autres photos ; sa propre photo n'apparaît jamais devant lui.
- **Deux comptes du même foyer** : ils votent normalement ; seule la fraude caractérisée (histoire 9) est écartée.
- **Concours sans aucun vote** : il se clôt sans podium ; les participants publiés reçoivent la récompense de participation.
- **Changement de pays d'un participant** : la récompense compte pour le pays de rattachement au moment où elle est versée, comme tout gain de jeu.
- **Épreuve « carte » sur un micro-État** (Gambie, Cap-Vert, Seychelles) : le pays reste désignable par la liste, et la carte permet d'agrandir la zone.
- **Épreuve « ordre » avec deux valeurs égales** : elle n'est jamais générée, et refusée à la saisie.
- **Média d'épreuve introuvable au moment du tirage** : l'épreuve n'est pas servie ; si elle est déjà affichée et ne se charge pas, elle ne compte pas contre le membre (règle de la 013).
- **Duel direct sur une épreuve « ordre » ou « paires »** : les deux joueurs disposent du même temps majoré ; la manche se résout comme une question à choix multiple.

## Requirements *(mandatory)*

### Functional Requirements

**Types de réponse (famille A)**

- **FR-001** : Une épreuve porte exactement un type de réponse parmi : choix multiple (existant), carte, ordre, paires.
- **FR-002** : Une épreuve « carte » demande de désigner un pays d'Afrique ; sa bonne réponse est un seul pays.
- **FR-003** : Une épreuve « ordre » présente de 3 à 6 éléments, un critère explicite (du plus ancien au plus récent, du plus peuplé au moins peuplé…) et un ordre attendu unique.
- **FR-004** : Une épreuve « paires » présente de 3 à 5 paires ; chaque élément de gauche a exactement un correspondant à droite.
- **FR-005** : Les éléments d'une épreuve « ordre » et les deux colonnes d'une épreuve « paires » sont présentés dans un ordre aléatoire, jamais dans l'ordre de la bonne réponse.
- **FR-006** : Une réponse « ordre » ou « paires » est juste si et seulement si elle est entièrement correcte. Il n'y a pas de réponse partiellement juste.
- **FR-007** : La correction de chaque type montre la bonne réponse complète (pays mis en évidence, ordre attendu avec ses valeurs, paires correctes) et l'explication.
- **FR-008** : La bonne réponse d'une épreuve, quel que soit son type, n'est jamais transmise au membre avant qu'il ait répondu ou que le temps soit écoulé.
- **FR-009** : Le temps imparti peut différer selon le type de réponse : l'ordre et les paires disposent d'un temps majoré, réglable par l'administrateur.
- **FR-010** : Une épreuve « carte » peut toujours être résolue sans pointeur, par une liste de pays, sans désavantage de temps.
- **FR-011** : Les nouveaux types se jouent dans tous les cadres existants (partie libre, défi du jour, défi de la semaine, duel différé, duel direct, partie par pays) avec les mêmes règles de gain, de non-rejeu et de temps serveur.
- **FR-012** : Le tirage varié de la 013 traite les types de réponse comme des formes : une partie fait alterner les types quand le vivier le permet.

**Épreuves à média**

- **FR-013** : Un administrateur peut déposer une photo ou un extrait sonore comme énoncé d'une épreuve, sans fournir d'adresse.
- **FR-014** : Une photo d'épreuve est contrôlée (formats courants, poids et dimensions bornés) ; un extrait sonore aussi (formats courants, poids borné, durée de 30 secondes au plus).
- **FR-015** : Le temps imparti d'une épreuve à média ne commence qu'une fois le média présenté, avec la marge de chargement du socle.
- **FR-016** : Une épreuve à média dont le fichier n'existe plus n'est jamais servie.

**Vivier et dérivation**

- **FR-017** : Le formulaire de saisie s'adapte au type de réponse et refuse toute épreuve incohérente : élément en double, valeurs égales dans un « ordre », correspondance ambiguë dans des « paires », pays hors d'Afrique pour une « carte ».
- **FR-018** : La dérivation propose des épreuves des nouveaux types à partir du contenu publié, au minimum : classer des pays par population et par superficie ; associer capitales et pays ; désigner sur la carte le pays d'une capitale, d'un site, d'un plat ; reconnaître le pays d'un plat ou d'un site par sa photo.
- **FR-019** : Une épreuve dérivée de type « paires » ne réunit jamais deux éléments partageant la valeur à associer ; une épreuve « ordre » ne réunit jamais deux valeurs égales ou trop proches pour être départagées sans ambiguïté ; une épreuve « carte » n'est dérivée que d'une valeur propre à un seul pays.
- **FR-020** : Une épreuve d'un nouveau type suit le cycle de revue de la 013 : candidate, puis jouable après acceptation, à revoir si sa source change, jamais reproposée après rejet.

**Concours : programmation**

- **FR-021** : Un administrateur crée un concours : format (bataille de photos pour cette feature), titre, thème, règlement, module de rattachement facultatif, dates d'ouverture de l'appel, d'ouverture du vote, de clôture du vote, nombre maximal de participations par membre (1 par défaut), minimum de participations publiées pour ouvrir le vote (4 par défaut), jury oui/non.
- **FR-022** : Les dates d'un concours sont ordonnées (ouverture de l'appel < ouverture du vote < clôture du vote) et le vote dure au moins 24 heures.
- **FR-023** : La phase d'un concours (à venir, appel à participation, vote, délibération, résultats, annulé) est déterminée à la lecture d'après ses dates et son état, sans aucune tâche de fond.
- **FR-024** : Avant l'ouverture de l'appel, tout le concours est modifiable et supprimable ; après, seuls la description et les dates encore à venir le sont.
- **FR-025** : Le cycle d'un concours est indépendant du type de création déposée, afin d'accueillir plus tard d'autres formats (vidéo, traduction, texte, idée) sans être refait.

**Concours : participation et modération**

- **FR-026** : Un membre connecté, non suspendu, dépose une participation (photo et légende courte) pendant l'appel à participation.
- **FR-027** : Une photo de participation est contrôlée et normalisée selon les règles des photos de membres de la plateforme.
- **FR-028** : Un membre ne dépasse pas le nombre maximal de participations du concours.
- **FR-029** : Toute participation naît « en attente » et n'est visible que de son auteur et des administrateurs jusqu'à sa modération.
- **FR-030** : Un administrateur accepte une participation, qui est alors publiée, ou la rejette avec un motif obligatoire ; l'auteur est notifié dans les deux cas.
- **FR-031** : Une participation rejetée peut être remplacée par une nouvelle tant que l'appel est ouvert.
- **FR-032** : L'auteur peut remplacer sa participation tant qu'elle est en attente, et la retirer à tout moment ; une participation retirée pendant le vote en sort, et ses confrontations ne comptent plus.
- **FR-033** : Si le minimum de participations publiées n'est pas atteint à l'ouverture du vote, le concours est annulé et ses participants sont notifiés.

**Concours : vote**

- **FR-034** : Pendant le vote, un membre connecté se voit présenter des confrontations de deux participations publiées, sans nom d'auteur ni décompte, et en désigne une.
- **FR-035** : Un votant ne voit jamais sa propre participation.
- **FR-036** : Un votant ne départage jamais deux fois la même paire.
- **FR-037** : Les confrontations sont choisies pour que chaque participation soit présentée un nombre comparable de fois, en privilégiant celles qui l'ont été le moins.
- **FR-038** : Le nombre de votes d'un membre dans un concours peut être plafonné par l'administrateur.
- **FR-039** : Aucun décompte, aucun classement provisoire, aucun ordre révélateur n'est visible par quiconque, administrateurs exceptés, avant la publication des résultats.
- **FR-040** : Une voix est ignorée si elle est exprimée moins d'une seconde après la présentation de la paire, ou par un compte créé après l'ouverture du vote, à l'adresse non vérifiée ou suspendu.
- **FR-041** : La plateforme signale à l'administrateur les comptes au comportement de vote anormal (rythme, volume, préférence systématique pour une même participation) ; l'administrateur peut en écarter les voix, avec une trace au journal d'audit.

**Concours : résultats et récompenses**

- **FR-042** : À la première lecture après la clôture du vote (ou après la délibération du jury), les résultats sont établis et figés.
- **FR-043** : Le classement est établi d'après la proportion de confrontations gagnées par chaque participation, parmi les voix comptées ; une participation présentée moins d'un nombre minimal de fois est classée après les autres, sans accès au podium.
- **FR-044** : À égalité, le jury départage s'il est prévu ; sinon les participations sont ex aequo.
- **FR-045** : Les résultats révèlent les auteurs et affichent, pour chaque participation, son rang et sa proportion de confrontations gagnées.
- **FR-046** : Chaque participation publiée et non retirée rapporte à son auteur une récompense de participation ; les trois premières rapportent une récompense de podium croissante avec le rang. Les montants sont réglables.
- **FR-047** : Les récompenses de concours sont versées en score de jeu (comptant pour le Championship au pays de rattachement du moment) et en réputation ; elles ne créditent aucun point d'engagement.
- **FR-048** : Chaque récompense est versée une seule fois, quel que soit le nombre de lectures ou de tentatives.
- **FR-049** : Les lauréats reçoivent une distinction et une notification ; les participants, une notification de résultats.
- **FR-050** : Un concours terminé reste consultable comme galerie de ses participations publiées.

**Jury**

- **FR-051** : Un concours avec jury entre en délibération à la clôture du vote ; les finalistes (les 10 premiers du vote, par défaut) sont soumis au jury.
- **FR-052** : Le jury, composé d'administrateurs habilités, fixe le podium parmi les finalistes ; le reste du classement suit le vote.
- **FR-053** : Sans décision du jury dans le délai prévu (7 jours par défaut), le classement communautaire s'applique.

**Signalement et protection**

- **FR-054** : Tout membre connecté peut signaler une participation publiée, une fois, avec un motif.
- **FR-055** : Au-delà du seuil de signalements de la plateforme, la participation est suspendue, retirée du vote et de la galerie, et ses confrontations ne comptent plus ; seul un administrateur la rétablit.
- **FR-056** : Les participations d'un compte suspendu sortent du vote et des résultats tant que dure la suspension.

**Visibilité, navigation, administration**

- **FR-057** : Les concours en cours apparaissent sur l'espace Activités et sur la page de leur module de rattachement ; les concours passés restent consultables.
- **FR-058** : Un visiteur non connecté voit le thème, le règlement, les participations publiées (pendant le vote, sans auteur) et les résultats ; déposer et voter exigent la connexion.
- **FR-059** : L'espace « Mes activités » du membre liste ses participations, leur état de modération et leurs résultats.
- **FR-060** : Le back-office offre : la liste des concours par phase, la file de modération des participations, le suivi des votes (volumes, comptes signalés), la délibération du jury, le réglage des récompenses.
- **FR-061** : Toute action d'administration sur un concours, une participation ou une voix est inscrite au journal d'audit.

### Key Entities

- **Épreuve** (existante, étendue) : gagne un type de réponse (choix multiple, carte, ordre, paires) et la donnée propre à ce type (pays attendu ; éléments, critère et ordre attendu ; paires), et peut porter un média déposé.
- **Réponse** (existante, étendue) : enregistre la réponse du membre dans la forme de son type (pays désigné, ordre proposé, paires proposées).
- **Concours** : format, titre, thème, règlement, module de rattachement, dates d'appel, de vote et de clôture, réglages (participations par membre, minimum pour voter, plafond de votes, jury, récompenses), état (actif, annulé, résultats établis).
- **Participation** : concours, auteur, création déposée (photo et légende pour cette feature), état (en attente, publiée, rejetée avec motif, retirée, suspendue), dates.
- **Confrontation** : concours, votant, les deux participations présentées, moment de présentation, participation choisie, moment du vote, voix comptée ou non (et pourquoi).
- **Résultat de concours** : participation, rang, proportion de confrontations gagnées, nombre de présentations, place fixée par le jury le cas échéant ; figé à l'établissement.
- **Décision de jury** : concours, podium retenu, administrateurs ayant délibéré, date.
- **Signalement de participation** : participation, membre, motif, date ; unique par membre et par participation.
- **Gain de jeu** (existant) : reçoit les récompenses de concours, avec la même garantie de versement unique.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001** : Dans une partie de dix épreuves d'un module dont le vivier contient les quatre types de réponse, au moins trois types différents apparaissent dans 90 % des parties.
- **SC-002** : Sur un jeu de contrôle, aucune bonne réponse d'une épreuve carte, ordre ou paires n'est observable avant la réponse : 100 % d'absence.
- **SC-003** : Un membre répond à une épreuve « ordre » de cinq éléments ou « paires » de quatre paires en moins de 30 secondes sur téléphone comme sur ordinateur.
- **SC-004** : Une épreuve « carte » est entièrement jouable au clavier seul.
- **SC-005** : À la livraison, chacun des quatre modules pilotes compte au moins 30 épreuves jouables d'un type autre que le choix multiple.
- **SC-006** : Un membre dépose une participation en moins de deux minutes depuis la page du concours.
- **SC-007** : Un votant enchaîne 20 confrontations en moins de deux minutes.
- **SC-008** : Dans un concours de 200 participations et 500 votants actifs, chaque participation publiée est présentée au moins 30 fois avant la clôture.
- **SC-009** : Sur un jeu de contrôle, aucun membre ne voit jamais sa propre participation en confrontation, et aucune paire ne lui est présentée deux fois : 100 %.
- **SC-010** : Avant la publication des résultats, aucun décompte ni classement n'est observable par un membre ou un visiteur : 100 % d'absence.
- **SC-011** : Les résultats d'un concours sont établis à leur date sans intervention humaine le jour même, et s'affichent en moins de trois secondes à la première consultation.
- **SC-012** : Relire mille fois les résultats d'un concours ne verse chaque récompense qu'une fois.
- **SC-013** : Les points d'engagement et le statut d'un membre sont strictement identiques avant et après sa participation, ses votes et ses récompenses de concours.
- **SC-014** : Un administrateur modère une participation (accepter ou rejeter avec motif) en moins de 20 secondes en moyenne.
- **SC-015** : Trois mois après l'ouverture, au moins 15 % des membres actifs sur l'espace Activités ont voté dans au moins un concours.

## Clarifications

### Session 2026-10-04

Décisions du porteur du projet, prises sur recommandation après analyse de la plateforme. Chacune est une **hypothèse renversable par le client** ; la renverser change les exigences citées, pas le reste de la spec.

- **Q : Faut-il livrer d'abord les concours, demandés explicitement, ou les nouveaux types d'épreuve ?** → R : les nouveaux types d'épreuve d'abord (famille A, P1). Ils changent visiblement le jeu à faible coût, sans rien changer au socle ; les concours exigent un cycle entièrement nouveau.
- **Q : Quel format de concours en premier ?** → R : la bataille de photos. Les défis vidéo, défis de traduction, récits à plusieurs mains, concours d'idées et votes citoyens viendront par vagues, sur le même cycle (FR-025).
- **H1 — Qui désigne le gagnant ?** → R : un **vote communautaire à l'aveugle par confrontations de paires** (FR-034 à FR-043), et non le nombre de J'aime. Le J'aime récompense la popularité d'un compte plus que la qualité d'une photo, se truque (comptes multiples, échanges entre amis), avantage la photo déposée la première, et crédite déjà des points d'engagement à son destinataire. Un jury pour fixer le podium est une option par concours (FR-051 à FR-053).
- **H2 — Que rapporte un concours ?** → R : du score de jeu et de la réputation, comme les épreuves de la 013, et une distinction pour les lauréats ; une récompense modeste pour participer, plus forte pour le podium (FR-046, FR-047). Aucun point d'engagement.
- **H3 — Modérer avant ou après publication ?** → R : **avant**, pour toute participation (FR-029, FR-030), plus le signalement avec suspension automatique après publication (FR-054, FR-055).
- **H4 — Durée des vidéos (pour les défis vidéo à venir, hors périmètre ici)** → R : 60 secondes au plus. Noté pour la vague suivante ; aucune exigence de cette feature n'en dépend.

## Assumptions

- La feature 013 (schéma de jeu, socle, revue des épreuves, tirage varié) est livrée ou fusionnée avant celle-ci, et ses règles de score, de défi, de duel et de Championship ne sont pas modifiées.
- Le seuil de signalements qui suspend une participation est celui des autres contenus communautaires de la plateforme (10).
- Les photos de participation suivent les règles déjà appliquées aux photos de membres (formats courants, poids et dimensions bornés, normalisation).
- Les participations de concours **ne portent pas de J'aime** dans cette feature : un J'aime crédite des points d'engagement à son destinataire, ce qui contredirait H1 et H2. Le partage d'une galerie de résultats reste possible.
- Voter ne rapporte ni score de jeu ni points ; au plus un peu de réputation, plafonnée par concours, pour ne pas récompenser le clic au hasard.
- Les dates des concours s'expriment dans le fuseau de référence de la plateforme, comme les défis de la 013.
- Les montants de récompense par défaut se règlent avec les règles du jeu existantes ; leur valeur initiale relève de la planification.
- L'ordre des dates d'indépendance n'est pas un champ publié aujourd'hui : les épreuves « ordre » sur ce thème sont saisies, pas dérivées.

## Dépendances

- **Feature 013 — Activités ludiques** : épreuves, parties, revue, dérivation, tirage, gains, Championship, réputation, distinctions, notifications du jeu.
- **Carte de l'Afrique** : composant déjà utilisé par le Championship, réutilisé pour les épreuves « carte ».
- **Validation des photos de membres** : règles déjà en service pour les contributions et les annonces.
- **Notifications, journal d'audit, signalements** : mécanismes existants de la plateforme.
- **Comptes** : vérification de l'adresse et date de création, pour l'admissibilité des voix (FR-040).
