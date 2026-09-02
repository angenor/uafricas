# Feature Specification: Notifications et Suggestions Intelligentes

**Feature Branch**: `001-notifications-suggestions`
**Created**: 2026-03-16
**Status**: Draft
**Input**: User description: "Feature 7, Notifications et suggestions intelligentes. Notifications quand un nouveau match est détecté, quand quelqu'un confirme un lien, quand un collaborateur modifie l'arbre. Suggestions proactives : 'Vous n'avez pas renseigné les parents de [X], voulez-vous compléter ?'. Détection de doublons potentiels dans son propre arbre."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Recevoir des notifications d'événements importants (Priority: P1)

L'utilisateur reçoit des notifications dans l'interface quand des événements significatifs se produisent : nouveau match détecté (Feature 4), confirmation mutuelle d'une correspondance, modification de son arbre par un collaborateur (Feature 6), invitation reçue. Les notifications sont accessibles via une icône cloche dans la navbar avec un compteur de notifications non lues. Un clic ouvre un panneau déroulant listant les notifications récentes.

**Why this priority**: Les notifications sont le lien entre toutes les features, sans elles, l'utilisateur ne sait pas quand quelque chose de nouveau se passe et doit vérifier chaque page manuellement.

**Independent Test**: Déclencher un matching qui crée une suggestion → vérifier qu'une notification apparaît dans la cloche de l'utilisateur.

**Acceptance Scenarios**:

1. **Given** un nouveau match détecté pour l'utilisateur, **When** il charge une page, **Then** l'icône cloche affiche un compteur "+1" et la notification « Nouvelle correspondance détectée pour [Personne X] » apparaît dans le panneau.
2. **Given** un collaborateur modifie une personne dans l'arbre partagé, **When** le propriétaire charge une page, **Then** il voit « [Collaborateur] a modifié [Personne] dans votre arbre ».
3. **Given** une invitation reçue, **When** l'utilisateur charge une page, **Then** il voit « [Propriétaire] vous invite à collaborer sur son arbre ».
4. **Given** l'utilisateur clique sur une notification, **When** elle contient un lien d'action, **Then** il est redirigé vers la page pertinente (Découvertes, Arbre, etc.).
5. **Given** l'utilisateur marque une notification comme lue, **When** il consulte le panneau, **Then** la notification n'est plus comptée dans le badge et apparaît en grisé.
6. **Given** 50+ notifications, **When** l'utilisateur ouvre le panneau, **Then** seules les 20 plus récentes sont affichées avec un lien « Voir toutes les notifications ».

---

### User Story 2 - Suggestions proactives de complétion d'arbre (Priority: P2)

Le système analyse l'arbre de l'utilisateur et génère des suggestions pour l'enrichir : personnes sans parents renseignés, personnes sans dates de naissance, branches peu développées. Ces suggestions apparaissent dans un espace dédié (panneau ou section de la page d'accueil de l'arbre) et guident l'utilisateur vers les actions à effectuer.

**Why this priority**: Les suggestions encouragent l'engagement et la complétion des données, ce qui améliore la qualité du matching (Feature 4). Mais elles ne sont pas bloquantes.

**Independent Test**: Avec un arbre contenant des personnes sans parents, vérifier que des suggestions « Compléter les parents de [X] » apparaissent.

**Acceptance Scenarios**:

1. **Given** un arbre avec 5 personnes sans parents, **When** l'utilisateur consulte ses suggestions, **Then** il voit 5 suggestions du type « Vous n'avez pas renseigné les parents de [X]. Voulez-vous compléter ? ».
2. **Given** une suggestion de complétion, **When** l'utilisateur clique dessus, **Then** il est redirigé vers la vue arbre centrée sur la personne avec le formulaire d'ajout de parent ouvert.
3. **Given** une personne sans date de naissance, **When** les suggestions sont calculées, **Then** une suggestion « [X] n'a pas de date de naissance » apparaît avec un lien vers la modification.
4. **Given** l'utilisateur complète les parents de X, **When** les suggestions sont recalculées, **Then** la suggestion pour X disparaît.

---

### User Story 3 - Détection de doublons dans son propre arbre (Priority: P2)

Le système détecte les personnes potentiellement en doublon au sein du même arbre (noms très similaires, dates proches). Une alerte est affichée avec les paires suspectes, permettant à l'utilisateur de confirmer (ce sont deux personnes différentes) ou de fusionner (c'est la même personne).

**Why this priority**: Les doublons dégradent la qualité des données et peuvent fausser le matching. Mais cette détection est une optimisation, pas un besoin de base.

**Independent Test**: Ajouter deux personnes "Diallo Ibrahim" et "Dialo Ibrahim" → vérifier qu'un doublon potentiel est détecté.

**Acceptance Scenarios**:

1. **Given** un arbre contenant "Diallo Ibrahim (1850)" et "Dialo Ibrahim (1852)", **When** la détection de doublons s'exécute, **Then** une alerte « Doublon potentiel détecté : Diallo Ibrahim ↔ Dialo Ibrahim (score 85%) » apparaît.
2. **Given** un doublon détecté, **When** l'utilisateur clique « Ce sont deux personnes différentes », **Then** le doublon est marqué comme ignoré et ne réapparaît plus.
3. **Given** un doublon détecté, **When** l'utilisateur clique « Fusionner », **Then** un assistant de fusion s'ouvre permettant de choisir quelles informations conserver de chaque version (nom, dates, liens).
4. **Given** la fusion effectuée, **When** l'utilisateur consulte son arbre, **Then** une seule personne existe avec les informations fusionnées et tous les liens des deux originaux.

---

### User Story 4 - Page de notifications complète (Priority: P3)

L'utilisateur peut accéder à une page dédiée listant toutes ses notifications (lues et non lues) avec pagination, filtres par type (matching, collaboration, suggestions) et actions groupées (tout marquer comme lu).

**Why this priority**: Fonctionnalité de confort pour les utilisateurs actifs, mais le panneau déroulant (US1) couvre le besoin de base.

**Independent Test**: Avec 30+ notifications de types variés, vérifier la pagination et le filtrage par type.

**Acceptance Scenarios**:

1. **Given** l'utilisateur avec 30 notifications, **When** il accède à la page Notifications, **Then** il voit une liste paginée (10 par page) triée par date décroissante.
2. **Given** la page Notifications, **When** l'utilisateur filtre par « Matching », **Then** seules les notifications de type matching sont affichées.
3. **Given** des notifications non lues, **When** l'utilisateur clique « Tout marquer comme lu », **Then** toutes les notifications passent en statut lu et le badge de la cloche se réinitialise à 0.

---

### Edge Cases

- Que se passe-t-il si l'utilisateur n'a aucune notification ? → Le panneau affiche « Aucune notification pour l'instant » avec une icône décorative.
- Que se passe-t-il si une notification fait référence à une personne qui a été supprimée ? → La notification reste visible avec la mention « (personne supprimée) » mais le lien d'action est désactivé.
- Comment la détection de doublons gère-t-elle les homonymes volontaires (père et fils portant le même nom) ? → Le score prend en compte les dates. Si les dates sont significativement différentes (>20 ans), le score de doublon est bas et aucune alerte n'est générée.
- Que se passe-t-il si la fusion de doublons implique une personne avec des liens familiaux contradictoires ? → L'assistant de fusion signale les conflits et l'utilisateur choisit quels liens conserver.
- Combien de suggestions proactives maximum sont affichées ? → Maximum 10 suggestions à la fois, triées par impact (personnes les plus connectées en priorité).
- Les notifications sont-elles envoyées par email ? → Non dans cette version. Uniquement des notifications in-app (badge + panneau). L'email est prévu pour une version future.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT créer une notification in-app pour chaque événement significatif : nouveau match détecté, confirmation mutuelle, modification d'arbre par collaborateur, invitation reçue, demande de contact reçue.
- **FR-002**: L'icône de notification (cloche) DOIT être visible dans la navbar sur toutes les pages, avec un compteur de notifications non lues.
- **FR-003**: Un clic sur la cloche DOIT ouvrir un panneau déroulant affichant les 20 notifications les plus récentes, triées par date.
- **FR-004**: Chaque notification DOIT contenir : un message descriptif, la date, un type (matching/collaboration/suggestion), et un lien d'action vers la page pertinente.
- **FR-005**: L'utilisateur DOIT pouvoir marquer une notification comme lue (individuellement ou toutes à la fois).
- **FR-006**: Le système DOIT générer des suggestions proactives basées sur l'analyse de l'arbre : personnes sans parents (< 2), personnes sans date de naissance, branches courtes.
- **FR-007**: Les suggestions DOIVENT être cliquables et rediriger vers l'action appropriée (ajout de parent, modification de personne, etc.).
- **FR-008**: Le système DOIT détecter les doublons potentiels au sein d'un même arbre en comparant noms, dates et lieux (réutilise l'algorithme de similarité de Feature 4).
- **FR-009**: Pour chaque doublon détecté, l'utilisateur DOIT pouvoir : ignorer (marquer comme non-doublon) ou fusionner (combiner les deux personnes en une seule).
- **FR-010**: La fusion DOIT conserver tous les liens familiaux des deux personnes et permettre à l'utilisateur de choisir les informations à garder (nom, dates, lieux) en cas de conflit.
- **FR-011**: Une page dédiée DOIT lister toutes les notifications avec pagination et filtres par type.
- **FR-012**: Les suggestions proactives DOIVENT se mettre à jour automatiquement quand l'arbre est modifié (ajout/suppression de personnes).
- **FR-013**: Les doublons ignorés NE DOIVENT PAS réapparaître dans les détections futures.

### Key Entities

- **Notification** : Message adressé à un utilisateur suite à un événement. Attributs : destinataire, type (matching, collaboration, suggestion, systeme), message, lien_action (URL), lu (boolean), created_at. Cycle de vie : non_lu → lu.
- **Suggestion proactive** : Recommandation calculée pour enrichir l'arbre. Types : parents_manquants, date_manquante, branche_courte. Attributs : personne concernée, type de suggestion, message, action proposée. Recalculée dynamiquement.
- **Doublon potentiel** : Paire de personnes dans le même arbre avec un score de similarité élevé. Attributs : personne_a, personne_b, score, statut (detecte, ignore, fusionne). La fusion crée une personne unique avec les informations combinées.

## Assumptions

- Les notifications sont stockées en base de données (nouvelle table `arbre_genealogique.notifications`). Pas de système de notification temps réel (WebSocket), l'utilisateur voit les notifications au prochain chargement de page (polling ou vérification à l'ouverture).
- Les suggestions proactives sont calculées côté client à partir des données de l'arbre déjà en mémoire. Pas de nouvel endpoint : c'est une extension du composable existant `useLayoutArbre`.
- La détection de doublons réutilise la normalisation phonétique et pg_trgm de Feature 4, mais comparée au sein d'un même arbre (pas entre arbres).
- La fusion de doublons est une opération complexe qui nécessite un endpoint dédié côté backend : soft-delete d'une personne, transfert de ses liens à l'autre, mise à jour des données choisies.
- Les notifications sont créées côté serveur dans les handlers existants (matching, collaboration) en ajoutant des appels INSERT dans la table notifications au moment des événements.
- Le badge de la navbar est géré par un composable global qui vérifie le nombre de notifications non lues au chargement de chaque page.
- Maximum 10 suggestions proactives affichées (triées par nombre de liens de la personne, les personnes les plus connectées en priorité).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 90% des événements significatifs génèrent une notification visible par l'utilisateur dans les 30 secondes suivant leur occurrence (au prochain chargement de page).
- **SC-002**: Le compteur de notifications non lues est correct à 100%, aucune notification manquée ni comptée en double.
- **SC-003**: 60% des utilisateurs interagissent avec au moins une suggestion proactive (cliquent pour compléter) dans les 7 jours suivant l'activation.
- **SC-004**: La détection de doublons identifie au moins 80% des paires similaires (score > 70%) au sein d'un arbre.
- **SC-005**: Le taux de faux positifs pour les doublons (paires marquées "ignorer") reste inférieur à 30%.
- **SC-006**: L'utilisateur peut consulter et agir sur une notification en moins de 10 secondes (du clic sur la cloche à l'action finale).
- **SC-007**: La fusion de doublons préserve 100% des liens familiaux des deux personnes originales, aucun lien perdu.
