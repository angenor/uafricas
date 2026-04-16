# Feature Specification: Afrolang — Ajustements salles publiques et privées

**Feature Branch**: `005-afrolang-salles`
**Created**: 2026-04-14
**Status**: Draft
**Input**: User description: Ajustement de la fonctionnalité `Afrolang`. Les salles publiques doivent s'appuyer sur la liste des groupes ethniques africains, autoriser la proposition de nouvelles salles en attente de validation par les administrateurs, distinguer deux types de modérateurs (Afrolang désignés vs modérateur de session élu dynamiquement), rendre fonctionnel le tableau blanc, offrir une rubrique « Ressources » et une messagerie instantanée écrite. Les salles privées sont créées depuis une salle publique, avec choix de motif, déclaration d'âge adulte, notice d'alerte pour l'apprentissage des enfants, mode privé/visible, gestion des demandes d'adhésion et limite de participants, les invitations pouvant être refusées. Toute salle privée est rattachée à une salle publique.

## Clarifications

### Session 2026-04-14

- Q: Quelle source référentielle pour la liste des groupes ethniques africains utilisée par les salles publiques Afrolang ? → A: Réutiliser le référentiel existant `country_profile.groupe_ethnique` comme source unique ; chaque salle publique Afrolang est rattachée à une entrée existante, évitant toute duplication.
- Q: Que devient une salle privée si son créateur quitte définitivement la plateforme ou supprime son compte ? → A: Archivage automatique de la salle (lecture seule, plus de nouvelle session possible) dès le départ définitif du créateur ; les participants sont notifiés.
- Q: Comment est proposée la création d'une salle privée depuis une salle publique ? → A: Pas de modal intrusif — un bouton permanent « Créer une salle privée » reste visible dans l'interface de la salle publique, accompagné d'une info-bulle de découverte uniquement à la toute première visite de l'utilisateur dans cette salle.
- Q: Quels types de ressources peuvent figurer dans la rubrique « Ressources » d'une salle ? → A: Ressources internes uploadées (fichiers : PDF, images, audios, vidéos) ET liens externes autorisés, mais tout lien externe doit être validé par un modérateur Afrolang attitré ou un administrateur avant publication.
- Q: Combien de salles privées un même membre peut-il avoir actives en même temps ? → A: Maximum 1 salle privée active par membre et par salle publique de rattachement. Un même membre peut donc posséder plusieurs salles privées actives s'il les rattache à des salles publiques différentes, mais ne peut jamais en avoir deux simultanément rattachées à la même salle publique.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Accéder à la salle publique de son groupe ethnique (Priority: P1)

Un membre ouvre Afrolang, parcourt l'annuaire des groupes ethniques africains et rejoint la salle publique correspondant à son groupe (ex. « Gurunsi »). Il y retrouve les ressources d'apprentissage (dictionnaire, alphabet, médias), participe à la visioconférence, échange par messagerie écrite et utilise le tableau blanc collaboratif. La salle est ouverte à tout membre authentifié, sans restriction supplémentaire.

**Why this priority**: C'est le cœur de l'expérience Afrolang. Sans cet accès fluide aux salles publiques ethniques, aucune autre fonctionnalité n'a de valeur.

**Independent Test**: Un testeur se connecte, ouvre l'annuaire des groupes ethniques, rejoint la salle « Gurunsi », consulte au moins une ressource, envoie un message écrit et trace sur le tableau blanc — tout cela dans une seule session.

**Acceptance Scenarios**:

1. **Given** la salle publique « Gurunsi » existe et est active, **When** un membre authentifié la rejoint, **Then** il accède aux ressources, à la messagerie écrite, au tableau blanc et à la visioconférence sans validation supplémentaire.
2. **Given** une salle publique est affichée, **When** un visiteur non authentifié tente d'y entrer, **Then** il est invité à se connecter avant d'accéder.
3. **Given** plusieurs participants sont dans une salle publique, **When** l'un trace sur le tableau blanc ou envoie un message écrit, **Then** tous les participants présents voient la mise à jour en temps réel.

---

### User Story 2 — Proposer la création d'une salle pour un groupe ethnique absent (Priority: P1)

Un membre cherche sa salle ethnique mais ne la trouve pas dans la liste. La plateforme l'informe qu'il peut proposer la création d'une salle publique et la soumettre pour validation par les administrateurs. Il renseigne le nom du groupe ethnique, la langue cible et une description, puis soumet la demande. Il est informé qu'un délai de validation s'applique et reçoit une notification dès que la salle est approuvée (ou refusée avec motif).

**Why this priority**: Sans ce mécanisme, les communautés absentes du référentiel initial ne peuvent pas rejoindre Afrolang. Cela conditionne la couverture panafricaine promise.

**Independent Test**: Un testeur recherche un groupe absent, déclenche la proposition, complète le formulaire et vérifie que la demande apparaît dans la file de modération admin ; après validation admin, la salle devient visible et rejoignable.

**Acceptance Scenarios**:

1. **Given** aucun résultat pour la recherche « Serer », **When** le membre clique sur « Proposer cette salle », **Then** un formulaire s'ouvre avec les champs requis et un message expliquant que la demande sera examinée par les administrateurs.
2. **Given** une proposition soumise, **When** l'administrateur la valide, **Then** la salle publique devient active et le proposant reçoit une notification de validation.
3. **Given** une proposition refusée, **When** l'administrateur inscrit un motif et refuse, **Then** le proposant reçoit la notification avec le motif et peut soumettre une version corrigée.
4. **Given** une proposition identique à une salle existante ou déjà en attente, **When** le membre la soumet, **Then** la plateforme bloque le doublon et pointe vers la salle existante/en attente.

---

### User Story 3 — Modérer une salle publique (modérateur Afrolang ou modérateur de session) (Priority: P1)

Chaque salle publique dispose d'un ou plusieurs modérateurs Afrolang désignés par l'administration pour leur connaissance de la langue. Quand aucun modérateur Afrolang n'est présent à l'ouverture d'une session, la première personne connectée devient automatiquement modérateur de session. Elle peut, si d'autres arrivent, transférer la modération à un autre participant. Dès qu'un modérateur Afrolang attitré rejoint la salle, il récupère automatiquement le rôle de modération.

**Why this priority**: La modération conditionne la qualité des échanges, la sécurité des participants et la pérennité culturelle des salles.

**Independent Test**: Un testeur vérifie trois séquences : (a) entrée solo = modérateur de session automatique, (b) transfert manuel à un autre participant, (c) arrivée d'un modérateur Afrolang attitré = reprise automatique du rôle.

**Acceptance Scenarios**:

1. **Given** une salle publique sans modérateur présent, **When** le premier participant rejoint, **Then** il reçoit le rôle de modérateur de session avec les outils associés.
2. **Given** un modérateur de session actif, **When** il transfère la modération à un autre participant, **Then** le rôle est retiré du premier et attribué au second de façon immédiate.
3. **Given** un modérateur de session actif, **When** un modérateur Afrolang désigné rejoint la salle, **Then** le système reprend automatiquement le rôle au modérateur de session et l'attribue au modérateur Afrolang, avec notification aux deux.
4. **Given** un modérateur Afrolang est présent, **When** il quitte la salle alors que d'autres participants restent, **Then** le rôle est automatiquement transféré à un participant restant (ou au suivant dans la file) sans interrompre la session.
5. **Given** un modérateur Afrolang est désigné par l'administration, **When** l'administrateur change l'affectation, **Then** la modification s'applique immédiatement aux futures sessions sans interrompre celle en cours.

---

### User Story 4 — Créer une salle privée depuis une salle publique (Priority: P2)

Un membre entrant dans une salle publique (ex. « Gurunsi ») voit un bouton permanent « Créer une salle privée » dans l'interface. À sa première visite, une info-bulle de découverte lui signale cette possibilité ; aux visites suivantes, seul le bouton reste visible (pas de modal intrusif). En cliquant, il choisit un motif (1. Apprentissage de la langue par mes enfants, 2. Réseautage avec un adulte pour apprentissage, 3. Échanges de groupe) et peut personnaliser la description. Il déclare être adulte (18 ans ou plus). Si le motif concerne l'apprentissage d'enfants, une notice d'alerte lui rappelle la nécessité qu'un adulte soit présent auprès des enfants. La salle privée créée est rattachée à la salle publique d'origine.

**Why this priority**: Les salles privées permettent des usages pédagogiques et familiaux impossibles en salle publique. Elles sont essentielles à l'adoption par les familles et les petits groupes d'apprentissage.

**Independent Test**: Un testeur entre dans une salle publique, accepte la proposition, sélectionne chacun des trois motifs et vérifie que l'alerte apparaît uniquement pour le motif « enfants », puis confirme que la salle privée créée est listée comme rattachée à la salle publique d'origine.

**Acceptance Scenarios**:

1. **Given** un membre est dans une salle publique, **When** il consulte l'interface, **Then** le bouton permanent « Créer une salle privée » est visible et, s'il s'agit de sa première visite dans cette salle, une info-bulle de découverte s'affiche une unique fois ; l'action est facultative et n'impacte pas sa participation à la salle publique.
2. **Given** le membre accepte, **When** il sélectionne le motif « Apprentissage par mes enfants » et tente de valider sans cocher la déclaration d'âge adulte, **Then** la création est bloquée avec un message explicite.
3. **Given** le motif « Apprentissage par mes enfants » est sélectionné, **When** le formulaire s'affiche, **Then** une notice d'alerte visible rappelle qu'un adulte doit être présent auprès des enfants.
4. **Given** une salle privée vient d'être créée, **When** on consulte la salle publique associée, **Then** la salle privée apparaît comme rattachée à cette salle publique.

---

### User Story 5 — Gérer la visibilité et les adhésions à une salle privée (Priority: P2)

Le créateur d'une salle privée choisit entre deux modes : (A) **Privée fermée** — la salle n'est pas visible depuis la salle publique ; seules les personnes que le créateur ajoute comme abonnés peuvent y accéder ; (B) **Privée visible** — la salle apparaît aux membres de la salle publique de rattachement, qui peuvent demander à y adhérer. Le créateur fixe une limite de participants et peut refuser une demande (avec motif facultatif « groupe complet » lorsque la limite est atteinte). Les personnes invitées à rejoindre une salle privée peuvent accepter ou refuser l'invitation.

**Why this priority**: La flexibilité visibilité/adhésion est indispensable pour couvrir les usages (famille fermée vs cercle d'apprentissage ouvert).

**Independent Test**: Un testeur crée deux salles privées — une fermée, une visible — puis vérifie qu'un autre membre ne voit que la visible, peut envoyer une demande d'adhésion, et que le créateur peut accepter, refuser ou recevoir automatiquement un refus « groupe complet » quand la limite est atteinte.

**Acceptance Scenarios**:

1. **Given** une salle privée en mode « fermée », **When** un membre tiers consulte la salle publique de rattachement, **Then** la salle privée n'apparaît pas dans la liste.
2. **Given** une salle privée en mode « visible », **When** un membre de la salle publique la consulte, **Then** il peut envoyer une demande d'adhésion.
3. **Given** une demande d'adhésion reçue, **When** le créateur accepte, **Then** le demandeur rejoint la salle et en est notifié.
4. **Given** une demande d'adhésion reçue, **When** le créateur refuse, **Then** le demandeur est notifié du refus.
5. **Given** la limite de participants est atteinte, **When** une demande d'adhésion arrive, **Then** elle est automatiquement indiquée comme « groupe complet » (le créateur peut tout de même manuellement accepter en augmentant la limite).
6. **Given** une personne reçoit une invitation directe à rejoindre une salle privée, **When** elle refuse, **Then** elle n'est pas ajoutée et le créateur est notifié du refus.

---

### User Story 6 — Utiliser le tableau blanc collaboratif, les ressources et la messagerie écrite (Priority: P2)

Dans toute salle (publique ou privée), les participants peuvent : (a) utiliser un tableau blanc collaboratif fonctionnel (tracés, formes, texte, effacement) synchronisé en temps réel ; (b) consulter la rubrique « Ressources » de la salle (dictionnaire de la langue s'il existe, alphabet, médias, liens pédagogiques) ; (c) échanger par une messagerie instantanée écrite pendant la session.

**Why this priority**: Ces trois outils transforment la salle en espace d'apprentissage actif et complètent la visioconférence.

**Independent Test**: Un testeur, dans une session à deux participants, écrit un message qui s'affiche chez l'autre, trace un élément sur le tableau blanc vu par l'autre, et ouvre au moins une ressource attachée à la salle.

**Acceptance Scenarios**:

1. **Given** une session en cours à au moins deux participants, **When** un participant trace sur le tableau blanc, **Then** les autres voient la mise à jour dans un délai perceptible comme instantané.
2. **Given** une salle possède des ressources publiées, **When** un participant ouvre la rubrique « Ressources », **Then** il accède au moins au dictionnaire de la langue (s'il existe), à l'alphabet et aux ressources ajoutées.
3. **Given** la messagerie écrite est ouverte, **When** un participant envoie un message, **Then** tous les participants présents reçoivent le message avec auteur et horodatage.
4. **Given** une session se termine, **When** la session est fermée, **Then** le tableau blanc garde son dernier état persisté et l'historique de messagerie écrite de la session est conservé selon la politique de rétention générale de la plateforme.
5. **Given** un modérateur est présent, **When** il demande l'effacement du tableau blanc, **Then** le tableau est réinitialisé pour tous les participants.

---

### Edge Cases

- Une proposition de salle publique mentionne un groupe ethnique déjà existant orthographié différemment : le système doit signaler le doublon probable avant soumission.
- Un modérateur Afrolang désigné perd sa connexion pendant une session : le rôle est temporairement transféré au participant suivant selon la règle « modérateur de session » et rendu dès son retour.
- Deux modérateurs Afrolang attitrés sont présents simultanément : un seul détient le rôle actif, l'autre dispose de droits équivalents en secours (pas de conflit).
- Le créateur d'une salle privée supprime son compte ou quitte définitivement la plateforme : la salle est automatiquement archivée (passage en lecture seule, plus de nouvelle session possible) et les participants sont notifiés de l'archivage.
- Une salle publique de rattachement est désactivée/supprimée alors que des salles privées y sont rattachées : les salles privées rattachées DOIVENT être archivées en cascade (passage en lecture seule, plus de nouvelle session possible) et leurs participants DOIVENT être notifiés. La suppression dure de la salle publique est interdite tant qu'au moins une salle privée active y est rattachée (la FK `salle_privee.salle_id` est en `ON DELETE RESTRICT`).
- Un mineur tente de créer une salle privée en cochant la déclaration d'âge adulte de manière mensongère : la plateforme doit conserver une trace datée de la déclaration à des fins de responsabilité.
- Une salle privée « visible » reçoit plusieurs acceptations concurrentes alors qu'elle approche sa limite : la limite doit être respectée de manière atomique (pas de dépassement par condition de course).
- Un membre ayant déjà une salle privée active dans une salle publique donnée tente d'en créer une seconde rattachée à la même salle publique : la création est bloquée avec un message explicite l'invitant à archiver ou supprimer sa salle actuelle dans cette salle publique au préalable (le membre reste libre d'en créer dans d'autres salles publiques où il n'en possède pas encore).

## Requirements *(mandatory)*

### Functional Requirements

#### Salles publiques et annuaire ethnique

- **FR-001**: Le système DOIT afficher les salles publiques organisées selon la liste des groupes ethniques africains référencés dans l'entité partagée de la plateforme (réutilisation du référentiel `country_profile.groupe_ethnique` comme source unique de vérité — aucune duplication).
- **FR-002**: Le système DOIT permettre à tout membre authentifié de rejoindre une salle publique sans restriction supplémentaire.
- **FR-003**: Le système DOIT permettre à un membre qui ne trouve pas sa salle ethnique de soumettre une proposition de création de salle publique (groupe ethnique, langue cible, description).
- **FR-004**: Le système DOIT informer le proposant qu'un délai de validation par les administrateurs s'applique avant que la salle devienne accessible.
- **FR-005**: Les administrateurs DOIVENT pouvoir approuver ou refuser une proposition de salle publique, avec motif de refus lorsque pertinent.
- **FR-006**: Le système DOIT notifier le proposant de l'issue (approbation ou refus avec motif) et rendre la salle approuvée immédiatement visible et rejoignable.
- **FR-007**: Le système DOIT détecter et signaler les propositions en doublon (même groupe ethnique, correspondance stricte ou proche) avant soumission.

#### Modération des salles publiques

- **FR-008**: Le système DOIT permettre aux administrateurs de désigner un ou plusieurs modérateurs Afrolang attitrés par salle publique, sur la base de leur connaissance de la langue et de leur disponibilité.
- **FR-009**: Lorsqu'une session démarre sans modérateur Afrolang présent, le système DOIT attribuer automatiquement le rôle de modérateur de session au premier participant à se connecter.
- **FR-010**: Un modérateur de session DOIT pouvoir transférer manuellement son rôle à un autre participant présent.
- **FR-011**: Dès qu'un modérateur Afrolang attitré rejoint la salle, le système DOIT lui attribuer automatiquement le rôle actif, en retirant celui du modérateur de session en place, et notifier les deux parties.
- **FR-012**: Lorsque le modérateur actif quitte la salle alors qu'elle reste peuplée, le système DOIT réattribuer automatiquement le rôle à un participant restant selon la même règle que l'ouverture de session.

#### Salles privées : création et motifs

- **FR-013**: Le système DOIT offrir dans l'interface de toute salle publique un point d'accès permanent (bouton « Créer une salle privée ») permettant de créer une salle privée rattachée à cette salle publique, accompagné d'une info-bulle de découverte affichée uniquement à la première visite du membre dans la salle publique (aucun modal intrusif répété).
- **FR-014**: La création d'une salle privée DOIT exiger le choix d'un motif parmi : (1) Apprentissage de la langue par mes enfants, (2) Réseautage avec un adulte pour apprentissage, (3) Échanges de groupe.
- **FR-015**: Le créateur DOIT pouvoir personnaliser la description de la salle privée.
- **FR-016**: Le créateur DOIT déclarer explicitement être majeur (18 ans ou plus) ; sans cette déclaration cochée, la création DOIT être bloquée.
- **FR-017**: Lorsque le motif sélectionné est « Apprentissage par mes enfants », le système DOIT afficher une notice d'alerte sur la nécessité qu'un adulte soit présent auprès des enfants pendant les sessions.
- **FR-018**: Toute salle privée DOIT être rattachée (indexée) à une unique salle publique d'origine, et cette relation DOIT être consultable depuis les deux côtés.

#### Salles privées : visibilité, adhésions, limites

- **FR-019**: Le créateur d'une salle privée DOIT pouvoir choisir entre deux modes de visibilité : privée fermée (inaccessible depuis la salle publique) ou privée visible (affichée aux membres de la salle publique de rattachement avec possibilité de demande d'adhésion).
- **FR-020**: En mode privé fermé, le créateur DOIT pouvoir envoyer des invitations aux membres de son choix ; aucune demande d'adhésion tierce ne peut être initiée dans ce mode. Les invités restent libres d'accepter ou refuser (cf. FR-025).
- **FR-021**: En mode visible, tout membre présent dans la salle publique de rattachement DOIT pouvoir envoyer une demande d'adhésion à la salle privée.
- **FR-022**: Le créateur DOIT pouvoir accepter ou refuser une demande d'adhésion, le demandeur étant notifié de la décision.
- **FR-023**: Le créateur DOIT pouvoir fixer une limite de participants pour sa salle privée.
- **FR-024**: Lorsque la limite de participants est atteinte, le système DOIT indiquer automatiquement le motif « groupe complet » aux nouveaux demandeurs, tout en permettant au créateur d'augmenter la limite pour accepter manuellement.
- **FR-025**: Une personne invitée directement à rejoindre une salle privée DOIT pouvoir refuser l'invitation ; le créateur est notifié du refus.

#### Outils d'une salle (publique et privée)

- **FR-026**: Le système DOIT fournir un tableau blanc collaboratif fonctionnel synchronisé en temps réel entre participants de la même session (tracés, formes, texte, effacement).
- **FR-027**: Le modérateur actif DOIT pouvoir effacer l'intégralité du tableau blanc, l'action étant propagée à tous les participants.
- **FR-028**: Chaque salle DOIT disposer d'une rubrique « Ressources » acceptant deux types de contenus : (a) fichiers internes uploadés sur la plateforme (PDF, images, audios, vidéos) publiables immédiatement par les modérateurs de la salle ; (b) liens externes vers des ressources tierces (dictionnaires en ligne, sites pédagogiques, vidéos externes) qui DOIVENT être validés par un modérateur Afrolang attitré ou un administrateur avant d'apparaître publiquement dans la rubrique. La rubrique comprend au minimum le dictionnaire de la langue (s'il existe) et l'alphabet.
- **FR-029**: Le système DOIT fournir une messagerie instantanée écrite accessible pendant la session, avec identification de l'auteur et horodatage des messages.
- **FR-030**: La messagerie écrite et le tableau blanc DOIVENT être disponibles à l'identique dans les salles publiques et privées.

#### Notifications et journalisation

- **FR-031**: Le système DOIT notifier les utilisateurs concernés pour : validation/refus d'une proposition de salle publique, changement de modérateur actif, invitation/demande/refus d'adhésion à une salle privée.
- **FR-032**: Le système DOIT journaliser les actions d'administration (désignation de modérateur, validation/refus de salle publique, changements de visibilité de salle privée) à des fins d'audit.
- **FR-033**: La déclaration d'âge adulte cochée lors de la création d'une salle privée DOIT être horodatée et conservée à des fins de responsabilité.
- **FR-034**: Lorsque le créateur d'une salle privée supprime son compte ou est définitivement désactivé, le système DOIT archiver automatiquement la salle privée (lecture seule, aucune nouvelle session possible) et notifier les participants de l'archivage.
- **FR-035**: Chaque membre ne DOIT posséder qu'une seule salle privée active rattachée à une même salle publique à un instant donné. Un membre peut donc avoir plusieurs salles privées actives simultanément si chacune est rattachée à une salle publique distincte. Toute tentative de créer une seconde salle privée dans une salle publique où le membre en possède déjà une active DOIT être bloquée, avec un message invitant à archiver ou supprimer la salle existante de cette salle publique au préalable.
- **FR-036**: Le créateur d'une salle privée DOIT pouvoir modifier à tout moment la limite de participants (`max_participants`) tant que la salle est active (non archivée). La nouvelle valeur DOIT être ≥ au nombre d'abonnés actuellement confirmés (sinon le système rejette la modification avec un message explicite). La modification prend effet immédiatement et permet, le cas échéant, d'accepter manuellement des demandes précédemment refusées pour motif « groupe complet » (FR-024).

### Key Entities *(include if feature involves data)*

- **Groupe ethnique** : entité référentielle existante de la plateforme (`country_profile.groupe_ethnique`), réutilisée sans duplication ; sert de clé d'organisation des salles publiques (une salle publique Afrolang est rattachée à une unique entrée de ce référentiel).
- **Salle publique** : espace d'apprentissage rattaché à un groupe ethnique, créé ou validé par l'administration, ouvert à tous les membres, doté d'une rubrique Ressources, d'une messagerie écrite, d'un tableau blanc et de modérateurs Afrolang attitrés.
- **Proposition de salle publique** : demande émanant d'un membre pour créer une salle correspondant à un groupe ethnique absent ; comporte un état (en attente, approuvée, refusée) et un motif de refus éventuel.
- **Salle privée** : sous-salle rattachée à une salle publique, créée par un membre majeur, assortie d'un motif, d'une visibilité (fermée/visible), d'une limite de participants et d'une liste d'abonnés/participants.
- **Motif de salle privée** : valeur parmi apprentissage_enfants, reseautage_adulte, echanges_groupe.
- **Abonné / demande d'adhésion** : relation entre un membre et une salle privée avec état (invité, demandé, accepté, refusé).
- **Modérateur Afrolang** : membre désigné par l'administration comme modérateur attitré d'une ou plusieurs salles publiques.
- **Rôle de modération de session** : état temporaire attribué au premier participant d'une session publique sans modérateur attitré, transférable à un autre participant, repris automatiquement par un modérateur Afrolang entrant.
- **Tableau blanc** : état collaboratif synchronisé en temps réel par session, persistable.
- **Ressource de salle** : élément pédagogique attaché à une salle, de type fichier interne (PDF, image, audio, vidéo uploadé) ou lien externe. Un champ d'état (publié / en_attente_validation / refusé) s'applique aux liens externes ; les fichiers internes sont publiables immédiatement.
- **Message de session** : message écrit envoyé pendant une session, avec auteur, contenu et horodatage.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 95 % des membres rejoignant une salle publique accèdent à la visioconférence, à la messagerie écrite et au tableau blanc en moins de 10 secondes après clic.
- **SC-002**: 100 % des propositions de salles publiques reçoivent une décision administrateur (approbation ou refus motivé) dans un délai maximum de 7 jours ouvrés après soumission.
- **SC-003**: Dans 100 % des sessions publiques démarrées sans modérateur Afrolang, un modérateur de session est actif dès l'arrivée du premier participant et le basculement vers un modérateur Afrolang entrant se fait en moins de 5 secondes.
- **SC-004**: Les tracés sur le tableau blanc sont visibles par les autres participants en moins de 500 ms dans 95 % des cas lors de sessions à 10 participants ou moins.
- **SC-005**: 100 % des créations de salles privées comportant le motif « Apprentissage par mes enfants » affichent la notice d'alerte et exigent la déclaration d'âge adulte avant validation.
- **SC-006**: Aucune salle privée ne peut dépasser sa limite de participants même en cas d'acceptations concurrentes (0 incident de dépassement sur un échantillon de 1000 scénarios concurrents).
- **SC-007**: 90 % des demandes d'adhésion à une salle privée visible reçoivent une réponse (acceptation ou refus) sous 48 heures, incluant les refus automatiques « groupe complet ».
- **SC-008**: Dans 100 % des salles publiques actives, la rubrique « Ressources » propose au minimum l'alphabet et, lorsqu'il existe, un dictionnaire de la langue cible.
- **SC-009**: 100 % des liens externes soumis à la rubrique « Ressources » transitent par un état « en attente de validation » et n'apparaissent publiquement qu'après décision d'un modérateur Afrolang attitré ou d'un administrateur ; la médiane du temps de validation reste inférieure à 72 heures ouvrées.
- **SC-010**: 0 membre ne peut posséder plus d'une salle privée active rattachée à une même salle publique à un instant donné ; les tentatives de dépassement sont bloquées avec un message explicite dans 100 % des cas.

## Assumptions

- La liste des groupes ethniques africains provient du référentiel existant `country_profile.groupe_ethnique` (source unique de vérité) ; l'enrichissement éventuel (langue cible, alphabet) se fait via les champs de cette entité et de la salle publique elle-même, sans table parallèle.
- Les modérateurs Afrolang sont désignés via le back-office administration existant, en s'appuyant sur les rôles IAM déjà en place.
- La visioconférence et la signalisation temps réel reposent sur l'infrastructure média existante (déjà utilisée par Afrolang) ; la présente spécification définit les comportements fonctionnels indépendamment de cette infrastructure.
- L'historique de la messagerie écrite d'une session est conservé pour la durée de la session puis archivé comme trace pédagogique, conformément à la politique générale de rétention de la plateforme.
- Les notifications utilisent les canaux existants de la plateforme (in-app et courriel) sans nécessité d'un canal dédié.
- La déclaration d'âge adulte repose sur une case à cocher et une journalisation horodatée ; la plateforme ne procède pas à une vérification d'identité formelle pour cette déclaration.
- Le refus d'une proposition de salle publique n'empêche pas la soumission d'une version corrigée.
- La limite de participants par défaut d'une salle privée reprend la valeur déjà pratiquée par la plateforme et reste modifiable par le créateur.
