# Feature Specification: Administrateurs de salle publique & propositions communautaires

**Feature Branch**: `001-admin-salles-publiques`
**Created**: 2026-05-10
**Status**: Draft
**Input**: User description: "Faire en sorte qu'on puisse créer des administrateurs par salle publique. Il aurons des facultés que nous définirons plus tard. Une personne (tout utilisateur connecté) doit pouvoir proposer une salle publique à valider par l'administrateur de la plateforme."

## User Scenarios & Testing

### User Story 1 : Proposer une salle publique en tant que membre (Priority: P1)

Tout utilisateur authentifié de la plateforme peut soumettre une proposition de nouvelle salle publique (langue cible, groupe ethnique, pays d'origine, description, justification). La proposition entre en file d'attente et reste invisible des autres utilisateurs jusqu'à validation par un administrateur de la plateforme.

**Why this priority**: C'est le point d'entrée fonctionnel de toute la fonctionnalité. Sans propositions communautaires, il n'y a ni nouvelle salle publique à modérer, ni administrateur de salle à nommer. C'est aussi la mécanique qui ouvre la création de contenu Afrolang aux contributeurs réguliers.

**Independent Test**: Un utilisateur connecté ouvre le formulaire de proposition, le remplit et le soumet ; il reçoit une confirmation que sa demande est en attente de validation et peut la consulter dans son espace personnel. La file d'attente apparaît côté administrateur de la plateforme. Aucun autre composant (nomination d'admin de salle) n'est requis pour valider ce flux.

**Acceptance Scenarios**:

1. **Given** un utilisateur connecté avec un compte vérifié, **When** il soumet une proposition de salle publique complète, **Then** la proposition est enregistrée avec le statut « en attente », l'utilisateur reçoit une confirmation et la salle proposée n'est visible d'aucun visiteur public ni des autres membres.
2. **Given** un utilisateur non connecté, **When** il tente d'accéder au formulaire de proposition, **Then** il est invité à se connecter et ne peut soumettre aucune proposition.
3. **Given** un utilisateur a déjà une proposition en attente identique (même langue + même groupe ethnique), **When** il en soumet une seconde, **Then** la plateforme l'en informe et bloque la duplication.
4. **Given** un utilisateur connecté, **When** il consulte son espace personnel, **Then** il voit la liste de ses propositions avec leur statut courant (en attente, validée, rejetée) et le commentaire éventuel de l'administrateur de la plateforme.

---

### User Story 2 : Valider ou rejeter une proposition (administrateur de la plateforme) (Priority: P1)

L'administrateur de la plateforme consulte la file d'attente des propositions de salles publiques, examine chaque dossier, puis valide ou rejette la proposition. Une validation crée la salle publique correspondante. Un rejet est accompagné d'un commentaire transmis à l'auteur.

**Why this priority**: C'est l'autre moitié de la boucle de création communautaire de salles publiques. Sans validation, aucune proposition n'aboutit et la fonctionnalité n'a aucun effet visible. Doit donc être livrée avec US1.

**Independent Test**: Avec au moins une proposition existante, l'administrateur de la plateforme peut, depuis son tableau de bord, valider la proposition (la salle publique apparaît alors dans le catalogue Afrolang public) ou la rejeter avec commentaire (l'auteur en est notifié). Testable sans la fonctionnalité d'admin de salle.

**Acceptance Scenarios**:

1. **Given** une proposition « en attente », **When** l'administrateur de la plateforme la valide, **Then** une salle publique est créée avec les informations soumises, devient visible dans le catalogue public, et l'auteur de la proposition est notifié.
2. **Given** une proposition « en attente », **When** l'administrateur de la plateforme la rejette en saisissant un commentaire obligatoire, **Then** la proposition passe au statut « rejetée », aucune salle n'est créée, et l'auteur reçoit la notification avec le commentaire.
3. **Given** une proposition déjà traitée (validée ou rejetée), **When** un autre administrateur ouvre la même proposition, **Then** elle apparaît en lecture seule avec l'identité du décideur, la date et la décision.
4. **Given** une proposition validée, **When** on consulte la salle publique nouvellement créée, **Then** l'auteur d'origine y figure comme contributeur initial (information traçable, sans droit particulier accordé automatiquement).

---

### User Story 3 : Nommer un administrateur de salle publique (Priority: P2)

L'administrateur de la plateforme désigne, pour une salle publique existante, un ou plusieurs utilisateurs comme « administrateurs de cette salle ». Ce statut est porté par un rôle distinct, visible dans l'interface, et constitue le réceptacle de pouvoirs futurs (dont la définition est hors du périmètre de cette spécification).

**Why this priority**: C'est la deuxième moitié de la demande utilisateur, mais elle peut être livrée après la boucle proposition/validation et reste fonctionnelle même sans capacités opérationnelles attachées (la nomination est elle-même un acte significatif et auditable).

**Independent Test**: Sur une salle publique donnée, l'administrateur de la plateforme nomme un membre comme administrateur de la salle. Le membre voit son nouveau statut dans la salle ; les autres visiteurs voient la liste des administrateurs de la salle. Aucun nouveau pouvoir opérationnel n'est requis pour valider ce flux, la nomination en elle-même suffit.

**Acceptance Scenarios**:

1. **Given** une salle publique active et un utilisateur authentifié de la plateforme, **When** l'administrateur de la plateforme nomme cet utilisateur administrateur de cette salle, **Then** la nomination est enregistrée, l'utilisateur est notifié, et son statut apparaît publiquement dans la fiche de la salle.
2. **Given** un utilisateur déjà administrateur de la salle, **When** l'administrateur de la plateforme tente de le nommer à nouveau, **Then** la plateforme empêche le doublon et indique que la personne occupe déjà ce rôle.
3. **Given** un administrateur de salle nommé, **When** l'administrateur de la plateforme révoque sa nomination, **Then** le statut est retiré, l'utilisateur est notifié, et l'historique de la nomination/révocation reste consultable pour audit.
4. **Given** une salle publique avec plusieurs administrateurs, **When** un visiteur consulte la salle, **Then** la liste des administrateurs de la salle est visible (nom, rôle), sans confusion avec l'administrateur de la plateforme.

---

### Edge Cases

- Un utilisateur soumet une proposition pour une salle qui dupliquerait une salle publique déjà active → alerte côté administrateur de la plateforme avant validation, qui peut rejeter avec un motif standardisé.
- L'auteur d'une proposition supprime son compte avant la décision → la proposition reste instructible, l'auteur est marqué « compte clôturé », sans notification.
- Un administrateur de la plateforme nomme un utilisateur dont le compte est ensuite désactivé/supprimé → la nomination est automatiquement suspendue ; à la réactivation éventuelle, elle reste suspendue tant qu'un administrateur ne la rétablit pas explicitement.
- Une salle publique est archivée (cessation d'activité) → toutes les nominations d'administrateurs de cette salle sont automatiquement suspendues sans suppression de l'historique.
- Volume élevé de propositions simultanées sur le même couple langue/groupe ethnique → la première validée crée la salle ; les autres sont automatiquement rejetées avec un motif « salle déjà créée » et lien vers la salle existante.
- Tentatives répétées de soumissions de mauvaise qualité par un même utilisateur → seuil de propositions rejetées récentes au-delà duquel l'utilisateur est temporairement empêché de proposer (durée et seuils précisés en phase de planification).

## Requirements

### Functional Requirements

#### Propositions de salle publique

- **FR-001**: Tout utilisateur authentifié de la plateforme dont le compte est en état actif DOIT pouvoir soumettre une proposition de salle publique.
- **FR-002**: Une proposition DOIT contenir au minimum la langue cible, le groupe ethnique associé, au moins un pays d'origine, un titre, une description et une justification de l'utilité communautaire.
- **FR-003**: La plateforme DOIT empêcher qu'un même utilisateur ait plus d'une proposition « en attente » portant sur le même couple (langue, groupe ethnique).
- **FR-004**: Une proposition DOIT avoir un statut explicite parmi : en attente, validée, rejetée, retirée par l'auteur.
- **FR-005**: L'auteur d'une proposition DOIT pouvoir consulter à tout moment l'état de ses propositions et le commentaire éventuel de l'administrateur de la plateforme.
- **FR-006**: L'auteur d'une proposition DOIT pouvoir retirer une proposition tant qu'elle est « en attente ».
- **FR-007**: Les propositions « en attente » NE DOIVENT être visibles que de leur auteur et des administrateurs de la plateforme.

#### Modération par l'administrateur de la plateforme

- **FR-008**: L'administrateur de la plateforme DOIT pouvoir lister toutes les propositions, filtrer par statut, langue, groupe ethnique, auteur et date de soumission.
- **FR-009**: L'administrateur de la plateforme DOIT pouvoir valider une proposition « en attente », ce qui crée la salle publique correspondante de manière atomique.
- **FR-010**: L'administrateur de la plateforme DOIT pouvoir rejeter une proposition « en attente » avec un commentaire obligatoire transmis à l'auteur.
- **FR-011**: Toute décision (validation, rejet) DOIT enregistrer le décideur, la date et le commentaire, et figer la proposition en lecture seule.
- **FR-012**: L'auteur DOIT être notifié de toute décision sur sa proposition.

#### Administrateurs de salle publique

- **FR-013**: L'administrateur de la plateforme DOIT pouvoir désigner, pour une salle publique active, un ou plusieurs utilisateurs authentifiés comme « administrateurs de cette salle ».
- **FR-014**: La plateforme DOIT empêcher qu'un même utilisateur soit nommé deux fois administrateur d'une même salle simultanément.
- **FR-015**: L'administrateur de la plateforme DOIT pouvoir révoquer la nomination d'un administrateur de salle à tout moment, avec un motif.
- **FR-016**: L'historique complet des nominations et révocations d'administrateurs de salle DOIT être conservé et consultable à des fins d'audit.
- **FR-017**: La liste des administrateurs d'une salle publique DOIT être publiquement visible sur la fiche de cette salle, distinctement marquée comme rôle propre à la salle (non confondu avec l'administrateur de la plateforme).
- **FR-018**: La plateforme DOIT distinguer, partout où des rôles sont affichés, « administrateur de la plateforme » et « administrateur de la salle X ».
- **FR-019**: Les capacités opérationnelles concrètes attachées au rôle « administrateur de salle publique » SONT HORS PÉRIMÈTRE de cette spécification ; le rôle est créé et nommable, mais ses pouvoirs effectifs feront l'objet d'une spécification ultérieure. La plateforme DOIT toutefois exposer ce rôle via un point d'autorisation centralisé permettant d'y attacher des capacités plus tard sans rupture de compatibilité.
- **FR-020**: La nomination ou la révocation d'un administrateur de salle DOIT déclencher une notification à l'utilisateur concerné.

#### Cycle de vie

- **FR-021**: Lorsqu'une salle publique est archivée ou désactivée, toutes les nominations d'administrateurs de cette salle DOIVENT être automatiquement suspendues (statut neutre, sans suppression d'historique).
- **FR-022**: Lorsqu'un compte utilisateur est désactivé ou supprimé, ses nominations d'administrateur de salle DOIVENT être automatiquement suspendues, et ses propositions « en attente » DOIVENT rester instructibles tout en marquant l'auteur « compte clôturé ».
- **FR-023**: Toutes les actions clés (soumission, retrait, validation, rejet, nomination, révocation, suspension automatique) DOIVENT être tracées dans le journal d'audit existant, avec auteur, date, cible et motif.

### Key Entities

- **Proposition de salle publique** : demande créée par un utilisateur authentifié pour obtenir la création d'une salle publique. Porte les informations de la salle souhaitée (langue, groupe ethnique, pays d'origine, titre, description, justification), un statut (en attente / validée / rejetée / retirée), un auteur, une date de soumission, et, après décision, un décideur, une date de décision et un commentaire.
- **Salle publique** : entité existante de la plateforme. Étendue ici par la possibilité d'avoir une liste d'administrateurs propres, et par une trace de la proposition d'origine (le cas échéant).
- **Nomination d'administrateur de salle** : lien entre un utilisateur et une salle publique, traduisant l'attribution du rôle « administrateur de cette salle ». Porte un statut (active / révoquée / suspendue), une date de nomination, le décideur, une éventuelle date de révocation et un motif.
- **Notification** : message envoyé à l'auteur d'une proposition (décision) ou à un utilisateur nommé/révoqué (changement de rôle de salle).
- **Entrée d'audit** : enregistrement traçant toute action structurante (soumission, retrait, validation, rejet, nomination, révocation, suspension automatique), réutilise le mécanisme d'audit existant.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Au moins 80 % des propositions soumises par des utilisateurs reçoivent une décision (validation ou rejet motivé) en moins de 7 jours calendaires.
- **SC-002**: Un utilisateur authentifié peut soumettre une proposition complète en moins de 5 minutes lors de sa première utilisation, sans assistance externe.
- **SC-003**: Aucune proposition « en attente » n'est visible d'un utilisateur autre que son auteur ou un administrateur de la plateforme (vérifiable par audit ciblé : 0 fuite sur 100 contrôles aléatoires).
- **SC-004**: 100 % des décisions sur les propositions sont notifiées à leur auteur dans les 60 secondes suivant la décision.
- **SC-005**: 100 % des nominations et révocations d'administrateurs de salle sont consultables dans l'historique d'audit, avec auteur et horodatage, indéfiniment.
- **SC-006**: Sur la fiche publique d'une salle, un visiteur identifie en moins de 10 secondes les administrateurs de cette salle et les distingue de l'administrateur de la plateforme.
- **SC-007**: Le taux de doublons (deux propositions actives identiques d'un même auteur) est strictement nul après mise en production.
- **SC-008**: 100 % des nominations d'administrateurs de salle sont automatiquement suspendues dans les 60 secondes suivant l'archivage de la salle ou la désactivation du compte de l'utilisateur nommé.

## Assumptions

- La notion existante d'« administrateur de la plateforme » (rôle global) reste inchangée ; cette fonctionnalité ne crée qu'un nouveau rôle scopé à la salle.
- Les capacités effectives du rôle « administrateur de salle publique » feront l'objet d'une spécification ultérieure ; cette spécification se contente de créer le rôle, sa nomination, sa révocation, sa visibilité publique et son intégration au modèle d'autorisation pour pouvoir y attacher des pouvoirs sans refonte.
- Les propositions concernent uniquement les salles publiques (pas les salles privées, qui restent créables directement par tout utilisateur via le flux existant).
- Le mécanisme de notification réutilise le canal de notifications existant (in-app, e-mail au minimum pour les décisions de modération).
- La traçabilité s'appuie sur le service d'audit existant, sans nouveau mécanisme parallèle.
- Un utilisateur dont la proposition a été validée n'est pas automatiquement nommé administrateur de la salle créée ; il est seulement enregistré comme contributeur d'origine.
