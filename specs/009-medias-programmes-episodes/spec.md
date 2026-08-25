# Feature Specification: Médias : programmes conteneurs, épisodes, thématiques multiples et couverture panafricaine

**Feature Branch**: `009-medias-programmes-episodes`

**Created**: 2026-08-08

**Status**: Draft

**Input**: User description: "mise à jour majeur au niveau de télé/radio, front et back office.
- Une chaine peut avoir 1 ou plusieurs thématiques.
- Une chaine peut concerner plusieurs pays ou toute l'afrique
- Un programme n'est pas une vidéo(dans télé)/audio(dans radio) mais plutôt un groupe/catégorie de vidéo/audio
- Les vidéos(dans télé)/audio(dans radio) doivent donc etre regroupé par programme
- Sur les chaines moi les programmation quotidien, hebdomadaire
- Quand un programme est hebdomadaire par exemple, il va ajouter des vidéo(dans télé)/audio(dans radio) chaque semaine au programme concerné"

## Clarifications

### Session 2026-08-08

- **Q1 : Quel épisode passe à une occurrence de créneau ?** → **Rotation** dans l'ordre des épisodes de
  l'émission, calculée depuis la date d'entrée en vigueur du créneau. Aucune date de diffusion n'est
  saisie par le détenteur : l'ordre de l'émission suffit, et l'antenne ne connaît jamais de trou dès
  qu'il existe un épisode publié.
- **Q2 : Un épisode ajouté par un co-détenteur est-il diffusé immédiatement ?** → **Non : modération
  systématique.** Chaque épisode passe par la file de validation administrative avant d'entrer en
  diffusion. La cadence hebdomadaire dépend donc du délai de traitement, ce que la spécification prend
  en charge explicitement (anticipation de l'échéance, suivi de l'état par le détenteur).
- **Q3 : Sur quoi portent réactions, commentaires, partages et signalements ?** → **Sur les deux
  niveaux, indépendamment.** Une émission se suit et se commente en tant que série ; un épisode se
  commente pour lui-même. Deux jeux de compteurs distincts, deux surfaces de modération.

## Contexte et problème

Aujourd'hui, sur les espaces Télé et Radio, un **programme est confondu avec un unique fichier média** :
chaque « programme télé » porte une seule vidéo, chaque « programme radio » un seul audio. Une émission
récurrente (un magazine hebdomadaire, un journal quotidien) ne peut donc pas exister en tant que telle :
son détenteur doit recréer un « programme » complet à chaque nouvelle diffusion, ce qui fragmente la
collection, casse l'abonnement mental du public à l'émission et rend la grille de programmation
ingérable dans la durée.

Par ailleurs, une chaîne ne peut déclarer **qu'une seule catégorie** et **qu'un seul territoire**, alors
que la réalité du paysage audiovisuel panafricain est multi-thématique et souvent multi-pays, voire
continentale.

Cette évolution recadre le modèle éditorial des espaces Télé et Radio, côté public **et** back-office.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Regrouper les vidéos et audios sous un programme (Priority: P1)

Un co-détenteur de chaîne TV (ou de station radio) crée une **émission**, par exemple « Journal de
l'Afrique » : puis y verse ses **épisodes** successifs (une vidéo pour la télé, un audio pour la radio).
Le public qui visite la chaîne découvre d'abord la liste des émissions, puis, en ouvrant une émission,
la collection de ses épisodes du plus récent au plus ancien. L'administrateur retrouve la même
hiérarchie dans le back-office.

**Why this priority**: C'est le recadrage structurel dont tout le reste dépend. Sans lui, la grille de
programmation, la cadence hebdomadaire et l'exposition éditoriale n'ont pas d'objet auquel s'accrocher.
Livrée seule, cette histoire suffit déjà à transformer une collection plate de vidéos en catalogue
d'émissions navigable.

**Independent Test**: Créer une chaîne, y créer une émission, y ajouter trois épisodes, les faire
valider, puis vérifier depuis la page publique de la chaîne que l'émission apparaît une seule fois et
donne accès à ses trois épisodes dans l'ordre déclaré. Vérifier que les contenus antérieurs à la reprise
de données restent accessibles et rattachés à une émission.

**Acceptance Scenarios**:

1. **Given** une chaîne TV publiée dont je suis co-détenteur, **When** je crée une émission « Débats
   africains » sans y joindre aucun fichier, **Then** l'émission est enregistrée et le système ne me
   réclame ni vidéo ni audio à ce stade.
2. **Given** l'émission « Débats africains », **When** j'y ajoute un épisode avec sa vidéo et son titre,
   **Then** l'épisode prend rang à la fin de l'ordre de l'émission, part en attente de validation
   administrative, et n'est ni lisible par le public ni diffusé tant qu'il n'est pas validé.
3. **Given** une chaîne comportant trois émissions et douze épisodes, **When** un visiteur ouvre la page
   de la chaîne, **Then** il voit trois blocs d'émissions, et non douze vignettes indifférenciées, et
   chaque bloc annonce son nombre d'épisodes disponibles.
4. **Given** un épisode publié, **When** un visiteur l'ouvre, **Then** la page de l'épisode indique
   l'émission dont il relève et la chaîne (ou station) qui le diffuse, et propose les autres épisodes de
   la même émission.
5. **Given** les contenus existants avant la mise à jour (chaque « programme » portant un seul fichier),
   **When** la reprise de données est appliquée, **Then** chacun devient une émission contenant
   exactement un épisode, et toutes les adresses publiques qui pointaient vers l'ancien contenu
   continuent de mener au contenu correspondant.
6. **Given** une émission dont aucun épisode n'est publié, **When** un visiteur consulte la chaîne,
   **Then** l'émission n'apparaît pas dans l'espace public, mais reste visible et modifiable par ses
   co-détenteurs et par l'administrateur.
7. **Given** une station radio, **When** j'y crée une émission et j'y verse des audios, **Then** le
   comportement est identique à celui de la télé, à la nature du fichier près.
8. **Given** un épisode que j'ai soumis, **When** l'administrateur le valide ou le rejette, **Then** je
   suis notifié de la décision : assortie du motif en cas de rejet, et je retrouve l'état de chacun de
   mes épisodes soumis depuis l'espace de gestion de mon support.

---

### User Story 2 - Programmer une émission au rythme quotidien ou hebdomadaire (Priority: P2)

Un co-détenteur inscrit une **émission**, et non plus un fichier isolé, dans la grille de sa chaîne :
« Journal de l'Afrique, tous les jours à 20h00 », « Débats africains, chaque samedi à 18h00 ». L'émission
déclare sa cadence, et le détenteur y ajoute un nouvel épisode à chaque échéance. À chaque occurrence du
créneau, l'antenne avance d'un cran dans l'ordre des épisodes de l'émission et reprend au premier une
fois le dernier atteint. Le public voit, sur la chaîne, ce qui est à l'antenne maintenant et ce qui suit,
avec l'épisode retenu pour l'occurrence en cours.

**Why this priority**: C'est la demande explicite du commanditaire (« programmation quotidienne,
hebdomadaire ») et ce qui donne son sens au regroupement de l'histoire 1. Elle est testable dès que les
émissions existent.

**Independent Test**: Créer une émission hebdomadaire comportant trois épisodes validés, la programmer un
jour et une heure donnés, puis vérifier que la grille publique de la chaîne annonce la bonne émission et
le premier épisode à la première occurrence, le deuxième à la suivante, et qu'après le troisième la
rotation revient au premier en signalant une rediffusion.

**Acceptance Scenarios**:

1. **Given** une émission de ma chaîne, **When** je la programme « tous les jours à 20h00 », **Then** la
   grille publique de la chaîne affiche cette occurrence pour chaque jour.
2. **Given** une émission programmée « chaque samedi à 18h00 » et comportant trois épisodes validés,
   **When** j'ouvre la chaîne un samedi à 18h30 pendant la durée déclarée, **Then** le bandeau « à
   l'antenne » désigne cette émission et l'épisode que la rotation retient pour cette occurrence.
3. **Given** une émission hebdomadaire dont la rotation a déjà parcouru tous les épisodes, **When** une
   nouvelle semaine commence sans qu'un épisode inédit ait été validé, **Then** la rotation reprend au
   premier épisode, le public voit la mention « rediffusion », et les co-détenteurs sont alertés que
   l'échéance d'ajout est dépassée.
4. **Given** une émission hebdomadaire en rotation, **When** j'ajoute un nouvel épisode et qu'il est
   validé, **Then** il prend rang à la fin de l'ordre sans modifier l'épisode annoncé pour l'occurrence
   en cours, et il entre dans le cycle à partir de l'occurrence suivante.
5. **Given** une émission programmée dont aucun épisode n'est encore validé, **When** l'occurrence
   survient, **Then** le créneau n'est pas annoncé au public et ses co-détenteurs sont alertés que
   l'émission est programmée sans contenu diffusable.
6. **Given** une émission en rotation, **When** un de ses épisodes est retiré, suspendu ou déplacé vers
   une autre émission, **Then** le cycle se recalcule sur les épisodes restants et l'antenne continue
   sans interruption ni doublon dans le même cycle.
7. **Given** deux émissions programmées sur des plages qui se chevauchent le même jour, **When** je
   valide la seconde, **Then** le système refuse l'enregistrement et m'indique la plage en conflit, sans
   modifier la grille existante.
8. **Given** une grille comportant des créneaux hérités qui pointaient vers un fichier isolé, **When** la
   reprise de données est appliquée, **Then** chaque créneau pointe vers l'émission issue de ce fichier,
   sans perte de jour, d'heure, de durée ni de fuseau.
9. **Given** une émission programmée, **When** je la retire de la grille ou que je la dépublie, **Then**
   ses créneaux cessent d'être annoncés au public sans que les épisodes déjà publiés deviennent
   inaccessibles.
10. **Given** une grille déclarée dans un fuseau donné, **When** un visiteur situé dans un autre fuseau
    la consulte, **Then** l'horaire affiché indique explicitement le référentiel horaire employé.

---

### User Story 3 - Déclarer plusieurs thématiques par chaîne ou station (Priority: P3)

Un administrateur : ou le membre qui soumet sa chaîne, sélectionne une ou plusieurs thématiques parmi
le référentiel éditorial de la plateforme. Le public filtre les chaînes et stations par thématique et
retrouve toutes celles qui la traitent, même partiellement.

**Why this priority**: Améliore nettement la découverte, mais l'espace reste utilisable sans. Dépend du
référentiel de thèmes déjà en place.

**Independent Test**: Attribuer trois thématiques à une chaîne, filtrer l'espace Télé sur chacune des
trois, et vérifier que la chaîne remonte dans les trois cas et une seule fois par filtre.

**Acceptance Scenarios**:

1. **Given** le formulaire de création d'une chaîne, **When** je sélectionne trois thématiques, **Then**
   les trois sont enregistrées et affichées sur la fiche publique de la chaîne.
2. **Given** une chaîne publiée, **When** je tente de la publier sans aucune thématique, **Then** le
   système refuse et m'indique qu'au moins une thématique est requise.
3. **Given** une chaîne portant les thématiques A et B, **When** un visiteur filtre l'espace Télé sur la
   thématique A puis sur la thématique B, **Then** la chaîne apparaît dans les deux résultats, sans
   doublon à l'intérieur d'un même résultat.
4. **Given** une chaîne héritée ne portant qu'une catégorie unique, **When** la reprise de données est
   appliquée, **Then** cette catégorie devient sa première thématique et la chaîne reste consultable et
   filtrable sans intervention manuelle.
5. **Given** une station radio, **When** j'édite ses thématiques, **Then** le comportement est identique
   à celui d'une chaîne TV.
6. **Given** une émission rattachée à une chaîne, **When** elle déclare son propre thème, **Then** ce
   thème reste indépendant des thématiques de la chaîne et ne les remplace pas.

---

### User Story 4 - Déclarer une couverture multi-territoires ou panafricaine (Priority: P4)

Une chaîne ou une station déclare soit une liste de territoires africains qu'elle couvre, soit une
couverture continentale (« toute l'Afrique »). Le public filtre par territoire et retrouve les supports
locaux comme les supports panafricains.

**Why this priority**: Corrige une limite réelle du modèle actuel, mais sans elle l'espace fonctionne, 
avec une couverture approximative.

**Independent Test**: Déclarer une chaîne couvrant trois territoires et une autre en couverture
continentale, puis filtrer sur l'un des trois territoires et vérifier que les deux chaînes remontent.

**Acceptance Scenarios**:

1. **Given** le formulaire d'une chaîne, **When** je sélectionne quatre territoires, **Then** les quatre
   sont enregistrés et affichés sur la fiche publique.
2. **Given** le formulaire d'une chaîne, **When** je coche « toute l'Afrique », **Then** la sélection
   individuelle de territoires est neutralisée, et la fiche publique annonce une couverture
   continentale.
3. **Given** une chaîne en couverture continentale, **When** un visiteur filtre sur un territoire
   quelconque, **Then** la chaîne remonte dans les résultats.
4. **Given** une chaîne couvrant les territoires X et Y, **When** un visiteur filtre sur le territoire Z,
   **Then** la chaîne ne remonte pas.
5. **Given** une chaîne héritée rattachée à un territoire unique, **When** la reprise de données est
   appliquée, **Then** ce territoire devient son unique territoire de couverture.
6. **Given** une chaîne publiée, **When** je tente de l'enregistrer sans aucun territoire ni couverture
   continentale, **Then** le système refuse et m'indique qu'une couverture est requise.

---

### User Story 5 - Réagir à une émission comme à un épisode (Priority: P5)

Un visiteur commente un épisode qu'il vient de regarder, et commente séparément l'émission elle-même en
tant que série suivie. Les deux conversations coexistent sans se mélanger, chacune avec ses propres
compteurs. Un modérateur traite les signalements déposés à l'un comme à l'autre niveau.

**Why this priority**: Prolonge l'existant plutôt qu'il ne le crée, les interactions fonctionnent déjà,
elles doivent seulement gagner un second point d'ancrage. Livrable après le recadrage structurel.

**Independent Test**: Déposer une réaction et un commentaire sur un épisode, puis sur son émission, et
vérifier que chaque fil et chaque compteur reste distinct, et que le partage de l'un ne pointe pas vers
l'autre.

**Acceptance Scenarios**:

1. **Given** un épisode publié, **When** je le commente, **Then** mon commentaire apparaît sur l'épisode
   et n'apparaît pas sur le fil de l'émission.
2. **Given** une émission publiée, **When** je réagis à l'émission, **Then** le compteur de l'émission
   s'incrémente et aucun compteur d'épisode ne bouge.
3. **Given** une émission et ses épisodes portant chacun leurs interactions, **When** je consulte
   l'émission, **Then** ses propres compteurs et ceux de ses épisodes sont présentés distinctement, sans
   être confondus en un total unique.
4. **Given** un épisode déplacé vers une autre émission, **When** je consulte ses interactions, **Then**
   elles l'ont suivi intégralement, et les fils des deux émissions sont inchangés.
5. **Given** un contenu signalé, **When** le seuil de suspension automatique est franchi, **Then** seul
   le niveau signalé est suspendu : signaler un épisode ne suspend pas son émission, signaler une
   émission retire ses épisodes de l'espace public sans les supprimer.
6. **Given** les interactions déposées avant la mise à jour, **When** la reprise de données est
   appliquée, **Then** elles restent rattachées à l'épisode issu du contenu qu'elles visaient.

---

### Edge Cases

- **Émission vide** : une émission sans aucun épisode publié n'apparaît pas dans l'espace public, mais
  reste gérable par ses co-détenteurs et l'administration.
- **Épisode orphelin** : aucun épisode ne peut exister sans émission de rattachement ; à la reprise de
  données, tout fichier sans rattachement rejoint une émission créée à partir de son propre intitulé.
- **Déplacement d'un épisode** d'une émission vers une autre : l'épisode conserve ses réactions,
  commentaires, partages et signalements, le décompte des deux émissions est mis à jour, et le cycle de
  rotation des deux émissions se recalcule.
- **Suppression d'une émission contenant des épisodes** : refusée tant que l'émission contient des
  épisodes publiés ; le retrait exige de déplacer ou de retirer les épisodes au préalable.
- **Suspension** : suspendre une émission retire du public tous ses épisodes sans les supprimer ;
  suspendre un support retire ses émissions et leurs épisodes.
- **Émission programmée sans aucun épisode validé** : le créneau n'est pas annoncé au public, la
  rotation n'a rien à faire tourner : et les co-détenteurs sont alertés.
- **Rotation épuisée** : quand le cycle repasse sur un épisode déjà diffusé, l'antenne le signale comme
  rediffusion et alerte les co-détenteurs de l'échéance dépassée ; il n'y a jamais d'espace vide dès
  qu'un épisode existe.
- **Ordre modifié en cours de cycle** : retirer, suspendre ou ajouter un épisode recalcule le cycle sans
  changer ce qui est déjà annoncé pour l'occurrence en cours.
- **Épisode en attente de validation** : il n'entre ni dans la rotation, ni dans les compteurs publics,
  ni dans la liste publique de son émission ; son détenteur le voit et en suit l'état.
- **Épisode rejeté par la modération** : son auteur est notifié avec le motif ; l'épisode n'entre jamais
  dans la rotation et peut être corrigé puis resoumis.
- **Chevauchement de créneaux** sur un même support : refusé, avec désignation de la plage en conflit.
- **Créneau franchissant minuit** : refusé ; le détenteur le scinde en deux créneaux.
- **Couverture contradictoire** : « toute l'Afrique » et une liste de territoires sont mutuellement
  exclusifs.
- **Chaîne ou station sans thématique héritée** : tolérée en lecture, mais toute modification ultérieure
  impose d'en déclarer au moins une.
- **Mise en avant** : la mise en avant d'un support et la vedette de l'espace Télé désignent désormais un
  épisode précis, situé dans son émission ; retirer cet épisode libère la mise en avant sans en laisser
  une seconde active.
- **Volume** : une chaîne comptant plusieurs dizaines d'émissions et plusieurs centaines d'épisodes reste
  navigable, la liste des épisodes d'une émission étant paginée ou chargée progressivement.
- **Signalement** : le seuil de suspension automatique déjà en vigueur s'applique au niveau où le
  signalement est déposé, sans retirer mécaniquement les contenus voisins, signaler un épisode ne
  suspend pas son émission.
- **File de modération engorgée** : les épisodes en attente s'accumulent sans bloquer l'antenne, la
  rotation continuant sur les épisodes déjà validés ; l'administration voit l'ancienneté de chaque
  épisode en attente et les échéances de créneau qu'il approche.

## Requirements *(mandatory)*

### Structure éditoriale : émissions et épisodes

- **FR-001**: Le système MUST distinguer deux niveaux distincts sous un support (chaîne TV ou station
  radio) : l'**émission** (autrefois nommée « programme »), qui est un regroupement, et l'**épisode**,
  qui porte le fichier vidéo (télé) ou audio (radio).
- **FR-002**: Une émission MUST appartenir à exactement un support, et un épisode MUST appartenir à
  exactement une émission.
- **FR-003**: Une émission MUST pouvoir être créée et enregistrée sans contenir aucun épisode.
- **FR-004**: Un épisode MUST porter au minimum un intitulé et un fichier ou lien média ; sa date de
  publication est enregistrée automatiquement et ne conditionne pas sa diffusion.
- **FR-005**: Les épisodes d'une émission MUST être **ordonnés de façon stable et explicite** : un rang
  déterminé par le numéro d'épisode lorsqu'il est renseigné, et à défaut par l'ordre d'ajout. Ce rang est
  ce que la rotation de programmation parcourt (FR-016).
- **FR-006**: Le système MUST permettre à un co-détenteur de réordonner manuellement les épisodes d'une
  émission, et MUST refléter ce nouvel ordre dans la rotation à partir de l'occurrence suivante.
- **FR-007**: Un épisode ajouté à une émission MUST prendre rang **à la fin** de l'ordre existant, sans
  déplacer les épisodes déjà en place.
- **FR-008**: Le système MUST empêcher la publication d'un épisode dépourvu de fichier ou de lien média.
- **FR-009**: Le système MUST permettre de déplacer un épisode d'une émission vers une autre émission du
  même support, en conservant ses interactions associées et en recalculant le cycle de rotation des deux
  émissions.
- **FR-010**: Le système MUST refuser la suppression d'une émission tant qu'elle contient des épisodes
  publiés.
- **FR-011**: Le système MUST propager l'état d'une émission à ses épisodes pour l'exposition publique :
  une émission non publiée ou suspendue MUST rendre ses épisodes invisibles au public.
- **FR-012**: Les compteurs affichés sur une émission (nombre d'épisodes, date du dernier épisode) MUST
  ne tenir compte que des épisodes visibles par le public consultant.

### Cadence et grille de programmation

- **FR-013**: Une émission MUST déclarer une cadence parmi : quotidienne, hebdomadaire, ou ponctuelle
  (sans périodicité).
- **FR-014**: La grille de programmation d'un support MUST porter sur des **émissions**, et non plus sur
  des fichiers isolés.
- **FR-015**: Un créneau MUST déclarer une récurrence quotidienne ou hebdomadaire, une heure de début,
  une durée, un référentiel horaire et une date d'entrée en vigueur ; un créneau hebdomadaire MUST
  désigner exactement un jour de la semaine, un créneau quotidien aucun.
- **FR-016**: Le système MUST déterminer l'épisode diffusé à une occurrence par **rotation** : les
  occurrences du créneau sont comptées depuis sa date d'entrée en vigueur, et l'occurrence de rang N
  diffuse l'épisode de rang N dans l'ordre de l'émission (FR-005), le cycle reprenant au premier épisode
  une fois le dernier atteint.
- **FR-017**: La rotation MUST être déterministe : deux consultations d'une même occurrence, par des
  visiteurs différents ou à des instants différents de la plage, MUST désigner le même épisode.
- **FR-018**: La rotation MUST ne retenir que les épisodes **publiés** ; un épisode en attente de
  validation, rejeté, suspendu ou supprimé MUST en être exclu.
- **FR-019**: L'ajout, le retrait, la suspension, le déplacement ou le réordonnancement d'un épisode MUST
  recalculer le cycle sans modifier l'épisode déjà annoncé pour l'occurrence en cours.
- **FR-020**: Lorsque la rotation rejoue un épisode déjà diffusé sur ce créneau, le système MUST le
  signaler au public comme une **rediffusion**.
- **FR-021**: Lorsqu'une émission programmée ne compte aucun épisode publié, le système MUST ne pas
  annoncer le créneau au public et MUST alerter les co-détenteurs du support.
- **FR-022**: Le système MUST refuser deux créneaux qui se chevauchent sur un même support, en désignant
  la plage en conflit et sans altérer la grille existante.
- **FR-023**: Le système MUST refuser un créneau qui franchirait minuit.
- **FR-024**: Le système MUST signaler aux co-détenteurs d'une émission périodique l'approche et le
  dépassement de l'échéance d'ajout du prochain épisode, en cohérence avec la cadence déclarée. L'alerte
  d'approche MUST intervenir suffisamment tôt pour laisser place au délai de validation administrative
  (FR-040).
- **FR-025**: L'espace public d'un support MUST indiquer ce qui est à l'antenne à l'instant de la
  consultation et ce qui suit, en nommant l'émission et l'épisode concernés, et en signalant le cas
  échéant la rediffusion.
- **FR-026**: L'horaire affiché d'un créneau MUST indiquer explicitement le référentiel horaire employé.
- **FR-027**: Le retrait d'une émission de la grille ou sa dépublication MUST cesser d'annoncer ses
  créneaux au public sans rendre ses épisodes déjà publiés inaccessibles.

### Thématiques du support

- **FR-028**: Une chaîne TV et une station radio MUST pouvoir déclarer une ou plusieurs thématiques
  issues du référentiel éditorial média de la plateforme.
- **FR-029**: Le système MUST exiger au moins une thématique pour publier ou modifier un support.
- **FR-030**: Le système MUST permettre de filtrer les supports par thématique dans l'espace public, un
  support multi-thématique remontant sur chacune de ses thématiques sans doublon au sein d'un résultat.
- **FR-031**: La fiche publique d'un support MUST afficher l'ensemble de ses thématiques.
- **FR-032**: Le thème éventuellement porté par une émission MUST rester indépendant des thématiques de
  son support.

### Couverture territoriale du support

- **FR-033**: Une chaîne TV et une station radio MUST pouvoir déclarer soit une liste de territoires
  couverts, soit une couverture continentale (« toute l'Afrique »).
- **FR-034**: Le système MUST rendre ces deux modes mutuellement exclusifs.
- **FR-035**: Le système MUST exiger une couverture explicite (au moins un territoire, ou la couverture
  continentale) pour publier ou modifier un support.
- **FR-036**: Un support en couverture continentale MUST remonter dans le filtre de n'importe quel
  territoire africain.
- **FR-037**: La fiche publique d'un support MUST afficher sa couverture, sous forme de liste de
  territoires ou de mention continentale.

### Back-office et gestion par les détenteurs

- **FR-038**: Le back-office MUST offrir, pour la télé comme pour la radio, la gestion des émissions
  (création, modification, changement d'état, suppression) et la gestion des épisodes au sein d'une
  émission, y compris leur ordre.
- **FR-039**: Les co-détenteurs d'un support MUST pouvoir créer des émissions, y ajouter des épisodes et
  gérer la grille de programmation, selon les rôles de détention déjà en vigueur.
- **FR-040**: Tout épisode ajouté par un co-détenteur MUST être placé en **attente de validation
  administrative** : il n'est ni visible du public, ni compté, ni intégré à la rotation tant qu'un
  administrateur ne l'a pas validé.
- **FR-041**: Le système MUST notifier l'auteur de la décision de modération, validation ou rejet
  motivé : et MUST permettre de corriger puis resoumettre un épisode rejeté.
- **FR-042**: Le co-détenteur MUST pouvoir suivre, depuis l'espace de gestion de son support, l'état de
  chacun de ses épisodes soumis (en attente, validé, rejeté) et la date de soumission.
- **FR-043**: La file de modération MUST présenter les épisodes en attente avec leur ancienneté, leur
  émission, leur support et l'échéance de créneau qu'ils approchent le cas échéant, afin qu'un épisode
  attendu à l'antenne ne soit pas traité au même rang qu'un contenu sans échéance.
- **FR-044**: Le parcours de proposition d'un média par un membre MUST être aligné sur la nouvelle
  structure : proposer une émission, ou proposer un épisode pour une émission existante.
- **FR-045**: Toute mutation portant sur une émission, un épisode, son ordre, une thématique, une
  couverture territoriale, un créneau ou une décision de modération MUST être tracée dans le journal
  d'audit, avec l'auteur et l'état avant et après.
- **FR-046**: Les listes du back-office MUST permettre de retrouver une émission par support, par état et
  par cadence, et un épisode par émission, par état et par date de soumission.

### Interactions, mise en avant et continuité

- **FR-047**: Réactions, commentaires, partages et signalements MUST pouvoir être déposés **aussi bien
  sur une émission que sur un épisode**, les deux niveaux étant indépendants.
- **FR-048**: Les compteurs et les fils de discussion des deux niveaux MUST rester distincts : le système
  MUST NOT confondre en un total unique les interactions d'une émission et celles de ses épisodes, et
  l'interface MUST présenter les deux séparément lorsqu'elle les affiche ensemble.
- **FR-049**: Le partage d'un épisode MUST mener à l'épisode, le partage d'une émission à l'émission.
- **FR-050**: Le seuil de suspension automatique sur signalement MUST s'appliquer au niveau où les
  signalements sont déposés : la suspension d'un épisode MUST NOT suspendre son émission ; la suspension
  d'une émission retire ses épisodes de l'espace public sans les supprimer.
- **FR-051**: Les interactions existantes MUST être préservées par la reprise de données et rester
  rattachées à l'épisode issu du contenu qu'elles visaient.
- **FR-052**: La mise en avant d'un support et la vedette de l'espace Télé MUST désigner un épisode
  précis ; le système MUST garantir qu'une seule mise en avant est active par support et une seule
  vedette pour l'espace Télé.
- **FR-053**: Le retrait ou la dépublication d'un épisode mis en avant MUST libérer la mise en avant sans
  laisser une seconde mise en avant active.
- **FR-054**: Les règles d'attribution de points d'engagement MUST rester applicables après le recadrage
  et MUST désigner sans ambiguïté le contenu concerné et son auteur bénéficiaire, pour une interaction
  déposée sur une émission comme sur un épisode.
- **FR-055**: La reprise de données MUST convertir chaque contenu existant en une émission contenant un
  épisode unique, sans perte de métadonnée, de fichier, d'interaction ni de rattachement à un support.
- **FR-056**: Les adresses publiques existantes des contenus MUST continuer de mener au contenu
  correspondant après la reprise de données.
- **FR-057**: La reprise de données MUST convertir la catégorie unique d'un support en sa première
  thématique et son territoire unique en son unique territoire de couverture.
- **FR-058**: La reprise de données MUST rattacher chaque créneau hérité à l'émission issue du contenu
  qu'il désignait, en conservant jour, heure, durée et fuseau, et en fixant sa date d'entrée en vigueur à
  la date de reprise.

### Key Entities

- **Support média** : une chaîne TV ou une station radio. Porte son identité éditoriale, ses contacts,
  son origine de publication, ses **thématiques** (une ou plusieurs) et sa **couverture territoriale**
  (liste de territoires ou continentale). Détenu par un ou plusieurs membres selon des rôles.
- **Émission** (anciennement « programme ») : regroupement de contenus au sein d'un support. Porte un
  intitulé, une description, une image, des intervenants, un thème éventuel, une **cadence**
  (quotidienne, hebdomadaire, ponctuelle) et un état de publication. Ne porte aucun fichier média.
- **Épisode** : unité diffusable rattachée à exactement une émission. Porte un intitulé, un fichier ou
  lien vidéo (télé) ou audio (radio), un **rang** dans l'ordre de son émission, un numéro facultatif, une
  durée, une date de soumission et un état de publication (en attente, publié, rejeté, suspendu).
- **Ordre des épisodes** : suite stable des épisodes publiés d'une émission, déterminée par le numéro
  d'épisode ou l'ordre d'ajout, modifiable par le détenteur. C'est le support du cycle de rotation.
- **Thématique média** : entrée du référentiel éditorial de la plateforme, partagée par les supports et
  les émissions. Relation multiple avec le support.
- **Couverture territoriale** : rattachement d'un support à un ou plusieurs territoires africains, ou
  déclaration d'une portée continentale exclusive de toute liste.
- **Créneau de programmation** : inscription d'une **émission** dans la grille d'un support, avec
  récurrence (quotidienne ou hebdomadaire), jour le cas échéant, heure de début, durée, référentiel
  horaire et date d'entrée en vigueur, cette dernière servant d'origine au comptage des occurrences. Ne
  matérialise aucune occurrence à l'avance.
- **Occurrence** : une survenue datée d'un créneau, déduite à la consultation et jamais enregistrée. Son
  rang détermine, par rotation, l'épisode diffusé.
- **Détention de support** : lien entre un membre et un support, avec un rôle déterminant ce qu'il peut
  éditer, publier et programmer.
- **Proposition de média** : soumission par un membre d'une émission ou d'un épisode, en attente de
  décision administrative.
- **Interaction** : réaction, commentaire, partage ou signalement déposé **soit sur une émission, soit
  sur un épisode**, les deux niveaux portant des fils et des compteurs distincts.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100 % des vidéos et audios présents avant la mise à jour restent accessibles au public
  après la reprise de données, et 100 % des adresses publiques antérieures mènent toujours au contenu
  correspondant.
- **SC-002**: Un co-détenteur soumet un nouvel épisode à une émission existante en moins de 2 minutes et
  au plus 3 étapes, sans avoir à ressaisir la description, l'image ni les intervenants de l'émission.
- **SC-003**: Depuis la page d'un support, un visiteur atteint n'importe quel épisode publié en 2 clics
  au maximum.
- **SC-004**: 100 % des supports publiés déclarent au moins une thématique et une couverture
  territoriale explicite.
- **SC-005**: Pour 100 % des créneaux actifs dont l'émission compte au moins un épisode publié, la
  mention « à l'antenne » désigne l'émission attendue et un épisode existant, ou une rediffusion
  explicitement signalée : jamais un emplacement vide. Les créneaux dont l'émission n'a aucun épisode
  publié ne sont pas annoncés.
- **SC-006**: Deux consultations de la même occurrence désignent le même épisode dans 100 % des cas, et
  l'ajout d'un épisode ne modifie jamais l'épisode annoncé pour l'occurrence en cours.
- **SC-007**: 95 % des épisodes soumis sont traités par la modération avant l'échéance du créneau qu'ils
  visent, et aucun épisode ne reste en attente plus de 7 jours sans décision.
- **SC-008**: 100 % des décisions de modération donnent lieu à une notification à leur auteur, assortie
  d'un motif en cas de rejet.
- **SC-009**: Un support comptant 50 émissions et 500 épisodes s'affiche entièrement navigable, la
  première vue apparaissant en moins de 2 secondes sur une connexion mobile courante.
- **SC-010**: Un filtre par thématique ou par territoire renvoie ses résultats en moins d'1 seconde
  perçue.
- **SC-011**: Le nombre de contenus créés par les détenteurs pour une même émission récurrente augmente,
  et le nombre d'émissions dupliquées (même intitulé, même support) tombe à zéro.
- **SC-012**: 100 % des mutations sur émissions, épisodes, ordres, thématiques, couvertures, créneaux et
  décisions de modération apparaissent dans le journal d'audit avec leur auteur.

## Assumptions

- Le périmètre couvre **symétriquement la télé et la radio** : ce que la spécification énonce pour une
  chaîne TV et ses vidéos vaut pour une station radio et ses audios, à la nature du fichier près. Le
  commanditaire annonce explicitement une mise à jour « au niveau de télé/radio ».
- Le terme **« programme »** employé par le commanditaire est repris ici sous le nom d'**émission** pour
  lever l'ambiguïté avec l'unité diffusable ; le libellé retenu dans l'interface publique reste à
  arbitrer et n'affecte pas les exigences.
- Les **thématiques** proviennent du référentiel éditorial média déjà en place sur la plateforme (44
  entrées), et non d'une saisie libre ; ce référentiel n'est pas étendu par cette évolution.
- La **couverture territoriale** s'appuie sur le référentiel des territoires déjà utilisé par la
  plateforme, l'interface parlant de « territoire » là où le modèle de données parle de pays.
- **Aucune tâche de fond** n'est introduite : ce qui est à l'antenne se déduit à la consultation, comme
  la grille actuelle. Le rang d'occurrence se calcule à partir de la date d'entrée en vigueur du créneau,
  ce qui rend la rotation reproductible sans rien enregistrer. L'ajout d'un épisode périodique reste une
  action humaine du détenteur ; le système rappelle l'échéance mais ne génère aucun contenu.
- La **rotation** ne suppose aucune date de diffusion saisie par le détenteur : l'ordre des épisodes
  suffit. Un détenteur qui veut maîtriser l'épisode d'une date donnée agit sur cet ordre.
- La **modération systématique des épisodes** s'appuie sur la file de validation des médias déjà en
  place, élargie aux épisodes soumis par les co-détenteurs. Elle suppose une administration réactive :
  les alertes d'échéance sont calées en amont pour absorber ce délai, et un retard de modération ne
  coupe jamais l'antenne : la rotation continue sur les épisodes déjà validés.
- La **reprise de données** est réalisée en même temps que la livraison, sans période intermédiaire où
  les deux modèles coexisteraient. Les contenus repris sont considérés comme déjà validés.
- Les mécanismes existants de **détention de supports**, de **modération sur signalement**, d'**audit**
  et d'**attribution de points d'engagement** sont réutilisés tels quels et adaptés aux nouvelles cibles,
  sans redéfinition de leurs règles. Les interactions gagnant un second niveau de rattachement,
  l'émission devient une cible supplémentaire pour ces mécanismes, sans que leurs seuils changent.
- La **mise en avant** d'un contenu (par support et pour l'espace Télé) continue de désigner une unité
  diffusable, donc un épisode.
- Les pages publiques de l'espace Télé et Radio conservent leur identité visuelle actuelle ; cette
  évolution modifie ce qu'elles présentent, pas leur charte.
