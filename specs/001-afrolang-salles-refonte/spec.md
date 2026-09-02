# Feature Specification: Refonte salles Afrolang, streaming direct & salles privées par code secret

**Feature Branch**: `001-afrolang-salles-refonte`
**Created**: 2026-04-15
**Status**: Draft
**Input**: User description : « la feature des salles ne fonctionne pas comme prévu, il y a eu un léger écart entre ce que je voulais et ce qui a été implémenté. Voici ce que je voulais :
- sur la page `/afrolang`, pas besoin de la section `Annuaire des groupes ethniques`
- dans la section `Toutes les salles publiques`, le bouton `Démarrer` doit démarrer systématiquement le livestreaming de la salle publique
- une salle est en fait une session de streaming, pas besoin de page `/afrolang/salle-privee/[id].vue`
- une salle privée se crée dans la session livestreaming d'une salle publique ou dans le widget qui révèle la liste des salles privées. Pas besoin d'une page pour créer une salle privée, un modale suffira
- une salle privée (streaming) a pour particularité de pouvoir être créée par n'importe qui à raison d'une salle privée par user par salle publique
- le bouton canal privée révèle l'ensemble des salles privées, c'est bien mais il faut savoir que la seule condition pour entrer dans une salle privée c'est de saisir le code secret de cette salle défini par l'auteur lors de la création ».

## Clarifications

### Session 2026-04-15

- Q: Une salle privée est-elle un objet durable (dormant hors live) ou une session jetable détruite en fin de live ? → A: Objet durable ; la salle privée est un enregistrement persistant (utilisateur auteur, salle publique) qui alterne entre dormant et session live en cours, avec code secret stable tant que l'auteur ne le change pas.
- Q: Que faire des salles privées legacy (ancien modèle adhésions/invitations/modérateurs attitrés) ? → A: Table rase : suppression (hard delete) des salles privées existantes et des tables/colonnes legacy associées ; produit pas encore en production, aucun impact utilisateur.
- Q: Qui peut démarrer / rejoindre une session live d'une salle publique ? → A: **Les salles publiques sont créées exclusivement par un administrateur**. Une fois créées, **n'importe quel utilisateur connecté** peut démarrer une session live ou rejoindre une session en cours : il n'y a pas de notion d'« hôte créateur » côté utilisateur final.
- Q: Où se situe le widget « Canal privé » qui liste les salles privées ? → A: Sur la carte de chaque salle publique dans la liste `/afrolang`, composant dropdown **déjà existant**. La refonte n'ajoute pas un nouvel emplacement, elle cable l'accès aux salles privées via saisie du code secret depuis ce widget. Une salle publique peut contenir plusieurs salles privées (créées par des utilisateurs distincts) ; la règle d'unicité « une salle privée par user par salle publique » reste inchangée côté auteur.
- Q: Le démarrage d'une session privée dépend-il de l'état de la salle publique parente ? → A: Indépendance totale. Une salle privée peut être créée, démarrée et tourner à tout moment, que la salle publique ait ou non une session live en cours. La salle publique sert uniquement de contexte thématique (langue / groupe ethnique).

## Contexte

La feature « Afrolang : salles » a déjà été livrée (branche `005-afrolang-salles`). Le livrable actuel s'écarte toutefois de l'intention produit sur plusieurs points : une section annuaire non désirée en page d'accueil, un bouton « Démarrer » qui ne lance pas réellement le livestream, une page dédiée pour chaque salle privée, un processus de création de salle privée via page plutôt que modale, un contrôle d'accès basé sur adhésion/invitation plutôt que sur un simple code secret, et une limitation de création de salle privée réservée à certains profils. La présente spécification décrit les ajustements attendus pour remettre la feature en conformité avec l'intention produit.

## User Scenarios & Testing *(mandatory)*

### User Story 1 : Lancer/rejoindre un livestream public en un clic (Priority: P1)

Un visiteur connecté arrive sur la page Afrolang et consulte la liste des salles publiques. D'un clic sur « Démarrer » (ou « Rejoindre » si le live est déjà en cours), il entre immédiatement dans la session de streaming audio/vidéo de la salle publique choisie, sans passer par une page intermédiaire de fiche salle.

**Why this priority** : c'est le cœur de la promesse produit, une salle Afrolang se matérialise par une session de streaming. Sans ce raccourci, l'utilisateur ne comprend pas que « salle = session live » et la feature entière perd son sens.

**Independent Test** : depuis `/afrolang`, cliquer sur le bouton « Démarrer » d'une salle publique et vérifier que l'on entre directement dans l'interface de livestream correspondante (diffusion si hôte, visionnage si spectateur).

**Acceptance Scenarios** :

1. **Given** une salle publique « X » (créée par un administrateur) sans session en cours et un utilisateur connecté quelconque, **When** il clique sur « Démarrer » depuis la liste des salles publiques, **Then** une nouvelle session live est lancée et l'utilisateur entre dans la session comme participant actif (pas de rôle « hôte » exclusif).
2. **Given** une salle publique « X » avec session live en cours et un utilisateur connecté, **When** il clique sur « Rejoindre », **Then** il entre dans la session live en tant que participant.
3. **Given** un utilisateur non connecté, **When** il clique sur « Démarrer » / « Rejoindre », **Then** il est redirigé vers le flux d'authentification puis ramené à la session après connexion.
4. **Given** la page `/afrolang` chargée, **When** l'utilisateur la parcourt, **Then** aucune section « Annuaire des groupes ethniques » n'est présente.
5. **Given** un utilisateur connecté non-administrateur, **When** il explore l'interface publique Afrolang, **Then** aucun point d'entrée ne lui permet de créer une salle publique (création réservée à l'admin via l'interface d'administration).

---

### User Story 2 : Créer sa salle privée depuis un live en un modale (Priority: P1)

Un utilisateur connecté participe à une session live de salle publique. Il souhaite prolonger l'échange avec un cercle restreint : depuis l'interface du livestream, il ouvre un modale « Créer ma salle privée », saisit un titre et un code secret, valide, et sa salle privée est immédiatement créée et associée à la salle publique en cours. Il peut la partager en communiquant le code secret aux personnes de son choix.

**Why this priority** : c'est la nouvelle mécanique d'ouverture de la feature, n'importe quel utilisateur peut créer son cercle privé à la volée sans friction.

**Independent Test** : dans une session livestream publique, ouvrir le modale de création de salle privée, remplir titre + code secret, soumettre, et vérifier que la salle privée apparaît dans le widget « Canal privé » de la salle publique.

**Acceptance Scenarios** :

1. **Given** un utilisateur connecté dans la session livestream de la salle publique « X » et qui n'a pas encore de salle privée rattachée à « X », **When** il ouvre le modale « Créer ma salle privée », saisit un titre et un code secret valides, et valide, **Then** la salle privée est créée, l'utilisateur en est l'auteur et elle est listée dans le « Canal privé » de « X ».
2. **Given** un utilisateur connecté qui a déjà créé une salle privée rattachée à la salle publique « X », **When** il tente d'en créer une seconde pour la même salle publique, **Then** la création est refusée avec un message explicite (« Vous avez déjà une salle privée pour cette salle publique ») et, à la place, un raccourci « Ouvrir ma salle privée » est proposé.
3. **Given** le même utilisateur, **When** il se trouve dans la session livestream d'une autre salle publique « Y » pour laquelle il n'a pas encore de salle privée, **Then** il peut créer une salle privée distincte pour « Y ».
4. **Given** l'ancienne page `/afrolang/salle-privee/[id].vue`, **When** un utilisateur tente d'y accéder, **Then** il est redirigé vers le parcours attendu (session live publique parente ou liste des salles publiques), cette page n'existe plus comme destination.

---

### User Story 3 : Accéder à une salle privée par code secret (Priority: P1)

Un utilisateur connecté, dans le contexte d'une salle publique (que ce soit depuis la liste des salles publiques ou depuis une session live en cours), ouvre le widget « Canal privé » qui révèle les salles privées rattachées à cette salle publique. Il choisit une salle privée, saisit le code secret que l'auteur lui a communiqué, et entre directement dans la session de streaming privée.

**Why this priority** : c'est le nouveau modèle d'accès, léger, sans workflow d'adhésion/invitation. Sans lui, les salles privées ne sont pas accessibles selon la règle produit.

**Independent Test** : ouvrir le widget « Canal privé », sélectionner une salle privée listée, entrer un code secret correct, et vérifier que l'utilisateur entre dans la session privée ; entrer un code incorrect et vérifier que l'accès est refusé avec message clair.

**Acceptance Scenarios** :

1. **Given** une salle privée « P » rattachée à la salle publique « X », **When** un utilisateur connecté ouvre « Canal privé » depuis « X » et saisit le code secret correct de « P », **Then** il entre dans la session de streaming privée « P ».
2. **Given** un code secret incorrect, **When** l'utilisateur soumet, **Then** l'accès est refusé avec un message « Code incorrect » et la saisie est proposée à nouveau.
3. **Given** l'auteur de la salle privée « P », **When** il ouvre « P » depuis le widget « Canal privé », **Then** il entre directement sans saisie de code (l'auteur est reconnu par son identité authentifiée).
4. **Given** l'utilisateur a déjà saisi le bon code secret au cours de sa session applicative, **When** il rouvre la même salle privée dans la même session, **Then** son accès est conservé sans re-saisie (décision d'ergonomie, voir Assumptions).

---

### User Story 4 : Créer/ouvrir sa salle privée depuis le widget « Canal privé » (Priority: P2)

Un utilisateur connecté consulte une salle publique (ou sa session live) et ouvre le widget « Canal privé ». Au-dessus (ou à côté) de la liste des salles privées existantes, il voit un bouton « Créer ma salle privée » qui ouvre le même modale de création que depuis le live. La règle « une salle privée par user par salle publique » s'applique : s'il en a déjà une, le bouton devient « Ouvrir ma salle privée ».

**Why this priority** : complète l'US2 en offrant un second point d'entrée naturel (liste) pour créer. C'est un doublon fonctionnel utile mais pas bloquant si l'US2 est livrée.

**Independent Test** : ouvrir « Canal privé » depuis une salle publique, cliquer sur « Créer ma salle privée », compléter le modale, vérifier l'apparition de la salle privée dans la liste.

**Acceptance Scenarios** :

1. **Given** un utilisateur sans salle privée pour la salle publique « X », **When** il ouvre le widget « Canal privé » de « X », **Then** il voit un bouton « Créer ma salle privée ».
2. **Given** un utilisateur qui a déjà une salle privée pour « X », **When** il ouvre le widget, **Then** il voit un raccourci « Ouvrir ma salle privée » au lieu du bouton de création.

---

### Edge Cases

- L'utilisateur tente de créer une salle privée sans être connecté → redirection vers authentification puis retour dans le contexte.
- Le code secret saisi est vide ou ne respecte pas les règles de format → validation côté formulaire avant soumission.
- Un visiteur tente de deviner le code secret par essais répétés → rate limit appliqué (voir A3).
- Une salle publique est supprimée ou désactivée alors que des salles privées y sont rattachées → les salles privées associées deviennent inaccessibles et sont également archivées (cohérence avec la règle déjà existante d'archivage en cascade).
- Un utilisateur est déjà dans une session live publique et clique « Démarrer » sur une autre salle publique → il est proposé de quitter la session en cours avant d'en rejoindre une nouvelle.
- Un auteur modifie le code secret de sa salle privée (si permis, voir FR-011) → les utilisateurs déjà à l'intérieur conservent leur accès pour la session courante, les nouveaux entrants doivent utiliser le nouveau code.
- Données existantes : des salles privées ont pu être créées sous l'ancien modèle (adhésions, invitations, modérateurs attitrés). La présente refonte remplace ces mécanismes par le code secret ; la stratégie de reprise des données est traitée dans la section Assumptions (A4).

## Requirements *(mandatory)*

### Functional Requirements

#### Page `/afrolang`

- **FR-001** : La page `/afrolang` NE DOIT PLUS afficher la section « Annuaire des groupes ethniques ». Les autres sections existantes sont conservées.
- **FR-002** : Dans la section « Toutes les salles publiques », chaque carte DOIT proposer un bouton principal « Démarrer » (ou libellé équivalent selon le contexte : « Rejoindre » si une session est déjà en cours) qui lance/rejoint directement le livestream de la salle publique.
- **FR-003** : Le clic sur ce bouton DOIT emmener l'utilisateur dans l'interface de session de streaming en une navigation (pas de fiche intermédiaire imposée).

#### Salles publiques & livestream

- **FR-004** : Le système DOIT traiter « salle publique » et « session de streaming » comme indissociables du point de vue parcours : entrer dans une salle publique = entrer dans sa session live (en cours ou nouvellement lancée).
- **FR-005** : Les salles publiques DOIVENT être créées exclusivement par un administrateur (via l'interface d'administration). Aucun utilisateur final non-admin ne DOIT pouvoir créer une salle publique.
- **FR-005b** : Une fois une salle publique créée, **n'importe quel utilisateur connecté** DOIT pouvoir démarrer une nouvelle session live (si aucune n'est en cours) ou rejoindre la session en cours. Il n'existe pas de rôle d'« hôte exclusif » attaché à la salle ; tous les participants connectés ont les mêmes droits d'accès à la session.

#### Suppression page salle privée

- **FR-006** : La route/page `/afrolang/salle-privee/[id].vue` NE DOIT PLUS exister comme destination accessible. Toute tentative d'accès DOIT être redirigée vers un parcours valide (salle publique parente ou liste des salles publiques).
- **FR-007** : Toutes les interactions liées à une salle privée (création, accès, gestion minimale par l'auteur) DOIVENT avoir lieu via modale(s) et widget, sans navigation vers une page dédiée.

#### Création salle privée par modale

- **FR-008** : Un modale de création de salle privée DOIT être accessible depuis (a) le widget « Canal privé » dropdown déjà présent sur la carte de chaque salle publique dans `/afrolang`, et (b) l'interface de session livestream d'une salle publique (pour créer à la volée pendant un live). Les deux points d'entrée ouvrent le même modale.
- **FR-009** : N'importe quel utilisateur connecté DOIT pouvoir créer une salle privée ; aucune restriction de rôle ou de profil supplémentaire ne s'applique.
- **FR-010** : Le système DOIT appliquer la règle **une salle privée maximum par utilisateur par salle publique**. Toute tentative de création supplémentaire pour la même salle publique DOIT être refusée avec message explicite et proposer l'ouverture de la salle privée existante.
- **FR-011** : À la création, l'auteur DOIT définir au minimum un titre et un code secret. L'auteur peut modifier ultérieurement le code secret de sa propre salle privée.

#### Accès par code secret

- **FR-012** : Le widget « Canal privé » (dropdown **déjà présent sur chaque carte de salle publique** dans la liste `/afrolang`) DOIT lister toutes les salles privées durables rattachées à la salle publique concernée (titre, auteur, état dormante / session live en cours). Une même salle publique PEUT contenir plusieurs salles privées créées par des utilisateurs distincts. Les salles dormantes restent listées et accessibles par leur auteur (qui peut les rouvrir) ou par tout utilisateur disposant du code secret (qui pourra y entrer dès qu'une session live y est active).
- **FR-013** : Pour entrer dans une salle privée, un utilisateur connecté autre que l'auteur DOIT saisir le code secret défini par l'auteur. **Aucun autre mécanisme d'accès n'est requis** (suppression des adhésions, invitations, modérateurs attitrés sur les salles privées).
- **FR-014** : L'auteur d'une salle privée DOIT pouvoir entrer dans sa propre salle sans saisir le code secret, son identité authentifiée suffisant.
- **FR-015** : Une saisie de code incorrecte DOIT être refusée avec message clair et NE DOIT PAS divulguer d'information susceptible de faciliter le devinement (ex. ne pas confirmer que la salle existe/est vide).

#### Couplage salle privée ↔ salle publique

- **FR-016** : Une salle privée DOIT toujours être rattachée à exactement une salle publique (son contexte parent thématique : langue / groupe ethnique).
- **FR-017** : Si la salle publique parente est archivée/désactivée par un administrateur, ses salles privées rattachées DOIVENT l'être également (archivage en cascade, cohérent avec la logique existante).
- **FR-018** : Une salle privée est **indépendante** de l'état de session de sa salle publique parente. Elle peut être créée, démarrée, rejointe et tourner à tout moment, y compris si la salle publique n'a aucune session live en cours. Le lien salle publique ↔ salle privée est uniquement un rattachement thématique.

### Key Entities *(include if feature involves data)*

- **Salle publique** : unité Afrolang associée à un groupe ethnique / une langue, créée exclusivement par un administrateur. Peut avoir au plus une session live en cours. Aucun utilisateur final n'en est propriétaire.
- **Session de streaming (publique)** : matérialisation temporelle d'une salle publique ; démarrée par n'importe quel utilisateur connecté lorsqu'aucune session n'est en cours ; rejointe par tout utilisateur connecté tant qu'elle est active.
- **Salle privée** : objet durable rattaché à une salle publique unique (contexte thématique), créé par n'importe quel utilisateur connecté, identifié par un titre et protégé par un code secret défini par l'auteur. Alterne entre états « dormante » (aucune session live en cours) et « session live en cours » ; l'auteur peut rouvrir la salle à volonté, **indépendamment de l'état de la salle publique parente**. Règle d'unicité : au plus un enregistrement par couple (utilisateur auteur, salle publique). Une même salle publique peut donc contenir plusieurs salles privées créées par des utilisateurs différents.
- **Code secret** : chaîne définie par l'auteur d'une salle privée à la création, modifiable par l'auteur, vérifiée à l'entrée de tout utilisateur non-auteur.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001** : Depuis `/afrolang`, un utilisateur connecté entre dans une session live publique en **≤ 2 clics** (carte + « Démarrer »/« Rejoindre ») et en **≤ 3 secondes** de navigation perçue.
- **SC-002** : 100 % des cartes de la section « Toutes les salles publiques » exposent un bouton fonctionnel qui lance ou rejoint réellement le livestream (aucune carte ne mène à une fiche sans live).
- **SC-003** : La section « Annuaire des groupes ethniques » est absente de `/afrolang` (audit visuel + absence du conteneur associé).
- **SC-004** : Un utilisateur peut créer sa salle privée depuis un modale en **≤ 4 champs à saisir** et **≤ 15 secondes** entre ouverture du modale et confirmation de création.
- **SC-005** : La règle d'unicité « 1 salle privée par user par salle publique » est respectée à 100 % (toute seconde création pour la même salle publique par le même utilisateur est refusée côté produit et côté données).
- **SC-006** : L'accès à une salle privée par un non-auteur avec code correct réussit en **≤ 2 secondes** ; l'accès avec code incorrect échoue avec message explicite et protection contre les essais répétés (voir A3).
- **SC-007** : Aucun parcours utilisateur ne dépend de l'ancienne page `/afrolang/salle-privee/[id].vue` après livraison (audit de routes + redirection en place).
- **SC-008** : Parcours d'onboarding de la feature : ≥ 90 % des utilisateurs testés comprennent au premier essai que « une salle = une session de streaming » (mesuré via test utilisateur qualitatif ou taux de clic direct sur « Démarrer » sans détour par une fiche).

## Assumptions

- **A1 : Auteur salle publique et salle privée** : le créateur d'une salle publique peut lui-même créer au plus une salle privée rattachée à sa propre salle publique (il est un utilisateur comme un autre au regard de FR-010).
- **A2 : Mémorisation de l'accès code** : une fois le code secret correctement saisi par un utilisateur pour une salle privée donnée, l'accès est maintenu pour la durée de sa session applicative courante, afin d'éviter la re-saisie. La mémorisation se termine avec la session.
- **A3 : Protection contre essais répétés** : un rate limit s'applique à la vérification du code secret (ex. 5 tentatives par minute par utilisateur et par salle privée, valeur à caler lors du design technique) pour contrer le devinement par force brute.
- **A4 : Reprise des données existantes : table rase.** Le produit n'étant pas encore en production, toutes les salles privées créées sous l'ancien modèle sont supprimées (hard delete) et les tables/colonnes legacy propres aux salles privées sont retirées : `afrolang.salle_privee_adhesion`, mécanismes d'invitation et de modérateurs attitrés spécifiques aux salles privées. Aucune migration de données n'est requise. La modération admin générale et les autres tables Afrolang (propositions, messagerie, ressources, modérateurs de salles publiques) restent inchangées.
- **A5 : Format du code secret** : chaîne saisie libre par l'auteur, longueur minimale raisonnable (ex. 4 caractères) et maximale bornée ; pas d'exigence de complexité forte (code convivial à communiquer à l'oral/écrit), la protection principale reposant sur la non-divulgation et le rate limit (A3).
- **A6 : Un seul live actif par utilisateur** : un utilisateur ne peut être présent que dans une seule session live (publique ou privée) à la fois ; changer de salle ferme la précédente.
- **A7 : Périmètre inchangé ailleurs** : les autres fonctionnalités Afrolang déjà livrées (tableau blanc, chat, ressources dans les salles publiques, messagerie écrite, etc.) ne sont pas remises en cause par la présente refonte.

## Out of Scope

- Refonte du design global de la page `/afrolang` au-delà du retrait de l'annuaire.
- Évolution des fonctionnalités en séance (tableau blanc, chat, ressources), hors périmètre.
- Modération administrative des salles privées par des tiers (le modèle adhésion/invitation/modérateurs attitrés est supprimé côté salles privées ; la modération admin globale des contenus Afrolang reste inchangée).
- Enregistrement vidéo / rejouabilité des sessions privées (non adressé ici).
