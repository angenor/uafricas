# Feature Specification: Édition Interactive de l'Arbre Généalogique

**Feature Branch**: `001-edition-arbre`
**Created**: 2026-03-16
**Status**: Draft
**Input**: User description: "Feature 3 — Édition interactive de l'arbre. Ajouter/modifier/supprimer des membres depuis la vue arbre (clic pour ajouter un parent, un enfant, un conjoint). Formulaire guidé pour ne pas créer de liens incohérents (ex : quelqu'un ne peut pas être son propre ancêtre). Indicateur de complétude (branches incomplètes où il manque des parents)."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Ajouter un membre depuis la vue arbre (Priority: P1)

L'utilisateur connecté, depuis la page de visualisation de son arbre, clique sur un nœud (personne existante) puis choisit une action contextuelle : « Ajouter un parent », « Ajouter un enfant » ou « Ajouter un conjoint ». Un formulaire guidé s'ouvre pour saisir les informations de la nouvelle personne. À la validation, la nouvelle personne et son lien familial sont créés simultanément, et l'arbre se met à jour en temps réel sans rechargement de page.

**Why this priority**: C'est la fonctionnalité centrale de cette feature — sans elle, l'utilisateur doit quitter la visualisation pour ajouter des membres, ce qui casse le flux d'exploration. L'ajout contextuel depuis l'arbre est le principal gain d'ergonomie.

**Independent Test**: Depuis la vue arbre avec au moins 1 personne, cliquer sur un nœud, choisir « Ajouter un enfant », remplir le formulaire, valider → la nouvelle personne apparaît dans l'arbre avec le lien correct.

**Acceptance Scenarios**:

1. **Given** un arbre avec une personne P visible, **When** l'utilisateur clique sur P puis choisit « Ajouter un enfant », **Then** un formulaire de création s'ouvre avec le type de lien pré-rempli (parent→enfant).
2. **Given** le formulaire d'ajout ouvert, **When** l'utilisateur saisit le nom et valide, **Then** la nouvelle personne apparaît dans l'arbre reliée à P, sans rechargement de page.
3. **Given** un nœud P sélectionné, **When** l'utilisateur choisit « Ajouter un parent », **Then** le formulaire s'ouvre et le lien est pré-configuré (nouvelle personne = parent de P).
4. **Given** un nœud P sélectionné, **When** l'utilisateur choisit « Ajouter un conjoint », **Then** le formulaire s'ouvre et le lien conjoint est pré-configuré.
5. **Given** le formulaire d'ajout ouvert dans le panneau latéral, **When** l'utilisateur clique sur « Annuler » ou « Retour », **Then** le panneau revient à l'affichage de la mini-fiche sans modification de l'arbre.

---

### User Story 2 - Validation guidée des liens (Priority: P1)

Le système empêche la création de liens familiaux incohérents. Lorsque l'utilisateur tente de créer un lien qui introduirait un cycle (ex : une personne serait son propre ancêtre) ou un doublon, un message d'erreur clair explique le problème. Le formulaire guide l'utilisateur en filtrant les types de liens disponibles selon le contexte (ex : si la personne a déjà 2 parents de genres différents, ne pas proposer un 3ème parent de même type).

**Why this priority**: Sans validation guidée, l'utilisateur pourrait créer des arbres incohérents, ce qui compromettrait la fiabilité des données et la lisibilité de l'arbre. C'est inséparable de US1.

**Independent Test**: Tenter de créer un lien circulaire (A parent de B, B parent de C, C parent de A) → le système bloque avec un message explicite.

**Acceptance Scenarios**:

1. **Given** un arbre où A est parent de B, **When** l'utilisateur tente de faire B parent de A, **Then** le système affiche une erreur « Lien circulaire détecté » et empêche la création.
2. **Given** une personne P ayant déjà un père et une mère, **When** l'utilisateur ouvre « Ajouter un parent » pour P, **Then** le type « parent (générique) » est le seul proposé (père et mère déjà pris).
3. **Given** un lien conjoint existant entre A et B, **When** l'utilisateur tente de recréer le même lien, **Then** le système affiche « Ce lien existe déjà ».
4. **Given** le formulaire d'ajout de parent pour une personne P, **When** le système détecte que P a déjà 2 parents, **Then** un avertissement indique que P a déjà 2 parents mais permet d'ajouter un parent supplémentaire (ex : parent adoptif).

---

### User Story 3 - Modifier une personne depuis la vue arbre (Priority: P2)

L'utilisateur peut modifier les informations d'une personne directement depuis la vue arbre, sans naviguer vers la page de détail. En cliquant sur un nœud et en choisissant « Modifier », un formulaire pré-rempli s'ouvre permettant de modifier le nom, les prénoms, le genre, les dates et lieux de naissance/décès. Les modifications sont reflétées instantanément dans le nœud de l'arbre.

**Why this priority**: Complémentaire à l'ajout, la modification en place évite les allers-retours entre pages. Priorité P2 car l'utilisateur peut toujours modifier via la page détail existante (Feature 1).

**Independent Test**: Cliquer sur un nœud, choisir « Modifier », changer le prénom, valider → le nœud affiche immédiatement le nouveau prénom.

**Acceptance Scenarios**:

1. **Given** un nœud P dans l'arbre, **When** l'utilisateur clique sur P puis choisit « Modifier », **Then** un formulaire s'ouvre pré-rempli avec toutes les informations actuelles de P.
2. **Given** le formulaire de modification ouvert, **When** l'utilisateur modifie le prénom et valide, **Then** le nœud P se met à jour instantanément avec le nouveau prénom.
3. **Given** le formulaire de modification ouvert, **When** l'utilisateur entre une date de décès antérieure à la date de naissance, **Then** une erreur de validation s'affiche.

---

### User Story 4 - Supprimer une personne depuis la vue arbre (Priority: P2)

L'utilisateur peut supprimer une personne de son arbre directement depuis la vue arbre. En cliquant sur un nœud et en choisissant « Supprimer », une confirmation s'affiche indiquant les conséquences (suppression en cascade des liens familiaux associés). Après confirmation, la personne et ses liens disparaissent de l'arbre.

**Why this priority**: Nécessaire pour corriger les erreurs de saisie, mais priorité P2 car la suppression est déjà possible via la page détail existante (Feature 1).

**Independent Test**: Cliquer sur un nœud, choisir « Supprimer », confirmer → le nœud et ses liens disparaissent de l'arbre.

**Acceptance Scenarios**:

1. **Given** un nœud P dans l'arbre avec 2 liens, **When** l'utilisateur choisit « Supprimer » sur P, **Then** une confirmation s'affiche indiquant « Cette personne sera retirée de l'arbre. 2 liens familiaux seront supprimés. »
2. **Given** la confirmation affichée, **When** l'utilisateur confirme, **Then** le nœud P et ses liens disparaissent de l'arbre sans rechargement.
3. **Given** la confirmation affichée, **When** l'utilisateur annule, **Then** rien ne change.
4. **Given** un arbre avec une seule personne, **When** l'utilisateur la supprime, **Then** l'arbre affiche l'état vide avec l'invitation à ajouter une personne.

---

### User Story 5 - Indicateur de branches incomplètes (Priority: P3)

L'arbre affiche des indicateurs visuels sur les personnes dont l'ascendance est incomplète (il manque un ou deux parents). Ces indicateurs permettent à l'utilisateur d'identifier rapidement les « branches à compléter » et de cliquer dessus pour ajouter les parents manquants directement.

**Why this priority**: Fonctionnalité de guidage qui enrichit l'expérience mais n'est pas bloquante pour l'utilisation de base. Elle encourage l'utilisateur à enrichir son arbre progressivement.

**Independent Test**: Avec un arbre contenant des personnes sans parents renseignés, vérifier que des indicateurs visuels apparaissent sur ces nœuds.

**Acceptance Scenarios**:

1. **Given** un arbre où la personne P n'a aucun parent renseigné, **When** l'arbre est affiché, **Then** le nœud P affiche un indicateur visuel (ex : badge ou icône) signalant « Parents manquants ».
2. **Given** une personne P ayant un seul parent (ex : père uniquement), **When** l'arbre est affiché, **Then** le nœud P affiche un indicateur « Mère manquante ».
3. **Given** un indicateur de branche incomplète sur P, **When** l'utilisateur clique sur l'indicateur, **Then** le formulaire d'ajout de parent s'ouvre pour P.
4. **Given** une personne P ayant ses 2 parents renseignés, **When** l'arbre est affiché, **Then** aucun indicateur d'incomplétude n'est affiché sur P.
5. **Given** un arbre entier, **When** l'utilisateur consulte la barre d'outils, **Then** un compteur indique le nombre total de branches incomplètes (ex : « 5 branches à compléter »).

---

### Edge Cases

- Que se passe-t-il si l'utilisateur tente d'ajouter un parent à une personne qui a déjà 3+ parents ? → Autorisé avec avertissement (familles recomposées, adoption).
- Que se passe-t-il si la connexion réseau est perdue pendant l'ajout ? → Message d'erreur « Connexion perdue. Veuillez réessayer. » Le formulaire reste ouvert avec les données saisies.
- Comment gérer l'ajout simultané par la même session (ex : double-clic rapide sur « Valider ») ? → Le bouton est désactivé après le premier clic pendant le traitement.
- Que se passe-t-il si l'utilisateur supprime une personne qui est le nœud actuellement centré dans la visualisation ? → L'arbre recentre automatiquement sur la première personne restante, ou affiche l'état vide si l'arbre est devenu vide.
- Comment réagit le formulaire si le nom saisi est identique à une personne existante dans l'arbre ? → Aucun blocage (homonymes courants dans les familles), mais un avertissement discret « Une personne portant le même nom existe déjà dans votre arbre ».

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT proposer un menu contextuel sur chaque nœud de l'arbre avec les actions : « Ajouter un parent », « Ajouter un enfant », « Ajouter un conjoint », « Modifier », « Supprimer ».
- **FR-002**: Le formulaire d'ajout DOIT pré-configurer le type de lien en fonction de l'action choisie (parent, enfant ou conjoint) et de la personne source.
- **FR-003**: Le système DOIT empêcher la création de liens circulaires (une personne ne peut pas être son propre ancêtre direct ou indirect) avec un message d'erreur explicite.
- **FR-004**: Le système DOIT empêcher la création de liens en doublon avec un message d'erreur explicite.
- **FR-005**: Le formulaire d'ajout de parent DOIT adapter les types de lien proposés en fonction des parents existants (ex : si un père existe déjà, proposer « mère » ou « parent » en priorité).
- **FR-006**: Après ajout, modification ou suppression, l'arbre DOIT se mettre à jour instantanément sans rechargement de page.
- **FR-007**: Le formulaire de modification DOIT être pré-rempli avec les informations actuelles de la personne.
- **FR-008**: La suppression DOIT afficher une confirmation détaillant les conséquences (nombre de liens qui seront supprimés).
- **FR-009**: Le système DOIT afficher un indicateur visuel sur les nœuds dont l'ascendance est incomplète (0 ou 1 parent renseigné).
- **FR-010**: L'indicateur d'incomplétude DOIT être cliquable et ouvrir directement le formulaire d'ajout de parent.
- **FR-011**: La barre d'outils DOIT afficher un compteur global du nombre de branches incomplètes dans l'arbre.
- **FR-012**: Le formulaire d'ajout DOIT valider la cohérence des dates (décès ≥ naissance) avant soumission.
- **FR-013**: Le bouton de validation du formulaire DOIT être désactivé pendant le traitement pour empêcher les soumissions multiples.
- **FR-014**: En cas d'erreur réseau, le formulaire DOIT rester ouvert avec les données saisies et afficher un message invitant à réessayer.
- **FR-015**: Le système DOIT afficher un avertissement discret si le nom saisi correspond à une personne existante dans l'arbre (détection d'homonymes).

### Key Entities

- **Menu contextuel** : Ensemble d'actions disponibles au clic sur un nœud de l'arbre. Actions possibles : ajouter parent/enfant/conjoint, modifier, supprimer. Apparaît à proximité du nœud cliqué.
- **Formulaire guidé** : Formulaire de création/modification adapté au contexte. Pré-configure le type de lien, filtre les options invalides, valide les données avant soumission.
- **Indicateur d'incomplétude** : Marqueur visuel sur un nœud signalant que la personne n'a pas tous ses parents renseignés. Cliquable pour déclencher l'ajout du parent manquant.
- **Compteur de branches incomplètes** : Nombre global affiché dans la barre d'outils, indiquant combien de personnes dans l'arbre ont une ascendance incomplète.

## Assumptions

- Cette feature s'appuie sur la page de visualisation existante (Feature 2 — `/arbre-genealogique/visualisation`). Le menu contextuel et les formulaires s'intègrent dans cette page.
- Les API CRUD existantes (Feature 1) sont utilisées pour créer, modifier et supprimer les personnes et liens. Aucun nouvel endpoint n'est nécessaire a priori.
- La détection de cycles est déjà implémentée côté backend (Feature 1). Le frontend affiche le message d'erreur retourné par l'API.
- Le formulaire de création dans l'arbre utilise les mêmes champs que le formulaire existant (Feature 1) : nom, prénoms, genre, dates et lieux de naissance/décès.
- Un parent manquant est défini comme : une personne ayant moins de 2 liens de type parent (père, mère, parent) entrant. C'est un calcul côté client basé sur les données de l'arbre complet.
- Le menu contextuel remplace ou complète le panneau contextuel (mini-fiche) existant de Feature 2. Les deux coexistent : la mini-fiche s'ouvre au premier clic, les boutons d'actions (ajouter parent/enfant/conjoint, modifier, supprimer) sont intégrés dans la mini-fiche.
- Le formulaire d'ajout/modification s'affiche dans le panneau latéral existant (desktop) / bottom sheet (mobile) en remplaçant le contenu de la mini-fiche. Un bouton « Retour » permet de revenir à la mini-fiche. Pas de modal overlay.

## Clarifications

### Session 2026-03-16

- Q: Où le formulaire d'ajout/modification apparaît-il (modal, panneau latéral, ou nouveau panneau) ? → A: Remplacer le contenu du panneau latéral/bottom sheet existant (mini-fiche → formulaire), avec bouton retour.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: L'utilisateur peut ajouter une personne et son lien familial depuis la vue arbre en moins de 30 secondes (du clic sur le nœud à la validation du formulaire).
- **SC-002**: 100% des tentatives de création de liens circulaires sont bloquées avec un message compréhensible avant soumission ou au retour serveur.
- **SC-003**: La mise à jour visuelle de l'arbre après ajout/modification/suppression se fait en moins de 1 seconde.
- **SC-004**: 90% des utilisateurs identifient et complètent au moins une branche incomplète lors de leur première session d'édition, guidés par les indicateurs visuels.
- **SC-005**: Le nombre moyen de personnes ajoutées par session augmente de 50% par rapport à l'ajout via la page liste (Feature 1), grâce au flux contextuel.
- **SC-006**: Aucune perte de données saisies en cas d'erreur réseau — le formulaire conserve les informations et permet de réessayer.
