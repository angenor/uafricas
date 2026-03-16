# Feature Specification: Visualisation et Navigation de l'Arbre Généalogique

**Feature Branch**: `001-visualisation-arbre`
**Created**: 2026-03-15
**Status**: Draft
**Input**: User description: "Feature 2 — Visualisation et navigation de l'arbre. Affichage graphique de l'arbre de l'utilisateur connecté. Navigation ascendante (ancêtres) et descendante (descendants). Vue centrée sur une personne avec ses liens directs. Responsive pour mobile. À ce stade, chaque utilisateur ne voit que son propre arbre."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Visualiser son arbre sous forme graphique (Priority: P1)

L'utilisateur connecté accède à la page dédiée de visualisation (`/arbre-genealogique/visualisation`) depuis la page index de son arbre, et voit un rendu graphique de l'ensemble de ses personnes et liens familiaux. Chaque personne est représentée par un nœud visuel (photo ou initiales, nom, dates de vie). Les liens familiaux (parent-enfant, conjoint) sont représentés par des connexions visuelles entre les nœuds. L'arbre s'affiche automatiquement centré sur la première personne ajoutée ou sur une personne choisie par défaut.

**Why this priority**: Sans visualisation graphique, la fonctionnalité n'a aucune valeur — c'est le cœur de cette feature. L'utilisateur a déjà une liste (Feature 1) ; ici il obtient enfin une représentation spatiale de sa famille.

**Independent Test**: Peut être testé en ajoutant 5-10 personnes avec des liens parent-enfant et conjoint via Feature 1, puis en accédant à la page de visualisation pour vérifier que toutes les personnes et connexions apparaissent correctement.

**Acceptance Scenarios**:

1. **Given** un utilisateur connecté avec un arbre contenant 8 personnes et 7 liens, **When** il accède à la page de visualisation, **Then** il voit les 8 nœuds reliés par des connexions visuelles reflétant les liens familiaux.
2. **Given** un utilisateur connecté sans arbre (aucune personne ajoutée), **When** il accède à la page de visualisation, **Then** il voit un message d'état vide l'invitant à ajouter sa première personne.
3. **Given** un arbre affiché graphiquement, **When** l'utilisateur observe un nœud, **Then** il peut voir la photo (ou initiales), le nom complet, et les années de naissance/décès de la personne.
4. **Given** un arbre avec des liens de type père, mère et conjoint, **When** l'arbre est affiché, **Then** les liens parent-enfant sont visuellement distincts des liens conjoint (orientation, couleur ou style différent).

---

### User Story 2 - Naviguer en vue centrée sur une personne (Priority: P1)

L'utilisateur peut cliquer sur n'importe quel nœud de l'arbre pour centrer la vue sur cette personne. La vue centrée affiche la personne sélectionnée au centre avec ses liens directs visibles : ses parents au-dessus, ses enfants en dessous, et ses conjoints à côté. Cela permet d'explorer l'arbre personne par personne.

**Why this priority**: La navigation centrée est essentielle pour explorer un arbre de plus de quelques personnes. Sans elle, l'arbre est statique et inexploitable pour les familles de taille moyenne à grande.

**Independent Test**: Peut être testé en cliquant sur différentes personnes dans l'arbre et en vérifiant que la vue se recentre correctement à chaque fois, affichant les liens directs de la personne sélectionnée.

**Acceptance Scenarios**:

1. **Given** un arbre affiché avec une personne P au centre, **When** l'utilisateur clique sur un autre nœud Q, **Then** la vue se recentre sur Q avec une transition fluide et un panneau contextuel s'ouvre affichant les informations clés de Q.
2. **Given** un panneau contextuel ouvert pour une personne P, **When** l'utilisateur clique sur « Voir détail », **Then** il est redirigé vers la page de fiche détail existante de P.
3. **Given** une vue centrée sur une personne P, **When** P a 2 parents, 3 enfants et 1 conjoint, **Then** tous ces liens directs sont visibles à l'écran autour de P.
3. **Given** une vue centrée sur une personne feuille (sans enfants), **When** l'utilisateur la consulte, **Then** seuls les parents et éventuels conjoints sont affichés, sans espace vide gênant en dessous.

---

### User Story 3 - Naviguer en mode ascendant (ancêtres) (Priority: P2)

L'utilisateur peut activer un mode de navigation ascendant qui affiche la lignée d'ancêtres à partir d'une personne sélectionnée. Ce mode montre les parents, grands-parents, arrière-grands-parents, etc., de façon hiérarchique vers le haut. Seuls les ancêtres directs sont affichés dans ce mode, simplifiant la lecture pour les généalogies profondes.

**Why this priority**: La navigation ascendante répond au besoin courant de retracer ses origines. C'est un mode de consultation complémentaire à la vue complète, particulièrement utile quand l'arbre est dense.

**Independent Test**: Peut être testé en sélectionnant une personne ayant au moins 3 générations d'ancêtres et en activant le mode ascendant pour vérifier que seuls les ancêtres directs sont affichés.

**Acceptance Scenarios**:

1. **Given** une personne P avec des ancêtres sur 4 générations, **When** l'utilisateur active le mode ascendant depuis P, **Then** il voit P, ses parents, ses grands-parents et ses arrière-grands-parents disposés hiérarchiquement.
2. **Given** le mode ascendant actif, **When** l'utilisateur clique sur un ancêtre A, **Then** la vue se recentre sur A et affiche les ancêtres de A.
3. **Given** une personne sans parents renseignés, **When** l'utilisateur active le mode ascendant, **Then** seule cette personne est affichée avec un indicateur qu'aucun ancêtre n'est renseigné.

---

### User Story 4 - Naviguer en mode descendant (descendants) (Priority: P2)

L'utilisateur peut activer un mode de navigation descendant qui affiche la descendance à partir d'une personne sélectionnée. Ce mode montre les enfants, petits-enfants, arrière-petits-enfants, etc. C'est le pendant du mode ascendant, utile pour visualiser la ramification familiale à partir d'un ancêtre fondateur.

**Why this priority**: Complémentaire au mode ascendant, la navigation descendante est essentielle pour explorer « vers le bas » de l'arbre. La priorité P2 reflète que la vue complète (P1) couvre déjà partiellement ce besoin.

**Independent Test**: Peut être testé en sélectionnant un ancêtre fondateur ayant une descendance sur 3+ générations et en activant le mode descendant pour vérifier l'affichage hiérarchique correct.

**Acceptance Scenarios**:

1. **Given** une personne P avec des descendants sur 3 générations, **When** l'utilisateur active le mode descendant depuis P, **Then** il voit P, ses enfants, ses petits-enfants disposés hiérarchiquement vers le bas.
2. **Given** le mode descendant actif depuis un ancêtre A avec 5 enfants, **When** l'arbre est affiché, **Then** les 5 branches descendantes sont visibles et navigables.
3. **Given** une personne sans enfants, **When** l'utilisateur active le mode descendant, **Then** seule cette personne est affichée avec un indicateur qu'aucun descendant n'est renseigné.

---

### User Story 5 - Utiliser la visualisation sur mobile (Priority: P2)

L'utilisateur accède à la visualisation de l'arbre depuis un appareil mobile (smartphone ou tablette). L'interface s'adapte à la taille d'écran réduite tout en conservant la lisibilité et la navigabilité. Les gestes tactiles (pincement pour zoomer, glisser pour déplacer) sont pris en charge.

**Why this priority**: Une part significative des utilisateurs africains accède au web principalement via mobile. La visualisation doit être fonctionnelle sur petit écran pour atteindre l'audience cible.

**Independent Test**: Peut être testé en accédant à la page de visualisation depuis un appareil mobile ou un émulateur, avec un arbre de 10+ personnes, et en vérifiant navigation et lisibilité.

**Acceptance Scenarios**:

1. **Given** un utilisateur sur smartphone (écran < 768px), **When** il accède à la visualisation de son arbre, **Then** l'arbre s'affiche dans une vue adaptée qui utilise toute la largeur disponible.
2. **Given** un arbre affiché sur mobile, **When** l'utilisateur pince l'écran, **Then** la vue zoome/dézoome de façon fluide.
3. **Given** un arbre affiché sur mobile, **When** l'utilisateur glisse avec le doigt, **Then** la vue se déplace (pan) pour explorer les parties non visibles de l'arbre.
4. **Given** un arbre sur mobile, **When** l'utilisateur tapote un nœud, **Then** la vue se centre sur cette personne et affiche ses liens directs.

---

### User Story 6 - Zoomer et se déplacer dans l'arbre (Priority: P3)

L'utilisateur peut zoomer et se déplacer librement dans la visualisation de l'arbre, y compris sur desktop. Cela permet d'avoir une vue d'ensemble (zoom arrière) ou de se concentrer sur un sous-ensemble de l'arbre (zoom avant). Un bouton de réinitialisation permet de revenir à la vue initiale.

**Why this priority**: Le zoom/pan améliore l'expérience mais n'est critique que pour les arbres de grande taille. Les petits arbres (< 15 personnes) sont lisibles sans zoom.

**Independent Test**: Peut être testé avec un arbre de 20+ personnes en vérifiant les contrôles de zoom (molette, boutons +/-) et le déplacement (glisser-déposer).

**Acceptance Scenarios**:

1. **Given** un arbre affiché sur desktop, **When** l'utilisateur utilise la molette de la souris, **Then** la vue zoome/dézoome de façon progressive.
2. **Given** un arbre zoomé, **When** l'utilisateur maintient le clic et déplace la souris, **Then** la vue se déplace dans la direction correspondante.
3. **Given** un arbre zoomé et déplacé, **When** l'utilisateur clique sur le bouton de réinitialisation, **Then** la vue revient à la position et au zoom initiaux.

---

### Edge Cases

- Que se passe-t-il quand l'arbre ne contient qu'une seule personne sans aucun lien ? → Affichage d'un nœud unique avec invitation à ajouter des liens.
- Que se passe-t-il quand un arbre est très large (30+ personnes sur une même génération) ? → Défilement horizontal ou regroupement visuel pour éviter le débordement.
- Comment gérer les liens de type « parent » (genre non précisé) visuellement ? → Traitement identique aux liens père/mère avec un style neutre.
- Que se passe-t-il si deux conjoints ont des enfants communs ? → Les enfants sont positionnés sous le couple, reliés aux deux parents.
- Comment réagit l'affichage si une personne a plus de 2 parents renseignés (ex : père, mère, et parent adoptif) ? → Tous les parents sont affichés au-dessus de la personne, disposés horizontalement.
- Que se passe-t-il si l'utilisateur accède à la visualisation sans être connecté ? → Redirection vers la page de connexion.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT afficher l'arbre généalogique de l'utilisateur connecté sous forme de graphe visuel avec des nœuds (personnes) et des arêtes (liens familiaux).
- **FR-002**: Chaque nœud DOIT afficher au minimum : la photo ou les initiales de la personne, son nom complet, et ses années de naissance/décès si renseignées.
- **FR-003**: Les liens parent-enfant DOIVENT être visuellement distinguables des liens conjoint (orientation, couleur ou style différent).
- **FR-004**: Le système DOIT permettre de centrer la vue sur n'importe quelle personne en cliquant/tapotant sur son nœud. Le clic ouvre un panneau contextuel (mini-fiche) et recentre la vue sur cette personne.
- **FR-005**: La vue centrée DOIT afficher la personne sélectionnée avec ses liens directs visibles : parents au-dessus, enfants en dessous, conjoints à côté.
- **FR-016**: Le système DOIT afficher par défaut 3 générations autour de la personne centrée (1 au-dessus, la personne, 1 en dessous) et permettre d'étendre l'affichage à la demande (bouton « voir plus » ou interaction) pour charger les générations suivantes.
- **FR-006**: Le système DOIT proposer un mode de navigation ascendant (ancêtres uniquement) à partir d'une personne sélectionnée.
- **FR-007**: Le système DOIT proposer un mode de navigation descendant (descendants uniquement) à partir d'une personne sélectionnée.
- **FR-008**: Le système DOIT permettre le zoom (avant/arrière) et le déplacement (pan) dans la visualisation.
- **FR-009**: Le système DOIT supporter les gestes tactiles sur mobile : pincement pour zoomer, glisser pour déplacer, tapotement pour sélectionner.
- **FR-010**: Le système DOIT afficher un état vide informatif quand l'utilisateur n'a aucune personne dans son arbre, avec une action pour en ajouter une.
- **FR-011**: Le système DOIT être responsive et utilisable sur les écrans de 320px à 2560px de largeur.
- **FR-012**: Chaque utilisateur ne DOIT voir que son propre arbre — aucun accès aux arbres d'autres utilisateurs.
- **FR-013**: Le système DOIT proposer un bouton de réinitialisation pour revenir à la vue initiale (zoom et position par défaut).
- **FR-014**: La transition lors du recentrage sur une personne DOIT être animée de façon fluide pour maintenir le contexte spatial.
- **FR-015**: Le système DOIT permettre de naviguer depuis la visualisation vers la fiche détail d'une personne (page existante de Feature 1). Un clic sur un nœud ouvre un panneau contextuel (mini-fiche) affichant les informations clés de la personne, avec un bouton « Voir détail » pour naviguer vers la page complète.

### Key Entities

- **Personne (nœud)** : Représentation visuelle d'un individu dans l'arbre. Attributs affichés : photo/initiales, nom complet, années de vie. Sert de point d'interaction pour la navigation.
- **Lien familial (arête)** : Connexion visuelle entre deux personnes. Quatre types : père, mère, parent (générique), conjoint. Orientation et style visuels distincts selon le type.
- **Arbre (conteneur)** : L'ensemble des personnes et liens d'un utilisateur, structuré en générations. Un seul arbre par utilisateur. Sert de périmètre de données pour la visualisation.
- **Vue (mode d'affichage)** : Trois modes possibles — vue complète (tout l'arbre), vue ascendante (ancêtres d'une personne), vue descendante (descendants d'une personne). Chaque mode filtre les nœuds et arêtes affichés.

## Assumptions

- L'API existante (Feature 1) fournit toutes les données nécessaires pour construire la visualisation côté client. Si un point d'accès dédié est nécessaire pour des raisons de performance (récupérer tout l'arbre en un appel), cela sera déterminé lors de la phase de planification.
- Les arbres des utilisateurs contiennent typiquement entre 5 et 100 personnes. La visualisation doit rester fluide jusqu'à au moins 200 personnes. L'affichage est progressif par génération (3 générations visibles par défaut autour de la personne centrée, expansion à la demande) pour garantir la performance et la lisibilité.
- La disposition générationnelle (parents en haut, enfants en bas) est la convention standard. Les conjoints sont placés côte à côte sur la même ligne générationnelle.
- L'accès à la visualisation nécessite une authentification (JWT). Les utilisateurs non connectés sont redirigés vers la page de connexion.
- Le lien vers la fiche détail (FR-015) utilise la page existante `/arbre-genealogique/[id]` de Feature 1.
- La visualisation est une page dédiée (`/arbre-genealogique/visualisation`), distincte de la page index (liste paginée) et de la page détail. La page index existante inclut un lien/bouton vers la visualisation.

## Clarifications

### Session 2026-03-15

- Q: Comment l'utilisateur accède-t-il à la visualisation graphique (remplacement de l'index, nouvelle page dédiée, ou onglet sur l'index) ? → A: Nouvelle page dédiée (`/arbre-genealogique/visualisation`) accessible depuis la page index.
- Q: Pour les grands arbres (100+ personnes), toutes les générations sont-elles visibles simultanément ou l'affichage est-il progressif ? → A: Progressif par génération — 3 générations autour de la personne centrée, expansion à la demande.
- Q: Comment distinguer le clic pour recentrer la vue (FR-004) du clic pour naviguer vers la fiche détail (FR-015) ? → A: Un clic ouvre un panneau contextuel (mini-fiche) avec bouton « Voir détail » pour naviguer vers la page complète. Le recentrage se fait simultanément.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: L'utilisateur peut identifier visuellement les relations familiales entre toutes les personnes de son arbre en moins de 10 secondes après le chargement.
- **SC-002**: L'utilisateur peut naviguer de n'importe quelle personne à n'importe quelle autre personne de son arbre en moins de 5 clics/tapotements.
- **SC-003**: La visualisation se charge et s'affiche de façon interactive en moins de 3 secondes pour un arbre de 50 personnes.
- **SC-004**: L'utilisateur peut accomplir les actions de zoom, déplacement et recentrage sur mobile aussi facilement que sur desktop — aucune fonctionnalité n'est perdue sur petit écran.
- **SC-005**: 90% des utilisateurs testés parviennent à trouver les ancêtres d'une personne donnée sans aide ni instruction préalable.
- **SC-006**: Le basculement entre les modes de vue (complète, ascendante, descendante) se fait en moins de 1 seconde.
