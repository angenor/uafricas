# Feature Specification: Matching et Découverte de Parents

**Feature Branch**: `001-matching-arbres`
**Created**: 2026-03-16
**Status**: Draft
**Input**: User description: "Feature 4, Matching et découverte de parents. L'algorithme compare les arbres de tous les utilisateurs pour détecter des ancêtres ou des personnes en commun, en se basant sur le rapprochement des noms, lieux et dates. Quand un match potentiel est trouvé, les deux utilisateurs reçoivent une suggestion. L'utilisateur peut confirmer ou rejeter le match. Si les deux confirment, leurs arbres se connectent et chacun découvre une nouvelle branche familiale. Page Découvertes listant les matchs en attente et confirmés."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Recevoir des suggestions de correspondances (Priority: P1)

L'utilisateur connecté accède à la page « Découvertes » et voit une liste de correspondances potentielles trouvées par le système. Chaque suggestion indique la personne de son arbre qui pourrait correspondre à une personne d'un autre arbre, avec un score de confiance et les critères de rapprochement (nom similaire, lieu proche, dates compatibles). L'utilisateur peut consulter les détails de chaque suggestion pour évaluer sa pertinence.

**Why this priority**: Sans suggestions, la feature n'existe pas. C'est le cœur du produit : la raison pour laquelle les utilisateurs enrichissent leur arbre.

**Independent Test**: Créer deux comptes utilisateurs avec des personnes aux noms/lieux/dates similaires → vérifier qu'une suggestion apparaît dans la page Découvertes de chaque utilisateur.

**Acceptance Scenarios**:

1. **Given** l'utilisateur A a "Diallo Ibrahim, né en 1850 à Ségou" dans son arbre et l'utilisateur B a "Diallo Ibrahim, né vers 1848 à Ségou" dans le sien, **When** le système exécute le matching, **Then** les deux utilisateurs reçoivent une suggestion de correspondance avec un score de confiance élevé.
2. **Given** une suggestion existe pour l'utilisateur, **When** il accède à la page Découvertes, **Then** il voit la suggestion avec : le nom de la personne de son arbre, les critères de rapprochement (nom, lieu, date), le score de confiance (ex : 85%), et un indicateur anonymisé de l'autre utilisateur (pas de nom ni de détails personnels avant confirmation mutuelle).
3. **Given** aucune correspondance n'a été trouvée, **When** l'utilisateur accède à la page Découvertes, **Then** il voit un message d'état vide l'encourageant à enrichir son arbre pour augmenter ses chances de découverte.
4. **Given** plusieurs suggestions existent, **When** l'utilisateur consulte la page Découvertes, **Then** les suggestions sont triées par score de confiance décroissant.

---

### User Story 2 - Confirmer ou rejeter une correspondance (Priority: P1)

L'utilisateur peut examiner chaque suggestion et décider de la confirmer (« Oui, c'est bien la même personne ») ou de la rejeter (« Non, ce n'est pas la bonne personne »). La confirmation est unilatérale : elle ne prend effet que lorsque les deux utilisateurs concernés confirment la même correspondance. Un rejet supprime définitivement la suggestion pour cet utilisateur.

**Why this priority**: Indissociable de US1, les suggestions n'ont de valeur que si l'utilisateur peut agir dessus.

**Independent Test**: Confirmer une suggestion côté utilisateur A → vérifier que le statut passe à « en attente de l'autre utilisateur ». Puis confirmer côté utilisateur B → vérifier que le statut passe à « confirmé ».

**Acceptance Scenarios**:

1. **Given** une suggestion en attente, **When** l'utilisateur A clique « Confirmer », **Then** le statut de la suggestion pour A passe à « confirmé de mon côté, en attente de l'autre » et A voit un message "En attente de confirmation de l'autre membre".
2. **Given** l'utilisateur A a confirmé et l'utilisateur B confirme la même suggestion, **When** B clique « Confirmer », **Then** la correspondance est validée et les deux utilisateurs sont notifiés.
3. **Given** une suggestion en attente, **When** l'utilisateur clique « Rejeter », **Then** la suggestion disparaît de sa liste et ne réapparaîtra jamais pour cette paire de personnes.
4. **Given** l'utilisateur A a confirmé mais B rejette, **When** B rejette, **Then** la correspondance est annulée pour les deux et A est informé que "L'autre membre n'a pas confirmé cette correspondance".

---

### User Story 3 - Découvrir de nouvelles branches après confirmation mutuelle (Priority: P1)

Quand les deux utilisateurs confirment une correspondance, les deux personnes matchées sont « fusionnées » logiquement : la personne devient un nœud partagé entre les deux arbres. Chaque utilisateur peut alors voir les branches de l'autre arbre rattachées à cette personne commune. Les informations personnelles de l'autre utilisateur (identité, email) restent masquées, seules les données de l'arbre (personnes, liens) sont partagées.

**Why this priority**: C'est la promesse de valeur de la plateforme, la découverte de branches familiales inconnues. Sans cette US, les confirmations n'aboutissent à rien.

**Independent Test**: Après confirmation mutuelle d'un match entre les arbres de A et B, vérifier que A voit les ancêtres/descendants de B rattachés à la personne commune dans sa propre vue arbre.

**Acceptance Scenarios**:

1. **Given** une correspondance confirmée mutuellement entre "Ibrahim Diallo" de l'arbre A et "Ibrahim Diallo" de l'arbre B, **When** l'utilisateur A consulte son arbre, **Then** il voit les branches de B rattachées à Ibrahim (les parents, enfants de l'arbre B), marquées visuellement comme « branches découvertes ».
2. **Given** des branches découvertes visibles dans l'arbre de A, **When** A consulte un nœud provenant de l'arbre de B, **Then** il voit les informations de la personne (nom, dates, lieu) mais pas l'identité du propriétaire de l'arbre B.
3. **Given** une correspondance confirmée, **When** l'utilisateur consulte la page Découvertes, **Then** la correspondance apparaît dans la section « Correspondances confirmées » avec la date de confirmation et le nombre de nouvelles personnes découvertes.

---

### User Story 4 - Consulter l'historique des découvertes (Priority: P2)

La page « Découvertes » organise les correspondances en trois sections : suggestions en attente (nouvelles), confirmations en cours (un seul côté a confirmé), et correspondances confirmées (les deux ont confirmé). L'utilisateur peut filtrer et paginer chaque section.

**Why this priority**: Interface de gestion essentielle mais les US1-3 sont prioritaires car elles constituent le flux principal.

**Independent Test**: Avec un mélange de correspondances dans les 3 états, vérifier que la page les affiche correctement dans les bonnes sections.

**Acceptance Scenarios**:

1. **Given** l'utilisateur a 3 suggestions en attente, 1 en cours de confirmation et 2 confirmées, **When** il accède à la page Découvertes, **Then** il voit les 3 sections avec les compteurs corrects.
2. **Given** la section "Suggestions en attente" contient plus de 10 éléments, **When** l'utilisateur fait défiler, **Then** une pagination ou un chargement progressif permet d'accéder aux éléments suivants.
3. **Given** la page Découvertes, **When** l'utilisateur consulte une correspondance confirmée, **Then** il voit un résumé : personne commune, date de confirmation, nombre de nouvelles personnes découvertes, et un lien « Voir dans l'arbre » pour naviguer vers la visualisation centrée sur cette personne.

---

### User Story 5 - Exécution automatique du matching (Priority: P2)

Le système exécute automatiquement l'algorithme de matching à intervalles réguliers (ou après chaque ajout significatif à un arbre) pour détecter de nouvelles correspondances potentielles. L'utilisateur est notifié quand de nouvelles suggestions sont disponibles.

**Why this priority**: L'automatisation est essentielle à long terme mais un déclenchement manuel initial suffit pour le MVP.

**Independent Test**: Ajouter une nouvelle personne à son arbre → vérifier qu'une tâche de matching est déclenchée et que de nouvelles suggestions apparaissent si des correspondances existent.

**Acceptance Scenarios**:

1. **Given** l'utilisateur A ajoute une personne "Kouyaté Fatoumata, née en 1885 à Kankan", **When** le matching s'exécute, **Then** si un autre arbre contient une personne similaire, une nouvelle suggestion est créée.
2. **Given** de nouvelles suggestions sont créées pour l'utilisateur, **When** il revient sur la page Découvertes, **Then** il voit un indicateur « X nouvelle(s) suggestion(s) » sur les suggestions récentes (< 7 jours).
3. **Given** le matching s'exécute, **When** deux personnes ont déjà été matchées (confirmées ou rejetées), **Then** le système ne crée pas de nouvelle suggestion en doublon.

---

### Edge Cases

- Que se passe-t-il si un utilisateur supprime une personne qui fait partie d'une correspondance en attente ? → La suggestion est automatiquement annulée et disparaît de la liste de l'autre utilisateur.
- Que se passe-t-il si un utilisateur supprime une personne qui fait partie d'une correspondance confirmée ? → Le lien inter-arbres est rompu et l'autre utilisateur est notifié que "Une branche découverte n'est plus disponible".
- Comment gérer les homonymes parfaits (même nom, même lieu, même date) qui ne sont PAS la même personne ? → Le système propose la suggestion avec un score élevé, mais l'utilisateur peut la rejeter. Le rejet est définitif pour cette paire.
- Comment le matching gère-t-il les noms avec des variantes orthographiques (Diallo / Dialo / Dyallo) ? → L'algorithme de rapprochement prend en compte les variantes phonétiques courantes pour les noms africains.
- Que se passe-t-il si le même utilisateur crée deux comptes avec des arbres différents ? → Le matching traite chaque compte indépendamment. Il n'y a pas de détection de comptes multiples dans cette feature.
- Comment le matching gère-t-il les dates approximatives (uniquement l'année vs date complète) ? → L'algorithme utilise une tolérance temporelle (ex : ±5 ans si seule l'année est renseignée) pour le score de confiance.
- Que se passe-t-il si un arbre contient très peu de personnes (1-2) ? → Le matching est moins efficace mais fonctionne quand même. Le score de confiance sera naturellement plus bas car moins de critères croisés sont disponibles.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT comparer les personnes de tous les arbres entre eux pour détecter des correspondances potentielles basées sur la similarité des noms, lieux de naissance/décès et dates.
- **FR-002**: Chaque suggestion de correspondance DOIT inclure : les deux personnes comparées (une de chaque arbre), un score de confiance (0-100%), et les critères de rapprochement ayant contribué au score.
- **FR-003**: Le score de confiance DOIT pondérer au minimum trois critères : similarité du nom (incluant variantes phonétiques), proximité géographique des lieux, et compatibilité des dates (avec tolérance pour les dates partielles).
- **FR-004**: Les informations personnelles de l'autre utilisateur (nom, email, identité) DOIVENT rester masquées tant que la correspondance n'est pas mutuellement confirmée. Seules les données de la personne dans l'arbre (nom, dates, lieu) sont visibles dans la suggestion.
- **FR-005**: L'utilisateur DOIT pouvoir confirmer ou rejeter chaque suggestion individuellement.
- **FR-006**: Une correspondance ne DOIT être considérée comme validée que lorsque les DEUX utilisateurs ont confirmé la même suggestion.
- **FR-007**: Un rejet DOIT être définitif, la même paire de personnes ne DOIT pas générer de nouvelle suggestion.
- **FR-008**: Après confirmation mutuelle, chaque utilisateur DOIT pouvoir voir l'intégralité de l'arbre de l'autre utilisateur (toutes les personnes et liens), marquées visuellement comme « branches découvertes ». L'arbre complet de l'autre est exposé, pas seulement les branches proches de la personne commune.
- **FR-009**: La page « Découvertes » DOIT afficher trois sections : suggestions en attente, confirmations en cours (un seul côté confirmé), correspondances confirmées.
- **FR-010**: Le matching DOIT s'exécuter en deux temps : une vérification rapide synchrone (correspondances par nom exact) à chaque ajout de personne, puis une tâche de fond pour le matching profond (variantes phonétiques, dates approximatives, lieux). Les résultats du matching profond sont disponibles ultérieurement.
- **FR-011**: Le système DOIT empêcher les suggestions en doublon (même paire de personnes déjà matchée, confirmée ou rejetée).
- **FR-012**: Si une personne impliquée dans une correspondance est supprimée, le système DOIT annuler automatiquement la suggestion/correspondance et notifier l'autre utilisateur si nécessaire.
- **FR-013**: Les branches découvertes DOIVENT être en lecture seule, l'utilisateur ne peut pas modifier les personnes provenant de l'arbre d'un autre utilisateur.
- **FR-014**: Le système DOIT notifier l'utilisateur quand de nouvelles suggestions sont disponibles (indicateur sur la page Découvertes, et/ou notification dans l'interface).
- **FR-015**: Après confirmation mutuelle, chaque utilisateur DOIT pouvoir envoyer une demande de contact à l'autre. L'autre utilisateur peut accepter (ses coordonnées deviennent visibles) ou refuser (la relation reste limitée à la visualisation des branches).
- **FR-016**: Tant que la demande de contact n'est pas acceptée, aucune information personnelle (nom, email, photo de profil) du propriétaire de l'arbre ne DOIT être exposée, seul un identifiant anonyme (ex : « Membre #1234 ») est affiché.

### Key Entities

- **Suggestion de correspondance** : Proposition générée par le système indiquant qu'une personne de l'arbre A pourrait être la même personne qu'une personne de l'arbre B. Attributs : personne A (rattachement), personne B (rattachement), score de confiance, critères de rapprochement, statut (en_attente, confirmee_a, confirmee_b, confirmee, rejetee). Cycle de vie : en_attente → confirmée d'un côté → confirmée mutuellement (ou rejetée).
- **Correspondance confirmée** : Lien validé mutuellement entre deux personnes de deux arbres différents. Établit un pont permettant la visibilité des branches de l'autre arbre. Attributs : suggestion source, date de confirmation, personnes reliées.
- **Branche découverte** : Ensemble de personnes et liens provenant de l'arbre d'un autre utilisateur, devenues visibles après confirmation d'une correspondance. Lecture seule pour le destinataire. Marquées visuellement comme provenant d'un autre arbre.
- **Demande de contact** : Requête envoyée par un utilisateur à l'autre après confirmation mutuelle d'une correspondance. Statut : en_attente, acceptee, refusee. Si acceptée, les informations de profil (nom, email) des deux utilisateurs deviennent mutuellement visibles.
- **Critères de rapprochement** : Détail des raisons pour lesquelles deux personnes sont considérées comme potentiellement identiques. Sous-scores par critère : nom (ex : 90%), lieu (ex : 100%), date (ex : 70%). Permet à l'utilisateur de juger la pertinence de la suggestion.

## Assumptions

- Le matching ne s'exécute qu'entre arbres d'utilisateurs différents, jamais au sein du même arbre.
- Le score de confiance est calculé côté serveur. L'algorithme exact (pondération, seuils, phonétique) sera déterminé lors de la phase de planification, mais un seuil minimum (ex : 60%) est appliqué pour éviter les suggestions non pertinentes.
- La « fusion logique » des personnes matchées ne modifie pas les données de l'un ou l'autre arbre. C'est une relation de type « pont » entre deux rattachements de deux arbres distincts, chaque arbre conserve ses propres données.
- Les branches découvertes sont mises à jour en temps réel si l'autre utilisateur modifie son arbre (ajout/modification de personnes rattachées à la personne commune).
- La notification de nouvelles suggestions se fait via un indicateur visuel dans l'interface (badge/compteur sur le menu), pas par email dans un premier temps.
- Le matching considère toutes les personnes de l'arbre, pas uniquement celles sans parents. Plus un arbre est riche, plus les correspondances sont fiables.
- La page Découvertes est accessible depuis le menu principal de la plateforme, avec un badge indiquant le nombre de nouvelles suggestions.
- Après confirmation mutuelle, l'intégralité de l'arbre de l'autre utilisateur est visible (toutes personnes et liens), pas seulement les branches proches de la personne commune. Cela maximise la valeur de découverte.

## Clarifications

### Session 2026-03-16

- Q: Combien de générations de l'arbre de l'autre utilisateur sont visibles après confirmation mutuelle ? → A: Arbre complet : toutes les personnes et liens de l'autre utilisateur sont visibles.
- Q: Le matching s'exécute-t-il de façon synchrone, asynchrone ou hybride ? → A: Hybride : vérification rapide synchrone (nom exact) à chaque ajout, puis tâche de fond pour le matching profond (phonétique, dates, lieux).
- Q: Les utilisateurs peuvent-ils communiquer après confirmation mutuelle ? → A: Demande de contact optionnelle, l'utilisateur peut envoyer une demande, l'autre accepte ou refuse. Identité masquée tant que le contact n'est pas accepté.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Le système détecte au moins 80% des correspondances évidentes (même nom exact, même lieu, dates à ±5 ans) entre les arbres de la plateforme.
- **SC-002**: Le taux de faux positifs (suggestions rejetées) reste inférieur à 40% des suggestions totales générées.
- **SC-003**: L'utilisateur peut consulter et agir sur une suggestion (confirmer/rejeter) en moins de 30 secondes.
- **SC-004**: Après confirmation mutuelle, les branches découvertes sont visibles dans l'arbre de l'utilisateur en moins de 5 secondes.
- **SC-005**: 70% des utilisateurs ayant reçu des suggestions consultent la page Découvertes dans les 48 heures suivant la notification.
- **SC-006**: Le matching d'un nouvel ajout de personne contre tous les arbres existants produit des résultats en moins de 30 secondes pour une base de 10 000 personnes.
- **SC-007**: Aucune information personnelle (identité, email) de l'autre utilisateur n'est exposée avant la confirmation mutuelle.
