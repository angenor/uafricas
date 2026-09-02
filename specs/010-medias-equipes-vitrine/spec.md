# Feature Specification: Médias : équipes éditoriales et recentrage des vitrines Télé & Radio

**Feature Branch**: `010-medias-equipes-vitrine`

**Created**: 2026-08-10

**Status**: Draft

**Input**: User description: "nouvelles corrections majeures de la page télé et radio. Tout ce qui est valable pour Télé est valable pour Radio. Section « Nos télés Africaines » : titre chaîne, une partie de description si c'est long, équipe de direction et de gestion (directeur, producteur, concepteur), rendre la fonction dynamique lors de la saisie ; programmes (image de couverture, description coupée avec des pointillés si c'est long). On ne veut pas que les vidéos soient affichées, juste chaîne (nom, description, équipe) puis les programmes (image de couverture, nom, petite description). Page de détail d'une chaîne : nom, description tronquée avec bouton « voir plus », équipe tronquée avec « voir plus » (nom, prénom, fonction, territoire, contact), programmes (périodicité, pas périodique, journalier, hebdomadaire, mensuel etc. , nom, description, équipe propre au programme, liste des vidéos), pas d'image de couverture cette fois. Page de détail d'un programme : périodicité, nom, image de couverture affichée, description, équipe (nom, prénom, fonction, territoire, contact), liste des vidéos."

## Contexte

Les espaces Télé (`/medias/tele`) et Radio (`/medias/radio/africans`, `/medias/radio/nationales`) exposent aujourd'hui, dans chaque section de chaîne ou de station, un lecteur vidéo/audio, l'épisode mis en avant et une rangée de vignettes d'épisodes par programme. Le commanditaire constate que cette vitrine noie l'information éditoriale : le visiteur voit des vidéos avant de savoir **qui** produit la chaîne et **quels programmes** elle propose.

Cette feature recentre les trois niveaux de lecture :

| Niveau | Ce que le visiteur doit y trouver |
|--------|-----------------------------------|
| Section de la vitrine (« Nos télés africaines » / « Nos radios africaines ») | La chaîne (nom, description, équipe) puis ses programmes (couverture, nom, courte description). **Aucune vidéo.** |
| Page de détail d'une chaîne / station | La chaîne en entier, son équipe complète, et ses programmes détaillés (périodicité, description, équipe propre, vidéos). |
| Page de détail d'un programme | Le programme en entier : périodicité, couverture, description, équipe propre, vidéos. |

Elle introduit surtout une notion absente du modèle : l'**équipe éditoriale nommée**. Aujourd'hui une chaîne n'a aucune information d'équipe, et un programme ne dispose que de deux champs de texte libre (« info animateur », « info producteur ») qui ne permettent ni de nommer plusieurs personnes, ni de préciser leur fonction, leur territoire ou leur contact.

**Périmètre de parité** : toute exigence énoncée pour la Télé (chaîne, programme télé, épisode vidéo) s'applique à l'identique à la Radio (station, programme radio, épisode audio). Cette parité n'est pas répétée exigence par exigence ; elle est portée par FR-060.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - La vitrine annonce des chaînes et des programmes, plus des vidéos (Priority: P1)

Un visiteur arrive sur l'espace Télé et fait défiler les sections. Chaque section lui présente une chaîne : son nom, le début de sa description, son équipe de direction et de gestion. Sous la chaîne, il découvre les programmes de celle-ci sous forme de cartes : image de couverture, nom du programme, courte description tronquée par des points de suspension quand elle dépasse. Aucun lecteur, aucune vignette d'épisode, aucune vidéo lisible n'apparaît dans la section : pour voir ou écouter, le visiteur ouvre la chaîne ou le programme.

**Why this priority**: C'est la correction visible qui motive la demande. Elle est livrable seule, sans nouvelle donnée à saisir (l'équipe s'affiche dès qu'elle existe, et se masque tant qu'elle est vide), et transforme immédiatement la lecture de la vitrine.

**Independent Test**: Ouvrir `/medias/tele` sans être connecté et vérifier qu'aucune section ne contient d'élément lisible, que chaque section nomme sa chaîne, en montre un extrait de description, et liste ses programmes avec couverture, nom et description tronquée. Idem sur les deux espaces Radio.

**Acceptance Scenarios**:

1. **Given** une chaîne publiée avec 3 programmes dont l'un compte 12 épisodes publiés, **When** le visiteur atteint la section de cette chaîne, **Then** il voit 3 cartes de programme et aucune vignette d'épisode ni lecteur.
2. **Given** une chaîne dont la description compte 900 caractères, **When** la section s'affiche, **Then** seule une amorce est visible, terminée par des points de suspension, et le nom de la chaîne mène à sa page de détail.
3. **Given** un programme dont la description compte 400 caractères, **When** sa carte s'affiche dans la section, **Then** la description est coupée par des points de suspension et la carte mène à la page de détail du programme.
4. **Given** un programme sans aucun épisode publié, **When** la section s'affiche, **Then** le programme reste visible dans la liste (il annonce une offre éditoriale, pas un catalogue de fichiers).
5. **Given** une chaîne sans équipe renseignée, **When** la section s'affiche, **Then** aucun bloc d'équipe vide n'est affiché et la mise en page reste cohérente.
6. **Given** une chaîne dont la grille annonce une diffusion en cours, **When** la section s'affiche, **Then** le bandeau « en cours de diffusion / à suivre » reste visible, en texte, sans lecteur associé.
7. **Given** une chaîne portant 40 programmes, **When** la section s'affiche, **Then** aucun programme n'est masqué en silence : soit ils sont tous listés, soit la section annonce le total et mène au reste sur la page de la chaîne.

---

### User Story 2 - Déclarer l'équipe d'une chaîne et l'équipe d'un programme (Priority: P1)

Le détenteur d'une chaîne (ou un administrateur) déclare l'équipe de direction et de gestion de sa chaîne : pour chaque personne, un nom, un prénom, une fonction, un territoire et un contact. La fonction n'est pas figée : au moment de la saisie, le formulaire propose les fonctions déjà employées sur la plateforme (directeur, producteur, concepteur, …) et accepte n'importe quelle fonction nouvelle, qui devient à son tour proposée. Il ajoute autant de personnes que nécessaire, les réordonne, en retire. Quand la personne est inscrite sur UAfricas, il peut rattacher sa fiche à son compte pour que son nom mène à son profil public, un simple confort d'affichage, jamais une condition de saisie ni un droit accordé. Il déclare de la même manière l'équipe propre à chacun de ses programmes, qui peut différer de celle de la chaîne.

**Why this priority**: Sans cette saisie, les blocs « équipe » de la vitrine et des pages de détail resteront toujours vides. US1 et US2 sont donc les deux moitiés d'un même MVP, et elles restent testables séparément.

**Independent Test**: Se connecter comme détenteur d'une chaîne, ajouter trois membres d'équipe dont un avec une fonction inédite, enregistrer, recharger : les trois membres reviennent dans l'ordre déclaré et la fonction inédite est ensuite proposée à la saisie suivante.

**Acceptance Scenarios**:

1. **Given** un détenteur sur la fiche de sa chaîne, **When** il ajoute une personne avec nom, prénom, fonction, territoire et contact, **Then** la personne est enregistrée et apparaît immédiatement dans l'équipe affichée publiquement.
2. **Given** le champ « fonction » vierge, **When** l'utilisateur commence à saisir, **Then** les fonctions déjà déclarées ailleurs lui sont proposées, et il peut valider une fonction absente de la liste.
3. **Given** une équipe de 5 personnes, **When** le détenteur en retire une et déplace une autre en tête, **Then** l'affichage public reflète le nouvel ordre sans la personne retirée.
4. **Given** un programme d'une chaîne qui possède déjà une équipe, **When** le détenteur déclare l'équipe du programme, **Then** les deux équipes coexistent sans se recopier ni s'écraser.
5. **Given** un visiteur non détenteur, **When** il consulte la chaîne, **Then** il ne dispose d'aucun moyen de modifier l'équipe.
6. **Given** une personne saisie sans territoire ni contact, **When** l'équipe s'affiche, **Then** la fiche de cette personne n'affiche pas de libellé vide.
7. **Given** un membre d'équipe rattaché à un compte UAfricas existant, **When** un visiteur consulte l'équipe, **Then** le nom de ce membre mène à son profil public, tandis que les membres non rattachés s'affichent en texte simple.
8. **Given** une équipe dont aucun membre n'est inscrit sur la plateforme, **When** le détenteur l'enregistre, **Then** l'enregistrement aboutit sans qu'aucun rattachement de compte soit exigé.

---

### User Story 3 - Page de détail d'une chaîne restructurée (Priority: P2)

Le visiteur ouvre une chaîne. Il lit son nom, puis le début de sa description ; un bouton « voir plus » déplie le reste, et « voir moins » le replie. Sous la description, l'équipe de la chaîne est présentée par personne (nom, prénom, fonction, territoire, contact) ; au-delà d'un certain nombre de personnes, seules les premières sont visibles et un « voir plus » révèle les autres. Vient ensuite la liste des programmes de la chaîne : pour chacun, sa périodicité, son nom, sa description, son équipe propre et la liste de ses vidéos. Aucune image de couverture de programme n'est affichée sur cette page.

**Why this priority**: Complète la lecture ouverte par US1 mais suppose la vitrine corrigée pour avoir du sens ; la page reste consultable en l'état d'ici là.

**Independent Test**: Ouvrir `/medias/chaines/<slug>` et vérifier successivement : le pliage/dépliage de la description, le pliage/dépliage de l'équipe, la présence de la périodicité et de l'équipe pour chaque programme, la liste des vidéos, et l'absence de couverture de programme.

**Acceptance Scenarios**:

1. **Given** une chaîne à description courte, **When** la page s'ouvre, **Then** la description est affichée en entier et aucun bouton « voir plus » n'apparaît.
2. **Given** une chaîne à description longue, **When** le visiteur active « voir plus », **Then** l'intégralité de la description s'affiche sans rechargement de page, et le bouton propose alors de replier.
3. **Given** une chaîne dont l'équipe compte 11 personnes, **When** la page s'ouvre, **Then** un sous-ensemble est visible et « voir plus » révèle les 11.
4. **Given** un programme hebdomadaire de la chaîne, **When** la page s'ouvre, **Then** la périodicité « hebdomadaire » est lisible à côté du nom du programme.
5. **Given** un programme comptant 12 épisodes publiés, **When** la page s'ouvre, **Then** ses 12 vidéos sont listées et lisibles depuis cette page.
6. **Given** n'importe quel programme, **When** la page s'ouvre, **Then** son image de couverture n'est pas affichée.

---

### User Story 4 - Page de détail d'un programme (Priority: P2)

Le visiteur ouvre un programme. La page lui montre sa périodicité, son nom, son image de couverture, sa description, son équipe (nom, prénom, fonction, territoire, contact) et la liste de ses vidéos.

**Why this priority**: C'est la destination des cartes de programme de US1 ; elle donne au programme la page complète que la vitrine ne montre plus.

**Independent Test**: Ouvrir `/medias/emissions-tele/<slug>` et vérifier la présence des six blocs, dont l'image de couverture, absente de la page chaîne mais présente ici et sur la vitrine.

**Acceptance Scenarios**:

1. **Given** un programme avec image de couverture, **When** la page s'ouvre, **Then** la couverture est affichée (contrairement à la page de la chaîne).
2. **Given** un programme sans image de couverture, **When** la page s'ouvre, **Then** la mise en page reste cohérente sans emplacement vide signalé.
3. **Given** un programme dont l'équipe diffère de celle de la chaîne, **When** la page s'ouvre, **Then** c'est bien l'équipe du programme qui est affichée.
4. **Given** un programme sans épisode publié, **When** la page s'ouvre, **Then** la page reste consultable et signale explicitement qu'aucune vidéo n'est encore disponible.

---

### User Story 5 - Périodicité enrichie et déclarée (Priority: P3)

Le détenteur choisit la périodicité d'un programme dans un référentiel qui couvre l'absence de périodicité et les cadences usuelles (non périodique, journalier, hebdomadaire, mensuel, et les autres retenues). Cette périodicité est lue par le visiteur sur la page de la chaîne et sur la page du programme.

**Why this priority**: Le référentiel actuel ne compte que trois valeurs et ne couvre pas les cadences demandées ; le reste de la feature fonctionne avec l'existant, ce qui rend cette histoire livrable après coup.

**Independent Test**: Créer un programme mensuel, vérifier que « mensuel » est proposé à la saisie et lisible sur les deux pages publiques.

**Acceptance Scenarios**:

1. **Given** le formulaire d'un programme, **When** le détenteur ouvre le choix de périodicité, **Then** il y trouve exactement quatre valeurs : non périodique, journalier, hebdomadaire, mensuel.
2. **Given** un programme existant créé avant cette feature, **When** il est consulté, **Then** sa périodicité reste celle déclarée auparavant, sans valeur perdue ni requalifiée à tort.
3. **Given** un programme non périodique, **When** il est affiché, **Then** sa périodicité est présentée comme « non périodique » et non comme une absence d'information.

---

### Edge Cases

- **Description absente** : chaîne ou programme sans description, aucun bloc vide, aucune ellipse orpheline.
- **Équipe vide** : chaîne ou programme sans aucun membre, le bloc « équipe » disparaît au lieu d'afficher un cadre vide (vitrine et pages de détail).
- **Membre incomplet** : personne sans territoire ni contact, seuls les champs renseignés s'affichent.
- **Homonymes** : deux personnes de même nom et prénom dans une même équipe, l'ajout reste possible, elles se distinguent par leur fonction.
- **Même personne dans deux équipes** : quelqu'un peut figurer à la fois dans l'équipe de la chaîne et dans celle d'un programme, avec des fonctions différentes.
- **Fonction saisie avec une casse ou des espaces différents** (« Directeur » / « directeur » / « directeur  »), les suggestions ne doivent pas se démultiplier en variantes quasi identiques.
- **Programme sans épisode** : visible sur la vitrine et sur la page chaîne, avec une liste de vidéos explicitement vide.
- **Chaîne sans programme** : la section reste affichée avec l'identité et l'équipe, et signale l'absence de programmes.
- **Description tronquée exactement à la limite** : pas d'ellipse quand le texte tient entièrement.
- **Grand nombre de programmes** (plus de 30 sur une chaîne) : la section de la vitrine reste lisible, ne dégrade pas le défilement, et si elle n'affiche pas tout, elle annonce le total et mène au reste (FR-008).
- **Suppression d'un programme portant une équipe** : l'équipe du programme disparaît avec lui, celle de la chaîne est intacte.
- **Périodicité d'un programme importé/ancien** : conservée telle quelle après extension du référentiel.
- **Membre rattaché à un compte devenu indisponible** (désactivé ou supprimé) : la fiche d'équipe survit et s'affiche en texte simple, sans lien mort.
- **Rattachement d'un compte à deux fiches d'équipes différentes** : autorisé, la même personne peut diriger une chaîne et animer un programme.

## Requirements *(mandatory)*

### Functional Requirements

#### Section de vitrine (« Nos télés africaines » / « Nos radios africaines »)

- **FR-001**: La section d'une chaîne DOIT afficher, dans cet ordre : le nom de la chaîne, un extrait de sa description, son équipe, puis la liste de ses programmes.
- **FR-002**: La section NE DOIT afficher aucun média lisible, ni lecteur vidéo, ni lecteur audio, ni vignette d'épisode, ni liste d'épisodes. Le bandeau textuel de programmation (« en cours de diffusion » / « à suivre ») échappe à cette exigence : il n'expose aucun média lisible et reste affiché.
- **FR-003**: L'extrait de description d'une chaîne DOIT être tronqué au-delà d'une longueur d'affichage définie et signalé par des points de suspension ; en deçà, la description s'affiche entière sans ellipse.
- **FR-004**: Chaque programme DOIT être présenté par son image de couverture, son nom et une courte description tronquée par des points de suspension si elle dépasse.
- **FR-005**: Un programme DOIT rester listé même s'il ne compte aucun épisode publié.
- **FR-006**: Le nom de la chaîne DOIT mener à sa page de détail, et chaque carte de programme à la page de détail de ce programme.
- **FR-007**: Les blocs sans contenu (description absente, équipe vide, aucun programme, champ non renseigné d'un membre d'équipe) NE DOIVENT PAS laisser de cadre ni de libellé vide, en vitrine comme sur les pages de détail.
- **FR-008**: La section DOIT lister tous les programmes de la chaîne. Si un plafond d'affichage s'applique, la section DOIT **annoncer le nombre total** de programmes et offrir un accès au reste sur la page de la chaîne, une troncature silencieuse est proscrite.

#### Équipes

- **FR-010**: Une chaîne (ou station) DOIT pouvoir porter une équipe composée de zéro à plusieurs personnes.
- **FR-011**: Un programme DOIT pouvoir porter sa propre équipe, indépendante de celle de sa chaîne, l'une peut exister sans l'autre, et leur contenu peut différer entièrement.
- **FR-012**: Chaque personne d'une équipe DOIT porter : un nom, un prénom, une fonction, un territoire et un contact. Le nom et la fonction sont obligatoires ; prénom, territoire et contact sont facultatifs.
- **FR-013**: Un membre d'équipe DOIT pouvoir être rattaché, **de façon facultative**, à un compte UAfricas existant. Une équipe DOIT rester déclarable intégralement sans qu'aucun de ses membres soit inscrit sur la plateforme.
- **FR-014**: Lorsqu'un membre est rattaché à un compte, son nom affiché publiquement DOIT mener à son profil public ; sans rattachement, il s'affiche en texte simple. Le rattachement NE DOIT conférer aucun droit sur le support ni sur le programme.
- **FR-015**: Le champ « fonction » DOIT être dynamique à la saisie : les fonctions déjà employées sur la plateforme sont proposées, toute fonction nouvelle est acceptée puis rejoint les propositions, et ces propositions NE DOIVENT PAS présenter plusieurs variantes d'un même libellé ne différant que par la casse ou les espaces superflus.
- **FR-016**: Un gestionnaire DOIT pouvoir ajouter, modifier, retirer et réordonner les personnes d'une équipe, l'ordre déclaré étant l'ordre d'affichage public.
- **FR-017**: La saisie d'une équipe DOIT être ouverte aux détenteurs du support concerné et aux administrateurs, et fermée à tout autre visiteur.
- **FR-018**: Toute création, modification ou suppression d'un membre d'équipe DOIT être tracée dans le journal d'audit de la plateforme.
- **FR-019**: Le retrait d'un programme DOIT emporter le retrait de son équipe, sans effet sur l'équipe de la chaîne.

#### Page de détail d'une chaîne / station

- **FR-020**: La page DOIT afficher le nom de la chaîne.
- **FR-021**: La description DOIT être présentée tronquée si elle est longue, avec une commande « voir plus » qui déplie l'intégralité sans rechargement, et une commande inverse pour replier.
- **FR-022**: Aucune commande « voir plus » NE DOIT apparaître lorsque la description tient entièrement dans l'espace visible.
- **FR-023**: L'équipe de la chaîne DOIT être présentée personne par personne avec nom, prénom, fonction, territoire et contact.
- **FR-024**: Au-delà d'un seuil défini de personnes, l'équipe DOIT être partiellement affichée avec une commande « voir plus » révélant le reste.
- **FR-025**: La page DOIT lister les programmes de la chaîne avec, pour chacun : sa périodicité, son nom, sa description, son équipe propre et la liste de ses vidéos.
- **FR-026**: La page NE DOIT PAS afficher l'image de couverture des programmes.
- **FR-027**: Les vidéos listées sur cette page DOIVENT être consultables depuis celle-ci.

#### Page de détail d'un programme

- **FR-030**: La page DOIT afficher la périodicité, le nom, l'image de couverture, la description, l'équipe du programme et la liste de ses vidéos.
- **FR-031**: L'image de couverture d'un programme DOIT être affichée sur la vitrine et sur la page de détail du programme, et sur ces deux emplacements seulement.
- **FR-032**: L'équipe affichée DOIT être celle du programme, jamais celle de la chaîne par défaut.
- **FR-033**: Un programme sans vidéo publiée DOIT rester consultable et annoncer explicitement l'absence de vidéo.
- **FR-034**: Les mentions héritées « Animation » et « Production » (deux champs de texte libre) NE DOIVENT plus être affichées sur la page d'un programme : l'équipe les remplace. Deux sources concurrentes pour la même information sont proscrites.

#### Périodicité

- **FR-040**: Le référentiel de périodicité DOIT comporter exactement quatre valeurs : **non périodique**, **journalier**, **hebdomadaire**, **mensuel**. Aucune autre cadence n'est déclarable.
- **FR-041**: Les périodicités DOIVENT être présentées à l'utilisateur par des libellés français lisibles, identiques à la saisie et à l'affichage public.
- **FR-042**: La périodicité par défaut d'un programme DOIT être « non périodique ».
- **FR-043**: L'extension du référentiel NE DOIT PAS altérer la périodicité des programmes déjà déclarés : les trois cadences existantes correspondent terme à terme à trois des quatre valeurs cibles, « mensuel » étant la seule valeur neuve.
- **FR-044**: La périodicité DOIT être lisible sur la page de détail de la chaîne et sur la page de détail du programme.

#### Parité Radio

- **FR-060**: Toutes les exigences FR-001 à FR-044 DOIVENT s'appliquer à l'identique aux stations radio, à leurs programmes radio et à leurs contenus audio, en substituant « station » à « chaîne » et « audio » à « vidéo » dans les libellés destinés au visiteur.

### Key Entities

- **Support média** (chaîne de télévision ou station de radio) : entité existante, porteuse de son nom, de sa description et désormais d'une équipe.
- **Programme** (émission télé ou radio) : entité existante rattachée à un support, porteuse de son nom, de sa description, de son image de couverture, de sa **périodicité** et désormais de sa propre équipe.
- **Contenu** (épisode vidéo ou audio) : entité existante rattachée à un programme ; listée sur les pages de détail, retirée des sections de vitrine.
- **Membre d'équipe** (nouveau) : personne rattachée soit à un support, soit à un programme, décrite par nom, prénom, fonction, territoire, contact et rang d'affichage, avec un lien **facultatif** vers un compte UAfricas. Un même individu peut apparaître dans plusieurs équipes avec des fonctions différentes.
- **Fonction** (nouveau, dérivé) : libellé libre porté par un membre d'équipe ; l'ensemble des libellés déjà employés constitue le référentiel proposé à la saisie, sans liste figée.
- **Périodicité** : référentiel fermé de quatre cadences déclarables sur un programme, non périodique, journalier, hebdomadaire, mensuel.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Sur les trois espaces de vitrine (Télé, Radio africaine, Radio nationale), 100 % des sections de chaîne ou de station s'affichent sans aucun média lisible.
- **SC-002**: Depuis une section de vitrine, un visiteur atteint la page d'un programme en un seul clic, et la page d'une chaîne en un seul clic.
- **SC-003**: Un gestionnaire déclare une équipe de trois personnes, dont une avec une fonction inédite, en moins de deux minutes et sans quitter la fiche du support.
- **SC-004**: Une fonction saisie pour la première fois est proposée lors de la saisie suivante dans 100 % des cas.
- **SC-005**: Sur la page de détail d'une chaîne, la description complète et l'équipe complète sont accessibles sans rechargement de page et sans navigation supplémentaire.
- **SC-006**: 100 % des programmes affichent une périodicité intelligible, y compris ceux créés avant la feature.
- **SC-007**: Aucune section, aucune page de détail n'affiche de bloc, cadre ou libellé vide lorsque la donnée correspondante est absente, vérifié sur un jeu couvrant chaîne sans description, chaîne sans équipe, chaîne sans programme, programme sans vidéo.
- **SC-008**: Une section de vitrine portant 30 programmes les affiche tous et reste consultable sans dégradation perceptible du défilement ; au-delà, le total est annoncé et le reste accessible en un clic.
- **SC-009**: Les adresses publiques déjà indexées des chaînes, stations et programmes continuent de résoudre après la feature.
- **SC-010**: Une équipe dont aucun membre n'est inscrit sur la plateforme s'enregistre et s'affiche intégralement ; un membre rattaché à un compte mène à son profil public en un clic.
- **SC-011**: Le bandeau de programmation reste visible dans 100 % des sections dont la grille annonce une diffusion, sans réintroduire de média lisible.

## Assumptions

- **Périmètre visuel uniquement pour les vidéos** : « ne pas afficher les vidéos » vise les **sections de vitrine**. Le contenu vedette en tête de l'espace Télé (bandeau plein écran) et les pages de détail conservent leurs médias lisibles.
- **Le contact d'un membre d'équipe est public** : il est présenté à tout visiteur, connecté ou non, au même titre que les coordonnées déjà publiées d'une chaîne. Il s'agit d'un contact professionnel déclaré volontairement par le gestionnaire du support, non d'une donnée personnelle d'un compte de la plateforme.
- **Le territoire d'un membre est saisi librement**, en cohérence avec la terminologie « territoire » retenue sur le reste de la plateforme.
- **Un membre d'équipe est d'abord une fiche descriptive**, rattachable facultativement à un compte UAfricas (FR-013, FR-014). Le rattachement ne sert qu'à lier la fiche au profil public : il ne confère aucun droit, et les droits de gestion restent portés par le dispositif de détention de support existant. Une chaîne dont aucun dirigeant n'est inscrit déclare son équipe normalement.
- **Les champs de texte libre existants « info animateur » et « info producteur »** portés par les programmes deviennent redondants avec l'équipe du programme. Ils sont **conservés en base** : aucune saisie antérieure n'est perdue, mais **cessent d'être affichés** (FR-034) et sont signalés comme hérités dans les formulaires d'édition, où ils restent visibles en lecture le temps que les gestionnaires reportent leur contenu dans l'équipe.
- **Seuils de troncature** : les longueurs d'extrait (description de chaîne, description de programme) et le seuil de repli de l'équipe sont des choix d'ergonomie fixés à la réalisation, pas des paramètres exposés à l'utilisateur.
- **La périodicité n'est pas affichée dans les sections de vitrine** : le commanditaire n'en fait mention que sur les deux pages de détail.
- **Les fonctions de gestion existantes sont préservées** : signalement, réactions, partage, propositions d'idée et demandes d'animation restent accessibles là où ils le sont aujourd'hui, la vitrine perdant seulement ses éléments lisibles. Les réactions et le partage, aujourd'hui attachés à l'épisode mis en avant dans la section, perdent leur cible : ils sont reportés sur les pages de détail, seules à porter désormais un contenu identifié.
- **Le bandeau de programmation reste en vitrine** : « en cours de diffusion » et « à suivre » sont du texte, non un média lisible ; ils survivent au retrait des vidéos et prennent place entre l'équipe de la chaîne et la liste des programmes.
- **Aucun changement de droits** : les détenteurs de support conservent exactement le périmètre d'action actuel, augmenté de la gestion des équipes.
- **La suppression d'un membre d'équipe est définitive** du point de vue de l'affichage ; sa conservation éventuelle relève des conventions de suppression douce déjà en vigueur sur la plateforme.
