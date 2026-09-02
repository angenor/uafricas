# Feature Specification: Marché Africain, acheter, vendre, troquer, donner

**Feature Branch**: `001-marche-achat-vente-troc-don`  
**Created**: 2026-05-26  
**Status**: Draft  
**Input**: User description: "la page marche-africain index.vue et [id].vue n'est pas encore fonctionnelle, on veut un espace où on peut acheter, vendre, troquer et donner"

## Contexte

La page « Marché Africain » permet déjà de **parcourir** et **consulter** des annonces (liste filtrable, recherche, page de détail). Mais l'espace n'est pas opérationnel pour les membres :

- le bouton « Publier » ouvre un message « fonctionnalité bientôt disponible » ;
- le bouton « Je suis intéressé(e) » sur le détail affiche une simple alerte sans effet réel ;
- un membre ne peut ni créer, ni modifier, ni retirer ses propres annonces ;
- aucun moyen d'entrer réellement en relation avec l'auteur d'une annonce.

Cette fonctionnalité rend le marché **vivant** : tout membre peut publier ce qu'il veut **vendre**, **troquer** ou **donner**, et tout visiteur peut **acheter / prendre / proposer un échange** en entrant en contact avec l'auteur.

## Clarifications

### Session 2026-05-26

- Q: Limites des photos d'une annonce (nombre et poids) → A: 5 photos max par annonce, 3 Mo max par photo, formats JPEG/PNG/WebP.
- Q: Qui peut publier, contacter et gérer des annonces ? → A: Uniquement les comptes actifs (email vérifié) ; les comptes en attente de vérification en sont exclus.
- Q: Règle d'expiration des annonces ? → A: Pas d'expiration automatique ; une annonce reste publiée jusqu'à conclusion ou suppression par l'auteur (ou retrait admin).
- Q: Plafond du nombre d'annonces actives par membre ? → A: Aucune limite ; la maîtrise des abus repose sur la restriction aux comptes vérifiés et la modération admin a posteriori.
- Q: Granularité de l'état de conclusion d'une annonce ? → A: Un seul état générique « conclue » ; l'issue (vendue/donnée/échangée) se déduit du type d'opération.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Publier une annonce (vendre, troquer, donner) (Priority: P1)

Un membre connecté veut mettre en ligne une annonce. Il choisit le type d'opération (**Vente**, **Troc** ou **Don**), renseigne un titre, une description, une catégorie, l'état de l'article, éventuellement un prix (et la devise) pour une vente, la quantité, les territoires ciblés, une localisation (ville, adresse, position) et ajoute une ou plusieurs photos. À la validation, l'annonce est **publiée immédiatement** et devient visible de tous dans le marché.

**Why this priority**: C'est le cœur du « pas fonctionnel ». Sans publication par les membres, le marché reste une vitrine vide alimentée seulement par l'administration. C'est le minimum viable qui transforme la page en véritable espace d'échange.

**Independent Test**: Se connecter en tant que membre, ouvrir le formulaire de publication, créer une annonce de chaque type (Vente, Troc, Don) avec au moins une photo, valider, puis vérifier qu'elle apparaît immédiatement dans la liste et sur sa page de détail.

**Acceptance Scenarios**:

1. **Given** un membre connecté, **When** il remplit le formulaire de publication avec un type, un titre, une description, une catégorie et au moins une photo puis valide, **Then** l'annonce est créée, publiée immédiatement et visible dans la liste du marché.
2. **Given** une annonce de type **Don** ou **Troc**, **When** l'auteur ne saisit pas de prix, **Then** l'annonce s'affiche comme « Gratuit » (Don) ou sans montant (Troc) sans bloquer la publication.
3. **Given** une annonce de type **Vente**, **When** l'auteur saisit un prix et une devise et coche « négociable », **Then** le prix et la mention « négociable » apparaissent sur l'annonce.
4. **Given** un visiteur **non connecté**, **When** il clique sur « Publier », **Then** il est invité à se connecter avant de pouvoir publier.
5. **Given** un formulaire incomplet (champ obligatoire manquant ou aucune photo), **When** le membre tente de valider, **Then** la publication est refusée avec un message indiquant les champs à corriger.

---

### User Story 2 - Entrer en relation avec l'auteur d'une annonce (acheter / prendre / proposer un troc) (Priority: P1)

Un membre connecté consulte une annonce qui l'intéresse. Il clique sur « Contacter » / « Je suis intéressé(e) ». Une conversation privée s'ouvre avec l'auteur de l'annonce (rattachée à l'annonce concernée), via la messagerie privée de la plateforme. Les deux membres peuvent alors échanger en temps réel pour convenir de l'achat, du don ou des modalités d'un troc. L'échange et le paiement éventuel se règlent entre les parties, hors plateforme (mise en relation uniquement).

**Why this priority**: Un marché sans moyen de contact ne permet ni d'acheter, ni de prendre un don, ni de proposer un troc. Cette interaction est aussi essentielle que la publication pour que l'espace soit « fonctionnel ».

**Independent Test**: Avec deux comptes membres, publier une annonce avec le premier, consulter cette annonce avec le second, cliquer sur « Contacter », envoyer un message, puis vérifier côté auteur que la conversation rattachée à l'annonce est bien reçue.

**Acceptance Scenarios**:

1. **Given** un membre connecté qui consulte une annonce d'un autre membre, **When** il clique sur « Contacter », **Then** une conversation privée rattachée à cette annonce s'ouvre avec l'auteur et il peut envoyer un message.
2. **Given** un visiteur **non connecté** sur une page d'annonce, **When** il veut contacter l'auteur, **Then** il est invité à se connecter d'abord.
3. **Given** l'auteur d'une annonce, **When** il consulte sa propre annonce, **Then** le bouton « Contacter » n'est pas proposé (on ne se contacte pas soi-même).
4. **Given** une conversation déjà ouverte sur une annonce, **When** un nouveau message est envoyé, **Then** le destinataire en est notifié et le retrouve dans sa messagerie.

---

### User Story 3 - Gérer ses propres annonces (Priority: P2)

L'auteur d'une annonce veut la gérer après publication. Depuis un espace « Mes annonces », il retrouve la liste de ses publications avec leur état, peut **modifier** le contenu et les photos, **marquer comme conclue** (vendue / donnée / échangée) pour la retirer de la liste publique tout en gardant l'historique, ou la **supprimer**.

**Why this priority**: Indispensable à la qualité du marché (annonces à jour, retrait des biens déjà cédés) mais non bloquant pour le premier échange de valeur (publier + contacter).

**Independent Test**: Publier une annonce, ouvrir « Mes annonces », modifier le titre et une photo, vérifier la mise à jour publique, puis marquer l'annonce comme conclue et vérifier qu'elle disparaît de la liste publique mais reste dans « Mes annonces ».

**Acceptance Scenarios**:

1. **Given** un membre ayant publié des annonces, **When** il ouvre « Mes annonces », **Then** il voit la liste de ses annonces avec leur état (publiée, conclue, suspendue).
2. **Given** l'auteur d'une annonce, **When** il en modifie le contenu ou les photos, **Then** les changements sont reflétés immédiatement sur la page publique.
3. **Given** l'auteur d'une annonce active, **When** il la marque comme conclue, **Then** elle est retirée de la liste publique mais reste consultable dans « Mes annonces ».
4. **Given** l'auteur d'une annonce, **When** il la supprime, **Then** elle n'apparaît plus nulle part publiquement.
5. **Given** un membre, **When** il tente de modifier ou supprimer une annonce dont il n'est pas l'auteur, **Then** l'action est refusée.

---

### User Story 4 - Sauvegarder une annonce en favori (Priority: P3)

Un membre connecté qui parcourt le marché veut retrouver facilement des annonces qui l'intéressent. Il les ajoute à ses favoris depuis la carte ou la page de détail, et retrouve sa liste de favoris à tout moment.

**Why this priority**: Améliore le confort et l'engagement, mais non essentiel pour acheter/vendre/troquer/donner.

**Independent Test**: Ajouter plusieurs annonces aux favoris, ouvrir la liste des favoris, vérifier qu'elles y figurent, en retirer une et vérifier sa disparition.

**Acceptance Scenarios**:

1. **Given** un membre connecté, **When** il ajoute une annonce à ses favoris, **Then** elle apparaît dans sa liste de favoris.
2. **Given** une annonce déjà en favori, **When** le membre la retire, **Then** elle disparaît de ses favoris.
3. **Given** un visiteur non connecté, **When** il veut ajouter un favori, **Then** il est invité à se connecter.

---

### Edge Cases

- **Don / Troc sans prix** : l'affichage doit indiquer « Gratuit » (Don) ou ne montrer aucun montant (Troc) sans donner l'impression d'une erreur.
- **Annonce retirée pendant consultation** : si une annonce est supprimée, conclue ou suspendue alors qu'un visiteur l'a ouverte, l'accès au contact et à la page doit échouer proprement (message « annonce introuvable ou plus disponible »).
- **Photos** : formats JPEG/PNG/WebP, 3 Mo max par photo, 5 photos max par annonce, avec désignation d'une photo principale ; un fichier non conforme (format, poids, ou au-delà de 5) est rejeté avec un message clair.
- **Auteur = visiteur** : un membre ne peut pas se contacter lui-même ni mettre sa propre annonce en favori de façon absurde (comportement clairement défini).
- **Pas d'expiration automatique** : une annonce publiée reste visible tant que l'auteur ne l'a pas conclue ni supprimée (et tant qu'un admin ne l'a pas retirée). Aucune disparition liée au temps.
- **Modération a posteriori** : une annonce publiée immédiatement peut être suspendue ou supprimée par un administrateur ; l'auteur doit comprendre l'état de son annonce.
- **Quantité** : pour une vente en lot, la quantité minimale doit être affichée et cohérente.
- **Contenu inapproprié** : la responsabilité initiale repose sur l'auteur (publication immédiate) ; l'administration dispose des moyens de retrait.

## Requirements *(mandatory)*

### Functional Requirements

**Publication et types d'opération**

- **FR-001**: Tout membre connecté MUST pouvoir publier une annonce de type **Vente**, **Troc** ou **Don**.
- **FR-002**: Le formulaire de publication MUST permettre de renseigner : type d'opération, titre, description, catégorie, état de l'article, quantité, territoires ciblés, ville/adresse/position (facultatives), et au moins une photo.
- **FR-003**: Pour une **Vente**, le membre MUST pouvoir saisir un prix, une devise et indiquer si le prix est négociable ; pour un **Don** ou un **Troc**, le prix MUST être facultatif.
- **FR-004**: Le système MUST publier l'annonce **immédiatement** après validation (visible de tous sans validation préalable).
- **FR-005**: Le système MUST permettre d'ajouter jusqu'à **5 photos** par annonce (au moins une), d'en désigner une comme principale, et MUST refuser tout fichier non conforme : formats acceptés **JPEG, PNG, WebP**, **3 Mo maximum** par photo.
- **FR-006**: Le système MUST refuser la publication si un champ obligatoire est manquant ou si aucune photo n'est fournie, avec un message d'erreur explicite.
- **FR-007**: Le système MUST réserver la publication aux **comptes actifs** (email vérifié) ; un visiteur non connecté MUST être invité à se connecter, et un compte en attente de vérification MUST être invité à vérifier son email avant de publier.

**Consultation et recherche** *(existant, à préserver)*

- **FR-008**: Le système MUST afficher la liste des annonces publiées avec filtres (catégorie, type d'opération, fourchette de prix), recherche textuelle, tri (récent, prix croissant/décroissant) et pagination.
- **FR-009**: Le système MUST afficher une page de détail d'annonce (photos, description, prix/mention, localisation, catégorie, date, auteur).
- **FR-010**: Le système MUST n'exposer publiquement que les annonces dans l'état **publiée** (excluant les annonces conclues, suspendues ou supprimées). Aucune expiration automatique n'est appliquée : une annonce publiée le reste jusqu'à conclusion ou suppression.

**Mise en relation acheteur ↔ auteur**

- **FR-011**: Un membre connecté MUST pouvoir contacter l'auteur d'une annonce depuis sa page de détail, ce qui MUST ouvrir une conversation privée rattachée à l'annonce concernée via la messagerie privée de la plateforme.
- **FR-012**: Le système MUST notifier l'auteur lorsqu'un membre le contacte au sujet d'une de ses annonces.
- **FR-013**: Le système MUST NOT proposer à un membre de contacter sa propre annonce.
- **FR-014**: Le système MUST réserver le contact d'un auteur et la gestion des favoris aux **comptes actifs** (email vérifié) ; un visiteur non connecté MUST être invité à se connecter.
- **FR-015**: Le périmètre MUST se limiter à la mise en relation ; le système ne gère **pas** de paiement ni de règlement en ligne (transaction réglée entre les parties hors plateforme).

**Gestion de ses annonces**

- **FR-016**: L'auteur MUST pouvoir consulter la liste de ses propres annonces (« Mes annonces ») avec leur état.
- **FR-017**: L'auteur MUST pouvoir modifier le contenu et les photos de ses annonces ; les modifications MUST être reflétées sur la page publique.
- **FR-018**: L'auteur MUST pouvoir marquer une annonce comme **conclue** (état générique unique ; l'issue vendue/donnée/échangée se déduit du type d'opération), ce qui la retire de la liste publique tout en la conservant dans « Mes annonces ».
- **FR-019**: L'auteur MUST pouvoir supprimer ses annonces.
- **FR-020**: Le système MUST empêcher un membre de modifier, conclure ou supprimer une annonce dont il n'est pas l'auteur.

**Favoris**

- **FR-021**: Un membre connecté MUST pouvoir ajouter une annonce à ses favoris et l'en retirer.
- **FR-022**: Un membre connecté MUST pouvoir consulter sa liste de favoris.

**Modération**

- **FR-023**: Un administrateur MUST pouvoir suspendre ou supprimer toute annonce après sa publication (modération a posteriori).
- **FR-024**: Le système MUST refléter clairement l'état d'une annonce à son auteur (publiée, conclue, suspendue).

### Key Entities *(include if feature involves data)*

- **Annonce** : l'offre publiée par un membre. Attributs clés : type d'opération (vente / troc / don), titre, description, catégorie, état de l'article (neuf / occasion / reconditionné / sans objet), prix et devise (facultatifs hors vente), prix négociable, quantité, localisation (territoires ciblés, ville, adresse, position), état du cycle de vie (publiée, conclue, suspendue, supprimée), date de publication, auteur, nombre de vues.
- **Photo d'annonce** : image associée à une annonce, avec un ordre d'affichage et l'indication de la photo principale.
- **Territoire ciblé** : un ou plusieurs territoires que l'annonce concerne (relation multiple avec l'annonce).
- **Favori** : lien entre un membre et une annonce qu'il a sauvegardée.
- **Conversation d'annonce** : échange privé entre un membre intéressé et l'auteur, rattaché à une annonce, porté par la messagerie privée de la plateforme.
- **Auteur / Membre** : l'utilisateur qui publie une annonce ou qui contacte un auteur.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Un membre peut publier une annonce complète (avec photo) en moins de 3 minutes, et celle-ci apparaît dans la liste publique en moins de 5 secondes après validation.
- **SC-002**: 100 % des annonces publiées par un membre sont visibles de tous immédiatement (aucune attente de validation), et restent gérables par leur auteur.
- **SC-003**: Un membre intéressé peut entrer en contact avec l'auteur d'une annonce en au plus 2 actions depuis la page de détail, et l'auteur reçoit le message.
- **SC-004**: 90 % des membres testeurs réussissent à publier une annonce et à contacter un auteur du premier coup, sans aide.
- **SC-005**: Les trois types d'opération (vendre, troquer, donner) sont publiables et clairement distinguables à l'affichage (badge / mention dédiée).
- **SC-006**: Un auteur peut retirer une annonce conclue de la vue publique en 1 action, et elle n'apparaît plus dans les listes publiques tout en restant dans son historique.
- **SC-007**: Aucune action de publication, contact, modification ou favori n'est possible sans connexion ; les visiteurs non connectés sont systématiquement redirigés vers la connexion.

## Assumptions

- **Modèle de marché** : petites annonces de **mise en relation** uniquement. Aucun paiement, panier, commande ou règlement en ligne n'est inclus (choix confirmé : « mise en relation seule »).
- **Publication immédiate** : les annonces des membres sont visibles sans validation préalable (choix confirmé). La qualité est gérée par modération a posteriori côté administration.
- **Canal de contact** : la mise en relation s'appuie sur la **messagerie privée temps réel existante** de la plateforme, en rattachant la conversation à l'annonce (choix confirmé).
- **Types d'opération retenus** : Vente, Troc, Don. Les autres valeurs techniques éventuelles (association, opportunité) ne sont pas exposées dans ce parcours membre.
- **Terminologie** : l'interface affiche « territoire » plutôt que « pays » (convention projet), et désigne l'émetteur de l'annonce comme « auteur / annonceur » plutôt que « vendeur » (car troc et don ne sont pas des ventes).
- **Réutilisation de l'existant** : la consultation/recherche/détail déjà en place est conservée ; la gestion administrative (CRUD complet, modération) existe déjà et est étendue par la gestion membre.
- **Référentiels** : catégories, territoires et devises proviennent des référentiels existants de la plateforme.
- **Anti-abus** : aucun plafond sur le nombre d'annonces actives par membre ; la maîtrise des abus repose sur la restriction aux comptes vérifiés (FR-007) et la modération a posteriori (FR-023).
