# Feature Specification: Ressources de session livestream Afrolang & fermeture administrative pour abus

**Feature Branch**: `001-ressources-fermeture-session`
**Created**: 2026-05-24
**Status**: Draft
**Input**: User description: "Pour une session livestream afrolang: Ajouter la section ressources (PDF, Word, Vidéos, Accompagnateurs) :ajoutable par tout type d'utilisateur connecté. d'autre part Faire en sorte que l'administrateur puisse fermer une session (dans ce cas la salle est désactivée jusqu'à réactivation). Il arrive qu'une salle fasse l'objet de plusieurs abus donc l'admin peut être en droit de fermer la session jusqu'à ce que lui-même débloque la session."

## Clarifications

### Session 2026-05-24

- Q: Visibilité des ressources de session déposées dans une salle privée (par code d'accès) ? → A: Lecture publique pour les sessions de salles publiques ; pour les sessions de salles privées, lecture restreinte aux comptes ayant historiquement validé le code d'accès de la salle (mémorisation persistante côté serveur, indépendante de la durée de vie du JWT de 4 h).
- Q: Quelles plateformes vidéo sont acceptées pour une ressource de type « Vidéo » ? → A: **YouTube uniquement** (whitelist stricte, validation par regex sur les domaines `youtube.com`, `youtu.be`, `m.youtube.com`, `www.youtube.com` et extraction de l'identifiant vidéo pour l'embed standardisé).
- Q: Périmètre de rattachement des ressources (session vs salle) ? → A: **Les ressources sont rattachées à la salle**, pas à une session individuelle. Elles restent visibles dans toutes les sessions passées, en cours et futures de la salle. La section « Ressources » affichée dans l'écran d'une session est en réalité un miroir de la liste des ressources de la salle hôte. Suppression en cascade uniquement à la suppression de la salle. Cette table de ressources **contribuées librement** par tout utilisateur authentifié est **distincte** de la table `ressource_salle` (modérée, créée en feature `005-afrolang-salles`) qui reste en place pour les ressources officielles ajoutées par les modérateurs/administrateurs de salle.
- Q: Périmètre et consentement d'une recommandation d'« accompagnateur » ? → A: **Tout membre actif de la plateforme est sélectionnable** comme accompagnateur. À réception de la notification, la personne recommandée DOIT explicitement **accepter** ou **refuser** la recommandation. Tant que la recommandation est en état « en attente », elle reste invisible pour les autres utilisateurs (seule l'auteur de la recommandation et la personne recommandée la voient) ; elle ne devient publiquement visible dans la section ressources qu'après acceptation. Un refus ferme définitivement la recommandation et notifie l'auteur (sans révéler de motif obligatoire). Une recommandation acceptée peut être retirée à tout moment par la personne recommandée elle-même ou par son auteur.
- Q: Étendue des notifications lors d'une fermeture administrative de session pour abus ? → A: Notifier **(i)** les administrateurs nommés de la salle publique / le créateur de la salle privée ET **(ii)** les participants présents au moment précis de la coupure (notification persistante post-éjection mentionnant « session fermée par administration », sans divulguer le motif détaillé). Pas de notification individuelle aux contributeurs de ressources (leurs dépôts restent intacts, en lecture seule pendant la désactivation).

## User Scenarios & Testing *(mandatory)*

### User Story 1 : Partage de ressources pédagogiques au niveau de la salle (Priority: P1)

Pendant qu'une session livestream Afrolang est en cours (ou consultable dans son historique), n'importe quel utilisateur authentifié peut alimenter une section « Ressources » affichée dans l'écran de la session. Les éléments déposés (document écrit PDF/traitement de texte, vidéo YouTube, accompagnateur recommandé, lien web complémentaire) sont **rattachés à la salle hôte**, pas à la session : ils restent visibles dans toutes les sessions futures de la même salle et constituent ainsi un corpus pédagogique cumulatif. Les participants, présents ou futurs, voient instantanément ces ressources et peuvent les consulter, télécharger les fichiers et contacter les accompagnateurs recommandés.

**Why this priority**: C'est le cœur fonctionnel demandé : enrichir collaborativement l'apprentissage autour d'une session. Sans cette capacité, la session reste éphémère ; avec elle, chaque session devient un mini-corpus pédagogique réutilisable. C'est la fonctionnalité qui apporte le plus de valeur immédiate aux apprenants et aux animateurs.

**Independent Test**: Ouvrir une session livestream existante avec deux comptes utilisateurs distincts (par exemple un apprenant et un visiteur curieux). Depuis l'un, ajouter un PDF, une vidéo YouTube, et recommander un accompagnateur ; depuis l'autre, vérifier que les trois éléments apparaissent dans la section ressources de la session, qu'ils sont consultables, et que l'auteur de chaque dépôt est identifiable. Démarrer ensuite une **nouvelle session** dans la même salle et confirmer que les trois ressources y apparaissent également (persistance au niveau salle).

**Acceptance Scenarios**:

1. **Given** une session livestream Afrolang en cours et un utilisateur authentifié non modérateur, **When** il téléverse un fichier PDF valide (≤ 20 Mo) avec un titre et une description courte, **Then** le document apparaît immédiatement dans la section ressources de la session pour tous les participants, avec le nom de l'auteur, la date d'ajout et un bouton de téléchargement, **et** il est désormais visible dans toutes les sessions ultérieures de la même salle.
2. **Given** une session livestream et un utilisateur authentifié, **When** il ajoute une ressource de type « Vidéo » en collant une URL YouTube valide, **Then** la vidéo est listée dans la section ressources avec aperçu/lien cliquable et auteur identifié, et reste persistante au niveau salle.
3. **Given** une session livestream et un utilisateur authentifié, **When** il recommande un autre membre de la plateforme comme « accompagnateur » en sélectionnant son profil et en justifiant la recommandation, **Then** une recommandation est créée en état `en_attente` (invisible aux autres utilisateurs), la personne recommandée reçoit une notification avec boutons **Accepter** / **Refuser** ; **When** elle accepte, la recommandation devient visible dans la sous-section « Accompagnateurs » avec lien vers son profil public et motif, et l'auteur reçoit une notification de confirmation ; **When** elle refuse, la recommandation reste invisible et l'auteur reçoit une notification neutre.
4. **Given** un utilisateur ayant déposé une ressource, **When** il consulte une session de la salle, **Then** il peut retirer (supprimer) la ressource qu'il a lui-même déposée, et celle-ci disparaît instantanément des sessions de la salle.
5. **Given** un utilisateur non authentifié (visiteur public) consultant une session de salle publique, **When** il accède à la section ressources, **Then** il voit les ressources existantes mais ne peut pas en ajouter (l'action d'ajout exige une authentification).
6. **Given** une salle ayant connu plusieurs sessions, **When** un utilisateur authentifié consulte la session la plus récente, **Then** il voit l'agrégat de toutes les ressources contribuées au fil des sessions précédentes, classées par date d'ajout décroissante.

---

### User Story 2 : Fermeture administrative d'une session pour abus (Priority: P1)

Un administrateur de la plateforme constate (par signalement ou par observation directe) qu'une session livestream Afrolang fait l'objet de comportements abusifs répétés (propos haineux, harcèlement, contenu illicite, débordements). Pour protéger la communauté, il déclenche une **fermeture administrative** de la session. Cette action interrompt immédiatement la session en cours si elle est active, désactive l'accès à la salle qui hébergeait cette session, et empêche le démarrage de toute nouvelle session dans cette salle. La salle reste désactivée tant qu'un administrateur (pas nécessairement le même) ne procède pas explicitement à sa réactivation. Aucun autre rôle (modérateur attitré, créateur de salle privée, administrateur de salle publique) ne peut lever cette désactivation.

**Why this priority**: C'est le mécanisme de sécurité ultime, indispensable dès que la plateforme accueille des sessions ouvertes. Sans lui, la modération en direct (mute, kick) ne suffit pas face à un schéma d'abus récidiviste. Cette capacité est non négociable pour la conformité et la confiance des utilisateurs.

**Independent Test**: Avec une session livestream active comportant plusieurs participants, un administrateur déclenche la fermeture depuis le panneau d'administration en motivant l'action. Vérifier que (1) tous les participants sont immédiatement éjectés avec un message indiquant la fermeture administrative, (2) la salle apparaît désormais en état « désactivée pour abus » dans l'annuaire public, (3) toute tentative de re-jointure ou de démarrage d'une nouvelle session échoue avec un message explicite, (4) seul un autre administrateur peut la réactiver, et (5) l'événement est tracé (qui, quand, motif).

**Acceptance Scenarios**:

1. **Given** une session livestream Afrolang en cours avec N participants et un compte administrateur plateforme, **When** l'administrateur déclenche « Fermer la session pour abus » en saisissant un motif obligatoire (≥ 10 caractères), **Then** la session est interrompue instantanément, tous les participants sont déconnectés du flux live et reçoivent une notification persistante « Session fermée par l'administration » (sans motif détaillé), les administrateurs nommés / créateur de la salle reçoivent une notification détaillée avec motif, la salle hôte passe à l'état « désactivée par administration », et l'événement est journalisé dans l'audit avec auteur, horodatage et motif.
2. **Given** une salle désactivée par administration, **When** un utilisateur tente d'y entrer ou de démarrer une nouvelle session, **Then** l'action est refusée avec un message clair (« Salle temporairement désactivée par l'administration ») et la possibilité de signaler ou contacter le support.
3. **Given** une salle désactivée par administration, **When** un modérateur attitré, l'administrateur de salle publique nommé, ou le créateur de la salle privée tente de la réactiver, **Then** l'action est refusée, seul un compte administrateur plateforme peut réactiver.
4. **Given** une salle désactivée par administration et un compte administrateur plateforme, **When** ce dernier déclenche la « Réactivation » avec un commentaire facultatif, **Then** la salle redevient accessible, l'événement de réactivation est journalisé, et les administrateurs de la salle (s'il y en a) sont notifiés.
5. **Given** une salle désactivée par administration, **When** on consulte l'annuaire public ou la fiche détaillée, **Then** la salle reste visible mais clairement marquée comme désactivée, sans exposer le motif détaillé au grand public (le motif n'est lisible que par les administrateurs).
6. **Given** une salle déjà désactivée par administration, **When** un administrateur tente une nouvelle fermeture, **Then** l'action est refusée comme redondante (idempotence).

---

### User Story 3 : Consultation et historique de l'état d'une salle (Priority: P2)

Les administrateurs de la plateforme et, dans une moindre mesure, les administrateurs d'une salle publique ou créateurs de salle privée, doivent pouvoir consulter l'historique des fermetures et réactivations administratives d'une salle pour comprendre les motifs récurrents, ajuster leurs propres pratiques de modération, ou contester une décision auprès du support.

**Why this priority**: Indispensable pour la transparence et la gouvernance, mais pas bloquant pour le MVP fermeture/réactivation lui-même.

**Independent Test**: Sur une salle ayant subi au moins deux cycles fermeture → réactivation, un administrateur ouvre la fiche de la salle et consulte l'onglet « Historique de modération » : la liste chronologique des évènements (fermeture, réactivation) avec auteur, date, motif est lisible.

**Acceptance Scenarios**:

1. **Given** une salle ayant connu une fermeture suivie d'une réactivation, **When** un administrateur ouvre la fiche d'administration de la salle, **Then** il voit l'historique complet (fermeture(s) + réactivation(s)) avec auteur, horodatage et motif/commentaire de chaque évènement.
2. **Given** un administrateur de salle publique nommé sur la salle (rôle non-plateforme), **When** il consulte sa fiche, **Then** il voit les fermetures administratives passées (dates et motifs) afin de comprendre les antécédents de modération.

---

### Edge Cases

- **Ressource trop volumineuse** : l'utilisateur tente d'envoyer un PDF/document de plus de 20 Mo, le système rejette le téléversement avec un message expliquant la limite, sans bloquer la session.
- **Type de fichier non autorisé** : un utilisateur essaie d'envoyer un exécutable ou un format hors liste blanche, refus immédiat avec message explicite.
- **URL vidéo non YouTube** : l'utilisateur colle un lien Vimeo, Dailymotion, TikTok, MP4 direct, etc., refus immédiat avec message indiquant que seules les URLs YouTube sont acceptées dans cette version, et suggestion de re-héberger sur YouTube si la vidéo lui appartient.
- **URL YouTube malformée ou identifiant introuvable** : l'URL contient le bon domaine mais l'identifiant vidéo (11 caractères) n'est pas extractible, refus avec message demandant de coller l'URL complète depuis la barre d'adresse YouTube.
- **Auto-recommandation comme accompagnateur** : un utilisateur tente de se recommander lui-même, refus, l'utilisateur doit recommander une autre personne.
- **Recommandation d'un utilisateur inactif/suspendu** : l'accompagnateur recommandé est dans un état incompatible, refus avec message neutre.
- **Suppression de la ressource par son auteur après que d'autres l'ont consultée** : la ressource disparaît de la session ; les utilisateurs ayant déjà téléchargé conservent leur copie locale.
- **Auteur de ressource suspendu après dépôt** : les ressources qu'il a déposées restent visibles mais marquées « auteur indisponible » ; un administrateur peut les retirer manuellement.
- **Fermeture pendant qu'une autre opération de modération est en cours** (ex : un modérateur de session vient de muter quelqu'un) : la fermeture administrative prime et termine immédiatement la session, indépendamment des actions en cours.
- **Salle privée fermée par administration** : même comportement qu'une salle publique, le code d'accès continue d'exister mais toute tentative d'entrée est refusée tant que la salle reste désactivée.
- **Tentative concurrente de réactivation par deux administrateurs** : la première opération prend effet, la seconde est traitée comme idempotente.
- **Salle déjà supprimée (soft-delete)** : ni fermeture, ni réactivation possibles ; les actions retournent un message indiquant que la salle n'existe plus.
- **Ressource ajoutée par un utilisateur de la session, puis session fermée pour abus** : les ressources non illicites restent attachées à la session pour consultation par les administrateurs ; après réactivation de la salle, elles redeviennent visibles aux utilisateurs.

## Requirements *(mandatory)*

### Functional Requirements

#### Ressources de session

- **FR-001**: Le système DOIT exposer, dans l'écran de chaque session livestream Afrolang (en cours, terminée, archivée mais non supprimée), une section « Ressources contribuées » qui affiche l'ensemble des ressources rattachées à la **salle hôte** de la session (et non à la session elle-même). Cette agrégation est partagée entre toutes les sessions de la même salle. L'accès en lecture suit les règles suivantes :
  - **Salle publique** : lecture ouverte à tout visiteur (authentifié ou non).
  - **Salle privée par code d'accès** : lecture restreinte aux comptes authentifiés ayant historiquement validé le code d'accès de la salle. Cette autorisation est mémorisée de façon persistante côté serveur (table de jointure utilisateur × salle privée) et survit à l'expiration du JWT d'accès de 4 h ; elle est révoquée si l'administrateur de la salle privée change le code d'accès (les utilisateurs précédemment autorisés doivent alors revalider le nouveau code).
  - Les administrateurs de la plateforme ont toujours un accès en lecture, quel que soit le type de salle.
- **FR-001-bis**: Cette section « Ressources contribuées » DOIT être **distincte** de la liste existante des « ressources officielles » modérées (`afrolang.ressource_salle` livrée en feature `005-afrolang-salles`). Les deux listes coexistent dans l'écran de la session, clairement étiquetées, et utilisent des entrepôts de données séparés. La nouvelle liste « contribuée » N'EST PAS modérée a priori (publication immédiate) ; l'ancienne reste modérée.
- **FR-002**: Tout utilisateur authentifié dont le compte est en état actif DOIT pouvoir ajouter une ressource à la section « Ressources contribuées » depuis l'écran d'une session de la salle, indépendamment de son rôle (apprenant, contributeur, animateur, modérateur, administrateur). La ressource est rattachée à la **salle hôte** de la session (FR-001) et devient instantanément visible dans toutes les sessions de cette salle.
- **FR-003**: Le système DOIT prendre en charge au minimum quatre types de ressources : (a) document écrit (PDF, traitement de texte type `.doc`/`.docx`/`.odt`), (b) vidéo référencée par URL d'une plateforme publique reconnue, (c) accompagnateur (référence à un autre membre actif de la plateforme), (d) lien web complémentaire.
- **FR-004**: Pour un document, le système DOIT exiger un titre (≤ 120 caractères), accepter une description optionnelle (≤ 500 caractères), valider la taille du fichier (≤ 20 Mo) et restreindre les formats à une liste blanche (au minimum : PDF, DOC, DOCX, ODT).
- **FR-005**: Pour une vidéo, le système DOIT exiger un titre et une URL au format valide pointant vers **YouTube uniquement** (domaines acceptés : `youtube.com`, `www.youtube.com`, `m.youtube.com`, `youtu.be`, URLs `watch?v=`, `youtu.be/<id>`, `embed/<id>` et `shorts/<id>`). Le système DOIT extraire l'identifiant vidéo à partir de l'URL, le stocker en plus de l'URL d'origine pour permettre un embed standardisé, et rejeter avec un message explicite toute URL ne correspondant pas à ce schéma (autres plateformes vidéo non acceptées dans cette version). Aucun flux vidéo n'est copié ni rehébergé.
- **FR-006**: Pour un accompagnateur, le système DOIT exiger la sélection d'un membre tiers actif distinct de l'auteur de la recommandation, accompagnée d'un motif (≥ 20 caractères) expliquant la pertinence de la recommandation. La recommandation est créée à l'état **`en_attente`** et N'EST PAS publiquement visible. Une notification (`afrolang.accompagnateur.recommandation_recue`) est envoyée à la personne recommandée, qui DOIT explicitement **accepter** ou **refuser** :
  - **Acceptation** → l'état passe à `acceptee`, la recommandation devient visible dans la section « Ressources contribuées » de la salle, et une notification est envoyée à l'auteur.
  - **Refus** → l'état passe à `refusee`, la recommandation reste invisible et une notification neutre est envoyée à l'auteur (le motif de refus est facultatif et n'est pas exposé).
  - **Sans réponse** → l'état reste `en_attente` indéfiniment, sans expiration automatique dans cette version.
  Tant que l'état est `en_attente` ou `refusee`, seuls l'auteur et la personne recommandée voient l'entrée (l'auteur la voit pour ne pas en re-créer une identique). Les autres utilisateurs et visiteurs ne la voient pas.
- **FR-006-bis**: La personne recommandée DOIT pouvoir, à tout moment après acceptation, retirer son consentement ; la recommandation passe alors à l'état `retiree` et disparaît instantanément de l'affichage public. L'auteur de la recommandation peut également la supprimer à tout moment (FR-008).
- **FR-007**: Le système DOIT enregistrer pour chaque ressource son auteur, sa date d'ajout, son type, son contenu **et l'identifiant de la salle de rattachement** (clef étrangère vers `afrolang.salle`), afin de permettre la traçabilité, l'agrégation par salle et la suppression par l'auteur. L'identifiant de la session depuis laquelle la ressource a été déposée DOIT être conservé à titre informatif (colonne `session_origine_id` nullable, sans contrainte d'unicité), mais N'EST PAS utilisé comme filtre de lecture.
- **FR-008**: L'auteur d'une ressource DOIT pouvoir retirer (supprimer) sa propre ressource à tout moment ; un administrateur de la plateforme DOIT également pouvoir retirer toute ressource jugée non pertinente ou inappropriée.
- **FR-009**: Lorsqu'un utilisateur est recommandé comme accompagnateur, le système DOIT lui adresser une notification l'informant de la recommandation, avec lien vers la salle concernée et boutons d'action **Accepter** / **Refuser** (cf. FR-006).
- **FR-010**: Le système DOIT empêcher l'ajout de ressources à une salle désactivée par administration ; pendant la désactivation, la section reste consultable par les administrateurs mais en lecture seule pour les autres rôles.
- **FR-011**: Le système DOIT appliquer une limitation raisonnable au volume de dépôts par utilisateur et par salle afin de prévenir les abus (par défaut : 10 ressources par salle par utilisateur par 24 h glissantes).
- **FR-012**: Le système DOIT journaliser dans l'audit chaque ajout et chaque suppression de ressource (auteur, type, identifiant de salle, identifiant de session d'origine, horodatage).
- **FR-012-bis**: Lorsque la salle est supprimée (soft-delete), toutes les ressources contribuées rattachées DOIVENT être marquées comme supprimées en cascade ; aucune ressource orpheline ne doit subsister.

#### Fermeture / réactivation administrative

- **FR-013**: Un administrateur de la plateforme DOIT pouvoir, depuis l'interface d'administration, déclencher la fermeture d'une session livestream Afrolang en cours pour motif d'abus, en saisissant un motif textuel obligatoire (≥ 10 caractères, ≤ 1000 caractères).
- **FR-014**: La fermeture administrative DOIT interrompre instantanément le flux live, déconnecter tous les participants en cours, et marquer la session comme terminée pour cause administrative.
- **FR-015**: La fermeture administrative d'une session DOIT entraîner la désactivation de la salle qui hébergeait cette session, empêchant toute nouvelle session dans cette salle et toute jointure.
- **FR-016**: Une salle désactivée par administration NE PEUT être réactivée QUE par un compte administrateur de la plateforme (pas par modérateur attitré, ni administrateur de salle publique, ni créateur de salle privée).
- **FR-017**: Lors d'une réactivation administrative, le système DOIT permettre la saisie d'un commentaire optionnel, journaliser l'évènement, et rendre la salle de nouveau accessible aux jointures et aux nouvelles sessions.
- **FR-018**: Le système DOIT journaliser chaque fermeture et chaque réactivation administrative (auteur, salle, session le cas échéant, horodatage, motif/commentaire) dans l'audit.
- **FR-019**: Lors d'une fermeture administrative, le système DOIT envoyer deux familles de notifications :
  - **(i)** Aux administrateurs nommés d'une salle publique (le cas échéant) et au créateur d'une salle privée : notification détaillée incluant le motif administratif complet.
  - **(ii)** Aux participants présents dans la session à l'instant exact de la coupure (recensés via la liste LiveKit des participants connectés) : notification persistante post-éjection mentionnant que la session a été fermée par l'administration de la plateforme, **sans divulguer le motif détaillé**, avec un lien vers la fiche publique de la salle (badge « désactivée par administration ») et un canal de contact support.
  Aucune notification individuelle n'est envoyée aux contributeurs de ressources qui n'étaient pas présents au moment de la coupure (leurs dépôts restent intacts, basculés en lecture seule pendant la désactivation conformément à FR-010).
  Lors d'une **réactivation** administrative, seule la famille (i) est notifiée (admins de salle / créateur). Les participants éjectés ne sont pas re-notifiés individuellement.
- **FR-020**: Le système DOIT exposer publiquement le statut « désactivée par administration » d'une salle (badge / mention sur l'annuaire et la fiche) sans divulguer le motif détaillé ; le motif reste consultable uniquement par les administrateurs.
- **FR-021**: Le système DOIT empêcher toute fermeture administrative redondante (idempotence) et tout accès non-administrateur aux endpoints de fermeture/réactivation.
- **FR-022**: Le système DOIT exposer aux administrateurs un historique chronologique des fermetures et réactivations administratives par salle.

### Key Entities *(include if feature involves data)*

- **Ressource contribuée de salle** : élément (document, vidéo YouTube, accompagnateur, lien) rattaché à une **salle Afrolang**. Attributs métier : type, titre, description, contenu (fichier / URL YouTube + identifiant vidéo extrait / référence utilisateur / motif de recommandation), auteur (utilisateur authentifié), date d'ajout, salle de rattachement (FK obligatoire), session d'origine (FK optionnelle, à titre informatif), état (active / retirée). Entité distincte de la « ressource officielle modérée » existante (`afrolang.ressource_salle`).
- **Session livestream Afrolang** (existante) : sert de point d'accès UX à la liste agrégée des ressources contribuées de la salle hôte, sans posséder elle-même de ressources.
- **Salle Afrolang** (existante, publique ou privée) : enrichie (a) d'une relation 1-N avec « Ressource contribuée de salle », (b) d'un état « désactivée par administration » et (c) d'une relation 1-N avec un historique de modération administrative.
- **Évènement de modération administrative** : trace immuable d'une fermeture ou d'une réactivation administrative d'une salle. Attributs métier : salle concernée, session concernée le cas échéant, type d'action (fermeture / réactivation), auteur administrateur, horodatage, motif/commentaire.
- **Recommandation d'accompagnateur** : sous-type particulier de ressource contribuée de salle, reliant l'auteur de la recommandation à un membre tiers recommandé avec un motif justificatif. Cycle de vie : `en_attente` → `acceptee` (visible publiquement) | `refusee` (invisible, fermée) | `retiree` (visible un temps puis fermée à l'initiative de la personne recommandée). Attributs additionnels : date de notification envoyée, date d'acceptation/refus/retrait, motif facultatif de refus (non exposé publiquement).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 95 % des utilisateurs authentifiés ayant participé à au moins une session livestream peuvent ajouter une première ressource (document, vidéo, accompagnateur, lien) en moins de 90 secondes sans aide externe.
- **SC-002**: 100 % des ressources ajoutées par un utilisateur authentifié actif sont visibles par les autres participants présents dans la session en moins de 3 secondes après le dépôt.
- **SC-003**: Le téléversement d'un document de taille moyenne (5 Mo) aboutit avec succès dans 99 % des cas dans des conditions réseau standard.
- **SC-004**: Aucune ressource ne peut être ajoutée par un visiteur non authentifié (taux de tentatives non-authentifiées abouties = 0 %).
- **SC-005**: Une fermeture administrative déclenchée depuis le panneau d'administration interrompt la session live et désactive la salle en moins de 5 secondes pour 99 % des cas.
- **SC-006**: 100 % des tentatives de réactivation effectuées par un non-administrateur sont refusées et tracées.
- **SC-007**: 100 % des fermetures et des réactivations administratives sont retrouvables dans l'historique d'audit d'une salle, avec auteur, horodatage et motif.
- **SC-008**: Sur les 12 mois suivant la mise en production, le délai moyen entre le signalement d'un abus avéré et la fermeture administrative effective est inférieur à 30 minutes (mesure opérationnelle hors plateforme).
- **SC-009**: 90 % des utilisateurs recommandés comme accompagnateurs reçoivent et consultent leur notification de recommandation dans les 24 h suivant le dépôt.

## Assumptions

- **A-1**: L'« administrateur » habilité à fermer une session pour abus et à réactiver une salle désigne le rôle « administrateur de la plateforme » au sens IAM existant, et non l'administrateur d'une salle publique ni le créateur d'une salle privée, cohérent avec le contexte « cas d'abus / niveau plateforme ».
- **A-2**: Les ressources contribuées sont rattachées à la **salle** Afrolang (corpus cumulatif partagé par toutes les sessions de la salle), et non à une session individuelle. L'écran de chaque session affiche l'agrégat de la salle hôte. La session d'origine est conservée comme métadonnée informative mais ne sert pas de filtre.
- **A-3**: La table `afrolang.ressource_salle` modérée (livrée en `005-afrolang-salles`) reste en place et indépendante. La nouvelle table de ressources contribuées s'y ajoute : les deux listes coexistent dans l'écran de la session avec des étiquetages distincts (« Ressources officielles » vs « Ressources contribuées par la communauté »).
- **A-4**: « Accompagnateur » désigne un membre actif de la plateforme recommandé pour partager son expertise sur la langue/culture de la salle. La recommandation requiert le **consentement explicite a posteriori** de la personne recommandée (acceptation via notification) avant publication ; sans acceptation, elle n'apparaît pas dans la liste publique. Aucun engagement contractuel ni obligation de réponse aux sollicitations qui en découleraient.
- **A-5**: Les types de fichiers de documents acceptés couvrent au minimum PDF, DOC, DOCX, ODT ; les formats média natifs (vidéos uploadées) ne sont pas pris en charge dans cette version, la vidéo passe par URL externe pour éviter la surcharge de stockage et de modération.
- **A-6**: La limite par défaut de 10 ressources / salle / utilisateur / 24 h glissantes est un garde-fou ajustable ultérieurement par configuration, non un choix produit définitif.
- **A-7**: La désactivation d'une salle par administration suspend la jointure et la création de nouvelles sessions, mais ne supprime ni la salle ni ses contenus historiques (ressources, messages persistés) : elle reste « gelée » jusqu'à réactivation.
- **A-8**: Les ressources ajoutées sont publiées immédiatement sans modération a priori (cohérent avec « ajoutable par tout utilisateur connecté ») ; la modération est a posteriori via la capacité de retrait administratif (FR-008) et la fermeture administrative en cas de dérive systémique.

## Dependencies

- Système IAM existant (rôles « administrateur de la plateforme », « utilisateur actif »).
- Modèle de salle et session Afrolang existant (salles publiques par groupe ethnique, salles privées par code d'accès, sessions livestream LiveKit, modération de session déjà livrée en feature `001-session-moderation`).
- Système d'audit existant (`audit::log_action`).
- Système de notifications existant (typologie `afrolang.*`).
- Stockage de fichiers uploadés (espace `./uploads/` existant, sous-dossier dédié aux ressources de session à créer).
