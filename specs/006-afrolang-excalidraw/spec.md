# Feature Specification: Migration du tableau blanc Afrolang vers Excalidraw

**Feature Branch**: `006-afrolang-excalidraw`
**Created**: 2026-04-24
**Status**: Draft
**Input**: Migrer le tableau blanc collaboratif Afrolang de tldraw v4 vers Excalidraw (self-hosted, open-source, licence MIT) pour résoudre la disparition automatique de la barre d'outils en production, causée par le mécanisme anti-contournement de licence commerciale de tldraw v3+/v4.

## Clarifications

### Session 2026-04-24

- Q: Qui est autorisé à dessiner sur le tableau blanc d'une session ? → A: Tous les participants peuvent dessiner par défaut ; seule la commande « Effacer tout » reste réservée au modérateur.
- Q: Comment résoudre les éditions simultanées du même élément par plusieurs participants ? → A: Last-write-wins global — la dernière scène reçue écrase intégralement l'état local, sans fusion par élément ni verrouillage.
- Q: Quelle est la taille maximale cible d'une session (participants actifs connectés simultanément) ? → A: Jusqu'à 100 participants actifs par session.
- Q: Que se passe-t-il après une coupure transitoire du canal temps réel en cours de session ? → A: Resynchronisation automatique sur le dernier snapshot serveur dès la reconnexion, puis reprise normale de la diffusion temps réel.
- Q: Les participants peuvent-ils insérer des images dans le tableau blanc, et si oui sous quelles limites ? → A: Autorisé pour tous les participants, avec limite côté client de 2 Mo par image et formats restreints à JPEG/PNG ; images refusées côté client avec message utilisateur.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Un animateur Afrolang dispose d'un tableau blanc qui reste utilisable pendant toute la session (Priority: P1)

Un enseignant/animateur de groupe ethnique ouvre sa salle Afrolang (publique ou privée) sur la plateforme en production (`www.africans-world.org`), démarre la session avec ses apprenants, ouvre le tableau blanc intégré et s'en sert pendant 30 à 90 minutes pour illustrer du vocabulaire, tracer des cartes, annoter des textes. Pendant toute la durée de la session, l'ensemble des outils de dessin reste visible et cliquable sans qu'aucun watermark ou désactivation d'interface ne vienne interrompre le cours.

**Why this priority** : c'est la contrainte bloquante qui déclenche la migration. Aujourd'hui, la barre d'outils du tableau blanc disparaît d'elle-même au bout de quelques secondes en production, rendant la fonctionnalité totalement inutilisable pour les animateurs et leurs apprenants. Sans cette correction, le tableau blanc Afrolang n'a aucune valeur pratique sur l'environnement public.

**Independent Test** : ouvrir une session Afrolang en production (salle publique puis salle privée), activer le tableau blanc, laisser la page ouverte et interagir avec les outils pendant au moins 15 minutes ; la barre d'outils doit rester présente et fonctionnelle en continu, sans watermark ni blocage d'UI.

**Acceptance Scenarios** :

1. **Given** un animateur modérateur connecté à une salle Afrolang publique en production, **When** il ouvre le tableau blanc et interagit pendant 15 minutes, **Then** tous les outils de dessin restent visibles et réactifs sans interruption.
2. **Given** un participant rejoint une salle Afrolang privée via code secret en production, **When** il ouvre le tableau blanc pour suivre la leçon, **Then** la barre d'outils s'affiche immédiatement et ne disparaît pas au cours de la session.
3. **Given** une session Afrolang ouverte depuis plus de 60 minutes, **When** un utilisateur retourne sur l'onglet du tableau blanc, **Then** l'interface demeure pleinement opérationnelle sans aucun indicateur de licence manquante.

---

### User Story 2 — Les participants d'une session voient les tracés des autres en quasi-temps réel (Priority: P2)

Dans une salle Afrolang active, le modérateur trace un mot, une carte ou une illustration ; tous les apprenants connectés voient l'ajout apparaître sur leur propre tableau blanc en moins d'une demi-seconde. Les opérations inverses (un apprenant qui annote librement, sans autorisation individuelle préalable) sont également diffusées à l'ensemble du groupe. La sensation générale est celle d'un tableau partagé, non d'une vue dédoublée.

**Why this priority** : la collaboration temps réel est le cœur métier d'Afrolang. Toutefois, elle dépend fonctionnellement de la correction du bug P1 : si la barre d'outils disparaît, il n'y a rien à collaborer. Une fois le P1 corrigé, cette parité fonctionnelle avec l'ancienne implémentation devient le critère de non-régression le plus important.

**Independent Test** : ouvrir deux navigateurs sur deux comptes différents dans la même session Afrolang, tracer une forme côté A, vérifier qu'elle apparaît côté B en moins de 500 ms ; puis déplacer un élément, le supprimer, ajouter du texte — chaque opération doit se répliquer dans les deux sens sans boucle d'écho ni duplication.

**Acceptance Scenarios** :

1. **Given** deux participants connectés à la même session, **When** l'un trace un élément, **Then** l'autre voit l'élément apparaître en moins de 500 ms.
2. **Given** un élément partagé visible par deux participants, **When** l'un le déplace ou le supprime, **Then** la modification est reflétée côté pair sans réapparition fantôme.
3. **Given** deux participants dessinent simultanément, **When** leurs opérations sont diffusées, **Then** aucune opération ne revient en boucle sur son auteur d'origine (pas d'écho).

---

### User Story 3 — Un modérateur retrouve son tableau à l'identique après fermeture et réouverture (Priority: P2)

Un modérateur trace du contenu pédagogique pendant la session, ferme le tableau blanc (ou quitte puis revient sur la page), et rouvre le tableau : le contenu précédent est automatiquement restauré à son dernier état sauvegardé. Lorsqu'il souhaite repartir de zéro, un bouton « Effacer tout » lui permet de vider le tableau pour l'ensemble des participants en une action.

**Why this priority** : la persistance est indispensable pour les sessions longues et pour permettre une reprise de cours. Elle doit être préservée à l'identique de l'implémentation actuelle (le contrat de persistance côté serveur ne change pas).

**Independent Test** : en tant que modérateur, tracer du contenu, fermer l'onglet, rouvrir la session depuis la même URL ; le dernier snapshot doit être restauré. Cliquer sur « Effacer tout » ; le tableau se vide chez tous les participants et aucun contenu n'est rechargé au prochain accès.

**Acceptance Scenarios** :

1. **Given** un modérateur a dessiné du contenu et attendu au moins 30 secondes, **When** il ferme puis rouvre le tableau blanc, **Then** le dernier contenu tracé est restauré.
2. **Given** plusieurs participants sont présents avec du contenu affiché, **When** le modérateur clique sur « Effacer tout », **Then** le tableau se vide simultanément chez tous les participants et à la prochaine ouverture aucun contenu ancien ne reste.
3. **Given** seuls les modérateurs disposent du droit d'effacement global, **When** un participant non modérateur inspecte l'interface, **Then** le bouton « Effacer tout » n'est pas accessible.

---

### User Story 4 — Mode dégradé lorsque la connexion temps réel est indisponible (Priority: P3)

Lorsqu'un utilisateur ouvre le tableau blanc avant que la connexion temps réel soit établie, ou si celle-ci est coupée, il peut toujours dessiner localement sans crash ni erreur visible. Ses tracés ne sont simplement pas diffusés aux autres ni sauvegardés tant que la connexion n'est pas rétablie.

**Why this priority** : garantit qu'une panne partielle n'empêche pas la préparation individuelle et n'entraîne pas d'erreur console visible.

**Independent Test** : ouvrir le tableau blanc hors salle active (pas de connexion temps réel), tracer des éléments ; l'interface doit rester fluide, aucune erreur ne doit apparaître dans la console, aucun appel serveur ne doit échouer bruyamment.

**Acceptance Scenarios** :

1. **Given** la connexion temps réel est absente ou déconnectée, **When** un utilisateur ouvre et utilise le tableau blanc, **Then** le dessin local fonctionne sans erreur ni interruption.
2. **Given** un modérateur sans connexion temps réel, **When** il dessine, **Then** aucun snapshot n'est envoyé au serveur et aucune erreur visible n'est levée.
3. **Given** un participant perd puis retrouve sa connexion temps réel au cours de la session, **When** la connexion est rétablie, **Then** son tableau se resynchronise automatiquement sur le dernier snapshot persisté et reprend la diffusion/réception temps réel sans intervention manuelle.

---

### Edge Cases

- **Snapshot historique au format incompatible** : si une session contient un snapshot sauvegardé par l'ancien moteur, le tableau blanc doit s'ouvrir sur un état vide plutôt que de crasher ou d'afficher une erreur utilisateur.
- **Participant rejoignant en cours de session** : un nouvel arrivant doit voir l'état courant du tableau au moment où il ouvre l'outil (via le dernier snapshot persisté), et non un tableau vide.
- **Double modérateur** : si plusieurs modérateurs envoient des snapshots simultanément, le dernier reçu fait foi ; aucune corruption de persistance ne doit survenir.
- **Édition simultanée du même élément** : si deux participants modifient simultanément une même forme (déplacement + redimensionnement par exemple), la dernière scène reçue fait foi et écrase intégralement l'état local ; l'opération perdante est visuellement abandonnée, sans erreur ni notification utilisateur.
- **Coupure réseau transitoire en cours de session** : lorsque le canal temps réel revient après une coupure (wifi défaillant, bascule réseau), le participant se resynchronise automatiquement sur le dernier snapshot persisté ; les opérations produites localement pendant la coupure ne sont ni diffusées ni persistées rétroactivement et peuvent être perdues si un snapshot plus récent les écrase.
- **Session sans interaction** : un tableau blanc ouvert sans aucun tracé ne doit pas produire d'erreurs périodiques liées à l'envoi d'un snapshot vide.
- **Fermeture brutale de l'onglet** : à la fermeture, un dernier snapshot doit être tenté si possible, mais l'absence de flush ne doit pas bloquer la fermeture.
- **Internationalisation** : l'interface du tableau blanc doit s'afficher en français par défaut pour cohérence avec le reste de la plateforme.
- **Image trop lourde ou format invalide** : lorsqu'un participant tente de coller ou importer une image qui dépasse 2 Mo ou n'est ni JPEG ni PNG, un message d'erreur utilisateur explicite est affiché localement ; l'image n'est ni ajoutée au tableau local, ni diffusée, ni persistée.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001** : Le tableau blanc MUST permettre de tracer, sélectionner, déplacer, supprimer, ajouter du texte, des formes, des flèches et du dessin libre depuis un navigateur desktop moderne.
- **FR-001a** : Le tableau blanc MUST permettre à tout participant de coller ou importer des images bitmap dans le tableau, à condition que chaque image respecte une taille maximale de 2 Mo et soit au format JPEG ou PNG ; les images qui ne respectent pas ces limites MUST être refusées côté client avec un message utilisateur explicite, sans jamais être diffusées aux autres participants ni persistées.
- **FR-002** : Le tableau blanc MUST diffuser chaque opération locale aux autres participants connectés à la même session via le canal temps réel existant, sans nécessiter de nouvelle infrastructure serveur.
- **FR-003** : Le tableau blanc MUST appliquer les opérations reçues des pairs sans générer de boucle d'écho (l'auteur d'une opération ne doit jamais recevoir sa propre opération en retour).
- **FR-004** : Lorsqu'un utilisateur ouvre le tableau d'une session existante, le système MUST restaurer automatiquement le dernier état sauvegardé pour cette session.
- **FR-005** : Le modérateur MUST déclencher un enregistrement complet de l'état du tableau au plus toutes les 30 secondes pendant la session, ainsi qu'un enregistrement final au moment de la fermeture.
- **FR-006** : Seul un modérateur MUST pouvoir déclencher la commande « Effacer tout », qui vide le tableau chez tous les participants et réinitialise l'état persisté.
- **FR-007** : Le bouton « Effacer tout » MUST être masqué pour tout participant sans rôle modérateur.
- **FR-007a** : Tout participant connecté à la session (modérateur ou non) MUST pouvoir dessiner, sélectionner, déplacer, supprimer et annoter librement les éléments du tableau ; aucune autorisation individuelle n'est requise pour prendre la main.
- **FR-008** : Si la connexion temps réel est indisponible, le dessin local MUST continuer à fonctionner sans erreur visible, sans diffusion ni persistance.
- **FR-009** : Si un état persisté ancien n'est pas reconnu par le nouveau moteur, le tableau MUST s'ouvrir sur un état vide sans lever d'erreur utilisateur.
- **FR-010** : L'interface du tableau blanc MUST être présentée en français par défaut.
- **FR-011** : La barre d'outils de dessin MUST rester visible et interactive pendant toute la durée d'une session, sans auto-dissimulation ni watermark imposé par le moteur choisi.
- **FR-012** : Le composant pédagogique qui encapsule le tableau (vue split-screen de la salle Afrolang) MUST conserver la même interface d'intégration (mêmes informations transmises : identifiant de session, rôle modérateur, canal temps réel) ; aucun changement n'est requis dans les pages de salles publiques ou privées qui l'utilisent.
- **FR-013** : Le contrat de persistance serveur (récupération, enregistrement, effacement d'un tableau par session) MUST rester inchangé ; seul le contenu sérialisé évolue de format.
- **FR-014** : En cas d'éditions concurrentes sur le même élément par plusieurs participants, le système MUST appliquer une règle de convergence « last-write-wins global » : la dernière scène reçue remplace intégralement l'état local affiché, sans fusion granulaire par élément ni mécanisme de verrouillage.
- **FR-015** : Le tableau blanc MUST rester fonctionnel (dessin local réactif, diffusion temps réel, persistance) jusqu'à 100 participants actifs connectés simultanément dans la même session Afrolang, sans dégradation perceptible de l'expérience utilisateur individuelle.
- **FR-016** : Lorsque le canal temps réel est rétabli après une coupure transitoire en cours de session, le tableau blanc MUST automatiquement récupérer le dernier snapshot persisté côté serveur, l'appliquer comme état courant, puis reprendre la diffusion et la réception des opérations temps réel. Aucune action manuelle de l'utilisateur (rechargement, fermeture/réouverture) ne doit être nécessaire pour retrouver un état cohérent.

### Key Entities

- **Session Afrolang** : unité pédagogique identifiée, publique ou privée, à laquelle est associé au plus un état de tableau blanc persistant.
- **État du tableau (snapshot)** : représentation complète et sérialisable du contenu graphique d'un tableau à un instant donné, associée à une session ; remplace et écrase l'état précédent à chaque enregistrement.
- **Opération de tableau** : événement diffusé aux pairs décrivant les éléments présents sur le tableau après une interaction locale ; n'est pas persisté individuellement mais alimente la vue des participants en temps réel.
- **Rôle modérateur** : attribut d'un participant conférant le droit de déclencher les enregistrements périodiques et l'effacement global du tableau.

### Out of Scope

- Ajout de curseurs collaboratifs en temps réel (position live des pointeurs des autres participants) — peut être étudié ultérieurement.
- Partage de bibliothèques de formes entre sessions.
- Conversion ou reprise des anciens états persistés dans l'ancien format (traités par lecture défensive retournant un tableau vide).
- Support tactile/mobile (desktop uniquement pour cette itération).
- Amélioration de la latence réseau au-delà du canal temps réel existant.
- Modification des pages de salle publique et privée, ni du composant conteneur qui monte le tableau dans la vue Afrolang : l'intégration reste transparente.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001** : Sur l'environnement de production, 100 % des sessions Afrolang (publiques et privées) disposent d'une barre d'outils de tableau blanc visible et fonctionnelle pendant toute leur durée, sans aucune disparition automatique.
- **SC-002** : Dans 95 % des mesures réalisées sur un réseau standard, une opération de dessin produite par un participant est visible chez les autres participants de la même session en moins de 500 millisecondes.
- **SC-003** : 100 % des sessions pour lesquelles un modérateur a dessiné du contenu restaurent ce contenu lors d'une réouverture ultérieure, tant qu'aucun effacement global n'a eu lieu.
- **SC-004** : 0 erreur visible dans la console navigateur lors de l'ouverture, de l'utilisation et de la fermeture du tableau blanc en mode nominal comme en mode dégradé (connexion temps réel absente).
- **SC-005** : 0 dépendance restante à une librairie graphique imposant une licence commerciale pour un usage en production.
- **SC-006** : Aucune régression fonctionnelle n'est observée sur les pages de salle publique et privée : le tableau blanc s'ouvre, se ferme et se rouvre aux mêmes emplacements et aux mêmes moments qu'avant la migration.
- **SC-007** : La fonctionnalité « Effacer tout » réinitialise le tableau chez 100 % des participants connectés et garantit qu'aucun ancien contenu n'est rechargé à la prochaine ouverture.
- **SC-008** : Une session contenant jusqu'à 100 participants actifs simultanés conserve une latence de diffusion sous 500 ms (SC-002) et une barre d'outils pleinement fonctionnelle pour chaque participant, sans dégradation perçue.

## Assumptions

- Les navigateurs utilisés sont des versions récentes (≤ 12 mois) de Chrome, Firefox, Safari ou Edge ; le support des anciens navigateurs n'est pas visé.
- Les snapshots antérieurs persistés peuvent être ignorés car la fonctionnalité n'a jamais été exploitable en production (aucun contenu pédagogique de valeur à migrer).
- L'appareil de chaque utilisateur dispose d'un pointeur (souris ou trackpad) ; le tactile n'est pas garanti.
- Le canal de diffusion temps réel existant supporte une charge compatible avec un groupe pouvant atteindre 100 participants actifs en simultané ; au-delà, aucun engagement de qualité de service n'est pris dans le cadre de cette itération.
- Les modérateurs acceptent que l'intervalle de sauvegarde soit d'environ 30 secondes et qu'une interruption brutale juste avant le prochain enregistrement puisse entraîner la perte du contenu tracé dans l'intervalle.

## Dependencies

- Disponibilité fonctionnelle du canal temps réel existant entre les participants d'une même session Afrolang.
- Disponibilité fonctionnelle des points d'accès serveur existants de récupération, d'enregistrement et d'effacement du tableau par session ; leur contrat reste inchangé.
- Processus de build et de déploiement existant permettant de régénérer et publier l'actif du tableau blanc servi statiquement par la plateforme.
