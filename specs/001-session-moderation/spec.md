# Feature Specification: Modération de session Afrolang — mise en évidence et permissions tableau blanc

**Feature Branch**: `001-session-moderation`
**Created**: 2026-05-10
**Status**: Draft
**Input**: User description: "dans une session public(livestreame), Donner l'option à l'administrateur de la salle de mettre en évidence une personne. Donner la possibilité à l'administrateur de la salle de désigner qui peut écrire sur le tableau blanc. L'admin de la plateforme et l'admin de la salle sont d'office autorisé à ecrire sur le tableau blanc. Pour les session privé, les créateur de la salle privé sont considéré comme les administrateur de leurs salles"

## Clarifications

### Session 2026-05-10

- Q: Comportement par défaut du tableau blanc à l'ouverture d'une session ? → A: Seuls les administrateurs (plateforme + salle) écrivent par défaut ; l'admin accorde individuellement les permissions aux participants choisis (mode « modération proactive »). Changement de comportement assumé par rapport à l'existant.
- Q: Statut des modérateurs attitrés de salle publique (table `salle_moderateur` issue de la feature 005) en session ? → A: Ils ont les mêmes pouvoirs en session que l'admin de salle (permissions tableau blanc + spotlight) et écrivent d'office sur le tableau blanc.
- Q: Persistance des permissions tableau blanc entre sessions d'une même salle ? → A: Hors périmètre pour cette feature. Permissions strictement session-scoped (remises à zéro à chaque session). La persistance (liste d'autorisés permanents) sera traitée dans une feature ultérieure.
- Q: Cibles éligibles de la mise en évidence (spotlight) ? → A: Tous les participants connectés à la session, quel que soit leur état caméra/micro. **Important** : le pouvoir de déclencher un spotlight est restreint à l'administrateur de la plateforme et à l'administrateur de la salle (et au créateur de salle privée, mais le spotlight n'existe pas en privé). Les modérateurs attitrés peuvent gérer les permissions tableau blanc mais **pas** le spotlight.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Contrôler qui peut écrire sur le tableau blanc (Priority: P1)

Pendant une session Afrolang (publique ou privée), l'administrateur de la salle souhaite éviter le chaos sur le tableau blanc partagé. Il ouvre un panneau « Permissions tableau blanc » qui liste les participants présents, et coche/décoche individuellement qui a le droit d'écrire. Les participants non autorisés voient le tableau en lecture seule (barre d'outils désactivée avec un message « lecture seule ») ; les participants autorisés voient leurs outils d'édition apparaître immédiatement, sans avoir à rejoindre la session. L'administrateur de la plateforme et l'administrateur de la salle peuvent toujours écrire, sans que cette permission puisse leur être retirée.

**Why this priority**: Le tableau blanc est l'outil pédagogique central des sessions Afrolang. Sans contrôle d'écriture, n'importe quel participant peut effacer le contenu ou écrire des perturbations, ce qui rend les sessions publiques inutilisables. Ce contrôle est un prérequis à l'ouverture de plus grandes audiences.

**Independent Test**: Démarrer une session avec 3 participants connectés ; l'administrateur ouvre le panneau, autorise un seul participant nommé, et observe que (1) le participant autorisé peut tracer une forme sur le tableau et la voir apparaître chez tous les autres, (2) les deux autres participants ont leurs outils désactivés et voient un libellé « lecture seule », (3) l'administrateur lui-même conserve l'écriture. Retrait de la permission : l'éditeur du participant redevient en lecture seule en moins de 2 secondes.

**Acceptance Scenarios**:

1. **Given** une session publique active avec 5 participants et le créateur de salle connecté comme administrateur, **When** l'administrateur ouvre le panneau « Permissions tableau blanc » et active la permission pour le participant Alice, **Then** Alice voit ses outils d'écriture activés dans un délai inférieur à 2 secondes et peut dessiner sur le tableau commun.
2. **Given** une session avec Alice autorisée à écrire, **When** l'administrateur retire la permission d'Alice, **Then** Alice voit immédiatement sa barre d'outils désactivée, le curseur passe en mode lecture seule, et toute opération en cours est interrompue côté serveur.
3. **Given** un administrateur de salle écrit sur le tableau, **When** l'administrateur de la plateforme tente de retirer la permission d'écriture de l'administrateur de salle, **Then** l'action est refusée avec un message « Cette permission ne peut pas être retirée à un administrateur ».
4. **Given** une session privée et le créateur de cette salle connecté, **When** ce créateur ouvre le panneau « Permissions tableau blanc », **Then** il y a accès et peut accorder/retirer des permissions comme le ferait un administrateur de salle publique.
5. **Given** un participant non autorisé qui a déjà du contenu antérieur sur le tableau (dessiné quand il était autorisé), **When** sa permission est retirée, **Then** le contenu déjà partagé reste visible et n'est pas effacé.
6. **Given** un participant non autorisé, **When** il tente d'envoyer une opération de dessin (via raccourci clavier ou tentative malveillante), **Then** l'opération est rejetée côté serveur et n'apparaît pas chez les autres participants.

---

### User Story 2 - Le créateur d'une salle privée est reconnu comme administrateur de cette salle (Priority: P1)

Quand un utilisateur crée une salle privée Afrolang, il devient automatiquement l'administrateur de cette salle pour toutes les sessions qui s'y déroulent. Il a accès aux mêmes leviers de modération que l'administrateur d'une salle publique (permissions tableau blanc, et toute future capacité de modération en session). Aucune nomination explicite n'est requise.

**Why this priority**: Sans cette règle de permissions, le créateur d'une salle privée ne pourrait pas exercer la modération attendue dans sa propre salle, ce qui bloque l'usage de US1 pour les salles privées (cas d'usage majoritaire des sessions Afrolang). Cette règle débloque l'ensemble du périmètre de modération pour les sessions privées.

**Independent Test**: Créer une nouvelle salle privée en tant qu'utilisateur lambda, ouvrir une session, et vérifier que les options de modération (panneau permissions tableau blanc) sont disponibles pour le créateur exactement comme pour un administrateur de salle publique.

**Acceptance Scenarios**:

1. **Given** un utilisateur qui vient de créer une salle privée et démarre une session dans cette salle, **When** il ouvre l'interface de session, **Then** il voit le panneau de modération « Permissions tableau blanc » disponible et fonctionnel.
2. **Given** un participant invité (non créateur) d'une salle privée, **When** il rejoint la session, **Then** il ne voit pas le panneau de modération et ne peut pas modifier les permissions des autres participants.
3. **Given** une salle privée dont le créateur originel est l'utilisateur A, **When** un autre utilisateur entre dans la session avec le code d'accès, **Then** seul A garde les droits d'administrateur de salle (le code d'accès ne confère pas la modération).

---

### User Story 3 - Mettre en évidence un intervenant dans une session publique livestreamée (Priority: P2)

Pendant une session publique diffusée en livestream, l'administrateur de la salle veut attirer l'attention de tous les spectateurs sur un intervenant précis (un conteur, un linguiste invité, un participant qui pose une question). Il ouvre la liste des participants présents et clique sur « Mettre en évidence » à côté du nom choisi. Sur l'écran de tous les spectateurs et participants, le flux vidéo/avatar de cette personne est agrandi et placé au centre, avec une bordure distinctive et un libellé « En vedette ». L'administrateur peut basculer la mise en évidence d'un autre participant (la précédente est automatiquement retirée), ou désactiver complètement la mise en évidence pour revenir à la disposition normale (mosaïque/orateur actif).

**Why this priority**: La mise en évidence améliore la qualité narrative et pédagogique des sessions publiques en grand format. Elle n'est pas un bloqueur fonctionnel (les sessions tournent sans), mais elle augmente fortement la valeur perçue des livestreams et la qualité de l'expérience spectateur.

**Independent Test**: Démarrer une session publique avec au moins 3 participants visibles ; en tant qu'administrateur, mettre en évidence le participant Bob, et vérifier que (1) tous les autres participants et spectateurs voient le flux de Bob agrandi avec une bordure distinctive et le libellé « En vedette » en moins de 2 secondes, (2) cliquer sur « Mettre en évidence » pour un autre participant transfère automatiquement la mise en évidence, (3) « Désactiver la mise en évidence » rétablit la disposition normale chez tous.

**Acceptance Scenarios**:

1. **Given** une session publique livestreamée avec 4 participants vidéo actifs, **When** l'administrateur met en évidence Bob, **Then** chez tous les participants et spectateurs le flux de Bob est agrandi au centre avec une bordure distinctive et le libellé « En vedette » apparaît, dans un délai inférieur à 2 secondes.
2. **Given** Bob mis en évidence, **When** l'administrateur met en évidence Carole, **Then** la mise en évidence est transférée à Carole et la disposition de Bob revient à la normale, sans étape intermédiaire visible.
3. **Given** une personne mise en évidence, **When** cette personne quitte la session (déconnexion, perte de réseau), **Then** la mise en évidence est automatiquement levée chez tous les spectateurs et la disposition normale est rétablie.
4. **Given** une session privée (non livestreamée), **When** le créateur ouvre la liste des participants, **Then** l'option « Mettre en évidence » n'est pas proposée (la fonctionnalité est limitée aux sessions publiques).
5. **Given** un nouveau spectateur qui rejoint la session pendant qu'un intervenant est mis en évidence, **When** sa connexion s'établit, **Then** il voit immédiatement la mise en évidence active sans avoir à demander une resynchronisation.

---

### Edge Cases

- **Conflit de mise en évidence** : si deux administrateurs (par ex. admin plateforme + admin salle) déclenchent simultanément une mise en évidence sur deux personnes différentes, la dernière action en date l'emporte (et est diffusée à tous), et le panneau du premier reflète l'état actuel.
- **Permission accordée à un participant absent** : si l'administrateur accorde l'écriture à un utilisateur qui n'est pas connecté à la session, la permission est mémorisée pour la durée de la session et s'applique automatiquement dès qu'il rejoint.
- **Reconnexion après coupure réseau** : un participant autorisé qui se déconnecte puis se reconnecte retrouve son droit d'écriture sans intervention de l'administrateur, tant que la session est toujours active.
- **Fin de session** : toutes les permissions individuelles accordées et la mise en évidence active sont effacées à la clôture de la session ; une session suivante repart sur un état par défaut « personne d'autre que les administrateurs n'a le droit d'écrire » et « aucune mise en évidence active ».
- **Retrait d'un administrateur de salle pendant une session** : si la nomination d'un administrateur de salle est révoquée pendant qu'il est dans la session (action de l'admin plateforme dans la console d'administration), il perd ses droits de modération à la première action ; les permissions qu'il avait déjà accordées restent en vigueur.
- **Spotlight sur soi-même** : un administrateur peut se mettre lui-même en évidence ; cela respecte la même règle de transfert (annule toute mise en évidence précédente).
- **Suspension du créateur d'une salle privée** : si le compte du créateur est désactivé/suspendu par la plateforme, ses droits d'administrateur sur ses salles privées sont suspendus le temps de la suspension (les sessions existantes ne perdent pas les permissions déjà appliquées, mais aucune nouvelle action de modération n'est possible).

## Requirements *(mandatory)*

### Functional Requirements

**Définition des rôles**

- **FR-001**: Le système DOIT reconnaître comme « modérateur de session » disposant de pouvoirs de modération en session Afrolang : (a) tout administrateur de la plateforme (rôle global), (b) tout utilisateur nommé administrateur d'une salle publique pour cette salle, (c) tout utilisateur nommé modérateur attitré d'une salle publique pour cette salle, (d) pour les salles privées, l'utilisateur qui a créé la salle privée. Les quatre catégories écrivent d'office sur le tableau blanc et peuvent gérer les permissions tableau blanc des autres participants.
- **FR-001b**: Le système DOIT restreindre le pouvoir de **mise en évidence** (spotlight) aux seuls administrateurs de la plateforme et administrateurs de la salle (et, pour les salles privées, au créateur de la salle — bien que le spotlight ne soit pas proposé en privé). Les modérateurs attitrés n'ont pas accès au spotlight.
- **FR-002**: Le système DOIT considérer le rôle d'administrateur de la plateforme comme supérieur en privilège ; seul l'administrateur plateforme peut révoquer/nommer les administrateurs de salle hors session ; seuls l'administrateur plateforme et l'administrateur de salle peuvent nommer/révoquer les modérateurs attitrés.
- **FR-003**: Le système DOIT recalculer dynamiquement le statut d'administrateur d'un utilisateur en session si sa nomination change (révocation, suspension, nomination), sans nécessiter une déconnexion/reconnexion.

**Permissions tableau blanc**

- **FR-010**: Le système DOIT, par défaut au démarrage d'une session, n'autoriser à écrire sur le tableau blanc que les modérateurs de session (admin plateforme, admin salle, modérateur attitré, créateur de salle privée) ; tous les autres participants sont en lecture seule.
- **FR-011**: Le système DOIT permettre à un administrateur d'accorder, individuellement, la permission d'écrire sur le tableau blanc à n'importe quel participant nommé de la session.
- **FR-012**: Le système DOIT permettre à un administrateur de retirer une permission d'écriture précédemment accordée à un participant non administrateur.
- **FR-013**: Le système DOIT empêcher tout utilisateur (y compris un autre administrateur) de retirer la permission d'écriture à un administrateur ; l'écriture est garantie d'office aux administrateurs.
- **FR-014**: Le système DOIT propager l'état de permission d'un participant à son interface en moins de 2 secondes après une action de l'administrateur, sans nécessiter de rafraîchissement manuel.
- **FR-015**: Le système DOIT refuser, côté serveur, toute opération d'édition du tableau blanc émise par un participant qui n'est pas autorisé à écrire, et ne pas la propager aux autres participants.
- **FR-016**: Le système DOIT préserver le contenu existant du tableau blanc lorsqu'un participant perd sa permission d'écriture (aucun effacement automatique des contributions passées).
- **FR-017**: Le système DOIT réinitialiser toutes les permissions individuelles à la fin de la session (clôture explicite ou inactivité prolongée) ; chaque nouvelle session repart de l'état par défaut FR-010.
- **FR-018**: Le système DOIT afficher à chaque participant, dans son interface de tableau blanc, un état visuel clair indiquant s'il est en lecture seule ou en écriture (ex. libellé, état de la barre d'outils).

**Mise en évidence d'un participant (sessions publiques livestreamées)**

- **FR-020**: Le système DOIT permettre, uniquement dans les sessions publiques livestreamées, à un administrateur (plateforme ou salle uniquement — pas aux modérateurs attitrés) de désigner n'importe quel participant présent comme « mis en évidence », quel que soit l'état de sa caméra ou de son micro.
- **FR-021**: Le système DOIT n'autoriser qu'une seule personne mise en évidence à la fois ; mettre en évidence un nouveau participant retire automatiquement la mise en évidence en cours.
- **FR-022**: Le système DOIT permettre à un administrateur de désactiver complètement la mise en évidence active (retour à la disposition par défaut sans personne en vedette).
- **FR-023**: Le système DOIT diffuser le changement de mise en évidence à tous les participants et spectateurs connectés en moins de 2 secondes.
- **FR-024**: Le système DOIT, lorsqu'un nouveau spectateur ou participant rejoint une session avec une mise en évidence active, lui appliquer immédiatement cet état (pas de désynchronisation visuelle).
- **FR-025**: Le système DOIT lever automatiquement la mise en évidence si la personne mise en évidence quitte la session, perd définitivement sa connexion, ou est éjectée.
- **FR-026**: Le système DOIT afficher visuellement la personne mise en évidence de manière distinctive (mise en avant centrale, bordure ou libellé « En vedette ») chez tous les participants et spectateurs.
- **FR-027**: Le système NE DOIT PAS proposer la mise en évidence dans les sessions privées (interface masquée pour le créateur de salle privée dans ce contexte).

**Audit et observabilité**

- **FR-030**: Le système DOIT enregistrer dans le journal d'audit chaque action de modération en session : accord/retrait de permission d'écriture, mise en évidence, désactivation de mise en évidence, en conservant l'auteur, la cible, l'horodatage et la salle/session.

### Key Entities

- **Session de salle Afrolang** : instance vivante d'une salle (publique ou privée) où des participants interagissent. Porte un état éphémère : permissions d'écriture individuelles accordées, identité de la personne mise en évidence le cas échéant. Cet état est effacé à la clôture de la session.
- **Permission d'écriture tableau blanc** : autorisation donnée à un participant nommé d'une session d'éditer le tableau blanc partagé. Caractérisée par : participant cible, administrateur qui a accordé, horodatage d'octroi. Existe uniquement pendant la session.
- **Mise en évidence** : désignation d'un unique participant comme « en vedette » pendant une session publique. Caractérisée par : participant cible, administrateur qui a désigné, horodatage. Au plus une instance active par session.
- **Administrateur de salle** : utilisateur disposant des privilèges de modération sur une salle donnée. Source de ce statut : nomination administrateur plateforme (rôle global), nomination administrateur de salle publique (existant), ou création d'une salle privée.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Dans 100% des sessions publiques observées, aucun participant non autorisé ne peut écrire sur le tableau blanc partagé (vérifié par audit post-session).
- **SC-002**: Le délai entre l'action de modération de l'administrateur (accord/retrait de permission, mise en évidence) et la prise en compte visuelle chez tous les participants est inférieur à 2 secondes au 95e centile.
- **SC-003**: Le taux de signalements de « perturbation sur le tableau blanc » par les organisateurs de session diminue de 80% par rapport à la situation antérieure (mesuré 30 jours après le déploiement).
- **SC-004**: 90% des administrateurs de salle découvrent et utilisent au moins une fois le panneau « Permissions tableau blanc » lors de leur première session animée après déploiement (mesuré par audit).
- **SC-005**: 100% des créateurs de salles privées peuvent exercer la modération sans formation ni intervention support (mesuré par taux de complétion d'un cas de test guidé < 60 secondes).
- **SC-006**: Aucune action de modération en session n'aboutit à une perte ou un effacement involontaire du contenu existant du tableau blanc (taux d'incident = 0).

## Assumptions

- La fonctionnalité « mise en évidence » s'applique exclusivement aux sessions publiques livestreamées ; les sessions privées en sont exemptées (cas d'usage : sessions privées sont par nature en petit comité, la mosaïque par défaut suffit).
- Une seule personne peut être mise en évidence à la fois (modèle classique de spotlight). Le multi-spotlight est explicitement hors périmètre.
- L'état par défaut au démarrage d'une session est : « personne hors administrateurs n'écrit sur le tableau blanc » et « aucune mise en évidence active ». Les permissions ne sont pas mémorisées d'une session à l'autre.
- Les administrateurs (plateforme + salle) ont par construction le droit d'écrire sur le tableau blanc et ne peuvent en être privés — cette règle prime sur toute autre permission individuelle.
- La révocation d'une nomination d'administrateur de salle prend effet immédiatement, y compris si l'administrateur concerné est en session.
- L'interface de modération (panneau permissions, bouton mise en évidence) est rendue uniquement aux utilisateurs reconnus comme administrateurs de la salle ; un participant standard ne la voit pas et le serveur rejette toute tentative d'action de modération de sa part.
- Les permissions accordées avant déconnexion sont conservées le temps de la session ; un participant qui se reconnecte retrouve son état.
