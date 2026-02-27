# Feature Specification: Retrouve Amis

**Feature Branch**: `001-retrouve-amis`
**Created**: 2026-02-27
**Status**: Draft
**Input**: Fonctionnalité permettant de retrouver des amis perdus de vue grâce au recoupement d'informations entre avis de recherche et profils utilisateurs consentants.

## Clarifications

### Session 2026-02-27

- Q: Les avis de recherche sont-ils visibles publiquement ou traités uniquement en arrière-plan ? → A: Privés (arrière-plan uniquement) — seul le moteur de recoupement traite les avis, aucun autre utilisateur ne peut les consulter.
- Q: Comment protéger contre les tentatives répétées de contact non désiré après un refus ? → A: Blocage automatique — après un refus, aucune correspondance future n'est possible entre ces deux utilisateurs (blacklist mutuelle implicite).
- Q: Comment présenter les correspondances multiples pour un même avis ? → A: Toutes les correspondances qualifiées (score >= 60%) sont présentées en liste triée par score décroissant, sans limitation de nombre.
- Q: Combien de temps conserver les avis clôturés et correspondances associées ? → A: Conservation indéfinie — l'utilisateur peut toujours consulter son historique complet.
- Q: Quelles informations de profil sont utilisées pour le recoupement "trouvable" ? → A: Profil existant + champs supplémentaires optionnels — l'utilisateur peut ajouter un parcours (écoles fréquentées, villes de résidence passées, périodes) pour améliorer la qualité du recoupement.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Déposer un avis de recherche (Priority: P1)

Un utilisateur connecté souhaite retrouver un ami perdu de vue. Il accède à la section "Retrouve Amis" et remplit un formulaire d'avis de recherche en décrivant la personne qu'il cherche : nom/prénom (connus ou approximatifs), école ou université fréquentée, ville ou pays, période approximative (années), et tout détail complémentaire (surnom, activité, contexte de la rencontre). L'avis est publié et le système commence immédiatement à chercher des correspondances.

**Why this priority**: C'est la fonctionnalité fondamentale sans laquelle rien d'autre ne peut fonctionner. Un utilisateur doit pouvoir exprimer sa recherche pour que le système puisse travailler.

**Independent Test**: Peut être testé en créant un avis de recherche et en vérifiant qu'il est bien enregistré, visible dans le tableau de bord de l'utilisateur, et modifiable.

**Acceptance Scenarios**:

1. **Given** un utilisateur connecté, **When** il remplit le formulaire d'avis de recherche avec au minimum un nom et une ville/pays, **Then** l'avis est créé avec le statut "actif" et apparaît dans son tableau de bord.
2. **Given** un utilisateur connecté, **When** il remplit le formulaire sans les champs obligatoires (nom et au moins un critère géographique ou temporel), **Then** un message d'erreur lui indique les champs manquants.
3. **Given** un utilisateur ayant un avis actif, **When** il modifie les informations de son avis, **Then** les modifications sont enregistrées et le recoupement est relancé.
4. **Given** un utilisateur ayant un avis actif, **When** il décide de clôturer son avis (ami retrouvé ou abandon), **Then** l'avis passe au statut "clôturé" et n'apparaît plus dans les recoupements.

---

### User Story 2 - Recevoir une notification de correspondance (Priority: P1)

Lorsque le système détecte une correspondance suffisante entre un avis de recherche et un autre avis ou un profil utilisateur consentant, les deux parties reçoivent une notification. La notification indique qu'une correspondance potentielle a été trouvée, sans révéler immédiatement l'identité complète de l'autre partie. L'utilisateur peut alors consulter un résumé anonymisé de la correspondance.

**Why this priority**: Sans le mécanisme de correspondance et de notification, la fonctionnalité n'a aucune utilité. C'est le coeur de la proposition de valeur.

**Independent Test**: Peut être testé en créant deux avis de recherche complémentaires (A cherche B et B cherche A) et en vérifiant que les deux reçoivent une notification.

**Acceptance Scenarios**:

1. **Given** un avis de recherche de l'utilisateur A décrivant "Jean Dupont, Lycée Moderne d'Abidjan, 2005-2008", **When** l'utilisateur B dépose un avis cherchant "Marie Koné, Lycée Moderne d'Abidjan, 2006-2009" et que l'utilisateur A correspond au profil décrit par B, **Then** les deux utilisateurs reçoivent une notification de correspondance potentielle.
2. **Given** un utilisateur inscrit ayant activé l'option "je suis trouvable" dans son profil, **When** un avis de recherche correspond à ses informations de profil (nom, ville, école, période), **Then** les deux parties reçoivent une notification.
3. **Given** une correspondance détectée, **When** l'utilisateur consulte la notification, **Then** il voit un résumé anonymisé (initiales, ville, période, niveau de correspondance en pourcentage) sans les coordonnées de l'autre personne.

---

### User Story 3 - Accepter le contact et partager ses coordonnées (Priority: P2)

Après avoir reçu une notification de correspondance, un utilisateur consulte le résumé anonymisé et décide s'il souhaite entrer en contact. S'il accepte, l'autre partie est notifiée de cet intérêt. Lorsque les deux parties ont mutuellement accepté, leurs coordonnées de contact (choisies par chacun) sont partagées de manière sécurisée.

**Why this priority**: C'est l'aboutissement du parcours. Sans la mise en contact sécurisée, la fonctionnalité reste incomplète mais le recoupement seul a déjà de la valeur.

**Independent Test**: Peut être testé avec deux utilisateurs ayant une correspondance active, en vérifiant le flux d'acceptation mutuelle et le partage de coordonnées.

**Acceptance Scenarios**:

1. **Given** une correspondance notifiée à l'utilisateur A, **When** A accepte le contact, **Then** l'utilisateur B reçoit une notification lui indiquant que quelqu'un souhaite entrer en contact.
2. **Given** A a accepté et B accepte aussi, **When** le consentement mutuel est établi, **Then** chaque utilisateur voit les coordonnées que l'autre a choisi de partager (email, téléphone, ou message interne).
3. **Given** une correspondance notifiée, **When** l'utilisateur refuse le contact, **Then** l'autre partie n'est pas informée du refus et la correspondance est marquée comme "déclinée" côté refusant.
4. **Given** une correspondance en attente depuis plus de 30 jours sans réponse, **When** le délai expire, **Then** la correspondance est automatiquement archivée.

---

### User Story 4 - Gérer la visibilité de son profil (Priority: P2)

Un utilisateur inscrit peut choisir d'activer ou désactiver l'option "je suis trouvable" dans les paramètres de son profil. Lorsque cette option est active, ses informations de profil (nom, ville, école, période) sont incluses dans le moteur de recoupement. Il peut à tout moment désactiver cette option.

**Why this priority**: Le consentement des utilisateurs à être trouvables est essentiel pour le respect de la vie privée et pour alimenter la base de recoupement au-delà des seuls avis de recherche.

**Independent Test**: Peut être testé en activant/désactivant l'option et en vérifiant que le profil apparaît ou non dans les résultats de recoupement.

**Acceptance Scenarios**:

1. **Given** un utilisateur connecté, **When** il active l'option "je suis trouvable" dans son profil, **Then** ses informations sont disponibles pour le recoupement.
2. **Given** un utilisateur trouvable, **When** il désactive l'option, **Then** ses informations sont immédiatement retirées du recoupement et les correspondances en cours basées uniquement sur son profil sont annulées.
3. **Given** un nouvel utilisateur, **When** il s'inscrit, **Then** l'option "je suis trouvable" est désactivée par défaut (opt-in explicite).

---

### User Story 5 - Consulter et gérer son tableau de bord Retrouve Amis (Priority: P3)

L'utilisateur accède à un tableau de bord dédié listant ses avis de recherche actifs/clôturés, ses correspondances en cours, et l'historique de ses mises en contact réussies. Il peut filtrer et trier ses avis et correspondances.

**Why this priority**: Le tableau de bord améliore l'expérience utilisateur mais n'est pas indispensable au fonctionnement de base.

**Independent Test**: Peut être testé en vérifiant que tous les avis et correspondances d'un utilisateur sont correctement affichés et filtrables.

**Acceptance Scenarios**:

1. **Given** un utilisateur ayant 3 avis de recherche (2 actifs, 1 clôturé), **When** il accède à son tableau de bord, **Then** il voit la liste complète avec les statuts et le nombre de correspondances pour chaque avis.
2. **Given** un utilisateur avec des correspondances, **When** il filtre par statut "en attente", **Then** seules les correspondances en attente de réponse sont affichées.

---

### Edge Cases

- Que se passe-t-il si un utilisateur dépose un avis de recherche sur lui-même (se cherche lui-même) ? Le système doit détecter et empêcher l'auto-correspondance.
- Que se passe-t-il si un utilisateur abuse du système en déposant de nombreux avis frauduleux ? Un mécanisme de limitation (maximum 10 avis actifs simultanés) et de signalement est nécessaire.
- Que se passe-t-il si la personne recherchée n'est pas inscrite et qu'aucun autre avis ne correspond ? L'utilisateur est informé qu'aucune correspondance n'a été trouvée pour l'instant, et sera notifié dès qu'une correspondance apparaîtra.
- Que se passe-t-il si un utilisateur supprime son compte alors qu'il a des correspondances en cours ? Les correspondances sont annulées et l'autre partie est notifiée que la correspondance n'est plus disponible.
- Comment gérer les homonymes ? Le score de correspondance prend en compte plusieurs critères (nom + lieu + période) pour réduire les faux positifs. Un score minimum de 60% est requis pour notifier. En cas de correspondances multiples (homonymes), toutes sont présentées triées par score décroissant et l'utilisateur choisit la bonne personne.
- Que se passe-t-il si un utilisateur signale un avis de recherche comme abusif ou malveillant ? L'avis est suspendu en attente de modération par un administrateur.
- Que se passe-t-il si un utilisateur refuse un contact puis est retrouvé via un nouvel avis ? Impossible : après un refus, une blacklist automatique empêche toute future correspondance entre ces deux utilisateurs.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT permettre aux utilisateurs connectés de créer un avis de recherche avec les champs suivants : nom/prénom de la personne (obligatoire), surnom (optionnel), école/université (optionnel), ville (optionnel), pays (optionnel), période approximative début/fin en années (optionnel), description complémentaire (optionnel). Au moins un critère en plus du nom est obligatoire.
- **FR-002**: Le système DOIT permettre à l'utilisateur de modifier ou clôturer ses avis de recherche à tout moment.
- **FR-003**: Le système DOIT effectuer un recoupement automatique entre chaque avis de recherche et : (a) les autres avis de recherche actifs, (b) les profils utilisateurs ayant consenti à être trouvables. Les avis de recherche sont strictement privés : aucun utilisateur ne peut consulter les avis des autres. Seul le moteur de recoupement y accède.
- **FR-004**: Le système DOIT calculer un score de correspondance basé sur la similarité des critères (nom, lieu, période, école) et ne notifier que lorsque le score atteint un seuil minimum de 60%.
- **FR-005**: Le système DOIT envoyer une notification aux deux parties lorsqu'une correspondance suffisante est détectée. Lorsqu'un avis génère plusieurs correspondances qualifiées, elles sont toutes présentées à l'utilisateur en liste triée par score décroissant.
- **FR-006**: Le système DOIT afficher un résumé anonymisé de la correspondance (initiales, ville, période, score) sans révéler l'identité complète ni les coordonnées.
- **FR-007**: Le système DOIT implémenter un mécanisme de consentement mutuel : les coordonnées ne sont partagées que lorsque les deux parties acceptent le contact.
- **FR-008**: Le système DOIT permettre à chaque utilisateur de choisir quelles coordonnées partager (email, téléphone, ou messagerie interne).
- **FR-009**: Le système DOIT permettre à chaque utilisateur d'activer/désactiver l'option "je suis trouvable" dans son profil (désactivée par défaut). Le recoupement utilise les informations existantes du profil (nom, ville, pays) complétées par des champs optionnels dédiés : écoles/universités fréquentées, villes de résidence passées, et périodes associées.
- **FR-010**: Le système DOIT limiter le nombre d'avis de recherche actifs à 10 par utilisateur.
- **FR-011**: Le système DOIT permettre aux utilisateurs de signaler un avis de recherche comme abusif, déclenchant une modération administrative.
- **FR-012**: Le système DOIT archiver automatiquement les correspondances sans réponse après 30 jours.
- **FR-018**: Le système DOIT conserver indéfiniment les avis de recherche clôturés et les correspondances associées, permettant à l'utilisateur de consulter son historique complet à tout moment.
- **FR-013**: Le système DOIT relancer le recoupement lorsqu'un nouvel avis est créé ou qu'un profil utilisateur active l'option "trouvable".
- **FR-014**: Le système DOIT fournir un tableau de bord listant les avis de recherche, correspondances et mises en contact de l'utilisateur.
- **FR-015**: Le système DOIT empêcher l'auto-correspondance (un utilisateur ne peut pas correspondre avec son propre avis ou profil).
- **FR-016**: Le système DOIT permettre aux administrateurs de modérer les avis signalés (approuver, suspendre, supprimer).
- **FR-017**: Le système DOIT bloquer automatiquement toute future correspondance entre deux utilisateurs lorsque l'un refuse le contact de l'autre (blacklist mutuelle implicite), afin de prévenir le harcèlement et les tentatives répétées de contact non désiré.

### Key Entities

- **Avis de recherche** : Représente la demande d'un utilisateur pour retrouver une personne. Attributs principaux : auteur, nom/prénom recherché, surnom, école, ville, pays, période (début/fin), description, statut (actif, clôturé, suspendu), date de création.
- **Correspondance** : Représente un recoupement positif entre un avis et un autre avis ou un profil utilisateur. Attributs : avis source, cible (avis ou profil), score de correspondance, statut (en attente, acceptée mutuellement, déclinée, archivée), dates d'acceptation de chaque partie.
- **Consentement trouvable** : Attribut du profil utilisateur indiquant s'il accepte d'être inclus dans le moteur de recoupement. Lié aux informations de profil existantes (nom, ville, pays) et enrichi par des champs optionnels dédiés : parcours scolaire (écoles/universités avec périodes), villes de résidence passées (avec périodes).
- **Parcours trouvable** : Ensemble des informations optionnelles ajoutées par un utilisateur "trouvable" pour améliorer le recoupement. Attributs : école/université (nom, ville, période début/fin), ville de résidence passée (nom, pays, période début/fin). Un utilisateur peut ajouter plusieurs entrées.
- **Signalement** : Signalement d'un avis de recherche par un utilisateur. Attributs : avis signalé, auteur du signalement, motif, statut de modération.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Les utilisateurs peuvent créer un avis de recherche complet en moins de 3 minutes.
- **SC-002**: Le système détecte et notifie une correspondance dans un délai de 5 minutes après la création ou modification d'un avis ou l'activation du mode "trouvable".
- **SC-003**: 90% des utilisateurs comprennent le résumé anonymisé et prennent une décision (accepter/refuser) sans assistance.
- **SC-004**: Le taux de faux positifs (correspondances non pertinentes) reste inférieur à 20% des notifications envoyées.
- **SC-005**: 100% des partages de coordonnées nécessitent un consentement mutuel vérifié des deux parties.
- **SC-006**: Les avis signalés sont modérés par un administrateur dans un délai de 48 heures.

## Assumptions

- Les utilisateurs sont déjà inscrits et connectés sur la plateforme UAfricas pour utiliser cette fonctionnalité.
- Les informations de profil existantes (nom, ville, pays) peuvent être exploitées pour le recoupement lorsque l'utilisateur consent à être trouvable.
- La recherche par similarité de noms prend en compte les variantes orthographiques courantes et les noms africains (accents, transcriptions).
- Le système de notification utilise le mécanisme de notification existant de la plateforme (ou en crée un si inexistant).
- La messagerie interne comme option de contact utilise un canal simple intégré à la plateforme.
- La modération des avis signalés est effectuée manuellement par les administrateurs via le back-office existant.
