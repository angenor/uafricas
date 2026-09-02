# Feature Specification: Événements en streaming direct sur la plateforme

**Feature Branch**: `001-evenements-streaming`
**Created**: 2026-05-26
**Status**: Draft
**Input**: User description: "on aimerai que les evements organisé sur la page evenements puisse aussi avoir lieu en streaming directement sur la plateforme utilisant livekit comme afrolang"

## Clarifications

### Session 2026-05-26

- Q: Quel modèle d'interaction pour le streaming d'un événement ? → A: **Webinaire (diffusion)** : l'organisateur et les intervenants désignés diffusent caméra/micro ; les participants regardent et interagissent par chat texte + réactions ; un participant peut être promu intervenant le temps de prendre la parole.
- Q: Qui peut rejoindre la salle de streaming en direct ? → A: **Inscrits + organisateur** : seuls les membres inscrits à l'événement et l'organisateur accèdent au direct ; un non-inscrit est invité à s'inscrire d'abord.
- Q: Faut-il enregistrer le direct pour un replay ? → A: **Hors périmètre (MVP)** : direct uniquement, pas d'enregistrement ni de replay dans cette version.
- Q: Quand la limite de participants simultanés est atteinte, que se passe-t-il pour un nouvel arrivant éligible ? → A: **Refuser avec message** : pas de file d'attente ; message clair « Capacité atteinte, réessayez plus tard ». Une place se libère au départ d'un participant.
- Q: Comment un participant devient-il intervenant (diffusion caméra/micro) ? → A: **Promotion à la volée uniquement**, au démarrage seul l'organisateur diffuse ; il promeut/rétrograde les participants pendant le direct. Aucune désignation préalable de co-animateurs.
- Q: Si un direct est encore en cours à la fermeture de la fenêtre de diffusion, que se passe-t-il ? → A: **Continue jusqu'à clôture** : le direct reste ouvert tant que l'organisateur ne le clôture pas, même au-delà de la fenêtre, avec un arrêt de sécurité automatique absolu lointain (par défaut 2 h après l'heure de fin prévue).
- Q: Un spectateur peut-il signaler qu'il souhaite prendre la parole (« lever la main ») ? → A: **Oui, lever la main**, signal éphémère ; l'organisateur voit la liste des demandes et peut promouvoir (ou ignorer).
- Q: Que se passe-t-il si l'infrastructure de streaming est temporairement injoignable ? → A: **Erreur + repli lien externe**, message d'erreur clair avec possibilité de réessayer ; si un lien en ligne externe est configuré sur l'événement, le proposer en repli.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Assister au direct d'un événement (Priority: P1)

Un membre inscrit à un événement en ligne (ou hybride) rejoint le direct depuis la page de l'événement à l'heure prévue, regarde la diffusion vidéo de l'organisateur et des intervenants, et suit l'événement sans quitter la plateforme.

**Why this priority**: C'est le cœur de la demande, permettre qu'un événement « ait lieu » réellement sur la plateforme. Sans cette capacité de visionnage en direct, la fonctionnalité n'apporte aucune valeur. C'est le MVP minimal démontrable.

**Independent Test**: Avec un événement dont le direct est ouvert et un compte inscrit, cliquer sur « Rejoindre le direct » depuis la page de l'événement et vérifier que la diffusion vidéo/audio des intervenants est visible et audible.

**Acceptance Scenarios**:

1. **Given** un membre inscrit à un événement en ligne dont le direct est ouvert, **When** il ouvre la page de l'événement, **Then** un bouton « Rejoindre le direct » est visible et actif.
2. **Given** ce membre a rejoint le direct, **When** l'organisateur diffuse sa caméra et son micro, **Then** le membre voit la vidéo et entend l'audio de l'organisateur en temps quasi réel.
3. **Given** un membre **non inscrit** consultant un événement dont le direct est ouvert, **When** il regarde la page, **Then** il est invité à s'inscrire d'abord et ne peut pas rejoindre le direct tant qu'il n'est pas inscrit.
4. **Given** un visiteur **non connecté**, **When** il consulte la page d'un événement en direct, **Then** il est invité à se connecter puis à s'inscrire, et ne peut pas accéder au direct.

---

### User Story 2 - Animer l'événement en tant qu'organisateur (Priority: P1)

L'organisateur d'un événement en ligne ouvre le direct au moment voulu, active sa caméra et son micro pour diffuser, partage éventuellement son écran, et clôture le direct à la fin.

**Why this priority**: Sans diffuseur, il n'y a pas de contenu à regarder. La capacité de l'organisateur à émettre est aussi indispensable que la capacité d'audience à regarder ; les deux forment ensemble l'expérience minimale.

**Independent Test**: Avec un compte organisateur, ouvrir le direct de son propre événement, activer caméra/micro et confirmer qu'un second compte inscrit reçoit bien le flux.

**Acceptance Scenarios**:

1. **Given** un organisateur sur la page de son événement en ligne durant la fenêtre de diffusion, **When** il déclenche l'ouverture du direct, **Then** la salle de streaming devient active et l'événement apparaît « en direct ».
2. **Given** le direct est ouvert, **When** l'organisateur active sa caméra et son micro, **Then** son flux est diffusé à tous les participants présents.
3. **Given** le direct est en cours, **When** l'organisateur le clôture, **Then** la salle se ferme, les participants en sont informés et l'événement n'apparaît plus « en direct ».
4. **Given** l'organisateur partage son écran, **When** il sélectionne une fenêtre, **Then** le partage d'écran est diffusé en plus (ou à la place) de sa caméra.

---

### User Story 3 - Interagir pendant le direct (chat & réactions) (Priority: P2)

Pendant le direct, les participants posent des questions et réagissent via un chat texte et des réactions (emojis) visibles par tous, sans interrompre la diffusion.

**Why this priority**: L'interactivité augmente fortement l'engagement et la valeur perçue d'un webinaire, mais l'événement reste visionnable et utile sans elle. C'est un enrichissement, pas le socle.

**Independent Test**: Deux comptes inscrits dans le même direct ; l'un envoie un message texte / une réaction et l'on vérifie que l'autre le voit apparaître en temps réel.

**Acceptance Scenarios**:

1. **Given** un participant dans le direct, **When** il envoie un message dans le chat, **Then** son message s'affiche pour tous les participants présents avec son nom.
2. **Given** un participant dans le direct, **When** il envoie une réaction (emoji), **Then** la réaction est visible des autres participants de façon éphémère.
3. **Given** le direct est terminé, **When** un membre rouvre la page de l'événement, **Then** les messages de chat du direct ne sont plus affichés (chat éphémère, non archivé).

---

### User Story 4 - Donner la parole à un participant (Priority: P3)

Un spectateur signale qu'il souhaite parler (« lever la main ») ; l'organisateur voit les demandes et promeut ponctuellement un participant en intervenant pour qu'il prenne la parole (caméra/micro), puis le repasse en simple spectateur.

**Why this priority**: Utile pour les questions/réponses et tables rondes légères, mais l'événement fonctionne pleinement sans cette bascule. Améliore l'expérience pour une minorité de cas.

**Independent Test**: Dans un direct, l'organisateur promeut un compte participant ; vérifier que ce compte peut alors diffuser caméra/micro, puis qu'il revient spectateur après rétrogradation.

**Acceptance Scenarios**:

1. **Given** un participant spectateur dans le direct, **When** il « lève la main », **Then** sa demande de parole apparaît à l'organisateur dans la liste des demandes.
2. **Given** un participant spectateur dans le direct, **When** l'organisateur le promeut intervenant, **Then** ce participant peut activer sa caméra et son micro et son flux est diffusé.
3. **Given** un participant promu intervenant, **When** l'organisateur le rétrograde, **Then** son flux caméra/micro cesse d'être diffusé et il redevient spectateur.
4. **Given** un participant qui perturbe, **When** l'organisateur le retire du direct, **Then** ce participant est déconnecté de la salle.

---

### Edge Cases

- **Avant l'heure** : un inscrit ouvre la page avant l'ouverture de la fenêtre de diffusion → le direct est annoncé comme « pas encore commencé » avec l'heure prévue, et le bouton de jointure est inactif.
- **Après la fin** : une fois le direct clôturé (par l'organisateur ou par l'arrêt de sécurité absolu), il est marqué « terminé », aucune jointure n'est possible et aucun replay n'est disponible (hors périmètre MVP).
- **Dépassement de l'heure de fin** : le direct peut se poursuivre au-delà de l'heure de fin prévue ; il reste ouvert et joignable jusqu'à la clôture par l'organisateur ou jusqu'à l'arrêt de sécurité absolu. Les participants sont prévenus avant l'arrêt de sécurité.
- **Organisateur absent / direct non ouvert** : durant la fenêtre, si l'organisateur n'a pas encore ouvert le direct, les inscrits voient « En attente de l'organisateur ».
- **Événement présentiel uniquement** : aucun bouton de direct n'est proposé (le streaming ne concerne que les événements en ligne ou hybrides).
- **Événement annulé** : si l'événement passe à « annulé », le direct est inaccessible et tout direct en cours est clôturé.
- **Désinscription pendant le direct** : un participant qui se désinscrit perd l'accès au direct.
- **Coupure réseau** : un participant déconnecté involontairement peut rejoindre à nouveau tant que le direct est ouvert, en retrouvant son rôle (spectateur ou intervenant).
- **Capacité** : si une limite de participants simultanés s'applique et est atteinte, le nouvel arrivant est refusé avec un message clair (« Capacité atteinte, réessayez plus tard ») ; il n'y a pas de file d'attente, une place se libère au départ d'un participant.
- **Deux ouvertures concurrentes** : si l'organisateur tente d'ouvrir un direct déjà ouvert, il rejoint le direct existant plutôt que d'en créer un second.
- **Streaming injoignable** : si l'infrastructure de streaming ne répond pas, l'utilisateur voit un message d'erreur clair avec un bouton « Réessayer » ; si l'événement possède un lien en ligne externe, celui-ci est proposé en repli.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: La plateforme MUST permettre qu'un événement de format « en ligne » ou « hybride » dispose d'un direct vidéo hébergé sur la plateforme, sans dépendre d'un service externe pour le visionnage.
- **FR-002**: La plateforme MUST réserver l'accès au direct aux membres inscrits à l'événement et à son organisateur ; un membre non inscrit MUST être invité à s'inscrire avant de pouvoir rejoindre.
- **FR-003**: Un visiteur non connecté MUST être invité à se connecter (puis à s'inscrire) et NE doit PAS pouvoir rejoindre le direct.
- **FR-004**: L'organisateur MUST pouvoir ouvrir (démarrer) le direct de son événement durant la fenêtre de diffusion autorisée.
- **FR-005**: Au démarrage du direct, l'organisateur MUST être le seul diffuseur ; il MUST pouvoir diffuser caméra et micro et partager son écran. Les intervenants sont uniquement des participants promus à la volée par l'organisateur (pas de désignation préalable de co-animateurs).
- **FR-006**: Les participants spectateurs MUST pouvoir regarder et écouter la diffusion sans diffuser eux-mêmes leur caméra/micro par défaut.
- **FR-007**: La plateforme MUST permettre aux participants présents d'échanger via un chat texte visible de tous pendant le direct.
- **FR-008**: La plateforme MUST permettre aux participants présents d'envoyer des réactions (emojis) éphémères visibles des autres.
- **FR-009**: L'organisateur MUST pouvoir promouvoir un participant spectateur en intervenant (autorisé à diffuser caméra/micro) puis le rétrograder.
- **FR-010**: L'organisateur MUST pouvoir retirer (déconnecter) un participant du direct.
- **FR-011**: L'organisateur MUST pouvoir clôturer le direct ; à la clôture, tous les participants MUST être informés et la salle fermée.
- **FR-012**: La plateforme MUST autoriser l'ouverture du direct à partir d'un délai avant l'heure de début (avant : indisponible). Une fois ouvert, le direct reste accessible (ouverture et nouvelles jointures) tant que l'organisateur ne l'a pas clôturé, même au-delà de l'heure de fin prévue ; la plateforme MUST appliquer un arrêt de sécurité automatique absolu si le direct dépasse une durée limite. Les valeurs exactes sont précisées dans les Hypothèses.
- **FR-013**: La page de l'événement MUST indiquer clairement l'état du direct (à venir / en attente de l'organisateur / en direct / terminé) et proposer le bouton de jointure uniquement quand c'est pertinent.
- **FR-014**: La plateforme MUST permettre à un participant déconnecté involontairement de rejoindre à nouveau tant que le direct est ouvert, en retrouvant son rôle.
- **FR-015**: La plateforme MUST empêcher la création de deux directs simultanés pour le même événement (une seule salle active à la fois).
- **FR-016**: Lorsqu'un événement est annulé, la plateforme MUST rendre le direct inaccessible et clôturer tout direct en cours.
- **FR-017**: La plateforme MUST notifier les inscrits du démarrage du direct via le système de notification existant (cloche + temps réel).
- **FR-018**: La plateforme MUST journaliser (audit) les actions sensibles du direct : ouverture, clôture, promotion/rétrogradation d'intervenant, retrait d'un participant.
- **FR-019**: La plateforme NE doit PAS proposer de bouton de direct pour les événements présentiels uniquement.
- **FR-020**: Lorsque la limite de participants simultanés est atteinte, la plateforme MUST refuser le nouvel arrivant avec un message explicite (« Capacité atteinte, réessayez plus tard ») sans le placer dans une file d'attente.
- **FR-021**: Le média (vidéo/audio) du direct MUST circuler via l'infrastructure de streaming temps réel de la plateforme et NE doit PAS être stocké/enregistré dans cette version.
- **FR-022**: Un participant spectateur MUST pouvoir « lever la main » pour signaler son souhait de parler ; l'organisateur MUST voir la liste des demandes de parole et pouvoir promouvoir ou ignorer chaque demande. Le signal de demande de parole est éphémère.
- **FR-023**: Si l'infrastructure de streaming est injoignable lors de l'ouverture ou de la jointure d'un direct, la plateforme MUST afficher un message d'erreur clair avec possibilité de réessayer et, lorsqu'un lien en ligne externe est configuré sur l'événement, MUST le proposer en repli.

### Key Entities *(include if feature involves data)*

- **Événement** *(existant, étendu)* : rencontre organisée, avec un format (présentiel / en ligne / hybride), un horaire de début et de fin, un organisateur et des inscriptions. La capacité de direct s'applique aux formats en ligne et hybride.
- **Inscription à l'événement** *(existant)* : lien entre un membre et un événement déterminant l'éligibilité à rejoindre le direct.
- **Session de direct** *(nouveau)* : instance de diffusion en temps réel rattachée à un événement, avec un état (en attente / en cours / terminée), une heure d'ouverture et de clôture, et un organisateur responsable. Une seule session active par événement.
- **Participant au direct** *(nouveau)* : présence d'un membre dans une session, avec un rôle (organisateur / intervenant / spectateur) déterminant son droit de diffuser, et son moment d'entrée/sortie.
- **Message de chat** *(nouveau, éphémère)* : message texte émis pendant le direct, attribué à son auteur, visible des participants présents.
- **Réaction** *(nouveau, éphémère)* : emoji envoyé pendant le direct, visible des participants de façon transitoire.
- **Demande de parole** *(nouveau, éphémère)* : signal « lever la main » émis par un spectateur, visible de l'organisateur, résolu par une promotion ou un retrait du signal.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un membre inscrit peut passer de la page de l'événement au visionnage du direct en moins de 15 secondes et 2 actions maximum.
- **SC-002**: La diffusion vidéo/audio est perçue par les spectateurs avec un délai inférieur à 5 secondes par rapport à l'émetteur dans des conditions réseau normales.
- **SC-003**: Au moins 95 % des tentatives de jointure par des membres éligibles aboutissent à un flux visible et audible du premier coup.
- **SC-004**: Un événement en ligne peut accueillir au moins 100 spectateurs simultanés sans dégradation perceptible de la qualité de diffusion.
- **SC-005**: 100 % des membres non éligibles (non inscrits ou non connectés) se voient refuser l'accès au direct et orienter vers l'action requise (s'inscrire / se connecter).
- **SC-006**: Un message de chat ou une réaction émis pendant le direct apparaît chez les autres participants en moins de 2 secondes.
- **SC-007**: L'état du direct affiché sur la page de l'événement (à venir / en direct / terminé) correspond à la réalité dans 100 % des cas observés.

## Assumptions

- **Périmètre des formats** : le direct ne concerne que les événements de format « en ligne » et « hybride ». Les événements « présentiel » n'ont pas de direct.
- **Fenêtre de diffusion** : par défaut, le direct peut être ouvert à partir de 15 minutes avant l'heure de début. Une fois ouvert, il reste accessible (et joignable) tant que l'organisateur ne le clôture pas, même au-delà de l'heure de fin prévue. Un arrêt de sécurité automatique absolu clôture le direct au plus tard 2 heures après l'heure de fin prévue (ou, à défaut d'heure de fin, après une durée maximale par défaut), avec préavis aux participants. Ces marges reprennent l'esprit des fenêtres déjà utilisées ailleurs sur la plateforme.
- **Réutilisation de l'infrastructure** : le streaming s'appuie sur la même technologie temps réel que les salles afrolang (déjà en place), avec la même configuration serveur. Aucun nouveau service d'infrastructure n'est introduit.
- **Modèle webinaire** : par défaut seuls l'organisateur et les intervenants diffusent ; les spectateurs sont en réception. La promotion d'un spectateur en intervenant est ponctuelle et réversible.
- **Capacité** : une limite de participants simultanés peut s'appliquer par événement (réutilisant les mécanismes existants), avec pour objectif minimal ≥ 100 spectateurs (SC-004). À la limite, le nouvel arrivant est refusé avec un message explicite, pas de file d'attente.
- **Chat & réactions** : éphémères, non archivés après la fin du direct (cohérent avec l'absence d'enregistrement). La modération du chat se limite, pour le MVP, au retrait d'un participant du direct par l'organisateur.
- **Notifications** : la notification de démarrage du direct réutilise le système de cloche unifié et le canal temps réel existants.
- **Hors périmètre (MVP)** : enregistrement et replay, accès public anonyme au direct, sous-titrage/traduction en direct, sondages, billetterie payante, planification de plusieurs sessions par événement.

## Dependencies

- Système d'inscription aux événements existant (détermine l'éligibilité au direct).
- Infrastructure de streaming temps réel existante (utilisée pour afrolang).
- Système de notifications (cloche + temps réel) existant.
- Système d'audit existant pour la journalisation des actions sensibles.
