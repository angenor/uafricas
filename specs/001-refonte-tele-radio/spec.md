# Feature Specification: Refonte des pages Télé et Radio Africans

**Feature Branch**: `001-refonte-tele-radio`  
**Created**: 2026-07-19  
**Status**: Draft  
**Input**: User description: "il faut un remaniment/ajustement des pages `uafricas_frontend/app/pages/medias/tele.vue` et `uafricas_frontend/app/pages/medias/radio/africans.vue` / `uafricas_frontend/app/pages/medias/radio/nationales.vue` (ces deux dernieres pages sont identique, elles se disting par le filtre et devrons rester distinct (ne pas les fusion). Voici l'objectif qu'on veut atteindre `documentations/Tele_et_radio_Africans_concept.docx.md`"

## Contexte

Africans expose aujourd'hui trois pages médias : une page Télé et deux pages Radio (« Africans » et « Nationales »). Elles présentent chaînes ou stations sous forme d'une grille de vignettes filtrable, surmontée d'un lecteur unique. Les deux pages Radio sont identiques à leur filtre près.

Le concept « Télé et radio Africans » vise une expérience différente : un contenu vedette qui s'impose à l'ouverture de la page Télé, puis une découverte **chaîne par chaîne au défilement**, et une plateforme **participative** où les acteurs des médias africains proposent, programment et diffusent des contenus au service de l'union et du développement de l'Afrique — chaque publication étant validée par un administrateur avant diffusion.

Les deux pages Radio doivent **rester deux pages distinctes**, différenciées par leur périmètre éditorial : leur fusion est explicitement exclue.

## Clarifications

### Session 2026-07-19

- Q: Comment traiter les droits de diffusion des contenus soumis par les membres et les organes de presse ? → A: Aucune déclaration de droits n'est demandée au contributeur ; la vérification de la licéité de diffusion incombe à l'administrateur au moment de la validation.
- Q: Qui est le « détenteur » d'une chaîne ou d'une station — celui qui gère ses contenus, établit sa grille et reçoit les propositions d'idées ? → A: Co-détention : une chaîne ou une station peut compter plusieurs co-détenteurs, chacun pouvant y programmer des contenus.
- Q: Un contenu déjà validé et diffusé peut-il être modifié par ses co-détenteurs, et cela déclenche-t-il une revalidation ? → A: Les métadonnées (titre, description, image, thème) sont modifiables immédiatement ; seul le remplacement du fichier média replace le contenu en attente de validation.
- Q: Les contenus diffusés doivent-ils être hébergés par la plateforme, pointés par un lien externe, ou les deux ? → A: Les deux, sans restriction d'usage : un média externe peut être mis en vedette et programmé au même titre qu'un fichier hébergé.
- Q: Où vit le lecteur audio sur les pages Radio, et que met-on en tête de page ? → A: Une barre de lecture persistante ancrée en bas de l'écran, visible en permanence ; la tête de page accueille un bandeau d'accroche, sans lecteur.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — La page Télé : un programme vedette plein écran, puis les chaînes en sections (Priority: P1)

Un visiteur ouvre la page Télé. Un programme vidéo, choisi éditorialement comme vedette de **toute la page**, occupe immédiatement la totalité de l'écran — toute la hauteur et toute la largeur — et commence à jouer. Aucune grille, aucun filtre, aucune statistique ne vient le concurrencer à ce stade. C'est en faisant défiler que le visiteur découvre les chaînes, présentées les unes après les autres, chacune dans **sa propre section** portant son identité et **son propre contenu mis en évidence**. Depuis une section, il peut lancer un autre programme de la chaîne sans quitter la page.

**Why this priority**: C'est la demande structurante du concept et le premier contact du visiteur avec la Télé Africans. Sans cette bascule d'une grille de vignettes vers une expérience éditorialisée plein écran, le remaniement n'a pas eu lieu. Livrée seule, cette story transforme déjà la page en vitrine immersive.

**Independent Test**: Peut être testé en désignant un programme comme vedette générale, en ouvrant la page Télé et en vérifiant que ce programme occupe tout l'écran à l'ouverture, puis que le défilement révèle une section par chaîne avec son contenu mis en évidence et ses autres programmes accessibles.

**Acceptance Scenarios**:

1. **Given** un programme vidéo publié et désigné comme vedette générale de la page Télé, **When** le visiteur ouvre la page, **Then** ce programme occupe la totalité de la hauteur et de la largeur de la fenêtre et démarre sa lecture sans action de sa part.
2. **Given** la vedette générale affichée, **When** le visiteur fait défiler vers le bas, **Then** il atteint les sections de chaînes, une section par chaîne, dans un ordre stable d'une visite à l'autre.
3. **Given** une section de chaîne affichée, **When** le visiteur la consulte, **Then** elle présente le nom de la chaîne, son territoire et sa catégorie, le contenu mis en évidence pour cette chaîne, et l'accès à ses autres programmes.
4. **Given** une section de chaîne, **When** le visiteur choisit un autre programme de cette chaîne, **Then** ce programme se lit dans la section sans recharger la page ni faire perdre la position de défilement.
5. **Given** aucun programme n'est désigné comme vedette générale, **When** le visiteur ouvre la page, **Then** le système met en avant à sa place le programme publié le plus récent, toutes chaînes confondues, sans jamais afficher d'écran vide.
6. **Given** la vedette démarre automatiquement, **When** le visiteur arrive sur la page, **Then** le son est coupé par défaut et des commandes visibles — activation du son, pause — sont utilisables à la souris comme au clavier dès l'ouverture.
7. **Given** la vedette occupe tout l'écran, **When** le visiteur cherche la suite du contenu, **Then** un repère visuel lui indique que la page se poursuit vers le bas et permet d'atteindre la première section de chaîne.
8. **Given** un visiteur sur téléphone, **When** il ouvre la page Télé, **Then** il bénéficie de la même mise en avant plein écran que sur ordinateur, adaptée au format de son écran.
9. **Given** une chaîne publiée ne possédant aucun programme publié, **When** la page s'affiche, **Then** cette chaîne n'occupe pas de section vide.

---

### User Story 2 — Les pages Radio Africans et Radio Nationales : les stations en sections, deux pages distinctes (Priority: P1)

Un auditeur ouvre Radio Africans ou Radio Nationales. Chaque page conserve son adresse, son titre et **son périmètre éditorial propre**, mais toutes deux adoptent la même structure de découverte : **une section par station**, chacune portant l'identité de la station et **son contenu mis en évidence**, avec ses autres contenus accessibles dans la même section. L'écoute se poursuit sans interruption pendant que l'auditeur fait défiler la page ou passe d'une section à l'autre.

**Why this priority**: Les deux pages Radio constituent la moitié du périmètre demandé et souffrent du même défaut que la Télé : une grille indifférenciée sans éditorialisation. Surtout, leur différenciation n'est aujourd'hui pas effective — Radio Africans n'applique aucun filtre et affiche donc aussi les stations nationales. La contrainte de non-fusion étant explicite, la distinction doit devenir réelle dès la première livraison.

**Independent Test**: Peut être testé en ouvrant chacune des deux pages et en vérifiant qu'elles présentent des ensembles de stations conformes à leur périmètre respectif, sous forme de sections avec contenu mis en évidence, et que la lecture audio survit au défilement et au changement de section.

**Acceptance Scenarios**:

1. **Given** des stations et contenus publiés par Africans elle-même, **When** l'auditeur ouvre Radio Africans, **Then** il voit une section par station, chacune présentant l'identité de la station et son contenu mis en évidence.
2. **Given** des stations rattachées à des territoires africains, **When** l'auditeur ouvre Radio Nationales, **Then** seules ces stations sont présentées, à l'adresse et sous le titre propres à cette page.
3. **Given** les deux pages livrées, **When** on compare leurs contenus, **Then** aucune station n'apparaît sur les deux : son origine — publication propre de la plateforme ou rattachement à un territoire — tranche seule son rattachement.
4. **Given** les deux pages livrées, **When** on compare leurs adresses, **Then** elles restent deux pages séparées conservant leurs adresses actuelles, sans redirection de l'une vers l'autre, et restent atteignables depuis le hub Radios.
5. **Given** un contenu audio en cours d'écoute, **When** l'auditeur fait défiler la page ou ouvre une autre section, **Then** la lecture se poursuit sans coupure et la barre de lecture ancrée en bas d'écran continue d'afficher le contenu écouté et ses commandes.
6. **Given** une section de station, **When** l'auditeur choisit un autre contenu de cette station, **Then** ce contenu remplace l'écoute en cours sans recharger la page et sans superposition sonore.
7. **Given** une station disposant d'un flux en direct, **When** l'auditeur consulte sa section, **Then** le direct lui est proposé au même titre que les contenus enregistrés de la station.
8. **Given** le périmètre d'une page ne retourne aucune station, **When** la page s'affiche, **Then** un message explicite l'indique et propose une action de repli, sans page blanche.

---

### User Story 3 — Réagir, commenter et partager un contenu (Priority: P2)

Un membre connecté regarde ou écoute un programme. Sous ce programme, il peut exprimer une réaction, laisser un commentaire lisible par les autres, partager le contenu dans l'espace communautés d'Africans avec une légende de son choix, et le relayer vers les réseaux sociaux externes. Les compteurs de réactions et le fil de commentaires sont visibles de tous, y compris des visiteurs non connectés, qui sont invités à se connecter s'ils tentent de participer.

**Why this priority**: L'approche participative est le premier élément d'originalité revendiqué par le concept. Elle transforme une consultation passive en engagement, mais suppose que l'expérience de consultation (P1) existe d'abord.

**Independent Test**: Peut être testé en réagissant, commentant et partageant un programme depuis les pages Télé et Radio, puis en vérifiant que la réaction est comptabilisée une seule fois par membre, que le commentaire apparaît dans le fil, et que le partage apparaît dans l'espace communautés.

**Acceptance Scenarios**:

1. **Given** un membre connecté devant un programme, **When** il exprime une réaction, **Then** le compteur correspondant s'incrémente immédiatement et sa réaction reste visible s'il revient sur la page.
2. **Given** un membre ayant déjà réagi, **When** il exprime la réaction opposée ou retire la sienne, **Then** son ancienne réaction est remplacée ou retirée sans jamais compter deux fois.
3. **Given** un membre connecté, **When** il publie un commentaire sur un programme, **Then** le commentaire apparaît dans le fil du programme avec son auteur et sa date, et reste visible après rechargement.
4. **Given** un membre connecté, **When** il partage un programme dans l'espace communautés avec une légende, **Then** une publication reprenant le programme et sa légende apparaît dans l'espace communautés.
5. **Given** un membre connecté, **When** il relaie un programme vers un réseau social externe, **Then** le lien partagé affiche le titre, la description et l'image du programme dans l'aperçu du réseau social.
6. **Given** un visiteur non connecté, **When** il tente de réagir, commenter ou partager, **Then** il est invité à se connecter et revient sur le programme concerné après connexion.
7. **Given** un programme retiré de l'antenne, **When** on ouvre un partage pointant vers lui, **Then** le contenu n'est plus accessible et un message explicite est présenté.

---

### User Story 4 — Proposer une chaîne ou un contenu en tant que partie prenante, validé par l'administrateur (Priority: P2)

Un organe de presse, un journaliste, un communicateur, un créateur de contenu, un influenceur, un réalisateur ou un producteur souhaite diffuser sur Africans. Il déclare la chaîne ou la station qu'il représente en précisant **son rôle de partie prenante** (choisi dans une liste, avec une option « Autre » suivie d'un champ de précision), puis soumet ses contenus vidéo ou audio en indiquant **le thème phare** auquel ils se rattachent (liste de référence, également avec « Autre » et précision). Rien n'est diffusé tant qu'un administrateur n'a pas validé la soumission ; le contributeur suit l'avancement de sa demande et reçoit le motif en cas de refus.

**Why this priority**: Le concept exige explicitement que « n'importe quel organe de presse puisse diffuser ses contenus » et que « chaque publication soit validée par l'admin ». Aujourd'hui, à l'inverse, tout membre connecté peut publier une chaîne ou un programme immédiatement visible du public, sans aucune validation : cette story ferme cette brèche autant qu'elle ouvre la participation.

**Independent Test**: Peut être testé en soumettant une chaîne et un contenu depuis un compte membre, en vérifiant qu'ils n'apparaissent nulle part publiquement, puis en les validant depuis le back-office et en constatant leur apparition sur la page correspondante.

**Acceptance Scenarios**:

1. **Given** un membre autorisé à contribuer, **When** il déclare une nouvelle chaîne ou station, **Then** un rôle de partie prenante lui est demandé parmi la liste de référence, avec une option « Autre » ouvrant un champ de précision obligatoire.
2. **Given** un membre soumettant un contenu, **When** il remplit le formulaire, **Then** un thème phare lui est demandé parmi la liste de référence, avec une option « Autre » ouvrant un champ de précision obligatoire.
3. **Given** une chaîne ou un contenu soumis, **When** on consulte les pages publiques Télé et Radio, **Then** la soumission n'y apparaît pas tant qu'elle n'est pas validée.
4. **Given** une soumission en attente, **When** un administrateur la valide, **Then** elle devient visible publiquement dans la section de sa chaîne ou station.
5. **Given** une soumission en attente, **When** un administrateur la refuse en indiquant un motif, **Then** le contributeur voit sa soumission refusée accompagnée du motif, et le contenu ne devient jamais public.
6. **Given** un contributeur, **When** il consulte ses soumissions, **Then** il en voit l'état — en attente, validée, refusée — sans ambiguïté.
7. **Given** un contenu déjà diffusé, **When** un administrateur le retire de l'antenne, **Then** il disparaît des pages publiques tout en restant consultable et réactivable dans le back-office.
8. **Given** une soumission de contenu, **When** elle est enregistrée, **Then** son auteur, sa date et l'auteur de la décision de modération sont conservés à des fins de traçabilité.
9. **Given** un contenu validé et diffusé, **When** un co-détenteur en modifie le titre, la description, l'image ou le thème, **Then** la modification est visible publiquement sans nouvelle validation.
10. **Given** un contenu validé et diffusé, **When** un co-détenteur remplace son fichier vidéo ou audio, **Then** le contenu repasse en attente de validation.

---

### User Story 5 — Programmer des contenus qui se diffusent automatiquement (Priority: P3)

L'un des co-détenteurs d'une chaîne ou d'une station dispose de contenus déjà téléversés. Il établit une grille : tel contenu passe tous les jours à telle heure, tel autre chaque semaine à tel jour et telle heure. À l'heure dite, la section de la chaîne ou de la station diffuse automatiquement le contenu prévu, sans intervention humaine. Le visiteur voit ce qui passe « en ce moment » et ce qui suit.

**Why this priority**: La programmation automatique de contenus préchargés est l'élément qui distingue une véritable chaîne d'un simple catalogue. Elle a une valeur forte mais suppose que les contenus, les chaînes et le circuit de validation existent déjà ; elle est donc livrable dans un second temps.

**Independent Test**: Peut être testé en planifiant un contenu à une heure proche, puis en constatant qu'à l'échéance la section de la chaîne diffuse ce contenu et affiche l'élément suivant de la grille.

**Acceptance Scenarios**:

1. **Given** un co-détenteur de chaîne avec des contenus disponibles, **When** il planifie un contenu quotidien à une heure donnée, **Then** la grille de la chaîne affiche ce créneau pour chaque jour.
2. **Given** un co-détenteur de chaîne, **When** il planifie un contenu hebdomadaire pour un jour et une heure donnés, **Then** la grille affiche ce créneau pour ce jour de la semaine uniquement.
3. **Given** une grille active, **When** l'heure d'un créneau est atteinte, **Then** la section de la chaîne ou de la station diffuse le contenu prévu sans intervention humaine.
4. **Given** un visiteur consultant une section, **When** un contenu programmé est en cours, **Then** il voit ce qui passe en ce moment ainsi que le créneau suivant.
5. **Given** deux contenus planifiés sur un créneau qui se chevauche pour une même chaîne, **When** la grille est enregistrée, **Then** le conflit est signalé et l'enregistrement est refusé tant qu'il n'est pas résolu.
6. **Given** un contenu programmé retiré de l'antenne ou supprimé, **When** son créneau arrive, **Then** la diffusion bascule sur le contenu mis en évidence de la chaîne et le créneau est signalé comme invalide à ses co-détenteurs.
7. **Given** un créneau planifié, **When** des visiteurs situés dans des fuseaux horaires différents le consultent, **Then** l'horaire affiché est cohérent et son référentiel horaire est explicite.
8. **Given** aucun créneau en cours, **When** un visiteur ouvre une section, **Then** le contenu mis en évidence de la chaîne est diffusé par défaut.

---

### User Story 6 — S'engager : proposer une idée, demander l'animation d'un programme, trouver un réalisateur ou un producteur (Priority: P3)

Un visiteur ou une partie prenante veut peser sur la programmation. Il propose une idée de contenu pour une chaîne ou un programme donné. Une chaîne ou un individu — journaliste, communicateur, créateur, influenceur, réalisateur, producteur — demande l'acquisition et l'animation d'un programme existant. Un porteur de projet recherche des réalisateurs ou des producteurs pour de nouveaux contenus et les contacte.

**Why this priority**: L'engagement des parties prenantes et des citoyens africains enrichit fortement le dispositif, mais il n'a d'objet qu'une fois les chaînes, les contenus et la communauté en place.

**Independent Test**: Peut être testé en déposant une idée de contenu sur une chaîne, en soumettant une demande d'animation de programme, et en recherchant un réalisateur puis en le contactant — chacune de ces actions devant aboutir à une trace consultable par son destinataire.

**Acceptance Scenarios**:

1. **Given** un visiteur connecté sur une section de chaîne ou un programme, **When** il propose une idée de contenu, **Then** sa proposition est enregistrée, rattachée à la chaîne ou au programme visé, et consultable par les administrateurs et les co-détenteurs concernés.
2. **Given** une chaîne ou un individu identifié comme partie prenante, **When** il demande l'acquisition et l'animation d'un programme, **Then** sa demande est enregistrée avec son rôle et son motif, et son état de traitement lui est visible.
3. **Given** un porteur de projet, **When** il recherche des réalisateurs ou des producteurs, **Then** il obtient une liste de profils déclarés comme tels, filtrable par territoire et par spécialité.
4. **Given** un profil de réalisateur ou de producteur trouvé, **When** le porteur de projet le contacte, **Then** l'échange s'ouvre par les moyens de mise en relation existants de la plateforme.
5. **Given** une demande d'animation traitée, **When** l'administrateur l'accepte ou la refuse, **Then** le demandeur en est informé, avec le motif en cas de refus.

---

### User Story 7 — Signaler un contenu contraire aux règles (Priority: P3)

Tout visiteur connecté peut signaler un contenu qui promeut la violence, le racisme, la discrimination, la mauvaise gouvernance ou la corruption. Les règles de contenu sont énoncées clairement sur les pages. Au-delà d'un seuil de signalements distincts, le contenu est automatiquement retiré de l'antenne publique en attendant l'arbitrage d'un administrateur, qui peut le rétablir ou le supprimer définitivement.

**Why this priority**: L'interdiction de ces contenus est une exigence explicite du concept et une protection nécessaire dès que la contribution s'ouvre. Elle reste en P3 car le circuit de validation préalable (P2) constitue déjà un premier filtre.

**Independent Test**: Peut être testé en signalant un contenu depuis plusieurs comptes distincts jusqu'au seuil, en constatant son retrait automatique des pages publiques, puis son rétablissement depuis le back-office.

**Acceptance Scenarios**:

1. **Given** un visiteur connecté devant un contenu, **When** il le signale en choisissant un motif, **Then** son signalement est enregistré et il en reçoit confirmation.
2. **Given** un visiteur ayant déjà signalé un contenu, **When** il le signale de nouveau, **Then** le compteur de signalements n'augmente pas.
3. **Given** un contenu atteignant le seuil de signalements distincts, **When** le seuil est franchi, **Then** le contenu est retiré des pages publiques sans intervention manuelle et apparaît dans la file de modération.
4. **Given** un contenu retiré automatiquement, **When** un administrateur l'examine et le juge conforme, **Then** il peut le rétablir et son compteur de signalements est réinitialisé.
5. **Given** les pages Télé et Radio, **When** un visiteur les consulte, **Then** les règles de contenu interdit y sont accessibles en clair.

---

### Edge Cases

- **Vedette générale indisponible** : le programme désigné comme vedette est retiré de l'antenne, supprimé ou son fichier est illisible entre deux visites → repli sur le programme publié le plus récent, sans écran vide ni lecteur en erreur.
- **Aucune chaîne ni station publiée** : la page ne doit ni rester blanche ni afficher un lecteur vide, mais expliquer la situation.
- **Chaîne ou station sans contenu mis en évidence désigné** : le système en choisit un selon une règle déterministe plutôt que de laisser la section muette.
- **Contenu sans fichier média exploitable** : un programme dont le fichier vidéo ou audio est absent ou illisible ne doit jamais être proposé publiquement.
- **Média externe défaillant** : un contenu hébergé chez un tiers dont le lecteur refuse la lecture automatique, devient indisponible ou est retiré à la source doit basculer sur un repli explicite, sans lecteur en erreur ni section muette — y compris lorsqu'il est vedette ou programmé.
- **Volumétrie élevée** : une page comportant plusieurs dizaines de chaînes ou de stations doit rester fluide au défilement et ne pas précharger simultanément tous les médias.
- **Connexion lente ou forfait de données limité** : la lecture automatique plein écran ne doit pas consommer inutilement de données avant que le visiteur n'ait manifesté son intérêt.
- **Accessibilité** : un média qui démarre seul doit être arrêtable ; la navigation entre sections et le pilotage de la lecture doivent être possibles au clavier et compréhensibles par un lecteur d'écran.
- **Double lecture simultanée** : lancer un contenu dans une section alors qu'un autre est déjà en cours ne doit jamais produire deux flux audio superposés.
- **Station sans flux ni contenu** : une station dépourvue de tout contenu diffusable ne doit pas apparaître comme écoutable.
- **Périmètre vide sur une page Radio** : Radio Nationales sans aucune station de son périmètre doit afficher un message explicite, pas une page vide.
- **Station relevant des deux périmètres** : une station qui pourrait figurer sur les deux pages Radio doit être arbitrée par une règle explicite, jamais laissée au hasard.
- **Contenu signalé pendant sa diffusion** : un contenu retiré alors qu'il est en cours de lecture chez un visiteur doit cesser d'être servi aux nouvelles ouvertures.
- **Créneau de programmation orphelin** : un créneau pointant vers un contenu supprimé ne doit pas interrompre la diffusion de la chaîne.
- **Contribution avec « Autre »** : un rôle ou un thème « Autre » sans précision saisie doit être refusé à l'enregistrement.
- **Contributeur dont le compte est désactivé** : ses contenus déjà validés et ses soumissions en attente doivent avoir un sort défini et explicite.
- **Droits contestés après publication** : un contenu dont l'autorisation de diffusion est mise en cause après coup doit pouvoir être retiré de l'antenne sans délai, aucune décharge n'ayant été recueillie en amont.
- **Co-détenteurs en concurrence** : deux co-détenteurs modifiant la grille d'une même chaîne au même moment ne doivent ni écraser silencieusement le travail de l'autre, ni produire de créneaux qui se chevauchent.
- **Dernier co-détenteur retiré** : une chaîne qui n'a plus aucun co-détenteur doit rester diffusée et administrable, sa grille restant modifiable par un administrateur.

## Requirements *(mandatory)*

### Functional Requirements

#### Page Télé — mise en avant générale et sections (US1)

- **FR-001**: Le système DOIT permettre de désigner **un** programme vidéo comme vedette générale de la page Télé, indépendamment des mises en avant propres à chaque chaîne.
- **FR-002**: La page Télé DOIT afficher la vedette générale à l'ouverture sur la totalité de la hauteur et de la largeur de la fenêtre, avant tout autre contenu.
- **FR-003**: La vedette générale DOIT démarrer sa lecture sans action du visiteur, son coupé par défaut, et offrir dès l'ouverture des commandes visibles d'activation du son et de pause, utilisables à la souris et au clavier.
- **FR-004**: La page Télé DOIT révéler les chaînes au défilement, à raison d'**une section par chaîne**, dans un ordre stable entre deux visites.
- **FR-005**: Chaque section de chaîne DOIT présenter l'identité de la chaîne — nom, description, territoire, catégorie — et **un contenu mis en évidence propre à cette chaîne**, selon la forme définie en FR-022.
- **FR-006**: Chaque section de chaîne DOIT donner accès aux autres programmes publiés de cette chaîne et permettre d'en lancer un sans recharger la page ni perdre la position de défilement.
- **FR-007**: En l'absence de vedette générale désignée, le système DOIT mettre en avant le programme publié le plus récent, toutes chaînes confondues.
- **FR-008**: Le système NE DOIT PAS créer de section pour une chaîne dépourvue de programme publié.
- **FR-009**: La page Télé DOIT indiquer visuellement que le contenu se poursuit sous la vedette et permettre d'atteindre la première section de chaîne.
- **FR-010**: Le système DOIT retirer de la page Télé le contenu vedette codé en dur actuellement en place, la mise en avant devenant pilotable depuis le back-office.
- **FR-011**: La mise en avant plein écran DOIT être servie aussi bien sur téléphone que sur ordinateur, adaptée au format de l'écran ; le comportement actuel qui prive les visiteurs mobiles de tout contenu vedette DOIT cesser.

#### Pages Radio — sections et distinction des deux pages (US2)

- **FR-012**: Les pages Radio Africans et Radio Nationales DOIVENT rester **deux pages distinctes**, conserver leurs adresses actuelles et leurs titres propres, rester atteignables depuis le hub Radios ; aucune fusion ni redirection de l'une vers l'autre n'est autorisée.
- **FR-013**: Chaque page Radio DOIT présenter les stations de son périmètre éditorial à raison d'**une section par station**, chaque section portant l'identité de la station et **son contenu mis en évidence**. Elle DOIT être coiffée d'un bandeau d'accroche portant son identité éditoriale propre, sans lecteur en tête de page.
- **FR-014**: La répartition entre les deux pages Radio DOIT suivre l'**origine de la publication** : **Radio Africans** ne présente que les stations et contenus publiés par Africans elle-même — production propre de la plateforme, relevant d'une décision éditoriale de ses créateurs ; **Radio Nationales** ne présente que les stations et contenus rattachés à un territoire africain. Une station relève d'une seule des deux pages et NE DOIT PAS apparaître sur les deux.
- **FR-015**: Chaque section de station DOIT donner accès aux autres contenus publiés de cette station et permettre d'en lancer un sans recharger la page.
- **FR-016**: Une station disposant d'un flux en direct DOIT rester écoutable en direct, celui-ci étant proposé dans sa section au même titre que ses contenus enregistrés.
- **FR-017**: La lecture audio DOIT se poursuivre sans coupure lorsque l'auditeur fait défiler la page ou change de section. Une **barre de lecture persistante ancrée en bas de l'écran** DOIT rester visible en permanence dès qu'une écoute est en cours et présenter le contenu écouté, sa station et les commandes de lecture, de pause et de volume.
- **FR-018**: Le système NE DOIT JAMAIS produire deux flux audio simultanés : lancer un contenu DOIT interrompre celui en cours.
- **FR-019**: Lorsqu'une page Radio ne retourne aucune station, le système DOIT afficher un message explicite et une action de repli.
- **FR-020**: Le système DOIT exposer publiquement les contenus des stations radio, à parité avec ce qui est déjà exposé pour les programmes de télévision.
- **FR-021**: Les pages Télé et Radio, qui sont des pages publiques, NE DOIVENT PAS utiliser les composants d'interface réservés au back-office.
- **FR-022**: Sur les pages Télé comme Radio, chaque section DOIT prendre la forme d'un **bloc empilé de hauteur naturelle** : le contenu mis en évidence présenté en bandeau avec son titre, sa description et son action de lecture, suivi des autres contenus de la chaîne ou de la station en **une rangée horizontale défilante**. Les sections NE DOIVENT PAS occuper chacune un écran entier ; seule la vedette générale de la page Télé est plein écran.

#### Interactions communautaires (US3)

- **FR-023**: Un membre connecté DOIT pouvoir exprimer une réaction sur un contenu télé ou radio, une seule réaction étant retenue par membre et par contenu, modifiable et annulable.
- **FR-024**: Un membre connecté DOIT pouvoir commenter un contenu ; les commentaires DOIVENT être visibles de tous avec leur auteur et leur date.
- **FR-025**: Un membre connecté DOIT pouvoir partager un contenu dans l'espace communautés d'Africans avec une légende facultative ; le partage DOIT y apparaître.
- **FR-026**: Tout contenu télé ou radio DOIT pouvoir être relayé vers les réseaux sociaux externes, avec un aperçu comportant son titre, sa description et son image.
- **FR-027**: Les compteurs de réactions et les commentaires DOIVENT être visibles des visiteurs non connectés ; toute tentative de participation DOIT déclencher une invitation à se connecter, avec retour au contenu concerné après connexion.
- **FR-028**: Un partage pointant vers un contenu retiré de l'antenne DOIT présenter un message explicite plutôt que le contenu.

#### Contribution des parties prenantes et validation (US4)

- **FR-029**: Le système DOIT proposer, à la déclaration d'une chaîne ou d'une station, un **rôle de partie prenante** choisi dans la liste de référence, assorti d'une option « Autre » ouvrant un champ de précision obligatoire.
- **FR-030**: Le système DOIT proposer, à la soumission d'un contenu, un **thème phare** choisi dans la liste de référence, assorti d'une option « Autre » ouvrant un champ de précision obligatoire.
- **FR-031**: Toute chaîne, station ou contenu soumis DOIT rester invisible du public tant qu'un administrateur ne l'a pas validé.
- **FR-032**: Le système NE DOIT PAS permettre la publication publique directe d'une chaîne, d'une station ou d'un contenu sans passage par la validation administrative. Après validation, les co-détenteurs DOIVENT pouvoir modifier librement les métadonnées d'un contenu — titre, description, image, thème phare — sans nouvelle validation ; en revanche, le remplacement de son fichier vidéo ou audio DOIT replacer le contenu en attente de validation.
- **FR-033**: Un administrateur DOIT pouvoir valider ou refuser une soumission en motivant son refus, et retirer de l'antenne un contenu déjà diffusé sans le supprimer. Aucune déclaration de droits n'étant demandée au contributeur, l'examen de la licéité de diffusion — droits d'auteur, autorisation de rediffusion — incombe à l'administrateur au moment de cette validation.
- **FR-034**: Un contributeur DOIT pouvoir consulter l'état de ses soumissions — en attente, validée, refusée — et le motif d'un refus.
- **FR-035**: Le système DOIT conserver, pour chaque soumission, son auteur, sa date et l'auteur de la décision de modération.
- **FR-036**: Tout membre connecté DOIT pouvoir soumettre une chaîne, une station ou un contenu sans accréditation préalable, la maîtrise reposant intégralement sur la validation administrative exigée en FR-031 et FR-032. Ces soumissions portent sur des chaînes, stations et contenus rattachés à un territoire : la publication sous la bannière Radio Africans reste une décision éditoriale de la plateforme (FR-014).

#### Programmation automatique (US5)

- **FR-037**: Une chaîne ou une station DOIT pouvoir compter **plusieurs co-détenteurs**, chacun pouvant planifier la diffusion de contenus déjà téléversés selon une récurrence quotidienne ou hebdomadaire, à un horaire donné. Le membre dont la déclaration a été validée en est le premier co-détenteur ; un administrateur DOIT pouvoir ajouter ou retirer un co-détenteur.
- **FR-038**: À l'échéance d'un créneau, le système DOIT diffuser automatiquement le contenu prévu dans la section de la chaîne ou de la station, sans intervention humaine.
- **FR-039**: Chaque section DOIT indiquer le contenu en cours de diffusion et le créneau suivant lorsqu'une grille est active.
- **FR-040**: Le système DOIT refuser l'enregistrement de deux créneaux se chevauchant pour une même chaîne ou station et signaler le conflit.
- **FR-041**: Lorsqu'un créneau pointe vers un contenu devenu indisponible, le système DOIT basculer sur le contenu mis en évidence de la chaîne et signaler le créneau invalide à ses co-détenteurs.
- **FR-042**: Les horaires de programmation DOIVENT reposer sur un référentiel horaire explicite et être affichés de façon cohérente quel que soit le fuseau du visiteur.
- **FR-043**: En l'absence de créneau en cours, la section DOIT diffuser le contenu mis en évidence de la chaîne ou de la station.

#### Engagement des parties prenantes et des citoyens (US6)

- **FR-044**: Un visiteur connecté DOIT pouvoir proposer une idée de contenu rattachée à une chaîne, une station ou un programme précis.
- **FR-045**: Une chaîne ou un individu identifié comme partie prenante DOIT pouvoir demander l'acquisition et l'animation d'un programme, en précisant son rôle et son motif. L'acceptation d'une telle demande DOIT ajouter le demandeur aux co-détenteurs de la chaîne ou de la station concernée.
- **FR-046**: Le système DOIT permettre de rechercher des profils déclarés réalisateurs ou producteurs, avec filtrage par territoire et par spécialité, et de les contacter par les moyens de mise en relation existants.
- **FR-047**: Les propositions d'idées et les demandes d'animation DOIVENT être consultables par les administrateurs et par les co-détenteurs concernés, et leur état de traitement DOIT être visible du demandeur.

#### Modération et contenus interdits (US7)

- **FR-048**: Les pages Télé et Radio DOIVENT énoncer en clair les règles de contenu : sont interdits les contenus violents ou promouvant la violence, le racisme, la discrimination, la mauvaise gouvernance ou la corruption.
- **FR-049**: Un visiteur connecté DOIT pouvoir signaler un contenu en choisissant un motif ; un même membre NE DOIT PAS pouvoir faire croître le compteur en signalant plusieurs fois le même contenu.
- **FR-050**: Au franchissement d'un seuil de signalements distincts, le système DOIT retirer automatiquement le contenu des pages publiques et le porter dans la file de modération.
- **FR-051**: Un administrateur DOIT pouvoir rétablir un contenu retiré automatiquement, ce qui réinitialise son compteur de signalements, ou le supprimer définitivement.

#### Exigences transversales

- **FR-052**: Toutes les pages concernées DOIVENT rester utilisables sur mobile, sans défilement horizontal, la lecture automatique plein écran ne devant pas consommer de données inutilement avant manifestation d'intérêt du visiteur.
- **FR-053**: Les commandes de lecture, la navigation entre sections et les actions communautaires DOIVENT être accessibles au clavier et compréhensibles par un lecteur d'écran.
- **FR-054**: Une page comportant plusieurs dizaines de chaînes ou de stations NE DOIT PAS précharger simultanément l'ensemble des médias.
- **FR-055**: Toute décision de modération, de validation ou de retrait, ainsi que toute modification de grille de programmation ou de la liste des co-détenteurs, DOIT être tracée avec son auteur, sa date et l'objet concerné.
- **FR-056**: Un contenu DOIT pouvoir être indifféremment téléversé sur la plateforme ou désigné par un lien vers un hébergement externe, **sans restriction d'usage** : les deux formes peuvent être mises en vedette et programmées en grille. Lorsque le média provient d'un hébergement externe, le système DOIT appliquer le comportement attendu — lecture automatique, son coupé, démarrage à l'échéance — dans la limite de ce que permet le lecteur tiers, et présenter un repli explicite lorsque celui-ci refuse la lecture ou devient indisponible.

### Key Entities

- **Chaîne (télé)** : entité éditoriale regroupant des programmes vidéo. Porte un nom, une description, un territoire, une catégorie, une langue, un rôle de partie prenante déclaré, un état de publication et un contenu mis en évidence.
- **Station (radio)** : équivalent radio de la chaîne, rattachée à un périmètre éditorial qui détermine la page sur laquelle elle apparaît. Porte également un contenu mis en évidence, et éventuellement un flux en direct.
- **Programme (contenu)** : unité diffusable, vidéo pour la télé ou audio pour la radio. Porte un titre, une description, une image, un média, un thème phare, un rattachement à une chaîne ou une station, l'identité de son contributeur et un état de publication — en attente, validé, refusé, retiré. Le remplacement de son média le ramène en attente.
- **Mise en avant** : désignation éditoriale d'un contenu. Deux portées coexistent — **la vedette générale**, unique pour toute la page Télé, et **la mise en évidence par chaîne ou station**, unique au sein de chacune.
- **Rôle de partie prenante** : qualité déclarée du porteur d'une chaîne ou d'une station. Valeurs de référence : chaîne de télé africaine, radio africaine, journaliste, communicateur, créateur de contenu, influenceur, réalisateur, producteur, **Autre** (avec précision obligatoire). Le visiteur constitue une partie prenante du dispositif sans être porteur de chaîne : il suit les contenus et propose des sujets.
- **Thème phare (émission)** : sujet éditorial d'un contenu, choisi dans une liste de référence assortie d'une option **Autre** avec précision. Valeurs de référence : Retour des cerveaux ; Histoire de l'Afrique ; Valeurs africaines et développement ; Journal de l'Afrique ; Haro sur les hommes de l'Afrique ; L'intellectuel africain et développement ; Afrique et technologies ; Savoirs faire d'Afrique ; Cuisine de chez nous ; Politique africaine ; De la thèse à l'action locale ; La voix du terrain en Afrique ; Débats africains ; Mystères africains ; Droit africain ; Environnement d'Afrique ; Regards de la jeunesse africaine ; Femmes d'Afrique ; Gouvernance d'Afrique aux défis ; Infrastructures d'Afrique ; Santé et développement ; Numérique et développement africain ; Traditions d'Afrique ; Mondialisation et coopération africaine ; Commerce africain et unité africaine ; Développement durable ; Le monde de demain et mondialisation ; Immigration et l'avenir de l'Afrique ; Sports d'Afrique ; Rendez-vous des hauts et des bas ; Éducation — Les carrés de l'instruction en Afrique ; Éducation — Les carrés de l'école de la vie ; Éducation — Les carrés de l'éducation à l'africaine ; L'Afrique que nous voulons ; Messages aux gouvernants ; Cinéma africain ; Séries d'Afrique ; Documentaires africains ; Safari d'Afrique ; Futurs génies d'Afrique ; Innovations simples chez nous ; Complexes d'Afrique ; Afrique Société ; Afrique Solidarité.
- **Co-détention** : rattachement d'un membre à une chaîne ou une station, lui ouvrant la gestion de ses contenus, de sa grille et la réception des propositions d'idées. Une chaîne ou une station en compte un ou plusieurs. Le premier co-détenteur est le membre dont la déclaration a été validée ; les suivants entrent par acceptation d'une demande d'animation ou par décision d'un administrateur.
- **Créneau de programmation** : planification de la diffusion d'un contenu sur une chaîne ou une station, avec une récurrence quotidienne ou hebdomadaire, un jour le cas échéant, un horaire, une durée et l'identité du co-détenteur qui l'a posée.
- **Interaction communautaire** : réaction, commentaire ou partage émis par un membre sur un contenu. Une seule réaction retenue par membre et par contenu.
- **Proposition d'engagement** : idée de contenu déposée par un visiteur, ou demande d'acquisition et d'animation d'un programme déposée par une partie prenante. Porte son auteur, sa cible, son motif et son état de traitement.
- **Signalement** : alerte d'un membre sur un contenu contraire aux règles. Un signalement au plus par membre et par contenu ; leur nombre déclenche un retrait automatique au-delà d'un seuil.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: À l'ouverture de la page Télé, le contenu vedette occupe 100 % de la surface visible et, s'il est hébergé par la plateforme, commence à jouer en moins de 3 secondes sur une connexion mobile standard.
- **SC-002**: Un visiteur atteint la première section de chaîne en un seul geste de défilement, sans avoir à chercher où continuer.
- **SC-003**: Depuis n'importe quelle section, lancer un autre contenu de la chaîne ou de la station demande au plus deux actions.
- **SC-004**: 8 visiteurs sur 10 mis en situation identifient sans aide la chaîne ou la station à laquelle appartient le contenu qu'ils regardent ou écoutent.
- **SC-005**: Les deux pages Radio restent joignables à leurs adresses actuelles et aucune station n'apparaît sur une page dont elle ne relève pas, vérifiable en comparant leurs contenus respectifs.
- **SC-006**: L'écoute d'un contenu radio survit à 100 % des défilements et changements de section, sans coupure ni double flux audio, la barre de lecture restant visible et pilotable tout du long.
- **SC-007**: 100 % des chaînes, stations et contenus visibles publiquement ont fait l'objet d'une validation administrative explicite ; aucun contenu ne peut apparaître publiquement sans elle.
- **SC-008**: Un membre partage un contenu dans l'espace communautés en moins de 30 secondes et au plus 3 actions.
- **SC-009**: Un contenu atteignant le seuil de signalements distincts disparaît des pages publiques en moins d'une minute, sans intervention humaine.
- **SC-010**: Un contenu planifié hébergé par la plateforme démarre à l'horaire prévu avec un écart inférieur à une minute ; pour un média externe, le déclenchement est émis à l'heure prévue, l'écart constaté dépendant alors du lecteur tiers.
- **SC-011**: Une page comportant 50 chaînes ou stations reste défilable de façon fluide et n'engage le chargement d'un média qu'au moment où le visiteur s'y intéresse.
- **SC-012**: L'ensemble des commandes de lecture et des actions communautaires est atteignable et actionnable au clavier seul.
- **SC-013**: Aucune page publique concernée ne présente de défilement horizontal, du plus petit écran mobile courant au grand écran de bureau.

## Hypothèses

- **H-001**: Le MVP correspond aux deux stories P1 — la page Télé et les deux pages Radio remaniées. Chaque story ultérieure est livrable indépendamment et apporte de la valeur seule.
- **H-002**: Les pages Télé et Radio restent consultables sans connexion ; seules les actions de participation — réagir, commenter, partager, proposer, signaler — exigent un compte.
- **H-003**: L'« espace communautés » désigne l'espace de publications déjà existant sur la plateforme ; les partages de contenus télé et radio s'y ajoutent aux sources déjà présentes plutôt que de constituer un espace nouveau.
- **H-004**: Les mécanismes de réaction, de partage, de signalement avec seuil et de contribution modérée déjà éprouvés ailleurs sur la plateforme servent de référence de comportement, pour la cohérence de l'expérience.
- **H-005**: La déclaration d'une chaîne ou d'une station reste un acte encadré : elle n'est jamais visible publiquement avant validation, quel que soit l'arbitrage retenu sur le périmètre des contributeurs autorisés.
- **H-006**: Le contenu vedette actuellement codé en dur sur la page Télé est un provisoire assumé, destiné à être remplacé par la mise en avant pilotable ; sa disparition n'est pas une régression.
- **H-007**: Le seuil de signalements déclenchant un retrait automatique s'aligne sur celui déjà retenu pour les contributions comparables de la plateforme.
- **H-008**: Les libellés visibles emploient « territoire » là où le modèle de données parle de pays, conformément à l'usage établi sur le reste du site.
- **H-009**: Les contenus diffusés peuvent indifféremment être téléversés sur la plateforme ou pointer vers un hébergement externe, sans restriction d'usage (FR-056) ; aucune capacité de diffusion en direct produite par Africans n'est introduite par cette évolution.
- **H-010**: Les pages hub Médias et Radios ne sont pas remaniées par cette évolution ; seuls leurs liens vers les trois pages concernées doivent rester valides.
- **H-011**: La page Télé n'étant pas dédoublée, la règle d'origine qui répartit les stations entre les deux pages Radio ne s'y applique pas : chaînes propres à Africans et chaînes de territoires y cohabitent dans la même page.
- **H-012**: Aucune décharge de droits n'est recueillie auprès des contributeurs. Le risque de rediffusion non autorisée est assumé et reporté sur l'examen administratif préalable à toute publication (FR-033), complété par le retrait de l'antenne et le signalement en cas de contestation ultérieure.

## Dépendances

- **D-001**: L'origine de publication qui répartit les stations entre les deux pages Radio — production propre de la plateforme ou rattachement à un territoire — doit être portée de façon fiable par les données et vérifiable de bout en bout. Elle n'est aujourd'hui pas effective : la page Radio Africans n'applique aucun filtre. Les stations déjà enregistrées devront être qualifiées.
- **D-002**: Les contenus des stations radio doivent être exposés publiquement, ce qui n'est pas le cas aujourd'hui alors que leur équivalent télévision l'est.
- **D-003**: L'espace de publications communautaires doit pouvoir accueillir une nouvelle source de partage correspondant aux contenus télé et radio.
- **D-004**: Les profils de réalisateurs et de producteurs recherchés en US6 supposent que cette qualité soit déclarée et exploitable sur les profils membres.
- **D-005**: La diffusion automatique programmée suppose un référentiel horaire commun côté serveur, aucune planification récurrente de ce type n'existant aujourd'hui sur la plateforme.
- **D-006**: Une maquette de formulaire « Ajouter un programme » existe déjà sans être reliée à quoi que ce soit ; son modèle de champs peut servir de point de départ à la soumission de contenu de l'US4.

## Hors périmètre

- La fusion des pages Radio Africans et Radio Nationales, explicitement exclue.
- Le remaniement des pages hub Médias et Radios.
- La diffusion en direct produite par Africans elle-même : seuls les contenus enregistrés et les flux tiers déjà référencés sont concernés.
- La monétisation, la publicité et le partage de revenus avec les chaînes partenaires.
- Les applications mobiles natives et la diffusion vers des téléviseurs connectés.
- La modération automatisée par analyse de contenu : la validation reste humaine.
- La reprise ou la migration de contenus depuis des plateformes tierces.
