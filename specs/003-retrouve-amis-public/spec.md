# Feature Specification: Avis de Recherche Publics par Défaut

**Feature Branch**: `003-retrouve-amis-public`
**Created**: 2026-03-15
**Status**: Draft
**Input**: User description: "Formulaire de questions pour les recherches de personnes et affichage public de toutes les publications sur /retrouve-amis pour tous les visiteurs, y compris non connectés"

## Clarifications

### Session 2026-03-15

- Q: Quelles sont les questions/champs exacts du formulaire de recherche ? → A: Le formulaire comprend 10 champs : (1) Souhaitez-vous être anonyme, (2) Qui recherchez-vous (homme/femme), (3) Type de relation (amis enfance, amis école/université, collègue, connaissance, frère/sœur, parent), (4) Noms de la personne, (5) Surnoms, (6) Comment la personne vous connaît (de nom), (7) Lieu de rencontre, 3 sous-options cumulables : localité (précisez), école (précisez), ville (précisez) ou jamais rencontré, (8) Photo de la personne (upload optionnel), (9) Description physique (optionnel), (10) Souhaitez-vous partager vos coordonnées ou celles de vos proches en cas de match.
- Q: Les avis doivent-ils être modérés avant publication ? → A: Non : publication immédiate sans modération. La modération se fait uniquement a posteriori via le mécanisme de signalement (suspension auto après 3 signalements distincts).
- Q: Relation avec les specs 001 et 002 ? → A: La spec 003 remplace le formulaire et le modèle de visibilité (tout public par défaut), mais réutilise le moteur de recoupement automatique (001) et les fonctionnalités de partage social (002) comme fonctionnalités complémentaires.
- Q: Options pour le genre de la personne recherchée ? → A: Homme / Femme uniquement (deux options).
- Q: Format des coordonnées à partager en cas de match ? → A: Champs structurés séparés : email (optionnel), téléphone (optionnel), WhatsApp (optionnel). Au moins un champ requis si l'option de partage est activée.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Consulter les avis de recherche sans connexion (Priority: P1)

En tant que visiteur non connecté, je souhaite pouvoir consulter la page `/retrouve-amis` et voir tous les avis de recherche publiés, afin de reconnaître éventuellement une personne recherchée sans avoir besoin de créer un compte.

**Why this priority**: C'est le changement fondamental par rapport aux specs précédentes, tous les avis sont désormais publics et visibles par défaut. Sans cette visibilité universelle, le reste de la fonctionnalité n'a pas de sens. Cela maximise les chances de retrouvailles en exposant les recherches au plus grand nombre.

**Independent Test**: Ouvrir `/retrouve-amis` dans un navigateur en mode privé (non connecté) et vérifier que la liste des avis de recherche actifs est visible avec les informations pertinentes.

**Acceptance Scenarios**:

1. **Given** des avis de recherche actifs existent, **When** un visiteur non connecté accède à `/retrouve-amis`, **Then** il voit une liste paginée de tous les avis publics avec pour chaque avis : le type de relation, le nom/prénom recherché, le lieu de rencontre, la photo (si disponible), et la description.
2. **Given** la page `/retrouve-amis`, **When** un visiteur la consulte, **Then** les avis sont triés par date de publication (les plus récents en premier) et affichent le nombre de jours depuis la publication.
3. **Given** aucun avis de recherche actif, **When** un visiteur accède à `/retrouve-amis`, **Then** un message indique qu'aucun avis n'est disponible pour le moment et invite à créer le premier.
4. **Given** la page `/retrouve-amis` avec des avis, **When** un visiteur fait défiler la page, **Then** le système charge les avis suivants (pagination ou scroll infini).
5. **Given** un avis dont l'auteur a choisi l'anonymat, **When** un visiteur consulte cet avis, **Then** aucune information sur l'auteur n'est affichée (ni pseudonyme, ni initiale).

---

### User Story 2 - Remplir le formulaire de recherche (Priority: P1)

En tant qu'utilisateur connecté, je souhaite remplir un formulaire structuré avec des questions précises pour décrire la personne que je recherche, afin que mon avis soit clair et maximise les chances d'identification.

**Why this priority**: Co-priorité P1, le formulaire de questions est le point d'entrée de la fonctionnalité. Les questions posées déterminent la qualité des avis publiés et donc la probabilité de retrouvailles.

**Independent Test**: Se connecter, accéder au formulaire de création d'avis, remplir toutes les questions, soumettre et vérifier que l'avis apparaît immédiatement sur la page `/retrouve-amis` publique.

**Acceptance Scenarios**:

1. **Given** un utilisateur connecté, **When** il accède au formulaire de création d'avis, **Then** il voit un formulaire structuré en étapes avec les questions suivantes :
   - **Étape 1 : Préférences** : "Souhaitez-vous être anonyme ?" (oui/non) et "Souhaitez-vous partager vos coordonnées ou celles de vos proches en cas de match ?" (oui/non, si oui : champs email, téléphone, WhatsApp, au moins un requis)
   - **Étape 2 : Identité de la personne** : "Qui recherchez-vous ?" (homme/femme), "Noms de la personne" (obligatoire), "Surnoms" (optionnel), "Comment la personne vous connaît ?" (texte libre)
   - **Étape 3 : Relation** : "Quelle est votre relation ?" (choix : amis d'enfance, amis d'école/université, collègue, connaissance, frère/sœur, parent)
   - **Étape 4 : Lieu de rencontre** : "Où vous êtes-vous rencontrés ?", 3 sous-options cumulables : localité (avec champ de précision), école (avec champ de précision), ville (avec champ de précision), ou "Jamais rencontré"
   - **Étape 5 : Photo et description** : "Avez-vous une photo de la personne ?" (upload optionnel), "Pouvez-vous décrire la personne ?" (texte libre, optionnel)
   - **Étape 6 : Récapitulatif** : résumé de toutes les réponses avant soumission
2. **Given** un formulaire rempli avec les champs obligatoires (nom de la personne + au moins un lieu de rencontre OU type de relation), **When** l'utilisateur soumet l'avis, **Then** l'avis est immédiatement publié et visible sur `/retrouve-amis` pour tous les visiteurs.
3. **Given** un formulaire incomplet (nom manquant), **When** l'utilisateur tente de soumettre, **Then** des messages d'erreur clairs indiquent les champs à compléter.
4. **Given** un avis soumis avec succès, **When** le système confirme la création, **Then** l'utilisateur voit un message de succès avec un lien vers son avis sur la page publique.

---

### User Story 3 - Filtrer et rechercher parmi les avis publics (Priority: P2)

En tant que visiteur (connecté ou non), je souhaite pouvoir filtrer et rechercher parmi les avis de recherche sur `/retrouve-amis` par critères (type de relation, lieu, nom), afin de trouver plus facilement si quelqu'un recherche une personne que je connais.

**Why this priority**: Les filtres améliorent significativement l'expérience de découverte mais ne sont pas indispensables au fonctionnement de base.

**Independent Test**: Accéder à `/retrouve-amis`, utiliser les filtres par type de relation et vérifier que seuls les avis correspondants s'affichent.

**Acceptance Scenarios**:

1. **Given** la page `/retrouve-amis`, **When** un visiteur sélectionne un type de relation dans le filtre (ex: "amis d'école"), **Then** seuls les avis de ce type sont affichés.
2. **Given** la page `/retrouve-amis`, **When** un visiteur saisit un terme dans la barre de recherche textuelle, **Then** les résultats sont filtrés par pertinence (recherche sur nom, surnom, école, ville, localité, description).
3. **Given** des filtres actifs, **When** le visiteur réinitialise les filtres, **Then** tous les avis actifs sont à nouveau affichés.
4. **Given** des filtres actifs ne correspondant à aucun avis, **When** le visiteur consulte les résultats, **Then** un message indique qu'aucun résultat ne correspond et propose de modifier les critères.

---

### User Story 4 - Gérer ses avis publiés (Priority: P2)

En tant qu'auteur d'un avis, je souhaite pouvoir modifier ou clôturer mes avis de recherche, et voir les coordonnées de l'auteur protégées sur la page publique.

**Why this priority**: La gestion post-publication est nécessaire mais secondaire par rapport à la publication elle-même.

**Independent Test**: Créer un avis, le modifier, puis le clôturer et vérifier que les changements se reflètent sur la page publique.

**Acceptance Scenarios**:

1. **Given** un auteur connecté avec un avis publié, **When** il modifie les informations de son avis, **Then** les modifications sont immédiatement visibles sur la page publique.
2. **Given** un auteur connecté, **When** il clôture son avis (personne retrouvée), **Then** l'avis affiche un bandeau "Personne retrouvée !" sur la page publique et reste visible à titre informatif.
3. **Given** un avis publié sur `/retrouve-amis` dont l'auteur n'a PAS choisi l'anonymat, **When** n'importe quel visiteur le consulte, **Then** seul un pseudonyme (prénom + initiale) est affiché.
4. **Given** un avis publié dont l'auteur a choisi l'anonymat, **When** n'importe quel visiteur le consulte, **Then** aucune information sur l'auteur n'est visible.

---

### Edge Cases

- Que se passe-t-il si un utilisateur publie un avis avec des informations fausses ou malveillantes ? Un mécanisme de signalement est disponible pour les utilisateurs connectés, avec suspension automatique après 3 signalements distincts.
- Que se passe-t-il si la personne recherchée est sur la plateforme et ne souhaite pas être retrouvée ? La personne peut demander le retrait de l'avis via un bouton dédié, déclenchant une suspension immédiate et une modération administrative.
- Que se passe-t-il si un avis contient des informations sensibles (adresse, numéro de téléphone) dans le champ description ? Un avertissement est affiché lors de la saisie rappelant de ne pas inclure de données personnelles directes. Les champs structurés empêchent la saisie libre de telles informations.
- Que se passe-t-il si un visiteur reconnaît la personne recherchée ? Il est invité à se connecter (ou créer un compte) pour contacter l'auteur de manière sécurisée via la plateforme.
- Comment le SEO est-il géré pour les avis publics ? Chaque avis dispose de balises Open Graph et de métadonnées pour le référencement. La page `/retrouve-amis` est indexable.
- Que se passe-t-il si l'auteur a choisi l'anonymat et qu'un match est trouvé ? Les coordonnées ne sont partagées que si l'auteur a explicitement choisi de les partager (question 10 du formulaire). Si anonyme sans partage de coordonnées, le contact passe uniquement par la messagerie interne de la plateforme.
- Que se passe-t-il si l'utilisateur uploade une photo non conforme (format invalide, taille excessive, contenu inapproprié) ? Le système valide le format (JPEG, PNG, WebP), limite la taille et affiche un message d'erreur clair.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT afficher tous les avis de recherche actifs sur la page `/retrouve-amis` de manière publique, sans nécessiter d'authentification.
- **FR-002**: Le système DOIT proposer un formulaire de création d'avis structuré en 6 étapes avec les questions suivantes :
  - Étape 1 : Préférences : anonymat (oui/non), partage de coordonnées en cas de match (oui/non, si oui : email, téléphone, WhatsApp, au moins un requis)
  - Étape 2 : Identité : genre recherché (homme/femme), noms de la personne (obligatoire), surnoms (optionnel), "comment la personne vous connaît" (texte libre)
  - Étape 3 : Relation : type de relation (choix parmi : amis d'enfance, amis d'école/université, collègue, connaissance, frère/sœur, parent)
  - Étape 4 : Lieu de rencontre : 3 sous-options cumulables (localité + précision, école + précision, ville + précision) ou "jamais rencontré"
  - Étape 5 : Photo et description : upload photo optionnel, description physique (texte libre, optionnel)
  - Étape 6 : Récapitulatif : résumé complet avant soumission
- **FR-003**: Le système DOIT publier chaque nouvel avis automatiquement sur la page `/retrouve-amis` dès sa soumission (pas d'option privé/public, tout est public par défaut).
- **FR-004**: Le système DOIT protéger l'identité de l'auteur selon son choix : si anonymat activé, aucune information auteur affichée ; sinon, pseudonyme uniquement (prénom + initiale du nom, ex: "Amadou D.").
- **FR-005**: Le système DOIT permettre la pagination des avis sur `/retrouve-amis` avec un tri par date de publication décroissante.
- **FR-006**: Le système DOIT permettre le filtrage des avis par type de relation, lieu (localité, école, ville) sur la page `/retrouve-amis`.
- **FR-007**: Le système DOIT permettre une recherche textuelle (full-text) sur les avis publics (nom, surnom, école, ville, localité, description).
- **FR-008**: Le système DOIT permettre à l'auteur de modifier ou clôturer ses avis à tout moment.
- **FR-009**: Le système DOIT afficher un bandeau "Personne retrouvée !" sur les avis clôturés, tout en les gardant visibles sur la page publique.
- **FR-010**: Le système DOIT permettre aux utilisateurs connectés de signaler un avis (motif + description) et suspendre automatiquement un avis ayant reçu 3 signalements d'utilisateurs distincts.
- **FR-011**: Le système DOIT permettre à une personne concernée par un avis de demander son retrait via un bouton dédié, déclenchant une suspension immédiate et une notification aux administrateurs.
- **FR-012**: Le système DOIT inclure des balises Open Graph (og:title, og:description, og:image, og:url) sur la page de chaque avis pour permettre un aperçu riche lors du partage.
- **FR-013**: Le système DOIT limiter le nombre d'avis actifs à 10 par utilisateur.
- **FR-014**: Le système DOIT inviter les visiteurs non connectés qui reconnaissent une personne recherchée à se connecter/s'inscrire pour contacter l'auteur via la plateforme.
- **FR-015**: Le système DOIT permettre l'upload d'une photo optionnelle de la personne recherchée (formats acceptés : JPEG, PNG, WebP) avec une limite de taille raisonnable.
- **FR-016**: Le système DOIT permettre à l'auteur de choisir de partager ses coordonnées (ou celles de ses proches) uniquement en cas de match via 3 champs structurés : email (optionnel), téléphone (optionnel), WhatsApp (optionnel), au moins un requis si l'option est activée. Ces coordonnées DOIVENT être stockées de manière sécurisée et ne DOIVENT jamais être affichées publiquement.

### Key Entities

- **Avis de recherche public** : Publication décrivant une personne recherchée. Attributs : auteur (pseudonyme ou anonyme), anonymat (oui/non), genre recherché (homme/femme), noms, surnoms, comment la personne connaît l'auteur, type de relation (amis d'enfance, amis d'école/université, collègue, connaissance, frère/sœur, parent), lieux de rencontre (localité, école, ville, cumulables, ou "jamais rencontré"), photo (optionnel), description physique (optionnel), coordonnées de contact en cas de match : email, téléphone, WhatsApp (optionnels, non publics, au moins un requis si partage activé), statut (actif, clôturé, suspendu), date de publication.
- **Signalement** : Signalement d'un avis par un utilisateur connecté. Attributs : avis signalé, auteur du signalement, motif, statut de traitement.
- **Demande de retrait** : Requête d'une personne se reconnaissant dans un avis. Attributs : avis concerné, demandeur, motif, statut (en attente, approuvée, rejetée), date de suspension automatique.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: La page `/retrouve-amis` est accessible et affiche les avis sans connexion dans 100% des cas.
- **SC-002**: Les utilisateurs peuvent remplir le formulaire de recherche (6 étapes) et publier un avis en moins de 5 minutes.
- **SC-003**: 90% des visiteurs de la page `/retrouve-amis` comprennent le contenu des avis et les actions disponibles en moins de 30 secondes.
- **SC-004**: Le taux de signalements validés (abus confirmés) reste inférieur à 2% du total des avis publiés.
- **SC-005**: Le nombre de visites sur la page `/retrouve-amis` augmente de 50% par rapport à la version nécessitant une connexion, dans les 3 mois suivant le déploiement.
- **SC-006**: Les avis sont correctement indexés par les moteurs de recherche et génèrent un aperçu riche (Open Graph) lors du partage sur les réseaux sociaux.

## Assumptions

- Ce feature remplace le formulaire de création d'avis et le modèle de visibilité des specs 001 et 002 par un nouveau formulaire (10 questions) et un modèle "tout public par défaut".
- Le moteur de recoupement automatique (spec 001) et les fonctionnalités de partage social (spec 002) sont réutilisés comme fonctionnalités complémentaires.
- La publication est immédiate sans modération préalable, la modération se fait a posteriori via signalements.
- Le système d'authentification, de signalement et de blacklist existant est réutilisé.
- La page `/retrouve-amis` existante est adaptée pour afficher les avis publiquement au lieu de ne montrer que le dashboard.
- Les coordonnées partagées en cas de match ne sont jamais affichées publiquement : elles ne sont révélées qu'après consentement mutuel.
