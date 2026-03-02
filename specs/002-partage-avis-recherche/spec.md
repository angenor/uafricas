# Feature Specification: Partage Public des Avis de Recherche

**Feature Branch**: `002-partage-avis-recherche`
**Created**: 2026-03-02
**Status**: Draft
**Input**: User description: "Rendre les avis de recherche publics et partageables sur les réseaux sociaux pour augmenter les probabilités de retrouvailles, avec protection anti-harcèlement"

## Clarifications

### Session 2026-03-02

- Q: Les avis publics sont-ils uniquement accessibles via lien direct ou existe-t-il une page de listing publique ? → A: Listing public + lien direct — une page publique `/retrouve-amis/rechercher` permet de parcourir et filtrer les avis publics (par pays, ville, école, période).
- Q: Que se passe-t-il après qu'une personne demande le retrait d'un avis la concernant ? → A: Suspension immédiate + arbitrage admin — l'avis est automatiquement suspendu dès la demande, puis un administrateur tranche sous 72h (maintien ou retrait définitif).
- Q: Comment comptabiliser les partages pour mesurer SC-003 ? → A: Compteur intégré par avis — chaque clic sur un bouton de partage incrémente un compteur persistant, visible sur la page publique (ex: "Partagé 12 fois").
- Q: Les signalements d'avis publics doivent-ils être ouverts aux visiteurs non connectés ? → A: Connexion requise uniquement — seuls les utilisateurs connectés peuvent signaler. Élimine le risque de spam de signalements.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Rendre un avis de recherche public (Priority: P1)

En tant qu'auteur d'un avis de recherche, je souhaite pouvoir rendre mon avis visible publiquement (sans nécessiter de connexion) afin que toute personne sur Internet puisse le consulter et potentiellement reconnaître la personne recherchée.

**Why this priority**: C'est la fonctionnalité fondamentale — sans page publique, il n'y a rien à partager. C'est le prérequis de toutes les autres stories.

**Independent Test**: Créer un avis, activer la visibilité publique, puis accéder à la page via un navigateur en navigation privée (non connecté) et vérifier que les informations autorisées sont visibles.

**Acceptance Scenarios**:

1. **Given** un avis de recherche actif, **When** l'auteur active l'option "Rendre public", **Then** une page publique est créée avec une URL unique lisible (slug) et l'avis est accessible sans authentification.
2. **Given** un avis rendu public, **When** un visiteur non connecté accède à l'URL publique, **Then** il voit les informations de l'avis (nom recherché, école, ville, pays, période, description) mais PAS les coordonnées de l'auteur ni son nom complet.
3. **Given** un avis public, **When** l'auteur désactive la visibilité publique, **Then** la page publique renvoie un message "Cet avis n'est plus disponible" et n'est plus indexée.
4. **Given** un avis suspendu par un administrateur, **When** un visiteur accède à l'URL publique, **Then** la page affiche "Cet avis a été temporairement retiré" sans détails.

---

### User Story 2 - Partager un avis sur les réseaux sociaux (Priority: P2)

En tant qu'auteur ou visiteur de la page publique d'un avis, je souhaite partager facilement l'avis sur les réseaux sociaux (WhatsApp, Facebook, X/Twitter, LinkedIn, copier le lien) pour maximiser la visibilité auprès des communautés africaines.

**Why this priority**: Le partage social est le mécanisme principal pour atteindre des personnes hors de la plateforme. WhatsApp est prioritaire car c'est le réseau dominant en Afrique.

**Independent Test**: Accéder à la page publique d'un avis, cliquer sur le bouton de partage WhatsApp et vérifier que le message pré-rempli contient le texte attendu et le lien.

**Acceptance Scenarios**:

1. **Given** une page publique d'avis, **When** un visiteur clique sur le bouton "Partager sur WhatsApp", **Then** WhatsApp s'ouvre avec un message pré-formaté contenant un résumé de l'avis et le lien de la page publique.
2. **Given** une page publique d'avis, **When** un visiteur clique sur "Partager sur Facebook", **Then** la boîte de dialogue de partage Facebook s'ouvre avec le lien, le titre et l'aperçu (Open Graph) de l'avis.
3. **Given** une page publique d'avis, **When** un visiteur clique sur "Copier le lien", **Then** le lien est copié dans le presse-papiers et un message de confirmation s'affiche.
4. **Given** une page publique d'avis, **When** un réseau social ou une application de messagerie charge le lien, **Then** un aperçu riche (titre, description, image) s'affiche grâce aux balises Open Graph et Twitter Card.

---

### User Story 3 - Répondre à un avis public en tant que témoin (Priority: P3)

En tant que visiteur ayant reconnu la personne recherchée (ou étant cette personne), je souhaite pouvoir contacter l'auteur de l'avis de manière sécurisée via la plateforme, sans avoir accès à ses informations personnelles.

**Why this priority**: C'est l'objectif final du partage — permettre aux personnes touchées de répondre. Le contact doit obligatoirement passer par la plateforme pour protéger les deux parties.

**Independent Test**: Accéder à une page publique, remplir le formulaire de réponse, puis vérifier que l'auteur de l'avis reçoit une notification avec le message.

**Acceptance Scenarios**:

1. **Given** une page publique d'avis, **When** un visiteur non inscrit clique sur "Je connais cette personne" ou "C'est moi", **Then** il est invité à créer un compte ou se connecter avant de pouvoir envoyer un message.
2. **Given** un utilisateur connecté sur une page publique, **When** il remplit le formulaire de réponse (type: "Je suis cette personne" / "Je la connais" / "J'ai des informations") avec un message, **Then** l'auteur de l'avis reçoit une notification et le message est visible dans son espace de correspondances.
3. **Given** un utilisateur qui a déjà répondu à un avis, **When** il tente de répondre à nouveau au même avis, **Then** le système lui indique qu'il a déjà répondu et lui propose de consulter sa correspondance existante.
4. **Given** un auteur qui reçoit une réponse, **When** il consulte la réponse dans son espace, **Then** il voit le type de réponse, le message, et peut choisir d'accepter ou de décliner le contact (comme pour les correspondances automatiques).

---

### User Story 4 - Protections anti-harcèlement (Priority: P1)

En tant que personne recherchée ou auteur d'un avis, je souhaite être protégé contre le harcèlement, l'usurpation d'identité et les abus rendus possibles par la visibilité publique des avis.

**Why this priority**: Co-priorité P1 avec la page publique — la protection est indissociable de la mise en visibilité. Sans ces protections, la fonctionnalité présenterait des risques inacceptables.

**Independent Test**: Tenter d'accéder aux coordonnées de l'auteur depuis la page publique, vérifier qu'elles ne sont pas exposées. Tester le système de signalement et le blocage.

**Acceptance Scenarios**:

1. **Given** une page publique d'avis, **When** un visiteur consulte la page, **Then** aucune information permettant d'identifier ou contacter directement l'auteur n'est visible (pas de nom complet, pas d'email, pas de téléphone) — seul un pseudonyme ou prénom + initiale est affiché.
2. **Given** un auteur qui reçoit des messages inappropriés via le formulaire de réponse, **When** il signale un message, **Then** le message est marqué pour modération et l'auteur peut bloquer l'expéditeur (ajout à la blacklist existante).
3. **Given** un avis public qui reçoit plus de 3 signalements de visiteurs différents, **When** le seuil est atteint, **Then** l'avis est automatiquement suspendu et un administrateur est notifié pour examen.
4. **Given** un utilisateur connecté, **When** il accède à un avis public le concernant et qu'il ne souhaite pas être retrouvé, **Then** il peut demander le retrait de l'avis via un bouton "Cet avis me concerne — demander le retrait", l'avis est immédiatement suspendu, et l'auteur + les administrateurs sont notifiés.
5a. **Given** une demande de retrait en cours, **When** un administrateur examine la demande dans les 72h, **Then** il peut décider du maintien (réactivation de l'avis) ou du retrait définitif (suppression de la page publique).
5b. **Given** une demande de retrait sans décision admin après 72h, **When** le délai expire, **Then** l'avis reste suspendu jusqu'à décision manuelle (pas de réactivation automatique).
6. **Given** un avis public, **When** un visiteur non connecté clique sur "Signaler", **Then** il est invité à se connecter ou créer un compte avant de pouvoir soumettre un signalement.

---

### User Story 5 - Parcourir les avis publics (Priority: P2)

En tant que visiteur (connecté ou non), je souhaite pouvoir parcourir et rechercher parmi les avis de recherche publics sur une page dédiée, afin de découvrir si quelqu'un recherche une personne que je connais.

**Why this priority**: Co-priorité P2 avec le partage social — le listing public augmente la découvrabilité organique et le référencement (SEO), créant un effet de réseau où un visiteur venu pour un avis peut en découvrir d'autres.

**Independent Test**: Accéder à `/retrouve-amis/rechercher` sans être connecté, filtrer par pays et vérifier que seuls les avis publics et actifs apparaissent.

**Acceptance Scenarios**:

1. **Given** des avis de recherche publics et actifs, **When** un visiteur accède à `/retrouve-amis/rechercher`, **Then** il voit une liste paginée des avis publics avec un résumé (nom recherché, ville, pays, période) et peut accéder à la page détaillée de chaque avis.
2. **Given** la page de listing public, **When** un visiteur filtre par pays, ville ou école, **Then** seuls les avis correspondants aux critères sont affichés.
3. **Given** la page de listing public, **When** un visiteur effectue une recherche textuelle, **Then** les résultats sont triés par pertinence en utilisant la recherche full-text existante.
4. **Given** aucun avis public correspondant aux critères de recherche, **When** un visiteur effectue une recherche, **Then** un message "Aucun avis ne correspond à votre recherche" est affiché avec une suggestion de modifier les critères.

---

### Edge Cases

- Que se passe-t-il quand un avis public est clôturé par l'auteur ? → La page publique affiche "Cette personne a été retrouvée !" sans possibilité de répondre, et les boutons de partage sont retirés.
- Que se passe-t-il quand l'auteur supprime son compte ? → Les avis publics sont automatiquement dépubliés et affichent "Cet avis n'est plus disponible".
- Comment gérer les avis en doublon (même personne recherchée par plusieurs auteurs) ? → Chaque avis reste indépendant avec sa propre page publique. Le système ne fusionne pas les avis.
- Que se passe-t-il si quelqu'un partage le lien après la dépublication ? → La page affiche un message clair et propose de consulter d'autres avis actifs.
- Comment limiter le spam de réponses ? → Un utilisateur ne peut répondre qu'une seule fois par avis. Un rate limit est appliqué (maximum 10 réponses par jour par utilisateur sur l'ensemble des avis).
- Que se passe-t-il si la personne recherchée est mineure ? → L'auteur doit confirmer que la recherche concerne une période passée (la personne est aujourd'hui adulte). Un avertissement est affiché lors de la création.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Le système DOIT permettre à l'auteur d'un avis de recherche actif d'activer/désactiver la visibilité publique via un interrupteur dans l'interface de gestion de ses avis.
- **FR-002**: Le système DOIT générer une URL publique unique et lisible pour chaque avis rendu public (format: `/retrouve-amis/public/{slug}` où le slug est basé sur un identifiant court et non prédictible).
- **FR-003**: La page publique DOIT afficher : nom de la personne recherchée, prénom (si renseigné), école, ville, pays, période, description. Elle NE DOIT PAS afficher : nom complet de l'auteur, email, téléphone, ou tout identifiant permettant un contact direct.
- **FR-004**: La page publique DOIT inclure les balises Open Graph (og:title, og:description, og:image, og:url) et Twitter Card pour un aperçu riche lors du partage sur les réseaux sociaux.
- **FR-005**: Le système DOIT proposer des boutons de partage pour : WhatsApp (prioritaire), Facebook, X/Twitter, LinkedIn, et copie du lien. Chaque clic DOIT incrémenter un compteur de partages persistant par avis, affiché publiquement sur la page (ex: "Partagé 12 fois").
- **FR-006**: Le système DOIT permettre aux utilisateurs connectés de répondre à un avis public via un formulaire structuré (type de réponse + message) et cette réponse DOIT créer une correspondance dans le système existant.
- **FR-007**: Le système DOIT limiter chaque utilisateur à une seule réponse par avis public et à 10 réponses par jour maximum tous avis confondus.
- **FR-008**: Le système DOIT afficher l'auteur de l'avis sous un format anonymisé (prénom + initiale du nom, ex: "Amadou D.") sur la page publique.
- **FR-009**: Le système DOIT permettre aux utilisateurs connectés de signaler un avis public (motif + description) et suspendre automatiquement un avis ayant reçu 3 signalements ou plus d'utilisateurs distincts. Les visiteurs non connectés DOIVENT être redirigés vers la connexion avant de pouvoir signaler.
- **FR-010**: Le système DOIT offrir un mécanisme "Cet avis me concerne — demander le retrait" qui suspend immédiatement l'avis, notifie l'auteur et les administrateurs. Un administrateur DOIT statuer sous 72h (maintien ou retrait définitif). En l'absence de décision, l'avis reste suspendu.
- **FR-011**: Le système DOIT afficher un message adapté sur la page publique selon l'état de l'avis : actif (contenu complet), clôturé ("personne retrouvée"), suspendu ("avis temporairement retiré"), dépublié ("avis non disponible").
- **FR-012**: Les visiteurs non connectés souhaitant répondre à un avis DOIVENT être redirigés vers l'inscription/connexion puis renvoyés vers l'avis après authentification.
- **FR-013**: Le système DOIT empêcher l'indexation des pages publiques d'avis non actifs ou dépubliés via les directives `noindex, nofollow`.
- **FR-014**: Le système DOIT proposer une page publique de listing (`/retrouve-amis/rechercher`) affichant les avis publics actifs avec pagination, filtres (pays, ville, école) et recherche textuelle full-text.

### Key Entities

- **Avis de Recherche (étendu)** : L'entité existante enrichie avec les attributs de visibilité publique (statut public, slug, date de publication publique, compteur de partages).
- **Réponse Publique** : Représente la réponse d'un visiteur à un avis public. Contient le type de réponse ("je suis cette personne", "je la connais", "j'ai des informations"), le message, et le lien vers la correspondance créée.
- **Demande de Retrait** : Requête émise par une personne se reconnaissant dans un avis et souhaitant son retrait. Contient le motif, l'état de traitement (en_attente, approuvée, rejetée), la date de suspension automatique, et la décision admin avec délai de 72h.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Le taux de correspondances réussies (état "mutuelle") augmente d'au moins 30% dans les 6 mois suivant le déploiement grâce aux réponses issues du partage social.
- **SC-002**: Au moins 40% des avis de recherche actifs sont rendus publics par leurs auteurs dans les 3 mois suivant le lancement.
- **SC-003**: Chaque avis public est partagé en moyenne au moins 2 fois sur les réseaux sociaux (mesuré via le compteur de partages intégré par avis).
- **SC-004**: Le taux de signalements validés (abus confirmés par la modération) reste inférieur à 2% du total des avis publics, démontrant l'efficacité des protections.
- **SC-005**: 95% des visiteurs de la page publique peuvent comprendre l'objet de l'avis et les actions disponibles en moins de 30 secondes.
- **SC-006**: Le délai moyen entre la publication d'un avis et la première réponse via le partage social est inférieur à 14 jours.

## Assumptions

- Les utilisateurs de la plateforme sont principalement en Afrique et utilisent WhatsApp comme moyen de communication principal — d'où la priorité donnée au partage WhatsApp.
- La plateforme dispose déjà d'un système d'authentification, de gestion des signalements et de blacklist réutilisables pour cette fonctionnalité.
- Les pages publiques seront accessibles via le domaine principal de la plateforme (www.africans-world.org).
- Le slug d'URL utilise un identifiant court non séquentiel pour des raisons de lisibilité et de prévention de l'énumération.
- L'image par défaut utilisée pour l'aperçu Open Graph (en l'absence de photo) sera une image générique de la plateforme avec le texte de l'avis superposé.
- Les réponses des visiteurs sont intégrées au système de correspondances existant, ce qui permet de réutiliser le workflow d'acceptation mutuelle.
