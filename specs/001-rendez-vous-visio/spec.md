# Feature Specification: Rendez-vous en visioconférence entre membres amis

**Feature Branch**: `001-rendez-vous-visio`  
**Created**: 2026-05-26  
**Status**: Draft  
**Input**: User description: "Système de prise de rendez-vous en visioconférence pair-à-pair (WebRTC/PeerJS) entre membres amis de la plateforme UAfricas : proposer/répondre/contre-proposer un rendez-vous, gérer ses rendez-vous, rejoindre une salle visio 1-à-1, notifications temps réel + cloche persistante, réutilisation du domaine social (amitié, MembreLight, messagerie)."

## Clarifications

### Session 2026-05-26

- Q: Cycle de vie d'une proposition « proposé » dont l'heure est passée sans réponse ? → A: Reste « proposé » en base ; considéré expiré par calcul (date dépassée) → classé « passés », aucune action possible. Pas de nouveau statut, pas de tâche planifiée.
- Q: Marquage « terminé » d'un rendez-vous accepté (manuel ou automatique) ? → A: Automatique par calcul : accepté + fenêtre écoulée → traité « terminé/passé » et rangé dans « passés ». Pas de statut `terminé` persisté, pas de bouton de clôture, pas de tâche planifiée.
- Q: Emplacement de la vue de gestion des rendez-vous (Story 3) ? → A: Intégrée comme section/onglet du panneau de messagerie privée flottant existant (global), aux côtés des conversations.
- Q: Résolution des actions concurrentes (ex. annulation vs acceptation simultanées) ? → A: Verrouillage optimiste : revérification du statut et du tour à l'exécution côté serveur ; la première transition validée gagne, la seconde est rejetée avec un message clair.

## User Scenarios & Testing *(mandatory)*

Cette fonctionnalité permet à deux membres déjà amis de la plateforme d'organiser et de tenir un entretien vidéo en face-à-face (mentorat, entretiens, mises en relation panafricaines). La plateforme orchestre uniquement la prise de rendez-vous ; le flux vidéo circule directement entre les deux participants, sans transiter par les serveurs de la plateforme.

### User Story 1 - Proposer un rendez-vous à un ami (Priority: P1)

Depuis la page de profil d'un membre avec qui il est ami, un membre connecté ouvre un formulaire et propose un entretien vidéo en renseignant un sujet, une description facultative, une date, une heure et une durée parmi des créneaux prédéfinis. À l'envoi, le rendez-vous est enregistré au statut « proposé » et l'ami destinataire est averti en temps réel et par une notification persistante (cloche).

**Why this priority**: C'est le point d'entrée et la condition d'existence de toute la fonctionnalité — sans proposition, aucun rendez-vous ne peut exister. Cette story livre déjà une valeur autonome : un membre peut formuler une demande d'entretien ciblée vers un ami.

**Independent Test**: Connecté en tant que membre A, ami de B, ouvrir le profil de B, soumettre un rendez-vous valide, et vérifier que B reçoit une notification temps réel + cloche et que le rendez-vous apparaît au statut « proposé » côté A et B.

**Acceptance Scenarios**:

1. **Given** A et B sont amis et A est connecté sur le profil de B, **When** A renseigne un sujet, une date/heure future et une durée valides puis envoie, **Then** un rendez-vous au statut « proposé » est créé et B reçoit une notification temps réel et une notification cloche.
2. **Given** A est sur le profil de B avec qui il n'est PAS ami, **When** A consulte la page, **Then** l'action « proposer un rendez-vous » n'est pas proposée (une invitation à se lier d'amitié peut être affichée à la place).
3. **Given** A remplit le formulaire avec une date dans le passé, **When** A envoie, **Then** la proposition est refusée avec un message d'erreur clair et aucun rendez-vous n'est créé.
4. **Given** A tente de proposer un rendez-vous à lui-même, **When** A envoie, **Then** l'action est refusée.
5. **Given** A omet le sujet ou la durée, **When** A envoie, **Then** l'action est refusée avec indication du champ manquant.
6. **Given** A et B sont dans une relation de blocage, **When** A tente d'accéder à l'action, **Then** elle n'est pas disponible et toute tentative est rejetée côté serveur.

---

### User Story 2 - Répondre à une proposition (Priority: P2)

Le destinataire d'une proposition consulte ses rendez-vous reçus et choisit d'accepter, de refuser, ou de contre-proposer un nouveau créneau. L'acceptation fige le créneau ; le refus clôt la proposition ; la contre-proposition redéfinit date/heure/durée, repasse le rendez-vous au statut « proposé » et bascule l'initiative vers l'autre partie, qui doit à son tour répondre. Les contre-propositions peuvent s'enchaîner dans les deux sens jusqu'à accord. Chaque réponse notifie l'autre partie en temps réel et par cloche.

**Why this priority**: La négociation du créneau est nécessaire pour qu'un rendez-vous devienne effectif. Sans cette story, une proposition resterait sans suite. Elle s'appuie sur la story 1 mais reste testable indépendamment dès qu'une proposition existe.

**Independent Test**: À partir d'une proposition existante de A vers B, vérifier successivement que B peut accepter (statut « accepté », A notifié), refuser (statut « refusé », A notifié), et contre-proposer (statut « proposé », initiative basculée vers A, A notifié).

**Acceptance Scenarios**:

1. **Given** une proposition « proposé » reçue par B, **When** B accepte, **Then** le statut devient « accepté », le créneau est figé et A est notifié en temps réel + cloche.
2. **Given** une proposition « proposé » reçue par B, **When** B refuse, **Then** le statut devient « refusé » et A est notifié.
3. **Given** une proposition « proposé » reçue par B, **When** B contre-propose une nouvelle date/heure/durée, **Then** le rendez-vous reste « proposé », c'est désormais A qui doit répondre, et A est notifié.
4. **Given** un rendez-vous déjà « accepté », **When** une partie tente de contre-proposer, **Then** l'action est refusée car le créneau est figé.
5. **Given** une proposition dont c'est le tour de A de répondre, **When** B (qui a posé le dernier créneau) tente d'accepter ou contre-proposer à nouveau, **Then** l'action est refusée (seule la partie dont c'est le tour peut répondre).

---

### User Story 3 - Gérer ses rendez-vous (Priority: P3)

Chaque membre dispose d'une vue listant ses rendez-vous, filtrable par état : « en attente de ma réponse », « en attente de l'autre », « à venir » (acceptés futurs) et « passés ». Chaque rendez-vous affiche l'autre membre (photo, nom, fonction, pays), le sujet, la date/heure, la durée et le statut, avec les actions contextuelles disponibles (accepter, refuser, contre-proposer, annuler). L'une ou l'autre partie peut annuler un rendez-vous proposé ou accepté. Un lien direct vers la messagerie privée existante avec ce membre est proposé.

**Why this priority**: Donne une vue d'ensemble et le contrôle dans la durée (suivi, annulation, accès rapide à la conversation). Indispensable à l'usage réel mais postérieure aux mécanismes de proposition/réponse.

**Independent Test**: Créer plusieurs rendez-vous dans des états différents, ouvrir la vue de gestion, vérifier que chaque filtre affiche les bons rendez-vous, que les informations de l'autre membre s'affichent, que l'annulation fonctionne et notifie l'autre, et que le lien messagerie ouvre la conversation avec ce membre.

**Acceptance Scenarios**:

1. **Given** des rendez-vous dans divers états, **When** le membre applique le filtre « en attente de ma réponse », **Then** seuls les rendez-vous « proposé » dont c'est son tour de répondre apparaissent.
2. **Given** un rendez-vous « accepté » à venir, **When** une des deux parties l'annule, **Then** le statut devient « annulé » et l'autre partie est notifiée.
3. **Given** un rendez-vous affiché, **When** le membre clique sur le lien messagerie, **Then** la conversation privée existante avec l'autre membre s'ouvre.
4. **Given** la vue de gestion, **When** elle s'affiche, **Then** chaque rendez-vous montre l'autre membre (photo, nom, fonction, pays), le sujet, la date/heure, la durée et le statut.
5. **Given** un rendez-vous « refusé » ou « annulé », **When** le membre consulte le filtre « passés », **Then** il y figure sans action de négociation possible.

---

### User Story 4 - Rejoindre la visioconférence (Priority: P4)

Pour un rendez-vous au statut « accepté », un bouton « Rejoindre » devient actif dans une fenêtre temporelle autour de l'heure prévue. En le cliquant, le membre entre dans une salle d'entretien vidéo en tête-à-tête : connexion directe entre les deux participants, affichage de son propre aperçu et du flux distant, et contrôles pour couper/activer micro et caméra et quitter l'appel. L'interface gère explicitement les états d'attente, de connexion, de connexion établie, de départ de l'autre et d'échec, avec un repli proposant la messagerie privée si la connexion échoue. Le rendez-vous peut être marqué « terminé » à la fin de l'appel ou une fois la fenêtre écoulée.

**Why this priority**: C'est l'aboutissement (la rencontre réelle), mais il dépend de l'existence d'un rendez-vous accepté. La complexité technique (connexion directe entre pairs) en fait la dernière tranche, livrable une fois les stories 1–3 en place.

**Independent Test**: À partir d'un rendez-vous accepté dont l'heure approche, vérifier que le bouton « Rejoindre » s'active dans la fenêtre prévue, que les deux participants voient leurs flux respectifs, que les contrôles micro/caméra/quitter fonctionnent, que les états (attente, connecté, l'autre a quitté, échec) s'affichent clairement, et qu'un échec propose la messagerie en repli.

**Acceptance Scenarios**:

1. **Given** un rendez-vous « accepté » dont l'heure n'est pas encore dans la fenêtre d'ouverture, **When** le membre consulte le rendez-vous, **Then** le bouton « Rejoindre » est inactif (avec indication de l'horaire d'ouverture).
2. **Given** un rendez-vous « accepté » dans la fenêtre d'ouverture, **When** le membre clique « Rejoindre », **Then** il entre dans la salle et voit son aperçu local en attendant l'autre participant.
3. **Given** les deux participants ont rejoint, **When** la connexion directe s'établit, **Then** chacun voit le flux vidéo de l'autre et peut couper/activer son micro et sa caméra.
4. **Given** un participant dans la salle, **When** l'autre quitte l'appel, **Then** un état « l'autre a quitté » est affiché clairement.
5. **Given** une connexion directe impossible à établir, **When** l'échec est détecté, **Then** un message d'erreur clair est affiché et la messagerie privée est proposée en repli.
6. **Given** un rendez-vous « accepté » dont la fenêtre de visioconférence est écoulée, **When** une partie consulte ses rendez-vous, **Then** il est automatiquement classé « passés » (terminé par calcul) et le bouton « Rejoindre » n'est plus proposé.

---

### Edge Cases

- **Relation rompue après proposition** : si A et B cessent d'être amis (ou se bloquent) alors qu'un rendez-vous est proposé/accepté, les actions de négociation et le bouton « Rejoindre » deviennent indisponibles.
- **Date passée avant réponse** : si l'heure d'un rendez-vous « proposé » est dépassée sans réponse, son statut reste « proposé » en base mais il est considéré comme expiré par calcul (date dépassée) : il est classé dans « passés », n'est plus négociable ni rejoignable, et n'expose aucune action. Aucun statut « expiré » dédié ni tâche planifiée.
- **Une seule personne rejoint** : la salle reste en état « en attente que l'autre rejoigne » jusqu'à la fin de la fenêtre.
- **Connexion derrière un réseau restrictif (NAT symétrique)** : la connexion directe peut échouer pour une minorité de membres (~15 %) ; l'échec doit être annoncé clairement et la messagerie privée proposée en repli.
- **Annulation pendant l'attente d'entrée en salle** : si une partie annule un rendez-vous accepté juste avant l'heure, l'autre en est informé et la salle ne s'ouvre pas.
- **Actions concurrentes** : si les deux parties agissent quasi simultanément (ex. annulation contre acceptation), la revérification serveur n'applique que la première transition valide ; la seconde action est rejetée avec un message clair plutôt que de produire un état incohérent.
- **Double soumission / propositions multiples** : un membre peut avoir plusieurs rendez-vous distincts avec le même ami ; les soumissions répétées identiques ne doivent pas créer de doublons silencieux.
- **Membre non connecté** : la fonctionnalité (point d'entrée profil, vue de gestion, salle) n'est pas accessible.
- **Contenu sensible** : le sujet/description du rendez-vous ne doit jamais apparaître dans les journaux d'audit.

## Requirements *(mandatory)*

### Functional Requirements

#### Permissions et visibilité

- **FR-001**: Le système MUST n'autoriser la proposition d'un rendez-vous qu'entre deux membres dont la relation d'amitié est active (état « amis »).
- **FR-002**: Le système MUST n'afficher le point d'entrée de proposition sur le profil d'un membre que si le visiteur connecté est ami avec ce membre.
- **FR-003**: Le système MUST interdire toute action de rendez-vous (proposer, accepter, refuser, contre-proposer, rejoindre) en cas de relation de blocage entre les deux membres, et rejeter ces tentatives côté serveur.
- **FR-004**: Le système MUST masquer entièrement la fonctionnalité aux membres non connectés.
- **FR-005**: Pour un membre connecté non ami, le système MAY afficher une invitation à se lier d'amitié comme préalable.

#### Proposition (Story 1)

- **FR-006**: Les membres MUST pouvoir proposer un rendez-vous comprenant un sujet (titre court obligatoire), une description (facultative), une date, une heure et une durée.
- **FR-007**: Le système MUST proposer un ensemble fixe de durées : 15, 30, 45 et 60 minutes.
- **FR-008**: Le système MUST rejeter une proposition dont la date/heure n'est pas dans le futur.
- **FR-009**: Le système MUST rejeter une proposition adressée à soi-même.
- **FR-010**: Le système MUST rejeter une proposition dont le sujet ou la durée est absent.
- **FR-011**: À la création, le système MUST enregistrer le rendez-vous au statut « proposé » et désigner le destinataire comme la partie devant répondre.

#### Réponse et négociation (Story 2)

- **FR-012**: La partie dont c'est le tour de répondre MUST pouvoir accepter, refuser ou contre-proposer un rendez-vous « proposé ».
- **FR-013**: L'acceptation MUST faire passer le rendez-vous au statut « accepté » et figer définitivement le créneau (date/heure/durée).
- **FR-014**: Le refus MUST faire passer le rendez-vous au statut « refusé », sans possibilité de négociation ultérieure.
- **FR-015**: Une contre-proposition MUST redéfinir date/heure/durée, conserver le statut « proposé » et basculer l'initiative vers l'autre partie.
- **FR-016**: Le système MUST permettre l'enchaînement de contre-propositions dans les deux sens tant qu'aucune acceptation n'est intervenue.
- **FR-017**: Le système MUST n'autoriser une réponse (accepter/refuser/contre-proposer) qu'à la partie dont c'est le tour, et rejeter les actions de l'autre partie.
- **FR-018**: Le système MUST interdire toute contre-proposition ou modification de créneau sur un rendez-vous « accepté ».
- **FR-018b**: Le système MUST traiter par calcul (sans transition de statut ni tâche planifiée) un rendez-vous « proposé » dont l'heure est dépassée comme expiré : il conserve le statut « proposé », est classé « passés » et n'autorise plus aucune action (accepter, refuser, contre-proposer).

#### Gestion (Story 3)

- **FR-019**: Les membres MUST disposer d'une vue listant leurs rendez-vous, filtrable par : « en attente de ma réponse », « en attente de l'autre », « à venir » (acceptés futurs), « passés ». Cette vue MUST être intégrée comme section/onglet du panneau de messagerie privée flottant existant (global), aux côtés des conversations.
- **FR-020**: Chaque rendez-vous affiché MUST présenter l'autre membre (photo, nom, fonction, pays), le sujet, la date/heure, la durée et le statut.
- **FR-021**: Le système MUST exposer pour chaque rendez-vous les seules actions contextuelles pertinentes selon son statut et le tour de réponse.
- **FR-022**: L'une ou l'autre partie MUST pouvoir annuler un rendez-vous « proposé » ou « accepté », ce qui le fait passer au statut « annulé ».
- **FR-023**: Le système MUST proposer, depuis chaque rendez-vous, un accès direct à la conversation privée existante avec l'autre membre.

#### Visioconférence (Story 4)

- **FR-024**: Le système MUST n'activer le bouton « Rejoindre » que pour un rendez-vous « accepté » et seulement à l'intérieur d'une fenêtre temporelle autour de l'heure prévue (par défaut : à partir de 5 minutes avant le début et jusqu'à la fin estimée du créneau + 15 minutes de marge).
- **FR-025**: La salle d'entretien MUST établir une connexion vidéo/audio directe entre les deux participants, sans que le média transite par les serveurs applicatifs de la plateforme.
- **FR-026**: La salle MUST afficher l'aperçu local du membre et le flux distant de l'autre participant.
- **FR-027**: La salle MUST offrir les contrôles : couper/activer le micro, couper/activer la caméra, quitter l'appel.
- **FR-028**: La salle MUST gérer et afficher clairement les états : en attente que l'autre rejoigne, connexion en cours, connecté, l'autre a quitté, échec de connexion.
- **FR-029**: En cas d'échec de connexion, le système MUST afficher un message clair et proposer la messagerie privée comme repli.
- **FR-030**: Le système MUST traiter par calcul (sans transition de statut persisté ni tâche planifiée) un rendez-vous « accepté » dont la fenêtre de visioconférence est écoulée comme « terminé/passé » : il est classé « passés » et n'expose plus le bouton « Rejoindre ». Aucun statut `terminé` persisté ni action manuelle de clôture.

#### Notifications

- **FR-031**: Le système MUST notifier l'autre partie en temps réel ET par notification persistante (cloche) pour chacun des événements : rendez-vous proposé, accepté, refusé, contre-proposé, annulé.
- **FR-032**: Le système MUST réutiliser le canal de notifications temps réel et le système de notifications persistantes (cloche) du domaine social existant.

#### Sécurité et journalisation

- **FR-033**: Le système MUST consigner les actions de rendez-vous dans le journal d'audit sans jamais y inclure de contenu sensible (sujet, description).
- **FR-034**: Le système MUST revérifier, côté serveur et à chaque action, l'amitié active et l'absence de blocage avant d'exécuter l'action.
- **FR-035**: Le système MUST résoudre les actions concurrentes par verrouillage optimiste : revérifier le statut courant et le tour de réponse au moment de l'exécution, n'appliquer que la première transition valide, et rejeter toute action devenue invalide avec un message clair (le rendez-vous a déjà été modifié).

### Key Entities *(include if feature involves data)*

- **Rendez-vous** : un entretien vidéo proposé entre deux membres amis. Attributs : identifiant unique, les deux participants (initiateur et destinataire), sujet, description facultative, date/heure prévue, durée (15/30/45/60 min), statut persisté (proposé, accepté, refusé, annulé) — les notions « expiré » et « terminé/passé » sont dérivées par calcul à partir du statut et de la date/fenêtre, sans valeur d'énumération dédiée, partie devant répondre (« tour »), horodatages de création/mise à jour, marqueur de suppression logique. Relation : rattaché à une relation d'amitié existante du domaine social ; lié à la conversation privée existante entre les deux membres pour l'accès à la messagerie.
- **Notification de rendez-vous** : message persistant (cloche) destiné à une partie, indiquant un événement de rendez-vous (proposé, accepté, refusé, contre-proposé, annulé), avec acteur, statut lu/non lu et horodatage. Réutilise le mécanisme de notification du domaine social.
- **Salle d'entretien** : espace de connexion vidéo éphémère associé à un rendez-vous accepté, identifiant chacun des deux participants de façon à ce que chaque côté sache à qui se connecter, sans échange préalable d'identifiants via la plateforme. Aucun média n'y est stocké.
- **Membre (format léger)** : représentation publique d'un membre réutilisée dans les listes et cartes (identifiant, nom, prénom, photo, fonction, pays). Réutilise le format partagé du domaine social.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un membre peut proposer un rendez-vous à un ami en moins de 60 secondes à partir de l'ouverture du profil.
- **SC-002**: Le destinataire est averti (temps réel + cloche) d'une proposition, acceptation, refus, contre-proposition ou annulation en moins de 5 secondes après l'action de l'autre partie.
- **SC-003**: 100 % des tentatives invalides (date passée, soi-même, sujet/durée manquant, non-ami, blocage, action hors tour) sont rejetées avec un message d'erreur compréhensible.
- **SC-004**: Pour deux participants sur des réseaux compatibles, la connexion vidéo s'établit en moins de 10 secondes après que les deux ont rejoint la salle.
- **SC-005**: Le flux vidéo ne consomme pas de bande passante des serveurs applicatifs (média entièrement direct entre participants).
- **SC-006**: En cas d'échec de connexion vidéo, 100 % des sessions affichent un message clair et un repli vers la messagerie privée.
- **SC-007**: Chaque rendez-vous est correctement classé dans le bon filtre de la vue de gestion dans 100 % des cas selon son statut et le tour de réponse.
- **SC-008**: Aucun contenu sensible (sujet, description) n'apparaît dans les journaux d'audit (vérifiable par inspection des entrées d'audit).

## Assumptions

- **Réutilisation du domaine social** : l'état d'amitié (« amis »), le blocage, le format de membre léger (photo/nom/fonction/pays), le canal temps réel et les notifications cloche, ainsi que la messagerie privée, existent déjà et sont réutilisés tels quels.
- **Fenêtre de visioconférence** : par défaut, le bouton « Rejoindre » s'active 5 minutes avant l'heure prévue et reste actif jusqu'à la fin estimée du créneau + 15 minutes ; ces valeurs sont des défauts ajustables.
- **« Terminé » et « expiré » dérivés par calcul** : ni « terminé » ni « expiré » ne sont des statuts persistés. Un « accepté » dont la fenêtre est écoulée est traité « terminé/passé », et un « proposé » dont l'heure est dépassée est traité « expiré/passé » — dans les deux cas par calcul (statut + date/fenêtre), sans tâche planifiée ni action manuelle de clôture.
- **Plusieurs rendez-vous par paire d'amis** : deux amis peuvent avoir plusieurs rendez-vous distincts ; il n'y a pas de limite imposée dans ce lot, mais les soumissions strictement identiques ne créent pas de doublons silencieux.
- **Format de durée** : seuls 15, 30, 45 et 60 minutes sont proposés ; la durée sert à estimer la fin du créneau et la fenêtre d'ouverture de la salle.
- **Notifications de rendez-vous** : elles s'ajoutent aux types de notifications social existants (proposé, accepté, refusé, contre-proposé, annulé).
- **Décisions techniques déjà prises** (reportées au plan d'implémentation) : visioconférence pair-à-pair (WebRTC) via un service de signalisation cloud public configurable ; identifiants de pair déterministes dérivés de l'identifiant du rendez-vous et de chaque participant (pas d'échange via le backend) ; serveurs de relais réseau (STUN publics) uniquement, sans serveur TURN dans ce lot (limite NAT symétrique documentée), liste de serveurs réseau configurable pour un ajout ultérieur ; nouvelle table dans le schéma de données `social` (UUID, suppression logique, horodatages, enums, conventions de nommage du projet) via migration idempotente intégrée à l'orchestrateur de schémas.

## Dependencies

- Domaine social existant : relation d'amitié, blocage, format de membre léger, notifications temps réel + cloche, messagerie privée.
- Page de profil membre `/profil/[id]` : point d'intégration du bouton de proposition (carte « Entrer en contact », aux côtés des actions d'amitié et de message).
- Panneau de messagerie privée flottant global existant : hôte de la vue de gestion des rendez-vous (Story 3), sous forme de section/onglet aux côtés des conversations.
- Service de signalisation vidéo externe (cloud public) pour l'établissement de la connexion directe entre pairs.

## Out of Scope *(pour ce lot)*

- Serveur de relais réseau dédié (TURN) et auto-hébergement du service de signalisation (cloud public uniquement pour l'instant).
- Rendez-vous de groupe (uniquement 1-à-1 entre deux amis).
- Disponibilités/calendrier publiés à l'avance (on reste sur de la proposition libre + accord).
- Enregistrement des appels, partage d'écran, messagerie pendant l'appel (la messagerie privée existante reste le canal texte).
- Rappels par e-mail ou notifications planifiées avant le rendez-vous.
