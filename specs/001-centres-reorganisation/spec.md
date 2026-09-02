# Feature Specification: Réorganisation des centres culturels (routes + administration)

**Feature Branch**: `001-centres-reorganisation`
**Created**: 2026-04-19
**Status**: Draft
**Input**: User description: "dans l'admin, on doit pouvoir ajouter des centre culturel qui s'afficherons dans @uafricas_frontend/app/pages/africain-afro-americain/index.vue , changer le lien `/africain-afro-americain` en `/centres` , lorsqu'on clique sur un centre, on tombe sur `/site/[id]` ce n'est pas logique, reoganiser pour qu'on soit sur `/centres/[id]` , on ne doit pas pouvoir y ajouter directement une programmation. Les programmations doivent pouvoir s'ajouter dans le backoffice par l'admin. Reorganiser et renommer la page de détails d'une programmation pour que le lien soit cohérent"

## Clarifications

### Session 2026-04-19

- Q: Motif canonique d'URL pour le détail d'une programmation ? → A: `/centres/{centreId}/programmations/{programmationId}` (pluriel, aligné sur la ressource collection parent `/centres`)
- Q: Source des visuels du carrousel d'en-tête de `/centres` ? → A: Agrégation automatique des images de couverture des centres publiés (aucune gestion séparée côté admin)
- Q: Ordre d'affichage des programmations sur la fiche publique d'un centre ? → A: Programmations à venir d'abord (date croissante), puis programmations passées (date décroissante)
- Q: Stratégie pour les centres et programmations existants au déploiement ? → A: Publication automatique de tout l'existant (contexte non-production, pas de risque de fuite de contenu)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Administration des centres culturels visibles en public (Priority: P1)

Un administrateur doit pouvoir créer, modifier et publier un centre culturel depuis le back-office. Une fois publié, le centre apparaît dans la liste publique des centres culturels africains et afro-descendants avec son image de couverture, son nom, sa localisation et sa description. Les visiteurs peuvent parcourir cette liste et ouvrir la fiche détaillée de chaque centre.

**Why this priority**: C'est la valeur métier principale, sans la chaîne « l'admin crée → le visiteur voit », la page publique des centres culturels reste non actualisable par les équipes éditoriales. Cette user story livre à elle seule un produit utilisable (plateforme éditoriale + vitrine publique).

**Independent Test**: Se connecter en admin, créer un nouveau centre culturel (nom, description, pays, image), le publier, puis vérifier en navigation publique anonyme que le centre apparaît dans la liste et que sa fiche détaillée est accessible.

**Acceptance Scenarios**:

1. **Given** un administrateur authentifié, **When** il crée un centre culturel avec les informations minimales requises et le publie, **Then** ce centre apparaît dans la liste publique des centres culturels dans la foulée (sans redéploiement).
2. **Given** un centre culturel déjà publié, **When** l'administrateur modifie son nom ou son image de couverture, **Then** les modifications sont reflétées sur la page publique lors de la prochaine consultation.
3. **Given** un administrateur, **When** il dépublie ou supprime logiquement un centre, **Then** ce centre n'apparaît plus dans la liste publique et sa fiche détaillée retourne une réponse « introuvable ».
4. **Given** un visiteur anonyme, **When** il arrive sur la liste des centres, **Then** il voit le nombre total de centres actifs et peut cliquer sur une carte pour ouvrir la fiche détaillée.

---

### User Story 2 - Routes publiques cohérentes et hiérarchiques (Priority: P1)

Les routes publiques des centres culturels doivent refléter la hiérarchie métier et être lisibles par les utilisateurs et les moteurs de recherche. La liste publique est servie sous `/centres`, la fiche détaillée d'un centre sous `/centres/{id}`, et la fiche détaillée d'une programmation rattachée à un centre suit la même hiérarchie (sous `/centres/{id}/...`). Les anciennes URLs (`/africain-afro-americain`, `/site/{id}`, `/site/{id}/programmation/{programmationId}`) redirigent vers les nouvelles pour préserver les liens externes existants.

**Why this priority**: Les routes actuelles (`/africain-afro-americain`, `/site/{id}`) sont incohérentes (nommage non aligné sur le domaine métier « centres culturels », rupture de hiérarchie parent/enfant). Cette refonte améliore la compréhension, la navigation par fil d'Ariane, le référencement et la maintenabilité, sans casser les liens déjà partagés.

**Independent Test**: Ouvrir chacune des trois anciennes URLs en navigateur et vérifier qu'elles redirigent (redirection permanente) vers les nouvelles URLs équivalentes, puis confirmer que le contenu affiché est identique à l'ancien comportement.

**Acceptance Scenarios**:

1. **Given** un visiteur qui saisit `/africain-afro-americain`, **When** la page se charge, **Then** il est redirigé de façon permanente vers `/centres` et voit la liste des centres culturels.
2. **Given** un centre existant d'identifiant `X`, **When** un visiteur ouvre `/site/X`, **Then** il est redirigé vers `/centres/X` et voit la fiche détaillée.
3. **Given** une programmation existante d'identifiant `P` rattachée au centre `X`, **When** un visiteur ouvre l'ancienne URL de détail programmation, **Then** il est redirigé vers la nouvelle URL cohérente (sous `/centres/{id}/...`) et voit le détail de la programmation.
4. **Given** tous les liens internes de la plateforme (menus, fils d'Ariane, cartes de centre), **When** un utilisateur clique sur un lien, **Then** il arrive directement sur la nouvelle URL (aucun lien interne ne doit pointer vers une ancienne URL).
5. **Given** un visiteur sur une fiche centre, **When** il clique sur une programmation listée, **Then** l'URL affichée respecte la hiérarchie parent-enfant (le chemin contient la référence au centre parent).

---

### User Story 3 - Administration exclusive des programmations (Priority: P2)

Les programmations (événements culturels d'un centre) ne peuvent être créées ou modifiées que par un administrateur depuis le back-office. La fiche publique d'un centre affiche ses programmations en lecture seule, sans bouton, formulaire, ni lien permettant à un visiteur ou un utilisateur authentifié non-admin d'ajouter une nouvelle programmation depuis les pages publiques.

**Why this priority**: Gouvernance éditoriale : seule l'équipe admin valide le contenu mis en avant (crédibilité, modération, qualité). La règle doit être appliquée de façon stricte sur les pages publiques.

**Independent Test**: Visiter la fiche publique d'un centre en tant que visiteur anonyme puis en tant qu'utilisateur standard authentifié : vérifier qu'aucune action de création de programmation n'est offerte. Puis se connecter en admin, créer une programmation rattachée à ce centre via le back-office, et confirmer qu'elle apparaît sur la fiche publique.

**Acceptance Scenarios**:

1. **Given** un visiteur anonyme ou un utilisateur authentifié non-admin, **When** il consulte la fiche publique d'un centre, **Then** aucun élément d'interface ne lui permet de créer une programmation.
2. **Given** un administrateur, **When** il ouvre la section « Programmations » du back-office, **Then** il peut créer, modifier, publier ou supprimer logiquement une programmation et la rattacher à un centre existant.
3. **Given** une programmation créée et publiée par l'admin, **When** un visiteur consulte la fiche du centre parent, **Then** la programmation apparaît dans la liste des programmations du centre avec un lien vers sa fiche détaillée.
4. **Given** une tentative d'accès direct à une URL de création de programmation publique, **When** elle n'est pas autorisée, **Then** l'utilisateur voit un message clair ou est redirigé vers la page publique du centre.

---

### Edge Cases

- Un visiteur partage un lien externe avec une ancienne URL (`/africain-afro-americain` ou `/site/{id}`) sur les réseaux sociaux : la redirection permanente doit fonctionner et préserver l'aperçu social (titre, description, image).
- Un identifiant de centre passé dans l'URL n'existe pas ou a été supprimé logiquement : afficher une page « introuvable » claire proposant un retour à la liste `/centres`.
- Une programmation existe mais son centre parent a été supprimé : la fiche programmation doit soit afficher un message explicatif, soit rediriger vers la liste des centres.
- Un administrateur crée un centre sans image de couverture : la page publique doit afficher un visuel par défaut cohérent et ne pas laisser d'espace vide.
- Un centre est publié sans aucune programmation : sa fiche publique doit afficher clairement « Aucune programmation pour le moment » plutôt qu'une section vide.
- Un visiteur accède à l'URL d'une programmation dont le centre parent dans l'URL ne correspond pas (lien corrompu) : rediriger vers la bonne URL canonique de la programmation ou afficher l'erreur appropriée.
- Les anciennes URLs sont indexées par les moteurs de recherche : la redirection doit être permanente pour transférer l'autorité de référencement aux nouvelles URLs.

## Requirements *(mandatory)*

### Functional Requirements

**Administration des centres culturels (User Story 1)**

- **FR-001** : Le back-office administrateur DOIT permettre de créer un centre culturel en saisissant au minimum un nom, une description, un pays, et optionnellement une image de couverture et des métadonnées éditoriales.
- **FR-002** : Le back-office administrateur DOIT permettre de modifier les informations d'un centre culturel existant et de publier ou dépublier ce centre.
- **FR-003** : Le back-office administrateur DOIT permettre de supprimer logiquement (archivage récupérable) un centre culturel.
- **FR-004** : Seuls les centres publiés (non archivés) DOIVENT apparaître dans la liste publique et être accessibles via leur fiche détaillée publique.
- **FR-005** : La liste publique des centres DOIT refléter les créations, modifications et suppressions effectuées par l'administrateur sans redéploiement applicatif.
- **FR-005a** : Le carrousel d'en-tête de `/centres` DOIT afficher automatiquement les images de couverture des centres publiés (sans gestion éditoriale séparée). Lorsqu'aucun centre publié n'a d'image de couverture, le carrousel DOIT basculer sur un visuel par défaut cohérent avec la charte.

**Routes publiques cohérentes (User Story 2)**

- **FR-006** : La liste publique des centres culturels DOIT être servie à l'URL canonique `/centres`.
- **FR-007** : La fiche détaillée d'un centre culturel DOIT être servie à l'URL canonique `/centres/{centreId}`.
- **FR-008** : La fiche détaillée d'une programmation DOIT être servie à l'URL canonique `/centres/{centreId}/programmations/{programmationId}` (segment parent et segment collection au pluriel).
- **FR-009** : L'ancienne URL `/africain-afro-americain` DOIT rediriger de façon permanente vers `/centres`.
- **FR-010** : L'ancienne URL `/site/{centreId}` DOIT rediriger de façon permanente vers `/centres/{centreId}`.
- **FR-011** : L'ancienne URL de détail d'une programmation (sous `/site/{centreId}/...`) DOIT rediriger de façon permanente vers la nouvelle URL hiérarchique équivalente.
- **FR-012** : Tous les liens internes de l'application (menus de navigation, fils d'Ariane, cartes, widgets, notifications, emails transactionnels) DOIVENT pointer vers les nouvelles URLs canoniques et ne plus référencer les anciennes.
- **FR-013** : Les fils d'Ariane et boutons « retour » des fiches centre et programmation DOIVENT exprimer la hiérarchie (liste des centres → centre → programmation).

**Administration exclusive des programmations (User Story 3)**

- **FR-014** : Aucune page publique NE DOIT exposer d'action (bouton, lien, formulaire) permettant à un visiteur ou un utilisateur non-admin de créer ou modifier une programmation.
- **FR-015** : Le back-office administrateur DOIT rester le seul canal de création, modification, publication et suppression logique des programmations.
- **FR-016** : Toute tentative d'accès direct à un écran de création ou de modification de programmation par un utilisateur non-admin DOIT être refusée avec un message clair ou une redirection vers la page publique.
- **FR-017** : La fiche publique d'un centre DOIT afficher en lecture seule la liste de ses programmations publiées, avec un lien vers la fiche détaillée de chacune (selon la nouvelle URL hiérarchique).
- **FR-017a** : La liste des programmations sur la fiche publique d'un centre DOIT être triée en affichant d'abord les programmations à venir par date croissante (la plus proche en haut), puis les programmations passées par date décroissante (la plus récemment passée en premier).

**Qualité générale**

- **FR-018** : Les pages `/centres` et `/centres/{centreId}` DOIVENT rester accessibles sans authentification.
- **FR-019** : Les métadonnées de partage social (titre, description, image) des pages `/centres` et `/centres/{centreId}` DOIVENT refléter le centre affiché.

### Key Entities *(include if feature involves data)*

- **Centre culturel** : entité éditoriale représentant un lieu ou une organisation culturelle africaine ou afro-descendante. Attributs clés : identifiant, nom, description, pays, image de couverture, état de publication, date de création, date de mise à jour, état d'archivage. Relation : possède plusieurs programmations.
- **Programmation** : événement ou activité culturelle rattachée à un centre (date, titre, description, visuel, état de publication). Relation : appartient à un centre culturel.
- **Administrateur** : utilisateur disposant des droits d'édition sur les centres et les programmations depuis le back-office.
- **Visiteur public** : utilisateur anonyme ou authentifié non-admin qui consulte en lecture seule les centres publiés et leurs programmations publiées.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001** : Un administrateur peut créer et publier un nouveau centre culturel visible en public en moins de 3 minutes, sans intervention technique.
- **SC-002** : 100 % des anciens liens publics (`/africain-afro-americain`, `/site/{id}`, ancienne URL de détail programmation) retournent une redirection permanente vers la nouvelle URL équivalente.
- **SC-003** : 100 % des liens internes de l'application (audit automatique ou manuel) pointent vers les nouvelles URLs après la mise en production.
- **SC-004** : Aucun utilisateur non-admin ne dispose d'une action de création de programmation visible ou accessible depuis les pages publiques (validé par test anonyme et test utilisateur standard).
- **SC-005** : La page `/centres` affiche sa liste de centres en moins de 2 secondes sur une connexion standard (95ᵉ percentile).
- **SC-006** : Les moteurs de recherche indexent les nouvelles URLs canoniques sans doublon des anciennes (vérifiable via outils de référencement dans les 30 jours suivant la mise en ligne).
- **SC-007** : Le taux de clic de la liste des centres vers une fiche centre reste équivalent ou supérieur à celui observé sur `/africain-afro-americain` avant la refonte (mesure comparative sur 14 jours).

## Assumptions

- Les centres culturels et les programmations sont déjà modélisés dans le système ; la refonte vise l'expérience publique et l'exhaustivité du back-office, pas la création d'un nouveau domaine métier.
- Les redirections permanentes sont la stratégie retenue pour préserver le référencement et les liens externes, plutôt qu'une suppression des anciennes URLs.
- Un administrateur désigne tout utilisateur disposant du rôle permettant déjà l'accès au back-office des centres et programmations ; aucun nouveau rôle n'est introduit par cette feature.
- L'image de couverture par défaut pour un centre sans visuel est un visuel générique cohérent avec la charte graphique.
- Les programmations publiées par l'admin deviennent immédiatement visibles publiquement sur la fiche du centre, sans étape de modération supplémentaire propre à cette feature.
- Les exports, emails ou notifications citant des URLs de centres ou programmations seront régénérés avec les nouvelles URLs ; les envois historiques déjà partagés restent valides grâce aux redirections permanentes.
- Au moment du déploiement de cette feature, la plateforme n'est pas encore en production : l'ensemble des centres et programmations déjà présents en base est considéré comme publié (aucune mise en sommeil ni revalidation admin requise). Cette hypothèse devra être réévaluée pour toute future refonte post-mise en production.
- Vocabulaire « publié / dépublié / archivage récupérable » : ces termes fonctionnels correspondent au flag SQL `culture.centre_culturel.actif BOOLEAN` (défaut `TRUE`). Basculer à `FALSE` rend le centre invisible en public sans perte de donnée (restaurable en remettant à `TRUE`). La table `culture.programmation_centre` n'a pas de flag propre : la visibilité d'une programmation suit celle de son centre parent. Aucun mécanisme distinct de soft-delete (`deleted_at`) n'existe sur ces tables.

## Dependencies

- Droits d'accès admin existants pour la gestion des centres et programmations (aucun nouveau rôle requis).
- Mécanisme de redirection permanente des anciennes vers les nouvelles URLs fourni par la plateforme.
- Charte graphique pour le visuel de couverture par défaut d'un centre sans image.
