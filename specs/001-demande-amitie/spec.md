# Feature Specification: Demande d'amitié entre membres

**Feature Branch**: `001-demande-amitie`
**Created**: 2026-05-24
**Status**: Draft
**Input**: User description: "pour cette plateforme, on aimerait que toute personne connecté puisse faire une demande d'amitié à n'importe qui sur /profil"

## Clarifications

### Session 2026-05-24

- Q: La liste d'amis d'un membre est-elle visible par les autres, ou privée ? → A: Privée : seul le membre voit sa propre liste d'amis ; personne d'autre ne la voit.
- Q: Le « ne plus me solliciter » est-il anti-resollicitation seule ou un blocage plus large ? → A: Blocage utilisateur dédié : un membre peut bloquer un autre à tout moment (empêche demandes + rompt/empêche amitié et messagerie) ; le refus reste distinct (simple clôture).
- Q: Quelle longueur maximale pour un message du chat ? → A: 2000 caractères.
- Q: Peut-on éditer/supprimer un message envoyé ? → A: Suppression seule de ses propres messages ; pas d'édition.
- Q: Un nouveau message crée-t-il une notification persistante ? → A: Non : indicateur du bouton flottant seul ; le centre de notifications reste réservé aux évènements de relation (demandes, acceptations).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Envoyer une demande d'amitié à un membre (Priority: P1)

Un membre connecté parcourt l'annuaire des membres (`/profil`) ou consulte la fiche détaillée d'un autre membre (`/profil/{id}`). Il souhaite entrer en relation avec cette personne et déclenche une demande d'amitié d'un simple geste. Le destinataire est averti qu'une demande l'attend.

**Why this priority**: C'est le cœur de la fonctionnalité et le point d'entrée de toute relation. Sans la capacité d'émettre une demande, rien d'autre n'a de sens. Cette tranche, à elle seule, permet déjà à un membre d'initier un contact et au destinataire d'être notifié.

**Independent Test**: Se connecter, ouvrir la fiche d'un autre membre, cliquer sur « Demander en ami », vérifier que l'état du bouton passe à « Demande envoyée » et que le destinataire reçoit une notification. Livre de la valeur : un membre peut signaler son intérêt à entrer en relation.

**Acceptance Scenarios**:

1. **Given** un membre connecté consultant la fiche d'un autre membre avec lequel il n'a aucune relation, **When** il déclenche une demande d'amitié, **Then** une demande au statut « en attente » est enregistrée et le destinataire reçoit une notification.
2. **Given** un membre connecté sur l'annuaire `/profil`, **When** il déclenche une demande d'amitié depuis la carte d'un membre, **Then** la carte reflète immédiatement l'état « demande envoyée ».
3. **Given** un visiteur non connecté consultant une fiche membre, **When** il tente de déclencher une demande d'amitié, **Then** il est invité à se connecter (ou à créer un compte) avant de pouvoir poursuivre.
4. **Given** un membre ayant déjà une demande en attente vers un destinataire, **When** il consulte à nouveau la fiche de ce destinataire, **Then** l'action affiche « Demande en attente » et n'autorise pas l'envoi d'un doublon.

---

### User Story 2 - Répondre à une demande reçue (Priority: P1)

Un membre reçoit une demande d'amitié. Depuis son espace personnel (notifications et/ou page de gestion des demandes), il consulte qui l'a sollicité et décide d'accepter ou de refuser. En cas d'acceptation, une relation d'amitié mutuelle est établie ; en cas de refus, la demande est close.

**Why this priority**: Une demande sans possibilité de réponse est sans issue. Accepter ou refuser ferme la boucle relationnelle et constitue, avec l'envoi, le minimum viable de la fonctionnalité.

**Independent Test**: En tant que destinataire d'une demande en attente, ouvrir la liste des demandes reçues, accepter l'une et refuser l'autre, puis vérifier que la première devient une amitié visible des deux côtés et que la seconde disparaît des demandes actives.

**Acceptance Scenarios**:

1. **Given** un membre avec une demande d'amitié en attente, **When** il l'accepte, **Then** une amitié mutuelle est établie et apparaît dans la liste d'amis des deux membres, et l'émetteur est notifié de l'acceptation.
2. **Given** un membre avec une demande d'amitié en attente, **When** il la refuse, **Then** la demande passe au statut « refusée » et n'apparaît plus parmi les demandes actives.
3. **Given** un membre A ayant envoyé une demande à B, **When** B envoie à son tour une demande à A (demande croisée), **Then** le système établit directement l'amitié mutuelle sans créer de seconde demande en attente.

---

### User Story 3 - Discuter en temps réel avec un ami via le chat flottant (Priority: P2)

Une fois deux membres devenus amis, un **bouton flottant de messagerie**, présent sur toutes les pages de la plateforme, permet au membre d'ouvrir une **fenêtre de chat flottante**. Cette fenêtre affiche sa liste d'amis ; il choisit un ami et échange avec lui des messages **en temps réel**. C'est la valeur concrète débloquée par l'amitié.

**Why this priority**: C'est la finalité de l'amitié sur cette plateforme : pouvoir échanger. Elle dépend de US1 et US2 (il faut être amis pour discuter) mais constitue la principale valeur perçue. Prioritaire juste après le cycle envoyer/répondre.

**Independent Test**: Avec deux comptes devenus amis, ouvrir le bouton flottant sur n'importe quelle page, sélectionner l'ami dans la liste, envoyer un message, et vérifier qu'il apparaît quasi instantanément côté destinataire ; vérifier qu'un membre non-ami n'apparaît pas dans la liste et n'est pas joignable.

**Acceptance Scenarios**:

1. **Given** un membre connecté ayant au moins un ami, **When** il clique sur le bouton flottant de messagerie présent sur la page courante, **Then** une fenêtre flottante s'ouvre et affiche sa liste d'amis.
2. **Given** la fenêtre de chat ouverte, **When** le membre sélectionne un ami et envoie un message, **Then** le message est remis à cet ami en temps réel et apparaît dans la conversation des deux membres.
3. **Given** une conversation existante, **When** un nouveau message arrive alors que la fenêtre est fermée, **Then** le bouton flottant signale la présence de messages non lus.
4. **Given** deux membres qui cessent d'être amis (retrait d'ami), **When** l'un consulte sa liste d'amis dans le chat, **Then** l'autre n'y figure plus et la conversation n'est plus accessible.
5. **Given** un membre sans aucun ami, **When** il ouvre la fenêtre de chat, **Then** la liste est vide et un message l'invite à se faire des amis.

---

### User Story 4 - Consulter et gérer ses relations et demandes (Priority: P3)

Un membre dispose d'un espace où il visualise ses amis, les demandes qu'il a reçues (en attente de réponse) et les demandes qu'il a envoyées (en attente de la réponse d'autrui). Il peut annuler une demande qu'il a émise tant qu'elle n'a pas reçu de réponse, et retirer un ami existant.

**Why this priority**: La gestion donne au membre le contrôle et la lisibilité de ses relations. Elle est très utile mais reste secondaire par rapport au cycle envoyer/répondre.

**Independent Test**: Envoyer plusieurs demandes, en annuler une, recevoir des demandes, en accepter une puis retirer cet ami ; vérifier que chaque liste (amis, demandes reçues, demandes envoyées) reflète fidèlement l'état après chaque action.

**Acceptance Scenarios**:

1. **Given** un membre ayant des amis et des demandes en cours, **When** il ouvre son espace de gestion des relations, **Then** il voit distinctement ses amis, ses demandes reçues et ses demandes envoyées.
2. **Given** un membre ayant émis une demande encore en attente, **When** il l'annule, **Then** la demande est retirée et le destinataire ne peut plus y répondre.
3. **Given** deux membres amis, **When** l'un retire l'autre de ses amis, **Then** la relation d'amitié est rompue des deux côtés.

---

### Edge Cases

- **Auto-demande** : un membre ne peut pas s'envoyer une demande à lui-même ; l'action n'est pas proposée sur sa propre fiche.
- **Relation déjà existante** : si les deux membres sont déjà amis, l'action d'envoi n'est pas proposée (état « Amis »).
- **Demande déjà en attente** : aucun doublon ne peut être créé tant qu'une demande est en attente dans un sens ou dans l'autre.
- **Demande croisée** : si chacun a sollicité l'autre, l'amitié est établie automatiquement (voir US2).
- **Destinataire indisponible** : un membre suspendu, bloqué ou supprimé ne peut pas recevoir de demande, et sa fiche ne propose pas l'action.
- **Re-sollicitation après refus** : après un refus, l'émetteur peut soumettre une nouvelle demande, sauf si le destinataire l'a explicitement bloqué (voir FR-013).
- **Demande vers un bloqueur** : un membre bloqué par X ne peut pas envoyer de demande à X ; l'action n'aboutit pas.
- **Blocage d'un ami** : bloquer un membre déjà ami rompt l'amitié et rend leur conversation inaccessible des deux côtés.
- **Notification déjà lue / demande déjà traitée** : répondre à une demande déjà traitée (par double clic ou onglet obsolète) n'altère pas l'état final et signale que la demande n'est plus active.
- **Volume de sollicitations** : un membre qui envoie un grand nombre de demandes en peu de temps doit être limité afin d'éviter le harcèlement (voir FR-014).
- **Chat sans ami** : si le membre n'a aucun ami, la fenêtre flottante affiche une liste vide avec une invitation à se faire des amis.
- **Ami hors ligne** : un message envoyé à un ami non connecté doit être conservé et lui être présenté à sa prochaine connexion (messages non lus).
- **Perte de connexion temps réel** : en cas de coupure réseau, le chat doit se resynchroniser à la reconnexion sans perte de messages déjà envoyés.
- **Amitié rompue pendant une conversation ouverte** : la conversation devient inaccessible pour de nouveaux échanges des deux côtés (voir FR-025).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Tout membre **connecté et actif** DOIT pouvoir envoyer une demande d'amitié à n'importe quel autre membre actif depuis l'annuaire `/profil` et depuis la fiche détaillée d'un membre `/profil/{id}`.
- **FR-002**: Le système NE DOIT PAS proposer ni autoriser l'envoi d'une demande d'amitié à soi-même.
- **FR-003**: Le système NE DOIT PAS permettre la création d'une demande en doublon lorsqu'une demande est déjà en attente entre les deux membres (dans un sens ou dans l'autre), ni lorsqu'une amitié existe déjà.
- **FR-004**: Lorsqu'un membre déclenche une demande d'amitié, le système DOIT enregistrer une demande au statut « en attente » identifiant l'émetteur, le destinataire et la date.
- **FR-005**: Le destinataire d'une demande DOIT être notifié de la réception d'une nouvelle demande d'amitié.
- **FR-006**: Le destinataire DOIT pouvoir **accepter** ou **refuser** une demande en attente.
- **FR-007**: À l'acceptation d'une demande, le système DOIT établir une **relation d'amitié mutuelle** visible des deux membres, et notifier l'émetteur de l'acceptation.
- **FR-008**: Au refus d'une demande, le système DOIT clôturer la demande (statut « refusée ») et la retirer des demandes actives, sans notifier l'émetteur du refus.
- **FR-009**: Si un membre B envoie une demande à un membre A alors qu'une demande de A vers B est déjà en attente, le système DOIT établir directement l'amitié mutuelle plutôt que de créer une seconde demande.
- **FR-010**: L'émetteur DOIT pouvoir **annuler** une demande qu'il a envoyée tant qu'elle est en attente ; après annulation, le destinataire ne peut plus y répondre.
- **FR-011**: Chaque membre DOIT pouvoir consulter ses **amis**, ses **demandes reçues en attente** et ses **demandes envoyées en attente**.
- **FR-012**: Un membre DOIT pouvoir **retirer un ami** ; la rupture s'applique des deux côtés.
- **FR-013**: Un membre DOIT pouvoir **bloquer** un autre membre à tout moment (indépendamment de tout refus). Le blocage : (a) empêche le membre bloqué d'envoyer une demande d'amitié au bloqueur ; (b) rompt une éventuelle amitié existante entre eux ; (c) rend la messagerie entre eux inaccessible. Un membre DOIT pouvoir **débloquer** ultérieurement un membre qu'il a bloqué.
- **FR-013a**: Le **refus** d'une demande reste distinct du blocage : il clôture simplement la demande (statut « refusée ») sans bloquer l'émetteur, qui peut donc, sauf blocage explicite, soumettre une nouvelle demande.
- **FR-014**: Le système DOIT limiter le rythme d'envoi de demandes par un même membre afin de prévenir le harcèlement et le spam (limite par fenêtre de temps).
- **FR-015**: Le système NE DOIT PAS proposer l'action d'envoi sur la fiche d'un membre indisponible (suspendu, bloqué ou supprimé) ni laisser aboutir une telle demande.
- **FR-016**: L'état de la relation entre le membre courant et un autre membre (aucune relation, demande envoyée, demande reçue, amis, indisponible) DOIT être reflété de façon claire et immédiate sur l'annuaire et la fiche détaillée.
- **FR-017**: Les notifications liées aux demandes d'amitié DOIVENT pouvoir être marquées comme lues et consultées par le destinataire.
- **FR-018**: Une amitié acceptée DOIT débloquer une **messagerie privée en temps réel** entre les deux amis.
- **FR-019**: Un **bouton flottant de messagerie** DOIT être présent sur toutes les pages de la plateforme pour tout membre connecté, donnant accès à la messagerie.
- **FR-020**: Au clic sur le bouton flottant, une **fenêtre de chat flottante** DOIT s'ouvrir et afficher la **liste des amis** du membre ; le membre DOIT pouvoir y sélectionner un ami pour ouvrir leur conversation.
- **FR-021**: Le système DOIT remettre les messages échangés entre deux amis **en temps réel** (le destinataire voit le message quasi instantanément sans recharger la page).
- **FR-022**: Le système NE DOIT autoriser l'échange de messages **qu'entre membres amis** ; un membre non-ami n'apparaît pas dans la liste et ne peut être contacté.
- **FR-023**: Le système DOIT conserver l'**historique** des messages d'une conversation afin que les amis le retrouvent en rouvrant la fenêtre.
- **FR-024**: Le bouton flottant DOIT signaler la présence de **messages non lus** (indicateur visuel), et les messages DOIVENT pouvoir être marqués comme lus à l'ouverture de la conversation. Les nouveaux messages NE créent PAS d'entrée dans le centre de notifications (qui reste réservé aux évènements de relation : nouvelle demande reçue, demande acceptée) ; seul l'indicateur du bouton flottant les signale.
- **FR-025**: Lorsqu'une amitié est rompue (retrait d'ami), l'autre membre DOIT disparaître de la liste d'amis du chat et la conversation NE DOIT plus être accessible pour de nouveaux échanges.
- **FR-026**: La liste d'amis d'un membre DOIT rester **privée** : seul le membre lui-même peut consulter sa propre liste d'amis ; elle n'est exposée à aucun autre membre, ni sur la fiche publique `/profil/{id}`, ni ailleurs.
- **FR-027**: Un message du chat DOIT être limité à **2000 caractères** ; au-delà, l'envoi est refusé avec un message d'erreur clair. Un message vide ne peut pas être envoyé.
- **FR-028**: Un membre DOIT pouvoir **supprimer ses propres messages** ; la suppression se reflète dans la conversation des deux participants. L'**édition** d'un message n'est PAS prévue dans ce périmètre.

### Key Entities *(include if feature involves data)*

- **Demande d'amitié** : représente une sollicitation d'un membre émetteur vers un membre destinataire. Attributs clés : émetteur, destinataire, statut (en attente, acceptée, refusée, annulée), date d'émission, date de traitement. Contrainte : au plus une demande active entre deux membres donnés.
- **Amitié** : relation mutuelle entre deux membres une fois une demande acceptée. Attributs clés : les deux membres, date d'établissement. Symétrique (l'ordre des membres n'a pas d'importance).
- **Notification** : avis adressé à un membre l'informant d'un évènement de relation (nouvelle demande reçue, demande acceptée). Attributs clés : destinataire, type, état lu/non-lu, date.
- **Blocage** : relation orientée par laquelle un membre (bloqueur) empêche un autre (bloqué) de le solliciter et de communiquer avec lui. Attributs clés : bloqueur, bloqué, date. Réversible (déblocage).
- **Conversation** : fil d'échange privé entre deux membres amis. Attributs clés : les deux participants (amis), date de création, date du dernier message. Une seule conversation par paire d'amis.
- **Message** : contenu échangé dans une conversation. Attributs clés : conversation, expéditeur, contenu textuel, date d'envoi, état lu/non-lu.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un membre connecté peut envoyer une demande d'amitié depuis une fiche membre en **moins de 5 secondes** et en **au plus 2 interactions** (ouvrir l'action, confirmer si nécessaire).
- **SC-002**: Le destinataire voit la demande dans ses demandes reçues au plus tard à son **prochain chargement de page** ; lorsque la messagerie temps réel (US3) est active, la notification apparaît en **moins de 5 secondes** sans rechargement.
- **SC-003**: **100 %** des envois en doublon, auto-demandes et demandes vers des membres indisponibles sont rejetés (aucun état incohérent créé).
- **SC-004**: Après acceptation, l'amitié apparaît dans la liste d'amis des **deux** membres, vérifiable immédiatement des deux côtés.
- **SC-005**: Un membre comprend l'état de sa relation avec un autre membre (aucune, en attente, amis) sans aide extérieure dans **au moins 90 %** des cas lors d'un test d'utilisabilité.
- **SC-006**: La limite anti-spam empêche qu'un membre dépasse le seuil défini de demandes par fenêtre de temps, vérifiable en tentant de dépasser le seuil.
- **SC-007**: Le bouton flottant de messagerie est accessible sur **100 %** des pages pour un membre connecté, en **1 clic**.
- **SC-008**: Un message envoyé entre deux amis connectés est remis au destinataire en **moins de 2 secondes**.
- **SC-009**: **100 %** des tentatives d'échange avec un membre non-ami sont empêchées (aucun message remis à un non-ami).
- **SC-010**: L'historique d'une conversation est intégralement retrouvé à la réouverture de la fenêtre (aucun message perdu).

## Assumptions

- La page `/profil` est l'annuaire public des membres et `/profil/{id}` la fiche détaillée d'un membre ; l'action de demande d'amitié s'ajoute à ces deux emplacements.
- Seuls les membres au statut « actif » peuvent émettre ou recevoir des demandes ; les comptes en attente de vérification, suspendus, bloqués ou supprimés en sont exclus.
- Une demande d'amitié est **bidirectionnelle avec acceptation explicite** (modèle « demande → accepte/refuse »), conformément aux conventions usuelles des réseaux de membres.
- Le refus n'est **pas** notifié à l'émetteur, par discrétion ; seule l'acceptation l'est.
- Les notifications réutilisent un mécanisme de notification existant de la plateforme plutôt que d'en introduire un nouveau.
- La fonctionnalité s'appuie sur l'identité des membres déjà gérée par la plateforme ; aucun nouveau type de compte n'est introduit.
- Le seuil exact de la limite anti-spam (FR-014) sera fixé à une valeur raisonnable par défaut lors de la conception (ex. quelques dizaines de demandes par jour).
- La messagerie débloquée par l'amitié est, dans ce périmètre, **textuelle** (échange de messages écrits) ; l'envoi de fichiers/médias n'est pas inclus.
- Le « temps réel » s'appuie sur le mécanisme d'échange instantané déjà disponible sur la plateforme plutôt que d'en introduire un nouveau.
- Le bouton flottant et la fenêtre de chat sont visibles uniquement pour les membres connectés ; ils n'apparaissent pas pour les visiteurs anonymes.
- Une conversation est strictement à deux participants (pas de discussions de groupe dans ce périmètre).
