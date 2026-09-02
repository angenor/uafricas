# Feature Specification: Système d'engagement / gamification AFRICANS, Phase 1 (fondation) + barème vérifiable

**Feature Branch**: `001-engagement-gamification`
**Created**: 2026-07-06
**Status**: Draft
**Input**: User description: "Système d'engagement / gamification AFRICANS, Phase 1 (fondation) + barème vérifiable. Moteur de points paramétrable, compte d'engagement par utilisateur (solde global, solde mensuel, réputation séparée, niveau dérivé), journal des points, table de règles paramétrables, attribution non-bloquante déclenchée par actions mesurables. Barème vérifiable côté serveur uniquement : contributions validées par modération, factcheck correct/faux, paliers de likes. Statuts Membre / Premium / Influenceur Platinum + badges. Écrans « Mes points » et back-office. Anti-abus (idempotence, plafonds, dédup). Hors périmètre : partages externes, quiz/jeux, cadeaux, publicité, dons."

## Contexte et cadrage

Cette spécification couvre **uniquement la Phase 1** du système d'engagement AFRICANS : la **fondation** (compte d'engagement, moteur de points paramétrable, journal, statuts/badges) et le **barème vérifiable côté serveur**. Elle pose les bases sur lesquelles s'appuieront les phases ultérieures.

**Explicitement hors périmètre de cette phase** (chantiers futurs distincts) :

- Points pour **partages sur réseaux sociaux externes** (nécessite un suivi de clics de boutons, invérifiable de manière fiable).
- Points liés aux **quiz, jeux et concours** (l'infrastructure ludique n'existe pas encore).
- **Cadeaux entre utilisateurs** (Gô, Boro, Digbate, Lass, Viemogo) et **cadeaux partenaires** (sites touristiques).
- **Publicité payante** et **module de dons / financement volontaire** (aucune passerelle de paiement n'existe).

Ces exclusions sont délibérées : la fondation doit être livrée et éprouvée avant d'y greffer récompenses et monétisation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Gagner des points pour une contribution validée (Priority: P1)

Un membre publie un contenu (récit ou proverbe Codimoi, sous-titres/traduction VidAfrica, idée Ideaforces, bonne pratique BadGoodHabit). Lorsqu'un modérateur **valide** cette contribution, le membre reçoit automatiquement des points d'engagement, qui augmentent son solde et peuvent faire évoluer son statut. Si l'équipe distingue la contribution comme **« mise en avant »**, le membre reçoit un bonus.

**Why this priority**: C'est le cœur du moteur : sans attribution de points sur une action réelle et vérifiable, aucun autre élément (solde, niveau, badge, classement) n'a de sens. C'est le plus petit incrément qui prouve la boucle complète « action mesurable → points → statut ».

**Independent Test**: Créer une contribution, la faire valider en modération, puis vérifier que le solde du membre a augmenté du bon montant, qu'une ligne apparaît dans son journal de points, et que valider deux fois la même contribution n'attribue pas les points en double.

**Acceptance Scenarios**:

1. **Given** un membre avec 0 point et une contribution Codimoi en attente, **When** un modérateur valide la contribution, **Then** le solde du membre augmente du montant « contribution standard » et une entrée horodatée apparaît dans son journal.
2. **Given** une contribution déjà validée ayant déjà donné des points, **When** un modérateur re-déclenche la validation (ou une action équivalente), **Then** aucun point supplémentaire n'est attribué (idempotence).
3. **Given** une contribution validée, **When** l'équipe la marque « mise en avant », **Then** le membre reçoit le bonus « mise en avant » (en supplément ou en remplacement selon la règle configurée), tracé dans le journal.
4. **Given** un membre auteur d'une contribution disposant de droits de modération, **When** il valide ou met en avant sa propre contribution, **Then** aucun point n'est attribué (interdiction d'auto-attribution).

---

### User Story 2 - Consulter mes points, mon statut et mes badges (Priority: P1)

Un membre connecté ouvre son profil et voit son **solde de points** (global et du mois en cours), son **niveau/statut** (Membre, Premium, Influenceur Platinum), son **score de réputation**, et l'**historique** des points gagnés/perdus avec le motif de chaque mouvement. Son **badge de statut** est visible sur son profil et sous ses contenus publiés.

**Why this priority**: La gamification n'a d'effet sur l'engagement que si l'utilisateur **voit** sa progression. Cette vue rend le moteur perceptible et motivant ; sans elle, les points sont invisibles et inutiles.

**Independent Test**: Se connecter avec un membre ayant un historique de points, ouvrir la vue « Mes points » et vérifier l'exactitude du solde, du statut affiché, du badge, et de la liste des mouvements ; vérifier que le badge apparaît aussi sous un contenu publié par ce membre.

**Acceptance Scenarios**:

1. **Given** un membre avec 250 points, **When** il ouvre sa vue « Mes points », **Then** il voit son solde global (250), son solde mensuel, le statut « Premium » et le badge associé.
2. **Given** un membre franchissant un seuil de niveau, **When** son solde passe la valeur seuil, **Then** son statut affiché et son badge reflètent le nouveau niveau sans action manuelle.
3. **Given** un membre ayant gagné puis perdu des points, **When** il consulte son historique, **Then** il voit chaque mouvement (positif ou négatif) avec sa date, son motif et le solde résultant.
4. **Given** un visiteur consultant le profil public d'un membre, **When** la page se charge, **Then** le badge de statut du membre est visible mais le détail du journal reste privé (réservé au titulaire et aux administrateurs).

---

### User Story 3 - Gagner des points quand ma publication devient populaire (Priority: P2)

L'auteur d'une publication reçoit des points lorsque celle-ci franchit des **paliers de popularité** exprimés en nombre de « j'aime » (par ex. 100, 500, 1 000 likes). Chaque palier n'est récompensé **qu'une seule fois** par publication. Les paliers et les montants sont paramétrables.

**Why this priority**: Récompense la qualité perçue par la communauté plutôt que le simple volume de publication. Elle dépend d'un mécanisme d'agrégation des « j'aime » réparti sur des contenus de natures différentes, ce qui la rend plus complexe que l'US1 : d'où P2.

**Independent Test**: Faire monter le nombre de likes d'une publication au-delà d'un palier, vérifier que l'auteur reçoit les points de ce palier une seule fois, puis franchir le palier supérieur et vérifier l'attribution du palier suivant sans re-récompenser le précédent.

**Acceptance Scenarios**:

1. **Given** une publication de 99 likes dont l'auteur n'a reçu aucun point de popularité, **When** un 100ᵉ « j'aime » est enregistré, **Then** l'auteur reçoit les points du palier « 100 » une seule fois.
2. **Given** une publication ayant déjà déclenché le palier « 100 », **When** son nombre de likes redescend puis remonte au-dessus de 100, **Then** aucun point supplémentaire n'est attribué pour ce palier (non ré-attribuable).
3. **Given** une publication à 500 likes ayant déclenché « 100 » et « 500 », **When** elle atteint 1 000 likes, **Then** seul le palier « 1 000 » est nouvellement récompensé.
4. **Given** un membre qui « aime » sa propre publication, **When** ce « j'aime » est compté, **Then** il n'est pas pris en compte pour l'atteinte des paliers récompensés (anti-abus).

---

### User Story 4 - Gagner ou perdre des points via FactCheck (Priority: P2)

Un membre soumet une vérification (FactCheck). Si les modérateurs la jugent **correcte et la valident**, le membre gagne des points. Si la vérification est jugée **abusive / fausse après contrôle**, le membre **perd des points** et son **score de réputation** est dégradé.

**Why this priority**: Introduit la dimension négative (perte de points) et la **réputation** comme signal distinct du solde. Essentiel pour l'intégrité éditoriale, mais s'appuie sur le moteur de l'US1 : d'où P2.

**Independent Test**: Faire valider un factcheck et vérifier le gain de points ; faire juger un factcheck comme faux et vérifier la perte de points **et** la baisse de réputation, sans que le solde ne devienne incohérent.

**Acceptance Scenarios**:

1. **Given** un factcheck en attente, **When** un modérateur le juge correct et le valide, **Then** l'auteur gagne les points « factcheck validé » et une entrée apparaît dans son journal.
2. **Given** un factcheck en attente, **When** un modérateur le juge abusif/faux, **Then** l'auteur perd les points « factcheck faux » et son score de réputation diminue.
3. **Given** un membre dont le solde de points est inférieur au malus, **When** un malus « factcheck faux » s'applique, **Then** le solde est ajusté selon la règle de plancher définie (jamais un solde incohérent), et la réputation baisse indépendamment du solde.

---

### User Story 5 - Administrer le barème et auditer les points (Priority: P2)

Un administrateur configure, depuis le back-office, la **table des règles** (montant de chaque type d'action, seuils de niveaux, paliers de likes, plafonds anti-abus journaliers/mensuels) **sans intervention technique**, et consulte le **journal global** des points pour investiguer un litige ou un abus.

**Why this priority**: Le barème doit être ajustable sans redéploiement (exigence explicite : « éviter le hardcoding »). Nécessaire pour exploiter le système dans la durée, mais le moteur peut d'abord fonctionner avec des valeurs par défaut : d'où P2.

**Independent Test**: Modifier le montant d'une règle dans le back-office, déclencher l'action correspondante et vérifier que le nouveau montant s'applique ; ouvrir le journal global filtré par membre et voir la liste des mouvements.

**Acceptance Scenarios**:

1. **Given** un administrateur sur l'écran des règles, **When** il change le montant d'une action et enregistre, **Then** les attributions suivantes utilisent le nouveau montant et le changement est tracé dans l'audit.
2. **Given** un administrateur sur le journal global, **When** il filtre par membre et par période, **Then** il voit les mouvements correspondants avec type d'action, référence, points et date.
3. **Given** un administrateur, **When** il ajuste un plafond journalier, **Then** les gains d'un membre au-delà du nouveau plafond sont écrêtés le jour même.

---

### Edge Cases

- **Contribution validée puis annulée / supprimée / dé-publiée** : les points déjà attribués **ne sont pas repris** (FR-025) ; seul le malus « FactCheck faux » retire des points.
- **Franchissement de plusieurs paliers de likes d'un coup** (import massif, correction de compteur) : chaque palier franchi est récompensé une fois, sans doublon.
- **Solde qui repasse sous un seuil de niveau** après un malus : le statut est recalculé à la baisse, le badge reflète le niveau courant.
- **Plafond journalier/mensuel atteint** : les points au-delà du plafond ne sont pas crédités ; le journal indique l'écrêtage plutôt que d'ignorer silencieusement l'action.
- **Contenus / likes antérieurs au lancement** : **non récompensés** (FR-024), tous les comptes démarrent à zéro à la mise en service.
- **Auto-action** : aimer sa propre publication, valider/mettre en avant sa propre contribution → jamais de points.
- **Réinitialisation mensuelle** : le solde mensuel repart à zéro au changement de mois ; le solde global n'est jamais remis à zéro.
- **Panne du moteur d'attribution** : une action métier réussie (validation, like) ne doit jamais échouer parce que l'attribution de points a échoué (attribution non-bloquante) ; les points manqués sont rattrapables.

## Requirements *(mandatory)*

### Functional Requirements

**Compte d'engagement**

- **FR-001**: Le système DOIT tenir, pour chaque utilisateur, un compte d'engagement comportant un **solde de points global**, un **solde de points mensuel**, un **score de réputation** distinct du solde de points, et la date du dernier mouvement.
- **FR-002**: Le système DOIT dériver un **niveau/statut** à partir du solde de points global parmi au moins : **Membre**, **Premium**, **Influenceur Platinum**, selon des seuils **paramétrables**.
- **FR-003**: Le système DOIT maintenir le **solde de réputation séparément** du solde de points : la réputation N'EST PAS dépensable et n'entre pas dans le calcul du niveau (sauf décision produit contraire ultérieure).
- **FR-004**: Le système DOIT réinitialiser le **solde mensuel** au début de chaque mois sans affecter le solde global.

**Journal et traçabilité**

- **FR-005**: Le système DOIT enregistrer chaque mouvement de points dans un **journal** comportant au minimum : utilisateur concerné, type d'action, référence de l'objet source, nombre de points (positif ou négatif), impact éventuel sur la réputation, et date.
- **FR-006**: Le système DOIT permettre au titulaire de consulter **son propre** historique de points, et aux administrateurs de consulter le **journal global** filtrable par membre et par période.

**Moteur d'attribution**

- **FR-007**: Le système DOIT attribuer des points de manière **non-bloquante** : l'échec de l'attribution ne DOIT jamais faire échouer l'action métier déclencheuse (validation de contribution, enregistrement d'un like, jugement de factcheck).
- **FR-008**: Le système DOIT garantir l'**idempotence** de l'attribution : une même action identifiée par (type d'action, référence) ne peut créditer/débiter les points qu'une seule fois.
- **FR-009**: Le système DOIT **interdire l'auto-attribution** : un utilisateur ne gagne pas de points pour des actions qu'il déclenche sur son propre contenu (auto-like, auto-validation, auto-mise en avant).
- **FR-010**: Le système DOIT appliquer des **plafonds anti-abus paramétrables** par type d'action (au moins journalier et mensuel) et **écrêter** les gains au-delà du plafond, en traçant l'écrêtage.

**Barème vérifiable (Phase 1)**

- **FR-011**: Le système DOIT créditer l'auteur d'une **contribution validée par modération** dans Codimoi, VidAfrica, Ideaforces et BadGoodHabit d'un montant « contribution standard » paramétrable (valeur indicative : +2).
- **FR-012**: Le système DOIT créditer un bonus « **mise en avant** » paramétrable (valeur indicative : +5) lorsqu'une contribution est distinguée par l'équipe AFRICANS.
- **FR-013**: Le système DOIT créditer l'auteur d'un **FactCheck jugé correct et validé** d'un montant paramétrable (valeur indicative : +3).
- **FR-014**: Le système DOIT débiter l'auteur d'un **FactCheck jugé abusif/faux après contrôle** d'un montant paramétrable (valeur indicative : −2) **et** dégrader son **score de réputation**.
- **FR-015**: Le système DOIT créditer l'auteur d'une **publication** lorsqu'elle franchit un **palier de popularité** exprimé en nombre de « j'aime », selon des paliers **paramétrables** (valeurs indicatives : 100 → +10, 500 → +30, 1 000 → +50).
- **FR-016**: Le système DOIT ne récompenser **chaque palier de popularité qu'une seule fois par publication**, sans ré-attribution si le compteur de likes redescend puis remonte.
- **FR-017**: Le système DOIT calculer la popularité d'une publication en **agrégeant les « j'aime » à travers les différents types de contenus** de la plateforme via une **référence unifiée (type d'objet + identifiant d'objet)**, en ne comptant que les réactions positives et en excluant l'auto-like.

**Statuts, badges et visibilité**

- **FR-018**: Le système DOIT afficher, dans une vue **« Mes points / mon statut / mes badges »** du profil, le solde global, le solde mensuel, la réputation, le niveau et l'historique des mouvements.
- **FR-019**: Le système DOIT afficher le **badge de statut** d'un membre sur son **profil public** et **sous les contenus** qu'il publie, sans exposer le détail privé de son journal.
- **FR-020**: Le système DOIT recalculer le niveau et le badge **automatiquement** dès qu'un mouvement de points fait franchir (à la hausse ou à la baisse) un seuil de niveau.
- **FR-021**: En Phase 1, les statuts Premium/Platinum se traduisent **uniquement par des badges visuels** ; l'**impact algorithmique sur la visibilité** (priorisation dans les fils, slots « à la une », poids dans le ranking) est **explicitement reporté** à une phase ultérieure et N'EST PAS implémenté ici.
- **FR-024**: Le système NE DOIT PAS attribuer de points **rétroactivement** au lancement : seules les actions récompensables **postérieures** à la mise en service génèrent des points ; les contributions validées et les « j'aime » antérieurs ne créditent aucun point (tous les comptes démarrent à zéro).
- **FR-025**: Le système NE DOIT PAS **reprendre automatiquement** les points déjà attribués lorsqu'une contribution est ensuite supprimée, dé-publiée ou re-jugée. **Seule exception** : le malus « FactCheck faux » (FR-014), qui est une règle de barème explicite. Les points acquis restent acquis.

**Administration**

- **FR-022**: Le système DOIT permettre à un administrateur de **configurer le barème** (montants par type d'action, seuils de niveaux, paliers de popularité, plafonds) via une **table de règles paramétrable**, sans intervention technique ni redéploiement.
- **FR-023**: Le système DOIT **tracer dans l'audit** toute modification du barème et toute correction manuelle de points effectuée par un administrateur.

### Key Entities *(include if feature involves data)*

- **Compte d'engagement** : représente le capital d'engagement d'un utilisateur. Attributs : solde de points global, solde de points mensuel, score de réputation, niveau/statut courant (dérivé), date du dernier mouvement. Relation : 1–1 avec un utilisateur.
- **Mouvement de points (journal)** : trace immuable d'un gain ou d'une perte. Attributs : utilisateur, type d'action, référence de l'objet source (type d'objet + identifiant), points (signés), impact réputation, date. Relation : N–1 avec le compte d'engagement.
- **Règle de barème** : paramètre configurable d'une action donnant/retirant des points. Attributs : type d'action, montant de points, montant de réputation, plafond journalier, plafond mensuel, actif/inactif. Utilisée par le moteur au moment de l'attribution.
- **Palier de popularité** : seuil de « j'aime » d'une publication déclenchant une récompense unique. Attributs : seuil (nombre de likes), points, actif/inactif ; et l'état « palier déjà récompensé » par publication.
- **Seuil de niveau** : borne de solde définissant un statut (Membre / Premium / Influenceur Platinum). Attributs : nom du niveau, seuil d'entrée, description du badge/avantage.
- **Publication (référence unifiée)** : abstraction (type d'objet + identifiant) désignant tout contenu susceptible de recevoir des « j'aime » et d'avoir un auteur, quel que soit le domaine d'origine (Codimoi, FactCheck, bibliothèque humaine, vidéo, fiche pays…).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Lorsqu'une action récompensable est finalisée (contribution validée, palier de likes franchi, factcheck jugé), le solde de l'utilisateur reflète le mouvement en **moins de 5 secondes** dans sa vue « Mes points ».
- **SC-002**: **100 %** des mouvements de points sont traçables dans le journal avec type d'action, référence, montant et date (aucun point « fantôme »).
- **SC-003**: Aucune action métier déclencheuse (validation, like, jugement) n'échoue à cause du moteur de points : **taux d'échec métier imputable à l'engagement = 0 %**.
- **SC-004**: Rejouer une même action récompensable (double validation, oscillation de likes autour d'un palier) n'attribue **jamais** de points en double : **0 doublon** constaté sur un jeu de tests d'idempotence.
- **SC-005**: Un administrateur peut modifier un montant du barème et constater son application sur une nouvelle action **sans redéploiement** et en **moins de 2 minutes**.
- **SC-006**: Le franchissement d'un seuil de niveau met à jour le statut et le badge affichés **sans action manuelle** de l'utilisateur, visible dès le rechargement de la vue profil.
- **SC-007**: Un utilisateur atteignant/dépassant un plafond journalier ne se voit créditer **aucun point au-delà du plafond** ce jour-là, l'écrêtage étant visible dans son journal.

## Assumptions

- La **réputation** est un score **distinct et non dépensable** ; en Phase 1 elle est affichée et alimentée (notamment par le factcheck faux) mais ne conditionne aucun avantage ni aucun calcul de niveau. (Décision produit à confirmer.)
- Les **seuils de niveaux** retenus par défaut sont : Membre **0–199**, Premium **≥ 200**, Influenceur Platinum **≥ 1 000**, **paramétrables**. Valeurs indicatives issues de la description, à figer par le produit.
- Les **montants du barème** (contribution +2, mise en avant +5, factcheck +3 / −2, paliers 100/500/1000 → 10/30/50) sont **indicatifs et paramétrables** ; la calibration finale relève de la stratégie éditoriale.
- Le **solde de points ne descend pas en dessous de zéro** par défaut (plancher à 0) ; la réputation peut, elle, être négative (à confirmer au design).
- La **popularité** ne compte que les **réactions positives** (« j'aime »), pas les réactions négatives, et exclut l'auto-like.
- Seules les règles **vérifiables côté serveur** sont câblées en Phase 1 ; les partages externes, quiz et concours restent hors périmètre.
- Les badges sont **symboliques/visuels** en Phase 1 (aucun avantage transactionnel comme cadeaux ou visibilité payante).

## Décisions produit (tranchées)

1. **Impact algorithmique de la visibilité** (Premium/Platinum) : **reporté**, Phase 1 = badges visuels uniquement (FR-021).
2. **Rétroactivité au lancement** : **non rétroactif**, tous les comptes démarrent à zéro (FR-024).
3. **Reprise de points (clawback)** : **pas de reprise automatique**, sauf le malus « FactCheck faux » (FR-025).
4. **Séparation points dépensables ↔ réputation non dépensable** : oui, séparés (FR-003).
5. **Seuils de niveaux et calibration du barème** : valeurs indicatives, **paramétrables** en base (FR-002, FR-022) ; calibration fine confiée au produit/éditorial et ajustable sans redéploiement.
