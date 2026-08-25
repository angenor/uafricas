# Feature Specification : Africanité : publications éphémères

**Feature Branch** : `012-africanite-ephemere`

**Created** : 2026-08-25

**Status** : Draft

**Input** : « Africanité : publications éphémères en tête du fil d'actualité. En tête de la page `/` (fil d'actualité), une rangée de pastilles rondes : d'abord un cercle « + Africanité » pour publier la sienne, puis les ami(e)s ayant une publication active. L'anneau de chaque pastille distingue « pas encore regardée » de « déjà vue ». Cliquer ouvre une visionneuse plein écran qui enchaîne les publications d'un auteur, puis passe à l'auteur suivant. »

## Contexte

La maquette (cadre Figma « Desktop », nœuds `1:8152` et `1:8162`) ne dessine que la rangée : un cercle, une icône, le libellé « Africanité ». Aucun autre écran n'en montre le contenu, et **rien n'existe côté produit** : c'est une fonctionnalité neuve, pas un habillage à porter.

La plateforme n'a jamais eu de contenu éphémère. Tout ce qu'un membre y publie : publication Codimoi, avis de recherche, contribution citoyenne, partage, reste consultable indéfiniment. Une africanité inverse cette règle : elle est faite pour disparaître. C'est ce renversement, et non l'affichage en pastilles rondes, qui fait la difficulté de cette feature : il touche la rétention des données, la modération et la vie privée.

## User Scenarios & Testing *(mandatory)*

### User Story 1 : Publier une africanité (Priority : P1)

Un membre veut partager un moment de son quotidien africain, un plat, un tissu, une rue, sans que cela s'ajoute définitivement à son mur. Il clique le cercle « + Africanité » en tête du fil, choisit sa forme (une image, une vidéo courte, ou quelques mots sur un fond coloré), y ajoute éventuellement une légende, et publie. Sa pastille apparaît aussitôt en tête de rangée.

**Why this priority** : sans publication, il n'y a rien à regarder. C'est la seule histoire qui, livrée seule, produit déjà quelque chose d'observable, un membre voit sa propre africanité et la voit disparaître au bout du temps imparti.

**Independent Test** : publier une africanité, vérifier qu'elle apparaît en tête de rangée pour son auteur, puis vérifier qu'elle a disparu après le délai de vie.

**Acceptance Scenarios** :

1. **Given** un membre connecté sur le fil d'actualité, **When** il publie via le cercle « + Africanité », sous l'une quelconque des trois formes, **Then** sa pastille apparaît immédiatement en première position de la rangée.
2. **Given** une africanité publiée il y a plus que sa durée de vie, **When** un membre charge le fil, **Then** cette africanité n'apparaît plus, ni dans la rangée ni dans la visionneuse.
3. **Given** un visiteur non connecté, **When** il consulte le fil, **Then** le cercle de publication l'invite à se connecter et ne présente aucun formulaire.
4. **Given** un membre qui a déjà une africanité active, **When** il en publie une seconde, **Then** les deux coexistent sous une seule pastille, dans l'ordre de publication.

---

### User Story 2 : Regarder les africanités de ses ami(e)s (Priority : P1)

Un membre voit en tête de fil les pastilles des personnes ayant une africanité active. L'anneau lui dit lesquelles il n'a pas encore regardées. Il clique, la visionneuse s'ouvre en plein écran, enchaîne les africanités de cet auteur, puis passe à l'auteur suivant. Il peut avancer, reculer, ou fermer à tout moment.

**Why this priority** : publier sans lecteur n'a aucune valeur. P1 avec l'histoire 1 : les deux forment le minimum viable.

**Independent Test** : avec deux comptes, publier depuis l'un, vérifier depuis l'autre que la pastille apparaît avec un anneau « non vue », l'ouvrir, et vérifier que l'anneau change d'état.

**Acceptance Scenarios** :

1. **Given** une africanité qu'il n'a pas encore ouverte, **When** le membre regarde la rangée, **Then** l'anneau de la pastille la signale comme non vue.
2. **Given** une africanité déjà ouverte, **When** le membre recharge le fil, **Then** l'anneau la signale comme vue, et elle passe après les non vues dans l'ordre de la rangée.
3. **Given** la visionneuse ouverte sur la dernière africanité d'un auteur, **When** le membre avance, **Then** la visionneuse passe à la première africanité de l'auteur suivant.
4. **Given** la visionneuse ouverte sur la dernière africanité du dernier auteur, **When** le membre avance, **Then** la visionneuse se ferme et rend le fil.
5. **Given** la visionneuse ouverte, **When** le membre appuie sur Échap ou clique la croix, **Then** elle se ferme et l'état « vue » des africanités déjà affichées est conservé.

---

### User Story 3 : Retirer son africanité avant l'heure (Priority : P2)

Un membre regrette ce qu'il a publié. Depuis la visionneuse, sur sa propre africanité, il la supprime. Elle disparaît immédiatement pour tout le monde.

**Why this priority** : un contenu qu'on ne peut pas retirer avant sa disparition automatique est un piège. Ce n'est pas le minimum viable, mais c'est le premier ajout indispensable.

**Independent Test** : publier, supprimer, vérifier depuis un second compte que la pastille a disparu sans attendre l'expiration.

**Acceptance Scenarios** :

1. **Given** un membre regardant sa propre africanité, **When** il la supprime, **Then** elle disparaît de sa rangée et de celle des autres au chargement suivant.
2. **Given** un membre regardant l'africanité d'autrui, **When** il ouvre les actions, **Then** aucune option de suppression ne lui est proposée.

---

### User Story 4 : Savoir qui a regardé (Priority : P3)

L'auteur d'une africanité veut savoir qui l'a vue. Sur sa propre africanité, la visionneuse affiche le nombre de vues et la liste des personnes.

**Why this priority** : c'est ce qui donne son sens social au format, mais rien ne casse sans. Livrable après coup.

**Independent Test** : publier depuis un compte, ouvrir depuis deux autres, vérifier que l'auteur voit deux noms.

**Acceptance Scenarios** :

1. **Given** une africanité vue par trois membres, **When** son auteur l'ouvre, **Then** il voit le décompte et la liste des trois.
2. **Given** une africanité qu'un membre regarde plusieurs fois, **When** l'auteur consulte la liste, **Then** ce membre n'y figure qu'une fois.

---

### User Story 5 : Signaler une africanité (Priority : P2)

Un membre tombe sur une africanité choquante. Il la signale depuis la visionneuse. Le contenu part en modération.

**Why this priority** : un format éphémère est précisément celui où un contenu abusif a le plus de chances d'échapper à toute revue. Le signalement doit exister dès la mise en ligne du format, pas après.

**Independent Test** : signaler une africanité depuis un compte, vérifier qu'elle remonte dans la file de modération administrative.

**Acceptance Scenarios** :

1. **Given** une africanité affichée, **When** un membre la signale, **Then** un accusé lui est rendu et le signalement est enregistré.
2. **Given** une africanité signalée, **When** un administrateur consulte la file de modération, **Then** elle y figure avec son média, son auteur et son motif, **y compris après l'heure d'expiration**, le signalement ayant gelé la destruction du média.
3. **Given** une africanité que le même membre signale deux fois, **When** la file est consultée, **Then** un seul signalement est compté.

---

### Edge Cases

- **Aucune africanité active** : la rangée n'affiche que le cercle de publication, sans espace vide ni pastille fantôme.
- **Africanité expirant pendant la lecture** : la visionneuse ouverte poursuit la lecture en cours plutôt que de se fermer brutalement ; le rechargement suivant ne la montre plus.
- **Auteur dont l'amitié est rompue** pendant qu'une africanité est active : elle cesse d'être visible dès la rupture.
- **Auteur dont le compte est supprimé ou suspendu** : ses africanités cessent immédiatement d'être visibles.
- **Média illisible ou trop lourd** : la publication est refusée avec un message qui dit la limite dépassée, avant tout envoi complet du fichier.
- **Membre sans ami(e)s** : la rangée n'affiche que son propre cercle, et un texte dit où trouver des membres.
- **Deux onglets ouverts** : marquer une africanité comme vue dans l'un n'exige pas de recharger l'autre pour publier ou lire.
- **Signalement après destruction** : la modération reçoit le signalement et voit que le média a disparu, plutôt qu'un écran vide ou une erreur.
- **Vidéo plus longue que la limite** : le refus dit la durée mesurée et la durée admise, sans avoir attendu l'envoi complet du fichier.
- **Texte sur fond coloré sans texte** : la publication est refusée, un fond coloré nu ne dit rien.
- **Amitié rompue pendant la lecture** : la visionneuse ouverte poursuit ce qui est déjà chargé ; le rechargement suivant ne montre plus rien de cet auteur.

## Requirements *(mandatory)*

### Functional Requirements

**Publication**

- **FR-001** : Un membre authentifié DOIT pouvoir publier une africanité sous l'une de trois formes : une image, une vidéo courte, ou un texte posé sur un fond coloré de son choix.
- **FR-001a** : Une africanité DOIT porter une seule forme ; les trois ne se combinent pas.
- **FR-001b** : La forme « texte sur fond coloré » NE DOIT exiger aucun dépôt de fichier, et son texte DOIT être borné en longueur pour rester lisible en un écran.
- **FR-002** : Le système DOIT refuser toute publication dont le fichier dépasse les limites de format, de poids ou, pour la vidéo, de durée, en indiquant laquelle est dépassée et avant tout envoi complet du fichier.
- **FR-003** : Une africanité DOIT porter une échéance fixée à sa création, au-delà de laquelle elle cesse d'être servie.
- **FR-004** : Un visiteur non authentifié NE DOIT PAS pouvoir publier ; l'entrée de publication DOIT le mener à la connexion.
- **FR-005** : Un membre DOIT pouvoir publier plusieurs africanités actives simultanément ; elles se regroupent sous une seule pastille à son nom.

**Consultation**

- **FR-006** : Le système DOIT présenter, en tête du fil d'actualité, une rangée des **ami(e)s du lecteur** ayant au moins une africanité active, précédée du cercle de publication du lecteur.
- **FR-006a** : Une africanité NE DOIT être servie qu'à son auteur et aux membres liés à lui par une amitié en vigueur **à l'instant de la lecture**.
- **FR-006b** : Un lecteur sans ami(e)s DOIT voir son seul cercle de publication, accompagné d'une sortie vers l'annuaire des membres.
- **FR-007** : Le système NE DOIT JAMAIS servir une africanité échue à un membre, quelle que soit la voie d'accès, son auteur compris.
- **FR-008** : La rangée DOIT distinguer visuellement les auteurs dont le lecteur n'a pas tout vu de ceux dont il a tout vu, et présenter les premiers en tête.
- **FR-009** : L'ouverture d'une pastille DOIT enchaîner les africanités de son auteur dans l'ordre de publication, puis passer à l'auteur suivant de la rangée.
- **FR-010** : Le lecteur DOIT pouvoir avancer, reculer et fermer à tout moment ; la fermeture DOIT conserver ce qui a été marqué comme vu.
- **FR-011** : Le système DOIT enregistrer qu'un lecteur a vu une africanité, une seule fois par lecteur et par africanité.
- **FR-012** : Une africanité DOIT cesser d'être visible d'un lecteur dès que l'amitié qui l'y donnait droit est rompue, ou dès que le compte de l'auteur est suspendu ou supprimé, sans attendre l'échéance ni aucun traitement différé.

**Retrait et modération**

- **FR-013** : Un auteur DOIT pouvoir supprimer sa propre africanité avant son échéance ; elle cesse alors d'être servie immédiatement.
- **FR-014** : Un membre NE DOIT PAS pouvoir supprimer l'africanité d'autrui.
- **FR-015** : Tout membre authentifié DOIT pouvoir signaler une africanité qu'il consulte, une seule fois par africanité.
- **FR-016** : Une africanité signalée DOIT rester consultable par la modération administrative, avec son média, son auteur et le motif du signalement.
- **FR-017** : Un administrateur DOIT pouvoir retirer une africanité signalée avant son échéance.
- **FR-018** : Toute création, suppression et décision de modération DOIT être journalisée comme les autres mutations de la plateforme.
- **FR-018a** : Le média d'une africanité échue DOIT être détruit, et la destruction DOIT être constatable, un média échu mais encore présent est un état transitoire, pas un état stable.
- **FR-018b** : Un signalement en attente de décision DOIT **empêcher** la destruction du média de l'africanité qu'il vise, y compris après son échéance.
- **FR-018c** : Une fois la modération statuée, la destruction DOIT reprendre son cours.
- **FR-018d** : Un signalement portant sur une africanité déjà détruite DOIT être accepté et remonter en modération, en indiquant que le média n'est plus disponible, le refuser laisserait un abus sans trace.

**Vues**

- **FR-019** : L'auteur d'une africanité DOIT voir le nombre de lecteurs distincts et leur identité.
- **FR-020** : Un lecteur NE DOIT PAS voir qui d'autre a regardé une africanité dont il n'est pas l'auteur.

### Key Entities

- **Africanité** : une publication éphémère. Porte son auteur, sa **forme** (image, vidéo courte ou texte sur fond coloré), son contenu selon cette forme, une légende facultative, son instant de publication et son échéance. Traverse trois états : *active*, *échue* (invisible de tous, média encore présent), *détruite* (média effacé). Le passage d'active à échue ne déclenche rien : il se constate à la lecture.
- **Vue d'africanité** : le fait qu'un membre donné a regardé une africanité donnée. Unique par couple lecteur/africanité. Sert deux usages distincts : l'état de l'anneau côté lecteur, le décompte côté auteur.
- **Signalement d'africanité** : le fait qu'un membre a signalé une africanité, avec son motif. Unique par couple signaleur/africanité. Survit à l'échéance **et à la destruction** de l'africanité qu'il vise, sans quoi la modération n'aurait jamais rien à examiner. Tant qu'il est en attente, il gèle la destruction du média.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001** : Un membre publie une africanité en moins de 30 secondes, depuis le fil, sans changer de page.
- **SC-002** : Aucune africanité échue n'est jamais visible : sur un jeu de contrôle mêlant contenus actifs et échus, 100 % des échus sont absents de toutes les vues.
- **SC-003** : La rangée s'affiche en même temps que le reste du fil, sans retarder l'apparition des publications.
- **SC-004** : Un lecteur passe d'une africanité à la suivante sans attente perceptible sur une connexion mobile ordinaire.
- **SC-005** : Une africanité signalée reste examinable par la modération tant qu'aucune décision n'a été prise, y compris après sa disparition côté membres ; une africanité non signalée ne laisse plus aucun média sur le stockage passé son échéance.
- **SC-006** : Un auteur retire son africanité en moins de trois gestes depuis la visionneuse.
- **SC-007** : Aucun membre ne peut accéder à l'identité des lecteurs d'une africanité dont il n'est pas l'auteur.

## Clarifications

### Session 2026-08-25

- **Q1 : Qui voit l'africanité d'un membre ?** → **Ses ami(e)s uniquement.** Le droit de lecture se recalcule à chaque lecture depuis le lien d'amitié ; une rupture coupe l'accès aussitôt, sans traitement différé. Un membre sans ami(e)s voit une rangée réduite à son propre cercle, accompagnée d'une sortie vers l'annuaire.
- **Q2 : Que devient une africanité passée son heure ?** → **Détruite, sauf si signalée.** Trois états se distinguent désormais : *active* (visible), *échue* (invisible de tous, média encore présent), *détruite* (média effacé). Un signalement **gèle** le passage à l'état détruit tant que la modération n'a pas statué.
- **Q3 : Quels médias accepte une africanité ?** → **Image, vidéo courte ou texte sur fond coloré.** Les trois formes coexistent ; le texte sur fond coloré réemploie le vocabulaire déjà en place pour les proverbes Codimoi et n'exige aucun dépôt de fichier.

## Assumptions

Ces choix ont été faits faute de précision, sur la base des usages courants et des habitudes déjà prises par la plateforme. Ils sont à confirmer ou à corriger.

- **Durée de vie de 24 heures** à compter de la publication, la convention du format, et un repère que les membres connaissent sans qu'on le leur explique.
- **Légende facultative et courte**, sans mise en forme ni lien, pour que l'image reste le propos.
- **L'auteur voit ses lecteurs** (FR-019) : c'est l'usage du format, et le lecteur en est informé avant d'ouvrir.
- **Une africanité échue n'est plus consultable par personne, pas même son auteur** ; s'il veut la garder, il garde son fichier. La modération fait exception, et seulement sur signalement.
- **Aucune notification** n'est émise à la publication d'une africanité : le format vit de la consultation du fil, et notifier chaque publication de chaque ami(e) serait une source de bruit.
- **Pas de réponse ni de réaction** à une africanité dans cette version : ce serait un fil de discussion éphémère, soit une feature de plus.
- **La rangée n'est pas paginée** : au-delà d'une poignée d'auteurs, elle défile horizontalement.
- **L'expiration est constatée à la lecture**, sans traitement déclenché à l'heure dite : c'est le mécanisme déjà retenu ailleurs sur la plateforme pour les échéances.
- **Le signalement réutilise le mécanisme existant** plutôt que d'en inventer un pour ce format.
- **Périmètre exclu de cette version** : archives personnelles des africanités passées, mise en avant d'une africanité sur le profil, africanités des salles ou des modules (seuls les membres en publient), et diffusion hors de la plateforme.

## Dépendances

- Le lien d'amitié entre membres, et son état à l'instant de la lecture, le droit de lecture en dépend entièrement.
- Le stockage et le service des fichiers déposés par les membres, déjà en place pour les photos de contribution.
- La file de modération administrative et le journal d'audit, déjà en service pour les autres contenus signalables.
- Le fil d'actualité de la page d'accueil, où la rangée prend place, livré, et qui sert déjà neuf sources.
